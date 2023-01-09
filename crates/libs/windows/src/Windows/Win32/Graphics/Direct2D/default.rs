impl ::core::default::Default for D2D1_2DAFFINETRANSFORM_PROP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_2DAFFINETRANSFORM_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_2DAFFINETRANSFORM_PROP").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_3DPERSPECTIVETRANSFORM_INTERPOLATION_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_3DPERSPECTIVETRANSFORM_INTERPOLATION_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_3DPERSPECTIVETRANSFORM_INTERPOLATION_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_3DPERSPECTIVETRANSFORM_PROP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_3DPERSPECTIVETRANSFORM_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_3DPERSPECTIVETRANSFORM_PROP").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_3DTRANSFORM_INTERPOLATION_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_3DTRANSFORM_INTERPOLATION_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_3DTRANSFORM_INTERPOLATION_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_3DTRANSFORM_PROP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_3DTRANSFORM_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_3DTRANSFORM_PROP").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_ANTIALIAS_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_ANTIALIAS_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_ANTIALIAS_MODE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::default::Default for D2D1_ARC_SEGMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::cmp::PartialEq for D2D1_ARC_SEGMENT {
    fn eq(&self, other: &Self) -> bool {
        self.point == other.point && self.size == other.size && self.rotationAngle == other.rotationAngle && self.sweepDirection == other.sweepDirection && self.arcSize == other.arcSize
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::cmp::Eq for D2D1_ARC_SEGMENT {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::fmt::Debug for D2D1_ARC_SEGMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D2D1_ARC_SEGMENT").field("point", &self.point).field("size", &self.size).field("rotationAngle", &self.rotationAngle).field("sweepDirection", &self.sweepDirection).field("arcSize", &self.arcSize).finish()
    }
}
impl ::core::default::Default for D2D1_ARC_SIZE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_ARC_SIZE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_ARC_SIZE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_ARITHMETICCOMPOSITE_PROP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_ARITHMETICCOMPOSITE_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_ARITHMETICCOMPOSITE_PROP").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_ATLAS_PROP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_ATLAS_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_ATLAS_PROP").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_BITMAPSOURCE_ALPHA_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_BITMAPSOURCE_ALPHA_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_BITMAPSOURCE_ALPHA_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_BITMAPSOURCE_INTERPOLATION_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_BITMAPSOURCE_INTERPOLATION_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_BITMAPSOURCE_INTERPOLATION_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_BITMAPSOURCE_ORIENTATION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_BITMAPSOURCE_ORIENTATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_BITMAPSOURCE_ORIENTATION").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_BITMAPSOURCE_PROP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_BITMAPSOURCE_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_BITMAPSOURCE_PROP").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_BITMAP_BRUSH_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D2D1_BITMAP_BRUSH_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        self.extendModeX == other.extendModeX && self.extendModeY == other.extendModeY && self.interpolationMode == other.interpolationMode
    }
}
impl ::core::cmp::Eq for D2D1_BITMAP_BRUSH_PROPERTIES {}
impl ::core::fmt::Debug for D2D1_BITMAP_BRUSH_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D2D1_BITMAP_BRUSH_PROPERTIES").field("extendModeX", &self.extendModeX).field("extendModeY", &self.extendModeY).field("interpolationMode", &self.interpolationMode).finish()
    }
}
impl ::core::default::Default for D2D1_BITMAP_BRUSH_PROPERTIES1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D2D1_BITMAP_BRUSH_PROPERTIES1 {
    fn eq(&self, other: &Self) -> bool {
        self.extendModeX == other.extendModeX && self.extendModeY == other.extendModeY && self.interpolationMode == other.interpolationMode
    }
}
impl ::core::cmp::Eq for D2D1_BITMAP_BRUSH_PROPERTIES1 {}
impl ::core::fmt::Debug for D2D1_BITMAP_BRUSH_PROPERTIES1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D2D1_BITMAP_BRUSH_PROPERTIES1").field("extendModeX", &self.extendModeX).field("extendModeY", &self.extendModeY).field("interpolationMode", &self.interpolationMode).finish()
    }
}
impl ::core::default::Default for D2D1_BITMAP_INTERPOLATION_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_BITMAP_INTERPOLATION_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_BITMAP_INTERPOLATION_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_BITMAP_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_BITMAP_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_BITMAP_OPTIONS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D2D1_BITMAP_OPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D2D1_BITMAP_OPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D2D1_BITMAP_OPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D2D1_BITMAP_OPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D2D1_BITMAP_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::default::Default for D2D1_BITMAP_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::cmp::PartialEq for D2D1_BITMAP_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        self.pixelFormat == other.pixelFormat && self.dpiX == other.dpiX && self.dpiY == other.dpiY
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::cmp::Eq for D2D1_BITMAP_PROPERTIES {}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::fmt::Debug for D2D1_BITMAP_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D2D1_BITMAP_PROPERTIES").field("pixelFormat", &self.pixelFormat).field("dpiX", &self.dpiX).field("dpiY", &self.dpiY).finish()
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::default::Default for D2D1_BITMAP_PROPERTIES1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::cmp::PartialEq for D2D1_BITMAP_PROPERTIES1 {
    fn eq(&self, other: &Self) -> bool {
        self.pixelFormat == other.pixelFormat && self.dpiX == other.dpiX && self.dpiY == other.dpiY && self.bitmapOptions == other.bitmapOptions && self.colorContext == other.colorContext
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::cmp::Eq for D2D1_BITMAP_PROPERTIES1 {}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::fmt::Debug for D2D1_BITMAP_PROPERTIES1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D2D1_BITMAP_PROPERTIES1").field("pixelFormat", &self.pixelFormat).field("dpiX", &self.dpiX).field("dpiY", &self.dpiY).field("bitmapOptions", &self.bitmapOptions).field("colorContext", &self.colorContext).finish()
    }
}
impl ::core::default::Default for D2D1_BLEND {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_BLEND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_BLEND").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_BLEND_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D2D1_BLEND_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.sourceBlend == other.sourceBlend && self.destinationBlend == other.destinationBlend && self.blendOperation == other.blendOperation && self.sourceBlendAlpha == other.sourceBlendAlpha && self.destinationBlendAlpha == other.destinationBlendAlpha && self.blendOperationAlpha == other.blendOperationAlpha && self.blendFactor == other.blendFactor
    }
}
impl ::core::cmp::Eq for D2D1_BLEND_DESCRIPTION {}
impl ::core::fmt::Debug for D2D1_BLEND_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D2D1_BLEND_DESCRIPTION").field("sourceBlend", &self.sourceBlend).field("destinationBlend", &self.destinationBlend).field("blendOperation", &self.blendOperation).field("sourceBlendAlpha", &self.sourceBlendAlpha).field("destinationBlendAlpha", &self.destinationBlendAlpha).field("blendOperationAlpha", &self.blendOperationAlpha).field("blendFactor", &self.blendFactor).finish()
    }
}
impl ::core::default::Default for D2D1_BLEND_OPERATION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_BLEND_OPERATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_BLEND_OPERATION").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_BLEND_PROP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_BLEND_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_BLEND_PROP").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_BORDER_EDGE_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_BORDER_EDGE_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_BORDER_EDGE_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_BORDER_PROP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_BORDER_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_BORDER_PROP").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_BRIGHTNESS_PROP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_BRIGHTNESS_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_BRIGHTNESS_PROP").field(&self.0).finish()
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::default::Default for D2D1_BRUSH_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::cmp::PartialEq for D2D1_BRUSH_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        self.opacity == other.opacity && self.transform == other.transform
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::cmp::Eq for D2D1_BRUSH_PROPERTIES {}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::fmt::Debug for D2D1_BRUSH_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D2D1_BRUSH_PROPERTIES").field("opacity", &self.opacity).field("transform", &self.transform).finish()
    }
}
impl ::core::default::Default for D2D1_BUFFER_PRECISION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_BUFFER_PRECISION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_BUFFER_PRECISION").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_CAP_STYLE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_CAP_STYLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_CAP_STYLE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_CHANGE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_CHANGE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_CHANGE_TYPE").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D2D1_CHANGE_TYPE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D2D1_CHANGE_TYPE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D2D1_CHANGE_TYPE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D2D1_CHANGE_TYPE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D2D1_CHANGE_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for D2D1_CHANNEL_DEPTH {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_CHANNEL_DEPTH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_CHANNEL_DEPTH").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_CHANNEL_SELECTOR {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_CHANNEL_SELECTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_CHANNEL_SELECTOR").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_CHROMAKEY_PROP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_CHROMAKEY_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_CHROMAKEY_PROP").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_COLORMANAGEMENT_ALPHA_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_COLORMANAGEMENT_ALPHA_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_COLORMANAGEMENT_ALPHA_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_COLORMANAGEMENT_PROP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_COLORMANAGEMENT_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_COLORMANAGEMENT_PROP").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_COLORMANAGEMENT_QUALITY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_COLORMANAGEMENT_QUALITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_COLORMANAGEMENT_QUALITY").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_COLORMANAGEMENT_RENDERING_INTENT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_COLORMANAGEMENT_RENDERING_INTENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_COLORMANAGEMENT_RENDERING_INTENT").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_COLORMATRIX_PROP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_COLORMATRIX_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_COLORMATRIX_PROP").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_COLOR_CONTEXT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_COLOR_CONTEXT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_COLOR_CONTEXT_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_COLOR_INTERPOLATION_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_COLOR_INTERPOLATION_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_COLOR_INTERPOLATION_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_COLOR_SPACE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_COLOR_SPACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_COLOR_SPACE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_COMBINE_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_COMBINE_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_COMBINE_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for D2D1_COMPOSITE_PROP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_COMPOSITE_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_COMPOSITE_PROP").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_CONTRAST_PROP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_CONTRAST_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_CONTRAST_PROP").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_CONVOLVEMATRIX_PROP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_CONVOLVEMATRIX_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_CONVOLVEMATRIX_PROP").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_CONVOLVEMATRIX_SCALE_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_CONVOLVEMATRIX_SCALE_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_CONVOLVEMATRIX_SCALE_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_CREATION_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D2D1_CREATION_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        self.threadingMode == other.threadingMode && self.debugLevel == other.debugLevel && self.options == other.options
    }
}
impl ::core::cmp::Eq for D2D1_CREATION_PROPERTIES {}
impl ::core::fmt::Debug for D2D1_CREATION_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D2D1_CREATION_PROPERTIES").field("threadingMode", &self.threadingMode).field("debugLevel", &self.debugLevel).field("options", &self.options).finish()
    }
}
impl ::core::default::Default for D2D1_CROP_PROP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_CROP_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_CROP_PROP").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_CROSSFADE_PROP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_CROSSFADE_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_CROSSFADE_PROP").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for D2D1_CUSTOM_VERTEX_BUFFER_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::PartialEq for D2D1_CUSTOM_VERTEX_BUFFER_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        self.shaderBufferWithInputSignature == other.shaderBufferWithInputSignature && self.shaderBufferSize == other.shaderBufferSize && self.inputElements == other.inputElements && self.elementCount == other.elementCount && self.stride == other.stride
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::Eq for D2D1_CUSTOM_VERTEX_BUFFER_PROPERTIES {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::fmt::Debug for D2D1_CUSTOM_VERTEX_BUFFER_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D2D1_CUSTOM_VERTEX_BUFFER_PROPERTIES").field("shaderBufferWithInputSignature", &self.shaderBufferWithInputSignature).field("shaderBufferSize", &self.shaderBufferSize).field("inputElements", &self.inputElements).field("elementCount", &self.elementCount).field("stride", &self.stride).finish()
    }
}
impl ::core::default::Default for D2D1_DASH_STYLE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_DASH_STYLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_DASH_STYLE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_DC_INITIALIZE_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_DC_INITIALIZE_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_DC_INITIALIZE_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_DEBUG_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_DEBUG_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_DEBUG_LEVEL").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_DEVICE_CONTEXT_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_DEVICE_CONTEXT_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_DEVICE_CONTEXT_OPTIONS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D2D1_DEVICE_CONTEXT_OPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D2D1_DEVICE_CONTEXT_OPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D2D1_DEVICE_CONTEXT_OPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D2D1_DEVICE_CONTEXT_OPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D2D1_DEVICE_CONTEXT_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for D2D1_DIRECTIONALBLUR_OPTIMIZATION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_DIRECTIONALBLUR_OPTIMIZATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_DIRECTIONALBLUR_OPTIMIZATION").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_DIRECTIONALBLUR_PROP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_DIRECTIONALBLUR_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_DIRECTIONALBLUR_PROP").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_DISCRETETRANSFER_PROP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_DISCRETETRANSFER_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_DISCRETETRANSFER_PROP").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_DISPLACEMENTMAP_PROP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_DISPLACEMENTMAP_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_DISPLACEMENTMAP_PROP").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_DISTANTDIFFUSE_PROP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_DISTANTDIFFUSE_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_DISTANTDIFFUSE_PROP").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_DISTANTDIFFUSE_SCALE_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_DISTANTDIFFUSE_SCALE_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_DISTANTDIFFUSE_SCALE_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_DISTANTSPECULAR_PROP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_DISTANTSPECULAR_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_DISTANTSPECULAR_PROP").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_DISTANTSPECULAR_SCALE_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_DISTANTSPECULAR_SCALE_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_DISTANTSPECULAR_SCALE_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_DPICOMPENSATION_INTERPOLATION_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_DPICOMPENSATION_INTERPOLATION_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_DPICOMPENSATION_INTERPOLATION_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_DPICOMPENSATION_PROP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_DPICOMPENSATION_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_DPICOMPENSATION_PROP").field(&self.0).finish()
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::default::Default for D2D1_DRAWING_STATE_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::cmp::PartialEq for D2D1_DRAWING_STATE_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.antialiasMode == other.antialiasMode && self.textAntialiasMode == other.textAntialiasMode && self.tag1 == other.tag1 && self.tag2 == other.tag2 && self.transform == other.transform
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::cmp::Eq for D2D1_DRAWING_STATE_DESCRIPTION {}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::fmt::Debug for D2D1_DRAWING_STATE_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D2D1_DRAWING_STATE_DESCRIPTION").field("antialiasMode", &self.antialiasMode).field("textAntialiasMode", &self.textAntialiasMode).field("tag1", &self.tag1).field("tag2", &self.tag2).field("transform", &self.transform).finish()
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::default::Default for D2D1_DRAWING_STATE_DESCRIPTION1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::cmp::PartialEq for D2D1_DRAWING_STATE_DESCRIPTION1 {
    fn eq(&self, other: &Self) -> bool {
        self.antialiasMode == other.antialiasMode && self.textAntialiasMode == other.textAntialiasMode && self.tag1 == other.tag1 && self.tag2 == other.tag2 && self.transform == other.transform && self.primitiveBlend == other.primitiveBlend && self.unitMode == other.unitMode
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::cmp::Eq for D2D1_DRAWING_STATE_DESCRIPTION1 {}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::fmt::Debug for D2D1_DRAWING_STATE_DESCRIPTION1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D2D1_DRAWING_STATE_DESCRIPTION1").field("antialiasMode", &self.antialiasMode).field("textAntialiasMode", &self.textAntialiasMode).field("tag1", &self.tag1).field("tag2", &self.tag2).field("transform", &self.transform).field("primitiveBlend", &self.primitiveBlend).field("unitMode", &self.unitMode).finish()
    }
}
impl ::core::default::Default for D2D1_DRAW_TEXT_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_DRAW_TEXT_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_DRAW_TEXT_OPTIONS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D2D1_DRAW_TEXT_OPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D2D1_DRAW_TEXT_OPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D2D1_DRAW_TEXT_OPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D2D1_DRAW_TEXT_OPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D2D1_DRAW_TEXT_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for D2D1_EDGEDETECTION_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_EDGEDETECTION_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_EDGEDETECTION_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_EDGEDETECTION_PROP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_EDGEDETECTION_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_EDGEDETECTION_PROP").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::default::Default for D2D1_EFFECT_INPUT_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::cmp::PartialEq for D2D1_EFFECT_INPUT_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.effect == other.effect && self.inputIndex == other.inputIndex && self.inputRectangle == other.inputRectangle
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::cmp::Eq for D2D1_EFFECT_INPUT_DESCRIPTION {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::fmt::Debug for D2D1_EFFECT_INPUT_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D2D1_EFFECT_INPUT_DESCRIPTION").field("effect", &self.effect).field("inputIndex", &self.inputIndex).field("inputRectangle", &self.inputRectangle).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::default::Default for D2D1_ELLIPSE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::cmp::PartialEq for D2D1_ELLIPSE {
    fn eq(&self, other: &Self) -> bool {
        self.point == other.point && self.radiusX == other.radiusX && self.radiusY == other.radiusY
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::cmp::Eq for D2D1_ELLIPSE {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::fmt::Debug for D2D1_ELLIPSE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D2D1_ELLIPSE").field("point", &self.point).field("radiusX", &self.radiusX).field("radiusY", &self.radiusY).finish()
    }
}
impl ::core::default::Default for D2D1_EMBOSS_PROP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_EMBOSS_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_EMBOSS_PROP").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_EXPOSURE_PROP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_EXPOSURE_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_EXPOSURE_PROP").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_EXTEND_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_EXTEND_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_EXTEND_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_FACTORY_OPTIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D2D1_FACTORY_OPTIONS {
    fn eq(&self, other: &Self) -> bool {
        self.debugLevel == other.debugLevel
    }
}
impl ::core::cmp::Eq for D2D1_FACTORY_OPTIONS {}
impl ::core::fmt::Debug for D2D1_FACTORY_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D2D1_FACTORY_OPTIONS").field("debugLevel", &self.debugLevel).finish()
    }
}
impl ::core::default::Default for D2D1_FACTORY_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_FACTORY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_FACTORY_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_FEATURE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_FEATURE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_FEATURE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D2D1_FEATURE_DATA_D3D10_X_HARDWARE_OPTIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D2D1_FEATURE_DATA_D3D10_X_HARDWARE_OPTIONS {
    fn eq(&self, other: &Self) -> bool {
        self.computeShaders_Plus_RawAndStructuredBuffers_Via_Shader_4_x == other.computeShaders_Plus_RawAndStructuredBuffers_Via_Shader_4_x
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D2D1_FEATURE_DATA_D3D10_X_HARDWARE_OPTIONS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D2D1_FEATURE_DATA_D3D10_X_HARDWARE_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D2D1_FEATURE_DATA_D3D10_X_HARDWARE_OPTIONS").field("computeShaders_Plus_RawAndStructuredBuffers_Via_Shader_4_x", &self.computeShaders_Plus_RawAndStructuredBuffers_Via_Shader_4_x).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D2D1_FEATURE_DATA_DOUBLES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D2D1_FEATURE_DATA_DOUBLES {
    fn eq(&self, other: &Self) -> bool {
        self.doublePrecisionFloatShaderOps == other.doublePrecisionFloatShaderOps
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D2D1_FEATURE_DATA_DOUBLES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D2D1_FEATURE_DATA_DOUBLES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D2D1_FEATURE_DATA_DOUBLES").field("doublePrecisionFloatShaderOps", &self.doublePrecisionFloatShaderOps).finish()
    }
}
impl ::core::default::Default for D2D1_FEATURE_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_FEATURE_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_FEATURE_LEVEL").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_FILTER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_FILTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_FILTER").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_FLOOD_PROP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_FLOOD_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_FLOOD_PROP").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_GAMMA {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_GAMMA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_GAMMA").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_GAMMA1 {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_GAMMA1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_GAMMA1").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_GAMMATRANSFER_PROP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_GAMMATRANSFER_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_GAMMATRANSFER_PROP").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_GAUSSIANBLUR_OPTIMIZATION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_GAUSSIANBLUR_OPTIMIZATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_GAUSSIANBLUR_OPTIMIZATION").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_GAUSSIANBLUR_PROP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_GAUSSIANBLUR_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_GAUSSIANBLUR_PROP").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_GEOMETRY_RELATION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_GEOMETRY_RELATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_GEOMETRY_RELATION").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_GEOMETRY_SIMPLIFICATION_OPTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_GEOMETRY_SIMPLIFICATION_OPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_GEOMETRY_SIMPLIFICATION_OPTION").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::default::Default for D2D1_GRADIENT_MESH_PATCH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::cmp::PartialEq for D2D1_GRADIENT_MESH_PATCH {
    fn eq(&self, other: &Self) -> bool {
        self.point00 == other.point00
            && self.point01 == other.point01
            && self.point02 == other.point02
            && self.point03 == other.point03
            && self.point10 == other.point10
            && self.point11 == other.point11
            && self.point12 == other.point12
            && self.point13 == other.point13
            && self.point20 == other.point20
            && self.point21 == other.point21
            && self.point22 == other.point22
            && self.point23 == other.point23
            && self.point30 == other.point30
            && self.point31 == other.point31
            && self.point32 == other.point32
            && self.point33 == other.point33
            && self.color00 == other.color00
            && self.color03 == other.color03
            && self.color30 == other.color30
            && self.color33 == other.color33
            && self.topEdgeMode == other.topEdgeMode
            && self.leftEdgeMode == other.leftEdgeMode
            && self.bottomEdgeMode == other.bottomEdgeMode
            && self.rightEdgeMode == other.rightEdgeMode
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::cmp::Eq for D2D1_GRADIENT_MESH_PATCH {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::fmt::Debug for D2D1_GRADIENT_MESH_PATCH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D2D1_GRADIENT_MESH_PATCH")
            .field("point00", &self.point00)
            .field("point01", &self.point01)
            .field("point02", &self.point02)
            .field("point03", &self.point03)
            .field("point10", &self.point10)
            .field("point11", &self.point11)
            .field("point12", &self.point12)
            .field("point13", &self.point13)
            .field("point20", &self.point20)
            .field("point21", &self.point21)
            .field("point22", &self.point22)
            .field("point23", &self.point23)
            .field("point30", &self.point30)
            .field("point31", &self.point31)
            .field("point32", &self.point32)
            .field("point33", &self.point33)
            .field("color00", &self.color00)
            .field("color03", &self.color03)
            .field("color30", &self.color30)
            .field("color33", &self.color33)
            .field("topEdgeMode", &self.topEdgeMode)
            .field("leftEdgeMode", &self.leftEdgeMode)
            .field("bottomEdgeMode", &self.bottomEdgeMode)
            .field("rightEdgeMode", &self.rightEdgeMode)
            .finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::default::Default for D2D1_GRADIENT_STOP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::cmp::PartialEq for D2D1_GRADIENT_STOP {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position && self.color == other.color
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::cmp::Eq for D2D1_GRADIENT_STOP {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::fmt::Debug for D2D1_GRADIENT_STOP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D2D1_GRADIENT_STOP").field("position", &self.position).field("color", &self.color).finish()
    }
}
impl ::core::default::Default for D2D1_HDRTONEMAP_DISPLAY_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_HDRTONEMAP_DISPLAY_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_HDRTONEMAP_DISPLAY_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_HDRTONEMAP_PROP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_HDRTONEMAP_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_HDRTONEMAP_PROP").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_HIGHLIGHTSANDSHADOWS_INPUT_GAMMA {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_HIGHLIGHTSANDSHADOWS_INPUT_GAMMA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_HIGHLIGHTSANDSHADOWS_INPUT_GAMMA").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_HIGHLIGHTSANDSHADOWS_PROP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_HIGHLIGHTSANDSHADOWS_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_HIGHLIGHTSANDSHADOWS_PROP").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_HISTOGRAM_PROP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_HISTOGRAM_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_HISTOGRAM_PROP").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_HUEROTATION_PROP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_HUEROTATION_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_HUEROTATION_PROP").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_HUETORGB_INPUT_COLOR_SPACE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_HUETORGB_INPUT_COLOR_SPACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_HUETORGB_INPUT_COLOR_SPACE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_HUETORGB_PROP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_HUETORGB_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_HUETORGB_PROP").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::core::default::Default for D2D1_HWND_RENDER_TARGET_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::core::cmp::PartialEq for D2D1_HWND_RENDER_TARGET_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        self.hwnd == other.hwnd && self.pixelSize == other.pixelSize && self.presentOptions == other.presentOptions
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::core::cmp::Eq for D2D1_HWND_RENDER_TARGET_PROPERTIES {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::core::fmt::Debug for D2D1_HWND_RENDER_TARGET_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D2D1_HWND_RENDER_TARGET_PROPERTIES").field("hwnd", &self.hwnd).field("pixelSize", &self.pixelSize).field("presentOptions", &self.presentOptions).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::default::Default for D2D1_IMAGE_BRUSH_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::cmp::PartialEq for D2D1_IMAGE_BRUSH_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        self.sourceRectangle == other.sourceRectangle && self.extendModeX == other.extendModeX && self.extendModeY == other.extendModeY && self.interpolationMode == other.interpolationMode
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::cmp::Eq for D2D1_IMAGE_BRUSH_PROPERTIES {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::fmt::Debug for D2D1_IMAGE_BRUSH_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D2D1_IMAGE_BRUSH_PROPERTIES").field("sourceRectangle", &self.sourceRectangle).field("extendModeX", &self.extendModeX).field("extendModeY", &self.extendModeY).field("interpolationMode", &self.interpolationMode).finish()
    }
}
impl ::core::default::Default for D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for D2D1_IMAGE_SOURCE_LOADING_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_IMAGE_SOURCE_LOADING_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_IMAGE_SOURCE_LOADING_OPTIONS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D2D1_IMAGE_SOURCE_LOADING_OPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D2D1_IMAGE_SOURCE_LOADING_OPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D2D1_IMAGE_SOURCE_LOADING_OPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D2D1_IMAGE_SOURCE_LOADING_OPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D2D1_IMAGE_SOURCE_LOADING_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for D2D1_INK_BEZIER_SEGMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D2D1_INK_BEZIER_SEGMENT {
    fn eq(&self, other: &Self) -> bool {
        self.point1 == other.point1 && self.point2 == other.point2 && self.point3 == other.point3
    }
}
impl ::core::cmp::Eq for D2D1_INK_BEZIER_SEGMENT {}
impl ::core::fmt::Debug for D2D1_INK_BEZIER_SEGMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D2D1_INK_BEZIER_SEGMENT").field("point1", &self.point1).field("point2", &self.point2).field("point3", &self.point3).finish()
    }
}
impl ::core::default::Default for D2D1_INK_NIB_SHAPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_INK_NIB_SHAPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_INK_NIB_SHAPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_INK_POINT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D2D1_INK_POINT {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.radius == other.radius
    }
}
impl ::core::cmp::Eq for D2D1_INK_POINT {}
impl ::core::fmt::Debug for D2D1_INK_POINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D2D1_INK_POINT").field("x", &self.x).field("y", &self.y).field("radius", &self.radius).finish()
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::default::Default for D2D1_INK_STYLE_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::cmp::PartialEq for D2D1_INK_STYLE_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        self.nibShape == other.nibShape && self.nibTransform == other.nibTransform
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::cmp::Eq for D2D1_INK_STYLE_PROPERTIES {}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::fmt::Debug for D2D1_INK_STYLE_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D2D1_INK_STYLE_PROPERTIES").field("nibShape", &self.nibShape).field("nibTransform", &self.nibTransform).finish()
    }
}
impl ::core::default::Default for D2D1_INPUT_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D2D1_INPUT_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.filter == other.filter && self.levelOfDetailCount == other.levelOfDetailCount
    }
}
impl ::core::cmp::Eq for D2D1_INPUT_DESCRIPTION {}
impl ::core::fmt::Debug for D2D1_INPUT_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D2D1_INPUT_DESCRIPTION").field("filter", &self.filter).field("levelOfDetailCount", &self.levelOfDetailCount).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for D2D1_INPUT_ELEMENT_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::PartialEq for D2D1_INPUT_ELEMENT_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.semanticName == other.semanticName && self.semanticIndex == other.semanticIndex && self.format == other.format && self.inputSlot == other.inputSlot && self.alignedByteOffset == other.alignedByteOffset
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::Eq for D2D1_INPUT_ELEMENT_DESC {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::fmt::Debug for D2D1_INPUT_ELEMENT_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D2D1_INPUT_ELEMENT_DESC").field("semanticName", &self.semanticName).field("semanticIndex", &self.semanticIndex).field("format", &self.format).field("inputSlot", &self.inputSlot).field("alignedByteOffset", &self.alignedByteOffset).finish()
    }
}
impl ::core::default::Default for D2D1_INTERPOLATION_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_INTERPOLATION_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_INTERPOLATION_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_INTERPOLATION_MODE_DEFINITION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_INTERPOLATION_MODE_DEFINITION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_INTERPOLATION_MODE_DEFINITION").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_LAYER_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_LAYER_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_LAYER_OPTIONS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D2D1_LAYER_OPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D2D1_LAYER_OPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D2D1_LAYER_OPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D2D1_LAYER_OPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D2D1_LAYER_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for D2D1_LAYER_OPTIONS1 {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_LAYER_OPTIONS1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_LAYER_OPTIONS1").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D2D1_LAYER_OPTIONS1 {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D2D1_LAYER_OPTIONS1 {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D2D1_LAYER_OPTIONS1 {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D2D1_LAYER_OPTIONS1 {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D2D1_LAYER_OPTIONS1 {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::core::default::Default for D2D1_LAYER_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::core::cmp::PartialEq for D2D1_LAYER_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.contentBounds == other.contentBounds && self.geometricMask == other.geometricMask && self.maskAntialiasMode == other.maskAntialiasMode && self.maskTransform == other.maskTransform && self.opacity == other.opacity && self.opacityBrush == other.opacityBrush && self.layerOptions == other.layerOptions
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::core::cmp::Eq for D2D1_LAYER_PARAMETERS {}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::core::fmt::Debug for D2D1_LAYER_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D2D1_LAYER_PARAMETERS").field("contentBounds", &self.contentBounds).field("geometricMask", &self.geometricMask).field("maskAntialiasMode", &self.maskAntialiasMode).field("maskTransform", &self.maskTransform).field("opacity", &self.opacity).field("opacityBrush", &self.opacityBrush).field("layerOptions", &self.layerOptions).finish()
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::core::default::Default for D2D1_LAYER_PARAMETERS1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::core::cmp::PartialEq for D2D1_LAYER_PARAMETERS1 {
    fn eq(&self, other: &Self) -> bool {
        self.contentBounds == other.contentBounds && self.geometricMask == other.geometricMask && self.maskAntialiasMode == other.maskAntialiasMode && self.maskTransform == other.maskTransform && self.opacity == other.opacity && self.opacityBrush == other.opacityBrush && self.layerOptions == other.layerOptions
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::core::cmp::Eq for D2D1_LAYER_PARAMETERS1 {}
#[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::core::fmt::Debug for D2D1_LAYER_PARAMETERS1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D2D1_LAYER_PARAMETERS1").field("contentBounds", &self.contentBounds).field("geometricMask", &self.geometricMask).field("maskAntialiasMode", &self.maskAntialiasMode).field("maskTransform", &self.maskTransform).field("opacity", &self.opacity).field("opacityBrush", &self.opacityBrush).field("layerOptions", &self.layerOptions).finish()
    }
}
impl ::core::default::Default for D2D1_LINEARTRANSFER_PROP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_LINEARTRANSFER_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_LINEARTRANSFER_PROP").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::default::Default for D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::cmp::PartialEq for D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        self.startPoint == other.startPoint && self.endPoint == other.endPoint
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::cmp::Eq for D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::fmt::Debug for D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES").field("startPoint", &self.startPoint).field("endPoint", &self.endPoint).finish()
    }
}
impl ::core::default::Default for D2D1_LINE_JOIN {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_LINE_JOIN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_LINE_JOIN").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_LOOKUPTABLE3D_PROP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_LOOKUPTABLE3D_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_LOOKUPTABLE3D_PROP").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_MAPPED_RECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D2D1_MAPPED_RECT {
    fn eq(&self, other: &Self) -> bool {
        self.pitch == other.pitch && self.bits == other.bits
    }
}
impl ::core::cmp::Eq for D2D1_MAPPED_RECT {}
impl ::core::fmt::Debug for D2D1_MAPPED_RECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D2D1_MAPPED_RECT").field("pitch", &self.pitch).field("bits", &self.bits).finish()
    }
}
impl ::core::default::Default for D2D1_MAP_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_MAP_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_MAP_OPTIONS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D2D1_MAP_OPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D2D1_MAP_OPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D2D1_MAP_OPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D2D1_MAP_OPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D2D1_MAP_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for D2D1_MORPHOLOGY_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_MORPHOLOGY_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_MORPHOLOGY_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_MORPHOLOGY_PROP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_MORPHOLOGY_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_MORPHOLOGY_PROP").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_OPACITYMETADATA_PROP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_OPACITYMETADATA_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_OPACITYMETADATA_PROP").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_OPACITY_MASK_CONTENT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_OPACITY_MASK_CONTENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_OPACITY_MASK_CONTENT").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_OPACITY_PROP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_OPACITY_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_OPACITY_PROP").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_ORIENTATION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_ORIENTATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_ORIENTATION").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_PATCH_EDGE_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_PATCH_EDGE_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_PATCH_EDGE_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_PIXEL_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_PIXEL_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_PIXEL_OPTIONS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D2D1_PIXEL_OPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D2D1_PIXEL_OPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D2D1_PIXEL_OPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D2D1_PIXEL_OPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D2D1_PIXEL_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for D2D1_POINTDIFFUSE_PROP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_POINTDIFFUSE_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_POINTDIFFUSE_PROP").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_POINTDIFFUSE_SCALE_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_POINTDIFFUSE_SCALE_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_POINTDIFFUSE_SCALE_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_POINTSPECULAR_PROP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_POINTSPECULAR_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_POINTSPECULAR_PROP").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_POINTSPECULAR_SCALE_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_POINTSPECULAR_SCALE_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_POINTSPECULAR_SCALE_MODE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::default::Default for D2D1_POINT_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::cmp::PartialEq for D2D1_POINT_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.point == other.point && self.unitTangentVector == other.unitTangentVector && self.endSegment == other.endSegment && self.endFigure == other.endFigure && self.lengthToEndSegment == other.lengthToEndSegment
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::cmp::Eq for D2D1_POINT_DESCRIPTION {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::fmt::Debug for D2D1_POINT_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D2D1_POINT_DESCRIPTION").field("point", &self.point).field("unitTangentVector", &self.unitTangentVector).field("endSegment", &self.endSegment).field("endFigure", &self.endFigure).field("lengthToEndSegment", &self.lengthToEndSegment).finish()
    }
}
impl ::core::default::Default for D2D1_POSTERIZE_PROP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_POSTERIZE_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_POSTERIZE_PROP").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_PRESENT_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_PRESENT_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_PRESENT_OPTIONS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D2D1_PRESENT_OPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D2D1_PRESENT_OPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D2D1_PRESENT_OPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D2D1_PRESENT_OPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D2D1_PRESENT_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for D2D1_PRIMITIVE_BLEND {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_PRIMITIVE_BLEND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_PRIMITIVE_BLEND").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_PRINT_CONTROL_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D2D1_PRINT_CONTROL_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        self.fontSubset == other.fontSubset && self.rasterDPI == other.rasterDPI && self.colorSpace == other.colorSpace
    }
}
impl ::core::cmp::Eq for D2D1_PRINT_CONTROL_PROPERTIES {}
impl ::core::fmt::Debug for D2D1_PRINT_CONTROL_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D2D1_PRINT_CONTROL_PROPERTIES").field("fontSubset", &self.fontSubset).field("rasterDPI", &self.rasterDPI).field("colorSpace", &self.colorSpace).finish()
    }
}
impl ::core::default::Default for D2D1_PRINT_FONT_SUBSET_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_PRINT_FONT_SUBSET_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_PRINT_FONT_SUBSET_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_PROPERTY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_PROPERTY").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_PROPERTY_BINDING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for D2D1_PROPERTY_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_PROPERTY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_PROPERTY_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::default::Default for D2D1_QUADRATIC_BEZIER_SEGMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::cmp::PartialEq for D2D1_QUADRATIC_BEZIER_SEGMENT {
    fn eq(&self, other: &Self) -> bool {
        self.point1 == other.point1 && self.point2 == other.point2
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::cmp::Eq for D2D1_QUADRATIC_BEZIER_SEGMENT {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::fmt::Debug for D2D1_QUADRATIC_BEZIER_SEGMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D2D1_QUADRATIC_BEZIER_SEGMENT").field("point1", &self.point1).field("point2", &self.point2).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::default::Default for D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::cmp::PartialEq for D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        self.center == other.center && self.gradientOriginOffset == other.gradientOriginOffset && self.radiusX == other.radiusX && self.radiusY == other.radiusY
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::cmp::Eq for D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::fmt::Debug for D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES").field("center", &self.center).field("gradientOriginOffset", &self.gradientOriginOffset).field("radiusX", &self.radiusX).field("radiusY", &self.radiusY).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::default::Default for D2D1_RENDERING_CONTROLS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::cmp::PartialEq for D2D1_RENDERING_CONTROLS {
    fn eq(&self, other: &Self) -> bool {
        self.bufferPrecision == other.bufferPrecision && self.tileSize == other.tileSize
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::cmp::Eq for D2D1_RENDERING_CONTROLS {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::fmt::Debug for D2D1_RENDERING_CONTROLS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D2D1_RENDERING_CONTROLS").field("bufferPrecision", &self.bufferPrecision).field("tileSize", &self.tileSize).finish()
    }
}
impl ::core::default::Default for D2D1_RENDERING_PRIORITY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_RENDERING_PRIORITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_RENDERING_PRIORITY").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::default::Default for D2D1_RENDER_TARGET_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::cmp::PartialEq for D2D1_RENDER_TARGET_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        self.r#type == other.r#type && self.pixelFormat == other.pixelFormat && self.dpiX == other.dpiX && self.dpiY == other.dpiY && self.usage == other.usage && self.minLevel == other.minLevel
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::cmp::Eq for D2D1_RENDER_TARGET_PROPERTIES {}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::fmt::Debug for D2D1_RENDER_TARGET_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D2D1_RENDER_TARGET_PROPERTIES").field("type", &self.r#type).field("pixelFormat", &self.pixelFormat).field("dpiX", &self.dpiX).field("dpiY", &self.dpiY).field("usage", &self.usage).field("minLevel", &self.minLevel).finish()
    }
}
impl ::core::default::Default for D2D1_RENDER_TARGET_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_RENDER_TARGET_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_RENDER_TARGET_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_RENDER_TARGET_USAGE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_RENDER_TARGET_USAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_RENDER_TARGET_USAGE").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D2D1_RENDER_TARGET_USAGE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D2D1_RENDER_TARGET_USAGE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D2D1_RENDER_TARGET_USAGE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D2D1_RENDER_TARGET_USAGE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D2D1_RENDER_TARGET_USAGE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for D2D1_RESOURCE_TEXTURE_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D2D1_RESOURCE_TEXTURE_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        self.extents == other.extents && self.dimensions == other.dimensions && self.bufferPrecision == other.bufferPrecision && self.channelDepth == other.channelDepth && self.filter == other.filter && self.extendModes == other.extendModes
    }
}
impl ::core::cmp::Eq for D2D1_RESOURCE_TEXTURE_PROPERTIES {}
impl ::core::fmt::Debug for D2D1_RESOURCE_TEXTURE_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D2D1_RESOURCE_TEXTURE_PROPERTIES").field("extents", &self.extents).field("dimensions", &self.dimensions).field("bufferPrecision", &self.bufferPrecision).field("channelDepth", &self.channelDepth).field("filter", &self.filter).field("extendModes", &self.extendModes).finish()
    }
}
impl ::core::default::Default for D2D1_RGBTOHUE_OUTPUT_COLOR_SPACE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_RGBTOHUE_OUTPUT_COLOR_SPACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_RGBTOHUE_OUTPUT_COLOR_SPACE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_RGBTOHUE_PROP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_RGBTOHUE_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_RGBTOHUE_PROP").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::default::Default for D2D1_ROUNDED_RECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::cmp::PartialEq for D2D1_ROUNDED_RECT {
    fn eq(&self, other: &Self) -> bool {
        self.rect == other.rect && self.radiusX == other.radiusX && self.radiusY == other.radiusY
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::cmp::Eq for D2D1_ROUNDED_RECT {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::fmt::Debug for D2D1_ROUNDED_RECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D2D1_ROUNDED_RECT").field("rect", &self.rect).field("radiusX", &self.radiusX).field("radiusY", &self.radiusY).finish()
    }
}
impl ::core::default::Default for D2D1_SATURATION_PROP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_SATURATION_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_SATURATION_PROP").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_SCALE_INTERPOLATION_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_SCALE_INTERPOLATION_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_SCALE_INTERPOLATION_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_SCALE_PROP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_SCALE_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_SCALE_PROP").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_SEPIA_PROP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_SEPIA_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_SEPIA_PROP").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_SHADOW_OPTIMIZATION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_SHADOW_OPTIMIZATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_SHADOW_OPTIMIZATION").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_SHADOW_PROP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_SHADOW_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_SHADOW_PROP").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_SHARPEN_PROP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_SHARPEN_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_SHARPEN_PROP").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::default::Default for D2D1_SIMPLE_COLOR_PROFILE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::cmp::PartialEq for D2D1_SIMPLE_COLOR_PROFILE {
    fn eq(&self, other: &Self) -> bool {
        self.redPrimary == other.redPrimary && self.greenPrimary == other.greenPrimary && self.bluePrimary == other.bluePrimary && self.whitePointXZ == other.whitePointXZ && self.gamma == other.gamma
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::cmp::Eq for D2D1_SIMPLE_COLOR_PROFILE {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::fmt::Debug for D2D1_SIMPLE_COLOR_PROFILE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D2D1_SIMPLE_COLOR_PROFILE").field("redPrimary", &self.redPrimary).field("greenPrimary", &self.greenPrimary).field("bluePrimary", &self.bluePrimary).field("whitePointXZ", &self.whitePointXZ).field("gamma", &self.gamma).finish()
    }
}
impl ::core::default::Default for D2D1_SPOTDIFFUSE_PROP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_SPOTDIFFUSE_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_SPOTDIFFUSE_PROP").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_SPOTDIFFUSE_SCALE_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_SPOTDIFFUSE_SCALE_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_SPOTDIFFUSE_SCALE_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_SPOTSPECULAR_PROP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_SPOTSPECULAR_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_SPOTSPECULAR_PROP").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_SPOTSPECULAR_SCALE_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_SPOTSPECULAR_SCALE_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_SPOTSPECULAR_SCALE_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_SPRITE_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_SPRITE_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_SPRITE_OPTIONS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D2D1_SPRITE_OPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D2D1_SPRITE_OPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D2D1_SPRITE_OPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D2D1_SPRITE_OPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D2D1_SPRITE_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for D2D1_STRAIGHTEN_PROP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_STRAIGHTEN_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_STRAIGHTEN_PROP").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_STRAIGHTEN_SCALE_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_STRAIGHTEN_SCALE_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_STRAIGHTEN_SCALE_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_STROKE_STYLE_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D2D1_STROKE_STYLE_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        self.startCap == other.startCap && self.endCap == other.endCap && self.dashCap == other.dashCap && self.lineJoin == other.lineJoin && self.miterLimit == other.miterLimit && self.dashStyle == other.dashStyle && self.dashOffset == other.dashOffset
    }
}
impl ::core::cmp::Eq for D2D1_STROKE_STYLE_PROPERTIES {}
impl ::core::fmt::Debug for D2D1_STROKE_STYLE_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D2D1_STROKE_STYLE_PROPERTIES").field("startCap", &self.startCap).field("endCap", &self.endCap).field("dashCap", &self.dashCap).field("lineJoin", &self.lineJoin).field("miterLimit", &self.miterLimit).field("dashStyle", &self.dashStyle).field("dashOffset", &self.dashOffset).finish()
    }
}
impl ::core::default::Default for D2D1_STROKE_STYLE_PROPERTIES1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D2D1_STROKE_STYLE_PROPERTIES1 {
    fn eq(&self, other: &Self) -> bool {
        self.startCap == other.startCap && self.endCap == other.endCap && self.dashCap == other.dashCap && self.lineJoin == other.lineJoin && self.miterLimit == other.miterLimit && self.dashStyle == other.dashStyle && self.dashOffset == other.dashOffset && self.transformType == other.transformType
    }
}
impl ::core::cmp::Eq for D2D1_STROKE_STYLE_PROPERTIES1 {}
impl ::core::fmt::Debug for D2D1_STROKE_STYLE_PROPERTIES1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D2D1_STROKE_STYLE_PROPERTIES1").field("startCap", &self.startCap).field("endCap", &self.endCap).field("dashCap", &self.dashCap).field("lineJoin", &self.lineJoin).field("miterLimit", &self.miterLimit).field("dashStyle", &self.dashStyle).field("dashOffset", &self.dashOffset).field("transformType", &self.transformType).finish()
    }
}
impl ::core::default::Default for D2D1_STROKE_TRANSFORM_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_STROKE_TRANSFORM_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_STROKE_TRANSFORM_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_SUBPROPERTY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_SUBPROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_SUBPROPERTY").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_SVG_ASPECT_ALIGN {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_SVG_ASPECT_ALIGN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_SVG_ASPECT_ALIGN").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_SVG_ASPECT_SCALING {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_SVG_ASPECT_SCALING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_SVG_ASPECT_SCALING").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_SVG_ATTRIBUTE_POD_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_SVG_ATTRIBUTE_POD_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_SVG_ATTRIBUTE_POD_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_SVG_ATTRIBUTE_STRING_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_SVG_ATTRIBUTE_STRING_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_SVG_ATTRIBUTE_STRING_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_SVG_DISPLAY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_SVG_DISPLAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_SVG_DISPLAY").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_SVG_LENGTH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D2D1_SVG_LENGTH {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.units == other.units
    }
}
impl ::core::cmp::Eq for D2D1_SVG_LENGTH {}
impl ::core::fmt::Debug for D2D1_SVG_LENGTH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D2D1_SVG_LENGTH").field("value", &self.value).field("units", &self.units).finish()
    }
}
impl ::core::default::Default for D2D1_SVG_LENGTH_UNITS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_SVG_LENGTH_UNITS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_SVG_LENGTH_UNITS").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_SVG_LINE_CAP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_SVG_LINE_CAP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_SVG_LINE_CAP").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_SVG_LINE_JOIN {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_SVG_LINE_JOIN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_SVG_LINE_JOIN").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_SVG_OVERFLOW {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_SVG_OVERFLOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_SVG_OVERFLOW").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_SVG_PAINT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_SVG_PAINT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_SVG_PAINT_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_SVG_PATH_COMMAND {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_SVG_PATH_COMMAND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_SVG_PATH_COMMAND").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D2D1_SVG_PRESERVE_ASPECT_RATIO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D2D1_SVG_PRESERVE_ASPECT_RATIO {
    fn eq(&self, other: &Self) -> bool {
        self.defer == other.defer && self.align == other.align && self.meetOrSlice == other.meetOrSlice
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D2D1_SVG_PRESERVE_ASPECT_RATIO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D2D1_SVG_PRESERVE_ASPECT_RATIO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D2D1_SVG_PRESERVE_ASPECT_RATIO").field("defer", &self.defer).field("align", &self.align).field("meetOrSlice", &self.meetOrSlice).finish()
    }
}
impl ::core::default::Default for D2D1_SVG_UNIT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_SVG_UNIT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_SVG_UNIT_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_SVG_VIEWBOX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D2D1_SVG_VIEWBOX {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.width == other.width && self.height == other.height
    }
}
impl ::core::cmp::Eq for D2D1_SVG_VIEWBOX {}
impl ::core::fmt::Debug for D2D1_SVG_VIEWBOX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D2D1_SVG_VIEWBOX").field("x", &self.x).field("y", &self.y).field("width", &self.width).field("height", &self.height).finish()
    }
}
impl ::core::default::Default for D2D1_SVG_VISIBILITY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_SVG_VISIBILITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_SVG_VISIBILITY").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_SWEEP_DIRECTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_SWEEP_DIRECTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_SWEEP_DIRECTION").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_TABLETRANSFER_PROP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_TABLETRANSFER_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_TABLETRANSFER_PROP").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_TEMPERATUREANDTINT_PROP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_TEMPERATUREANDTINT_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_TEMPERATUREANDTINT_PROP").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_TEXT_ANTIALIAS_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_TEXT_ANTIALIAS_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_TEXT_ANTIALIAS_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_THREADING_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_THREADING_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_THREADING_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_TILE_PROP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_TILE_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_TILE_PROP").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_TINT_PROP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_TINT_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_TINT_PROP").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_TRANSFORMED_IMAGE_SOURCE_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_TRANSFORMED_IMAGE_SOURCE_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_TRANSFORMED_IMAGE_SOURCE_OPTIONS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D2D1_TRANSFORMED_IMAGE_SOURCE_OPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D2D1_TRANSFORMED_IMAGE_SOURCE_OPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D2D1_TRANSFORMED_IMAGE_SOURCE_OPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D2D1_TRANSFORMED_IMAGE_SOURCE_OPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D2D1_TRANSFORMED_IMAGE_SOURCE_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for D2D1_TRANSFORMED_IMAGE_SOURCE_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D2D1_TRANSFORMED_IMAGE_SOURCE_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        self.orientation == other.orientation && self.scaleX == other.scaleX && self.scaleY == other.scaleY && self.interpolationMode == other.interpolationMode && self.options == other.options
    }
}
impl ::core::cmp::Eq for D2D1_TRANSFORMED_IMAGE_SOURCE_PROPERTIES {}
impl ::core::fmt::Debug for D2D1_TRANSFORMED_IMAGE_SOURCE_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D2D1_TRANSFORMED_IMAGE_SOURCE_PROPERTIES").field("orientation", &self.orientation).field("scaleX", &self.scaleX).field("scaleY", &self.scaleY).field("interpolationMode", &self.interpolationMode).field("options", &self.options).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::default::Default for D2D1_TRIANGLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::cmp::PartialEq for D2D1_TRIANGLE {
    fn eq(&self, other: &Self) -> bool {
        self.point1 == other.point1 && self.point2 == other.point2 && self.point3 == other.point3
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::cmp::Eq for D2D1_TRIANGLE {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::fmt::Debug for D2D1_TRIANGLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D2D1_TRIANGLE").field("point1", &self.point1).field("point2", &self.point2).field("point3", &self.point3).finish()
    }
}
impl ::core::default::Default for D2D1_TURBULENCE_PROP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_TURBULENCE_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_TURBULENCE_PROP").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_UNIT_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_UNIT_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_UNIT_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_VERTEX_BUFFER_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D2D1_VERTEX_BUFFER_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        self.inputCount == other.inputCount && self.usage == other.usage && self.data == other.data && self.byteWidth == other.byteWidth
    }
}
impl ::core::cmp::Eq for D2D1_VERTEX_BUFFER_PROPERTIES {}
impl ::core::fmt::Debug for D2D1_VERTEX_BUFFER_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D2D1_VERTEX_BUFFER_PROPERTIES").field("inputCount", &self.inputCount).field("usage", &self.usage).field("data", &self.data).field("byteWidth", &self.byteWidth).finish()
    }
}
impl ::core::default::Default for D2D1_VERTEX_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_VERTEX_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_VERTEX_OPTIONS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D2D1_VERTEX_OPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D2D1_VERTEX_OPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D2D1_VERTEX_OPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D2D1_VERTEX_OPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D2D1_VERTEX_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for D2D1_VERTEX_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D2D1_VERTEX_RANGE {
    fn eq(&self, other: &Self) -> bool {
        self.startVertex == other.startVertex && self.vertexCount == other.vertexCount
    }
}
impl ::core::cmp::Eq for D2D1_VERTEX_RANGE {}
impl ::core::fmt::Debug for D2D1_VERTEX_RANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D2D1_VERTEX_RANGE").field("startVertex", &self.startVertex).field("vertexCount", &self.vertexCount).finish()
    }
}
impl ::core::default::Default for D2D1_VERTEX_USAGE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_VERTEX_USAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_VERTEX_USAGE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_VIGNETTE_PROP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_VIGNETTE_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_VIGNETTE_PROP").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_WHITELEVELADJUSTMENT_PROP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_WHITELEVELADJUSTMENT_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_WHITELEVELADJUSTMENT_PROP").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_WINDOW_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_WINDOW_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_WINDOW_STATE").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D2D1_WINDOW_STATE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D2D1_WINDOW_STATE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D2D1_WINDOW_STATE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D2D1_WINDOW_STATE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D2D1_WINDOW_STATE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for D2D1_YCBCR_CHROMA_SUBSAMPLING {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_YCBCR_CHROMA_SUBSAMPLING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_YCBCR_CHROMA_SUBSAMPLING").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_YCBCR_INTERPOLATION_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_YCBCR_INTERPOLATION_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_YCBCR_INTERPOLATION_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_YCBCR_PROP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_YCBCR_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_YCBCR_PROP").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID2D1AnalysisTransform {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1AnalysisTransform {}
impl ::core::fmt::Debug for ID2D1AnalysisTransform {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1AnalysisTransform").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID2D1Bitmap {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1Bitmap {}
impl ::core::fmt::Debug for ID2D1Bitmap {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1Bitmap").field(&self.0).finish()
    }
}
impl ID2D1Bitmap {
    pub unsafe fn GetFactory(&self) -> ::windows::core::Result<ID2D1Factory> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetFactory)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1Factory as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
}
impl ::core::cmp::PartialEq for ID2D1Bitmap1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1Bitmap1 {}
impl ::core::fmt::Debug for ID2D1Bitmap1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1Bitmap1").field(&self.0).finish()
    }
}
impl ID2D1Bitmap1 {
    pub unsafe fn GetFactory(&self) -> ::windows::core::Result<ID2D1Factory> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetFactory)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1Factory as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetSize(&self) -> Common::D2D_SIZE_F {
        let mut result__: Common::D2D_SIZE_F = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetSize)(::windows::core::Vtable::as_raw(self), &mut result__);
        result__
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetPixelSize(&self) -> Common::D2D_SIZE_U {
        let mut result__: Common::D2D_SIZE_U = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetPixelSize)(::windows::core::Vtable::as_raw(self), &mut result__);
        result__
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetPixelFormat(&self) -> Common::D2D1_PIXEL_FORMAT {
        let mut result__: Common::D2D1_PIXEL_FORMAT = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetPixelFormat)(::windows::core::Vtable::as_raw(self), &mut result__);
        result__
    }
    pub unsafe fn GetDpi(&self, dpix: *mut f32, dpiy: *mut f32) {
        (::windows::core::Vtable::vtable(self).base__.GetDpi)(::windows::core::Vtable::as_raw(self), dpix, dpiy)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CopyFromBitmap<P0>(&self, destpoint: ::core::option::Option<*const Common::D2D_POINT_2U>, bitmap: P0, srcrect: ::core::option::Option<*const Common::D2D_RECT_U>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Bitmap>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CopyFromBitmap)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(destpoint.unwrap_or(::std::ptr::null())), bitmap.into().abi(), ::core::mem::transmute(srcrect.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CopyFromRenderTarget<P0>(&self, destpoint: ::core::option::Option<*const Common::D2D_POINT_2U>, rendertarget: P0, srcrect: ::core::option::Option<*const Common::D2D_RECT_U>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1RenderTarget>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CopyFromRenderTarget)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(destpoint.unwrap_or(::std::ptr::null())), rendertarget.into().abi(), ::core::mem::transmute(srcrect.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CopyFromMemory(&self, dstrect: ::core::option::Option<*const Common::D2D_RECT_U>, srcdata: *const ::core::ffi::c_void, pitch: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CopyFromMemory)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(dstrect.unwrap_or(::std::ptr::null())), srcdata, pitch).ok()
    }
}
impl ::core::cmp::PartialEq for ID2D1BitmapBrush {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1BitmapBrush {}
impl ::core::fmt::Debug for ID2D1BitmapBrush {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1BitmapBrush").field(&self.0).finish()
    }
}
impl ID2D1BitmapBrush {
    pub unsafe fn GetFactory(&self) -> ::windows::core::Result<ID2D1Factory> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetFactory)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1Factory as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn SetOpacity(&self, opacity: f32) {
        (::windows::core::Vtable::vtable(self).base__.SetOpacity)(::windows::core::Vtable::as_raw(self), opacity)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn SetTransform(&self, transform: *const super::super::super::Foundation::Numerics::Matrix3x2) {
        (::windows::core::Vtable::vtable(self).base__.SetTransform)(::windows::core::Vtable::as_raw(self), transform)
    }
    pub unsafe fn GetOpacity(&self) -> f32 {
        (::windows::core::Vtable::vtable(self).base__.GetOpacity)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn GetTransform(&self, transform: *mut super::super::super::Foundation::Numerics::Matrix3x2) {
        (::windows::core::Vtable::vtable(self).base__.GetTransform)(::windows::core::Vtable::as_raw(self), transform)
    }
}
impl ::core::cmp::PartialEq for ID2D1BitmapBrush1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1BitmapBrush1 {}
impl ::core::fmt::Debug for ID2D1BitmapBrush1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1BitmapBrush1").field(&self.0).finish()
    }
}
impl ID2D1BitmapBrush1 {
    pub unsafe fn GetFactory(&self) -> ::windows::core::Result<ID2D1Factory> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetFactory)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1Factory as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn SetOpacity(&self, opacity: f32) {
        (::windows::core::Vtable::vtable(self).base__.base__.SetOpacity)(::windows::core::Vtable::as_raw(self), opacity)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn SetTransform(&self, transform: *const super::super::super::Foundation::Numerics::Matrix3x2) {
        (::windows::core::Vtable::vtable(self).base__.base__.SetTransform)(::windows::core::Vtable::as_raw(self), transform)
    }
    pub unsafe fn GetOpacity(&self) -> f32 {
        (::windows::core::Vtable::vtable(self).base__.base__.GetOpacity)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn GetTransform(&self, transform: *mut super::super::super::Foundation::Numerics::Matrix3x2) {
        (::windows::core::Vtable::vtable(self).base__.base__.GetTransform)(::windows::core::Vtable::as_raw(self), transform)
    }
    pub unsafe fn SetExtendModeX(&self, extendmodex: D2D1_EXTEND_MODE) {
        (::windows::core::Vtable::vtable(self).base__.SetExtendModeX)(::windows::core::Vtable::as_raw(self), extendmodex)
    }
    pub unsafe fn SetExtendModeY(&self, extendmodey: D2D1_EXTEND_MODE) {
        (::windows::core::Vtable::vtable(self).base__.SetExtendModeY)(::windows::core::Vtable::as_raw(self), extendmodey)
    }
    pub unsafe fn SetInterpolationMode(&self, interpolationmode: D2D1_BITMAP_INTERPOLATION_MODE) {
        (::windows::core::Vtable::vtable(self).base__.SetInterpolationMode)(::windows::core::Vtable::as_raw(self), interpolationmode)
    }
    pub unsafe fn SetBitmap<P0>(&self, bitmap: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Bitmap>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetBitmap)(::windows::core::Vtable::as_raw(self), bitmap.into().abi())
    }
    pub unsafe fn GetExtendModeX(&self) -> D2D1_EXTEND_MODE {
        (::windows::core::Vtable::vtable(self).base__.GetExtendModeX)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetExtendModeY(&self) -> D2D1_EXTEND_MODE {
        (::windows::core::Vtable::vtable(self).base__.GetExtendModeY)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetInterpolationMode(&self) -> D2D1_BITMAP_INTERPOLATION_MODE {
        (::windows::core::Vtable::vtable(self).base__.GetInterpolationMode)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetBitmap(&self) -> ::windows::core::Result<ID2D1Bitmap> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetBitmap)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1Bitmap as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
}
impl ::core::cmp::PartialEq for ID2D1BitmapRenderTarget {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1BitmapRenderTarget {}
impl ::core::fmt::Debug for ID2D1BitmapRenderTarget {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1BitmapRenderTarget").field(&self.0).finish()
    }
}
impl ID2D1BitmapRenderTarget {
    pub unsafe fn GetFactory(&self) -> ::windows::core::Result<ID2D1Factory> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetFactory)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1Factory as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateBitmap(&self, size: Common::D2D_SIZE_U, srcdata: ::core::option::Option<*const ::core::ffi::c_void>, pitch: u32, bitmapproperties: *const D2D1_BITMAP_PROPERTIES) -> ::windows::core::Result<ID2D1Bitmap> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateBitmap)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(size), ::core::mem::transmute(srcdata.unwrap_or(::std::ptr::null())), pitch, bitmapproperties, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Imaging\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
    pub unsafe fn CreateBitmapFromWicBitmap<P0>(&self, wicbitmapsource: P0, bitmapproperties: ::core::option::Option<*const D2D1_BITMAP_PROPERTIES>) -> ::windows::core::Result<ID2D1Bitmap>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Imaging::IWICBitmapSource>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateBitmapFromWicBitmap)(::windows::core::Vtable::as_raw(self), wicbitmapsource.into().abi(), ::core::mem::transmute(bitmapproperties.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSharedBitmap(&self, riid: *const ::windows::core::GUID, data: *mut ::core::ffi::c_void, bitmapproperties: ::core::option::Option<*const D2D1_BITMAP_PROPERTIES>, bitmap: *mut ::core::option::Option<ID2D1Bitmap>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CreateSharedBitmap)(::windows::core::Vtable::as_raw(self), riid, data, ::core::mem::transmute(bitmapproperties.unwrap_or(::std::ptr::null())), ::core::mem::transmute(bitmap)).ok()
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn CreateBitmapBrush<P0>(&self, bitmap: P0, bitmapbrushproperties: ::core::option::Option<*const D2D1_BITMAP_BRUSH_PROPERTIES>, brushproperties: ::core::option::Option<*const D2D1_BRUSH_PROPERTIES>) -> ::windows::core::Result<ID2D1BitmapBrush>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Bitmap>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateBitmapBrush)(::windows::core::Vtable::as_raw(self), bitmap.into().abi(), ::core::mem::transmute(bitmapbrushproperties.unwrap_or(::std::ptr::null())), ::core::mem::transmute(brushproperties.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn CreateSolidColorBrush(&self, color: *const Common::D2D1_COLOR_F, brushproperties: ::core::option::Option<*const D2D1_BRUSH_PROPERTIES>) -> ::windows::core::Result<ID2D1SolidColorBrush> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateSolidColorBrush)(::windows::core::Vtable::as_raw(self), color, ::core::mem::transmute(brushproperties.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CreateGradientStopCollection(&self, gradientstops: &[D2D1_GRADIENT_STOP], colorinterpolationgamma: D2D1_GAMMA, extendmode: D2D1_EXTEND_MODE) -> ::windows::core::Result<ID2D1GradientStopCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateGradientStopCollection)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(gradientstops.as_ptr()), gradientstops.len() as _, colorinterpolationgamma, extendmode, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn CreateLinearGradientBrush<P0>(&self, lineargradientbrushproperties: *const D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES, brushproperties: ::core::option::Option<*const D2D1_BRUSH_PROPERTIES>, gradientstopcollection: P0) -> ::windows::core::Result<ID2D1LinearGradientBrush>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1GradientStopCollection>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateLinearGradientBrush)(::windows::core::Vtable::as_raw(self), lineargradientbrushproperties, ::core::mem::transmute(brushproperties.unwrap_or(::std::ptr::null())), gradientstopcollection.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn CreateRadialGradientBrush<P0>(&self, radialgradientbrushproperties: *const D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES, brushproperties: ::core::option::Option<*const D2D1_BRUSH_PROPERTIES>, gradientstopcollection: P0) -> ::windows::core::Result<ID2D1RadialGradientBrush>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1GradientStopCollection>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateRadialGradientBrush)(::windows::core::Vtable::as_raw(self), radialgradientbrushproperties, ::core::mem::transmute(brushproperties.unwrap_or(::std::ptr::null())), gradientstopcollection.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateCompatibleRenderTarget(&self, desiredsize: ::core::option::Option<*const Common::D2D_SIZE_F>, desiredpixelsize: ::core::option::Option<*const Common::D2D_SIZE_U>, desiredformat: ::core::option::Option<*const Common::D2D1_PIXEL_FORMAT>, options: D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS) -> ::windows::core::Result<ID2D1BitmapRenderTarget> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateCompatibleRenderTarget)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(desiredsize.unwrap_or(::std::ptr::null())), ::core::mem::transmute(desiredpixelsize.unwrap_or(::std::ptr::null())), ::core::mem::transmute(desiredformat.unwrap_or(::std::ptr::null())), options, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CreateLayer(&self, size: ::core::option::Option<*const Common::D2D_SIZE_F>) -> ::windows::core::Result<ID2D1Layer> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateLayer)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(size.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateMesh(&self) -> ::windows::core::Result<ID2D1Mesh> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateMesh)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawLine<P0, P1>(&self, point0: Common::D2D_POINT_2F, point1: Common::D2D_POINT_2F, brush: P0, strokewidth: f32, strokestyle: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DrawLine)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(point0), ::core::mem::transmute(point1), brush.into().abi(), strokewidth, strokestyle.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawRectangle<P0, P1>(&self, rect: *const Common::D2D_RECT_F, brush: P0, strokewidth: f32, strokestyle: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DrawRectangle)(::windows::core::Vtable::as_raw(self), rect, brush.into().abi(), strokewidth, strokestyle.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn FillRectangle<P0>(&self, rect: *const Common::D2D_RECT_F, brush: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.FillRectangle)(::windows::core::Vtable::as_raw(self), rect, brush.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawRoundedRectangle<P0, P1>(&self, roundedrect: *const D2D1_ROUNDED_RECT, brush: P0, strokewidth: f32, strokestyle: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DrawRoundedRectangle)(::windows::core::Vtable::as_raw(self), roundedrect, brush.into().abi(), strokewidth, strokestyle.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn FillRoundedRectangle<P0>(&self, roundedrect: *const D2D1_ROUNDED_RECT, brush: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.FillRoundedRectangle)(::windows::core::Vtable::as_raw(self), roundedrect, brush.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawEllipse<P0, P1>(&self, ellipse: *const D2D1_ELLIPSE, brush: P0, strokewidth: f32, strokestyle: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DrawEllipse)(::windows::core::Vtable::as_raw(self), ellipse, brush.into().abi(), strokewidth, strokestyle.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn FillEllipse<P0>(&self, ellipse: *const D2D1_ELLIPSE, brush: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.FillEllipse)(::windows::core::Vtable::as_raw(self), ellipse, brush.into().abi())
    }
    pub unsafe fn DrawGeometry<P0, P1, P2>(&self, geometry: P0, brush: P1, strokewidth: f32, strokestyle: P2)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Geometry>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P2: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DrawGeometry)(::windows::core::Vtable::as_raw(self), geometry.into().abi(), brush.into().abi(), strokewidth, strokestyle.into().abi())
    }
    pub unsafe fn FillGeometry<P0, P1, P2>(&self, geometry: P0, brush: P1, opacitybrush: P2)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Geometry>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P2: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.FillGeometry)(::windows::core::Vtable::as_raw(self), geometry.into().abi(), brush.into().abi(), opacitybrush.into().abi())
    }
    pub unsafe fn FillMesh<P0, P1>(&self, mesh: P0, brush: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Mesh>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.FillMesh)(::windows::core::Vtable::as_raw(self), mesh.into().abi(), brush.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn FillOpacityMask<P0, P1>(&self, opacitymask: P0, brush: P1, content: D2D1_OPACITY_MASK_CONTENT, destinationrectangle: ::core::option::Option<*const Common::D2D_RECT_F>, sourcerectangle: ::core::option::Option<*const Common::D2D_RECT_F>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Bitmap>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.FillOpacityMask)(::windows::core::Vtable::as_raw(self), opacitymask.into().abi(), brush.into().abi(), content, ::core::mem::transmute(destinationrectangle.unwrap_or(::std::ptr::null())), ::core::mem::transmute(sourcerectangle.unwrap_or(::std::ptr::null())))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawBitmap<P0>(&self, bitmap: P0, destinationrectangle: ::core::option::Option<*const Common::D2D_RECT_F>, opacity: f32, interpolationmode: D2D1_BITMAP_INTERPOLATION_MODE, sourcerectangle: ::core::option::Option<*const Common::D2D_RECT_F>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Bitmap>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DrawBitmap)(::windows::core::Vtable::as_raw(self), bitmap.into().abi(), ::core::mem::transmute(destinationrectangle.unwrap_or(::std::ptr::null())), opacity, interpolationmode, ::core::mem::transmute(sourcerectangle.unwrap_or(::std::ptr::null())))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn DrawText<P0, P1>(&self, string: &[u16], textformat: P0, layoutrect: *const Common::D2D_RECT_F, defaultfillbrush: P1, options: D2D1_DRAW_TEXT_OPTIONS, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::DirectWrite::IDWriteTextFormat>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DrawText)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(string.as_ptr()), string.len() as _, textformat.into().abi(), layoutrect, defaultfillbrush.into().abi(), options, measuringmode)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn DrawTextLayout<P0, P1>(&self, origin: Common::D2D_POINT_2F, textlayout: P0, defaultfillbrush: P1, options: D2D1_DRAW_TEXT_OPTIONS)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::DirectWrite::IDWriteTextLayout>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DrawTextLayout)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(origin), textlayout.into().abi(), defaultfillbrush.into().abi(), options)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn DrawGlyphRun<P0>(&self, baselineorigin: Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, foregroundbrush: P0, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DrawGlyphRun)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(baselineorigin), glyphrun, foregroundbrush.into().abi(), measuringmode)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn SetTransform(&self, transform: *const super::super::super::Foundation::Numerics::Matrix3x2) {
        (::windows::core::Vtable::vtable(self).base__.SetTransform)(::windows::core::Vtable::as_raw(self), transform)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn GetTransform(&self, transform: *mut super::super::super::Foundation::Numerics::Matrix3x2) {
        (::windows::core::Vtable::vtable(self).base__.GetTransform)(::windows::core::Vtable::as_raw(self), transform)
    }
    pub unsafe fn SetAntialiasMode(&self, antialiasmode: D2D1_ANTIALIAS_MODE) {
        (::windows::core::Vtable::vtable(self).base__.SetAntialiasMode)(::windows::core::Vtable::as_raw(self), antialiasmode)
    }
    pub unsafe fn GetAntialiasMode(&self) -> D2D1_ANTIALIAS_MODE {
        (::windows::core::Vtable::vtable(self).base__.GetAntialiasMode)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn SetTextAntialiasMode(&self, textantialiasmode: D2D1_TEXT_ANTIALIAS_MODE) {
        (::windows::core::Vtable::vtable(self).base__.SetTextAntialiasMode)(::windows::core::Vtable::as_raw(self), textantialiasmode)
    }
    pub unsafe fn GetTextAntialiasMode(&self) -> D2D1_TEXT_ANTIALIAS_MODE {
        (::windows::core::Vtable::vtable(self).base__.GetTextAntialiasMode)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(feature = "Win32_Graphics_DirectWrite")]
    pub unsafe fn SetTextRenderingParams<P0>(&self, textrenderingparams: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::DirectWrite::IDWriteRenderingParams>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetTextRenderingParams)(::windows::core::Vtable::as_raw(self), textrenderingparams.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(feature = "Win32_Graphics_DirectWrite")]
    pub unsafe fn GetTextRenderingParams(&self) -> ::windows::core::Result<super::DirectWrite::IDWriteRenderingParams> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetTextRenderingParams)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <super::DirectWrite::IDWriteRenderingParams as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn SetTags(&self, tag1: u64, tag2: u64) {
        (::windows::core::Vtable::vtable(self).base__.SetTags)(::windows::core::Vtable::as_raw(self), tag1, tag2)
    }
    pub unsafe fn GetTags(&self, tag1: ::core::option::Option<*mut u64>, tag2: ::core::option::Option<*mut u64>) {
        (::windows::core::Vtable::vtable(self).base__.GetTags)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(tag1.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(tag2.unwrap_or(::std::ptr::null_mut())))
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn PushLayer<P0>(&self, layerparameters: *const D2D1_LAYER_PARAMETERS, layer: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Layer>>,
    {
        (::windows::core::Vtable::vtable(self).base__.PushLayer)(::windows::core::Vtable::as_raw(self), layerparameters, layer.into().abi())
    }
    pub unsafe fn PopLayer(&self) {
        (::windows::core::Vtable::vtable(self).base__.PopLayer)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn Flush(&self, tag1: ::core::option::Option<*mut u64>, tag2: ::core::option::Option<*mut u64>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Flush)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(tag1.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(tag2.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SaveDrawingState<P0>(&self, drawingstateblock: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1DrawingStateBlock>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SaveDrawingState)(::windows::core::Vtable::as_raw(self), drawingstateblock.into().abi())
    }
    pub unsafe fn RestoreDrawingState<P0>(&self, drawingstateblock: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1DrawingStateBlock>>,
    {
        (::windows::core::Vtable::vtable(self).base__.RestoreDrawingState)(::windows::core::Vtable::as_raw(self), drawingstateblock.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn PushAxisAlignedClip(&self, cliprect: *const Common::D2D_RECT_F, antialiasmode: D2D1_ANTIALIAS_MODE) {
        (::windows::core::Vtable::vtable(self).base__.PushAxisAlignedClip)(::windows::core::Vtable::as_raw(self), cliprect, antialiasmode)
    }
    pub unsafe fn PopAxisAlignedClip(&self) {
        (::windows::core::Vtable::vtable(self).base__.PopAxisAlignedClip)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn Clear(&self, clearcolor: ::core::option::Option<*const Common::D2D1_COLOR_F>) {
        (::windows::core::Vtable::vtable(self).base__.Clear)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(clearcolor.unwrap_or(::std::ptr::null())))
    }
    pub unsafe fn BeginDraw(&self) {
        (::windows::core::Vtable::vtable(self).base__.BeginDraw)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn EndDraw(&self, tag1: ::core::option::Option<*mut u64>, tag2: ::core::option::Option<*mut u64>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.EndDraw)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(tag1.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(tag2.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetPixelFormat(&self) -> Common::D2D1_PIXEL_FORMAT {
        let mut result__: Common::D2D1_PIXEL_FORMAT = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetPixelFormat)(::windows::core::Vtable::as_raw(self), &mut result__);
        result__
    }
    pub unsafe fn SetDpi(&self, dpix: f32, dpiy: f32) {
        (::windows::core::Vtable::vtable(self).base__.SetDpi)(::windows::core::Vtable::as_raw(self), dpix, dpiy)
    }
    pub unsafe fn GetDpi(&self, dpix: *mut f32, dpiy: *mut f32) {
        (::windows::core::Vtable::vtable(self).base__.GetDpi)(::windows::core::Vtable::as_raw(self), dpix, dpiy)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetSize(&self) -> Common::D2D_SIZE_F {
        let mut result__: Common::D2D_SIZE_F = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetSize)(::windows::core::Vtable::as_raw(self), &mut result__);
        result__
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetPixelSize(&self) -> Common::D2D_SIZE_U {
        let mut result__: Common::D2D_SIZE_U = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetPixelSize)(::windows::core::Vtable::as_raw(self), &mut result__);
        result__
    }
    pub unsafe fn GetMaximumBitmapSize(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.GetMaximumBitmapSize)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn IsSupported(&self, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.IsSupported)(::windows::core::Vtable::as_raw(self), rendertargetproperties)
    }
}
impl ::core::cmp::PartialEq for ID2D1BlendTransform {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1BlendTransform {}
impl ::core::fmt::Debug for ID2D1BlendTransform {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1BlendTransform").field(&self.0).finish()
    }
}
impl ID2D1BlendTransform {
    pub unsafe fn GetInputCount(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.base__.GetInputCount)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn SetOutputBuffer(&self, bufferprecision: D2D1_BUFFER_PRECISION, channeldepth: D2D1_CHANNEL_DEPTH) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetOutputBuffer)(::windows::core::Vtable::as_raw(self), bufferprecision, channeldepth).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCached<P0>(&self, iscached: P0)
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetCached)(::windows::core::Vtable::as_raw(self), iscached.into())
    }
}
impl ::core::cmp::PartialEq for ID2D1BorderTransform {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1BorderTransform {}
impl ::core::fmt::Debug for ID2D1BorderTransform {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1BorderTransform").field(&self.0).finish()
    }
}
impl ID2D1BorderTransform {
    pub unsafe fn GetInputCount(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.base__.GetInputCount)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn SetOutputBuffer(&self, bufferprecision: D2D1_BUFFER_PRECISION, channeldepth: D2D1_CHANNEL_DEPTH) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetOutputBuffer)(::windows::core::Vtable::as_raw(self), bufferprecision, channeldepth).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCached<P0>(&self, iscached: P0)
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetCached)(::windows::core::Vtable::as_raw(self), iscached.into())
    }
}
impl ::core::cmp::PartialEq for ID2D1BoundsAdjustmentTransform {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1BoundsAdjustmentTransform {}
impl ::core::fmt::Debug for ID2D1BoundsAdjustmentTransform {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1BoundsAdjustmentTransform").field(&self.0).finish()
    }
}
impl ID2D1BoundsAdjustmentTransform {
    pub unsafe fn GetInputCount(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.GetInputCount)(::windows::core::Vtable::as_raw(self))
    }
}
impl ::core::cmp::PartialEq for ID2D1Brush {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1Brush {}
impl ::core::fmt::Debug for ID2D1Brush {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1Brush").field(&self.0).finish()
    }
}
impl ID2D1Brush {
    pub unsafe fn GetFactory(&self) -> ::windows::core::Result<ID2D1Factory> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFactory)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1Factory as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
}
impl ::core::cmp::PartialEq for ID2D1ColorContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1ColorContext {}
impl ::core::fmt::Debug for ID2D1ColorContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1ColorContext").field(&self.0).finish()
    }
}
impl ID2D1ColorContext {
    pub unsafe fn GetFactory(&self) -> ::windows::core::Result<ID2D1Factory> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFactory)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1Factory as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
}
impl ::core::cmp::PartialEq for ID2D1ColorContext1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1ColorContext1 {}
impl ::core::fmt::Debug for ID2D1ColorContext1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1ColorContext1").field(&self.0).finish()
    }
}
impl ID2D1ColorContext1 {
    pub unsafe fn GetFactory(&self) -> ::windows::core::Result<ID2D1Factory> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetFactory)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1Factory as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn GetColorSpace(&self) -> D2D1_COLOR_SPACE {
        (::windows::core::Vtable::vtable(self).base__.GetColorSpace)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetProfileSize(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.GetProfileSize)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetProfile(&self, profile: &mut [u8]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetProfile)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(profile.as_ptr()), profile.len() as _).ok()
    }
}
impl ::core::cmp::PartialEq for ID2D1CommandList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1CommandList {}
impl ::core::fmt::Debug for ID2D1CommandList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1CommandList").field(&self.0).finish()
    }
}
impl ID2D1CommandList {
    pub unsafe fn GetFactory(&self) -> ::windows::core::Result<ID2D1Factory> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetFactory)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1Factory as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
}
impl ::core::cmp::PartialEq for ID2D1CommandSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1CommandSink {}
impl ::core::fmt::Debug for ID2D1CommandSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1CommandSink").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID2D1CommandSink1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1CommandSink1 {}
impl ::core::fmt::Debug for ID2D1CommandSink1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1CommandSink1").field(&self.0).finish()
    }
}
impl ID2D1CommandSink1 {
    pub unsafe fn BeginDraw(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.BeginDraw)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn EndDraw(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.EndDraw)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetAntialiasMode(&self, antialiasmode: D2D1_ANTIALIAS_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetAntialiasMode)(::windows::core::Vtable::as_raw(self), antialiasmode).ok()
    }
    pub unsafe fn SetTags(&self, tag1: u64, tag2: u64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetTags)(::windows::core::Vtable::as_raw(self), tag1, tag2).ok()
    }
    pub unsafe fn SetTextAntialiasMode(&self, textantialiasmode: D2D1_TEXT_ANTIALIAS_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetTextAntialiasMode)(::windows::core::Vtable::as_raw(self), textantialiasmode).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(feature = "Win32_Graphics_DirectWrite")]
    pub unsafe fn SetTextRenderingParams<P0>(&self, textrenderingparams: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::DirectWrite::IDWriteRenderingParams>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetTextRenderingParams)(::windows::core::Vtable::as_raw(self), textrenderingparams.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn SetTransform(&self, transform: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetTransform)(::windows::core::Vtable::as_raw(self), transform).ok()
    }
    pub unsafe fn SetPrimitiveBlend(&self, primitiveblend: D2D1_PRIMITIVE_BLEND) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetPrimitiveBlend)(::windows::core::Vtable::as_raw(self), primitiveblend).ok()
    }
    pub unsafe fn SetUnitMode(&self, unitmode: D2D1_UNIT_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetUnitMode)(::windows::core::Vtable::as_raw(self), unitmode).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn Clear(&self, color: ::core::option::Option<*const Common::D2D1_COLOR_F>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Clear)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(color.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn DrawGlyphRun<P0>(&self, baselineorigin: Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, glyphrundescription: ::core::option::Option<*const super::DirectWrite::DWRITE_GLYPH_RUN_DESCRIPTION>, foregroundbrush: P0, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DrawGlyphRun)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(baselineorigin), glyphrun, ::core::mem::transmute(glyphrundescription.unwrap_or(::std::ptr::null())), foregroundbrush.into().abi(), measuringmode).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawLine<P0, P1>(&self, point0: Common::D2D_POINT_2F, point1: Common::D2D_POINT_2F, brush: P0, strokewidth: f32, strokestyle: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DrawLine)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(point0), ::core::mem::transmute(point1), brush.into().abi(), strokewidth, strokestyle.into().abi()).ok()
    }
    pub unsafe fn DrawGeometry<P0, P1, P2>(&self, geometry: P0, brush: P1, strokewidth: f32, strokestyle: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Geometry>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P2: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DrawGeometry)(::windows::core::Vtable::as_raw(self), geometry.into().abi(), brush.into().abi(), strokewidth, strokestyle.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawRectangle<P0, P1>(&self, rect: *const Common::D2D_RECT_F, brush: P0, strokewidth: f32, strokestyle: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DrawRectangle)(::windows::core::Vtable::as_raw(self), rect, brush.into().abi(), strokewidth, strokestyle.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawBitmap<P0>(&self, bitmap: P0, destinationrectangle: ::core::option::Option<*const Common::D2D_RECT_F>, opacity: f32, interpolationmode: D2D1_INTERPOLATION_MODE, sourcerectangle: ::core::option::Option<*const Common::D2D_RECT_F>, perspectivetransform: ::core::option::Option<*const Common::D2D_MATRIX_4X4_F>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Bitmap>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DrawBitmap)(::windows::core::Vtable::as_raw(self), bitmap.into().abi(), ::core::mem::transmute(destinationrectangle.unwrap_or(::std::ptr::null())), opacity, interpolationmode, ::core::mem::transmute(sourcerectangle.unwrap_or(::std::ptr::null())), ::core::mem::transmute(perspectivetransform.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawImage<P0>(&self, image: P0, targetoffset: ::core::option::Option<*const Common::D2D_POINT_2F>, imagerectangle: ::core::option::Option<*const Common::D2D_RECT_F>, interpolationmode: D2D1_INTERPOLATION_MODE, compositemode: Common::D2D1_COMPOSITE_MODE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Image>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DrawImage)(::windows::core::Vtable::as_raw(self), image.into().abi(), ::core::mem::transmute(targetoffset.unwrap_or(::std::ptr::null())), ::core::mem::transmute(imagerectangle.unwrap_or(::std::ptr::null())), interpolationmode, compositemode).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawGdiMetafile<P0>(&self, gdimetafile: P0, targetoffset: ::core::option::Option<*const Common::D2D_POINT_2F>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1GdiMetafile>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DrawGdiMetafile)(::windows::core::Vtable::as_raw(self), gdimetafile.into().abi(), ::core::mem::transmute(targetoffset.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn FillMesh<P0, P1>(&self, mesh: P0, brush: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Mesh>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.FillMesh)(::windows::core::Vtable::as_raw(self), mesh.into().abi(), brush.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn FillOpacityMask<P0, P1>(&self, opacitymask: P0, brush: P1, destinationrectangle: ::core::option::Option<*const Common::D2D_RECT_F>, sourcerectangle: ::core::option::Option<*const Common::D2D_RECT_F>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Bitmap>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.FillOpacityMask)(::windows::core::Vtable::as_raw(self), opacitymask.into().abi(), brush.into().abi(), ::core::mem::transmute(destinationrectangle.unwrap_or(::std::ptr::null())), ::core::mem::transmute(sourcerectangle.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn FillGeometry<P0, P1, P2>(&self, geometry: P0, brush: P1, opacitybrush: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Geometry>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P2: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.FillGeometry)(::windows::core::Vtable::as_raw(self), geometry.into().abi(), brush.into().abi(), opacitybrush.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn FillRectangle<P0>(&self, rect: *const Common::D2D_RECT_F, brush: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.FillRectangle)(::windows::core::Vtable::as_raw(self), rect, brush.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn PushAxisAlignedClip(&self, cliprect: *const Common::D2D_RECT_F, antialiasmode: D2D1_ANTIALIAS_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.PushAxisAlignedClip)(::windows::core::Vtable::as_raw(self), cliprect, antialiasmode).ok()
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn PushLayer<P0>(&self, layerparameters1: *const D2D1_LAYER_PARAMETERS1, layer: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Layer>>,
    {
        (::windows::core::Vtable::vtable(self).base__.PushLayer)(::windows::core::Vtable::as_raw(self), layerparameters1, layer.into().abi()).ok()
    }
    pub unsafe fn PopAxisAlignedClip(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.PopAxisAlignedClip)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn PopLayer(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.PopLayer)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
impl ::core::cmp::PartialEq for ID2D1CommandSink2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1CommandSink2 {}
impl ::core::fmt::Debug for ID2D1CommandSink2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1CommandSink2").field(&self.0).finish()
    }
}
impl ID2D1CommandSink2 {
    pub unsafe fn BeginDraw(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.BeginDraw)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn EndDraw(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.EndDraw)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetAntialiasMode(&self, antialiasmode: D2D1_ANTIALIAS_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetAntialiasMode)(::windows::core::Vtable::as_raw(self), antialiasmode).ok()
    }
    pub unsafe fn SetTags(&self, tag1: u64, tag2: u64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetTags)(::windows::core::Vtable::as_raw(self), tag1, tag2).ok()
    }
    pub unsafe fn SetTextAntialiasMode(&self, textantialiasmode: D2D1_TEXT_ANTIALIAS_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetTextAntialiasMode)(::windows::core::Vtable::as_raw(self), textantialiasmode).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(feature = "Win32_Graphics_DirectWrite")]
    pub unsafe fn SetTextRenderingParams<P0>(&self, textrenderingparams: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::DirectWrite::IDWriteRenderingParams>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetTextRenderingParams)(::windows::core::Vtable::as_raw(self), textrenderingparams.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn SetTransform(&self, transform: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetTransform)(::windows::core::Vtable::as_raw(self), transform).ok()
    }
    pub unsafe fn SetPrimitiveBlend(&self, primitiveblend: D2D1_PRIMITIVE_BLEND) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPrimitiveBlend)(::windows::core::Vtable::as_raw(self), primitiveblend).ok()
    }
    pub unsafe fn SetUnitMode(&self, unitmode: D2D1_UNIT_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetUnitMode)(::windows::core::Vtable::as_raw(self), unitmode).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn Clear(&self, color: ::core::option::Option<*const Common::D2D1_COLOR_F>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.Clear)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(color.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn DrawGlyphRun<P0>(&self, baselineorigin: Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, glyphrundescription: ::core::option::Option<*const super::DirectWrite::DWRITE_GLYPH_RUN_DESCRIPTION>, foregroundbrush: P0, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.DrawGlyphRun)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(baselineorigin), glyphrun, ::core::mem::transmute(glyphrundescription.unwrap_or(::std::ptr::null())), foregroundbrush.into().abi(), measuringmode).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawLine<P0, P1>(&self, point0: Common::D2D_POINT_2F, point1: Common::D2D_POINT_2F, brush: P0, strokewidth: f32, strokestyle: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.DrawLine)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(point0), ::core::mem::transmute(point1), brush.into().abi(), strokewidth, strokestyle.into().abi()).ok()
    }
    pub unsafe fn DrawGeometry<P0, P1, P2>(&self, geometry: P0, brush: P1, strokewidth: f32, strokestyle: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Geometry>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P2: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.DrawGeometry)(::windows::core::Vtable::as_raw(self), geometry.into().abi(), brush.into().abi(), strokewidth, strokestyle.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawRectangle<P0, P1>(&self, rect: *const Common::D2D_RECT_F, brush: P0, strokewidth: f32, strokestyle: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.DrawRectangle)(::windows::core::Vtable::as_raw(self), rect, brush.into().abi(), strokewidth, strokestyle.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawBitmap<P0>(&self, bitmap: P0, destinationrectangle: ::core::option::Option<*const Common::D2D_RECT_F>, opacity: f32, interpolationmode: D2D1_INTERPOLATION_MODE, sourcerectangle: ::core::option::Option<*const Common::D2D_RECT_F>, perspectivetransform: ::core::option::Option<*const Common::D2D_MATRIX_4X4_F>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Bitmap>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.DrawBitmap)(::windows::core::Vtable::as_raw(self), bitmap.into().abi(), ::core::mem::transmute(destinationrectangle.unwrap_or(::std::ptr::null())), opacity, interpolationmode, ::core::mem::transmute(sourcerectangle.unwrap_or(::std::ptr::null())), ::core::mem::transmute(perspectivetransform.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawImage<P0>(&self, image: P0, targetoffset: ::core::option::Option<*const Common::D2D_POINT_2F>, imagerectangle: ::core::option::Option<*const Common::D2D_RECT_F>, interpolationmode: D2D1_INTERPOLATION_MODE, compositemode: Common::D2D1_COMPOSITE_MODE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Image>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.DrawImage)(::windows::core::Vtable::as_raw(self), image.into().abi(), ::core::mem::transmute(targetoffset.unwrap_or(::std::ptr::null())), ::core::mem::transmute(imagerectangle.unwrap_or(::std::ptr::null())), interpolationmode, compositemode).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawGdiMetafile<P0>(&self, gdimetafile: P0, targetoffset: ::core::option::Option<*const Common::D2D_POINT_2F>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1GdiMetafile>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.DrawGdiMetafile)(::windows::core::Vtable::as_raw(self), gdimetafile.into().abi(), ::core::mem::transmute(targetoffset.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn FillMesh<P0, P1>(&self, mesh: P0, brush: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Mesh>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.FillMesh)(::windows::core::Vtable::as_raw(self), mesh.into().abi(), brush.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn FillOpacityMask<P0, P1>(&self, opacitymask: P0, brush: P1, destinationrectangle: ::core::option::Option<*const Common::D2D_RECT_F>, sourcerectangle: ::core::option::Option<*const Common::D2D_RECT_F>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Bitmap>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.FillOpacityMask)(::windows::core::Vtable::as_raw(self), opacitymask.into().abi(), brush.into().abi(), ::core::mem::transmute(destinationrectangle.unwrap_or(::std::ptr::null())), ::core::mem::transmute(sourcerectangle.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn FillGeometry<P0, P1, P2>(&self, geometry: P0, brush: P1, opacitybrush: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Geometry>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P2: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.FillGeometry)(::windows::core::Vtable::as_raw(self), geometry.into().abi(), brush.into().abi(), opacitybrush.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn FillRectangle<P0>(&self, rect: *const Common::D2D_RECT_F, brush: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.FillRectangle)(::windows::core::Vtable::as_raw(self), rect, brush.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn PushAxisAlignedClip(&self, cliprect: *const Common::D2D_RECT_F, antialiasmode: D2D1_ANTIALIAS_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.PushAxisAlignedClip)(::windows::core::Vtable::as_raw(self), cliprect, antialiasmode).ok()
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn PushLayer<P0>(&self, layerparameters1: *const D2D1_LAYER_PARAMETERS1, layer: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Layer>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.PushLayer)(::windows::core::Vtable::as_raw(self), layerparameters1, layer.into().abi()).ok()
    }
    pub unsafe fn PopAxisAlignedClip(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.PopAxisAlignedClip)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn PopLayer(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.PopLayer)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetPrimitiveBlend1(&self, primitiveblend: D2D1_PRIMITIVE_BLEND) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetPrimitiveBlend1)(::windows::core::Vtable::as_raw(self), primitiveblend).ok()
    }
}
impl ::core::cmp::PartialEq for ID2D1CommandSink3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1CommandSink3 {}
impl ::core::fmt::Debug for ID2D1CommandSink3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1CommandSink3").field(&self.0).finish()
    }
}
impl ID2D1CommandSink3 {
    pub unsafe fn BeginDraw(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.BeginDraw)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn EndDraw(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.EndDraw)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetAntialiasMode(&self, antialiasmode: D2D1_ANTIALIAS_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetAntialiasMode)(::windows::core::Vtable::as_raw(self), antialiasmode).ok()
    }
    pub unsafe fn SetTags(&self, tag1: u64, tag2: u64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetTags)(::windows::core::Vtable::as_raw(self), tag1, tag2).ok()
    }
    pub unsafe fn SetTextAntialiasMode(&self, textantialiasmode: D2D1_TEXT_ANTIALIAS_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetTextAntialiasMode)(::windows::core::Vtable::as_raw(self), textantialiasmode).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(feature = "Win32_Graphics_DirectWrite")]
    pub unsafe fn SetTextRenderingParams<P0>(&self, textrenderingparams: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::DirectWrite::IDWriteRenderingParams>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetTextRenderingParams)(::windows::core::Vtable::as_raw(self), textrenderingparams.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn SetTransform(&self, transform: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetTransform)(::windows::core::Vtable::as_raw(self), transform).ok()
    }
    pub unsafe fn SetPrimitiveBlend(&self, primitiveblend: D2D1_PRIMITIVE_BLEND) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrimitiveBlend)(::windows::core::Vtable::as_raw(self), primitiveblend).ok()
    }
    pub unsafe fn SetUnitMode(&self, unitmode: D2D1_UNIT_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetUnitMode)(::windows::core::Vtable::as_raw(self), unitmode).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn Clear(&self, color: ::core::option::Option<*const Common::D2D1_COLOR_F>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.Clear)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(color.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn DrawGlyphRun<P0>(&self, baselineorigin: Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, glyphrundescription: ::core::option::Option<*const super::DirectWrite::DWRITE_GLYPH_RUN_DESCRIPTION>, foregroundbrush: P0, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.DrawGlyphRun)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(baselineorigin), glyphrun, ::core::mem::transmute(glyphrundescription.unwrap_or(::std::ptr::null())), foregroundbrush.into().abi(), measuringmode).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawLine<P0, P1>(&self, point0: Common::D2D_POINT_2F, point1: Common::D2D_POINT_2F, brush: P0, strokewidth: f32, strokestyle: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.DrawLine)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(point0), ::core::mem::transmute(point1), brush.into().abi(), strokewidth, strokestyle.into().abi()).ok()
    }
    pub unsafe fn DrawGeometry<P0, P1, P2>(&self, geometry: P0, brush: P1, strokewidth: f32, strokestyle: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Geometry>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P2: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.DrawGeometry)(::windows::core::Vtable::as_raw(self), geometry.into().abi(), brush.into().abi(), strokewidth, strokestyle.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawRectangle<P0, P1>(&self, rect: *const Common::D2D_RECT_F, brush: P0, strokewidth: f32, strokestyle: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.DrawRectangle)(::windows::core::Vtable::as_raw(self), rect, brush.into().abi(), strokewidth, strokestyle.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawBitmap<P0>(&self, bitmap: P0, destinationrectangle: ::core::option::Option<*const Common::D2D_RECT_F>, opacity: f32, interpolationmode: D2D1_INTERPOLATION_MODE, sourcerectangle: ::core::option::Option<*const Common::D2D_RECT_F>, perspectivetransform: ::core::option::Option<*const Common::D2D_MATRIX_4X4_F>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Bitmap>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.DrawBitmap)(::windows::core::Vtable::as_raw(self), bitmap.into().abi(), ::core::mem::transmute(destinationrectangle.unwrap_or(::std::ptr::null())), opacity, interpolationmode, ::core::mem::transmute(sourcerectangle.unwrap_or(::std::ptr::null())), ::core::mem::transmute(perspectivetransform.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawImage<P0>(&self, image: P0, targetoffset: ::core::option::Option<*const Common::D2D_POINT_2F>, imagerectangle: ::core::option::Option<*const Common::D2D_RECT_F>, interpolationmode: D2D1_INTERPOLATION_MODE, compositemode: Common::D2D1_COMPOSITE_MODE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Image>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.DrawImage)(::windows::core::Vtable::as_raw(self), image.into().abi(), ::core::mem::transmute(targetoffset.unwrap_or(::std::ptr::null())), ::core::mem::transmute(imagerectangle.unwrap_or(::std::ptr::null())), interpolationmode, compositemode).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawGdiMetafile<P0>(&self, gdimetafile: P0, targetoffset: ::core::option::Option<*const Common::D2D_POINT_2F>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1GdiMetafile>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.DrawGdiMetafile)(::windows::core::Vtable::as_raw(self), gdimetafile.into().abi(), ::core::mem::transmute(targetoffset.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn FillMesh<P0, P1>(&self, mesh: P0, brush: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Mesh>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.FillMesh)(::windows::core::Vtable::as_raw(self), mesh.into().abi(), brush.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn FillOpacityMask<P0, P1>(&self, opacitymask: P0, brush: P1, destinationrectangle: ::core::option::Option<*const Common::D2D_RECT_F>, sourcerectangle: ::core::option::Option<*const Common::D2D_RECT_F>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Bitmap>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.FillOpacityMask)(::windows::core::Vtable::as_raw(self), opacitymask.into().abi(), brush.into().abi(), ::core::mem::transmute(destinationrectangle.unwrap_or(::std::ptr::null())), ::core::mem::transmute(sourcerectangle.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn FillGeometry<P0, P1, P2>(&self, geometry: P0, brush: P1, opacitybrush: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Geometry>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P2: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.FillGeometry)(::windows::core::Vtable::as_raw(self), geometry.into().abi(), brush.into().abi(), opacitybrush.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn FillRectangle<P0>(&self, rect: *const Common::D2D_RECT_F, brush: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.FillRectangle)(::windows::core::Vtable::as_raw(self), rect, brush.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn PushAxisAlignedClip(&self, cliprect: *const Common::D2D_RECT_F, antialiasmode: D2D1_ANTIALIAS_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.PushAxisAlignedClip)(::windows::core::Vtable::as_raw(self), cliprect, antialiasmode).ok()
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn PushLayer<P0>(&self, layerparameters1: *const D2D1_LAYER_PARAMETERS1, layer: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Layer>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.PushLayer)(::windows::core::Vtable::as_raw(self), layerparameters1, layer.into().abi()).ok()
    }
    pub unsafe fn PopAxisAlignedClip(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.PopAxisAlignedClip)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn PopLayer(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.PopLayer)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetPrimitiveBlend1(&self, primitiveblend: D2D1_PRIMITIVE_BLEND) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPrimitiveBlend1)(::windows::core::Vtable::as_raw(self), primitiveblend).ok()
    }
    pub unsafe fn DrawInk<P0, P1, P2>(&self, ink: P0, brush: P1, inkstyle: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Ink>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P2: ::std::convert::Into<::windows::core::InParam<ID2D1InkStyle>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DrawInk)(::windows::core::Vtable::as_raw(self), ink.into().abi(), brush.into().abi(), inkstyle.into().abi()).ok()
    }
    pub unsafe fn DrawGradientMesh<P0>(&self, gradientmesh: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1GradientMesh>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DrawGradientMesh)(::windows::core::Vtable::as_raw(self), gradientmesh.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawGdiMetafile2<P0>(&self, gdimetafile: P0, destinationrectangle: ::core::option::Option<*const Common::D2D_RECT_F>, sourcerectangle: ::core::option::Option<*const Common::D2D_RECT_F>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1GdiMetafile>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DrawGdiMetafile2)(::windows::core::Vtable::as_raw(self), gdimetafile.into().abi(), ::core::mem::transmute(destinationrectangle.unwrap_or(::std::ptr::null())), ::core::mem::transmute(sourcerectangle.unwrap_or(::std::ptr::null()))).ok()
    }
}
impl ::core::cmp::PartialEq for ID2D1CommandSink4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1CommandSink4 {}
impl ::core::fmt::Debug for ID2D1CommandSink4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1CommandSink4").field(&self.0).finish()
    }
}
impl ID2D1CommandSink4 {
    pub unsafe fn BeginDraw(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.BeginDraw)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn EndDraw(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.EndDraw)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetAntialiasMode(&self, antialiasmode: D2D1_ANTIALIAS_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetAntialiasMode)(::windows::core::Vtable::as_raw(self), antialiasmode).ok()
    }
    pub unsafe fn SetTags(&self, tag1: u64, tag2: u64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetTags)(::windows::core::Vtable::as_raw(self), tag1, tag2).ok()
    }
    pub unsafe fn SetTextAntialiasMode(&self, textantialiasmode: D2D1_TEXT_ANTIALIAS_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetTextAntialiasMode)(::windows::core::Vtable::as_raw(self), textantialiasmode).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(feature = "Win32_Graphics_DirectWrite")]
    pub unsafe fn SetTextRenderingParams<P0>(&self, textrenderingparams: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::DirectWrite::IDWriteRenderingParams>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetTextRenderingParams)(::windows::core::Vtable::as_raw(self), textrenderingparams.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn SetTransform(&self, transform: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetTransform)(::windows::core::Vtable::as_raw(self), transform).ok()
    }
    pub unsafe fn SetPrimitiveBlend(&self, primitiveblend: D2D1_PRIMITIVE_BLEND) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetPrimitiveBlend)(::windows::core::Vtable::as_raw(self), primitiveblend).ok()
    }
    pub unsafe fn SetUnitMode(&self, unitmode: D2D1_UNIT_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetUnitMode)(::windows::core::Vtable::as_raw(self), unitmode).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn Clear(&self, color: ::core::option::Option<*const Common::D2D1_COLOR_F>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.Clear)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(color.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn DrawGlyphRun<P0>(&self, baselineorigin: Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, glyphrundescription: ::core::option::Option<*const super::DirectWrite::DWRITE_GLYPH_RUN_DESCRIPTION>, foregroundbrush: P0, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.DrawGlyphRun)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(baselineorigin), glyphrun, ::core::mem::transmute(glyphrundescription.unwrap_or(::std::ptr::null())), foregroundbrush.into().abi(), measuringmode).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawLine<P0, P1>(&self, point0: Common::D2D_POINT_2F, point1: Common::D2D_POINT_2F, brush: P0, strokewidth: f32, strokestyle: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.DrawLine)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(point0), ::core::mem::transmute(point1), brush.into().abi(), strokewidth, strokestyle.into().abi()).ok()
    }
    pub unsafe fn DrawGeometry<P0, P1, P2>(&self, geometry: P0, brush: P1, strokewidth: f32, strokestyle: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Geometry>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P2: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.DrawGeometry)(::windows::core::Vtable::as_raw(self), geometry.into().abi(), brush.into().abi(), strokewidth, strokestyle.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawRectangle<P0, P1>(&self, rect: *const Common::D2D_RECT_F, brush: P0, strokewidth: f32, strokestyle: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.DrawRectangle)(::windows::core::Vtable::as_raw(self), rect, brush.into().abi(), strokewidth, strokestyle.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawBitmap<P0>(&self, bitmap: P0, destinationrectangle: ::core::option::Option<*const Common::D2D_RECT_F>, opacity: f32, interpolationmode: D2D1_INTERPOLATION_MODE, sourcerectangle: ::core::option::Option<*const Common::D2D_RECT_F>, perspectivetransform: ::core::option::Option<*const Common::D2D_MATRIX_4X4_F>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Bitmap>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.DrawBitmap)(::windows::core::Vtable::as_raw(self), bitmap.into().abi(), ::core::mem::transmute(destinationrectangle.unwrap_or(::std::ptr::null())), opacity, interpolationmode, ::core::mem::transmute(sourcerectangle.unwrap_or(::std::ptr::null())), ::core::mem::transmute(perspectivetransform.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawImage<P0>(&self, image: P0, targetoffset: ::core::option::Option<*const Common::D2D_POINT_2F>, imagerectangle: ::core::option::Option<*const Common::D2D_RECT_F>, interpolationmode: D2D1_INTERPOLATION_MODE, compositemode: Common::D2D1_COMPOSITE_MODE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Image>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.DrawImage)(::windows::core::Vtable::as_raw(self), image.into().abi(), ::core::mem::transmute(targetoffset.unwrap_or(::std::ptr::null())), ::core::mem::transmute(imagerectangle.unwrap_or(::std::ptr::null())), interpolationmode, compositemode).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawGdiMetafile<P0>(&self, gdimetafile: P0, targetoffset: ::core::option::Option<*const Common::D2D_POINT_2F>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1GdiMetafile>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.DrawGdiMetafile)(::windows::core::Vtable::as_raw(self), gdimetafile.into().abi(), ::core::mem::transmute(targetoffset.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn FillMesh<P0, P1>(&self, mesh: P0, brush: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Mesh>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.FillMesh)(::windows::core::Vtable::as_raw(self), mesh.into().abi(), brush.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn FillOpacityMask<P0, P1>(&self, opacitymask: P0, brush: P1, destinationrectangle: ::core::option::Option<*const Common::D2D_RECT_F>, sourcerectangle: ::core::option::Option<*const Common::D2D_RECT_F>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Bitmap>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.FillOpacityMask)(::windows::core::Vtable::as_raw(self), opacitymask.into().abi(), brush.into().abi(), ::core::mem::transmute(destinationrectangle.unwrap_or(::std::ptr::null())), ::core::mem::transmute(sourcerectangle.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn FillGeometry<P0, P1, P2>(&self, geometry: P0, brush: P1, opacitybrush: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Geometry>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P2: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.FillGeometry)(::windows::core::Vtable::as_raw(self), geometry.into().abi(), brush.into().abi(), opacitybrush.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn FillRectangle<P0>(&self, rect: *const Common::D2D_RECT_F, brush: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.FillRectangle)(::windows::core::Vtable::as_raw(self), rect, brush.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn PushAxisAlignedClip(&self, cliprect: *const Common::D2D_RECT_F, antialiasmode: D2D1_ANTIALIAS_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.PushAxisAlignedClip)(::windows::core::Vtable::as_raw(self), cliprect, antialiasmode).ok()
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn PushLayer<P0>(&self, layerparameters1: *const D2D1_LAYER_PARAMETERS1, layer: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Layer>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.PushLayer)(::windows::core::Vtable::as_raw(self), layerparameters1, layer.into().abi()).ok()
    }
    pub unsafe fn PopAxisAlignedClip(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.PopAxisAlignedClip)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn PopLayer(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.PopLayer)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetPrimitiveBlend1(&self, primitiveblend: D2D1_PRIMITIVE_BLEND) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrimitiveBlend1)(::windows::core::Vtable::as_raw(self), primitiveblend).ok()
    }
    pub unsafe fn DrawInk<P0, P1, P2>(&self, ink: P0, brush: P1, inkstyle: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Ink>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P2: ::std::convert::Into<::windows::core::InParam<ID2D1InkStyle>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.DrawInk)(::windows::core::Vtable::as_raw(self), ink.into().abi(), brush.into().abi(), inkstyle.into().abi()).ok()
    }
    pub unsafe fn DrawGradientMesh<P0>(&self, gradientmesh: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1GradientMesh>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.DrawGradientMesh)(::windows::core::Vtable::as_raw(self), gradientmesh.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawGdiMetafile2<P0>(&self, gdimetafile: P0, destinationrectangle: ::core::option::Option<*const Common::D2D_RECT_F>, sourcerectangle: ::core::option::Option<*const Common::D2D_RECT_F>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1GdiMetafile>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.DrawGdiMetafile2)(::windows::core::Vtable::as_raw(self), gdimetafile.into().abi(), ::core::mem::transmute(destinationrectangle.unwrap_or(::std::ptr::null())), ::core::mem::transmute(sourcerectangle.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn DrawSpriteBatch<P0, P1>(&self, spritebatch: P0, startindex: u32, spritecount: u32, bitmap: P1, interpolationmode: D2D1_BITMAP_INTERPOLATION_MODE, spriteoptions: D2D1_SPRITE_OPTIONS) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1SpriteBatch>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Bitmap>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DrawSpriteBatch)(::windows::core::Vtable::as_raw(self), spritebatch.into().abi(), startindex, spritecount, bitmap.into().abi(), interpolationmode, spriteoptions).ok()
    }
}
impl ::core::cmp::PartialEq for ID2D1CommandSink5 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1CommandSink5 {}
impl ::core::fmt::Debug for ID2D1CommandSink5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1CommandSink5").field(&self.0).finish()
    }
}
impl ID2D1CommandSink5 {
    pub unsafe fn BeginDraw(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.BeginDraw)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn EndDraw(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.EndDraw)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetAntialiasMode(&self, antialiasmode: D2D1_ANTIALIAS_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetAntialiasMode)(::windows::core::Vtable::as_raw(self), antialiasmode).ok()
    }
    pub unsafe fn SetTags(&self, tag1: u64, tag2: u64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetTags)(::windows::core::Vtable::as_raw(self), tag1, tag2).ok()
    }
    pub unsafe fn SetTextAntialiasMode(&self, textantialiasmode: D2D1_TEXT_ANTIALIAS_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetTextAntialiasMode)(::windows::core::Vtable::as_raw(self), textantialiasmode).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(feature = "Win32_Graphics_DirectWrite")]
    pub unsafe fn SetTextRenderingParams<P0>(&self, textrenderingparams: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::DirectWrite::IDWriteRenderingParams>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetTextRenderingParams)(::windows::core::Vtable::as_raw(self), textrenderingparams.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn SetTransform(&self, transform: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetTransform)(::windows::core::Vtable::as_raw(self), transform).ok()
    }
    pub unsafe fn SetPrimitiveBlend(&self, primitiveblend: D2D1_PRIMITIVE_BLEND) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetPrimitiveBlend)(::windows::core::Vtable::as_raw(self), primitiveblend).ok()
    }
    pub unsafe fn SetUnitMode(&self, unitmode: D2D1_UNIT_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetUnitMode)(::windows::core::Vtable::as_raw(self), unitmode).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn Clear(&self, color: ::core::option::Option<*const Common::D2D1_COLOR_F>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.Clear)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(color.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn DrawGlyphRun<P0>(&self, baselineorigin: Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, glyphrundescription: ::core::option::Option<*const super::DirectWrite::DWRITE_GLYPH_RUN_DESCRIPTION>, foregroundbrush: P0, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.DrawGlyphRun)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(baselineorigin), glyphrun, ::core::mem::transmute(glyphrundescription.unwrap_or(::std::ptr::null())), foregroundbrush.into().abi(), measuringmode).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawLine<P0, P1>(&self, point0: Common::D2D_POINT_2F, point1: Common::D2D_POINT_2F, brush: P0, strokewidth: f32, strokestyle: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.DrawLine)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(point0), ::core::mem::transmute(point1), brush.into().abi(), strokewidth, strokestyle.into().abi()).ok()
    }
    pub unsafe fn DrawGeometry<P0, P1, P2>(&self, geometry: P0, brush: P1, strokewidth: f32, strokestyle: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Geometry>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P2: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.DrawGeometry)(::windows::core::Vtable::as_raw(self), geometry.into().abi(), brush.into().abi(), strokewidth, strokestyle.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawRectangle<P0, P1>(&self, rect: *const Common::D2D_RECT_F, brush: P0, strokewidth: f32, strokestyle: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.DrawRectangle)(::windows::core::Vtable::as_raw(self), rect, brush.into().abi(), strokewidth, strokestyle.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawBitmap<P0>(&self, bitmap: P0, destinationrectangle: ::core::option::Option<*const Common::D2D_RECT_F>, opacity: f32, interpolationmode: D2D1_INTERPOLATION_MODE, sourcerectangle: ::core::option::Option<*const Common::D2D_RECT_F>, perspectivetransform: ::core::option::Option<*const Common::D2D_MATRIX_4X4_F>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Bitmap>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.DrawBitmap)(::windows::core::Vtable::as_raw(self), bitmap.into().abi(), ::core::mem::transmute(destinationrectangle.unwrap_or(::std::ptr::null())), opacity, interpolationmode, ::core::mem::transmute(sourcerectangle.unwrap_or(::std::ptr::null())), ::core::mem::transmute(perspectivetransform.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawImage<P0>(&self, image: P0, targetoffset: ::core::option::Option<*const Common::D2D_POINT_2F>, imagerectangle: ::core::option::Option<*const Common::D2D_RECT_F>, interpolationmode: D2D1_INTERPOLATION_MODE, compositemode: Common::D2D1_COMPOSITE_MODE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Image>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.DrawImage)(::windows::core::Vtable::as_raw(self), image.into().abi(), ::core::mem::transmute(targetoffset.unwrap_or(::std::ptr::null())), ::core::mem::transmute(imagerectangle.unwrap_or(::std::ptr::null())), interpolationmode, compositemode).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawGdiMetafile<P0>(&self, gdimetafile: P0, targetoffset: ::core::option::Option<*const Common::D2D_POINT_2F>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1GdiMetafile>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.DrawGdiMetafile)(::windows::core::Vtable::as_raw(self), gdimetafile.into().abi(), ::core::mem::transmute(targetoffset.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn FillMesh<P0, P1>(&self, mesh: P0, brush: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Mesh>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.FillMesh)(::windows::core::Vtable::as_raw(self), mesh.into().abi(), brush.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn FillOpacityMask<P0, P1>(&self, opacitymask: P0, brush: P1, destinationrectangle: ::core::option::Option<*const Common::D2D_RECT_F>, sourcerectangle: ::core::option::Option<*const Common::D2D_RECT_F>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Bitmap>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.FillOpacityMask)(::windows::core::Vtable::as_raw(self), opacitymask.into().abi(), brush.into().abi(), ::core::mem::transmute(destinationrectangle.unwrap_or(::std::ptr::null())), ::core::mem::transmute(sourcerectangle.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn FillGeometry<P0, P1, P2>(&self, geometry: P0, brush: P1, opacitybrush: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Geometry>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P2: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.FillGeometry)(::windows::core::Vtable::as_raw(self), geometry.into().abi(), brush.into().abi(), opacitybrush.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn FillRectangle<P0>(&self, rect: *const Common::D2D_RECT_F, brush: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.FillRectangle)(::windows::core::Vtable::as_raw(self), rect, brush.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn PushAxisAlignedClip(&self, cliprect: *const Common::D2D_RECT_F, antialiasmode: D2D1_ANTIALIAS_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.PushAxisAlignedClip)(::windows::core::Vtable::as_raw(self), cliprect, antialiasmode).ok()
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn PushLayer<P0>(&self, layerparameters1: *const D2D1_LAYER_PARAMETERS1, layer: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Layer>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.PushLayer)(::windows::core::Vtable::as_raw(self), layerparameters1, layer.into().abi()).ok()
    }
    pub unsafe fn PopAxisAlignedClip(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.PopAxisAlignedClip)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn PopLayer(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.PopLayer)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetPrimitiveBlend1(&self, primitiveblend: D2D1_PRIMITIVE_BLEND) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetPrimitiveBlend1)(::windows::core::Vtable::as_raw(self), primitiveblend).ok()
    }
    pub unsafe fn DrawInk<P0, P1, P2>(&self, ink: P0, brush: P1, inkstyle: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Ink>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P2: ::std::convert::Into<::windows::core::InParam<ID2D1InkStyle>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.DrawInk)(::windows::core::Vtable::as_raw(self), ink.into().abi(), brush.into().abi(), inkstyle.into().abi()).ok()
    }
    pub unsafe fn DrawGradientMesh<P0>(&self, gradientmesh: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1GradientMesh>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.DrawGradientMesh)(::windows::core::Vtable::as_raw(self), gradientmesh.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawGdiMetafile2<P0>(&self, gdimetafile: P0, destinationrectangle: ::core::option::Option<*const Common::D2D_RECT_F>, sourcerectangle: ::core::option::Option<*const Common::D2D_RECT_F>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1GdiMetafile>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.DrawGdiMetafile2)(::windows::core::Vtable::as_raw(self), gdimetafile.into().abi(), ::core::mem::transmute(destinationrectangle.unwrap_or(::std::ptr::null())), ::core::mem::transmute(sourcerectangle.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn DrawSpriteBatch<P0, P1>(&self, spritebatch: P0, startindex: u32, spritecount: u32, bitmap: P1, interpolationmode: D2D1_BITMAP_INTERPOLATION_MODE, spriteoptions: D2D1_SPRITE_OPTIONS) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1SpriteBatch>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Bitmap>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.DrawSpriteBatch)(::windows::core::Vtable::as_raw(self), spritebatch.into().abi(), startindex, spritecount, bitmap.into().abi(), interpolationmode, spriteoptions).ok()
    }
    pub unsafe fn SetPrimitiveBlend2(&self, primitiveblend: D2D1_PRIMITIVE_BLEND) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetPrimitiveBlend2)(::windows::core::Vtable::as_raw(self), primitiveblend).ok()
    }
}
impl ::core::cmp::PartialEq for ID2D1ComputeInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1ComputeInfo {}
impl ::core::fmt::Debug for ID2D1ComputeInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1ComputeInfo").field(&self.0).finish()
    }
}
impl ID2D1ComputeInfo {
    pub unsafe fn SetInputDescription(&self, inputindex: u32, inputdescription: D2D1_INPUT_DESCRIPTION) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetInputDescription)(::windows::core::Vtable::as_raw(self), inputindex, ::core::mem::transmute(inputdescription)).ok()
    }
    pub unsafe fn SetOutputBuffer(&self, bufferprecision: D2D1_BUFFER_PRECISION, channeldepth: D2D1_CHANNEL_DEPTH) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetOutputBuffer)(::windows::core::Vtable::as_raw(self), bufferprecision, channeldepth).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCached<P0>(&self, iscached: P0)
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetCached)(::windows::core::Vtable::as_raw(self), iscached.into())
    }
    pub unsafe fn SetInstructionCountHint(&self, instructioncount: u32) {
        (::windows::core::Vtable::vtable(self).base__.SetInstructionCountHint)(::windows::core::Vtable::as_raw(self), instructioncount)
    }
}
impl ::core::cmp::PartialEq for ID2D1ComputeTransform {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1ComputeTransform {}
impl ::core::fmt::Debug for ID2D1ComputeTransform {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1ComputeTransform").field(&self.0).finish()
    }
}
impl ID2D1ComputeTransform {
    pub unsafe fn GetInputCount(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.base__.GetInputCount)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MapOutputRectToInputRects(&self, outputrect: *const super::super::Foundation::RECT, inputrects: &mut [super::super::Foundation::RECT]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.MapOutputRectToInputRects)(::windows::core::Vtable::as_raw(self), outputrect, ::core::mem::transmute(inputrects.as_ptr()), inputrects.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MapInputRectsToOutputRect(&self, inputrects: *const super::super::Foundation::RECT, inputopaquesubrects: *const super::super::Foundation::RECT, inputrectcount: u32, outputrect: *mut super::super::Foundation::RECT, outputopaquesubrect: *mut super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.MapInputRectsToOutputRect)(::windows::core::Vtable::as_raw(self), inputrects, inputopaquesubrects, inputrectcount, outputrect, outputopaquesubrect).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MapInvalidRect(&self, inputindex: u32, invalidinputrect: super::super::Foundation::RECT) -> ::windows::core::Result<super::super::Foundation::RECT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.MapInvalidRect)(::windows::core::Vtable::as_raw(self), inputindex, ::core::mem::transmute(invalidinputrect), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for ID2D1ConcreteTransform {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1ConcreteTransform {}
impl ::core::fmt::Debug for ID2D1ConcreteTransform {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1ConcreteTransform").field(&self.0).finish()
    }
}
impl ID2D1ConcreteTransform {
    pub unsafe fn GetInputCount(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.GetInputCount)(::windows::core::Vtable::as_raw(self))
    }
}
impl ::core::cmp::PartialEq for ID2D1DCRenderTarget {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1DCRenderTarget {}
impl ::core::fmt::Debug for ID2D1DCRenderTarget {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1DCRenderTarget").field(&self.0).finish()
    }
}
impl ID2D1DCRenderTarget {
    pub unsafe fn GetFactory(&self) -> ::windows::core::Result<ID2D1Factory> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetFactory)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1Factory as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateBitmap(&self, size: Common::D2D_SIZE_U, srcdata: ::core::option::Option<*const ::core::ffi::c_void>, pitch: u32, bitmapproperties: *const D2D1_BITMAP_PROPERTIES) -> ::windows::core::Result<ID2D1Bitmap> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateBitmap)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(size), ::core::mem::transmute(srcdata.unwrap_or(::std::ptr::null())), pitch, bitmapproperties, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Imaging\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
    pub unsafe fn CreateBitmapFromWicBitmap<P0>(&self, wicbitmapsource: P0, bitmapproperties: ::core::option::Option<*const D2D1_BITMAP_PROPERTIES>) -> ::windows::core::Result<ID2D1Bitmap>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Imaging::IWICBitmapSource>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateBitmapFromWicBitmap)(::windows::core::Vtable::as_raw(self), wicbitmapsource.into().abi(), ::core::mem::transmute(bitmapproperties.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSharedBitmap(&self, riid: *const ::windows::core::GUID, data: *mut ::core::ffi::c_void, bitmapproperties: ::core::option::Option<*const D2D1_BITMAP_PROPERTIES>, bitmap: *mut ::core::option::Option<ID2D1Bitmap>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CreateSharedBitmap)(::windows::core::Vtable::as_raw(self), riid, data, ::core::mem::transmute(bitmapproperties.unwrap_or(::std::ptr::null())), ::core::mem::transmute(bitmap)).ok()
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn CreateBitmapBrush<P0>(&self, bitmap: P0, bitmapbrushproperties: ::core::option::Option<*const D2D1_BITMAP_BRUSH_PROPERTIES>, brushproperties: ::core::option::Option<*const D2D1_BRUSH_PROPERTIES>) -> ::windows::core::Result<ID2D1BitmapBrush>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Bitmap>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateBitmapBrush)(::windows::core::Vtable::as_raw(self), bitmap.into().abi(), ::core::mem::transmute(bitmapbrushproperties.unwrap_or(::std::ptr::null())), ::core::mem::transmute(brushproperties.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn CreateSolidColorBrush(&self, color: *const Common::D2D1_COLOR_F, brushproperties: ::core::option::Option<*const D2D1_BRUSH_PROPERTIES>) -> ::windows::core::Result<ID2D1SolidColorBrush> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateSolidColorBrush)(::windows::core::Vtable::as_raw(self), color, ::core::mem::transmute(brushproperties.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CreateGradientStopCollection(&self, gradientstops: &[D2D1_GRADIENT_STOP], colorinterpolationgamma: D2D1_GAMMA, extendmode: D2D1_EXTEND_MODE) -> ::windows::core::Result<ID2D1GradientStopCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateGradientStopCollection)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(gradientstops.as_ptr()), gradientstops.len() as _, colorinterpolationgamma, extendmode, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn CreateLinearGradientBrush<P0>(&self, lineargradientbrushproperties: *const D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES, brushproperties: ::core::option::Option<*const D2D1_BRUSH_PROPERTIES>, gradientstopcollection: P0) -> ::windows::core::Result<ID2D1LinearGradientBrush>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1GradientStopCollection>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateLinearGradientBrush)(::windows::core::Vtable::as_raw(self), lineargradientbrushproperties, ::core::mem::transmute(brushproperties.unwrap_or(::std::ptr::null())), gradientstopcollection.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn CreateRadialGradientBrush<P0>(&self, radialgradientbrushproperties: *const D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES, brushproperties: ::core::option::Option<*const D2D1_BRUSH_PROPERTIES>, gradientstopcollection: P0) -> ::windows::core::Result<ID2D1RadialGradientBrush>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1GradientStopCollection>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateRadialGradientBrush)(::windows::core::Vtable::as_raw(self), radialgradientbrushproperties, ::core::mem::transmute(brushproperties.unwrap_or(::std::ptr::null())), gradientstopcollection.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateCompatibleRenderTarget(&self, desiredsize: ::core::option::Option<*const Common::D2D_SIZE_F>, desiredpixelsize: ::core::option::Option<*const Common::D2D_SIZE_U>, desiredformat: ::core::option::Option<*const Common::D2D1_PIXEL_FORMAT>, options: D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS) -> ::windows::core::Result<ID2D1BitmapRenderTarget> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateCompatibleRenderTarget)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(desiredsize.unwrap_or(::std::ptr::null())), ::core::mem::transmute(desiredpixelsize.unwrap_or(::std::ptr::null())), ::core::mem::transmute(desiredformat.unwrap_or(::std::ptr::null())), options, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CreateLayer(&self, size: ::core::option::Option<*const Common::D2D_SIZE_F>) -> ::windows::core::Result<ID2D1Layer> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateLayer)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(size.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateMesh(&self) -> ::windows::core::Result<ID2D1Mesh> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateMesh)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawLine<P0, P1>(&self, point0: Common::D2D_POINT_2F, point1: Common::D2D_POINT_2F, brush: P0, strokewidth: f32, strokestyle: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DrawLine)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(point0), ::core::mem::transmute(point1), brush.into().abi(), strokewidth, strokestyle.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawRectangle<P0, P1>(&self, rect: *const Common::D2D_RECT_F, brush: P0, strokewidth: f32, strokestyle: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DrawRectangle)(::windows::core::Vtable::as_raw(self), rect, brush.into().abi(), strokewidth, strokestyle.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn FillRectangle<P0>(&self, rect: *const Common::D2D_RECT_F, brush: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.FillRectangle)(::windows::core::Vtable::as_raw(self), rect, brush.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawRoundedRectangle<P0, P1>(&self, roundedrect: *const D2D1_ROUNDED_RECT, brush: P0, strokewidth: f32, strokestyle: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DrawRoundedRectangle)(::windows::core::Vtable::as_raw(self), roundedrect, brush.into().abi(), strokewidth, strokestyle.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn FillRoundedRectangle<P0>(&self, roundedrect: *const D2D1_ROUNDED_RECT, brush: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.FillRoundedRectangle)(::windows::core::Vtable::as_raw(self), roundedrect, brush.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawEllipse<P0, P1>(&self, ellipse: *const D2D1_ELLIPSE, brush: P0, strokewidth: f32, strokestyle: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DrawEllipse)(::windows::core::Vtable::as_raw(self), ellipse, brush.into().abi(), strokewidth, strokestyle.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn FillEllipse<P0>(&self, ellipse: *const D2D1_ELLIPSE, brush: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.FillEllipse)(::windows::core::Vtable::as_raw(self), ellipse, brush.into().abi())
    }
    pub unsafe fn DrawGeometry<P0, P1, P2>(&self, geometry: P0, brush: P1, strokewidth: f32, strokestyle: P2)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Geometry>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P2: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DrawGeometry)(::windows::core::Vtable::as_raw(self), geometry.into().abi(), brush.into().abi(), strokewidth, strokestyle.into().abi())
    }
    pub unsafe fn FillGeometry<P0, P1, P2>(&self, geometry: P0, brush: P1, opacitybrush: P2)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Geometry>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P2: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.FillGeometry)(::windows::core::Vtable::as_raw(self), geometry.into().abi(), brush.into().abi(), opacitybrush.into().abi())
    }
    pub unsafe fn FillMesh<P0, P1>(&self, mesh: P0, brush: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Mesh>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.FillMesh)(::windows::core::Vtable::as_raw(self), mesh.into().abi(), brush.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn FillOpacityMask<P0, P1>(&self, opacitymask: P0, brush: P1, content: D2D1_OPACITY_MASK_CONTENT, destinationrectangle: ::core::option::Option<*const Common::D2D_RECT_F>, sourcerectangle: ::core::option::Option<*const Common::D2D_RECT_F>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Bitmap>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.FillOpacityMask)(::windows::core::Vtable::as_raw(self), opacitymask.into().abi(), brush.into().abi(), content, ::core::mem::transmute(destinationrectangle.unwrap_or(::std::ptr::null())), ::core::mem::transmute(sourcerectangle.unwrap_or(::std::ptr::null())))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawBitmap<P0>(&self, bitmap: P0, destinationrectangle: ::core::option::Option<*const Common::D2D_RECT_F>, opacity: f32, interpolationmode: D2D1_BITMAP_INTERPOLATION_MODE, sourcerectangle: ::core::option::Option<*const Common::D2D_RECT_F>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Bitmap>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DrawBitmap)(::windows::core::Vtable::as_raw(self), bitmap.into().abi(), ::core::mem::transmute(destinationrectangle.unwrap_or(::std::ptr::null())), opacity, interpolationmode, ::core::mem::transmute(sourcerectangle.unwrap_or(::std::ptr::null())))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn DrawText<P0, P1>(&self, string: &[u16], textformat: P0, layoutrect: *const Common::D2D_RECT_F, defaultfillbrush: P1, options: D2D1_DRAW_TEXT_OPTIONS, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::DirectWrite::IDWriteTextFormat>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DrawText)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(string.as_ptr()), string.len() as _, textformat.into().abi(), layoutrect, defaultfillbrush.into().abi(), options, measuringmode)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn DrawTextLayout<P0, P1>(&self, origin: Common::D2D_POINT_2F, textlayout: P0, defaultfillbrush: P1, options: D2D1_DRAW_TEXT_OPTIONS)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::DirectWrite::IDWriteTextLayout>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DrawTextLayout)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(origin), textlayout.into().abi(), defaultfillbrush.into().abi(), options)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn DrawGlyphRun<P0>(&self, baselineorigin: Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, foregroundbrush: P0, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DrawGlyphRun)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(baselineorigin), glyphrun, foregroundbrush.into().abi(), measuringmode)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn SetTransform(&self, transform: *const super::super::super::Foundation::Numerics::Matrix3x2) {
        (::windows::core::Vtable::vtable(self).base__.SetTransform)(::windows::core::Vtable::as_raw(self), transform)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn GetTransform(&self, transform: *mut super::super::super::Foundation::Numerics::Matrix3x2) {
        (::windows::core::Vtable::vtable(self).base__.GetTransform)(::windows::core::Vtable::as_raw(self), transform)
    }
    pub unsafe fn SetAntialiasMode(&self, antialiasmode: D2D1_ANTIALIAS_MODE) {
        (::windows::core::Vtable::vtable(self).base__.SetAntialiasMode)(::windows::core::Vtable::as_raw(self), antialiasmode)
    }
    pub unsafe fn GetAntialiasMode(&self) -> D2D1_ANTIALIAS_MODE {
        (::windows::core::Vtable::vtable(self).base__.GetAntialiasMode)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn SetTextAntialiasMode(&self, textantialiasmode: D2D1_TEXT_ANTIALIAS_MODE) {
        (::windows::core::Vtable::vtable(self).base__.SetTextAntialiasMode)(::windows::core::Vtable::as_raw(self), textantialiasmode)
    }
    pub unsafe fn GetTextAntialiasMode(&self) -> D2D1_TEXT_ANTIALIAS_MODE {
        (::windows::core::Vtable::vtable(self).base__.GetTextAntialiasMode)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(feature = "Win32_Graphics_DirectWrite")]
    pub unsafe fn SetTextRenderingParams<P0>(&self, textrenderingparams: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::DirectWrite::IDWriteRenderingParams>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetTextRenderingParams)(::windows::core::Vtable::as_raw(self), textrenderingparams.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(feature = "Win32_Graphics_DirectWrite")]
    pub unsafe fn GetTextRenderingParams(&self) -> ::windows::core::Result<super::DirectWrite::IDWriteRenderingParams> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetTextRenderingParams)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <super::DirectWrite::IDWriteRenderingParams as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn SetTags(&self, tag1: u64, tag2: u64) {
        (::windows::core::Vtable::vtable(self).base__.SetTags)(::windows::core::Vtable::as_raw(self), tag1, tag2)
    }
    pub unsafe fn GetTags(&self, tag1: ::core::option::Option<*mut u64>, tag2: ::core::option::Option<*mut u64>) {
        (::windows::core::Vtable::vtable(self).base__.GetTags)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(tag1.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(tag2.unwrap_or(::std::ptr::null_mut())))
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn PushLayer<P0>(&self, layerparameters: *const D2D1_LAYER_PARAMETERS, layer: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Layer>>,
    {
        (::windows::core::Vtable::vtable(self).base__.PushLayer)(::windows::core::Vtable::as_raw(self), layerparameters, layer.into().abi())
    }
    pub unsafe fn PopLayer(&self) {
        (::windows::core::Vtable::vtable(self).base__.PopLayer)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn Flush(&self, tag1: ::core::option::Option<*mut u64>, tag2: ::core::option::Option<*mut u64>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Flush)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(tag1.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(tag2.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SaveDrawingState<P0>(&self, drawingstateblock: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1DrawingStateBlock>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SaveDrawingState)(::windows::core::Vtable::as_raw(self), drawingstateblock.into().abi())
    }
    pub unsafe fn RestoreDrawingState<P0>(&self, drawingstateblock: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1DrawingStateBlock>>,
    {
        (::windows::core::Vtable::vtable(self).base__.RestoreDrawingState)(::windows::core::Vtable::as_raw(self), drawingstateblock.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn PushAxisAlignedClip(&self, cliprect: *const Common::D2D_RECT_F, antialiasmode: D2D1_ANTIALIAS_MODE) {
        (::windows::core::Vtable::vtable(self).base__.PushAxisAlignedClip)(::windows::core::Vtable::as_raw(self), cliprect, antialiasmode)
    }
    pub unsafe fn PopAxisAlignedClip(&self) {
        (::windows::core::Vtable::vtable(self).base__.PopAxisAlignedClip)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn Clear(&self, clearcolor: ::core::option::Option<*const Common::D2D1_COLOR_F>) {
        (::windows::core::Vtable::vtable(self).base__.Clear)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(clearcolor.unwrap_or(::std::ptr::null())))
    }
    pub unsafe fn BeginDraw(&self) {
        (::windows::core::Vtable::vtable(self).base__.BeginDraw)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn EndDraw(&self, tag1: ::core::option::Option<*mut u64>, tag2: ::core::option::Option<*mut u64>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.EndDraw)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(tag1.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(tag2.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetPixelFormat(&self) -> Common::D2D1_PIXEL_FORMAT {
        let mut result__: Common::D2D1_PIXEL_FORMAT = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetPixelFormat)(::windows::core::Vtable::as_raw(self), &mut result__);
        result__
    }
    pub unsafe fn SetDpi(&self, dpix: f32, dpiy: f32) {
        (::windows::core::Vtable::vtable(self).base__.SetDpi)(::windows::core::Vtable::as_raw(self), dpix, dpiy)
    }
    pub unsafe fn GetDpi(&self, dpix: *mut f32, dpiy: *mut f32) {
        (::windows::core::Vtable::vtable(self).base__.GetDpi)(::windows::core::Vtable::as_raw(self), dpix, dpiy)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetSize(&self) -> Common::D2D_SIZE_F {
        let mut result__: Common::D2D_SIZE_F = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetSize)(::windows::core::Vtable::as_raw(self), &mut result__);
        result__
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetPixelSize(&self) -> Common::D2D_SIZE_U {
        let mut result__: Common::D2D_SIZE_U = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetPixelSize)(::windows::core::Vtable::as_raw(self), &mut result__);
        result__
    }
    pub unsafe fn GetMaximumBitmapSize(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.GetMaximumBitmapSize)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn IsSupported(&self, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.IsSupported)(::windows::core::Vtable::as_raw(self), rendertargetproperties)
    }
}
impl ::core::cmp::PartialEq for ID2D1Device {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1Device {}
impl ::core::fmt::Debug for ID2D1Device {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1Device").field(&self.0).finish()
    }
}
impl ID2D1Device {
    pub unsafe fn GetFactory(&self) -> ::windows::core::Result<ID2D1Factory> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFactory)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1Factory as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
}
impl ::core::cmp::PartialEq for ID2D1Device1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1Device1 {}
impl ::core::fmt::Debug for ID2D1Device1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1Device1").field(&self.0).finish()
    }
}
impl ID2D1Device1 {
    pub unsafe fn GetFactory(&self) -> ::windows::core::Result<ID2D1Factory> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetFactory)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1Factory as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn CreateDeviceContext(&self, options: D2D1_DEVICE_CONTEXT_OPTIONS) -> ::windows::core::Result<ID2D1DeviceContext> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateDeviceContext)(::windows::core::Vtable::as_raw(self), options, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Imaging\"`, `\"Win32_Storage_Xps_Printing\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Imaging", feature = "Win32_Storage_Xps_Printing"))]
    pub unsafe fn CreatePrintControl<P0, P1>(&self, wicfactory: P0, documenttarget: P1, printcontrolproperties: ::core::option::Option<*const D2D1_PRINT_CONTROL_PROPERTIES>) -> ::windows::core::Result<ID2D1PrintControl>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Imaging::IWICImagingFactory>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::super::Storage::Xps::Printing::IPrintDocumentPackageTarget>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreatePrintControl)(::windows::core::Vtable::as_raw(self), wicfactory.into().abi(), documenttarget.into().abi(), ::core::mem::transmute(printcontrolproperties.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetMaximumTextureMemory(&self, maximuminbytes: u64) {
        (::windows::core::Vtable::vtable(self).base__.SetMaximumTextureMemory)(::windows::core::Vtable::as_raw(self), maximuminbytes)
    }
    pub unsafe fn GetMaximumTextureMemory(&self) -> u64 {
        (::windows::core::Vtable::vtable(self).base__.GetMaximumTextureMemory)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn ClearResources(&self, millisecondssinceuse: u32) {
        (::windows::core::Vtable::vtable(self).base__.ClearResources)(::windows::core::Vtable::as_raw(self), millisecondssinceuse)
    }
}
impl ::core::cmp::PartialEq for ID2D1Device2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1Device2 {}
impl ::core::fmt::Debug for ID2D1Device2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1Device2").field(&self.0).finish()
    }
}
impl ID2D1Device2 {
    pub unsafe fn GetFactory(&self) -> ::windows::core::Result<ID2D1Factory> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetFactory)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1Factory as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn CreateDeviceContext(&self, options: D2D1_DEVICE_CONTEXT_OPTIONS) -> ::windows::core::Result<ID2D1DeviceContext> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateDeviceContext)(::windows::core::Vtable::as_raw(self), options, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Imaging\"`, `\"Win32_Storage_Xps_Printing\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Imaging", feature = "Win32_Storage_Xps_Printing"))]
    pub unsafe fn CreatePrintControl<P0, P1>(&self, wicfactory: P0, documenttarget: P1, printcontrolproperties: ::core::option::Option<*const D2D1_PRINT_CONTROL_PROPERTIES>) -> ::windows::core::Result<ID2D1PrintControl>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Imaging::IWICImagingFactory>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::super::Storage::Xps::Printing::IPrintDocumentPackageTarget>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreatePrintControl)(::windows::core::Vtable::as_raw(self), wicfactory.into().abi(), documenttarget.into().abi(), ::core::mem::transmute(printcontrolproperties.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetMaximumTextureMemory(&self, maximuminbytes: u64) {
        (::windows::core::Vtable::vtable(self).base__.base__.SetMaximumTextureMemory)(::windows::core::Vtable::as_raw(self), maximuminbytes)
    }
    pub unsafe fn GetMaximumTextureMemory(&self) -> u64 {
        (::windows::core::Vtable::vtable(self).base__.base__.GetMaximumTextureMemory)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn ClearResources(&self, millisecondssinceuse: u32) {
        (::windows::core::Vtable::vtable(self).base__.base__.ClearResources)(::windows::core::Vtable::as_raw(self), millisecondssinceuse)
    }
    pub unsafe fn GetRenderingPriority(&self) -> D2D1_RENDERING_PRIORITY {
        (::windows::core::Vtable::vtable(self).base__.GetRenderingPriority)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn SetRenderingPriority(&self, renderingpriority: D2D1_RENDERING_PRIORITY) {
        (::windows::core::Vtable::vtable(self).base__.SetRenderingPriority)(::windows::core::Vtable::as_raw(self), renderingpriority)
    }
    pub unsafe fn CreateDeviceContext2(&self, options: D2D1_DEVICE_CONTEXT_OPTIONS) -> ::windows::core::Result<ID2D1DeviceContext1> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateDeviceContext2)(::windows::core::Vtable::as_raw(self), options, result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for ID2D1Device3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1Device3 {}
impl ::core::fmt::Debug for ID2D1Device3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1Device3").field(&self.0).finish()
    }
}
impl ID2D1Device3 {
    pub unsafe fn GetFactory(&self) -> ::windows::core::Result<ID2D1Factory> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetFactory)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1Factory as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn CreateDeviceContext(&self, options: D2D1_DEVICE_CONTEXT_OPTIONS) -> ::windows::core::Result<ID2D1DeviceContext> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateDeviceContext)(::windows::core::Vtable::as_raw(self), options, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Imaging\"`, `\"Win32_Storage_Xps_Printing\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Imaging", feature = "Win32_Storage_Xps_Printing"))]
    pub unsafe fn CreatePrintControl<P0, P1>(&self, wicfactory: P0, documenttarget: P1, printcontrolproperties: ::core::option::Option<*const D2D1_PRINT_CONTROL_PROPERTIES>) -> ::windows::core::Result<ID2D1PrintControl>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Imaging::IWICImagingFactory>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::super::Storage::Xps::Printing::IPrintDocumentPackageTarget>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreatePrintControl)(::windows::core::Vtable::as_raw(self), wicfactory.into().abi(), documenttarget.into().abi(), ::core::mem::transmute(printcontrolproperties.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetMaximumTextureMemory(&self, maximuminbytes: u64) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetMaximumTextureMemory)(::windows::core::Vtable::as_raw(self), maximuminbytes)
    }
    pub unsafe fn GetMaximumTextureMemory(&self) -> u64 {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetMaximumTextureMemory)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn ClearResources(&self, millisecondssinceuse: u32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ClearResources)(::windows::core::Vtable::as_raw(self), millisecondssinceuse)
    }
    pub unsafe fn GetRenderingPriority(&self) -> D2D1_RENDERING_PRIORITY {
        (::windows::core::Vtable::vtable(self).base__.base__.GetRenderingPriority)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn SetRenderingPriority(&self, renderingpriority: D2D1_RENDERING_PRIORITY) {
        (::windows::core::Vtable::vtable(self).base__.base__.SetRenderingPriority)(::windows::core::Vtable::as_raw(self), renderingpriority)
    }
    pub unsafe fn CreateDeviceContext2(&self, options: D2D1_DEVICE_CONTEXT_OPTIONS) -> ::windows::core::Result<ID2D1DeviceContext1> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateDeviceContext2)(::windows::core::Vtable::as_raw(self), options, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateDeviceContext3(&self, options: D2D1_DEVICE_CONTEXT_OPTIONS) -> ::windows::core::Result<ID2D1DeviceContext2> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateDeviceContext3)(::windows::core::Vtable::as_raw(self), options, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FlushDeviceContexts<P0>(&self, bitmap: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Bitmap>>,
    {
        (::windows::core::Vtable::vtable(self).base__.FlushDeviceContexts)(::windows::core::Vtable::as_raw(self), bitmap.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn GetDxgiDevice(&self) -> ::windows::core::Result<super::Dxgi::IDXGIDevice> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDxgiDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for ID2D1Device4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1Device4 {}
impl ::core::fmt::Debug for ID2D1Device4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1Device4").field(&self.0).finish()
    }
}
impl ID2D1Device4 {
    pub unsafe fn GetFactory(&self) -> ::windows::core::Result<ID2D1Factory> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetFactory)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1Factory as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn CreateDeviceContext(&self, options: D2D1_DEVICE_CONTEXT_OPTIONS) -> ::windows::core::Result<ID2D1DeviceContext> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateDeviceContext)(::windows::core::Vtable::as_raw(self), options, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Imaging\"`, `\"Win32_Storage_Xps_Printing\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Imaging", feature = "Win32_Storage_Xps_Printing"))]
    pub unsafe fn CreatePrintControl<P0, P1>(&self, wicfactory: P0, documenttarget: P1, printcontrolproperties: ::core::option::Option<*const D2D1_PRINT_CONTROL_PROPERTIES>) -> ::windows::core::Result<ID2D1PrintControl>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Imaging::IWICImagingFactory>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::super::Storage::Xps::Printing::IPrintDocumentPackageTarget>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreatePrintControl)(::windows::core::Vtable::as_raw(self), wicfactory.into().abi(), documenttarget.into().abi(), ::core::mem::transmute(printcontrolproperties.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetMaximumTextureMemory(&self, maximuminbytes: u64) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetMaximumTextureMemory)(::windows::core::Vtable::as_raw(self), maximuminbytes)
    }
    pub unsafe fn GetMaximumTextureMemory(&self) -> u64 {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetMaximumTextureMemory)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn ClearResources(&self, millisecondssinceuse: u32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.ClearResources)(::windows::core::Vtable::as_raw(self), millisecondssinceuse)
    }
    pub unsafe fn GetRenderingPriority(&self) -> D2D1_RENDERING_PRIORITY {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetRenderingPriority)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn SetRenderingPriority(&self, renderingpriority: D2D1_RENDERING_PRIORITY) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetRenderingPriority)(::windows::core::Vtable::as_raw(self), renderingpriority)
    }
    pub unsafe fn CreateDeviceContext2(&self, options: D2D1_DEVICE_CONTEXT_OPTIONS) -> ::windows::core::Result<ID2D1DeviceContext1> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateDeviceContext2)(::windows::core::Vtable::as_raw(self), options, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateDeviceContext3(&self, options: D2D1_DEVICE_CONTEXT_OPTIONS) -> ::windows::core::Result<ID2D1DeviceContext2> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateDeviceContext3)(::windows::core::Vtable::as_raw(self), options, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FlushDeviceContexts<P0>(&self, bitmap: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Bitmap>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.FlushDeviceContexts)(::windows::core::Vtable::as_raw(self), bitmap.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn GetDxgiDevice(&self) -> ::windows::core::Result<super::Dxgi::IDXGIDevice> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetDxgiDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateDeviceContext4(&self, options: D2D1_DEVICE_CONTEXT_OPTIONS) -> ::windows::core::Result<ID2D1DeviceContext3> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateDeviceContext4)(::windows::core::Vtable::as_raw(self), options, result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for ID2D1Device5 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1Device5 {}
impl ::core::fmt::Debug for ID2D1Device5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1Device5").field(&self.0).finish()
    }
}
impl ID2D1Device5 {
    pub unsafe fn GetFactory(&self) -> ::windows::core::Result<ID2D1Factory> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.GetFactory)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1Factory as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn CreateDeviceContext(&self, options: D2D1_DEVICE_CONTEXT_OPTIONS) -> ::windows::core::Result<ID2D1DeviceContext> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateDeviceContext)(::windows::core::Vtable::as_raw(self), options, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Imaging\"`, `\"Win32_Storage_Xps_Printing\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Imaging", feature = "Win32_Storage_Xps_Printing"))]
    pub unsafe fn CreatePrintControl<P0, P1>(&self, wicfactory: P0, documenttarget: P1, printcontrolproperties: ::core::option::Option<*const D2D1_PRINT_CONTROL_PROPERTIES>) -> ::windows::core::Result<ID2D1PrintControl>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Imaging::IWICImagingFactory>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::super::Storage::Xps::Printing::IPrintDocumentPackageTarget>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreatePrintControl)(::windows::core::Vtable::as_raw(self), wicfactory.into().abi(), documenttarget.into().abi(), ::core::mem::transmute(printcontrolproperties.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetMaximumTextureMemory(&self, maximuminbytes: u64) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetMaximumTextureMemory)(::windows::core::Vtable::as_raw(self), maximuminbytes)
    }
    pub unsafe fn GetMaximumTextureMemory(&self) -> u64 {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetMaximumTextureMemory)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn ClearResources(&self, millisecondssinceuse: u32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.ClearResources)(::windows::core::Vtable::as_raw(self), millisecondssinceuse)
    }
    pub unsafe fn GetRenderingPriority(&self) -> D2D1_RENDERING_PRIORITY {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetRenderingPriority)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn SetRenderingPriority(&self, renderingpriority: D2D1_RENDERING_PRIORITY) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetRenderingPriority)(::windows::core::Vtable::as_raw(self), renderingpriority)
    }
    pub unsafe fn CreateDeviceContext2(&self, options: D2D1_DEVICE_CONTEXT_OPTIONS) -> ::windows::core::Result<ID2D1DeviceContext1> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateDeviceContext2)(::windows::core::Vtable::as_raw(self), options, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateDeviceContext3(&self, options: D2D1_DEVICE_CONTEXT_OPTIONS) -> ::windows::core::Result<ID2D1DeviceContext2> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateDeviceContext3)(::windows::core::Vtable::as_raw(self), options, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FlushDeviceContexts<P0>(&self, bitmap: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Bitmap>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.FlushDeviceContexts)(::windows::core::Vtable::as_raw(self), bitmap.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn GetDxgiDevice(&self) -> ::windows::core::Result<super::Dxgi::IDXGIDevice> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetDxgiDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateDeviceContext4(&self, options: D2D1_DEVICE_CONTEXT_OPTIONS) -> ::windows::core::Result<ID2D1DeviceContext3> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateDeviceContext4)(::windows::core::Vtable::as_raw(self), options, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateDeviceContext5(&self, options: D2D1_DEVICE_CONTEXT_OPTIONS) -> ::windows::core::Result<ID2D1DeviceContext4> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateDeviceContext5)(::windows::core::Vtable::as_raw(self), options, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetMaximumColorGlyphCacheMemory(&self, maximuminbytes: u64) {
        (::windows::core::Vtable::vtable(self).base__.SetMaximumColorGlyphCacheMemory)(::windows::core::Vtable::as_raw(self), maximuminbytes)
    }
    pub unsafe fn GetMaximumColorGlyphCacheMemory(&self) -> u64 {
        (::windows::core::Vtable::vtable(self).base__.GetMaximumColorGlyphCacheMemory)(::windows::core::Vtable::as_raw(self))
    }
}
impl ::core::cmp::PartialEq for ID2D1Device6 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1Device6 {}
impl ::core::fmt::Debug for ID2D1Device6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1Device6").field(&self.0).finish()
    }
}
impl ID2D1Device6 {
    pub unsafe fn GetFactory(&self) -> ::windows::core::Result<ID2D1Factory> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.GetFactory)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1Factory as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn CreateDeviceContext(&self, options: D2D1_DEVICE_CONTEXT_OPTIONS) -> ::windows::core::Result<ID2D1DeviceContext> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CreateDeviceContext)(::windows::core::Vtable::as_raw(self), options, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Imaging\"`, `\"Win32_Storage_Xps_Printing\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Imaging", feature = "Win32_Storage_Xps_Printing"))]
    pub unsafe fn CreatePrintControl<P0, P1>(&self, wicfactory: P0, documenttarget: P1, printcontrolproperties: ::core::option::Option<*const D2D1_PRINT_CONTROL_PROPERTIES>) -> ::windows::core::Result<ID2D1PrintControl>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Imaging::IWICImagingFactory>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::super::Storage::Xps::Printing::IPrintDocumentPackageTarget>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CreatePrintControl)(::windows::core::Vtable::as_raw(self), wicfactory.into().abi(), documenttarget.into().abi(), ::core::mem::transmute(printcontrolproperties.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetMaximumTextureMemory(&self, maximuminbytes: u64) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.SetMaximumTextureMemory)(::windows::core::Vtable::as_raw(self), maximuminbytes)
    }
    pub unsafe fn GetMaximumTextureMemory(&self) -> u64 {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.GetMaximumTextureMemory)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn ClearResources(&self, millisecondssinceuse: u32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.ClearResources)(::windows::core::Vtable::as_raw(self), millisecondssinceuse)
    }
    pub unsafe fn GetRenderingPriority(&self) -> D2D1_RENDERING_PRIORITY {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetRenderingPriority)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn SetRenderingPriority(&self, renderingpriority: D2D1_RENDERING_PRIORITY) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetRenderingPriority)(::windows::core::Vtable::as_raw(self), renderingpriority)
    }
    pub unsafe fn CreateDeviceContext2(&self, options: D2D1_DEVICE_CONTEXT_OPTIONS) -> ::windows::core::Result<ID2D1DeviceContext1> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateDeviceContext2)(::windows::core::Vtable::as_raw(self), options, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateDeviceContext3(&self, options: D2D1_DEVICE_CONTEXT_OPTIONS) -> ::windows::core::Result<ID2D1DeviceContext2> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateDeviceContext3)(::windows::core::Vtable::as_raw(self), options, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FlushDeviceContexts<P0>(&self, bitmap: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Bitmap>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.FlushDeviceContexts)(::windows::core::Vtable::as_raw(self), bitmap.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn GetDxgiDevice(&self) -> ::windows::core::Result<super::Dxgi::IDXGIDevice> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetDxgiDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateDeviceContext4(&self, options: D2D1_DEVICE_CONTEXT_OPTIONS) -> ::windows::core::Result<ID2D1DeviceContext3> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateDeviceContext4)(::windows::core::Vtable::as_raw(self), options, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateDeviceContext5(&self, options: D2D1_DEVICE_CONTEXT_OPTIONS) -> ::windows::core::Result<ID2D1DeviceContext4> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateDeviceContext5)(::windows::core::Vtable::as_raw(self), options, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetMaximumColorGlyphCacheMemory(&self, maximuminbytes: u64) {
        (::windows::core::Vtable::vtable(self).base__.base__.SetMaximumColorGlyphCacheMemory)(::windows::core::Vtable::as_raw(self), maximuminbytes)
    }
    pub unsafe fn GetMaximumColorGlyphCacheMemory(&self) -> u64 {
        (::windows::core::Vtable::vtable(self).base__.base__.GetMaximumColorGlyphCacheMemory)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn CreateDeviceContext6(&self, options: D2D1_DEVICE_CONTEXT_OPTIONS) -> ::windows::core::Result<ID2D1DeviceContext5> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateDeviceContext6)(::windows::core::Vtable::as_raw(self), options, result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for ID2D1DeviceContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1DeviceContext {}
impl ::core::fmt::Debug for ID2D1DeviceContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1DeviceContext").field(&self.0).finish()
    }
}
impl ID2D1DeviceContext {
    pub unsafe fn GetFactory(&self) -> ::windows::core::Result<ID2D1Factory> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetFactory)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1Factory as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateBitmap(&self, size: Common::D2D_SIZE_U, srcdata: ::core::option::Option<*const ::core::ffi::c_void>, pitch: u32, bitmapproperties: *const D2D1_BITMAP_PROPERTIES) -> ::windows::core::Result<ID2D1Bitmap> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateBitmap)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(size), ::core::mem::transmute(srcdata.unwrap_or(::std::ptr::null())), pitch, bitmapproperties, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Imaging\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
    pub unsafe fn CreateBitmapFromWicBitmap<P0>(&self, wicbitmapsource: P0, bitmapproperties: ::core::option::Option<*const D2D1_BITMAP_PROPERTIES>) -> ::windows::core::Result<ID2D1Bitmap>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Imaging::IWICBitmapSource>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateBitmapFromWicBitmap)(::windows::core::Vtable::as_raw(self), wicbitmapsource.into().abi(), ::core::mem::transmute(bitmapproperties.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSharedBitmap(&self, riid: *const ::windows::core::GUID, data: *mut ::core::ffi::c_void, bitmapproperties: ::core::option::Option<*const D2D1_BITMAP_PROPERTIES>, bitmap: *mut ::core::option::Option<ID2D1Bitmap>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CreateSharedBitmap)(::windows::core::Vtable::as_raw(self), riid, data, ::core::mem::transmute(bitmapproperties.unwrap_or(::std::ptr::null())), ::core::mem::transmute(bitmap)).ok()
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn CreateBitmapBrush<P0>(&self, bitmap: P0, bitmapbrushproperties: ::core::option::Option<*const D2D1_BITMAP_BRUSH_PROPERTIES>, brushproperties: ::core::option::Option<*const D2D1_BRUSH_PROPERTIES>) -> ::windows::core::Result<ID2D1BitmapBrush>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Bitmap>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateBitmapBrush)(::windows::core::Vtable::as_raw(self), bitmap.into().abi(), ::core::mem::transmute(bitmapbrushproperties.unwrap_or(::std::ptr::null())), ::core::mem::transmute(brushproperties.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn CreateSolidColorBrush(&self, color: *const Common::D2D1_COLOR_F, brushproperties: ::core::option::Option<*const D2D1_BRUSH_PROPERTIES>) -> ::windows::core::Result<ID2D1SolidColorBrush> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateSolidColorBrush)(::windows::core::Vtable::as_raw(self), color, ::core::mem::transmute(brushproperties.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CreateGradientStopCollection(&self, gradientstops: &[D2D1_GRADIENT_STOP], colorinterpolationgamma: D2D1_GAMMA, extendmode: D2D1_EXTEND_MODE) -> ::windows::core::Result<ID2D1GradientStopCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateGradientStopCollection)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(gradientstops.as_ptr()), gradientstops.len() as _, colorinterpolationgamma, extendmode, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn CreateLinearGradientBrush<P0>(&self, lineargradientbrushproperties: *const D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES, brushproperties: ::core::option::Option<*const D2D1_BRUSH_PROPERTIES>, gradientstopcollection: P0) -> ::windows::core::Result<ID2D1LinearGradientBrush>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1GradientStopCollection>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateLinearGradientBrush)(::windows::core::Vtable::as_raw(self), lineargradientbrushproperties, ::core::mem::transmute(brushproperties.unwrap_or(::std::ptr::null())), gradientstopcollection.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn CreateRadialGradientBrush<P0>(&self, radialgradientbrushproperties: *const D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES, brushproperties: ::core::option::Option<*const D2D1_BRUSH_PROPERTIES>, gradientstopcollection: P0) -> ::windows::core::Result<ID2D1RadialGradientBrush>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1GradientStopCollection>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateRadialGradientBrush)(::windows::core::Vtable::as_raw(self), radialgradientbrushproperties, ::core::mem::transmute(brushproperties.unwrap_or(::std::ptr::null())), gradientstopcollection.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateCompatibleRenderTarget(&self, desiredsize: ::core::option::Option<*const Common::D2D_SIZE_F>, desiredpixelsize: ::core::option::Option<*const Common::D2D_SIZE_U>, desiredformat: ::core::option::Option<*const Common::D2D1_PIXEL_FORMAT>, options: D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS) -> ::windows::core::Result<ID2D1BitmapRenderTarget> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateCompatibleRenderTarget)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(desiredsize.unwrap_or(::std::ptr::null())), ::core::mem::transmute(desiredpixelsize.unwrap_or(::std::ptr::null())), ::core::mem::transmute(desiredformat.unwrap_or(::std::ptr::null())), options, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CreateLayer(&self, size: ::core::option::Option<*const Common::D2D_SIZE_F>) -> ::windows::core::Result<ID2D1Layer> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateLayer)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(size.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateMesh(&self) -> ::windows::core::Result<ID2D1Mesh> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateMesh)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawLine<P0, P1>(&self, point0: Common::D2D_POINT_2F, point1: Common::D2D_POINT_2F, brush: P0, strokewidth: f32, strokestyle: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DrawLine)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(point0), ::core::mem::transmute(point1), brush.into().abi(), strokewidth, strokestyle.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawRectangle<P0, P1>(&self, rect: *const Common::D2D_RECT_F, brush: P0, strokewidth: f32, strokestyle: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DrawRectangle)(::windows::core::Vtable::as_raw(self), rect, brush.into().abi(), strokewidth, strokestyle.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn FillRectangle<P0>(&self, rect: *const Common::D2D_RECT_F, brush: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.FillRectangle)(::windows::core::Vtable::as_raw(self), rect, brush.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawRoundedRectangle<P0, P1>(&self, roundedrect: *const D2D1_ROUNDED_RECT, brush: P0, strokewidth: f32, strokestyle: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DrawRoundedRectangle)(::windows::core::Vtable::as_raw(self), roundedrect, brush.into().abi(), strokewidth, strokestyle.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn FillRoundedRectangle<P0>(&self, roundedrect: *const D2D1_ROUNDED_RECT, brush: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.FillRoundedRectangle)(::windows::core::Vtable::as_raw(self), roundedrect, brush.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawEllipse<P0, P1>(&self, ellipse: *const D2D1_ELLIPSE, brush: P0, strokewidth: f32, strokestyle: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DrawEllipse)(::windows::core::Vtable::as_raw(self), ellipse, brush.into().abi(), strokewidth, strokestyle.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn FillEllipse<P0>(&self, ellipse: *const D2D1_ELLIPSE, brush: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.FillEllipse)(::windows::core::Vtable::as_raw(self), ellipse, brush.into().abi())
    }
    pub unsafe fn DrawGeometry<P0, P1, P2>(&self, geometry: P0, brush: P1, strokewidth: f32, strokestyle: P2)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Geometry>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P2: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DrawGeometry)(::windows::core::Vtable::as_raw(self), geometry.into().abi(), brush.into().abi(), strokewidth, strokestyle.into().abi())
    }
    pub unsafe fn FillGeometry<P0, P1, P2>(&self, geometry: P0, brush: P1, opacitybrush: P2)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Geometry>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P2: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.FillGeometry)(::windows::core::Vtable::as_raw(self), geometry.into().abi(), brush.into().abi(), opacitybrush.into().abi())
    }
    pub unsafe fn FillMesh<P0, P1>(&self, mesh: P0, brush: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Mesh>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.FillMesh)(::windows::core::Vtable::as_raw(self), mesh.into().abi(), brush.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn FillOpacityMask<P0, P1>(&self, opacitymask: P0, brush: P1, content: D2D1_OPACITY_MASK_CONTENT, destinationrectangle: ::core::option::Option<*const Common::D2D_RECT_F>, sourcerectangle: ::core::option::Option<*const Common::D2D_RECT_F>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Bitmap>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.FillOpacityMask)(::windows::core::Vtable::as_raw(self), opacitymask.into().abi(), brush.into().abi(), content, ::core::mem::transmute(destinationrectangle.unwrap_or(::std::ptr::null())), ::core::mem::transmute(sourcerectangle.unwrap_or(::std::ptr::null())))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawBitmap<P0>(&self, bitmap: P0, destinationrectangle: ::core::option::Option<*const Common::D2D_RECT_F>, opacity: f32, interpolationmode: D2D1_BITMAP_INTERPOLATION_MODE, sourcerectangle: ::core::option::Option<*const Common::D2D_RECT_F>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Bitmap>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DrawBitmap)(::windows::core::Vtable::as_raw(self), bitmap.into().abi(), ::core::mem::transmute(destinationrectangle.unwrap_or(::std::ptr::null())), opacity, interpolationmode, ::core::mem::transmute(sourcerectangle.unwrap_or(::std::ptr::null())))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn DrawText<P0, P1>(&self, string: &[u16], textformat: P0, layoutrect: *const Common::D2D_RECT_F, defaultfillbrush: P1, options: D2D1_DRAW_TEXT_OPTIONS, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::DirectWrite::IDWriteTextFormat>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DrawText)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(string.as_ptr()), string.len() as _, textformat.into().abi(), layoutrect, defaultfillbrush.into().abi(), options, measuringmode)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn DrawTextLayout<P0, P1>(&self, origin: Common::D2D_POINT_2F, textlayout: P0, defaultfillbrush: P1, options: D2D1_DRAW_TEXT_OPTIONS)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::DirectWrite::IDWriteTextLayout>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DrawTextLayout)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(origin), textlayout.into().abi(), defaultfillbrush.into().abi(), options)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn DrawGlyphRun<P0>(&self, baselineorigin: Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, foregroundbrush: P0, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DrawGlyphRun)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(baselineorigin), glyphrun, foregroundbrush.into().abi(), measuringmode)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn SetTransform(&self, transform: *const super::super::super::Foundation::Numerics::Matrix3x2) {
        (::windows::core::Vtable::vtable(self).base__.SetTransform)(::windows::core::Vtable::as_raw(self), transform)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn GetTransform(&self, transform: *mut super::super::super::Foundation::Numerics::Matrix3x2) {
        (::windows::core::Vtable::vtable(self).base__.GetTransform)(::windows::core::Vtable::as_raw(self), transform)
    }
    pub unsafe fn SetAntialiasMode(&self, antialiasmode: D2D1_ANTIALIAS_MODE) {
        (::windows::core::Vtable::vtable(self).base__.SetAntialiasMode)(::windows::core::Vtable::as_raw(self), antialiasmode)
    }
    pub unsafe fn GetAntialiasMode(&self) -> D2D1_ANTIALIAS_MODE {
        (::windows::core::Vtable::vtable(self).base__.GetAntialiasMode)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn SetTextAntialiasMode(&self, textantialiasmode: D2D1_TEXT_ANTIALIAS_MODE) {
        (::windows::core::Vtable::vtable(self).base__.SetTextAntialiasMode)(::windows::core::Vtable::as_raw(self), textantialiasmode)
    }
    pub unsafe fn GetTextAntialiasMode(&self) -> D2D1_TEXT_ANTIALIAS_MODE {
        (::windows::core::Vtable::vtable(self).base__.GetTextAntialiasMode)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(feature = "Win32_Graphics_DirectWrite")]
    pub unsafe fn SetTextRenderingParams<P0>(&self, textrenderingparams: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::DirectWrite::IDWriteRenderingParams>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetTextRenderingParams)(::windows::core::Vtable::as_raw(self), textrenderingparams.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(feature = "Win32_Graphics_DirectWrite")]
    pub unsafe fn GetTextRenderingParams(&self) -> ::windows::core::Result<super::DirectWrite::IDWriteRenderingParams> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetTextRenderingParams)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <super::DirectWrite::IDWriteRenderingParams as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn SetTags(&self, tag1: u64, tag2: u64) {
        (::windows::core::Vtable::vtable(self).base__.SetTags)(::windows::core::Vtable::as_raw(self), tag1, tag2)
    }
    pub unsafe fn GetTags(&self, tag1: ::core::option::Option<*mut u64>, tag2: ::core::option::Option<*mut u64>) {
        (::windows::core::Vtable::vtable(self).base__.GetTags)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(tag1.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(tag2.unwrap_or(::std::ptr::null_mut())))
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn PushLayer<P0>(&self, layerparameters: *const D2D1_LAYER_PARAMETERS, layer: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Layer>>,
    {
        (::windows::core::Vtable::vtable(self).base__.PushLayer)(::windows::core::Vtable::as_raw(self), layerparameters, layer.into().abi())
    }
    pub unsafe fn PopLayer(&self) {
        (::windows::core::Vtable::vtable(self).base__.PopLayer)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn Flush(&self, tag1: ::core::option::Option<*mut u64>, tag2: ::core::option::Option<*mut u64>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Flush)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(tag1.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(tag2.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SaveDrawingState<P0>(&self, drawingstateblock: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1DrawingStateBlock>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SaveDrawingState)(::windows::core::Vtable::as_raw(self), drawingstateblock.into().abi())
    }
    pub unsafe fn RestoreDrawingState<P0>(&self, drawingstateblock: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1DrawingStateBlock>>,
    {
        (::windows::core::Vtable::vtable(self).base__.RestoreDrawingState)(::windows::core::Vtable::as_raw(self), drawingstateblock.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn PushAxisAlignedClip(&self, cliprect: *const Common::D2D_RECT_F, antialiasmode: D2D1_ANTIALIAS_MODE) {
        (::windows::core::Vtable::vtable(self).base__.PushAxisAlignedClip)(::windows::core::Vtable::as_raw(self), cliprect, antialiasmode)
    }
    pub unsafe fn PopAxisAlignedClip(&self) {
        (::windows::core::Vtable::vtable(self).base__.PopAxisAlignedClip)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn Clear(&self, clearcolor: ::core::option::Option<*const Common::D2D1_COLOR_F>) {
        (::windows::core::Vtable::vtable(self).base__.Clear)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(clearcolor.unwrap_or(::std::ptr::null())))
    }
    pub unsafe fn BeginDraw(&self) {
        (::windows::core::Vtable::vtable(self).base__.BeginDraw)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn EndDraw(&self, tag1: ::core::option::Option<*mut u64>, tag2: ::core::option::Option<*mut u64>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.EndDraw)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(tag1.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(tag2.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetPixelFormat(&self) -> Common::D2D1_PIXEL_FORMAT {
        let mut result__: Common::D2D1_PIXEL_FORMAT = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetPixelFormat)(::windows::core::Vtable::as_raw(self), &mut result__);
        result__
    }
    pub unsafe fn SetDpi(&self, dpix: f32, dpiy: f32) {
        (::windows::core::Vtable::vtable(self).base__.SetDpi)(::windows::core::Vtable::as_raw(self), dpix, dpiy)
    }
    pub unsafe fn GetDpi(&self, dpix: *mut f32, dpiy: *mut f32) {
        (::windows::core::Vtable::vtable(self).base__.GetDpi)(::windows::core::Vtable::as_raw(self), dpix, dpiy)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetSize(&self) -> Common::D2D_SIZE_F {
        let mut result__: Common::D2D_SIZE_F = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetSize)(::windows::core::Vtable::as_raw(self), &mut result__);
        result__
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetPixelSize(&self) -> Common::D2D_SIZE_U {
        let mut result__: Common::D2D_SIZE_U = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetPixelSize)(::windows::core::Vtable::as_raw(self), &mut result__);
        result__
    }
    pub unsafe fn GetMaximumBitmapSize(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.GetMaximumBitmapSize)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn IsSupported(&self, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.IsSupported)(::windows::core::Vtable::as_raw(self), rendertargetproperties)
    }
}
impl ::core::cmp::PartialEq for ID2D1DeviceContext1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1DeviceContext1 {}
impl ::core::fmt::Debug for ID2D1DeviceContext1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1DeviceContext1").field(&self.0).finish()
    }
}
impl ID2D1DeviceContext1 {
    pub unsafe fn GetFactory(&self) -> ::windows::core::Result<ID2D1Factory> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetFactory)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1Factory as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateBitmap(&self, size: Common::D2D_SIZE_U, srcdata: ::core::option::Option<*const ::core::ffi::c_void>, pitch: u32, bitmapproperties: *const D2D1_BITMAP_PROPERTIES) -> ::windows::core::Result<ID2D1Bitmap> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateBitmap)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(size), ::core::mem::transmute(srcdata.unwrap_or(::std::ptr::null())), pitch, bitmapproperties, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Imaging\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
    pub unsafe fn CreateBitmapFromWicBitmap<P0>(&self, wicbitmapsource: P0, bitmapproperties: ::core::option::Option<*const D2D1_BITMAP_PROPERTIES>) -> ::windows::core::Result<ID2D1Bitmap>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Imaging::IWICBitmapSource>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateBitmapFromWicBitmap)(::windows::core::Vtable::as_raw(self), wicbitmapsource.into().abi(), ::core::mem::transmute(bitmapproperties.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSharedBitmap(&self, riid: *const ::windows::core::GUID, data: *mut ::core::ffi::c_void, bitmapproperties: ::core::option::Option<*const D2D1_BITMAP_PROPERTIES>, bitmap: *mut ::core::option::Option<ID2D1Bitmap>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.CreateSharedBitmap)(::windows::core::Vtable::as_raw(self), riid, data, ::core::mem::transmute(bitmapproperties.unwrap_or(::std::ptr::null())), ::core::mem::transmute(bitmap)).ok()
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn CreateBitmapBrush<P0>(&self, bitmap: P0, bitmapbrushproperties: ::core::option::Option<*const D2D1_BITMAP_BRUSH_PROPERTIES>, brushproperties: ::core::option::Option<*const D2D1_BRUSH_PROPERTIES>) -> ::windows::core::Result<ID2D1BitmapBrush>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Bitmap>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateBitmapBrush)(::windows::core::Vtable::as_raw(self), bitmap.into().abi(), ::core::mem::transmute(bitmapbrushproperties.unwrap_or(::std::ptr::null())), ::core::mem::transmute(brushproperties.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn CreateSolidColorBrush(&self, color: *const Common::D2D1_COLOR_F, brushproperties: ::core::option::Option<*const D2D1_BRUSH_PROPERTIES>) -> ::windows::core::Result<ID2D1SolidColorBrush> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateSolidColorBrush)(::windows::core::Vtable::as_raw(self), color, ::core::mem::transmute(brushproperties.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CreateGradientStopCollection(&self, gradientstops: &[D2D1_GRADIENT_STOP], colorinterpolationgamma: D2D1_GAMMA, extendmode: D2D1_EXTEND_MODE) -> ::windows::core::Result<ID2D1GradientStopCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateGradientStopCollection)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(gradientstops.as_ptr()), gradientstops.len() as _, colorinterpolationgamma, extendmode, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn CreateLinearGradientBrush<P0>(&self, lineargradientbrushproperties: *const D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES, brushproperties: ::core::option::Option<*const D2D1_BRUSH_PROPERTIES>, gradientstopcollection: P0) -> ::windows::core::Result<ID2D1LinearGradientBrush>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1GradientStopCollection>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateLinearGradientBrush)(::windows::core::Vtable::as_raw(self), lineargradientbrushproperties, ::core::mem::transmute(brushproperties.unwrap_or(::std::ptr::null())), gradientstopcollection.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn CreateRadialGradientBrush<P0>(&self, radialgradientbrushproperties: *const D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES, brushproperties: ::core::option::Option<*const D2D1_BRUSH_PROPERTIES>, gradientstopcollection: P0) -> ::windows::core::Result<ID2D1RadialGradientBrush>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1GradientStopCollection>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateRadialGradientBrush)(::windows::core::Vtable::as_raw(self), radialgradientbrushproperties, ::core::mem::transmute(brushproperties.unwrap_or(::std::ptr::null())), gradientstopcollection.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateCompatibleRenderTarget(&self, desiredsize: ::core::option::Option<*const Common::D2D_SIZE_F>, desiredpixelsize: ::core::option::Option<*const Common::D2D_SIZE_U>, desiredformat: ::core::option::Option<*const Common::D2D1_PIXEL_FORMAT>, options: D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS) -> ::windows::core::Result<ID2D1BitmapRenderTarget> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateCompatibleRenderTarget)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(desiredsize.unwrap_or(::std::ptr::null())), ::core::mem::transmute(desiredpixelsize.unwrap_or(::std::ptr::null())), ::core::mem::transmute(desiredformat.unwrap_or(::std::ptr::null())), options, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CreateLayer(&self, size: ::core::option::Option<*const Common::D2D_SIZE_F>) -> ::windows::core::Result<ID2D1Layer> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateLayer)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(size.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateMesh(&self) -> ::windows::core::Result<ID2D1Mesh> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateMesh)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawLine<P0, P1>(&self, point0: Common::D2D_POINT_2F, point1: Common::D2D_POINT_2F, brush: P0, strokewidth: f32, strokestyle: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.DrawLine)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(point0), ::core::mem::transmute(point1), brush.into().abi(), strokewidth, strokestyle.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawRectangle<P0, P1>(&self, rect: *const Common::D2D_RECT_F, brush: P0, strokewidth: f32, strokestyle: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.DrawRectangle)(::windows::core::Vtable::as_raw(self), rect, brush.into().abi(), strokewidth, strokestyle.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn FillRectangle<P0>(&self, rect: *const Common::D2D_RECT_F, brush: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.FillRectangle)(::windows::core::Vtable::as_raw(self), rect, brush.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawRoundedRectangle<P0, P1>(&self, roundedrect: *const D2D1_ROUNDED_RECT, brush: P0, strokewidth: f32, strokestyle: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.DrawRoundedRectangle)(::windows::core::Vtable::as_raw(self), roundedrect, brush.into().abi(), strokewidth, strokestyle.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn FillRoundedRectangle<P0>(&self, roundedrect: *const D2D1_ROUNDED_RECT, brush: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.FillRoundedRectangle)(::windows::core::Vtable::as_raw(self), roundedrect, brush.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawEllipse<P0, P1>(&self, ellipse: *const D2D1_ELLIPSE, brush: P0, strokewidth: f32, strokestyle: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.DrawEllipse)(::windows::core::Vtable::as_raw(self), ellipse, brush.into().abi(), strokewidth, strokestyle.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn FillEllipse<P0>(&self, ellipse: *const D2D1_ELLIPSE, brush: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.FillEllipse)(::windows::core::Vtable::as_raw(self), ellipse, brush.into().abi())
    }
    pub unsafe fn DrawGeometry<P0, P1, P2>(&self, geometry: P0, brush: P1, strokewidth: f32, strokestyle: P2)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Geometry>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P2: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.DrawGeometry)(::windows::core::Vtable::as_raw(self), geometry.into().abi(), brush.into().abi(), strokewidth, strokestyle.into().abi())
    }
    pub unsafe fn FillGeometry<P0, P1, P2>(&self, geometry: P0, brush: P1, opacitybrush: P2)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Geometry>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P2: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.FillGeometry)(::windows::core::Vtable::as_raw(self), geometry.into().abi(), brush.into().abi(), opacitybrush.into().abi())
    }
    pub unsafe fn FillMesh<P0, P1>(&self, mesh: P0, brush: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Mesh>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.FillMesh)(::windows::core::Vtable::as_raw(self), mesh.into().abi(), brush.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn FillOpacityMask<P0, P1>(&self, opacitymask: P0, brush: P1, content: D2D1_OPACITY_MASK_CONTENT, destinationrectangle: ::core::option::Option<*const Common::D2D_RECT_F>, sourcerectangle: ::core::option::Option<*const Common::D2D_RECT_F>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Bitmap>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.FillOpacityMask)(::windows::core::Vtable::as_raw(self), opacitymask.into().abi(), brush.into().abi(), content, ::core::mem::transmute(destinationrectangle.unwrap_or(::std::ptr::null())), ::core::mem::transmute(sourcerectangle.unwrap_or(::std::ptr::null())))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawBitmap<P0>(&self, bitmap: P0, destinationrectangle: ::core::option::Option<*const Common::D2D_RECT_F>, opacity: f32, interpolationmode: D2D1_BITMAP_INTERPOLATION_MODE, sourcerectangle: ::core::option::Option<*const Common::D2D_RECT_F>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Bitmap>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.DrawBitmap)(::windows::core::Vtable::as_raw(self), bitmap.into().abi(), ::core::mem::transmute(destinationrectangle.unwrap_or(::std::ptr::null())), opacity, interpolationmode, ::core::mem::transmute(sourcerectangle.unwrap_or(::std::ptr::null())))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn DrawText<P0, P1>(&self, string: &[u16], textformat: P0, layoutrect: *const Common::D2D_RECT_F, defaultfillbrush: P1, options: D2D1_DRAW_TEXT_OPTIONS, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::DirectWrite::IDWriteTextFormat>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.DrawText)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(string.as_ptr()), string.len() as _, textformat.into().abi(), layoutrect, defaultfillbrush.into().abi(), options, measuringmode)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn DrawTextLayout<P0, P1>(&self, origin: Common::D2D_POINT_2F, textlayout: P0, defaultfillbrush: P1, options: D2D1_DRAW_TEXT_OPTIONS)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::DirectWrite::IDWriteTextLayout>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.DrawTextLayout)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(origin), textlayout.into().abi(), defaultfillbrush.into().abi(), options)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn DrawGlyphRun<P0>(&self, baselineorigin: Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, foregroundbrush: P0, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.DrawGlyphRun)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(baselineorigin), glyphrun, foregroundbrush.into().abi(), measuringmode)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn SetTransform(&self, transform: *const super::super::super::Foundation::Numerics::Matrix3x2) {
        (::windows::core::Vtable::vtable(self).base__.base__.SetTransform)(::windows::core::Vtable::as_raw(self), transform)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn GetTransform(&self, transform: *mut super::super::super::Foundation::Numerics::Matrix3x2) {
        (::windows::core::Vtable::vtable(self).base__.base__.GetTransform)(::windows::core::Vtable::as_raw(self), transform)
    }
    pub unsafe fn SetAntialiasMode(&self, antialiasmode: D2D1_ANTIALIAS_MODE) {
        (::windows::core::Vtable::vtable(self).base__.base__.SetAntialiasMode)(::windows::core::Vtable::as_raw(self), antialiasmode)
    }
    pub unsafe fn GetAntialiasMode(&self) -> D2D1_ANTIALIAS_MODE {
        (::windows::core::Vtable::vtable(self).base__.base__.GetAntialiasMode)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn SetTextAntialiasMode(&self, textantialiasmode: D2D1_TEXT_ANTIALIAS_MODE) {
        (::windows::core::Vtable::vtable(self).base__.base__.SetTextAntialiasMode)(::windows::core::Vtable::as_raw(self), textantialiasmode)
    }
    pub unsafe fn GetTextAntialiasMode(&self) -> D2D1_TEXT_ANTIALIAS_MODE {
        (::windows::core::Vtable::vtable(self).base__.base__.GetTextAntialiasMode)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(feature = "Win32_Graphics_DirectWrite")]
    pub unsafe fn SetTextRenderingParams<P0>(&self, textrenderingparams: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::DirectWrite::IDWriteRenderingParams>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetTextRenderingParams)(::windows::core::Vtable::as_raw(self), textrenderingparams.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(feature = "Win32_Graphics_DirectWrite")]
    pub unsafe fn GetTextRenderingParams(&self) -> ::windows::core::Result<super::DirectWrite::IDWriteRenderingParams> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetTextRenderingParams)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <super::DirectWrite::IDWriteRenderingParams as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn SetTags(&self, tag1: u64, tag2: u64) {
        (::windows::core::Vtable::vtable(self).base__.base__.SetTags)(::windows::core::Vtable::as_raw(self), tag1, tag2)
    }
    pub unsafe fn GetTags(&self, tag1: ::core::option::Option<*mut u64>, tag2: ::core::option::Option<*mut u64>) {
        (::windows::core::Vtable::vtable(self).base__.base__.GetTags)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(tag1.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(tag2.unwrap_or(::std::ptr::null_mut())))
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn PushLayer<P0>(&self, layerparameters: *const D2D1_LAYER_PARAMETERS, layer: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Layer>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.PushLayer)(::windows::core::Vtable::as_raw(self), layerparameters, layer.into().abi())
    }
    pub unsafe fn PopLayer(&self) {
        (::windows::core::Vtable::vtable(self).base__.base__.PopLayer)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn Flush(&self, tag1: ::core::option::Option<*mut u64>, tag2: ::core::option::Option<*mut u64>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.Flush)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(tag1.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(tag2.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SaveDrawingState<P0>(&self, drawingstateblock: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1DrawingStateBlock>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SaveDrawingState)(::windows::core::Vtable::as_raw(self), drawingstateblock.into().abi())
    }
    pub unsafe fn RestoreDrawingState<P0>(&self, drawingstateblock: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1DrawingStateBlock>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.RestoreDrawingState)(::windows::core::Vtable::as_raw(self), drawingstateblock.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn PushAxisAlignedClip(&self, cliprect: *const Common::D2D_RECT_F, antialiasmode: D2D1_ANTIALIAS_MODE) {
        (::windows::core::Vtable::vtable(self).base__.base__.PushAxisAlignedClip)(::windows::core::Vtable::as_raw(self), cliprect, antialiasmode)
    }
    pub unsafe fn PopAxisAlignedClip(&self) {
        (::windows::core::Vtable::vtable(self).base__.base__.PopAxisAlignedClip)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn Clear(&self, clearcolor: ::core::option::Option<*const Common::D2D1_COLOR_F>) {
        (::windows::core::Vtable::vtable(self).base__.base__.Clear)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(clearcolor.unwrap_or(::std::ptr::null())))
    }
    pub unsafe fn BeginDraw(&self) {
        (::windows::core::Vtable::vtable(self).base__.base__.BeginDraw)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn EndDraw(&self, tag1: ::core::option::Option<*mut u64>, tag2: ::core::option::Option<*mut u64>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.EndDraw)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(tag1.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(tag2.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetPixelFormat(&self) -> Common::D2D1_PIXEL_FORMAT {
        let mut result__: Common::D2D1_PIXEL_FORMAT = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetPixelFormat)(::windows::core::Vtable::as_raw(self), &mut result__);
        result__
    }
    pub unsafe fn SetDpi(&self, dpix: f32, dpiy: f32) {
        (::windows::core::Vtable::vtable(self).base__.base__.SetDpi)(::windows::core::Vtable::as_raw(self), dpix, dpiy)
    }
    pub unsafe fn GetDpi(&self, dpix: *mut f32, dpiy: *mut f32) {
        (::windows::core::Vtable::vtable(self).base__.base__.GetDpi)(::windows::core::Vtable::as_raw(self), dpix, dpiy)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetSize(&self) -> Common::D2D_SIZE_F {
        let mut result__: Common::D2D_SIZE_F = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetSize)(::windows::core::Vtable::as_raw(self), &mut result__);
        result__
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetPixelSize(&self) -> Common::D2D_SIZE_U {
        let mut result__: Common::D2D_SIZE_U = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetPixelSize)(::windows::core::Vtable::as_raw(self), &mut result__);
        result__
    }
    pub unsafe fn GetMaximumBitmapSize(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.base__.GetMaximumBitmapSize)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn IsSupported(&self, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.base__.IsSupported)(::windows::core::Vtable::as_raw(self), rendertargetproperties)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateBitmap2(&self, size: Common::D2D_SIZE_U, sourcedata: ::core::option::Option<*const ::core::ffi::c_void>, pitch: u32, bitmapproperties: *const D2D1_BITMAP_PROPERTIES1) -> ::windows::core::Result<ID2D1Bitmap1> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateBitmap2)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(size), ::core::mem::transmute(sourcedata.unwrap_or(::std::ptr::null())), pitch, bitmapproperties, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Imaging\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
    pub unsafe fn CreateBitmapFromWicBitmap2<P0>(&self, wicbitmapsource: P0, bitmapproperties: ::core::option::Option<*const D2D1_BITMAP_PROPERTIES1>) -> ::windows::core::Result<ID2D1Bitmap1>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Imaging::IWICBitmapSource>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateBitmapFromWicBitmap2)(::windows::core::Vtable::as_raw(self), wicbitmapsource.into().abi(), ::core::mem::transmute(bitmapproperties.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateColorContext(&self, space: D2D1_COLOR_SPACE, profile: ::core::option::Option<&[u8]>) -> ::windows::core::Result<ID2D1ColorContext> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateColorContext)(::windows::core::Vtable::as_raw(self), space, ::core::mem::transmute(profile.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), profile.as_deref().map_or(0, |slice| slice.len() as _), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateColorContextFromFilename<P0>(&self, filename: P0) -> ::windows::core::Result<ID2D1ColorContext>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateColorContextFromFilename)(::windows::core::Vtable::as_raw(self), filename.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub unsafe fn CreateColorContextFromWicColorContext<P0>(&self, wiccolorcontext: P0) -> ::windows::core::Result<ID2D1ColorContext>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Imaging::IWICColorContext>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateColorContextFromWicColorContext)(::windows::core::Vtable::as_raw(self), wiccolorcontext.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateBitmapFromDxgiSurface<P0>(&self, surface: P0, bitmapproperties: ::core::option::Option<*const D2D1_BITMAP_PROPERTIES1>) -> ::windows::core::Result<ID2D1Bitmap1>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Dxgi::IDXGISurface>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateBitmapFromDxgiSurface)(::windows::core::Vtable::as_raw(self), surface.into().abi(), ::core::mem::transmute(bitmapproperties.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateEffect(&self, effectid: *const ::windows::core::GUID) -> ::windows::core::Result<ID2D1Effect> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateEffect)(::windows::core::Vtable::as_raw(self), effectid, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CreateGradientStopCollection2(&self, straightalphagradientstops: &[D2D1_GRADIENT_STOP], preinterpolationspace: D2D1_COLOR_SPACE, postinterpolationspace: D2D1_COLOR_SPACE, bufferprecision: D2D1_BUFFER_PRECISION, extendmode: D2D1_EXTEND_MODE, colorinterpolationmode: D2D1_COLOR_INTERPOLATION_MODE) -> ::windows::core::Result<ID2D1GradientStopCollection1> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateGradientStopCollection2)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(straightalphagradientstops.as_ptr()), straightalphagradientstops.len() as _, preinterpolationspace, postinterpolationspace, bufferprecision, extendmode, colorinterpolationmode, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn CreateImageBrush<P0>(&self, image: P0, imagebrushproperties: *const D2D1_IMAGE_BRUSH_PROPERTIES, brushproperties: ::core::option::Option<*const D2D1_BRUSH_PROPERTIES>) -> ::windows::core::Result<ID2D1ImageBrush>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Image>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateImageBrush)(::windows::core::Vtable::as_raw(self), image.into().abi(), imagebrushproperties, ::core::mem::transmute(brushproperties.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn CreateBitmapBrush2<P0>(&self, bitmap: P0, bitmapbrushproperties: ::core::option::Option<*const D2D1_BITMAP_BRUSH_PROPERTIES1>, brushproperties: ::core::option::Option<*const D2D1_BRUSH_PROPERTIES>) -> ::windows::core::Result<ID2D1BitmapBrush1>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Bitmap>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateBitmapBrush2)(::windows::core::Vtable::as_raw(self), bitmap.into().abi(), ::core::mem::transmute(bitmapbrushproperties.unwrap_or(::std::ptr::null())), ::core::mem::transmute(brushproperties.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateCommandList(&self) -> ::windows::core::Result<ID2D1CommandList> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateCommandList)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn IsDxgiFormatSupported(&self, format: super::Dxgi::Common::DXGI_FORMAT) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.IsDxgiFormatSupported)(::windows::core::Vtable::as_raw(self), format)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsBufferPrecisionSupported(&self, bufferprecision: D2D1_BUFFER_PRECISION) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.IsBufferPrecisionSupported)(::windows::core::Vtable::as_raw(self), bufferprecision)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetImageLocalBounds<P0>(&self, image: P0) -> ::windows::core::Result<Common::D2D_RECT_F>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Image>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetImageLocalBounds)(::windows::core::Vtable::as_raw(self), image.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetImageWorldBounds<P0>(&self, image: P0) -> ::windows::core::Result<Common::D2D_RECT_F>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Image>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetImageWorldBounds)(::windows::core::Vtable::as_raw(self), image.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn GetGlyphRunWorldBounds(&self, baselineorigin: Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE) -> ::windows::core::Result<Common::D2D_RECT_F> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetGlyphRunWorldBounds)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(baselineorigin), glyphrun, measuringmode, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<ID2D1Device> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1Device as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn SetTarget<P0>(&self, image: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Image>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetTarget)(::windows::core::Vtable::as_raw(self), image.into().abi())
    }
    pub unsafe fn GetTarget(&self) -> ::windows::core::Result<ID2D1Image> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetTarget)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1Image as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetRenderingControls(&self, renderingcontrols: *const D2D1_RENDERING_CONTROLS) {
        (::windows::core::Vtable::vtable(self).base__.SetRenderingControls)(::windows::core::Vtable::as_raw(self), renderingcontrols)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetRenderingControls(&self) -> D2D1_RENDERING_CONTROLS {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetRenderingControls)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        result__.assume_init()
    }
    pub unsafe fn SetPrimitiveBlend(&self, primitiveblend: D2D1_PRIMITIVE_BLEND) {
        (::windows::core::Vtable::vtable(self).base__.SetPrimitiveBlend)(::windows::core::Vtable::as_raw(self), primitiveblend)
    }
    pub unsafe fn GetPrimitiveBlend(&self) -> D2D1_PRIMITIVE_BLEND {
        (::windows::core::Vtable::vtable(self).base__.GetPrimitiveBlend)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn SetUnitMode(&self, unitmode: D2D1_UNIT_MODE) {
        (::windows::core::Vtable::vtable(self).base__.SetUnitMode)(::windows::core::Vtable::as_raw(self), unitmode)
    }
    pub unsafe fn GetUnitMode(&self) -> D2D1_UNIT_MODE {
        (::windows::core::Vtable::vtable(self).base__.GetUnitMode)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn DrawGlyphRun2<P0>(&self, baselineorigin: Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, glyphrundescription: ::core::option::Option<*const super::DirectWrite::DWRITE_GLYPH_RUN_DESCRIPTION>, foregroundbrush: P0, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DrawGlyphRun2)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(baselineorigin), glyphrun, ::core::mem::transmute(glyphrundescription.unwrap_or(::std::ptr::null())), foregroundbrush.into().abi(), measuringmode)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawImage<P0>(&self, image: P0, targetoffset: ::core::option::Option<*const Common::D2D_POINT_2F>, imagerectangle: ::core::option::Option<*const Common::D2D_RECT_F>, interpolationmode: D2D1_INTERPOLATION_MODE, compositemode: Common::D2D1_COMPOSITE_MODE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Image>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DrawImage)(::windows::core::Vtable::as_raw(self), image.into().abi(), ::core::mem::transmute(targetoffset.unwrap_or(::std::ptr::null())), ::core::mem::transmute(imagerectangle.unwrap_or(::std::ptr::null())), interpolationmode, compositemode)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawGdiMetafile<P0>(&self, gdimetafile: P0, targetoffset: ::core::option::Option<*const Common::D2D_POINT_2F>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1GdiMetafile>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DrawGdiMetafile)(::windows::core::Vtable::as_raw(self), gdimetafile.into().abi(), ::core::mem::transmute(targetoffset.unwrap_or(::std::ptr::null())))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawBitmap2<P0>(&self, bitmap: P0, destinationrectangle: ::core::option::Option<*const Common::D2D_RECT_F>, opacity: f32, interpolationmode: D2D1_INTERPOLATION_MODE, sourcerectangle: ::core::option::Option<*const Common::D2D_RECT_F>, perspectivetransform: ::core::option::Option<*const Common::D2D_MATRIX_4X4_F>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Bitmap>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DrawBitmap2)(::windows::core::Vtable::as_raw(self), bitmap.into().abi(), ::core::mem::transmute(destinationrectangle.unwrap_or(::std::ptr::null())), opacity, interpolationmode, ::core::mem::transmute(sourcerectangle.unwrap_or(::std::ptr::null())), ::core::mem::transmute(perspectivetransform.unwrap_or(::std::ptr::null())))
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn PushLayer2<P0>(&self, layerparameters: *const D2D1_LAYER_PARAMETERS1, layer: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Layer>>,
    {
        (::windows::core::Vtable::vtable(self).base__.PushLayer2)(::windows::core::Vtable::as_raw(self), layerparameters, layer.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn InvalidateEffectInputRectangle<P0>(&self, effect: P0, input: u32, inputrectangle: *const Common::D2D_RECT_F) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Effect>>,
    {
        (::windows::core::Vtable::vtable(self).base__.InvalidateEffectInputRectangle)(::windows::core::Vtable::as_raw(self), effect.into().abi(), input, inputrectangle).ok()
    }
    pub unsafe fn GetEffectInvalidRectangleCount<P0>(&self, effect: P0) -> ::windows::core::Result<u32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Effect>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetEffectInvalidRectangleCount)(::windows::core::Vtable::as_raw(self), effect.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetEffectInvalidRectangles<P0>(&self, effect: P0, rectangles: &mut [Common::D2D_RECT_F]) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Effect>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetEffectInvalidRectangles)(::windows::core::Vtable::as_raw(self), effect.into().abi(), ::core::mem::transmute(rectangles.as_ptr()), rectangles.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetEffectRequiredInputRectangles<P0>(&self, rendereffect: P0, renderimagerectangle: ::core::option::Option<*const Common::D2D_RECT_F>, inputdescriptions: *const D2D1_EFFECT_INPUT_DESCRIPTION, requiredinputrects: *mut Common::D2D_RECT_F, inputcount: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Effect>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetEffectRequiredInputRectangles)(::windows::core::Vtable::as_raw(self), rendereffect.into().abi(), ::core::mem::transmute(renderimagerectangle.unwrap_or(::std::ptr::null())), inputdescriptions, requiredinputrects, inputcount).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn FillOpacityMask2<P0, P1>(&self, opacitymask: P0, brush: P1, destinationrectangle: ::core::option::Option<*const Common::D2D_RECT_F>, sourcerectangle: ::core::option::Option<*const Common::D2D_RECT_F>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Bitmap>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.FillOpacityMask2)(::windows::core::Vtable::as_raw(self), opacitymask.into().abi(), brush.into().abi(), ::core::mem::transmute(destinationrectangle.unwrap_or(::std::ptr::null())), ::core::mem::transmute(sourcerectangle.unwrap_or(::std::ptr::null())))
    }
}
impl ::core::cmp::PartialEq for ID2D1DeviceContext2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1DeviceContext2 {}
impl ::core::fmt::Debug for ID2D1DeviceContext2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1DeviceContext2").field(&self.0).finish()
    }
}
impl ID2D1DeviceContext2 {
    pub unsafe fn GetFactory(&self) -> ::windows::core::Result<ID2D1Factory> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetFactory)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1Factory as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateBitmap(&self, size: Common::D2D_SIZE_U, srcdata: ::core::option::Option<*const ::core::ffi::c_void>, pitch: u32, bitmapproperties: *const D2D1_BITMAP_PROPERTIES) -> ::windows::core::Result<ID2D1Bitmap> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateBitmap)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(size), ::core::mem::transmute(srcdata.unwrap_or(::std::ptr::null())), pitch, bitmapproperties, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Imaging\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
    pub unsafe fn CreateBitmapFromWicBitmap<P0>(&self, wicbitmapsource: P0, bitmapproperties: ::core::option::Option<*const D2D1_BITMAP_PROPERTIES>) -> ::windows::core::Result<ID2D1Bitmap>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Imaging::IWICBitmapSource>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateBitmapFromWicBitmap)(::windows::core::Vtable::as_raw(self), wicbitmapsource.into().abi(), ::core::mem::transmute(bitmapproperties.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSharedBitmap(&self, riid: *const ::windows::core::GUID, data: *mut ::core::ffi::c_void, bitmapproperties: ::core::option::Option<*const D2D1_BITMAP_PROPERTIES>, bitmap: *mut ::core::option::Option<ID2D1Bitmap>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateSharedBitmap)(::windows::core::Vtable::as_raw(self), riid, data, ::core::mem::transmute(bitmapproperties.unwrap_or(::std::ptr::null())), ::core::mem::transmute(bitmap)).ok()
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn CreateBitmapBrush<P0>(&self, bitmap: P0, bitmapbrushproperties: ::core::option::Option<*const D2D1_BITMAP_BRUSH_PROPERTIES>, brushproperties: ::core::option::Option<*const D2D1_BRUSH_PROPERTIES>) -> ::windows::core::Result<ID2D1BitmapBrush>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Bitmap>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateBitmapBrush)(::windows::core::Vtable::as_raw(self), bitmap.into().abi(), ::core::mem::transmute(bitmapbrushproperties.unwrap_or(::std::ptr::null())), ::core::mem::transmute(brushproperties.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn CreateSolidColorBrush(&self, color: *const Common::D2D1_COLOR_F, brushproperties: ::core::option::Option<*const D2D1_BRUSH_PROPERTIES>) -> ::windows::core::Result<ID2D1SolidColorBrush> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateSolidColorBrush)(::windows::core::Vtable::as_raw(self), color, ::core::mem::transmute(brushproperties.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CreateGradientStopCollection(&self, gradientstops: &[D2D1_GRADIENT_STOP], colorinterpolationgamma: D2D1_GAMMA, extendmode: D2D1_EXTEND_MODE) -> ::windows::core::Result<ID2D1GradientStopCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateGradientStopCollection)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(gradientstops.as_ptr()), gradientstops.len() as _, colorinterpolationgamma, extendmode, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn CreateLinearGradientBrush<P0>(&self, lineargradientbrushproperties: *const D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES, brushproperties: ::core::option::Option<*const D2D1_BRUSH_PROPERTIES>, gradientstopcollection: P0) -> ::windows::core::Result<ID2D1LinearGradientBrush>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1GradientStopCollection>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateLinearGradientBrush)(::windows::core::Vtable::as_raw(self), lineargradientbrushproperties, ::core::mem::transmute(brushproperties.unwrap_or(::std::ptr::null())), gradientstopcollection.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn CreateRadialGradientBrush<P0>(&self, radialgradientbrushproperties: *const D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES, brushproperties: ::core::option::Option<*const D2D1_BRUSH_PROPERTIES>, gradientstopcollection: P0) -> ::windows::core::Result<ID2D1RadialGradientBrush>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1GradientStopCollection>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateRadialGradientBrush)(::windows::core::Vtable::as_raw(self), radialgradientbrushproperties, ::core::mem::transmute(brushproperties.unwrap_or(::std::ptr::null())), gradientstopcollection.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateCompatibleRenderTarget(&self, desiredsize: ::core::option::Option<*const Common::D2D_SIZE_F>, desiredpixelsize: ::core::option::Option<*const Common::D2D_SIZE_U>, desiredformat: ::core::option::Option<*const Common::D2D1_PIXEL_FORMAT>, options: D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS) -> ::windows::core::Result<ID2D1BitmapRenderTarget> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateCompatibleRenderTarget)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(desiredsize.unwrap_or(::std::ptr::null())), ::core::mem::transmute(desiredpixelsize.unwrap_or(::std::ptr::null())), ::core::mem::transmute(desiredformat.unwrap_or(::std::ptr::null())), options, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CreateLayer(&self, size: ::core::option::Option<*const Common::D2D_SIZE_F>) -> ::windows::core::Result<ID2D1Layer> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateLayer)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(size.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateMesh(&self) -> ::windows::core::Result<ID2D1Mesh> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateMesh)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawLine<P0, P1>(&self, point0: Common::D2D_POINT_2F, point1: Common::D2D_POINT_2F, brush: P0, strokewidth: f32, strokestyle: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.DrawLine)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(point0), ::core::mem::transmute(point1), brush.into().abi(), strokewidth, strokestyle.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawRectangle<P0, P1>(&self, rect: *const Common::D2D_RECT_F, brush: P0, strokewidth: f32, strokestyle: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.DrawRectangle)(::windows::core::Vtable::as_raw(self), rect, brush.into().abi(), strokewidth, strokestyle.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn FillRectangle<P0>(&self, rect: *const Common::D2D_RECT_F, brush: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.FillRectangle)(::windows::core::Vtable::as_raw(self), rect, brush.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawRoundedRectangle<P0, P1>(&self, roundedrect: *const D2D1_ROUNDED_RECT, brush: P0, strokewidth: f32, strokestyle: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.DrawRoundedRectangle)(::windows::core::Vtable::as_raw(self), roundedrect, brush.into().abi(), strokewidth, strokestyle.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn FillRoundedRectangle<P0>(&self, roundedrect: *const D2D1_ROUNDED_RECT, brush: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.FillRoundedRectangle)(::windows::core::Vtable::as_raw(self), roundedrect, brush.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawEllipse<P0, P1>(&self, ellipse: *const D2D1_ELLIPSE, brush: P0, strokewidth: f32, strokestyle: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.DrawEllipse)(::windows::core::Vtable::as_raw(self), ellipse, brush.into().abi(), strokewidth, strokestyle.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn FillEllipse<P0>(&self, ellipse: *const D2D1_ELLIPSE, brush: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.FillEllipse)(::windows::core::Vtable::as_raw(self), ellipse, brush.into().abi())
    }
    pub unsafe fn DrawGeometry<P0, P1, P2>(&self, geometry: P0, brush: P1, strokewidth: f32, strokestyle: P2)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Geometry>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P2: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.DrawGeometry)(::windows::core::Vtable::as_raw(self), geometry.into().abi(), brush.into().abi(), strokewidth, strokestyle.into().abi())
    }
    pub unsafe fn FillGeometry<P0, P1, P2>(&self, geometry: P0, brush: P1, opacitybrush: P2)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Geometry>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P2: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.FillGeometry)(::windows::core::Vtable::as_raw(self), geometry.into().abi(), brush.into().abi(), opacitybrush.into().abi())
    }
    pub unsafe fn FillMesh<P0, P1>(&self, mesh: P0, brush: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Mesh>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.FillMesh)(::windows::core::Vtable::as_raw(self), mesh.into().abi(), brush.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn FillOpacityMask<P0, P1>(&self, opacitymask: P0, brush: P1, content: D2D1_OPACITY_MASK_CONTENT, destinationrectangle: ::core::option::Option<*const Common::D2D_RECT_F>, sourcerectangle: ::core::option::Option<*const Common::D2D_RECT_F>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Bitmap>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.FillOpacityMask)(::windows::core::Vtable::as_raw(self), opacitymask.into().abi(), brush.into().abi(), content, ::core::mem::transmute(destinationrectangle.unwrap_or(::std::ptr::null())), ::core::mem::transmute(sourcerectangle.unwrap_or(::std::ptr::null())))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawBitmap<P0>(&self, bitmap: P0, destinationrectangle: ::core::option::Option<*const Common::D2D_RECT_F>, opacity: f32, interpolationmode: D2D1_BITMAP_INTERPOLATION_MODE, sourcerectangle: ::core::option::Option<*const Common::D2D_RECT_F>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Bitmap>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.DrawBitmap)(::windows::core::Vtable::as_raw(self), bitmap.into().abi(), ::core::mem::transmute(destinationrectangle.unwrap_or(::std::ptr::null())), opacity, interpolationmode, ::core::mem::transmute(sourcerectangle.unwrap_or(::std::ptr::null())))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn DrawText<P0, P1>(&self, string: &[u16], textformat: P0, layoutrect: *const Common::D2D_RECT_F, defaultfillbrush: P1, options: D2D1_DRAW_TEXT_OPTIONS, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::DirectWrite::IDWriteTextFormat>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.DrawText)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(string.as_ptr()), string.len() as _, textformat.into().abi(), layoutrect, defaultfillbrush.into().abi(), options, measuringmode)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn DrawTextLayout<P0, P1>(&self, origin: Common::D2D_POINT_2F, textlayout: P0, defaultfillbrush: P1, options: D2D1_DRAW_TEXT_OPTIONS)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::DirectWrite::IDWriteTextLayout>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.DrawTextLayout)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(origin), textlayout.into().abi(), defaultfillbrush.into().abi(), options)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn DrawGlyphRun<P0>(&self, baselineorigin: Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, foregroundbrush: P0, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.DrawGlyphRun)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(baselineorigin), glyphrun, foregroundbrush.into().abi(), measuringmode)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn SetTransform(&self, transform: *const super::super::super::Foundation::Numerics::Matrix3x2) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetTransform)(::windows::core::Vtable::as_raw(self), transform)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn GetTransform(&self, transform: *mut super::super::super::Foundation::Numerics::Matrix3x2) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetTransform)(::windows::core::Vtable::as_raw(self), transform)
    }
    pub unsafe fn SetAntialiasMode(&self, antialiasmode: D2D1_ANTIALIAS_MODE) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetAntialiasMode)(::windows::core::Vtable::as_raw(self), antialiasmode)
    }
    pub unsafe fn GetAntialiasMode(&self) -> D2D1_ANTIALIAS_MODE {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetAntialiasMode)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn SetTextAntialiasMode(&self, textantialiasmode: D2D1_TEXT_ANTIALIAS_MODE) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetTextAntialiasMode)(::windows::core::Vtable::as_raw(self), textantialiasmode)
    }
    pub unsafe fn GetTextAntialiasMode(&self) -> D2D1_TEXT_ANTIALIAS_MODE {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetTextAntialiasMode)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(feature = "Win32_Graphics_DirectWrite")]
    pub unsafe fn SetTextRenderingParams<P0>(&self, textrenderingparams: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::DirectWrite::IDWriteRenderingParams>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetTextRenderingParams)(::windows::core::Vtable::as_raw(self), textrenderingparams.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(feature = "Win32_Graphics_DirectWrite")]
    pub unsafe fn GetTextRenderingParams(&self) -> ::windows::core::Result<super::DirectWrite::IDWriteRenderingParams> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetTextRenderingParams)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <super::DirectWrite::IDWriteRenderingParams as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn SetTags(&self, tag1: u64, tag2: u64) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetTags)(::windows::core::Vtable::as_raw(self), tag1, tag2)
    }
    pub unsafe fn GetTags(&self, tag1: ::core::option::Option<*mut u64>, tag2: ::core::option::Option<*mut u64>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetTags)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(tag1.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(tag2.unwrap_or(::std::ptr::null_mut())))
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn PushLayer<P0>(&self, layerparameters: *const D2D1_LAYER_PARAMETERS, layer: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Layer>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.PushLayer)(::windows::core::Vtable::as_raw(self), layerparameters, layer.into().abi())
    }
    pub unsafe fn PopLayer(&self) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.PopLayer)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn Flush(&self, tag1: ::core::option::Option<*mut u64>, tag2: ::core::option::Option<*mut u64>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.Flush)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(tag1.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(tag2.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SaveDrawingState<P0>(&self, drawingstateblock: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1DrawingStateBlock>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SaveDrawingState)(::windows::core::Vtable::as_raw(self), drawingstateblock.into().abi())
    }
    pub unsafe fn RestoreDrawingState<P0>(&self, drawingstateblock: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1DrawingStateBlock>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.RestoreDrawingState)(::windows::core::Vtable::as_raw(self), drawingstateblock.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn PushAxisAlignedClip(&self, cliprect: *const Common::D2D_RECT_F, antialiasmode: D2D1_ANTIALIAS_MODE) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.PushAxisAlignedClip)(::windows::core::Vtable::as_raw(self), cliprect, antialiasmode)
    }
    pub unsafe fn PopAxisAlignedClip(&self) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.PopAxisAlignedClip)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn Clear(&self, clearcolor: ::core::option::Option<*const Common::D2D1_COLOR_F>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.Clear)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(clearcolor.unwrap_or(::std::ptr::null())))
    }
    pub unsafe fn BeginDraw(&self) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.BeginDraw)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn EndDraw(&self, tag1: ::core::option::Option<*mut u64>, tag2: ::core::option::Option<*mut u64>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.EndDraw)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(tag1.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(tag2.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetPixelFormat(&self) -> Common::D2D1_PIXEL_FORMAT {
        let mut result__: Common::D2D1_PIXEL_FORMAT = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetPixelFormat)(::windows::core::Vtable::as_raw(self), &mut result__);
        result__
    }
    pub unsafe fn SetDpi(&self, dpix: f32, dpiy: f32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetDpi)(::windows::core::Vtable::as_raw(self), dpix, dpiy)
    }
    pub unsafe fn GetDpi(&self, dpix: *mut f32, dpiy: *mut f32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetDpi)(::windows::core::Vtable::as_raw(self), dpix, dpiy)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetSize(&self) -> Common::D2D_SIZE_F {
        let mut result__: Common::D2D_SIZE_F = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetSize)(::windows::core::Vtable::as_raw(self), &mut result__);
        result__
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetPixelSize(&self) -> Common::D2D_SIZE_U {
        let mut result__: Common::D2D_SIZE_U = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetPixelSize)(::windows::core::Vtable::as_raw(self), &mut result__);
        result__
    }
    pub unsafe fn GetMaximumBitmapSize(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetMaximumBitmapSize)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn IsSupported(&self, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.IsSupported)(::windows::core::Vtable::as_raw(self), rendertargetproperties)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateBitmap2(&self, size: Common::D2D_SIZE_U, sourcedata: ::core::option::Option<*const ::core::ffi::c_void>, pitch: u32, bitmapproperties: *const D2D1_BITMAP_PROPERTIES1) -> ::windows::core::Result<ID2D1Bitmap1> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateBitmap2)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(size), ::core::mem::transmute(sourcedata.unwrap_or(::std::ptr::null())), pitch, bitmapproperties, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Imaging\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
    pub unsafe fn CreateBitmapFromWicBitmap2<P0>(&self, wicbitmapsource: P0, bitmapproperties: ::core::option::Option<*const D2D1_BITMAP_PROPERTIES1>) -> ::windows::core::Result<ID2D1Bitmap1>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Imaging::IWICBitmapSource>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateBitmapFromWicBitmap2)(::windows::core::Vtable::as_raw(self), wicbitmapsource.into().abi(), ::core::mem::transmute(bitmapproperties.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateColorContext(&self, space: D2D1_COLOR_SPACE, profile: ::core::option::Option<&[u8]>) -> ::windows::core::Result<ID2D1ColorContext> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateColorContext)(::windows::core::Vtable::as_raw(self), space, ::core::mem::transmute(profile.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), profile.as_deref().map_or(0, |slice| slice.len() as _), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateColorContextFromFilename<P0>(&self, filename: P0) -> ::windows::core::Result<ID2D1ColorContext>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateColorContextFromFilename)(::windows::core::Vtable::as_raw(self), filename.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub unsafe fn CreateColorContextFromWicColorContext<P0>(&self, wiccolorcontext: P0) -> ::windows::core::Result<ID2D1ColorContext>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Imaging::IWICColorContext>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateColorContextFromWicColorContext)(::windows::core::Vtable::as_raw(self), wiccolorcontext.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateBitmapFromDxgiSurface<P0>(&self, surface: P0, bitmapproperties: ::core::option::Option<*const D2D1_BITMAP_PROPERTIES1>) -> ::windows::core::Result<ID2D1Bitmap1>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Dxgi::IDXGISurface>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateBitmapFromDxgiSurface)(::windows::core::Vtable::as_raw(self), surface.into().abi(), ::core::mem::transmute(bitmapproperties.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateEffect(&self, effectid: *const ::windows::core::GUID) -> ::windows::core::Result<ID2D1Effect> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateEffect)(::windows::core::Vtable::as_raw(self), effectid, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CreateGradientStopCollection2(&self, straightalphagradientstops: &[D2D1_GRADIENT_STOP], preinterpolationspace: D2D1_COLOR_SPACE, postinterpolationspace: D2D1_COLOR_SPACE, bufferprecision: D2D1_BUFFER_PRECISION, extendmode: D2D1_EXTEND_MODE, colorinterpolationmode: D2D1_COLOR_INTERPOLATION_MODE) -> ::windows::core::Result<ID2D1GradientStopCollection1> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateGradientStopCollection2)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(straightalphagradientstops.as_ptr()), straightalphagradientstops.len() as _, preinterpolationspace, postinterpolationspace, bufferprecision, extendmode, colorinterpolationmode, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn CreateImageBrush<P0>(&self, image: P0, imagebrushproperties: *const D2D1_IMAGE_BRUSH_PROPERTIES, brushproperties: ::core::option::Option<*const D2D1_BRUSH_PROPERTIES>) -> ::windows::core::Result<ID2D1ImageBrush>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Image>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateImageBrush)(::windows::core::Vtable::as_raw(self), image.into().abi(), imagebrushproperties, ::core::mem::transmute(brushproperties.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn CreateBitmapBrush2<P0>(&self, bitmap: P0, bitmapbrushproperties: ::core::option::Option<*const D2D1_BITMAP_BRUSH_PROPERTIES1>, brushproperties: ::core::option::Option<*const D2D1_BRUSH_PROPERTIES>) -> ::windows::core::Result<ID2D1BitmapBrush1>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Bitmap>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateBitmapBrush2)(::windows::core::Vtable::as_raw(self), bitmap.into().abi(), ::core::mem::transmute(bitmapbrushproperties.unwrap_or(::std::ptr::null())), ::core::mem::transmute(brushproperties.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateCommandList(&self) -> ::windows::core::Result<ID2D1CommandList> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateCommandList)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn IsDxgiFormatSupported(&self, format: super::Dxgi::Common::DXGI_FORMAT) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.base__.IsDxgiFormatSupported)(::windows::core::Vtable::as_raw(self), format)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsBufferPrecisionSupported(&self, bufferprecision: D2D1_BUFFER_PRECISION) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.base__.IsBufferPrecisionSupported)(::windows::core::Vtable::as_raw(self), bufferprecision)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetImageLocalBounds<P0>(&self, image: P0) -> ::windows::core::Result<Common::D2D_RECT_F>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Image>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetImageLocalBounds)(::windows::core::Vtable::as_raw(self), image.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetImageWorldBounds<P0>(&self, image: P0) -> ::windows::core::Result<Common::D2D_RECT_F>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Image>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetImageWorldBounds)(::windows::core::Vtable::as_raw(self), image.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn GetGlyphRunWorldBounds(&self, baselineorigin: Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE) -> ::windows::core::Result<Common::D2D_RECT_F> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetGlyphRunWorldBounds)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(baselineorigin), glyphrun, measuringmode, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<ID2D1Device> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1Device as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn SetTarget<P0>(&self, image: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Image>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetTarget)(::windows::core::Vtable::as_raw(self), image.into().abi())
    }
    pub unsafe fn GetTarget(&self) -> ::windows::core::Result<ID2D1Image> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetTarget)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1Image as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetRenderingControls(&self, renderingcontrols: *const D2D1_RENDERING_CONTROLS) {
        (::windows::core::Vtable::vtable(self).base__.base__.SetRenderingControls)(::windows::core::Vtable::as_raw(self), renderingcontrols)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetRenderingControls(&self) -> D2D1_RENDERING_CONTROLS {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetRenderingControls)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        result__.assume_init()
    }
    pub unsafe fn SetPrimitiveBlend(&self, primitiveblend: D2D1_PRIMITIVE_BLEND) {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPrimitiveBlend)(::windows::core::Vtable::as_raw(self), primitiveblend)
    }
    pub unsafe fn GetPrimitiveBlend(&self) -> D2D1_PRIMITIVE_BLEND {
        (::windows::core::Vtable::vtable(self).base__.base__.GetPrimitiveBlend)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn SetUnitMode(&self, unitmode: D2D1_UNIT_MODE) {
        (::windows::core::Vtable::vtable(self).base__.base__.SetUnitMode)(::windows::core::Vtable::as_raw(self), unitmode)
    }
    pub unsafe fn GetUnitMode(&self) -> D2D1_UNIT_MODE {
        (::windows::core::Vtable::vtable(self).base__.base__.GetUnitMode)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn DrawGlyphRun2<P0>(&self, baselineorigin: Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, glyphrundescription: ::core::option::Option<*const super::DirectWrite::DWRITE_GLYPH_RUN_DESCRIPTION>, foregroundbrush: P0, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.DrawGlyphRun2)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(baselineorigin), glyphrun, ::core::mem::transmute(glyphrundescription.unwrap_or(::std::ptr::null())), foregroundbrush.into().abi(), measuringmode)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawImage<P0>(&self, image: P0, targetoffset: ::core::option::Option<*const Common::D2D_POINT_2F>, imagerectangle: ::core::option::Option<*const Common::D2D_RECT_F>, interpolationmode: D2D1_INTERPOLATION_MODE, compositemode: Common::D2D1_COMPOSITE_MODE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Image>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.DrawImage)(::windows::core::Vtable::as_raw(self), image.into().abi(), ::core::mem::transmute(targetoffset.unwrap_or(::std::ptr::null())), ::core::mem::transmute(imagerectangle.unwrap_or(::std::ptr::null())), interpolationmode, compositemode)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawGdiMetafile<P0>(&self, gdimetafile: P0, targetoffset: ::core::option::Option<*const Common::D2D_POINT_2F>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1GdiMetafile>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.DrawGdiMetafile)(::windows::core::Vtable::as_raw(self), gdimetafile.into().abi(), ::core::mem::transmute(targetoffset.unwrap_or(::std::ptr::null())))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawBitmap2<P0>(&self, bitmap: P0, destinationrectangle: ::core::option::Option<*const Common::D2D_RECT_F>, opacity: f32, interpolationmode: D2D1_INTERPOLATION_MODE, sourcerectangle: ::core::option::Option<*const Common::D2D_RECT_F>, perspectivetransform: ::core::option::Option<*const Common::D2D_MATRIX_4X4_F>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Bitmap>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.DrawBitmap2)(::windows::core::Vtable::as_raw(self), bitmap.into().abi(), ::core::mem::transmute(destinationrectangle.unwrap_or(::std::ptr::null())), opacity, interpolationmode, ::core::mem::transmute(sourcerectangle.unwrap_or(::std::ptr::null())), ::core::mem::transmute(perspectivetransform.unwrap_or(::std::ptr::null())))
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn PushLayer2<P0>(&self, layerparameters: *const D2D1_LAYER_PARAMETERS1, layer: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Layer>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.PushLayer2)(::windows::core::Vtable::as_raw(self), layerparameters, layer.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn InvalidateEffectInputRectangle<P0>(&self, effect: P0, input: u32, inputrectangle: *const Common::D2D_RECT_F) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Effect>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.InvalidateEffectInputRectangle)(::windows::core::Vtable::as_raw(self), effect.into().abi(), input, inputrectangle).ok()
    }
    pub unsafe fn GetEffectInvalidRectangleCount<P0>(&self, effect: P0) -> ::windows::core::Result<u32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Effect>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetEffectInvalidRectangleCount)(::windows::core::Vtable::as_raw(self), effect.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetEffectInvalidRectangles<P0>(&self, effect: P0, rectangles: &mut [Common::D2D_RECT_F]) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Effect>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.GetEffectInvalidRectangles)(::windows::core::Vtable::as_raw(self), effect.into().abi(), ::core::mem::transmute(rectangles.as_ptr()), rectangles.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetEffectRequiredInputRectangles<P0>(&self, rendereffect: P0, renderimagerectangle: ::core::option::Option<*const Common::D2D_RECT_F>, inputdescriptions: *const D2D1_EFFECT_INPUT_DESCRIPTION, requiredinputrects: *mut Common::D2D_RECT_F, inputcount: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Effect>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.GetEffectRequiredInputRectangles)(::windows::core::Vtable::as_raw(self), rendereffect.into().abi(), ::core::mem::transmute(renderimagerectangle.unwrap_or(::std::ptr::null())), inputdescriptions, requiredinputrects, inputcount).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn FillOpacityMask2<P0, P1>(&self, opacitymask: P0, brush: P1, destinationrectangle: ::core::option::Option<*const Common::D2D_RECT_F>, sourcerectangle: ::core::option::Option<*const Common::D2D_RECT_F>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Bitmap>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.FillOpacityMask2)(::windows::core::Vtable::as_raw(self), opacitymask.into().abi(), brush.into().abi(), ::core::mem::transmute(destinationrectangle.unwrap_or(::std::ptr::null())), ::core::mem::transmute(sourcerectangle.unwrap_or(::std::ptr::null())))
    }
    pub unsafe fn CreateFilledGeometryRealization<P0>(&self, geometry: P0, flatteningtolerance: f32) -> ::windows::core::Result<ID2D1GeometryRealization>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Geometry>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateFilledGeometryRealization)(::windows::core::Vtable::as_raw(self), geometry.into().abi(), flatteningtolerance, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateStrokedGeometryRealization<P0, P1>(&self, geometry: P0, flatteningtolerance: f32, strokewidth: f32, strokestyle: P1) -> ::windows::core::Result<ID2D1GeometryRealization>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Geometry>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateStrokedGeometryRealization)(::windows::core::Vtable::as_raw(self), geometry.into().abi(), flatteningtolerance, strokewidth, strokestyle.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn DrawGeometryRealization<P0, P1>(&self, geometryrealization: P0, brush: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1GeometryRealization>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DrawGeometryRealization)(::windows::core::Vtable::as_raw(self), geometryrealization.into().abi(), brush.into().abi())
    }
}
impl ::core::cmp::PartialEq for ID2D1DeviceContext3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1DeviceContext3 {}
impl ::core::fmt::Debug for ID2D1DeviceContext3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1DeviceContext3").field(&self.0).finish()
    }
}
impl ID2D1DeviceContext3 {
    pub unsafe fn GetFactory(&self) -> ::windows::core::Result<ID2D1Factory> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetFactory)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1Factory as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateBitmap(&self, size: Common::D2D_SIZE_U, srcdata: ::core::option::Option<*const ::core::ffi::c_void>, pitch: u32, bitmapproperties: *const D2D1_BITMAP_PROPERTIES) -> ::windows::core::Result<ID2D1Bitmap> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateBitmap)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(size), ::core::mem::transmute(srcdata.unwrap_or(::std::ptr::null())), pitch, bitmapproperties, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Imaging\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
    pub unsafe fn CreateBitmapFromWicBitmap<P0>(&self, wicbitmapsource: P0, bitmapproperties: ::core::option::Option<*const D2D1_BITMAP_PROPERTIES>) -> ::windows::core::Result<ID2D1Bitmap>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Imaging::IWICBitmapSource>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateBitmapFromWicBitmap)(::windows::core::Vtable::as_raw(self), wicbitmapsource.into().abi(), ::core::mem::transmute(bitmapproperties.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSharedBitmap(&self, riid: *const ::windows::core::GUID, data: *mut ::core::ffi::c_void, bitmapproperties: ::core::option::Option<*const D2D1_BITMAP_PROPERTIES>, bitmap: *mut ::core::option::Option<ID2D1Bitmap>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateSharedBitmap)(::windows::core::Vtable::as_raw(self), riid, data, ::core::mem::transmute(bitmapproperties.unwrap_or(::std::ptr::null())), ::core::mem::transmute(bitmap)).ok()
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn CreateBitmapBrush<P0>(&self, bitmap: P0, bitmapbrushproperties: ::core::option::Option<*const D2D1_BITMAP_BRUSH_PROPERTIES>, brushproperties: ::core::option::Option<*const D2D1_BRUSH_PROPERTIES>) -> ::windows::core::Result<ID2D1BitmapBrush>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Bitmap>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateBitmapBrush)(::windows::core::Vtable::as_raw(self), bitmap.into().abi(), ::core::mem::transmute(bitmapbrushproperties.unwrap_or(::std::ptr::null())), ::core::mem::transmute(brushproperties.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn CreateSolidColorBrush(&self, color: *const Common::D2D1_COLOR_F, brushproperties: ::core::option::Option<*const D2D1_BRUSH_PROPERTIES>) -> ::windows::core::Result<ID2D1SolidColorBrush> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateSolidColorBrush)(::windows::core::Vtable::as_raw(self), color, ::core::mem::transmute(brushproperties.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CreateGradientStopCollection(&self, gradientstops: &[D2D1_GRADIENT_STOP], colorinterpolationgamma: D2D1_GAMMA, extendmode: D2D1_EXTEND_MODE) -> ::windows::core::Result<ID2D1GradientStopCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateGradientStopCollection)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(gradientstops.as_ptr()), gradientstops.len() as _, colorinterpolationgamma, extendmode, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn CreateLinearGradientBrush<P0>(&self, lineargradientbrushproperties: *const D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES, brushproperties: ::core::option::Option<*const D2D1_BRUSH_PROPERTIES>, gradientstopcollection: P0) -> ::windows::core::Result<ID2D1LinearGradientBrush>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1GradientStopCollection>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateLinearGradientBrush)(::windows::core::Vtable::as_raw(self), lineargradientbrushproperties, ::core::mem::transmute(brushproperties.unwrap_or(::std::ptr::null())), gradientstopcollection.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn CreateRadialGradientBrush<P0>(&self, radialgradientbrushproperties: *const D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES, brushproperties: ::core::option::Option<*const D2D1_BRUSH_PROPERTIES>, gradientstopcollection: P0) -> ::windows::core::Result<ID2D1RadialGradientBrush>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1GradientStopCollection>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateRadialGradientBrush)(::windows::core::Vtable::as_raw(self), radialgradientbrushproperties, ::core::mem::transmute(brushproperties.unwrap_or(::std::ptr::null())), gradientstopcollection.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateCompatibleRenderTarget(&self, desiredsize: ::core::option::Option<*const Common::D2D_SIZE_F>, desiredpixelsize: ::core::option::Option<*const Common::D2D_SIZE_U>, desiredformat: ::core::option::Option<*const Common::D2D1_PIXEL_FORMAT>, options: D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS) -> ::windows::core::Result<ID2D1BitmapRenderTarget> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateCompatibleRenderTarget)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(desiredsize.unwrap_or(::std::ptr::null())), ::core::mem::transmute(desiredpixelsize.unwrap_or(::std::ptr::null())), ::core::mem::transmute(desiredformat.unwrap_or(::std::ptr::null())), options, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CreateLayer(&self, size: ::core::option::Option<*const Common::D2D_SIZE_F>) -> ::windows::core::Result<ID2D1Layer> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateLayer)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(size.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateMesh(&self) -> ::windows::core::Result<ID2D1Mesh> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateMesh)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawLine<P0, P1>(&self, point0: Common::D2D_POINT_2F, point1: Common::D2D_POINT_2F, brush: P0, strokewidth: f32, strokestyle: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.DrawLine)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(point0), ::core::mem::transmute(point1), brush.into().abi(), strokewidth, strokestyle.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawRectangle<P0, P1>(&self, rect: *const Common::D2D_RECT_F, brush: P0, strokewidth: f32, strokestyle: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.DrawRectangle)(::windows::core::Vtable::as_raw(self), rect, brush.into().abi(), strokewidth, strokestyle.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn FillRectangle<P0>(&self, rect: *const Common::D2D_RECT_F, brush: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.FillRectangle)(::windows::core::Vtable::as_raw(self), rect, brush.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawRoundedRectangle<P0, P1>(&self, roundedrect: *const D2D1_ROUNDED_RECT, brush: P0, strokewidth: f32, strokestyle: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.DrawRoundedRectangle)(::windows::core::Vtable::as_raw(self), roundedrect, brush.into().abi(), strokewidth, strokestyle.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn FillRoundedRectangle<P0>(&self, roundedrect: *const D2D1_ROUNDED_RECT, brush: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.FillRoundedRectangle)(::windows::core::Vtable::as_raw(self), roundedrect, brush.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawEllipse<P0, P1>(&self, ellipse: *const D2D1_ELLIPSE, brush: P0, strokewidth: f32, strokestyle: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.DrawEllipse)(::windows::core::Vtable::as_raw(self), ellipse, brush.into().abi(), strokewidth, strokestyle.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn FillEllipse<P0>(&self, ellipse: *const D2D1_ELLIPSE, brush: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.FillEllipse)(::windows::core::Vtable::as_raw(self), ellipse, brush.into().abi())
    }
    pub unsafe fn DrawGeometry<P0, P1, P2>(&self, geometry: P0, brush: P1, strokewidth: f32, strokestyle: P2)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Geometry>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P2: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.DrawGeometry)(::windows::core::Vtable::as_raw(self), geometry.into().abi(), brush.into().abi(), strokewidth, strokestyle.into().abi())
    }
    pub unsafe fn FillGeometry<P0, P1, P2>(&self, geometry: P0, brush: P1, opacitybrush: P2)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Geometry>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P2: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.FillGeometry)(::windows::core::Vtable::as_raw(self), geometry.into().abi(), brush.into().abi(), opacitybrush.into().abi())
    }
    pub unsafe fn FillMesh<P0, P1>(&self, mesh: P0, brush: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Mesh>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.FillMesh)(::windows::core::Vtable::as_raw(self), mesh.into().abi(), brush.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn FillOpacityMask<P0, P1>(&self, opacitymask: P0, brush: P1, content: D2D1_OPACITY_MASK_CONTENT, destinationrectangle: ::core::option::Option<*const Common::D2D_RECT_F>, sourcerectangle: ::core::option::Option<*const Common::D2D_RECT_F>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Bitmap>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.FillOpacityMask)(::windows::core::Vtable::as_raw(self), opacitymask.into().abi(), brush.into().abi(), content, ::core::mem::transmute(destinationrectangle.unwrap_or(::std::ptr::null())), ::core::mem::transmute(sourcerectangle.unwrap_or(::std::ptr::null())))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawBitmap<P0>(&self, bitmap: P0, destinationrectangle: ::core::option::Option<*const Common::D2D_RECT_F>, opacity: f32, interpolationmode: D2D1_BITMAP_INTERPOLATION_MODE, sourcerectangle: ::core::option::Option<*const Common::D2D_RECT_F>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Bitmap>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.DrawBitmap)(::windows::core::Vtable::as_raw(self), bitmap.into().abi(), ::core::mem::transmute(destinationrectangle.unwrap_or(::std::ptr::null())), opacity, interpolationmode, ::core::mem::transmute(sourcerectangle.unwrap_or(::std::ptr::null())))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn DrawText<P0, P1>(&self, string: &[u16], textformat: P0, layoutrect: *const Common::D2D_RECT_F, defaultfillbrush: P1, options: D2D1_DRAW_TEXT_OPTIONS, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::DirectWrite::IDWriteTextFormat>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.DrawText)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(string.as_ptr()), string.len() as _, textformat.into().abi(), layoutrect, defaultfillbrush.into().abi(), options, measuringmode)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn DrawTextLayout<P0, P1>(&self, origin: Common::D2D_POINT_2F, textlayout: P0, defaultfillbrush: P1, options: D2D1_DRAW_TEXT_OPTIONS)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::DirectWrite::IDWriteTextLayout>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.DrawTextLayout)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(origin), textlayout.into().abi(), defaultfillbrush.into().abi(), options)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn DrawGlyphRun<P0>(&self, baselineorigin: Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, foregroundbrush: P0, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.DrawGlyphRun)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(baselineorigin), glyphrun, foregroundbrush.into().abi(), measuringmode)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn SetTransform(&self, transform: *const super::super::super::Foundation::Numerics::Matrix3x2) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetTransform)(::windows::core::Vtable::as_raw(self), transform)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn GetTransform(&self, transform: *mut super::super::super::Foundation::Numerics::Matrix3x2) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetTransform)(::windows::core::Vtable::as_raw(self), transform)
    }
    pub unsafe fn SetAntialiasMode(&self, antialiasmode: D2D1_ANTIALIAS_MODE) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetAntialiasMode)(::windows::core::Vtable::as_raw(self), antialiasmode)
    }
    pub unsafe fn GetAntialiasMode(&self) -> D2D1_ANTIALIAS_MODE {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetAntialiasMode)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn SetTextAntialiasMode(&self, textantialiasmode: D2D1_TEXT_ANTIALIAS_MODE) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetTextAntialiasMode)(::windows::core::Vtable::as_raw(self), textantialiasmode)
    }
    pub unsafe fn GetTextAntialiasMode(&self) -> D2D1_TEXT_ANTIALIAS_MODE {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetTextAntialiasMode)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(feature = "Win32_Graphics_DirectWrite")]
    pub unsafe fn SetTextRenderingParams<P0>(&self, textrenderingparams: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::DirectWrite::IDWriteRenderingParams>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetTextRenderingParams)(::windows::core::Vtable::as_raw(self), textrenderingparams.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(feature = "Win32_Graphics_DirectWrite")]
    pub unsafe fn GetTextRenderingParams(&self) -> ::windows::core::Result<super::DirectWrite::IDWriteRenderingParams> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetTextRenderingParams)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <super::DirectWrite::IDWriteRenderingParams as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn SetTags(&self, tag1: u64, tag2: u64) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetTags)(::windows::core::Vtable::as_raw(self), tag1, tag2)
    }
    pub unsafe fn GetTags(&self, tag1: ::core::option::Option<*mut u64>, tag2: ::core::option::Option<*mut u64>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetTags)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(tag1.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(tag2.unwrap_or(::std::ptr::null_mut())))
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn PushLayer<P0>(&self, layerparameters: *const D2D1_LAYER_PARAMETERS, layer: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Layer>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.PushLayer)(::windows::core::Vtable::as_raw(self), layerparameters, layer.into().abi())
    }
    pub unsafe fn PopLayer(&self) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.PopLayer)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn Flush(&self, tag1: ::core::option::Option<*mut u64>, tag2: ::core::option::Option<*mut u64>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.Flush)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(tag1.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(tag2.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SaveDrawingState<P0>(&self, drawingstateblock: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1DrawingStateBlock>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SaveDrawingState)(::windows::core::Vtable::as_raw(self), drawingstateblock.into().abi())
    }
    pub unsafe fn RestoreDrawingState<P0>(&self, drawingstateblock: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1DrawingStateBlock>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.RestoreDrawingState)(::windows::core::Vtable::as_raw(self), drawingstateblock.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn PushAxisAlignedClip(&self, cliprect: *const Common::D2D_RECT_F, antialiasmode: D2D1_ANTIALIAS_MODE) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.PushAxisAlignedClip)(::windows::core::Vtable::as_raw(self), cliprect, antialiasmode)
    }
    pub unsafe fn PopAxisAlignedClip(&self) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.PopAxisAlignedClip)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn Clear(&self, clearcolor: ::core::option::Option<*const Common::D2D1_COLOR_F>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.Clear)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(clearcolor.unwrap_or(::std::ptr::null())))
    }
    pub unsafe fn BeginDraw(&self) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.BeginDraw)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn EndDraw(&self, tag1: ::core::option::Option<*mut u64>, tag2: ::core::option::Option<*mut u64>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.EndDraw)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(tag1.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(tag2.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetPixelFormat(&self) -> Common::D2D1_PIXEL_FORMAT {
        let mut result__: Common::D2D1_PIXEL_FORMAT = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetPixelFormat)(::windows::core::Vtable::as_raw(self), &mut result__);
        result__
    }
    pub unsafe fn SetDpi(&self, dpix: f32, dpiy: f32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetDpi)(::windows::core::Vtable::as_raw(self), dpix, dpiy)
    }
    pub unsafe fn GetDpi(&self, dpix: *mut f32, dpiy: *mut f32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetDpi)(::windows::core::Vtable::as_raw(self), dpix, dpiy)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetSize(&self) -> Common::D2D_SIZE_F {
        let mut result__: Common::D2D_SIZE_F = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetSize)(::windows::core::Vtable::as_raw(self), &mut result__);
        result__
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetPixelSize(&self) -> Common::D2D_SIZE_U {
        let mut result__: Common::D2D_SIZE_U = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetPixelSize)(::windows::core::Vtable::as_raw(self), &mut result__);
        result__
    }
    pub unsafe fn GetMaximumBitmapSize(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetMaximumBitmapSize)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn IsSupported(&self, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.IsSupported)(::windows::core::Vtable::as_raw(self), rendertargetproperties)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateBitmap2(&self, size: Common::D2D_SIZE_U, sourcedata: ::core::option::Option<*const ::core::ffi::c_void>, pitch: u32, bitmapproperties: *const D2D1_BITMAP_PROPERTIES1) -> ::windows::core::Result<ID2D1Bitmap1> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateBitmap2)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(size), ::core::mem::transmute(sourcedata.unwrap_or(::std::ptr::null())), pitch, bitmapproperties, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Imaging\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
    pub unsafe fn CreateBitmapFromWicBitmap2<P0>(&self, wicbitmapsource: P0, bitmapproperties: ::core::option::Option<*const D2D1_BITMAP_PROPERTIES1>) -> ::windows::core::Result<ID2D1Bitmap1>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Imaging::IWICBitmapSource>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateBitmapFromWicBitmap2)(::windows::core::Vtable::as_raw(self), wicbitmapsource.into().abi(), ::core::mem::transmute(bitmapproperties.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateColorContext(&self, space: D2D1_COLOR_SPACE, profile: ::core::option::Option<&[u8]>) -> ::windows::core::Result<ID2D1ColorContext> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateColorContext)(::windows::core::Vtable::as_raw(self), space, ::core::mem::transmute(profile.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), profile.as_deref().map_or(0, |slice| slice.len() as _), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateColorContextFromFilename<P0>(&self, filename: P0) -> ::windows::core::Result<ID2D1ColorContext>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateColorContextFromFilename)(::windows::core::Vtable::as_raw(self), filename.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub unsafe fn CreateColorContextFromWicColorContext<P0>(&self, wiccolorcontext: P0) -> ::windows::core::Result<ID2D1ColorContext>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Imaging::IWICColorContext>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateColorContextFromWicColorContext)(::windows::core::Vtable::as_raw(self), wiccolorcontext.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateBitmapFromDxgiSurface<P0>(&self, surface: P0, bitmapproperties: ::core::option::Option<*const D2D1_BITMAP_PROPERTIES1>) -> ::windows::core::Result<ID2D1Bitmap1>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Dxgi::IDXGISurface>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateBitmapFromDxgiSurface)(::windows::core::Vtable::as_raw(self), surface.into().abi(), ::core::mem::transmute(bitmapproperties.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateEffect(&self, effectid: *const ::windows::core::GUID) -> ::windows::core::Result<ID2D1Effect> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateEffect)(::windows::core::Vtable::as_raw(self), effectid, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CreateGradientStopCollection2(&self, straightalphagradientstops: &[D2D1_GRADIENT_STOP], preinterpolationspace: D2D1_COLOR_SPACE, postinterpolationspace: D2D1_COLOR_SPACE, bufferprecision: D2D1_BUFFER_PRECISION, extendmode: D2D1_EXTEND_MODE, colorinterpolationmode: D2D1_COLOR_INTERPOLATION_MODE) -> ::windows::core::Result<ID2D1GradientStopCollection1> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateGradientStopCollection2)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(straightalphagradientstops.as_ptr()), straightalphagradientstops.len() as _, preinterpolationspace, postinterpolationspace, bufferprecision, extendmode, colorinterpolationmode, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn CreateImageBrush<P0>(&self, image: P0, imagebrushproperties: *const D2D1_IMAGE_BRUSH_PROPERTIES, brushproperties: ::core::option::Option<*const D2D1_BRUSH_PROPERTIES>) -> ::windows::core::Result<ID2D1ImageBrush>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Image>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateImageBrush)(::windows::core::Vtable::as_raw(self), image.into().abi(), imagebrushproperties, ::core::mem::transmute(brushproperties.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn CreateBitmapBrush2<P0>(&self, bitmap: P0, bitmapbrushproperties: ::core::option::Option<*const D2D1_BITMAP_BRUSH_PROPERTIES1>, brushproperties: ::core::option::Option<*const D2D1_BRUSH_PROPERTIES>) -> ::windows::core::Result<ID2D1BitmapBrush1>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Bitmap>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateBitmapBrush2)(::windows::core::Vtable::as_raw(self), bitmap.into().abi(), ::core::mem::transmute(bitmapbrushproperties.unwrap_or(::std::ptr::null())), ::core::mem::transmute(brushproperties.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateCommandList(&self) -> ::windows::core::Result<ID2D1CommandList> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateCommandList)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn IsDxgiFormatSupported(&self, format: super::Dxgi::Common::DXGI_FORMAT) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.IsDxgiFormatSupported)(::windows::core::Vtable::as_raw(self), format)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsBufferPrecisionSupported(&self, bufferprecision: D2D1_BUFFER_PRECISION) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.IsBufferPrecisionSupported)(::windows::core::Vtable::as_raw(self), bufferprecision)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetImageLocalBounds<P0>(&self, image: P0) -> ::windows::core::Result<Common::D2D_RECT_F>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Image>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetImageLocalBounds)(::windows::core::Vtable::as_raw(self), image.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetImageWorldBounds<P0>(&self, image: P0) -> ::windows::core::Result<Common::D2D_RECT_F>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Image>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetImageWorldBounds)(::windows::core::Vtable::as_raw(self), image.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn GetGlyphRunWorldBounds(&self, baselineorigin: Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE) -> ::windows::core::Result<Common::D2D_RECT_F> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetGlyphRunWorldBounds)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(baselineorigin), glyphrun, measuringmode, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<ID2D1Device> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1Device as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn SetTarget<P0>(&self, image: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Image>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetTarget)(::windows::core::Vtable::as_raw(self), image.into().abi())
    }
    pub unsafe fn GetTarget(&self) -> ::windows::core::Result<ID2D1Image> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetTarget)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1Image as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetRenderingControls(&self, renderingcontrols: *const D2D1_RENDERING_CONTROLS) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetRenderingControls)(::windows::core::Vtable::as_raw(self), renderingcontrols)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetRenderingControls(&self) -> D2D1_RENDERING_CONTROLS {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetRenderingControls)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        result__.assume_init()
    }
    pub unsafe fn SetPrimitiveBlend(&self, primitiveblend: D2D1_PRIMITIVE_BLEND) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrimitiveBlend)(::windows::core::Vtable::as_raw(self), primitiveblend)
    }
    pub unsafe fn GetPrimitiveBlend(&self) -> D2D1_PRIMITIVE_BLEND {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetPrimitiveBlend)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn SetUnitMode(&self, unitmode: D2D1_UNIT_MODE) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetUnitMode)(::windows::core::Vtable::as_raw(self), unitmode)
    }
    pub unsafe fn GetUnitMode(&self) -> D2D1_UNIT_MODE {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetUnitMode)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn DrawGlyphRun2<P0>(&self, baselineorigin: Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, glyphrundescription: ::core::option::Option<*const super::DirectWrite::DWRITE_GLYPH_RUN_DESCRIPTION>, foregroundbrush: P0, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.DrawGlyphRun2)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(baselineorigin), glyphrun, ::core::mem::transmute(glyphrundescription.unwrap_or(::std::ptr::null())), foregroundbrush.into().abi(), measuringmode)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawImage<P0>(&self, image: P0, targetoffset: ::core::option::Option<*const Common::D2D_POINT_2F>, imagerectangle: ::core::option::Option<*const Common::D2D_RECT_F>, interpolationmode: D2D1_INTERPOLATION_MODE, compositemode: Common::D2D1_COMPOSITE_MODE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Image>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.DrawImage)(::windows::core::Vtable::as_raw(self), image.into().abi(), ::core::mem::transmute(targetoffset.unwrap_or(::std::ptr::null())), ::core::mem::transmute(imagerectangle.unwrap_or(::std::ptr::null())), interpolationmode, compositemode)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawGdiMetafile<P0>(&self, gdimetafile: P0, targetoffset: ::core::option::Option<*const Common::D2D_POINT_2F>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1GdiMetafile>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.DrawGdiMetafile)(::windows::core::Vtable::as_raw(self), gdimetafile.into().abi(), ::core::mem::transmute(targetoffset.unwrap_or(::std::ptr::null())))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawBitmap2<P0>(&self, bitmap: P0, destinationrectangle: ::core::option::Option<*const Common::D2D_RECT_F>, opacity: f32, interpolationmode: D2D1_INTERPOLATION_MODE, sourcerectangle: ::core::option::Option<*const Common::D2D_RECT_F>, perspectivetransform: ::core::option::Option<*const Common::D2D_MATRIX_4X4_F>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Bitmap>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.DrawBitmap2)(::windows::core::Vtable::as_raw(self), bitmap.into().abi(), ::core::mem::transmute(destinationrectangle.unwrap_or(::std::ptr::null())), opacity, interpolationmode, ::core::mem::transmute(sourcerectangle.unwrap_or(::std::ptr::null())), ::core::mem::transmute(perspectivetransform.unwrap_or(::std::ptr::null())))
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn PushLayer2<P0>(&self, layerparameters: *const D2D1_LAYER_PARAMETERS1, layer: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Layer>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.PushLayer2)(::windows::core::Vtable::as_raw(self), layerparameters, layer.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn InvalidateEffectInputRectangle<P0>(&self, effect: P0, input: u32, inputrectangle: *const Common::D2D_RECT_F) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Effect>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.InvalidateEffectInputRectangle)(::windows::core::Vtable::as_raw(self), effect.into().abi(), input, inputrectangle).ok()
    }
    pub unsafe fn GetEffectInvalidRectangleCount<P0>(&self, effect: P0) -> ::windows::core::Result<u32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Effect>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetEffectInvalidRectangleCount)(::windows::core::Vtable::as_raw(self), effect.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetEffectInvalidRectangles<P0>(&self, effect: P0, rectangles: &mut [Common::D2D_RECT_F]) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Effect>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetEffectInvalidRectangles)(::windows::core::Vtable::as_raw(self), effect.into().abi(), ::core::mem::transmute(rectangles.as_ptr()), rectangles.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetEffectRequiredInputRectangles<P0>(&self, rendereffect: P0, renderimagerectangle: ::core::option::Option<*const Common::D2D_RECT_F>, inputdescriptions: *const D2D1_EFFECT_INPUT_DESCRIPTION, requiredinputrects: *mut Common::D2D_RECT_F, inputcount: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Effect>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetEffectRequiredInputRectangles)(::windows::core::Vtable::as_raw(self), rendereffect.into().abi(), ::core::mem::transmute(renderimagerectangle.unwrap_or(::std::ptr::null())), inputdescriptions, requiredinputrects, inputcount).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn FillOpacityMask2<P0, P1>(&self, opacitymask: P0, brush: P1, destinationrectangle: ::core::option::Option<*const Common::D2D_RECT_F>, sourcerectangle: ::core::option::Option<*const Common::D2D_RECT_F>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Bitmap>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.FillOpacityMask2)(::windows::core::Vtable::as_raw(self), opacitymask.into().abi(), brush.into().abi(), ::core::mem::transmute(destinationrectangle.unwrap_or(::std::ptr::null())), ::core::mem::transmute(sourcerectangle.unwrap_or(::std::ptr::null())))
    }
    pub unsafe fn CreateFilledGeometryRealization<P0>(&self, geometry: P0, flatteningtolerance: f32) -> ::windows::core::Result<ID2D1GeometryRealization>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Geometry>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateFilledGeometryRealization)(::windows::core::Vtable::as_raw(self), geometry.into().abi(), flatteningtolerance, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateStrokedGeometryRealization<P0, P1>(&self, geometry: P0, flatteningtolerance: f32, strokewidth: f32, strokestyle: P1) -> ::windows::core::Result<ID2D1GeometryRealization>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Geometry>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateStrokedGeometryRealization)(::windows::core::Vtable::as_raw(self), geometry.into().abi(), flatteningtolerance, strokewidth, strokestyle.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn DrawGeometryRealization<P0, P1>(&self, geometryrealization: P0, brush: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1GeometryRealization>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.DrawGeometryRealization)(::windows::core::Vtable::as_raw(self), geometryrealization.into().abi(), brush.into().abi())
    }
    pub unsafe fn CreateInk(&self, startpoint: *const D2D1_INK_POINT) -> ::windows::core::Result<ID2D1Ink> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateInk)(::windows::core::Vtable::as_raw(self), startpoint, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn CreateInkStyle(&self, inkstyleproperties: ::core::option::Option<*const D2D1_INK_STYLE_PROPERTIES>) -> ::windows::core::Result<ID2D1InkStyle> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateInkStyle)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(inkstyleproperties.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CreateGradientMesh(&self, patches: &[D2D1_GRADIENT_MESH_PATCH]) -> ::windows::core::Result<ID2D1GradientMesh> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateGradientMesh)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(patches.as_ptr()), patches.len() as _, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Imaging\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Imaging"))]
    pub unsafe fn CreateImageSourceFromWic<P0>(&self, wicbitmapsource: P0, loadingoptions: D2D1_IMAGE_SOURCE_LOADING_OPTIONS, alphamode: Common::D2D1_ALPHA_MODE) -> ::windows::core::Result<ID2D1ImageSourceFromWic>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Imaging::IWICBitmapSource>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateImageSourceFromWic)(::windows::core::Vtable::as_raw(self), wicbitmapsource.into().abi(), loadingoptions, alphamode, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateLookupTable3D(&self, precision: D2D1_BUFFER_PRECISION, extents: &[u32; 3], data: &[u8], strides: &[u32; 2]) -> ::windows::core::Result<ID2D1LookupTable3D> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateLookupTable3D)(::windows::core::Vtable::as_raw(self), precision, ::core::mem::transmute(extents.as_ptr()), ::core::mem::transmute(data.as_ptr()), data.len() as _, ::core::mem::transmute(strides.as_ptr()), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateImageSourceFromDxgi(&self, surfaces: &[super::Dxgi::IDXGISurface], colorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE, options: D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS) -> ::windows::core::Result<ID2D1ImageSource> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateImageSourceFromDxgi)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(surfaces.as_ptr()), surfaces.len() as _, colorspace, options, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetGradientMeshWorldBounds<P0>(&self, gradientmesh: P0) -> ::windows::core::Result<Common::D2D_RECT_F>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1GradientMesh>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetGradientMeshWorldBounds)(::windows::core::Vtable::as_raw(self), gradientmesh.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn DrawInk<P0, P1, P2>(&self, ink: P0, brush: P1, inkstyle: P2)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Ink>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P2: ::std::convert::Into<::windows::core::InParam<ID2D1InkStyle>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DrawInk)(::windows::core::Vtable::as_raw(self), ink.into().abi(), brush.into().abi(), inkstyle.into().abi())
    }
    pub unsafe fn DrawGradientMesh<P0>(&self, gradientmesh: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1GradientMesh>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DrawGradientMesh)(::windows::core::Vtable::as_raw(self), gradientmesh.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawGdiMetafile2<P0>(&self, gdimetafile: P0, destinationrectangle: ::core::option::Option<*const Common::D2D_RECT_F>, sourcerectangle: ::core::option::Option<*const Common::D2D_RECT_F>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1GdiMetafile>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DrawGdiMetafile2)(::windows::core::Vtable::as_raw(self), gdimetafile.into().abi(), ::core::mem::transmute(destinationrectangle.unwrap_or(::std::ptr::null())), ::core::mem::transmute(sourcerectangle.unwrap_or(::std::ptr::null())))
    }
    pub unsafe fn CreateTransformedImageSource<P0>(&self, imagesource: P0, properties: *const D2D1_TRANSFORMED_IMAGE_SOURCE_PROPERTIES) -> ::windows::core::Result<ID2D1TransformedImageSource>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1ImageSource>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateTransformedImageSource)(::windows::core::Vtable::as_raw(self), imagesource.into().abi(), properties, result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for ID2D1DeviceContext4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1DeviceContext4 {}
impl ::core::fmt::Debug for ID2D1DeviceContext4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1DeviceContext4").field(&self.0).finish()
    }
}
impl ID2D1DeviceContext4 {
    pub unsafe fn GetFactory(&self) -> ::windows::core::Result<ID2D1Factory> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.GetFactory)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1Factory as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateBitmap(&self, size: Common::D2D_SIZE_U, srcdata: ::core::option::Option<*const ::core::ffi::c_void>, pitch: u32, bitmapproperties: *const D2D1_BITMAP_PROPERTIES) -> ::windows::core::Result<ID2D1Bitmap> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateBitmap)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(size), ::core::mem::transmute(srcdata.unwrap_or(::std::ptr::null())), pitch, bitmapproperties, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Imaging\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
    pub unsafe fn CreateBitmapFromWicBitmap<P0>(&self, wicbitmapsource: P0, bitmapproperties: ::core::option::Option<*const D2D1_BITMAP_PROPERTIES>) -> ::windows::core::Result<ID2D1Bitmap>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Imaging::IWICBitmapSource>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateBitmapFromWicBitmap)(::windows::core::Vtable::as_raw(self), wicbitmapsource.into().abi(), ::core::mem::transmute(bitmapproperties.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSharedBitmap(&self, riid: *const ::windows::core::GUID, data: *mut ::core::ffi::c_void, bitmapproperties: ::core::option::Option<*const D2D1_BITMAP_PROPERTIES>, bitmap: *mut ::core::option::Option<ID2D1Bitmap>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateSharedBitmap)(::windows::core::Vtable::as_raw(self), riid, data, ::core::mem::transmute(bitmapproperties.unwrap_or(::std::ptr::null())), ::core::mem::transmute(bitmap)).ok()
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn CreateBitmapBrush<P0>(&self, bitmap: P0, bitmapbrushproperties: ::core::option::Option<*const D2D1_BITMAP_BRUSH_PROPERTIES>, brushproperties: ::core::option::Option<*const D2D1_BRUSH_PROPERTIES>) -> ::windows::core::Result<ID2D1BitmapBrush>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Bitmap>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateBitmapBrush)(::windows::core::Vtable::as_raw(self), bitmap.into().abi(), ::core::mem::transmute(bitmapbrushproperties.unwrap_or(::std::ptr::null())), ::core::mem::transmute(brushproperties.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn CreateSolidColorBrush(&self, color: *const Common::D2D1_COLOR_F, brushproperties: ::core::option::Option<*const D2D1_BRUSH_PROPERTIES>) -> ::windows::core::Result<ID2D1SolidColorBrush> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateSolidColorBrush)(::windows::core::Vtable::as_raw(self), color, ::core::mem::transmute(brushproperties.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CreateGradientStopCollection(&self, gradientstops: &[D2D1_GRADIENT_STOP], colorinterpolationgamma: D2D1_GAMMA, extendmode: D2D1_EXTEND_MODE) -> ::windows::core::Result<ID2D1GradientStopCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateGradientStopCollection)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(gradientstops.as_ptr()), gradientstops.len() as _, colorinterpolationgamma, extendmode, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn CreateLinearGradientBrush<P0>(&self, lineargradientbrushproperties: *const D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES, brushproperties: ::core::option::Option<*const D2D1_BRUSH_PROPERTIES>, gradientstopcollection: P0) -> ::windows::core::Result<ID2D1LinearGradientBrush>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1GradientStopCollection>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateLinearGradientBrush)(::windows::core::Vtable::as_raw(self), lineargradientbrushproperties, ::core::mem::transmute(brushproperties.unwrap_or(::std::ptr::null())), gradientstopcollection.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn CreateRadialGradientBrush<P0>(&self, radialgradientbrushproperties: *const D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES, brushproperties: ::core::option::Option<*const D2D1_BRUSH_PROPERTIES>, gradientstopcollection: P0) -> ::windows::core::Result<ID2D1RadialGradientBrush>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1GradientStopCollection>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateRadialGradientBrush)(::windows::core::Vtable::as_raw(self), radialgradientbrushproperties, ::core::mem::transmute(brushproperties.unwrap_or(::std::ptr::null())), gradientstopcollection.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateCompatibleRenderTarget(&self, desiredsize: ::core::option::Option<*const Common::D2D_SIZE_F>, desiredpixelsize: ::core::option::Option<*const Common::D2D_SIZE_U>, desiredformat: ::core::option::Option<*const Common::D2D1_PIXEL_FORMAT>, options: D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS) -> ::windows::core::Result<ID2D1BitmapRenderTarget> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateCompatibleRenderTarget)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(desiredsize.unwrap_or(::std::ptr::null())), ::core::mem::transmute(desiredpixelsize.unwrap_or(::std::ptr::null())), ::core::mem::transmute(desiredformat.unwrap_or(::std::ptr::null())), options, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CreateLayer(&self, size: ::core::option::Option<*const Common::D2D_SIZE_F>) -> ::windows::core::Result<ID2D1Layer> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateLayer)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(size.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateMesh(&self) -> ::windows::core::Result<ID2D1Mesh> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateMesh)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawLine<P0, P1>(&self, point0: Common::D2D_POINT_2F, point1: Common::D2D_POINT_2F, brush: P0, strokewidth: f32, strokestyle: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.DrawLine)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(point0), ::core::mem::transmute(point1), brush.into().abi(), strokewidth, strokestyle.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawRectangle<P0, P1>(&self, rect: *const Common::D2D_RECT_F, brush: P0, strokewidth: f32, strokestyle: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.DrawRectangle)(::windows::core::Vtable::as_raw(self), rect, brush.into().abi(), strokewidth, strokestyle.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn FillRectangle<P0>(&self, rect: *const Common::D2D_RECT_F, brush: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.FillRectangle)(::windows::core::Vtable::as_raw(self), rect, brush.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawRoundedRectangle<P0, P1>(&self, roundedrect: *const D2D1_ROUNDED_RECT, brush: P0, strokewidth: f32, strokestyle: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.DrawRoundedRectangle)(::windows::core::Vtable::as_raw(self), roundedrect, brush.into().abi(), strokewidth, strokestyle.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn FillRoundedRectangle<P0>(&self, roundedrect: *const D2D1_ROUNDED_RECT, brush: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.FillRoundedRectangle)(::windows::core::Vtable::as_raw(self), roundedrect, brush.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawEllipse<P0, P1>(&self, ellipse: *const D2D1_ELLIPSE, brush: P0, strokewidth: f32, strokestyle: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.DrawEllipse)(::windows::core::Vtable::as_raw(self), ellipse, brush.into().abi(), strokewidth, strokestyle.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn FillEllipse<P0>(&self, ellipse: *const D2D1_ELLIPSE, brush: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.FillEllipse)(::windows::core::Vtable::as_raw(self), ellipse, brush.into().abi())
    }
    pub unsafe fn DrawGeometry<P0, P1, P2>(&self, geometry: P0, brush: P1, strokewidth: f32, strokestyle: P2)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Geometry>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P2: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.DrawGeometry)(::windows::core::Vtable::as_raw(self), geometry.into().abi(), brush.into().abi(), strokewidth, strokestyle.into().abi())
    }
    pub unsafe fn FillGeometry<P0, P1, P2>(&self, geometry: P0, brush: P1, opacitybrush: P2)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Geometry>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P2: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.FillGeometry)(::windows::core::Vtable::as_raw(self), geometry.into().abi(), brush.into().abi(), opacitybrush.into().abi())
    }
    pub unsafe fn FillMesh<P0, P1>(&self, mesh: P0, brush: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Mesh>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.FillMesh)(::windows::core::Vtable::as_raw(self), mesh.into().abi(), brush.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn FillOpacityMask<P0, P1>(&self, opacitymask: P0, brush: P1, content: D2D1_OPACITY_MASK_CONTENT, destinationrectangle: ::core::option::Option<*const Common::D2D_RECT_F>, sourcerectangle: ::core::option::Option<*const Common::D2D_RECT_F>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Bitmap>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.FillOpacityMask)(::windows::core::Vtable::as_raw(self), opacitymask.into().abi(), brush.into().abi(), content, ::core::mem::transmute(destinationrectangle.unwrap_or(::std::ptr::null())), ::core::mem::transmute(sourcerectangle.unwrap_or(::std::ptr::null())))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawBitmap<P0>(&self, bitmap: P0, destinationrectangle: ::core::option::Option<*const Common::D2D_RECT_F>, opacity: f32, interpolationmode: D2D1_BITMAP_INTERPOLATION_MODE, sourcerectangle: ::core::option::Option<*const Common::D2D_RECT_F>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Bitmap>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.DrawBitmap)(::windows::core::Vtable::as_raw(self), bitmap.into().abi(), ::core::mem::transmute(destinationrectangle.unwrap_or(::std::ptr::null())), opacity, interpolationmode, ::core::mem::transmute(sourcerectangle.unwrap_or(::std::ptr::null())))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn DrawText<P0, P1>(&self, string: &[u16], textformat: P0, layoutrect: *const Common::D2D_RECT_F, defaultfillbrush: P1, options: D2D1_DRAW_TEXT_OPTIONS, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::DirectWrite::IDWriteTextFormat>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.DrawText)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(string.as_ptr()), string.len() as _, textformat.into().abi(), layoutrect, defaultfillbrush.into().abi(), options, measuringmode)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn DrawTextLayout<P0, P1>(&self, origin: Common::D2D_POINT_2F, textlayout: P0, defaultfillbrush: P1, options: D2D1_DRAW_TEXT_OPTIONS)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::DirectWrite::IDWriteTextLayout>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.DrawTextLayout)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(origin), textlayout.into().abi(), defaultfillbrush.into().abi(), options)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn DrawGlyphRun<P0>(&self, baselineorigin: Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, foregroundbrush: P0, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.DrawGlyphRun)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(baselineorigin), glyphrun, foregroundbrush.into().abi(), measuringmode)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn SetTransform(&self, transform: *const super::super::super::Foundation::Numerics::Matrix3x2) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetTransform)(::windows::core::Vtable::as_raw(self), transform)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn GetTransform(&self, transform: *mut super::super::super::Foundation::Numerics::Matrix3x2) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetTransform)(::windows::core::Vtable::as_raw(self), transform)
    }
    pub unsafe fn SetAntialiasMode(&self, antialiasmode: D2D1_ANTIALIAS_MODE) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetAntialiasMode)(::windows::core::Vtable::as_raw(self), antialiasmode)
    }
    pub unsafe fn GetAntialiasMode(&self) -> D2D1_ANTIALIAS_MODE {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetAntialiasMode)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn SetTextAntialiasMode(&self, textantialiasmode: D2D1_TEXT_ANTIALIAS_MODE) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetTextAntialiasMode)(::windows::core::Vtable::as_raw(self), textantialiasmode)
    }
    pub unsafe fn GetTextAntialiasMode(&self) -> D2D1_TEXT_ANTIALIAS_MODE {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetTextAntialiasMode)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(feature = "Win32_Graphics_DirectWrite")]
    pub unsafe fn SetTextRenderingParams<P0>(&self, textrenderingparams: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::DirectWrite::IDWriteRenderingParams>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetTextRenderingParams)(::windows::core::Vtable::as_raw(self), textrenderingparams.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(feature = "Win32_Graphics_DirectWrite")]
    pub unsafe fn GetTextRenderingParams(&self) -> ::windows::core::Result<super::DirectWrite::IDWriteRenderingParams> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetTextRenderingParams)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <super::DirectWrite::IDWriteRenderingParams as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn SetTags(&self, tag1: u64, tag2: u64) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetTags)(::windows::core::Vtable::as_raw(self), tag1, tag2)
    }
    pub unsafe fn GetTags(&self, tag1: ::core::option::Option<*mut u64>, tag2: ::core::option::Option<*mut u64>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetTags)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(tag1.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(tag2.unwrap_or(::std::ptr::null_mut())))
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn PushLayer<P0>(&self, layerparameters: *const D2D1_LAYER_PARAMETERS, layer: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Layer>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.PushLayer)(::windows::core::Vtable::as_raw(self), layerparameters, layer.into().abi())
    }
    pub unsafe fn PopLayer(&self) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.PopLayer)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn Flush(&self, tag1: ::core::option::Option<*mut u64>, tag2: ::core::option::Option<*mut u64>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.Flush)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(tag1.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(tag2.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SaveDrawingState<P0>(&self, drawingstateblock: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1DrawingStateBlock>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SaveDrawingState)(::windows::core::Vtable::as_raw(self), drawingstateblock.into().abi())
    }
    pub unsafe fn RestoreDrawingState<P0>(&self, drawingstateblock: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1DrawingStateBlock>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.RestoreDrawingState)(::windows::core::Vtable::as_raw(self), drawingstateblock.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn PushAxisAlignedClip(&self, cliprect: *const Common::D2D_RECT_F, antialiasmode: D2D1_ANTIALIAS_MODE) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.PushAxisAlignedClip)(::windows::core::Vtable::as_raw(self), cliprect, antialiasmode)
    }
    pub unsafe fn PopAxisAlignedClip(&self) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.PopAxisAlignedClip)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn Clear(&self, clearcolor: ::core::option::Option<*const Common::D2D1_COLOR_F>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.Clear)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(clearcolor.unwrap_or(::std::ptr::null())))
    }
    pub unsafe fn BeginDraw(&self) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.BeginDraw)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn EndDraw(&self, tag1: ::core::option::Option<*mut u64>, tag2: ::core::option::Option<*mut u64>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.EndDraw)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(tag1.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(tag2.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetPixelFormat(&self) -> Common::D2D1_PIXEL_FORMAT {
        let mut result__: Common::D2D1_PIXEL_FORMAT = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetPixelFormat)(::windows::core::Vtable::as_raw(self), &mut result__);
        result__
    }
    pub unsafe fn SetDpi(&self, dpix: f32, dpiy: f32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetDpi)(::windows::core::Vtable::as_raw(self), dpix, dpiy)
    }
    pub unsafe fn GetDpi(&self, dpix: *mut f32, dpiy: *mut f32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetDpi)(::windows::core::Vtable::as_raw(self), dpix, dpiy)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetSize(&self) -> Common::D2D_SIZE_F {
        let mut result__: Common::D2D_SIZE_F = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetSize)(::windows::core::Vtable::as_raw(self), &mut result__);
        result__
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetPixelSize(&self) -> Common::D2D_SIZE_U {
        let mut result__: Common::D2D_SIZE_U = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetPixelSize)(::windows::core::Vtable::as_raw(self), &mut result__);
        result__
    }
    pub unsafe fn GetMaximumBitmapSize(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetMaximumBitmapSize)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn IsSupported(&self, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.IsSupported)(::windows::core::Vtable::as_raw(self), rendertargetproperties)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateBitmap2(&self, size: Common::D2D_SIZE_U, sourcedata: ::core::option::Option<*const ::core::ffi::c_void>, pitch: u32, bitmapproperties: *const D2D1_BITMAP_PROPERTIES1) -> ::windows::core::Result<ID2D1Bitmap1> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateBitmap2)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(size), ::core::mem::transmute(sourcedata.unwrap_or(::std::ptr::null())), pitch, bitmapproperties, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Imaging\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
    pub unsafe fn CreateBitmapFromWicBitmap2<P0>(&self, wicbitmapsource: P0, bitmapproperties: ::core::option::Option<*const D2D1_BITMAP_PROPERTIES1>) -> ::windows::core::Result<ID2D1Bitmap1>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Imaging::IWICBitmapSource>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateBitmapFromWicBitmap2)(::windows::core::Vtable::as_raw(self), wicbitmapsource.into().abi(), ::core::mem::transmute(bitmapproperties.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateColorContext(&self, space: D2D1_COLOR_SPACE, profile: ::core::option::Option<&[u8]>) -> ::windows::core::Result<ID2D1ColorContext> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateColorContext)(::windows::core::Vtable::as_raw(self), space, ::core::mem::transmute(profile.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), profile.as_deref().map_or(0, |slice| slice.len() as _), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateColorContextFromFilename<P0>(&self, filename: P0) -> ::windows::core::Result<ID2D1ColorContext>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateColorContextFromFilename)(::windows::core::Vtable::as_raw(self), filename.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub unsafe fn CreateColorContextFromWicColorContext<P0>(&self, wiccolorcontext: P0) -> ::windows::core::Result<ID2D1ColorContext>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Imaging::IWICColorContext>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateColorContextFromWicColorContext)(::windows::core::Vtable::as_raw(self), wiccolorcontext.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateBitmapFromDxgiSurface<P0>(&self, surface: P0, bitmapproperties: ::core::option::Option<*const D2D1_BITMAP_PROPERTIES1>) -> ::windows::core::Result<ID2D1Bitmap1>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Dxgi::IDXGISurface>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateBitmapFromDxgiSurface)(::windows::core::Vtable::as_raw(self), surface.into().abi(), ::core::mem::transmute(bitmapproperties.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateEffect(&self, effectid: *const ::windows::core::GUID) -> ::windows::core::Result<ID2D1Effect> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateEffect)(::windows::core::Vtable::as_raw(self), effectid, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CreateGradientStopCollection2(&self, straightalphagradientstops: &[D2D1_GRADIENT_STOP], preinterpolationspace: D2D1_COLOR_SPACE, postinterpolationspace: D2D1_COLOR_SPACE, bufferprecision: D2D1_BUFFER_PRECISION, extendmode: D2D1_EXTEND_MODE, colorinterpolationmode: D2D1_COLOR_INTERPOLATION_MODE) -> ::windows::core::Result<ID2D1GradientStopCollection1> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateGradientStopCollection2)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(straightalphagradientstops.as_ptr()), straightalphagradientstops.len() as _, preinterpolationspace, postinterpolationspace, bufferprecision, extendmode, colorinterpolationmode, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn CreateImageBrush<P0>(&self, image: P0, imagebrushproperties: *const D2D1_IMAGE_BRUSH_PROPERTIES, brushproperties: ::core::option::Option<*const D2D1_BRUSH_PROPERTIES>) -> ::windows::core::Result<ID2D1ImageBrush>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Image>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateImageBrush)(::windows::core::Vtable::as_raw(self), image.into().abi(), imagebrushproperties, ::core::mem::transmute(brushproperties.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn CreateBitmapBrush2<P0>(&self, bitmap: P0, bitmapbrushproperties: ::core::option::Option<*const D2D1_BITMAP_BRUSH_PROPERTIES1>, brushproperties: ::core::option::Option<*const D2D1_BRUSH_PROPERTIES>) -> ::windows::core::Result<ID2D1BitmapBrush1>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Bitmap>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateBitmapBrush2)(::windows::core::Vtable::as_raw(self), bitmap.into().abi(), ::core::mem::transmute(bitmapbrushproperties.unwrap_or(::std::ptr::null())), ::core::mem::transmute(brushproperties.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateCommandList(&self) -> ::windows::core::Result<ID2D1CommandList> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateCommandList)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn IsDxgiFormatSupported(&self, format: super::Dxgi::Common::DXGI_FORMAT) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.IsDxgiFormatSupported)(::windows::core::Vtable::as_raw(self), format)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsBufferPrecisionSupported(&self, bufferprecision: D2D1_BUFFER_PRECISION) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.IsBufferPrecisionSupported)(::windows::core::Vtable::as_raw(self), bufferprecision)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetImageLocalBounds<P0>(&self, image: P0) -> ::windows::core::Result<Common::D2D_RECT_F>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Image>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetImageLocalBounds)(::windows::core::Vtable::as_raw(self), image.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetImageWorldBounds<P0>(&self, image: P0) -> ::windows::core::Result<Common::D2D_RECT_F>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Image>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetImageWorldBounds)(::windows::core::Vtable::as_raw(self), image.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn GetGlyphRunWorldBounds(&self, baselineorigin: Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE) -> ::windows::core::Result<Common::D2D_RECT_F> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetGlyphRunWorldBounds)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(baselineorigin), glyphrun, measuringmode, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<ID2D1Device> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1Device as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn SetTarget<P0>(&self, image: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Image>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetTarget)(::windows::core::Vtable::as_raw(self), image.into().abi())
    }
    pub unsafe fn GetTarget(&self) -> ::windows::core::Result<ID2D1Image> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetTarget)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1Image as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetRenderingControls(&self, renderingcontrols: *const D2D1_RENDERING_CONTROLS) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetRenderingControls)(::windows::core::Vtable::as_raw(self), renderingcontrols)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetRenderingControls(&self) -> D2D1_RENDERING_CONTROLS {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetRenderingControls)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        result__.assume_init()
    }
    pub unsafe fn SetPrimitiveBlend(&self, primitiveblend: D2D1_PRIMITIVE_BLEND) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetPrimitiveBlend)(::windows::core::Vtable::as_raw(self), primitiveblend)
    }
    pub unsafe fn GetPrimitiveBlend(&self) -> D2D1_PRIMITIVE_BLEND {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetPrimitiveBlend)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn SetUnitMode(&self, unitmode: D2D1_UNIT_MODE) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetUnitMode)(::windows::core::Vtable::as_raw(self), unitmode)
    }
    pub unsafe fn GetUnitMode(&self) -> D2D1_UNIT_MODE {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetUnitMode)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn DrawGlyphRun2<P0>(&self, baselineorigin: Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, glyphrundescription: ::core::option::Option<*const super::DirectWrite::DWRITE_GLYPH_RUN_DESCRIPTION>, foregroundbrush: P0, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.DrawGlyphRun2)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(baselineorigin), glyphrun, ::core::mem::transmute(glyphrundescription.unwrap_or(::std::ptr::null())), foregroundbrush.into().abi(), measuringmode)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawImage<P0>(&self, image: P0, targetoffset: ::core::option::Option<*const Common::D2D_POINT_2F>, imagerectangle: ::core::option::Option<*const Common::D2D_RECT_F>, interpolationmode: D2D1_INTERPOLATION_MODE, compositemode: Common::D2D1_COMPOSITE_MODE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Image>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.DrawImage)(::windows::core::Vtable::as_raw(self), image.into().abi(), ::core::mem::transmute(targetoffset.unwrap_or(::std::ptr::null())), ::core::mem::transmute(imagerectangle.unwrap_or(::std::ptr::null())), interpolationmode, compositemode)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawGdiMetafile<P0>(&self, gdimetafile: P0, targetoffset: ::core::option::Option<*const Common::D2D_POINT_2F>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1GdiMetafile>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.DrawGdiMetafile)(::windows::core::Vtable::as_raw(self), gdimetafile.into().abi(), ::core::mem::transmute(targetoffset.unwrap_or(::std::ptr::null())))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawBitmap2<P0>(&self, bitmap: P0, destinationrectangle: ::core::option::Option<*const Common::D2D_RECT_F>, opacity: f32, interpolationmode: D2D1_INTERPOLATION_MODE, sourcerectangle: ::core::option::Option<*const Common::D2D_RECT_F>, perspectivetransform: ::core::option::Option<*const Common::D2D_MATRIX_4X4_F>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Bitmap>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.DrawBitmap2)(::windows::core::Vtable::as_raw(self), bitmap.into().abi(), ::core::mem::transmute(destinationrectangle.unwrap_or(::std::ptr::null())), opacity, interpolationmode, ::core::mem::transmute(sourcerectangle.unwrap_or(::std::ptr::null())), ::core::mem::transmute(perspectivetransform.unwrap_or(::std::ptr::null())))
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn PushLayer2<P0>(&self, layerparameters: *const D2D1_LAYER_PARAMETERS1, layer: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Layer>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.PushLayer2)(::windows::core::Vtable::as_raw(self), layerparameters, layer.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn InvalidateEffectInputRectangle<P0>(&self, effect: P0, input: u32, inputrectangle: *const Common::D2D_RECT_F) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Effect>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.InvalidateEffectInputRectangle)(::windows::core::Vtable::as_raw(self), effect.into().abi(), input, inputrectangle).ok()
    }
    pub unsafe fn GetEffectInvalidRectangleCount<P0>(&self, effect: P0) -> ::windows::core::Result<u32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Effect>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetEffectInvalidRectangleCount)(::windows::core::Vtable::as_raw(self), effect.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetEffectInvalidRectangles<P0>(&self, effect: P0, rectangles: &mut [Common::D2D_RECT_F]) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Effect>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetEffectInvalidRectangles)(::windows::core::Vtable::as_raw(self), effect.into().abi(), ::core::mem::transmute(rectangles.as_ptr()), rectangles.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetEffectRequiredInputRectangles<P0>(&self, rendereffect: P0, renderimagerectangle: ::core::option::Option<*const Common::D2D_RECT_F>, inputdescriptions: *const D2D1_EFFECT_INPUT_DESCRIPTION, requiredinputrects: *mut Common::D2D_RECT_F, inputcount: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Effect>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetEffectRequiredInputRectangles)(::windows::core::Vtable::as_raw(self), rendereffect.into().abi(), ::core::mem::transmute(renderimagerectangle.unwrap_or(::std::ptr::null())), inputdescriptions, requiredinputrects, inputcount).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn FillOpacityMask2<P0, P1>(&self, opacitymask: P0, brush: P1, destinationrectangle: ::core::option::Option<*const Common::D2D_RECT_F>, sourcerectangle: ::core::option::Option<*const Common::D2D_RECT_F>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Bitmap>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.FillOpacityMask2)(::windows::core::Vtable::as_raw(self), opacitymask.into().abi(), brush.into().abi(), ::core::mem::transmute(destinationrectangle.unwrap_or(::std::ptr::null())), ::core::mem::transmute(sourcerectangle.unwrap_or(::std::ptr::null())))
    }
    pub unsafe fn CreateFilledGeometryRealization<P0>(&self, geometry: P0, flatteningtolerance: f32) -> ::windows::core::Result<ID2D1GeometryRealization>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Geometry>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateFilledGeometryRealization)(::windows::core::Vtable::as_raw(self), geometry.into().abi(), flatteningtolerance, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateStrokedGeometryRealization<P0, P1>(&self, geometry: P0, flatteningtolerance: f32, strokewidth: f32, strokestyle: P1) -> ::windows::core::Result<ID2D1GeometryRealization>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Geometry>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateStrokedGeometryRealization)(::windows::core::Vtable::as_raw(self), geometry.into().abi(), flatteningtolerance, strokewidth, strokestyle.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn DrawGeometryRealization<P0, P1>(&self, geometryrealization: P0, brush: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1GeometryRealization>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.DrawGeometryRealization)(::windows::core::Vtable::as_raw(self), geometryrealization.into().abi(), brush.into().abi())
    }
    pub unsafe fn CreateInk(&self, startpoint: *const D2D1_INK_POINT) -> ::windows::core::Result<ID2D1Ink> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateInk)(::windows::core::Vtable::as_raw(self), startpoint, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn CreateInkStyle(&self, inkstyleproperties: ::core::option::Option<*const D2D1_INK_STYLE_PROPERTIES>) -> ::windows::core::Result<ID2D1InkStyle> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateInkStyle)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(inkstyleproperties.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CreateGradientMesh(&self, patches: &[D2D1_GRADIENT_MESH_PATCH]) -> ::windows::core::Result<ID2D1GradientMesh> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateGradientMesh)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(patches.as_ptr()), patches.len() as _, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Imaging\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Imaging"))]
    pub unsafe fn CreateImageSourceFromWic<P0>(&self, wicbitmapsource: P0, loadingoptions: D2D1_IMAGE_SOURCE_LOADING_OPTIONS, alphamode: Common::D2D1_ALPHA_MODE) -> ::windows::core::Result<ID2D1ImageSourceFromWic>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Imaging::IWICBitmapSource>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateImageSourceFromWic)(::windows::core::Vtable::as_raw(self), wicbitmapsource.into().abi(), loadingoptions, alphamode, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateLookupTable3D(&self, precision: D2D1_BUFFER_PRECISION, extents: &[u32; 3], data: &[u8], strides: &[u32; 2]) -> ::windows::core::Result<ID2D1LookupTable3D> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateLookupTable3D)(::windows::core::Vtable::as_raw(self), precision, ::core::mem::transmute(extents.as_ptr()), ::core::mem::transmute(data.as_ptr()), data.len() as _, ::core::mem::transmute(strides.as_ptr()), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateImageSourceFromDxgi(&self, surfaces: &[super::Dxgi::IDXGISurface], colorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE, options: D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS) -> ::windows::core::Result<ID2D1ImageSource> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateImageSourceFromDxgi)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(surfaces.as_ptr()), surfaces.len() as _, colorspace, options, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetGradientMeshWorldBounds<P0>(&self, gradientmesh: P0) -> ::windows::core::Result<Common::D2D_RECT_F>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1GradientMesh>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetGradientMeshWorldBounds)(::windows::core::Vtable::as_raw(self), gradientmesh.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn DrawInk<P0, P1, P2>(&self, ink: P0, brush: P1, inkstyle: P2)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Ink>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P2: ::std::convert::Into<::windows::core::InParam<ID2D1InkStyle>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.DrawInk)(::windows::core::Vtable::as_raw(self), ink.into().abi(), brush.into().abi(), inkstyle.into().abi())
    }
    pub unsafe fn DrawGradientMesh<P0>(&self, gradientmesh: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1GradientMesh>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.DrawGradientMesh)(::windows::core::Vtable::as_raw(self), gradientmesh.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawGdiMetafile2<P0>(&self, gdimetafile: P0, destinationrectangle: ::core::option::Option<*const Common::D2D_RECT_F>, sourcerectangle: ::core::option::Option<*const Common::D2D_RECT_F>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1GdiMetafile>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.DrawGdiMetafile2)(::windows::core::Vtable::as_raw(self), gdimetafile.into().abi(), ::core::mem::transmute(destinationrectangle.unwrap_or(::std::ptr::null())), ::core::mem::transmute(sourcerectangle.unwrap_or(::std::ptr::null())))
    }
    pub unsafe fn CreateTransformedImageSource<P0>(&self, imagesource: P0, properties: *const D2D1_TRANSFORMED_IMAGE_SOURCE_PROPERTIES) -> ::windows::core::Result<ID2D1TransformedImageSource>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1ImageSource>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateTransformedImageSource)(::windows::core::Vtable::as_raw(self), imagesource.into().abi(), properties, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateSpriteBatch(&self) -> ::windows::core::Result<ID2D1SpriteBatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateSpriteBatch)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn DrawSpriteBatch<P0, P1>(&self, spritebatch: P0, startindex: u32, spritecount: u32, bitmap: P1, interpolationmode: D2D1_BITMAP_INTERPOLATION_MODE, spriteoptions: D2D1_SPRITE_OPTIONS)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1SpriteBatch>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Bitmap>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DrawSpriteBatch)(::windows::core::Vtable::as_raw(self), spritebatch.into().abi(), startindex, spritecount, bitmap.into().abi(), interpolationmode, spriteoptions)
    }
}
impl ::core::cmp::PartialEq for ID2D1DeviceContext5 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1DeviceContext5 {}
impl ::core::fmt::Debug for ID2D1DeviceContext5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1DeviceContext5").field(&self.0).finish()
    }
}
impl ID2D1DeviceContext5 {
    pub unsafe fn GetFactory(&self) -> ::windows::core::Result<ID2D1Factory> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.GetFactory)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1Factory as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateBitmap(&self, size: Common::D2D_SIZE_U, srcdata: ::core::option::Option<*const ::core::ffi::c_void>, pitch: u32, bitmapproperties: *const D2D1_BITMAP_PROPERTIES) -> ::windows::core::Result<ID2D1Bitmap> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CreateBitmap)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(size), ::core::mem::transmute(srcdata.unwrap_or(::std::ptr::null())), pitch, bitmapproperties, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Imaging\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
    pub unsafe fn CreateBitmapFromWicBitmap<P0>(&self, wicbitmapsource: P0, bitmapproperties: ::core::option::Option<*const D2D1_BITMAP_PROPERTIES>) -> ::windows::core::Result<ID2D1Bitmap>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Imaging::IWICBitmapSource>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CreateBitmapFromWicBitmap)(::windows::core::Vtable::as_raw(self), wicbitmapsource.into().abi(), ::core::mem::transmute(bitmapproperties.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSharedBitmap(&self, riid: *const ::windows::core::GUID, data: *mut ::core::ffi::c_void, bitmapproperties: ::core::option::Option<*const D2D1_BITMAP_PROPERTIES>, bitmap: *mut ::core::option::Option<ID2D1Bitmap>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CreateSharedBitmap)(::windows::core::Vtable::as_raw(self), riid, data, ::core::mem::transmute(bitmapproperties.unwrap_or(::std::ptr::null())), ::core::mem::transmute(bitmap)).ok()
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn CreateBitmapBrush<P0>(&self, bitmap: P0, bitmapbrushproperties: ::core::option::Option<*const D2D1_BITMAP_BRUSH_PROPERTIES>, brushproperties: ::core::option::Option<*const D2D1_BRUSH_PROPERTIES>) -> ::windows::core::Result<ID2D1BitmapBrush>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Bitmap>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CreateBitmapBrush)(::windows::core::Vtable::as_raw(self), bitmap.into().abi(), ::core::mem::transmute(bitmapbrushproperties.unwrap_or(::std::ptr::null())), ::core::mem::transmute(brushproperties.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn CreateSolidColorBrush(&self, color: *const Common::D2D1_COLOR_F, brushproperties: ::core::option::Option<*const D2D1_BRUSH_PROPERTIES>) -> ::windows::core::Result<ID2D1SolidColorBrush> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CreateSolidColorBrush)(::windows::core::Vtable::as_raw(self), color, ::core::mem::transmute(brushproperties.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CreateGradientStopCollection(&self, gradientstops: &[D2D1_GRADIENT_STOP], colorinterpolationgamma: D2D1_GAMMA, extendmode: D2D1_EXTEND_MODE) -> ::windows::core::Result<ID2D1GradientStopCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CreateGradientStopCollection)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(gradientstops.as_ptr()), gradientstops.len() as _, colorinterpolationgamma, extendmode, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn CreateLinearGradientBrush<P0>(&self, lineargradientbrushproperties: *const D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES, brushproperties: ::core::option::Option<*const D2D1_BRUSH_PROPERTIES>, gradientstopcollection: P0) -> ::windows::core::Result<ID2D1LinearGradientBrush>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1GradientStopCollection>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CreateLinearGradientBrush)(::windows::core::Vtable::as_raw(self), lineargradientbrushproperties, ::core::mem::transmute(brushproperties.unwrap_or(::std::ptr::null())), gradientstopcollection.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn CreateRadialGradientBrush<P0>(&self, radialgradientbrushproperties: *const D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES, brushproperties: ::core::option::Option<*const D2D1_BRUSH_PROPERTIES>, gradientstopcollection: P0) -> ::windows::core::Result<ID2D1RadialGradientBrush>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1GradientStopCollection>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CreateRadialGradientBrush)(::windows::core::Vtable::as_raw(self), radialgradientbrushproperties, ::core::mem::transmute(brushproperties.unwrap_or(::std::ptr::null())), gradientstopcollection.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateCompatibleRenderTarget(&self, desiredsize: ::core::option::Option<*const Common::D2D_SIZE_F>, desiredpixelsize: ::core::option::Option<*const Common::D2D_SIZE_U>, desiredformat: ::core::option::Option<*const Common::D2D1_PIXEL_FORMAT>, options: D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS) -> ::windows::core::Result<ID2D1BitmapRenderTarget> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CreateCompatibleRenderTarget)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(desiredsize.unwrap_or(::std::ptr::null())), ::core::mem::transmute(desiredpixelsize.unwrap_or(::std::ptr::null())), ::core::mem::transmute(desiredformat.unwrap_or(::std::ptr::null())), options, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CreateLayer(&self, size: ::core::option::Option<*const Common::D2D_SIZE_F>) -> ::windows::core::Result<ID2D1Layer> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CreateLayer)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(size.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateMesh(&self) -> ::windows::core::Result<ID2D1Mesh> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CreateMesh)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawLine<P0, P1>(&self, point0: Common::D2D_POINT_2F, point1: Common::D2D_POINT_2F, brush: P0, strokewidth: f32, strokestyle: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.DrawLine)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(point0), ::core::mem::transmute(point1), brush.into().abi(), strokewidth, strokestyle.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawRectangle<P0, P1>(&self, rect: *const Common::D2D_RECT_F, brush: P0, strokewidth: f32, strokestyle: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.DrawRectangle)(::windows::core::Vtable::as_raw(self), rect, brush.into().abi(), strokewidth, strokestyle.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn FillRectangle<P0>(&self, rect: *const Common::D2D_RECT_F, brush: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.FillRectangle)(::windows::core::Vtable::as_raw(self), rect, brush.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawRoundedRectangle<P0, P1>(&self, roundedrect: *const D2D1_ROUNDED_RECT, brush: P0, strokewidth: f32, strokestyle: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.DrawRoundedRectangle)(::windows::core::Vtable::as_raw(self), roundedrect, brush.into().abi(), strokewidth, strokestyle.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn FillRoundedRectangle<P0>(&self, roundedrect: *const D2D1_ROUNDED_RECT, brush: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.FillRoundedRectangle)(::windows::core::Vtable::as_raw(self), roundedrect, brush.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawEllipse<P0, P1>(&self, ellipse: *const D2D1_ELLIPSE, brush: P0, strokewidth: f32, strokestyle: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.DrawEllipse)(::windows::core::Vtable::as_raw(self), ellipse, brush.into().abi(), strokewidth, strokestyle.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn FillEllipse<P0>(&self, ellipse: *const D2D1_ELLIPSE, brush: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.FillEllipse)(::windows::core::Vtable::as_raw(self), ellipse, brush.into().abi())
    }
    pub unsafe fn DrawGeometry<P0, P1, P2>(&self, geometry: P0, brush: P1, strokewidth: f32, strokestyle: P2)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Geometry>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P2: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.DrawGeometry)(::windows::core::Vtable::as_raw(self), geometry.into().abi(), brush.into().abi(), strokewidth, strokestyle.into().abi())
    }
    pub unsafe fn FillGeometry<P0, P1, P2>(&self, geometry: P0, brush: P1, opacitybrush: P2)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Geometry>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P2: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.FillGeometry)(::windows::core::Vtable::as_raw(self), geometry.into().abi(), brush.into().abi(), opacitybrush.into().abi())
    }
    pub unsafe fn FillMesh<P0, P1>(&self, mesh: P0, brush: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Mesh>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.FillMesh)(::windows::core::Vtable::as_raw(self), mesh.into().abi(), brush.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn FillOpacityMask<P0, P1>(&self, opacitymask: P0, brush: P1, content: D2D1_OPACITY_MASK_CONTENT, destinationrectangle: ::core::option::Option<*const Common::D2D_RECT_F>, sourcerectangle: ::core::option::Option<*const Common::D2D_RECT_F>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Bitmap>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.FillOpacityMask)(::windows::core::Vtable::as_raw(self), opacitymask.into().abi(), brush.into().abi(), content, ::core::mem::transmute(destinationrectangle.unwrap_or(::std::ptr::null())), ::core::mem::transmute(sourcerectangle.unwrap_or(::std::ptr::null())))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawBitmap<P0>(&self, bitmap: P0, destinationrectangle: ::core::option::Option<*const Common::D2D_RECT_F>, opacity: f32, interpolationmode: D2D1_BITMAP_INTERPOLATION_MODE, sourcerectangle: ::core::option::Option<*const Common::D2D_RECT_F>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Bitmap>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.DrawBitmap)(::windows::core::Vtable::as_raw(self), bitmap.into().abi(), ::core::mem::transmute(destinationrectangle.unwrap_or(::std::ptr::null())), opacity, interpolationmode, ::core::mem::transmute(sourcerectangle.unwrap_or(::std::ptr::null())))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn DrawText<P0, P1>(&self, string: &[u16], textformat: P0, layoutrect: *const Common::D2D_RECT_F, defaultfillbrush: P1, options: D2D1_DRAW_TEXT_OPTIONS, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::DirectWrite::IDWriteTextFormat>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.DrawText)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(string.as_ptr()), string.len() as _, textformat.into().abi(), layoutrect, defaultfillbrush.into().abi(), options, measuringmode)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn DrawTextLayout<P0, P1>(&self, origin: Common::D2D_POINT_2F, textlayout: P0, defaultfillbrush: P1, options: D2D1_DRAW_TEXT_OPTIONS)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::DirectWrite::IDWriteTextLayout>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.DrawTextLayout)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(origin), textlayout.into().abi(), defaultfillbrush.into().abi(), options)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn DrawGlyphRun<P0>(&self, baselineorigin: Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, foregroundbrush: P0, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.DrawGlyphRun)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(baselineorigin), glyphrun, foregroundbrush.into().abi(), measuringmode)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn SetTransform(&self, transform: *const super::super::super::Foundation::Numerics::Matrix3x2) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.SetTransform)(::windows::core::Vtable::as_raw(self), transform)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn GetTransform(&self, transform: *mut super::super::super::Foundation::Numerics::Matrix3x2) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.GetTransform)(::windows::core::Vtable::as_raw(self), transform)
    }
    pub unsafe fn SetAntialiasMode(&self, antialiasmode: D2D1_ANTIALIAS_MODE) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.SetAntialiasMode)(::windows::core::Vtable::as_raw(self), antialiasmode)
    }
    pub unsafe fn GetAntialiasMode(&self) -> D2D1_ANTIALIAS_MODE {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.GetAntialiasMode)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn SetTextAntialiasMode(&self, textantialiasmode: D2D1_TEXT_ANTIALIAS_MODE) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.SetTextAntialiasMode)(::windows::core::Vtable::as_raw(self), textantialiasmode)
    }
    pub unsafe fn GetTextAntialiasMode(&self) -> D2D1_TEXT_ANTIALIAS_MODE {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.GetTextAntialiasMode)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(feature = "Win32_Graphics_DirectWrite")]
    pub unsafe fn SetTextRenderingParams<P0>(&self, textrenderingparams: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::DirectWrite::IDWriteRenderingParams>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.SetTextRenderingParams)(::windows::core::Vtable::as_raw(self), textrenderingparams.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(feature = "Win32_Graphics_DirectWrite")]
    pub unsafe fn GetTextRenderingParams(&self) -> ::windows::core::Result<super::DirectWrite::IDWriteRenderingParams> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.GetTextRenderingParams)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <super::DirectWrite::IDWriteRenderingParams as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn SetTags(&self, tag1: u64, tag2: u64) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.SetTags)(::windows::core::Vtable::as_raw(self), tag1, tag2)
    }
    pub unsafe fn GetTags(&self, tag1: ::core::option::Option<*mut u64>, tag2: ::core::option::Option<*mut u64>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.GetTags)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(tag1.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(tag2.unwrap_or(::std::ptr::null_mut())))
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn PushLayer<P0>(&self, layerparameters: *const D2D1_LAYER_PARAMETERS, layer: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Layer>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.PushLayer)(::windows::core::Vtable::as_raw(self), layerparameters, layer.into().abi())
    }
    pub unsafe fn PopLayer(&self) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.PopLayer)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn Flush(&self, tag1: ::core::option::Option<*mut u64>, tag2: ::core::option::Option<*mut u64>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.Flush)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(tag1.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(tag2.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SaveDrawingState<P0>(&self, drawingstateblock: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1DrawingStateBlock>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.SaveDrawingState)(::windows::core::Vtable::as_raw(self), drawingstateblock.into().abi())
    }
    pub unsafe fn RestoreDrawingState<P0>(&self, drawingstateblock: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1DrawingStateBlock>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.RestoreDrawingState)(::windows::core::Vtable::as_raw(self), drawingstateblock.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn PushAxisAlignedClip(&self, cliprect: *const Common::D2D_RECT_F, antialiasmode: D2D1_ANTIALIAS_MODE) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.PushAxisAlignedClip)(::windows::core::Vtable::as_raw(self), cliprect, antialiasmode)
    }
    pub unsafe fn PopAxisAlignedClip(&self) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.PopAxisAlignedClip)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn Clear(&self, clearcolor: ::core::option::Option<*const Common::D2D1_COLOR_F>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.Clear)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(clearcolor.unwrap_or(::std::ptr::null())))
    }
    pub unsafe fn BeginDraw(&self) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.BeginDraw)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn EndDraw(&self, tag1: ::core::option::Option<*mut u64>, tag2: ::core::option::Option<*mut u64>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.EndDraw)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(tag1.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(tag2.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetPixelFormat(&self) -> Common::D2D1_PIXEL_FORMAT {
        let mut result__: Common::D2D1_PIXEL_FORMAT = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.GetPixelFormat)(::windows::core::Vtable::as_raw(self), &mut result__);
        result__
    }
    pub unsafe fn SetDpi(&self, dpix: f32, dpiy: f32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.SetDpi)(::windows::core::Vtable::as_raw(self), dpix, dpiy)
    }
    pub unsafe fn GetDpi(&self, dpix: *mut f32, dpiy: *mut f32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.GetDpi)(::windows::core::Vtable::as_raw(self), dpix, dpiy)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetSize(&self) -> Common::D2D_SIZE_F {
        let mut result__: Common::D2D_SIZE_F = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.GetSize)(::windows::core::Vtable::as_raw(self), &mut result__);
        result__
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetPixelSize(&self) -> Common::D2D_SIZE_U {
        let mut result__: Common::D2D_SIZE_U = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.GetPixelSize)(::windows::core::Vtable::as_raw(self), &mut result__);
        result__
    }
    pub unsafe fn GetMaximumBitmapSize(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.GetMaximumBitmapSize)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn IsSupported(&self, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.IsSupported)(::windows::core::Vtable::as_raw(self), rendertargetproperties)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateBitmap2(&self, size: Common::D2D_SIZE_U, sourcedata: ::core::option::Option<*const ::core::ffi::c_void>, pitch: u32, bitmapproperties: *const D2D1_BITMAP_PROPERTIES1) -> ::windows::core::Result<ID2D1Bitmap1> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateBitmap2)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(size), ::core::mem::transmute(sourcedata.unwrap_or(::std::ptr::null())), pitch, bitmapproperties, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Imaging\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
    pub unsafe fn CreateBitmapFromWicBitmap2<P0>(&self, wicbitmapsource: P0, bitmapproperties: ::core::option::Option<*const D2D1_BITMAP_PROPERTIES1>) -> ::windows::core::Result<ID2D1Bitmap1>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Imaging::IWICBitmapSource>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateBitmapFromWicBitmap2)(::windows::core::Vtable::as_raw(self), wicbitmapsource.into().abi(), ::core::mem::transmute(bitmapproperties.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateColorContext(&self, space: D2D1_COLOR_SPACE, profile: ::core::option::Option<&[u8]>) -> ::windows::core::Result<ID2D1ColorContext> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateColorContext)(::windows::core::Vtable::as_raw(self), space, ::core::mem::transmute(profile.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), profile.as_deref().map_or(0, |slice| slice.len() as _), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateColorContextFromFilename<P0>(&self, filename: P0) -> ::windows::core::Result<ID2D1ColorContext>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateColorContextFromFilename)(::windows::core::Vtable::as_raw(self), filename.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub unsafe fn CreateColorContextFromWicColorContext<P0>(&self, wiccolorcontext: P0) -> ::windows::core::Result<ID2D1ColorContext>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Imaging::IWICColorContext>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateColorContextFromWicColorContext)(::windows::core::Vtable::as_raw(self), wiccolorcontext.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateBitmapFromDxgiSurface<P0>(&self, surface: P0, bitmapproperties: ::core::option::Option<*const D2D1_BITMAP_PROPERTIES1>) -> ::windows::core::Result<ID2D1Bitmap1>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Dxgi::IDXGISurface>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateBitmapFromDxgiSurface)(::windows::core::Vtable::as_raw(self), surface.into().abi(), ::core::mem::transmute(bitmapproperties.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateEffect(&self, effectid: *const ::windows::core::GUID) -> ::windows::core::Result<ID2D1Effect> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateEffect)(::windows::core::Vtable::as_raw(self), effectid, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CreateGradientStopCollection2(&self, straightalphagradientstops: &[D2D1_GRADIENT_STOP], preinterpolationspace: D2D1_COLOR_SPACE, postinterpolationspace: D2D1_COLOR_SPACE, bufferprecision: D2D1_BUFFER_PRECISION, extendmode: D2D1_EXTEND_MODE, colorinterpolationmode: D2D1_COLOR_INTERPOLATION_MODE) -> ::windows::core::Result<ID2D1GradientStopCollection1> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateGradientStopCollection2)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(straightalphagradientstops.as_ptr()), straightalphagradientstops.len() as _, preinterpolationspace, postinterpolationspace, bufferprecision, extendmode, colorinterpolationmode, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn CreateImageBrush<P0>(&self, image: P0, imagebrushproperties: *const D2D1_IMAGE_BRUSH_PROPERTIES, brushproperties: ::core::option::Option<*const D2D1_BRUSH_PROPERTIES>) -> ::windows::core::Result<ID2D1ImageBrush>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Image>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateImageBrush)(::windows::core::Vtable::as_raw(self), image.into().abi(), imagebrushproperties, ::core::mem::transmute(brushproperties.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn CreateBitmapBrush2<P0>(&self, bitmap: P0, bitmapbrushproperties: ::core::option::Option<*const D2D1_BITMAP_BRUSH_PROPERTIES1>, brushproperties: ::core::option::Option<*const D2D1_BRUSH_PROPERTIES>) -> ::windows::core::Result<ID2D1BitmapBrush1>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Bitmap>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateBitmapBrush2)(::windows::core::Vtable::as_raw(self), bitmap.into().abi(), ::core::mem::transmute(bitmapbrushproperties.unwrap_or(::std::ptr::null())), ::core::mem::transmute(brushproperties.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateCommandList(&self) -> ::windows::core::Result<ID2D1CommandList> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateCommandList)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn IsDxgiFormatSupported(&self, format: super::Dxgi::Common::DXGI_FORMAT) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.IsDxgiFormatSupported)(::windows::core::Vtable::as_raw(self), format)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsBufferPrecisionSupported(&self, bufferprecision: D2D1_BUFFER_PRECISION) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.IsBufferPrecisionSupported)(::windows::core::Vtable::as_raw(self), bufferprecision)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetImageLocalBounds<P0>(&self, image: P0) -> ::windows::core::Result<Common::D2D_RECT_F>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Image>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetImageLocalBounds)(::windows::core::Vtable::as_raw(self), image.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetImageWorldBounds<P0>(&self, image: P0) -> ::windows::core::Result<Common::D2D_RECT_F>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Image>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetImageWorldBounds)(::windows::core::Vtable::as_raw(self), image.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn GetGlyphRunWorldBounds(&self, baselineorigin: Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE) -> ::windows::core::Result<Common::D2D_RECT_F> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetGlyphRunWorldBounds)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(baselineorigin), glyphrun, measuringmode, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<ID2D1Device> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1Device as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn SetTarget<P0>(&self, image: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Image>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetTarget)(::windows::core::Vtable::as_raw(self), image.into().abi())
    }
    pub unsafe fn GetTarget(&self) -> ::windows::core::Result<ID2D1Image> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetTarget)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1Image as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetRenderingControls(&self, renderingcontrols: *const D2D1_RENDERING_CONTROLS) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetRenderingControls)(::windows::core::Vtable::as_raw(self), renderingcontrols)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetRenderingControls(&self) -> D2D1_RENDERING_CONTROLS {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetRenderingControls)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        result__.assume_init()
    }
    pub unsafe fn SetPrimitiveBlend(&self, primitiveblend: D2D1_PRIMITIVE_BLEND) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetPrimitiveBlend)(::windows::core::Vtable::as_raw(self), primitiveblend)
    }
    pub unsafe fn GetPrimitiveBlend(&self) -> D2D1_PRIMITIVE_BLEND {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetPrimitiveBlend)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn SetUnitMode(&self, unitmode: D2D1_UNIT_MODE) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetUnitMode)(::windows::core::Vtable::as_raw(self), unitmode)
    }
    pub unsafe fn GetUnitMode(&self) -> D2D1_UNIT_MODE {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetUnitMode)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn DrawGlyphRun2<P0>(&self, baselineorigin: Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, glyphrundescription: ::core::option::Option<*const super::DirectWrite::DWRITE_GLYPH_RUN_DESCRIPTION>, foregroundbrush: P0, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.DrawGlyphRun2)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(baselineorigin), glyphrun, ::core::mem::transmute(glyphrundescription.unwrap_or(::std::ptr::null())), foregroundbrush.into().abi(), measuringmode)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawImage<P0>(&self, image: P0, targetoffset: ::core::option::Option<*const Common::D2D_POINT_2F>, imagerectangle: ::core::option::Option<*const Common::D2D_RECT_F>, interpolationmode: D2D1_INTERPOLATION_MODE, compositemode: Common::D2D1_COMPOSITE_MODE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Image>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.DrawImage)(::windows::core::Vtable::as_raw(self), image.into().abi(), ::core::mem::transmute(targetoffset.unwrap_or(::std::ptr::null())), ::core::mem::transmute(imagerectangle.unwrap_or(::std::ptr::null())), interpolationmode, compositemode)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawGdiMetafile<P0>(&self, gdimetafile: P0, targetoffset: ::core::option::Option<*const Common::D2D_POINT_2F>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1GdiMetafile>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.DrawGdiMetafile)(::windows::core::Vtable::as_raw(self), gdimetafile.into().abi(), ::core::mem::transmute(targetoffset.unwrap_or(::std::ptr::null())))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawBitmap2<P0>(&self, bitmap: P0, destinationrectangle: ::core::option::Option<*const Common::D2D_RECT_F>, opacity: f32, interpolationmode: D2D1_INTERPOLATION_MODE, sourcerectangle: ::core::option::Option<*const Common::D2D_RECT_F>, perspectivetransform: ::core::option::Option<*const Common::D2D_MATRIX_4X4_F>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Bitmap>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.DrawBitmap2)(::windows::core::Vtable::as_raw(self), bitmap.into().abi(), ::core::mem::transmute(destinationrectangle.unwrap_or(::std::ptr::null())), opacity, interpolationmode, ::core::mem::transmute(sourcerectangle.unwrap_or(::std::ptr::null())), ::core::mem::transmute(perspectivetransform.unwrap_or(::std::ptr::null())))
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn PushLayer2<P0>(&self, layerparameters: *const D2D1_LAYER_PARAMETERS1, layer: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Layer>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.PushLayer2)(::windows::core::Vtable::as_raw(self), layerparameters, layer.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn InvalidateEffectInputRectangle<P0>(&self, effect: P0, input: u32, inputrectangle: *const Common::D2D_RECT_F) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Effect>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.InvalidateEffectInputRectangle)(::windows::core::Vtable::as_raw(self), effect.into().abi(), input, inputrectangle).ok()
    }
    pub unsafe fn GetEffectInvalidRectangleCount<P0>(&self, effect: P0) -> ::windows::core::Result<u32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Effect>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetEffectInvalidRectangleCount)(::windows::core::Vtable::as_raw(self), effect.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetEffectInvalidRectangles<P0>(&self, effect: P0, rectangles: &mut [Common::D2D_RECT_F]) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Effect>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetEffectInvalidRectangles)(::windows::core::Vtable::as_raw(self), effect.into().abi(), ::core::mem::transmute(rectangles.as_ptr()), rectangles.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetEffectRequiredInputRectangles<P0>(&self, rendereffect: P0, renderimagerectangle: ::core::option::Option<*const Common::D2D_RECT_F>, inputdescriptions: *const D2D1_EFFECT_INPUT_DESCRIPTION, requiredinputrects: *mut Common::D2D_RECT_F, inputcount: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Effect>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetEffectRequiredInputRectangles)(::windows::core::Vtable::as_raw(self), rendereffect.into().abi(), ::core::mem::transmute(renderimagerectangle.unwrap_or(::std::ptr::null())), inputdescriptions, requiredinputrects, inputcount).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn FillOpacityMask2<P0, P1>(&self, opacitymask: P0, brush: P1, destinationrectangle: ::core::option::Option<*const Common::D2D_RECT_F>, sourcerectangle: ::core::option::Option<*const Common::D2D_RECT_F>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Bitmap>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.FillOpacityMask2)(::windows::core::Vtable::as_raw(self), opacitymask.into().abi(), brush.into().abi(), ::core::mem::transmute(destinationrectangle.unwrap_or(::std::ptr::null())), ::core::mem::transmute(sourcerectangle.unwrap_or(::std::ptr::null())))
    }
    pub unsafe fn CreateFilledGeometryRealization<P0>(&self, geometry: P0, flatteningtolerance: f32) -> ::windows::core::Result<ID2D1GeometryRealization>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Geometry>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateFilledGeometryRealization)(::windows::core::Vtable::as_raw(self), geometry.into().abi(), flatteningtolerance, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateStrokedGeometryRealization<P0, P1>(&self, geometry: P0, flatteningtolerance: f32, strokewidth: f32, strokestyle: P1) -> ::windows::core::Result<ID2D1GeometryRealization>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Geometry>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateStrokedGeometryRealization)(::windows::core::Vtable::as_raw(self), geometry.into().abi(), flatteningtolerance, strokewidth, strokestyle.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn DrawGeometryRealization<P0, P1>(&self, geometryrealization: P0, brush: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1GeometryRealization>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.DrawGeometryRealization)(::windows::core::Vtable::as_raw(self), geometryrealization.into().abi(), brush.into().abi())
    }
    pub unsafe fn CreateInk(&self, startpoint: *const D2D1_INK_POINT) -> ::windows::core::Result<ID2D1Ink> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateInk)(::windows::core::Vtable::as_raw(self), startpoint, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn CreateInkStyle(&self, inkstyleproperties: ::core::option::Option<*const D2D1_INK_STYLE_PROPERTIES>) -> ::windows::core::Result<ID2D1InkStyle> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateInkStyle)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(inkstyleproperties.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CreateGradientMesh(&self, patches: &[D2D1_GRADIENT_MESH_PATCH]) -> ::windows::core::Result<ID2D1GradientMesh> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateGradientMesh)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(patches.as_ptr()), patches.len() as _, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Imaging\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Imaging"))]
    pub unsafe fn CreateImageSourceFromWic<P0>(&self, wicbitmapsource: P0, loadingoptions: D2D1_IMAGE_SOURCE_LOADING_OPTIONS, alphamode: Common::D2D1_ALPHA_MODE) -> ::windows::core::Result<ID2D1ImageSourceFromWic>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Imaging::IWICBitmapSource>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateImageSourceFromWic)(::windows::core::Vtable::as_raw(self), wicbitmapsource.into().abi(), loadingoptions, alphamode, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateLookupTable3D(&self, precision: D2D1_BUFFER_PRECISION, extents: &[u32; 3], data: &[u8], strides: &[u32; 2]) -> ::windows::core::Result<ID2D1LookupTable3D> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateLookupTable3D)(::windows::core::Vtable::as_raw(self), precision, ::core::mem::transmute(extents.as_ptr()), ::core::mem::transmute(data.as_ptr()), data.len() as _, ::core::mem::transmute(strides.as_ptr()), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateImageSourceFromDxgi(&self, surfaces: &[super::Dxgi::IDXGISurface], colorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE, options: D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS) -> ::windows::core::Result<ID2D1ImageSource> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateImageSourceFromDxgi)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(surfaces.as_ptr()), surfaces.len() as _, colorspace, options, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetGradientMeshWorldBounds<P0>(&self, gradientmesh: P0) -> ::windows::core::Result<Common::D2D_RECT_F>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1GradientMesh>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetGradientMeshWorldBounds)(::windows::core::Vtable::as_raw(self), gradientmesh.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn DrawInk<P0, P1, P2>(&self, ink: P0, brush: P1, inkstyle: P2)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Ink>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P2: ::std::convert::Into<::windows::core::InParam<ID2D1InkStyle>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.DrawInk)(::windows::core::Vtable::as_raw(self), ink.into().abi(), brush.into().abi(), inkstyle.into().abi())
    }
    pub unsafe fn DrawGradientMesh<P0>(&self, gradientmesh: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1GradientMesh>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.DrawGradientMesh)(::windows::core::Vtable::as_raw(self), gradientmesh.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawGdiMetafile2<P0>(&self, gdimetafile: P0, destinationrectangle: ::core::option::Option<*const Common::D2D_RECT_F>, sourcerectangle: ::core::option::Option<*const Common::D2D_RECT_F>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1GdiMetafile>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.DrawGdiMetafile2)(::windows::core::Vtable::as_raw(self), gdimetafile.into().abi(), ::core::mem::transmute(destinationrectangle.unwrap_or(::std::ptr::null())), ::core::mem::transmute(sourcerectangle.unwrap_or(::std::ptr::null())))
    }
    pub unsafe fn CreateTransformedImageSource<P0>(&self, imagesource: P0, properties: *const D2D1_TRANSFORMED_IMAGE_SOURCE_PROPERTIES) -> ::windows::core::Result<ID2D1TransformedImageSource>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1ImageSource>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateTransformedImageSource)(::windows::core::Vtable::as_raw(self), imagesource.into().abi(), properties, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateSpriteBatch(&self) -> ::windows::core::Result<ID2D1SpriteBatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateSpriteBatch)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn DrawSpriteBatch<P0, P1>(&self, spritebatch: P0, startindex: u32, spritecount: u32, bitmap: P1, interpolationmode: D2D1_BITMAP_INTERPOLATION_MODE, spriteoptions: D2D1_SPRITE_OPTIONS)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1SpriteBatch>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Bitmap>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.DrawSpriteBatch)(::windows::core::Vtable::as_raw(self), spritebatch.into().abi(), startindex, spritecount, bitmap.into().abi(), interpolationmode, spriteoptions)
    }
    pub unsafe fn CreateSvgGlyphStyle(&self) -> ::windows::core::Result<ID2D1SvgGlyphStyle> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateSvgGlyphStyle)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn DrawText2<P0, P1, P2>(&self, string: &[u16], textformat: P0, layoutrect: *const Common::D2D_RECT_F, defaultfillbrush: P1, svgglyphstyle: P2, colorpaletteindex: u32, options: D2D1_DRAW_TEXT_OPTIONS, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::DirectWrite::IDWriteTextFormat>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P2: ::std::convert::Into<::windows::core::InParam<ID2D1SvgGlyphStyle>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DrawText2)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(string.as_ptr()), string.len() as _, textformat.into().abi(), layoutrect, defaultfillbrush.into().abi(), svgglyphstyle.into().abi(), colorpaletteindex, options, measuringmode)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn DrawTextLayout2<P0, P1, P2>(&self, origin: Common::D2D_POINT_2F, textlayout: P0, defaultfillbrush: P1, svgglyphstyle: P2, colorpaletteindex: u32, options: D2D1_DRAW_TEXT_OPTIONS)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::DirectWrite::IDWriteTextLayout>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P2: ::std::convert::Into<::windows::core::InParam<ID2D1SvgGlyphStyle>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DrawTextLayout2)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(origin), textlayout.into().abi(), defaultfillbrush.into().abi(), svgglyphstyle.into().abi(), colorpaletteindex, options)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn DrawColorBitmapGlyphRun(&self, glyphimageformat: super::DirectWrite::DWRITE_GLYPH_IMAGE_FORMATS, baselineorigin: Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE, bitmapsnapoption: D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION) {
        (::windows::core::Vtable::vtable(self).base__.DrawColorBitmapGlyphRun)(::windows::core::Vtable::as_raw(self), glyphimageformat, ::core::mem::transmute(baselineorigin), glyphrun, measuringmode, bitmapsnapoption)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn DrawSvgGlyphRun<P0, P1>(&self, baselineorigin: Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, defaultfillbrush: P0, svgglyphstyle: P1, colorpaletteindex: u32, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1SvgGlyphStyle>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DrawSvgGlyphRun)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(baselineorigin), glyphrun, defaultfillbrush.into().abi(), svgglyphstyle.into().abi(), colorpaletteindex, measuringmode)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn GetColorBitmapGlyphImage<P0, P1>(&self, glyphimageformat: super::DirectWrite::DWRITE_GLYPH_IMAGE_FORMATS, glyphorigin: Common::D2D_POINT_2F, fontface: P0, fontemsize: f32, glyphindex: u16, issideways: P1, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, dpix: f32, dpiy: f32, glyphtransform: *mut super::super::super::Foundation::Numerics::Matrix3x2, glyphimage: *mut ::core::option::Option<ID2D1Image>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::DirectWrite::IDWriteFontFace>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetColorBitmapGlyphImage)(::windows::core::Vtable::as_raw(self), glyphimageformat, ::core::mem::transmute(glyphorigin), fontface.into().abi(), fontemsize, glyphindex, issideways.into(), ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), dpix, dpiy, glyphtransform, ::core::mem::transmute(glyphimage)).ok()
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn GetSvgGlyphImage<P0, P1, P2, P3>(&self, glyphorigin: Common::D2D_POINT_2F, fontface: P0, fontemsize: f32, glyphindex: u16, issideways: P1, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, defaultfillbrush: P2, svgglyphstyle: P3, colorpaletteindex: u32, glyphtransform: *mut super::super::super::Foundation::Numerics::Matrix3x2, glyphimage: *mut ::core::option::Option<ID2D1CommandList>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::DirectWrite::IDWriteFontFace>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
        P2: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P3: ::std::convert::Into<::windows::core::InParam<ID2D1SvgGlyphStyle>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetSvgGlyphImage)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(glyphorigin), fontface.into().abi(), fontemsize, glyphindex, issideways.into(), ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), defaultfillbrush.into().abi(), svgglyphstyle.into().abi(), colorpaletteindex, glyphtransform, ::core::mem::transmute(glyphimage)).ok()
    }
}
impl ::core::cmp::PartialEq for ID2D1DeviceContext6 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1DeviceContext6 {}
impl ::core::fmt::Debug for ID2D1DeviceContext6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1DeviceContext6").field(&self.0).finish()
    }
}
impl ID2D1DeviceContext6 {
    pub unsafe fn GetFactory(&self) -> ::windows::core::Result<ID2D1Factory> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.GetFactory)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1Factory as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateBitmap(&self, size: Common::D2D_SIZE_U, srcdata: ::core::option::Option<*const ::core::ffi::c_void>, pitch: u32, bitmapproperties: *const D2D1_BITMAP_PROPERTIES) -> ::windows::core::Result<ID2D1Bitmap> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CreateBitmap)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(size), ::core::mem::transmute(srcdata.unwrap_or(::std::ptr::null())), pitch, bitmapproperties, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Imaging\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
    pub unsafe fn CreateBitmapFromWicBitmap<P0>(&self, wicbitmapsource: P0, bitmapproperties: ::core::option::Option<*const D2D1_BITMAP_PROPERTIES>) -> ::windows::core::Result<ID2D1Bitmap>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Imaging::IWICBitmapSource>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CreateBitmapFromWicBitmap)(::windows::core::Vtable::as_raw(self), wicbitmapsource.into().abi(), ::core::mem::transmute(bitmapproperties.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSharedBitmap(&self, riid: *const ::windows::core::GUID, data: *mut ::core::ffi::c_void, bitmapproperties: ::core::option::Option<*const D2D1_BITMAP_PROPERTIES>, bitmap: *mut ::core::option::Option<ID2D1Bitmap>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CreateSharedBitmap)(::windows::core::Vtable::as_raw(self), riid, data, ::core::mem::transmute(bitmapproperties.unwrap_or(::std::ptr::null())), ::core::mem::transmute(bitmap)).ok()
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn CreateBitmapBrush<P0>(&self, bitmap: P0, bitmapbrushproperties: ::core::option::Option<*const D2D1_BITMAP_BRUSH_PROPERTIES>, brushproperties: ::core::option::Option<*const D2D1_BRUSH_PROPERTIES>) -> ::windows::core::Result<ID2D1BitmapBrush>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Bitmap>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CreateBitmapBrush)(::windows::core::Vtable::as_raw(self), bitmap.into().abi(), ::core::mem::transmute(bitmapbrushproperties.unwrap_or(::std::ptr::null())), ::core::mem::transmute(brushproperties.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn CreateSolidColorBrush(&self, color: *const Common::D2D1_COLOR_F, brushproperties: ::core::option::Option<*const D2D1_BRUSH_PROPERTIES>) -> ::windows::core::Result<ID2D1SolidColorBrush> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CreateSolidColorBrush)(::windows::core::Vtable::as_raw(self), color, ::core::mem::transmute(brushproperties.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CreateGradientStopCollection(&self, gradientstops: &[D2D1_GRADIENT_STOP], colorinterpolationgamma: D2D1_GAMMA, extendmode: D2D1_EXTEND_MODE) -> ::windows::core::Result<ID2D1GradientStopCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CreateGradientStopCollection)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(gradientstops.as_ptr()), gradientstops.len() as _, colorinterpolationgamma, extendmode, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn CreateLinearGradientBrush<P0>(&self, lineargradientbrushproperties: *const D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES, brushproperties: ::core::option::Option<*const D2D1_BRUSH_PROPERTIES>, gradientstopcollection: P0) -> ::windows::core::Result<ID2D1LinearGradientBrush>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1GradientStopCollection>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CreateLinearGradientBrush)(::windows::core::Vtable::as_raw(self), lineargradientbrushproperties, ::core::mem::transmute(brushproperties.unwrap_or(::std::ptr::null())), gradientstopcollection.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn CreateRadialGradientBrush<P0>(&self, radialgradientbrushproperties: *const D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES, brushproperties: ::core::option::Option<*const D2D1_BRUSH_PROPERTIES>, gradientstopcollection: P0) -> ::windows::core::Result<ID2D1RadialGradientBrush>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1GradientStopCollection>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CreateRadialGradientBrush)(::windows::core::Vtable::as_raw(self), radialgradientbrushproperties, ::core::mem::transmute(brushproperties.unwrap_or(::std::ptr::null())), gradientstopcollection.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateCompatibleRenderTarget(&self, desiredsize: ::core::option::Option<*const Common::D2D_SIZE_F>, desiredpixelsize: ::core::option::Option<*const Common::D2D_SIZE_U>, desiredformat: ::core::option::Option<*const Common::D2D1_PIXEL_FORMAT>, options: D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS) -> ::windows::core::Result<ID2D1BitmapRenderTarget> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CreateCompatibleRenderTarget)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(desiredsize.unwrap_or(::std::ptr::null())), ::core::mem::transmute(desiredpixelsize.unwrap_or(::std::ptr::null())), ::core::mem::transmute(desiredformat.unwrap_or(::std::ptr::null())), options, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CreateLayer(&self, size: ::core::option::Option<*const Common::D2D_SIZE_F>) -> ::windows::core::Result<ID2D1Layer> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CreateLayer)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(size.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateMesh(&self) -> ::windows::core::Result<ID2D1Mesh> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CreateMesh)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawLine<P0, P1>(&self, point0: Common::D2D_POINT_2F, point1: Common::D2D_POINT_2F, brush: P0, strokewidth: f32, strokestyle: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.DrawLine)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(point0), ::core::mem::transmute(point1), brush.into().abi(), strokewidth, strokestyle.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawRectangle<P0, P1>(&self, rect: *const Common::D2D_RECT_F, brush: P0, strokewidth: f32, strokestyle: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.DrawRectangle)(::windows::core::Vtable::as_raw(self), rect, brush.into().abi(), strokewidth, strokestyle.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn FillRectangle<P0>(&self, rect: *const Common::D2D_RECT_F, brush: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.FillRectangle)(::windows::core::Vtable::as_raw(self), rect, brush.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawRoundedRectangle<P0, P1>(&self, roundedrect: *const D2D1_ROUNDED_RECT, brush: P0, strokewidth: f32, strokestyle: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.DrawRoundedRectangle)(::windows::core::Vtable::as_raw(self), roundedrect, brush.into().abi(), strokewidth, strokestyle.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn FillRoundedRectangle<P0>(&self, roundedrect: *const D2D1_ROUNDED_RECT, brush: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.FillRoundedRectangle)(::windows::core::Vtable::as_raw(self), roundedrect, brush.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawEllipse<P0, P1>(&self, ellipse: *const D2D1_ELLIPSE, brush: P0, strokewidth: f32, strokestyle: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.DrawEllipse)(::windows::core::Vtable::as_raw(self), ellipse, brush.into().abi(), strokewidth, strokestyle.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn FillEllipse<P0>(&self, ellipse: *const D2D1_ELLIPSE, brush: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.FillEllipse)(::windows::core::Vtable::as_raw(self), ellipse, brush.into().abi())
    }
    pub unsafe fn DrawGeometry<P0, P1, P2>(&self, geometry: P0, brush: P1, strokewidth: f32, strokestyle: P2)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Geometry>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P2: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.DrawGeometry)(::windows::core::Vtable::as_raw(self), geometry.into().abi(), brush.into().abi(), strokewidth, strokestyle.into().abi())
    }
    pub unsafe fn FillGeometry<P0, P1, P2>(&self, geometry: P0, brush: P1, opacitybrush: P2)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Geometry>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P2: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.FillGeometry)(::windows::core::Vtable::as_raw(self), geometry.into().abi(), brush.into().abi(), opacitybrush.into().abi())
    }
    pub unsafe fn FillMesh<P0, P1>(&self, mesh: P0, brush: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Mesh>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.FillMesh)(::windows::core::Vtable::as_raw(self), mesh.into().abi(), brush.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn FillOpacityMask<P0, P1>(&self, opacitymask: P0, brush: P1, content: D2D1_OPACITY_MASK_CONTENT, destinationrectangle: ::core::option::Option<*const Common::D2D_RECT_F>, sourcerectangle: ::core::option::Option<*const Common::D2D_RECT_F>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Bitmap>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.FillOpacityMask)(::windows::core::Vtable::as_raw(self), opacitymask.into().abi(), brush.into().abi(), content, ::core::mem::transmute(destinationrectangle.unwrap_or(::std::ptr::null())), ::core::mem::transmute(sourcerectangle.unwrap_or(::std::ptr::null())))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawBitmap<P0>(&self, bitmap: P0, destinationrectangle: ::core::option::Option<*const Common::D2D_RECT_F>, opacity: f32, interpolationmode: D2D1_BITMAP_INTERPOLATION_MODE, sourcerectangle: ::core::option::Option<*const Common::D2D_RECT_F>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Bitmap>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.DrawBitmap)(::windows::core::Vtable::as_raw(self), bitmap.into().abi(), ::core::mem::transmute(destinationrectangle.unwrap_or(::std::ptr::null())), opacity, interpolationmode, ::core::mem::transmute(sourcerectangle.unwrap_or(::std::ptr::null())))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn DrawText<P0, P1>(&self, string: &[u16], textformat: P0, layoutrect: *const Common::D2D_RECT_F, defaultfillbrush: P1, options: D2D1_DRAW_TEXT_OPTIONS, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::DirectWrite::IDWriteTextFormat>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.DrawText)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(string.as_ptr()), string.len() as _, textformat.into().abi(), layoutrect, defaultfillbrush.into().abi(), options, measuringmode)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn DrawTextLayout<P0, P1>(&self, origin: Common::D2D_POINT_2F, textlayout: P0, defaultfillbrush: P1, options: D2D1_DRAW_TEXT_OPTIONS)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::DirectWrite::IDWriteTextLayout>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.DrawTextLayout)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(origin), textlayout.into().abi(), defaultfillbrush.into().abi(), options)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn DrawGlyphRun<P0>(&self, baselineorigin: Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, foregroundbrush: P0, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.DrawGlyphRun)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(baselineorigin), glyphrun, foregroundbrush.into().abi(), measuringmode)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn SetTransform(&self, transform: *const super::super::super::Foundation::Numerics::Matrix3x2) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.SetTransform)(::windows::core::Vtable::as_raw(self), transform)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn GetTransform(&self, transform: *mut super::super::super::Foundation::Numerics::Matrix3x2) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.GetTransform)(::windows::core::Vtable::as_raw(self), transform)
    }
    pub unsafe fn SetAntialiasMode(&self, antialiasmode: D2D1_ANTIALIAS_MODE) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.SetAntialiasMode)(::windows::core::Vtable::as_raw(self), antialiasmode)
    }
    pub unsafe fn GetAntialiasMode(&self) -> D2D1_ANTIALIAS_MODE {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.GetAntialiasMode)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn SetTextAntialiasMode(&self, textantialiasmode: D2D1_TEXT_ANTIALIAS_MODE) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.SetTextAntialiasMode)(::windows::core::Vtable::as_raw(self), textantialiasmode)
    }
    pub unsafe fn GetTextAntialiasMode(&self) -> D2D1_TEXT_ANTIALIAS_MODE {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.GetTextAntialiasMode)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(feature = "Win32_Graphics_DirectWrite")]
    pub unsafe fn SetTextRenderingParams<P0>(&self, textrenderingparams: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::DirectWrite::IDWriteRenderingParams>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.SetTextRenderingParams)(::windows::core::Vtable::as_raw(self), textrenderingparams.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(feature = "Win32_Graphics_DirectWrite")]
    pub unsafe fn GetTextRenderingParams(&self) -> ::windows::core::Result<super::DirectWrite::IDWriteRenderingParams> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.GetTextRenderingParams)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <super::DirectWrite::IDWriteRenderingParams as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn SetTags(&self, tag1: u64, tag2: u64) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.SetTags)(::windows::core::Vtable::as_raw(self), tag1, tag2)
    }
    pub unsafe fn GetTags(&self, tag1: ::core::option::Option<*mut u64>, tag2: ::core::option::Option<*mut u64>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.GetTags)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(tag1.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(tag2.unwrap_or(::std::ptr::null_mut())))
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn PushLayer<P0>(&self, layerparameters: *const D2D1_LAYER_PARAMETERS, layer: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Layer>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.PushLayer)(::windows::core::Vtable::as_raw(self), layerparameters, layer.into().abi())
    }
    pub unsafe fn PopLayer(&self) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.PopLayer)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn Flush(&self, tag1: ::core::option::Option<*mut u64>, tag2: ::core::option::Option<*mut u64>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.Flush)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(tag1.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(tag2.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SaveDrawingState<P0>(&self, drawingstateblock: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1DrawingStateBlock>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.SaveDrawingState)(::windows::core::Vtable::as_raw(self), drawingstateblock.into().abi())
    }
    pub unsafe fn RestoreDrawingState<P0>(&self, drawingstateblock: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1DrawingStateBlock>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.RestoreDrawingState)(::windows::core::Vtable::as_raw(self), drawingstateblock.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn PushAxisAlignedClip(&self, cliprect: *const Common::D2D_RECT_F, antialiasmode: D2D1_ANTIALIAS_MODE) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.PushAxisAlignedClip)(::windows::core::Vtable::as_raw(self), cliprect, antialiasmode)
    }
    pub unsafe fn PopAxisAlignedClip(&self) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.PopAxisAlignedClip)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn Clear(&self, clearcolor: ::core::option::Option<*const Common::D2D1_COLOR_F>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.Clear)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(clearcolor.unwrap_or(::std::ptr::null())))
    }
    pub unsafe fn BeginDraw(&self) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.BeginDraw)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn EndDraw(&self, tag1: ::core::option::Option<*mut u64>, tag2: ::core::option::Option<*mut u64>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.EndDraw)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(tag1.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(tag2.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetPixelFormat(&self) -> Common::D2D1_PIXEL_FORMAT {
        let mut result__: Common::D2D1_PIXEL_FORMAT = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.GetPixelFormat)(::windows::core::Vtable::as_raw(self), &mut result__);
        result__
    }
    pub unsafe fn SetDpi(&self, dpix: f32, dpiy: f32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.SetDpi)(::windows::core::Vtable::as_raw(self), dpix, dpiy)
    }
    pub unsafe fn GetDpi(&self, dpix: *mut f32, dpiy: *mut f32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.GetDpi)(::windows::core::Vtable::as_raw(self), dpix, dpiy)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetSize(&self) -> Common::D2D_SIZE_F {
        let mut result__: Common::D2D_SIZE_F = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.GetSize)(::windows::core::Vtable::as_raw(self), &mut result__);
        result__
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetPixelSize(&self) -> Common::D2D_SIZE_U {
        let mut result__: Common::D2D_SIZE_U = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.GetPixelSize)(::windows::core::Vtable::as_raw(self), &mut result__);
        result__
    }
    pub unsafe fn GetMaximumBitmapSize(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.GetMaximumBitmapSize)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn IsSupported(&self, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.IsSupported)(::windows::core::Vtable::as_raw(self), rendertargetproperties)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateBitmap2(&self, size: Common::D2D_SIZE_U, sourcedata: ::core::option::Option<*const ::core::ffi::c_void>, pitch: u32, bitmapproperties: *const D2D1_BITMAP_PROPERTIES1) -> ::windows::core::Result<ID2D1Bitmap1> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CreateBitmap2)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(size), ::core::mem::transmute(sourcedata.unwrap_or(::std::ptr::null())), pitch, bitmapproperties, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Imaging\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
    pub unsafe fn CreateBitmapFromWicBitmap2<P0>(&self, wicbitmapsource: P0, bitmapproperties: ::core::option::Option<*const D2D1_BITMAP_PROPERTIES1>) -> ::windows::core::Result<ID2D1Bitmap1>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Imaging::IWICBitmapSource>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CreateBitmapFromWicBitmap2)(::windows::core::Vtable::as_raw(self), wicbitmapsource.into().abi(), ::core::mem::transmute(bitmapproperties.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateColorContext(&self, space: D2D1_COLOR_SPACE, profile: ::core::option::Option<&[u8]>) -> ::windows::core::Result<ID2D1ColorContext> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CreateColorContext)(::windows::core::Vtable::as_raw(self), space, ::core::mem::transmute(profile.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), profile.as_deref().map_or(0, |slice| slice.len() as _), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateColorContextFromFilename<P0>(&self, filename: P0) -> ::windows::core::Result<ID2D1ColorContext>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CreateColorContextFromFilename)(::windows::core::Vtable::as_raw(self), filename.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub unsafe fn CreateColorContextFromWicColorContext<P0>(&self, wiccolorcontext: P0) -> ::windows::core::Result<ID2D1ColorContext>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Imaging::IWICColorContext>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CreateColorContextFromWicColorContext)(::windows::core::Vtable::as_raw(self), wiccolorcontext.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateBitmapFromDxgiSurface<P0>(&self, surface: P0, bitmapproperties: ::core::option::Option<*const D2D1_BITMAP_PROPERTIES1>) -> ::windows::core::Result<ID2D1Bitmap1>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Dxgi::IDXGISurface>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CreateBitmapFromDxgiSurface)(::windows::core::Vtable::as_raw(self), surface.into().abi(), ::core::mem::transmute(bitmapproperties.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateEffect(&self, effectid: *const ::windows::core::GUID) -> ::windows::core::Result<ID2D1Effect> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CreateEffect)(::windows::core::Vtable::as_raw(self), effectid, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CreateGradientStopCollection2(&self, straightalphagradientstops: &[D2D1_GRADIENT_STOP], preinterpolationspace: D2D1_COLOR_SPACE, postinterpolationspace: D2D1_COLOR_SPACE, bufferprecision: D2D1_BUFFER_PRECISION, extendmode: D2D1_EXTEND_MODE, colorinterpolationmode: D2D1_COLOR_INTERPOLATION_MODE) -> ::windows::core::Result<ID2D1GradientStopCollection1> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CreateGradientStopCollection2)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(straightalphagradientstops.as_ptr()), straightalphagradientstops.len() as _, preinterpolationspace, postinterpolationspace, bufferprecision, extendmode, colorinterpolationmode, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn CreateImageBrush<P0>(&self, image: P0, imagebrushproperties: *const D2D1_IMAGE_BRUSH_PROPERTIES, brushproperties: ::core::option::Option<*const D2D1_BRUSH_PROPERTIES>) -> ::windows::core::Result<ID2D1ImageBrush>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Image>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CreateImageBrush)(::windows::core::Vtable::as_raw(self), image.into().abi(), imagebrushproperties, ::core::mem::transmute(brushproperties.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn CreateBitmapBrush2<P0>(&self, bitmap: P0, bitmapbrushproperties: ::core::option::Option<*const D2D1_BITMAP_BRUSH_PROPERTIES1>, brushproperties: ::core::option::Option<*const D2D1_BRUSH_PROPERTIES>) -> ::windows::core::Result<ID2D1BitmapBrush1>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Bitmap>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CreateBitmapBrush2)(::windows::core::Vtable::as_raw(self), bitmap.into().abi(), ::core::mem::transmute(bitmapbrushproperties.unwrap_or(::std::ptr::null())), ::core::mem::transmute(brushproperties.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateCommandList(&self) -> ::windows::core::Result<ID2D1CommandList> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CreateCommandList)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn IsDxgiFormatSupported(&self, format: super::Dxgi::Common::DXGI_FORMAT) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.IsDxgiFormatSupported)(::windows::core::Vtable::as_raw(self), format)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsBufferPrecisionSupported(&self, bufferprecision: D2D1_BUFFER_PRECISION) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.IsBufferPrecisionSupported)(::windows::core::Vtable::as_raw(self), bufferprecision)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetImageLocalBounds<P0>(&self, image: P0) -> ::windows::core::Result<Common::D2D_RECT_F>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Image>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.GetImageLocalBounds)(::windows::core::Vtable::as_raw(self), image.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetImageWorldBounds<P0>(&self, image: P0) -> ::windows::core::Result<Common::D2D_RECT_F>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Image>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.GetImageWorldBounds)(::windows::core::Vtable::as_raw(self), image.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn GetGlyphRunWorldBounds(&self, baselineorigin: Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE) -> ::windows::core::Result<Common::D2D_RECT_F> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.GetGlyphRunWorldBounds)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(baselineorigin), glyphrun, measuringmode, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<ID2D1Device> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1Device as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn SetTarget<P0>(&self, image: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Image>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.SetTarget)(::windows::core::Vtable::as_raw(self), image.into().abi())
    }
    pub unsafe fn GetTarget(&self) -> ::windows::core::Result<ID2D1Image> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.GetTarget)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1Image as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetRenderingControls(&self, renderingcontrols: *const D2D1_RENDERING_CONTROLS) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.SetRenderingControls)(::windows::core::Vtable::as_raw(self), renderingcontrols)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetRenderingControls(&self) -> D2D1_RENDERING_CONTROLS {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.GetRenderingControls)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        result__.assume_init()
    }
    pub unsafe fn SetPrimitiveBlend(&self, primitiveblend: D2D1_PRIMITIVE_BLEND) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.SetPrimitiveBlend)(::windows::core::Vtable::as_raw(self), primitiveblend)
    }
    pub unsafe fn GetPrimitiveBlend(&self) -> D2D1_PRIMITIVE_BLEND {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.GetPrimitiveBlend)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn SetUnitMode(&self, unitmode: D2D1_UNIT_MODE) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.SetUnitMode)(::windows::core::Vtable::as_raw(self), unitmode)
    }
    pub unsafe fn GetUnitMode(&self) -> D2D1_UNIT_MODE {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.GetUnitMode)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn DrawGlyphRun2<P0>(&self, baselineorigin: Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, glyphrundescription: ::core::option::Option<*const super::DirectWrite::DWRITE_GLYPH_RUN_DESCRIPTION>, foregroundbrush: P0, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.DrawGlyphRun2)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(baselineorigin), glyphrun, ::core::mem::transmute(glyphrundescription.unwrap_or(::std::ptr::null())), foregroundbrush.into().abi(), measuringmode)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawImage<P0>(&self, image: P0, targetoffset: ::core::option::Option<*const Common::D2D_POINT_2F>, imagerectangle: ::core::option::Option<*const Common::D2D_RECT_F>, interpolationmode: D2D1_INTERPOLATION_MODE, compositemode: Common::D2D1_COMPOSITE_MODE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Image>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.DrawImage)(::windows::core::Vtable::as_raw(self), image.into().abi(), ::core::mem::transmute(targetoffset.unwrap_or(::std::ptr::null())), ::core::mem::transmute(imagerectangle.unwrap_or(::std::ptr::null())), interpolationmode, compositemode)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawGdiMetafile<P0>(&self, gdimetafile: P0, targetoffset: ::core::option::Option<*const Common::D2D_POINT_2F>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1GdiMetafile>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.DrawGdiMetafile)(::windows::core::Vtable::as_raw(self), gdimetafile.into().abi(), ::core::mem::transmute(targetoffset.unwrap_or(::std::ptr::null())))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawBitmap2<P0>(&self, bitmap: P0, destinationrectangle: ::core::option::Option<*const Common::D2D_RECT_F>, opacity: f32, interpolationmode: D2D1_INTERPOLATION_MODE, sourcerectangle: ::core::option::Option<*const Common::D2D_RECT_F>, perspectivetransform: ::core::option::Option<*const Common::D2D_MATRIX_4X4_F>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Bitmap>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.DrawBitmap2)(::windows::core::Vtable::as_raw(self), bitmap.into().abi(), ::core::mem::transmute(destinationrectangle.unwrap_or(::std::ptr::null())), opacity, interpolationmode, ::core::mem::transmute(sourcerectangle.unwrap_or(::std::ptr::null())), ::core::mem::transmute(perspectivetransform.unwrap_or(::std::ptr::null())))
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn PushLayer2<P0>(&self, layerparameters: *const D2D1_LAYER_PARAMETERS1, layer: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Layer>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.PushLayer2)(::windows::core::Vtable::as_raw(self), layerparameters, layer.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn InvalidateEffectInputRectangle<P0>(&self, effect: P0, input: u32, inputrectangle: *const Common::D2D_RECT_F) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Effect>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.InvalidateEffectInputRectangle)(::windows::core::Vtable::as_raw(self), effect.into().abi(), input, inputrectangle).ok()
    }
    pub unsafe fn GetEffectInvalidRectangleCount<P0>(&self, effect: P0) -> ::windows::core::Result<u32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Effect>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.GetEffectInvalidRectangleCount)(::windows::core::Vtable::as_raw(self), effect.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetEffectInvalidRectangles<P0>(&self, effect: P0, rectangles: &mut [Common::D2D_RECT_F]) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Effect>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.GetEffectInvalidRectangles)(::windows::core::Vtable::as_raw(self), effect.into().abi(), ::core::mem::transmute(rectangles.as_ptr()), rectangles.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetEffectRequiredInputRectangles<P0>(&self, rendereffect: P0, renderimagerectangle: ::core::option::Option<*const Common::D2D_RECT_F>, inputdescriptions: *const D2D1_EFFECT_INPUT_DESCRIPTION, requiredinputrects: *mut Common::D2D_RECT_F, inputcount: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Effect>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.GetEffectRequiredInputRectangles)(::windows::core::Vtable::as_raw(self), rendereffect.into().abi(), ::core::mem::transmute(renderimagerectangle.unwrap_or(::std::ptr::null())), inputdescriptions, requiredinputrects, inputcount).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn FillOpacityMask2<P0, P1>(&self, opacitymask: P0, brush: P1, destinationrectangle: ::core::option::Option<*const Common::D2D_RECT_F>, sourcerectangle: ::core::option::Option<*const Common::D2D_RECT_F>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Bitmap>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.FillOpacityMask2)(::windows::core::Vtable::as_raw(self), opacitymask.into().abi(), brush.into().abi(), ::core::mem::transmute(destinationrectangle.unwrap_or(::std::ptr::null())), ::core::mem::transmute(sourcerectangle.unwrap_or(::std::ptr::null())))
    }
    pub unsafe fn CreateFilledGeometryRealization<P0>(&self, geometry: P0, flatteningtolerance: f32) -> ::windows::core::Result<ID2D1GeometryRealization>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Geometry>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateFilledGeometryRealization)(::windows::core::Vtable::as_raw(self), geometry.into().abi(), flatteningtolerance, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateStrokedGeometryRealization<P0, P1>(&self, geometry: P0, flatteningtolerance: f32, strokewidth: f32, strokestyle: P1) -> ::windows::core::Result<ID2D1GeometryRealization>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Geometry>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateStrokedGeometryRealization)(::windows::core::Vtable::as_raw(self), geometry.into().abi(), flatteningtolerance, strokewidth, strokestyle.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn DrawGeometryRealization<P0, P1>(&self, geometryrealization: P0, brush: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1GeometryRealization>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.DrawGeometryRealization)(::windows::core::Vtable::as_raw(self), geometryrealization.into().abi(), brush.into().abi())
    }
    pub unsafe fn CreateInk(&self, startpoint: *const D2D1_INK_POINT) -> ::windows::core::Result<ID2D1Ink> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateInk)(::windows::core::Vtable::as_raw(self), startpoint, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn CreateInkStyle(&self, inkstyleproperties: ::core::option::Option<*const D2D1_INK_STYLE_PROPERTIES>) -> ::windows::core::Result<ID2D1InkStyle> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateInkStyle)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(inkstyleproperties.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CreateGradientMesh(&self, patches: &[D2D1_GRADIENT_MESH_PATCH]) -> ::windows::core::Result<ID2D1GradientMesh> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateGradientMesh)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(patches.as_ptr()), patches.len() as _, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Imaging\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Imaging"))]
    pub unsafe fn CreateImageSourceFromWic<P0>(&self, wicbitmapsource: P0, loadingoptions: D2D1_IMAGE_SOURCE_LOADING_OPTIONS, alphamode: Common::D2D1_ALPHA_MODE) -> ::windows::core::Result<ID2D1ImageSourceFromWic>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Imaging::IWICBitmapSource>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateImageSourceFromWic)(::windows::core::Vtable::as_raw(self), wicbitmapsource.into().abi(), loadingoptions, alphamode, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateLookupTable3D(&self, precision: D2D1_BUFFER_PRECISION, extents: &[u32; 3], data: &[u8], strides: &[u32; 2]) -> ::windows::core::Result<ID2D1LookupTable3D> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateLookupTable3D)(::windows::core::Vtable::as_raw(self), precision, ::core::mem::transmute(extents.as_ptr()), ::core::mem::transmute(data.as_ptr()), data.len() as _, ::core::mem::transmute(strides.as_ptr()), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateImageSourceFromDxgi(&self, surfaces: &[super::Dxgi::IDXGISurface], colorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE, options: D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS) -> ::windows::core::Result<ID2D1ImageSource> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateImageSourceFromDxgi)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(surfaces.as_ptr()), surfaces.len() as _, colorspace, options, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetGradientMeshWorldBounds<P0>(&self, gradientmesh: P0) -> ::windows::core::Result<Common::D2D_RECT_F>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1GradientMesh>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetGradientMeshWorldBounds)(::windows::core::Vtable::as_raw(self), gradientmesh.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn DrawInk<P0, P1, P2>(&self, ink: P0, brush: P1, inkstyle: P2)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Ink>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P2: ::std::convert::Into<::windows::core::InParam<ID2D1InkStyle>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.DrawInk)(::windows::core::Vtable::as_raw(self), ink.into().abi(), brush.into().abi(), inkstyle.into().abi())
    }
    pub unsafe fn DrawGradientMesh<P0>(&self, gradientmesh: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1GradientMesh>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.DrawGradientMesh)(::windows::core::Vtable::as_raw(self), gradientmesh.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawGdiMetafile2<P0>(&self, gdimetafile: P0, destinationrectangle: ::core::option::Option<*const Common::D2D_RECT_F>, sourcerectangle: ::core::option::Option<*const Common::D2D_RECT_F>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1GdiMetafile>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.DrawGdiMetafile2)(::windows::core::Vtable::as_raw(self), gdimetafile.into().abi(), ::core::mem::transmute(destinationrectangle.unwrap_or(::std::ptr::null())), ::core::mem::transmute(sourcerectangle.unwrap_or(::std::ptr::null())))
    }
    pub unsafe fn CreateTransformedImageSource<P0>(&self, imagesource: P0, properties: *const D2D1_TRANSFORMED_IMAGE_SOURCE_PROPERTIES) -> ::windows::core::Result<ID2D1TransformedImageSource>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1ImageSource>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateTransformedImageSource)(::windows::core::Vtable::as_raw(self), imagesource.into().abi(), properties, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateSpriteBatch(&self) -> ::windows::core::Result<ID2D1SpriteBatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateSpriteBatch)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn DrawSpriteBatch<P0, P1>(&self, spritebatch: P0, startindex: u32, spritecount: u32, bitmap: P1, interpolationmode: D2D1_BITMAP_INTERPOLATION_MODE, spriteoptions: D2D1_SPRITE_OPTIONS)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1SpriteBatch>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Bitmap>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.DrawSpriteBatch)(::windows::core::Vtable::as_raw(self), spritebatch.into().abi(), startindex, spritecount, bitmap.into().abi(), interpolationmode, spriteoptions)
    }
    pub unsafe fn CreateSvgGlyphStyle(&self) -> ::windows::core::Result<ID2D1SvgGlyphStyle> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateSvgGlyphStyle)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn DrawText2<P0, P1, P2>(&self, string: &[u16], textformat: P0, layoutrect: *const Common::D2D_RECT_F, defaultfillbrush: P1, svgglyphstyle: P2, colorpaletteindex: u32, options: D2D1_DRAW_TEXT_OPTIONS, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::DirectWrite::IDWriteTextFormat>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P2: ::std::convert::Into<::windows::core::InParam<ID2D1SvgGlyphStyle>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.DrawText2)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(string.as_ptr()), string.len() as _, textformat.into().abi(), layoutrect, defaultfillbrush.into().abi(), svgglyphstyle.into().abi(), colorpaletteindex, options, measuringmode)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn DrawTextLayout2<P0, P1, P2>(&self, origin: Common::D2D_POINT_2F, textlayout: P0, defaultfillbrush: P1, svgglyphstyle: P2, colorpaletteindex: u32, options: D2D1_DRAW_TEXT_OPTIONS)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::DirectWrite::IDWriteTextLayout>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P2: ::std::convert::Into<::windows::core::InParam<ID2D1SvgGlyphStyle>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.DrawTextLayout2)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(origin), textlayout.into().abi(), defaultfillbrush.into().abi(), svgglyphstyle.into().abi(), colorpaletteindex, options)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn DrawColorBitmapGlyphRun(&self, glyphimageformat: super::DirectWrite::DWRITE_GLYPH_IMAGE_FORMATS, baselineorigin: Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE, bitmapsnapoption: D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION) {
        (::windows::core::Vtable::vtable(self).base__.base__.DrawColorBitmapGlyphRun)(::windows::core::Vtable::as_raw(self), glyphimageformat, ::core::mem::transmute(baselineorigin), glyphrun, measuringmode, bitmapsnapoption)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn DrawSvgGlyphRun<P0, P1>(&self, baselineorigin: Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, defaultfillbrush: P0, svgglyphstyle: P1, colorpaletteindex: u32, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1SvgGlyphStyle>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.DrawSvgGlyphRun)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(baselineorigin), glyphrun, defaultfillbrush.into().abi(), svgglyphstyle.into().abi(), colorpaletteindex, measuringmode)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn GetColorBitmapGlyphImage<P0, P1>(&self, glyphimageformat: super::DirectWrite::DWRITE_GLYPH_IMAGE_FORMATS, glyphorigin: Common::D2D_POINT_2F, fontface: P0, fontemsize: f32, glyphindex: u16, issideways: P1, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, dpix: f32, dpiy: f32, glyphtransform: *mut super::super::super::Foundation::Numerics::Matrix3x2, glyphimage: *mut ::core::option::Option<ID2D1Image>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::DirectWrite::IDWriteFontFace>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.GetColorBitmapGlyphImage)(::windows::core::Vtable::as_raw(self), glyphimageformat, ::core::mem::transmute(glyphorigin), fontface.into().abi(), fontemsize, glyphindex, issideways.into(), ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), dpix, dpiy, glyphtransform, ::core::mem::transmute(glyphimage)).ok()
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn GetSvgGlyphImage<P0, P1, P2, P3>(&self, glyphorigin: Common::D2D_POINT_2F, fontface: P0, fontemsize: f32, glyphindex: u16, issideways: P1, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, defaultfillbrush: P2, svgglyphstyle: P3, colorpaletteindex: u32, glyphtransform: *mut super::super::super::Foundation::Numerics::Matrix3x2, glyphimage: *mut ::core::option::Option<ID2D1CommandList>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::DirectWrite::IDWriteFontFace>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
        P2: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P3: ::std::convert::Into<::windows::core::InParam<ID2D1SvgGlyphStyle>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.GetSvgGlyphImage)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(glyphorigin), fontface.into().abi(), fontemsize, glyphindex, issideways.into(), ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), defaultfillbrush.into().abi(), svgglyphstyle.into().abi(), colorpaletteindex, glyphtransform, ::core::mem::transmute(glyphimage)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_System_Com"))]
    pub unsafe fn CreateSvgDocument<P0>(&self, inputxmlstream: P0, viewportsize: Common::D2D_SIZE_F) -> ::windows::core::Result<ID2D1SvgDocument>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IStream>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateSvgDocument)(::windows::core::Vtable::as_raw(self), inputxmlstream.into().abi(), ::core::mem::transmute(viewportsize), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn DrawSvgDocument<P0>(&self, svgdocument: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1SvgDocument>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DrawSvgDocument)(::windows::core::Vtable::as_raw(self), svgdocument.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateColorContextFromDxgiColorSpace(&self, colorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE) -> ::windows::core::Result<ID2D1ColorContext1> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateColorContextFromDxgiColorSpace)(::windows::core::Vtable::as_raw(self), colorspace, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CreateColorContextFromSimpleColorProfile(&self, simpleprofile: *const D2D1_SIMPLE_COLOR_PROFILE) -> ::windows::core::Result<ID2D1ColorContext1> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateColorContextFromSimpleColorProfile)(::windows::core::Vtable::as_raw(self), simpleprofile, result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for ID2D1DrawInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1DrawInfo {}
impl ::core::fmt::Debug for ID2D1DrawInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1DrawInfo").field(&self.0).finish()
    }
}
impl ID2D1DrawInfo {
    pub unsafe fn SetInputDescription(&self, inputindex: u32, inputdescription: D2D1_INPUT_DESCRIPTION) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetInputDescription)(::windows::core::Vtable::as_raw(self), inputindex, ::core::mem::transmute(inputdescription)).ok()
    }
    pub unsafe fn SetOutputBuffer(&self, bufferprecision: D2D1_BUFFER_PRECISION, channeldepth: D2D1_CHANNEL_DEPTH) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetOutputBuffer)(::windows::core::Vtable::as_raw(self), bufferprecision, channeldepth).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCached<P0>(&self, iscached: P0)
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetCached)(::windows::core::Vtable::as_raw(self), iscached.into())
    }
    pub unsafe fn SetInstructionCountHint(&self, instructioncount: u32) {
        (::windows::core::Vtable::vtable(self).base__.SetInstructionCountHint)(::windows::core::Vtable::as_raw(self), instructioncount)
    }
}
impl ::core::cmp::PartialEq for ID2D1DrawTransform {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1DrawTransform {}
impl ::core::fmt::Debug for ID2D1DrawTransform {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1DrawTransform").field(&self.0).finish()
    }
}
impl ID2D1DrawTransform {
    pub unsafe fn GetInputCount(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.base__.GetInputCount)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MapOutputRectToInputRects(&self, outputrect: *const super::super::Foundation::RECT, inputrects: &mut [super::super::Foundation::RECT]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.MapOutputRectToInputRects)(::windows::core::Vtable::as_raw(self), outputrect, ::core::mem::transmute(inputrects.as_ptr()), inputrects.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MapInputRectsToOutputRect(&self, inputrects: *const super::super::Foundation::RECT, inputopaquesubrects: *const super::super::Foundation::RECT, inputrectcount: u32, outputrect: *mut super::super::Foundation::RECT, outputopaquesubrect: *mut super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.MapInputRectsToOutputRect)(::windows::core::Vtable::as_raw(self), inputrects, inputopaquesubrects, inputrectcount, outputrect, outputopaquesubrect).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MapInvalidRect(&self, inputindex: u32, invalidinputrect: super::super::Foundation::RECT) -> ::windows::core::Result<super::super::Foundation::RECT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.MapInvalidRect)(::windows::core::Vtable::as_raw(self), inputindex, ::core::mem::transmute(invalidinputrect), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for ID2D1DrawingStateBlock {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1DrawingStateBlock {}
impl ::core::fmt::Debug for ID2D1DrawingStateBlock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1DrawingStateBlock").field(&self.0).finish()
    }
}
impl ID2D1DrawingStateBlock {
    pub unsafe fn GetFactory(&self) -> ::windows::core::Result<ID2D1Factory> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFactory)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1Factory as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
}
impl ::core::cmp::PartialEq for ID2D1DrawingStateBlock1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1DrawingStateBlock1 {}
impl ::core::fmt::Debug for ID2D1DrawingStateBlock1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1DrawingStateBlock1").field(&self.0).finish()
    }
}
impl ID2D1DrawingStateBlock1 {
    pub unsafe fn GetFactory(&self) -> ::windows::core::Result<ID2D1Factory> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetFactory)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1Factory as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn GetDescription(&self, statedescription: *mut D2D1_DRAWING_STATE_DESCRIPTION) {
        (::windows::core::Vtable::vtable(self).base__.GetDescription)(::windows::core::Vtable::as_raw(self), statedescription)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn SetDescription(&self, statedescription: *const D2D1_DRAWING_STATE_DESCRIPTION) {
        (::windows::core::Vtable::vtable(self).base__.SetDescription)(::windows::core::Vtable::as_raw(self), statedescription)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(feature = "Win32_Graphics_DirectWrite")]
    pub unsafe fn SetTextRenderingParams<P0>(&self, textrenderingparams: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::DirectWrite::IDWriteRenderingParams>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetTextRenderingParams)(::windows::core::Vtable::as_raw(self), textrenderingparams.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(feature = "Win32_Graphics_DirectWrite")]
    pub unsafe fn GetTextRenderingParams(&self) -> ::windows::core::Result<super::DirectWrite::IDWriteRenderingParams> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetTextRenderingParams)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <super::DirectWrite::IDWriteRenderingParams as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
}
impl ::core::cmp::PartialEq for ID2D1Effect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1Effect {}
impl ::core::fmt::Debug for ID2D1Effect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1Effect").field(&self.0).finish()
    }
}
impl ID2D1Effect {
    pub unsafe fn GetPropertyCount(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.GetPropertyCount)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetPropertyName(&self, index: u32, name: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetPropertyName)(::windows::core::Vtable::as_raw(self), index, ::core::mem::transmute(name.as_ptr()), name.len() as _).ok()
    }
    pub unsafe fn GetPropertyNameLength(&self, index: u32) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.GetPropertyNameLength)(::windows::core::Vtable::as_raw(self), index)
    }
    pub unsafe fn GetType(&self, index: u32) -> D2D1_PROPERTY_TYPE {
        (::windows::core::Vtable::vtable(self).base__.GetType)(::windows::core::Vtable::as_raw(self), index)
    }
    pub unsafe fn GetPropertyIndex<P0>(&self, name: P0) -> u32
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetPropertyIndex)(::windows::core::Vtable::as_raw(self), name.into().abi())
    }
    pub unsafe fn SetValueByName<P0>(&self, name: P0, r#type: D2D1_PROPERTY_TYPE, data: &[u8]) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetValueByName)(::windows::core::Vtable::as_raw(self), name.into().abi(), r#type, ::core::mem::transmute(data.as_ptr()), data.len() as _).ok()
    }
    pub unsafe fn SetValue(&self, index: u32, r#type: D2D1_PROPERTY_TYPE, data: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetValue)(::windows::core::Vtable::as_raw(self), index, r#type, ::core::mem::transmute(data.as_ptr()), data.len() as _).ok()
    }
    pub unsafe fn GetValueByName<P0>(&self, name: P0, r#type: D2D1_PROPERTY_TYPE, data: &mut [u8]) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetValueByName)(::windows::core::Vtable::as_raw(self), name.into().abi(), r#type, ::core::mem::transmute(data.as_ptr()), data.len() as _).ok()
    }
    pub unsafe fn GetValue(&self, index: u32, r#type: D2D1_PROPERTY_TYPE, data: &mut [u8]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetValue)(::windows::core::Vtable::as_raw(self), index, r#type, ::core::mem::transmute(data.as_ptr()), data.len() as _).ok()
    }
    pub unsafe fn GetValueSize(&self, index: u32) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.GetValueSize)(::windows::core::Vtable::as_raw(self), index)
    }
    pub unsafe fn GetSubProperties(&self, index: u32) -> ::windows::core::Result<ID2D1Properties> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetSubProperties)(::windows::core::Vtable::as_raw(self), index, result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for ID2D1EffectContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1EffectContext {}
impl ::core::fmt::Debug for ID2D1EffectContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1EffectContext").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID2D1EffectContext1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1EffectContext1 {}
impl ::core::fmt::Debug for ID2D1EffectContext1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1EffectContext1").field(&self.0).finish()
    }
}
impl ID2D1EffectContext1 {
    pub unsafe fn GetDpi(&self, dpix: *mut f32, dpiy: *mut f32) {
        (::windows::core::Vtable::vtable(self).base__.GetDpi)(::windows::core::Vtable::as_raw(self), dpix, dpiy)
    }
    pub unsafe fn CreateEffect(&self, effectid: *const ::windows::core::GUID) -> ::windows::core::Result<ID2D1Effect> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateEffect)(::windows::core::Vtable::as_raw(self), effectid, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetMaximumSupportedFeatureLevel(&self, featurelevels: &[super::Direct3D::D3D_FEATURE_LEVEL]) -> ::windows::core::Result<super::Direct3D::D3D_FEATURE_LEVEL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetMaximumSupportedFeatureLevel)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(featurelevels.as_ptr()), featurelevels.len() as _, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateTransformNodeFromEffect<P0>(&self, effect: P0) -> ::windows::core::Result<ID2D1TransformNode>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Effect>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateTransformNodeFromEffect)(::windows::core::Vtable::as_raw(self), effect.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateBlendTransform(&self, numinputs: u32, blenddescription: *const D2D1_BLEND_DESCRIPTION) -> ::windows::core::Result<ID2D1BlendTransform> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateBlendTransform)(::windows::core::Vtable::as_raw(self), numinputs, blenddescription, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateBorderTransform(&self, extendmodex: D2D1_EXTEND_MODE, extendmodey: D2D1_EXTEND_MODE) -> ::windows::core::Result<ID2D1BorderTransform> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateBorderTransform)(::windows::core::Vtable::as_raw(self), extendmodex, extendmodey, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateOffsetTransform(&self, offset: super::super::Foundation::POINT) -> ::windows::core::Result<ID2D1OffsetTransform> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateOffsetTransform)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(offset), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateBoundsAdjustmentTransform(&self, outputrectangle: *const super::super::Foundation::RECT) -> ::windows::core::Result<ID2D1BoundsAdjustmentTransform> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateBoundsAdjustmentTransform)(::windows::core::Vtable::as_raw(self), outputrectangle, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn LoadPixelShader(&self, shaderid: *const ::windows::core::GUID, shaderbuffer: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.LoadPixelShader)(::windows::core::Vtable::as_raw(self), shaderid, ::core::mem::transmute(shaderbuffer.as_ptr()), shaderbuffer.len() as _).ok()
    }
    pub unsafe fn LoadVertexShader(&self, resourceid: *const ::windows::core::GUID, shaderbuffer: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.LoadVertexShader)(::windows::core::Vtable::as_raw(self), resourceid, ::core::mem::transmute(shaderbuffer.as_ptr()), shaderbuffer.len() as _).ok()
    }
    pub unsafe fn LoadComputeShader(&self, resourceid: *const ::windows::core::GUID, shaderbuffer: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.LoadComputeShader)(::windows::core::Vtable::as_raw(self), resourceid, ::core::mem::transmute(shaderbuffer.as_ptr()), shaderbuffer.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsShaderLoaded(&self, shaderid: *const ::windows::core::GUID) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.IsShaderLoaded)(::windows::core::Vtable::as_raw(self), shaderid)
    }
    pub unsafe fn CreateResourceTexture(&self, resourceid: ::core::option::Option<*const ::windows::core::GUID>, resourcetextureproperties: *const D2D1_RESOURCE_TEXTURE_PROPERTIES, data: ::core::option::Option<&[u8]>, strides: ::core::option::Option<*const u32>) -> ::windows::core::Result<ID2D1ResourceTexture> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateResourceTexture)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(resourceid.unwrap_or(::std::ptr::null())), resourcetextureproperties, ::core::mem::transmute(data.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), ::core::mem::transmute(strides.unwrap_or(::std::ptr::null())), data.as_deref().map_or(0, |slice| slice.len() as _), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FindResourceTexture(&self, resourceid: *const ::windows::core::GUID) -> ::windows::core::Result<ID2D1ResourceTexture> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.FindResourceTexture)(::windows::core::Vtable::as_raw(self), resourceid, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateVertexBuffer(&self, vertexbufferproperties: *const D2D1_VERTEX_BUFFER_PROPERTIES, resourceid: ::core::option::Option<*const ::windows::core::GUID>, customvertexbufferproperties: ::core::option::Option<*const D2D1_CUSTOM_VERTEX_BUFFER_PROPERTIES>) -> ::windows::core::Result<ID2D1VertexBuffer> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateVertexBuffer)(::windows::core::Vtable::as_raw(self), vertexbufferproperties, ::core::mem::transmute(resourceid.unwrap_or(::std::ptr::null())), ::core::mem::transmute(customvertexbufferproperties.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FindVertexBuffer(&self, resourceid: *const ::windows::core::GUID) -> ::windows::core::Result<ID2D1VertexBuffer> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.FindVertexBuffer)(::windows::core::Vtable::as_raw(self), resourceid, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateColorContext(&self, space: D2D1_COLOR_SPACE, profile: ::core::option::Option<&[u8]>) -> ::windows::core::Result<ID2D1ColorContext> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateColorContext)(::windows::core::Vtable::as_raw(self), space, ::core::mem::transmute(profile.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), profile.as_deref().map_or(0, |slice| slice.len() as _), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateColorContextFromFilename<P0>(&self, filename: P0) -> ::windows::core::Result<ID2D1ColorContext>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateColorContextFromFilename)(::windows::core::Vtable::as_raw(self), filename.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub unsafe fn CreateColorContextFromWicColorContext<P0>(&self, wiccolorcontext: P0) -> ::windows::core::Result<ID2D1ColorContext>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Imaging::IWICColorContext>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateColorContextFromWicColorContext)(::windows::core::Vtable::as_raw(self), wiccolorcontext.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CheckFeatureSupport(&self, feature: D2D1_FEATURE, featuresupportdata: *mut ::core::ffi::c_void, featuresupportdatasize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CheckFeatureSupport)(::windows::core::Vtable::as_raw(self), feature, featuresupportdata, featuresupportdatasize).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsBufferPrecisionSupported(&self, bufferprecision: D2D1_BUFFER_PRECISION) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.IsBufferPrecisionSupported)(::windows::core::Vtable::as_raw(self), bufferprecision)
    }
}
impl ::core::cmp::PartialEq for ID2D1EffectContext2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1EffectContext2 {}
impl ::core::fmt::Debug for ID2D1EffectContext2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1EffectContext2").field(&self.0).finish()
    }
}
impl ID2D1EffectContext2 {
    pub unsafe fn GetDpi(&self, dpix: *mut f32, dpiy: *mut f32) {
        (::windows::core::Vtable::vtable(self).base__.base__.GetDpi)(::windows::core::Vtable::as_raw(self), dpix, dpiy)
    }
    pub unsafe fn CreateEffect(&self, effectid: *const ::windows::core::GUID) -> ::windows::core::Result<ID2D1Effect> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateEffect)(::windows::core::Vtable::as_raw(self), effectid, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetMaximumSupportedFeatureLevel(&self, featurelevels: &[super::Direct3D::D3D_FEATURE_LEVEL]) -> ::windows::core::Result<super::Direct3D::D3D_FEATURE_LEVEL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetMaximumSupportedFeatureLevel)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(featurelevels.as_ptr()), featurelevels.len() as _, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateTransformNodeFromEffect<P0>(&self, effect: P0) -> ::windows::core::Result<ID2D1TransformNode>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Effect>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateTransformNodeFromEffect)(::windows::core::Vtable::as_raw(self), effect.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateBlendTransform(&self, numinputs: u32, blenddescription: *const D2D1_BLEND_DESCRIPTION) -> ::windows::core::Result<ID2D1BlendTransform> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateBlendTransform)(::windows::core::Vtable::as_raw(self), numinputs, blenddescription, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateBorderTransform(&self, extendmodex: D2D1_EXTEND_MODE, extendmodey: D2D1_EXTEND_MODE) -> ::windows::core::Result<ID2D1BorderTransform> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateBorderTransform)(::windows::core::Vtable::as_raw(self), extendmodex, extendmodey, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateOffsetTransform(&self, offset: super::super::Foundation::POINT) -> ::windows::core::Result<ID2D1OffsetTransform> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateOffsetTransform)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(offset), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateBoundsAdjustmentTransform(&self, outputrectangle: *const super::super::Foundation::RECT) -> ::windows::core::Result<ID2D1BoundsAdjustmentTransform> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateBoundsAdjustmentTransform)(::windows::core::Vtable::as_raw(self), outputrectangle, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn LoadPixelShader(&self, shaderid: *const ::windows::core::GUID, shaderbuffer: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.LoadPixelShader)(::windows::core::Vtable::as_raw(self), shaderid, ::core::mem::transmute(shaderbuffer.as_ptr()), shaderbuffer.len() as _).ok()
    }
    pub unsafe fn LoadVertexShader(&self, resourceid: *const ::windows::core::GUID, shaderbuffer: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.LoadVertexShader)(::windows::core::Vtable::as_raw(self), resourceid, ::core::mem::transmute(shaderbuffer.as_ptr()), shaderbuffer.len() as _).ok()
    }
    pub unsafe fn LoadComputeShader(&self, resourceid: *const ::windows::core::GUID, shaderbuffer: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.LoadComputeShader)(::windows::core::Vtable::as_raw(self), resourceid, ::core::mem::transmute(shaderbuffer.as_ptr()), shaderbuffer.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsShaderLoaded(&self, shaderid: *const ::windows::core::GUID) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.base__.IsShaderLoaded)(::windows::core::Vtable::as_raw(self), shaderid)
    }
    pub unsafe fn CreateResourceTexture(&self, resourceid: ::core::option::Option<*const ::windows::core::GUID>, resourcetextureproperties: *const D2D1_RESOURCE_TEXTURE_PROPERTIES, data: ::core::option::Option<&[u8]>, strides: ::core::option::Option<*const u32>) -> ::windows::core::Result<ID2D1ResourceTexture> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateResourceTexture)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(resourceid.unwrap_or(::std::ptr::null())), resourcetextureproperties, ::core::mem::transmute(data.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), ::core::mem::transmute(strides.unwrap_or(::std::ptr::null())), data.as_deref().map_or(0, |slice| slice.len() as _), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FindResourceTexture(&self, resourceid: *const ::windows::core::GUID) -> ::windows::core::Result<ID2D1ResourceTexture> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.FindResourceTexture)(::windows::core::Vtable::as_raw(self), resourceid, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateVertexBuffer(&self, vertexbufferproperties: *const D2D1_VERTEX_BUFFER_PROPERTIES, resourceid: ::core::option::Option<*const ::windows::core::GUID>, customvertexbufferproperties: ::core::option::Option<*const D2D1_CUSTOM_VERTEX_BUFFER_PROPERTIES>) -> ::windows::core::Result<ID2D1VertexBuffer> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateVertexBuffer)(::windows::core::Vtable::as_raw(self), vertexbufferproperties, ::core::mem::transmute(resourceid.unwrap_or(::std::ptr::null())), ::core::mem::transmute(customvertexbufferproperties.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FindVertexBuffer(&self, resourceid: *const ::windows::core::GUID) -> ::windows::core::Result<ID2D1VertexBuffer> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.FindVertexBuffer)(::windows::core::Vtable::as_raw(self), resourceid, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateColorContext(&self, space: D2D1_COLOR_SPACE, profile: ::core::option::Option<&[u8]>) -> ::windows::core::Result<ID2D1ColorContext> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateColorContext)(::windows::core::Vtable::as_raw(self), space, ::core::mem::transmute(profile.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), profile.as_deref().map_or(0, |slice| slice.len() as _), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateColorContextFromFilename<P0>(&self, filename: P0) -> ::windows::core::Result<ID2D1ColorContext>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateColorContextFromFilename)(::windows::core::Vtable::as_raw(self), filename.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub unsafe fn CreateColorContextFromWicColorContext<P0>(&self, wiccolorcontext: P0) -> ::windows::core::Result<ID2D1ColorContext>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Imaging::IWICColorContext>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateColorContextFromWicColorContext)(::windows::core::Vtable::as_raw(self), wiccolorcontext.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CheckFeatureSupport(&self, feature: D2D1_FEATURE, featuresupportdata: *mut ::core::ffi::c_void, featuresupportdatasize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.CheckFeatureSupport)(::windows::core::Vtable::as_raw(self), feature, featuresupportdata, featuresupportdatasize).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsBufferPrecisionSupported(&self, bufferprecision: D2D1_BUFFER_PRECISION) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.base__.IsBufferPrecisionSupported)(::windows::core::Vtable::as_raw(self), bufferprecision)
    }
    pub unsafe fn CreateLookupTable3D(&self, precision: D2D1_BUFFER_PRECISION, extents: &[u32; 3], data: &[u8], strides: &[u32; 2]) -> ::windows::core::Result<ID2D1LookupTable3D> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateLookupTable3D)(::windows::core::Vtable::as_raw(self), precision, ::core::mem::transmute(extents.as_ptr()), ::core::mem::transmute(data.as_ptr()), data.len() as _, ::core::mem::transmute(strides.as_ptr()), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for ID2D1EffectImpl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1EffectImpl {}
impl ::core::fmt::Debug for ID2D1EffectImpl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1EffectImpl").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID2D1EllipseGeometry {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1EllipseGeometry {}
impl ::core::fmt::Debug for ID2D1EllipseGeometry {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1EllipseGeometry").field(&self.0).finish()
    }
}
impl ID2D1EllipseGeometry {
    pub unsafe fn GetFactory(&self) -> ::windows::core::Result<ID2D1Factory> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetFactory)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1Factory as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn GetBounds(&self, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>) -> ::windows::core::Result<Common::D2D_RECT_F> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetBounds)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn GetWidenedBounds<P0>(&self, strokewidth: f32, strokestyle: P0, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32) -> ::windows::core::Result<Common::D2D_RECT_F>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetWidenedBounds)(::windows::core::Vtable::as_raw(self), strokewidth, strokestyle.into().abi(), ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), flatteningtolerance, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn StrokeContainsPoint<P0>(&self, point: Common::D2D_POINT_2F, strokewidth: f32, strokestyle: P0, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.StrokeContainsPoint)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(point), strokewidth, strokestyle.into().abi(), ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), flatteningtolerance, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn FillContainsPoint(&self, point: Common::D2D_POINT_2F, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.FillContainsPoint)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(point), ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), flatteningtolerance, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn CompareWithGeometry<P0>(&self, inputgeometry: P0, inputgeometrytransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32) -> ::windows::core::Result<D2D1_GEOMETRY_RELATION>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Geometry>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CompareWithGeometry)(::windows::core::Vtable::as_raw(self), inputgeometry.into().abi(), ::core::mem::transmute(inputgeometrytransform.unwrap_or(::std::ptr::null())), flatteningtolerance, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn Simplify<P0>(&self, simplificationoption: D2D1_GEOMETRY_SIMPLIFICATION_OPTION, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32, geometrysink: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<Common::ID2D1SimplifiedGeometrySink>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Simplify)(::windows::core::Vtable::as_raw(self), simplificationoption, ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), flatteningtolerance, geometrysink.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn Tessellate<P0>(&self, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32, tessellationsink: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1TessellationSink>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Tessellate)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), flatteningtolerance, tessellationsink.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn CombineWithGeometry<P0, P1>(&self, inputgeometry: P0, combinemode: D2D1_COMBINE_MODE, inputgeometrytransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32, geometrysink: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Geometry>>,
        P1: ::std::convert::Into<::windows::core::InParam<Common::ID2D1SimplifiedGeometrySink>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CombineWithGeometry)(::windows::core::Vtable::as_raw(self), inputgeometry.into().abi(), combinemode, ::core::mem::transmute(inputgeometrytransform.unwrap_or(::std::ptr::null())), flatteningtolerance, geometrysink.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn Outline<P0>(&self, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32, geometrysink: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<Common::ID2D1SimplifiedGeometrySink>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Outline)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), flatteningtolerance, geometrysink.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn ComputeArea(&self, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ComputeArea)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), flatteningtolerance, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn ComputeLength(&self, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ComputeLength)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), flatteningtolerance, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn ComputePointAtLength(&self, length: f32, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32, point: ::core::option::Option<*mut Common::D2D_POINT_2F>, unittangentvector: ::core::option::Option<*mut Common::D2D_POINT_2F>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ComputePointAtLength)(::windows::core::Vtable::as_raw(self), length, ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), flatteningtolerance, ::core::mem::transmute(point.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(unittangentvector.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn Widen<P0, P1>(&self, strokewidth: f32, strokestyle: P0, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32, geometrysink: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
        P1: ::std::convert::Into<::windows::core::InParam<Common::ID2D1SimplifiedGeometrySink>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Widen)(::windows::core::Vtable::as_raw(self), strokewidth, strokestyle.into().abi(), ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), flatteningtolerance, geometrysink.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for ID2D1Factory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1Factory {}
impl ::core::fmt::Debug for ID2D1Factory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1Factory").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID2D1Factory1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1Factory1 {}
impl ::core::fmt::Debug for ID2D1Factory1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1Factory1").field(&self.0).finish()
    }
}
impl ID2D1Factory1 {
    pub unsafe fn ReloadSystemMetrics(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ReloadSystemMetrics)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetDesktopDpi(&self, dpix: *mut f32, dpiy: *mut f32) {
        (::windows::core::Vtable::vtable(self).base__.GetDesktopDpi)(::windows::core::Vtable::as_raw(self), dpix, dpiy)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CreateRectangleGeometry(&self, rectangle: *const Common::D2D_RECT_F) -> ::windows::core::Result<ID2D1RectangleGeometry> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateRectangleGeometry)(::windows::core::Vtable::as_raw(self), rectangle, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CreateRoundedRectangleGeometry(&self, roundedrectangle: *const D2D1_ROUNDED_RECT) -> ::windows::core::Result<ID2D1RoundedRectangleGeometry> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateRoundedRectangleGeometry)(::windows::core::Vtable::as_raw(self), roundedrectangle, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CreateEllipseGeometry(&self, ellipse: *const D2D1_ELLIPSE) -> ::windows::core::Result<ID2D1EllipseGeometry> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateEllipseGeometry)(::windows::core::Vtable::as_raw(self), ellipse, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CreateGeometryGroup(&self, fillmode: Common::D2D1_FILL_MODE, geometries: &[ID2D1Geometry]) -> ::windows::core::Result<ID2D1GeometryGroup> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateGeometryGroup)(::windows::core::Vtable::as_raw(self), fillmode, ::core::mem::transmute(geometries.as_ptr()), geometries.len() as _, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn CreateTransformedGeometry<P0>(&self, sourcegeometry: P0, transform: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::Result<ID2D1TransformedGeometry>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Geometry>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateTransformedGeometry)(::windows::core::Vtable::as_raw(self), sourcegeometry.into().abi(), transform, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreatePathGeometry(&self) -> ::windows::core::Result<ID2D1PathGeometry> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreatePathGeometry)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateStrokeStyle(&self, strokestyleproperties: *const D2D1_STROKE_STYLE_PROPERTIES, dashes: ::core::option::Option<&[f32]>) -> ::windows::core::Result<ID2D1StrokeStyle> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateStrokeStyle)(::windows::core::Vtable::as_raw(self), strokestyleproperties, ::core::mem::transmute(dashes.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), dashes.as_deref().map_or(0, |slice| slice.len() as _), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn CreateDrawingStateBlock<P0>(&self, drawingstatedescription: ::core::option::Option<*const D2D1_DRAWING_STATE_DESCRIPTION>, textrenderingparams: P0) -> ::windows::core::Result<ID2D1DrawingStateBlock>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::DirectWrite::IDWriteRenderingParams>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateDrawingStateBlock)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(drawingstatedescription.unwrap_or(::std::ptr::null())), textrenderingparams.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Imaging\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
    pub unsafe fn CreateWicBitmapRenderTarget<P0>(&self, target: P0, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES) -> ::windows::core::Result<ID2D1RenderTarget>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Imaging::IWICBitmap>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateWicBitmapRenderTarget)(::windows::core::Vtable::as_raw(self), target.into().abi(), rendertargetproperties, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateHwndRenderTarget(&self, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES, hwndrendertargetproperties: *const D2D1_HWND_RENDER_TARGET_PROPERTIES) -> ::windows::core::Result<ID2D1HwndRenderTarget> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateHwndRenderTarget)(::windows::core::Vtable::as_raw(self), rendertargetproperties, hwndrendertargetproperties, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateDxgiSurfaceRenderTarget<P0>(&self, dxgisurface: P0, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES) -> ::windows::core::Result<ID2D1RenderTarget>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Dxgi::IDXGISurface>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateDxgiSurfaceRenderTarget)(::windows::core::Vtable::as_raw(self), dxgisurface.into().abi(), rendertargetproperties, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateDCRenderTarget(&self, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES) -> ::windows::core::Result<ID2D1DCRenderTarget> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateDCRenderTarget)(::windows::core::Vtable::as_raw(self), rendertargetproperties, result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for ID2D1Factory2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1Factory2 {}
impl ::core::fmt::Debug for ID2D1Factory2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1Factory2").field(&self.0).finish()
    }
}
impl ID2D1Factory2 {
    pub unsafe fn ReloadSystemMetrics(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.ReloadSystemMetrics)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetDesktopDpi(&self, dpix: *mut f32, dpiy: *mut f32) {
        (::windows::core::Vtable::vtable(self).base__.base__.GetDesktopDpi)(::windows::core::Vtable::as_raw(self), dpix, dpiy)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CreateRectangleGeometry(&self, rectangle: *const Common::D2D_RECT_F) -> ::windows::core::Result<ID2D1RectangleGeometry> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateRectangleGeometry)(::windows::core::Vtable::as_raw(self), rectangle, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CreateRoundedRectangleGeometry(&self, roundedrectangle: *const D2D1_ROUNDED_RECT) -> ::windows::core::Result<ID2D1RoundedRectangleGeometry> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateRoundedRectangleGeometry)(::windows::core::Vtable::as_raw(self), roundedrectangle, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CreateEllipseGeometry(&self, ellipse: *const D2D1_ELLIPSE) -> ::windows::core::Result<ID2D1EllipseGeometry> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateEllipseGeometry)(::windows::core::Vtable::as_raw(self), ellipse, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CreateGeometryGroup(&self, fillmode: Common::D2D1_FILL_MODE, geometries: &[ID2D1Geometry]) -> ::windows::core::Result<ID2D1GeometryGroup> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateGeometryGroup)(::windows::core::Vtable::as_raw(self), fillmode, ::core::mem::transmute(geometries.as_ptr()), geometries.len() as _, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn CreateTransformedGeometry<P0>(&self, sourcegeometry: P0, transform: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::Result<ID2D1TransformedGeometry>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Geometry>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateTransformedGeometry)(::windows::core::Vtable::as_raw(self), sourcegeometry.into().abi(), transform, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreatePathGeometry(&self) -> ::windows::core::Result<ID2D1PathGeometry> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreatePathGeometry)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateStrokeStyle(&self, strokestyleproperties: *const D2D1_STROKE_STYLE_PROPERTIES, dashes: ::core::option::Option<&[f32]>) -> ::windows::core::Result<ID2D1StrokeStyle> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateStrokeStyle)(::windows::core::Vtable::as_raw(self), strokestyleproperties, ::core::mem::transmute(dashes.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), dashes.as_deref().map_or(0, |slice| slice.len() as _), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn CreateDrawingStateBlock<P0>(&self, drawingstatedescription: ::core::option::Option<*const D2D1_DRAWING_STATE_DESCRIPTION>, textrenderingparams: P0) -> ::windows::core::Result<ID2D1DrawingStateBlock>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::DirectWrite::IDWriteRenderingParams>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateDrawingStateBlock)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(drawingstatedescription.unwrap_or(::std::ptr::null())), textrenderingparams.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Imaging\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
    pub unsafe fn CreateWicBitmapRenderTarget<P0>(&self, target: P0, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES) -> ::windows::core::Result<ID2D1RenderTarget>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Imaging::IWICBitmap>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateWicBitmapRenderTarget)(::windows::core::Vtable::as_raw(self), target.into().abi(), rendertargetproperties, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateHwndRenderTarget(&self, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES, hwndrendertargetproperties: *const D2D1_HWND_RENDER_TARGET_PROPERTIES) -> ::windows::core::Result<ID2D1HwndRenderTarget> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateHwndRenderTarget)(::windows::core::Vtable::as_raw(self), rendertargetproperties, hwndrendertargetproperties, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateDxgiSurfaceRenderTarget<P0>(&self, dxgisurface: P0, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES) -> ::windows::core::Result<ID2D1RenderTarget>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Dxgi::IDXGISurface>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateDxgiSurfaceRenderTarget)(::windows::core::Vtable::as_raw(self), dxgisurface.into().abi(), rendertargetproperties, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateDCRenderTarget(&self, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES) -> ::windows::core::Result<ID2D1DCRenderTarget> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateDCRenderTarget)(::windows::core::Vtable::as_raw(self), rendertargetproperties, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn CreateDevice<P0>(&self, dxgidevice: P0) -> ::windows::core::Result<ID2D1Device>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Dxgi::IDXGIDevice>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateDevice)(::windows::core::Vtable::as_raw(self), dxgidevice.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateStrokeStyle2(&self, strokestyleproperties: *const D2D1_STROKE_STYLE_PROPERTIES1, dashes: ::core::option::Option<&[f32]>) -> ::windows::core::Result<ID2D1StrokeStyle1> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateStrokeStyle2)(::windows::core::Vtable::as_raw(self), strokestyleproperties, ::core::mem::transmute(dashes.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), dashes.as_deref().map_or(0, |slice| slice.len() as _), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreatePathGeometry2(&self) -> ::windows::core::Result<ID2D1PathGeometry1> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreatePathGeometry2)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn CreateDrawingStateBlock2<P0>(&self, drawingstatedescription: ::core::option::Option<*const D2D1_DRAWING_STATE_DESCRIPTION1>, textrenderingparams: P0) -> ::windows::core::Result<ID2D1DrawingStateBlock1>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::DirectWrite::IDWriteRenderingParams>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateDrawingStateBlock2)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(drawingstatedescription.unwrap_or(::std::ptr::null())), textrenderingparams.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateGdiMetafile<P0>(&self, metafilestream: P0) -> ::windows::core::Result<ID2D1GdiMetafile>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IStream>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateGdiMetafile)(::windows::core::Vtable::as_raw(self), metafilestream.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RegisterEffectFromStream<P0>(&self, classid: *const ::windows::core::GUID, propertyxml: P0, bindings: ::core::option::Option<&[D2D1_PROPERTY_BINDING]>, effectfactory: PD2D1_EFFECT_FACTORY) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IStream>>,
    {
        (::windows::core::Vtable::vtable(self).base__.RegisterEffectFromStream)(::windows::core::Vtable::as_raw(self), classid, propertyxml.into().abi(), ::core::mem::transmute(bindings.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), bindings.as_deref().map_or(0, |slice| slice.len() as _), effectfactory).ok()
    }
    pub unsafe fn RegisterEffectFromString<P0>(&self, classid: *const ::windows::core::GUID, propertyxml: P0, bindings: ::core::option::Option<&[D2D1_PROPERTY_BINDING]>, effectfactory: PD2D1_EFFECT_FACTORY) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.RegisterEffectFromString)(::windows::core::Vtable::as_raw(self), classid, propertyxml.into().abi(), ::core::mem::transmute(bindings.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), bindings.as_deref().map_or(0, |slice| slice.len() as _), effectfactory).ok()
    }
    pub unsafe fn UnregisterEffect(&self, classid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.UnregisterEffect)(::windows::core::Vtable::as_raw(self), classid).ok()
    }
    pub unsafe fn GetRegisteredEffects(&self, effects: ::core::option::Option<&mut [::windows::core::GUID]>, effectsreturned: ::core::option::Option<*mut u32>, effectsregistered: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetRegisteredEffects)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(effects.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), effects.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(effectsreturned.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(effectsregistered.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetEffectProperties(&self, effectid: *const ::windows::core::GUID) -> ::windows::core::Result<ID2D1Properties> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetEffectProperties)(::windows::core::Vtable::as_raw(self), effectid, result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for ID2D1Factory3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1Factory3 {}
impl ::core::fmt::Debug for ID2D1Factory3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1Factory3").field(&self.0).finish()
    }
}
impl ID2D1Factory3 {
    pub unsafe fn ReloadSystemMetrics(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ReloadSystemMetrics)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetDesktopDpi(&self, dpix: *mut f32, dpiy: *mut f32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetDesktopDpi)(::windows::core::Vtable::as_raw(self), dpix, dpiy)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CreateRectangleGeometry(&self, rectangle: *const Common::D2D_RECT_F) -> ::windows::core::Result<ID2D1RectangleGeometry> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateRectangleGeometry)(::windows::core::Vtable::as_raw(self), rectangle, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CreateRoundedRectangleGeometry(&self, roundedrectangle: *const D2D1_ROUNDED_RECT) -> ::windows::core::Result<ID2D1RoundedRectangleGeometry> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateRoundedRectangleGeometry)(::windows::core::Vtable::as_raw(self), roundedrectangle, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CreateEllipseGeometry(&self, ellipse: *const D2D1_ELLIPSE) -> ::windows::core::Result<ID2D1EllipseGeometry> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateEllipseGeometry)(::windows::core::Vtable::as_raw(self), ellipse, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CreateGeometryGroup(&self, fillmode: Common::D2D1_FILL_MODE, geometries: &[ID2D1Geometry]) -> ::windows::core::Result<ID2D1GeometryGroup> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateGeometryGroup)(::windows::core::Vtable::as_raw(self), fillmode, ::core::mem::transmute(geometries.as_ptr()), geometries.len() as _, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn CreateTransformedGeometry<P0>(&self, sourcegeometry: P0, transform: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::Result<ID2D1TransformedGeometry>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Geometry>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateTransformedGeometry)(::windows::core::Vtable::as_raw(self), sourcegeometry.into().abi(), transform, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreatePathGeometry(&self) -> ::windows::core::Result<ID2D1PathGeometry> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreatePathGeometry)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateStrokeStyle(&self, strokestyleproperties: *const D2D1_STROKE_STYLE_PROPERTIES, dashes: ::core::option::Option<&[f32]>) -> ::windows::core::Result<ID2D1StrokeStyle> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateStrokeStyle)(::windows::core::Vtable::as_raw(self), strokestyleproperties, ::core::mem::transmute(dashes.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), dashes.as_deref().map_or(0, |slice| slice.len() as _), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn CreateDrawingStateBlock<P0>(&self, drawingstatedescription: ::core::option::Option<*const D2D1_DRAWING_STATE_DESCRIPTION>, textrenderingparams: P0) -> ::windows::core::Result<ID2D1DrawingStateBlock>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::DirectWrite::IDWriteRenderingParams>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateDrawingStateBlock)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(drawingstatedescription.unwrap_or(::std::ptr::null())), textrenderingparams.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Imaging\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
    pub unsafe fn CreateWicBitmapRenderTarget<P0>(&self, target: P0, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES) -> ::windows::core::Result<ID2D1RenderTarget>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Imaging::IWICBitmap>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateWicBitmapRenderTarget)(::windows::core::Vtable::as_raw(self), target.into().abi(), rendertargetproperties, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateHwndRenderTarget(&self, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES, hwndrendertargetproperties: *const D2D1_HWND_RENDER_TARGET_PROPERTIES) -> ::windows::core::Result<ID2D1HwndRenderTarget> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateHwndRenderTarget)(::windows::core::Vtable::as_raw(self), rendertargetproperties, hwndrendertargetproperties, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateDxgiSurfaceRenderTarget<P0>(&self, dxgisurface: P0, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES) -> ::windows::core::Result<ID2D1RenderTarget>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Dxgi::IDXGISurface>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateDxgiSurfaceRenderTarget)(::windows::core::Vtable::as_raw(self), dxgisurface.into().abi(), rendertargetproperties, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateDCRenderTarget(&self, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES) -> ::windows::core::Result<ID2D1DCRenderTarget> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateDCRenderTarget)(::windows::core::Vtable::as_raw(self), rendertargetproperties, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn CreateDevice<P0>(&self, dxgidevice: P0) -> ::windows::core::Result<ID2D1Device>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Dxgi::IDXGIDevice>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateDevice)(::windows::core::Vtable::as_raw(self), dxgidevice.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateStrokeStyle2(&self, strokestyleproperties: *const D2D1_STROKE_STYLE_PROPERTIES1, dashes: ::core::option::Option<&[f32]>) -> ::windows::core::Result<ID2D1StrokeStyle1> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateStrokeStyle2)(::windows::core::Vtable::as_raw(self), strokestyleproperties, ::core::mem::transmute(dashes.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), dashes.as_deref().map_or(0, |slice| slice.len() as _), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreatePathGeometry2(&self) -> ::windows::core::Result<ID2D1PathGeometry1> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreatePathGeometry2)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn CreateDrawingStateBlock2<P0>(&self, drawingstatedescription: ::core::option::Option<*const D2D1_DRAWING_STATE_DESCRIPTION1>, textrenderingparams: P0) -> ::windows::core::Result<ID2D1DrawingStateBlock1>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::DirectWrite::IDWriteRenderingParams>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateDrawingStateBlock2)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(drawingstatedescription.unwrap_or(::std::ptr::null())), textrenderingparams.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateGdiMetafile<P0>(&self, metafilestream: P0) -> ::windows::core::Result<ID2D1GdiMetafile>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IStream>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateGdiMetafile)(::windows::core::Vtable::as_raw(self), metafilestream.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RegisterEffectFromStream<P0>(&self, classid: *const ::windows::core::GUID, propertyxml: P0, bindings: ::core::option::Option<&[D2D1_PROPERTY_BINDING]>, effectfactory: PD2D1_EFFECT_FACTORY) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IStream>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.RegisterEffectFromStream)(::windows::core::Vtable::as_raw(self), classid, propertyxml.into().abi(), ::core::mem::transmute(bindings.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), bindings.as_deref().map_or(0, |slice| slice.len() as _), effectfactory).ok()
    }
    pub unsafe fn RegisterEffectFromString<P0>(&self, classid: *const ::windows::core::GUID, propertyxml: P0, bindings: ::core::option::Option<&[D2D1_PROPERTY_BINDING]>, effectfactory: PD2D1_EFFECT_FACTORY) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.RegisterEffectFromString)(::windows::core::Vtable::as_raw(self), classid, propertyxml.into().abi(), ::core::mem::transmute(bindings.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), bindings.as_deref().map_or(0, |slice| slice.len() as _), effectfactory).ok()
    }
    pub unsafe fn UnregisterEffect(&self, classid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.UnregisterEffect)(::windows::core::Vtable::as_raw(self), classid).ok()
    }
    pub unsafe fn GetRegisteredEffects(&self, effects: ::core::option::Option<&mut [::windows::core::GUID]>, effectsreturned: ::core::option::Option<*mut u32>, effectsregistered: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetRegisteredEffects)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(effects.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), effects.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(effectsreturned.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(effectsregistered.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetEffectProperties(&self, effectid: *const ::windows::core::GUID) -> ::windows::core::Result<ID2D1Properties> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetEffectProperties)(::windows::core::Vtable::as_raw(self), effectid, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn CreateDevice2<P0>(&self, dxgidevice: P0) -> ::windows::core::Result<ID2D1Device1>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Dxgi::IDXGIDevice>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateDevice2)(::windows::core::Vtable::as_raw(self), dxgidevice.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for ID2D1Factory4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1Factory4 {}
impl ::core::fmt::Debug for ID2D1Factory4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1Factory4").field(&self.0).finish()
    }
}
impl ID2D1Factory4 {
    pub unsafe fn ReloadSystemMetrics(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.ReloadSystemMetrics)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetDesktopDpi(&self, dpix: *mut f32, dpiy: *mut f32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetDesktopDpi)(::windows::core::Vtable::as_raw(self), dpix, dpiy)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CreateRectangleGeometry(&self, rectangle: *const Common::D2D_RECT_F) -> ::windows::core::Result<ID2D1RectangleGeometry> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateRectangleGeometry)(::windows::core::Vtable::as_raw(self), rectangle, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CreateRoundedRectangleGeometry(&self, roundedrectangle: *const D2D1_ROUNDED_RECT) -> ::windows::core::Result<ID2D1RoundedRectangleGeometry> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateRoundedRectangleGeometry)(::windows::core::Vtable::as_raw(self), roundedrectangle, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CreateEllipseGeometry(&self, ellipse: *const D2D1_ELLIPSE) -> ::windows::core::Result<ID2D1EllipseGeometry> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateEllipseGeometry)(::windows::core::Vtable::as_raw(self), ellipse, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CreateGeometryGroup(&self, fillmode: Common::D2D1_FILL_MODE, geometries: &[ID2D1Geometry]) -> ::windows::core::Result<ID2D1GeometryGroup> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateGeometryGroup)(::windows::core::Vtable::as_raw(self), fillmode, ::core::mem::transmute(geometries.as_ptr()), geometries.len() as _, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn CreateTransformedGeometry<P0>(&self, sourcegeometry: P0, transform: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::Result<ID2D1TransformedGeometry>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Geometry>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateTransformedGeometry)(::windows::core::Vtable::as_raw(self), sourcegeometry.into().abi(), transform, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreatePathGeometry(&self) -> ::windows::core::Result<ID2D1PathGeometry> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreatePathGeometry)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateStrokeStyle(&self, strokestyleproperties: *const D2D1_STROKE_STYLE_PROPERTIES, dashes: ::core::option::Option<&[f32]>) -> ::windows::core::Result<ID2D1StrokeStyle> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateStrokeStyle)(::windows::core::Vtable::as_raw(self), strokestyleproperties, ::core::mem::transmute(dashes.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), dashes.as_deref().map_or(0, |slice| slice.len() as _), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn CreateDrawingStateBlock<P0>(&self, drawingstatedescription: ::core::option::Option<*const D2D1_DRAWING_STATE_DESCRIPTION>, textrenderingparams: P0) -> ::windows::core::Result<ID2D1DrawingStateBlock>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::DirectWrite::IDWriteRenderingParams>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateDrawingStateBlock)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(drawingstatedescription.unwrap_or(::std::ptr::null())), textrenderingparams.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Imaging\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
    pub unsafe fn CreateWicBitmapRenderTarget<P0>(&self, target: P0, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES) -> ::windows::core::Result<ID2D1RenderTarget>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Imaging::IWICBitmap>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateWicBitmapRenderTarget)(::windows::core::Vtable::as_raw(self), target.into().abi(), rendertargetproperties, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateHwndRenderTarget(&self, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES, hwndrendertargetproperties: *const D2D1_HWND_RENDER_TARGET_PROPERTIES) -> ::windows::core::Result<ID2D1HwndRenderTarget> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateHwndRenderTarget)(::windows::core::Vtable::as_raw(self), rendertargetproperties, hwndrendertargetproperties, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateDxgiSurfaceRenderTarget<P0>(&self, dxgisurface: P0, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES) -> ::windows::core::Result<ID2D1RenderTarget>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Dxgi::IDXGISurface>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateDxgiSurfaceRenderTarget)(::windows::core::Vtable::as_raw(self), dxgisurface.into().abi(), rendertargetproperties, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateDCRenderTarget(&self, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES) -> ::windows::core::Result<ID2D1DCRenderTarget> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateDCRenderTarget)(::windows::core::Vtable::as_raw(self), rendertargetproperties, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn CreateDevice<P0>(&self, dxgidevice: P0) -> ::windows::core::Result<ID2D1Device>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Dxgi::IDXGIDevice>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateDevice)(::windows::core::Vtable::as_raw(self), dxgidevice.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateStrokeStyle2(&self, strokestyleproperties: *const D2D1_STROKE_STYLE_PROPERTIES1, dashes: ::core::option::Option<&[f32]>) -> ::windows::core::Result<ID2D1StrokeStyle1> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateStrokeStyle2)(::windows::core::Vtable::as_raw(self), strokestyleproperties, ::core::mem::transmute(dashes.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), dashes.as_deref().map_or(0, |slice| slice.len() as _), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreatePathGeometry2(&self) -> ::windows::core::Result<ID2D1PathGeometry1> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreatePathGeometry2)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn CreateDrawingStateBlock2<P0>(&self, drawingstatedescription: ::core::option::Option<*const D2D1_DRAWING_STATE_DESCRIPTION1>, textrenderingparams: P0) -> ::windows::core::Result<ID2D1DrawingStateBlock1>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::DirectWrite::IDWriteRenderingParams>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateDrawingStateBlock2)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(drawingstatedescription.unwrap_or(::std::ptr::null())), textrenderingparams.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateGdiMetafile<P0>(&self, metafilestream: P0) -> ::windows::core::Result<ID2D1GdiMetafile>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IStream>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateGdiMetafile)(::windows::core::Vtable::as_raw(self), metafilestream.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RegisterEffectFromStream<P0>(&self, classid: *const ::windows::core::GUID, propertyxml: P0, bindings: ::core::option::Option<&[D2D1_PROPERTY_BINDING]>, effectfactory: PD2D1_EFFECT_FACTORY) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IStream>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.RegisterEffectFromStream)(::windows::core::Vtable::as_raw(self), classid, propertyxml.into().abi(), ::core::mem::transmute(bindings.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), bindings.as_deref().map_or(0, |slice| slice.len() as _), effectfactory).ok()
    }
    pub unsafe fn RegisterEffectFromString<P0>(&self, classid: *const ::windows::core::GUID, propertyxml: P0, bindings: ::core::option::Option<&[D2D1_PROPERTY_BINDING]>, effectfactory: PD2D1_EFFECT_FACTORY) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.RegisterEffectFromString)(::windows::core::Vtable::as_raw(self), classid, propertyxml.into().abi(), ::core::mem::transmute(bindings.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), bindings.as_deref().map_or(0, |slice| slice.len() as _), effectfactory).ok()
    }
    pub unsafe fn UnregisterEffect(&self, classid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.UnregisterEffect)(::windows::core::Vtable::as_raw(self), classid).ok()
    }
    pub unsafe fn GetRegisteredEffects(&self, effects: ::core::option::Option<&mut [::windows::core::GUID]>, effectsreturned: ::core::option::Option<*mut u32>, effectsregistered: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetRegisteredEffects)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(effects.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), effects.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(effectsreturned.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(effectsregistered.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetEffectProperties(&self, effectid: *const ::windows::core::GUID) -> ::windows::core::Result<ID2D1Properties> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetEffectProperties)(::windows::core::Vtable::as_raw(self), effectid, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn CreateDevice2<P0>(&self, dxgidevice: P0) -> ::windows::core::Result<ID2D1Device1>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Dxgi::IDXGIDevice>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateDevice2)(::windows::core::Vtable::as_raw(self), dxgidevice.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn CreateDevice3<P0>(&self, dxgidevice: P0) -> ::windows::core::Result<ID2D1Device2>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Dxgi::IDXGIDevice>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateDevice3)(::windows::core::Vtable::as_raw(self), dxgidevice.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for ID2D1Factory5 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1Factory5 {}
impl ::core::fmt::Debug for ID2D1Factory5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1Factory5").field(&self.0).finish()
    }
}
impl ID2D1Factory5 {
    pub unsafe fn ReloadSystemMetrics(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.ReloadSystemMetrics)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetDesktopDpi(&self, dpix: *mut f32, dpiy: *mut f32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetDesktopDpi)(::windows::core::Vtable::as_raw(self), dpix, dpiy)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CreateRectangleGeometry(&self, rectangle: *const Common::D2D_RECT_F) -> ::windows::core::Result<ID2D1RectangleGeometry> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateRectangleGeometry)(::windows::core::Vtable::as_raw(self), rectangle, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CreateRoundedRectangleGeometry(&self, roundedrectangle: *const D2D1_ROUNDED_RECT) -> ::windows::core::Result<ID2D1RoundedRectangleGeometry> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateRoundedRectangleGeometry)(::windows::core::Vtable::as_raw(self), roundedrectangle, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CreateEllipseGeometry(&self, ellipse: *const D2D1_ELLIPSE) -> ::windows::core::Result<ID2D1EllipseGeometry> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateEllipseGeometry)(::windows::core::Vtable::as_raw(self), ellipse, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CreateGeometryGroup(&self, fillmode: Common::D2D1_FILL_MODE, geometries: &[ID2D1Geometry]) -> ::windows::core::Result<ID2D1GeometryGroup> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateGeometryGroup)(::windows::core::Vtable::as_raw(self), fillmode, ::core::mem::transmute(geometries.as_ptr()), geometries.len() as _, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn CreateTransformedGeometry<P0>(&self, sourcegeometry: P0, transform: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::Result<ID2D1TransformedGeometry>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Geometry>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateTransformedGeometry)(::windows::core::Vtable::as_raw(self), sourcegeometry.into().abi(), transform, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreatePathGeometry(&self) -> ::windows::core::Result<ID2D1PathGeometry> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreatePathGeometry)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateStrokeStyle(&self, strokestyleproperties: *const D2D1_STROKE_STYLE_PROPERTIES, dashes: ::core::option::Option<&[f32]>) -> ::windows::core::Result<ID2D1StrokeStyle> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateStrokeStyle)(::windows::core::Vtable::as_raw(self), strokestyleproperties, ::core::mem::transmute(dashes.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), dashes.as_deref().map_or(0, |slice| slice.len() as _), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn CreateDrawingStateBlock<P0>(&self, drawingstatedescription: ::core::option::Option<*const D2D1_DRAWING_STATE_DESCRIPTION>, textrenderingparams: P0) -> ::windows::core::Result<ID2D1DrawingStateBlock>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::DirectWrite::IDWriteRenderingParams>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateDrawingStateBlock)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(drawingstatedescription.unwrap_or(::std::ptr::null())), textrenderingparams.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Imaging\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
    pub unsafe fn CreateWicBitmapRenderTarget<P0>(&self, target: P0, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES) -> ::windows::core::Result<ID2D1RenderTarget>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Imaging::IWICBitmap>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateWicBitmapRenderTarget)(::windows::core::Vtable::as_raw(self), target.into().abi(), rendertargetproperties, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateHwndRenderTarget(&self, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES, hwndrendertargetproperties: *const D2D1_HWND_RENDER_TARGET_PROPERTIES) -> ::windows::core::Result<ID2D1HwndRenderTarget> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateHwndRenderTarget)(::windows::core::Vtable::as_raw(self), rendertargetproperties, hwndrendertargetproperties, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateDxgiSurfaceRenderTarget<P0>(&self, dxgisurface: P0, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES) -> ::windows::core::Result<ID2D1RenderTarget>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Dxgi::IDXGISurface>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateDxgiSurfaceRenderTarget)(::windows::core::Vtable::as_raw(self), dxgisurface.into().abi(), rendertargetproperties, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateDCRenderTarget(&self, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES) -> ::windows::core::Result<ID2D1DCRenderTarget> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateDCRenderTarget)(::windows::core::Vtable::as_raw(self), rendertargetproperties, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn CreateDevice<P0>(&self, dxgidevice: P0) -> ::windows::core::Result<ID2D1Device>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Dxgi::IDXGIDevice>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateDevice)(::windows::core::Vtable::as_raw(self), dxgidevice.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateStrokeStyle2(&self, strokestyleproperties: *const D2D1_STROKE_STYLE_PROPERTIES1, dashes: ::core::option::Option<&[f32]>) -> ::windows::core::Result<ID2D1StrokeStyle1> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateStrokeStyle2)(::windows::core::Vtable::as_raw(self), strokestyleproperties, ::core::mem::transmute(dashes.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), dashes.as_deref().map_or(0, |slice| slice.len() as _), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreatePathGeometry2(&self) -> ::windows::core::Result<ID2D1PathGeometry1> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreatePathGeometry2)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn CreateDrawingStateBlock2<P0>(&self, drawingstatedescription: ::core::option::Option<*const D2D1_DRAWING_STATE_DESCRIPTION1>, textrenderingparams: P0) -> ::windows::core::Result<ID2D1DrawingStateBlock1>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::DirectWrite::IDWriteRenderingParams>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateDrawingStateBlock2)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(drawingstatedescription.unwrap_or(::std::ptr::null())), textrenderingparams.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateGdiMetafile<P0>(&self, metafilestream: P0) -> ::windows::core::Result<ID2D1GdiMetafile>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IStream>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateGdiMetafile)(::windows::core::Vtable::as_raw(self), metafilestream.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RegisterEffectFromStream<P0>(&self, classid: *const ::windows::core::GUID, propertyxml: P0, bindings: ::core::option::Option<&[D2D1_PROPERTY_BINDING]>, effectfactory: PD2D1_EFFECT_FACTORY) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IStream>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.RegisterEffectFromStream)(::windows::core::Vtable::as_raw(self), classid, propertyxml.into().abi(), ::core::mem::transmute(bindings.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), bindings.as_deref().map_or(0, |slice| slice.len() as _), effectfactory).ok()
    }
    pub unsafe fn RegisterEffectFromString<P0>(&self, classid: *const ::windows::core::GUID, propertyxml: P0, bindings: ::core::option::Option<&[D2D1_PROPERTY_BINDING]>, effectfactory: PD2D1_EFFECT_FACTORY) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.RegisterEffectFromString)(::windows::core::Vtable::as_raw(self), classid, propertyxml.into().abi(), ::core::mem::transmute(bindings.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), bindings.as_deref().map_or(0, |slice| slice.len() as _), effectfactory).ok()
    }
    pub unsafe fn UnregisterEffect(&self, classid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.UnregisterEffect)(::windows::core::Vtable::as_raw(self), classid).ok()
    }
    pub unsafe fn GetRegisteredEffects(&self, effects: ::core::option::Option<&mut [::windows::core::GUID]>, effectsreturned: ::core::option::Option<*mut u32>, effectsregistered: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetRegisteredEffects)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(effects.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), effects.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(effectsreturned.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(effectsregistered.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetEffectProperties(&self, effectid: *const ::windows::core::GUID) -> ::windows::core::Result<ID2D1Properties> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetEffectProperties)(::windows::core::Vtable::as_raw(self), effectid, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn CreateDevice2<P0>(&self, dxgidevice: P0) -> ::windows::core::Result<ID2D1Device1>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Dxgi::IDXGIDevice>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateDevice2)(::windows::core::Vtable::as_raw(self), dxgidevice.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn CreateDevice3<P0>(&self, dxgidevice: P0) -> ::windows::core::Result<ID2D1Device2>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Dxgi::IDXGIDevice>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateDevice3)(::windows::core::Vtable::as_raw(self), dxgidevice.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn CreateDevice4<P0>(&self, dxgidevice: P0) -> ::windows::core::Result<ID2D1Device3>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Dxgi::IDXGIDevice>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateDevice4)(::windows::core::Vtable::as_raw(self), dxgidevice.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for ID2D1Factory6 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1Factory6 {}
impl ::core::fmt::Debug for ID2D1Factory6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1Factory6").field(&self.0).finish()
    }
}
impl ID2D1Factory6 {
    pub unsafe fn ReloadSystemMetrics(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.ReloadSystemMetrics)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetDesktopDpi(&self, dpix: *mut f32, dpiy: *mut f32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.GetDesktopDpi)(::windows::core::Vtable::as_raw(self), dpix, dpiy)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CreateRectangleGeometry(&self, rectangle: *const Common::D2D_RECT_F) -> ::windows::core::Result<ID2D1RectangleGeometry> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CreateRectangleGeometry)(::windows::core::Vtable::as_raw(self), rectangle, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CreateRoundedRectangleGeometry(&self, roundedrectangle: *const D2D1_ROUNDED_RECT) -> ::windows::core::Result<ID2D1RoundedRectangleGeometry> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CreateRoundedRectangleGeometry)(::windows::core::Vtable::as_raw(self), roundedrectangle, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CreateEllipseGeometry(&self, ellipse: *const D2D1_ELLIPSE) -> ::windows::core::Result<ID2D1EllipseGeometry> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CreateEllipseGeometry)(::windows::core::Vtable::as_raw(self), ellipse, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CreateGeometryGroup(&self, fillmode: Common::D2D1_FILL_MODE, geometries: &[ID2D1Geometry]) -> ::windows::core::Result<ID2D1GeometryGroup> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CreateGeometryGroup)(::windows::core::Vtable::as_raw(self), fillmode, ::core::mem::transmute(geometries.as_ptr()), geometries.len() as _, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn CreateTransformedGeometry<P0>(&self, sourcegeometry: P0, transform: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::Result<ID2D1TransformedGeometry>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Geometry>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CreateTransformedGeometry)(::windows::core::Vtable::as_raw(self), sourcegeometry.into().abi(), transform, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreatePathGeometry(&self) -> ::windows::core::Result<ID2D1PathGeometry> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CreatePathGeometry)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateStrokeStyle(&self, strokestyleproperties: *const D2D1_STROKE_STYLE_PROPERTIES, dashes: ::core::option::Option<&[f32]>) -> ::windows::core::Result<ID2D1StrokeStyle> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CreateStrokeStyle)(::windows::core::Vtable::as_raw(self), strokestyleproperties, ::core::mem::transmute(dashes.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), dashes.as_deref().map_or(0, |slice| slice.len() as _), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn CreateDrawingStateBlock<P0>(&self, drawingstatedescription: ::core::option::Option<*const D2D1_DRAWING_STATE_DESCRIPTION>, textrenderingparams: P0) -> ::windows::core::Result<ID2D1DrawingStateBlock>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::DirectWrite::IDWriteRenderingParams>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CreateDrawingStateBlock)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(drawingstatedescription.unwrap_or(::std::ptr::null())), textrenderingparams.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Imaging\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
    pub unsafe fn CreateWicBitmapRenderTarget<P0>(&self, target: P0, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES) -> ::windows::core::Result<ID2D1RenderTarget>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Imaging::IWICBitmap>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CreateWicBitmapRenderTarget)(::windows::core::Vtable::as_raw(self), target.into().abi(), rendertargetproperties, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateHwndRenderTarget(&self, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES, hwndrendertargetproperties: *const D2D1_HWND_RENDER_TARGET_PROPERTIES) -> ::windows::core::Result<ID2D1HwndRenderTarget> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CreateHwndRenderTarget)(::windows::core::Vtable::as_raw(self), rendertargetproperties, hwndrendertargetproperties, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateDxgiSurfaceRenderTarget<P0>(&self, dxgisurface: P0, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES) -> ::windows::core::Result<ID2D1RenderTarget>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Dxgi::IDXGISurface>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CreateDxgiSurfaceRenderTarget)(::windows::core::Vtable::as_raw(self), dxgisurface.into().abi(), rendertargetproperties, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateDCRenderTarget(&self, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES) -> ::windows::core::Result<ID2D1DCRenderTarget> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CreateDCRenderTarget)(::windows::core::Vtable::as_raw(self), rendertargetproperties, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn CreateDevice<P0>(&self, dxgidevice: P0) -> ::windows::core::Result<ID2D1Device>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Dxgi::IDXGIDevice>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateDevice)(::windows::core::Vtable::as_raw(self), dxgidevice.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateStrokeStyle2(&self, strokestyleproperties: *const D2D1_STROKE_STYLE_PROPERTIES1, dashes: ::core::option::Option<&[f32]>) -> ::windows::core::Result<ID2D1StrokeStyle1> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateStrokeStyle2)(::windows::core::Vtable::as_raw(self), strokestyleproperties, ::core::mem::transmute(dashes.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), dashes.as_deref().map_or(0, |slice| slice.len() as _), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreatePathGeometry2(&self) -> ::windows::core::Result<ID2D1PathGeometry1> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreatePathGeometry2)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn CreateDrawingStateBlock2<P0>(&self, drawingstatedescription: ::core::option::Option<*const D2D1_DRAWING_STATE_DESCRIPTION1>, textrenderingparams: P0) -> ::windows::core::Result<ID2D1DrawingStateBlock1>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::DirectWrite::IDWriteRenderingParams>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateDrawingStateBlock2)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(drawingstatedescription.unwrap_or(::std::ptr::null())), textrenderingparams.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateGdiMetafile<P0>(&self, metafilestream: P0) -> ::windows::core::Result<ID2D1GdiMetafile>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IStream>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateGdiMetafile)(::windows::core::Vtable::as_raw(self), metafilestream.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RegisterEffectFromStream<P0>(&self, classid: *const ::windows::core::GUID, propertyxml: P0, bindings: ::core::option::Option<&[D2D1_PROPERTY_BINDING]>, effectfactory: PD2D1_EFFECT_FACTORY) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IStream>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.RegisterEffectFromStream)(::windows::core::Vtable::as_raw(self), classid, propertyxml.into().abi(), ::core::mem::transmute(bindings.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), bindings.as_deref().map_or(0, |slice| slice.len() as _), effectfactory).ok()
    }
    pub unsafe fn RegisterEffectFromString<P0>(&self, classid: *const ::windows::core::GUID, propertyxml: P0, bindings: ::core::option::Option<&[D2D1_PROPERTY_BINDING]>, effectfactory: PD2D1_EFFECT_FACTORY) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.RegisterEffectFromString)(::windows::core::Vtable::as_raw(self), classid, propertyxml.into().abi(), ::core::mem::transmute(bindings.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), bindings.as_deref().map_or(0, |slice| slice.len() as _), effectfactory).ok()
    }
    pub unsafe fn UnregisterEffect(&self, classid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.UnregisterEffect)(::windows::core::Vtable::as_raw(self), classid).ok()
    }
    pub unsafe fn GetRegisteredEffects(&self, effects: ::core::option::Option<&mut [::windows::core::GUID]>, effectsreturned: ::core::option::Option<*mut u32>, effectsregistered: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetRegisteredEffects)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(effects.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), effects.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(effectsreturned.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(effectsregistered.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetEffectProperties(&self, effectid: *const ::windows::core::GUID) -> ::windows::core::Result<ID2D1Properties> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetEffectProperties)(::windows::core::Vtable::as_raw(self), effectid, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn CreateDevice2<P0>(&self, dxgidevice: P0) -> ::windows::core::Result<ID2D1Device1>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Dxgi::IDXGIDevice>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateDevice2)(::windows::core::Vtable::as_raw(self), dxgidevice.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn CreateDevice3<P0>(&self, dxgidevice: P0) -> ::windows::core::Result<ID2D1Device2>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Dxgi::IDXGIDevice>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateDevice3)(::windows::core::Vtable::as_raw(self), dxgidevice.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn CreateDevice4<P0>(&self, dxgidevice: P0) -> ::windows::core::Result<ID2D1Device3>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Dxgi::IDXGIDevice>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateDevice4)(::windows::core::Vtable::as_raw(self), dxgidevice.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn CreateDevice5<P0>(&self, dxgidevice: P0) -> ::windows::core::Result<ID2D1Device4>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Dxgi::IDXGIDevice>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateDevice5)(::windows::core::Vtable::as_raw(self), dxgidevice.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for ID2D1Factory7 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1Factory7 {}
impl ::core::fmt::Debug for ID2D1Factory7 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1Factory7").field(&self.0).finish()
    }
}
impl ID2D1Factory7 {
    pub unsafe fn ReloadSystemMetrics(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.ReloadSystemMetrics)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetDesktopDpi(&self, dpix: *mut f32, dpiy: *mut f32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.GetDesktopDpi)(::windows::core::Vtable::as_raw(self), dpix, dpiy)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CreateRectangleGeometry(&self, rectangle: *const Common::D2D_RECT_F) -> ::windows::core::Result<ID2D1RectangleGeometry> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CreateRectangleGeometry)(::windows::core::Vtable::as_raw(self), rectangle, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CreateRoundedRectangleGeometry(&self, roundedrectangle: *const D2D1_ROUNDED_RECT) -> ::windows::core::Result<ID2D1RoundedRectangleGeometry> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CreateRoundedRectangleGeometry)(::windows::core::Vtable::as_raw(self), roundedrectangle, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CreateEllipseGeometry(&self, ellipse: *const D2D1_ELLIPSE) -> ::windows::core::Result<ID2D1EllipseGeometry> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CreateEllipseGeometry)(::windows::core::Vtable::as_raw(self), ellipse, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CreateGeometryGroup(&self, fillmode: Common::D2D1_FILL_MODE, geometries: &[ID2D1Geometry]) -> ::windows::core::Result<ID2D1GeometryGroup> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CreateGeometryGroup)(::windows::core::Vtable::as_raw(self), fillmode, ::core::mem::transmute(geometries.as_ptr()), geometries.len() as _, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn CreateTransformedGeometry<P0>(&self, sourcegeometry: P0, transform: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::Result<ID2D1TransformedGeometry>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Geometry>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CreateTransformedGeometry)(::windows::core::Vtable::as_raw(self), sourcegeometry.into().abi(), transform, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreatePathGeometry(&self) -> ::windows::core::Result<ID2D1PathGeometry> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CreatePathGeometry)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateStrokeStyle(&self, strokestyleproperties: *const D2D1_STROKE_STYLE_PROPERTIES, dashes: ::core::option::Option<&[f32]>) -> ::windows::core::Result<ID2D1StrokeStyle> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CreateStrokeStyle)(::windows::core::Vtable::as_raw(self), strokestyleproperties, ::core::mem::transmute(dashes.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), dashes.as_deref().map_or(0, |slice| slice.len() as _), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn CreateDrawingStateBlock<P0>(&self, drawingstatedescription: ::core::option::Option<*const D2D1_DRAWING_STATE_DESCRIPTION>, textrenderingparams: P0) -> ::windows::core::Result<ID2D1DrawingStateBlock>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::DirectWrite::IDWriteRenderingParams>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CreateDrawingStateBlock)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(drawingstatedescription.unwrap_or(::std::ptr::null())), textrenderingparams.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Imaging\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
    pub unsafe fn CreateWicBitmapRenderTarget<P0>(&self, target: P0, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES) -> ::windows::core::Result<ID2D1RenderTarget>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Imaging::IWICBitmap>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CreateWicBitmapRenderTarget)(::windows::core::Vtable::as_raw(self), target.into().abi(), rendertargetproperties, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateHwndRenderTarget(&self, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES, hwndrendertargetproperties: *const D2D1_HWND_RENDER_TARGET_PROPERTIES) -> ::windows::core::Result<ID2D1HwndRenderTarget> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CreateHwndRenderTarget)(::windows::core::Vtable::as_raw(self), rendertargetproperties, hwndrendertargetproperties, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateDxgiSurfaceRenderTarget<P0>(&self, dxgisurface: P0, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES) -> ::windows::core::Result<ID2D1RenderTarget>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Dxgi::IDXGISurface>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CreateDxgiSurfaceRenderTarget)(::windows::core::Vtable::as_raw(self), dxgisurface.into().abi(), rendertargetproperties, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateDCRenderTarget(&self, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES) -> ::windows::core::Result<ID2D1DCRenderTarget> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CreateDCRenderTarget)(::windows::core::Vtable::as_raw(self), rendertargetproperties, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn CreateDevice<P0>(&self, dxgidevice: P0) -> ::windows::core::Result<ID2D1Device>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Dxgi::IDXGIDevice>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CreateDevice)(::windows::core::Vtable::as_raw(self), dxgidevice.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateStrokeStyle2(&self, strokestyleproperties: *const D2D1_STROKE_STYLE_PROPERTIES1, dashes: ::core::option::Option<&[f32]>) -> ::windows::core::Result<ID2D1StrokeStyle1> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CreateStrokeStyle2)(::windows::core::Vtable::as_raw(self), strokestyleproperties, ::core::mem::transmute(dashes.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), dashes.as_deref().map_or(0, |slice| slice.len() as _), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreatePathGeometry2(&self) -> ::windows::core::Result<ID2D1PathGeometry1> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CreatePathGeometry2)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn CreateDrawingStateBlock2<P0>(&self, drawingstatedescription: ::core::option::Option<*const D2D1_DRAWING_STATE_DESCRIPTION1>, textrenderingparams: P0) -> ::windows::core::Result<ID2D1DrawingStateBlock1>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::DirectWrite::IDWriteRenderingParams>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CreateDrawingStateBlock2)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(drawingstatedescription.unwrap_or(::std::ptr::null())), textrenderingparams.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateGdiMetafile<P0>(&self, metafilestream: P0) -> ::windows::core::Result<ID2D1GdiMetafile>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IStream>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CreateGdiMetafile)(::windows::core::Vtable::as_raw(self), metafilestream.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RegisterEffectFromStream<P0>(&self, classid: *const ::windows::core::GUID, propertyxml: P0, bindings: ::core::option::Option<&[D2D1_PROPERTY_BINDING]>, effectfactory: PD2D1_EFFECT_FACTORY) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IStream>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.RegisterEffectFromStream)(::windows::core::Vtable::as_raw(self), classid, propertyxml.into().abi(), ::core::mem::transmute(bindings.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), bindings.as_deref().map_or(0, |slice| slice.len() as _), effectfactory).ok()
    }
    pub unsafe fn RegisterEffectFromString<P0>(&self, classid: *const ::windows::core::GUID, propertyxml: P0, bindings: ::core::option::Option<&[D2D1_PROPERTY_BINDING]>, effectfactory: PD2D1_EFFECT_FACTORY) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.RegisterEffectFromString)(::windows::core::Vtable::as_raw(self), classid, propertyxml.into().abi(), ::core::mem::transmute(bindings.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), bindings.as_deref().map_or(0, |slice| slice.len() as _), effectfactory).ok()
    }
    pub unsafe fn UnregisterEffect(&self, classid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.UnregisterEffect)(::windows::core::Vtable::as_raw(self), classid).ok()
    }
    pub unsafe fn GetRegisteredEffects(&self, effects: ::core::option::Option<&mut [::windows::core::GUID]>, effectsreturned: ::core::option::Option<*mut u32>, effectsregistered: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.GetRegisteredEffects)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(effects.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), effects.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(effectsreturned.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(effectsregistered.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetEffectProperties(&self, effectid: *const ::windows::core::GUID) -> ::windows::core::Result<ID2D1Properties> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.GetEffectProperties)(::windows::core::Vtable::as_raw(self), effectid, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn CreateDevice2<P0>(&self, dxgidevice: P0) -> ::windows::core::Result<ID2D1Device1>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Dxgi::IDXGIDevice>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateDevice2)(::windows::core::Vtable::as_raw(self), dxgidevice.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn CreateDevice3<P0>(&self, dxgidevice: P0) -> ::windows::core::Result<ID2D1Device2>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Dxgi::IDXGIDevice>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateDevice3)(::windows::core::Vtable::as_raw(self), dxgidevice.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn CreateDevice4<P0>(&self, dxgidevice: P0) -> ::windows::core::Result<ID2D1Device3>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Dxgi::IDXGIDevice>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateDevice4)(::windows::core::Vtable::as_raw(self), dxgidevice.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn CreateDevice5<P0>(&self, dxgidevice: P0) -> ::windows::core::Result<ID2D1Device4>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Dxgi::IDXGIDevice>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateDevice5)(::windows::core::Vtable::as_raw(self), dxgidevice.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn CreateDevice6<P0>(&self, dxgidevice: P0) -> ::windows::core::Result<ID2D1Device5>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Dxgi::IDXGIDevice>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateDevice6)(::windows::core::Vtable::as_raw(self), dxgidevice.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for ID2D1GdiInteropRenderTarget {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1GdiInteropRenderTarget {}
impl ::core::fmt::Debug for ID2D1GdiInteropRenderTarget {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1GdiInteropRenderTarget").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID2D1GdiMetafile {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1GdiMetafile {}
impl ::core::fmt::Debug for ID2D1GdiMetafile {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1GdiMetafile").field(&self.0).finish()
    }
}
impl ID2D1GdiMetafile {
    pub unsafe fn GetFactory(&self) -> ::windows::core::Result<ID2D1Factory> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFactory)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1Factory as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
}
impl ::core::cmp::PartialEq for ID2D1GdiMetafile1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1GdiMetafile1 {}
impl ::core::fmt::Debug for ID2D1GdiMetafile1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1GdiMetafile1").field(&self.0).finish()
    }
}
impl ID2D1GdiMetafile1 {
    pub unsafe fn GetFactory(&self) -> ::windows::core::Result<ID2D1Factory> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetFactory)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1Factory as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn Stream<P0>(&self, sink: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1GdiMetafileSink>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Stream)(::windows::core::Vtable::as_raw(self), sink.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetBounds(&self) -> ::windows::core::Result<Common::D2D_RECT_F> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetBounds)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for ID2D1GdiMetafileSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1GdiMetafileSink {}
impl ::core::fmt::Debug for ID2D1GdiMetafileSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1GdiMetafileSink").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID2D1GdiMetafileSink1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1GdiMetafileSink1 {}
impl ::core::fmt::Debug for ID2D1GdiMetafileSink1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1GdiMetafileSink1").field(&self.0).finish()
    }
}
impl ID2D1GdiMetafileSink1 {
    pub unsafe fn ProcessRecord(&self, recordtype: u32, recorddata: ::core::option::Option<*const ::core::ffi::c_void>, recorddatasize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ProcessRecord)(::windows::core::Vtable::as_raw(self), recordtype, ::core::mem::transmute(recorddata.unwrap_or(::std::ptr::null())), recorddatasize).ok()
    }
}
impl ::core::cmp::PartialEq for ID2D1Geometry {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1Geometry {}
impl ::core::fmt::Debug for ID2D1Geometry {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1Geometry").field(&self.0).finish()
    }
}
impl ID2D1Geometry {
    pub unsafe fn GetFactory(&self) -> ::windows::core::Result<ID2D1Factory> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFactory)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1Factory as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
}
impl ::core::cmp::PartialEq for ID2D1GeometryGroup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1GeometryGroup {}
impl ::core::fmt::Debug for ID2D1GeometryGroup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1GeometryGroup").field(&self.0).finish()
    }
}
impl ID2D1GeometryGroup {
    pub unsafe fn GetFactory(&self) -> ::windows::core::Result<ID2D1Factory> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetFactory)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1Factory as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn GetBounds(&self, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>) -> ::windows::core::Result<Common::D2D_RECT_F> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetBounds)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn GetWidenedBounds<P0>(&self, strokewidth: f32, strokestyle: P0, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32) -> ::windows::core::Result<Common::D2D_RECT_F>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetWidenedBounds)(::windows::core::Vtable::as_raw(self), strokewidth, strokestyle.into().abi(), ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), flatteningtolerance, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn StrokeContainsPoint<P0>(&self, point: Common::D2D_POINT_2F, strokewidth: f32, strokestyle: P0, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.StrokeContainsPoint)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(point), strokewidth, strokestyle.into().abi(), ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), flatteningtolerance, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn FillContainsPoint(&self, point: Common::D2D_POINT_2F, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.FillContainsPoint)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(point), ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), flatteningtolerance, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn CompareWithGeometry<P0>(&self, inputgeometry: P0, inputgeometrytransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32) -> ::windows::core::Result<D2D1_GEOMETRY_RELATION>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Geometry>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CompareWithGeometry)(::windows::core::Vtable::as_raw(self), inputgeometry.into().abi(), ::core::mem::transmute(inputgeometrytransform.unwrap_or(::std::ptr::null())), flatteningtolerance, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn Simplify<P0>(&self, simplificationoption: D2D1_GEOMETRY_SIMPLIFICATION_OPTION, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32, geometrysink: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<Common::ID2D1SimplifiedGeometrySink>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Simplify)(::windows::core::Vtable::as_raw(self), simplificationoption, ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), flatteningtolerance, geometrysink.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn Tessellate<P0>(&self, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32, tessellationsink: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1TessellationSink>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Tessellate)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), flatteningtolerance, tessellationsink.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn CombineWithGeometry<P0, P1>(&self, inputgeometry: P0, combinemode: D2D1_COMBINE_MODE, inputgeometrytransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32, geometrysink: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Geometry>>,
        P1: ::std::convert::Into<::windows::core::InParam<Common::ID2D1SimplifiedGeometrySink>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CombineWithGeometry)(::windows::core::Vtable::as_raw(self), inputgeometry.into().abi(), combinemode, ::core::mem::transmute(inputgeometrytransform.unwrap_or(::std::ptr::null())), flatteningtolerance, geometrysink.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn Outline<P0>(&self, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32, geometrysink: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<Common::ID2D1SimplifiedGeometrySink>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Outline)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), flatteningtolerance, geometrysink.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn ComputeArea(&self, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ComputeArea)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), flatteningtolerance, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn ComputeLength(&self, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ComputeLength)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), flatteningtolerance, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn ComputePointAtLength(&self, length: f32, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32, point: ::core::option::Option<*mut Common::D2D_POINT_2F>, unittangentvector: ::core::option::Option<*mut Common::D2D_POINT_2F>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ComputePointAtLength)(::windows::core::Vtable::as_raw(self), length, ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), flatteningtolerance, ::core::mem::transmute(point.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(unittangentvector.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn Widen<P0, P1>(&self, strokewidth: f32, strokestyle: P0, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32, geometrysink: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
        P1: ::std::convert::Into<::windows::core::InParam<Common::ID2D1SimplifiedGeometrySink>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Widen)(::windows::core::Vtable::as_raw(self), strokewidth, strokestyle.into().abi(), ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), flatteningtolerance, geometrysink.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for ID2D1GeometryRealization {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1GeometryRealization {}
impl ::core::fmt::Debug for ID2D1GeometryRealization {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1GeometryRealization").field(&self.0).finish()
    }
}
impl ID2D1GeometryRealization {
    pub unsafe fn GetFactory(&self) -> ::windows::core::Result<ID2D1Factory> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFactory)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1Factory as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::cmp::PartialEq for ID2D1GeometrySink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::cmp::Eq for ID2D1GeometrySink {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ::core::fmt::Debug for ID2D1GeometrySink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1GeometrySink").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ID2D1GeometrySink {
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetFillMode(&self, fillmode: Common::D2D1_FILL_MODE) {
        (::windows::core::Vtable::vtable(self).base__.SetFillMode)(::windows::core::Vtable::as_raw(self), fillmode)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetSegmentFlags(&self, vertexflags: Common::D2D1_PATH_SEGMENT) {
        (::windows::core::Vtable::vtable(self).base__.SetSegmentFlags)(::windows::core::Vtable::as_raw(self), vertexflags)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn BeginFigure(&self, startpoint: Common::D2D_POINT_2F, figurebegin: Common::D2D1_FIGURE_BEGIN) {
        (::windows::core::Vtable::vtable(self).base__.BeginFigure)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(startpoint), figurebegin)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn AddLines(&self, points: &[Common::D2D_POINT_2F]) {
        (::windows::core::Vtable::vtable(self).base__.AddLines)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(points.as_ptr()), points.len() as _)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn AddBeziers(&self, beziers: &[Common::D2D1_BEZIER_SEGMENT]) {
        (::windows::core::Vtable::vtable(self).base__.AddBeziers)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(beziers.as_ptr()), beziers.len() as _)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn EndFigure(&self, figureend: Common::D2D1_FIGURE_END) {
        (::windows::core::Vtable::vtable(self).base__.EndFigure)(::windows::core::Vtable::as_raw(self), figureend)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Close)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
impl ::core::cmp::PartialEq for ID2D1GradientMesh {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1GradientMesh {}
impl ::core::fmt::Debug for ID2D1GradientMesh {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1GradientMesh").field(&self.0).finish()
    }
}
impl ID2D1GradientMesh {
    pub unsafe fn GetFactory(&self) -> ::windows::core::Result<ID2D1Factory> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFactory)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1Factory as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
}
impl ::core::cmp::PartialEq for ID2D1GradientStopCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1GradientStopCollection {}
impl ::core::fmt::Debug for ID2D1GradientStopCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1GradientStopCollection").field(&self.0).finish()
    }
}
impl ID2D1GradientStopCollection {
    pub unsafe fn GetFactory(&self) -> ::windows::core::Result<ID2D1Factory> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFactory)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1Factory as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
}
impl ::core::cmp::PartialEq for ID2D1GradientStopCollection1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1GradientStopCollection1 {}
impl ::core::fmt::Debug for ID2D1GradientStopCollection1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1GradientStopCollection1").field(&self.0).finish()
    }
}
impl ID2D1GradientStopCollection1 {
    pub unsafe fn GetFactory(&self) -> ::windows::core::Result<ID2D1Factory> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetFactory)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1Factory as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn GetGradientStopCount(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.GetGradientStopCount)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetGradientStops(&self, gradientstops: &mut [D2D1_GRADIENT_STOP]) {
        (::windows::core::Vtable::vtable(self).base__.GetGradientStops)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(gradientstops.as_ptr()), gradientstops.len() as _)
    }
    pub unsafe fn GetColorInterpolationGamma(&self) -> D2D1_GAMMA {
        (::windows::core::Vtable::vtable(self).base__.GetColorInterpolationGamma)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetExtendMode(&self) -> D2D1_EXTEND_MODE {
        (::windows::core::Vtable::vtable(self).base__.GetExtendMode)(::windows::core::Vtable::as_raw(self))
    }
}
impl ::core::cmp::PartialEq for ID2D1HwndRenderTarget {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1HwndRenderTarget {}
impl ::core::fmt::Debug for ID2D1HwndRenderTarget {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1HwndRenderTarget").field(&self.0).finish()
    }
}
impl ID2D1HwndRenderTarget {
    pub unsafe fn GetFactory(&self) -> ::windows::core::Result<ID2D1Factory> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetFactory)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1Factory as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateBitmap(&self, size: Common::D2D_SIZE_U, srcdata: ::core::option::Option<*const ::core::ffi::c_void>, pitch: u32, bitmapproperties: *const D2D1_BITMAP_PROPERTIES) -> ::windows::core::Result<ID2D1Bitmap> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateBitmap)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(size), ::core::mem::transmute(srcdata.unwrap_or(::std::ptr::null())), pitch, bitmapproperties, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Imaging\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Imaging"))]
    pub unsafe fn CreateBitmapFromWicBitmap<P0>(&self, wicbitmapsource: P0, bitmapproperties: ::core::option::Option<*const D2D1_BITMAP_PROPERTIES>) -> ::windows::core::Result<ID2D1Bitmap>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Imaging::IWICBitmapSource>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateBitmapFromWicBitmap)(::windows::core::Vtable::as_raw(self), wicbitmapsource.into().abi(), ::core::mem::transmute(bitmapproperties.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSharedBitmap(&self, riid: *const ::windows::core::GUID, data: *mut ::core::ffi::c_void, bitmapproperties: ::core::option::Option<*const D2D1_BITMAP_PROPERTIES>, bitmap: *mut ::core::option::Option<ID2D1Bitmap>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CreateSharedBitmap)(::windows::core::Vtable::as_raw(self), riid, data, ::core::mem::transmute(bitmapproperties.unwrap_or(::std::ptr::null())), ::core::mem::transmute(bitmap)).ok()
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn CreateBitmapBrush<P0>(&self, bitmap: P0, bitmapbrushproperties: ::core::option::Option<*const D2D1_BITMAP_BRUSH_PROPERTIES>, brushproperties: ::core::option::Option<*const D2D1_BRUSH_PROPERTIES>) -> ::windows::core::Result<ID2D1BitmapBrush>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Bitmap>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateBitmapBrush)(::windows::core::Vtable::as_raw(self), bitmap.into().abi(), ::core::mem::transmute(bitmapbrushproperties.unwrap_or(::std::ptr::null())), ::core::mem::transmute(brushproperties.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn CreateSolidColorBrush(&self, color: *const Common::D2D1_COLOR_F, brushproperties: ::core::option::Option<*const D2D1_BRUSH_PROPERTIES>) -> ::windows::core::Result<ID2D1SolidColorBrush> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateSolidColorBrush)(::windows::core::Vtable::as_raw(self), color, ::core::mem::transmute(brushproperties.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CreateGradientStopCollection(&self, gradientstops: &[D2D1_GRADIENT_STOP], colorinterpolationgamma: D2D1_GAMMA, extendmode: D2D1_EXTEND_MODE) -> ::windows::core::Result<ID2D1GradientStopCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateGradientStopCollection)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(gradientstops.as_ptr()), gradientstops.len() as _, colorinterpolationgamma, extendmode, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn CreateLinearGradientBrush<P0>(&self, lineargradientbrushproperties: *const D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES, brushproperties: ::core::option::Option<*const D2D1_BRUSH_PROPERTIES>, gradientstopcollection: P0) -> ::windows::core::Result<ID2D1LinearGradientBrush>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1GradientStopCollection>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateLinearGradientBrush)(::windows::core::Vtable::as_raw(self), lineargradientbrushproperties, ::core::mem::transmute(brushproperties.unwrap_or(::std::ptr::null())), gradientstopcollection.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn CreateRadialGradientBrush<P0>(&self, radialgradientbrushproperties: *const D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES, brushproperties: ::core::option::Option<*const D2D1_BRUSH_PROPERTIES>, gradientstopcollection: P0) -> ::windows::core::Result<ID2D1RadialGradientBrush>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1GradientStopCollection>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateRadialGradientBrush)(::windows::core::Vtable::as_raw(self), radialgradientbrushproperties, ::core::mem::transmute(brushproperties.unwrap_or(::std::ptr::null())), gradientstopcollection.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateCompatibleRenderTarget(&self, desiredsize: ::core::option::Option<*const Common::D2D_SIZE_F>, desiredpixelsize: ::core::option::Option<*const Common::D2D_SIZE_U>, desiredformat: ::core::option::Option<*const Common::D2D1_PIXEL_FORMAT>, options: D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS) -> ::windows::core::Result<ID2D1BitmapRenderTarget> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateCompatibleRenderTarget)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(desiredsize.unwrap_or(::std::ptr::null())), ::core::mem::transmute(desiredpixelsize.unwrap_or(::std::ptr::null())), ::core::mem::transmute(desiredformat.unwrap_or(::std::ptr::null())), options, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn CreateLayer(&self, size: ::core::option::Option<*const Common::D2D_SIZE_F>) -> ::windows::core::Result<ID2D1Layer> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateLayer)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(size.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateMesh(&self) -> ::windows::core::Result<ID2D1Mesh> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateMesh)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawLine<P0, P1>(&self, point0: Common::D2D_POINT_2F, point1: Common::D2D_POINT_2F, brush: P0, strokewidth: f32, strokestyle: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DrawLine)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(point0), ::core::mem::transmute(point1), brush.into().abi(), strokewidth, strokestyle.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawRectangle<P0, P1>(&self, rect: *const Common::D2D_RECT_F, brush: P0, strokewidth: f32, strokestyle: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DrawRectangle)(::windows::core::Vtable::as_raw(self), rect, brush.into().abi(), strokewidth, strokestyle.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn FillRectangle<P0>(&self, rect: *const Common::D2D_RECT_F, brush: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.FillRectangle)(::windows::core::Vtable::as_raw(self), rect, brush.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawRoundedRectangle<P0, P1>(&self, roundedrect: *const D2D1_ROUNDED_RECT, brush: P0, strokewidth: f32, strokestyle: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DrawRoundedRectangle)(::windows::core::Vtable::as_raw(self), roundedrect, brush.into().abi(), strokewidth, strokestyle.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn FillRoundedRectangle<P0>(&self, roundedrect: *const D2D1_ROUNDED_RECT, brush: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.FillRoundedRectangle)(::windows::core::Vtable::as_raw(self), roundedrect, brush.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawEllipse<P0, P1>(&self, ellipse: *const D2D1_ELLIPSE, brush: P0, strokewidth: f32, strokestyle: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DrawEllipse)(::windows::core::Vtable::as_raw(self), ellipse, brush.into().abi(), strokewidth, strokestyle.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn FillEllipse<P0>(&self, ellipse: *const D2D1_ELLIPSE, brush: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.FillEllipse)(::windows::core::Vtable::as_raw(self), ellipse, brush.into().abi())
    }
    pub unsafe fn DrawGeometry<P0, P1, P2>(&self, geometry: P0, brush: P1, strokewidth: f32, strokestyle: P2)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Geometry>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P2: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DrawGeometry)(::windows::core::Vtable::as_raw(self), geometry.into().abi(), brush.into().abi(), strokewidth, strokestyle.into().abi())
    }
    pub unsafe fn FillGeometry<P0, P1, P2>(&self, geometry: P0, brush: P1, opacitybrush: P2)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Geometry>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
        P2: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.FillGeometry)(::windows::core::Vtable::as_raw(self), geometry.into().abi(), brush.into().abi(), opacitybrush.into().abi())
    }
    pub unsafe fn FillMesh<P0, P1>(&self, mesh: P0, brush: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Mesh>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.FillMesh)(::windows::core::Vtable::as_raw(self), mesh.into().abi(), brush.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn FillOpacityMask<P0, P1>(&self, opacitymask: P0, brush: P1, content: D2D1_OPACITY_MASK_CONTENT, destinationrectangle: ::core::option::Option<*const Common::D2D_RECT_F>, sourcerectangle: ::core::option::Option<*const Common::D2D_RECT_F>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Bitmap>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.FillOpacityMask)(::windows::core::Vtable::as_raw(self), opacitymask.into().abi(), brush.into().abi(), content, ::core::mem::transmute(destinationrectangle.unwrap_or(::std::ptr::null())), ::core::mem::transmute(sourcerectangle.unwrap_or(::std::ptr::null())))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawBitmap<P0>(&self, bitmap: P0, destinationrectangle: ::core::option::Option<*const Common::D2D_RECT_F>, opacity: f32, interpolationmode: D2D1_BITMAP_INTERPOLATION_MODE, sourcerectangle: ::core::option::Option<*const Common::D2D_RECT_F>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Bitmap>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DrawBitmap)(::windows::core::Vtable::as_raw(self), bitmap.into().abi(), ::core::mem::transmute(destinationrectangle.unwrap_or(::std::ptr::null())), opacity, interpolationmode, ::core::mem::transmute(sourcerectangle.unwrap_or(::std::ptr::null())))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn DrawText<P0, P1>(&self, string: &[u16], textformat: P0, layoutrect: *const Common::D2D_RECT_F, defaultfillbrush: P1, options: D2D1_DRAW_TEXT_OPTIONS, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::DirectWrite::IDWriteTextFormat>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DrawText)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(string.as_ptr()), string.len() as _, textformat.into().abi(), layoutrect, defaultfillbrush.into().abi(), options, measuringmode)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn DrawTextLayout<P0, P1>(&self, origin: Common::D2D_POINT_2F, textlayout: P0, defaultfillbrush: P1, options: D2D1_DRAW_TEXT_OPTIONS)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::DirectWrite::IDWriteTextLayout>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DrawTextLayout)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(origin), textlayout.into().abi(), defaultfillbrush.into().abi(), options)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn DrawGlyphRun<P0>(&self, baselineorigin: Common::D2D_POINT_2F, glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN, foregroundbrush: P0, measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Brush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DrawGlyphRun)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(baselineorigin), glyphrun, foregroundbrush.into().abi(), measuringmode)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn SetTransform(&self, transform: *const super::super::super::Foundation::Numerics::Matrix3x2) {
        (::windows::core::Vtable::vtable(self).base__.SetTransform)(::windows::core::Vtable::as_raw(self), transform)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn GetTransform(&self, transform: *mut super::super::super::Foundation::Numerics::Matrix3x2) {
        (::windows::core::Vtable::vtable(self).base__.GetTransform)(::windows::core::Vtable::as_raw(self), transform)
    }
    pub unsafe fn SetAntialiasMode(&self, antialiasmode: D2D1_ANTIALIAS_MODE) {
        (::windows::core::Vtable::vtable(self).base__.SetAntialiasMode)(::windows::core::Vtable::as_raw(self), antialiasmode)
    }
    pub unsafe fn GetAntialiasMode(&self) -> D2D1_ANTIALIAS_MODE {
        (::windows::core::Vtable::vtable(self).base__.GetAntialiasMode)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn SetTextAntialiasMode(&self, textantialiasmode: D2D1_TEXT_ANTIALIAS_MODE) {
        (::windows::core::Vtable::vtable(self).base__.SetTextAntialiasMode)(::windows::core::Vtable::as_raw(self), textantialiasmode)
    }
    pub unsafe fn GetTextAntialiasMode(&self) -> D2D1_TEXT_ANTIALIAS_MODE {
        (::windows::core::Vtable::vtable(self).base__.GetTextAntialiasMode)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(feature = "Win32_Graphics_DirectWrite")]
    pub unsafe fn SetTextRenderingParams<P0>(&self, textrenderingparams: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::DirectWrite::IDWriteRenderingParams>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetTextRenderingParams)(::windows::core::Vtable::as_raw(self), textrenderingparams.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectWrite\"`*"]
    #[cfg(feature = "Win32_Graphics_DirectWrite")]
    pub unsafe fn GetTextRenderingParams(&self) -> ::windows::core::Result<super::DirectWrite::IDWriteRenderingParams> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetTextRenderingParams)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <super::DirectWrite::IDWriteRenderingParams as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn SetTags(&self, tag1: u64, tag2: u64) {
        (::windows::core::Vtable::vtable(self).base__.SetTags)(::windows::core::Vtable::as_raw(self), tag1, tag2)
    }
    pub unsafe fn GetTags(&self, tag1: ::core::option::Option<*mut u64>, tag2: ::core::option::Option<*mut u64>) {
        (::windows::core::Vtable::vtable(self).base__.GetTags)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(tag1.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(tag2.unwrap_or(::std::ptr::null_mut())))
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn PushLayer<P0>(&self, layerparameters: *const D2D1_LAYER_PARAMETERS, layer: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Layer>>,
    {
        (::windows::core::Vtable::vtable(self).base__.PushLayer)(::windows::core::Vtable::as_raw(self), layerparameters, layer.into().abi())
    }
    pub unsafe fn PopLayer(&self) {
        (::windows::core::Vtable::vtable(self).base__.PopLayer)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn Flush(&self, tag1: ::core::option::Option<*mut u64>, tag2: ::core::option::Option<*mut u64>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Flush)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(tag1.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(tag2.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SaveDrawingState<P0>(&self, drawingstateblock: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1DrawingStateBlock>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SaveDrawingState)(::windows::core::Vtable::as_raw(self), drawingstateblock.into().abi())
    }
    pub unsafe fn RestoreDrawingState<P0>(&self, drawingstateblock: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1DrawingStateBlock>>,
    {
        (::windows::core::Vtable::vtable(self).base__.RestoreDrawingState)(::windows::core::Vtable::as_raw(self), drawingstateblock.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn PushAxisAlignedClip(&self, cliprect: *const Common::D2D_RECT_F, antialiasmode: D2D1_ANTIALIAS_MODE) {
        (::windows::core::Vtable::vtable(self).base__.PushAxisAlignedClip)(::windows::core::Vtable::as_raw(self), cliprect, antialiasmode)
    }
    pub unsafe fn PopAxisAlignedClip(&self) {
        (::windows::core::Vtable::vtable(self).base__.PopAxisAlignedClip)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn Clear(&self, clearcolor: ::core::option::Option<*const Common::D2D1_COLOR_F>) {
        (::windows::core::Vtable::vtable(self).base__.Clear)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(clearcolor.unwrap_or(::std::ptr::null())))
    }
    pub unsafe fn BeginDraw(&self) {
        (::windows::core::Vtable::vtable(self).base__.BeginDraw)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn EndDraw(&self, tag1: ::core::option::Option<*mut u64>, tag2: ::core::option::Option<*mut u64>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.EndDraw)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(tag1.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(tag2.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetPixelFormat(&self) -> Common::D2D1_PIXEL_FORMAT {
        let mut result__: Common::D2D1_PIXEL_FORMAT = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetPixelFormat)(::windows::core::Vtable::as_raw(self), &mut result__);
        result__
    }
    pub unsafe fn SetDpi(&self, dpix: f32, dpiy: f32) {
        (::windows::core::Vtable::vtable(self).base__.SetDpi)(::windows::core::Vtable::as_raw(self), dpix, dpiy)
    }
    pub unsafe fn GetDpi(&self, dpix: *mut f32, dpiy: *mut f32) {
        (::windows::core::Vtable::vtable(self).base__.GetDpi)(::windows::core::Vtable::as_raw(self), dpix, dpiy)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetSize(&self) -> Common::D2D_SIZE_F {
        let mut result__: Common::D2D_SIZE_F = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetSize)(::windows::core::Vtable::as_raw(self), &mut result__);
        result__
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetPixelSize(&self) -> Common::D2D_SIZE_U {
        let mut result__: Common::D2D_SIZE_U = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetPixelSize)(::windows::core::Vtable::as_raw(self), &mut result__);
        result__
    }
    pub unsafe fn GetMaximumBitmapSize(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.GetMaximumBitmapSize)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn IsSupported(&self, rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.IsSupported)(::windows::core::Vtable::as_raw(self), rendertargetproperties)
    }
}
impl ::core::cmp::PartialEq for ID2D1Image {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1Image {}
impl ::core::fmt::Debug for ID2D1Image {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1Image").field(&self.0).finish()
    }
}
impl ID2D1Image {
    pub unsafe fn GetFactory(&self) -> ::windows::core::Result<ID2D1Factory> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFactory)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1Factory as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
}
impl ::core::cmp::PartialEq for ID2D1ImageBrush {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1ImageBrush {}
impl ::core::fmt::Debug for ID2D1ImageBrush {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1ImageBrush").field(&self.0).finish()
    }
}
impl ID2D1ImageBrush {
    pub unsafe fn GetFactory(&self) -> ::windows::core::Result<ID2D1Factory> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetFactory)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1Factory as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn SetOpacity(&self, opacity: f32) {
        (::windows::core::Vtable::vtable(self).base__.SetOpacity)(::windows::core::Vtable::as_raw(self), opacity)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn SetTransform(&self, transform: *const super::super::super::Foundation::Numerics::Matrix3x2) {
        (::windows::core::Vtable::vtable(self).base__.SetTransform)(::windows::core::Vtable::as_raw(self), transform)
    }
    pub unsafe fn GetOpacity(&self) -> f32 {
        (::windows::core::Vtable::vtable(self).base__.GetOpacity)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn GetTransform(&self, transform: *mut super::super::super::Foundation::Numerics::Matrix3x2) {
        (::windows::core::Vtable::vtable(self).base__.GetTransform)(::windows::core::Vtable::as_raw(self), transform)
    }
}
impl ::core::cmp::PartialEq for ID2D1ImageSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1ImageSource {}
impl ::core::fmt::Debug for ID2D1ImageSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1ImageSource").field(&self.0).finish()
    }
}
impl ID2D1ImageSource {
    pub unsafe fn GetFactory(&self) -> ::windows::core::Result<ID2D1Factory> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetFactory)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1Factory as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
}
impl ::core::cmp::PartialEq for ID2D1ImageSourceFromWic {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1ImageSourceFromWic {}
impl ::core::fmt::Debug for ID2D1ImageSourceFromWic {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1ImageSourceFromWic").field(&self.0).finish()
    }
}
impl ID2D1ImageSourceFromWic {
    pub unsafe fn GetFactory(&self) -> ::windows::core::Result<ID2D1Factory> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetFactory)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1Factory as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn OfferResources(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OfferResources)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TryReclaimResources(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.TryReclaimResources)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for ID2D1Ink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1Ink {}
impl ::core::fmt::Debug for ID2D1Ink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1Ink").field(&self.0).finish()
    }
}
impl ID2D1Ink {
    pub unsafe fn GetFactory(&self) -> ::windows::core::Result<ID2D1Factory> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFactory)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1Factory as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
}
impl ::core::cmp::PartialEq for ID2D1InkStyle {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1InkStyle {}
impl ::core::fmt::Debug for ID2D1InkStyle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1InkStyle").field(&self.0).finish()
    }
}
impl ID2D1InkStyle {
    pub unsafe fn GetFactory(&self) -> ::windows::core::Result<ID2D1Factory> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFactory)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1Factory as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
}
impl ::core::cmp::PartialEq for ID2D1Layer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1Layer {}
impl ::core::fmt::Debug for ID2D1Layer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1Layer").field(&self.0).finish()
    }
}
impl ID2D1Layer {
    pub unsafe fn GetFactory(&self) -> ::windows::core::Result<ID2D1Factory> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFactory)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1Factory as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
}
impl ::core::cmp::PartialEq for ID2D1LinearGradientBrush {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1LinearGradientBrush {}
impl ::core::fmt::Debug for ID2D1LinearGradientBrush {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1LinearGradientBrush").field(&self.0).finish()
    }
}
impl ID2D1LinearGradientBrush {
    pub unsafe fn GetFactory(&self) -> ::windows::core::Result<ID2D1Factory> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetFactory)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1Factory as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn SetOpacity(&self, opacity: f32) {
        (::windows::core::Vtable::vtable(self).base__.SetOpacity)(::windows::core::Vtable::as_raw(self), opacity)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn SetTransform(&self, transform: *const super::super::super::Foundation::Numerics::Matrix3x2) {
        (::windows::core::Vtable::vtable(self).base__.SetTransform)(::windows::core::Vtable::as_raw(self), transform)
    }
    pub unsafe fn GetOpacity(&self) -> f32 {
        (::windows::core::Vtable::vtable(self).base__.GetOpacity)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn GetTransform(&self, transform: *mut super::super::super::Foundation::Numerics::Matrix3x2) {
        (::windows::core::Vtable::vtable(self).base__.GetTransform)(::windows::core::Vtable::as_raw(self), transform)
    }
}
impl ::core::cmp::PartialEq for ID2D1LookupTable3D {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1LookupTable3D {}
impl ::core::fmt::Debug for ID2D1LookupTable3D {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1LookupTable3D").field(&self.0).finish()
    }
}
impl ID2D1LookupTable3D {
    pub unsafe fn GetFactory(&self) -> ::windows::core::Result<ID2D1Factory> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFactory)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1Factory as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
}
impl ::core::cmp::PartialEq for ID2D1Mesh {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1Mesh {}
impl ::core::fmt::Debug for ID2D1Mesh {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1Mesh").field(&self.0).finish()
    }
}
impl ID2D1Mesh {
    pub unsafe fn GetFactory(&self) -> ::windows::core::Result<ID2D1Factory> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFactory)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1Factory as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
}
impl ::core::cmp::PartialEq for ID2D1Multithread {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1Multithread {}
impl ::core::fmt::Debug for ID2D1Multithread {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1Multithread").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID2D1OffsetTransform {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1OffsetTransform {}
impl ::core::fmt::Debug for ID2D1OffsetTransform {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1OffsetTransform").field(&self.0).finish()
    }
}
impl ID2D1OffsetTransform {
    pub unsafe fn GetInputCount(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.GetInputCount)(::windows::core::Vtable::as_raw(self))
    }
}
impl ::core::cmp::PartialEq for ID2D1PathGeometry {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1PathGeometry {}
impl ::core::fmt::Debug for ID2D1PathGeometry {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1PathGeometry").field(&self.0).finish()
    }
}
impl ID2D1PathGeometry {
    pub unsafe fn GetFactory(&self) -> ::windows::core::Result<ID2D1Factory> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetFactory)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1Factory as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn GetBounds(&self, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>) -> ::windows::core::Result<Common::D2D_RECT_F> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetBounds)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn GetWidenedBounds<P0>(&self, strokewidth: f32, strokestyle: P0, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32) -> ::windows::core::Result<Common::D2D_RECT_F>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetWidenedBounds)(::windows::core::Vtable::as_raw(self), strokewidth, strokestyle.into().abi(), ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), flatteningtolerance, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn StrokeContainsPoint<P0>(&self, point: Common::D2D_POINT_2F, strokewidth: f32, strokestyle: P0, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.StrokeContainsPoint)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(point), strokewidth, strokestyle.into().abi(), ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), flatteningtolerance, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn FillContainsPoint(&self, point: Common::D2D_POINT_2F, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.FillContainsPoint)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(point), ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), flatteningtolerance, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn CompareWithGeometry<P0>(&self, inputgeometry: P0, inputgeometrytransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32) -> ::windows::core::Result<D2D1_GEOMETRY_RELATION>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Geometry>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CompareWithGeometry)(::windows::core::Vtable::as_raw(self), inputgeometry.into().abi(), ::core::mem::transmute(inputgeometrytransform.unwrap_or(::std::ptr::null())), flatteningtolerance, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn Simplify<P0>(&self, simplificationoption: D2D1_GEOMETRY_SIMPLIFICATION_OPTION, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32, geometrysink: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<Common::ID2D1SimplifiedGeometrySink>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Simplify)(::windows::core::Vtable::as_raw(self), simplificationoption, ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), flatteningtolerance, geometrysink.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn Tessellate<P0>(&self, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32, tessellationsink: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1TessellationSink>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Tessellate)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), flatteningtolerance, tessellationsink.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn CombineWithGeometry<P0, P1>(&self, inputgeometry: P0, combinemode: D2D1_COMBINE_MODE, inputgeometrytransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32, geometrysink: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Geometry>>,
        P1: ::std::convert::Into<::windows::core::InParam<Common::ID2D1SimplifiedGeometrySink>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CombineWithGeometry)(::windows::core::Vtable::as_raw(self), inputgeometry.into().abi(), combinemode, ::core::mem::transmute(inputgeometrytransform.unwrap_or(::std::ptr::null())), flatteningtolerance, geometrysink.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn Outline<P0>(&self, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32, geometrysink: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<Common::ID2D1SimplifiedGeometrySink>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Outline)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), flatteningtolerance, geometrysink.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn ComputeArea(&self, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ComputeArea)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), flatteningtolerance, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn ComputeLength(&self, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ComputeLength)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), flatteningtolerance, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn ComputePointAtLength(&self, length: f32, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32, point: ::core::option::Option<*mut Common::D2D_POINT_2F>, unittangentvector: ::core::option::Option<*mut Common::D2D_POINT_2F>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ComputePointAtLength)(::windows::core::Vtable::as_raw(self), length, ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), flatteningtolerance, ::core::mem::transmute(point.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(unittangentvector.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn Widen<P0, P1>(&self, strokewidth: f32, strokestyle: P0, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32, geometrysink: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
        P1: ::std::convert::Into<::windows::core::InParam<Common::ID2D1SimplifiedGeometrySink>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Widen)(::windows::core::Vtable::as_raw(self), strokewidth, strokestyle.into().abi(), ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), flatteningtolerance, geometrysink.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for ID2D1PathGeometry1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1PathGeometry1 {}
impl ::core::fmt::Debug for ID2D1PathGeometry1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1PathGeometry1").field(&self.0).finish()
    }
}
impl ID2D1PathGeometry1 {
    pub unsafe fn GetFactory(&self) -> ::windows::core::Result<ID2D1Factory> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetFactory)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1Factory as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn GetBounds(&self, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>) -> ::windows::core::Result<Common::D2D_RECT_F> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetBounds)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn GetWidenedBounds<P0>(&self, strokewidth: f32, strokestyle: P0, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32) -> ::windows::core::Result<Common::D2D_RECT_F>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetWidenedBounds)(::windows::core::Vtable::as_raw(self), strokewidth, strokestyle.into().abi(), ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), flatteningtolerance, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn StrokeContainsPoint<P0>(&self, point: Common::D2D_POINT_2F, strokewidth: f32, strokestyle: P0, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.StrokeContainsPoint)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(point), strokewidth, strokestyle.into().abi(), ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), flatteningtolerance, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn FillContainsPoint(&self, point: Common::D2D_POINT_2F, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.FillContainsPoint)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(point), ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), flatteningtolerance, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn CompareWithGeometry<P0>(&self, inputgeometry: P0, inputgeometrytransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32) -> ::windows::core::Result<D2D1_GEOMETRY_RELATION>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Geometry>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CompareWithGeometry)(::windows::core::Vtable::as_raw(self), inputgeometry.into().abi(), ::core::mem::transmute(inputgeometrytransform.unwrap_or(::std::ptr::null())), flatteningtolerance, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn Simplify<P0>(&self, simplificationoption: D2D1_GEOMETRY_SIMPLIFICATION_OPTION, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32, geometrysink: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<Common::ID2D1SimplifiedGeometrySink>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.Simplify)(::windows::core::Vtable::as_raw(self), simplificationoption, ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), flatteningtolerance, geometrysink.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn Tessellate<P0>(&self, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32, tessellationsink: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1TessellationSink>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.Tessellate)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), flatteningtolerance, tessellationsink.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn CombineWithGeometry<P0, P1>(&self, inputgeometry: P0, combinemode: D2D1_COMBINE_MODE, inputgeometrytransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32, geometrysink: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Geometry>>,
        P1: ::std::convert::Into<::windows::core::InParam<Common::ID2D1SimplifiedGeometrySink>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.CombineWithGeometry)(::windows::core::Vtable::as_raw(self), inputgeometry.into().abi(), combinemode, ::core::mem::transmute(inputgeometrytransform.unwrap_or(::std::ptr::null())), flatteningtolerance, geometrysink.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn Outline<P0>(&self, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32, geometrysink: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<Common::ID2D1SimplifiedGeometrySink>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.Outline)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), flatteningtolerance, geometrysink.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn ComputeArea(&self, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.ComputeArea)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), flatteningtolerance, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn ComputeLength(&self, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.ComputeLength)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), flatteningtolerance, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn ComputePointAtLength(&self, length: f32, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32, point: ::core::option::Option<*mut Common::D2D_POINT_2F>, unittangentvector: ::core::option::Option<*mut Common::D2D_POINT_2F>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.ComputePointAtLength)(::windows::core::Vtable::as_raw(self), length, ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), flatteningtolerance, ::core::mem::transmute(point.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(unittangentvector.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn Widen<P0, P1>(&self, strokewidth: f32, strokestyle: P0, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32, geometrysink: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
        P1: ::std::convert::Into<::windows::core::InParam<Common::ID2D1SimplifiedGeometrySink>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.Widen)(::windows::core::Vtable::as_raw(self), strokewidth, strokestyle.into().abi(), ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), flatteningtolerance, geometrysink.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn Open(&self) -> ::windows::core::Result<ID2D1GeometrySink> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Open)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn Stream<P0>(&self, geometrysink: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1GeometrySink>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Stream)(::windows::core::Vtable::as_raw(self), geometrysink.into().abi()).ok()
    }
    pub unsafe fn GetSegmentCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetSegmentCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetFigureCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFigureCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for ID2D1PrintControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1PrintControl {}
impl ::core::fmt::Debug for ID2D1PrintControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1PrintControl").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID2D1Properties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1Properties {}
impl ::core::fmt::Debug for ID2D1Properties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1Properties").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID2D1RadialGradientBrush {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1RadialGradientBrush {}
impl ::core::fmt::Debug for ID2D1RadialGradientBrush {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1RadialGradientBrush").field(&self.0).finish()
    }
}
impl ID2D1RadialGradientBrush {
    pub unsafe fn GetFactory(&self) -> ::windows::core::Result<ID2D1Factory> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetFactory)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1Factory as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn SetOpacity(&self, opacity: f32) {
        (::windows::core::Vtable::vtable(self).base__.SetOpacity)(::windows::core::Vtable::as_raw(self), opacity)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn SetTransform(&self, transform: *const super::super::super::Foundation::Numerics::Matrix3x2) {
        (::windows::core::Vtable::vtable(self).base__.SetTransform)(::windows::core::Vtable::as_raw(self), transform)
    }
    pub unsafe fn GetOpacity(&self) -> f32 {
        (::windows::core::Vtable::vtable(self).base__.GetOpacity)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn GetTransform(&self, transform: *mut super::super::super::Foundation::Numerics::Matrix3x2) {
        (::windows::core::Vtable::vtable(self).base__.GetTransform)(::windows::core::Vtable::as_raw(self), transform)
    }
}
impl ::core::cmp::PartialEq for ID2D1RectangleGeometry {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1RectangleGeometry {}
impl ::core::fmt::Debug for ID2D1RectangleGeometry {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1RectangleGeometry").field(&self.0).finish()
    }
}
impl ID2D1RectangleGeometry {
    pub unsafe fn GetFactory(&self) -> ::windows::core::Result<ID2D1Factory> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetFactory)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1Factory as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn GetBounds(&self, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>) -> ::windows::core::Result<Common::D2D_RECT_F> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetBounds)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn GetWidenedBounds<P0>(&self, strokewidth: f32, strokestyle: P0, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32) -> ::windows::core::Result<Common::D2D_RECT_F>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetWidenedBounds)(::windows::core::Vtable::as_raw(self), strokewidth, strokestyle.into().abi(), ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), flatteningtolerance, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn StrokeContainsPoint<P0>(&self, point: Common::D2D_POINT_2F, strokewidth: f32, strokestyle: P0, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.StrokeContainsPoint)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(point), strokewidth, strokestyle.into().abi(), ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), flatteningtolerance, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn FillContainsPoint(&self, point: Common::D2D_POINT_2F, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.FillContainsPoint)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(point), ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), flatteningtolerance, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn CompareWithGeometry<P0>(&self, inputgeometry: P0, inputgeometrytransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32) -> ::windows::core::Result<D2D1_GEOMETRY_RELATION>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Geometry>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CompareWithGeometry)(::windows::core::Vtable::as_raw(self), inputgeometry.into().abi(), ::core::mem::transmute(inputgeometrytransform.unwrap_or(::std::ptr::null())), flatteningtolerance, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn Simplify<P0>(&self, simplificationoption: D2D1_GEOMETRY_SIMPLIFICATION_OPTION, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32, geometrysink: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<Common::ID2D1SimplifiedGeometrySink>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Simplify)(::windows::core::Vtable::as_raw(self), simplificationoption, ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), flatteningtolerance, geometrysink.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn Tessellate<P0>(&self, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32, tessellationsink: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1TessellationSink>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Tessellate)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), flatteningtolerance, tessellationsink.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn CombineWithGeometry<P0, P1>(&self, inputgeometry: P0, combinemode: D2D1_COMBINE_MODE, inputgeometrytransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32, geometrysink: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Geometry>>,
        P1: ::std::convert::Into<::windows::core::InParam<Common::ID2D1SimplifiedGeometrySink>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CombineWithGeometry)(::windows::core::Vtable::as_raw(self), inputgeometry.into().abi(), combinemode, ::core::mem::transmute(inputgeometrytransform.unwrap_or(::std::ptr::null())), flatteningtolerance, geometrysink.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn Outline<P0>(&self, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32, geometrysink: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<Common::ID2D1SimplifiedGeometrySink>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Outline)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), flatteningtolerance, geometrysink.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn ComputeArea(&self, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ComputeArea)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), flatteningtolerance, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn ComputeLength(&self, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ComputeLength)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), flatteningtolerance, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn ComputePointAtLength(&self, length: f32, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32, point: ::core::option::Option<*mut Common::D2D_POINT_2F>, unittangentvector: ::core::option::Option<*mut Common::D2D_POINT_2F>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ComputePointAtLength)(::windows::core::Vtable::as_raw(self), length, ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), flatteningtolerance, ::core::mem::transmute(point.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(unittangentvector.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn Widen<P0, P1>(&self, strokewidth: f32, strokestyle: P0, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32, geometrysink: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
        P1: ::std::convert::Into<::windows::core::InParam<Common::ID2D1SimplifiedGeometrySink>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Widen)(::windows::core::Vtable::as_raw(self), strokewidth, strokestyle.into().abi(), ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), flatteningtolerance, geometrysink.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for ID2D1RenderInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1RenderInfo {}
impl ::core::fmt::Debug for ID2D1RenderInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1RenderInfo").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID2D1RenderTarget {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1RenderTarget {}
impl ::core::fmt::Debug for ID2D1RenderTarget {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1RenderTarget").field(&self.0).finish()
    }
}
impl ID2D1RenderTarget {
    pub unsafe fn GetFactory(&self) -> ::windows::core::Result<ID2D1Factory> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFactory)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1Factory as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
}
impl ::core::cmp::PartialEq for ID2D1Resource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1Resource {}
impl ::core::fmt::Debug for ID2D1Resource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1Resource").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID2D1ResourceTexture {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1ResourceTexture {}
impl ::core::fmt::Debug for ID2D1ResourceTexture {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1ResourceTexture").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID2D1RoundedRectangleGeometry {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1RoundedRectangleGeometry {}
impl ::core::fmt::Debug for ID2D1RoundedRectangleGeometry {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1RoundedRectangleGeometry").field(&self.0).finish()
    }
}
impl ID2D1RoundedRectangleGeometry {
    pub unsafe fn GetFactory(&self) -> ::windows::core::Result<ID2D1Factory> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetFactory)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1Factory as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn GetBounds(&self, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>) -> ::windows::core::Result<Common::D2D_RECT_F> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetBounds)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn GetWidenedBounds<P0>(&self, strokewidth: f32, strokestyle: P0, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32) -> ::windows::core::Result<Common::D2D_RECT_F>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetWidenedBounds)(::windows::core::Vtable::as_raw(self), strokewidth, strokestyle.into().abi(), ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), flatteningtolerance, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn StrokeContainsPoint<P0>(&self, point: Common::D2D_POINT_2F, strokewidth: f32, strokestyle: P0, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.StrokeContainsPoint)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(point), strokewidth, strokestyle.into().abi(), ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), flatteningtolerance, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn FillContainsPoint(&self, point: Common::D2D_POINT_2F, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.FillContainsPoint)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(point), ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), flatteningtolerance, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn CompareWithGeometry<P0>(&self, inputgeometry: P0, inputgeometrytransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32) -> ::windows::core::Result<D2D1_GEOMETRY_RELATION>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Geometry>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CompareWithGeometry)(::windows::core::Vtable::as_raw(self), inputgeometry.into().abi(), ::core::mem::transmute(inputgeometrytransform.unwrap_or(::std::ptr::null())), flatteningtolerance, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn Simplify<P0>(&self, simplificationoption: D2D1_GEOMETRY_SIMPLIFICATION_OPTION, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32, geometrysink: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<Common::ID2D1SimplifiedGeometrySink>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Simplify)(::windows::core::Vtable::as_raw(self), simplificationoption, ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), flatteningtolerance, geometrysink.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn Tessellate<P0>(&self, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32, tessellationsink: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1TessellationSink>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Tessellate)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), flatteningtolerance, tessellationsink.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn CombineWithGeometry<P0, P1>(&self, inputgeometry: P0, combinemode: D2D1_COMBINE_MODE, inputgeometrytransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32, geometrysink: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Geometry>>,
        P1: ::std::convert::Into<::windows::core::InParam<Common::ID2D1SimplifiedGeometrySink>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CombineWithGeometry)(::windows::core::Vtable::as_raw(self), inputgeometry.into().abi(), combinemode, ::core::mem::transmute(inputgeometrytransform.unwrap_or(::std::ptr::null())), flatteningtolerance, geometrysink.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn Outline<P0>(&self, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32, geometrysink: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<Common::ID2D1SimplifiedGeometrySink>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Outline)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), flatteningtolerance, geometrysink.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn ComputeArea(&self, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ComputeArea)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), flatteningtolerance, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn ComputeLength(&self, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ComputeLength)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), flatteningtolerance, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn ComputePointAtLength(&self, length: f32, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32, point: ::core::option::Option<*mut Common::D2D_POINT_2F>, unittangentvector: ::core::option::Option<*mut Common::D2D_POINT_2F>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ComputePointAtLength)(::windows::core::Vtable::as_raw(self), length, ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), flatteningtolerance, ::core::mem::transmute(point.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(unittangentvector.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn Widen<P0, P1>(&self, strokewidth: f32, strokestyle: P0, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32, geometrysink: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
        P1: ::std::convert::Into<::windows::core::InParam<Common::ID2D1SimplifiedGeometrySink>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Widen)(::windows::core::Vtable::as_raw(self), strokewidth, strokestyle.into().abi(), ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), flatteningtolerance, geometrysink.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for ID2D1SolidColorBrush {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1SolidColorBrush {}
impl ::core::fmt::Debug for ID2D1SolidColorBrush {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1SolidColorBrush").field(&self.0).finish()
    }
}
impl ID2D1SolidColorBrush {
    pub unsafe fn GetFactory(&self) -> ::windows::core::Result<ID2D1Factory> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetFactory)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1Factory as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn SetOpacity(&self, opacity: f32) {
        (::windows::core::Vtable::vtable(self).base__.SetOpacity)(::windows::core::Vtable::as_raw(self), opacity)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn SetTransform(&self, transform: *const super::super::super::Foundation::Numerics::Matrix3x2) {
        (::windows::core::Vtable::vtable(self).base__.SetTransform)(::windows::core::Vtable::as_raw(self), transform)
    }
    pub unsafe fn GetOpacity(&self) -> f32 {
        (::windows::core::Vtable::vtable(self).base__.GetOpacity)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn GetTransform(&self, transform: *mut super::super::super::Foundation::Numerics::Matrix3x2) {
        (::windows::core::Vtable::vtable(self).base__.GetTransform)(::windows::core::Vtable::as_raw(self), transform)
    }
}
impl ::core::cmp::PartialEq for ID2D1SourceTransform {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1SourceTransform {}
impl ::core::fmt::Debug for ID2D1SourceTransform {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1SourceTransform").field(&self.0).finish()
    }
}
impl ID2D1SourceTransform {
    pub unsafe fn GetInputCount(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.base__.GetInputCount)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MapOutputRectToInputRects(&self, outputrect: *const super::super::Foundation::RECT, inputrects: &mut [super::super::Foundation::RECT]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.MapOutputRectToInputRects)(::windows::core::Vtable::as_raw(self), outputrect, ::core::mem::transmute(inputrects.as_ptr()), inputrects.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MapInputRectsToOutputRect(&self, inputrects: *const super::super::Foundation::RECT, inputopaquesubrects: *const super::super::Foundation::RECT, inputrectcount: u32, outputrect: *mut super::super::Foundation::RECT, outputopaquesubrect: *mut super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.MapInputRectsToOutputRect)(::windows::core::Vtable::as_raw(self), inputrects, inputopaquesubrects, inputrectcount, outputrect, outputopaquesubrect).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MapInvalidRect(&self, inputindex: u32, invalidinputrect: super::super::Foundation::RECT) -> ::windows::core::Result<super::super::Foundation::RECT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.MapInvalidRect)(::windows::core::Vtable::as_raw(self), inputindex, ::core::mem::transmute(invalidinputrect), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for ID2D1SpriteBatch {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1SpriteBatch {}
impl ::core::fmt::Debug for ID2D1SpriteBatch {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1SpriteBatch").field(&self.0).finish()
    }
}
impl ID2D1SpriteBatch {
    pub unsafe fn GetFactory(&self) -> ::windows::core::Result<ID2D1Factory> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFactory)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1Factory as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
}
impl ::core::cmp::PartialEq for ID2D1StrokeStyle {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1StrokeStyle {}
impl ::core::fmt::Debug for ID2D1StrokeStyle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1StrokeStyle").field(&self.0).finish()
    }
}
impl ID2D1StrokeStyle {
    pub unsafe fn GetFactory(&self) -> ::windows::core::Result<ID2D1Factory> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFactory)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1Factory as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
}
impl ::core::cmp::PartialEq for ID2D1StrokeStyle1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1StrokeStyle1 {}
impl ::core::fmt::Debug for ID2D1StrokeStyle1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1StrokeStyle1").field(&self.0).finish()
    }
}
impl ID2D1StrokeStyle1 {
    pub unsafe fn GetFactory(&self) -> ::windows::core::Result<ID2D1Factory> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetFactory)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1Factory as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn GetStartCap(&self) -> D2D1_CAP_STYLE {
        (::windows::core::Vtable::vtable(self).base__.GetStartCap)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetEndCap(&self) -> D2D1_CAP_STYLE {
        (::windows::core::Vtable::vtable(self).base__.GetEndCap)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetDashCap(&self) -> D2D1_CAP_STYLE {
        (::windows::core::Vtable::vtable(self).base__.GetDashCap)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetMiterLimit(&self) -> f32 {
        (::windows::core::Vtable::vtable(self).base__.GetMiterLimit)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetLineJoin(&self) -> D2D1_LINE_JOIN {
        (::windows::core::Vtable::vtable(self).base__.GetLineJoin)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetDashOffset(&self) -> f32 {
        (::windows::core::Vtable::vtable(self).base__.GetDashOffset)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetDashStyle(&self) -> D2D1_DASH_STYLE {
        (::windows::core::Vtable::vtable(self).base__.GetDashStyle)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetDashesCount(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.GetDashesCount)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetDashes(&self, dashes: &mut [f32]) {
        (::windows::core::Vtable::vtable(self).base__.GetDashes)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(dashes.as_ptr()), dashes.len() as _)
    }
}
impl ::core::cmp::PartialEq for ID2D1SvgAttribute {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1SvgAttribute {}
impl ::core::fmt::Debug for ID2D1SvgAttribute {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1SvgAttribute").field(&self.0).finish()
    }
}
impl ID2D1SvgAttribute {
    pub unsafe fn GetFactory(&self) -> ::windows::core::Result<ID2D1Factory> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFactory)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1Factory as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
}
impl ::core::cmp::PartialEq for ID2D1SvgDocument {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1SvgDocument {}
impl ::core::fmt::Debug for ID2D1SvgDocument {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1SvgDocument").field(&self.0).finish()
    }
}
impl ID2D1SvgDocument {
    pub unsafe fn GetFactory(&self) -> ::windows::core::Result<ID2D1Factory> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFactory)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1Factory as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
}
impl ::core::cmp::PartialEq for ID2D1SvgElement {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1SvgElement {}
impl ::core::fmt::Debug for ID2D1SvgElement {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1SvgElement").field(&self.0).finish()
    }
}
impl ID2D1SvgElement {
    pub unsafe fn GetFactory(&self) -> ::windows::core::Result<ID2D1Factory> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFactory)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1Factory as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
}
impl ::core::cmp::PartialEq for ID2D1SvgGlyphStyle {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1SvgGlyphStyle {}
impl ::core::fmt::Debug for ID2D1SvgGlyphStyle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1SvgGlyphStyle").field(&self.0).finish()
    }
}
impl ID2D1SvgGlyphStyle {
    pub unsafe fn GetFactory(&self) -> ::windows::core::Result<ID2D1Factory> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFactory)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1Factory as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
}
impl ::core::cmp::PartialEq for ID2D1SvgPaint {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1SvgPaint {}
impl ::core::fmt::Debug for ID2D1SvgPaint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1SvgPaint").field(&self.0).finish()
    }
}
impl ID2D1SvgPaint {
    pub unsafe fn GetFactory(&self) -> ::windows::core::Result<ID2D1Factory> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetFactory)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1Factory as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn GetElement(&self) -> ::windows::core::Result<ID2D1SvgElement> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetElement)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1SvgElement as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<ID2D1SvgAttribute> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Clone)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for ID2D1SvgPathData {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1SvgPathData {}
impl ::core::fmt::Debug for ID2D1SvgPathData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1SvgPathData").field(&self.0).finish()
    }
}
impl ID2D1SvgPathData {
    pub unsafe fn GetFactory(&self) -> ::windows::core::Result<ID2D1Factory> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetFactory)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1Factory as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn GetElement(&self) -> ::windows::core::Result<ID2D1SvgElement> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetElement)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1SvgElement as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<ID2D1SvgAttribute> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Clone)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for ID2D1SvgPointCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1SvgPointCollection {}
impl ::core::fmt::Debug for ID2D1SvgPointCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1SvgPointCollection").field(&self.0).finish()
    }
}
impl ID2D1SvgPointCollection {
    pub unsafe fn GetFactory(&self) -> ::windows::core::Result<ID2D1Factory> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetFactory)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1Factory as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn GetElement(&self) -> ::windows::core::Result<ID2D1SvgElement> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetElement)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1SvgElement as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<ID2D1SvgAttribute> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Clone)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for ID2D1SvgStrokeDashArray {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1SvgStrokeDashArray {}
impl ::core::fmt::Debug for ID2D1SvgStrokeDashArray {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1SvgStrokeDashArray").field(&self.0).finish()
    }
}
impl ID2D1SvgStrokeDashArray {
    pub unsafe fn GetFactory(&self) -> ::windows::core::Result<ID2D1Factory> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetFactory)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1Factory as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn GetElement(&self) -> ::windows::core::Result<ID2D1SvgElement> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetElement)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1SvgElement as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<ID2D1SvgAttribute> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Clone)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for ID2D1TessellationSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1TessellationSink {}
impl ::core::fmt::Debug for ID2D1TessellationSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1TessellationSink").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID2D1Transform {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1Transform {}
impl ::core::fmt::Debug for ID2D1Transform {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1Transform").field(&self.0).finish()
    }
}
impl ID2D1Transform {
    pub unsafe fn GetInputCount(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.GetInputCount)(::windows::core::Vtable::as_raw(self))
    }
}
impl ::core::cmp::PartialEq for ID2D1TransformGraph {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1TransformGraph {}
impl ::core::fmt::Debug for ID2D1TransformGraph {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1TransformGraph").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID2D1TransformNode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1TransformNode {}
impl ::core::fmt::Debug for ID2D1TransformNode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1TransformNode").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID2D1TransformedGeometry {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1TransformedGeometry {}
impl ::core::fmt::Debug for ID2D1TransformedGeometry {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1TransformedGeometry").field(&self.0).finish()
    }
}
impl ID2D1TransformedGeometry {
    pub unsafe fn GetFactory(&self) -> ::windows::core::Result<ID2D1Factory> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetFactory)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1Factory as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn GetBounds(&self, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>) -> ::windows::core::Result<Common::D2D_RECT_F> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetBounds)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn GetWidenedBounds<P0>(&self, strokewidth: f32, strokestyle: P0, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32) -> ::windows::core::Result<Common::D2D_RECT_F>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetWidenedBounds)(::windows::core::Vtable::as_raw(self), strokewidth, strokestyle.into().abi(), ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), flatteningtolerance, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn StrokeContainsPoint<P0>(&self, point: Common::D2D_POINT_2F, strokewidth: f32, strokestyle: P0, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.StrokeContainsPoint)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(point), strokewidth, strokestyle.into().abi(), ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), flatteningtolerance, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn FillContainsPoint(&self, point: Common::D2D_POINT_2F, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.FillContainsPoint)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(point), ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), flatteningtolerance, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn CompareWithGeometry<P0>(&self, inputgeometry: P0, inputgeometrytransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32) -> ::windows::core::Result<D2D1_GEOMETRY_RELATION>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Geometry>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CompareWithGeometry)(::windows::core::Vtable::as_raw(self), inputgeometry.into().abi(), ::core::mem::transmute(inputgeometrytransform.unwrap_or(::std::ptr::null())), flatteningtolerance, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn Simplify<P0>(&self, simplificationoption: D2D1_GEOMETRY_SIMPLIFICATION_OPTION, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32, geometrysink: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<Common::ID2D1SimplifiedGeometrySink>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Simplify)(::windows::core::Vtable::as_raw(self), simplificationoption, ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), flatteningtolerance, geometrysink.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn Tessellate<P0>(&self, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32, tessellationsink: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1TessellationSink>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Tessellate)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), flatteningtolerance, tessellationsink.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn CombineWithGeometry<P0, P1>(&self, inputgeometry: P0, combinemode: D2D1_COMBINE_MODE, inputgeometrytransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32, geometrysink: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1Geometry>>,
        P1: ::std::convert::Into<::windows::core::InParam<Common::ID2D1SimplifiedGeometrySink>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CombineWithGeometry)(::windows::core::Vtable::as_raw(self), inputgeometry.into().abi(), combinemode, ::core::mem::transmute(inputgeometrytransform.unwrap_or(::std::ptr::null())), flatteningtolerance, geometrysink.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn Outline<P0>(&self, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32, geometrysink: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<Common::ID2D1SimplifiedGeometrySink>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Outline)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), flatteningtolerance, geometrysink.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn ComputeArea(&self, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ComputeArea)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), flatteningtolerance, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn ComputeLength(&self, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ComputeLength)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), flatteningtolerance, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn ComputePointAtLength(&self, length: f32, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32, point: ::core::option::Option<*mut Common::D2D_POINT_2F>, unittangentvector: ::core::option::Option<*mut Common::D2D_POINT_2F>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ComputePointAtLength)(::windows::core::Vtable::as_raw(self), length, ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), flatteningtolerance, ::core::mem::transmute(point.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(unittangentvector.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn Widen<P0, P1>(&self, strokewidth: f32, strokestyle: P0, worldtransform: ::core::option::Option<*const super::super::super::Foundation::Numerics::Matrix3x2>, flatteningtolerance: f32, geometrysink: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID2D1StrokeStyle>>,
        P1: ::std::convert::Into<::windows::core::InParam<Common::ID2D1SimplifiedGeometrySink>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Widen)(::windows::core::Vtable::as_raw(self), strokewidth, strokestyle.into().abi(), ::core::mem::transmute(worldtransform.unwrap_or(::std::ptr::null())), flatteningtolerance, geometrysink.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for ID2D1TransformedImageSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1TransformedImageSource {}
impl ::core::fmt::Debug for ID2D1TransformedImageSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1TransformedImageSource").field(&self.0).finish()
    }
}
impl ID2D1TransformedImageSource {
    pub unsafe fn GetFactory(&self) -> ::windows::core::Result<ID2D1Factory> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetFactory)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID2D1Factory as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
}
impl ::core::cmp::PartialEq for ID2D1VertexBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1VertexBuffer {}
impl ::core::fmt::Debug for ID2D1VertexBuffer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1VertexBuffer").field(&self.0).finish()
    }
}
