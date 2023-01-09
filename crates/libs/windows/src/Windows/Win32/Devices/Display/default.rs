impl ::core::default::Default for AR_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AR_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AR_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for Adapter {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for Adapter {
    fn eq(&self, other: &Self) -> bool {
        self.AdapterName == other.AdapterName && self.numSources == other.numSources && self.sources == other.sources
    }
}
impl ::core::cmp::Eq for Adapter {}
impl ::core::fmt::Debug for Adapter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("Adapter").field("AdapterName", &self.AdapterName).field("numSources", &self.numSources).field("sources", &self.sources).finish()
    }
}
impl ::core::default::Default for Adapters {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for Adapters {
    fn eq(&self, other: &Self) -> bool {
        self.numAdapters == other.numAdapters && self.adapter == other.adapter
    }
}
impl ::core::cmp::Eq for Adapters {}
impl ::core::fmt::Debug for Adapters {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("Adapters").field("numAdapters", &self.numAdapters).field("adapter", &self.adapter).finish()
    }
}
impl ::core::default::Default for BACKLIGHT_OPTIMIZATION_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for BACKLIGHT_OPTIMIZATION_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BACKLIGHT_OPTIMIZATION_LEVEL").field(&self.0).finish()
    }
}
impl ::core::default::Default for BACKLIGHT_REDUCTION_GAMMA_RAMP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for BACKLIGHT_REDUCTION_GAMMA_RAMP {
    fn eq(&self, other: &Self) -> bool {
        self.R == other.R && self.G == other.G && self.B == other.B
    }
}
impl ::core::cmp::Eq for BACKLIGHT_REDUCTION_GAMMA_RAMP {}
impl ::core::fmt::Debug for BACKLIGHT_REDUCTION_GAMMA_RAMP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BACKLIGHT_REDUCTION_GAMMA_RAMP").field("R", &self.R).field("G", &self.G).field("B", &self.B).finish()
    }
}
impl ::core::default::Default for BANK_POSITION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for BANK_POSITION {
    fn eq(&self, other: &Self) -> bool {
        self.ReadBankPosition == other.ReadBankPosition && self.WriteBankPosition == other.WriteBankPosition
    }
}
impl ::core::cmp::Eq for BANK_POSITION {}
impl ::core::fmt::Debug for BANK_POSITION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BANK_POSITION").field("ReadBankPosition", &self.ReadBankPosition).field("WriteBankPosition", &self.WriteBankPosition).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for BLENDOBJ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::PartialEq for BLENDOBJ {
    fn eq(&self, other: &Self) -> bool {
        self.BlendFunction == other.BlendFunction
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::Eq for BLENDOBJ {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::fmt::Debug for BLENDOBJ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BLENDOBJ").field("BlendFunction", &self.BlendFunction).finish()
    }
}
impl ::core::default::Default for BRIGHTNESS_INTERFACE_VERSION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for BRIGHTNESS_INTERFACE_VERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BRIGHTNESS_INTERFACE_VERSION").field(&self.0).finish()
    }
}
impl ::core::default::Default for BRIGHTNESS_LEVEL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for BRIGHTNESS_LEVEL {
    fn eq(&self, other: &Self) -> bool {
        self.Count == other.Count && self.Level == other.Level
    }
}
impl ::core::cmp::Eq for BRIGHTNESS_LEVEL {}
impl ::core::fmt::Debug for BRIGHTNESS_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BRIGHTNESS_LEVEL").field("Count", &self.Count).field("Level", &self.Level).finish()
    }
}
impl ::core::default::Default for BRIGHTNESS_NIT_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for BRIGHTNESS_NIT_RANGE {
    fn eq(&self, other: &Self) -> bool {
        self.MinLevelInMillinit == other.MinLevelInMillinit && self.MaxLevelInMillinit == other.MaxLevelInMillinit && self.StepSizeInMillinit == other.StepSizeInMillinit
    }
}
impl ::core::cmp::Eq for BRIGHTNESS_NIT_RANGE {}
impl ::core::fmt::Debug for BRIGHTNESS_NIT_RANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BRIGHTNESS_NIT_RANGE").field("MinLevelInMillinit", &self.MinLevelInMillinit).field("MaxLevelInMillinit", &self.MaxLevelInMillinit).field("StepSizeInMillinit", &self.StepSizeInMillinit).finish()
    }
}
impl ::core::default::Default for BRIGHTNESS_NIT_RANGES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for BRIGHTNESS_NIT_RANGES {
    fn eq(&self, other: &Self) -> bool {
        self.NormalRangeCount == other.NormalRangeCount && self.RangeCount == other.RangeCount && self.PreferredMaximumBrightness == other.PreferredMaximumBrightness && self.SupportedRanges == other.SupportedRanges
    }
}
impl ::core::cmp::Eq for BRIGHTNESS_NIT_RANGES {}
impl ::core::fmt::Debug for BRIGHTNESS_NIT_RANGES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BRIGHTNESS_NIT_RANGES").field("NormalRangeCount", &self.NormalRangeCount).field("RangeCount", &self.RangeCount).field("PreferredMaximumBrightness", &self.PreferredMaximumBrightness).field("SupportedRanges", &self.SupportedRanges).finish()
    }
}
impl ::core::default::Default for BRUSHOBJ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for BRUSHOBJ {
    fn eq(&self, other: &Self) -> bool {
        self.iSolidColor == other.iSolidColor && self.pvRbrush == other.pvRbrush && self.flColorType == other.flColorType
    }
}
impl ::core::cmp::Eq for BRUSHOBJ {}
impl ::core::fmt::Debug for BRUSHOBJ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BRUSHOBJ").field("iSolidColor", &self.iSolidColor).field("pvRbrush", &self.pvRbrush).field("flColorType", &self.flColorType).finish()
    }
}
impl ::core::default::Default for BlackScreenDiagnosticsCalloutParam {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for BlackScreenDiagnosticsCalloutParam {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BlackScreenDiagnosticsCalloutParam").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CDDDXGK_REDIRBITMAPPRESENTINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CDDDXGK_REDIRBITMAPPRESENTINFO {
    fn eq(&self, other: &Self) -> bool {
        self.NumDirtyRects == other.NumDirtyRects && self.DirtyRect == other.DirtyRect && self.NumContexts == other.NumContexts && self.hContext == other.hContext && self.bDoNotSynchronizeWithDxContent == other.bDoNotSynchronizeWithDxContent
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CDDDXGK_REDIRBITMAPPRESENTINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CDDDXGK_REDIRBITMAPPRESENTINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CDDDXGK_REDIRBITMAPPRESENTINFO").field("NumDirtyRects", &self.NumDirtyRects).field("DirtyRect", &self.DirtyRect).field("NumContexts", &self.NumContexts).field("hContext", &self.hContext).field("bDoNotSynchronizeWithDxContent", &self.bDoNotSynchronizeWithDxContent).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Console"))]
impl ::core::default::Default for CHAR_IMAGE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CHROMATICITY_COORDINATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CHROMATICITY_COORDINATE {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl ::core::cmp::Eq for CHROMATICITY_COORDINATE {}
impl ::core::fmt::Debug for CHROMATICITY_COORDINATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CHROMATICITY_COORDINATE").field("x", &self.x).field("y", &self.y).finish()
    }
}
impl ::core::default::Default for CIECHROMA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CIECHROMA {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.Y == other.Y
    }
}
impl ::core::cmp::Eq for CIECHROMA {}
impl ::core::fmt::Debug for CIECHROMA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CIECHROMA").field("x", &self.x).field("y", &self.y).field("Y", &self.Y).finish()
    }
}
impl ::core::default::Default for CLIPLINE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CLIPLINE {
    fn eq(&self, other: &Self) -> bool {
        self.ptfxA == other.ptfxA && self.ptfxB == other.ptfxB && self.lStyleState == other.lStyleState && self.c == other.c && self.arun == other.arun
    }
}
impl ::core::cmp::Eq for CLIPLINE {}
impl ::core::fmt::Debug for CLIPLINE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLIPLINE").field("ptfxA", &self.ptfxA).field("ptfxB", &self.ptfxB).field("lStyleState", &self.lStyleState).field("c", &self.c).field("arun", &self.arun).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CLIPOBJ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CLIPOBJ {
    fn eq(&self, other: &Self) -> bool {
        self.iUniq == other.iUniq && self.rclBounds == other.rclBounds && self.iDComplexity == other.iDComplexity && self.iFComplexity == other.iFComplexity && self.iMode == other.iMode && self.fjOptions == other.fjOptions
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CLIPOBJ {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CLIPOBJ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLIPOBJ").field("iUniq", &self.iUniq).field("rclBounds", &self.rclBounds).field("iDComplexity", &self.iDComplexity).field("iFComplexity", &self.iFComplexity).field("iMode", &self.iMode).field("fjOptions", &self.fjOptions).finish()
    }
}
impl ::core::default::Default for COLORINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for COLORINFO {
    fn eq(&self, other: &Self) -> bool {
        self.Red == other.Red && self.Green == other.Green && self.Blue == other.Blue && self.Cyan == other.Cyan && self.Magenta == other.Magenta && self.Yellow == other.Yellow && self.AlignmentWhite == other.AlignmentWhite && self.RedGamma == other.RedGamma && self.GreenGamma == other.GreenGamma && self.BlueGamma == other.BlueGamma && self.MagentaInCyanDye == other.MagentaInCyanDye && self.YellowInCyanDye == other.YellowInCyanDye && self.CyanInMagentaDye == other.CyanInMagentaDye && self.YellowInMagentaDye == other.YellowInMagentaDye && self.CyanInYellowDye == other.CyanInYellowDye && self.MagentaInYellowDye == other.MagentaInYellowDye
    }
}
impl ::core::cmp::Eq for COLORINFO {}
impl ::core::fmt::Debug for COLORINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COLORINFO")
            .field("Red", &self.Red)
            .field("Green", &self.Green)
            .field("Blue", &self.Blue)
            .field("Cyan", &self.Cyan)
            .field("Magenta", &self.Magenta)
            .field("Yellow", &self.Yellow)
            .field("AlignmentWhite", &self.AlignmentWhite)
            .field("RedGamma", &self.RedGamma)
            .field("GreenGamma", &self.GreenGamma)
            .field("BlueGamma", &self.BlueGamma)
            .field("MagentaInCyanDye", &self.MagentaInCyanDye)
            .field("YellowInCyanDye", &self.YellowInCyanDye)
            .field("CyanInMagentaDye", &self.CyanInMagentaDye)
            .field("YellowInMagentaDye", &self.YellowInMagentaDye)
            .field("CyanInYellowDye", &self.CyanInYellowDye)
            .field("MagentaInYellowDye", &self.MagentaInYellowDye)
            .finish()
    }
}
impl ::core::default::Default for COLORSPACE_TRANSFORM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for COLORSPACE_TRANSFORM_1DLUT_CAP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for COLORSPACE_TRANSFORM_3x4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for COLORSPACE_TRANSFORM_3x4 {
    fn eq(&self, other: &Self) -> bool {
        self.ColorMatrix3x4 == other.ColorMatrix3x4 && self.ScalarMultiplier == other.ScalarMultiplier && self.LookupTable1D == other.LookupTable1D
    }
}
impl ::core::cmp::Eq for COLORSPACE_TRANSFORM_3x4 {}
impl ::core::fmt::Debug for COLORSPACE_TRANSFORM_3x4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COLORSPACE_TRANSFORM_3x4").field("ColorMatrix3x4", &self.ColorMatrix3x4).field("ScalarMultiplier", &self.ScalarMultiplier).field("LookupTable1D", &self.LookupTable1D).finish()
    }
}
impl ::core::default::Default for COLORSPACE_TRANSFORM_DATA_CAP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for COLORSPACE_TRANSFORM_DATA_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for COLORSPACE_TRANSFORM_DATA_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COLORSPACE_TRANSFORM_DATA_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for COLORSPACE_TRANSFORM_MATRIX_CAP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for COLORSPACE_TRANSFORM_MATRIX_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for COLORSPACE_TRANSFORM_MATRIX_V2 {
    fn eq(&self, other: &Self) -> bool {
        self.StageControlLookupTable1DDegamma == other.StageControlLookupTable1DDegamma && self.LookupTable1DDegamma == other.LookupTable1DDegamma && self.StageControlColorMatrix3x3 == other.StageControlColorMatrix3x3 && self.ColorMatrix3x3 == other.ColorMatrix3x3 && self.StageControlLookupTable1DRegamma == other.StageControlLookupTable1DRegamma && self.LookupTable1DRegamma == other.LookupTable1DRegamma
    }
}
impl ::core::cmp::Eq for COLORSPACE_TRANSFORM_MATRIX_V2 {}
impl ::core::fmt::Debug for COLORSPACE_TRANSFORM_MATRIX_V2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COLORSPACE_TRANSFORM_MATRIX_V2").field("StageControlLookupTable1DDegamma", &self.StageControlLookupTable1DDegamma).field("LookupTable1DDegamma", &self.LookupTable1DDegamma).field("StageControlColorMatrix3x3", &self.StageControlColorMatrix3x3).field("ColorMatrix3x3", &self.ColorMatrix3x3).field("StageControlLookupTable1DRegamma", &self.StageControlLookupTable1DRegamma).field("LookupTable1DRegamma", &self.LookupTable1DRegamma).finish()
    }
}
impl ::core::default::Default for COLORSPACE_TRANSFORM_SET_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for COLORSPACE_TRANSFORM_STAGE_CONTROL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for COLORSPACE_TRANSFORM_STAGE_CONTROL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COLORSPACE_TRANSFORM_STAGE_CONTROL").field(&self.0).finish()
    }
}
impl ::core::default::Default for COLORSPACE_TRANSFORM_TARGET_CAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for COLORSPACE_TRANSFORM_TARGET_CAPS_VERSION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for COLORSPACE_TRANSFORM_TARGET_CAPS_VERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COLORSPACE_TRANSFORM_TARGET_CAPS_VERSION").field(&self.0).finish()
    }
}
impl ::core::default::Default for COLORSPACE_TRANSFORM_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for COLORSPACE_TRANSFORM_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COLORSPACE_TRANSFORM_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DEVHTADJDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DEVHTADJDATA {
    fn eq(&self, other: &Self) -> bool {
        self.DeviceFlags == other.DeviceFlags && self.DeviceXDPI == other.DeviceXDPI && self.DeviceYDPI == other.DeviceYDPI && self.pDefHTInfo == other.pDefHTInfo && self.pAdjHTInfo == other.pAdjHTInfo
    }
}
impl ::core::cmp::Eq for DEVHTADJDATA {}
impl ::core::fmt::Debug for DEVHTADJDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVHTADJDATA").field("DeviceFlags", &self.DeviceFlags).field("DeviceXDPI", &self.DeviceXDPI).field("DeviceYDPI", &self.DeviceYDPI).field("pDefHTInfo", &self.pDefHTInfo).field("pAdjHTInfo", &self.pAdjHTInfo).finish()
    }
}
impl ::core::default::Default for DEVHTINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DEVHTINFO {
    fn eq(&self, other: &Self) -> bool {
        self.HTFlags == other.HTFlags && self.HTPatternSize == other.HTPatternSize && self.DevPelsDPI == other.DevPelsDPI && self.ColorInfo == other.ColorInfo
    }
}
impl ::core::cmp::Eq for DEVHTINFO {}
impl ::core::fmt::Debug for DEVHTINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVHTINFO").field("HTFlags", &self.HTFlags).field("HTPatternSize", &self.HTPatternSize).field("DevPelsDPI", &self.DevPelsDPI).field("ColorInfo", &self.ColorInfo).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for DEVINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::PartialEq for DEVINFO {
    fn eq(&self, other: &Self) -> bool {
        self.flGraphicsCaps == other.flGraphicsCaps && self.lfDefaultFont == other.lfDefaultFont && self.lfAnsiVarFont == other.lfAnsiVarFont && self.lfAnsiFixFont == other.lfAnsiFixFont && self.cFonts == other.cFonts && self.iDitherFormat == other.iDitherFormat && self.cxDither == other.cxDither && self.cyDither == other.cyDither && self.hpalDefault == other.hpalDefault && self.flGraphicsCaps2 == other.flGraphicsCaps2
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::Eq for DEVINFO {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::fmt::Debug for DEVINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVINFO").field("flGraphicsCaps", &self.flGraphicsCaps).field("lfDefaultFont", &self.lfDefaultFont).field("lfAnsiVarFont", &self.lfAnsiVarFont).field("lfAnsiFixFont", &self.lfAnsiFixFont).field("cFonts", &self.cFonts).field("iDitherFormat", &self.iDitherFormat).field("cxDither", &self.cxDither).field("cyDither", &self.cyDither).field("hpalDefault", &self.hpalDefault).field("flGraphicsCaps2", &self.flGraphicsCaps2).finish()
    }
}
impl ::core::default::Default for DISPLAYCONFIG_2DREGION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DISPLAYCONFIG_2DREGION {
    fn eq(&self, other: &Self) -> bool {
        self.cx == other.cx && self.cy == other.cy
    }
}
impl ::core::cmp::Eq for DISPLAYCONFIG_2DREGION {}
impl ::core::fmt::Debug for DISPLAYCONFIG_2DREGION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DISPLAYCONFIG_2DREGION").field("cx", &self.cx).field("cy", &self.cy).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DISPLAYCONFIG_ADAPTER_NAME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DISPLAYCONFIG_ADAPTER_NAME {
    fn eq(&self, other: &Self) -> bool {
        self.header == other.header && self.adapterDevicePath == other.adapterDevicePath
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DISPLAYCONFIG_ADAPTER_NAME {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DISPLAYCONFIG_ADAPTER_NAME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DISPLAYCONFIG_ADAPTER_NAME").field("header", &self.header).field("adapterDevicePath", &self.adapterDevicePath).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DISPLAYCONFIG_DESKTOP_IMAGE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DISPLAYCONFIG_DESKTOP_IMAGE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.PathSourceSize == other.PathSourceSize && self.DesktopImageRegion == other.DesktopImageRegion && self.DesktopImageClip == other.DesktopImageClip
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DISPLAYCONFIG_DESKTOP_IMAGE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DISPLAYCONFIG_DESKTOP_IMAGE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DISPLAYCONFIG_DESKTOP_IMAGE_INFO").field("PathSourceSize", &self.PathSourceSize).field("DesktopImageRegion", &self.DesktopImageRegion).field("DesktopImageClip", &self.DesktopImageClip).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DISPLAYCONFIG_DEVICE_INFO_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DISPLAYCONFIG_DEVICE_INFO_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.r#type == other.r#type && self.size == other.size && self.adapterId == other.adapterId && self.id == other.id
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DISPLAYCONFIG_DEVICE_INFO_HEADER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DISPLAYCONFIG_DEVICE_INFO_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DISPLAYCONFIG_DEVICE_INFO_HEADER").field("type", &self.r#type).field("size", &self.size).field("adapterId", &self.adapterId).field("id", &self.id).finish()
    }
}
impl ::core::default::Default for DISPLAYCONFIG_DEVICE_INFO_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DISPLAYCONFIG_DEVICE_INFO_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPLAYCONFIG_DEVICE_INFO_TYPE").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DISPLAYCONFIG_GET_MONITOR_SPECIALIZATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DISPLAYCONFIG_MODE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DISPLAYCONFIG_MODE_INFO_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DISPLAYCONFIG_MODE_INFO_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPLAYCONFIG_MODE_INFO_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DISPLAYCONFIG_PATH_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DISPLAYCONFIG_PATH_SOURCE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DISPLAYCONFIG_PATH_TARGET_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DISPLAYCONFIG_PIXELFORMAT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DISPLAYCONFIG_PIXELFORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPLAYCONFIG_PIXELFORMAT").field(&self.0).finish()
    }
}
impl ::core::default::Default for DISPLAYCONFIG_RATIONAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DISPLAYCONFIG_RATIONAL {
    fn eq(&self, other: &Self) -> bool {
        self.Numerator == other.Numerator && self.Denominator == other.Denominator
    }
}
impl ::core::cmp::Eq for DISPLAYCONFIG_RATIONAL {}
impl ::core::fmt::Debug for DISPLAYCONFIG_RATIONAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DISPLAYCONFIG_RATIONAL").field("Numerator", &self.Numerator).field("Denominator", &self.Denominator).finish()
    }
}
impl ::core::default::Default for DISPLAYCONFIG_ROTATION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DISPLAYCONFIG_ROTATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPLAYCONFIG_ROTATION").field(&self.0).finish()
    }
}
impl ::core::default::Default for DISPLAYCONFIG_SCALING {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DISPLAYCONFIG_SCALING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPLAYCONFIG_SCALING").field(&self.0).finish()
    }
}
impl ::core::default::Default for DISPLAYCONFIG_SCANLINE_ORDERING {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DISPLAYCONFIG_SCANLINE_ORDERING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPLAYCONFIG_SCANLINE_ORDERING").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DISPLAYCONFIG_SDR_WHITE_LEVEL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DISPLAYCONFIG_SDR_WHITE_LEVEL {
    fn eq(&self, other: &Self) -> bool {
        self.header == other.header && self.SDRWhiteLevel == other.SDRWhiteLevel
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DISPLAYCONFIG_SDR_WHITE_LEVEL {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DISPLAYCONFIG_SDR_WHITE_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DISPLAYCONFIG_SDR_WHITE_LEVEL").field("header", &self.header).field("SDRWhiteLevel", &self.SDRWhiteLevel).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DISPLAYCONFIG_SET_ADVANCED_COLOR_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DISPLAYCONFIG_SET_MONITOR_SPECIALIZATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DISPLAYCONFIG_SET_TARGET_PERSISTENCE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DISPLAYCONFIG_SOURCE_DEVICE_NAME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DISPLAYCONFIG_SOURCE_DEVICE_NAME {
    fn eq(&self, other: &Self) -> bool {
        self.header == other.header && self.viewGdiDeviceName == other.viewGdiDeviceName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DISPLAYCONFIG_SOURCE_DEVICE_NAME {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DISPLAYCONFIG_SOURCE_DEVICE_NAME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DISPLAYCONFIG_SOURCE_DEVICE_NAME").field("header", &self.header).field("viewGdiDeviceName", &self.viewGdiDeviceName).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DISPLAYCONFIG_SOURCE_MODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DISPLAYCONFIG_SOURCE_MODE {
    fn eq(&self, other: &Self) -> bool {
        self.width == other.width && self.height == other.height && self.pixelFormat == other.pixelFormat && self.position == other.position
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DISPLAYCONFIG_SOURCE_MODE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DISPLAYCONFIG_SOURCE_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DISPLAYCONFIG_SOURCE_MODE").field("width", &self.width).field("height", &self.height).field("pixelFormat", &self.pixelFormat).field("position", &self.position).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DISPLAYCONFIG_TARGET_BASE_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DISPLAYCONFIG_TARGET_BASE_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.header == other.header && self.baseOutputTechnology == other.baseOutputTechnology
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DISPLAYCONFIG_TARGET_BASE_TYPE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DISPLAYCONFIG_TARGET_BASE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DISPLAYCONFIG_TARGET_BASE_TYPE").field("header", &self.header).field("baseOutputTechnology", &self.baseOutputTechnology).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DISPLAYCONFIG_TARGET_DEVICE_NAME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DISPLAYCONFIG_TARGET_DEVICE_NAME_FLAGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DISPLAYCONFIG_TARGET_MODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DISPLAYCONFIG_TARGET_PREFERRED_MODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DISPLAYCONFIG_TOPOLOGY_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DISPLAYCONFIG_TOPOLOGY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPLAYCONFIG_TOPOLOGY_ID").field(&self.0).finish()
    }
}
impl ::core::default::Default for DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY").field(&self.0).finish()
    }
}
impl ::core::default::Default for DISPLAYCONFIG_VIDEO_SIGNAL_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DISPLAY_BRIGHTNESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DISPLAY_BRIGHTNESS {
    fn eq(&self, other: &Self) -> bool {
        self.ucDisplayPolicy == other.ucDisplayPolicy && self.ucACBrightness == other.ucACBrightness && self.ucDCBrightness == other.ucDCBrightness
    }
}
impl ::core::cmp::Eq for DISPLAY_BRIGHTNESS {}
impl ::core::fmt::Debug for DISPLAY_BRIGHTNESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DISPLAY_BRIGHTNESS").field("ucDisplayPolicy", &self.ucDisplayPolicy).field("ucACBrightness", &self.ucACBrightness).field("ucDCBrightness", &self.ucDCBrightness).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DRH_APIBITMAPDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DRH_APIBITMAPDATA {
    fn eq(&self, other: &Self) -> bool {
        self.pso == other.pso && self.b == other.b
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DRH_APIBITMAPDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DRH_APIBITMAPDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRH_APIBITMAPDATA").field("pso", &self.pso).field("b", &self.b).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DRIVEROBJ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DRVENABLEDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DRVENABLEDATA {
    fn eq(&self, other: &Self) -> bool {
        self.iDriverVersion == other.iDriverVersion && self.c == other.c && self.pdrvfn == other.pdrvfn
    }
}
impl ::core::cmp::Eq for DRVENABLEDATA {}
impl ::core::fmt::Debug for DRVENABLEDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRVENABLEDATA").field("iDriverVersion", &self.iDriverVersion).field("c", &self.c).field("pdrvfn", &self.pdrvfn).finish()
    }
}
impl ::core::default::Default for DRVFN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DSI_CONTROL_TRANSMISSION_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DSI_CONTROL_TRANSMISSION_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DSI_CONTROL_TRANSMISSION_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DXGK_WIN32K_PARAM_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DXGK_WIN32K_PARAM_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.PathsArray == other.PathsArray && self.ModesArray == other.ModesArray && self.NumPathArrayElements == other.NumPathArrayElements && self.NumModeArrayElements == other.NumModeArrayElements && self.SDCFlags == other.SDCFlags
    }
}
impl ::core::cmp::Eq for DXGK_WIN32K_PARAM_DATA {}
impl ::core::fmt::Debug for DXGK_WIN32K_PARAM_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGK_WIN32K_PARAM_DATA").field("PathsArray", &self.PathsArray).field("ModesArray", &self.ModesArray).field("NumPathArrayElements", &self.NumPathArrayElements).field("NumModeArrayElements", &self.NumModeArrayElements).field("SDCFlags", &self.SDCFlags).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DisplayMode {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DisplayModes {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for EMFINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::PartialEq for EMFINFO {
    fn eq(&self, other: &Self) -> bool {
        self.nSize == other.nSize && self.hdc == other.hdc && self.pvEMF == other.pvEMF && self.pvCurrentRecord == other.pvCurrentRecord
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::Eq for EMFINFO {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::fmt::Debug for EMFINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMFINFO").field("nSize", &self.nSize).field("hdc", &self.hdc).field("pvEMF", &self.pvEMF).field("pvCurrentRecord", &self.pvCurrentRecord).finish()
    }
}
impl ::core::default::Default for ENGSAFESEMAPHORE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ENGSAFESEMAPHORE {
    fn eq(&self, other: &Self) -> bool {
        self.hsem == other.hsem && self.lCount == other.lCount
    }
}
impl ::core::cmp::Eq for ENGSAFESEMAPHORE {}
impl ::core::fmt::Debug for ENGSAFESEMAPHORE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENGSAFESEMAPHORE").field("hsem", &self.hsem).field("lCount", &self.lCount).finish()
    }
}
impl ::core::default::Default for ENG_DEVICE_ATTRIBUTE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ENG_DEVICE_ATTRIBUTE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ENG_DEVICE_ATTRIBUTE").field(&self.0).finish()
    }
}
impl ::core::default::Default for ENG_EVENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ENG_EVENT {
    fn eq(&self, other: &Self) -> bool {
        self.pKEvent == other.pKEvent && self.fFlags == other.fFlags
    }
}
impl ::core::cmp::Eq for ENG_EVENT {}
impl ::core::fmt::Debug for ENG_EVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENG_EVENT").field("pKEvent", &self.pKEvent).field("fFlags", &self.fFlags).finish()
    }
}
impl ::core::default::Default for ENG_SYSTEM_ATTRIBUTE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ENG_SYSTEM_ATTRIBUTE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ENG_SYSTEM_ATTRIBUTE").field(&self.0).finish()
    }
}
impl ::core::default::Default for ENG_TIME_FIELDS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ENG_TIME_FIELDS {
    fn eq(&self, other: &Self) -> bool {
        self.usYear == other.usYear && self.usMonth == other.usMonth && self.usDay == other.usDay && self.usHour == other.usHour && self.usMinute == other.usMinute && self.usSecond == other.usSecond && self.usMilliseconds == other.usMilliseconds && self.usWeekday == other.usWeekday
    }
}
impl ::core::cmp::Eq for ENG_TIME_FIELDS {}
impl ::core::fmt::Debug for ENG_TIME_FIELDS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENG_TIME_FIELDS").field("usYear", &self.usYear).field("usMonth", &self.usMonth).field("usDay", &self.usDay).field("usHour", &self.usHour).field("usMinute", &self.usMinute).field("usSecond", &self.usSecond).field("usMilliseconds", &self.usMilliseconds).field("usWeekday", &self.usWeekday).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ENUMRECTS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ENUMRECTS {
    fn eq(&self, other: &Self) -> bool {
        self.c == other.c && self.arcl == other.arcl
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ENUMRECTS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ENUMRECTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENUMRECTS").field("c", &self.c).field("arcl", &self.arcl).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FD_DEVICEMETRICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FD_DEVICEMETRICS {
    fn eq(&self, other: &Self) -> bool {
        self.flRealizedType == other.flRealizedType
            && self.pteBase == other.pteBase
            && self.pteSide == other.pteSide
            && self.lD == other.lD
            && self.fxMaxAscender == other.fxMaxAscender
            && self.fxMaxDescender == other.fxMaxDescender
            && self.ptlUnderline1 == other.ptlUnderline1
            && self.ptlStrikeOut == other.ptlStrikeOut
            && self.ptlULThickness == other.ptlULThickness
            && self.ptlSOThickness == other.ptlSOThickness
            && self.cxMax == other.cxMax
            && self.cyMax == other.cyMax
            && self.cjGlyphMax == other.cjGlyphMax
            && self.fdxQuantized == other.fdxQuantized
            && self.lNonLinearExtLeading == other.lNonLinearExtLeading
            && self.lNonLinearIntLeading == other.lNonLinearIntLeading
            && self.lNonLinearMaxCharWidth == other.lNonLinearMaxCharWidth
            && self.lNonLinearAvgCharWidth == other.lNonLinearAvgCharWidth
            && self.lMinA == other.lMinA
            && self.lMinC == other.lMinC
            && self.lMinD == other.lMinD
            && self.alReserved == other.alReserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FD_DEVICEMETRICS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FD_DEVICEMETRICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FD_DEVICEMETRICS")
            .field("flRealizedType", &self.flRealizedType)
            .field("pteBase", &self.pteBase)
            .field("pteSide", &self.pteSide)
            .field("lD", &self.lD)
            .field("fxMaxAscender", &self.fxMaxAscender)
            .field("fxMaxDescender", &self.fxMaxDescender)
            .field("ptlUnderline1", &self.ptlUnderline1)
            .field("ptlStrikeOut", &self.ptlStrikeOut)
            .field("ptlULThickness", &self.ptlULThickness)
            .field("ptlSOThickness", &self.ptlSOThickness)
            .field("cxMax", &self.cxMax)
            .field("cyMax", &self.cyMax)
            .field("cjGlyphMax", &self.cjGlyphMax)
            .field("fdxQuantized", &self.fdxQuantized)
            .field("lNonLinearExtLeading", &self.lNonLinearExtLeading)
            .field("lNonLinearIntLeading", &self.lNonLinearIntLeading)
            .field("lNonLinearMaxCharWidth", &self.lNonLinearMaxCharWidth)
            .field("lNonLinearAvgCharWidth", &self.lNonLinearAvgCharWidth)
            .field("lMinA", &self.lMinA)
            .field("lMinC", &self.lMinC)
            .field("lMinD", &self.lMinD)
            .field("alReserved", &self.alReserved)
            .finish()
    }
}
impl ::core::default::Default for FD_GLYPHATTR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FD_GLYPHATTR {
    fn eq(&self, other: &Self) -> bool {
        self.cjThis == other.cjThis && self.cGlyphs == other.cGlyphs && self.iMode == other.iMode && self.aGlyphAttr == other.aGlyphAttr
    }
}
impl ::core::cmp::Eq for FD_GLYPHATTR {}
impl ::core::fmt::Debug for FD_GLYPHATTR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FD_GLYPHATTR").field("cjThis", &self.cjThis).field("cGlyphs", &self.cGlyphs).field("iMode", &self.iMode).field("aGlyphAttr", &self.aGlyphAttr).finish()
    }
}
impl ::core::default::Default for FD_GLYPHSET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FD_GLYPHSET {
    fn eq(&self, other: &Self) -> bool {
        self.cjThis == other.cjThis && self.flAccel == other.flAccel && self.cGlyphsSupported == other.cGlyphsSupported && self.cRuns == other.cRuns && self.awcrun == other.awcrun
    }
}
impl ::core::cmp::Eq for FD_GLYPHSET {}
impl ::core::fmt::Debug for FD_GLYPHSET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FD_GLYPHSET").field("cjThis", &self.cjThis).field("flAccel", &self.flAccel).field("cGlyphsSupported", &self.cGlyphsSupported).field("cRuns", &self.cRuns).field("awcrun", &self.awcrun).finish()
    }
}
impl ::core::default::Default for FD_KERNINGPAIR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FD_KERNINGPAIR {
    fn eq(&self, other: &Self) -> bool {
        self.wcFirst == other.wcFirst && self.wcSecond == other.wcSecond && self.fwdKern == other.fwdKern
    }
}
impl ::core::cmp::Eq for FD_KERNINGPAIR {}
impl ::core::fmt::Debug for FD_KERNINGPAIR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FD_KERNINGPAIR").field("wcFirst", &self.wcFirst).field("wcSecond", &self.wcSecond).field("fwdKern", &self.fwdKern).finish()
    }
}
impl ::core::default::Default for FD_LIGATURE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FD_LIGATURE {
    fn eq(&self, other: &Self) -> bool {
        self.culThis == other.culThis && self.ulType == other.ulType && self.cLigatures == other.cLigatures && self.alig == other.alig
    }
}
impl ::core::cmp::Eq for FD_LIGATURE {}
impl ::core::fmt::Debug for FD_LIGATURE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FD_LIGATURE").field("culThis", &self.culThis).field("ulType", &self.ulType).field("cLigatures", &self.cLigatures).field("alig", &self.alig).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for FD_XFORM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for FD_XFORM {
    fn eq(&self, other: &Self) -> bool {
        self.eXX == other.eXX && self.eXY == other.eXY && self.eYX == other.eYX && self.eYY == other.eYY
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for FD_XFORM {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::fmt::Debug for FD_XFORM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FD_XFORM").field("eXX", &self.eXX).field("eXY", &self.eXY).field("eYX", &self.eYX).field("eYY", &self.eYY).finish()
    }
}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for FD_XFORM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::PartialEq for FD_XFORM {
    fn eq(&self, other: &Self) -> bool {
        self.eXX == other.eXX && self.eXY == other.eXY && self.eYX == other.eYX && self.eYY == other.eYY
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::Eq for FD_XFORM {}
#[cfg(target_arch = "x86")]
impl ::core::fmt::Debug for FD_XFORM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FD_XFORM").field("eXX", &self.eXX).field("eXY", &self.eXY).field("eYX", &self.eYX).field("eYY", &self.eYY).finish()
    }
}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for FLOATOBJ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::PartialEq for FLOATOBJ {
    fn eq(&self, other: &Self) -> bool {
        self.ul1 == other.ul1 && self.ul2 == other.ul2
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::Eq for FLOATOBJ {}
#[cfg(target_arch = "x86")]
impl ::core::fmt::Debug for FLOATOBJ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FLOATOBJ").field("ul1", &self.ul1).field("ul2", &self.ul2).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for FLOATOBJ_XFORM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for FLOATOBJ_XFORM {
    fn eq(&self, other: &Self) -> bool {
        self.eM11 == other.eM11 && self.eM12 == other.eM12 && self.eM21 == other.eM21 && self.eM22 == other.eM22 && self.eDx == other.eDx && self.eDy == other.eDy
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for FLOATOBJ_XFORM {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::fmt::Debug for FLOATOBJ_XFORM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FLOATOBJ_XFORM").field("eM11", &self.eM11).field("eM12", &self.eM12).field("eM21", &self.eM21).field("eM22", &self.eM22).field("eDx", &self.eDx).field("eDy", &self.eDy).finish()
    }
}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for FLOATOBJ_XFORM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::PartialEq for FLOATOBJ_XFORM {
    fn eq(&self, other: &Self) -> bool {
        self.eM11 == other.eM11 && self.eM12 == other.eM12 && self.eM21 == other.eM21 && self.eM22 == other.eM22 && self.eDx == other.eDx && self.eDy == other.eDy
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::Eq for FLOATOBJ_XFORM {}
#[cfg(target_arch = "x86")]
impl ::core::fmt::Debug for FLOATOBJ_XFORM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FLOATOBJ_XFORM").field("eM11", &self.eM11).field("eM12", &self.eM12).field("eM21", &self.eM21).field("eM22", &self.eM22).field("eDx", &self.eDx).field("eDy", &self.eDy).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for FLOAT_LONG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for FLOAT_LONG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FONTDIFF {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FONTDIFF {
    fn eq(&self, other: &Self) -> bool {
        self.jReserved1 == other.jReserved1 && self.jReserved2 == other.jReserved2 && self.jReserved3 == other.jReserved3 && self.bWeight == other.bWeight && self.usWinWeight == other.usWinWeight && self.fsSelection == other.fsSelection && self.fwdAveCharWidth == other.fwdAveCharWidth && self.fwdMaxCharInc == other.fwdMaxCharInc && self.ptlCaret == other.ptlCaret
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FONTDIFF {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FONTDIFF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FONTDIFF").field("jReserved1", &self.jReserved1).field("jReserved2", &self.jReserved2).field("jReserved3", &self.jReserved3).field("bWeight", &self.bWeight).field("usWinWeight", &self.usWinWeight).field("fsSelection", &self.fsSelection).field("fwdAveCharWidth", &self.fwdAveCharWidth).field("fwdMaxCharInc", &self.fwdMaxCharInc).field("ptlCaret", &self.ptlCaret).finish()
    }
}
impl ::core::default::Default for FONTINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FONTINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cjThis == other.cjThis && self.flCaps == other.flCaps && self.cGlyphsSupported == other.cGlyphsSupported && self.cjMaxGlyph1 == other.cjMaxGlyph1 && self.cjMaxGlyph4 == other.cjMaxGlyph4 && self.cjMaxGlyph8 == other.cjMaxGlyph8 && self.cjMaxGlyph32 == other.cjMaxGlyph32
    }
}
impl ::core::cmp::Eq for FONTINFO {}
impl ::core::fmt::Debug for FONTINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FONTINFO").field("cjThis", &self.cjThis).field("flCaps", &self.flCaps).field("cGlyphsSupported", &self.cGlyphsSupported).field("cjMaxGlyph1", &self.cjMaxGlyph1).field("cjMaxGlyph4", &self.cjMaxGlyph4).field("cjMaxGlyph8", &self.cjMaxGlyph8).field("cjMaxGlyph32", &self.cjMaxGlyph32).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FONTOBJ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FONTOBJ {
    fn eq(&self, other: &Self) -> bool {
        self.iUniq == other.iUniq && self.iFace == other.iFace && self.cxMax == other.cxMax && self.flFontType == other.flFontType && self.iTTUniq == other.iTTUniq && self.iFile == other.iFile && self.sizLogResPpi == other.sizLogResPpi && self.ulStyleSize == other.ulStyleSize && self.pvConsumer == other.pvConsumer && self.pvProducer == other.pvProducer
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FONTOBJ {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FONTOBJ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FONTOBJ").field("iUniq", &self.iUniq).field("iFace", &self.iFace).field("cxMax", &self.cxMax).field("flFontType", &self.flFontType).field("iTTUniq", &self.iTTUniq).field("iFile", &self.iFile).field("sizLogResPpi", &self.sizLogResPpi).field("ulStyleSize", &self.ulStyleSize).field("pvConsumer", &self.pvConsumer).field("pvProducer", &self.pvProducer).finish()
    }
}
impl ::core::default::Default for FONTSIM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FONTSIM {
    fn eq(&self, other: &Self) -> bool {
        self.dpBold == other.dpBold && self.dpItalic == other.dpItalic && self.dpBoldItalic == other.dpBoldItalic
    }
}
impl ::core::cmp::Eq for FONTSIM {}
impl ::core::fmt::Debug for FONTSIM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FONTSIM").field("dpBold", &self.dpBold).field("dpItalic", &self.dpItalic).field("dpBoldItalic", &self.dpBoldItalic).finish()
    }
}
#[cfg(feature = "Win32_System_Console")]
impl ::core::default::Default for FONT_IMAGE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Console")]
impl ::core::cmp::PartialEq for FONT_IMAGE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.FontSize == other.FontSize && self.ImageBits == other.ImageBits
    }
}
#[cfg(feature = "Win32_System_Console")]
impl ::core::cmp::Eq for FONT_IMAGE_INFO {}
#[cfg(feature = "Win32_System_Console")]
impl ::core::fmt::Debug for FONT_IMAGE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FONT_IMAGE_INFO").field("FontSize", &self.FontSize).field("ImageBits", &self.ImageBits).finish()
    }
}
#[cfg(feature = "Win32_System_Console")]
impl ::core::default::Default for FSCNTL_SCREEN_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Console")]
impl ::core::cmp::PartialEq for FSCNTL_SCREEN_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Position == other.Position && self.ScreenSize == other.ScreenSize && self.nNumberOfChars == other.nNumberOfChars
    }
}
#[cfg(feature = "Win32_System_Console")]
impl ::core::cmp::Eq for FSCNTL_SCREEN_INFO {}
#[cfg(feature = "Win32_System_Console")]
impl ::core::fmt::Debug for FSCNTL_SCREEN_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FSCNTL_SCREEN_INFO").field("Position", &self.Position).field("ScreenSize", &self.ScreenSize).field("nNumberOfChars", &self.nNumberOfChars).finish()
    }
}
#[cfg(feature = "Win32_System_Console")]
impl ::core::default::Default for FSVIDEO_COPY_FRAME_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Console")]
impl ::core::cmp::PartialEq for FSVIDEO_COPY_FRAME_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        self.SrcScreen == other.SrcScreen && self.DestScreen == other.DestScreen
    }
}
#[cfg(feature = "Win32_System_Console")]
impl ::core::cmp::Eq for FSVIDEO_COPY_FRAME_BUFFER {}
#[cfg(feature = "Win32_System_Console")]
impl ::core::fmt::Debug for FSVIDEO_COPY_FRAME_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FSVIDEO_COPY_FRAME_BUFFER").field("SrcScreen", &self.SrcScreen).field("DestScreen", &self.DestScreen).finish()
    }
}
impl ::core::default::Default for FSVIDEO_CURSOR_POSITION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FSVIDEO_CURSOR_POSITION {
    fn eq(&self, other: &Self) -> bool {
        self.Coord == other.Coord && self.dwType == other.dwType
    }
}
impl ::core::cmp::Eq for FSVIDEO_CURSOR_POSITION {}
impl ::core::fmt::Debug for FSVIDEO_CURSOR_POSITION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FSVIDEO_CURSOR_POSITION").field("Coord", &self.Coord).field("dwType", &self.dwType).finish()
    }
}
impl ::core::default::Default for FSVIDEO_MODE_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FSVIDEO_MODE_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.VideoMode == other.VideoMode && self.VideoMemory == other.VideoMemory
    }
}
impl ::core::cmp::Eq for FSVIDEO_MODE_INFORMATION {}
impl ::core::fmt::Debug for FSVIDEO_MODE_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FSVIDEO_MODE_INFORMATION").field("VideoMode", &self.VideoMode).field("VideoMemory", &self.VideoMemory).finish()
    }
}
#[cfg(feature = "Win32_System_Console")]
impl ::core::default::Default for FSVIDEO_REVERSE_MOUSE_POINTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Console")]
impl ::core::cmp::PartialEq for FSVIDEO_REVERSE_MOUSE_POINTER {
    fn eq(&self, other: &Self) -> bool {
        self.Screen == other.Screen && self.dwType == other.dwType
    }
}
#[cfg(feature = "Win32_System_Console")]
impl ::core::cmp::Eq for FSVIDEO_REVERSE_MOUSE_POINTER {}
#[cfg(feature = "Win32_System_Console")]
impl ::core::fmt::Debug for FSVIDEO_REVERSE_MOUSE_POINTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FSVIDEO_REVERSE_MOUSE_POINTER").field("Screen", &self.Screen).field("dwType", &self.dwType).finish()
    }
}
#[cfg(feature = "Win32_System_Console")]
impl ::core::default::Default for FSVIDEO_SCREEN_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Console")]
impl ::core::cmp::PartialEq for FSVIDEO_SCREEN_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.ScreenSize == other.ScreenSize && self.FontSize == other.FontSize
    }
}
#[cfg(feature = "Win32_System_Console")]
impl ::core::cmp::Eq for FSVIDEO_SCREEN_INFORMATION {}
#[cfg(feature = "Win32_System_Console")]
impl ::core::fmt::Debug for FSVIDEO_SCREEN_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FSVIDEO_SCREEN_INFORMATION").field("ScreenSize", &self.ScreenSize).field("FontSize", &self.FontSize).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Console"))]
impl ::core::default::Default for FSVIDEO_WRITE_TO_FRAME_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Console"))]
impl ::core::cmp::PartialEq for FSVIDEO_WRITE_TO_FRAME_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        self.SrcBuffer == other.SrcBuffer && self.DestScreen == other.DestScreen
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Console"))]
impl ::core::cmp::Eq for FSVIDEO_WRITE_TO_FRAME_BUFFER {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Console"))]
impl ::core::fmt::Debug for FSVIDEO_WRITE_TO_FRAME_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FSVIDEO_WRITE_TO_FRAME_BUFFER").field("SrcBuffer", &self.SrcBuffer).field("DestScreen", &self.DestScreen).finish()
    }
}
impl ::core::default::Default for GAMMARAMP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for GAMMARAMP {
    fn eq(&self, other: &Self) -> bool {
        self.Red == other.Red && self.Green == other.Green && self.Blue == other.Blue
    }
}
impl ::core::cmp::Eq for GAMMARAMP {}
impl ::core::fmt::Debug for GAMMARAMP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GAMMARAMP").field("Red", &self.Red).field("Green", &self.Green).field("Blue", &self.Blue).finish()
    }
}
impl ::core::default::Default for GAMMA_RAMP_DXGI_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for GAMMA_RAMP_DXGI_1 {
    fn eq(&self, other: &Self) -> bool {
        self.Scale == other.Scale && self.Offset == other.Offset && self.GammaCurve == other.GammaCurve
    }
}
impl ::core::cmp::Eq for GAMMA_RAMP_DXGI_1 {}
impl ::core::fmt::Debug for GAMMA_RAMP_DXGI_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GAMMA_RAMP_DXGI_1").field("Scale", &self.Scale).field("Offset", &self.Offset).field("GammaCurve", &self.GammaCurve).finish()
    }
}
impl ::core::default::Default for GAMMA_RAMP_RGB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for GAMMA_RAMP_RGB {
    fn eq(&self, other: &Self) -> bool {
        self.Red == other.Red && self.Green == other.Green && self.Blue == other.Blue
    }
}
impl ::core::cmp::Eq for GAMMA_RAMP_RGB {}
impl ::core::fmt::Debug for GAMMA_RAMP_RGB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GAMMA_RAMP_RGB").field("Red", &self.Red).field("Green", &self.Green).field("Blue", &self.Blue).finish()
    }
}
impl ::core::default::Default for GAMMA_RAMP_RGB256x3x16 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for GAMMA_RAMP_RGB256x3x16 {
    fn eq(&self, other: &Self) -> bool {
        self.Red == other.Red && self.Green == other.Green && self.Blue == other.Blue
    }
}
impl ::core::cmp::Eq for GAMMA_RAMP_RGB256x3x16 {}
impl ::core::fmt::Debug for GAMMA_RAMP_RGB256x3x16 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GAMMA_RAMP_RGB256x3x16").field("Red", &self.Red).field("Green", &self.Green).field("Blue", &self.Blue).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GDIINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GDIINFO {
    fn eq(&self, other: &Self) -> bool {
        self.ulVersion == other.ulVersion
            && self.ulTechnology == other.ulTechnology
            && self.ulHorzSize == other.ulHorzSize
            && self.ulVertSize == other.ulVertSize
            && self.ulHorzRes == other.ulHorzRes
            && self.ulVertRes == other.ulVertRes
            && self.cBitsPixel == other.cBitsPixel
            && self.cPlanes == other.cPlanes
            && self.ulNumColors == other.ulNumColors
            && self.flRaster == other.flRaster
            && self.ulLogPixelsX == other.ulLogPixelsX
            && self.ulLogPixelsY == other.ulLogPixelsY
            && self.flTextCaps == other.flTextCaps
            && self.ulDACRed == other.ulDACRed
            && self.ulDACGreen == other.ulDACGreen
            && self.ulDACBlue == other.ulDACBlue
            && self.ulAspectX == other.ulAspectX
            && self.ulAspectY == other.ulAspectY
            && self.ulAspectXY == other.ulAspectXY
            && self.xStyleStep == other.xStyleStep
            && self.yStyleStep == other.yStyleStep
            && self.denStyleStep == other.denStyleStep
            && self.ptlPhysOffset == other.ptlPhysOffset
            && self.szlPhysSize == other.szlPhysSize
            && self.ulNumPalReg == other.ulNumPalReg
            && self.ciDevice == other.ciDevice
            && self.ulDevicePelsDPI == other.ulDevicePelsDPI
            && self.ulPrimaryOrder == other.ulPrimaryOrder
            && self.ulHTPatternSize == other.ulHTPatternSize
            && self.ulHTOutputFormat == other.ulHTOutputFormat
            && self.flHTFlags == other.flHTFlags
            && self.ulVRefresh == other.ulVRefresh
            && self.ulBltAlignment == other.ulBltAlignment
            && self.ulPanningHorzRes == other.ulPanningHorzRes
            && self.ulPanningVertRes == other.ulPanningVertRes
            && self.xPanningAlignment == other.xPanningAlignment
            && self.yPanningAlignment == other.yPanningAlignment
            && self.cxHTPat == other.cxHTPat
            && self.cyHTPat == other.cyHTPat
            && self.pHTPatA == other.pHTPatA
            && self.pHTPatB == other.pHTPatB
            && self.pHTPatC == other.pHTPatC
            && self.flShadeBlend == other.flShadeBlend
            && self.ulPhysicalPixelCharacteristics == other.ulPhysicalPixelCharacteristics
            && self.ulPhysicalPixelGamma == other.ulPhysicalPixelGamma
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GDIINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for GDIINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GDIINFO")
            .field("ulVersion", &self.ulVersion)
            .field("ulTechnology", &self.ulTechnology)
            .field("ulHorzSize", &self.ulHorzSize)
            .field("ulVertSize", &self.ulVertSize)
            .field("ulHorzRes", &self.ulHorzRes)
            .field("ulVertRes", &self.ulVertRes)
            .field("cBitsPixel", &self.cBitsPixel)
            .field("cPlanes", &self.cPlanes)
            .field("ulNumColors", &self.ulNumColors)
            .field("flRaster", &self.flRaster)
            .field("ulLogPixelsX", &self.ulLogPixelsX)
            .field("ulLogPixelsY", &self.ulLogPixelsY)
            .field("flTextCaps", &self.flTextCaps)
            .field("ulDACRed", &self.ulDACRed)
            .field("ulDACGreen", &self.ulDACGreen)
            .field("ulDACBlue", &self.ulDACBlue)
            .field("ulAspectX", &self.ulAspectX)
            .field("ulAspectY", &self.ulAspectY)
            .field("ulAspectXY", &self.ulAspectXY)
            .field("xStyleStep", &self.xStyleStep)
            .field("yStyleStep", &self.yStyleStep)
            .field("denStyleStep", &self.denStyleStep)
            .field("ptlPhysOffset", &self.ptlPhysOffset)
            .field("szlPhysSize", &self.szlPhysSize)
            .field("ulNumPalReg", &self.ulNumPalReg)
            .field("ciDevice", &self.ciDevice)
            .field("ulDevicePelsDPI", &self.ulDevicePelsDPI)
            .field("ulPrimaryOrder", &self.ulPrimaryOrder)
            .field("ulHTPatternSize", &self.ulHTPatternSize)
            .field("ulHTOutputFormat", &self.ulHTOutputFormat)
            .field("flHTFlags", &self.flHTFlags)
            .field("ulVRefresh", &self.ulVRefresh)
            .field("ulBltAlignment", &self.ulBltAlignment)
            .field("ulPanningHorzRes", &self.ulPanningHorzRes)
            .field("ulPanningVertRes", &self.ulPanningVertRes)
            .field("xPanningAlignment", &self.xPanningAlignment)
            .field("yPanningAlignment", &self.yPanningAlignment)
            .field("cxHTPat", &self.cxHTPat)
            .field("cyHTPat", &self.cyHTPat)
            .field("pHTPatA", &self.pHTPatA)
            .field("pHTPatB", &self.pHTPatB)
            .field("pHTPatC", &self.pHTPatC)
            .field("flShadeBlend", &self.flShadeBlend)
            .field("ulPhysicalPixelCharacteristics", &self.ulPhysicalPixelCharacteristics)
            .field("ulPhysicalPixelGamma", &self.ulPhysicalPixelGamma)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GLYPHBITS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GLYPHBITS {
    fn eq(&self, other: &Self) -> bool {
        self.ptlOrigin == other.ptlOrigin && self.sizlBitmap == other.sizlBitmap && self.aj == other.aj
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GLYPHBITS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for GLYPHBITS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GLYPHBITS").field("ptlOrigin", &self.ptlOrigin).field("sizlBitmap", &self.sizlBitmap).field("aj", &self.aj).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GLYPHDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GLYPHDEF {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GLYPHPOS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GLYPHPOS {
    fn eq(&self, other: &Self) -> bool {
        self.hg == other.hg && self.pgdf == other.pgdf && self.ptl == other.ptl
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GLYPHPOS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for GLYPHPOS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GLYPHPOS").field("hg", &self.hg).field("pgdf", &self.pgdf).field("ptl", &self.ptl).finish()
    }
}
impl ::core::cmp::PartialEq for ICloneViewHelper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICloneViewHelper {}
impl ::core::fmt::Debug for ICloneViewHelper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICloneViewHelper").field(&self.0).finish()
    }
}
impl ::core::default::Default for IFIEXTRA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IFIEXTRA {
    fn eq(&self, other: &Self) -> bool {
        self.ulIdentifier == other.ulIdentifier && self.dpFontSig == other.dpFontSig && self.cig == other.cig && self.dpDesignVector == other.dpDesignVector && self.dpAxesInfoW == other.dpAxesInfoW && self.aulReserved == other.aulReserved
    }
}
impl ::core::cmp::Eq for IFIEXTRA {}
impl ::core::fmt::Debug for IFIEXTRA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IFIEXTRA").field("ulIdentifier", &self.ulIdentifier).field("dpFontSig", &self.dpFontSig).field("cig", &self.cig).field("dpDesignVector", &self.dpDesignVector).field("dpAxesInfoW", &self.dpAxesInfoW).field("aulReserved", &self.aulReserved).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for IFIMETRICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for IFIMETRICS {
    fn eq(&self, other: &Self) -> bool {
        self.cjThis == other.cjThis
            && self.cjIfiExtra == other.cjIfiExtra
            && self.dpwszFamilyName == other.dpwszFamilyName
            && self.dpwszStyleName == other.dpwszStyleName
            && self.dpwszFaceName == other.dpwszFaceName
            && self.dpwszUniqueName == other.dpwszUniqueName
            && self.dpFontSim == other.dpFontSim
            && self.lEmbedId == other.lEmbedId
            && self.lItalicAngle == other.lItalicAngle
            && self.lCharBias == other.lCharBias
            && self.dpCharSets == other.dpCharSets
            && self.jWinCharSet == other.jWinCharSet
            && self.jWinPitchAndFamily == other.jWinPitchAndFamily
            && self.usWinWeight == other.usWinWeight
            && self.flInfo == other.flInfo
            && self.fsSelection == other.fsSelection
            && self.fsType == other.fsType
            && self.fwdUnitsPerEm == other.fwdUnitsPerEm
            && self.fwdLowestPPEm == other.fwdLowestPPEm
            && self.fwdWinAscender == other.fwdWinAscender
            && self.fwdWinDescender == other.fwdWinDescender
            && self.fwdMacAscender == other.fwdMacAscender
            && self.fwdMacDescender == other.fwdMacDescender
            && self.fwdMacLineGap == other.fwdMacLineGap
            && self.fwdTypoAscender == other.fwdTypoAscender
            && self.fwdTypoDescender == other.fwdTypoDescender
            && self.fwdTypoLineGap == other.fwdTypoLineGap
            && self.fwdAveCharWidth == other.fwdAveCharWidth
            && self.fwdMaxCharInc == other.fwdMaxCharInc
            && self.fwdCapHeight == other.fwdCapHeight
            && self.fwdXHeight == other.fwdXHeight
            && self.fwdSubscriptXSize == other.fwdSubscriptXSize
            && self.fwdSubscriptYSize == other.fwdSubscriptYSize
            && self.fwdSubscriptXOffset == other.fwdSubscriptXOffset
            && self.fwdSubscriptYOffset == other.fwdSubscriptYOffset
            && self.fwdSuperscriptXSize == other.fwdSuperscriptXSize
            && self.fwdSuperscriptYSize == other.fwdSuperscriptYSize
            && self.fwdSuperscriptXOffset == other.fwdSuperscriptXOffset
            && self.fwdSuperscriptYOffset == other.fwdSuperscriptYOffset
            && self.fwdUnderscoreSize == other.fwdUnderscoreSize
            && self.fwdUnderscorePosition == other.fwdUnderscorePosition
            && self.fwdStrikeoutSize == other.fwdStrikeoutSize
            && self.fwdStrikeoutPosition == other.fwdStrikeoutPosition
            && self.chFirstChar == other.chFirstChar
            && self.chLastChar == other.chLastChar
            && self.chDefaultChar == other.chDefaultChar
            && self.chBreakChar == other.chBreakChar
            && self.wcFirstChar == other.wcFirstChar
            && self.wcLastChar == other.wcLastChar
            && self.wcDefaultChar == other.wcDefaultChar
            && self.wcBreakChar == other.wcBreakChar
            && self.ptlBaseline == other.ptlBaseline
            && self.ptlAspect == other.ptlAspect
            && self.ptlCaret == other.ptlCaret
            && self.rclFontBox == other.rclFontBox
            && self.achVendId == other.achVendId
            && self.cKerningPairs == other.cKerningPairs
            && self.ulPanoseCulture == other.ulPanoseCulture
            && self.panose == other.panose
            && self.Align == other.Align
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for IFIMETRICS {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for IFIMETRICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IFIMETRICS")
            .field("cjThis", &self.cjThis)
            .field("cjIfiExtra", &self.cjIfiExtra)
            .field("dpwszFamilyName", &self.dpwszFamilyName)
            .field("dpwszStyleName", &self.dpwszStyleName)
            .field("dpwszFaceName", &self.dpwszFaceName)
            .field("dpwszUniqueName", &self.dpwszUniqueName)
            .field("dpFontSim", &self.dpFontSim)
            .field("lEmbedId", &self.lEmbedId)
            .field("lItalicAngle", &self.lItalicAngle)
            .field("lCharBias", &self.lCharBias)
            .field("dpCharSets", &self.dpCharSets)
            .field("jWinCharSet", &self.jWinCharSet)
            .field("jWinPitchAndFamily", &self.jWinPitchAndFamily)
            .field("usWinWeight", &self.usWinWeight)
            .field("flInfo", &self.flInfo)
            .field("fsSelection", &self.fsSelection)
            .field("fsType", &self.fsType)
            .field("fwdUnitsPerEm", &self.fwdUnitsPerEm)
            .field("fwdLowestPPEm", &self.fwdLowestPPEm)
            .field("fwdWinAscender", &self.fwdWinAscender)
            .field("fwdWinDescender", &self.fwdWinDescender)
            .field("fwdMacAscender", &self.fwdMacAscender)
            .field("fwdMacDescender", &self.fwdMacDescender)
            .field("fwdMacLineGap", &self.fwdMacLineGap)
            .field("fwdTypoAscender", &self.fwdTypoAscender)
            .field("fwdTypoDescender", &self.fwdTypoDescender)
            .field("fwdTypoLineGap", &self.fwdTypoLineGap)
            .field("fwdAveCharWidth", &self.fwdAveCharWidth)
            .field("fwdMaxCharInc", &self.fwdMaxCharInc)
            .field("fwdCapHeight", &self.fwdCapHeight)
            .field("fwdXHeight", &self.fwdXHeight)
            .field("fwdSubscriptXSize", &self.fwdSubscriptXSize)
            .field("fwdSubscriptYSize", &self.fwdSubscriptYSize)
            .field("fwdSubscriptXOffset", &self.fwdSubscriptXOffset)
            .field("fwdSubscriptYOffset", &self.fwdSubscriptYOffset)
            .field("fwdSuperscriptXSize", &self.fwdSuperscriptXSize)
            .field("fwdSuperscriptYSize", &self.fwdSuperscriptYSize)
            .field("fwdSuperscriptXOffset", &self.fwdSuperscriptXOffset)
            .field("fwdSuperscriptYOffset", &self.fwdSuperscriptYOffset)
            .field("fwdUnderscoreSize", &self.fwdUnderscoreSize)
            .field("fwdUnderscorePosition", &self.fwdUnderscorePosition)
            .field("fwdStrikeoutSize", &self.fwdStrikeoutSize)
            .field("fwdStrikeoutPosition", &self.fwdStrikeoutPosition)
            .field("chFirstChar", &self.chFirstChar)
            .field("chLastChar", &self.chLastChar)
            .field("chDefaultChar", &self.chDefaultChar)
            .field("chBreakChar", &self.chBreakChar)
            .field("wcFirstChar", &self.wcFirstChar)
            .field("wcLastChar", &self.wcLastChar)
            .field("wcDefaultChar", &self.wcDefaultChar)
            .field("wcBreakChar", &self.wcBreakChar)
            .field("ptlBaseline", &self.ptlBaseline)
            .field("ptlAspect", &self.ptlAspect)
            .field("ptlCaret", &self.ptlCaret)
            .field("rclFontBox", &self.rclFontBox)
            .field("achVendId", &self.achVendId)
            .field("cKerningPairs", &self.cKerningPairs)
            .field("ulPanoseCulture", &self.ulPanoseCulture)
            .field("panose", &self.panose)
            .field("Align", &self.Align)
            .finish()
    }
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for IFIMETRICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for IFIMETRICS {
    fn eq(&self, other: &Self) -> bool {
        self.cjThis == other.cjThis
            && self.cjIfiExtra == other.cjIfiExtra
            && self.dpwszFamilyName == other.dpwszFamilyName
            && self.dpwszStyleName == other.dpwszStyleName
            && self.dpwszFaceName == other.dpwszFaceName
            && self.dpwszUniqueName == other.dpwszUniqueName
            && self.dpFontSim == other.dpFontSim
            && self.lEmbedId == other.lEmbedId
            && self.lItalicAngle == other.lItalicAngle
            && self.lCharBias == other.lCharBias
            && self.dpCharSets == other.dpCharSets
            && self.jWinCharSet == other.jWinCharSet
            && self.jWinPitchAndFamily == other.jWinPitchAndFamily
            && self.usWinWeight == other.usWinWeight
            && self.flInfo == other.flInfo
            && self.fsSelection == other.fsSelection
            && self.fsType == other.fsType
            && self.fwdUnitsPerEm == other.fwdUnitsPerEm
            && self.fwdLowestPPEm == other.fwdLowestPPEm
            && self.fwdWinAscender == other.fwdWinAscender
            && self.fwdWinDescender == other.fwdWinDescender
            && self.fwdMacAscender == other.fwdMacAscender
            && self.fwdMacDescender == other.fwdMacDescender
            && self.fwdMacLineGap == other.fwdMacLineGap
            && self.fwdTypoAscender == other.fwdTypoAscender
            && self.fwdTypoDescender == other.fwdTypoDescender
            && self.fwdTypoLineGap == other.fwdTypoLineGap
            && self.fwdAveCharWidth == other.fwdAveCharWidth
            && self.fwdMaxCharInc == other.fwdMaxCharInc
            && self.fwdCapHeight == other.fwdCapHeight
            && self.fwdXHeight == other.fwdXHeight
            && self.fwdSubscriptXSize == other.fwdSubscriptXSize
            && self.fwdSubscriptYSize == other.fwdSubscriptYSize
            && self.fwdSubscriptXOffset == other.fwdSubscriptXOffset
            && self.fwdSubscriptYOffset == other.fwdSubscriptYOffset
            && self.fwdSuperscriptXSize == other.fwdSuperscriptXSize
            && self.fwdSuperscriptYSize == other.fwdSuperscriptYSize
            && self.fwdSuperscriptXOffset == other.fwdSuperscriptXOffset
            && self.fwdSuperscriptYOffset == other.fwdSuperscriptYOffset
            && self.fwdUnderscoreSize == other.fwdUnderscoreSize
            && self.fwdUnderscorePosition == other.fwdUnderscorePosition
            && self.fwdStrikeoutSize == other.fwdStrikeoutSize
            && self.fwdStrikeoutPosition == other.fwdStrikeoutPosition
            && self.chFirstChar == other.chFirstChar
            && self.chLastChar == other.chLastChar
            && self.chDefaultChar == other.chDefaultChar
            && self.chBreakChar == other.chBreakChar
            && self.wcFirstChar == other.wcFirstChar
            && self.wcLastChar == other.wcLastChar
            && self.wcDefaultChar == other.wcDefaultChar
            && self.wcBreakChar == other.wcBreakChar
            && self.ptlBaseline == other.ptlBaseline
            && self.ptlAspect == other.ptlAspect
            && self.ptlCaret == other.ptlCaret
            && self.rclFontBox == other.rclFontBox
            && self.achVendId == other.achVendId
            && self.cKerningPairs == other.cKerningPairs
            && self.ulPanoseCulture == other.ulPanoseCulture
            && self.panose == other.panose
    }
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for IFIMETRICS {}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for IFIMETRICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IFIMETRICS")
            .field("cjThis", &self.cjThis)
            .field("cjIfiExtra", &self.cjIfiExtra)
            .field("dpwszFamilyName", &self.dpwszFamilyName)
            .field("dpwszStyleName", &self.dpwszStyleName)
            .field("dpwszFaceName", &self.dpwszFaceName)
            .field("dpwszUniqueName", &self.dpwszUniqueName)
            .field("dpFontSim", &self.dpFontSim)
            .field("lEmbedId", &self.lEmbedId)
            .field("lItalicAngle", &self.lItalicAngle)
            .field("lCharBias", &self.lCharBias)
            .field("dpCharSets", &self.dpCharSets)
            .field("jWinCharSet", &self.jWinCharSet)
            .field("jWinPitchAndFamily", &self.jWinPitchAndFamily)
            .field("usWinWeight", &self.usWinWeight)
            .field("flInfo", &self.flInfo)
            .field("fsSelection", &self.fsSelection)
            .field("fsType", &self.fsType)
            .field("fwdUnitsPerEm", &self.fwdUnitsPerEm)
            .field("fwdLowestPPEm", &self.fwdLowestPPEm)
            .field("fwdWinAscender", &self.fwdWinAscender)
            .field("fwdWinDescender", &self.fwdWinDescender)
            .field("fwdMacAscender", &self.fwdMacAscender)
            .field("fwdMacDescender", &self.fwdMacDescender)
            .field("fwdMacLineGap", &self.fwdMacLineGap)
            .field("fwdTypoAscender", &self.fwdTypoAscender)
            .field("fwdTypoDescender", &self.fwdTypoDescender)
            .field("fwdTypoLineGap", &self.fwdTypoLineGap)
            .field("fwdAveCharWidth", &self.fwdAveCharWidth)
            .field("fwdMaxCharInc", &self.fwdMaxCharInc)
            .field("fwdCapHeight", &self.fwdCapHeight)
            .field("fwdXHeight", &self.fwdXHeight)
            .field("fwdSubscriptXSize", &self.fwdSubscriptXSize)
            .field("fwdSubscriptYSize", &self.fwdSubscriptYSize)
            .field("fwdSubscriptXOffset", &self.fwdSubscriptXOffset)
            .field("fwdSubscriptYOffset", &self.fwdSubscriptYOffset)
            .field("fwdSuperscriptXSize", &self.fwdSuperscriptXSize)
            .field("fwdSuperscriptYSize", &self.fwdSuperscriptYSize)
            .field("fwdSuperscriptXOffset", &self.fwdSuperscriptXOffset)
            .field("fwdSuperscriptYOffset", &self.fwdSuperscriptYOffset)
            .field("fwdUnderscoreSize", &self.fwdUnderscoreSize)
            .field("fwdUnderscorePosition", &self.fwdUnderscorePosition)
            .field("fwdStrikeoutSize", &self.fwdStrikeoutSize)
            .field("fwdStrikeoutPosition", &self.fwdStrikeoutPosition)
            .field("chFirstChar", &self.chFirstChar)
            .field("chLastChar", &self.chLastChar)
            .field("chDefaultChar", &self.chDefaultChar)
            .field("chBreakChar", &self.chBreakChar)
            .field("wcFirstChar", &self.wcFirstChar)
            .field("wcLastChar", &self.wcLastChar)
            .field("wcDefaultChar", &self.wcDefaultChar)
            .field("wcBreakChar", &self.wcBreakChar)
            .field("ptlBaseline", &self.ptlBaseline)
            .field("ptlAspect", &self.ptlAspect)
            .field("ptlCaret", &self.ptlCaret)
            .field("rclFontBox", &self.rclFontBox)
            .field("achVendId", &self.achVendId)
            .field("cKerningPairs", &self.cKerningPairs)
            .field("ulPanoseCulture", &self.ulPanoseCulture)
            .field("panose", &self.panose)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INDIRECT_DISPLAY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for INDIRECT_DISPLAY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.DisplayAdapterLuid == other.DisplayAdapterLuid && self.Flags == other.Flags && self.NumMonitors == other.NumMonitors && self.DisplayAdapterTargetBase == other.DisplayAdapterTargetBase
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for INDIRECT_DISPLAY_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for INDIRECT_DISPLAY_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INDIRECT_DISPLAY_INFO").field("DisplayAdapterLuid", &self.DisplayAdapterLuid).field("Flags", &self.Flags).field("NumMonitors", &self.NumMonitors).field("DisplayAdapterTargetBase", &self.DisplayAdapterTargetBase).finish()
    }
}
impl ::core::cmp::PartialEq for IViewHelper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IViewHelper {}
impl ::core::fmt::Debug for IViewHelper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IViewHelper").field(&self.0).finish()
    }
}
impl ::core::default::Default for LIGATURE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for LIGATURE {
    fn eq(&self, other: &Self) -> bool {
        self.culSize == other.culSize && self.pwsz == other.pwsz && self.chglyph == other.chglyph && self.ahglyph == other.ahglyph
    }
}
impl ::core::cmp::Eq for LIGATURE {}
impl ::core::fmt::Debug for LIGATURE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LIGATURE").field("culSize", &self.culSize).field("pwsz", &self.pwsz).field("chglyph", &self.chglyph).field("ahglyph", &self.ahglyph).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for LINEATTRS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for LINEATTRS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MC_COLOR_TEMPERATURE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MC_COLOR_TEMPERATURE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MC_COLOR_TEMPERATURE").field(&self.0).finish()
    }
}
impl ::core::default::Default for MC_DISPLAY_TECHNOLOGY_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MC_DISPLAY_TECHNOLOGY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MC_DISPLAY_TECHNOLOGY_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for MC_DRIVE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MC_DRIVE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MC_DRIVE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for MC_GAIN_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MC_GAIN_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MC_GAIN_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for MC_POSITION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MC_POSITION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MC_POSITION_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for MC_SIZE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MC_SIZE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MC_SIZE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for MC_TIMING_REPORT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MC_VCP_CODE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MC_VCP_CODE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MC_VCP_CODE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for MIPI_DSI_CAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIPI_DSI_CAPS {
    fn eq(&self, other: &Self) -> bool {
        self.DSITypeMajor == other.DSITypeMajor
            && self.DSITypeMinor == other.DSITypeMinor
            && self.SpecVersionMajor == other.SpecVersionMajor
            && self.SpecVersionMinor == other.SpecVersionMinor
            && self.SpecVersionPatch == other.SpecVersionPatch
            && self.TargetMaximumReturnPacketSize == other.TargetMaximumReturnPacketSize
            && self.ResultCodeFlags == other.ResultCodeFlags
            && self.ResultCodeStatus == other.ResultCodeStatus
            && self.Revision == other.Revision
            && self.Level == other.Level
            && self.DeviceClassHi == other.DeviceClassHi
            && self.DeviceClassLo == other.DeviceClassLo
            && self.ManufacturerHi == other.ManufacturerHi
            && self.ManufacturerLo == other.ManufacturerLo
            && self.ProductHi == other.ProductHi
            && self.ProductLo == other.ProductLo
            && self.LengthHi == other.LengthHi
            && self.LengthLo == other.LengthLo
    }
}
impl ::core::cmp::Eq for MIPI_DSI_CAPS {}
impl ::core::fmt::Debug for MIPI_DSI_CAPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIPI_DSI_CAPS")
            .field("DSITypeMajor", &self.DSITypeMajor)
            .field("DSITypeMinor", &self.DSITypeMinor)
            .field("SpecVersionMajor", &self.SpecVersionMajor)
            .field("SpecVersionMinor", &self.SpecVersionMinor)
            .field("SpecVersionPatch", &self.SpecVersionPatch)
            .field("TargetMaximumReturnPacketSize", &self.TargetMaximumReturnPacketSize)
            .field("ResultCodeFlags", &self.ResultCodeFlags)
            .field("ResultCodeStatus", &self.ResultCodeStatus)
            .field("Revision", &self.Revision)
            .field("Level", &self.Level)
            .field("DeviceClassHi", &self.DeviceClassHi)
            .field("DeviceClassLo", &self.DeviceClassLo)
            .field("ManufacturerHi", &self.ManufacturerHi)
            .field("ManufacturerLo", &self.ManufacturerLo)
            .field("ProductHi", &self.ProductHi)
            .field("ProductLo", &self.ProductLo)
            .field("LengthHi", &self.LengthHi)
            .field("LengthLo", &self.LengthLo)
            .finish()
    }
}
impl ::core::default::Default for MIPI_DSI_PACKET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MIPI_DSI_RESET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MIPI_DSI_TRANSMISSION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for ORIENTATION_PREFERENCE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ORIENTATION_PREFERENCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ORIENTATION_PREFERENCE").field(&self.0).finish()
    }
}
impl ::core::default::Default for OUTPUT_COLOR_ENCODING {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OUTPUT_COLOR_ENCODING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OUTPUT_COLOR_ENCODING").field(&self.0).finish()
    }
}
impl ::core::default::Default for OUTPUT_WIRE_COLOR_SPACE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OUTPUT_WIRE_COLOR_SPACE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OUTPUT_WIRE_COLOR_SPACE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for OUTPUT_WIRE_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for OUTPUT_WIRE_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.ColorEncoding == other.ColorEncoding && self.BitsPerPixel == other.BitsPerPixel
    }
}
impl ::core::cmp::Eq for OUTPUT_WIRE_FORMAT {}
impl ::core::fmt::Debug for OUTPUT_WIRE_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OUTPUT_WIRE_FORMAT").field("ColorEncoding", &self.ColorEncoding).field("BitsPerPixel", &self.BitsPerPixel).finish()
    }
}
impl ::core::default::Default for PALOBJ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PALOBJ {
    fn eq(&self, other: &Self) -> bool {
        self.ulReserved == other.ulReserved
    }
}
impl ::core::cmp::Eq for PALOBJ {}
impl ::core::fmt::Debug for PALOBJ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PALOBJ").field("ulReserved", &self.ulReserved).finish()
    }
}
impl ::core::default::Default for PANEL_BRIGHTNESS_SENSOR_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for PANEL_GET_BACKLIGHT_REDUCTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PANEL_GET_BACKLIGHT_REDUCTION {
    fn eq(&self, other: &Self) -> bool {
        self.BacklightUsersetting == other.BacklightUsersetting && self.BacklightEffective == other.BacklightEffective && self.GammaRamp == other.GammaRamp
    }
}
impl ::core::cmp::Eq for PANEL_GET_BACKLIGHT_REDUCTION {}
impl ::core::fmt::Debug for PANEL_GET_BACKLIGHT_REDUCTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PANEL_GET_BACKLIGHT_REDUCTION").field("BacklightUsersetting", &self.BacklightUsersetting).field("BacklightEffective", &self.BacklightEffective).field("GammaRamp", &self.GammaRamp).finish()
    }
}
impl ::core::default::Default for PANEL_GET_BRIGHTNESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for PANEL_QUERY_BRIGHTNESS_CAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for PANEL_QUERY_BRIGHTNESS_RANGES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for PANEL_SET_BACKLIGHT_OPTIMIZATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PANEL_SET_BACKLIGHT_OPTIMIZATION {
    fn eq(&self, other: &Self) -> bool {
        self.Level == other.Level
    }
}
impl ::core::cmp::Eq for PANEL_SET_BACKLIGHT_OPTIMIZATION {}
impl ::core::fmt::Debug for PANEL_SET_BACKLIGHT_OPTIMIZATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PANEL_SET_BACKLIGHT_OPTIMIZATION").field("Level", &self.Level).finish()
    }
}
impl ::core::default::Default for PANEL_SET_BRIGHTNESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for PANEL_SET_BRIGHTNESS_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for PATHDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PATHDATA {
    fn eq(&self, other: &Self) -> bool {
        self.flags == other.flags && self.count == other.count && self.pptfx == other.pptfx
    }
}
impl ::core::cmp::Eq for PATHDATA {}
impl ::core::fmt::Debug for PATHDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PATHDATA").field("flags", &self.flags).field("count", &self.count).field("pptfx", &self.pptfx).finish()
    }
}
impl ::core::default::Default for PATHOBJ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PATHOBJ {
    fn eq(&self, other: &Self) -> bool {
        self.fl == other.fl && self.cCurves == other.cCurves
    }
}
impl ::core::cmp::Eq for PATHOBJ {}
impl ::core::fmt::Debug for PATHOBJ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PATHOBJ").field("fl", &self.fl).field("cCurves", &self.cCurves).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PERBANDINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PERBANDINFO {
    fn eq(&self, other: &Self) -> bool {
        self.bRepeatThisBand == other.bRepeatThisBand && self.szlBand == other.szlBand && self.ulHorzRes == other.ulHorzRes && self.ulVertRes == other.ulVertRes
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PERBANDINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PERBANDINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERBANDINFO").field("bRepeatThisBand", &self.bRepeatThisBand).field("szlBand", &self.szlBand).field("ulHorzRes", &self.ulHorzRes).field("ulVertRes", &self.ulVertRes).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PHYSICAL_MONITOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for POINTE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for POINTE {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for POINTE {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::fmt::Debug for POINTE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("POINTE").field("x", &self.x).field("y", &self.y).finish()
    }
}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for POINTE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::PartialEq for POINTE {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::Eq for POINTE {}
#[cfg(target_arch = "x86")]
impl ::core::fmt::Debug for POINTE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("POINTE").field("x", &self.x).field("y", &self.y).finish()
    }
}
impl ::core::default::Default for POINTFIX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for POINTFIX {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl ::core::cmp::Eq for POINTFIX {}
impl ::core::fmt::Debug for POINTFIX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("POINTFIX").field("x", &self.x).field("y", &self.y).finish()
    }
}
impl ::core::default::Default for POINTQF {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for POINTQF {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl ::core::cmp::Eq for POINTQF {}
impl ::core::fmt::Debug for POINTQF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("POINTQF").field("x", &self.x).field("y", &self.y).finish()
    }
}
impl ::core::default::Default for RECTFX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RECTFX {
    fn eq(&self, other: &Self) -> bool {
        self.xLeft == other.xLeft && self.yTop == other.yTop && self.xRight == other.xRight && self.yBottom == other.yBottom
    }
}
impl ::core::cmp::Eq for RECTFX {}
impl ::core::fmt::Debug for RECTFX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RECTFX").field("xLeft", &self.xLeft).field("yTop", &self.yTop).field("xRight", &self.xRight).field("yBottom", &self.yBottom).finish()
    }
}
impl ::core::default::Default for RUN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RUN {
    fn eq(&self, other: &Self) -> bool {
        self.iStart == other.iStart && self.iStop == other.iStop
    }
}
impl ::core::cmp::Eq for RUN {}
impl ::core::fmt::Debug for RUN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RUN").field("iStart", &self.iStart).field("iStop", &self.iStop).finish()
    }
}
impl ::core::default::Default for SET_ACTIVE_COLOR_PROFILE_NAME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SET_ACTIVE_COLOR_PROFILE_NAME {
    fn eq(&self, other: &Self) -> bool {
        self.ColorProfileName == other.ColorProfileName
    }
}
impl ::core::cmp::Eq for SET_ACTIVE_COLOR_PROFILE_NAME {}
impl ::core::fmt::Debug for SET_ACTIVE_COLOR_PROFILE_NAME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SET_ACTIVE_COLOR_PROFILE_NAME").field("ColorProfileName", &self.ColorProfileName).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for STROBJ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for STROBJ {
    fn eq(&self, other: &Self) -> bool {
        self.cGlyphs == other.cGlyphs && self.flAccel == other.flAccel && self.ulCharInc == other.ulCharInc && self.rclBkGround == other.rclBkGround && self.pgp == other.pgp && self.pwszOrg == other.pwszOrg
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for STROBJ {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for STROBJ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STROBJ").field("cGlyphs", &self.cGlyphs).field("flAccel", &self.flAccel).field("ulCharInc", &self.ulCharInc).field("rclBkGround", &self.rclBkGround).field("pgp", &self.pgp).field("pwszOrg", &self.pwszOrg).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SURFOBJ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SURFOBJ {
    fn eq(&self, other: &Self) -> bool {
        self.dhsurf == other.dhsurf && self.hsurf == other.hsurf && self.dhpdev == other.dhpdev && self.hdev == other.hdev && self.sizlBitmap == other.sizlBitmap && self.cjBits == other.cjBits && self.pvBits == other.pvBits && self.pvScan0 == other.pvScan0 && self.lDelta == other.lDelta && self.iUniq == other.iUniq && self.iBitmapFormat == other.iBitmapFormat && self.iType == other.iType && self.fjBitmap == other.fjBitmap
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SURFOBJ {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SURFOBJ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SURFOBJ").field("dhsurf", &self.dhsurf).field("hsurf", &self.hsurf).field("dhpdev", &self.dhpdev).field("hdev", &self.hdev).field("sizlBitmap", &self.sizlBitmap).field("cjBits", &self.cjBits).field("pvBits", &self.pvBits).field("pvScan0", &self.pvScan0).field("lDelta", &self.lDelta).field("iUniq", &self.iUniq).field("iBitmapFormat", &self.iBitmapFormat).field("iType", &self.iType).field("fjBitmap", &self.fjBitmap).finish()
    }
}
impl ::core::default::Default for Sources {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for Sources {
    fn eq(&self, other: &Self) -> bool {
        self.sourceId == other.sourceId && self.numTargets == other.numTargets && self.aTargets == other.aTargets
    }
}
impl ::core::cmp::Eq for Sources {}
impl ::core::fmt::Debug for Sources {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("Sources").field("sourceId", &self.sourceId).field("numTargets", &self.numTargets).field("aTargets", &self.aTargets).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TYPE1_FONT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TYPE1_FONT {
    fn eq(&self, other: &Self) -> bool {
        self.hPFM == other.hPFM && self.hPFB == other.hPFB && self.ulIdentifier == other.ulIdentifier
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TYPE1_FONT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TYPE1_FONT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TYPE1_FONT").field("hPFM", &self.hPFM).field("hPFB", &self.hPFB).field("ulIdentifier", &self.ulIdentifier).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for VGA_CHAR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for VGA_CHAR {
    fn eq(&self, other: &Self) -> bool {
        self.Char == other.Char && self.Attributes == other.Attributes
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for VGA_CHAR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for VGA_CHAR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VGA_CHAR").field("Char", &self.Char).field("Attributes", &self.Attributes).finish()
    }
}
impl ::core::default::Default for VIDEOPARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VIDEOPARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Guid == other.Guid
            && self.dwOffset == other.dwOffset
            && self.dwCommand == other.dwCommand
            && self.dwFlags == other.dwFlags
            && self.dwMode == other.dwMode
            && self.dwTVStandard == other.dwTVStandard
            && self.dwAvailableModes == other.dwAvailableModes
            && self.dwAvailableTVStandard == other.dwAvailableTVStandard
            && self.dwFlickerFilter == other.dwFlickerFilter
            && self.dwOverScanX == other.dwOverScanX
            && self.dwOverScanY == other.dwOverScanY
            && self.dwMaxUnscaledX == other.dwMaxUnscaledX
            && self.dwMaxUnscaledY == other.dwMaxUnscaledY
            && self.dwPositionX == other.dwPositionX
            && self.dwPositionY == other.dwPositionY
            && self.dwBrightness == other.dwBrightness
            && self.dwContrast == other.dwContrast
            && self.dwCPType == other.dwCPType
            && self.dwCPCommand == other.dwCPCommand
            && self.dwCPStandard == other.dwCPStandard
            && self.dwCPKey == other.dwCPKey
            && self.bCP_APSTriggerBits == other.bCP_APSTriggerBits
            && self.bOEMCopyProtection == other.bOEMCopyProtection
    }
}
impl ::core::cmp::Eq for VIDEOPARAMETERS {}
impl ::core::fmt::Debug for VIDEOPARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VIDEOPARAMETERS")
            .field("Guid", &self.Guid)
            .field("dwOffset", &self.dwOffset)
            .field("dwCommand", &self.dwCommand)
            .field("dwFlags", &self.dwFlags)
            .field("dwMode", &self.dwMode)
            .field("dwTVStandard", &self.dwTVStandard)
            .field("dwAvailableModes", &self.dwAvailableModes)
            .field("dwAvailableTVStandard", &self.dwAvailableTVStandard)
            .field("dwFlickerFilter", &self.dwFlickerFilter)
            .field("dwOverScanX", &self.dwOverScanX)
            .field("dwOverScanY", &self.dwOverScanY)
            .field("dwMaxUnscaledX", &self.dwMaxUnscaledX)
            .field("dwMaxUnscaledY", &self.dwMaxUnscaledY)
            .field("dwPositionX", &self.dwPositionX)
            .field("dwPositionY", &self.dwPositionY)
            .field("dwBrightness", &self.dwBrightness)
            .field("dwContrast", &self.dwContrast)
            .field("dwCPType", &self.dwCPType)
            .field("dwCPCommand", &self.dwCPCommand)
            .field("dwCPStandard", &self.dwCPStandard)
            .field("dwCPKey", &self.dwCPKey)
            .field("bCP_APSTriggerBits", &self.bCP_APSTriggerBits)
            .field("bOEMCopyProtection", &self.bOEMCopyProtection)
            .finish()
    }
}
impl ::core::default::Default for VIDEO_BANK_SELECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VIDEO_BANK_SELECT {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.Size == other.Size && self.BankingFlags == other.BankingFlags && self.BankingType == other.BankingType && self.PlanarHCBankingType == other.PlanarHCBankingType && self.BitmapWidthInBytes == other.BitmapWidthInBytes && self.BitmapSize == other.BitmapSize && self.Granularity == other.Granularity && self.PlanarHCGranularity == other.PlanarHCGranularity && self.CodeOffset == other.CodeOffset && self.PlanarHCBankCodeOffset == other.PlanarHCBankCodeOffset && self.PlanarHCEnableCodeOffset == other.PlanarHCEnableCodeOffset && self.PlanarHCDisableCodeOffset == other.PlanarHCDisableCodeOffset
    }
}
impl ::core::cmp::Eq for VIDEO_BANK_SELECT {}
impl ::core::fmt::Debug for VIDEO_BANK_SELECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VIDEO_BANK_SELECT")
            .field("Length", &self.Length)
            .field("Size", &self.Size)
            .field("BankingFlags", &self.BankingFlags)
            .field("BankingType", &self.BankingType)
            .field("PlanarHCBankingType", &self.PlanarHCBankingType)
            .field("BitmapWidthInBytes", &self.BitmapWidthInBytes)
            .field("BitmapSize", &self.BitmapSize)
            .field("Granularity", &self.Granularity)
            .field("PlanarHCGranularity", &self.PlanarHCGranularity)
            .field("CodeOffset", &self.CodeOffset)
            .field("PlanarHCBankCodeOffset", &self.PlanarHCBankCodeOffset)
            .field("PlanarHCEnableCodeOffset", &self.PlanarHCEnableCodeOffset)
            .field("PlanarHCDisableCodeOffset", &self.PlanarHCDisableCodeOffset)
            .finish()
    }
}
impl ::core::default::Default for VIDEO_BANK_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for VIDEO_BANK_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VIDEO_BANK_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for VIDEO_BRIGHTNESS_POLICY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for VIDEO_BRIGHTNESS_POLICY {
    fn eq(&self, other: &Self) -> bool {
        self.DefaultToBiosPolicy == other.DefaultToBiosPolicy && self.LevelCount == other.LevelCount && self.Level == other.Level
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for VIDEO_BRIGHTNESS_POLICY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for VIDEO_BRIGHTNESS_POLICY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VIDEO_BRIGHTNESS_POLICY").field("DefaultToBiosPolicy", &self.DefaultToBiosPolicy).field("LevelCount", &self.LevelCount).field("Level", &self.Level).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for VIDEO_BRIGHTNESS_POLICY_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for VIDEO_BRIGHTNESS_POLICY_0 {
    fn eq(&self, other: &Self) -> bool {
        self.BatteryLevel == other.BatteryLevel && self.Brightness == other.Brightness
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for VIDEO_BRIGHTNESS_POLICY_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for VIDEO_BRIGHTNESS_POLICY_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VIDEO_BRIGHTNESS_POLICY_0").field("BatteryLevel", &self.BatteryLevel).field("Brightness", &self.Brightness).finish()
    }
}
impl ::core::default::Default for VIDEO_CLUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for VIDEO_CLUTDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VIDEO_CLUTDATA {
    fn eq(&self, other: &Self) -> bool {
        self.Red == other.Red && self.Green == other.Green && self.Blue == other.Blue && self.Unused == other.Unused
    }
}
impl ::core::cmp::Eq for VIDEO_CLUTDATA {}
impl ::core::fmt::Debug for VIDEO_CLUTDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VIDEO_CLUTDATA").field("Red", &self.Red).field("Green", &self.Green).field("Blue", &self.Blue).field("Unused", &self.Unused).finish()
    }
}
impl ::core::default::Default for VIDEO_COLOR_CAPABILITIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VIDEO_COLOR_CAPABILITIES {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length
            && self.AttributeFlags == other.AttributeFlags
            && self.RedPhosphoreDecay == other.RedPhosphoreDecay
            && self.GreenPhosphoreDecay == other.GreenPhosphoreDecay
            && self.BluePhosphoreDecay == other.BluePhosphoreDecay
            && self.WhiteChromaticity_x == other.WhiteChromaticity_x
            && self.WhiteChromaticity_y == other.WhiteChromaticity_y
            && self.WhiteChromaticity_Y == other.WhiteChromaticity_Y
            && self.RedChromaticity_x == other.RedChromaticity_x
            && self.RedChromaticity_y == other.RedChromaticity_y
            && self.GreenChromaticity_x == other.GreenChromaticity_x
            && self.GreenChromaticity_y == other.GreenChromaticity_y
            && self.BlueChromaticity_x == other.BlueChromaticity_x
            && self.BlueChromaticity_y == other.BlueChromaticity_y
            && self.WhiteGamma == other.WhiteGamma
            && self.RedGamma == other.RedGamma
            && self.GreenGamma == other.GreenGamma
            && self.BlueGamma == other.BlueGamma
    }
}
impl ::core::cmp::Eq for VIDEO_COLOR_CAPABILITIES {}
impl ::core::fmt::Debug for VIDEO_COLOR_CAPABILITIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VIDEO_COLOR_CAPABILITIES")
            .field("Length", &self.Length)
            .field("AttributeFlags", &self.AttributeFlags)
            .field("RedPhosphoreDecay", &self.RedPhosphoreDecay)
            .field("GreenPhosphoreDecay", &self.GreenPhosphoreDecay)
            .field("BluePhosphoreDecay", &self.BluePhosphoreDecay)
            .field("WhiteChromaticity_x", &self.WhiteChromaticity_x)
            .field("WhiteChromaticity_y", &self.WhiteChromaticity_y)
            .field("WhiteChromaticity_Y", &self.WhiteChromaticity_Y)
            .field("RedChromaticity_x", &self.RedChromaticity_x)
            .field("RedChromaticity_y", &self.RedChromaticity_y)
            .field("GreenChromaticity_x", &self.GreenChromaticity_x)
            .field("GreenChromaticity_y", &self.GreenChromaticity_y)
            .field("BlueChromaticity_x", &self.BlueChromaticity_x)
            .field("BlueChromaticity_y", &self.BlueChromaticity_y)
            .field("WhiteGamma", &self.WhiteGamma)
            .field("RedGamma", &self.RedGamma)
            .field("GreenGamma", &self.GreenGamma)
            .field("BlueGamma", &self.BlueGamma)
            .finish()
    }
}
impl ::core::default::Default for VIDEO_COLOR_LUT_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VIDEO_COLOR_LUT_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.LutDataFormat == other.LutDataFormat && self.LutData == other.LutData
    }
}
impl ::core::cmp::Eq for VIDEO_COLOR_LUT_DATA {}
impl ::core::fmt::Debug for VIDEO_COLOR_LUT_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VIDEO_COLOR_LUT_DATA").field("Length", &self.Length).field("LutDataFormat", &self.LutDataFormat).field("LutData", &self.LutData).finish()
    }
}
impl ::core::default::Default for VIDEO_CURSOR_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VIDEO_CURSOR_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        self.Width == other.Width && self.Height == other.Height && self.Column == other.Column && self.Row == other.Row && self.Rate == other.Rate && self.Enable == other.Enable
    }
}
impl ::core::cmp::Eq for VIDEO_CURSOR_ATTRIBUTES {}
impl ::core::fmt::Debug for VIDEO_CURSOR_ATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VIDEO_CURSOR_ATTRIBUTES").field("Width", &self.Width).field("Height", &self.Height).field("Column", &self.Column).field("Row", &self.Row).field("Rate", &self.Rate).field("Enable", &self.Enable).finish()
    }
}
impl ::core::default::Default for VIDEO_CURSOR_POSITION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VIDEO_CURSOR_POSITION {
    fn eq(&self, other: &Self) -> bool {
        self.Column == other.Column && self.Row == other.Row
    }
}
impl ::core::cmp::Eq for VIDEO_CURSOR_POSITION {}
impl ::core::fmt::Debug for VIDEO_CURSOR_POSITION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VIDEO_CURSOR_POSITION").field("Column", &self.Column).field("Row", &self.Row).finish()
    }
}
impl ::core::default::Default for VIDEO_DEVICE_SESSION_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VIDEO_DEVICE_SESSION_STATUS {
    fn eq(&self, other: &Self) -> bool {
        self.bEnable == other.bEnable && self.bSuccess == other.bSuccess
    }
}
impl ::core::cmp::Eq for VIDEO_DEVICE_SESSION_STATUS {}
impl ::core::fmt::Debug for VIDEO_DEVICE_SESSION_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VIDEO_DEVICE_SESSION_STATUS").field("bEnable", &self.bEnable).field("bSuccess", &self.bSuccess).finish()
    }
}
impl ::core::default::Default for VIDEO_HARDWARE_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VIDEO_HARDWARE_STATE {
    fn eq(&self, other: &Self) -> bool {
        self.StateHeader == other.StateHeader && self.StateLength == other.StateLength
    }
}
impl ::core::cmp::Eq for VIDEO_HARDWARE_STATE {}
impl ::core::fmt::Debug for VIDEO_HARDWARE_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VIDEO_HARDWARE_STATE").field("StateHeader", &self.StateHeader).field("StateLength", &self.StateLength).finish()
    }
}
impl ::core::default::Default for VIDEO_HARDWARE_STATE_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VIDEO_HARDWARE_STATE_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length
            && self.PortValue == other.PortValue
            && self.AttribIndexDataState == other.AttribIndexDataState
            && self.BasicSequencerOffset == other.BasicSequencerOffset
            && self.BasicCrtContOffset == other.BasicCrtContOffset
            && self.BasicGraphContOffset == other.BasicGraphContOffset
            && self.BasicAttribContOffset == other.BasicAttribContOffset
            && self.BasicDacOffset == other.BasicDacOffset
            && self.BasicLatchesOffset == other.BasicLatchesOffset
            && self.ExtendedSequencerOffset == other.ExtendedSequencerOffset
            && self.ExtendedCrtContOffset == other.ExtendedCrtContOffset
            && self.ExtendedGraphContOffset == other.ExtendedGraphContOffset
            && self.ExtendedAttribContOffset == other.ExtendedAttribContOffset
            && self.ExtendedDacOffset == other.ExtendedDacOffset
            && self.ExtendedValidatorStateOffset == other.ExtendedValidatorStateOffset
            && self.ExtendedMiscDataOffset == other.ExtendedMiscDataOffset
            && self.PlaneLength == other.PlaneLength
            && self.Plane1Offset == other.Plane1Offset
            && self.Plane2Offset == other.Plane2Offset
            && self.Plane3Offset == other.Plane3Offset
            && self.Plane4Offset == other.Plane4Offset
            && self.VGAStateFlags == other.VGAStateFlags
            && self.DIBOffset == other.DIBOffset
            && self.DIBBitsPerPixel == other.DIBBitsPerPixel
            && self.DIBXResolution == other.DIBXResolution
            && self.DIBYResolution == other.DIBYResolution
            && self.DIBXlatOffset == other.DIBXlatOffset
            && self.DIBXlatLength == other.DIBXlatLength
            && self.VesaInfoOffset == other.VesaInfoOffset
            && self.FrameBufferData == other.FrameBufferData
    }
}
impl ::core::cmp::Eq for VIDEO_HARDWARE_STATE_HEADER {}
impl ::core::fmt::Debug for VIDEO_HARDWARE_STATE_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VIDEO_HARDWARE_STATE_HEADER")
            .field("Length", &self.Length)
            .field("PortValue", &self.PortValue)
            .field("AttribIndexDataState", &self.AttribIndexDataState)
            .field("BasicSequencerOffset", &self.BasicSequencerOffset)
            .field("BasicCrtContOffset", &self.BasicCrtContOffset)
            .field("BasicGraphContOffset", &self.BasicGraphContOffset)
            .field("BasicAttribContOffset", &self.BasicAttribContOffset)
            .field("BasicDacOffset", &self.BasicDacOffset)
            .field("BasicLatchesOffset", &self.BasicLatchesOffset)
            .field("ExtendedSequencerOffset", &self.ExtendedSequencerOffset)
            .field("ExtendedCrtContOffset", &self.ExtendedCrtContOffset)
            .field("ExtendedGraphContOffset", &self.ExtendedGraphContOffset)
            .field("ExtendedAttribContOffset", &self.ExtendedAttribContOffset)
            .field("ExtendedDacOffset", &self.ExtendedDacOffset)
            .field("ExtendedValidatorStateOffset", &self.ExtendedValidatorStateOffset)
            .field("ExtendedMiscDataOffset", &self.ExtendedMiscDataOffset)
            .field("PlaneLength", &self.PlaneLength)
            .field("Plane1Offset", &self.Plane1Offset)
            .field("Plane2Offset", &self.Plane2Offset)
            .field("Plane3Offset", &self.Plane3Offset)
            .field("Plane4Offset", &self.Plane4Offset)
            .field("VGAStateFlags", &self.VGAStateFlags)
            .field("DIBOffset", &self.DIBOffset)
            .field("DIBBitsPerPixel", &self.DIBBitsPerPixel)
            .field("DIBXResolution", &self.DIBXResolution)
            .field("DIBYResolution", &self.DIBYResolution)
            .field("DIBXlatOffset", &self.DIBXlatOffset)
            .field("DIBXlatLength", &self.DIBXlatLength)
            .field("VesaInfoOffset", &self.VesaInfoOffset)
            .field("FrameBufferData", &self.FrameBufferData)
            .finish()
    }
}
impl ::core::default::Default for VIDEO_LOAD_FONT_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VIDEO_LOAD_FONT_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.WidthInPixels == other.WidthInPixels && self.HeightInPixels == other.HeightInPixels && self.FontSize == other.FontSize && self.Font == other.Font
    }
}
impl ::core::cmp::Eq for VIDEO_LOAD_FONT_INFORMATION {}
impl ::core::fmt::Debug for VIDEO_LOAD_FONT_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VIDEO_LOAD_FONT_INFORMATION").field("WidthInPixels", &self.WidthInPixels).field("HeightInPixels", &self.HeightInPixels).field("FontSize", &self.FontSize).field("Font", &self.Font).finish()
    }
}
impl ::core::default::Default for VIDEO_LUT_RGB256WORDS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VIDEO_LUT_RGB256WORDS {
    fn eq(&self, other: &Self) -> bool {
        self.Red == other.Red && self.Green == other.Green && self.Blue == other.Blue
    }
}
impl ::core::cmp::Eq for VIDEO_LUT_RGB256WORDS {}
impl ::core::fmt::Debug for VIDEO_LUT_RGB256WORDS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VIDEO_LUT_RGB256WORDS").field("Red", &self.Red).field("Green", &self.Green).field("Blue", &self.Blue).finish()
    }
}
impl ::core::default::Default for VIDEO_MEMORY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VIDEO_MEMORY {
    fn eq(&self, other: &Self) -> bool {
        self.RequestedVirtualAddress == other.RequestedVirtualAddress
    }
}
impl ::core::cmp::Eq for VIDEO_MEMORY {}
impl ::core::fmt::Debug for VIDEO_MEMORY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VIDEO_MEMORY").field("RequestedVirtualAddress", &self.RequestedVirtualAddress).finish()
    }
}
impl ::core::default::Default for VIDEO_MEMORY_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VIDEO_MEMORY_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.VideoRamBase == other.VideoRamBase && self.VideoRamLength == other.VideoRamLength && self.FrameBufferBase == other.FrameBufferBase && self.FrameBufferLength == other.FrameBufferLength
    }
}
impl ::core::cmp::Eq for VIDEO_MEMORY_INFORMATION {}
impl ::core::fmt::Debug for VIDEO_MEMORY_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VIDEO_MEMORY_INFORMATION").field("VideoRamBase", &self.VideoRamBase).field("VideoRamLength", &self.VideoRamLength).field("FrameBufferBase", &self.FrameBufferBase).field("FrameBufferLength", &self.FrameBufferLength).finish()
    }
}
impl ::core::default::Default for VIDEO_MODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VIDEO_MODE {
    fn eq(&self, other: &Self) -> bool {
        self.RequestedMode == other.RequestedMode
    }
}
impl ::core::cmp::Eq for VIDEO_MODE {}
impl ::core::fmt::Debug for VIDEO_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VIDEO_MODE").field("RequestedMode", &self.RequestedMode).finish()
    }
}
impl ::core::default::Default for VIDEO_MODE_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VIDEO_MODE_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length
            && self.ModeIndex == other.ModeIndex
            && self.VisScreenWidth == other.VisScreenWidth
            && self.VisScreenHeight == other.VisScreenHeight
            && self.ScreenStride == other.ScreenStride
            && self.NumberOfPlanes == other.NumberOfPlanes
            && self.BitsPerPlane == other.BitsPerPlane
            && self.Frequency == other.Frequency
            && self.XMillimeter == other.XMillimeter
            && self.YMillimeter == other.YMillimeter
            && self.NumberRedBits == other.NumberRedBits
            && self.NumberGreenBits == other.NumberGreenBits
            && self.NumberBlueBits == other.NumberBlueBits
            && self.RedMask == other.RedMask
            && self.GreenMask == other.GreenMask
            && self.BlueMask == other.BlueMask
            && self.AttributeFlags == other.AttributeFlags
            && self.VideoMemoryBitmapWidth == other.VideoMemoryBitmapWidth
            && self.VideoMemoryBitmapHeight == other.VideoMemoryBitmapHeight
            && self.DriverSpecificAttributeFlags == other.DriverSpecificAttributeFlags
    }
}
impl ::core::cmp::Eq for VIDEO_MODE_INFORMATION {}
impl ::core::fmt::Debug for VIDEO_MODE_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VIDEO_MODE_INFORMATION")
            .field("Length", &self.Length)
            .field("ModeIndex", &self.ModeIndex)
            .field("VisScreenWidth", &self.VisScreenWidth)
            .field("VisScreenHeight", &self.VisScreenHeight)
            .field("ScreenStride", &self.ScreenStride)
            .field("NumberOfPlanes", &self.NumberOfPlanes)
            .field("BitsPerPlane", &self.BitsPerPlane)
            .field("Frequency", &self.Frequency)
            .field("XMillimeter", &self.XMillimeter)
            .field("YMillimeter", &self.YMillimeter)
            .field("NumberRedBits", &self.NumberRedBits)
            .field("NumberGreenBits", &self.NumberGreenBits)
            .field("NumberBlueBits", &self.NumberBlueBits)
            .field("RedMask", &self.RedMask)
            .field("GreenMask", &self.GreenMask)
            .field("BlueMask", &self.BlueMask)
            .field("AttributeFlags", &self.AttributeFlags)
            .field("VideoMemoryBitmapWidth", &self.VideoMemoryBitmapWidth)
            .field("VideoMemoryBitmapHeight", &self.VideoMemoryBitmapHeight)
            .field("DriverSpecificAttributeFlags", &self.DriverSpecificAttributeFlags)
            .finish()
    }
}
impl ::core::default::Default for VIDEO_MONITOR_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VIDEO_MONITOR_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.DescriptorSize == other.DescriptorSize && self.Descriptor == other.Descriptor
    }
}
impl ::core::cmp::Eq for VIDEO_MONITOR_DESCRIPTOR {}
impl ::core::fmt::Debug for VIDEO_MONITOR_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VIDEO_MONITOR_DESCRIPTOR").field("DescriptorSize", &self.DescriptorSize).field("Descriptor", &self.Descriptor).finish()
    }
}
impl ::core::default::Default for VIDEO_NUM_MODES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VIDEO_NUM_MODES {
    fn eq(&self, other: &Self) -> bool {
        self.NumModes == other.NumModes && self.ModeInformationLength == other.ModeInformationLength
    }
}
impl ::core::cmp::Eq for VIDEO_NUM_MODES {}
impl ::core::fmt::Debug for VIDEO_NUM_MODES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VIDEO_NUM_MODES").field("NumModes", &self.NumModes).field("ModeInformationLength", &self.ModeInformationLength).finish()
    }
}
impl ::core::default::Default for VIDEO_PALETTE_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VIDEO_PALETTE_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.NumEntries == other.NumEntries && self.FirstEntry == other.FirstEntry && self.Colors == other.Colors
    }
}
impl ::core::cmp::Eq for VIDEO_PALETTE_DATA {}
impl ::core::fmt::Debug for VIDEO_PALETTE_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VIDEO_PALETTE_DATA").field("NumEntries", &self.NumEntries).field("FirstEntry", &self.FirstEntry).field("Colors", &self.Colors).finish()
    }
}
impl ::core::default::Default for VIDEO_PERFORMANCE_COUNTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VIDEO_PERFORMANCE_COUNTER {
    fn eq(&self, other: &Self) -> bool {
        self.NbOfAllocationEvicted == other.NbOfAllocationEvicted
            && self.NbOfAllocationMarked == other.NbOfAllocationMarked
            && self.NbOfAllocationRestored == other.NbOfAllocationRestored
            && self.KBytesEvicted == other.KBytesEvicted
            && self.KBytesMarked == other.KBytesMarked
            && self.KBytesRestored == other.KBytesRestored
            && self.NbProcessCommited == other.NbProcessCommited
            && self.NbAllocationCommited == other.NbAllocationCommited
            && self.NbAllocationMarked == other.NbAllocationMarked
            && self.KBytesAllocated == other.KBytesAllocated
            && self.KBytesAvailable == other.KBytesAvailable
            && self.KBytesCurMarked == other.KBytesCurMarked
            && self.Reference == other.Reference
            && self.Unreference == other.Unreference
            && self.TrueReference == other.TrueReference
            && self.NbOfPageIn == other.NbOfPageIn
            && self.KBytesPageIn == other.KBytesPageIn
            && self.NbOfPageOut == other.NbOfPageOut
            && self.KBytesPageOut == other.KBytesPageOut
            && self.NbOfRotateOut == other.NbOfRotateOut
            && self.KBytesRotateOut == other.KBytesRotateOut
    }
}
impl ::core::cmp::Eq for VIDEO_PERFORMANCE_COUNTER {}
impl ::core::fmt::Debug for VIDEO_PERFORMANCE_COUNTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VIDEO_PERFORMANCE_COUNTER")
            .field("NbOfAllocationEvicted", &self.NbOfAllocationEvicted)
            .field("NbOfAllocationMarked", &self.NbOfAllocationMarked)
            .field("NbOfAllocationRestored", &self.NbOfAllocationRestored)
            .field("KBytesEvicted", &self.KBytesEvicted)
            .field("KBytesMarked", &self.KBytesMarked)
            .field("KBytesRestored", &self.KBytesRestored)
            .field("NbProcessCommited", &self.NbProcessCommited)
            .field("NbAllocationCommited", &self.NbAllocationCommited)
            .field("NbAllocationMarked", &self.NbAllocationMarked)
            .field("KBytesAllocated", &self.KBytesAllocated)
            .field("KBytesAvailable", &self.KBytesAvailable)
            .field("KBytesCurMarked", &self.KBytesCurMarked)
            .field("Reference", &self.Reference)
            .field("Unreference", &self.Unreference)
            .field("TrueReference", &self.TrueReference)
            .field("NbOfPageIn", &self.NbOfPageIn)
            .field("KBytesPageIn", &self.KBytesPageIn)
            .field("NbOfPageOut", &self.NbOfPageOut)
            .field("KBytesPageOut", &self.KBytesPageOut)
            .field("NbOfRotateOut", &self.NbOfRotateOut)
            .field("KBytesRotateOut", &self.KBytesRotateOut)
            .finish()
    }
}
impl ::core::default::Default for VIDEO_POINTER_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VIDEO_POINTER_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.Width == other.Width && self.Height == other.Height && self.WidthInBytes == other.WidthInBytes && self.Enable == other.Enable && self.Column == other.Column && self.Row == other.Row && self.Pixels == other.Pixels
    }
}
impl ::core::cmp::Eq for VIDEO_POINTER_ATTRIBUTES {}
impl ::core::fmt::Debug for VIDEO_POINTER_ATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VIDEO_POINTER_ATTRIBUTES").field("Flags", &self.Flags).field("Width", &self.Width).field("Height", &self.Height).field("WidthInBytes", &self.WidthInBytes).field("Enable", &self.Enable).field("Column", &self.Column).field("Row", &self.Row).field("Pixels", &self.Pixels).finish()
    }
}
impl ::core::default::Default for VIDEO_POINTER_CAPABILITIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VIDEO_POINTER_CAPABILITIES {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.MaxWidth == other.MaxWidth && self.MaxHeight == other.MaxHeight && self.HWPtrBitmapStart == other.HWPtrBitmapStart && self.HWPtrBitmapEnd == other.HWPtrBitmapEnd
    }
}
impl ::core::cmp::Eq for VIDEO_POINTER_CAPABILITIES {}
impl ::core::fmt::Debug for VIDEO_POINTER_CAPABILITIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VIDEO_POINTER_CAPABILITIES").field("Flags", &self.Flags).field("MaxWidth", &self.MaxWidth).field("MaxHeight", &self.MaxHeight).field("HWPtrBitmapStart", &self.HWPtrBitmapStart).field("HWPtrBitmapEnd", &self.HWPtrBitmapEnd).finish()
    }
}
impl ::core::default::Default for VIDEO_POINTER_POSITION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VIDEO_POINTER_POSITION {
    fn eq(&self, other: &Self) -> bool {
        self.Column == other.Column && self.Row == other.Row
    }
}
impl ::core::cmp::Eq for VIDEO_POINTER_POSITION {}
impl ::core::fmt::Debug for VIDEO_POINTER_POSITION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VIDEO_POINTER_POSITION").field("Column", &self.Column).field("Row", &self.Row).finish()
    }
}
impl ::core::default::Default for VIDEO_POWER_MANAGEMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VIDEO_POWER_MANAGEMENT {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.DPMSVersion == other.DPMSVersion && self.PowerState == other.PowerState
    }
}
impl ::core::cmp::Eq for VIDEO_POWER_MANAGEMENT {}
impl ::core::fmt::Debug for VIDEO_POWER_MANAGEMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VIDEO_POWER_MANAGEMENT").field("Length", &self.Length).field("DPMSVersion", &self.DPMSVersion).field("PowerState", &self.PowerState).finish()
    }
}
impl ::core::default::Default for VIDEO_POWER_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for VIDEO_POWER_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VIDEO_POWER_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for VIDEO_PUBLIC_ACCESS_RANGES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VIDEO_PUBLIC_ACCESS_RANGES {
    fn eq(&self, other: &Self) -> bool {
        self.InIoSpace == other.InIoSpace && self.MappedInIoSpace == other.MappedInIoSpace && self.VirtualAddress == other.VirtualAddress
    }
}
impl ::core::cmp::Eq for VIDEO_PUBLIC_ACCESS_RANGES {}
impl ::core::fmt::Debug for VIDEO_PUBLIC_ACCESS_RANGES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VIDEO_PUBLIC_ACCESS_RANGES").field("InIoSpace", &self.InIoSpace).field("MappedInIoSpace", &self.MappedInIoSpace).field("VirtualAddress", &self.VirtualAddress).finish()
    }
}
impl ::core::default::Default for VIDEO_QUERY_PERFORMANCE_COUNTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VIDEO_QUERY_PERFORMANCE_COUNTER {
    fn eq(&self, other: &Self) -> bool {
        self.BufferSize == other.BufferSize && self.Buffer == other.Buffer
    }
}
impl ::core::cmp::Eq for VIDEO_QUERY_PERFORMANCE_COUNTER {}
impl ::core::fmt::Debug for VIDEO_QUERY_PERFORMANCE_COUNTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VIDEO_QUERY_PERFORMANCE_COUNTER").field("BufferSize", &self.BufferSize).field("Buffer", &self.Buffer).finish()
    }
}
impl ::core::default::Default for VIDEO_REGISTER_VDM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VIDEO_REGISTER_VDM {
    fn eq(&self, other: &Self) -> bool {
        self.MinimumStateSize == other.MinimumStateSize
    }
}
impl ::core::cmp::Eq for VIDEO_REGISTER_VDM {}
impl ::core::fmt::Debug for VIDEO_REGISTER_VDM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VIDEO_REGISTER_VDM").field("MinimumStateSize", &self.MinimumStateSize).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for VIDEO_SHARE_MEMORY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for VIDEO_SHARE_MEMORY {
    fn eq(&self, other: &Self) -> bool {
        self.ProcessHandle == other.ProcessHandle && self.ViewOffset == other.ViewOffset && self.ViewSize == other.ViewSize && self.RequestedVirtualAddress == other.RequestedVirtualAddress
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for VIDEO_SHARE_MEMORY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for VIDEO_SHARE_MEMORY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VIDEO_SHARE_MEMORY").field("ProcessHandle", &self.ProcessHandle).field("ViewOffset", &self.ViewOffset).field("ViewSize", &self.ViewSize).field("RequestedVirtualAddress", &self.RequestedVirtualAddress).finish()
    }
}
impl ::core::default::Default for VIDEO_SHARE_MEMORY_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VIDEO_SHARE_MEMORY_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.SharedViewOffset == other.SharedViewOffset && self.SharedViewSize == other.SharedViewSize && self.VirtualAddress == other.VirtualAddress
    }
}
impl ::core::cmp::Eq for VIDEO_SHARE_MEMORY_INFORMATION {}
impl ::core::fmt::Debug for VIDEO_SHARE_MEMORY_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VIDEO_SHARE_MEMORY_INFORMATION").field("SharedViewOffset", &self.SharedViewOffset).field("SharedViewSize", &self.SharedViewSize).field("VirtualAddress", &self.VirtualAddress).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for VIDEO_VDM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for VIDEO_VDM {
    fn eq(&self, other: &Self) -> bool {
        self.ProcessHandle == other.ProcessHandle
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for VIDEO_VDM {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for VIDEO_VDM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VIDEO_VDM").field("ProcessHandle", &self.ProcessHandle).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for VIDEO_WIN32K_CALLBACKS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for VIDEO_WIN32K_CALLBACKS_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for VIDEO_WIN32K_CALLBACKS_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.CalloutType == other.CalloutType && self.PhysDisp == other.PhysDisp && self.Param == other.Param && self.Status == other.Status && self.LockUserSession == other.LockUserSession && self.IsPostDevice == other.IsPostDevice && self.SurpriseRemoval == other.SurpriseRemoval && self.WaitForQueueReady == other.WaitForQueueReady
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for VIDEO_WIN32K_CALLBACKS_PARAMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for VIDEO_WIN32K_CALLBACKS_PARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VIDEO_WIN32K_CALLBACKS_PARAMS").field("CalloutType", &self.CalloutType).field("PhysDisp", &self.PhysDisp).field("Param", &self.Param).field("Status", &self.Status).field("LockUserSession", &self.LockUserSession).field("IsPostDevice", &self.IsPostDevice).field("SurpriseRemoval", &self.SurpriseRemoval).field("WaitForQueueReady", &self.WaitForQueueReady).finish()
    }
}
impl ::core::default::Default for VIDEO_WIN32K_CALLBACKS_PARAMS_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for VIDEO_WIN32K_CALLBACKS_PARAMS_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VIDEO_WIN32K_CALLBACKS_PARAMS_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WCRUN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WCRUN {
    fn eq(&self, other: &Self) -> bool {
        self.wcLow == other.wcLow && self.cGlyphs == other.cGlyphs && self.phg == other.phg
    }
}
impl ::core::cmp::Eq for WCRUN {}
impl ::core::fmt::Debug for WCRUN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WCRUN").field("wcLow", &self.wcLow).field("cGlyphs", &self.cGlyphs).field("phg", &self.phg).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WNDOBJ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WNDOBJ {
    fn eq(&self, other: &Self) -> bool {
        self.coClient == other.coClient && self.pvConsumer == other.pvConsumer && self.rclClient == other.rclClient && self.psoOwner == other.psoOwner
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WNDOBJ {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WNDOBJ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WNDOBJ").field("coClient", &self.coClient).field("pvConsumer", &self.pvConsumer).field("rclClient", &self.rclClient).field("psoOwner", &self.psoOwner).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for XFORML {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for XFORML {
    fn eq(&self, other: &Self) -> bool {
        self.eM11 == other.eM11 && self.eM12 == other.eM12 && self.eM21 == other.eM21 && self.eM22 == other.eM22 && self.eDx == other.eDx && self.eDy == other.eDy
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for XFORML {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::fmt::Debug for XFORML {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("XFORML").field("eM11", &self.eM11).field("eM12", &self.eM12).field("eM21", &self.eM21).field("eM22", &self.eM22).field("eDx", &self.eDx).field("eDy", &self.eDy).finish()
    }
}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for XFORML {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::PartialEq for XFORML {
    fn eq(&self, other: &Self) -> bool {
        self.eM11 == other.eM11 && self.eM12 == other.eM12 && self.eM21 == other.eM21 && self.eM22 == other.eM22 && self.eDx == other.eDx && self.eDy == other.eDy
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::Eq for XFORML {}
#[cfg(target_arch = "x86")]
impl ::core::fmt::Debug for XFORML {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("XFORML").field("eM11", &self.eM11).field("eM12", &self.eM12).field("eM21", &self.eM21).field("eM22", &self.eM22).field("eDx", &self.eDx).field("eDy", &self.eDy).finish()
    }
}
impl ::core::default::Default for XFORMOBJ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for XFORMOBJ {
    fn eq(&self, other: &Self) -> bool {
        self.ulReserved == other.ulReserved
    }
}
impl ::core::cmp::Eq for XFORMOBJ {}
impl ::core::fmt::Debug for XFORMOBJ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("XFORMOBJ").field("ulReserved", &self.ulReserved).finish()
    }
}
impl ::core::default::Default for XLATEOBJ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for XLATEOBJ {
    fn eq(&self, other: &Self) -> bool {
        self.iUniq == other.iUniq && self.flXlate == other.flXlate && self.iSrcType == other.iSrcType && self.iDstType == other.iDstType && self.cEntries == other.cEntries && self.pulXlate == other.pulXlate
    }
}
impl ::core::cmp::Eq for XLATEOBJ {}
impl ::core::fmt::Debug for XLATEOBJ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("XLATEOBJ").field("iUniq", &self.iUniq).field("flXlate", &self.flXlate).field("iSrcType", &self.iSrcType).field("iDstType", &self.iDstType).field("cEntries", &self.cEntries).field("pulXlate", &self.pulXlate).finish()
    }
}
