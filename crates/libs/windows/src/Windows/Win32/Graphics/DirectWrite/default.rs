impl ::core::default::Default for DWRITE_AUTOMATIC_FONT_AXES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DWRITE_AUTOMATIC_FONT_AXES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_AUTOMATIC_FONT_AXES").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for DWRITE_AUTOMATIC_FONT_AXES {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DWRITE_AUTOMATIC_FONT_AXES {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DWRITE_AUTOMATIC_FONT_AXES {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DWRITE_AUTOMATIC_FONT_AXES {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DWRITE_AUTOMATIC_FONT_AXES {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for DWRITE_BASELINE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DWRITE_BASELINE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_BASELINE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DWRITE_BREAK_CONDITION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DWRITE_BREAK_CONDITION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_BREAK_CONDITION").field(&self.0).finish()
    }
}
impl ::core::default::Default for DWRITE_CARET_METRICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DWRITE_CARET_METRICS {
    fn eq(&self, other: &Self) -> bool {
        self.slopeRise == other.slopeRise && self.slopeRun == other.slopeRun && self.offset == other.offset
    }
}
impl ::core::cmp::Eq for DWRITE_CARET_METRICS {}
impl ::core::fmt::Debug for DWRITE_CARET_METRICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DWRITE_CARET_METRICS").field("slopeRise", &self.slopeRise).field("slopeRun", &self.slopeRun).field("offset", &self.offset).finish()
    }
}
impl ::core::default::Default for DWRITE_CLUSTER_METRICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DWRITE_CLUSTER_METRICS {
    fn eq(&self, other: &Self) -> bool {
        self.width == other.width && self.length == other.length && self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for DWRITE_CLUSTER_METRICS {}
impl ::core::fmt::Debug for DWRITE_CLUSTER_METRICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DWRITE_CLUSTER_METRICS").field("width", &self.width).field("length", &self.length).field("_bitfield", &self._bitfield).finish()
    }
}
impl ::core::default::Default for DWRITE_COLOR_F {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DWRITE_COLOR_F {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r && self.g == other.g && self.b == other.b && self.a == other.a
    }
}
impl ::core::cmp::Eq for DWRITE_COLOR_F {}
impl ::core::fmt::Debug for DWRITE_COLOR_F {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DWRITE_COLOR_F").field("r", &self.r).field("g", &self.g).field("b", &self.b).field("a", &self.a).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DWRITE_COLOR_GLYPH_RUN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DWRITE_COLOR_GLYPH_RUN {
    fn eq(&self, other: &Self) -> bool {
        self.glyphRun == other.glyphRun && self.glyphRunDescription == other.glyphRunDescription && self.baselineOriginX == other.baselineOriginX && self.baselineOriginY == other.baselineOriginY && self.runColor == other.runColor && self.paletteIndex == other.paletteIndex
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DWRITE_COLOR_GLYPH_RUN {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DWRITE_COLOR_GLYPH_RUN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DWRITE_COLOR_GLYPH_RUN").field("glyphRun", &self.glyphRun).field("glyphRunDescription", &self.glyphRunDescription).field("baselineOriginX", &self.baselineOriginX).field("baselineOriginY", &self.baselineOriginY).field("runColor", &self.runColor).field("paletteIndex", &self.paletteIndex).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DWRITE_COLOR_GLYPH_RUN1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DWRITE_COLOR_GLYPH_RUN1 {
    fn eq(&self, other: &Self) -> bool {
        self.Base == other.Base && self.glyphImageFormat == other.glyphImageFormat && self.measuringMode == other.measuringMode
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DWRITE_COLOR_GLYPH_RUN1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DWRITE_COLOR_GLYPH_RUN1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DWRITE_COLOR_GLYPH_RUN1").field("Base", &self.Base).field("glyphImageFormat", &self.glyphImageFormat).field("measuringMode", &self.measuringMode).finish()
    }
}
impl ::core::default::Default for DWRITE_CONTAINER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DWRITE_CONTAINER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_CONTAINER_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DWRITE_FACTORY_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DWRITE_FACTORY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_FACTORY_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DWRITE_FILE_FRAGMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DWRITE_FILE_FRAGMENT {
    fn eq(&self, other: &Self) -> bool {
        self.fileOffset == other.fileOffset && self.fragmentSize == other.fragmentSize
    }
}
impl ::core::cmp::Eq for DWRITE_FILE_FRAGMENT {}
impl ::core::fmt::Debug for DWRITE_FILE_FRAGMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DWRITE_FILE_FRAGMENT").field("fileOffset", &self.fileOffset).field("fragmentSize", &self.fragmentSize).finish()
    }
}
impl ::core::default::Default for DWRITE_FLOW_DIRECTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DWRITE_FLOW_DIRECTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_FLOW_DIRECTION").field(&self.0).finish()
    }
}
impl ::core::default::Default for DWRITE_FONT_AXIS_ATTRIBUTES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DWRITE_FONT_AXIS_ATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_FONT_AXIS_ATTRIBUTES").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for DWRITE_FONT_AXIS_ATTRIBUTES {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DWRITE_FONT_AXIS_ATTRIBUTES {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DWRITE_FONT_AXIS_ATTRIBUTES {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DWRITE_FONT_AXIS_ATTRIBUTES {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DWRITE_FONT_AXIS_ATTRIBUTES {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for DWRITE_FONT_AXIS_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DWRITE_FONT_AXIS_RANGE {
    fn eq(&self, other: &Self) -> bool {
        self.axisTag == other.axisTag && self.minValue == other.minValue && self.maxValue == other.maxValue
    }
}
impl ::core::cmp::Eq for DWRITE_FONT_AXIS_RANGE {}
impl ::core::fmt::Debug for DWRITE_FONT_AXIS_RANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DWRITE_FONT_AXIS_RANGE").field("axisTag", &self.axisTag).field("minValue", &self.minValue).field("maxValue", &self.maxValue).finish()
    }
}
impl ::core::default::Default for DWRITE_FONT_AXIS_TAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DWRITE_FONT_AXIS_TAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_FONT_AXIS_TAG").field(&self.0).finish()
    }
}
impl ::core::default::Default for DWRITE_FONT_AXIS_VALUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DWRITE_FONT_AXIS_VALUE {
    fn eq(&self, other: &Self) -> bool {
        self.axisTag == other.axisTag && self.value == other.value
    }
}
impl ::core::cmp::Eq for DWRITE_FONT_AXIS_VALUE {}
impl ::core::fmt::Debug for DWRITE_FONT_AXIS_VALUE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DWRITE_FONT_AXIS_VALUE").field("axisTag", &self.axisTag).field("value", &self.value).finish()
    }
}
impl ::core::default::Default for DWRITE_FONT_FACE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DWRITE_FONT_FACE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_FONT_FACE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DWRITE_FONT_FAMILY_MODEL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DWRITE_FONT_FAMILY_MODEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_FONT_FAMILY_MODEL").field(&self.0).finish()
    }
}
impl ::core::default::Default for DWRITE_FONT_FEATURE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DWRITE_FONT_FEATURE {
    fn eq(&self, other: &Self) -> bool {
        self.nameTag == other.nameTag && self.parameter == other.parameter
    }
}
impl ::core::cmp::Eq for DWRITE_FONT_FEATURE {}
impl ::core::fmt::Debug for DWRITE_FONT_FEATURE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DWRITE_FONT_FEATURE").field("nameTag", &self.nameTag).field("parameter", &self.parameter).finish()
    }
}
impl ::core::default::Default for DWRITE_FONT_FEATURE_TAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DWRITE_FONT_FEATURE_TAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_FONT_FEATURE_TAG").field(&self.0).finish()
    }
}
impl ::core::default::Default for DWRITE_FONT_FILE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DWRITE_FONT_FILE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_FONT_FILE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DWRITE_FONT_LINE_GAP_USAGE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DWRITE_FONT_LINE_GAP_USAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_FONT_LINE_GAP_USAGE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DWRITE_FONT_METRICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DWRITE_FONT_METRICS {
    fn eq(&self, other: &Self) -> bool {
        self.designUnitsPerEm == other.designUnitsPerEm && self.ascent == other.ascent && self.descent == other.descent && self.lineGap == other.lineGap && self.capHeight == other.capHeight && self.xHeight == other.xHeight && self.underlinePosition == other.underlinePosition && self.underlineThickness == other.underlineThickness && self.strikethroughPosition == other.strikethroughPosition && self.strikethroughThickness == other.strikethroughThickness
    }
}
impl ::core::cmp::Eq for DWRITE_FONT_METRICS {}
impl ::core::fmt::Debug for DWRITE_FONT_METRICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DWRITE_FONT_METRICS")
            .field("designUnitsPerEm", &self.designUnitsPerEm)
            .field("ascent", &self.ascent)
            .field("descent", &self.descent)
            .field("lineGap", &self.lineGap)
            .field("capHeight", &self.capHeight)
            .field("xHeight", &self.xHeight)
            .field("underlinePosition", &self.underlinePosition)
            .field("underlineThickness", &self.underlineThickness)
            .field("strikethroughPosition", &self.strikethroughPosition)
            .field("strikethroughThickness", &self.strikethroughThickness)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DWRITE_FONT_METRICS1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DWRITE_FONT_METRICS1 {
    fn eq(&self, other: &Self) -> bool {
        self.Base == other.Base && self.glyphBoxLeft == other.glyphBoxLeft && self.glyphBoxTop == other.glyphBoxTop && self.glyphBoxRight == other.glyphBoxRight && self.glyphBoxBottom == other.glyphBoxBottom && self.subscriptPositionX == other.subscriptPositionX && self.subscriptPositionY == other.subscriptPositionY && self.subscriptSizeX == other.subscriptSizeX && self.subscriptSizeY == other.subscriptSizeY && self.superscriptPositionX == other.superscriptPositionX && self.superscriptPositionY == other.superscriptPositionY && self.superscriptSizeX == other.superscriptSizeX && self.superscriptSizeY == other.superscriptSizeY && self.hasTypographicMetrics == other.hasTypographicMetrics
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DWRITE_FONT_METRICS1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DWRITE_FONT_METRICS1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DWRITE_FONT_METRICS1")
            .field("Base", &self.Base)
            .field("glyphBoxLeft", &self.glyphBoxLeft)
            .field("glyphBoxTop", &self.glyphBoxTop)
            .field("glyphBoxRight", &self.glyphBoxRight)
            .field("glyphBoxBottom", &self.glyphBoxBottom)
            .field("subscriptPositionX", &self.subscriptPositionX)
            .field("subscriptPositionY", &self.subscriptPositionY)
            .field("subscriptSizeX", &self.subscriptSizeX)
            .field("subscriptSizeY", &self.subscriptSizeY)
            .field("superscriptPositionX", &self.superscriptPositionX)
            .field("superscriptPositionY", &self.superscriptPositionY)
            .field("superscriptSizeX", &self.superscriptSizeX)
            .field("superscriptSizeY", &self.superscriptSizeY)
            .field("hasTypographicMetrics", &self.hasTypographicMetrics)
            .finish()
    }
}
impl ::core::default::Default for DWRITE_FONT_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DWRITE_FONT_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        self.propertyId == other.propertyId && self.propertyValue == other.propertyValue && self.localeName == other.localeName
    }
}
impl ::core::cmp::Eq for DWRITE_FONT_PROPERTY {}
impl ::core::fmt::Debug for DWRITE_FONT_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DWRITE_FONT_PROPERTY").field("propertyId", &self.propertyId).field("propertyValue", &self.propertyValue).field("localeName", &self.localeName).finish()
    }
}
impl ::core::default::Default for DWRITE_FONT_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DWRITE_FONT_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_FONT_PROPERTY_ID").field(&self.0).finish()
    }
}
impl ::core::default::Default for DWRITE_FONT_SIMULATIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DWRITE_FONT_SIMULATIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_FONT_SIMULATIONS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for DWRITE_FONT_SIMULATIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DWRITE_FONT_SIMULATIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DWRITE_FONT_SIMULATIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DWRITE_FONT_SIMULATIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DWRITE_FONT_SIMULATIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for DWRITE_FONT_SOURCE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DWRITE_FONT_SOURCE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_FONT_SOURCE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DWRITE_FONT_STRETCH {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DWRITE_FONT_STRETCH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_FONT_STRETCH").field(&self.0).finish()
    }
}
impl ::core::default::Default for DWRITE_FONT_STYLE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DWRITE_FONT_STYLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_FONT_STYLE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DWRITE_FONT_WEIGHT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DWRITE_FONT_WEIGHT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_FONT_WEIGHT").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::core::default::Default for DWRITE_GLYPH_IMAGE_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::core::cmp::PartialEq for DWRITE_GLYPH_IMAGE_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.imageData == other.imageData && self.imageDataSize == other.imageDataSize && self.uniqueDataId == other.uniqueDataId && self.pixelsPerEm == other.pixelsPerEm && self.pixelSize == other.pixelSize && self.horizontalLeftOrigin == other.horizontalLeftOrigin && self.horizontalRightOrigin == other.horizontalRightOrigin && self.verticalTopOrigin == other.verticalTopOrigin && self.verticalBottomOrigin == other.verticalBottomOrigin
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::core::cmp::Eq for DWRITE_GLYPH_IMAGE_DATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::core::fmt::Debug for DWRITE_GLYPH_IMAGE_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DWRITE_GLYPH_IMAGE_DATA").field("imageData", &self.imageData).field("imageDataSize", &self.imageDataSize).field("uniqueDataId", &self.uniqueDataId).field("pixelsPerEm", &self.pixelsPerEm).field("pixelSize", &self.pixelSize).field("horizontalLeftOrigin", &self.horizontalLeftOrigin).field("horizontalRightOrigin", &self.horizontalRightOrigin).field("verticalTopOrigin", &self.verticalTopOrigin).field("verticalBottomOrigin", &self.verticalBottomOrigin).finish()
    }
}
impl ::core::default::Default for DWRITE_GLYPH_IMAGE_FORMATS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DWRITE_GLYPH_IMAGE_FORMATS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_GLYPH_IMAGE_FORMATS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for DWRITE_GLYPH_IMAGE_FORMATS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DWRITE_GLYPH_IMAGE_FORMATS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DWRITE_GLYPH_IMAGE_FORMATS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DWRITE_GLYPH_IMAGE_FORMATS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DWRITE_GLYPH_IMAGE_FORMATS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for DWRITE_GLYPH_METRICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DWRITE_GLYPH_METRICS {
    fn eq(&self, other: &Self) -> bool {
        self.leftSideBearing == other.leftSideBearing && self.advanceWidth == other.advanceWidth && self.rightSideBearing == other.rightSideBearing && self.topSideBearing == other.topSideBearing && self.advanceHeight == other.advanceHeight && self.bottomSideBearing == other.bottomSideBearing && self.verticalOriginY == other.verticalOriginY
    }
}
impl ::core::cmp::Eq for DWRITE_GLYPH_METRICS {}
impl ::core::fmt::Debug for DWRITE_GLYPH_METRICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DWRITE_GLYPH_METRICS").field("leftSideBearing", &self.leftSideBearing).field("advanceWidth", &self.advanceWidth).field("rightSideBearing", &self.rightSideBearing).field("topSideBearing", &self.topSideBearing).field("advanceHeight", &self.advanceHeight).field("bottomSideBearing", &self.bottomSideBearing).field("verticalOriginY", &self.verticalOriginY).finish()
    }
}
impl ::core::default::Default for DWRITE_GLYPH_OFFSET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DWRITE_GLYPH_OFFSET {
    fn eq(&self, other: &Self) -> bool {
        self.advanceOffset == other.advanceOffset && self.ascenderOffset == other.ascenderOffset
    }
}
impl ::core::cmp::Eq for DWRITE_GLYPH_OFFSET {}
impl ::core::fmt::Debug for DWRITE_GLYPH_OFFSET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DWRITE_GLYPH_OFFSET").field("advanceOffset", &self.advanceOffset).field("ascenderOffset", &self.ascenderOffset).finish()
    }
}
impl ::core::default::Default for DWRITE_GLYPH_ORIENTATION_ANGLE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DWRITE_GLYPH_ORIENTATION_ANGLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_GLYPH_ORIENTATION_ANGLE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DWRITE_GLYPH_RUN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DWRITE_GLYPH_RUN {
    fn eq(&self, other: &Self) -> bool {
        self.fontFace == other.fontFace && self.fontEmSize == other.fontEmSize && self.glyphCount == other.glyphCount && self.glyphIndices == other.glyphIndices && self.glyphAdvances == other.glyphAdvances && self.glyphOffsets == other.glyphOffsets && self.isSideways == other.isSideways && self.bidiLevel == other.bidiLevel
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DWRITE_GLYPH_RUN {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DWRITE_GLYPH_RUN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DWRITE_GLYPH_RUN").field("fontFace", &self.fontFace).field("fontEmSize", &self.fontEmSize).field("glyphCount", &self.glyphCount).field("glyphIndices", &self.glyphIndices).field("glyphAdvances", &self.glyphAdvances).field("glyphOffsets", &self.glyphOffsets).field("isSideways", &self.isSideways).field("bidiLevel", &self.bidiLevel).finish()
    }
}
impl ::core::default::Default for DWRITE_GLYPH_RUN_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DWRITE_GLYPH_RUN_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.localeName == other.localeName && self.string == other.string && self.stringLength == other.stringLength && self.clusterMap == other.clusterMap && self.textPosition == other.textPosition
    }
}
impl ::core::cmp::Eq for DWRITE_GLYPH_RUN_DESCRIPTION {}
impl ::core::fmt::Debug for DWRITE_GLYPH_RUN_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DWRITE_GLYPH_RUN_DESCRIPTION").field("localeName", &self.localeName).field("string", &self.string).field("stringLength", &self.stringLength).field("clusterMap", &self.clusterMap).field("textPosition", &self.textPosition).finish()
    }
}
impl ::core::default::Default for DWRITE_GRID_FIT_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DWRITE_GRID_FIT_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_GRID_FIT_MODE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DWRITE_HIT_TEST_METRICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DWRITE_HIT_TEST_METRICS {
    fn eq(&self, other: &Self) -> bool {
        self.textPosition == other.textPosition && self.length == other.length && self.left == other.left && self.top == other.top && self.width == other.width && self.height == other.height && self.bidiLevel == other.bidiLevel && self.isText == other.isText && self.isTrimmed == other.isTrimmed
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DWRITE_HIT_TEST_METRICS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DWRITE_HIT_TEST_METRICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DWRITE_HIT_TEST_METRICS").field("textPosition", &self.textPosition).field("length", &self.length).field("left", &self.left).field("top", &self.top).field("width", &self.width).field("height", &self.height).field("bidiLevel", &self.bidiLevel).field("isText", &self.isText).field("isTrimmed", &self.isTrimmed).finish()
    }
}
impl ::core::default::Default for DWRITE_INFORMATIONAL_STRING_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DWRITE_INFORMATIONAL_STRING_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_INFORMATIONAL_STRING_ID").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DWRITE_INLINE_OBJECT_METRICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DWRITE_INLINE_OBJECT_METRICS {
    fn eq(&self, other: &Self) -> bool {
        self.width == other.width && self.height == other.height && self.baseline == other.baseline && self.supportsSideways == other.supportsSideways
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DWRITE_INLINE_OBJECT_METRICS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DWRITE_INLINE_OBJECT_METRICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DWRITE_INLINE_OBJECT_METRICS").field("width", &self.width).field("height", &self.height).field("baseline", &self.baseline).field("supportsSideways", &self.supportsSideways).finish()
    }
}
impl ::core::default::Default for DWRITE_JUSTIFICATION_OPPORTUNITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DWRITE_JUSTIFICATION_OPPORTUNITY {
    fn eq(&self, other: &Self) -> bool {
        self.expansionMinimum == other.expansionMinimum && self.expansionMaximum == other.expansionMaximum && self.compressionMaximum == other.compressionMaximum && self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for DWRITE_JUSTIFICATION_OPPORTUNITY {}
impl ::core::fmt::Debug for DWRITE_JUSTIFICATION_OPPORTUNITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DWRITE_JUSTIFICATION_OPPORTUNITY").field("expansionMinimum", &self.expansionMinimum).field("expansionMaximum", &self.expansionMaximum).field("compressionMaximum", &self.compressionMaximum).field("_bitfield", &self._bitfield).finish()
    }
}
impl ::core::default::Default for DWRITE_LINE_BREAKPOINT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DWRITE_LINE_BREAKPOINT {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for DWRITE_LINE_BREAKPOINT {}
impl ::core::fmt::Debug for DWRITE_LINE_BREAKPOINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DWRITE_LINE_BREAKPOINT").field("_bitfield", &self._bitfield).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DWRITE_LINE_METRICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DWRITE_LINE_METRICS {
    fn eq(&self, other: &Self) -> bool {
        self.length == other.length && self.trailingWhitespaceLength == other.trailingWhitespaceLength && self.newlineLength == other.newlineLength && self.height == other.height && self.baseline == other.baseline && self.isTrimmed == other.isTrimmed
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DWRITE_LINE_METRICS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DWRITE_LINE_METRICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DWRITE_LINE_METRICS").field("length", &self.length).field("trailingWhitespaceLength", &self.trailingWhitespaceLength).field("newlineLength", &self.newlineLength).field("height", &self.height).field("baseline", &self.baseline).field("isTrimmed", &self.isTrimmed).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DWRITE_LINE_METRICS1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DWRITE_LINE_METRICS1 {
    fn eq(&self, other: &Self) -> bool {
        self.Base == other.Base && self.leadingBefore == other.leadingBefore && self.leadingAfter == other.leadingAfter
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DWRITE_LINE_METRICS1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DWRITE_LINE_METRICS1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DWRITE_LINE_METRICS1").field("Base", &self.Base).field("leadingBefore", &self.leadingBefore).field("leadingAfter", &self.leadingAfter).finish()
    }
}
impl ::core::default::Default for DWRITE_LINE_SPACING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DWRITE_LINE_SPACING {
    fn eq(&self, other: &Self) -> bool {
        self.method == other.method && self.height == other.height && self.baseline == other.baseline && self.leadingBefore == other.leadingBefore && self.fontLineGapUsage == other.fontLineGapUsage
    }
}
impl ::core::cmp::Eq for DWRITE_LINE_SPACING {}
impl ::core::fmt::Debug for DWRITE_LINE_SPACING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DWRITE_LINE_SPACING").field("method", &self.method).field("height", &self.height).field("baseline", &self.baseline).field("leadingBefore", &self.leadingBefore).field("fontLineGapUsage", &self.fontLineGapUsage).finish()
    }
}
impl ::core::default::Default for DWRITE_LINE_SPACING_METHOD {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DWRITE_LINE_SPACING_METHOD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_LINE_SPACING_METHOD").field(&self.0).finish()
    }
}
impl ::core::default::Default for DWRITE_LOCALITY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DWRITE_LOCALITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_LOCALITY").field(&self.0).finish()
    }
}
impl ::core::default::Default for DWRITE_MATRIX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DWRITE_MATRIX {
    fn eq(&self, other: &Self) -> bool {
        self.m11 == other.m11 && self.m12 == other.m12 && self.m21 == other.m21 && self.m22 == other.m22 && self.dx == other.dx && self.dy == other.dy
    }
}
impl ::core::cmp::Eq for DWRITE_MATRIX {}
impl ::core::fmt::Debug for DWRITE_MATRIX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DWRITE_MATRIX").field("m11", &self.m11).field("m12", &self.m12).field("m21", &self.m21).field("m22", &self.m22).field("dx", &self.dx).field("dy", &self.dy).finish()
    }
}
impl ::core::default::Default for DWRITE_MEASURING_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DWRITE_MEASURING_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_MEASURING_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DWRITE_NUMBER_SUBSTITUTION_METHOD {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DWRITE_NUMBER_SUBSTITUTION_METHOD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_NUMBER_SUBSTITUTION_METHOD").field(&self.0).finish()
    }
}
impl ::core::default::Default for DWRITE_OPTICAL_ALIGNMENT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DWRITE_OPTICAL_ALIGNMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_OPTICAL_ALIGNMENT").field(&self.0).finish()
    }
}
impl ::core::default::Default for DWRITE_OUTLINE_THRESHOLD {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DWRITE_OUTLINE_THRESHOLD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_OUTLINE_THRESHOLD").field(&self.0).finish()
    }
}
impl ::core::default::Default for DWRITE_OVERHANG_METRICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DWRITE_OVERHANG_METRICS {
    fn eq(&self, other: &Self) -> bool {
        self.left == other.left && self.top == other.top && self.right == other.right && self.bottom == other.bottom
    }
}
impl ::core::cmp::Eq for DWRITE_OVERHANG_METRICS {}
impl ::core::fmt::Debug for DWRITE_OVERHANG_METRICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DWRITE_OVERHANG_METRICS").field("left", &self.left).field("top", &self.top).field("right", &self.right).field("bottom", &self.bottom).finish()
    }
}
impl ::core::default::Default for DWRITE_PANOSE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DWRITE_PANOSE_ARM_STYLE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DWRITE_PANOSE_ARM_STYLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_PANOSE_ARM_STYLE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DWRITE_PANOSE_ASPECT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DWRITE_PANOSE_ASPECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_PANOSE_ASPECT").field(&self.0).finish()
    }
}
impl ::core::default::Default for DWRITE_PANOSE_ASPECT_RATIO {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DWRITE_PANOSE_ASPECT_RATIO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_PANOSE_ASPECT_RATIO").field(&self.0).finish()
    }
}
impl ::core::default::Default for DWRITE_PANOSE_CHARACTER_RANGES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DWRITE_PANOSE_CHARACTER_RANGES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_PANOSE_CHARACTER_RANGES").field(&self.0).finish()
    }
}
impl ::core::default::Default for DWRITE_PANOSE_CONTRAST {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DWRITE_PANOSE_CONTRAST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_PANOSE_CONTRAST").field(&self.0).finish()
    }
}
impl ::core::default::Default for DWRITE_PANOSE_DECORATIVE_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DWRITE_PANOSE_DECORATIVE_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_PANOSE_DECORATIVE_CLASS").field(&self.0).finish()
    }
}
impl ::core::default::Default for DWRITE_PANOSE_DECORATIVE_TOPOLOGY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DWRITE_PANOSE_DECORATIVE_TOPOLOGY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_PANOSE_DECORATIVE_TOPOLOGY").field(&self.0).finish()
    }
}
impl ::core::default::Default for DWRITE_PANOSE_FAMILY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DWRITE_PANOSE_FAMILY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_PANOSE_FAMILY").field(&self.0).finish()
    }
}
impl ::core::default::Default for DWRITE_PANOSE_FILL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DWRITE_PANOSE_FILL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_PANOSE_FILL").field(&self.0).finish()
    }
}
impl ::core::default::Default for DWRITE_PANOSE_FINIALS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DWRITE_PANOSE_FINIALS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_PANOSE_FINIALS").field(&self.0).finish()
    }
}
impl ::core::default::Default for DWRITE_PANOSE_LETTERFORM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DWRITE_PANOSE_LETTERFORM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_PANOSE_LETTERFORM").field(&self.0).finish()
    }
}
impl ::core::default::Default for DWRITE_PANOSE_LINING {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DWRITE_PANOSE_LINING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_PANOSE_LINING").field(&self.0).finish()
    }
}
impl ::core::default::Default for DWRITE_PANOSE_MIDLINE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DWRITE_PANOSE_MIDLINE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_PANOSE_MIDLINE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DWRITE_PANOSE_PROPORTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DWRITE_PANOSE_PROPORTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_PANOSE_PROPORTION").field(&self.0).finish()
    }
}
impl ::core::default::Default for DWRITE_PANOSE_SCRIPT_FORM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DWRITE_PANOSE_SCRIPT_FORM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_PANOSE_SCRIPT_FORM").field(&self.0).finish()
    }
}
impl ::core::default::Default for DWRITE_PANOSE_SCRIPT_TOPOLOGY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DWRITE_PANOSE_SCRIPT_TOPOLOGY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_PANOSE_SCRIPT_TOPOLOGY").field(&self.0).finish()
    }
}
impl ::core::default::Default for DWRITE_PANOSE_SERIF_STYLE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DWRITE_PANOSE_SERIF_STYLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_PANOSE_SERIF_STYLE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DWRITE_PANOSE_SPACING {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DWRITE_PANOSE_SPACING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_PANOSE_SPACING").field(&self.0).finish()
    }
}
impl ::core::default::Default for DWRITE_PANOSE_STROKE_VARIATION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DWRITE_PANOSE_STROKE_VARIATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_PANOSE_STROKE_VARIATION").field(&self.0).finish()
    }
}
impl ::core::default::Default for DWRITE_PANOSE_SYMBOL_ASPECT_RATIO {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DWRITE_PANOSE_SYMBOL_ASPECT_RATIO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_PANOSE_SYMBOL_ASPECT_RATIO").field(&self.0).finish()
    }
}
impl ::core::default::Default for DWRITE_PANOSE_SYMBOL_KIND {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DWRITE_PANOSE_SYMBOL_KIND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_PANOSE_SYMBOL_KIND").field(&self.0).finish()
    }
}
impl ::core::default::Default for DWRITE_PANOSE_TOOL_KIND {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DWRITE_PANOSE_TOOL_KIND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_PANOSE_TOOL_KIND").field(&self.0).finish()
    }
}
impl ::core::default::Default for DWRITE_PANOSE_WEIGHT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DWRITE_PANOSE_WEIGHT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_PANOSE_WEIGHT").field(&self.0).finish()
    }
}
impl ::core::default::Default for DWRITE_PANOSE_XASCENT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DWRITE_PANOSE_XASCENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_PANOSE_XASCENT").field(&self.0).finish()
    }
}
impl ::core::default::Default for DWRITE_PANOSE_XHEIGHT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DWRITE_PANOSE_XHEIGHT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_PANOSE_XHEIGHT").field(&self.0).finish()
    }
}
impl ::core::default::Default for DWRITE_PARAGRAPH_ALIGNMENT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DWRITE_PARAGRAPH_ALIGNMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_PARAGRAPH_ALIGNMENT").field(&self.0).finish()
    }
}
impl ::core::default::Default for DWRITE_PIXEL_GEOMETRY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DWRITE_PIXEL_GEOMETRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_PIXEL_GEOMETRY").field(&self.0).finish()
    }
}
impl ::core::default::Default for DWRITE_READING_DIRECTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DWRITE_READING_DIRECTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_READING_DIRECTION").field(&self.0).finish()
    }
}
impl ::core::default::Default for DWRITE_RENDERING_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DWRITE_RENDERING_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_RENDERING_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DWRITE_RENDERING_MODE1 {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DWRITE_RENDERING_MODE1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_RENDERING_MODE1").field(&self.0).finish()
    }
}
impl ::core::default::Default for DWRITE_SCRIPT_ANALYSIS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DWRITE_SCRIPT_ANALYSIS {
    fn eq(&self, other: &Self) -> bool {
        self.script == other.script && self.shapes == other.shapes
    }
}
impl ::core::cmp::Eq for DWRITE_SCRIPT_ANALYSIS {}
impl ::core::fmt::Debug for DWRITE_SCRIPT_ANALYSIS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DWRITE_SCRIPT_ANALYSIS").field("script", &self.script).field("shapes", &self.shapes).finish()
    }
}
impl ::core::default::Default for DWRITE_SCRIPT_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DWRITE_SCRIPT_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        self.isoScriptCode == other.isoScriptCode && self.isoScriptNumber == other.isoScriptNumber && self.clusterLookahead == other.clusterLookahead && self.justificationCharacter == other.justificationCharacter && self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for DWRITE_SCRIPT_PROPERTIES {}
impl ::core::fmt::Debug for DWRITE_SCRIPT_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DWRITE_SCRIPT_PROPERTIES").field("isoScriptCode", &self.isoScriptCode).field("isoScriptNumber", &self.isoScriptNumber).field("clusterLookahead", &self.clusterLookahead).field("justificationCharacter", &self.justificationCharacter).field("_bitfield", &self._bitfield).finish()
    }
}
impl ::core::default::Default for DWRITE_SCRIPT_SHAPES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DWRITE_SCRIPT_SHAPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_SCRIPT_SHAPES").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for DWRITE_SCRIPT_SHAPES {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DWRITE_SCRIPT_SHAPES {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DWRITE_SCRIPT_SHAPES {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DWRITE_SCRIPT_SHAPES {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DWRITE_SCRIPT_SHAPES {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for DWRITE_SHAPING_GLYPH_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DWRITE_SHAPING_GLYPH_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for DWRITE_SHAPING_GLYPH_PROPERTIES {}
impl ::core::fmt::Debug for DWRITE_SHAPING_GLYPH_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DWRITE_SHAPING_GLYPH_PROPERTIES").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::core::default::Default for DWRITE_SHAPING_TEXT_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DWRITE_SHAPING_TEXT_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for DWRITE_SHAPING_TEXT_PROPERTIES {}
impl ::core::fmt::Debug for DWRITE_SHAPING_TEXT_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DWRITE_SHAPING_TEXT_PROPERTIES").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::core::default::Default for DWRITE_STRIKETHROUGH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DWRITE_STRIKETHROUGH {
    fn eq(&self, other: &Self) -> bool {
        self.width == other.width && self.thickness == other.thickness && self.offset == other.offset && self.readingDirection == other.readingDirection && self.flowDirection == other.flowDirection && self.localeName == other.localeName && self.measuringMode == other.measuringMode
    }
}
impl ::core::cmp::Eq for DWRITE_STRIKETHROUGH {}
impl ::core::fmt::Debug for DWRITE_STRIKETHROUGH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DWRITE_STRIKETHROUGH").field("width", &self.width).field("thickness", &self.thickness).field("offset", &self.offset).field("readingDirection", &self.readingDirection).field("flowDirection", &self.flowDirection).field("localeName", &self.localeName).field("measuringMode", &self.measuringMode).finish()
    }
}
impl ::core::default::Default for DWRITE_TEXTURE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DWRITE_TEXTURE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_TEXTURE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DWRITE_TEXT_ALIGNMENT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DWRITE_TEXT_ALIGNMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_TEXT_ALIGNMENT").field(&self.0).finish()
    }
}
impl ::core::default::Default for DWRITE_TEXT_ANTIALIAS_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DWRITE_TEXT_ANTIALIAS_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_TEXT_ANTIALIAS_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DWRITE_TEXT_METRICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DWRITE_TEXT_METRICS {
    fn eq(&self, other: &Self) -> bool {
        self.left == other.left && self.top == other.top && self.width == other.width && self.widthIncludingTrailingWhitespace == other.widthIncludingTrailingWhitespace && self.height == other.height && self.layoutWidth == other.layoutWidth && self.layoutHeight == other.layoutHeight && self.maxBidiReorderingDepth == other.maxBidiReorderingDepth && self.lineCount == other.lineCount
    }
}
impl ::core::cmp::Eq for DWRITE_TEXT_METRICS {}
impl ::core::fmt::Debug for DWRITE_TEXT_METRICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DWRITE_TEXT_METRICS").field("left", &self.left).field("top", &self.top).field("width", &self.width).field("widthIncludingTrailingWhitespace", &self.widthIncludingTrailingWhitespace).field("height", &self.height).field("layoutWidth", &self.layoutWidth).field("layoutHeight", &self.layoutHeight).field("maxBidiReorderingDepth", &self.maxBidiReorderingDepth).field("lineCount", &self.lineCount).finish()
    }
}
impl ::core::default::Default for DWRITE_TEXT_METRICS1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DWRITE_TEXT_METRICS1 {
    fn eq(&self, other: &Self) -> bool {
        self.Base == other.Base && self.heightIncludingTrailingWhitespace == other.heightIncludingTrailingWhitespace
    }
}
impl ::core::cmp::Eq for DWRITE_TEXT_METRICS1 {}
impl ::core::fmt::Debug for DWRITE_TEXT_METRICS1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DWRITE_TEXT_METRICS1").field("Base", &self.Base).field("heightIncludingTrailingWhitespace", &self.heightIncludingTrailingWhitespace).finish()
    }
}
impl ::core::default::Default for DWRITE_TEXT_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DWRITE_TEXT_RANGE {
    fn eq(&self, other: &Self) -> bool {
        self.startPosition == other.startPosition && self.length == other.length
    }
}
impl ::core::cmp::Eq for DWRITE_TEXT_RANGE {}
impl ::core::fmt::Debug for DWRITE_TEXT_RANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DWRITE_TEXT_RANGE").field("startPosition", &self.startPosition).field("length", &self.length).finish()
    }
}
impl ::core::default::Default for DWRITE_TRIMMING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DWRITE_TRIMMING {
    fn eq(&self, other: &Self) -> bool {
        self.granularity == other.granularity && self.delimiter == other.delimiter && self.delimiterCount == other.delimiterCount
    }
}
impl ::core::cmp::Eq for DWRITE_TRIMMING {}
impl ::core::fmt::Debug for DWRITE_TRIMMING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DWRITE_TRIMMING").field("granularity", &self.granularity).field("delimiter", &self.delimiter).field("delimiterCount", &self.delimiterCount).finish()
    }
}
impl ::core::default::Default for DWRITE_TRIMMING_GRANULARITY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DWRITE_TRIMMING_GRANULARITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_TRIMMING_GRANULARITY").field(&self.0).finish()
    }
}
impl ::core::default::Default for DWRITE_TYPOGRAPHIC_FEATURES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DWRITE_TYPOGRAPHIC_FEATURES {
    fn eq(&self, other: &Self) -> bool {
        self.features == other.features && self.featureCount == other.featureCount
    }
}
impl ::core::cmp::Eq for DWRITE_TYPOGRAPHIC_FEATURES {}
impl ::core::fmt::Debug for DWRITE_TYPOGRAPHIC_FEATURES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DWRITE_TYPOGRAPHIC_FEATURES").field("features", &self.features).field("featureCount", &self.featureCount).finish()
    }
}
impl ::core::default::Default for DWRITE_UNDERLINE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DWRITE_UNDERLINE {
    fn eq(&self, other: &Self) -> bool {
        self.width == other.width && self.thickness == other.thickness && self.offset == other.offset && self.runHeight == other.runHeight && self.readingDirection == other.readingDirection && self.flowDirection == other.flowDirection && self.localeName == other.localeName && self.measuringMode == other.measuringMode
    }
}
impl ::core::cmp::Eq for DWRITE_UNDERLINE {}
impl ::core::fmt::Debug for DWRITE_UNDERLINE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DWRITE_UNDERLINE").field("width", &self.width).field("thickness", &self.thickness).field("offset", &self.offset).field("runHeight", &self.runHeight).field("readingDirection", &self.readingDirection).field("flowDirection", &self.flowDirection).field("localeName", &self.localeName).field("measuringMode", &self.measuringMode).finish()
    }
}
impl ::core::default::Default for DWRITE_UNICODE_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DWRITE_UNICODE_RANGE {
    fn eq(&self, other: &Self) -> bool {
        self.first == other.first && self.last == other.last
    }
}
impl ::core::cmp::Eq for DWRITE_UNICODE_RANGE {}
impl ::core::fmt::Debug for DWRITE_UNICODE_RANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DWRITE_UNICODE_RANGE").field("first", &self.first).field("last", &self.last).finish()
    }
}
impl ::core::default::Default for DWRITE_VERTICAL_GLYPH_ORIENTATION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DWRITE_VERTICAL_GLYPH_ORIENTATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_VERTICAL_GLYPH_ORIENTATION").field(&self.0).finish()
    }
}
impl ::core::default::Default for DWRITE_WORD_WRAPPING {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DWRITE_WORD_WRAPPING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_WORD_WRAPPING").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDWriteAsyncResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDWriteAsyncResult {}
impl ::core::fmt::Debug for IDWriteAsyncResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDWriteAsyncResult").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDWriteBitmapRenderTarget {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDWriteBitmapRenderTarget {}
impl ::core::fmt::Debug for IDWriteBitmapRenderTarget {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDWriteBitmapRenderTarget").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDWriteBitmapRenderTarget1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDWriteBitmapRenderTarget1 {}
impl ::core::fmt::Debug for IDWriteBitmapRenderTarget1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDWriteBitmapRenderTarget1").field(&self.0).finish()
    }
}
impl IDWriteBitmapRenderTarget1 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DrawGlyphRun<P0, P1>(&self, baselineoriginx: f32, baselineoriginy: f32, measuringmode: DWRITE_MEASURING_MODE, glyphrun: *const DWRITE_GLYPH_RUN, renderingparams: P0, textcolor: P1, blackboxrect: ::core::option::Option<*mut super::super::Foundation::RECT>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteRenderingParams>>,
        P1: ::std::convert::Into<super::super::Foundation::COLORREF>,
    {
        (::windows::core::Vtable::vtable(self).base__.DrawGlyphRun)(::windows::core::Vtable::as_raw(self), baselineoriginx, baselineoriginy, measuringmode, glyphrun, renderingparams.into().abi(), textcolor.into(), ::core::mem::transmute(blackboxrect.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn GetMemoryDC(&self) -> super::Gdi::HDC {
        (::windows::core::Vtable::vtable(self).base__.GetMemoryDC)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetPixelsPerDip(&self) -> f32 {
        (::windows::core::Vtable::vtable(self).base__.GetPixelsPerDip)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn SetPixelsPerDip(&self, pixelsperdip: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetPixelsPerDip)(::windows::core::Vtable::as_raw(self), pixelsperdip).ok()
    }
    pub unsafe fn GetCurrentTransform(&self, transform: *mut DWRITE_MATRIX) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetCurrentTransform)(::windows::core::Vtable::as_raw(self), transform).ok()
    }
    pub unsafe fn SetCurrentTransform(&self, transform: ::core::option::Option<*const DWRITE_MATRIX>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetCurrentTransform)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(transform.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSize(&self) -> ::windows::core::Result<super::super::Foundation::SIZE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetSize)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Resize(&self, width: u32, height: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Resize)(::windows::core::Vtable::as_raw(self), width, height).ok()
    }
}
impl ::core::cmp::PartialEq for IDWriteColorGlyphRunEnumerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDWriteColorGlyphRunEnumerator {}
impl ::core::fmt::Debug for IDWriteColorGlyphRunEnumerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDWriteColorGlyphRunEnumerator").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDWriteColorGlyphRunEnumerator1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDWriteColorGlyphRunEnumerator1 {}
impl ::core::fmt::Debug for IDWriteColorGlyphRunEnumerator1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDWriteColorGlyphRunEnumerator1").field(&self.0).finish()
    }
}
impl IDWriteColorGlyphRunEnumerator1 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MoveNext(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.MoveNext)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetCurrentRun(&self) -> ::windows::core::Result<*mut DWRITE_COLOR_GLYPH_RUN> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCurrentRun)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IDWriteFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDWriteFactory {}
impl ::core::fmt::Debug for IDWriteFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDWriteFactory").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDWriteFactory1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDWriteFactory1 {}
impl ::core::fmt::Debug for IDWriteFactory1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDWriteFactory1").field(&self.0).finish()
    }
}
impl IDWriteFactory1 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSystemFontCollection<P0>(&self, fontcollection: *mut ::core::option::Option<IDWriteFontCollection>, checkforupdates: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetSystemFontCollection)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(fontcollection), checkforupdates.into()).ok()
    }
    pub unsafe fn CreateCustomFontCollection<P0>(&self, collectionloader: P0, collectionkey: *const ::core::ffi::c_void, collectionkeysize: u32) -> ::windows::core::Result<IDWriteFontCollection>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteFontCollectionLoader>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateCustomFontCollection)(::windows::core::Vtable::as_raw(self), collectionloader.into().abi(), collectionkey, collectionkeysize, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RegisterFontCollectionLoader<P0>(&self, fontcollectionloader: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteFontCollectionLoader>>,
    {
        (::windows::core::Vtable::vtable(self).base__.RegisterFontCollectionLoader)(::windows::core::Vtable::as_raw(self), fontcollectionloader.into().abi()).ok()
    }
    pub unsafe fn UnregisterFontCollectionLoader<P0>(&self, fontcollectionloader: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteFontCollectionLoader>>,
    {
        (::windows::core::Vtable::vtable(self).base__.UnregisterFontCollectionLoader)(::windows::core::Vtable::as_raw(self), fontcollectionloader.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateFontFileReference<P0>(&self, filepath: P0, lastwritetime: ::core::option::Option<*const super::super::Foundation::FILETIME>) -> ::windows::core::Result<IDWriteFontFile>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateFontFileReference)(::windows::core::Vtable::as_raw(self), filepath.into().abi(), ::core::mem::transmute(lastwritetime.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateCustomFontFileReference<P0>(&self, fontfilereferencekey: *const ::core::ffi::c_void, fontfilereferencekeysize: u32, fontfileloader: P0) -> ::windows::core::Result<IDWriteFontFile>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteFontFileLoader>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateCustomFontFileReference)(::windows::core::Vtable::as_raw(self), fontfilereferencekey, fontfilereferencekeysize, fontfileloader.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateFontFace(&self, fontfacetype: DWRITE_FONT_FACE_TYPE, fontfiles: &[IDWriteFontFile], faceindex: u32, fontfacesimulationflags: DWRITE_FONT_SIMULATIONS) -> ::windows::core::Result<IDWriteFontFace> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateFontFace)(::windows::core::Vtable::as_raw(self), fontfacetype, fontfiles.len() as _, ::core::mem::transmute(fontfiles.as_ptr()), faceindex, fontfacesimulationflags, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateRenderingParams(&self) -> ::windows::core::Result<IDWriteRenderingParams> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateRenderingParams)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn CreateMonitorRenderingParams<P0>(&self, monitor: P0) -> ::windows::core::Result<IDWriteRenderingParams>
    where
        P0: ::std::convert::Into<super::Gdi::HMONITOR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateMonitorRenderingParams)(::windows::core::Vtable::as_raw(self), monitor.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateCustomRenderingParams(&self, gamma: f32, enhancedcontrast: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE) -> ::windows::core::Result<IDWriteRenderingParams> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateCustomRenderingParams)(::windows::core::Vtable::as_raw(self), gamma, enhancedcontrast, cleartypelevel, pixelgeometry, renderingmode, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RegisterFontFileLoader<P0>(&self, fontfileloader: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteFontFileLoader>>,
    {
        (::windows::core::Vtable::vtable(self).base__.RegisterFontFileLoader)(::windows::core::Vtable::as_raw(self), fontfileloader.into().abi()).ok()
    }
    pub unsafe fn UnregisterFontFileLoader<P0>(&self, fontfileloader: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteFontFileLoader>>,
    {
        (::windows::core::Vtable::vtable(self).base__.UnregisterFontFileLoader)(::windows::core::Vtable::as_raw(self), fontfileloader.into().abi()).ok()
    }
    pub unsafe fn CreateTextFormat<P0, P1, P2>(&self, fontfamilyname: P0, fontcollection: P1, fontweight: DWRITE_FONT_WEIGHT, fontstyle: DWRITE_FONT_STYLE, fontstretch: DWRITE_FONT_STRETCH, fontsize: f32, localename: P2) -> ::windows::core::Result<IDWriteTextFormat>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<IDWriteFontCollection>>,
        P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateTextFormat)(::windows::core::Vtable::as_raw(self), fontfamilyname.into().abi(), fontcollection.into().abi(), fontweight, fontstyle, fontstretch, fontsize, localename.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateTypography(&self) -> ::windows::core::Result<IDWriteTypography> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateTypography)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetGdiInterop(&self) -> ::windows::core::Result<IDWriteGdiInterop> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetGdiInterop)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateTextLayout<P0>(&self, string: &[u16], textformat: P0, maxwidth: f32, maxheight: f32) -> ::windows::core::Result<IDWriteTextLayout>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteTextFormat>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateTextLayout)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(string.as_ptr()), string.len() as _, textformat.into().abi(), maxwidth, maxheight, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateGdiCompatibleTextLayout<P0, P1>(&self, string: &[u16], textformat: P0, layoutwidth: f32, layoutheight: f32, pixelsperdip: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, usegdinatural: P1) -> ::windows::core::Result<IDWriteTextLayout>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteTextFormat>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateGdiCompatibleTextLayout)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(string.as_ptr()), string.len() as _, textformat.into().abi(), layoutwidth, layoutheight, pixelsperdip, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), usegdinatural.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateEllipsisTrimmingSign<P0>(&self, textformat: P0) -> ::windows::core::Result<IDWriteInlineObject>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteTextFormat>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateEllipsisTrimmingSign)(::windows::core::Vtable::as_raw(self), textformat.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateTextAnalyzer(&self) -> ::windows::core::Result<IDWriteTextAnalyzer> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateTextAnalyzer)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateNumberSubstitution<P0, P1>(&self, substitutionmethod: DWRITE_NUMBER_SUBSTITUTION_METHOD, localename: P0, ignoreuseroverride: P1) -> ::windows::core::Result<IDWriteNumberSubstitution>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateNumberSubstitution)(::windows::core::Vtable::as_raw(self), substitutionmethod, localename.into().abi(), ignoreuseroverride.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateGlyphRunAnalysis(&self, glyphrun: *const DWRITE_GLYPH_RUN, pixelsperdip: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, renderingmode: DWRITE_RENDERING_MODE, measuringmode: DWRITE_MEASURING_MODE, baselineoriginx: f32, baselineoriginy: f32) -> ::windows::core::Result<IDWriteGlyphRunAnalysis> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateGlyphRunAnalysis)(::windows::core::Vtable::as_raw(self), glyphrun, pixelsperdip, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), renderingmode, measuringmode, baselineoriginx, baselineoriginy, result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IDWriteFactory2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDWriteFactory2 {}
impl ::core::fmt::Debug for IDWriteFactory2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDWriteFactory2").field(&self.0).finish()
    }
}
impl IDWriteFactory2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSystemFontCollection<P0>(&self, fontcollection: *mut ::core::option::Option<IDWriteFontCollection>, checkforupdates: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.GetSystemFontCollection)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(fontcollection), checkforupdates.into()).ok()
    }
    pub unsafe fn CreateCustomFontCollection<P0>(&self, collectionloader: P0, collectionkey: *const ::core::ffi::c_void, collectionkeysize: u32) -> ::windows::core::Result<IDWriteFontCollection>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteFontCollectionLoader>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateCustomFontCollection)(::windows::core::Vtable::as_raw(self), collectionloader.into().abi(), collectionkey, collectionkeysize, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RegisterFontCollectionLoader<P0>(&self, fontcollectionloader: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteFontCollectionLoader>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.RegisterFontCollectionLoader)(::windows::core::Vtable::as_raw(self), fontcollectionloader.into().abi()).ok()
    }
    pub unsafe fn UnregisterFontCollectionLoader<P0>(&self, fontcollectionloader: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteFontCollectionLoader>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.UnregisterFontCollectionLoader)(::windows::core::Vtable::as_raw(self), fontcollectionloader.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateFontFileReference<P0>(&self, filepath: P0, lastwritetime: ::core::option::Option<*const super::super::Foundation::FILETIME>) -> ::windows::core::Result<IDWriteFontFile>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateFontFileReference)(::windows::core::Vtable::as_raw(self), filepath.into().abi(), ::core::mem::transmute(lastwritetime.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateCustomFontFileReference<P0>(&self, fontfilereferencekey: *const ::core::ffi::c_void, fontfilereferencekeysize: u32, fontfileloader: P0) -> ::windows::core::Result<IDWriteFontFile>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteFontFileLoader>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateCustomFontFileReference)(::windows::core::Vtable::as_raw(self), fontfilereferencekey, fontfilereferencekeysize, fontfileloader.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateFontFace(&self, fontfacetype: DWRITE_FONT_FACE_TYPE, fontfiles: &[IDWriteFontFile], faceindex: u32, fontfacesimulationflags: DWRITE_FONT_SIMULATIONS) -> ::windows::core::Result<IDWriteFontFace> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateFontFace)(::windows::core::Vtable::as_raw(self), fontfacetype, fontfiles.len() as _, ::core::mem::transmute(fontfiles.as_ptr()), faceindex, fontfacesimulationflags, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateRenderingParams(&self) -> ::windows::core::Result<IDWriteRenderingParams> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateRenderingParams)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn CreateMonitorRenderingParams<P0>(&self, monitor: P0) -> ::windows::core::Result<IDWriteRenderingParams>
    where
        P0: ::std::convert::Into<super::Gdi::HMONITOR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateMonitorRenderingParams)(::windows::core::Vtable::as_raw(self), monitor.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateCustomRenderingParams(&self, gamma: f32, enhancedcontrast: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE) -> ::windows::core::Result<IDWriteRenderingParams> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateCustomRenderingParams)(::windows::core::Vtable::as_raw(self), gamma, enhancedcontrast, cleartypelevel, pixelgeometry, renderingmode, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RegisterFontFileLoader<P0>(&self, fontfileloader: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteFontFileLoader>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.RegisterFontFileLoader)(::windows::core::Vtable::as_raw(self), fontfileloader.into().abi()).ok()
    }
    pub unsafe fn UnregisterFontFileLoader<P0>(&self, fontfileloader: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteFontFileLoader>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.UnregisterFontFileLoader)(::windows::core::Vtable::as_raw(self), fontfileloader.into().abi()).ok()
    }
    pub unsafe fn CreateTextFormat<P0, P1, P2>(&self, fontfamilyname: P0, fontcollection: P1, fontweight: DWRITE_FONT_WEIGHT, fontstyle: DWRITE_FONT_STYLE, fontstretch: DWRITE_FONT_STRETCH, fontsize: f32, localename: P2) -> ::windows::core::Result<IDWriteTextFormat>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<IDWriteFontCollection>>,
        P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateTextFormat)(::windows::core::Vtable::as_raw(self), fontfamilyname.into().abi(), fontcollection.into().abi(), fontweight, fontstyle, fontstretch, fontsize, localename.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateTypography(&self) -> ::windows::core::Result<IDWriteTypography> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateTypography)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetGdiInterop(&self) -> ::windows::core::Result<IDWriteGdiInterop> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetGdiInterop)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateTextLayout<P0>(&self, string: &[u16], textformat: P0, maxwidth: f32, maxheight: f32) -> ::windows::core::Result<IDWriteTextLayout>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteTextFormat>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateTextLayout)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(string.as_ptr()), string.len() as _, textformat.into().abi(), maxwidth, maxheight, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateGdiCompatibleTextLayout<P0, P1>(&self, string: &[u16], textformat: P0, layoutwidth: f32, layoutheight: f32, pixelsperdip: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, usegdinatural: P1) -> ::windows::core::Result<IDWriteTextLayout>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteTextFormat>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateGdiCompatibleTextLayout)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(string.as_ptr()), string.len() as _, textformat.into().abi(), layoutwidth, layoutheight, pixelsperdip, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), usegdinatural.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateEllipsisTrimmingSign<P0>(&self, textformat: P0) -> ::windows::core::Result<IDWriteInlineObject>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteTextFormat>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateEllipsisTrimmingSign)(::windows::core::Vtable::as_raw(self), textformat.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateTextAnalyzer(&self) -> ::windows::core::Result<IDWriteTextAnalyzer> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateTextAnalyzer)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateNumberSubstitution<P0, P1>(&self, substitutionmethod: DWRITE_NUMBER_SUBSTITUTION_METHOD, localename: P0, ignoreuseroverride: P1) -> ::windows::core::Result<IDWriteNumberSubstitution>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateNumberSubstitution)(::windows::core::Vtable::as_raw(self), substitutionmethod, localename.into().abi(), ignoreuseroverride.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateGlyphRunAnalysis(&self, glyphrun: *const DWRITE_GLYPH_RUN, pixelsperdip: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, renderingmode: DWRITE_RENDERING_MODE, measuringmode: DWRITE_MEASURING_MODE, baselineoriginx: f32, baselineoriginy: f32) -> ::windows::core::Result<IDWriteGlyphRunAnalysis> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateGlyphRunAnalysis)(::windows::core::Vtable::as_raw(self), glyphrun, pixelsperdip, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), renderingmode, measuringmode, baselineoriginx, baselineoriginy, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEudcFontCollection<P0>(&self, fontcollection: *mut ::core::option::Option<IDWriteFontCollection>, checkforupdates: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetEudcFontCollection)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(fontcollection), checkforupdates.into()).ok()
    }
    pub unsafe fn CreateCustomRenderingParams2(&self, gamma: f32, enhancedcontrast: f32, enhancedcontrastgrayscale: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE) -> ::windows::core::Result<IDWriteRenderingParams1> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateCustomRenderingParams2)(::windows::core::Vtable::as_raw(self), gamma, enhancedcontrast, enhancedcontrastgrayscale, cleartypelevel, pixelgeometry, renderingmode, result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IDWriteFactory3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDWriteFactory3 {}
impl ::core::fmt::Debug for IDWriteFactory3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDWriteFactory3").field(&self.0).finish()
    }
}
impl IDWriteFactory3 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSystemFontCollection<P0>(&self, fontcollection: *mut ::core::option::Option<IDWriteFontCollection>, checkforupdates: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetSystemFontCollection)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(fontcollection), checkforupdates.into()).ok()
    }
    pub unsafe fn CreateCustomFontCollection<P0>(&self, collectionloader: P0, collectionkey: *const ::core::ffi::c_void, collectionkeysize: u32) -> ::windows::core::Result<IDWriteFontCollection>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteFontCollectionLoader>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateCustomFontCollection)(::windows::core::Vtable::as_raw(self), collectionloader.into().abi(), collectionkey, collectionkeysize, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RegisterFontCollectionLoader<P0>(&self, fontcollectionloader: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteFontCollectionLoader>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.RegisterFontCollectionLoader)(::windows::core::Vtable::as_raw(self), fontcollectionloader.into().abi()).ok()
    }
    pub unsafe fn UnregisterFontCollectionLoader<P0>(&self, fontcollectionloader: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteFontCollectionLoader>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.UnregisterFontCollectionLoader)(::windows::core::Vtable::as_raw(self), fontcollectionloader.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateFontFileReference<P0>(&self, filepath: P0, lastwritetime: ::core::option::Option<*const super::super::Foundation::FILETIME>) -> ::windows::core::Result<IDWriteFontFile>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateFontFileReference)(::windows::core::Vtable::as_raw(self), filepath.into().abi(), ::core::mem::transmute(lastwritetime.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateCustomFontFileReference<P0>(&self, fontfilereferencekey: *const ::core::ffi::c_void, fontfilereferencekeysize: u32, fontfileloader: P0) -> ::windows::core::Result<IDWriteFontFile>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteFontFileLoader>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateCustomFontFileReference)(::windows::core::Vtable::as_raw(self), fontfilereferencekey, fontfilereferencekeysize, fontfileloader.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateFontFace(&self, fontfacetype: DWRITE_FONT_FACE_TYPE, fontfiles: &[IDWriteFontFile], faceindex: u32, fontfacesimulationflags: DWRITE_FONT_SIMULATIONS) -> ::windows::core::Result<IDWriteFontFace> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateFontFace)(::windows::core::Vtable::as_raw(self), fontfacetype, fontfiles.len() as _, ::core::mem::transmute(fontfiles.as_ptr()), faceindex, fontfacesimulationflags, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateRenderingParams(&self) -> ::windows::core::Result<IDWriteRenderingParams> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateRenderingParams)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn CreateMonitorRenderingParams<P0>(&self, monitor: P0) -> ::windows::core::Result<IDWriteRenderingParams>
    where
        P0: ::std::convert::Into<super::Gdi::HMONITOR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateMonitorRenderingParams)(::windows::core::Vtable::as_raw(self), monitor.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateCustomRenderingParams(&self, gamma: f32, enhancedcontrast: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE) -> ::windows::core::Result<IDWriteRenderingParams> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateCustomRenderingParams)(::windows::core::Vtable::as_raw(self), gamma, enhancedcontrast, cleartypelevel, pixelgeometry, renderingmode, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RegisterFontFileLoader<P0>(&self, fontfileloader: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteFontFileLoader>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.RegisterFontFileLoader)(::windows::core::Vtable::as_raw(self), fontfileloader.into().abi()).ok()
    }
    pub unsafe fn UnregisterFontFileLoader<P0>(&self, fontfileloader: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteFontFileLoader>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.UnregisterFontFileLoader)(::windows::core::Vtable::as_raw(self), fontfileloader.into().abi()).ok()
    }
    pub unsafe fn CreateTextFormat<P0, P1, P2>(&self, fontfamilyname: P0, fontcollection: P1, fontweight: DWRITE_FONT_WEIGHT, fontstyle: DWRITE_FONT_STYLE, fontstretch: DWRITE_FONT_STRETCH, fontsize: f32, localename: P2) -> ::windows::core::Result<IDWriteTextFormat>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<IDWriteFontCollection>>,
        P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateTextFormat)(::windows::core::Vtable::as_raw(self), fontfamilyname.into().abi(), fontcollection.into().abi(), fontweight, fontstyle, fontstretch, fontsize, localename.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateTypography(&self) -> ::windows::core::Result<IDWriteTypography> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateTypography)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetGdiInterop(&self) -> ::windows::core::Result<IDWriteGdiInterop> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetGdiInterop)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateTextLayout<P0>(&self, string: &[u16], textformat: P0, maxwidth: f32, maxheight: f32) -> ::windows::core::Result<IDWriteTextLayout>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteTextFormat>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateTextLayout)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(string.as_ptr()), string.len() as _, textformat.into().abi(), maxwidth, maxheight, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateGdiCompatibleTextLayout<P0, P1>(&self, string: &[u16], textformat: P0, layoutwidth: f32, layoutheight: f32, pixelsperdip: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, usegdinatural: P1) -> ::windows::core::Result<IDWriteTextLayout>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteTextFormat>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateGdiCompatibleTextLayout)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(string.as_ptr()), string.len() as _, textformat.into().abi(), layoutwidth, layoutheight, pixelsperdip, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), usegdinatural.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateEllipsisTrimmingSign<P0>(&self, textformat: P0) -> ::windows::core::Result<IDWriteInlineObject>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteTextFormat>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateEllipsisTrimmingSign)(::windows::core::Vtable::as_raw(self), textformat.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateTextAnalyzer(&self) -> ::windows::core::Result<IDWriteTextAnalyzer> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateTextAnalyzer)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateNumberSubstitution<P0, P1>(&self, substitutionmethod: DWRITE_NUMBER_SUBSTITUTION_METHOD, localename: P0, ignoreuseroverride: P1) -> ::windows::core::Result<IDWriteNumberSubstitution>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateNumberSubstitution)(::windows::core::Vtable::as_raw(self), substitutionmethod, localename.into().abi(), ignoreuseroverride.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateGlyphRunAnalysis(&self, glyphrun: *const DWRITE_GLYPH_RUN, pixelsperdip: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, renderingmode: DWRITE_RENDERING_MODE, measuringmode: DWRITE_MEASURING_MODE, baselineoriginx: f32, baselineoriginy: f32) -> ::windows::core::Result<IDWriteGlyphRunAnalysis> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateGlyphRunAnalysis)(::windows::core::Vtable::as_raw(self), glyphrun, pixelsperdip, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), renderingmode, measuringmode, baselineoriginx, baselineoriginy, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEudcFontCollection<P0>(&self, fontcollection: *mut ::core::option::Option<IDWriteFontCollection>, checkforupdates: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.GetEudcFontCollection)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(fontcollection), checkforupdates.into()).ok()
    }
    pub unsafe fn CreateCustomRenderingParams2(&self, gamma: f32, enhancedcontrast: f32, enhancedcontrastgrayscale: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE) -> ::windows::core::Result<IDWriteRenderingParams1> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateCustomRenderingParams2)(::windows::core::Vtable::as_raw(self), gamma, enhancedcontrast, enhancedcontrastgrayscale, cleartypelevel, pixelgeometry, renderingmode, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetSystemFontFallback(&self) -> ::windows::core::Result<IDWriteFontFallback> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetSystemFontFallback)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateFontFallbackBuilder(&self) -> ::windows::core::Result<IDWriteFontFallbackBuilder> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateFontFallbackBuilder)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TranslateColorGlyphRun(&self, baselineoriginx: f32, baselineoriginy: f32, glyphrun: *const DWRITE_GLYPH_RUN, glyphrundescription: ::core::option::Option<*const DWRITE_GLYPH_RUN_DESCRIPTION>, measuringmode: DWRITE_MEASURING_MODE, worldtodevicetransform: ::core::option::Option<*const DWRITE_MATRIX>, colorpaletteindex: u32) -> ::windows::core::Result<IDWriteColorGlyphRunEnumerator> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.TranslateColorGlyphRun)(::windows::core::Vtable::as_raw(self), baselineoriginx, baselineoriginy, glyphrun, ::core::mem::transmute(glyphrundescription.unwrap_or(::std::ptr::null())), measuringmode, ::core::mem::transmute(worldtodevicetransform.unwrap_or(::std::ptr::null())), colorpaletteindex, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateCustomRenderingParams3(&self, gamma: f32, enhancedcontrast: f32, grayscaleenhancedcontrast: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE, gridfitmode: DWRITE_GRID_FIT_MODE) -> ::windows::core::Result<IDWriteRenderingParams2> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateCustomRenderingParams3)(::windows::core::Vtable::as_raw(self), gamma, enhancedcontrast, grayscaleenhancedcontrast, cleartypelevel, pixelgeometry, renderingmode, gridfitmode, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateGlyphRunAnalysis2(&self, glyphrun: *const DWRITE_GLYPH_RUN, transform: ::core::option::Option<*const DWRITE_MATRIX>, renderingmode: DWRITE_RENDERING_MODE, measuringmode: DWRITE_MEASURING_MODE, gridfitmode: DWRITE_GRID_FIT_MODE, antialiasmode: DWRITE_TEXT_ANTIALIAS_MODE, baselineoriginx: f32, baselineoriginy: f32) -> ::windows::core::Result<IDWriteGlyphRunAnalysis> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateGlyphRunAnalysis2)(::windows::core::Vtable::as_raw(self), glyphrun, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), renderingmode, measuringmode, gridfitmode, antialiasmode, baselineoriginx, baselineoriginy, result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IDWriteFactory4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDWriteFactory4 {}
impl ::core::fmt::Debug for IDWriteFactory4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDWriteFactory4").field(&self.0).finish()
    }
}
impl IDWriteFactory4 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSystemFontCollection<P0>(&self, fontcollection: *mut ::core::option::Option<IDWriteFontCollection>, checkforupdates: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetSystemFontCollection)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(fontcollection), checkforupdates.into()).ok()
    }
    pub unsafe fn CreateCustomFontCollection<P0>(&self, collectionloader: P0, collectionkey: *const ::core::ffi::c_void, collectionkeysize: u32) -> ::windows::core::Result<IDWriteFontCollection>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteFontCollectionLoader>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateCustomFontCollection)(::windows::core::Vtable::as_raw(self), collectionloader.into().abi(), collectionkey, collectionkeysize, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RegisterFontCollectionLoader<P0>(&self, fontcollectionloader: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteFontCollectionLoader>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.RegisterFontCollectionLoader)(::windows::core::Vtable::as_raw(self), fontcollectionloader.into().abi()).ok()
    }
    pub unsafe fn UnregisterFontCollectionLoader<P0>(&self, fontcollectionloader: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteFontCollectionLoader>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.UnregisterFontCollectionLoader)(::windows::core::Vtable::as_raw(self), fontcollectionloader.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateFontFileReference<P0>(&self, filepath: P0, lastwritetime: ::core::option::Option<*const super::super::Foundation::FILETIME>) -> ::windows::core::Result<IDWriteFontFile>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateFontFileReference)(::windows::core::Vtable::as_raw(self), filepath.into().abi(), ::core::mem::transmute(lastwritetime.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateCustomFontFileReference<P0>(&self, fontfilereferencekey: *const ::core::ffi::c_void, fontfilereferencekeysize: u32, fontfileloader: P0) -> ::windows::core::Result<IDWriteFontFile>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteFontFileLoader>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateCustomFontFileReference)(::windows::core::Vtable::as_raw(self), fontfilereferencekey, fontfilereferencekeysize, fontfileloader.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateFontFace(&self, fontfacetype: DWRITE_FONT_FACE_TYPE, fontfiles: &[IDWriteFontFile], faceindex: u32, fontfacesimulationflags: DWRITE_FONT_SIMULATIONS) -> ::windows::core::Result<IDWriteFontFace> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateFontFace)(::windows::core::Vtable::as_raw(self), fontfacetype, fontfiles.len() as _, ::core::mem::transmute(fontfiles.as_ptr()), faceindex, fontfacesimulationflags, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateRenderingParams(&self) -> ::windows::core::Result<IDWriteRenderingParams> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateRenderingParams)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn CreateMonitorRenderingParams<P0>(&self, monitor: P0) -> ::windows::core::Result<IDWriteRenderingParams>
    where
        P0: ::std::convert::Into<super::Gdi::HMONITOR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateMonitorRenderingParams)(::windows::core::Vtable::as_raw(self), monitor.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateCustomRenderingParams(&self, gamma: f32, enhancedcontrast: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE) -> ::windows::core::Result<IDWriteRenderingParams> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateCustomRenderingParams)(::windows::core::Vtable::as_raw(self), gamma, enhancedcontrast, cleartypelevel, pixelgeometry, renderingmode, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RegisterFontFileLoader<P0>(&self, fontfileloader: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteFontFileLoader>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.RegisterFontFileLoader)(::windows::core::Vtable::as_raw(self), fontfileloader.into().abi()).ok()
    }
    pub unsafe fn UnregisterFontFileLoader<P0>(&self, fontfileloader: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteFontFileLoader>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.UnregisterFontFileLoader)(::windows::core::Vtable::as_raw(self), fontfileloader.into().abi()).ok()
    }
    pub unsafe fn CreateTextFormat<P0, P1, P2>(&self, fontfamilyname: P0, fontcollection: P1, fontweight: DWRITE_FONT_WEIGHT, fontstyle: DWRITE_FONT_STYLE, fontstretch: DWRITE_FONT_STRETCH, fontsize: f32, localename: P2) -> ::windows::core::Result<IDWriteTextFormat>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<IDWriteFontCollection>>,
        P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateTextFormat)(::windows::core::Vtable::as_raw(self), fontfamilyname.into().abi(), fontcollection.into().abi(), fontweight, fontstyle, fontstretch, fontsize, localename.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateTypography(&self) -> ::windows::core::Result<IDWriteTypography> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateTypography)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetGdiInterop(&self) -> ::windows::core::Result<IDWriteGdiInterop> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetGdiInterop)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateTextLayout<P0>(&self, string: &[u16], textformat: P0, maxwidth: f32, maxheight: f32) -> ::windows::core::Result<IDWriteTextLayout>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteTextFormat>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateTextLayout)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(string.as_ptr()), string.len() as _, textformat.into().abi(), maxwidth, maxheight, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateGdiCompatibleTextLayout<P0, P1>(&self, string: &[u16], textformat: P0, layoutwidth: f32, layoutheight: f32, pixelsperdip: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, usegdinatural: P1) -> ::windows::core::Result<IDWriteTextLayout>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteTextFormat>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateGdiCompatibleTextLayout)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(string.as_ptr()), string.len() as _, textformat.into().abi(), layoutwidth, layoutheight, pixelsperdip, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), usegdinatural.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateEllipsisTrimmingSign<P0>(&self, textformat: P0) -> ::windows::core::Result<IDWriteInlineObject>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteTextFormat>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateEllipsisTrimmingSign)(::windows::core::Vtable::as_raw(self), textformat.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateTextAnalyzer(&self) -> ::windows::core::Result<IDWriteTextAnalyzer> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateTextAnalyzer)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateNumberSubstitution<P0, P1>(&self, substitutionmethod: DWRITE_NUMBER_SUBSTITUTION_METHOD, localename: P0, ignoreuseroverride: P1) -> ::windows::core::Result<IDWriteNumberSubstitution>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateNumberSubstitution)(::windows::core::Vtable::as_raw(self), substitutionmethod, localename.into().abi(), ignoreuseroverride.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateGlyphRunAnalysis(&self, glyphrun: *const DWRITE_GLYPH_RUN, pixelsperdip: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, renderingmode: DWRITE_RENDERING_MODE, measuringmode: DWRITE_MEASURING_MODE, baselineoriginx: f32, baselineoriginy: f32) -> ::windows::core::Result<IDWriteGlyphRunAnalysis> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateGlyphRunAnalysis)(::windows::core::Vtable::as_raw(self), glyphrun, pixelsperdip, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), renderingmode, measuringmode, baselineoriginx, baselineoriginy, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEudcFontCollection<P0>(&self, fontcollection: *mut ::core::option::Option<IDWriteFontCollection>, checkforupdates: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetEudcFontCollection)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(fontcollection), checkforupdates.into()).ok()
    }
    pub unsafe fn CreateCustomRenderingParams2(&self, gamma: f32, enhancedcontrast: f32, enhancedcontrastgrayscale: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE) -> ::windows::core::Result<IDWriteRenderingParams1> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateCustomRenderingParams2)(::windows::core::Vtable::as_raw(self), gamma, enhancedcontrast, enhancedcontrastgrayscale, cleartypelevel, pixelgeometry, renderingmode, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetSystemFontFallback(&self) -> ::windows::core::Result<IDWriteFontFallback> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetSystemFontFallback)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateFontFallbackBuilder(&self) -> ::windows::core::Result<IDWriteFontFallbackBuilder> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateFontFallbackBuilder)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TranslateColorGlyphRun(&self, baselineoriginx: f32, baselineoriginy: f32, glyphrun: *const DWRITE_GLYPH_RUN, glyphrundescription: ::core::option::Option<*const DWRITE_GLYPH_RUN_DESCRIPTION>, measuringmode: DWRITE_MEASURING_MODE, worldtodevicetransform: ::core::option::Option<*const DWRITE_MATRIX>, colorpaletteindex: u32) -> ::windows::core::Result<IDWriteColorGlyphRunEnumerator> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.TranslateColorGlyphRun)(::windows::core::Vtable::as_raw(self), baselineoriginx, baselineoriginy, glyphrun, ::core::mem::transmute(glyphrundescription.unwrap_or(::std::ptr::null())), measuringmode, ::core::mem::transmute(worldtodevicetransform.unwrap_or(::std::ptr::null())), colorpaletteindex, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateCustomRenderingParams3(&self, gamma: f32, enhancedcontrast: f32, grayscaleenhancedcontrast: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE, gridfitmode: DWRITE_GRID_FIT_MODE) -> ::windows::core::Result<IDWriteRenderingParams2> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateCustomRenderingParams3)(::windows::core::Vtable::as_raw(self), gamma, enhancedcontrast, grayscaleenhancedcontrast, cleartypelevel, pixelgeometry, renderingmode, gridfitmode, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateGlyphRunAnalysis2(&self, glyphrun: *const DWRITE_GLYPH_RUN, transform: ::core::option::Option<*const DWRITE_MATRIX>, renderingmode: DWRITE_RENDERING_MODE, measuringmode: DWRITE_MEASURING_MODE, gridfitmode: DWRITE_GRID_FIT_MODE, antialiasmode: DWRITE_TEXT_ANTIALIAS_MODE, baselineoriginx: f32, baselineoriginy: f32) -> ::windows::core::Result<IDWriteGlyphRunAnalysis> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateGlyphRunAnalysis2)(::windows::core::Vtable::as_raw(self), glyphrun, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), renderingmode, measuringmode, gridfitmode, antialiasmode, baselineoriginx, baselineoriginy, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateGlyphRunAnalysis3(&self, glyphrun: *const DWRITE_GLYPH_RUN, transform: ::core::option::Option<*const DWRITE_MATRIX>, renderingmode: DWRITE_RENDERING_MODE1, measuringmode: DWRITE_MEASURING_MODE, gridfitmode: DWRITE_GRID_FIT_MODE, antialiasmode: DWRITE_TEXT_ANTIALIAS_MODE, baselineoriginx: f32, baselineoriginy: f32) -> ::windows::core::Result<IDWriteGlyphRunAnalysis> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateGlyphRunAnalysis3)(::windows::core::Vtable::as_raw(self), glyphrun, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), renderingmode, measuringmode, gridfitmode, antialiasmode, baselineoriginx, baselineoriginy, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateCustomRenderingParams4(&self, gamma: f32, enhancedcontrast: f32, grayscaleenhancedcontrast: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE1, gridfitmode: DWRITE_GRID_FIT_MODE) -> ::windows::core::Result<IDWriteRenderingParams3> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateCustomRenderingParams4)(::windows::core::Vtable::as_raw(self), gamma, enhancedcontrast, grayscaleenhancedcontrast, cleartypelevel, pixelgeometry, renderingmode, gridfitmode, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateFontFaceReference<P0>(&self, fontfile: P0, faceindex: u32, fontsimulations: DWRITE_FONT_SIMULATIONS) -> ::windows::core::Result<IDWriteFontFaceReference>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteFontFile>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateFontFaceReference)(::windows::core::Vtable::as_raw(self), fontfile.into().abi(), faceindex, fontsimulations, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateFontFaceReference2<P0>(&self, filepath: P0, lastwritetime: ::core::option::Option<*const super::super::Foundation::FILETIME>, faceindex: u32, fontsimulations: DWRITE_FONT_SIMULATIONS) -> ::windows::core::Result<IDWriteFontFaceReference>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateFontFaceReference2)(::windows::core::Vtable::as_raw(self), filepath.into().abi(), ::core::mem::transmute(lastwritetime.unwrap_or(::std::ptr::null())), faceindex, fontsimulations, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetSystemFontSet(&self) -> ::windows::core::Result<IDWriteFontSet> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetSystemFontSet)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateFontSetBuilder(&self) -> ::windows::core::Result<IDWriteFontSetBuilder> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateFontSetBuilder)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateFontCollectionFromFontSet<P0>(&self, fontset: P0) -> ::windows::core::Result<IDWriteFontCollection1>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteFontSet>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateFontCollectionFromFontSet)(::windows::core::Vtable::as_raw(self), fontset.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSystemFontCollection2<P0, P1>(&self, includedownloadablefonts: P0, fontcollection: *mut ::core::option::Option<IDWriteFontCollection1>, checkforupdates: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetSystemFontCollection2)(::windows::core::Vtable::as_raw(self), includedownloadablefonts.into(), ::core::mem::transmute(fontcollection), checkforupdates.into()).ok()
    }
    pub unsafe fn GetFontDownloadQueue(&self) -> ::windows::core::Result<IDWriteFontDownloadQueue> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFontDownloadQueue)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IDWriteFactory5 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDWriteFactory5 {}
impl ::core::fmt::Debug for IDWriteFactory5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDWriteFactory5").field(&self.0).finish()
    }
}
impl IDWriteFactory5 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSystemFontCollection<P0>(&self, fontcollection: *mut ::core::option::Option<IDWriteFontCollection>, checkforupdates: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetSystemFontCollection)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(fontcollection), checkforupdates.into()).ok()
    }
    pub unsafe fn CreateCustomFontCollection<P0>(&self, collectionloader: P0, collectionkey: *const ::core::ffi::c_void, collectionkeysize: u32) -> ::windows::core::Result<IDWriteFontCollection>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteFontCollectionLoader>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateCustomFontCollection)(::windows::core::Vtable::as_raw(self), collectionloader.into().abi(), collectionkey, collectionkeysize, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RegisterFontCollectionLoader<P0>(&self, fontcollectionloader: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteFontCollectionLoader>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.RegisterFontCollectionLoader)(::windows::core::Vtable::as_raw(self), fontcollectionloader.into().abi()).ok()
    }
    pub unsafe fn UnregisterFontCollectionLoader<P0>(&self, fontcollectionloader: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteFontCollectionLoader>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.UnregisterFontCollectionLoader)(::windows::core::Vtable::as_raw(self), fontcollectionloader.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateFontFileReference<P0>(&self, filepath: P0, lastwritetime: ::core::option::Option<*const super::super::Foundation::FILETIME>) -> ::windows::core::Result<IDWriteFontFile>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateFontFileReference)(::windows::core::Vtable::as_raw(self), filepath.into().abi(), ::core::mem::transmute(lastwritetime.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateCustomFontFileReference<P0>(&self, fontfilereferencekey: *const ::core::ffi::c_void, fontfilereferencekeysize: u32, fontfileloader: P0) -> ::windows::core::Result<IDWriteFontFile>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteFontFileLoader>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateCustomFontFileReference)(::windows::core::Vtable::as_raw(self), fontfilereferencekey, fontfilereferencekeysize, fontfileloader.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateFontFace(&self, fontfacetype: DWRITE_FONT_FACE_TYPE, fontfiles: &[IDWriteFontFile], faceindex: u32, fontfacesimulationflags: DWRITE_FONT_SIMULATIONS) -> ::windows::core::Result<IDWriteFontFace> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateFontFace)(::windows::core::Vtable::as_raw(self), fontfacetype, fontfiles.len() as _, ::core::mem::transmute(fontfiles.as_ptr()), faceindex, fontfacesimulationflags, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateRenderingParams(&self) -> ::windows::core::Result<IDWriteRenderingParams> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateRenderingParams)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn CreateMonitorRenderingParams<P0>(&self, monitor: P0) -> ::windows::core::Result<IDWriteRenderingParams>
    where
        P0: ::std::convert::Into<super::Gdi::HMONITOR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateMonitorRenderingParams)(::windows::core::Vtable::as_raw(self), monitor.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateCustomRenderingParams(&self, gamma: f32, enhancedcontrast: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE) -> ::windows::core::Result<IDWriteRenderingParams> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateCustomRenderingParams)(::windows::core::Vtable::as_raw(self), gamma, enhancedcontrast, cleartypelevel, pixelgeometry, renderingmode, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RegisterFontFileLoader<P0>(&self, fontfileloader: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteFontFileLoader>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.RegisterFontFileLoader)(::windows::core::Vtable::as_raw(self), fontfileloader.into().abi()).ok()
    }
    pub unsafe fn UnregisterFontFileLoader<P0>(&self, fontfileloader: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteFontFileLoader>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.UnregisterFontFileLoader)(::windows::core::Vtable::as_raw(self), fontfileloader.into().abi()).ok()
    }
    pub unsafe fn CreateTextFormat<P0, P1, P2>(&self, fontfamilyname: P0, fontcollection: P1, fontweight: DWRITE_FONT_WEIGHT, fontstyle: DWRITE_FONT_STYLE, fontstretch: DWRITE_FONT_STRETCH, fontsize: f32, localename: P2) -> ::windows::core::Result<IDWriteTextFormat>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<IDWriteFontCollection>>,
        P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateTextFormat)(::windows::core::Vtable::as_raw(self), fontfamilyname.into().abi(), fontcollection.into().abi(), fontweight, fontstyle, fontstretch, fontsize, localename.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateTypography(&self) -> ::windows::core::Result<IDWriteTypography> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateTypography)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetGdiInterop(&self) -> ::windows::core::Result<IDWriteGdiInterop> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetGdiInterop)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateTextLayout<P0>(&self, string: &[u16], textformat: P0, maxwidth: f32, maxheight: f32) -> ::windows::core::Result<IDWriteTextLayout>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteTextFormat>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateTextLayout)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(string.as_ptr()), string.len() as _, textformat.into().abi(), maxwidth, maxheight, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateGdiCompatibleTextLayout<P0, P1>(&self, string: &[u16], textformat: P0, layoutwidth: f32, layoutheight: f32, pixelsperdip: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, usegdinatural: P1) -> ::windows::core::Result<IDWriteTextLayout>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteTextFormat>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateGdiCompatibleTextLayout)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(string.as_ptr()), string.len() as _, textformat.into().abi(), layoutwidth, layoutheight, pixelsperdip, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), usegdinatural.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateEllipsisTrimmingSign<P0>(&self, textformat: P0) -> ::windows::core::Result<IDWriteInlineObject>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteTextFormat>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateEllipsisTrimmingSign)(::windows::core::Vtable::as_raw(self), textformat.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateTextAnalyzer(&self) -> ::windows::core::Result<IDWriteTextAnalyzer> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateTextAnalyzer)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateNumberSubstitution<P0, P1>(&self, substitutionmethod: DWRITE_NUMBER_SUBSTITUTION_METHOD, localename: P0, ignoreuseroverride: P1) -> ::windows::core::Result<IDWriteNumberSubstitution>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateNumberSubstitution)(::windows::core::Vtable::as_raw(self), substitutionmethod, localename.into().abi(), ignoreuseroverride.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateGlyphRunAnalysis(&self, glyphrun: *const DWRITE_GLYPH_RUN, pixelsperdip: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, renderingmode: DWRITE_RENDERING_MODE, measuringmode: DWRITE_MEASURING_MODE, baselineoriginx: f32, baselineoriginy: f32) -> ::windows::core::Result<IDWriteGlyphRunAnalysis> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateGlyphRunAnalysis)(::windows::core::Vtable::as_raw(self), glyphrun, pixelsperdip, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), renderingmode, measuringmode, baselineoriginx, baselineoriginy, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEudcFontCollection<P0>(&self, fontcollection: *mut ::core::option::Option<IDWriteFontCollection>, checkforupdates: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetEudcFontCollection)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(fontcollection), checkforupdates.into()).ok()
    }
    pub unsafe fn CreateCustomRenderingParams2(&self, gamma: f32, enhancedcontrast: f32, enhancedcontrastgrayscale: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE) -> ::windows::core::Result<IDWriteRenderingParams1> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateCustomRenderingParams2)(::windows::core::Vtable::as_raw(self), gamma, enhancedcontrast, enhancedcontrastgrayscale, cleartypelevel, pixelgeometry, renderingmode, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetSystemFontFallback(&self) -> ::windows::core::Result<IDWriteFontFallback> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetSystemFontFallback)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateFontFallbackBuilder(&self) -> ::windows::core::Result<IDWriteFontFallbackBuilder> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateFontFallbackBuilder)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TranslateColorGlyphRun(&self, baselineoriginx: f32, baselineoriginy: f32, glyphrun: *const DWRITE_GLYPH_RUN, glyphrundescription: ::core::option::Option<*const DWRITE_GLYPH_RUN_DESCRIPTION>, measuringmode: DWRITE_MEASURING_MODE, worldtodevicetransform: ::core::option::Option<*const DWRITE_MATRIX>, colorpaletteindex: u32) -> ::windows::core::Result<IDWriteColorGlyphRunEnumerator> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.TranslateColorGlyphRun)(::windows::core::Vtable::as_raw(self), baselineoriginx, baselineoriginy, glyphrun, ::core::mem::transmute(glyphrundescription.unwrap_or(::std::ptr::null())), measuringmode, ::core::mem::transmute(worldtodevicetransform.unwrap_or(::std::ptr::null())), colorpaletteindex, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateCustomRenderingParams3(&self, gamma: f32, enhancedcontrast: f32, grayscaleenhancedcontrast: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE, gridfitmode: DWRITE_GRID_FIT_MODE) -> ::windows::core::Result<IDWriteRenderingParams2> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateCustomRenderingParams3)(::windows::core::Vtable::as_raw(self), gamma, enhancedcontrast, grayscaleenhancedcontrast, cleartypelevel, pixelgeometry, renderingmode, gridfitmode, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateGlyphRunAnalysis2(&self, glyphrun: *const DWRITE_GLYPH_RUN, transform: ::core::option::Option<*const DWRITE_MATRIX>, renderingmode: DWRITE_RENDERING_MODE, measuringmode: DWRITE_MEASURING_MODE, gridfitmode: DWRITE_GRID_FIT_MODE, antialiasmode: DWRITE_TEXT_ANTIALIAS_MODE, baselineoriginx: f32, baselineoriginy: f32) -> ::windows::core::Result<IDWriteGlyphRunAnalysis> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateGlyphRunAnalysis2)(::windows::core::Vtable::as_raw(self), glyphrun, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), renderingmode, measuringmode, gridfitmode, antialiasmode, baselineoriginx, baselineoriginy, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateGlyphRunAnalysis3(&self, glyphrun: *const DWRITE_GLYPH_RUN, transform: ::core::option::Option<*const DWRITE_MATRIX>, renderingmode: DWRITE_RENDERING_MODE1, measuringmode: DWRITE_MEASURING_MODE, gridfitmode: DWRITE_GRID_FIT_MODE, antialiasmode: DWRITE_TEXT_ANTIALIAS_MODE, baselineoriginx: f32, baselineoriginy: f32) -> ::windows::core::Result<IDWriteGlyphRunAnalysis> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateGlyphRunAnalysis3)(::windows::core::Vtable::as_raw(self), glyphrun, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), renderingmode, measuringmode, gridfitmode, antialiasmode, baselineoriginx, baselineoriginy, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateCustomRenderingParams4(&self, gamma: f32, enhancedcontrast: f32, grayscaleenhancedcontrast: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE1, gridfitmode: DWRITE_GRID_FIT_MODE) -> ::windows::core::Result<IDWriteRenderingParams3> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateCustomRenderingParams4)(::windows::core::Vtable::as_raw(self), gamma, enhancedcontrast, grayscaleenhancedcontrast, cleartypelevel, pixelgeometry, renderingmode, gridfitmode, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateFontFaceReference<P0>(&self, fontfile: P0, faceindex: u32, fontsimulations: DWRITE_FONT_SIMULATIONS) -> ::windows::core::Result<IDWriteFontFaceReference>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteFontFile>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateFontFaceReference)(::windows::core::Vtable::as_raw(self), fontfile.into().abi(), faceindex, fontsimulations, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateFontFaceReference2<P0>(&self, filepath: P0, lastwritetime: ::core::option::Option<*const super::super::Foundation::FILETIME>, faceindex: u32, fontsimulations: DWRITE_FONT_SIMULATIONS) -> ::windows::core::Result<IDWriteFontFaceReference>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateFontFaceReference2)(::windows::core::Vtable::as_raw(self), filepath.into().abi(), ::core::mem::transmute(lastwritetime.unwrap_or(::std::ptr::null())), faceindex, fontsimulations, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetSystemFontSet(&self) -> ::windows::core::Result<IDWriteFontSet> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetSystemFontSet)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateFontSetBuilder(&self) -> ::windows::core::Result<IDWriteFontSetBuilder> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateFontSetBuilder)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateFontCollectionFromFontSet<P0>(&self, fontset: P0) -> ::windows::core::Result<IDWriteFontCollection1>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteFontSet>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateFontCollectionFromFontSet)(::windows::core::Vtable::as_raw(self), fontset.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSystemFontCollection2<P0, P1>(&self, includedownloadablefonts: P0, fontcollection: *mut ::core::option::Option<IDWriteFontCollection1>, checkforupdates: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.GetSystemFontCollection2)(::windows::core::Vtable::as_raw(self), includedownloadablefonts.into(), ::core::mem::transmute(fontcollection), checkforupdates.into()).ok()
    }
    pub unsafe fn GetFontDownloadQueue(&self) -> ::windows::core::Result<IDWriteFontDownloadQueue> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetFontDownloadQueue)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn TranslateColorGlyphRun2(&self, baselineorigin: super::Direct2D::Common::D2D_POINT_2F, glyphrun: *const DWRITE_GLYPH_RUN, glyphrundescription: ::core::option::Option<*const DWRITE_GLYPH_RUN_DESCRIPTION>, desiredglyphimageformats: DWRITE_GLYPH_IMAGE_FORMATS, measuringmode: DWRITE_MEASURING_MODE, worldanddpitransform: ::core::option::Option<*const DWRITE_MATRIX>, colorpaletteindex: u32) -> ::windows::core::Result<IDWriteColorGlyphRunEnumerator1> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.TranslateColorGlyphRun2)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(baselineorigin), glyphrun, ::core::mem::transmute(glyphrundescription.unwrap_or(::std::ptr::null())), desiredglyphimageformats, measuringmode, ::core::mem::transmute(worldanddpitransform.unwrap_or(::std::ptr::null())), colorpaletteindex, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn ComputeGlyphOrigins(&self, glyphrun: *const DWRITE_GLYPH_RUN, baselineorigin: super::Direct2D::Common::D2D_POINT_2F) -> ::windows::core::Result<super::Direct2D::Common::D2D_POINT_2F> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ComputeGlyphOrigins)(::windows::core::Vtable::as_raw(self), glyphrun, ::core::mem::transmute(baselineorigin), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn ComputeGlyphOrigins2(&self, glyphrun: *const DWRITE_GLYPH_RUN, measuringmode: DWRITE_MEASURING_MODE, baselineorigin: super::Direct2D::Common::D2D_POINT_2F, worldanddpitransform: ::core::option::Option<*const DWRITE_MATRIX>) -> ::windows::core::Result<super::Direct2D::Common::D2D_POINT_2F> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ComputeGlyphOrigins2)(::windows::core::Vtable::as_raw(self), glyphrun, measuringmode, ::core::mem::transmute(baselineorigin), ::core::mem::transmute(worldanddpitransform.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IDWriteFactory6 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDWriteFactory6 {}
impl ::core::fmt::Debug for IDWriteFactory6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDWriteFactory6").field(&self.0).finish()
    }
}
impl IDWriteFactory6 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSystemFontCollection<P0>(&self, fontcollection: *mut ::core::option::Option<IDWriteFontCollection>, checkforupdates: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.GetSystemFontCollection)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(fontcollection), checkforupdates.into()).ok()
    }
    pub unsafe fn CreateCustomFontCollection<P0>(&self, collectionloader: P0, collectionkey: *const ::core::ffi::c_void, collectionkeysize: u32) -> ::windows::core::Result<IDWriteFontCollection>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteFontCollectionLoader>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CreateCustomFontCollection)(::windows::core::Vtable::as_raw(self), collectionloader.into().abi(), collectionkey, collectionkeysize, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RegisterFontCollectionLoader<P0>(&self, fontcollectionloader: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteFontCollectionLoader>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.RegisterFontCollectionLoader)(::windows::core::Vtable::as_raw(self), fontcollectionloader.into().abi()).ok()
    }
    pub unsafe fn UnregisterFontCollectionLoader<P0>(&self, fontcollectionloader: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteFontCollectionLoader>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.UnregisterFontCollectionLoader)(::windows::core::Vtable::as_raw(self), fontcollectionloader.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateFontFileReference<P0>(&self, filepath: P0, lastwritetime: ::core::option::Option<*const super::super::Foundation::FILETIME>) -> ::windows::core::Result<IDWriteFontFile>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CreateFontFileReference)(::windows::core::Vtable::as_raw(self), filepath.into().abi(), ::core::mem::transmute(lastwritetime.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateCustomFontFileReference<P0>(&self, fontfilereferencekey: *const ::core::ffi::c_void, fontfilereferencekeysize: u32, fontfileloader: P0) -> ::windows::core::Result<IDWriteFontFile>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteFontFileLoader>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CreateCustomFontFileReference)(::windows::core::Vtable::as_raw(self), fontfilereferencekey, fontfilereferencekeysize, fontfileloader.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateFontFace(&self, fontfacetype: DWRITE_FONT_FACE_TYPE, fontfiles: &[IDWriteFontFile], faceindex: u32, fontfacesimulationflags: DWRITE_FONT_SIMULATIONS) -> ::windows::core::Result<IDWriteFontFace> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CreateFontFace)(::windows::core::Vtable::as_raw(self), fontfacetype, fontfiles.len() as _, ::core::mem::transmute(fontfiles.as_ptr()), faceindex, fontfacesimulationflags, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateRenderingParams(&self) -> ::windows::core::Result<IDWriteRenderingParams> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CreateRenderingParams)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn CreateMonitorRenderingParams<P0>(&self, monitor: P0) -> ::windows::core::Result<IDWriteRenderingParams>
    where
        P0: ::std::convert::Into<super::Gdi::HMONITOR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CreateMonitorRenderingParams)(::windows::core::Vtable::as_raw(self), monitor.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateCustomRenderingParams(&self, gamma: f32, enhancedcontrast: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE) -> ::windows::core::Result<IDWriteRenderingParams> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CreateCustomRenderingParams)(::windows::core::Vtable::as_raw(self), gamma, enhancedcontrast, cleartypelevel, pixelgeometry, renderingmode, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RegisterFontFileLoader<P0>(&self, fontfileloader: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteFontFileLoader>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.RegisterFontFileLoader)(::windows::core::Vtable::as_raw(self), fontfileloader.into().abi()).ok()
    }
    pub unsafe fn UnregisterFontFileLoader<P0>(&self, fontfileloader: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteFontFileLoader>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.UnregisterFontFileLoader)(::windows::core::Vtable::as_raw(self), fontfileloader.into().abi()).ok()
    }
    pub unsafe fn CreateTextFormat<P0, P1, P2>(&self, fontfamilyname: P0, fontcollection: P1, fontweight: DWRITE_FONT_WEIGHT, fontstyle: DWRITE_FONT_STYLE, fontstretch: DWRITE_FONT_STRETCH, fontsize: f32, localename: P2) -> ::windows::core::Result<IDWriteTextFormat>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<IDWriteFontCollection>>,
        P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CreateTextFormat)(::windows::core::Vtable::as_raw(self), fontfamilyname.into().abi(), fontcollection.into().abi(), fontweight, fontstyle, fontstretch, fontsize, localename.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateTypography(&self) -> ::windows::core::Result<IDWriteTypography> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CreateTypography)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetGdiInterop(&self) -> ::windows::core::Result<IDWriteGdiInterop> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.GetGdiInterop)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateTextLayout<P0>(&self, string: &[u16], textformat: P0, maxwidth: f32, maxheight: f32) -> ::windows::core::Result<IDWriteTextLayout>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteTextFormat>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CreateTextLayout)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(string.as_ptr()), string.len() as _, textformat.into().abi(), maxwidth, maxheight, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateGdiCompatibleTextLayout<P0, P1>(&self, string: &[u16], textformat: P0, layoutwidth: f32, layoutheight: f32, pixelsperdip: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, usegdinatural: P1) -> ::windows::core::Result<IDWriteTextLayout>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteTextFormat>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CreateGdiCompatibleTextLayout)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(string.as_ptr()), string.len() as _, textformat.into().abi(), layoutwidth, layoutheight, pixelsperdip, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), usegdinatural.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateEllipsisTrimmingSign<P0>(&self, textformat: P0) -> ::windows::core::Result<IDWriteInlineObject>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteTextFormat>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CreateEllipsisTrimmingSign)(::windows::core::Vtable::as_raw(self), textformat.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateTextAnalyzer(&self) -> ::windows::core::Result<IDWriteTextAnalyzer> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CreateTextAnalyzer)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateNumberSubstitution<P0, P1>(&self, substitutionmethod: DWRITE_NUMBER_SUBSTITUTION_METHOD, localename: P0, ignoreuseroverride: P1) -> ::windows::core::Result<IDWriteNumberSubstitution>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CreateNumberSubstitution)(::windows::core::Vtable::as_raw(self), substitutionmethod, localename.into().abi(), ignoreuseroverride.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateGlyphRunAnalysis(&self, glyphrun: *const DWRITE_GLYPH_RUN, pixelsperdip: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, renderingmode: DWRITE_RENDERING_MODE, measuringmode: DWRITE_MEASURING_MODE, baselineoriginx: f32, baselineoriginy: f32) -> ::windows::core::Result<IDWriteGlyphRunAnalysis> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CreateGlyphRunAnalysis)(::windows::core::Vtable::as_raw(self), glyphrun, pixelsperdip, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), renderingmode, measuringmode, baselineoriginx, baselineoriginy, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEudcFontCollection<P0>(&self, fontcollection: *mut ::core::option::Option<IDWriteFontCollection>, checkforupdates: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetEudcFontCollection)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(fontcollection), checkforupdates.into()).ok()
    }
    pub unsafe fn CreateCustomRenderingParams2(&self, gamma: f32, enhancedcontrast: f32, enhancedcontrastgrayscale: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE) -> ::windows::core::Result<IDWriteRenderingParams1> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateCustomRenderingParams2)(::windows::core::Vtable::as_raw(self), gamma, enhancedcontrast, enhancedcontrastgrayscale, cleartypelevel, pixelgeometry, renderingmode, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetSystemFontFallback(&self) -> ::windows::core::Result<IDWriteFontFallback> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetSystemFontFallback)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateFontFallbackBuilder(&self) -> ::windows::core::Result<IDWriteFontFallbackBuilder> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateFontFallbackBuilder)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TranslateColorGlyphRun(&self, baselineoriginx: f32, baselineoriginy: f32, glyphrun: *const DWRITE_GLYPH_RUN, glyphrundescription: ::core::option::Option<*const DWRITE_GLYPH_RUN_DESCRIPTION>, measuringmode: DWRITE_MEASURING_MODE, worldtodevicetransform: ::core::option::Option<*const DWRITE_MATRIX>, colorpaletteindex: u32) -> ::windows::core::Result<IDWriteColorGlyphRunEnumerator> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.TranslateColorGlyphRun)(::windows::core::Vtable::as_raw(self), baselineoriginx, baselineoriginy, glyphrun, ::core::mem::transmute(glyphrundescription.unwrap_or(::std::ptr::null())), measuringmode, ::core::mem::transmute(worldtodevicetransform.unwrap_or(::std::ptr::null())), colorpaletteindex, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateCustomRenderingParams3(&self, gamma: f32, enhancedcontrast: f32, grayscaleenhancedcontrast: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE, gridfitmode: DWRITE_GRID_FIT_MODE) -> ::windows::core::Result<IDWriteRenderingParams2> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateCustomRenderingParams3)(::windows::core::Vtable::as_raw(self), gamma, enhancedcontrast, grayscaleenhancedcontrast, cleartypelevel, pixelgeometry, renderingmode, gridfitmode, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateGlyphRunAnalysis2(&self, glyphrun: *const DWRITE_GLYPH_RUN, transform: ::core::option::Option<*const DWRITE_MATRIX>, renderingmode: DWRITE_RENDERING_MODE, measuringmode: DWRITE_MEASURING_MODE, gridfitmode: DWRITE_GRID_FIT_MODE, antialiasmode: DWRITE_TEXT_ANTIALIAS_MODE, baselineoriginx: f32, baselineoriginy: f32) -> ::windows::core::Result<IDWriteGlyphRunAnalysis> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateGlyphRunAnalysis2)(::windows::core::Vtable::as_raw(self), glyphrun, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), renderingmode, measuringmode, gridfitmode, antialiasmode, baselineoriginx, baselineoriginy, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateGlyphRunAnalysis3(&self, glyphrun: *const DWRITE_GLYPH_RUN, transform: ::core::option::Option<*const DWRITE_MATRIX>, renderingmode: DWRITE_RENDERING_MODE1, measuringmode: DWRITE_MEASURING_MODE, gridfitmode: DWRITE_GRID_FIT_MODE, antialiasmode: DWRITE_TEXT_ANTIALIAS_MODE, baselineoriginx: f32, baselineoriginy: f32) -> ::windows::core::Result<IDWriteGlyphRunAnalysis> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateGlyphRunAnalysis3)(::windows::core::Vtable::as_raw(self), glyphrun, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), renderingmode, measuringmode, gridfitmode, antialiasmode, baselineoriginx, baselineoriginy, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateCustomRenderingParams4(&self, gamma: f32, enhancedcontrast: f32, grayscaleenhancedcontrast: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE1, gridfitmode: DWRITE_GRID_FIT_MODE) -> ::windows::core::Result<IDWriteRenderingParams3> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateCustomRenderingParams4)(::windows::core::Vtable::as_raw(self), gamma, enhancedcontrast, grayscaleenhancedcontrast, cleartypelevel, pixelgeometry, renderingmode, gridfitmode, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateFontFaceReference<P0>(&self, fontfile: P0, faceindex: u32, fontsimulations: DWRITE_FONT_SIMULATIONS) -> ::windows::core::Result<IDWriteFontFaceReference>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteFontFile>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateFontFaceReference)(::windows::core::Vtable::as_raw(self), fontfile.into().abi(), faceindex, fontsimulations, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateFontFaceReference2<P0>(&self, filepath: P0, lastwritetime: ::core::option::Option<*const super::super::Foundation::FILETIME>, faceindex: u32, fontsimulations: DWRITE_FONT_SIMULATIONS) -> ::windows::core::Result<IDWriteFontFaceReference>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateFontFaceReference2)(::windows::core::Vtable::as_raw(self), filepath.into().abi(), ::core::mem::transmute(lastwritetime.unwrap_or(::std::ptr::null())), faceindex, fontsimulations, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetSystemFontSet(&self) -> ::windows::core::Result<IDWriteFontSet> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetSystemFontSet)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateFontSetBuilder(&self) -> ::windows::core::Result<IDWriteFontSetBuilder> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateFontSetBuilder)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateFontCollectionFromFontSet<P0>(&self, fontset: P0) -> ::windows::core::Result<IDWriteFontCollection1>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteFontSet>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateFontCollectionFromFontSet)(::windows::core::Vtable::as_raw(self), fontset.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSystemFontCollection2<P0, P1>(&self, includedownloadablefonts: P0, fontcollection: *mut ::core::option::Option<IDWriteFontCollection1>, checkforupdates: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetSystemFontCollection2)(::windows::core::Vtable::as_raw(self), includedownloadablefonts.into(), ::core::mem::transmute(fontcollection), checkforupdates.into()).ok()
    }
    pub unsafe fn GetFontDownloadQueue(&self) -> ::windows::core::Result<IDWriteFontDownloadQueue> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetFontDownloadQueue)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn TranslateColorGlyphRun2(&self, baselineorigin: super::Direct2D::Common::D2D_POINT_2F, glyphrun: *const DWRITE_GLYPH_RUN, glyphrundescription: ::core::option::Option<*const DWRITE_GLYPH_RUN_DESCRIPTION>, desiredglyphimageformats: DWRITE_GLYPH_IMAGE_FORMATS, measuringmode: DWRITE_MEASURING_MODE, worldanddpitransform: ::core::option::Option<*const DWRITE_MATRIX>, colorpaletteindex: u32) -> ::windows::core::Result<IDWriteColorGlyphRunEnumerator1> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.TranslateColorGlyphRun2)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(baselineorigin), glyphrun, ::core::mem::transmute(glyphrundescription.unwrap_or(::std::ptr::null())), desiredglyphimageformats, measuringmode, ::core::mem::transmute(worldanddpitransform.unwrap_or(::std::ptr::null())), colorpaletteindex, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn ComputeGlyphOrigins(&self, glyphrun: *const DWRITE_GLYPH_RUN, baselineorigin: super::Direct2D::Common::D2D_POINT_2F) -> ::windows::core::Result<super::Direct2D::Common::D2D_POINT_2F> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.ComputeGlyphOrigins)(::windows::core::Vtable::as_raw(self), glyphrun, ::core::mem::transmute(baselineorigin), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn ComputeGlyphOrigins2(&self, glyphrun: *const DWRITE_GLYPH_RUN, measuringmode: DWRITE_MEASURING_MODE, baselineorigin: super::Direct2D::Common::D2D_POINT_2F, worldanddpitransform: ::core::option::Option<*const DWRITE_MATRIX>) -> ::windows::core::Result<super::Direct2D::Common::D2D_POINT_2F> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.ComputeGlyphOrigins2)(::windows::core::Vtable::as_raw(self), glyphrun, measuringmode, ::core::mem::transmute(baselineorigin), ::core::mem::transmute(worldanddpitransform.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateFontSetBuilder2(&self) -> ::windows::core::Result<IDWriteFontSetBuilder1> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateFontSetBuilder2)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateInMemoryFontFileLoader(&self) -> ::windows::core::Result<IDWriteInMemoryFontFileLoader> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateInMemoryFontFileLoader)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateHttpFontFileLoader<P0, P1>(&self, referrerurl: P0, extraheaders: P1) -> ::windows::core::Result<IDWriteRemoteFontFileLoader>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateHttpFontFileLoader)(::windows::core::Vtable::as_raw(self), referrerurl.into().abi(), extraheaders.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn AnalyzeContainerType(&self, filedata: *const ::core::ffi::c_void, filedatasize: u32) -> DWRITE_CONTAINER_TYPE {
        (::windows::core::Vtable::vtable(self).base__.AnalyzeContainerType)(::windows::core::Vtable::as_raw(self), filedata, filedatasize)
    }
    pub unsafe fn UnpackFontFile(&self, containertype: DWRITE_CONTAINER_TYPE, filedata: *const ::core::ffi::c_void, filedatasize: u32) -> ::windows::core::Result<IDWriteFontFileStream> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.UnpackFontFile)(::windows::core::Vtable::as_raw(self), containertype, filedata, filedatasize, result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IDWriteFactory7 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDWriteFactory7 {}
impl ::core::fmt::Debug for IDWriteFactory7 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDWriteFactory7").field(&self.0).finish()
    }
}
impl IDWriteFactory7 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSystemFontCollection<P0>(&self, fontcollection: *mut ::core::option::Option<IDWriteFontCollection>, checkforupdates: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.GetSystemFontCollection)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(fontcollection), checkforupdates.into()).ok()
    }
    pub unsafe fn CreateCustomFontCollection<P0>(&self, collectionloader: P0, collectionkey: *const ::core::ffi::c_void, collectionkeysize: u32) -> ::windows::core::Result<IDWriteFontCollection>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteFontCollectionLoader>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CreateCustomFontCollection)(::windows::core::Vtable::as_raw(self), collectionloader.into().abi(), collectionkey, collectionkeysize, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RegisterFontCollectionLoader<P0>(&self, fontcollectionloader: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteFontCollectionLoader>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.RegisterFontCollectionLoader)(::windows::core::Vtable::as_raw(self), fontcollectionloader.into().abi()).ok()
    }
    pub unsafe fn UnregisterFontCollectionLoader<P0>(&self, fontcollectionloader: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteFontCollectionLoader>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.UnregisterFontCollectionLoader)(::windows::core::Vtable::as_raw(self), fontcollectionloader.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateFontFileReference<P0>(&self, filepath: P0, lastwritetime: ::core::option::Option<*const super::super::Foundation::FILETIME>) -> ::windows::core::Result<IDWriteFontFile>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CreateFontFileReference)(::windows::core::Vtable::as_raw(self), filepath.into().abi(), ::core::mem::transmute(lastwritetime.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateCustomFontFileReference<P0>(&self, fontfilereferencekey: *const ::core::ffi::c_void, fontfilereferencekeysize: u32, fontfileloader: P0) -> ::windows::core::Result<IDWriteFontFile>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteFontFileLoader>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CreateCustomFontFileReference)(::windows::core::Vtable::as_raw(self), fontfilereferencekey, fontfilereferencekeysize, fontfileloader.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateFontFace(&self, fontfacetype: DWRITE_FONT_FACE_TYPE, fontfiles: &[IDWriteFontFile], faceindex: u32, fontfacesimulationflags: DWRITE_FONT_SIMULATIONS) -> ::windows::core::Result<IDWriteFontFace> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CreateFontFace)(::windows::core::Vtable::as_raw(self), fontfacetype, fontfiles.len() as _, ::core::mem::transmute(fontfiles.as_ptr()), faceindex, fontfacesimulationflags, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateRenderingParams(&self) -> ::windows::core::Result<IDWriteRenderingParams> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CreateRenderingParams)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn CreateMonitorRenderingParams<P0>(&self, monitor: P0) -> ::windows::core::Result<IDWriteRenderingParams>
    where
        P0: ::std::convert::Into<super::Gdi::HMONITOR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CreateMonitorRenderingParams)(::windows::core::Vtable::as_raw(self), monitor.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateCustomRenderingParams(&self, gamma: f32, enhancedcontrast: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE) -> ::windows::core::Result<IDWriteRenderingParams> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CreateCustomRenderingParams)(::windows::core::Vtable::as_raw(self), gamma, enhancedcontrast, cleartypelevel, pixelgeometry, renderingmode, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RegisterFontFileLoader<P0>(&self, fontfileloader: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteFontFileLoader>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.RegisterFontFileLoader)(::windows::core::Vtable::as_raw(self), fontfileloader.into().abi()).ok()
    }
    pub unsafe fn UnregisterFontFileLoader<P0>(&self, fontfileloader: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteFontFileLoader>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.UnregisterFontFileLoader)(::windows::core::Vtable::as_raw(self), fontfileloader.into().abi()).ok()
    }
    pub unsafe fn CreateTextFormat<P0, P1, P2>(&self, fontfamilyname: P0, fontcollection: P1, fontweight: DWRITE_FONT_WEIGHT, fontstyle: DWRITE_FONT_STYLE, fontstretch: DWRITE_FONT_STRETCH, fontsize: f32, localename: P2) -> ::windows::core::Result<IDWriteTextFormat>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<IDWriteFontCollection>>,
        P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CreateTextFormat)(::windows::core::Vtable::as_raw(self), fontfamilyname.into().abi(), fontcollection.into().abi(), fontweight, fontstyle, fontstretch, fontsize, localename.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateTypography(&self) -> ::windows::core::Result<IDWriteTypography> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CreateTypography)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetGdiInterop(&self) -> ::windows::core::Result<IDWriteGdiInterop> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.GetGdiInterop)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateTextLayout<P0>(&self, string: &[u16], textformat: P0, maxwidth: f32, maxheight: f32) -> ::windows::core::Result<IDWriteTextLayout>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteTextFormat>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CreateTextLayout)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(string.as_ptr()), string.len() as _, textformat.into().abi(), maxwidth, maxheight, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateGdiCompatibleTextLayout<P0, P1>(&self, string: &[u16], textformat: P0, layoutwidth: f32, layoutheight: f32, pixelsperdip: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, usegdinatural: P1) -> ::windows::core::Result<IDWriteTextLayout>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteTextFormat>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CreateGdiCompatibleTextLayout)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(string.as_ptr()), string.len() as _, textformat.into().abi(), layoutwidth, layoutheight, pixelsperdip, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), usegdinatural.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateEllipsisTrimmingSign<P0>(&self, textformat: P0) -> ::windows::core::Result<IDWriteInlineObject>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteTextFormat>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CreateEllipsisTrimmingSign)(::windows::core::Vtable::as_raw(self), textformat.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateTextAnalyzer(&self) -> ::windows::core::Result<IDWriteTextAnalyzer> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CreateTextAnalyzer)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateNumberSubstitution<P0, P1>(&self, substitutionmethod: DWRITE_NUMBER_SUBSTITUTION_METHOD, localename: P0, ignoreuseroverride: P1) -> ::windows::core::Result<IDWriteNumberSubstitution>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CreateNumberSubstitution)(::windows::core::Vtable::as_raw(self), substitutionmethod, localename.into().abi(), ignoreuseroverride.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateGlyphRunAnalysis(&self, glyphrun: *const DWRITE_GLYPH_RUN, pixelsperdip: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, renderingmode: DWRITE_RENDERING_MODE, measuringmode: DWRITE_MEASURING_MODE, baselineoriginx: f32, baselineoriginy: f32) -> ::windows::core::Result<IDWriteGlyphRunAnalysis> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CreateGlyphRunAnalysis)(::windows::core::Vtable::as_raw(self), glyphrun, pixelsperdip, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), renderingmode, measuringmode, baselineoriginx, baselineoriginy, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEudcFontCollection<P0>(&self, fontcollection: *mut ::core::option::Option<IDWriteFontCollection>, checkforupdates: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.GetEudcFontCollection)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(fontcollection), checkforupdates.into()).ok()
    }
    pub unsafe fn CreateCustomRenderingParams2(&self, gamma: f32, enhancedcontrast: f32, enhancedcontrastgrayscale: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE) -> ::windows::core::Result<IDWriteRenderingParams1> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CreateCustomRenderingParams2)(::windows::core::Vtable::as_raw(self), gamma, enhancedcontrast, enhancedcontrastgrayscale, cleartypelevel, pixelgeometry, renderingmode, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetSystemFontFallback(&self) -> ::windows::core::Result<IDWriteFontFallback> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetSystemFontFallback)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateFontFallbackBuilder(&self) -> ::windows::core::Result<IDWriteFontFallbackBuilder> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateFontFallbackBuilder)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TranslateColorGlyphRun(&self, baselineoriginx: f32, baselineoriginy: f32, glyphrun: *const DWRITE_GLYPH_RUN, glyphrundescription: ::core::option::Option<*const DWRITE_GLYPH_RUN_DESCRIPTION>, measuringmode: DWRITE_MEASURING_MODE, worldtodevicetransform: ::core::option::Option<*const DWRITE_MATRIX>, colorpaletteindex: u32) -> ::windows::core::Result<IDWriteColorGlyphRunEnumerator> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.TranslateColorGlyphRun)(::windows::core::Vtable::as_raw(self), baselineoriginx, baselineoriginy, glyphrun, ::core::mem::transmute(glyphrundescription.unwrap_or(::std::ptr::null())), measuringmode, ::core::mem::transmute(worldtodevicetransform.unwrap_or(::std::ptr::null())), colorpaletteindex, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateCustomRenderingParams3(&self, gamma: f32, enhancedcontrast: f32, grayscaleenhancedcontrast: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE, gridfitmode: DWRITE_GRID_FIT_MODE) -> ::windows::core::Result<IDWriteRenderingParams2> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateCustomRenderingParams3)(::windows::core::Vtable::as_raw(self), gamma, enhancedcontrast, grayscaleenhancedcontrast, cleartypelevel, pixelgeometry, renderingmode, gridfitmode, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateGlyphRunAnalysis2(&self, glyphrun: *const DWRITE_GLYPH_RUN, transform: ::core::option::Option<*const DWRITE_MATRIX>, renderingmode: DWRITE_RENDERING_MODE, measuringmode: DWRITE_MEASURING_MODE, gridfitmode: DWRITE_GRID_FIT_MODE, antialiasmode: DWRITE_TEXT_ANTIALIAS_MODE, baselineoriginx: f32, baselineoriginy: f32) -> ::windows::core::Result<IDWriteGlyphRunAnalysis> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateGlyphRunAnalysis2)(::windows::core::Vtable::as_raw(self), glyphrun, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), renderingmode, measuringmode, gridfitmode, antialiasmode, baselineoriginx, baselineoriginy, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateGlyphRunAnalysis3(&self, glyphrun: *const DWRITE_GLYPH_RUN, transform: ::core::option::Option<*const DWRITE_MATRIX>, renderingmode: DWRITE_RENDERING_MODE1, measuringmode: DWRITE_MEASURING_MODE, gridfitmode: DWRITE_GRID_FIT_MODE, antialiasmode: DWRITE_TEXT_ANTIALIAS_MODE, baselineoriginx: f32, baselineoriginy: f32) -> ::windows::core::Result<IDWriteGlyphRunAnalysis> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateGlyphRunAnalysis3)(::windows::core::Vtable::as_raw(self), glyphrun, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), renderingmode, measuringmode, gridfitmode, antialiasmode, baselineoriginx, baselineoriginy, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateCustomRenderingParams4(&self, gamma: f32, enhancedcontrast: f32, grayscaleenhancedcontrast: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE1, gridfitmode: DWRITE_GRID_FIT_MODE) -> ::windows::core::Result<IDWriteRenderingParams3> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateCustomRenderingParams4)(::windows::core::Vtable::as_raw(self), gamma, enhancedcontrast, grayscaleenhancedcontrast, cleartypelevel, pixelgeometry, renderingmode, gridfitmode, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateFontFaceReference<P0>(&self, fontfile: P0, faceindex: u32, fontsimulations: DWRITE_FONT_SIMULATIONS) -> ::windows::core::Result<IDWriteFontFaceReference>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteFontFile>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateFontFaceReference)(::windows::core::Vtable::as_raw(self), fontfile.into().abi(), faceindex, fontsimulations, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateFontFaceReference2<P0>(&self, filepath: P0, lastwritetime: ::core::option::Option<*const super::super::Foundation::FILETIME>, faceindex: u32, fontsimulations: DWRITE_FONT_SIMULATIONS) -> ::windows::core::Result<IDWriteFontFaceReference>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateFontFaceReference2)(::windows::core::Vtable::as_raw(self), filepath.into().abi(), ::core::mem::transmute(lastwritetime.unwrap_or(::std::ptr::null())), faceindex, fontsimulations, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetSystemFontSet(&self) -> ::windows::core::Result<IDWriteFontSet> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetSystemFontSet)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateFontSetBuilder(&self) -> ::windows::core::Result<IDWriteFontSetBuilder> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateFontSetBuilder)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateFontCollectionFromFontSet<P0>(&self, fontset: P0) -> ::windows::core::Result<IDWriteFontCollection1>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteFontSet>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateFontCollectionFromFontSet)(::windows::core::Vtable::as_raw(self), fontset.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSystemFontCollection2<P0, P1>(&self, includedownloadablefonts: P0, fontcollection: *mut ::core::option::Option<IDWriteFontCollection1>, checkforupdates: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetSystemFontCollection2)(::windows::core::Vtable::as_raw(self), includedownloadablefonts.into(), ::core::mem::transmute(fontcollection), checkforupdates.into()).ok()
    }
    pub unsafe fn GetFontDownloadQueue(&self) -> ::windows::core::Result<IDWriteFontDownloadQueue> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetFontDownloadQueue)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn TranslateColorGlyphRun2(&self, baselineorigin: super::Direct2D::Common::D2D_POINT_2F, glyphrun: *const DWRITE_GLYPH_RUN, glyphrundescription: ::core::option::Option<*const DWRITE_GLYPH_RUN_DESCRIPTION>, desiredglyphimageformats: DWRITE_GLYPH_IMAGE_FORMATS, measuringmode: DWRITE_MEASURING_MODE, worldanddpitransform: ::core::option::Option<*const DWRITE_MATRIX>, colorpaletteindex: u32) -> ::windows::core::Result<IDWriteColorGlyphRunEnumerator1> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.TranslateColorGlyphRun2)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(baselineorigin), glyphrun, ::core::mem::transmute(glyphrundescription.unwrap_or(::std::ptr::null())), desiredglyphimageformats, measuringmode, ::core::mem::transmute(worldanddpitransform.unwrap_or(::std::ptr::null())), colorpaletteindex, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn ComputeGlyphOrigins(&self, glyphrun: *const DWRITE_GLYPH_RUN, baselineorigin: super::Direct2D::Common::D2D_POINT_2F) -> ::windows::core::Result<super::Direct2D::Common::D2D_POINT_2F> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ComputeGlyphOrigins)(::windows::core::Vtable::as_raw(self), glyphrun, ::core::mem::transmute(baselineorigin), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn ComputeGlyphOrigins2(&self, glyphrun: *const DWRITE_GLYPH_RUN, measuringmode: DWRITE_MEASURING_MODE, baselineorigin: super::Direct2D::Common::D2D_POINT_2F, worldanddpitransform: ::core::option::Option<*const DWRITE_MATRIX>) -> ::windows::core::Result<super::Direct2D::Common::D2D_POINT_2F> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ComputeGlyphOrigins2)(::windows::core::Vtable::as_raw(self), glyphrun, measuringmode, ::core::mem::transmute(baselineorigin), ::core::mem::transmute(worldanddpitransform.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateFontSetBuilder2(&self) -> ::windows::core::Result<IDWriteFontSetBuilder1> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateFontSetBuilder2)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateInMemoryFontFileLoader(&self) -> ::windows::core::Result<IDWriteInMemoryFontFileLoader> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateInMemoryFontFileLoader)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateHttpFontFileLoader<P0, P1>(&self, referrerurl: P0, extraheaders: P1) -> ::windows::core::Result<IDWriteRemoteFontFileLoader>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateHttpFontFileLoader)(::windows::core::Vtable::as_raw(self), referrerurl.into().abi(), extraheaders.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn AnalyzeContainerType(&self, filedata: *const ::core::ffi::c_void, filedatasize: u32) -> DWRITE_CONTAINER_TYPE {
        (::windows::core::Vtable::vtable(self).base__.base__.AnalyzeContainerType)(::windows::core::Vtable::as_raw(self), filedata, filedatasize)
    }
    pub unsafe fn UnpackFontFile(&self, containertype: DWRITE_CONTAINER_TYPE, filedata: *const ::core::ffi::c_void, filedatasize: u32) -> ::windows::core::Result<IDWriteFontFileStream> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.UnpackFontFile)(::windows::core::Vtable::as_raw(self), containertype, filedata, filedatasize, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateFontFaceReference3<P0>(&self, fontfile: P0, faceindex: u32, fontsimulations: DWRITE_FONT_SIMULATIONS, fontaxisvalues: &[DWRITE_FONT_AXIS_VALUE]) -> ::windows::core::Result<IDWriteFontFaceReference1>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteFontFile>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateFontFaceReference3)(::windows::core::Vtable::as_raw(self), fontfile.into().abi(), faceindex, fontsimulations, ::core::mem::transmute(fontaxisvalues.as_ptr()), fontaxisvalues.len() as _, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateFontResource<P0>(&self, fontfile: P0, faceindex: u32) -> ::windows::core::Result<IDWriteFontResource>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteFontFile>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateFontResource)(::windows::core::Vtable::as_raw(self), fontfile.into().abi(), faceindex, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSystemFontSet2<P0>(&self, includedownloadablefonts: P0) -> ::windows::core::Result<IDWriteFontSet1>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetSystemFontSet2)(::windows::core::Vtable::as_raw(self), includedownloadablefonts.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSystemFontCollection3<P0>(&self, includedownloadablefonts: P0, fontfamilymodel: DWRITE_FONT_FAMILY_MODEL) -> ::windows::core::Result<IDWriteFontCollection2>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetSystemFontCollection3)(::windows::core::Vtable::as_raw(self), includedownloadablefonts.into(), fontfamilymodel, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateFontCollectionFromFontSet2<P0>(&self, fontset: P0, fontfamilymodel: DWRITE_FONT_FAMILY_MODEL) -> ::windows::core::Result<IDWriteFontCollection2>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteFontSet>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateFontCollectionFromFontSet2)(::windows::core::Vtable::as_raw(self), fontset.into().abi(), fontfamilymodel, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateFontSetBuilder3(&self) -> ::windows::core::Result<IDWriteFontSetBuilder2> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateFontSetBuilder3)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateTextFormat2<P0, P1, P2>(&self, fontfamilyname: P0, fontcollection: P1, fontaxisvalues: &[DWRITE_FONT_AXIS_VALUE], fontsize: f32, localename: P2) -> ::windows::core::Result<IDWriteTextFormat3>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<IDWriteFontCollection>>,
        P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateTextFormat2)(::windows::core::Vtable::as_raw(self), fontfamilyname.into().abi(), fontcollection.into().abi(), ::core::mem::transmute(fontaxisvalues.as_ptr()), fontaxisvalues.len() as _, fontsize, localename.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IDWriteFont {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDWriteFont {}
impl ::core::fmt::Debug for IDWriteFont {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDWriteFont").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDWriteFont1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDWriteFont1 {}
impl ::core::fmt::Debug for IDWriteFont1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDWriteFont1").field(&self.0).finish()
    }
}
impl IDWriteFont1 {
    pub unsafe fn GetFontFamily(&self) -> ::windows::core::Result<IDWriteFontFamily> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFontFamily)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetWeight(&self) -> DWRITE_FONT_WEIGHT {
        (::windows::core::Vtable::vtable(self).base__.GetWeight)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetStretch(&self) -> DWRITE_FONT_STRETCH {
        (::windows::core::Vtable::vtable(self).base__.GetStretch)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetStyle(&self) -> DWRITE_FONT_STYLE {
        (::windows::core::Vtable::vtable(self).base__.GetStyle)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSymbolFont(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.IsSymbolFont)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetFaceNames(&self) -> ::windows::core::Result<IDWriteLocalizedStrings> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFaceNames)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetInformationalStrings(&self, informationalstringid: DWRITE_INFORMATIONAL_STRING_ID, informationalstrings: *mut ::core::option::Option<IDWriteLocalizedStrings>, exists: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetInformationalStrings)(::windows::core::Vtable::as_raw(self), informationalstringid, ::core::mem::transmute(informationalstrings), exists).ok()
    }
    pub unsafe fn GetSimulations(&self) -> DWRITE_FONT_SIMULATIONS {
        (::windows::core::Vtable::vtable(self).base__.GetSimulations)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetMetrics(&self, fontmetrics: *mut DWRITE_FONT_METRICS) {
        (::windows::core::Vtable::vtable(self).base__.GetMetrics)(::windows::core::Vtable::as_raw(self), fontmetrics)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HasCharacter(&self, unicodevalue: u32) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.HasCharacter)(::windows::core::Vtable::as_raw(self), unicodevalue, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateFontFace(&self) -> ::windows::core::Result<IDWriteFontFace> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateFontFace)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IDWriteFont2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDWriteFont2 {}
impl ::core::fmt::Debug for IDWriteFont2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDWriteFont2").field(&self.0).finish()
    }
}
impl IDWriteFont2 {
    pub unsafe fn GetFontFamily(&self) -> ::windows::core::Result<IDWriteFontFamily> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetFontFamily)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetWeight(&self) -> DWRITE_FONT_WEIGHT {
        (::windows::core::Vtable::vtable(self).base__.base__.GetWeight)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetStretch(&self) -> DWRITE_FONT_STRETCH {
        (::windows::core::Vtable::vtable(self).base__.base__.GetStretch)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetStyle(&self) -> DWRITE_FONT_STYLE {
        (::windows::core::Vtable::vtable(self).base__.base__.GetStyle)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSymbolFont(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.base__.IsSymbolFont)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetFaceNames(&self) -> ::windows::core::Result<IDWriteLocalizedStrings> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetFaceNames)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetInformationalStrings(&self, informationalstringid: DWRITE_INFORMATIONAL_STRING_ID, informationalstrings: *mut ::core::option::Option<IDWriteLocalizedStrings>, exists: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetInformationalStrings)(::windows::core::Vtable::as_raw(self), informationalstringid, ::core::mem::transmute(informationalstrings), exists).ok()
    }
    pub unsafe fn GetSimulations(&self) -> DWRITE_FONT_SIMULATIONS {
        (::windows::core::Vtable::vtable(self).base__.base__.GetSimulations)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetMetrics(&self, fontmetrics: *mut DWRITE_FONT_METRICS) {
        (::windows::core::Vtable::vtable(self).base__.base__.GetMetrics)(::windows::core::Vtable::as_raw(self), fontmetrics)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HasCharacter(&self, unicodevalue: u32) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.HasCharacter)(::windows::core::Vtable::as_raw(self), unicodevalue, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateFontFace(&self) -> ::windows::core::Result<IDWriteFontFace> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateFontFace)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMetrics2(&self, fontmetrics: *mut DWRITE_FONT_METRICS1) {
        (::windows::core::Vtable::vtable(self).base__.GetMetrics2)(::windows::core::Vtable::as_raw(self), fontmetrics)
    }
    pub unsafe fn GetPanose(&self) -> DWRITE_PANOSE {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetPanose)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        result__.assume_init()
    }
    pub unsafe fn GetUnicodeRanges(&self, unicoderanges: ::core::option::Option<&mut [DWRITE_UNICODE_RANGE]>, actualrangecount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetUnicodeRanges)(::windows::core::Vtable::as_raw(self), unicoderanges.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(unicoderanges.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), actualrangecount).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsMonospacedFont(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.IsMonospacedFont)(::windows::core::Vtable::as_raw(self))
    }
}
impl ::core::cmp::PartialEq for IDWriteFont3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDWriteFont3 {}
impl ::core::fmt::Debug for IDWriteFont3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDWriteFont3").field(&self.0).finish()
    }
}
impl IDWriteFont3 {
    pub unsafe fn GetFontFamily(&self) -> ::windows::core::Result<IDWriteFontFamily> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetFontFamily)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetWeight(&self) -> DWRITE_FONT_WEIGHT {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetWeight)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetStretch(&self) -> DWRITE_FONT_STRETCH {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetStretch)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetStyle(&self) -> DWRITE_FONT_STYLE {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetStyle)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSymbolFont(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.IsSymbolFont)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetFaceNames(&self) -> ::windows::core::Result<IDWriteLocalizedStrings> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetFaceNames)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetInformationalStrings(&self, informationalstringid: DWRITE_INFORMATIONAL_STRING_ID, informationalstrings: *mut ::core::option::Option<IDWriteLocalizedStrings>, exists: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetInformationalStrings)(::windows::core::Vtable::as_raw(self), informationalstringid, ::core::mem::transmute(informationalstrings), exists).ok()
    }
    pub unsafe fn GetSimulations(&self) -> DWRITE_FONT_SIMULATIONS {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetSimulations)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetMetrics(&self, fontmetrics: *mut DWRITE_FONT_METRICS) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetMetrics)(::windows::core::Vtable::as_raw(self), fontmetrics)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HasCharacter(&self, unicodevalue: u32) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.HasCharacter)(::windows::core::Vtable::as_raw(self), unicodevalue, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateFontFace(&self) -> ::windows::core::Result<IDWriteFontFace> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateFontFace)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMetrics2(&self, fontmetrics: *mut DWRITE_FONT_METRICS1) {
        (::windows::core::Vtable::vtable(self).base__.base__.GetMetrics2)(::windows::core::Vtable::as_raw(self), fontmetrics)
    }
    pub unsafe fn GetPanose(&self) -> DWRITE_PANOSE {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetPanose)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        result__.assume_init()
    }
    pub unsafe fn GetUnicodeRanges(&self, unicoderanges: ::core::option::Option<&mut [DWRITE_UNICODE_RANGE]>, actualrangecount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetUnicodeRanges)(::windows::core::Vtable::as_raw(self), unicoderanges.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(unicoderanges.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), actualrangecount).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsMonospacedFont(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.base__.IsMonospacedFont)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsColorFont(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.IsColorFont)(::windows::core::Vtable::as_raw(self))
    }
}
impl ::core::cmp::PartialEq for IDWriteFontCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDWriteFontCollection {}
impl ::core::fmt::Debug for IDWriteFontCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDWriteFontCollection").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDWriteFontCollection1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDWriteFontCollection1 {}
impl ::core::fmt::Debug for IDWriteFontCollection1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDWriteFontCollection1").field(&self.0).finish()
    }
}
impl IDWriteFontCollection1 {
    pub unsafe fn GetFontFamilyCount(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.GetFontFamilyCount)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetFontFamily(&self, index: u32) -> ::windows::core::Result<IDWriteFontFamily> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFontFamily)(::windows::core::Vtable::as_raw(self), index, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindFamilyName<P0>(&self, familyname: P0, index: *mut u32, exists: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.FindFamilyName)(::windows::core::Vtable::as_raw(self), familyname.into().abi(), index, exists).ok()
    }
    pub unsafe fn GetFontFromFontFace<P0>(&self, fontface: P0) -> ::windows::core::Result<IDWriteFont>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteFontFace>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFontFromFontFace)(::windows::core::Vtable::as_raw(self), fontface.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IDWriteFontCollection2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDWriteFontCollection2 {}
impl ::core::fmt::Debug for IDWriteFontCollection2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDWriteFontCollection2").field(&self.0).finish()
    }
}
impl IDWriteFontCollection2 {
    pub unsafe fn GetFontFamilyCount(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.base__.GetFontFamilyCount)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetFontFamily(&self, index: u32) -> ::windows::core::Result<IDWriteFontFamily> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetFontFamily)(::windows::core::Vtable::as_raw(self), index, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindFamilyName<P0>(&self, familyname: P0, index: *mut u32, exists: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.FindFamilyName)(::windows::core::Vtable::as_raw(self), familyname.into().abi(), index, exists).ok()
    }
    pub unsafe fn GetFontFromFontFace<P0>(&self, fontface: P0) -> ::windows::core::Result<IDWriteFont>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteFontFace>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetFontFromFontFace)(::windows::core::Vtable::as_raw(self), fontface.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetFontSet(&self) -> ::windows::core::Result<IDWriteFontSet> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFontSet)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetFontFamily2(&self, index: u32) -> ::windows::core::Result<IDWriteFontFamily1> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFontFamily2)(::windows::core::Vtable::as_raw(self), index, result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IDWriteFontCollection3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDWriteFontCollection3 {}
impl ::core::fmt::Debug for IDWriteFontCollection3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDWriteFontCollection3").field(&self.0).finish()
    }
}
impl IDWriteFontCollection3 {
    pub unsafe fn GetFontFamilyCount(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetFontFamilyCount)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetFontFamily(&self, index: u32) -> ::windows::core::Result<IDWriteFontFamily> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetFontFamily)(::windows::core::Vtable::as_raw(self), index, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindFamilyName<P0>(&self, familyname: P0, index: *mut u32, exists: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.FindFamilyName)(::windows::core::Vtable::as_raw(self), familyname.into().abi(), index, exists).ok()
    }
    pub unsafe fn GetFontFromFontFace<P0>(&self, fontface: P0) -> ::windows::core::Result<IDWriteFont>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteFontFace>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetFontFromFontFace)(::windows::core::Vtable::as_raw(self), fontface.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetFontSet(&self) -> ::windows::core::Result<IDWriteFontSet> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetFontSet)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetFontFamily2(&self, index: u32) -> ::windows::core::Result<IDWriteFontFamily1> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetFontFamily2)(::windows::core::Vtable::as_raw(self), index, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetFontFamily3(&self, index: u32) -> ::windows::core::Result<IDWriteFontFamily2> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFontFamily3)(::windows::core::Vtable::as_raw(self), index, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetMatchingFonts<P0>(&self, familyname: P0, fontaxisvalues: &[DWRITE_FONT_AXIS_VALUE]) -> ::windows::core::Result<IDWriteFontList2>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetMatchingFonts)(::windows::core::Vtable::as_raw(self), familyname.into().abi(), ::core::mem::transmute(fontaxisvalues.as_ptr()), fontaxisvalues.len() as _, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetFontFamilyModel(&self) -> DWRITE_FONT_FAMILY_MODEL {
        (::windows::core::Vtable::vtable(self).base__.GetFontFamilyModel)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetFontSet2(&self) -> ::windows::core::Result<IDWriteFontSet1> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFontSet2)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IDWriteFontCollectionLoader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDWriteFontCollectionLoader {}
impl ::core::fmt::Debug for IDWriteFontCollectionLoader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDWriteFontCollectionLoader").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDWriteFontDownloadListener {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDWriteFontDownloadListener {}
impl ::core::fmt::Debug for IDWriteFontDownloadListener {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDWriteFontDownloadListener").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDWriteFontDownloadQueue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDWriteFontDownloadQueue {}
impl ::core::fmt::Debug for IDWriteFontDownloadQueue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDWriteFontDownloadQueue").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDWriteFontFace {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDWriteFontFace {}
impl ::core::fmt::Debug for IDWriteFontFace {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDWriteFontFace").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDWriteFontFace1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDWriteFontFace1 {}
impl ::core::fmt::Debug for IDWriteFontFace1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDWriteFontFace1").field(&self.0).finish()
    }
}
impl IDWriteFontFace1 {
    pub unsafe fn GetType(&self) -> DWRITE_FONT_FACE_TYPE {
        (::windows::core::Vtable::vtable(self).base__.GetType)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetFiles(&self, numberoffiles: *mut u32, fontfiles: ::core::option::Option<*mut ::core::option::Option<IDWriteFontFile>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetFiles)(::windows::core::Vtable::as_raw(self), numberoffiles, ::core::mem::transmute(fontfiles.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetIndex(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.GetIndex)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetSimulations(&self) -> DWRITE_FONT_SIMULATIONS {
        (::windows::core::Vtable::vtable(self).base__.GetSimulations)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSymbolFont(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.IsSymbolFont)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetMetrics(&self, fontfacemetrics: *mut DWRITE_FONT_METRICS) {
        (::windows::core::Vtable::vtable(self).base__.GetMetrics)(::windows::core::Vtable::as_raw(self), fontfacemetrics)
    }
    pub unsafe fn GetGlyphCount(&self) -> u16 {
        (::windows::core::Vtable::vtable(self).base__.GetGlyphCount)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesignGlyphMetrics<P0>(&self, glyphindices: *const u16, glyphcount: u32, glyphmetrics: *mut DWRITE_GLYPH_METRICS, issideways: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetDesignGlyphMetrics)(::windows::core::Vtable::as_raw(self), glyphindices, glyphcount, glyphmetrics, issideways.into()).ok()
    }
    pub unsafe fn GetGlyphIndices(&self, codepoints: *const u32, codepointcount: u32, glyphindices: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetGlyphIndices)(::windows::core::Vtable::as_raw(self), codepoints, codepointcount, glyphindices).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TryGetFontTable(&self, opentypetabletag: u32, tabledata: *mut *mut ::core::ffi::c_void, tablesize: *mut u32, tablecontext: *mut *mut ::core::ffi::c_void, exists: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.TryGetFontTable)(::windows::core::Vtable::as_raw(self), opentypetabletag, tabledata, tablesize, tablecontext, exists).ok()
    }
    pub unsafe fn ReleaseFontTable(&self, tablecontext: *const ::core::ffi::c_void) {
        (::windows::core::Vtable::vtable(self).base__.ReleaseFontTable)(::windows::core::Vtable::as_raw(self), tablecontext)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn GetGlyphRunOutline<P0, P1, P2>(&self, emsize: f32, glyphindices: *const u16, glyphadvances: ::core::option::Option<*const f32>, glyphoffsets: ::core::option::Option<*const DWRITE_GLYPH_OFFSET>, glyphcount: u32, issideways: P0, isrighttoleft: P1, geometrysink: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
        P2: ::std::convert::Into<::windows::core::InParam<super::Direct2D::Common::ID2D1SimplifiedGeometrySink>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetGlyphRunOutline)(::windows::core::Vtable::as_raw(self), emsize, glyphindices, ::core::mem::transmute(glyphadvances.unwrap_or(::std::ptr::null())), ::core::mem::transmute(glyphoffsets.unwrap_or(::std::ptr::null())), glyphcount, issideways.into(), isrighttoleft.into(), geometrysink.into().abi()).ok()
    }
    pub unsafe fn GetRecommendedRenderingMode<P0>(&self, emsize: f32, pixelsperdip: f32, measuringmode: DWRITE_MEASURING_MODE, renderingparams: P0) -> ::windows::core::Result<DWRITE_RENDERING_MODE>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteRenderingParams>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetRecommendedRenderingMode)(::windows::core::Vtable::as_raw(self), emsize, pixelsperdip, measuringmode, renderingparams.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetGdiCompatibleMetrics(&self, emsize: f32, pixelsperdip: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, fontfacemetrics: *mut DWRITE_FONT_METRICS) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetGdiCompatibleMetrics)(::windows::core::Vtable::as_raw(self), emsize, pixelsperdip, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), fontfacemetrics).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetGdiCompatibleGlyphMetrics<P0, P1>(&self, emsize: f32, pixelsperdip: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, usegdinatural: P0, glyphindices: *const u16, glyphcount: u32, glyphmetrics: *mut DWRITE_GLYPH_METRICS, issideways: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetGdiCompatibleGlyphMetrics)(::windows::core::Vtable::as_raw(self), emsize, pixelsperdip, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), usegdinatural.into(), glyphindices, glyphcount, glyphmetrics, issideways.into()).ok()
    }
}
impl ::core::cmp::PartialEq for IDWriteFontFace2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDWriteFontFace2 {}
impl ::core::fmt::Debug for IDWriteFontFace2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDWriteFontFace2").field(&self.0).finish()
    }
}
impl IDWriteFontFace2 {
    pub unsafe fn GetType(&self) -> DWRITE_FONT_FACE_TYPE {
        (::windows::core::Vtable::vtable(self).base__.base__.GetType)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetFiles(&self, numberoffiles: *mut u32, fontfiles: ::core::option::Option<*mut ::core::option::Option<IDWriteFontFile>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetFiles)(::windows::core::Vtable::as_raw(self), numberoffiles, ::core::mem::transmute(fontfiles.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetIndex(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.base__.GetIndex)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetSimulations(&self) -> DWRITE_FONT_SIMULATIONS {
        (::windows::core::Vtable::vtable(self).base__.base__.GetSimulations)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSymbolFont(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.base__.IsSymbolFont)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetMetrics(&self, fontfacemetrics: *mut DWRITE_FONT_METRICS) {
        (::windows::core::Vtable::vtable(self).base__.base__.GetMetrics)(::windows::core::Vtable::as_raw(self), fontfacemetrics)
    }
    pub unsafe fn GetGlyphCount(&self) -> u16 {
        (::windows::core::Vtable::vtable(self).base__.base__.GetGlyphCount)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesignGlyphMetrics<P0>(&self, glyphindices: *const u16, glyphcount: u32, glyphmetrics: *mut DWRITE_GLYPH_METRICS, issideways: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.GetDesignGlyphMetrics)(::windows::core::Vtable::as_raw(self), glyphindices, glyphcount, glyphmetrics, issideways.into()).ok()
    }
    pub unsafe fn GetGlyphIndices(&self, codepoints: *const u32, codepointcount: u32, glyphindices: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetGlyphIndices)(::windows::core::Vtable::as_raw(self), codepoints, codepointcount, glyphindices).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TryGetFontTable(&self, opentypetabletag: u32, tabledata: *mut *mut ::core::ffi::c_void, tablesize: *mut u32, tablecontext: *mut *mut ::core::ffi::c_void, exists: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.TryGetFontTable)(::windows::core::Vtable::as_raw(self), opentypetabletag, tabledata, tablesize, tablecontext, exists).ok()
    }
    pub unsafe fn ReleaseFontTable(&self, tablecontext: *const ::core::ffi::c_void) {
        (::windows::core::Vtable::vtable(self).base__.base__.ReleaseFontTable)(::windows::core::Vtable::as_raw(self), tablecontext)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn GetGlyphRunOutline<P0, P1, P2>(&self, emsize: f32, glyphindices: *const u16, glyphadvances: ::core::option::Option<*const f32>, glyphoffsets: ::core::option::Option<*const DWRITE_GLYPH_OFFSET>, glyphcount: u32, issideways: P0, isrighttoleft: P1, geometrysink: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
        P2: ::std::convert::Into<::windows::core::InParam<super::Direct2D::Common::ID2D1SimplifiedGeometrySink>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.GetGlyphRunOutline)(::windows::core::Vtable::as_raw(self), emsize, glyphindices, ::core::mem::transmute(glyphadvances.unwrap_or(::std::ptr::null())), ::core::mem::transmute(glyphoffsets.unwrap_or(::std::ptr::null())), glyphcount, issideways.into(), isrighttoleft.into(), geometrysink.into().abi()).ok()
    }
    pub unsafe fn GetRecommendedRenderingMode<P0>(&self, emsize: f32, pixelsperdip: f32, measuringmode: DWRITE_MEASURING_MODE, renderingparams: P0) -> ::windows::core::Result<DWRITE_RENDERING_MODE>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteRenderingParams>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetRecommendedRenderingMode)(::windows::core::Vtable::as_raw(self), emsize, pixelsperdip, measuringmode, renderingparams.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetGdiCompatibleMetrics(&self, emsize: f32, pixelsperdip: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, fontfacemetrics: *mut DWRITE_FONT_METRICS) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetGdiCompatibleMetrics)(::windows::core::Vtable::as_raw(self), emsize, pixelsperdip, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), fontfacemetrics).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetGdiCompatibleGlyphMetrics<P0, P1>(&self, emsize: f32, pixelsperdip: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, usegdinatural: P0, glyphindices: *const u16, glyphcount: u32, glyphmetrics: *mut DWRITE_GLYPH_METRICS, issideways: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.GetGdiCompatibleGlyphMetrics)(::windows::core::Vtable::as_raw(self), emsize, pixelsperdip, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), usegdinatural.into(), glyphindices, glyphcount, glyphmetrics, issideways.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMetrics2(&self, fontmetrics: *mut DWRITE_FONT_METRICS1) {
        (::windows::core::Vtable::vtable(self).base__.GetMetrics2)(::windows::core::Vtable::as_raw(self), fontmetrics)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetGdiCompatibleMetrics2(&self, emsize: f32, pixelsperdip: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, fontmetrics: *mut DWRITE_FONT_METRICS1) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetGdiCompatibleMetrics2)(::windows::core::Vtable::as_raw(self), emsize, pixelsperdip, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), fontmetrics).ok()
    }
    pub unsafe fn GetCaretMetrics(&self) -> DWRITE_CARET_METRICS {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCaretMetrics)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        result__.assume_init()
    }
    pub unsafe fn GetUnicodeRanges(&self, unicoderanges: ::core::option::Option<&mut [DWRITE_UNICODE_RANGE]>, actualrangecount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetUnicodeRanges)(::windows::core::Vtable::as_raw(self), unicoderanges.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(unicoderanges.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), actualrangecount).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsMonospacedFont(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.IsMonospacedFont)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesignGlyphAdvances<P0>(&self, glyphcount: u32, glyphindices: *const u16, glyphadvances: *mut i32, issideways: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetDesignGlyphAdvances)(::windows::core::Vtable::as_raw(self), glyphcount, glyphindices, glyphadvances, issideways.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetGdiCompatibleGlyphAdvances<P0, P1>(&self, emsize: f32, pixelsperdip: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, usegdinatural: P0, issideways: P1, glyphcount: u32, glyphindices: *const u16, glyphadvances: *mut i32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetGdiCompatibleGlyphAdvances)(::windows::core::Vtable::as_raw(self), emsize, pixelsperdip, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), usegdinatural.into(), issideways.into(), glyphcount, glyphindices, glyphadvances).ok()
    }
    pub unsafe fn GetKerningPairAdjustments(&self, glyphcount: u32, glyphindices: *const u16, glyphadvanceadjustments: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetKerningPairAdjustments)(::windows::core::Vtable::as_raw(self), glyphcount, glyphindices, glyphadvanceadjustments).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HasKerningPairs(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.HasKerningPairs)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRecommendedRenderingMode2<P0>(&self, fontemsize: f32, dpix: f32, dpiy: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, issideways: P0, outlinethreshold: DWRITE_OUTLINE_THRESHOLD, measuringmode: DWRITE_MEASURING_MODE) -> ::windows::core::Result<DWRITE_RENDERING_MODE>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetRecommendedRenderingMode2)(::windows::core::Vtable::as_raw(self), fontemsize, dpix, dpiy, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), issideways.into(), outlinethreshold, measuringmode, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetVerticalGlyphVariants(&self, glyphcount: u32, nominalglyphindices: *const u16, verticalglyphindices: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetVerticalGlyphVariants)(::windows::core::Vtable::as_raw(self), glyphcount, nominalglyphindices, verticalglyphindices).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HasVerticalGlyphVariants(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.HasVerticalGlyphVariants)(::windows::core::Vtable::as_raw(self))
    }
}
impl ::core::cmp::PartialEq for IDWriteFontFace3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDWriteFontFace3 {}
impl ::core::fmt::Debug for IDWriteFontFace3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDWriteFontFace3").field(&self.0).finish()
    }
}
impl IDWriteFontFace3 {
    pub unsafe fn GetType(&self) -> DWRITE_FONT_FACE_TYPE {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetType)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetFiles(&self, numberoffiles: *mut u32, fontfiles: ::core::option::Option<*mut ::core::option::Option<IDWriteFontFile>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetFiles)(::windows::core::Vtable::as_raw(self), numberoffiles, ::core::mem::transmute(fontfiles.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetIndex(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetIndex)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetSimulations(&self) -> DWRITE_FONT_SIMULATIONS {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetSimulations)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSymbolFont(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.IsSymbolFont)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetMetrics(&self, fontfacemetrics: *mut DWRITE_FONT_METRICS) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetMetrics)(::windows::core::Vtable::as_raw(self), fontfacemetrics)
    }
    pub unsafe fn GetGlyphCount(&self) -> u16 {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetGlyphCount)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesignGlyphMetrics<P0>(&self, glyphindices: *const u16, glyphcount: u32, glyphmetrics: *mut DWRITE_GLYPH_METRICS, issideways: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetDesignGlyphMetrics)(::windows::core::Vtable::as_raw(self), glyphindices, glyphcount, glyphmetrics, issideways.into()).ok()
    }
    pub unsafe fn GetGlyphIndices(&self, codepoints: *const u32, codepointcount: u32, glyphindices: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetGlyphIndices)(::windows::core::Vtable::as_raw(self), codepoints, codepointcount, glyphindices).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TryGetFontTable(&self, opentypetabletag: u32, tabledata: *mut *mut ::core::ffi::c_void, tablesize: *mut u32, tablecontext: *mut *mut ::core::ffi::c_void, exists: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.TryGetFontTable)(::windows::core::Vtable::as_raw(self), opentypetabletag, tabledata, tablesize, tablecontext, exists).ok()
    }
    pub unsafe fn ReleaseFontTable(&self, tablecontext: *const ::core::ffi::c_void) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ReleaseFontTable)(::windows::core::Vtable::as_raw(self), tablecontext)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn GetGlyphRunOutline<P0, P1, P2>(&self, emsize: f32, glyphindices: *const u16, glyphadvances: ::core::option::Option<*const f32>, glyphoffsets: ::core::option::Option<*const DWRITE_GLYPH_OFFSET>, glyphcount: u32, issideways: P0, isrighttoleft: P1, geometrysink: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
        P2: ::std::convert::Into<::windows::core::InParam<super::Direct2D::Common::ID2D1SimplifiedGeometrySink>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetGlyphRunOutline)(::windows::core::Vtable::as_raw(self), emsize, glyphindices, ::core::mem::transmute(glyphadvances.unwrap_or(::std::ptr::null())), ::core::mem::transmute(glyphoffsets.unwrap_or(::std::ptr::null())), glyphcount, issideways.into(), isrighttoleft.into(), geometrysink.into().abi()).ok()
    }
    pub unsafe fn GetRecommendedRenderingMode<P0>(&self, emsize: f32, pixelsperdip: f32, measuringmode: DWRITE_MEASURING_MODE, renderingparams: P0) -> ::windows::core::Result<DWRITE_RENDERING_MODE>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteRenderingParams>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetRecommendedRenderingMode)(::windows::core::Vtable::as_raw(self), emsize, pixelsperdip, measuringmode, renderingparams.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetGdiCompatibleMetrics(&self, emsize: f32, pixelsperdip: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, fontfacemetrics: *mut DWRITE_FONT_METRICS) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetGdiCompatibleMetrics)(::windows::core::Vtable::as_raw(self), emsize, pixelsperdip, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), fontfacemetrics).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetGdiCompatibleGlyphMetrics<P0, P1>(&self, emsize: f32, pixelsperdip: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, usegdinatural: P0, glyphindices: *const u16, glyphcount: u32, glyphmetrics: *mut DWRITE_GLYPH_METRICS, issideways: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetGdiCompatibleGlyphMetrics)(::windows::core::Vtable::as_raw(self), emsize, pixelsperdip, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), usegdinatural.into(), glyphindices, glyphcount, glyphmetrics, issideways.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMetrics2(&self, fontmetrics: *mut DWRITE_FONT_METRICS1) {
        (::windows::core::Vtable::vtable(self).base__.base__.GetMetrics2)(::windows::core::Vtable::as_raw(self), fontmetrics)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetGdiCompatibleMetrics2(&self, emsize: f32, pixelsperdip: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, fontmetrics: *mut DWRITE_FONT_METRICS1) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetGdiCompatibleMetrics2)(::windows::core::Vtable::as_raw(self), emsize, pixelsperdip, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), fontmetrics).ok()
    }
    pub unsafe fn GetCaretMetrics(&self) -> DWRITE_CARET_METRICS {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetCaretMetrics)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        result__.assume_init()
    }
    pub unsafe fn GetUnicodeRanges(&self, unicoderanges: ::core::option::Option<&mut [DWRITE_UNICODE_RANGE]>, actualrangecount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetUnicodeRanges)(::windows::core::Vtable::as_raw(self), unicoderanges.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(unicoderanges.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), actualrangecount).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsMonospacedFont(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.base__.IsMonospacedFont)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesignGlyphAdvances<P0>(&self, glyphcount: u32, glyphindices: *const u16, glyphadvances: *mut i32, issideways: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.GetDesignGlyphAdvances)(::windows::core::Vtable::as_raw(self), glyphcount, glyphindices, glyphadvances, issideways.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetGdiCompatibleGlyphAdvances<P0, P1>(&self, emsize: f32, pixelsperdip: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, usegdinatural: P0, issideways: P1, glyphcount: u32, glyphindices: *const u16, glyphadvances: *mut i32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.GetGdiCompatibleGlyphAdvances)(::windows::core::Vtable::as_raw(self), emsize, pixelsperdip, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), usegdinatural.into(), issideways.into(), glyphcount, glyphindices, glyphadvances).ok()
    }
    pub unsafe fn GetKerningPairAdjustments(&self, glyphcount: u32, glyphindices: *const u16, glyphadvanceadjustments: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetKerningPairAdjustments)(::windows::core::Vtable::as_raw(self), glyphcount, glyphindices, glyphadvanceadjustments).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HasKerningPairs(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.base__.HasKerningPairs)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRecommendedRenderingMode2<P0>(&self, fontemsize: f32, dpix: f32, dpiy: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, issideways: P0, outlinethreshold: DWRITE_OUTLINE_THRESHOLD, measuringmode: DWRITE_MEASURING_MODE) -> ::windows::core::Result<DWRITE_RENDERING_MODE>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetRecommendedRenderingMode2)(::windows::core::Vtable::as_raw(self), fontemsize, dpix, dpiy, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), issideways.into(), outlinethreshold, measuringmode, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetVerticalGlyphVariants(&self, glyphcount: u32, nominalglyphindices: *const u16, verticalglyphindices: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetVerticalGlyphVariants)(::windows::core::Vtable::as_raw(self), glyphcount, nominalglyphindices, verticalglyphindices).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HasVerticalGlyphVariants(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.base__.HasVerticalGlyphVariants)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsColorFont(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.IsColorFont)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetColorPaletteCount(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.GetColorPaletteCount)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetPaletteEntryCount(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.GetPaletteEntryCount)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetPaletteEntries(&self, colorpaletteindex: u32, firstentryindex: u32, paletteentries: &mut [DWRITE_COLOR_F]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetPaletteEntries)(::windows::core::Vtable::as_raw(self), colorpaletteindex, firstentryindex, paletteentries.len() as _, ::core::mem::transmute(paletteentries.as_ptr())).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRecommendedRenderingMode3<P0, P1>(&self, fontemsize: f32, dpix: f32, dpiy: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, issideways: P0, outlinethreshold: DWRITE_OUTLINE_THRESHOLD, measuringmode: DWRITE_MEASURING_MODE, renderingparams: P1, renderingmode: *mut DWRITE_RENDERING_MODE, gridfitmode: *mut DWRITE_GRID_FIT_MODE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
        P1: ::std::convert::Into<::windows::core::InParam<IDWriteRenderingParams>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetRecommendedRenderingMode3)(::windows::core::Vtable::as_raw(self), fontemsize, dpix, dpiy, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), issideways.into(), outlinethreshold, measuringmode, renderingparams.into().abi(), renderingmode, gridfitmode).ok()
    }
}
impl ::core::cmp::PartialEq for IDWriteFontFace4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDWriteFontFace4 {}
impl ::core::fmt::Debug for IDWriteFontFace4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDWriteFontFace4").field(&self.0).finish()
    }
}
impl IDWriteFontFace4 {
    pub unsafe fn GetType(&self) -> DWRITE_FONT_FACE_TYPE {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetType)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetFiles(&self, numberoffiles: *mut u32, fontfiles: ::core::option::Option<*mut ::core::option::Option<IDWriteFontFile>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetFiles)(::windows::core::Vtable::as_raw(self), numberoffiles, ::core::mem::transmute(fontfiles.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetIndex(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetIndex)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetSimulations(&self) -> DWRITE_FONT_SIMULATIONS {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetSimulations)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSymbolFont(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.IsSymbolFont)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetMetrics(&self, fontfacemetrics: *mut DWRITE_FONT_METRICS) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetMetrics)(::windows::core::Vtable::as_raw(self), fontfacemetrics)
    }
    pub unsafe fn GetGlyphCount(&self) -> u16 {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetGlyphCount)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesignGlyphMetrics<P0>(&self, glyphindices: *const u16, glyphcount: u32, glyphmetrics: *mut DWRITE_GLYPH_METRICS, issideways: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetDesignGlyphMetrics)(::windows::core::Vtable::as_raw(self), glyphindices, glyphcount, glyphmetrics, issideways.into()).ok()
    }
    pub unsafe fn GetGlyphIndices(&self, codepoints: *const u32, codepointcount: u32, glyphindices: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetGlyphIndices)(::windows::core::Vtable::as_raw(self), codepoints, codepointcount, glyphindices).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TryGetFontTable(&self, opentypetabletag: u32, tabledata: *mut *mut ::core::ffi::c_void, tablesize: *mut u32, tablecontext: *mut *mut ::core::ffi::c_void, exists: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.TryGetFontTable)(::windows::core::Vtable::as_raw(self), opentypetabletag, tabledata, tablesize, tablecontext, exists).ok()
    }
    pub unsafe fn ReleaseFontTable(&self, tablecontext: *const ::core::ffi::c_void) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.ReleaseFontTable)(::windows::core::Vtable::as_raw(self), tablecontext)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn GetGlyphRunOutline<P0, P1, P2>(&self, emsize: f32, glyphindices: *const u16, glyphadvances: ::core::option::Option<*const f32>, glyphoffsets: ::core::option::Option<*const DWRITE_GLYPH_OFFSET>, glyphcount: u32, issideways: P0, isrighttoleft: P1, geometrysink: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
        P2: ::std::convert::Into<::windows::core::InParam<super::Direct2D::Common::ID2D1SimplifiedGeometrySink>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetGlyphRunOutline)(::windows::core::Vtable::as_raw(self), emsize, glyphindices, ::core::mem::transmute(glyphadvances.unwrap_or(::std::ptr::null())), ::core::mem::transmute(glyphoffsets.unwrap_or(::std::ptr::null())), glyphcount, issideways.into(), isrighttoleft.into(), geometrysink.into().abi()).ok()
    }
    pub unsafe fn GetRecommendedRenderingMode<P0>(&self, emsize: f32, pixelsperdip: f32, measuringmode: DWRITE_MEASURING_MODE, renderingparams: P0) -> ::windows::core::Result<DWRITE_RENDERING_MODE>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteRenderingParams>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetRecommendedRenderingMode)(::windows::core::Vtable::as_raw(self), emsize, pixelsperdip, measuringmode, renderingparams.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetGdiCompatibleMetrics(&self, emsize: f32, pixelsperdip: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, fontfacemetrics: *mut DWRITE_FONT_METRICS) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetGdiCompatibleMetrics)(::windows::core::Vtable::as_raw(self), emsize, pixelsperdip, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), fontfacemetrics).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetGdiCompatibleGlyphMetrics<P0, P1>(&self, emsize: f32, pixelsperdip: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, usegdinatural: P0, glyphindices: *const u16, glyphcount: u32, glyphmetrics: *mut DWRITE_GLYPH_METRICS, issideways: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetGdiCompatibleGlyphMetrics)(::windows::core::Vtable::as_raw(self), emsize, pixelsperdip, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), usegdinatural.into(), glyphindices, glyphcount, glyphmetrics, issideways.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMetrics2(&self, fontmetrics: *mut DWRITE_FONT_METRICS1) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetMetrics2)(::windows::core::Vtable::as_raw(self), fontmetrics)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetGdiCompatibleMetrics2(&self, emsize: f32, pixelsperdip: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, fontmetrics: *mut DWRITE_FONT_METRICS1) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetGdiCompatibleMetrics2)(::windows::core::Vtable::as_raw(self), emsize, pixelsperdip, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), fontmetrics).ok()
    }
    pub unsafe fn GetCaretMetrics(&self) -> DWRITE_CARET_METRICS {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetCaretMetrics)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        result__.assume_init()
    }
    pub unsafe fn GetUnicodeRanges(&self, unicoderanges: ::core::option::Option<&mut [DWRITE_UNICODE_RANGE]>, actualrangecount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetUnicodeRanges)(::windows::core::Vtable::as_raw(self), unicoderanges.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(unicoderanges.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), actualrangecount).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsMonospacedFont(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.IsMonospacedFont)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesignGlyphAdvances<P0>(&self, glyphcount: u32, glyphindices: *const u16, glyphadvances: *mut i32, issideways: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetDesignGlyphAdvances)(::windows::core::Vtable::as_raw(self), glyphcount, glyphindices, glyphadvances, issideways.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetGdiCompatibleGlyphAdvances<P0, P1>(&self, emsize: f32, pixelsperdip: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, usegdinatural: P0, issideways: P1, glyphcount: u32, glyphindices: *const u16, glyphadvances: *mut i32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetGdiCompatibleGlyphAdvances)(::windows::core::Vtable::as_raw(self), emsize, pixelsperdip, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), usegdinatural.into(), issideways.into(), glyphcount, glyphindices, glyphadvances).ok()
    }
    pub unsafe fn GetKerningPairAdjustments(&self, glyphcount: u32, glyphindices: *const u16, glyphadvanceadjustments: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetKerningPairAdjustments)(::windows::core::Vtable::as_raw(self), glyphcount, glyphindices, glyphadvanceadjustments).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HasKerningPairs(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.HasKerningPairs)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRecommendedRenderingMode2<P0>(&self, fontemsize: f32, dpix: f32, dpiy: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, issideways: P0, outlinethreshold: DWRITE_OUTLINE_THRESHOLD, measuringmode: DWRITE_MEASURING_MODE) -> ::windows::core::Result<DWRITE_RENDERING_MODE>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetRecommendedRenderingMode2)(::windows::core::Vtable::as_raw(self), fontemsize, dpix, dpiy, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), issideways.into(), outlinethreshold, measuringmode, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetVerticalGlyphVariants(&self, glyphcount: u32, nominalglyphindices: *const u16, verticalglyphindices: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetVerticalGlyphVariants)(::windows::core::Vtable::as_raw(self), glyphcount, nominalglyphindices, verticalglyphindices).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HasVerticalGlyphVariants(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.HasVerticalGlyphVariants)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsColorFont(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.base__.IsColorFont)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetColorPaletteCount(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.base__.GetColorPaletteCount)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetPaletteEntryCount(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.base__.GetPaletteEntryCount)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetPaletteEntries(&self, colorpaletteindex: u32, firstentryindex: u32, paletteentries: &mut [DWRITE_COLOR_F]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetPaletteEntries)(::windows::core::Vtable::as_raw(self), colorpaletteindex, firstentryindex, paletteentries.len() as _, ::core::mem::transmute(paletteentries.as_ptr())).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRecommendedRenderingMode3<P0, P1>(&self, fontemsize: f32, dpix: f32, dpiy: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, issideways: P0, outlinethreshold: DWRITE_OUTLINE_THRESHOLD, measuringmode: DWRITE_MEASURING_MODE, renderingparams: P1, renderingmode: *mut DWRITE_RENDERING_MODE, gridfitmode: *mut DWRITE_GRID_FIT_MODE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
        P1: ::std::convert::Into<::windows::core::InParam<IDWriteRenderingParams>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.GetRecommendedRenderingMode3)(::windows::core::Vtable::as_raw(self), fontemsize, dpix, dpiy, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), issideways.into(), outlinethreshold, measuringmode, renderingparams.into().abi(), renderingmode, gridfitmode).ok()
    }
    pub unsafe fn GetFontFaceReference(&self) -> ::windows::core::Result<IDWriteFontFaceReference> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFontFaceReference)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPanose(&self) -> DWRITE_PANOSE {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetPanose)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        result__.assume_init()
    }
    pub unsafe fn GetWeight(&self) -> DWRITE_FONT_WEIGHT {
        (::windows::core::Vtable::vtable(self).base__.GetWeight)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetStretch(&self) -> DWRITE_FONT_STRETCH {
        (::windows::core::Vtable::vtable(self).base__.GetStretch)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetStyle(&self) -> DWRITE_FONT_STYLE {
        (::windows::core::Vtable::vtable(self).base__.GetStyle)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetFamilyNames(&self) -> ::windows::core::Result<IDWriteLocalizedStrings> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFamilyNames)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetFaceNames(&self) -> ::windows::core::Result<IDWriteLocalizedStrings> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFaceNames)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetInformationalStrings(&self, informationalstringid: DWRITE_INFORMATIONAL_STRING_ID, informationalstrings: *mut ::core::option::Option<IDWriteLocalizedStrings>, exists: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetInformationalStrings)(::windows::core::Vtable::as_raw(self), informationalstringid, ::core::mem::transmute(informationalstrings), exists).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HasCharacter(&self, unicodevalue: u32) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.HasCharacter)(::windows::core::Vtable::as_raw(self), unicodevalue)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRecommendedRenderingMode4<P0, P1>(&self, fontemsize: f32, dpix: f32, dpiy: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, issideways: P0, outlinethreshold: DWRITE_OUTLINE_THRESHOLD, measuringmode: DWRITE_MEASURING_MODE, renderingparams: P1, renderingmode: *mut DWRITE_RENDERING_MODE1, gridfitmode: *mut DWRITE_GRID_FIT_MODE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
        P1: ::std::convert::Into<::windows::core::InParam<IDWriteRenderingParams>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetRecommendedRenderingMode4)(::windows::core::Vtable::as_raw(self), fontemsize, dpix, dpiy, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), issideways.into(), outlinethreshold, measuringmode, renderingparams.into().abi(), renderingmode, gridfitmode).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsCharacterLocal(&self, unicodevalue: u32) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.IsCharacterLocal)(::windows::core::Vtable::as_raw(self), unicodevalue)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsGlyphLocal(&self, glyphid: u16) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.IsGlyphLocal)(::windows::core::Vtable::as_raw(self), glyphid)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AreCharactersLocal<P0>(&self, characters: &[u16], enqueueifnotlocal: P0) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.AreCharactersLocal)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(characters.as_ptr()), characters.len() as _, enqueueifnotlocal.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AreGlyphsLocal<P0>(&self, glyphindices: &[u16], enqueueifnotlocal: P0) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.AreGlyphsLocal)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(glyphindices.as_ptr()), glyphindices.len() as _, enqueueifnotlocal.into(), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IDWriteFontFace5 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDWriteFontFace5 {}
impl ::core::fmt::Debug for IDWriteFontFace5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDWriteFontFace5").field(&self.0).finish()
    }
}
impl IDWriteFontFace5 {
    pub unsafe fn GetType(&self) -> DWRITE_FONT_FACE_TYPE {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetType)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetFiles(&self, numberoffiles: *mut u32, fontfiles: ::core::option::Option<*mut ::core::option::Option<IDWriteFontFile>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetFiles)(::windows::core::Vtable::as_raw(self), numberoffiles, ::core::mem::transmute(fontfiles.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetIndex(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetIndex)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetSimulations(&self) -> DWRITE_FONT_SIMULATIONS {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetSimulations)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSymbolFont(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.IsSymbolFont)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetMetrics(&self, fontfacemetrics: *mut DWRITE_FONT_METRICS) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetMetrics)(::windows::core::Vtable::as_raw(self), fontfacemetrics)
    }
    pub unsafe fn GetGlyphCount(&self) -> u16 {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetGlyphCount)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesignGlyphMetrics<P0>(&self, glyphindices: *const u16, glyphcount: u32, glyphmetrics: *mut DWRITE_GLYPH_METRICS, issideways: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetDesignGlyphMetrics)(::windows::core::Vtable::as_raw(self), glyphindices, glyphcount, glyphmetrics, issideways.into()).ok()
    }
    pub unsafe fn GetGlyphIndices(&self, codepoints: *const u32, codepointcount: u32, glyphindices: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetGlyphIndices)(::windows::core::Vtable::as_raw(self), codepoints, codepointcount, glyphindices).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TryGetFontTable(&self, opentypetabletag: u32, tabledata: *mut *mut ::core::ffi::c_void, tablesize: *mut u32, tablecontext: *mut *mut ::core::ffi::c_void, exists: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.TryGetFontTable)(::windows::core::Vtable::as_raw(self), opentypetabletag, tabledata, tablesize, tablecontext, exists).ok()
    }
    pub unsafe fn ReleaseFontTable(&self, tablecontext: *const ::core::ffi::c_void) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.ReleaseFontTable)(::windows::core::Vtable::as_raw(self), tablecontext)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn GetGlyphRunOutline<P0, P1, P2>(&self, emsize: f32, glyphindices: *const u16, glyphadvances: ::core::option::Option<*const f32>, glyphoffsets: ::core::option::Option<*const DWRITE_GLYPH_OFFSET>, glyphcount: u32, issideways: P0, isrighttoleft: P1, geometrysink: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
        P2: ::std::convert::Into<::windows::core::InParam<super::Direct2D::Common::ID2D1SimplifiedGeometrySink>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetGlyphRunOutline)(::windows::core::Vtable::as_raw(self), emsize, glyphindices, ::core::mem::transmute(glyphadvances.unwrap_or(::std::ptr::null())), ::core::mem::transmute(glyphoffsets.unwrap_or(::std::ptr::null())), glyphcount, issideways.into(), isrighttoleft.into(), geometrysink.into().abi()).ok()
    }
    pub unsafe fn GetRecommendedRenderingMode<P0>(&self, emsize: f32, pixelsperdip: f32, measuringmode: DWRITE_MEASURING_MODE, renderingparams: P0) -> ::windows::core::Result<DWRITE_RENDERING_MODE>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteRenderingParams>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetRecommendedRenderingMode)(::windows::core::Vtable::as_raw(self), emsize, pixelsperdip, measuringmode, renderingparams.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetGdiCompatibleMetrics(&self, emsize: f32, pixelsperdip: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, fontfacemetrics: *mut DWRITE_FONT_METRICS) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetGdiCompatibleMetrics)(::windows::core::Vtable::as_raw(self), emsize, pixelsperdip, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), fontfacemetrics).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetGdiCompatibleGlyphMetrics<P0, P1>(&self, emsize: f32, pixelsperdip: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, usegdinatural: P0, glyphindices: *const u16, glyphcount: u32, glyphmetrics: *mut DWRITE_GLYPH_METRICS, issideways: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetGdiCompatibleGlyphMetrics)(::windows::core::Vtable::as_raw(self), emsize, pixelsperdip, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), usegdinatural.into(), glyphindices, glyphcount, glyphmetrics, issideways.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMetrics2(&self, fontmetrics: *mut DWRITE_FONT_METRICS1) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetMetrics2)(::windows::core::Vtable::as_raw(self), fontmetrics)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetGdiCompatibleMetrics2(&self, emsize: f32, pixelsperdip: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, fontmetrics: *mut DWRITE_FONT_METRICS1) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetGdiCompatibleMetrics2)(::windows::core::Vtable::as_raw(self), emsize, pixelsperdip, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), fontmetrics).ok()
    }
    pub unsafe fn GetCaretMetrics(&self) -> DWRITE_CARET_METRICS {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetCaretMetrics)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        result__.assume_init()
    }
    pub unsafe fn GetUnicodeRanges(&self, unicoderanges: ::core::option::Option<&mut [DWRITE_UNICODE_RANGE]>, actualrangecount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetUnicodeRanges)(::windows::core::Vtable::as_raw(self), unicoderanges.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(unicoderanges.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), actualrangecount).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsMonospacedFont(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.IsMonospacedFont)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesignGlyphAdvances<P0>(&self, glyphcount: u32, glyphindices: *const u16, glyphadvances: *mut i32, issideways: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetDesignGlyphAdvances)(::windows::core::Vtable::as_raw(self), glyphcount, glyphindices, glyphadvances, issideways.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetGdiCompatibleGlyphAdvances<P0, P1>(&self, emsize: f32, pixelsperdip: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, usegdinatural: P0, issideways: P1, glyphcount: u32, glyphindices: *const u16, glyphadvances: *mut i32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetGdiCompatibleGlyphAdvances)(::windows::core::Vtable::as_raw(self), emsize, pixelsperdip, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), usegdinatural.into(), issideways.into(), glyphcount, glyphindices, glyphadvances).ok()
    }
    pub unsafe fn GetKerningPairAdjustments(&self, glyphcount: u32, glyphindices: *const u16, glyphadvanceadjustments: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetKerningPairAdjustments)(::windows::core::Vtable::as_raw(self), glyphcount, glyphindices, glyphadvanceadjustments).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HasKerningPairs(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.HasKerningPairs)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRecommendedRenderingMode2<P0>(&self, fontemsize: f32, dpix: f32, dpiy: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, issideways: P0, outlinethreshold: DWRITE_OUTLINE_THRESHOLD, measuringmode: DWRITE_MEASURING_MODE) -> ::windows::core::Result<DWRITE_RENDERING_MODE>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetRecommendedRenderingMode2)(::windows::core::Vtable::as_raw(self), fontemsize, dpix, dpiy, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), issideways.into(), outlinethreshold, measuringmode, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetVerticalGlyphVariants(&self, glyphcount: u32, nominalglyphindices: *const u16, verticalglyphindices: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetVerticalGlyphVariants)(::windows::core::Vtable::as_raw(self), glyphcount, nominalglyphindices, verticalglyphindices).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HasVerticalGlyphVariants(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.HasVerticalGlyphVariants)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsColorFont(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.IsColorFont)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetColorPaletteCount(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetColorPaletteCount)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetPaletteEntryCount(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetPaletteEntryCount)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetPaletteEntries(&self, colorpaletteindex: u32, firstentryindex: u32, paletteentries: &mut [DWRITE_COLOR_F]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetPaletteEntries)(::windows::core::Vtable::as_raw(self), colorpaletteindex, firstentryindex, paletteentries.len() as _, ::core::mem::transmute(paletteentries.as_ptr())).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRecommendedRenderingMode3<P0, P1>(&self, fontemsize: f32, dpix: f32, dpiy: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, issideways: P0, outlinethreshold: DWRITE_OUTLINE_THRESHOLD, measuringmode: DWRITE_MEASURING_MODE, renderingparams: P1, renderingmode: *mut DWRITE_RENDERING_MODE, gridfitmode: *mut DWRITE_GRID_FIT_MODE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
        P1: ::std::convert::Into<::windows::core::InParam<IDWriteRenderingParams>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetRecommendedRenderingMode3)(::windows::core::Vtable::as_raw(self), fontemsize, dpix, dpiy, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), issideways.into(), outlinethreshold, measuringmode, renderingparams.into().abi(), renderingmode, gridfitmode).ok()
    }
    pub unsafe fn GetFontFaceReference(&self) -> ::windows::core::Result<IDWriteFontFaceReference> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetFontFaceReference)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPanose(&self) -> DWRITE_PANOSE {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetPanose)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        result__.assume_init()
    }
    pub unsafe fn GetWeight(&self) -> DWRITE_FONT_WEIGHT {
        (::windows::core::Vtable::vtable(self).base__.base__.GetWeight)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetStretch(&self) -> DWRITE_FONT_STRETCH {
        (::windows::core::Vtable::vtable(self).base__.base__.GetStretch)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetStyle(&self) -> DWRITE_FONT_STYLE {
        (::windows::core::Vtable::vtable(self).base__.base__.GetStyle)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetFamilyNames(&self) -> ::windows::core::Result<IDWriteLocalizedStrings> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetFamilyNames)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetFaceNames(&self) -> ::windows::core::Result<IDWriteLocalizedStrings> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetFaceNames)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetInformationalStrings(&self, informationalstringid: DWRITE_INFORMATIONAL_STRING_ID, informationalstrings: *mut ::core::option::Option<IDWriteLocalizedStrings>, exists: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetInformationalStrings)(::windows::core::Vtable::as_raw(self), informationalstringid, ::core::mem::transmute(informationalstrings), exists).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HasCharacter(&self, unicodevalue: u32) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.base__.HasCharacter)(::windows::core::Vtable::as_raw(self), unicodevalue)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRecommendedRenderingMode4<P0, P1>(&self, fontemsize: f32, dpix: f32, dpiy: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, issideways: P0, outlinethreshold: DWRITE_OUTLINE_THRESHOLD, measuringmode: DWRITE_MEASURING_MODE, renderingparams: P1, renderingmode: *mut DWRITE_RENDERING_MODE1, gridfitmode: *mut DWRITE_GRID_FIT_MODE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
        P1: ::std::convert::Into<::windows::core::InParam<IDWriteRenderingParams>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.GetRecommendedRenderingMode4)(::windows::core::Vtable::as_raw(self), fontemsize, dpix, dpiy, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), issideways.into(), outlinethreshold, measuringmode, renderingparams.into().abi(), renderingmode, gridfitmode).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsCharacterLocal(&self, unicodevalue: u32) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.base__.IsCharacterLocal)(::windows::core::Vtable::as_raw(self), unicodevalue)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsGlyphLocal(&self, glyphid: u16) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.base__.IsGlyphLocal)(::windows::core::Vtable::as_raw(self), glyphid)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AreCharactersLocal<P0>(&self, characters: &[u16], enqueueifnotlocal: P0) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.AreCharactersLocal)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(characters.as_ptr()), characters.len() as _, enqueueifnotlocal.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AreGlyphsLocal<P0>(&self, glyphindices: &[u16], enqueueifnotlocal: P0) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.AreGlyphsLocal)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(glyphindices.as_ptr()), glyphindices.len() as _, enqueueifnotlocal.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetGlyphImageFormats(&self, glyphid: u16, pixelsperemfirst: u32, pixelsperemlast: u32) -> ::windows::core::Result<DWRITE_GLYPH_IMAGE_FORMATS> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetGlyphImageFormats)(::windows::core::Vtable::as_raw(self), glyphid, pixelsperemfirst, pixelsperemlast, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetGlyphImageFormats2(&self) -> DWRITE_GLYPH_IMAGE_FORMATS {
        (::windows::core::Vtable::vtable(self).base__.GetGlyphImageFormats2)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn GetGlyphImageData(&self, glyphid: u16, pixelsperem: u32, glyphimageformat: DWRITE_GLYPH_IMAGE_FORMATS, glyphdata: *mut DWRITE_GLYPH_IMAGE_DATA, glyphdatacontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetGlyphImageData)(::windows::core::Vtable::as_raw(self), glyphid, pixelsperem, glyphimageformat, glyphdata, glyphdatacontext).ok()
    }
    pub unsafe fn ReleaseGlyphImageData(&self, glyphdatacontext: *mut ::core::ffi::c_void) {
        (::windows::core::Vtable::vtable(self).base__.ReleaseGlyphImageData)(::windows::core::Vtable::as_raw(self), glyphdatacontext)
    }
}
impl ::core::cmp::PartialEq for IDWriteFontFace6 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDWriteFontFace6 {}
impl ::core::fmt::Debug for IDWriteFontFace6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDWriteFontFace6").field(&self.0).finish()
    }
}
impl IDWriteFontFace6 {
    pub unsafe fn GetType(&self) -> DWRITE_FONT_FACE_TYPE {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.GetType)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetFiles(&self, numberoffiles: *mut u32, fontfiles: ::core::option::Option<*mut ::core::option::Option<IDWriteFontFile>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.GetFiles)(::windows::core::Vtable::as_raw(self), numberoffiles, ::core::mem::transmute(fontfiles.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetIndex(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.GetIndex)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetSimulations(&self) -> DWRITE_FONT_SIMULATIONS {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.GetSimulations)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSymbolFont(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.IsSymbolFont)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetMetrics(&self, fontfacemetrics: *mut DWRITE_FONT_METRICS) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.GetMetrics)(::windows::core::Vtable::as_raw(self), fontfacemetrics)
    }
    pub unsafe fn GetGlyphCount(&self) -> u16 {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.GetGlyphCount)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesignGlyphMetrics<P0>(&self, glyphindices: *const u16, glyphcount: u32, glyphmetrics: *mut DWRITE_GLYPH_METRICS, issideways: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.GetDesignGlyphMetrics)(::windows::core::Vtable::as_raw(self), glyphindices, glyphcount, glyphmetrics, issideways.into()).ok()
    }
    pub unsafe fn GetGlyphIndices(&self, codepoints: *const u32, codepointcount: u32, glyphindices: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.GetGlyphIndices)(::windows::core::Vtable::as_raw(self), codepoints, codepointcount, glyphindices).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TryGetFontTable(&self, opentypetabletag: u32, tabledata: *mut *mut ::core::ffi::c_void, tablesize: *mut u32, tablecontext: *mut *mut ::core::ffi::c_void, exists: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.TryGetFontTable)(::windows::core::Vtable::as_raw(self), opentypetabletag, tabledata, tablesize, tablecontext, exists).ok()
    }
    pub unsafe fn ReleaseFontTable(&self, tablecontext: *const ::core::ffi::c_void) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.ReleaseFontTable)(::windows::core::Vtable::as_raw(self), tablecontext)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn GetGlyphRunOutline<P0, P1, P2>(&self, emsize: f32, glyphindices: *const u16, glyphadvances: ::core::option::Option<*const f32>, glyphoffsets: ::core::option::Option<*const DWRITE_GLYPH_OFFSET>, glyphcount: u32, issideways: P0, isrighttoleft: P1, geometrysink: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
        P2: ::std::convert::Into<::windows::core::InParam<super::Direct2D::Common::ID2D1SimplifiedGeometrySink>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.GetGlyphRunOutline)(::windows::core::Vtable::as_raw(self), emsize, glyphindices, ::core::mem::transmute(glyphadvances.unwrap_or(::std::ptr::null())), ::core::mem::transmute(glyphoffsets.unwrap_or(::std::ptr::null())), glyphcount, issideways.into(), isrighttoleft.into(), geometrysink.into().abi()).ok()
    }
    pub unsafe fn GetRecommendedRenderingMode<P0>(&self, emsize: f32, pixelsperdip: f32, measuringmode: DWRITE_MEASURING_MODE, renderingparams: P0) -> ::windows::core::Result<DWRITE_RENDERING_MODE>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteRenderingParams>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.GetRecommendedRenderingMode)(::windows::core::Vtable::as_raw(self), emsize, pixelsperdip, measuringmode, renderingparams.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetGdiCompatibleMetrics(&self, emsize: f32, pixelsperdip: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, fontfacemetrics: *mut DWRITE_FONT_METRICS) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.GetGdiCompatibleMetrics)(::windows::core::Vtable::as_raw(self), emsize, pixelsperdip, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), fontfacemetrics).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetGdiCompatibleGlyphMetrics<P0, P1>(&self, emsize: f32, pixelsperdip: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, usegdinatural: P0, glyphindices: *const u16, glyphcount: u32, glyphmetrics: *mut DWRITE_GLYPH_METRICS, issideways: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.GetGdiCompatibleGlyphMetrics)(::windows::core::Vtable::as_raw(self), emsize, pixelsperdip, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), usegdinatural.into(), glyphindices, glyphcount, glyphmetrics, issideways.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMetrics2(&self, fontmetrics: *mut DWRITE_FONT_METRICS1) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetMetrics2)(::windows::core::Vtable::as_raw(self), fontmetrics)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetGdiCompatibleMetrics2(&self, emsize: f32, pixelsperdip: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, fontmetrics: *mut DWRITE_FONT_METRICS1) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetGdiCompatibleMetrics2)(::windows::core::Vtable::as_raw(self), emsize, pixelsperdip, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), fontmetrics).ok()
    }
    pub unsafe fn GetCaretMetrics(&self) -> DWRITE_CARET_METRICS {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetCaretMetrics)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        result__.assume_init()
    }
    pub unsafe fn GetUnicodeRanges(&self, unicoderanges: ::core::option::Option<&mut [DWRITE_UNICODE_RANGE]>, actualrangecount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetUnicodeRanges)(::windows::core::Vtable::as_raw(self), unicoderanges.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(unicoderanges.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), actualrangecount).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsMonospacedFont(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.IsMonospacedFont)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesignGlyphAdvances<P0>(&self, glyphcount: u32, glyphindices: *const u16, glyphadvances: *mut i32, issideways: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetDesignGlyphAdvances)(::windows::core::Vtable::as_raw(self), glyphcount, glyphindices, glyphadvances, issideways.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetGdiCompatibleGlyphAdvances<P0, P1>(&self, emsize: f32, pixelsperdip: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, usegdinatural: P0, issideways: P1, glyphcount: u32, glyphindices: *const u16, glyphadvances: *mut i32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetGdiCompatibleGlyphAdvances)(::windows::core::Vtable::as_raw(self), emsize, pixelsperdip, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), usegdinatural.into(), issideways.into(), glyphcount, glyphindices, glyphadvances).ok()
    }
    pub unsafe fn GetKerningPairAdjustments(&self, glyphcount: u32, glyphindices: *const u16, glyphadvanceadjustments: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetKerningPairAdjustments)(::windows::core::Vtable::as_raw(self), glyphcount, glyphindices, glyphadvanceadjustments).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HasKerningPairs(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.HasKerningPairs)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRecommendedRenderingMode2<P0>(&self, fontemsize: f32, dpix: f32, dpiy: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, issideways: P0, outlinethreshold: DWRITE_OUTLINE_THRESHOLD, measuringmode: DWRITE_MEASURING_MODE) -> ::windows::core::Result<DWRITE_RENDERING_MODE>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetRecommendedRenderingMode2)(::windows::core::Vtable::as_raw(self), fontemsize, dpix, dpiy, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), issideways.into(), outlinethreshold, measuringmode, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetVerticalGlyphVariants(&self, glyphcount: u32, nominalglyphindices: *const u16, verticalglyphindices: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetVerticalGlyphVariants)(::windows::core::Vtable::as_raw(self), glyphcount, nominalglyphindices, verticalglyphindices).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HasVerticalGlyphVariants(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.HasVerticalGlyphVariants)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsColorFont(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.IsColorFont)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetColorPaletteCount(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetColorPaletteCount)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetPaletteEntryCount(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetPaletteEntryCount)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetPaletteEntries(&self, colorpaletteindex: u32, firstentryindex: u32, paletteentries: &mut [DWRITE_COLOR_F]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetPaletteEntries)(::windows::core::Vtable::as_raw(self), colorpaletteindex, firstentryindex, paletteentries.len() as _, ::core::mem::transmute(paletteentries.as_ptr())).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRecommendedRenderingMode3<P0, P1>(&self, fontemsize: f32, dpix: f32, dpiy: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, issideways: P0, outlinethreshold: DWRITE_OUTLINE_THRESHOLD, measuringmode: DWRITE_MEASURING_MODE, renderingparams: P1, renderingmode: *mut DWRITE_RENDERING_MODE, gridfitmode: *mut DWRITE_GRID_FIT_MODE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
        P1: ::std::convert::Into<::windows::core::InParam<IDWriteRenderingParams>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetRecommendedRenderingMode3)(::windows::core::Vtable::as_raw(self), fontemsize, dpix, dpiy, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), issideways.into(), outlinethreshold, measuringmode, renderingparams.into().abi(), renderingmode, gridfitmode).ok()
    }
    pub unsafe fn GetFontFaceReference(&self) -> ::windows::core::Result<IDWriteFontFaceReference> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetFontFaceReference)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPanose(&self) -> DWRITE_PANOSE {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetPanose)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        result__.assume_init()
    }
    pub unsafe fn GetWeight(&self) -> DWRITE_FONT_WEIGHT {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetWeight)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetStretch(&self) -> DWRITE_FONT_STRETCH {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetStretch)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetStyle(&self) -> DWRITE_FONT_STYLE {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetStyle)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetFamilyNames(&self) -> ::windows::core::Result<IDWriteLocalizedStrings> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetFamilyNames)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetFaceNames(&self) -> ::windows::core::Result<IDWriteLocalizedStrings> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetFaceNames)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetInformationalStrings(&self, informationalstringid: DWRITE_INFORMATIONAL_STRING_ID, informationalstrings: *mut ::core::option::Option<IDWriteLocalizedStrings>, exists: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetInformationalStrings)(::windows::core::Vtable::as_raw(self), informationalstringid, ::core::mem::transmute(informationalstrings), exists).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HasCharacter(&self, unicodevalue: u32) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.HasCharacter)(::windows::core::Vtable::as_raw(self), unicodevalue)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRecommendedRenderingMode4<P0, P1>(&self, fontemsize: f32, dpix: f32, dpiy: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, issideways: P0, outlinethreshold: DWRITE_OUTLINE_THRESHOLD, measuringmode: DWRITE_MEASURING_MODE, renderingparams: P1, renderingmode: *mut DWRITE_RENDERING_MODE1, gridfitmode: *mut DWRITE_GRID_FIT_MODE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
        P1: ::std::convert::Into<::windows::core::InParam<IDWriteRenderingParams>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetRecommendedRenderingMode4)(::windows::core::Vtable::as_raw(self), fontemsize, dpix, dpiy, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), issideways.into(), outlinethreshold, measuringmode, renderingparams.into().abi(), renderingmode, gridfitmode).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsCharacterLocal(&self, unicodevalue: u32) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.IsCharacterLocal)(::windows::core::Vtable::as_raw(self), unicodevalue)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsGlyphLocal(&self, glyphid: u16) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.IsGlyphLocal)(::windows::core::Vtable::as_raw(self), glyphid)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AreCharactersLocal<P0>(&self, characters: &[u16], enqueueifnotlocal: P0) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.AreCharactersLocal)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(characters.as_ptr()), characters.len() as _, enqueueifnotlocal.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AreGlyphsLocal<P0>(&self, glyphindices: &[u16], enqueueifnotlocal: P0) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.AreGlyphsLocal)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(glyphindices.as_ptr()), glyphindices.len() as _, enqueueifnotlocal.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetGlyphImageFormats(&self, glyphid: u16, pixelsperemfirst: u32, pixelsperemlast: u32) -> ::windows::core::Result<DWRITE_GLYPH_IMAGE_FORMATS> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetGlyphImageFormats)(::windows::core::Vtable::as_raw(self), glyphid, pixelsperemfirst, pixelsperemlast, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetGlyphImageFormats2(&self) -> DWRITE_GLYPH_IMAGE_FORMATS {
        (::windows::core::Vtable::vtable(self).base__.base__.GetGlyphImageFormats2)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn GetGlyphImageData(&self, glyphid: u16, pixelsperem: u32, glyphimageformat: DWRITE_GLYPH_IMAGE_FORMATS, glyphdata: *mut DWRITE_GLYPH_IMAGE_DATA, glyphdatacontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetGlyphImageData)(::windows::core::Vtable::as_raw(self), glyphid, pixelsperem, glyphimageformat, glyphdata, glyphdatacontext).ok()
    }
    pub unsafe fn ReleaseGlyphImageData(&self, glyphdatacontext: *mut ::core::ffi::c_void) {
        (::windows::core::Vtable::vtable(self).base__.base__.ReleaseGlyphImageData)(::windows::core::Vtable::as_raw(self), glyphdatacontext)
    }
    pub unsafe fn GetFontAxisValueCount(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.GetFontAxisValueCount)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetFontAxisValues(&self, fontaxisvalues: &mut [DWRITE_FONT_AXIS_VALUE]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetFontAxisValues)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(fontaxisvalues.as_ptr()), fontaxisvalues.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HasVariations(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.HasVariations)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetFontResource(&self) -> ::windows::core::Result<IDWriteFontResource> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFontResource)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Equals<P0>(&self, fontface: P0) -> super::super::Foundation::BOOL
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteFontFace>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Equals)(::windows::core::Vtable::as_raw(self), fontface.into().abi())
    }
}
impl ::core::cmp::PartialEq for IDWriteFontFaceReference {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDWriteFontFaceReference {}
impl ::core::fmt::Debug for IDWriteFontFaceReference {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDWriteFontFaceReference").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDWriteFontFaceReference1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDWriteFontFaceReference1 {}
impl ::core::fmt::Debug for IDWriteFontFaceReference1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDWriteFontFaceReference1").field(&self.0).finish()
    }
}
impl IDWriteFontFaceReference1 {
    pub unsafe fn CreateFontFace(&self) -> ::windows::core::Result<IDWriteFontFace3> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateFontFace)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateFontFaceWithSimulations(&self, fontfacesimulationflags: DWRITE_FONT_SIMULATIONS) -> ::windows::core::Result<IDWriteFontFace3> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateFontFaceWithSimulations)(::windows::core::Vtable::as_raw(self), fontfacesimulationflags, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Equals<P0>(&self, fontfacereference: P0) -> super::super::Foundation::BOOL
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteFontFaceReference>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Equals)(::windows::core::Vtable::as_raw(self), fontfacereference.into().abi())
    }
    pub unsafe fn GetFontFaceIndex(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.GetFontFaceIndex)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetSimulations(&self) -> DWRITE_FONT_SIMULATIONS {
        (::windows::core::Vtable::vtable(self).base__.GetSimulations)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetFontFile(&self) -> ::windows::core::Result<IDWriteFontFile> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFontFile)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetLocalFileSize(&self) -> u64 {
        (::windows::core::Vtable::vtable(self).base__.GetLocalFileSize)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetFileSize(&self) -> u64 {
        (::windows::core::Vtable::vtable(self).base__.GetFileSize)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFileTime(&self) -> ::windows::core::Result<super::super::Foundation::FILETIME> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFileTime)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetLocality(&self) -> DWRITE_LOCALITY {
        (::windows::core::Vtable::vtable(self).base__.GetLocality)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn EnqueueFontDownloadRequest(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.EnqueueFontDownloadRequest)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn EnqueueCharacterDownloadRequest(&self, characters: &[u16]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.EnqueueCharacterDownloadRequest)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(characters.as_ptr()), characters.len() as _).ok()
    }
    pub unsafe fn EnqueueGlyphDownloadRequest(&self, glyphindices: &[u16]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.EnqueueGlyphDownloadRequest)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(glyphindices.as_ptr()), glyphindices.len() as _).ok()
    }
    pub unsafe fn EnqueueFileFragmentDownloadRequest(&self, fileoffset: u64, fragmentsize: u64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.EnqueueFileFragmentDownloadRequest)(::windows::core::Vtable::as_raw(self), fileoffset, fragmentsize).ok()
    }
}
impl ::core::cmp::PartialEq for IDWriteFontFallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDWriteFontFallback {}
impl ::core::fmt::Debug for IDWriteFontFallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDWriteFontFallback").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDWriteFontFallback1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDWriteFontFallback1 {}
impl ::core::fmt::Debug for IDWriteFontFallback1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDWriteFontFallback1").field(&self.0).finish()
    }
}
impl IDWriteFontFallback1 {
    pub unsafe fn MapCharacters<P0, P1, P2>(&self, analysissource: P0, textposition: u32, textlength: u32, basefontcollection: P1, basefamilyname: P2, baseweight: DWRITE_FONT_WEIGHT, basestyle: DWRITE_FONT_STYLE, basestretch: DWRITE_FONT_STRETCH, mappedlength: *mut u32, mappedfont: *mut ::core::option::Option<IDWriteFont>, scale: *mut f32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteTextAnalysisSource>>,
        P1: ::std::convert::Into<::windows::core::InParam<IDWriteFontCollection>>,
        P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.MapCharacters)(::windows::core::Vtable::as_raw(self), analysissource.into().abi(), textposition, textlength, basefontcollection.into().abi(), basefamilyname.into().abi(), baseweight, basestyle, basestretch, mappedlength, ::core::mem::transmute(mappedfont), scale).ok()
    }
}
impl ::core::cmp::PartialEq for IDWriteFontFallbackBuilder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDWriteFontFallbackBuilder {}
impl ::core::fmt::Debug for IDWriteFontFallbackBuilder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDWriteFontFallbackBuilder").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDWriteFontFamily {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDWriteFontFamily {}
impl ::core::fmt::Debug for IDWriteFontFamily {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDWriteFontFamily").field(&self.0).finish()
    }
}
impl IDWriteFontFamily {
    pub unsafe fn GetFontCollection(&self) -> ::windows::core::Result<IDWriteFontCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFontCollection)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetFontCount(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.GetFontCount)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetFont(&self, index: u32) -> ::windows::core::Result<IDWriteFont> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFont)(::windows::core::Vtable::as_raw(self), index, result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IDWriteFontFamily1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDWriteFontFamily1 {}
impl ::core::fmt::Debug for IDWriteFontFamily1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDWriteFontFamily1").field(&self.0).finish()
    }
}
impl IDWriteFontFamily1 {
    pub unsafe fn GetFontCollection(&self) -> ::windows::core::Result<IDWriteFontCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetFontCollection)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetFontCount(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.base__.GetFontCount)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetFont(&self, index: u32) -> ::windows::core::Result<IDWriteFont> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetFont)(::windows::core::Vtable::as_raw(self), index, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetFamilyNames(&self) -> ::windows::core::Result<IDWriteLocalizedStrings> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFamilyNames)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetFirstMatchingFont(&self, weight: DWRITE_FONT_WEIGHT, stretch: DWRITE_FONT_STRETCH, style: DWRITE_FONT_STYLE) -> ::windows::core::Result<IDWriteFont> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFirstMatchingFont)(::windows::core::Vtable::as_raw(self), weight, stretch, style, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetMatchingFonts(&self, weight: DWRITE_FONT_WEIGHT, stretch: DWRITE_FONT_STRETCH, style: DWRITE_FONT_STYLE) -> ::windows::core::Result<IDWriteFontList> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetMatchingFonts)(::windows::core::Vtable::as_raw(self), weight, stretch, style, result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IDWriteFontFamily2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDWriteFontFamily2 {}
impl ::core::fmt::Debug for IDWriteFontFamily2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDWriteFontFamily2").field(&self.0).finish()
    }
}
impl IDWriteFontFamily2 {
    pub unsafe fn GetFontCollection(&self) -> ::windows::core::Result<IDWriteFontCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetFontCollection)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetFontCount(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetFontCount)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetFont(&self, index: u32) -> ::windows::core::Result<IDWriteFont> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetFont)(::windows::core::Vtable::as_raw(self), index, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetFamilyNames(&self) -> ::windows::core::Result<IDWriteLocalizedStrings> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetFamilyNames)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetFirstMatchingFont(&self, weight: DWRITE_FONT_WEIGHT, stretch: DWRITE_FONT_STRETCH, style: DWRITE_FONT_STYLE) -> ::windows::core::Result<IDWriteFont> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetFirstMatchingFont)(::windows::core::Vtable::as_raw(self), weight, stretch, style, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetMatchingFonts(&self, weight: DWRITE_FONT_WEIGHT, stretch: DWRITE_FONT_STRETCH, style: DWRITE_FONT_STYLE) -> ::windows::core::Result<IDWriteFontList> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetMatchingFonts)(::windows::core::Vtable::as_raw(self), weight, stretch, style, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetFontLocality(&self, listindex: u32) -> DWRITE_LOCALITY {
        (::windows::core::Vtable::vtable(self).base__.GetFontLocality)(::windows::core::Vtable::as_raw(self), listindex)
    }
    pub unsafe fn GetFont2(&self, listindex: u32) -> ::windows::core::Result<IDWriteFont3> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFont2)(::windows::core::Vtable::as_raw(self), listindex, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetFontFaceReference(&self, listindex: u32) -> ::windows::core::Result<IDWriteFontFaceReference> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFontFaceReference)(::windows::core::Vtable::as_raw(self), listindex, result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IDWriteFontFile {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDWriteFontFile {}
impl ::core::fmt::Debug for IDWriteFontFile {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDWriteFontFile").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDWriteFontFileEnumerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDWriteFontFileEnumerator {}
impl ::core::fmt::Debug for IDWriteFontFileEnumerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDWriteFontFileEnumerator").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDWriteFontFileLoader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDWriteFontFileLoader {}
impl ::core::fmt::Debug for IDWriteFontFileLoader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDWriteFontFileLoader").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDWriteFontFileStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDWriteFontFileStream {}
impl ::core::fmt::Debug for IDWriteFontFileStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDWriteFontFileStream").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDWriteFontList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDWriteFontList {}
impl ::core::fmt::Debug for IDWriteFontList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDWriteFontList").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDWriteFontList1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDWriteFontList1 {}
impl ::core::fmt::Debug for IDWriteFontList1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDWriteFontList1").field(&self.0).finish()
    }
}
impl IDWriteFontList1 {
    pub unsafe fn GetFontCollection(&self) -> ::windows::core::Result<IDWriteFontCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFontCollection)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetFontCount(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.GetFontCount)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetFont(&self, index: u32) -> ::windows::core::Result<IDWriteFont> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFont)(::windows::core::Vtable::as_raw(self), index, result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IDWriteFontList2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDWriteFontList2 {}
impl ::core::fmt::Debug for IDWriteFontList2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDWriteFontList2").field(&self.0).finish()
    }
}
impl IDWriteFontList2 {
    pub unsafe fn GetFontCollection(&self) -> ::windows::core::Result<IDWriteFontCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetFontCollection)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetFontCount(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.base__.GetFontCount)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetFont(&self, index: u32) -> ::windows::core::Result<IDWriteFont> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetFont)(::windows::core::Vtable::as_raw(self), index, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetFontLocality(&self, listindex: u32) -> DWRITE_LOCALITY {
        (::windows::core::Vtable::vtable(self).base__.GetFontLocality)(::windows::core::Vtable::as_raw(self), listindex)
    }
    pub unsafe fn GetFont2(&self, listindex: u32) -> ::windows::core::Result<IDWriteFont3> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFont2)(::windows::core::Vtable::as_raw(self), listindex, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetFontFaceReference(&self, listindex: u32) -> ::windows::core::Result<IDWriteFontFaceReference> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFontFaceReference)(::windows::core::Vtable::as_raw(self), listindex, result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IDWriteFontResource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDWriteFontResource {}
impl ::core::fmt::Debug for IDWriteFontResource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDWriteFontResource").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDWriteFontSet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDWriteFontSet {}
impl ::core::fmt::Debug for IDWriteFontSet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDWriteFontSet").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDWriteFontSet1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDWriteFontSet1 {}
impl ::core::fmt::Debug for IDWriteFontSet1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDWriteFontSet1").field(&self.0).finish()
    }
}
impl IDWriteFontSet1 {
    pub unsafe fn GetFontCount(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.GetFontCount)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetFontFaceReference(&self, listindex: u32) -> ::windows::core::Result<IDWriteFontFaceReference> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFontFaceReference)(::windows::core::Vtable::as_raw(self), listindex, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindFontFaceReference<P0>(&self, fontfacereference: P0, listindex: *mut u32, exists: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteFontFaceReference>>,
    {
        (::windows::core::Vtable::vtable(self).base__.FindFontFaceReference)(::windows::core::Vtable::as_raw(self), fontfacereference.into().abi(), listindex, exists).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindFontFace<P0>(&self, fontface: P0, listindex: *mut u32, exists: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteFontFace>>,
    {
        (::windows::core::Vtable::vtable(self).base__.FindFontFace)(::windows::core::Vtable::as_raw(self), fontface.into().abi(), listindex, exists).ok()
    }
    pub unsafe fn GetPropertyValues(&self, propertyid: DWRITE_FONT_PROPERTY_ID) -> ::windows::core::Result<IDWriteStringList> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetPropertyValues)(::windows::core::Vtable::as_raw(self), propertyid, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPropertyValues2<P0>(&self, propertyid: DWRITE_FONT_PROPERTY_ID, preferredlocalenames: P0) -> ::windows::core::Result<IDWriteStringList>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetPropertyValues2)(::windows::core::Vtable::as_raw(self), propertyid, preferredlocalenames.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPropertyValues3(&self, listindex: u32, propertyid: DWRITE_FONT_PROPERTY_ID, exists: *mut super::super::Foundation::BOOL, values: *mut ::core::option::Option<IDWriteLocalizedStrings>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetPropertyValues3)(::windows::core::Vtable::as_raw(self), listindex, propertyid, exists, ::core::mem::transmute(values)).ok()
    }
    pub unsafe fn GetPropertyOccurrenceCount(&self, property: *const DWRITE_FONT_PROPERTY) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetPropertyOccurrenceCount)(::windows::core::Vtable::as_raw(self), property, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetMatchingFonts<P0>(&self, familyname: P0, fontweight: DWRITE_FONT_WEIGHT, fontstretch: DWRITE_FONT_STRETCH, fontstyle: DWRITE_FONT_STYLE) -> ::windows::core::Result<IDWriteFontSet>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetMatchingFonts)(::windows::core::Vtable::as_raw(self), familyname.into().abi(), fontweight, fontstretch, fontstyle, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetMatchingFonts2(&self, properties: &[DWRITE_FONT_PROPERTY]) -> ::windows::core::Result<IDWriteFontSet> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetMatchingFonts2)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(properties.as_ptr()), properties.len() as _, result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IDWriteFontSet2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDWriteFontSet2 {}
impl ::core::fmt::Debug for IDWriteFontSet2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDWriteFontSet2").field(&self.0).finish()
    }
}
impl IDWriteFontSet2 {
    pub unsafe fn GetFontCount(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.base__.GetFontCount)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetFontFaceReference(&self, listindex: u32) -> ::windows::core::Result<IDWriteFontFaceReference> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetFontFaceReference)(::windows::core::Vtable::as_raw(self), listindex, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindFontFaceReference<P0>(&self, fontfacereference: P0, listindex: *mut u32, exists: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteFontFaceReference>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.FindFontFaceReference)(::windows::core::Vtable::as_raw(self), fontfacereference.into().abi(), listindex, exists).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindFontFace<P0>(&self, fontface: P0, listindex: *mut u32, exists: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteFontFace>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.FindFontFace)(::windows::core::Vtable::as_raw(self), fontface.into().abi(), listindex, exists).ok()
    }
    pub unsafe fn GetPropertyValues(&self, propertyid: DWRITE_FONT_PROPERTY_ID) -> ::windows::core::Result<IDWriteStringList> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetPropertyValues)(::windows::core::Vtable::as_raw(self), propertyid, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPropertyValues2<P0>(&self, propertyid: DWRITE_FONT_PROPERTY_ID, preferredlocalenames: P0) -> ::windows::core::Result<IDWriteStringList>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetPropertyValues2)(::windows::core::Vtable::as_raw(self), propertyid, preferredlocalenames.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPropertyValues3(&self, listindex: u32, propertyid: DWRITE_FONT_PROPERTY_ID, exists: *mut super::super::Foundation::BOOL, values: *mut ::core::option::Option<IDWriteLocalizedStrings>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetPropertyValues3)(::windows::core::Vtable::as_raw(self), listindex, propertyid, exists, ::core::mem::transmute(values)).ok()
    }
    pub unsafe fn GetPropertyOccurrenceCount(&self, property: *const DWRITE_FONT_PROPERTY) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetPropertyOccurrenceCount)(::windows::core::Vtable::as_raw(self), property, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetMatchingFonts<P0>(&self, familyname: P0, fontweight: DWRITE_FONT_WEIGHT, fontstretch: DWRITE_FONT_STRETCH, fontstyle: DWRITE_FONT_STYLE) -> ::windows::core::Result<IDWriteFontSet>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetMatchingFonts)(::windows::core::Vtable::as_raw(self), familyname.into().abi(), fontweight, fontstretch, fontstyle, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetMatchingFonts2(&self, properties: &[DWRITE_FONT_PROPERTY]) -> ::windows::core::Result<IDWriteFontSet> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetMatchingFonts2)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(properties.as_ptr()), properties.len() as _, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetMatchingFonts3(&self, fontproperty: ::core::option::Option<*const DWRITE_FONT_PROPERTY>, fontaxisvalues: &[DWRITE_FONT_AXIS_VALUE]) -> ::windows::core::Result<IDWriteFontSet1> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetMatchingFonts3)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(fontproperty.unwrap_or(::std::ptr::null())), ::core::mem::transmute(fontaxisvalues.as_ptr()), fontaxisvalues.len() as _, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetFirstFontResources(&self) -> ::windows::core::Result<IDWriteFontSet1> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFirstFontResources)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetFilteredFonts(&self, indices: &[u32]) -> ::windows::core::Result<IDWriteFontSet1> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFilteredFonts)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(indices.as_ptr()), indices.len() as _, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFilteredFonts2<P0>(&self, fontaxisranges: &[DWRITE_FONT_AXIS_RANGE], selectanyrange: P0) -> ::windows::core::Result<IDWriteFontSet1>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFilteredFonts2)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(fontaxisranges.as_ptr()), fontaxisranges.len() as _, selectanyrange.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFilteredFonts3<P0>(&self, properties: ::core::option::Option<&[DWRITE_FONT_PROPERTY]>, selectanyproperty: P0) -> ::windows::core::Result<IDWriteFontSet1>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFilteredFonts3)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(properties.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), properties.as_deref().map_or(0, |slice| slice.len() as _), selectanyproperty.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFilteredFontIndices<P0>(&self, fontaxisranges: &[DWRITE_FONT_AXIS_RANGE], selectanyrange: P0, indices: &mut [u32], actualindexcount: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetFilteredFontIndices)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(fontaxisranges.as_ptr()), fontaxisranges.len() as _, selectanyrange.into(), ::core::mem::transmute(indices.as_ptr()), indices.len() as _, actualindexcount).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFilteredFontIndices2<P0>(&self, properties: &[DWRITE_FONT_PROPERTY], selectanyproperty: P0, indices: &mut [u32], actualindexcount: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetFilteredFontIndices2)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(properties.as_ptr()), properties.len() as _, selectanyproperty.into(), ::core::mem::transmute(indices.as_ptr()), indices.len() as _, actualindexcount).ok()
    }
    pub unsafe fn GetFontAxisRanges(&self, listindex: u32, fontaxisranges: &mut [DWRITE_FONT_AXIS_RANGE], actualfontaxisrangecount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetFontAxisRanges)(::windows::core::Vtable::as_raw(self), listindex, ::core::mem::transmute(fontaxisranges.as_ptr()), fontaxisranges.len() as _, actualfontaxisrangecount).ok()
    }
    pub unsafe fn GetFontAxisRanges2(&self, fontaxisranges: &mut [DWRITE_FONT_AXIS_RANGE], actualfontaxisrangecount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetFontAxisRanges2)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(fontaxisranges.as_ptr()), fontaxisranges.len() as _, actualfontaxisrangecount).ok()
    }
    pub unsafe fn GetFontFaceReference2(&self, listindex: u32) -> ::windows::core::Result<IDWriteFontFaceReference1> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFontFaceReference2)(::windows::core::Vtable::as_raw(self), listindex, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateFontResource(&self, listindex: u32) -> ::windows::core::Result<IDWriteFontResource> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateFontResource)(::windows::core::Vtable::as_raw(self), listindex, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateFontFace(&self, listindex: u32) -> ::windows::core::Result<IDWriteFontFace5> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateFontFace)(::windows::core::Vtable::as_raw(self), listindex, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetFontLocality(&self, listindex: u32) -> DWRITE_LOCALITY {
        (::windows::core::Vtable::vtable(self).base__.GetFontLocality)(::windows::core::Vtable::as_raw(self), listindex)
    }
}
impl ::core::cmp::PartialEq for IDWriteFontSet3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDWriteFontSet3 {}
impl ::core::fmt::Debug for IDWriteFontSet3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDWriteFontSet3").field(&self.0).finish()
    }
}
impl IDWriteFontSet3 {
    pub unsafe fn GetFontCount(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetFontCount)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetFontFaceReference(&self, listindex: u32) -> ::windows::core::Result<IDWriteFontFaceReference> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetFontFaceReference)(::windows::core::Vtable::as_raw(self), listindex, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindFontFaceReference<P0>(&self, fontfacereference: P0, listindex: *mut u32, exists: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteFontFaceReference>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.FindFontFaceReference)(::windows::core::Vtable::as_raw(self), fontfacereference.into().abi(), listindex, exists).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindFontFace<P0>(&self, fontface: P0, listindex: *mut u32, exists: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteFontFace>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.FindFontFace)(::windows::core::Vtable::as_raw(self), fontface.into().abi(), listindex, exists).ok()
    }
    pub unsafe fn GetPropertyValues(&self, propertyid: DWRITE_FONT_PROPERTY_ID) -> ::windows::core::Result<IDWriteStringList> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetPropertyValues)(::windows::core::Vtable::as_raw(self), propertyid, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPropertyValues2<P0>(&self, propertyid: DWRITE_FONT_PROPERTY_ID, preferredlocalenames: P0) -> ::windows::core::Result<IDWriteStringList>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetPropertyValues2)(::windows::core::Vtable::as_raw(self), propertyid, preferredlocalenames.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPropertyValues3(&self, listindex: u32, propertyid: DWRITE_FONT_PROPERTY_ID, exists: *mut super::super::Foundation::BOOL, values: *mut ::core::option::Option<IDWriteLocalizedStrings>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetPropertyValues3)(::windows::core::Vtable::as_raw(self), listindex, propertyid, exists, ::core::mem::transmute(values)).ok()
    }
    pub unsafe fn GetPropertyOccurrenceCount(&self, property: *const DWRITE_FONT_PROPERTY) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetPropertyOccurrenceCount)(::windows::core::Vtable::as_raw(self), property, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetMatchingFonts<P0>(&self, familyname: P0, fontweight: DWRITE_FONT_WEIGHT, fontstretch: DWRITE_FONT_STRETCH, fontstyle: DWRITE_FONT_STYLE) -> ::windows::core::Result<IDWriteFontSet>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetMatchingFonts)(::windows::core::Vtable::as_raw(self), familyname.into().abi(), fontweight, fontstretch, fontstyle, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetMatchingFonts2(&self, properties: &[DWRITE_FONT_PROPERTY]) -> ::windows::core::Result<IDWriteFontSet> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetMatchingFonts2)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(properties.as_ptr()), properties.len() as _, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetMatchingFonts3(&self, fontproperty: ::core::option::Option<*const DWRITE_FONT_PROPERTY>, fontaxisvalues: &[DWRITE_FONT_AXIS_VALUE]) -> ::windows::core::Result<IDWriteFontSet1> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetMatchingFonts3)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(fontproperty.unwrap_or(::std::ptr::null())), ::core::mem::transmute(fontaxisvalues.as_ptr()), fontaxisvalues.len() as _, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetFirstFontResources(&self) -> ::windows::core::Result<IDWriteFontSet1> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetFirstFontResources)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetFilteredFonts(&self, indices: &[u32]) -> ::windows::core::Result<IDWriteFontSet1> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetFilteredFonts)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(indices.as_ptr()), indices.len() as _, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFilteredFonts2<P0>(&self, fontaxisranges: &[DWRITE_FONT_AXIS_RANGE], selectanyrange: P0) -> ::windows::core::Result<IDWriteFontSet1>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetFilteredFonts2)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(fontaxisranges.as_ptr()), fontaxisranges.len() as _, selectanyrange.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFilteredFonts3<P0>(&self, properties: ::core::option::Option<&[DWRITE_FONT_PROPERTY]>, selectanyproperty: P0) -> ::windows::core::Result<IDWriteFontSet1>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetFilteredFonts3)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(properties.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), properties.as_deref().map_or(0, |slice| slice.len() as _), selectanyproperty.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFilteredFontIndices<P0>(&self, fontaxisranges: &[DWRITE_FONT_AXIS_RANGE], selectanyrange: P0, indices: &mut [u32], actualindexcount: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.GetFilteredFontIndices)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(fontaxisranges.as_ptr()), fontaxisranges.len() as _, selectanyrange.into(), ::core::mem::transmute(indices.as_ptr()), indices.len() as _, actualindexcount).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFilteredFontIndices2<P0>(&self, properties: &[DWRITE_FONT_PROPERTY], selectanyproperty: P0, indices: &mut [u32], actualindexcount: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.GetFilteredFontIndices2)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(properties.as_ptr()), properties.len() as _, selectanyproperty.into(), ::core::mem::transmute(indices.as_ptr()), indices.len() as _, actualindexcount).ok()
    }
    pub unsafe fn GetFontAxisRanges(&self, listindex: u32, fontaxisranges: &mut [DWRITE_FONT_AXIS_RANGE], actualfontaxisrangecount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetFontAxisRanges)(::windows::core::Vtable::as_raw(self), listindex, ::core::mem::transmute(fontaxisranges.as_ptr()), fontaxisranges.len() as _, actualfontaxisrangecount).ok()
    }
    pub unsafe fn GetFontAxisRanges2(&self, fontaxisranges: &mut [DWRITE_FONT_AXIS_RANGE], actualfontaxisrangecount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetFontAxisRanges2)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(fontaxisranges.as_ptr()), fontaxisranges.len() as _, actualfontaxisrangecount).ok()
    }
    pub unsafe fn GetFontFaceReference2(&self, listindex: u32) -> ::windows::core::Result<IDWriteFontFaceReference1> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetFontFaceReference2)(::windows::core::Vtable::as_raw(self), listindex, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateFontResource(&self, listindex: u32) -> ::windows::core::Result<IDWriteFontResource> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateFontResource)(::windows::core::Vtable::as_raw(self), listindex, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateFontFace(&self, listindex: u32) -> ::windows::core::Result<IDWriteFontFace5> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateFontFace)(::windows::core::Vtable::as_raw(self), listindex, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetFontLocality(&self, listindex: u32) -> DWRITE_LOCALITY {
        (::windows::core::Vtable::vtable(self).base__.base__.GetFontLocality)(::windows::core::Vtable::as_raw(self), listindex)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetExpirationEvent(&self) -> super::super::Foundation::HANDLE {
        (::windows::core::Vtable::vtable(self).base__.GetExpirationEvent)(::windows::core::Vtable::as_raw(self))
    }
}
impl ::core::cmp::PartialEq for IDWriteFontSetBuilder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDWriteFontSetBuilder {}
impl ::core::fmt::Debug for IDWriteFontSetBuilder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDWriteFontSetBuilder").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDWriteFontSetBuilder1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDWriteFontSetBuilder1 {}
impl ::core::fmt::Debug for IDWriteFontSetBuilder1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDWriteFontSetBuilder1").field(&self.0).finish()
    }
}
impl IDWriteFontSetBuilder1 {
    pub unsafe fn AddFontFaceReference<P0>(&self, fontfacereference: P0, properties: &[DWRITE_FONT_PROPERTY]) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteFontFaceReference>>,
    {
        (::windows::core::Vtable::vtable(self).base__.AddFontFaceReference)(::windows::core::Vtable::as_raw(self), fontfacereference.into().abi(), ::core::mem::transmute(properties.as_ptr()), properties.len() as _).ok()
    }
    pub unsafe fn AddFontFaceReference2<P0>(&self, fontfacereference: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteFontFaceReference>>,
    {
        (::windows::core::Vtable::vtable(self).base__.AddFontFaceReference2)(::windows::core::Vtable::as_raw(self), fontfacereference.into().abi()).ok()
    }
    pub unsafe fn AddFontSet<P0>(&self, fontset: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteFontSet>>,
    {
        (::windows::core::Vtable::vtable(self).base__.AddFontSet)(::windows::core::Vtable::as_raw(self), fontset.into().abi()).ok()
    }
    pub unsafe fn CreateFontSet(&self) -> ::windows::core::Result<IDWriteFontSet> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateFontSet)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IDWriteFontSetBuilder2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDWriteFontSetBuilder2 {}
impl ::core::fmt::Debug for IDWriteFontSetBuilder2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDWriteFontSetBuilder2").field(&self.0).finish()
    }
}
impl IDWriteFontSetBuilder2 {
    pub unsafe fn AddFontFaceReference<P0>(&self, fontfacereference: P0, properties: &[DWRITE_FONT_PROPERTY]) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteFontFaceReference>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.AddFontFaceReference)(::windows::core::Vtable::as_raw(self), fontfacereference.into().abi(), ::core::mem::transmute(properties.as_ptr()), properties.len() as _).ok()
    }
    pub unsafe fn AddFontFaceReference2<P0>(&self, fontfacereference: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteFontFaceReference>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.AddFontFaceReference2)(::windows::core::Vtable::as_raw(self), fontfacereference.into().abi()).ok()
    }
    pub unsafe fn AddFontSet<P0>(&self, fontset: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteFontSet>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.AddFontSet)(::windows::core::Vtable::as_raw(self), fontset.into().abi()).ok()
    }
    pub unsafe fn CreateFontSet(&self) -> ::windows::core::Result<IDWriteFontSet> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateFontSet)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn AddFontFile<P0>(&self, fontfile: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteFontFile>>,
    {
        (::windows::core::Vtable::vtable(self).base__.AddFontFile)(::windows::core::Vtable::as_raw(self), fontfile.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for IDWriteGdiInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDWriteGdiInterop {}
impl ::core::fmt::Debug for IDWriteGdiInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDWriteGdiInterop").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDWriteGdiInterop1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDWriteGdiInterop1 {}
impl ::core::fmt::Debug for IDWriteGdiInterop1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDWriteGdiInterop1").field(&self.0).finish()
    }
}
impl IDWriteGdiInterop1 {
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn CreateFontFromLOGFONT(&self, logfont: *const super::Gdi::LOGFONTW) -> ::windows::core::Result<IDWriteFont> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateFontFromLOGFONT)(::windows::core::Vtable::as_raw(self), logfont, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn ConvertFontToLOGFONT<P0>(&self, font: P0, logfont: *mut super::Gdi::LOGFONTW, issystemfont: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteFont>>,
    {
        (::windows::core::Vtable::vtable(self).base__.ConvertFontToLOGFONT)(::windows::core::Vtable::as_raw(self), font.into().abi(), logfont, issystemfont).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn ConvertFontFaceToLOGFONT<P0>(&self, font: P0, logfont: *mut super::Gdi::LOGFONTW) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteFontFace>>,
    {
        (::windows::core::Vtable::vtable(self).base__.ConvertFontFaceToLOGFONT)(::windows::core::Vtable::as_raw(self), font.into().abi(), logfont).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn CreateFontFaceFromHdc<P0>(&self, hdc: P0) -> ::windows::core::Result<IDWriteFontFace>
    where
        P0: ::std::convert::Into<super::Gdi::HDC>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateFontFaceFromHdc)(::windows::core::Vtable::as_raw(self), hdc.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn CreateBitmapRenderTarget<P0>(&self, hdc: P0, width: u32, height: u32) -> ::windows::core::Result<IDWriteBitmapRenderTarget>
    where
        P0: ::std::convert::Into<super::Gdi::HDC>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateBitmapRenderTarget)(::windows::core::Vtable::as_raw(self), hdc.into(), width, height, result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IDWriteGlyphRunAnalysis {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDWriteGlyphRunAnalysis {}
impl ::core::fmt::Debug for IDWriteGlyphRunAnalysis {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDWriteGlyphRunAnalysis").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDWriteInMemoryFontFileLoader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDWriteInMemoryFontFileLoader {}
impl ::core::fmt::Debug for IDWriteInMemoryFontFileLoader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDWriteInMemoryFontFileLoader").field(&self.0).finish()
    }
}
impl IDWriteInMemoryFontFileLoader {
    pub unsafe fn CreateStreamFromKey(&self, fontfilereferencekey: *const ::core::ffi::c_void, fontfilereferencekeysize: u32) -> ::windows::core::Result<IDWriteFontFileStream> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateStreamFromKey)(::windows::core::Vtable::as_raw(self), fontfilereferencekey, fontfilereferencekeysize, result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IDWriteInlineObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDWriteInlineObject {}
impl ::core::fmt::Debug for IDWriteInlineObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDWriteInlineObject").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDWriteLocalFontFileLoader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDWriteLocalFontFileLoader {}
impl ::core::fmt::Debug for IDWriteLocalFontFileLoader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDWriteLocalFontFileLoader").field(&self.0).finish()
    }
}
impl IDWriteLocalFontFileLoader {
    pub unsafe fn CreateStreamFromKey(&self, fontfilereferencekey: *const ::core::ffi::c_void, fontfilereferencekeysize: u32) -> ::windows::core::Result<IDWriteFontFileStream> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateStreamFromKey)(::windows::core::Vtable::as_raw(self), fontfilereferencekey, fontfilereferencekeysize, result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IDWriteLocalizedStrings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDWriteLocalizedStrings {}
impl ::core::fmt::Debug for IDWriteLocalizedStrings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDWriteLocalizedStrings").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDWriteNumberSubstitution {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDWriteNumberSubstitution {}
impl ::core::fmt::Debug for IDWriteNumberSubstitution {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDWriteNumberSubstitution").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDWritePixelSnapping {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDWritePixelSnapping {}
impl ::core::fmt::Debug for IDWritePixelSnapping {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDWritePixelSnapping").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDWriteRemoteFontFileLoader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDWriteRemoteFontFileLoader {}
impl ::core::fmt::Debug for IDWriteRemoteFontFileLoader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDWriteRemoteFontFileLoader").field(&self.0).finish()
    }
}
impl IDWriteRemoteFontFileLoader {
    pub unsafe fn CreateStreamFromKey(&self, fontfilereferencekey: *const ::core::ffi::c_void, fontfilereferencekeysize: u32) -> ::windows::core::Result<IDWriteFontFileStream> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateStreamFromKey)(::windows::core::Vtable::as_raw(self), fontfilereferencekey, fontfilereferencekeysize, result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IDWriteRemoteFontFileStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDWriteRemoteFontFileStream {}
impl ::core::fmt::Debug for IDWriteRemoteFontFileStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDWriteRemoteFontFileStream").field(&self.0).finish()
    }
}
impl IDWriteRemoteFontFileStream {
    pub unsafe fn ReadFileFragment(&self, fragmentstart: *mut *mut ::core::ffi::c_void, fileoffset: u64, fragmentsize: u64, fragmentcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ReadFileFragment)(::windows::core::Vtable::as_raw(self), fragmentstart, fileoffset, fragmentsize, fragmentcontext).ok()
    }
    pub unsafe fn ReleaseFileFragment(&self, fragmentcontext: *mut ::core::ffi::c_void) {
        (::windows::core::Vtable::vtable(self).base__.ReleaseFileFragment)(::windows::core::Vtable::as_raw(self), fragmentcontext)
    }
    pub unsafe fn GetFileSize(&self) -> ::windows::core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFileSize)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetLastWriteTime(&self) -> ::windows::core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetLastWriteTime)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IDWriteRenderingParams {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDWriteRenderingParams {}
impl ::core::fmt::Debug for IDWriteRenderingParams {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDWriteRenderingParams").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDWriteRenderingParams1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDWriteRenderingParams1 {}
impl ::core::fmt::Debug for IDWriteRenderingParams1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDWriteRenderingParams1").field(&self.0).finish()
    }
}
impl IDWriteRenderingParams1 {
    pub unsafe fn GetGamma(&self) -> f32 {
        (::windows::core::Vtable::vtable(self).base__.GetGamma)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetEnhancedContrast(&self) -> f32 {
        (::windows::core::Vtable::vtable(self).base__.GetEnhancedContrast)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetClearTypeLevel(&self) -> f32 {
        (::windows::core::Vtable::vtable(self).base__.GetClearTypeLevel)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetPixelGeometry(&self) -> DWRITE_PIXEL_GEOMETRY {
        (::windows::core::Vtable::vtable(self).base__.GetPixelGeometry)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetRenderingMode(&self) -> DWRITE_RENDERING_MODE {
        (::windows::core::Vtable::vtable(self).base__.GetRenderingMode)(::windows::core::Vtable::as_raw(self))
    }
}
impl ::core::cmp::PartialEq for IDWriteRenderingParams2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDWriteRenderingParams2 {}
impl ::core::fmt::Debug for IDWriteRenderingParams2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDWriteRenderingParams2").field(&self.0).finish()
    }
}
impl IDWriteRenderingParams2 {
    pub unsafe fn GetGamma(&self) -> f32 {
        (::windows::core::Vtable::vtable(self).base__.base__.GetGamma)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetEnhancedContrast(&self) -> f32 {
        (::windows::core::Vtable::vtable(self).base__.base__.GetEnhancedContrast)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetClearTypeLevel(&self) -> f32 {
        (::windows::core::Vtable::vtable(self).base__.base__.GetClearTypeLevel)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetPixelGeometry(&self) -> DWRITE_PIXEL_GEOMETRY {
        (::windows::core::Vtable::vtable(self).base__.base__.GetPixelGeometry)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetRenderingMode(&self) -> DWRITE_RENDERING_MODE {
        (::windows::core::Vtable::vtable(self).base__.base__.GetRenderingMode)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetGrayscaleEnhancedContrast(&self) -> f32 {
        (::windows::core::Vtable::vtable(self).base__.GetGrayscaleEnhancedContrast)(::windows::core::Vtable::as_raw(self))
    }
}
impl ::core::cmp::PartialEq for IDWriteRenderingParams3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDWriteRenderingParams3 {}
impl ::core::fmt::Debug for IDWriteRenderingParams3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDWriteRenderingParams3").field(&self.0).finish()
    }
}
impl IDWriteRenderingParams3 {
    pub unsafe fn GetGamma(&self) -> f32 {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetGamma)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetEnhancedContrast(&self) -> f32 {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetEnhancedContrast)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetClearTypeLevel(&self) -> f32 {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetClearTypeLevel)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetPixelGeometry(&self) -> DWRITE_PIXEL_GEOMETRY {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetPixelGeometry)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetRenderingMode(&self) -> DWRITE_RENDERING_MODE {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetRenderingMode)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetGrayscaleEnhancedContrast(&self) -> f32 {
        (::windows::core::Vtable::vtable(self).base__.base__.GetGrayscaleEnhancedContrast)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetGridFitMode(&self) -> DWRITE_GRID_FIT_MODE {
        (::windows::core::Vtable::vtable(self).base__.GetGridFitMode)(::windows::core::Vtable::as_raw(self))
    }
}
impl ::core::cmp::PartialEq for IDWriteStringList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDWriteStringList {}
impl ::core::fmt::Debug for IDWriteStringList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDWriteStringList").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDWriteTextAnalysisSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDWriteTextAnalysisSink {}
impl ::core::fmt::Debug for IDWriteTextAnalysisSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDWriteTextAnalysisSink").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDWriteTextAnalysisSink1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDWriteTextAnalysisSink1 {}
impl ::core::fmt::Debug for IDWriteTextAnalysisSink1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDWriteTextAnalysisSink1").field(&self.0).finish()
    }
}
impl IDWriteTextAnalysisSink1 {
    pub unsafe fn SetScriptAnalysis(&self, textposition: u32, textlength: u32, scriptanalysis: *const DWRITE_SCRIPT_ANALYSIS) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetScriptAnalysis)(::windows::core::Vtable::as_raw(self), textposition, textlength, scriptanalysis).ok()
    }
    pub unsafe fn SetLineBreakpoints(&self, textposition: u32, linebreakpoints: &[DWRITE_LINE_BREAKPOINT]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetLineBreakpoints)(::windows::core::Vtable::as_raw(self), textposition, linebreakpoints.len() as _, ::core::mem::transmute(linebreakpoints.as_ptr())).ok()
    }
    pub unsafe fn SetBidiLevel(&self, textposition: u32, textlength: u32, explicitlevel: u8, resolvedlevel: u8) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetBidiLevel)(::windows::core::Vtable::as_raw(self), textposition, textlength, explicitlevel, resolvedlevel).ok()
    }
    pub unsafe fn SetNumberSubstitution<P0>(&self, textposition: u32, textlength: u32, numbersubstitution: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteNumberSubstitution>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetNumberSubstitution)(::windows::core::Vtable::as_raw(self), textposition, textlength, numbersubstitution.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for IDWriteTextAnalysisSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDWriteTextAnalysisSource {}
impl ::core::fmt::Debug for IDWriteTextAnalysisSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDWriteTextAnalysisSource").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDWriteTextAnalysisSource1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDWriteTextAnalysisSource1 {}
impl ::core::fmt::Debug for IDWriteTextAnalysisSource1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDWriteTextAnalysisSource1").field(&self.0).finish()
    }
}
impl IDWriteTextAnalysisSource1 {
    pub unsafe fn GetTextAtPosition(&self, textposition: u32, textstring: *mut *mut u16, textlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetTextAtPosition)(::windows::core::Vtable::as_raw(self), textposition, textstring, textlength).ok()
    }
    pub unsafe fn GetTextBeforePosition(&self, textposition: u32, textstring: *mut *mut u16, textlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetTextBeforePosition)(::windows::core::Vtable::as_raw(self), textposition, textstring, textlength).ok()
    }
    pub unsafe fn GetParagraphReadingDirection(&self) -> DWRITE_READING_DIRECTION {
        (::windows::core::Vtable::vtable(self).base__.GetParagraphReadingDirection)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetLocaleName(&self, textposition: u32, textlength: *mut u32, localename: *mut *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetLocaleName)(::windows::core::Vtable::as_raw(self), textposition, textlength, localename).ok()
    }
    pub unsafe fn GetNumberSubstitution(&self, textposition: u32, textlength: *mut u32, numbersubstitution: *mut ::core::option::Option<IDWriteNumberSubstitution>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetNumberSubstitution)(::windows::core::Vtable::as_raw(self), textposition, textlength, ::core::mem::transmute(numbersubstitution)).ok()
    }
}
impl ::core::cmp::PartialEq for IDWriteTextAnalyzer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDWriteTextAnalyzer {}
impl ::core::fmt::Debug for IDWriteTextAnalyzer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDWriteTextAnalyzer").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDWriteTextAnalyzer1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDWriteTextAnalyzer1 {}
impl ::core::fmt::Debug for IDWriteTextAnalyzer1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDWriteTextAnalyzer1").field(&self.0).finish()
    }
}
impl IDWriteTextAnalyzer1 {
    pub unsafe fn AnalyzeScript<P0, P1>(&self, analysissource: P0, textposition: u32, textlength: u32, analysissink: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteTextAnalysisSource>>,
        P1: ::std::convert::Into<::windows::core::InParam<IDWriteTextAnalysisSink>>,
    {
        (::windows::core::Vtable::vtable(self).base__.AnalyzeScript)(::windows::core::Vtable::as_raw(self), analysissource.into().abi(), textposition, textlength, analysissink.into().abi()).ok()
    }
    pub unsafe fn AnalyzeBidi<P0, P1>(&self, analysissource: P0, textposition: u32, textlength: u32, analysissink: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteTextAnalysisSource>>,
        P1: ::std::convert::Into<::windows::core::InParam<IDWriteTextAnalysisSink>>,
    {
        (::windows::core::Vtable::vtable(self).base__.AnalyzeBidi)(::windows::core::Vtable::as_raw(self), analysissource.into().abi(), textposition, textlength, analysissink.into().abi()).ok()
    }
    pub unsafe fn AnalyzeNumberSubstitution<P0, P1>(&self, analysissource: P0, textposition: u32, textlength: u32, analysissink: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteTextAnalysisSource>>,
        P1: ::std::convert::Into<::windows::core::InParam<IDWriteTextAnalysisSink>>,
    {
        (::windows::core::Vtable::vtable(self).base__.AnalyzeNumberSubstitution)(::windows::core::Vtable::as_raw(self), analysissource.into().abi(), textposition, textlength, analysissink.into().abi()).ok()
    }
    pub unsafe fn AnalyzeLineBreakpoints<P0, P1>(&self, analysissource: P0, textposition: u32, textlength: u32, analysissink: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteTextAnalysisSource>>,
        P1: ::std::convert::Into<::windows::core::InParam<IDWriteTextAnalysisSink>>,
    {
        (::windows::core::Vtable::vtable(self).base__.AnalyzeLineBreakpoints)(::windows::core::Vtable::as_raw(self), analysissource.into().abi(), textposition, textlength, analysissink.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetGlyphs<P0, P1, P2, P3, P4, P5>(&self, textstring: P0, textlength: u32, fontface: P1, issideways: P2, isrighttoleft: P3, scriptanalysis: *const DWRITE_SCRIPT_ANALYSIS, localename: P4, numbersubstitution: P5, features: ::core::option::Option<*const *const DWRITE_TYPOGRAPHIC_FEATURES>, featurerangelengths: ::core::option::Option<*const u32>, featureranges: u32, maxglyphcount: u32, clustermap: *mut u16, textprops: *mut DWRITE_SHAPING_TEXT_PROPERTIES, glyphindices: *mut u16, glyphprops: *mut DWRITE_SHAPING_GLYPH_PROPERTIES, actualglyphcount: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<IDWriteFontFace>>,
        P2: ::std::convert::Into<super::super::Foundation::BOOL>,
        P3: ::std::convert::Into<super::super::Foundation::BOOL>,
        P4: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P5: ::std::convert::Into<::windows::core::InParam<IDWriteNumberSubstitution>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetGlyphs)(::windows::core::Vtable::as_raw(self), textstring.into().abi(), textlength, fontface.into().abi(), issideways.into(), isrighttoleft.into(), scriptanalysis, localename.into().abi(), numbersubstitution.into().abi(), ::core::mem::transmute(features.unwrap_or(::std::ptr::null())), ::core::mem::transmute(featurerangelengths.unwrap_or(::std::ptr::null())), featureranges, maxglyphcount, clustermap, textprops, glyphindices, glyphprops, actualglyphcount).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetGlyphPlacements<P0, P1, P2, P3, P4>(&self, textstring: P0, clustermap: *const u16, textprops: *mut DWRITE_SHAPING_TEXT_PROPERTIES, textlength: u32, glyphindices: *const u16, glyphprops: *const DWRITE_SHAPING_GLYPH_PROPERTIES, glyphcount: u32, fontface: P1, fontemsize: f32, issideways: P2, isrighttoleft: P3, scriptanalysis: *const DWRITE_SCRIPT_ANALYSIS, localename: P4, features: ::core::option::Option<*const *const DWRITE_TYPOGRAPHIC_FEATURES>, featurerangelengths: ::core::option::Option<*const u32>, featureranges: u32, glyphadvances: *mut f32, glyphoffsets: *mut DWRITE_GLYPH_OFFSET) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<IDWriteFontFace>>,
        P2: ::std::convert::Into<super::super::Foundation::BOOL>,
        P3: ::std::convert::Into<super::super::Foundation::BOOL>,
        P4: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetGlyphPlacements)(::windows::core::Vtable::as_raw(self), textstring.into().abi(), clustermap, textprops, textlength, glyphindices, glyphprops, glyphcount, fontface.into().abi(), fontemsize, issideways.into(), isrighttoleft.into(), scriptanalysis, localename.into().abi(), ::core::mem::transmute(features.unwrap_or(::std::ptr::null())), ::core::mem::transmute(featurerangelengths.unwrap_or(::std::ptr::null())), featureranges, glyphadvances, glyphoffsets).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetGdiCompatibleGlyphPlacements<P0, P1, P2, P3, P4, P5>(&self, textstring: P0, clustermap: *const u16, textprops: *const DWRITE_SHAPING_TEXT_PROPERTIES, textlength: u32, glyphindices: *const u16, glyphprops: *const DWRITE_SHAPING_GLYPH_PROPERTIES, glyphcount: u32, fontface: P1, fontemsize: f32, pixelsperdip: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, usegdinatural: P2, issideways: P3, isrighttoleft: P4, scriptanalysis: *const DWRITE_SCRIPT_ANALYSIS, localename: P5, features: ::core::option::Option<*const *const DWRITE_TYPOGRAPHIC_FEATURES>, featurerangelengths: ::core::option::Option<*const u32>, featureranges: u32, glyphadvances: *mut f32, glyphoffsets: *mut DWRITE_GLYPH_OFFSET) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<IDWriteFontFace>>,
        P2: ::std::convert::Into<super::super::Foundation::BOOL>,
        P3: ::std::convert::Into<super::super::Foundation::BOOL>,
        P4: ::std::convert::Into<super::super::Foundation::BOOL>,
        P5: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetGdiCompatibleGlyphPlacements)(
            ::windows::core::Vtable::as_raw(self),
            textstring.into().abi(),
            clustermap,
            textprops,
            textlength,
            glyphindices,
            glyphprops,
            glyphcount,
            fontface.into().abi(),
            fontemsize,
            pixelsperdip,
            ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())),
            usegdinatural.into(),
            issideways.into(),
            isrighttoleft.into(),
            scriptanalysis,
            localename.into().abi(),
            ::core::mem::transmute(features.unwrap_or(::std::ptr::null())),
            ::core::mem::transmute(featurerangelengths.unwrap_or(::std::ptr::null())),
            featureranges,
            glyphadvances,
            glyphoffsets,
        )
        .ok()
    }
}
impl ::core::cmp::PartialEq for IDWriteTextAnalyzer2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDWriteTextAnalyzer2 {}
impl ::core::fmt::Debug for IDWriteTextAnalyzer2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDWriteTextAnalyzer2").field(&self.0).finish()
    }
}
impl IDWriteTextAnalyzer2 {
    pub unsafe fn AnalyzeScript<P0, P1>(&self, analysissource: P0, textposition: u32, textlength: u32, analysissink: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteTextAnalysisSource>>,
        P1: ::std::convert::Into<::windows::core::InParam<IDWriteTextAnalysisSink>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.AnalyzeScript)(::windows::core::Vtable::as_raw(self), analysissource.into().abi(), textposition, textlength, analysissink.into().abi()).ok()
    }
    pub unsafe fn AnalyzeBidi<P0, P1>(&self, analysissource: P0, textposition: u32, textlength: u32, analysissink: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteTextAnalysisSource>>,
        P1: ::std::convert::Into<::windows::core::InParam<IDWriteTextAnalysisSink>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.AnalyzeBidi)(::windows::core::Vtable::as_raw(self), analysissource.into().abi(), textposition, textlength, analysissink.into().abi()).ok()
    }
    pub unsafe fn AnalyzeNumberSubstitution<P0, P1>(&self, analysissource: P0, textposition: u32, textlength: u32, analysissink: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteTextAnalysisSource>>,
        P1: ::std::convert::Into<::windows::core::InParam<IDWriteTextAnalysisSink>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.AnalyzeNumberSubstitution)(::windows::core::Vtable::as_raw(self), analysissource.into().abi(), textposition, textlength, analysissink.into().abi()).ok()
    }
    pub unsafe fn AnalyzeLineBreakpoints<P0, P1>(&self, analysissource: P0, textposition: u32, textlength: u32, analysissink: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteTextAnalysisSource>>,
        P1: ::std::convert::Into<::windows::core::InParam<IDWriteTextAnalysisSink>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.AnalyzeLineBreakpoints)(::windows::core::Vtable::as_raw(self), analysissource.into().abi(), textposition, textlength, analysissink.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetGlyphs<P0, P1, P2, P3, P4, P5>(&self, textstring: P0, textlength: u32, fontface: P1, issideways: P2, isrighttoleft: P3, scriptanalysis: *const DWRITE_SCRIPT_ANALYSIS, localename: P4, numbersubstitution: P5, features: ::core::option::Option<*const *const DWRITE_TYPOGRAPHIC_FEATURES>, featurerangelengths: ::core::option::Option<*const u32>, featureranges: u32, maxglyphcount: u32, clustermap: *mut u16, textprops: *mut DWRITE_SHAPING_TEXT_PROPERTIES, glyphindices: *mut u16, glyphprops: *mut DWRITE_SHAPING_GLYPH_PROPERTIES, actualglyphcount: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<IDWriteFontFace>>,
        P2: ::std::convert::Into<super::super::Foundation::BOOL>,
        P3: ::std::convert::Into<super::super::Foundation::BOOL>,
        P4: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P5: ::std::convert::Into<::windows::core::InParam<IDWriteNumberSubstitution>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.GetGlyphs)(::windows::core::Vtable::as_raw(self), textstring.into().abi(), textlength, fontface.into().abi(), issideways.into(), isrighttoleft.into(), scriptanalysis, localename.into().abi(), numbersubstitution.into().abi(), ::core::mem::transmute(features.unwrap_or(::std::ptr::null())), ::core::mem::transmute(featurerangelengths.unwrap_or(::std::ptr::null())), featureranges, maxglyphcount, clustermap, textprops, glyphindices, glyphprops, actualglyphcount).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetGlyphPlacements<P0, P1, P2, P3, P4>(&self, textstring: P0, clustermap: *const u16, textprops: *mut DWRITE_SHAPING_TEXT_PROPERTIES, textlength: u32, glyphindices: *const u16, glyphprops: *const DWRITE_SHAPING_GLYPH_PROPERTIES, glyphcount: u32, fontface: P1, fontemsize: f32, issideways: P2, isrighttoleft: P3, scriptanalysis: *const DWRITE_SCRIPT_ANALYSIS, localename: P4, features: ::core::option::Option<*const *const DWRITE_TYPOGRAPHIC_FEATURES>, featurerangelengths: ::core::option::Option<*const u32>, featureranges: u32, glyphadvances: *mut f32, glyphoffsets: *mut DWRITE_GLYPH_OFFSET) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<IDWriteFontFace>>,
        P2: ::std::convert::Into<super::super::Foundation::BOOL>,
        P3: ::std::convert::Into<super::super::Foundation::BOOL>,
        P4: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.GetGlyphPlacements)(::windows::core::Vtable::as_raw(self), textstring.into().abi(), clustermap, textprops, textlength, glyphindices, glyphprops, glyphcount, fontface.into().abi(), fontemsize, issideways.into(), isrighttoleft.into(), scriptanalysis, localename.into().abi(), ::core::mem::transmute(features.unwrap_or(::std::ptr::null())), ::core::mem::transmute(featurerangelengths.unwrap_or(::std::ptr::null())), featureranges, glyphadvances, glyphoffsets).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetGdiCompatibleGlyphPlacements<P0, P1, P2, P3, P4, P5>(&self, textstring: P0, clustermap: *const u16, textprops: *const DWRITE_SHAPING_TEXT_PROPERTIES, textlength: u32, glyphindices: *const u16, glyphprops: *const DWRITE_SHAPING_GLYPH_PROPERTIES, glyphcount: u32, fontface: P1, fontemsize: f32, pixelsperdip: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, usegdinatural: P2, issideways: P3, isrighttoleft: P4, scriptanalysis: *const DWRITE_SCRIPT_ANALYSIS, localename: P5, features: ::core::option::Option<*const *const DWRITE_TYPOGRAPHIC_FEATURES>, featurerangelengths: ::core::option::Option<*const u32>, featureranges: u32, glyphadvances: *mut f32, glyphoffsets: *mut DWRITE_GLYPH_OFFSET) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<IDWriteFontFace>>,
        P2: ::std::convert::Into<super::super::Foundation::BOOL>,
        P3: ::std::convert::Into<super::super::Foundation::BOOL>,
        P4: ::std::convert::Into<super::super::Foundation::BOOL>,
        P5: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.GetGdiCompatibleGlyphPlacements)(
            ::windows::core::Vtable::as_raw(self),
            textstring.into().abi(),
            clustermap,
            textprops,
            textlength,
            glyphindices,
            glyphprops,
            glyphcount,
            fontface.into().abi(),
            fontemsize,
            pixelsperdip,
            ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())),
            usegdinatural.into(),
            issideways.into(),
            isrighttoleft.into(),
            scriptanalysis,
            localename.into().abi(),
            ::core::mem::transmute(features.unwrap_or(::std::ptr::null())),
            ::core::mem::transmute(featurerangelengths.unwrap_or(::std::ptr::null())),
            featureranges,
            glyphadvances,
            glyphoffsets,
        )
        .ok()
    }
    pub unsafe fn ApplyCharacterSpacing(&self, leadingspacing: f32, trailingspacing: f32, minimumadvancewidth: f32, glyphcount: u32, clustermap: &[u16], glyphadvances: *const f32, glyphoffsets: *const DWRITE_GLYPH_OFFSET, glyphproperties: *const DWRITE_SHAPING_GLYPH_PROPERTIES, modifiedglyphadvances: *mut f32, modifiedglyphoffsets: *mut DWRITE_GLYPH_OFFSET) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ApplyCharacterSpacing)(::windows::core::Vtable::as_raw(self), leadingspacing, trailingspacing, minimumadvancewidth, clustermap.len() as _, glyphcount, ::core::mem::transmute(clustermap.as_ptr()), glyphadvances, glyphoffsets, glyphproperties, modifiedglyphadvances, modifiedglyphoffsets).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBaseline<P0, P1, P2, P3>(&self, fontface: P0, baseline: DWRITE_BASELINE, isvertical: P1, issimulationallowed: P2, scriptanalysis: DWRITE_SCRIPT_ANALYSIS, localename: P3, baselinecoordinate: *mut i32, exists: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteFontFace>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
        P2: ::std::convert::Into<super::super::Foundation::BOOL>,
        P3: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetBaseline)(::windows::core::Vtable::as_raw(self), fontface.into().abi(), baseline, isvertical.into(), issimulationallowed.into(), ::core::mem::transmute(scriptanalysis), localename.into().abi(), baselinecoordinate, exists).ok()
    }
    pub unsafe fn AnalyzeVerticalGlyphOrientation<P0, P1>(&self, analysissource: P0, textposition: u32, textlength: u32, analysissink: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteTextAnalysisSource1>>,
        P1: ::std::convert::Into<::windows::core::InParam<IDWriteTextAnalysisSink1>>,
    {
        (::windows::core::Vtable::vtable(self).base__.AnalyzeVerticalGlyphOrientation)(::windows::core::Vtable::as_raw(self), analysissource.into().abi(), textposition, textlength, analysissink.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetGlyphOrientationTransform<P0>(&self, glyphorientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE, issideways: P0, transform: *mut DWRITE_MATRIX) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetGlyphOrientationTransform)(::windows::core::Vtable::as_raw(self), glyphorientationangle, issideways.into(), transform).ok()
    }
    pub unsafe fn GetScriptProperties(&self, scriptanalysis: DWRITE_SCRIPT_ANALYSIS, scriptproperties: *mut DWRITE_SCRIPT_PROPERTIES) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetScriptProperties)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(scriptanalysis), scriptproperties).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTextComplexity<P0, P1>(&self, textstring: P0, textlength: u32, fontface: P1, istextsimple: *mut super::super::Foundation::BOOL, textlengthread: *mut u32, glyphindices: ::core::option::Option<*mut u16>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<IDWriteFontFace>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetTextComplexity)(::windows::core::Vtable::as_raw(self), textstring.into().abi(), textlength, fontface.into().abi(), istextsimple, textlengthread, ::core::mem::transmute(glyphindices.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetJustificationOpportunities<P0, P1>(&self, fontface: P0, fontemsize: f32, scriptanalysis: DWRITE_SCRIPT_ANALYSIS, textlength: u32, glyphcount: u32, textstring: P1, clustermap: *const u16, glyphproperties: *const DWRITE_SHAPING_GLYPH_PROPERTIES, justificationopportunities: *mut DWRITE_JUSTIFICATION_OPPORTUNITY) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteFontFace>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetJustificationOpportunities)(::windows::core::Vtable::as_raw(self), fontface.into().abi(), fontemsize, ::core::mem::transmute(scriptanalysis), textlength, glyphcount, textstring.into().abi(), clustermap, glyphproperties, justificationopportunities).ok()
    }
    pub unsafe fn JustifyGlyphAdvances(&self, linewidth: f32, glyphcount: u32, justificationopportunities: *const DWRITE_JUSTIFICATION_OPPORTUNITY, glyphadvances: *const f32, glyphoffsets: *const DWRITE_GLYPH_OFFSET, justifiedglyphadvances: *mut f32, justifiedglyphoffsets: ::core::option::Option<*mut DWRITE_GLYPH_OFFSET>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.JustifyGlyphAdvances)(::windows::core::Vtable::as_raw(self), linewidth, glyphcount, justificationopportunities, glyphadvances, glyphoffsets, justifiedglyphadvances, ::core::mem::transmute(justifiedglyphoffsets.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetJustifiedGlyphs<P0>(&self, fontface: P0, fontemsize: f32, scriptanalysis: DWRITE_SCRIPT_ANALYSIS, textlength: u32, glyphcount: u32, maxglyphcount: u32, clustermap: ::core::option::Option<*const u16>, glyphindices: *const u16, glyphadvances: *const f32, justifiedglyphadvances: *const f32, justifiedglyphoffsets: *const DWRITE_GLYPH_OFFSET, glyphproperties: *const DWRITE_SHAPING_GLYPH_PROPERTIES, actualglyphcount: *mut u32, modifiedclustermap: ::core::option::Option<*mut u16>, modifiedglyphindices: *mut u16, modifiedglyphadvances: *mut f32, modifiedglyphoffsets: *mut DWRITE_GLYPH_OFFSET) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteFontFace>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetJustifiedGlyphs)(::windows::core::Vtable::as_raw(self), fontface.into().abi(), fontemsize, ::core::mem::transmute(scriptanalysis), textlength, glyphcount, maxglyphcount, ::core::mem::transmute(clustermap.unwrap_or(::std::ptr::null())), glyphindices, glyphadvances, justifiedglyphadvances, justifiedglyphoffsets, glyphproperties, actualglyphcount, ::core::mem::transmute(modifiedclustermap.unwrap_or(::std::ptr::null_mut())), modifiedglyphindices, modifiedglyphadvances, modifiedglyphoffsets).ok()
    }
}
impl ::core::cmp::PartialEq for IDWriteTextFormat {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDWriteTextFormat {}
impl ::core::fmt::Debug for IDWriteTextFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDWriteTextFormat").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDWriteTextFormat1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDWriteTextFormat1 {}
impl ::core::fmt::Debug for IDWriteTextFormat1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDWriteTextFormat1").field(&self.0).finish()
    }
}
impl IDWriteTextFormat1 {
    pub unsafe fn SetTextAlignment(&self, textalignment: DWRITE_TEXT_ALIGNMENT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetTextAlignment)(::windows::core::Vtable::as_raw(self), textalignment).ok()
    }
    pub unsafe fn SetParagraphAlignment(&self, paragraphalignment: DWRITE_PARAGRAPH_ALIGNMENT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetParagraphAlignment)(::windows::core::Vtable::as_raw(self), paragraphalignment).ok()
    }
    pub unsafe fn SetWordWrapping(&self, wordwrapping: DWRITE_WORD_WRAPPING) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetWordWrapping)(::windows::core::Vtable::as_raw(self), wordwrapping).ok()
    }
    pub unsafe fn SetReadingDirection(&self, readingdirection: DWRITE_READING_DIRECTION) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetReadingDirection)(::windows::core::Vtable::as_raw(self), readingdirection).ok()
    }
    pub unsafe fn SetFlowDirection(&self, flowdirection: DWRITE_FLOW_DIRECTION) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetFlowDirection)(::windows::core::Vtable::as_raw(self), flowdirection).ok()
    }
    pub unsafe fn SetIncrementalTabStop(&self, incrementaltabstop: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetIncrementalTabStop)(::windows::core::Vtable::as_raw(self), incrementaltabstop).ok()
    }
    pub unsafe fn SetTrimming<P0>(&self, trimmingoptions: *const DWRITE_TRIMMING, trimmingsign: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteInlineObject>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetTrimming)(::windows::core::Vtable::as_raw(self), trimmingoptions, trimmingsign.into().abi()).ok()
    }
    pub unsafe fn SetLineSpacing(&self, linespacingmethod: DWRITE_LINE_SPACING_METHOD, linespacing: f32, baseline: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetLineSpacing)(::windows::core::Vtable::as_raw(self), linespacingmethod, linespacing, baseline).ok()
    }
    pub unsafe fn GetTextAlignment(&self) -> DWRITE_TEXT_ALIGNMENT {
        (::windows::core::Vtable::vtable(self).base__.GetTextAlignment)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetParagraphAlignment(&self) -> DWRITE_PARAGRAPH_ALIGNMENT {
        (::windows::core::Vtable::vtable(self).base__.GetParagraphAlignment)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetWordWrapping(&self) -> DWRITE_WORD_WRAPPING {
        (::windows::core::Vtable::vtable(self).base__.GetWordWrapping)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetReadingDirection(&self) -> DWRITE_READING_DIRECTION {
        (::windows::core::Vtable::vtable(self).base__.GetReadingDirection)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetFlowDirection(&self) -> DWRITE_FLOW_DIRECTION {
        (::windows::core::Vtable::vtable(self).base__.GetFlowDirection)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetIncrementalTabStop(&self) -> f32 {
        (::windows::core::Vtable::vtable(self).base__.GetIncrementalTabStop)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetTrimming(&self, trimmingoptions: *mut DWRITE_TRIMMING, trimmingsign: *mut ::core::option::Option<IDWriteInlineObject>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetTrimming)(::windows::core::Vtable::as_raw(self), trimmingoptions, ::core::mem::transmute(trimmingsign)).ok()
    }
    pub unsafe fn GetLineSpacing(&self, linespacingmethod: *mut DWRITE_LINE_SPACING_METHOD, linespacing: *mut f32, baseline: *mut f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetLineSpacing)(::windows::core::Vtable::as_raw(self), linespacingmethod, linespacing, baseline).ok()
    }
    pub unsafe fn GetFontCollection(&self) -> ::windows::core::Result<IDWriteFontCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFontCollection)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetFontFamilyNameLength(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.GetFontFamilyNameLength)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetFontFamilyName(&self, fontfamilyname: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetFontFamilyName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(fontfamilyname.as_ptr()), fontfamilyname.len() as _).ok()
    }
    pub unsafe fn GetFontWeight(&self) -> DWRITE_FONT_WEIGHT {
        (::windows::core::Vtable::vtable(self).base__.GetFontWeight)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetFontStyle(&self) -> DWRITE_FONT_STYLE {
        (::windows::core::Vtable::vtable(self).base__.GetFontStyle)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetFontStretch(&self) -> DWRITE_FONT_STRETCH {
        (::windows::core::Vtable::vtable(self).base__.GetFontStretch)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetFontSize(&self) -> f32 {
        (::windows::core::Vtable::vtable(self).base__.GetFontSize)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetLocaleNameLength(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.GetLocaleNameLength)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetLocaleName(&self, localename: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetLocaleName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(localename.as_ptr()), localename.len() as _).ok()
    }
}
impl ::core::cmp::PartialEq for IDWriteTextFormat2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDWriteTextFormat2 {}
impl ::core::fmt::Debug for IDWriteTextFormat2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDWriteTextFormat2").field(&self.0).finish()
    }
}
impl IDWriteTextFormat2 {
    pub unsafe fn SetTextAlignment(&self, textalignment: DWRITE_TEXT_ALIGNMENT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetTextAlignment)(::windows::core::Vtable::as_raw(self), textalignment).ok()
    }
    pub unsafe fn SetParagraphAlignment(&self, paragraphalignment: DWRITE_PARAGRAPH_ALIGNMENT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetParagraphAlignment)(::windows::core::Vtable::as_raw(self), paragraphalignment).ok()
    }
    pub unsafe fn SetWordWrapping(&self, wordwrapping: DWRITE_WORD_WRAPPING) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetWordWrapping)(::windows::core::Vtable::as_raw(self), wordwrapping).ok()
    }
    pub unsafe fn SetReadingDirection(&self, readingdirection: DWRITE_READING_DIRECTION) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetReadingDirection)(::windows::core::Vtable::as_raw(self), readingdirection).ok()
    }
    pub unsafe fn SetFlowDirection(&self, flowdirection: DWRITE_FLOW_DIRECTION) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetFlowDirection)(::windows::core::Vtable::as_raw(self), flowdirection).ok()
    }
    pub unsafe fn SetIncrementalTabStop(&self, incrementaltabstop: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetIncrementalTabStop)(::windows::core::Vtable::as_raw(self), incrementaltabstop).ok()
    }
    pub unsafe fn SetTrimming<P0>(&self, trimmingoptions: *const DWRITE_TRIMMING, trimmingsign: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteInlineObject>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetTrimming)(::windows::core::Vtable::as_raw(self), trimmingoptions, trimmingsign.into().abi()).ok()
    }
    pub unsafe fn SetLineSpacing(&self, linespacingmethod: DWRITE_LINE_SPACING_METHOD, linespacing: f32, baseline: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetLineSpacing)(::windows::core::Vtable::as_raw(self), linespacingmethod, linespacing, baseline).ok()
    }
    pub unsafe fn GetTextAlignment(&self) -> DWRITE_TEXT_ALIGNMENT {
        (::windows::core::Vtable::vtable(self).base__.base__.GetTextAlignment)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetParagraphAlignment(&self) -> DWRITE_PARAGRAPH_ALIGNMENT {
        (::windows::core::Vtable::vtable(self).base__.base__.GetParagraphAlignment)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetWordWrapping(&self) -> DWRITE_WORD_WRAPPING {
        (::windows::core::Vtable::vtable(self).base__.base__.GetWordWrapping)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetReadingDirection(&self) -> DWRITE_READING_DIRECTION {
        (::windows::core::Vtable::vtable(self).base__.base__.GetReadingDirection)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetFlowDirection(&self) -> DWRITE_FLOW_DIRECTION {
        (::windows::core::Vtable::vtable(self).base__.base__.GetFlowDirection)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetIncrementalTabStop(&self) -> f32 {
        (::windows::core::Vtable::vtable(self).base__.base__.GetIncrementalTabStop)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetTrimming(&self, trimmingoptions: *mut DWRITE_TRIMMING, trimmingsign: *mut ::core::option::Option<IDWriteInlineObject>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetTrimming)(::windows::core::Vtable::as_raw(self), trimmingoptions, ::core::mem::transmute(trimmingsign)).ok()
    }
    pub unsafe fn GetLineSpacing(&self, linespacingmethod: *mut DWRITE_LINE_SPACING_METHOD, linespacing: *mut f32, baseline: *mut f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetLineSpacing)(::windows::core::Vtable::as_raw(self), linespacingmethod, linespacing, baseline).ok()
    }
    pub unsafe fn GetFontCollection(&self) -> ::windows::core::Result<IDWriteFontCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetFontCollection)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetFontFamilyNameLength(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.base__.GetFontFamilyNameLength)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetFontFamilyName(&self, fontfamilyname: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetFontFamilyName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(fontfamilyname.as_ptr()), fontfamilyname.len() as _).ok()
    }
    pub unsafe fn GetFontWeight(&self) -> DWRITE_FONT_WEIGHT {
        (::windows::core::Vtable::vtable(self).base__.base__.GetFontWeight)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetFontStyle(&self) -> DWRITE_FONT_STYLE {
        (::windows::core::Vtable::vtable(self).base__.base__.GetFontStyle)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetFontStretch(&self) -> DWRITE_FONT_STRETCH {
        (::windows::core::Vtable::vtable(self).base__.base__.GetFontStretch)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetFontSize(&self) -> f32 {
        (::windows::core::Vtable::vtable(self).base__.base__.GetFontSize)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetLocaleNameLength(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.base__.GetLocaleNameLength)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetLocaleName(&self, localename: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetLocaleName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(localename.as_ptr()), localename.len() as _).ok()
    }
    pub unsafe fn SetVerticalGlyphOrientation(&self, glyphorientation: DWRITE_VERTICAL_GLYPH_ORIENTATION) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetVerticalGlyphOrientation)(::windows::core::Vtable::as_raw(self), glyphorientation).ok()
    }
    pub unsafe fn GetVerticalGlyphOrientation(&self) -> DWRITE_VERTICAL_GLYPH_ORIENTATION {
        (::windows::core::Vtable::vtable(self).base__.GetVerticalGlyphOrientation)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLastLineWrapping<P0>(&self, islastlinewrappingenabled: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetLastLineWrapping)(::windows::core::Vtable::as_raw(self), islastlinewrappingenabled.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLastLineWrapping(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.GetLastLineWrapping)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn SetOpticalAlignment(&self, opticalalignment: DWRITE_OPTICAL_ALIGNMENT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetOpticalAlignment)(::windows::core::Vtable::as_raw(self), opticalalignment).ok()
    }
    pub unsafe fn GetOpticalAlignment(&self) -> DWRITE_OPTICAL_ALIGNMENT {
        (::windows::core::Vtable::vtable(self).base__.GetOpticalAlignment)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn SetFontFallback<P0>(&self, fontfallback: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteFontFallback>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetFontFallback)(::windows::core::Vtable::as_raw(self), fontfallback.into().abi()).ok()
    }
    pub unsafe fn GetFontFallback(&self) -> ::windows::core::Result<IDWriteFontFallback> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFontFallback)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IDWriteTextFormat3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDWriteTextFormat3 {}
impl ::core::fmt::Debug for IDWriteTextFormat3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDWriteTextFormat3").field(&self.0).finish()
    }
}
impl IDWriteTextFormat3 {
    pub unsafe fn SetTextAlignment(&self, textalignment: DWRITE_TEXT_ALIGNMENT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetTextAlignment)(::windows::core::Vtable::as_raw(self), textalignment).ok()
    }
    pub unsafe fn SetParagraphAlignment(&self, paragraphalignment: DWRITE_PARAGRAPH_ALIGNMENT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetParagraphAlignment)(::windows::core::Vtable::as_raw(self), paragraphalignment).ok()
    }
    pub unsafe fn SetWordWrapping(&self, wordwrapping: DWRITE_WORD_WRAPPING) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetWordWrapping)(::windows::core::Vtable::as_raw(self), wordwrapping).ok()
    }
    pub unsafe fn SetReadingDirection(&self, readingdirection: DWRITE_READING_DIRECTION) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetReadingDirection)(::windows::core::Vtable::as_raw(self), readingdirection).ok()
    }
    pub unsafe fn SetFlowDirection(&self, flowdirection: DWRITE_FLOW_DIRECTION) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetFlowDirection)(::windows::core::Vtable::as_raw(self), flowdirection).ok()
    }
    pub unsafe fn SetIncrementalTabStop(&self, incrementaltabstop: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetIncrementalTabStop)(::windows::core::Vtable::as_raw(self), incrementaltabstop).ok()
    }
    pub unsafe fn SetTrimming<P0>(&self, trimmingoptions: *const DWRITE_TRIMMING, trimmingsign: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteInlineObject>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetTrimming)(::windows::core::Vtable::as_raw(self), trimmingoptions, trimmingsign.into().abi()).ok()
    }
    pub unsafe fn SetLineSpacing(&self, linespacingmethod: DWRITE_LINE_SPACING_METHOD, linespacing: f32, baseline: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetLineSpacing)(::windows::core::Vtable::as_raw(self), linespacingmethod, linespacing, baseline).ok()
    }
    pub unsafe fn GetTextAlignment(&self) -> DWRITE_TEXT_ALIGNMENT {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetTextAlignment)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetParagraphAlignment(&self) -> DWRITE_PARAGRAPH_ALIGNMENT {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetParagraphAlignment)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetWordWrapping(&self) -> DWRITE_WORD_WRAPPING {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetWordWrapping)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetReadingDirection(&self) -> DWRITE_READING_DIRECTION {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetReadingDirection)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetFlowDirection(&self) -> DWRITE_FLOW_DIRECTION {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetFlowDirection)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetIncrementalTabStop(&self) -> f32 {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetIncrementalTabStop)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetTrimming(&self, trimmingoptions: *mut DWRITE_TRIMMING, trimmingsign: *mut ::core::option::Option<IDWriteInlineObject>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetTrimming)(::windows::core::Vtable::as_raw(self), trimmingoptions, ::core::mem::transmute(trimmingsign)).ok()
    }
    pub unsafe fn GetLineSpacing(&self, linespacingmethod: *mut DWRITE_LINE_SPACING_METHOD, linespacing: *mut f32, baseline: *mut f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetLineSpacing)(::windows::core::Vtable::as_raw(self), linespacingmethod, linespacing, baseline).ok()
    }
    pub unsafe fn GetFontCollection(&self) -> ::windows::core::Result<IDWriteFontCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetFontCollection)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetFontFamilyNameLength(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetFontFamilyNameLength)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetFontFamilyName(&self, fontfamilyname: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetFontFamilyName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(fontfamilyname.as_ptr()), fontfamilyname.len() as _).ok()
    }
    pub unsafe fn GetFontWeight(&self) -> DWRITE_FONT_WEIGHT {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetFontWeight)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetFontStyle(&self) -> DWRITE_FONT_STYLE {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetFontStyle)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetFontStretch(&self) -> DWRITE_FONT_STRETCH {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetFontStretch)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetFontSize(&self) -> f32 {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetFontSize)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetLocaleNameLength(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetLocaleNameLength)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetLocaleName(&self, localename: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetLocaleName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(localename.as_ptr()), localename.len() as _).ok()
    }
    pub unsafe fn SetVerticalGlyphOrientation(&self, glyphorientation: DWRITE_VERTICAL_GLYPH_ORIENTATION) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetVerticalGlyphOrientation)(::windows::core::Vtable::as_raw(self), glyphorientation).ok()
    }
    pub unsafe fn GetVerticalGlyphOrientation(&self) -> DWRITE_VERTICAL_GLYPH_ORIENTATION {
        (::windows::core::Vtable::vtable(self).base__.base__.GetVerticalGlyphOrientation)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLastLineWrapping<P0>(&self, islastlinewrappingenabled: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetLastLineWrapping)(::windows::core::Vtable::as_raw(self), islastlinewrappingenabled.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLastLineWrapping(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.base__.GetLastLineWrapping)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn SetOpticalAlignment(&self, opticalalignment: DWRITE_OPTICAL_ALIGNMENT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetOpticalAlignment)(::windows::core::Vtable::as_raw(self), opticalalignment).ok()
    }
    pub unsafe fn GetOpticalAlignment(&self) -> DWRITE_OPTICAL_ALIGNMENT {
        (::windows::core::Vtable::vtable(self).base__.base__.GetOpticalAlignment)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn SetFontFallback<P0>(&self, fontfallback: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteFontFallback>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetFontFallback)(::windows::core::Vtable::as_raw(self), fontfallback.into().abi()).ok()
    }
    pub unsafe fn GetFontFallback(&self) -> ::windows::core::Result<IDWriteFontFallback> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetFontFallback)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetLineSpacing2(&self, linespacingoptions: *const DWRITE_LINE_SPACING) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetLineSpacing2)(::windows::core::Vtable::as_raw(self), linespacingoptions).ok()
    }
    pub unsafe fn GetLineSpacing2(&self, linespacingoptions: *mut DWRITE_LINE_SPACING) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetLineSpacing2)(::windows::core::Vtable::as_raw(self), linespacingoptions).ok()
    }
}
impl ::core::cmp::PartialEq for IDWriteTextLayout {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDWriteTextLayout {}
impl ::core::fmt::Debug for IDWriteTextLayout {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDWriteTextLayout").field(&self.0).finish()
    }
}
impl IDWriteTextLayout {
    pub unsafe fn SetTextAlignment(&self, textalignment: DWRITE_TEXT_ALIGNMENT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetTextAlignment)(::windows::core::Vtable::as_raw(self), textalignment).ok()
    }
    pub unsafe fn SetParagraphAlignment(&self, paragraphalignment: DWRITE_PARAGRAPH_ALIGNMENT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetParagraphAlignment)(::windows::core::Vtable::as_raw(self), paragraphalignment).ok()
    }
    pub unsafe fn SetWordWrapping(&self, wordwrapping: DWRITE_WORD_WRAPPING) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetWordWrapping)(::windows::core::Vtable::as_raw(self), wordwrapping).ok()
    }
    pub unsafe fn SetReadingDirection(&self, readingdirection: DWRITE_READING_DIRECTION) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetReadingDirection)(::windows::core::Vtable::as_raw(self), readingdirection).ok()
    }
    pub unsafe fn SetFlowDirection(&self, flowdirection: DWRITE_FLOW_DIRECTION) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetFlowDirection)(::windows::core::Vtable::as_raw(self), flowdirection).ok()
    }
    pub unsafe fn SetIncrementalTabStop(&self, incrementaltabstop: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetIncrementalTabStop)(::windows::core::Vtable::as_raw(self), incrementaltabstop).ok()
    }
    pub unsafe fn SetTrimming<P0>(&self, trimmingoptions: *const DWRITE_TRIMMING, trimmingsign: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteInlineObject>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetTrimming)(::windows::core::Vtable::as_raw(self), trimmingoptions, trimmingsign.into().abi()).ok()
    }
    pub unsafe fn SetLineSpacing(&self, linespacingmethod: DWRITE_LINE_SPACING_METHOD, linespacing: f32, baseline: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetLineSpacing)(::windows::core::Vtable::as_raw(self), linespacingmethod, linespacing, baseline).ok()
    }
    pub unsafe fn GetTextAlignment(&self) -> DWRITE_TEXT_ALIGNMENT {
        (::windows::core::Vtable::vtable(self).base__.GetTextAlignment)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetParagraphAlignment(&self) -> DWRITE_PARAGRAPH_ALIGNMENT {
        (::windows::core::Vtable::vtable(self).base__.GetParagraphAlignment)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetWordWrapping(&self) -> DWRITE_WORD_WRAPPING {
        (::windows::core::Vtable::vtable(self).base__.GetWordWrapping)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetReadingDirection(&self) -> DWRITE_READING_DIRECTION {
        (::windows::core::Vtable::vtable(self).base__.GetReadingDirection)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetFlowDirection(&self) -> DWRITE_FLOW_DIRECTION {
        (::windows::core::Vtable::vtable(self).base__.GetFlowDirection)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetIncrementalTabStop(&self) -> f32 {
        (::windows::core::Vtable::vtable(self).base__.GetIncrementalTabStop)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetTrimming(&self, trimmingoptions: *mut DWRITE_TRIMMING, trimmingsign: *mut ::core::option::Option<IDWriteInlineObject>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetTrimming)(::windows::core::Vtable::as_raw(self), trimmingoptions, ::core::mem::transmute(trimmingsign)).ok()
    }
    pub unsafe fn GetLineSpacing(&self, linespacingmethod: *mut DWRITE_LINE_SPACING_METHOD, linespacing: *mut f32, baseline: *mut f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetLineSpacing)(::windows::core::Vtable::as_raw(self), linespacingmethod, linespacing, baseline).ok()
    }
    pub unsafe fn GetFontCollection(&self) -> ::windows::core::Result<IDWriteFontCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFontCollection)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetFontFamilyNameLength(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.GetFontFamilyNameLength)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetFontFamilyName(&self, fontfamilyname: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetFontFamilyName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(fontfamilyname.as_ptr()), fontfamilyname.len() as _).ok()
    }
    pub unsafe fn GetFontWeight(&self) -> DWRITE_FONT_WEIGHT {
        (::windows::core::Vtable::vtable(self).base__.GetFontWeight)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetFontStyle(&self) -> DWRITE_FONT_STYLE {
        (::windows::core::Vtable::vtable(self).base__.GetFontStyle)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetFontStretch(&self) -> DWRITE_FONT_STRETCH {
        (::windows::core::Vtable::vtable(self).base__.GetFontStretch)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetFontSize(&self) -> f32 {
        (::windows::core::Vtable::vtable(self).base__.GetFontSize)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetLocaleNameLength(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.GetLocaleNameLength)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetLocaleName(&self, localename: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetLocaleName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(localename.as_ptr()), localename.len() as _).ok()
    }
}
impl ::core::cmp::PartialEq for IDWriteTextLayout1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDWriteTextLayout1 {}
impl ::core::fmt::Debug for IDWriteTextLayout1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDWriteTextLayout1").field(&self.0).finish()
    }
}
impl IDWriteTextLayout1 {
    pub unsafe fn SetTextAlignment(&self, textalignment: DWRITE_TEXT_ALIGNMENT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetTextAlignment)(::windows::core::Vtable::as_raw(self), textalignment).ok()
    }
    pub unsafe fn SetParagraphAlignment(&self, paragraphalignment: DWRITE_PARAGRAPH_ALIGNMENT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetParagraphAlignment)(::windows::core::Vtable::as_raw(self), paragraphalignment).ok()
    }
    pub unsafe fn SetWordWrapping(&self, wordwrapping: DWRITE_WORD_WRAPPING) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetWordWrapping)(::windows::core::Vtable::as_raw(self), wordwrapping).ok()
    }
    pub unsafe fn SetReadingDirection(&self, readingdirection: DWRITE_READING_DIRECTION) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetReadingDirection)(::windows::core::Vtable::as_raw(self), readingdirection).ok()
    }
    pub unsafe fn SetFlowDirection(&self, flowdirection: DWRITE_FLOW_DIRECTION) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetFlowDirection)(::windows::core::Vtable::as_raw(self), flowdirection).ok()
    }
    pub unsafe fn SetIncrementalTabStop(&self, incrementaltabstop: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetIncrementalTabStop)(::windows::core::Vtable::as_raw(self), incrementaltabstop).ok()
    }
    pub unsafe fn SetTrimming<P0>(&self, trimmingoptions: *const DWRITE_TRIMMING, trimmingsign: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteInlineObject>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetTrimming)(::windows::core::Vtable::as_raw(self), trimmingoptions, trimmingsign.into().abi()).ok()
    }
    pub unsafe fn SetLineSpacing(&self, linespacingmethod: DWRITE_LINE_SPACING_METHOD, linespacing: f32, baseline: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetLineSpacing)(::windows::core::Vtable::as_raw(self), linespacingmethod, linespacing, baseline).ok()
    }
    pub unsafe fn GetTextAlignment(&self) -> DWRITE_TEXT_ALIGNMENT {
        (::windows::core::Vtable::vtable(self).base__.base__.GetTextAlignment)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetParagraphAlignment(&self) -> DWRITE_PARAGRAPH_ALIGNMENT {
        (::windows::core::Vtable::vtable(self).base__.base__.GetParagraphAlignment)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetWordWrapping(&self) -> DWRITE_WORD_WRAPPING {
        (::windows::core::Vtable::vtable(self).base__.base__.GetWordWrapping)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetReadingDirection(&self) -> DWRITE_READING_DIRECTION {
        (::windows::core::Vtable::vtable(self).base__.base__.GetReadingDirection)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetFlowDirection(&self) -> DWRITE_FLOW_DIRECTION {
        (::windows::core::Vtable::vtable(self).base__.base__.GetFlowDirection)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetIncrementalTabStop(&self) -> f32 {
        (::windows::core::Vtable::vtable(self).base__.base__.GetIncrementalTabStop)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetTrimming(&self, trimmingoptions: *mut DWRITE_TRIMMING, trimmingsign: *mut ::core::option::Option<IDWriteInlineObject>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetTrimming)(::windows::core::Vtable::as_raw(self), trimmingoptions, ::core::mem::transmute(trimmingsign)).ok()
    }
    pub unsafe fn GetLineSpacing(&self, linespacingmethod: *mut DWRITE_LINE_SPACING_METHOD, linespacing: *mut f32, baseline: *mut f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetLineSpacing)(::windows::core::Vtable::as_raw(self), linespacingmethod, linespacing, baseline).ok()
    }
    pub unsafe fn GetFontCollection(&self) -> ::windows::core::Result<IDWriteFontCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetFontCollection)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetFontFamilyNameLength(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.base__.GetFontFamilyNameLength)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetFontFamilyName(&self, fontfamilyname: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetFontFamilyName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(fontfamilyname.as_ptr()), fontfamilyname.len() as _).ok()
    }
    pub unsafe fn GetFontWeight(&self) -> DWRITE_FONT_WEIGHT {
        (::windows::core::Vtable::vtable(self).base__.base__.GetFontWeight)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetFontStyle(&self) -> DWRITE_FONT_STYLE {
        (::windows::core::Vtable::vtable(self).base__.base__.GetFontStyle)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetFontStretch(&self) -> DWRITE_FONT_STRETCH {
        (::windows::core::Vtable::vtable(self).base__.base__.GetFontStretch)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetFontSize(&self) -> f32 {
        (::windows::core::Vtable::vtable(self).base__.base__.GetFontSize)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetLocaleNameLength(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.base__.GetLocaleNameLength)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetLocaleName(&self, localename: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetLocaleName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(localename.as_ptr()), localename.len() as _).ok()
    }
    pub unsafe fn SetMaxWidth(&self, maxwidth: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetMaxWidth)(::windows::core::Vtable::as_raw(self), maxwidth).ok()
    }
    pub unsafe fn SetMaxHeight(&self, maxheight: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetMaxHeight)(::windows::core::Vtable::as_raw(self), maxheight).ok()
    }
    pub unsafe fn SetFontCollection<P0>(&self, fontcollection: P0, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteFontCollection>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetFontCollection)(::windows::core::Vtable::as_raw(self), fontcollection.into().abi(), ::core::mem::transmute(textrange)).ok()
    }
    pub unsafe fn SetFontFamilyName<P0>(&self, fontfamilyname: P0, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetFontFamilyName)(::windows::core::Vtable::as_raw(self), fontfamilyname.into().abi(), ::core::mem::transmute(textrange)).ok()
    }
    pub unsafe fn SetFontWeight(&self, fontweight: DWRITE_FONT_WEIGHT, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetFontWeight)(::windows::core::Vtable::as_raw(self), fontweight, ::core::mem::transmute(textrange)).ok()
    }
    pub unsafe fn SetFontStyle(&self, fontstyle: DWRITE_FONT_STYLE, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetFontStyle)(::windows::core::Vtable::as_raw(self), fontstyle, ::core::mem::transmute(textrange)).ok()
    }
    pub unsafe fn SetFontStretch(&self, fontstretch: DWRITE_FONT_STRETCH, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetFontStretch)(::windows::core::Vtable::as_raw(self), fontstretch, ::core::mem::transmute(textrange)).ok()
    }
    pub unsafe fn SetFontSize(&self, fontsize: f32, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetFontSize)(::windows::core::Vtable::as_raw(self), fontsize, ::core::mem::transmute(textrange)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetUnderline<P0>(&self, hasunderline: P0, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetUnderline)(::windows::core::Vtable::as_raw(self), hasunderline.into(), ::core::mem::transmute(textrange)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetStrikethrough<P0>(&self, hasstrikethrough: P0, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetStrikethrough)(::windows::core::Vtable::as_raw(self), hasstrikethrough.into(), ::core::mem::transmute(textrange)).ok()
    }
    pub unsafe fn SetDrawingEffect<P0>(&self, drawingeffect: P0, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetDrawingEffect)(::windows::core::Vtable::as_raw(self), drawingeffect.into().abi(), ::core::mem::transmute(textrange)).ok()
    }
    pub unsafe fn SetInlineObject<P0>(&self, inlineobject: P0, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteInlineObject>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetInlineObject)(::windows::core::Vtable::as_raw(self), inlineobject.into().abi(), ::core::mem::transmute(textrange)).ok()
    }
    pub unsafe fn SetTypography<P0>(&self, typography: P0, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteTypography>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetTypography)(::windows::core::Vtable::as_raw(self), typography.into().abi(), ::core::mem::transmute(textrange)).ok()
    }
    pub unsafe fn SetLocaleName<P0>(&self, localename: P0, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetLocaleName)(::windows::core::Vtable::as_raw(self), localename.into().abi(), ::core::mem::transmute(textrange)).ok()
    }
    pub unsafe fn GetMaxWidth(&self) -> f32 {
        (::windows::core::Vtable::vtable(self).base__.GetMaxWidth)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetMaxHeight(&self) -> f32 {
        (::windows::core::Vtable::vtable(self).base__.GetMaxHeight)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetFontCollection2(&self, currentposition: u32, fontcollection: *mut ::core::option::Option<IDWriteFontCollection>, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetFontCollection2)(::windows::core::Vtable::as_raw(self), currentposition, ::core::mem::transmute(fontcollection), ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetFontFamilyNameLength2(&self, currentposition: u32, namelength: *mut u32, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetFontFamilyNameLength2)(::windows::core::Vtable::as_raw(self), currentposition, namelength, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetFontFamilyName2(&self, currentposition: u32, fontfamilyname: &mut [u16], textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetFontFamilyName2)(::windows::core::Vtable::as_raw(self), currentposition, ::core::mem::transmute(fontfamilyname.as_ptr()), fontfamilyname.len() as _, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetFontWeight2(&self, currentposition: u32, fontweight: *mut DWRITE_FONT_WEIGHT, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetFontWeight2)(::windows::core::Vtable::as_raw(self), currentposition, fontweight, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetFontStyle2(&self, currentposition: u32, fontstyle: *mut DWRITE_FONT_STYLE, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetFontStyle2)(::windows::core::Vtable::as_raw(self), currentposition, fontstyle, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetFontStretch2(&self, currentposition: u32, fontstretch: *mut DWRITE_FONT_STRETCH, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetFontStretch2)(::windows::core::Vtable::as_raw(self), currentposition, fontstretch, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetFontSize2(&self, currentposition: u32, fontsize: *mut f32, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetFontSize2)(::windows::core::Vtable::as_raw(self), currentposition, fontsize, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetUnderline(&self, currentposition: u32, hasunderline: *mut super::super::Foundation::BOOL, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetUnderline)(::windows::core::Vtable::as_raw(self), currentposition, hasunderline, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetStrikethrough(&self, currentposition: u32, hasstrikethrough: *mut super::super::Foundation::BOOL, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetStrikethrough)(::windows::core::Vtable::as_raw(self), currentposition, hasstrikethrough, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetDrawingEffect(&self, currentposition: u32, drawingeffect: *mut ::core::option::Option<::windows::core::IUnknown>, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetDrawingEffect)(::windows::core::Vtable::as_raw(self), currentposition, ::core::mem::transmute(drawingeffect), ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetInlineObject(&self, currentposition: u32, inlineobject: *mut ::core::option::Option<IDWriteInlineObject>, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetInlineObject)(::windows::core::Vtable::as_raw(self), currentposition, ::core::mem::transmute(inlineobject), ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetTypography(&self, currentposition: u32, typography: *mut ::core::option::Option<IDWriteTypography>, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetTypography)(::windows::core::Vtable::as_raw(self), currentposition, ::core::mem::transmute(typography), ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetLocaleNameLength2(&self, currentposition: u32, namelength: *mut u32, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetLocaleNameLength2)(::windows::core::Vtable::as_raw(self), currentposition, namelength, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetLocaleName2(&self, currentposition: u32, localename: &mut [u16], textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetLocaleName2)(::windows::core::Vtable::as_raw(self), currentposition, ::core::mem::transmute(localename.as_ptr()), localename.len() as _, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Draw<P0>(&self, clientdrawingcontext: ::core::option::Option<*const ::core::ffi::c_void>, renderer: P0, originx: f32, originy: f32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteTextRenderer>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Draw)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(clientdrawingcontext.unwrap_or(::std::ptr::null())), renderer.into().abi(), originx, originy).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLineMetrics(&self, linemetrics: ::core::option::Option<&mut [DWRITE_LINE_METRICS]>, actuallinecount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetLineMetrics)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(linemetrics.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), linemetrics.as_deref().map_or(0, |slice| slice.len() as _), actuallinecount).ok()
    }
    pub unsafe fn GetMetrics(&self, textmetrics: *mut DWRITE_TEXT_METRICS) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetMetrics)(::windows::core::Vtable::as_raw(self), textmetrics).ok()
    }
    pub unsafe fn GetOverhangMetrics(&self) -> ::windows::core::Result<DWRITE_OVERHANG_METRICS> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetOverhangMetrics)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetClusterMetrics(&self, clustermetrics: ::core::option::Option<&mut [DWRITE_CLUSTER_METRICS]>, actualclustercount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetClusterMetrics)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(clustermetrics.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), clustermetrics.as_deref().map_or(0, |slice| slice.len() as _), actualclustercount).ok()
    }
    pub unsafe fn DetermineMinWidth(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.DetermineMinWidth)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HitTestPoint(&self, pointx: f32, pointy: f32, istrailinghit: *mut super::super::Foundation::BOOL, isinside: *mut super::super::Foundation::BOOL, hittestmetrics: *mut DWRITE_HIT_TEST_METRICS) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.HitTestPoint)(::windows::core::Vtable::as_raw(self), pointx, pointy, istrailinghit, isinside, hittestmetrics).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HitTestTextPosition<P0>(&self, textposition: u32, istrailinghit: P0, pointx: *mut f32, pointy: *mut f32, hittestmetrics: *mut DWRITE_HIT_TEST_METRICS) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.HitTestTextPosition)(::windows::core::Vtable::as_raw(self), textposition, istrailinghit.into(), pointx, pointy, hittestmetrics).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HitTestTextRange(&self, textposition: u32, textlength: u32, originx: f32, originy: f32, hittestmetrics: ::core::option::Option<&mut [DWRITE_HIT_TEST_METRICS]>, actualhittestmetricscount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.HitTestTextRange)(::windows::core::Vtable::as_raw(self), textposition, textlength, originx, originy, ::core::mem::transmute(hittestmetrics.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), hittestmetrics.as_deref().map_or(0, |slice| slice.len() as _), actualhittestmetricscount).ok()
    }
}
impl ::core::cmp::PartialEq for IDWriteTextLayout2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDWriteTextLayout2 {}
impl ::core::fmt::Debug for IDWriteTextLayout2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDWriteTextLayout2").field(&self.0).finish()
    }
}
impl IDWriteTextLayout2 {
    pub unsafe fn SetTextAlignment(&self, textalignment: DWRITE_TEXT_ALIGNMENT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetTextAlignment)(::windows::core::Vtable::as_raw(self), textalignment).ok()
    }
    pub unsafe fn SetParagraphAlignment(&self, paragraphalignment: DWRITE_PARAGRAPH_ALIGNMENT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetParagraphAlignment)(::windows::core::Vtable::as_raw(self), paragraphalignment).ok()
    }
    pub unsafe fn SetWordWrapping(&self, wordwrapping: DWRITE_WORD_WRAPPING) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetWordWrapping)(::windows::core::Vtable::as_raw(self), wordwrapping).ok()
    }
    pub unsafe fn SetReadingDirection(&self, readingdirection: DWRITE_READING_DIRECTION) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetReadingDirection)(::windows::core::Vtable::as_raw(self), readingdirection).ok()
    }
    pub unsafe fn SetFlowDirection(&self, flowdirection: DWRITE_FLOW_DIRECTION) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetFlowDirection)(::windows::core::Vtable::as_raw(self), flowdirection).ok()
    }
    pub unsafe fn SetIncrementalTabStop(&self, incrementaltabstop: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetIncrementalTabStop)(::windows::core::Vtable::as_raw(self), incrementaltabstop).ok()
    }
    pub unsafe fn SetTrimming<P0>(&self, trimmingoptions: *const DWRITE_TRIMMING, trimmingsign: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteInlineObject>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetTrimming)(::windows::core::Vtable::as_raw(self), trimmingoptions, trimmingsign.into().abi()).ok()
    }
    pub unsafe fn SetLineSpacing(&self, linespacingmethod: DWRITE_LINE_SPACING_METHOD, linespacing: f32, baseline: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetLineSpacing)(::windows::core::Vtable::as_raw(self), linespacingmethod, linespacing, baseline).ok()
    }
    pub unsafe fn GetTextAlignment(&self) -> DWRITE_TEXT_ALIGNMENT {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetTextAlignment)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetParagraphAlignment(&self) -> DWRITE_PARAGRAPH_ALIGNMENT {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetParagraphAlignment)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetWordWrapping(&self) -> DWRITE_WORD_WRAPPING {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetWordWrapping)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetReadingDirection(&self) -> DWRITE_READING_DIRECTION {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetReadingDirection)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetFlowDirection(&self) -> DWRITE_FLOW_DIRECTION {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetFlowDirection)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetIncrementalTabStop(&self) -> f32 {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetIncrementalTabStop)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetTrimming(&self, trimmingoptions: *mut DWRITE_TRIMMING, trimmingsign: *mut ::core::option::Option<IDWriteInlineObject>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetTrimming)(::windows::core::Vtable::as_raw(self), trimmingoptions, ::core::mem::transmute(trimmingsign)).ok()
    }
    pub unsafe fn GetLineSpacing(&self, linespacingmethod: *mut DWRITE_LINE_SPACING_METHOD, linespacing: *mut f32, baseline: *mut f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetLineSpacing)(::windows::core::Vtable::as_raw(self), linespacingmethod, linespacing, baseline).ok()
    }
    pub unsafe fn GetFontCollection(&self) -> ::windows::core::Result<IDWriteFontCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetFontCollection)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetFontFamilyNameLength(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetFontFamilyNameLength)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetFontFamilyName(&self, fontfamilyname: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetFontFamilyName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(fontfamilyname.as_ptr()), fontfamilyname.len() as _).ok()
    }
    pub unsafe fn GetFontWeight(&self) -> DWRITE_FONT_WEIGHT {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetFontWeight)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetFontStyle(&self) -> DWRITE_FONT_STYLE {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetFontStyle)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetFontStretch(&self) -> DWRITE_FONT_STRETCH {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetFontStretch)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetFontSize(&self) -> f32 {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetFontSize)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetLocaleNameLength(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetLocaleNameLength)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetLocaleName(&self, localename: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetLocaleName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(localename.as_ptr()), localename.len() as _).ok()
    }
    pub unsafe fn SetMaxWidth(&self, maxwidth: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetMaxWidth)(::windows::core::Vtable::as_raw(self), maxwidth).ok()
    }
    pub unsafe fn SetMaxHeight(&self, maxheight: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetMaxHeight)(::windows::core::Vtable::as_raw(self), maxheight).ok()
    }
    pub unsafe fn SetFontCollection<P0>(&self, fontcollection: P0, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteFontCollection>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetFontCollection)(::windows::core::Vtable::as_raw(self), fontcollection.into().abi(), ::core::mem::transmute(textrange)).ok()
    }
    pub unsafe fn SetFontFamilyName<P0>(&self, fontfamilyname: P0, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetFontFamilyName)(::windows::core::Vtable::as_raw(self), fontfamilyname.into().abi(), ::core::mem::transmute(textrange)).ok()
    }
    pub unsafe fn SetFontWeight(&self, fontweight: DWRITE_FONT_WEIGHT, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetFontWeight)(::windows::core::Vtable::as_raw(self), fontweight, ::core::mem::transmute(textrange)).ok()
    }
    pub unsafe fn SetFontStyle(&self, fontstyle: DWRITE_FONT_STYLE, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetFontStyle)(::windows::core::Vtable::as_raw(self), fontstyle, ::core::mem::transmute(textrange)).ok()
    }
    pub unsafe fn SetFontStretch(&self, fontstretch: DWRITE_FONT_STRETCH, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetFontStretch)(::windows::core::Vtable::as_raw(self), fontstretch, ::core::mem::transmute(textrange)).ok()
    }
    pub unsafe fn SetFontSize(&self, fontsize: f32, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetFontSize)(::windows::core::Vtable::as_raw(self), fontsize, ::core::mem::transmute(textrange)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetUnderline<P0>(&self, hasunderline: P0, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetUnderline)(::windows::core::Vtable::as_raw(self), hasunderline.into(), ::core::mem::transmute(textrange)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetStrikethrough<P0>(&self, hasstrikethrough: P0, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetStrikethrough)(::windows::core::Vtable::as_raw(self), hasstrikethrough.into(), ::core::mem::transmute(textrange)).ok()
    }
    pub unsafe fn SetDrawingEffect<P0>(&self, drawingeffect: P0, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetDrawingEffect)(::windows::core::Vtable::as_raw(self), drawingeffect.into().abi(), ::core::mem::transmute(textrange)).ok()
    }
    pub unsafe fn SetInlineObject<P0>(&self, inlineobject: P0, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteInlineObject>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetInlineObject)(::windows::core::Vtable::as_raw(self), inlineobject.into().abi(), ::core::mem::transmute(textrange)).ok()
    }
    pub unsafe fn SetTypography<P0>(&self, typography: P0, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteTypography>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetTypography)(::windows::core::Vtable::as_raw(self), typography.into().abi(), ::core::mem::transmute(textrange)).ok()
    }
    pub unsafe fn SetLocaleName<P0>(&self, localename: P0, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetLocaleName)(::windows::core::Vtable::as_raw(self), localename.into().abi(), ::core::mem::transmute(textrange)).ok()
    }
    pub unsafe fn GetMaxWidth(&self) -> f32 {
        (::windows::core::Vtable::vtable(self).base__.base__.GetMaxWidth)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetMaxHeight(&self) -> f32 {
        (::windows::core::Vtable::vtable(self).base__.base__.GetMaxHeight)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetFontCollection2(&self, currentposition: u32, fontcollection: *mut ::core::option::Option<IDWriteFontCollection>, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetFontCollection2)(::windows::core::Vtable::as_raw(self), currentposition, ::core::mem::transmute(fontcollection), ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetFontFamilyNameLength2(&self, currentposition: u32, namelength: *mut u32, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetFontFamilyNameLength2)(::windows::core::Vtable::as_raw(self), currentposition, namelength, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetFontFamilyName2(&self, currentposition: u32, fontfamilyname: &mut [u16], textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetFontFamilyName2)(::windows::core::Vtable::as_raw(self), currentposition, ::core::mem::transmute(fontfamilyname.as_ptr()), fontfamilyname.len() as _, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetFontWeight2(&self, currentposition: u32, fontweight: *mut DWRITE_FONT_WEIGHT, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetFontWeight2)(::windows::core::Vtable::as_raw(self), currentposition, fontweight, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetFontStyle2(&self, currentposition: u32, fontstyle: *mut DWRITE_FONT_STYLE, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetFontStyle2)(::windows::core::Vtable::as_raw(self), currentposition, fontstyle, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetFontStretch2(&self, currentposition: u32, fontstretch: *mut DWRITE_FONT_STRETCH, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetFontStretch2)(::windows::core::Vtable::as_raw(self), currentposition, fontstretch, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetFontSize2(&self, currentposition: u32, fontsize: *mut f32, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetFontSize2)(::windows::core::Vtable::as_raw(self), currentposition, fontsize, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetUnderline(&self, currentposition: u32, hasunderline: *mut super::super::Foundation::BOOL, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetUnderline)(::windows::core::Vtable::as_raw(self), currentposition, hasunderline, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetStrikethrough(&self, currentposition: u32, hasstrikethrough: *mut super::super::Foundation::BOOL, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetStrikethrough)(::windows::core::Vtable::as_raw(self), currentposition, hasstrikethrough, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetDrawingEffect(&self, currentposition: u32, drawingeffect: *mut ::core::option::Option<::windows::core::IUnknown>, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetDrawingEffect)(::windows::core::Vtable::as_raw(self), currentposition, ::core::mem::transmute(drawingeffect), ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetInlineObject(&self, currentposition: u32, inlineobject: *mut ::core::option::Option<IDWriteInlineObject>, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetInlineObject)(::windows::core::Vtable::as_raw(self), currentposition, ::core::mem::transmute(inlineobject), ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetTypography(&self, currentposition: u32, typography: *mut ::core::option::Option<IDWriteTypography>, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetTypography)(::windows::core::Vtable::as_raw(self), currentposition, ::core::mem::transmute(typography), ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetLocaleNameLength2(&self, currentposition: u32, namelength: *mut u32, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetLocaleNameLength2)(::windows::core::Vtable::as_raw(self), currentposition, namelength, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetLocaleName2(&self, currentposition: u32, localename: &mut [u16], textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetLocaleName2)(::windows::core::Vtable::as_raw(self), currentposition, ::core::mem::transmute(localename.as_ptr()), localename.len() as _, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Draw<P0>(&self, clientdrawingcontext: ::core::option::Option<*const ::core::ffi::c_void>, renderer: P0, originx: f32, originy: f32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteTextRenderer>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.Draw)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(clientdrawingcontext.unwrap_or(::std::ptr::null())), renderer.into().abi(), originx, originy).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLineMetrics(&self, linemetrics: ::core::option::Option<&mut [DWRITE_LINE_METRICS]>, actuallinecount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetLineMetrics)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(linemetrics.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), linemetrics.as_deref().map_or(0, |slice| slice.len() as _), actuallinecount).ok()
    }
    pub unsafe fn GetMetrics(&self, textmetrics: *mut DWRITE_TEXT_METRICS) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetMetrics)(::windows::core::Vtable::as_raw(self), textmetrics).ok()
    }
    pub unsafe fn GetOverhangMetrics(&self) -> ::windows::core::Result<DWRITE_OVERHANG_METRICS> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetOverhangMetrics)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetClusterMetrics(&self, clustermetrics: ::core::option::Option<&mut [DWRITE_CLUSTER_METRICS]>, actualclustercount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetClusterMetrics)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(clustermetrics.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), clustermetrics.as_deref().map_or(0, |slice| slice.len() as _), actualclustercount).ok()
    }
    pub unsafe fn DetermineMinWidth(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.DetermineMinWidth)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HitTestPoint(&self, pointx: f32, pointy: f32, istrailinghit: *mut super::super::Foundation::BOOL, isinside: *mut super::super::Foundation::BOOL, hittestmetrics: *mut DWRITE_HIT_TEST_METRICS) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.HitTestPoint)(::windows::core::Vtable::as_raw(self), pointx, pointy, istrailinghit, isinside, hittestmetrics).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HitTestTextPosition<P0>(&self, textposition: u32, istrailinghit: P0, pointx: *mut f32, pointy: *mut f32, hittestmetrics: *mut DWRITE_HIT_TEST_METRICS) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.HitTestTextPosition)(::windows::core::Vtable::as_raw(self), textposition, istrailinghit.into(), pointx, pointy, hittestmetrics).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HitTestTextRange(&self, textposition: u32, textlength: u32, originx: f32, originy: f32, hittestmetrics: ::core::option::Option<&mut [DWRITE_HIT_TEST_METRICS]>, actualhittestmetricscount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.HitTestTextRange)(::windows::core::Vtable::as_raw(self), textposition, textlength, originx, originy, ::core::mem::transmute(hittestmetrics.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), hittestmetrics.as_deref().map_or(0, |slice| slice.len() as _), actualhittestmetricscount).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPairKerning<P0>(&self, ispairkerningenabled: P0, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetPairKerning)(::windows::core::Vtable::as_raw(self), ispairkerningenabled.into(), ::core::mem::transmute(textrange)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPairKerning(&self, currentposition: u32, ispairkerningenabled: *mut super::super::Foundation::BOOL, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetPairKerning)(::windows::core::Vtable::as_raw(self), currentposition, ispairkerningenabled, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetCharacterSpacing(&self, leadingspacing: f32, trailingspacing: f32, minimumadvancewidth: f32, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetCharacterSpacing)(::windows::core::Vtable::as_raw(self), leadingspacing, trailingspacing, minimumadvancewidth, ::core::mem::transmute(textrange)).ok()
    }
    pub unsafe fn GetCharacterSpacing(&self, currentposition: u32, leadingspacing: *mut f32, trailingspacing: *mut f32, minimumadvancewidth: *mut f32, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetCharacterSpacing)(::windows::core::Vtable::as_raw(self), currentposition, leadingspacing, trailingspacing, minimumadvancewidth, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut()))).ok()
    }
}
impl ::core::cmp::PartialEq for IDWriteTextLayout3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDWriteTextLayout3 {}
impl ::core::fmt::Debug for IDWriteTextLayout3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDWriteTextLayout3").field(&self.0).finish()
    }
}
impl IDWriteTextLayout3 {
    pub unsafe fn SetTextAlignment(&self, textalignment: DWRITE_TEXT_ALIGNMENT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetTextAlignment)(::windows::core::Vtable::as_raw(self), textalignment).ok()
    }
    pub unsafe fn SetParagraphAlignment(&self, paragraphalignment: DWRITE_PARAGRAPH_ALIGNMENT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetParagraphAlignment)(::windows::core::Vtable::as_raw(self), paragraphalignment).ok()
    }
    pub unsafe fn SetWordWrapping(&self, wordwrapping: DWRITE_WORD_WRAPPING) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetWordWrapping)(::windows::core::Vtable::as_raw(self), wordwrapping).ok()
    }
    pub unsafe fn SetReadingDirection(&self, readingdirection: DWRITE_READING_DIRECTION) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetReadingDirection)(::windows::core::Vtable::as_raw(self), readingdirection).ok()
    }
    pub unsafe fn SetFlowDirection(&self, flowdirection: DWRITE_FLOW_DIRECTION) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetFlowDirection)(::windows::core::Vtable::as_raw(self), flowdirection).ok()
    }
    pub unsafe fn SetIncrementalTabStop(&self, incrementaltabstop: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetIncrementalTabStop)(::windows::core::Vtable::as_raw(self), incrementaltabstop).ok()
    }
    pub unsafe fn SetTrimming<P0>(&self, trimmingoptions: *const DWRITE_TRIMMING, trimmingsign: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteInlineObject>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetTrimming)(::windows::core::Vtable::as_raw(self), trimmingoptions, trimmingsign.into().abi()).ok()
    }
    pub unsafe fn SetLineSpacing(&self, linespacingmethod: DWRITE_LINE_SPACING_METHOD, linespacing: f32, baseline: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetLineSpacing)(::windows::core::Vtable::as_raw(self), linespacingmethod, linespacing, baseline).ok()
    }
    pub unsafe fn GetTextAlignment(&self) -> DWRITE_TEXT_ALIGNMENT {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetTextAlignment)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetParagraphAlignment(&self) -> DWRITE_PARAGRAPH_ALIGNMENT {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetParagraphAlignment)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetWordWrapping(&self) -> DWRITE_WORD_WRAPPING {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetWordWrapping)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetReadingDirection(&self) -> DWRITE_READING_DIRECTION {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetReadingDirection)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetFlowDirection(&self) -> DWRITE_FLOW_DIRECTION {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetFlowDirection)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetIncrementalTabStop(&self) -> f32 {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetIncrementalTabStop)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetTrimming(&self, trimmingoptions: *mut DWRITE_TRIMMING, trimmingsign: *mut ::core::option::Option<IDWriteInlineObject>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetTrimming)(::windows::core::Vtable::as_raw(self), trimmingoptions, ::core::mem::transmute(trimmingsign)).ok()
    }
    pub unsafe fn GetLineSpacing(&self, linespacingmethod: *mut DWRITE_LINE_SPACING_METHOD, linespacing: *mut f32, baseline: *mut f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetLineSpacing)(::windows::core::Vtable::as_raw(self), linespacingmethod, linespacing, baseline).ok()
    }
    pub unsafe fn GetFontCollection(&self) -> ::windows::core::Result<IDWriteFontCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetFontCollection)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetFontFamilyNameLength(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetFontFamilyNameLength)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetFontFamilyName(&self, fontfamilyname: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetFontFamilyName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(fontfamilyname.as_ptr()), fontfamilyname.len() as _).ok()
    }
    pub unsafe fn GetFontWeight(&self) -> DWRITE_FONT_WEIGHT {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetFontWeight)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetFontStyle(&self) -> DWRITE_FONT_STYLE {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetFontStyle)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetFontStretch(&self) -> DWRITE_FONT_STRETCH {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetFontStretch)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetFontSize(&self) -> f32 {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetFontSize)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetLocaleNameLength(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetLocaleNameLength)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetLocaleName(&self, localename: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetLocaleName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(localename.as_ptr()), localename.len() as _).ok()
    }
    pub unsafe fn SetMaxWidth(&self, maxwidth: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetMaxWidth)(::windows::core::Vtable::as_raw(self), maxwidth).ok()
    }
    pub unsafe fn SetMaxHeight(&self, maxheight: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetMaxHeight)(::windows::core::Vtable::as_raw(self), maxheight).ok()
    }
    pub unsafe fn SetFontCollection<P0>(&self, fontcollection: P0, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteFontCollection>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetFontCollection)(::windows::core::Vtable::as_raw(self), fontcollection.into().abi(), ::core::mem::transmute(textrange)).ok()
    }
    pub unsafe fn SetFontFamilyName<P0>(&self, fontfamilyname: P0, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetFontFamilyName)(::windows::core::Vtable::as_raw(self), fontfamilyname.into().abi(), ::core::mem::transmute(textrange)).ok()
    }
    pub unsafe fn SetFontWeight(&self, fontweight: DWRITE_FONT_WEIGHT, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetFontWeight)(::windows::core::Vtable::as_raw(self), fontweight, ::core::mem::transmute(textrange)).ok()
    }
    pub unsafe fn SetFontStyle(&self, fontstyle: DWRITE_FONT_STYLE, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetFontStyle)(::windows::core::Vtable::as_raw(self), fontstyle, ::core::mem::transmute(textrange)).ok()
    }
    pub unsafe fn SetFontStretch(&self, fontstretch: DWRITE_FONT_STRETCH, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetFontStretch)(::windows::core::Vtable::as_raw(self), fontstretch, ::core::mem::transmute(textrange)).ok()
    }
    pub unsafe fn SetFontSize(&self, fontsize: f32, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetFontSize)(::windows::core::Vtable::as_raw(self), fontsize, ::core::mem::transmute(textrange)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetUnderline<P0>(&self, hasunderline: P0, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetUnderline)(::windows::core::Vtable::as_raw(self), hasunderline.into(), ::core::mem::transmute(textrange)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetStrikethrough<P0>(&self, hasstrikethrough: P0, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetStrikethrough)(::windows::core::Vtable::as_raw(self), hasstrikethrough.into(), ::core::mem::transmute(textrange)).ok()
    }
    pub unsafe fn SetDrawingEffect<P0>(&self, drawingeffect: P0, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetDrawingEffect)(::windows::core::Vtable::as_raw(self), drawingeffect.into().abi(), ::core::mem::transmute(textrange)).ok()
    }
    pub unsafe fn SetInlineObject<P0>(&self, inlineobject: P0, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteInlineObject>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetInlineObject)(::windows::core::Vtable::as_raw(self), inlineobject.into().abi(), ::core::mem::transmute(textrange)).ok()
    }
    pub unsafe fn SetTypography<P0>(&self, typography: P0, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteTypography>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetTypography)(::windows::core::Vtable::as_raw(self), typography.into().abi(), ::core::mem::transmute(textrange)).ok()
    }
    pub unsafe fn SetLocaleName<P0>(&self, localename: P0, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetLocaleName)(::windows::core::Vtable::as_raw(self), localename.into().abi(), ::core::mem::transmute(textrange)).ok()
    }
    pub unsafe fn GetMaxWidth(&self) -> f32 {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetMaxWidth)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetMaxHeight(&self) -> f32 {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetMaxHeight)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetFontCollection2(&self, currentposition: u32, fontcollection: *mut ::core::option::Option<IDWriteFontCollection>, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetFontCollection2)(::windows::core::Vtable::as_raw(self), currentposition, ::core::mem::transmute(fontcollection), ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetFontFamilyNameLength2(&self, currentposition: u32, namelength: *mut u32, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetFontFamilyNameLength2)(::windows::core::Vtable::as_raw(self), currentposition, namelength, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetFontFamilyName2(&self, currentposition: u32, fontfamilyname: &mut [u16], textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetFontFamilyName2)(::windows::core::Vtable::as_raw(self), currentposition, ::core::mem::transmute(fontfamilyname.as_ptr()), fontfamilyname.len() as _, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetFontWeight2(&self, currentposition: u32, fontweight: *mut DWRITE_FONT_WEIGHT, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetFontWeight2)(::windows::core::Vtable::as_raw(self), currentposition, fontweight, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetFontStyle2(&self, currentposition: u32, fontstyle: *mut DWRITE_FONT_STYLE, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetFontStyle2)(::windows::core::Vtable::as_raw(self), currentposition, fontstyle, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetFontStretch2(&self, currentposition: u32, fontstretch: *mut DWRITE_FONT_STRETCH, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetFontStretch2)(::windows::core::Vtable::as_raw(self), currentposition, fontstretch, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetFontSize2(&self, currentposition: u32, fontsize: *mut f32, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetFontSize2)(::windows::core::Vtable::as_raw(self), currentposition, fontsize, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetUnderline(&self, currentposition: u32, hasunderline: *mut super::super::Foundation::BOOL, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetUnderline)(::windows::core::Vtable::as_raw(self), currentposition, hasunderline, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetStrikethrough(&self, currentposition: u32, hasstrikethrough: *mut super::super::Foundation::BOOL, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetStrikethrough)(::windows::core::Vtable::as_raw(self), currentposition, hasstrikethrough, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetDrawingEffect(&self, currentposition: u32, drawingeffect: *mut ::core::option::Option<::windows::core::IUnknown>, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetDrawingEffect)(::windows::core::Vtable::as_raw(self), currentposition, ::core::mem::transmute(drawingeffect), ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetInlineObject(&self, currentposition: u32, inlineobject: *mut ::core::option::Option<IDWriteInlineObject>, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetInlineObject)(::windows::core::Vtable::as_raw(self), currentposition, ::core::mem::transmute(inlineobject), ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetTypography(&self, currentposition: u32, typography: *mut ::core::option::Option<IDWriteTypography>, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetTypography)(::windows::core::Vtable::as_raw(self), currentposition, ::core::mem::transmute(typography), ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetLocaleNameLength2(&self, currentposition: u32, namelength: *mut u32, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetLocaleNameLength2)(::windows::core::Vtable::as_raw(self), currentposition, namelength, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetLocaleName2(&self, currentposition: u32, localename: &mut [u16], textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetLocaleName2)(::windows::core::Vtable::as_raw(self), currentposition, ::core::mem::transmute(localename.as_ptr()), localename.len() as _, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Draw<P0>(&self, clientdrawingcontext: ::core::option::Option<*const ::core::ffi::c_void>, renderer: P0, originx: f32, originy: f32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteTextRenderer>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.Draw)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(clientdrawingcontext.unwrap_or(::std::ptr::null())), renderer.into().abi(), originx, originy).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLineMetrics(&self, linemetrics: ::core::option::Option<&mut [DWRITE_LINE_METRICS]>, actuallinecount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetLineMetrics)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(linemetrics.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), linemetrics.as_deref().map_or(0, |slice| slice.len() as _), actuallinecount).ok()
    }
    pub unsafe fn GetMetrics(&self, textmetrics: *mut DWRITE_TEXT_METRICS) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetMetrics)(::windows::core::Vtable::as_raw(self), textmetrics).ok()
    }
    pub unsafe fn GetOverhangMetrics(&self) -> ::windows::core::Result<DWRITE_OVERHANG_METRICS> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetOverhangMetrics)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetClusterMetrics(&self, clustermetrics: ::core::option::Option<&mut [DWRITE_CLUSTER_METRICS]>, actualclustercount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetClusterMetrics)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(clustermetrics.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), clustermetrics.as_deref().map_or(0, |slice| slice.len() as _), actualclustercount).ok()
    }
    pub unsafe fn DetermineMinWidth(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.DetermineMinWidth)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HitTestPoint(&self, pointx: f32, pointy: f32, istrailinghit: *mut super::super::Foundation::BOOL, isinside: *mut super::super::Foundation::BOOL, hittestmetrics: *mut DWRITE_HIT_TEST_METRICS) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.HitTestPoint)(::windows::core::Vtable::as_raw(self), pointx, pointy, istrailinghit, isinside, hittestmetrics).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HitTestTextPosition<P0>(&self, textposition: u32, istrailinghit: P0, pointx: *mut f32, pointy: *mut f32, hittestmetrics: *mut DWRITE_HIT_TEST_METRICS) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.HitTestTextPosition)(::windows::core::Vtable::as_raw(self), textposition, istrailinghit.into(), pointx, pointy, hittestmetrics).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HitTestTextRange(&self, textposition: u32, textlength: u32, originx: f32, originy: f32, hittestmetrics: ::core::option::Option<&mut [DWRITE_HIT_TEST_METRICS]>, actualhittestmetricscount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.HitTestTextRange)(::windows::core::Vtable::as_raw(self), textposition, textlength, originx, originy, ::core::mem::transmute(hittestmetrics.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), hittestmetrics.as_deref().map_or(0, |slice| slice.len() as _), actualhittestmetricscount).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPairKerning<P0>(&self, ispairkerningenabled: P0, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPairKerning)(::windows::core::Vtable::as_raw(self), ispairkerningenabled.into(), ::core::mem::transmute(textrange)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPairKerning(&self, currentposition: u32, ispairkerningenabled: *mut super::super::Foundation::BOOL, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetPairKerning)(::windows::core::Vtable::as_raw(self), currentposition, ispairkerningenabled, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetCharacterSpacing(&self, leadingspacing: f32, trailingspacing: f32, minimumadvancewidth: f32, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetCharacterSpacing)(::windows::core::Vtable::as_raw(self), leadingspacing, trailingspacing, minimumadvancewidth, ::core::mem::transmute(textrange)).ok()
    }
    pub unsafe fn GetCharacterSpacing(&self, currentposition: u32, leadingspacing: *mut f32, trailingspacing: *mut f32, minimumadvancewidth: *mut f32, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetCharacterSpacing)(::windows::core::Vtable::as_raw(self), currentposition, leadingspacing, trailingspacing, minimumadvancewidth, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetMetrics2(&self, textmetrics: *mut DWRITE_TEXT_METRICS1) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetMetrics2)(::windows::core::Vtable::as_raw(self), textmetrics).ok()
    }
    pub unsafe fn SetVerticalGlyphOrientation(&self, glyphorientation: DWRITE_VERTICAL_GLYPH_ORIENTATION) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetVerticalGlyphOrientation)(::windows::core::Vtable::as_raw(self), glyphorientation).ok()
    }
    pub unsafe fn GetVerticalGlyphOrientation(&self) -> DWRITE_VERTICAL_GLYPH_ORIENTATION {
        (::windows::core::Vtable::vtable(self).base__.GetVerticalGlyphOrientation)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLastLineWrapping<P0>(&self, islastlinewrappingenabled: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetLastLineWrapping)(::windows::core::Vtable::as_raw(self), islastlinewrappingenabled.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLastLineWrapping(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.GetLastLineWrapping)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn SetOpticalAlignment(&self, opticalalignment: DWRITE_OPTICAL_ALIGNMENT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetOpticalAlignment)(::windows::core::Vtable::as_raw(self), opticalalignment).ok()
    }
    pub unsafe fn GetOpticalAlignment(&self) -> DWRITE_OPTICAL_ALIGNMENT {
        (::windows::core::Vtable::vtable(self).base__.GetOpticalAlignment)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn SetFontFallback<P0>(&self, fontfallback: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteFontFallback>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetFontFallback)(::windows::core::Vtable::as_raw(self), fontfallback.into().abi()).ok()
    }
    pub unsafe fn GetFontFallback(&self) -> ::windows::core::Result<IDWriteFontFallback> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFontFallback)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IDWriteTextLayout4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDWriteTextLayout4 {}
impl ::core::fmt::Debug for IDWriteTextLayout4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDWriteTextLayout4").field(&self.0).finish()
    }
}
impl IDWriteTextLayout4 {
    pub unsafe fn SetTextAlignment(&self, textalignment: DWRITE_TEXT_ALIGNMENT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetTextAlignment)(::windows::core::Vtable::as_raw(self), textalignment).ok()
    }
    pub unsafe fn SetParagraphAlignment(&self, paragraphalignment: DWRITE_PARAGRAPH_ALIGNMENT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetParagraphAlignment)(::windows::core::Vtable::as_raw(self), paragraphalignment).ok()
    }
    pub unsafe fn SetWordWrapping(&self, wordwrapping: DWRITE_WORD_WRAPPING) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetWordWrapping)(::windows::core::Vtable::as_raw(self), wordwrapping).ok()
    }
    pub unsafe fn SetReadingDirection(&self, readingdirection: DWRITE_READING_DIRECTION) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetReadingDirection)(::windows::core::Vtable::as_raw(self), readingdirection).ok()
    }
    pub unsafe fn SetFlowDirection(&self, flowdirection: DWRITE_FLOW_DIRECTION) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetFlowDirection)(::windows::core::Vtable::as_raw(self), flowdirection).ok()
    }
    pub unsafe fn SetIncrementalTabStop(&self, incrementaltabstop: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetIncrementalTabStop)(::windows::core::Vtable::as_raw(self), incrementaltabstop).ok()
    }
    pub unsafe fn SetTrimming<P0>(&self, trimmingoptions: *const DWRITE_TRIMMING, trimmingsign: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteInlineObject>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetTrimming)(::windows::core::Vtable::as_raw(self), trimmingoptions, trimmingsign.into().abi()).ok()
    }
    pub unsafe fn SetLineSpacing(&self, linespacingmethod: DWRITE_LINE_SPACING_METHOD, linespacing: f32, baseline: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetLineSpacing)(::windows::core::Vtable::as_raw(self), linespacingmethod, linespacing, baseline).ok()
    }
    pub unsafe fn GetTextAlignment(&self) -> DWRITE_TEXT_ALIGNMENT {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetTextAlignment)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetParagraphAlignment(&self) -> DWRITE_PARAGRAPH_ALIGNMENT {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetParagraphAlignment)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetWordWrapping(&self) -> DWRITE_WORD_WRAPPING {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetWordWrapping)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetReadingDirection(&self) -> DWRITE_READING_DIRECTION {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetReadingDirection)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetFlowDirection(&self) -> DWRITE_FLOW_DIRECTION {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetFlowDirection)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetIncrementalTabStop(&self) -> f32 {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetIncrementalTabStop)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetTrimming(&self, trimmingoptions: *mut DWRITE_TRIMMING, trimmingsign: *mut ::core::option::Option<IDWriteInlineObject>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetTrimming)(::windows::core::Vtable::as_raw(self), trimmingoptions, ::core::mem::transmute(trimmingsign)).ok()
    }
    pub unsafe fn GetLineSpacing(&self, linespacingmethod: *mut DWRITE_LINE_SPACING_METHOD, linespacing: *mut f32, baseline: *mut f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetLineSpacing)(::windows::core::Vtable::as_raw(self), linespacingmethod, linespacing, baseline).ok()
    }
    pub unsafe fn GetFontCollection(&self) -> ::windows::core::Result<IDWriteFontCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetFontCollection)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetFontFamilyNameLength(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetFontFamilyNameLength)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetFontFamilyName(&self, fontfamilyname: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetFontFamilyName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(fontfamilyname.as_ptr()), fontfamilyname.len() as _).ok()
    }
    pub unsafe fn GetFontWeight(&self) -> DWRITE_FONT_WEIGHT {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetFontWeight)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetFontStyle(&self) -> DWRITE_FONT_STYLE {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetFontStyle)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetFontStretch(&self) -> DWRITE_FONT_STRETCH {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetFontStretch)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetFontSize(&self) -> f32 {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetFontSize)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetLocaleNameLength(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetLocaleNameLength)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetLocaleName(&self, localename: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetLocaleName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(localename.as_ptr()), localename.len() as _).ok()
    }
    pub unsafe fn SetMaxWidth(&self, maxwidth: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetMaxWidth)(::windows::core::Vtable::as_raw(self), maxwidth).ok()
    }
    pub unsafe fn SetMaxHeight(&self, maxheight: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetMaxHeight)(::windows::core::Vtable::as_raw(self), maxheight).ok()
    }
    pub unsafe fn SetFontCollection<P0>(&self, fontcollection: P0, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteFontCollection>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetFontCollection)(::windows::core::Vtable::as_raw(self), fontcollection.into().abi(), ::core::mem::transmute(textrange)).ok()
    }
    pub unsafe fn SetFontFamilyName<P0>(&self, fontfamilyname: P0, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetFontFamilyName)(::windows::core::Vtable::as_raw(self), fontfamilyname.into().abi(), ::core::mem::transmute(textrange)).ok()
    }
    pub unsafe fn SetFontWeight(&self, fontweight: DWRITE_FONT_WEIGHT, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetFontWeight)(::windows::core::Vtable::as_raw(self), fontweight, ::core::mem::transmute(textrange)).ok()
    }
    pub unsafe fn SetFontStyle(&self, fontstyle: DWRITE_FONT_STYLE, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetFontStyle)(::windows::core::Vtable::as_raw(self), fontstyle, ::core::mem::transmute(textrange)).ok()
    }
    pub unsafe fn SetFontStretch(&self, fontstretch: DWRITE_FONT_STRETCH, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetFontStretch)(::windows::core::Vtable::as_raw(self), fontstretch, ::core::mem::transmute(textrange)).ok()
    }
    pub unsafe fn SetFontSize(&self, fontsize: f32, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetFontSize)(::windows::core::Vtable::as_raw(self), fontsize, ::core::mem::transmute(textrange)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetUnderline<P0>(&self, hasunderline: P0, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetUnderline)(::windows::core::Vtable::as_raw(self), hasunderline.into(), ::core::mem::transmute(textrange)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetStrikethrough<P0>(&self, hasstrikethrough: P0, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetStrikethrough)(::windows::core::Vtable::as_raw(self), hasstrikethrough.into(), ::core::mem::transmute(textrange)).ok()
    }
    pub unsafe fn SetDrawingEffect<P0>(&self, drawingeffect: P0, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetDrawingEffect)(::windows::core::Vtable::as_raw(self), drawingeffect.into().abi(), ::core::mem::transmute(textrange)).ok()
    }
    pub unsafe fn SetInlineObject<P0>(&self, inlineobject: P0, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteInlineObject>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetInlineObject)(::windows::core::Vtable::as_raw(self), inlineobject.into().abi(), ::core::mem::transmute(textrange)).ok()
    }
    pub unsafe fn SetTypography<P0>(&self, typography: P0, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteTypography>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetTypography)(::windows::core::Vtable::as_raw(self), typography.into().abi(), ::core::mem::transmute(textrange)).ok()
    }
    pub unsafe fn SetLocaleName<P0>(&self, localename: P0, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetLocaleName)(::windows::core::Vtable::as_raw(self), localename.into().abi(), ::core::mem::transmute(textrange)).ok()
    }
    pub unsafe fn GetMaxWidth(&self) -> f32 {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetMaxWidth)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetMaxHeight(&self) -> f32 {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetMaxHeight)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetFontCollection2(&self, currentposition: u32, fontcollection: *mut ::core::option::Option<IDWriteFontCollection>, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetFontCollection2)(::windows::core::Vtable::as_raw(self), currentposition, ::core::mem::transmute(fontcollection), ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetFontFamilyNameLength2(&self, currentposition: u32, namelength: *mut u32, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetFontFamilyNameLength2)(::windows::core::Vtable::as_raw(self), currentposition, namelength, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetFontFamilyName2(&self, currentposition: u32, fontfamilyname: &mut [u16], textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetFontFamilyName2)(::windows::core::Vtable::as_raw(self), currentposition, ::core::mem::transmute(fontfamilyname.as_ptr()), fontfamilyname.len() as _, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetFontWeight2(&self, currentposition: u32, fontweight: *mut DWRITE_FONT_WEIGHT, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetFontWeight2)(::windows::core::Vtable::as_raw(self), currentposition, fontweight, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetFontStyle2(&self, currentposition: u32, fontstyle: *mut DWRITE_FONT_STYLE, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetFontStyle2)(::windows::core::Vtable::as_raw(self), currentposition, fontstyle, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetFontStretch2(&self, currentposition: u32, fontstretch: *mut DWRITE_FONT_STRETCH, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetFontStretch2)(::windows::core::Vtable::as_raw(self), currentposition, fontstretch, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetFontSize2(&self, currentposition: u32, fontsize: *mut f32, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetFontSize2)(::windows::core::Vtable::as_raw(self), currentposition, fontsize, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetUnderline(&self, currentposition: u32, hasunderline: *mut super::super::Foundation::BOOL, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetUnderline)(::windows::core::Vtable::as_raw(self), currentposition, hasunderline, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetStrikethrough(&self, currentposition: u32, hasstrikethrough: *mut super::super::Foundation::BOOL, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetStrikethrough)(::windows::core::Vtable::as_raw(self), currentposition, hasstrikethrough, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetDrawingEffect(&self, currentposition: u32, drawingeffect: *mut ::core::option::Option<::windows::core::IUnknown>, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetDrawingEffect)(::windows::core::Vtable::as_raw(self), currentposition, ::core::mem::transmute(drawingeffect), ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetInlineObject(&self, currentposition: u32, inlineobject: *mut ::core::option::Option<IDWriteInlineObject>, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetInlineObject)(::windows::core::Vtable::as_raw(self), currentposition, ::core::mem::transmute(inlineobject), ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetTypography(&self, currentposition: u32, typography: *mut ::core::option::Option<IDWriteTypography>, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetTypography)(::windows::core::Vtable::as_raw(self), currentposition, ::core::mem::transmute(typography), ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetLocaleNameLength2(&self, currentposition: u32, namelength: *mut u32, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetLocaleNameLength2)(::windows::core::Vtable::as_raw(self), currentposition, namelength, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetLocaleName2(&self, currentposition: u32, localename: &mut [u16], textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetLocaleName2)(::windows::core::Vtable::as_raw(self), currentposition, ::core::mem::transmute(localename.as_ptr()), localename.len() as _, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Draw<P0>(&self, clientdrawingcontext: ::core::option::Option<*const ::core::ffi::c_void>, renderer: P0, originx: f32, originy: f32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteTextRenderer>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.Draw)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(clientdrawingcontext.unwrap_or(::std::ptr::null())), renderer.into().abi(), originx, originy).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLineMetrics(&self, linemetrics: ::core::option::Option<&mut [DWRITE_LINE_METRICS]>, actuallinecount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetLineMetrics)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(linemetrics.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), linemetrics.as_deref().map_or(0, |slice| slice.len() as _), actuallinecount).ok()
    }
    pub unsafe fn GetMetrics(&self, textmetrics: *mut DWRITE_TEXT_METRICS) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetMetrics)(::windows::core::Vtable::as_raw(self), textmetrics).ok()
    }
    pub unsafe fn GetOverhangMetrics(&self) -> ::windows::core::Result<DWRITE_OVERHANG_METRICS> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetOverhangMetrics)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetClusterMetrics(&self, clustermetrics: ::core::option::Option<&mut [DWRITE_CLUSTER_METRICS]>, actualclustercount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetClusterMetrics)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(clustermetrics.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), clustermetrics.as_deref().map_or(0, |slice| slice.len() as _), actualclustercount).ok()
    }
    pub unsafe fn DetermineMinWidth(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.DetermineMinWidth)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HitTestPoint(&self, pointx: f32, pointy: f32, istrailinghit: *mut super::super::Foundation::BOOL, isinside: *mut super::super::Foundation::BOOL, hittestmetrics: *mut DWRITE_HIT_TEST_METRICS) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.HitTestPoint)(::windows::core::Vtable::as_raw(self), pointx, pointy, istrailinghit, isinside, hittestmetrics).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HitTestTextPosition<P0>(&self, textposition: u32, istrailinghit: P0, pointx: *mut f32, pointy: *mut f32, hittestmetrics: *mut DWRITE_HIT_TEST_METRICS) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.HitTestTextPosition)(::windows::core::Vtable::as_raw(self), textposition, istrailinghit.into(), pointx, pointy, hittestmetrics).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HitTestTextRange(&self, textposition: u32, textlength: u32, originx: f32, originy: f32, hittestmetrics: ::core::option::Option<&mut [DWRITE_HIT_TEST_METRICS]>, actualhittestmetricscount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.HitTestTextRange)(::windows::core::Vtable::as_raw(self), textposition, textlength, originx, originy, ::core::mem::transmute(hittestmetrics.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), hittestmetrics.as_deref().map_or(0, |slice| slice.len() as _), actualhittestmetricscount).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPairKerning<P0>(&self, ispairkerningenabled: P0, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPairKerning)(::windows::core::Vtable::as_raw(self), ispairkerningenabled.into(), ::core::mem::transmute(textrange)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPairKerning(&self, currentposition: u32, ispairkerningenabled: *mut super::super::Foundation::BOOL, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetPairKerning)(::windows::core::Vtable::as_raw(self), currentposition, ispairkerningenabled, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetCharacterSpacing(&self, leadingspacing: f32, trailingspacing: f32, minimumadvancewidth: f32, textrange: DWRITE_TEXT_RANGE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetCharacterSpacing)(::windows::core::Vtable::as_raw(self), leadingspacing, trailingspacing, minimumadvancewidth, ::core::mem::transmute(textrange)).ok()
    }
    pub unsafe fn GetCharacterSpacing(&self, currentposition: u32, leadingspacing: *mut f32, trailingspacing: *mut f32, minimumadvancewidth: *mut f32, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetCharacterSpacing)(::windows::core::Vtable::as_raw(self), currentposition, leadingspacing, trailingspacing, minimumadvancewidth, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetMetrics2(&self, textmetrics: *mut DWRITE_TEXT_METRICS1) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetMetrics2)(::windows::core::Vtable::as_raw(self), textmetrics).ok()
    }
    pub unsafe fn SetVerticalGlyphOrientation(&self, glyphorientation: DWRITE_VERTICAL_GLYPH_ORIENTATION) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetVerticalGlyphOrientation)(::windows::core::Vtable::as_raw(self), glyphorientation).ok()
    }
    pub unsafe fn GetVerticalGlyphOrientation(&self) -> DWRITE_VERTICAL_GLYPH_ORIENTATION {
        (::windows::core::Vtable::vtable(self).base__.base__.GetVerticalGlyphOrientation)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLastLineWrapping<P0>(&self, islastlinewrappingenabled: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetLastLineWrapping)(::windows::core::Vtable::as_raw(self), islastlinewrappingenabled.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLastLineWrapping(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.base__.GetLastLineWrapping)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn SetOpticalAlignment(&self, opticalalignment: DWRITE_OPTICAL_ALIGNMENT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetOpticalAlignment)(::windows::core::Vtable::as_raw(self), opticalalignment).ok()
    }
    pub unsafe fn GetOpticalAlignment(&self) -> DWRITE_OPTICAL_ALIGNMENT {
        (::windows::core::Vtable::vtable(self).base__.base__.GetOpticalAlignment)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn SetFontFallback<P0>(&self, fontfallback: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteFontFallback>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetFontFallback)(::windows::core::Vtable::as_raw(self), fontfallback.into().abi()).ok()
    }
    pub unsafe fn GetFontFallback(&self) -> ::windows::core::Result<IDWriteFontFallback> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetFontFallback)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn InvalidateLayout(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.InvalidateLayout)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetLineSpacing2(&self, linespacingoptions: *const DWRITE_LINE_SPACING) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetLineSpacing2)(::windows::core::Vtable::as_raw(self), linespacingoptions).ok()
    }
    pub unsafe fn GetLineSpacing2(&self, linespacingoptions: *mut DWRITE_LINE_SPACING) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetLineSpacing2)(::windows::core::Vtable::as_raw(self), linespacingoptions).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLineMetrics2(&self, linemetrics: ::core::option::Option<&mut [DWRITE_LINE_METRICS1]>, actuallinecount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetLineMetrics2)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(linemetrics.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), linemetrics.as_deref().map_or(0, |slice| slice.len() as _), actuallinecount).ok()
    }
}
impl ::core::cmp::PartialEq for IDWriteTextRenderer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDWriteTextRenderer {}
impl ::core::fmt::Debug for IDWriteTextRenderer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDWriteTextRenderer").field(&self.0).finish()
    }
}
impl IDWriteTextRenderer {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsPixelSnappingDisabled(&self, clientdrawingcontext: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.IsPixelSnappingDisabled)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(clientdrawingcontext.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCurrentTransform(&self, clientdrawingcontext: ::core::option::Option<*const ::core::ffi::c_void>, transform: *mut DWRITE_MATRIX) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetCurrentTransform)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(clientdrawingcontext.unwrap_or(::std::ptr::null())), transform).ok()
    }
    pub unsafe fn GetPixelsPerDip(&self, clientdrawingcontext: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetPixelsPerDip)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(clientdrawingcontext.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IDWriteTextRenderer1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDWriteTextRenderer1 {}
impl ::core::fmt::Debug for IDWriteTextRenderer1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDWriteTextRenderer1").field(&self.0).finish()
    }
}
impl IDWriteTextRenderer1 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsPixelSnappingDisabled(&self, clientdrawingcontext: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.IsPixelSnappingDisabled)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(clientdrawingcontext.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCurrentTransform(&self, clientdrawingcontext: ::core::option::Option<*const ::core::ffi::c_void>, transform: *mut DWRITE_MATRIX) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetCurrentTransform)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(clientdrawingcontext.unwrap_or(::std::ptr::null())), transform).ok()
    }
    pub unsafe fn GetPixelsPerDip(&self, clientdrawingcontext: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetPixelsPerDip)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(clientdrawingcontext.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DrawGlyphRun<P0>(&self, clientdrawingcontext: ::core::option::Option<*const ::core::ffi::c_void>, baselineoriginx: f32, baselineoriginy: f32, measuringmode: DWRITE_MEASURING_MODE, glyphrun: *const DWRITE_GLYPH_RUN, glyphrundescription: *const DWRITE_GLYPH_RUN_DESCRIPTION, clientdrawingeffect: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DrawGlyphRun)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(clientdrawingcontext.unwrap_or(::std::ptr::null())), baselineoriginx, baselineoriginy, measuringmode, glyphrun, glyphrundescription, clientdrawingeffect.into().abi()).ok()
    }
    pub unsafe fn DrawUnderline<P0>(&self, clientdrawingcontext: ::core::option::Option<*const ::core::ffi::c_void>, baselineoriginx: f32, baselineoriginy: f32, underline: *const DWRITE_UNDERLINE, clientdrawingeffect: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DrawUnderline)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(clientdrawingcontext.unwrap_or(::std::ptr::null())), baselineoriginx, baselineoriginy, underline, clientdrawingeffect.into().abi()).ok()
    }
    pub unsafe fn DrawStrikethrough<P0>(&self, clientdrawingcontext: ::core::option::Option<*const ::core::ffi::c_void>, baselineoriginx: f32, baselineoriginy: f32, strikethrough: *const DWRITE_STRIKETHROUGH, clientdrawingeffect: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DrawStrikethrough)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(clientdrawingcontext.unwrap_or(::std::ptr::null())), baselineoriginx, baselineoriginy, strikethrough, clientdrawingeffect.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DrawInlineObject<P0, P1, P2, P3>(&self, clientdrawingcontext: ::core::option::Option<*const ::core::ffi::c_void>, originx: f32, originy: f32, inlineobject: P0, issideways: P1, isrighttoleft: P2, clientdrawingeffect: P3) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDWriteInlineObject>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
        P2: ::std::convert::Into<super::super::Foundation::BOOL>,
        P3: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DrawInlineObject)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(clientdrawingcontext.unwrap_or(::std::ptr::null())), originx, originy, inlineobject.into().abi(), issideways.into(), isrighttoleft.into(), clientdrawingeffect.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for IDWriteTypography {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDWriteTypography {}
impl ::core::fmt::Debug for IDWriteTypography {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDWriteTypography").field(&self.0).finish()
    }
}
