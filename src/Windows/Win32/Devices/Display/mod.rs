#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct AR_STATE(pub i32);
pub const AR_ENABLED: AR_STATE = AR_STATE(0i32);
pub const AR_DISABLED: AR_STATE = AR_STATE(1i32);
pub const AR_SUPPRESSED: AR_STATE = AR_STATE(2i32);
pub const AR_REMOTESESSION: AR_STATE = AR_STATE(4i32);
pub const AR_MULTIMON: AR_STATE = AR_STATE(8i32);
pub const AR_NOSENSOR: AR_STATE = AR_STATE(16i32);
pub const AR_NOT_SUPPORTED: AR_STATE = AR_STATE(32i32);
pub const AR_DOCKED: AR_STATE = AR_STATE(64i32);
pub const AR_LAPTOP: AR_STATE = AR_STATE(128i32);
impl ::std::convert::From<i32> for AR_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AR_STATE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct Adapter {
    pub AdapterName: [u16; 128],
    pub numSources: i32,
    pub sources: [Sources; 1],
}
impl Adapter {}
impl ::std::default::Default for Adapter {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for Adapter {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("Adapter").field("AdapterName", &self.AdapterName).field("numSources", &self.numSources).field("sources", &self.sources).finish()
    }
}
impl ::std::cmp::PartialEq for Adapter {
    fn eq(&self, other: &Self) -> bool {
        self.AdapterName == other.AdapterName && self.numSources == other.numSources && self.sources == other.sources
    }
}
impl ::std::cmp::Eq for Adapter {}
unsafe impl ::windows::runtime::Abi for Adapter {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct Adapters {
    pub numAdapters: i32,
    pub adapter: [Adapter; 1],
}
impl Adapters {}
impl ::std::default::Default for Adapters {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for Adapters {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("Adapters").field("numAdapters", &self.numAdapters).field("adapter", &self.adapter).finish()
    }
}
impl ::std::cmp::PartialEq for Adapters {
    fn eq(&self, other: &Self) -> bool {
        self.numAdapters == other.numAdapters && self.adapter == other.adapter
    }
}
impl ::std::cmp::Eq for Adapters {}
unsafe impl ::windows::runtime::Abi for Adapters {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct BACKLIGHT_OPTIMIZATION_LEVEL(pub i32);
pub const BacklightOptimizationDisable: BACKLIGHT_OPTIMIZATION_LEVEL = BACKLIGHT_OPTIMIZATION_LEVEL(0i32);
pub const BacklightOptimizationDesktop: BACKLIGHT_OPTIMIZATION_LEVEL = BACKLIGHT_OPTIMIZATION_LEVEL(1i32);
pub const BacklightOptimizationDynamic: BACKLIGHT_OPTIMIZATION_LEVEL = BACKLIGHT_OPTIMIZATION_LEVEL(2i32);
pub const BacklightOptimizationDimmed: BACKLIGHT_OPTIMIZATION_LEVEL = BACKLIGHT_OPTIMIZATION_LEVEL(3i32);
pub const BacklightOptimizationEDR: BACKLIGHT_OPTIMIZATION_LEVEL = BACKLIGHT_OPTIMIZATION_LEVEL(4i32);
impl ::std::convert::From<i32> for BACKLIGHT_OPTIMIZATION_LEVEL {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for BACKLIGHT_OPTIMIZATION_LEVEL {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct BACKLIGHT_REDUCTION_GAMMA_RAMP {
    pub R: [u16; 256],
    pub G: [u16; 256],
    pub B: [u16; 256],
}
impl BACKLIGHT_REDUCTION_GAMMA_RAMP {}
impl ::std::default::Default for BACKLIGHT_REDUCTION_GAMMA_RAMP {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for BACKLIGHT_REDUCTION_GAMMA_RAMP {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("BACKLIGHT_REDUCTION_GAMMA_RAMP").field("R", &self.R).field("G", &self.G).field("B", &self.B).finish()
    }
}
impl ::std::cmp::PartialEq for BACKLIGHT_REDUCTION_GAMMA_RAMP {
    fn eq(&self, other: &Self) -> bool {
        self.R == other.R && self.G == other.G && self.B == other.B
    }
}
impl ::std::cmp::Eq for BACKLIGHT_REDUCTION_GAMMA_RAMP {}
unsafe impl ::windows::runtime::Abi for BACKLIGHT_REDUCTION_GAMMA_RAMP {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct BANK_POSITION {
    pub ReadBankPosition: u32,
    pub WriteBankPosition: u32,
}
impl BANK_POSITION {}
impl ::std::default::Default for BANK_POSITION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for BANK_POSITION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("BANK_POSITION").field("ReadBankPosition", &self.ReadBankPosition).field("WriteBankPosition", &self.WriteBankPosition).finish()
    }
}
impl ::std::cmp::PartialEq for BANK_POSITION {
    fn eq(&self, other: &Self) -> bool {
        self.ReadBankPosition == other.ReadBankPosition && self.WriteBankPosition == other.WriteBankPosition
    }
}
impl ::std::cmp::Eq for BANK_POSITION {}
unsafe impl ::windows::runtime::Abi for BANK_POSITION {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const BITMAP_ARRAY_BYTE: u32 = 3u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const BITMAP_BITS_BYTE_ALIGN: u32 = 8u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const BITMAP_BITS_PIXEL: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const BITMAP_BITS_WORD_ALIGN: u32 = 16u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const BITMAP_PLANES: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct BRIGHTNESS_INTERFACE_VERSION(pub i32);
pub const BRIGHTNESS_INTERFACE_VERSION_1: BRIGHTNESS_INTERFACE_VERSION = BRIGHTNESS_INTERFACE_VERSION(1i32);
pub const BRIGHTNESS_INTERFACE_VERSION_2: BRIGHTNESS_INTERFACE_VERSION = BRIGHTNESS_INTERFACE_VERSION(2i32);
pub const BRIGHTNESS_INTERFACE_VERSION_3: BRIGHTNESS_INTERFACE_VERSION = BRIGHTNESS_INTERFACE_VERSION(3i32);
impl ::std::convert::From<i32> for BRIGHTNESS_INTERFACE_VERSION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for BRIGHTNESS_INTERFACE_VERSION {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct BRIGHTNESS_LEVEL {
    pub Count: u8,
    pub Level: [u8; 103],
}
impl BRIGHTNESS_LEVEL {}
impl ::std::default::Default for BRIGHTNESS_LEVEL {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for BRIGHTNESS_LEVEL {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("BRIGHTNESS_LEVEL").field("Count", &self.Count).field("Level", &self.Level).finish()
    }
}
impl ::std::cmp::PartialEq for BRIGHTNESS_LEVEL {
    fn eq(&self, other: &Self) -> bool {
        self.Count == other.Count && self.Level == other.Level
    }
}
impl ::std::cmp::Eq for BRIGHTNESS_LEVEL {}
unsafe impl ::windows::runtime::Abi for BRIGHTNESS_LEVEL {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const BRIGHTNESS_MAX_LEVEL_COUNT: u32 = 103u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const BRIGHTNESS_MAX_NIT_RANGE_COUNT: u32 = 16u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct BRIGHTNESS_NIT_RANGE {
    pub MinLevelInMillinit: u32,
    pub MaxLevelInMillinit: u32,
    pub StepSizeInMillinit: u32,
}
impl BRIGHTNESS_NIT_RANGE {}
impl ::std::default::Default for BRIGHTNESS_NIT_RANGE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for BRIGHTNESS_NIT_RANGE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("BRIGHTNESS_NIT_RANGE").field("MinLevelInMillinit", &self.MinLevelInMillinit).field("MaxLevelInMillinit", &self.MaxLevelInMillinit).field("StepSizeInMillinit", &self.StepSizeInMillinit).finish()
    }
}
impl ::std::cmp::PartialEq for BRIGHTNESS_NIT_RANGE {
    fn eq(&self, other: &Self) -> bool {
        self.MinLevelInMillinit == other.MinLevelInMillinit && self.MaxLevelInMillinit == other.MaxLevelInMillinit && self.StepSizeInMillinit == other.StepSizeInMillinit
    }
}
impl ::std::cmp::Eq for BRIGHTNESS_NIT_RANGE {}
unsafe impl ::windows::runtime::Abi for BRIGHTNESS_NIT_RANGE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct BRIGHTNESS_NIT_RANGES {
    pub NormalRangeCount: u32,
    pub RangeCount: u32,
    pub PreferredMaximumBrightness: u32,
    pub SupportedRanges: [BRIGHTNESS_NIT_RANGE; 16],
}
impl BRIGHTNESS_NIT_RANGES {}
impl ::std::default::Default for BRIGHTNESS_NIT_RANGES {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for BRIGHTNESS_NIT_RANGES {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("BRIGHTNESS_NIT_RANGES").field("NormalRangeCount", &self.NormalRangeCount).field("RangeCount", &self.RangeCount).field("PreferredMaximumBrightness", &self.PreferredMaximumBrightness).field("SupportedRanges", &self.SupportedRanges).finish()
    }
}
impl ::std::cmp::PartialEq for BRIGHTNESS_NIT_RANGES {
    fn eq(&self, other: &Self) -> bool {
        self.NormalRangeCount == other.NormalRangeCount && self.RangeCount == other.RangeCount && self.PreferredMaximumBrightness == other.PreferredMaximumBrightness && self.SupportedRanges == other.SupportedRanges
    }
}
impl ::std::cmp::Eq for BRIGHTNESS_NIT_RANGES {}
unsafe impl ::windows::runtime::Abi for BRIGHTNESS_NIT_RANGES {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct BlackScreenDiagnosticsCalloutParam(pub i32);
pub const BlackScreenDiagnosticsData: BlackScreenDiagnosticsCalloutParam = BlackScreenDiagnosticsCalloutParam(1i32);
pub const BlackScreenDisplayRecovery: BlackScreenDiagnosticsCalloutParam = BlackScreenDiagnosticsCalloutParam(2i32);
impl ::std::convert::From<i32> for BlackScreenDiagnosticsCalloutParam {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for BlackScreenDiagnosticsCalloutParam {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Console"))]
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`, `Win32_System_Console`*"]
pub struct CHAR_IMAGE_INFO {
    pub CharInfo: super::super::System::Console::CHAR_INFO,
    pub FontImageInfo: FONT_IMAGE_INFO,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Console"))]
impl CHAR_IMAGE_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Console"))]
impl ::std::default::Default for CHAR_IMAGE_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Console"))]
impl ::std::cmp::PartialEq for CHAR_IMAGE_INFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Console"))]
impl ::std::cmp::Eq for CHAR_IMAGE_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Console"))]
unsafe impl ::windows::runtime::Abi for CHAR_IMAGE_INFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const CHAR_TYPE_LEADING: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const CHAR_TYPE_SBCS: u32 = 0u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const CHAR_TYPE_TRAILING: u32 = 3u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct CHROMATICITY_COORDINATE {
    pub x: f32,
    pub y: f32,
}
impl CHROMATICITY_COORDINATE {}
impl ::std::default::Default for CHROMATICITY_COORDINATE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CHROMATICITY_COORDINATE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CHROMATICITY_COORDINATE").field("x", &self.x).field("y", &self.y).finish()
    }
}
impl ::std::cmp::PartialEq for CHROMATICITY_COORDINATE {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl ::std::cmp::Eq for CHROMATICITY_COORDINATE {}
unsafe impl ::windows::runtime::Abi for CHROMATICITY_COORDINATE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct COLORSPACE_TRANSFORM {
    pub Type: COLORSPACE_TRANSFORM_TYPE,
    pub Data: COLORSPACE_TRANSFORM_0,
}
impl COLORSPACE_TRANSFORM {}
impl ::std::default::Default for COLORSPACE_TRANSFORM {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for COLORSPACE_TRANSFORM {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for COLORSPACE_TRANSFORM {}
unsafe impl ::windows::runtime::Abi for COLORSPACE_TRANSFORM {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub union COLORSPACE_TRANSFORM_0 {
    pub Rgb256x3x16: GAMMA_RAMP_RGB256x3x16,
    pub Dxgi1: GAMMA_RAMP_DXGI_1,
    pub T3x4: COLORSPACE_TRANSFORM_3x4,
    pub MatrixV2: COLORSPACE_TRANSFORM_MATRIX_V2,
}
impl COLORSPACE_TRANSFORM_0 {}
impl ::std::default::Default for COLORSPACE_TRANSFORM_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for COLORSPACE_TRANSFORM_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for COLORSPACE_TRANSFORM_0 {}
unsafe impl ::windows::runtime::Abi for COLORSPACE_TRANSFORM_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct COLORSPACE_TRANSFORM_1DLUT_CAP {
    pub NumberOfLUTEntries: u32,
    pub DataCap: COLORSPACE_TRANSFORM_DATA_CAP,
}
impl COLORSPACE_TRANSFORM_1DLUT_CAP {}
impl ::std::default::Default for COLORSPACE_TRANSFORM_1DLUT_CAP {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for COLORSPACE_TRANSFORM_1DLUT_CAP {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for COLORSPACE_TRANSFORM_1DLUT_CAP {}
unsafe impl ::windows::runtime::Abi for COLORSPACE_TRANSFORM_1DLUT_CAP {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct COLORSPACE_TRANSFORM_3x4 {
    pub ColorMatrix3x4: [f32; 12],
    pub ScalarMultiplier: f32,
    pub LookupTable1D: [GAMMA_RAMP_RGB; 4096],
}
impl COLORSPACE_TRANSFORM_3x4 {}
impl ::std::default::Default for COLORSPACE_TRANSFORM_3x4 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for COLORSPACE_TRANSFORM_3x4 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("COLORSPACE_TRANSFORM_3x4").field("ColorMatrix3x4", &self.ColorMatrix3x4).field("ScalarMultiplier", &self.ScalarMultiplier).field("LookupTable1D", &self.LookupTable1D).finish()
    }
}
impl ::std::cmp::PartialEq for COLORSPACE_TRANSFORM_3x4 {
    fn eq(&self, other: &Self) -> bool {
        self.ColorMatrix3x4 == other.ColorMatrix3x4 && self.ScalarMultiplier == other.ScalarMultiplier && self.LookupTable1D == other.LookupTable1D
    }
}
impl ::std::cmp::Eq for COLORSPACE_TRANSFORM_3x4 {}
unsafe impl ::windows::runtime::Abi for COLORSPACE_TRANSFORM_3x4 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct COLORSPACE_TRANSFORM_DATA_CAP {
    pub DataType: COLORSPACE_TRANSFORM_DATA_TYPE,
    pub Anonymous: COLORSPACE_TRANSFORM_DATA_CAP_0,
    pub NumericRangeMin: f32,
    pub NumericRangeMax: f32,
}
impl COLORSPACE_TRANSFORM_DATA_CAP {}
impl ::std::default::Default for COLORSPACE_TRANSFORM_DATA_CAP {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for COLORSPACE_TRANSFORM_DATA_CAP {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for COLORSPACE_TRANSFORM_DATA_CAP {}
unsafe impl ::windows::runtime::Abi for COLORSPACE_TRANSFORM_DATA_CAP {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub union COLORSPACE_TRANSFORM_DATA_CAP_0 {
    pub Anonymous1: COLORSPACE_TRANSFORM_DATA_CAP_0_0,
    pub Anonymous2: COLORSPACE_TRANSFORM_DATA_CAP_0_1,
    pub Value: u32,
}
impl COLORSPACE_TRANSFORM_DATA_CAP_0 {}
impl ::std::default::Default for COLORSPACE_TRANSFORM_DATA_CAP_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for COLORSPACE_TRANSFORM_DATA_CAP_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for COLORSPACE_TRANSFORM_DATA_CAP_0 {}
unsafe impl ::windows::runtime::Abi for COLORSPACE_TRANSFORM_DATA_CAP_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct COLORSPACE_TRANSFORM_DATA_CAP_0_0 {
    pub _bitfield: u32,
}
impl COLORSPACE_TRANSFORM_DATA_CAP_0_0 {}
impl ::std::default::Default for COLORSPACE_TRANSFORM_DATA_CAP_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for COLORSPACE_TRANSFORM_DATA_CAP_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous1_e__Struct").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::std::cmp::PartialEq for COLORSPACE_TRANSFORM_DATA_CAP_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for COLORSPACE_TRANSFORM_DATA_CAP_0_0 {}
unsafe impl ::windows::runtime::Abi for COLORSPACE_TRANSFORM_DATA_CAP_0_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct COLORSPACE_TRANSFORM_DATA_CAP_0_1 {
    pub _bitfield: u32,
}
impl COLORSPACE_TRANSFORM_DATA_CAP_0_1 {}
impl ::std::default::Default for COLORSPACE_TRANSFORM_DATA_CAP_0_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for COLORSPACE_TRANSFORM_DATA_CAP_0_1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous2_e__Struct").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::std::cmp::PartialEq for COLORSPACE_TRANSFORM_DATA_CAP_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for COLORSPACE_TRANSFORM_DATA_CAP_0_1 {}
unsafe impl ::windows::runtime::Abi for COLORSPACE_TRANSFORM_DATA_CAP_0_1 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct COLORSPACE_TRANSFORM_DATA_TYPE(pub i32);
pub const COLORSPACE_TRANSFORM_DATA_TYPE_FIXED_POINT: COLORSPACE_TRANSFORM_DATA_TYPE = COLORSPACE_TRANSFORM_DATA_TYPE(0i32);
pub const COLORSPACE_TRANSFORM_DATA_TYPE_FLOAT: COLORSPACE_TRANSFORM_DATA_TYPE = COLORSPACE_TRANSFORM_DATA_TYPE(1i32);
impl ::std::convert::From<i32> for COLORSPACE_TRANSFORM_DATA_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for COLORSPACE_TRANSFORM_DATA_TYPE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct COLORSPACE_TRANSFORM_MATRIX_CAP {
    pub Anonymous: COLORSPACE_TRANSFORM_MATRIX_CAP_0,
    pub DataCap: COLORSPACE_TRANSFORM_DATA_CAP,
}
impl COLORSPACE_TRANSFORM_MATRIX_CAP {}
impl ::std::default::Default for COLORSPACE_TRANSFORM_MATRIX_CAP {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for COLORSPACE_TRANSFORM_MATRIX_CAP {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for COLORSPACE_TRANSFORM_MATRIX_CAP {}
unsafe impl ::windows::runtime::Abi for COLORSPACE_TRANSFORM_MATRIX_CAP {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub union COLORSPACE_TRANSFORM_MATRIX_CAP_0 {
    pub Anonymous: COLORSPACE_TRANSFORM_MATRIX_CAP_0_0,
    pub Value: u32,
}
impl COLORSPACE_TRANSFORM_MATRIX_CAP_0 {}
impl ::std::default::Default for COLORSPACE_TRANSFORM_MATRIX_CAP_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for COLORSPACE_TRANSFORM_MATRIX_CAP_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for COLORSPACE_TRANSFORM_MATRIX_CAP_0 {}
unsafe impl ::windows::runtime::Abi for COLORSPACE_TRANSFORM_MATRIX_CAP_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct COLORSPACE_TRANSFORM_MATRIX_CAP_0_0 {
    pub _bitfield: u32,
}
impl COLORSPACE_TRANSFORM_MATRIX_CAP_0_0 {}
impl ::std::default::Default for COLORSPACE_TRANSFORM_MATRIX_CAP_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for COLORSPACE_TRANSFORM_MATRIX_CAP_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::std::cmp::PartialEq for COLORSPACE_TRANSFORM_MATRIX_CAP_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for COLORSPACE_TRANSFORM_MATRIX_CAP_0_0 {}
unsafe impl ::windows::runtime::Abi for COLORSPACE_TRANSFORM_MATRIX_CAP_0_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct COLORSPACE_TRANSFORM_MATRIX_V2 {
    pub StageControlLookupTable1DDegamma: COLORSPACE_TRANSFORM_STAGE_CONTROL,
    pub LookupTable1DDegamma: [GAMMA_RAMP_RGB; 4096],
    pub StageControlColorMatrix3x3: COLORSPACE_TRANSFORM_STAGE_CONTROL,
    pub ColorMatrix3x3: [f32; 9],
    pub StageControlLookupTable1DRegamma: COLORSPACE_TRANSFORM_STAGE_CONTROL,
    pub LookupTable1DRegamma: [GAMMA_RAMP_RGB; 4096],
}
impl COLORSPACE_TRANSFORM_MATRIX_V2 {}
impl ::std::default::Default for COLORSPACE_TRANSFORM_MATRIX_V2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for COLORSPACE_TRANSFORM_MATRIX_V2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("COLORSPACE_TRANSFORM_MATRIX_V2")
            .field("StageControlLookupTable1DDegamma", &self.StageControlLookupTable1DDegamma)
            .field("LookupTable1DDegamma", &self.LookupTable1DDegamma)
            .field("StageControlColorMatrix3x3", &self.StageControlColorMatrix3x3)
            .field("ColorMatrix3x3", &self.ColorMatrix3x3)
            .field("StageControlLookupTable1DRegamma", &self.StageControlLookupTable1DRegamma)
            .field("LookupTable1DRegamma", &self.LookupTable1DRegamma)
            .finish()
    }
}
impl ::std::cmp::PartialEq for COLORSPACE_TRANSFORM_MATRIX_V2 {
    fn eq(&self, other: &Self) -> bool {
        self.StageControlLookupTable1DDegamma == other.StageControlLookupTable1DDegamma && self.LookupTable1DDegamma == other.LookupTable1DDegamma && self.StageControlColorMatrix3x3 == other.StageControlColorMatrix3x3 && self.ColorMatrix3x3 == other.ColorMatrix3x3 && self.StageControlLookupTable1DRegamma == other.StageControlLookupTable1DRegamma && self.LookupTable1DRegamma == other.LookupTable1DRegamma
    }
}
impl ::std::cmp::Eq for COLORSPACE_TRANSFORM_MATRIX_V2 {}
unsafe impl ::windows::runtime::Abi for COLORSPACE_TRANSFORM_MATRIX_V2 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct COLORSPACE_TRANSFORM_SET_INPUT {
    pub OutputWireColorSpaceExpected: OUTPUT_WIRE_COLOR_SPACE_TYPE,
    pub OutputWireFormatExpected: OUTPUT_WIRE_FORMAT,
    pub ColorSpaceTransform: COLORSPACE_TRANSFORM,
}
impl COLORSPACE_TRANSFORM_SET_INPUT {}
impl ::std::default::Default for COLORSPACE_TRANSFORM_SET_INPUT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for COLORSPACE_TRANSFORM_SET_INPUT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for COLORSPACE_TRANSFORM_SET_INPUT {}
unsafe impl ::windows::runtime::Abi for COLORSPACE_TRANSFORM_SET_INPUT {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct COLORSPACE_TRANSFORM_STAGE_CONTROL(pub i32);
pub const ColorSpaceTransformStageControl_No_Change: COLORSPACE_TRANSFORM_STAGE_CONTROL = COLORSPACE_TRANSFORM_STAGE_CONTROL(0i32);
pub const ColorSpaceTransformStageControl_Enable: COLORSPACE_TRANSFORM_STAGE_CONTROL = COLORSPACE_TRANSFORM_STAGE_CONTROL(1i32);
pub const ColorSpaceTransformStageControl_Bypass: COLORSPACE_TRANSFORM_STAGE_CONTROL = COLORSPACE_TRANSFORM_STAGE_CONTROL(2i32);
impl ::std::convert::From<i32> for COLORSPACE_TRANSFORM_STAGE_CONTROL {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for COLORSPACE_TRANSFORM_STAGE_CONTROL {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct COLORSPACE_TRANSFORM_TARGET_CAPS {
    pub Version: COLORSPACE_TRANSFORM_TARGET_CAPS_VERSION,
    pub LookupTable1DDegammaCap: COLORSPACE_TRANSFORM_1DLUT_CAP,
    pub ColorMatrix3x3Cap: COLORSPACE_TRANSFORM_MATRIX_CAP,
    pub LookupTable1DRegammaCap: COLORSPACE_TRANSFORM_1DLUT_CAP,
}
impl COLORSPACE_TRANSFORM_TARGET_CAPS {}
impl ::std::default::Default for COLORSPACE_TRANSFORM_TARGET_CAPS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for COLORSPACE_TRANSFORM_TARGET_CAPS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for COLORSPACE_TRANSFORM_TARGET_CAPS {}
unsafe impl ::windows::runtime::Abi for COLORSPACE_TRANSFORM_TARGET_CAPS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct COLORSPACE_TRANSFORM_TARGET_CAPS_VERSION(pub i32);
pub const COLORSPACE_TRANSFORM_VERSION_DEFAULT: COLORSPACE_TRANSFORM_TARGET_CAPS_VERSION = COLORSPACE_TRANSFORM_TARGET_CAPS_VERSION(0i32);
pub const COLORSPACE_TRANSFORM_VERSION_1: COLORSPACE_TRANSFORM_TARGET_CAPS_VERSION = COLORSPACE_TRANSFORM_TARGET_CAPS_VERSION(1i32);
pub const COLORSPACE_TRANSFORM_VERSION_NOT_SUPPORTED: COLORSPACE_TRANSFORM_TARGET_CAPS_VERSION = COLORSPACE_TRANSFORM_TARGET_CAPS_VERSION(0i32);
impl ::std::convert::From<i32> for COLORSPACE_TRANSFORM_TARGET_CAPS_VERSION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for COLORSPACE_TRANSFORM_TARGET_CAPS_VERSION {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct COLORSPACE_TRANSFORM_TYPE(pub i32);
pub const COLORSPACE_TRANSFORM_TYPE_UNINITIALIZED: COLORSPACE_TRANSFORM_TYPE = COLORSPACE_TRANSFORM_TYPE(0i32);
pub const COLORSPACE_TRANSFORM_TYPE_DEFAULT: COLORSPACE_TRANSFORM_TYPE = COLORSPACE_TRANSFORM_TYPE(1i32);
pub const COLORSPACE_TRANSFORM_TYPE_RGB256x3x16: COLORSPACE_TRANSFORM_TYPE = COLORSPACE_TRANSFORM_TYPE(2i32);
pub const COLORSPACE_TRANSFORM_TYPE_DXGI_1: COLORSPACE_TRANSFORM_TYPE = COLORSPACE_TRANSFORM_TYPE(3i32);
pub const COLORSPACE_TRANSFORM_TYPE_MATRIX_3x4: COLORSPACE_TRANSFORM_TYPE = COLORSPACE_TRANSFORM_TYPE(4i32);
pub const COLORSPACE_TRANSFORM_TYPE_MATRIX_V2: COLORSPACE_TRANSFORM_TYPE = COLORSPACE_TRANSFORM_TYPE(5i32);
impl ::std::convert::From<i32> for COLORSPACE_TRANSFORM_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for COLORSPACE_TRANSFORM_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CapabilitiesRequestAndCapabilitiesReply<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hmonitor: Param0, pszasciicapabilitiesstring: super::super::Foundation::PSTR, dwcapabilitiesstringlengthincharacters: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CapabilitiesRequestAndCapabilitiesReply(hmonitor: super::super::Foundation::HANDLE, pszasciicapabilitiesstring: super::super::Foundation::PSTR, dwcapabilitiesstringlengthincharacters: u32) -> i32;
        }
        ::std::mem::transmute(CapabilitiesRequestAndCapabilitiesReply(hmonitor.into_param().abi(), ::std::mem::transmute(pszasciicapabilitiesstring), ::std::mem::transmute(dwcapabilitiesstringlengthincharacters)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Graphics_DirectDraw")]
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Graphics_DirectDraw`*"]
pub struct DD_GETHEAPALIGNMENTDATA {
    pub dwInstance: usize,
    pub dwHeap: u32,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub GetHeapAlignment: *mut ::std::ffi::c_void,
    pub Alignment: HEAPALIGNMENT,
}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl DD_GETHEAPALIGNMENTDATA {}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl ::std::default::Default for DD_GETHEAPALIGNMENTDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl ::std::cmp::PartialEq for DD_GETHEAPALIGNMENTDATA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl ::std::cmp::Eq for DD_GETHEAPALIGNMENTDATA {}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
unsafe impl ::windows::runtime::Abi for DD_GETHEAPALIGNMENTDATA {
    type Abi = Self;
}
#[cfg(feature = "Win32_System_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_System_PropertiesSystem`*"]
pub const DEVPKEY_Device_ActivityId: super::super::System::PropertiesSystem::PROPERTYKEY = super::super::System::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3305783056, 43612, 16967, [184, 48, 214, 166, 248, 234, 163, 16]),
    pid: 4u32,
};
#[cfg(feature = "Win32_System_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_System_PropertiesSystem`*"]
pub const DEVPKEY_Device_AdapterLuid: super::super::System::PropertiesSystem::PROPERTYKEY = super::super::System::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3305783056, 43612, 16967, [184, 48, 214, 166, 248, 234, 163, 16]),
    pid: 3u32,
};
#[cfg(feature = "Win32_System_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_System_PropertiesSystem`*"]
pub const DEVPKEY_Device_TerminalLuid: super::super::System::PropertiesSystem::PROPERTYKEY = super::super::System::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3305783056, 43612, 16967, [184, 48, 214, 166, 248, 234, 163, 16]),
    pid: 2u32,
};
#[cfg(feature = "Win32_System_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_System_PropertiesSystem`*"]
pub const DEVPKEY_IndirectDisplay: super::super::System::PropertiesSystem::PROPERTYKEY = super::super::System::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3305783056, 43612, 16967, [184, 48, 214, 166, 248, 234, 163, 16]),
    pid: 1u32,
};
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const DISPLAYPOLICY_AC: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const DISPLAYPOLICY_DC: u32 = 2u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct DISPLAY_BRIGHTNESS {
    pub ucDisplayPolicy: u8,
    pub ucACBrightness: u8,
    pub ucDCBrightness: u8,
}
impl DISPLAY_BRIGHTNESS {}
impl ::std::default::Default for DISPLAY_BRIGHTNESS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DISPLAY_BRIGHTNESS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DISPLAY_BRIGHTNESS").field("ucDisplayPolicy", &self.ucDisplayPolicy).field("ucACBrightness", &self.ucACBrightness).field("ucDCBrightness", &self.ucDCBrightness).finish()
    }
}
impl ::std::cmp::PartialEq for DISPLAY_BRIGHTNESS {
    fn eq(&self, other: &Self) -> bool {
        self.ucDisplayPolicy == other.ucDisplayPolicy && self.ucACBrightness == other.ucACBrightness && self.ucDCBrightness == other.ucDCBrightness
    }
}
impl ::std::cmp::Eq for DISPLAY_BRIGHTNESS {}
unsafe impl ::windows::runtime::Abi for DISPLAY_BRIGHTNESS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const DSI_CHECKSUM_ERROR_CORRECTED: u32 = 256u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const DSI_CHECKSUM_ERROR_NOT_CORRECTED: u32 = 512u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const DSI_CONTENTION_DETECTED: u32 = 128u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DSI_CONTROL_TRANSMISSION_MODE(pub i32);
pub const DCT_DEFAULT: DSI_CONTROL_TRANSMISSION_MODE = DSI_CONTROL_TRANSMISSION_MODE(0i32);
pub const DCT_FORCE_LOW_POWER: DSI_CONTROL_TRANSMISSION_MODE = DSI_CONTROL_TRANSMISSION_MODE(1i32);
pub const DCT_FORCE_HIGH_PERFORMANCE: DSI_CONTROL_TRANSMISSION_MODE = DSI_CONTROL_TRANSMISSION_MODE(2i32);
impl ::std::convert::From<i32> for DSI_CONTROL_TRANSMISSION_MODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DSI_CONTROL_TRANSMISSION_MODE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const DSI_DSI_DATA_TYPE_NOT_RECOGNIZED: u32 = 2048u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const DSI_DSI_PROTOCOL_VIOLATION: u32 = 32768u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const DSI_DSI_VC_ID_INVALID: u32 = 4096u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const DSI_EOT_SYNC_ERROR: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const DSI_ESCAPE_MODE_ENTRY_COMMAND_ERROR: u32 = 8u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const DSI_FALSE_CONTROL_ERROR: u32 = 64u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const DSI_INVALID_PACKET_INDEX: u32 = 255u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const DSI_INVALID_TRANSMISSION_LENGTH: u32 = 8192u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const DSI_LONG_PACKET_PAYLOAD_CHECKSUM_ERROR: u32 = 1024u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const DSI_LOW_POWER_TRANSMIT_SYNC_ERROR: u32 = 16u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const DSI_PACKET_EMBEDDED_PAYLOAD_SIZE: u32 = 8u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const DSI_PERIPHERAL_TIMEOUT_ERROR: u32 = 32u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const DSI_SOT_ERROR: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const DSI_SOT_SYNC_ERROR: u32 = 2u32;
pub const DXCORE_ADAPTER_ATTRIBUTE_D3D11_GRAPHICS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2353497707, 30083, 17677, [240, 240, 107, 173, 168, 149, 175, 75]);
pub const DXCORE_ADAPTER_ATTRIBUTE_D3D12_CORE_COMPUTE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(613296128, 42899, 18212, [171, 170, 35, 166, 222, 27, 224, 144]);
pub const DXCORE_ADAPTER_ATTRIBUTE_D3D12_GRAPHICS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(211734093, 12142, 20225, [140, 150, 232, 158, 51, 27, 71, 177]);
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct DXGK_WIN32K_PARAM_DATA {
    pub PathsArray: *mut ::std::ffi::c_void,
    pub ModesArray: *mut ::std::ffi::c_void,
    pub NumPathArrayElements: u32,
    pub NumModeArrayElements: u32,
    pub SDCFlags: u32,
}
impl DXGK_WIN32K_PARAM_DATA {}
impl ::std::default::Default for DXGK_WIN32K_PARAM_DATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DXGK_WIN32K_PARAM_DATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DXGK_WIN32K_PARAM_DATA").field("PathsArray", &self.PathsArray).field("ModesArray", &self.ModesArray).field("NumPathArrayElements", &self.NumPathArrayElements).field("NumModeArrayElements", &self.NumModeArrayElements).field("SDCFlags", &self.SDCFlags).finish()
    }
}
impl ::std::cmp::PartialEq for DXGK_WIN32K_PARAM_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.PathsArray == other.PathsArray && self.ModesArray == other.ModesArray && self.NumPathArrayElements == other.NumPathArrayElements && self.NumModeArrayElements == other.NumModeArrayElements && self.SDCFlags == other.SDCFlags
    }
}
impl ::std::cmp::Eq for DXGK_WIN32K_PARAM_DATA {}
unsafe impl ::windows::runtime::Abi for DXGK_WIN32K_PARAM_DATA {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const DXGK_WIN32K_PARAM_FLAG_DISABLEVIEW: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const DXGK_WIN32K_PARAM_FLAG_MODESWITCH: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const DXGK_WIN32K_PARAM_FLAG_UPDATEREGISTRY: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DegaussMonitor<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hmonitor: Param0) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DegaussMonitor(hmonitor: super::super::Foundation::HANDLE) -> i32;
        }
        ::std::mem::transmute(DegaussMonitor(hmonitor.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DestroyPhysicalMonitor<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hmonitor: Param0) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DestroyPhysicalMonitor(hmonitor: super::super::Foundation::HANDLE) -> i32;
        }
        ::std::mem::transmute(DestroyPhysicalMonitor(hmonitor.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DestroyPhysicalMonitors(dwphysicalmonitorarraysize: u32, pphysicalmonitorarray: *const PHYSICAL_MONITOR) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DestroyPhysicalMonitors(dwphysicalmonitorarraysize: u32, pphysicalmonitorarray: *const PHYSICAL_MONITOR) -> i32;
        }
        ::std::mem::transmute(DestroyPhysicalMonitors(::std::mem::transmute(dwphysicalmonitorarraysize), ::std::mem::transmute(pphysicalmonitorarray)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
pub struct DisplayMode {
    pub DeviceName: [u16; 32],
    pub devMode: super::super::Graphics::Gdi::DEVMODEW,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl DisplayMode {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::default::Default for DisplayMode {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::PartialEq for DisplayMode {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::Eq for DisplayMode {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::runtime::Abi for DisplayMode {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
pub struct DisplayModes {
    pub numDisplayModes: i32,
    pub displayMode: [DisplayMode; 1],
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl DisplayModes {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::default::Default for DisplayModes {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::PartialEq for DisplayModes {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::Eq for DisplayModes {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::runtime::Abi for DisplayModes {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct ENG_EVENT {
    pub pKEvent: *mut ::std::ffi::c_void,
    pub fFlags: u32,
}
impl ENG_EVENT {}
impl ::std::default::Default for ENG_EVENT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for ENG_EVENT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ENG_EVENT").field("pKEvent", &self.pKEvent).field("fFlags", &self.fFlags).finish()
    }
}
impl ::std::cmp::PartialEq for ENG_EVENT {
    fn eq(&self, other: &Self) -> bool {
        self.pKEvent == other.pKEvent && self.fFlags == other.fFlags
    }
}
impl ::std::cmp::Eq for ENG_EVENT {}
unsafe impl ::windows::runtime::Abi for ENG_EVENT {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_Console")]
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_System_Console`*"]
pub struct FONT_IMAGE_INFO {
    pub FontSize: super::super::System::Console::COORD,
    pub ImageBits: *mut u8,
}
#[cfg(feature = "Win32_System_Console")]
impl FONT_IMAGE_INFO {}
#[cfg(feature = "Win32_System_Console")]
impl ::std::default::Default for FONT_IMAGE_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Console")]
impl ::std::fmt::Debug for FONT_IMAGE_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FONT_IMAGE_INFO").field("FontSize", &self.FontSize).field("ImageBits", &self.ImageBits).finish()
    }
}
#[cfg(feature = "Win32_System_Console")]
impl ::std::cmp::PartialEq for FONT_IMAGE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.FontSize == other.FontSize && self.ImageBits == other.ImageBits
    }
}
#[cfg(feature = "Win32_System_Console")]
impl ::std::cmp::Eq for FONT_IMAGE_INFO {}
#[cfg(feature = "Win32_System_Console")]
unsafe impl ::windows::runtime::Abi for FONT_IMAGE_INFO {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_Console")]
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_System_Console`*"]
pub struct FSCNTL_SCREEN_INFO {
    pub Position: super::super::System::Console::COORD,
    pub ScreenSize: super::super::System::Console::COORD,
    pub nNumberOfChars: u32,
}
#[cfg(feature = "Win32_System_Console")]
impl FSCNTL_SCREEN_INFO {}
#[cfg(feature = "Win32_System_Console")]
impl ::std::default::Default for FSCNTL_SCREEN_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Console")]
impl ::std::fmt::Debug for FSCNTL_SCREEN_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FSCNTL_SCREEN_INFO").field("Position", &self.Position).field("ScreenSize", &self.ScreenSize).field("nNumberOfChars", &self.nNumberOfChars).finish()
    }
}
#[cfg(feature = "Win32_System_Console")]
impl ::std::cmp::PartialEq for FSCNTL_SCREEN_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Position == other.Position && self.ScreenSize == other.ScreenSize && self.nNumberOfChars == other.nNumberOfChars
    }
}
#[cfg(feature = "Win32_System_Console")]
impl ::std::cmp::Eq for FSCNTL_SCREEN_INFO {}
#[cfg(feature = "Win32_System_Console")]
unsafe impl ::windows::runtime::Abi for FSCNTL_SCREEN_INFO {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_Console")]
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_System_Console`*"]
pub struct FSVIDEO_COPY_FRAME_BUFFER {
    pub SrcScreen: FSCNTL_SCREEN_INFO,
    pub DestScreen: FSCNTL_SCREEN_INFO,
}
#[cfg(feature = "Win32_System_Console")]
impl FSVIDEO_COPY_FRAME_BUFFER {}
#[cfg(feature = "Win32_System_Console")]
impl ::std::default::Default for FSVIDEO_COPY_FRAME_BUFFER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Console")]
impl ::std::fmt::Debug for FSVIDEO_COPY_FRAME_BUFFER {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FSVIDEO_COPY_FRAME_BUFFER").field("SrcScreen", &self.SrcScreen).field("DestScreen", &self.DestScreen).finish()
    }
}
#[cfg(feature = "Win32_System_Console")]
impl ::std::cmp::PartialEq for FSVIDEO_COPY_FRAME_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        self.SrcScreen == other.SrcScreen && self.DestScreen == other.DestScreen
    }
}
#[cfg(feature = "Win32_System_Console")]
impl ::std::cmp::Eq for FSVIDEO_COPY_FRAME_BUFFER {}
#[cfg(feature = "Win32_System_Console")]
unsafe impl ::windows::runtime::Abi for FSVIDEO_COPY_FRAME_BUFFER {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct FSVIDEO_CURSOR_POSITION {
    pub Coord: VIDEO_CURSOR_POSITION,
    pub dwType: u32,
}
impl FSVIDEO_CURSOR_POSITION {}
impl ::std::default::Default for FSVIDEO_CURSOR_POSITION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FSVIDEO_CURSOR_POSITION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FSVIDEO_CURSOR_POSITION").field("Coord", &self.Coord).field("dwType", &self.dwType).finish()
    }
}
impl ::std::cmp::PartialEq for FSVIDEO_CURSOR_POSITION {
    fn eq(&self, other: &Self) -> bool {
        self.Coord == other.Coord && self.dwType == other.dwType
    }
}
impl ::std::cmp::Eq for FSVIDEO_CURSOR_POSITION {}
unsafe impl ::windows::runtime::Abi for FSVIDEO_CURSOR_POSITION {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct FSVIDEO_MODE_INFORMATION {
    pub VideoMode: VIDEO_MODE_INFORMATION,
    pub VideoMemory: VIDEO_MEMORY_INFORMATION,
}
impl FSVIDEO_MODE_INFORMATION {}
impl ::std::default::Default for FSVIDEO_MODE_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FSVIDEO_MODE_INFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FSVIDEO_MODE_INFORMATION").field("VideoMode", &self.VideoMode).field("VideoMemory", &self.VideoMemory).finish()
    }
}
impl ::std::cmp::PartialEq for FSVIDEO_MODE_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.VideoMode == other.VideoMode && self.VideoMemory == other.VideoMemory
    }
}
impl ::std::cmp::Eq for FSVIDEO_MODE_INFORMATION {}
unsafe impl ::windows::runtime::Abi for FSVIDEO_MODE_INFORMATION {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_Console")]
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_System_Console`*"]
pub struct FSVIDEO_REVERSE_MOUSE_POINTER {
    pub Screen: FSCNTL_SCREEN_INFO,
    pub dwType: u32,
}
#[cfg(feature = "Win32_System_Console")]
impl FSVIDEO_REVERSE_MOUSE_POINTER {}
#[cfg(feature = "Win32_System_Console")]
impl ::std::default::Default for FSVIDEO_REVERSE_MOUSE_POINTER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Console")]
impl ::std::fmt::Debug for FSVIDEO_REVERSE_MOUSE_POINTER {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FSVIDEO_REVERSE_MOUSE_POINTER").field("Screen", &self.Screen).field("dwType", &self.dwType).finish()
    }
}
#[cfg(feature = "Win32_System_Console")]
impl ::std::cmp::PartialEq for FSVIDEO_REVERSE_MOUSE_POINTER {
    fn eq(&self, other: &Self) -> bool {
        self.Screen == other.Screen && self.dwType == other.dwType
    }
}
#[cfg(feature = "Win32_System_Console")]
impl ::std::cmp::Eq for FSVIDEO_REVERSE_MOUSE_POINTER {}
#[cfg(feature = "Win32_System_Console")]
unsafe impl ::windows::runtime::Abi for FSVIDEO_REVERSE_MOUSE_POINTER {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_Console")]
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_System_Console`*"]
pub struct FSVIDEO_SCREEN_INFORMATION {
    pub ScreenSize: super::super::System::Console::COORD,
    pub FontSize: super::super::System::Console::COORD,
}
#[cfg(feature = "Win32_System_Console")]
impl FSVIDEO_SCREEN_INFORMATION {}
#[cfg(feature = "Win32_System_Console")]
impl ::std::default::Default for FSVIDEO_SCREEN_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Console")]
impl ::std::fmt::Debug for FSVIDEO_SCREEN_INFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FSVIDEO_SCREEN_INFORMATION").field("ScreenSize", &self.ScreenSize).field("FontSize", &self.FontSize).finish()
    }
}
#[cfg(feature = "Win32_System_Console")]
impl ::std::cmp::PartialEq for FSVIDEO_SCREEN_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.ScreenSize == other.ScreenSize && self.FontSize == other.FontSize
    }
}
#[cfg(feature = "Win32_System_Console")]
impl ::std::cmp::Eq for FSVIDEO_SCREEN_INFORMATION {}
#[cfg(feature = "Win32_System_Console")]
unsafe impl ::windows::runtime::Abi for FSVIDEO_SCREEN_INFORMATION {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Console"))]
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`, `Win32_System_Console`*"]
pub struct FSVIDEO_WRITE_TO_FRAME_BUFFER {
    pub SrcBuffer: *mut CHAR_IMAGE_INFO,
    pub DestScreen: FSCNTL_SCREEN_INFO,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Console"))]
impl FSVIDEO_WRITE_TO_FRAME_BUFFER {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Console"))]
impl ::std::default::Default for FSVIDEO_WRITE_TO_FRAME_BUFFER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Console"))]
impl ::std::fmt::Debug for FSVIDEO_WRITE_TO_FRAME_BUFFER {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FSVIDEO_WRITE_TO_FRAME_BUFFER").field("SrcBuffer", &self.SrcBuffer).field("DestScreen", &self.DestScreen).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Console"))]
impl ::std::cmp::PartialEq for FSVIDEO_WRITE_TO_FRAME_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        self.SrcBuffer == other.SrcBuffer && self.DestScreen == other.DestScreen
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Console"))]
impl ::std::cmp::Eq for FSVIDEO_WRITE_TO_FRAME_BUFFER {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Console"))]
unsafe impl ::windows::runtime::Abi for FSVIDEO_WRITE_TO_FRAME_BUFFER {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct GAMMA_RAMP_DXGI_1 {
    pub Scale: GAMMA_RAMP_RGB,
    pub Offset: GAMMA_RAMP_RGB,
    pub GammaCurve: [GAMMA_RAMP_RGB; 1025],
}
impl GAMMA_RAMP_DXGI_1 {}
impl ::std::default::Default for GAMMA_RAMP_DXGI_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for GAMMA_RAMP_DXGI_1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("GAMMA_RAMP_DXGI_1").field("Scale", &self.Scale).field("Offset", &self.Offset).field("GammaCurve", &self.GammaCurve).finish()
    }
}
impl ::std::cmp::PartialEq for GAMMA_RAMP_DXGI_1 {
    fn eq(&self, other: &Self) -> bool {
        self.Scale == other.Scale && self.Offset == other.Offset && self.GammaCurve == other.GammaCurve
    }
}
impl ::std::cmp::Eq for GAMMA_RAMP_DXGI_1 {}
unsafe impl ::windows::runtime::Abi for GAMMA_RAMP_DXGI_1 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct GAMMA_RAMP_RGB {
    pub Red: f32,
    pub Green: f32,
    pub Blue: f32,
}
impl GAMMA_RAMP_RGB {}
impl ::std::default::Default for GAMMA_RAMP_RGB {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for GAMMA_RAMP_RGB {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("GAMMA_RAMP_RGB").field("Red", &self.Red).field("Green", &self.Green).field("Blue", &self.Blue).finish()
    }
}
impl ::std::cmp::PartialEq for GAMMA_RAMP_RGB {
    fn eq(&self, other: &Self) -> bool {
        self.Red == other.Red && self.Green == other.Green && self.Blue == other.Blue
    }
}
impl ::std::cmp::Eq for GAMMA_RAMP_RGB {}
unsafe impl ::windows::runtime::Abi for GAMMA_RAMP_RGB {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct GAMMA_RAMP_RGB256x3x16 {
    pub Red: [u16; 256],
    pub Green: [u16; 256],
    pub Blue: [u16; 256],
}
impl GAMMA_RAMP_RGB256x3x16 {}
impl ::std::default::Default for GAMMA_RAMP_RGB256x3x16 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for GAMMA_RAMP_RGB256x3x16 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("GAMMA_RAMP_RGB256x3x16").field("Red", &self.Red).field("Green", &self.Green).field("Blue", &self.Blue).finish()
    }
}
impl ::std::cmp::PartialEq for GAMMA_RAMP_RGB256x3x16 {
    fn eq(&self, other: &Self) -> bool {
        self.Red == other.Red && self.Green == other.Green && self.Blue == other.Blue
    }
}
impl ::std::cmp::Eq for GAMMA_RAMP_RGB256x3x16 {}
unsafe impl ::windows::runtime::Abi for GAMMA_RAMP_RGB256x3x16 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const GETCONNECTEDIDS_SOURCE: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const GETCONNECTEDIDS_TARGET: u32 = 0u32;
pub const GUID_DEVINTERFACE_DISPLAY_ADAPTER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1531256861, 62194, 20283, [133, 187, 48, 255, 31, 149, 53, 153]);
pub const GUID_DEVINTERFACE_MONITOR: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3874519903, 61079, 19088, [176, 118, 51, 245, 123, 244, 234, 167]);
pub const GUID_DEVINTERFACE_VIDEO_OUTPUT_ARRIVAL: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(450487536, 63629, 17248, [186, 185, 76, 45, 85, 229, 100, 205]);
pub const GUID_DISPLAY_DEVICE_ARRIVAL: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(480268672, 42649, 17674, [154, 12, 222, 79, 190, 61, 221, 137]);
pub const GUID_MONITOR_OVERRIDE_PSEUDO_SPECIALIZED: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4053188655, 63599, 20378, [170, 21, 233, 206, 189, 254, 59, 150]);
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetAutoRotationState(pstate: *mut AR_STATE) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetAutoRotationState(pstate: *mut AR_STATE) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetAutoRotationState(::std::mem::transmute(pstate)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetCapabilitiesStringLength<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hmonitor: Param0, pdwcapabilitiesstringlengthincharacters: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetCapabilitiesStringLength(hmonitor: super::super::Foundation::HANDLE, pdwcapabilitiesstringlengthincharacters: *mut u32) -> i32;
        }
        ::std::mem::transmute(GetCapabilitiesStringLength(hmonitor.into_param().abi(), ::std::mem::transmute(pdwcapabilitiesstringlengthincharacters)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetDisplayAutoRotationPreferences(porientation: *mut ORIENTATION_PREFERENCE) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetDisplayAutoRotationPreferences(porientation: *mut ORIENTATION_PREFERENCE) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetDisplayAutoRotationPreferences(::std::mem::transmute(porientation)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetMonitorBrightness<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hmonitor: Param0, pdwminimumbrightness: *mut u32, pdwcurrentbrightness: *mut u32, pdwmaximumbrightness: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetMonitorBrightness(hmonitor: super::super::Foundation::HANDLE, pdwminimumbrightness: *mut u32, pdwcurrentbrightness: *mut u32, pdwmaximumbrightness: *mut u32) -> i32;
        }
        ::std::mem::transmute(GetMonitorBrightness(hmonitor.into_param().abi(), ::std::mem::transmute(pdwminimumbrightness), ::std::mem::transmute(pdwcurrentbrightness), ::std::mem::transmute(pdwmaximumbrightness)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetMonitorCapabilities<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hmonitor: Param0, pdwmonitorcapabilities: *mut u32, pdwsupportedcolortemperatures: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetMonitorCapabilities(hmonitor: super::super::Foundation::HANDLE, pdwmonitorcapabilities: *mut u32, pdwsupportedcolortemperatures: *mut u32) -> i32;
        }
        ::std::mem::transmute(GetMonitorCapabilities(hmonitor.into_param().abi(), ::std::mem::transmute(pdwmonitorcapabilities), ::std::mem::transmute(pdwsupportedcolortemperatures)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetMonitorColorTemperature<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hmonitor: Param0, pctcurrentcolortemperature: *mut MC_COLOR_TEMPERATURE) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetMonitorColorTemperature(hmonitor: super::super::Foundation::HANDLE, pctcurrentcolortemperature: *mut MC_COLOR_TEMPERATURE) -> i32;
        }
        ::std::mem::transmute(GetMonitorColorTemperature(hmonitor.into_param().abi(), ::std::mem::transmute(pctcurrentcolortemperature)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetMonitorContrast<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hmonitor: Param0, pdwminimumcontrast: *mut u32, pdwcurrentcontrast: *mut u32, pdwmaximumcontrast: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetMonitorContrast(hmonitor: super::super::Foundation::HANDLE, pdwminimumcontrast: *mut u32, pdwcurrentcontrast: *mut u32, pdwmaximumcontrast: *mut u32) -> i32;
        }
        ::std::mem::transmute(GetMonitorContrast(hmonitor.into_param().abi(), ::std::mem::transmute(pdwminimumcontrast), ::std::mem::transmute(pdwcurrentcontrast), ::std::mem::transmute(pdwmaximumcontrast)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetMonitorDisplayAreaPosition<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hmonitor: Param0, ptpositiontype: MC_POSITION_TYPE, pdwminimumposition: *mut u32, pdwcurrentposition: *mut u32, pdwmaximumposition: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetMonitorDisplayAreaPosition(hmonitor: super::super::Foundation::HANDLE, ptpositiontype: MC_POSITION_TYPE, pdwminimumposition: *mut u32, pdwcurrentposition: *mut u32, pdwmaximumposition: *mut u32) -> i32;
        }
        ::std::mem::transmute(GetMonitorDisplayAreaPosition(hmonitor.into_param().abi(), ::std::mem::transmute(ptpositiontype), ::std::mem::transmute(pdwminimumposition), ::std::mem::transmute(pdwcurrentposition), ::std::mem::transmute(pdwmaximumposition)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetMonitorDisplayAreaSize<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hmonitor: Param0, stsizetype: MC_SIZE_TYPE, pdwminimumwidthorheight: *mut u32, pdwcurrentwidthorheight: *mut u32, pdwmaximumwidthorheight: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetMonitorDisplayAreaSize(hmonitor: super::super::Foundation::HANDLE, stsizetype: MC_SIZE_TYPE, pdwminimumwidthorheight: *mut u32, pdwcurrentwidthorheight: *mut u32, pdwmaximumwidthorheight: *mut u32) -> i32;
        }
        ::std::mem::transmute(GetMonitorDisplayAreaSize(hmonitor.into_param().abi(), ::std::mem::transmute(stsizetype), ::std::mem::transmute(pdwminimumwidthorheight), ::std::mem::transmute(pdwcurrentwidthorheight), ::std::mem::transmute(pdwmaximumwidthorheight)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetMonitorRedGreenOrBlueDrive<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hmonitor: Param0, dtdrivetype: MC_DRIVE_TYPE, pdwminimumdrive: *mut u32, pdwcurrentdrive: *mut u32, pdwmaximumdrive: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetMonitorRedGreenOrBlueDrive(hmonitor: super::super::Foundation::HANDLE, dtdrivetype: MC_DRIVE_TYPE, pdwminimumdrive: *mut u32, pdwcurrentdrive: *mut u32, pdwmaximumdrive: *mut u32) -> i32;
        }
        ::std::mem::transmute(GetMonitorRedGreenOrBlueDrive(hmonitor.into_param().abi(), ::std::mem::transmute(dtdrivetype), ::std::mem::transmute(pdwminimumdrive), ::std::mem::transmute(pdwcurrentdrive), ::std::mem::transmute(pdwmaximumdrive)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetMonitorRedGreenOrBlueGain<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hmonitor: Param0, gtgaintype: MC_GAIN_TYPE, pdwminimumgain: *mut u32, pdwcurrentgain: *mut u32, pdwmaximumgain: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetMonitorRedGreenOrBlueGain(hmonitor: super::super::Foundation::HANDLE, gtgaintype: MC_GAIN_TYPE, pdwminimumgain: *mut u32, pdwcurrentgain: *mut u32, pdwmaximumgain: *mut u32) -> i32;
        }
        ::std::mem::transmute(GetMonitorRedGreenOrBlueGain(hmonitor.into_param().abi(), ::std::mem::transmute(gtgaintype), ::std::mem::transmute(pdwminimumgain), ::std::mem::transmute(pdwcurrentgain), ::std::mem::transmute(pdwmaximumgain)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetMonitorTechnologyType<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hmonitor: Param0, pdtydisplaytechnologytype: *mut MC_DISPLAY_TECHNOLOGY_TYPE) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetMonitorTechnologyType(hmonitor: super::super::Foundation::HANDLE, pdtydisplaytechnologytype: *mut MC_DISPLAY_TECHNOLOGY_TYPE) -> i32;
        }
        ::std::mem::transmute(GetMonitorTechnologyType(hmonitor.into_param().abi(), ::std::mem::transmute(pdtydisplaytechnologytype)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Graphics_Gdi`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetNumberOfPhysicalMonitorsFromHMONITOR<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HMONITOR>>(hmonitor: Param0, pdwnumberofphysicalmonitors: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetNumberOfPhysicalMonitorsFromHMONITOR(hmonitor: super::super::Graphics::Gdi::HMONITOR, pdwnumberofphysicalmonitors: *mut u32) -> i32;
        }
        ::std::mem::transmute(GetNumberOfPhysicalMonitorsFromHMONITOR(hmonitor.into_param().abi(), ::std::mem::transmute(pdwnumberofphysicalmonitors)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Graphics_Direct3D9`*"]
#[cfg(feature = "Win32_Graphics_Direct3D9")]
#[inline]
pub unsafe fn GetNumberOfPhysicalMonitorsFromIDirect3DDevice9<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Direct3D9::IDirect3DDevice9>>(pdirect3ddevice9: Param0) -> ::windows::runtime::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetNumberOfPhysicalMonitorsFromIDirect3DDevice9(pdirect3ddevice9: ::windows::runtime::RawPtr, pdwnumberofphysicalmonitors: *mut u32) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        GetNumberOfPhysicalMonitorsFromIDirect3DDevice9(pdirect3ddevice9.into_param().abi(), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn GetPhysicalMonitorsFromHMONITOR<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HMONITOR>>(hmonitor: Param0, dwphysicalmonitorarraysize: u32, pphysicalmonitorarray: *mut PHYSICAL_MONITOR) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPhysicalMonitorsFromHMONITOR(hmonitor: super::super::Graphics::Gdi::HMONITOR, dwphysicalmonitorarraysize: u32, pphysicalmonitorarray: *mut PHYSICAL_MONITOR) -> i32;
        }
        ::std::mem::transmute(GetPhysicalMonitorsFromHMONITOR(hmonitor.into_param().abi(), ::std::mem::transmute(dwphysicalmonitorarraysize), ::std::mem::transmute(pphysicalmonitorarray)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Direct3D9`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9"))]
#[inline]
pub unsafe fn GetPhysicalMonitorsFromIDirect3DDevice9<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Direct3D9::IDirect3DDevice9>>(pdirect3ddevice9: Param0, dwphysicalmonitorarraysize: u32, pphysicalmonitorarray: *mut PHYSICAL_MONITOR) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPhysicalMonitorsFromIDirect3DDevice9(pdirect3ddevice9: ::windows::runtime::RawPtr, dwphysicalmonitorarraysize: u32, pphysicalmonitorarray: *mut PHYSICAL_MONITOR) -> ::windows::runtime::HRESULT;
        }
        GetPhysicalMonitorsFromIDirect3DDevice9(pdirect3ddevice9.into_param().abi(), ::std::mem::transmute(dwphysicalmonitorarraysize), ::std::mem::transmute(pphysicalmonitorarray)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetTimingReport<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hmonitor: Param0, pmtrmonitortimingreport: *mut MC_TIMING_REPORT) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetTimingReport(hmonitor: super::super::Foundation::HANDLE, pmtrmonitortimingreport: *mut MC_TIMING_REPORT) -> i32;
        }
        ::std::mem::transmute(GetTimingReport(hmonitor.into_param().abi(), ::std::mem::transmute(pmtrmonitortimingreport)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetVCPFeatureAndVCPFeatureReply<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hmonitor: Param0, bvcpcode: u8, pvct: *mut MC_VCP_CODE_TYPE, pdwcurrentvalue: *mut u32, pdwmaximumvalue: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetVCPFeatureAndVCPFeatureReply(hmonitor: super::super::Foundation::HANDLE, bvcpcode: u8, pvct: *mut MC_VCP_CODE_TYPE, pdwcurrentvalue: *mut u32, pdwmaximumvalue: *mut u32) -> i32;
        }
        ::std::mem::transmute(GetVCPFeatureAndVCPFeatureReply(hmonitor.into_param().abi(), ::std::mem::transmute(bvcpcode), ::std::mem::transmute(pvct), ::std::mem::transmute(pdwcurrentvalue), ::std::mem::transmute(pdwmaximumvalue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Graphics_DirectDraw")]
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Graphics_DirectDraw`*"]
pub struct HEAPALIGNMENT {
    pub dwSize: u32,
    pub ddsCaps: super::super::Graphics::DirectDraw::DDSCAPS,
    pub dwReserved: u32,
    pub ExecuteBuffer: SURFACEALIGNMENT,
    pub Overlay: SURFACEALIGNMENT,
    pub Texture: SURFACEALIGNMENT,
    pub ZBuffer: SURFACEALIGNMENT,
    pub AlphaBuffer: SURFACEALIGNMENT,
    pub Offscreen: SURFACEALIGNMENT,
    pub FlipTarget: SURFACEALIGNMENT,
}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl HEAPALIGNMENT {}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl ::std::default::Default for HEAPALIGNMENT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl ::std::cmp::PartialEq for HEAPALIGNMENT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl ::std::cmp::Eq for HEAPALIGNMENT {}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
unsafe impl ::windows::runtime::Abi for HEAPALIGNMENT {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const HOST_DSI_BAD_TRANSMISSION_MODE: u32 = 4096u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const HOST_DSI_DEVICE_NOT_READY: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const HOST_DSI_DEVICE_RESET: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const HOST_DSI_DRIVER_REJECTED_PACKET: u32 = 1024u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const HOST_DSI_INTERFACE_RESET: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const HOST_DSI_INVALID_TRANSMISSION: u32 = 256u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const HOST_DSI_OS_REJECTED_PACKET: u32 = 512u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const HOST_DSI_TRANSMISSION_CANCELLED: u32 = 16u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const HOST_DSI_TRANSMISSION_DROPPED: u32 = 32u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const HOST_DSI_TRANSMISSION_TIMEOUT: u32 = 64u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ICloneViewHelper(pub ::windows::runtime::IUnknown);
impl ICloneViewHelper {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    pub unsafe fn GetConnectedIDs<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wszadaptorname: Param0, pulcount: *mut u32, pulid: *mut u32, ulflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), wszadaptorname.into_param().abi(), ::std::mem::transmute(pulcount), ::std::mem::transmute(pulid), ::std::mem::transmute(ulflags)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    pub unsafe fn GetActiveTopology<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wszadaptorname: Param0, ulsourceid: u32, pulcount: *mut u32, pultargetid: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), wszadaptorname.into_param().abi(), ::std::mem::transmute(ulsourceid), ::std::mem::transmute(pulcount), ::std::mem::transmute(pultargetid)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    pub unsafe fn SetActiveTopology<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wszadaptorname: Param0, ulsourceid: u32, ulcount: u32, pultargetid: *const u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), wszadaptorname.into_param().abi(), ::std::mem::transmute(ulsourceid), ::std::mem::transmute(ulcount), ::std::mem::transmute(pultargetid)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    pub unsafe fn Commit<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, ffinalcall: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ffinalcall.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ICloneViewHelper {
    type Vtable = ICloneViewHelper_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4137931972, 22066, 19843, [176, 161, 251, 136, 113, 43, 30, 183]);
}
impl ::std::convert::From<ICloneViewHelper> for ::windows::runtime::IUnknown {
    fn from(value: ICloneViewHelper) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ICloneViewHelper> for ::windows::runtime::IUnknown {
    fn from(value: &ICloneViewHelper) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ICloneViewHelper {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ICloneViewHelper {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICloneViewHelper_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wszadaptorname: super::super::Foundation::PWSTR, pulcount: *mut u32, pulid: *mut u32, ulflags: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wszadaptorname: super::super::Foundation::PWSTR, ulsourceid: u32, pulcount: *mut u32, pultargetid: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wszadaptorname: super::super::Foundation::PWSTR, ulsourceid: u32, ulcount: u32, pultargetid: *const u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ffinalcall: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
pub struct INDIRECT_DISPLAY_INFO {
    pub DisplayAdapterLuid: super::super::Foundation::LUID,
    pub Flags: u32,
    pub NumMonitors: u32,
    pub DisplayAdapterTargetBase: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl INDIRECT_DISPLAY_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for INDIRECT_DISPLAY_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for INDIRECT_DISPLAY_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("INDIRECT_DISPLAY_INFO").field("DisplayAdapterLuid", &self.DisplayAdapterLuid).field("Flags", &self.Flags).field("NumMonitors", &self.NumMonitors).field("DisplayAdapterTargetBase", &self.DisplayAdapterTargetBase).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for INDIRECT_DISPLAY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.DisplayAdapterLuid == other.DisplayAdapterLuid && self.Flags == other.Flags && self.NumMonitors == other.NumMonitors && self.DisplayAdapterTargetBase == other.DisplayAdapterTargetBase
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for INDIRECT_DISPLAY_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for INDIRECT_DISPLAY_INFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDIRECT_DISPLAY_INFO_FLAGS_CREATED_IDDCX_ADAPTER: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const IOCTL_COLORSPACE_TRANSFORM_QUERY_TARGET_CAPS: u32 = 2297856u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const IOCTL_COLORSPACE_TRANSFORM_SET: u32 = 2297860u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const IOCTL_FSVIDEO_COPY_FRAME_BUFFER: u32 = 3409920u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const IOCTL_FSVIDEO_REVERSE_MOUSE_POINTER: u32 = 3409928u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const IOCTL_FSVIDEO_SET_CURRENT_MODE: u32 = 3409932u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const IOCTL_FSVIDEO_SET_CURSOR_POSITION: u32 = 3409940u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const IOCTL_FSVIDEO_SET_SCREEN_INFORMATION: u32 = 3409936u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const IOCTL_FSVIDEO_WRITE_TO_FRAME_BUFFER: u32 = 3409924u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const IOCTL_MIPI_DSI_QUERY_CAPS: u32 = 2298880u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const IOCTL_MIPI_DSI_RESET: u32 = 2298888u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const IOCTL_MIPI_DSI_TRANSMISSION: u32 = 2298884u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const IOCTL_PANEL_GET_BACKLIGHT_REDUCTION: u32 = 2296856u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const IOCTL_PANEL_GET_BRIGHTNESS: u32 = 2296840u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const IOCTL_PANEL_QUERY_BRIGHTNESS_CAPS: u32 = 2296832u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const IOCTL_PANEL_QUERY_BRIGHTNESS_RANGES: u32 = 2296836u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const IOCTL_PANEL_SET_BACKLIGHT_OPTIMIZATION: u32 = 2296852u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const IOCTL_PANEL_SET_BRIGHTNESS: u32 = 2296844u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const IOCTL_PANEL_SET_BRIGHTNESS_STATE: u32 = 2296848u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const IOCTL_SET_ACTIVE_COLOR_PROFILE_NAME: u32 = 2297864u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const IOCTL_VIDEO_DISABLE_CURSOR: u32 = 2294820u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const IOCTL_VIDEO_DISABLE_POINTER: u32 = 2294844u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const IOCTL_VIDEO_DISABLE_VDM: u32 = 2293764u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const IOCTL_VIDEO_ENABLE_CURSOR: u32 = 2294816u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const IOCTL_VIDEO_ENABLE_POINTER: u32 = 2294840u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const IOCTL_VIDEO_ENABLE_VDM: u32 = 2293760u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const IOCTL_VIDEO_ENUM_MONITOR_PDO: u32 = 2293784u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const IOCTL_VIDEO_FREE_PUBLIC_ACCESS_RANGES: u32 = 2294884u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const IOCTL_VIDEO_GET_BANK_SELECT_CODE: u32 = 2294868u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const IOCTL_VIDEO_GET_CHILD_STATE: u32 = 2294912u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const IOCTL_VIDEO_GET_OUTPUT_DEVICE_POWER_STATE: u32 = 2293776u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const IOCTL_VIDEO_GET_POWER_MANAGEMENT: u32 = 2294896u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const IOCTL_VIDEO_HANDLE_VIDEOPARAMETERS: u32 = 2293792u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const IOCTL_VIDEO_INIT_WIN32K_CALLBACKS: u32 = 2293788u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const IOCTL_VIDEO_IS_VGA_DEVICE: u32 = 2293796u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const IOCTL_VIDEO_LOAD_AND_SET_FONT: u32 = 2294804u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const IOCTL_VIDEO_MAP_VIDEO_MEMORY: u32 = 2294872u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const IOCTL_VIDEO_MONITOR_DEVICE: u32 = 2293780u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const IOCTL_VIDEO_PREPARE_FOR_EARECOVERY: u32 = 2293804u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const IOCTL_VIDEO_QUERY_AVAIL_MODES: u32 = 2294784u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const IOCTL_VIDEO_QUERY_COLOR_CAPABILITIES: u32 = 2294888u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const IOCTL_VIDEO_QUERY_CURRENT_MODE: u32 = 2294792u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const IOCTL_VIDEO_QUERY_CURSOR_ATTR: u32 = 2294828u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const IOCTL_VIDEO_QUERY_CURSOR_POSITION: u32 = 2294836u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const IOCTL_VIDEO_QUERY_DISPLAY_BRIGHTNESS: u32 = 2294936u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const IOCTL_VIDEO_QUERY_NUM_AVAIL_MODES: u32 = 2294788u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const IOCTL_VIDEO_QUERY_POINTER_ATTR: u32 = 2294852u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const IOCTL_VIDEO_QUERY_POINTER_CAPABILITIES: u32 = 2294864u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const IOCTL_VIDEO_QUERY_POINTER_POSITION: u32 = 2294860u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const IOCTL_VIDEO_QUERY_PUBLIC_ACCESS_RANGES: u32 = 2294880u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const IOCTL_VIDEO_QUERY_SUPPORTED_BRIGHTNESS: u32 = 2294932u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const IOCTL_VIDEO_REGISTER_VDM: u32 = 2293768u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const IOCTL_VIDEO_RESET_DEVICE: u32 = 2294800u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const IOCTL_VIDEO_RESTORE_HARDWARE_STATE: u32 = 2294276u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const IOCTL_VIDEO_SAVE_HARDWARE_STATE: u32 = 2294272u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const IOCTL_VIDEO_SET_BANK_POSITION: u32 = 2294928u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const IOCTL_VIDEO_SET_CHILD_STATE_CONFIGURATION: u32 = 2294920u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const IOCTL_VIDEO_SET_COLOR_LUT_DATA: u32 = 2294908u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const IOCTL_VIDEO_SET_COLOR_REGISTERS: u32 = 2294812u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const IOCTL_VIDEO_SET_CURRENT_MODE: u32 = 2294796u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const IOCTL_VIDEO_SET_CURSOR_ATTR: u32 = 2294824u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const IOCTL_VIDEO_SET_CURSOR_POSITION: u32 = 2294832u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const IOCTL_VIDEO_SET_DISPLAY_BRIGHTNESS: u32 = 2294940u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const IOCTL_VIDEO_SET_OUTPUT_DEVICE_POWER_STATE: u32 = 2293772u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const IOCTL_VIDEO_SET_PALETTE_REGISTERS: u32 = 2294808u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const IOCTL_VIDEO_SET_POINTER_ATTR: u32 = 2294848u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const IOCTL_VIDEO_SET_POINTER_POSITION: u32 = 2294856u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const IOCTL_VIDEO_SET_POWER_MANAGEMENT: u32 = 2294892u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const IOCTL_VIDEO_SHARE_VIDEO_MEMORY: u32 = 2294900u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const IOCTL_VIDEO_SWITCH_DUALVIEW: u32 = 2294924u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const IOCTL_VIDEO_UNMAP_VIDEO_MEMORY: u32 = 2294876u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const IOCTL_VIDEO_UNSHARE_VIDEO_MEMORY: u32 = 2294904u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const IOCTL_VIDEO_USE_DEVICE_IN_SESSION: u32 = 2293800u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const IOCTL_VIDEO_VALIDATE_CHILD_STATE_CONFIGURATION: u32 = 2294916u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IViewHelper(pub ::windows::runtime::IUnknown);
impl IViewHelper {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    pub unsafe fn GetConnectedIDs<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wszadaptorname: Param0, pulcount: *mut u32, pulid: *mut u32, ulflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), wszadaptorname.into_param().abi(), ::std::mem::transmute(pulcount), ::std::mem::transmute(pulid), ::std::mem::transmute(ulflags)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    pub unsafe fn GetActiveTopology<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wszadaptorname: Param0, ulsourceid: u32, pulcount: *mut u32, pultargetid: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), wszadaptorname.into_param().abi(), ::std::mem::transmute(ulsourceid), ::std::mem::transmute(pulcount), ::std::mem::transmute(pultargetid)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    pub unsafe fn SetActiveTopology<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wszadaptorname: Param0, ulsourceid: u32, ulcount: u32, pultargetid: *const u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), wszadaptorname.into_param().abi(), ::std::mem::transmute(ulsourceid), ::std::mem::transmute(ulcount), ::std::mem::transmute(pultargetid)).ok()
    }
    #[doc = "*Required features: `Win32_Devices_Display`*"]
    pub unsafe fn Commit(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_System_Com`*"]
    pub unsafe fn SetConfiguration<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>>(&self, pistream: Param0) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), pistream.into_param().abi(), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_Display`*"]
    pub unsafe fn GetProceedOnNewConfiguration(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IViewHelper {
    type Vtable = IViewHelper_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3898396405, 43690, 18416, [181, 227, 97, 247, 174, 205, 196, 193]);
}
impl ::std::convert::From<IViewHelper> for ::windows::runtime::IUnknown {
    fn from(value: IViewHelper) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IViewHelper> for ::windows::runtime::IUnknown {
    fn from(value: &IViewHelper) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IViewHelper {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IViewHelper {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IViewHelper_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wszadaptorname: super::super::Foundation::PWSTR, pulcount: *mut u32, pulid: *mut u32, ulflags: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wszadaptorname: super::super::Foundation::PWSTR, ulsourceid: u32, pulcount: *mut u32, pultargetid: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wszadaptorname: super::super::Foundation::PWSTR, ulsourceid: u32, ulcount: u32, pultargetid: *const u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pistream: ::windows::runtime::RawPtr, pulstatus: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const MAX_PACKET_COUNT: u32 = 128u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const MC_CAPS_BRIGHTNESS: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const MC_CAPS_COLOR_TEMPERATURE: u32 = 8u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const MC_CAPS_CONTRAST: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const MC_CAPS_DEGAUSS: u32 = 64u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const MC_CAPS_DISPLAY_AREA_POSITION: u32 = 128u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const MC_CAPS_DISPLAY_AREA_SIZE: u32 = 256u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const MC_CAPS_MONITOR_TECHNOLOGY_TYPE: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const MC_CAPS_NONE: u32 = 0u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const MC_CAPS_RED_GREEN_BLUE_DRIVE: u32 = 32u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const MC_CAPS_RED_GREEN_BLUE_GAIN: u32 = 16u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const MC_CAPS_RESTORE_FACTORY_COLOR_DEFAULTS: u32 = 2048u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const MC_CAPS_RESTORE_FACTORY_DEFAULTS: u32 = 1024u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct MC_COLOR_TEMPERATURE(pub i32);
pub const MC_COLOR_TEMPERATURE_UNKNOWN: MC_COLOR_TEMPERATURE = MC_COLOR_TEMPERATURE(0i32);
pub const MC_COLOR_TEMPERATURE_4000K: MC_COLOR_TEMPERATURE = MC_COLOR_TEMPERATURE(1i32);
pub const MC_COLOR_TEMPERATURE_5000K: MC_COLOR_TEMPERATURE = MC_COLOR_TEMPERATURE(2i32);
pub const MC_COLOR_TEMPERATURE_6500K: MC_COLOR_TEMPERATURE = MC_COLOR_TEMPERATURE(3i32);
pub const MC_COLOR_TEMPERATURE_7500K: MC_COLOR_TEMPERATURE = MC_COLOR_TEMPERATURE(4i32);
pub const MC_COLOR_TEMPERATURE_8200K: MC_COLOR_TEMPERATURE = MC_COLOR_TEMPERATURE(5i32);
pub const MC_COLOR_TEMPERATURE_9300K: MC_COLOR_TEMPERATURE = MC_COLOR_TEMPERATURE(6i32);
pub const MC_COLOR_TEMPERATURE_10000K: MC_COLOR_TEMPERATURE = MC_COLOR_TEMPERATURE(7i32);
pub const MC_COLOR_TEMPERATURE_11500K: MC_COLOR_TEMPERATURE = MC_COLOR_TEMPERATURE(8i32);
impl ::std::convert::From<i32> for MC_COLOR_TEMPERATURE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MC_COLOR_TEMPERATURE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct MC_DISPLAY_TECHNOLOGY_TYPE(pub i32);
pub const MC_SHADOW_MASK_CATHODE_RAY_TUBE: MC_DISPLAY_TECHNOLOGY_TYPE = MC_DISPLAY_TECHNOLOGY_TYPE(0i32);
pub const MC_APERTURE_GRILL_CATHODE_RAY_TUBE: MC_DISPLAY_TECHNOLOGY_TYPE = MC_DISPLAY_TECHNOLOGY_TYPE(1i32);
pub const MC_THIN_FILM_TRANSISTOR: MC_DISPLAY_TECHNOLOGY_TYPE = MC_DISPLAY_TECHNOLOGY_TYPE(2i32);
pub const MC_LIQUID_CRYSTAL_ON_SILICON: MC_DISPLAY_TECHNOLOGY_TYPE = MC_DISPLAY_TECHNOLOGY_TYPE(3i32);
pub const MC_PLASMA: MC_DISPLAY_TECHNOLOGY_TYPE = MC_DISPLAY_TECHNOLOGY_TYPE(4i32);
pub const MC_ORGANIC_LIGHT_EMITTING_DIODE: MC_DISPLAY_TECHNOLOGY_TYPE = MC_DISPLAY_TECHNOLOGY_TYPE(5i32);
pub const MC_ELECTROLUMINESCENT: MC_DISPLAY_TECHNOLOGY_TYPE = MC_DISPLAY_TECHNOLOGY_TYPE(6i32);
pub const MC_MICROELECTROMECHANICAL: MC_DISPLAY_TECHNOLOGY_TYPE = MC_DISPLAY_TECHNOLOGY_TYPE(7i32);
pub const MC_FIELD_EMISSION_DEVICE: MC_DISPLAY_TECHNOLOGY_TYPE = MC_DISPLAY_TECHNOLOGY_TYPE(8i32);
impl ::std::convert::From<i32> for MC_DISPLAY_TECHNOLOGY_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MC_DISPLAY_TECHNOLOGY_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct MC_DRIVE_TYPE(pub i32);
pub const MC_RED_DRIVE: MC_DRIVE_TYPE = MC_DRIVE_TYPE(0i32);
pub const MC_GREEN_DRIVE: MC_DRIVE_TYPE = MC_DRIVE_TYPE(1i32);
pub const MC_BLUE_DRIVE: MC_DRIVE_TYPE = MC_DRIVE_TYPE(2i32);
impl ::std::convert::From<i32> for MC_DRIVE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MC_DRIVE_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct MC_GAIN_TYPE(pub i32);
pub const MC_RED_GAIN: MC_GAIN_TYPE = MC_GAIN_TYPE(0i32);
pub const MC_GREEN_GAIN: MC_GAIN_TYPE = MC_GAIN_TYPE(1i32);
pub const MC_BLUE_GAIN: MC_GAIN_TYPE = MC_GAIN_TYPE(2i32);
impl ::std::convert::From<i32> for MC_GAIN_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MC_GAIN_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct MC_POSITION_TYPE(pub i32);
pub const MC_HORIZONTAL_POSITION: MC_POSITION_TYPE = MC_POSITION_TYPE(0i32);
pub const MC_VERTICAL_POSITION: MC_POSITION_TYPE = MC_POSITION_TYPE(1i32);
impl ::std::convert::From<i32> for MC_POSITION_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MC_POSITION_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const MC_RESTORE_FACTORY_DEFAULTS_ENABLES_MONITOR_SETTINGS: u32 = 4096u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct MC_SIZE_TYPE(pub i32);
pub const MC_WIDTH: MC_SIZE_TYPE = MC_SIZE_TYPE(0i32);
pub const MC_HEIGHT: MC_SIZE_TYPE = MC_SIZE_TYPE(1i32);
impl ::std::convert::From<i32> for MC_SIZE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MC_SIZE_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const MC_SUPPORTED_COLOR_TEMPERATURE_10000K: u32 = 64u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const MC_SUPPORTED_COLOR_TEMPERATURE_11500K: u32 = 128u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const MC_SUPPORTED_COLOR_TEMPERATURE_4000K: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const MC_SUPPORTED_COLOR_TEMPERATURE_5000K: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const MC_SUPPORTED_COLOR_TEMPERATURE_6500K: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const MC_SUPPORTED_COLOR_TEMPERATURE_7500K: u32 = 8u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const MC_SUPPORTED_COLOR_TEMPERATURE_8200K: u32 = 16u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const MC_SUPPORTED_COLOR_TEMPERATURE_9300K: u32 = 32u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const MC_SUPPORTED_COLOR_TEMPERATURE_NONE: u32 = 0u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct MC_TIMING_REPORT {
    pub dwHorizontalFrequencyInHZ: u32,
    pub dwVerticalFrequencyInHZ: u32,
    pub bTimingStatusByte: u8,
}
impl MC_TIMING_REPORT {}
impl ::std::default::Default for MC_TIMING_REPORT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for MC_TIMING_REPORT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for MC_TIMING_REPORT {}
unsafe impl ::windows::runtime::Abi for MC_TIMING_REPORT {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct MC_VCP_CODE_TYPE(pub i32);
pub const MC_MOMENTARY: MC_VCP_CODE_TYPE = MC_VCP_CODE_TYPE(0i32);
pub const MC_SET_PARAMETER: MC_VCP_CODE_TYPE = MC_VCP_CODE_TYPE(1i32);
impl ::std::convert::From<i32> for MC_VCP_CODE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MC_VCP_CODE_TYPE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct MIPI_DSI_CAPS {
    pub DSITypeMajor: u8,
    pub DSITypeMinor: u8,
    pub SpecVersionMajor: u8,
    pub SpecVersionMinor: u8,
    pub SpecVersionPatch: u8,
    pub TargetMaximumReturnPacketSize: u16,
    pub ResultCodeFlags: u8,
    pub ResultCodeStatus: u8,
    pub Revision: u8,
    pub Level: u8,
    pub DeviceClassHi: u8,
    pub DeviceClassLo: u8,
    pub ManufacturerHi: u8,
    pub ManufacturerLo: u8,
    pub ProductHi: u8,
    pub ProductLo: u8,
    pub LengthHi: u8,
    pub LengthLo: u8,
}
impl MIPI_DSI_CAPS {}
impl ::std::default::Default for MIPI_DSI_CAPS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for MIPI_DSI_CAPS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MIPI_DSI_CAPS")
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
impl ::std::cmp::PartialEq for MIPI_DSI_CAPS {
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
impl ::std::cmp::Eq for MIPI_DSI_CAPS {}
unsafe impl ::windows::runtime::Abi for MIPI_DSI_CAPS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct MIPI_DSI_PACKET {
    pub Anonymous1: MIPI_DSI_PACKET_0,
    pub Anonymous2: MIPI_DSI_PACKET_1,
    pub EccFiller: u8,
    pub Payload: [u8; 8],
}
impl MIPI_DSI_PACKET {}
impl ::std::default::Default for MIPI_DSI_PACKET {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for MIPI_DSI_PACKET {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for MIPI_DSI_PACKET {}
unsafe impl ::windows::runtime::Abi for MIPI_DSI_PACKET {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub union MIPI_DSI_PACKET_0 {
    pub DataId: u8,
    pub Anonymous: MIPI_DSI_PACKET_0_0,
}
impl MIPI_DSI_PACKET_0 {}
impl ::std::default::Default for MIPI_DSI_PACKET_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for MIPI_DSI_PACKET_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for MIPI_DSI_PACKET_0 {}
unsafe impl ::windows::runtime::Abi for MIPI_DSI_PACKET_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct MIPI_DSI_PACKET_0_0 {
    pub _bitfield: u8,
}
impl MIPI_DSI_PACKET_0_0 {}
impl ::std::default::Default for MIPI_DSI_PACKET_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for MIPI_DSI_PACKET_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::std::cmp::PartialEq for MIPI_DSI_PACKET_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for MIPI_DSI_PACKET_0_0 {}
unsafe impl ::windows::runtime::Abi for MIPI_DSI_PACKET_0_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub union MIPI_DSI_PACKET_1 {
    pub Anonymous: MIPI_DSI_PACKET_1_0,
    pub LongWriteWordCount: u16,
}
impl MIPI_DSI_PACKET_1 {}
impl ::std::default::Default for MIPI_DSI_PACKET_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for MIPI_DSI_PACKET_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for MIPI_DSI_PACKET_1 {}
unsafe impl ::windows::runtime::Abi for MIPI_DSI_PACKET_1 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct MIPI_DSI_PACKET_1_0 {
    pub Data0: u8,
    pub Data1: u8,
}
impl MIPI_DSI_PACKET_1_0 {}
impl ::std::default::Default for MIPI_DSI_PACKET_1_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for MIPI_DSI_PACKET_1_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("Data0", &self.Data0).field("Data1", &self.Data1).finish()
    }
}
impl ::std::cmp::PartialEq for MIPI_DSI_PACKET_1_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Data0 == other.Data0 && self.Data1 == other.Data1
    }
}
impl ::std::cmp::Eq for MIPI_DSI_PACKET_1_0 {}
unsafe impl ::windows::runtime::Abi for MIPI_DSI_PACKET_1_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct MIPI_DSI_RESET {
    pub Flags: u32,
    pub Anonymous: MIPI_DSI_RESET_0,
}
impl MIPI_DSI_RESET {}
impl ::std::default::Default for MIPI_DSI_RESET {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for MIPI_DSI_RESET {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for MIPI_DSI_RESET {}
unsafe impl ::windows::runtime::Abi for MIPI_DSI_RESET {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub union MIPI_DSI_RESET_0 {
    pub Anonymous: MIPI_DSI_RESET_0_0,
    pub Results: u32,
}
impl MIPI_DSI_RESET_0 {}
impl ::std::default::Default for MIPI_DSI_RESET_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for MIPI_DSI_RESET_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for MIPI_DSI_RESET_0 {}
unsafe impl ::windows::runtime::Abi for MIPI_DSI_RESET_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct MIPI_DSI_RESET_0_0 {
    pub _bitfield: u32,
}
impl MIPI_DSI_RESET_0_0 {}
impl ::std::default::Default for MIPI_DSI_RESET_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for MIPI_DSI_RESET_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::std::cmp::PartialEq for MIPI_DSI_RESET_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for MIPI_DSI_RESET_0_0 {}
unsafe impl ::windows::runtime::Abi for MIPI_DSI_RESET_0_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct MIPI_DSI_TRANSMISSION {
    pub TotalBufferSize: u32,
    pub PacketCount: u8,
    pub FailedPacket: u8,
    pub Anonymous: MIPI_DSI_TRANSMISSION_0,
    pub ReadWordCount: u16,
    pub FinalCommandExtraPayload: u16,
    pub MipiErrors: u16,
    pub HostErrors: u16,
    pub Packets: [MIPI_DSI_PACKET; 1],
}
impl MIPI_DSI_TRANSMISSION {}
impl ::std::default::Default for MIPI_DSI_TRANSMISSION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for MIPI_DSI_TRANSMISSION {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for MIPI_DSI_TRANSMISSION {}
unsafe impl ::windows::runtime::Abi for MIPI_DSI_TRANSMISSION {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct MIPI_DSI_TRANSMISSION_0 {
    pub _bitfield: u16,
}
impl MIPI_DSI_TRANSMISSION_0 {}
impl ::std::default::Default for MIPI_DSI_TRANSMISSION_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for MIPI_DSI_TRANSMISSION_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::std::cmp::PartialEq for MIPI_DSI_TRANSMISSION_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for MIPI_DSI_TRANSMISSION_0 {}
unsafe impl ::windows::runtime::Abi for MIPI_DSI_TRANSMISSION_0 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ORIENTATION_PREFERENCE(pub i32);
pub const ORIENTATION_PREFERENCE_NONE: ORIENTATION_PREFERENCE = ORIENTATION_PREFERENCE(0i32);
pub const ORIENTATION_PREFERENCE_LANDSCAPE: ORIENTATION_PREFERENCE = ORIENTATION_PREFERENCE(1i32);
pub const ORIENTATION_PREFERENCE_PORTRAIT: ORIENTATION_PREFERENCE = ORIENTATION_PREFERENCE(2i32);
pub const ORIENTATION_PREFERENCE_LANDSCAPE_FLIPPED: ORIENTATION_PREFERENCE = ORIENTATION_PREFERENCE(4i32);
pub const ORIENTATION_PREFERENCE_PORTRAIT_FLIPPED: ORIENTATION_PREFERENCE = ORIENTATION_PREFERENCE(8i32);
impl ::std::convert::From<i32> for ORIENTATION_PREFERENCE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ORIENTATION_PREFERENCE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct OUTPUT_COLOR_ENCODING(pub i32);
pub const OUTPUT_COLOR_ENCODING_RGB: OUTPUT_COLOR_ENCODING = OUTPUT_COLOR_ENCODING(0i32);
pub const OUTPUT_COLOR_ENCODING_YCBCR444: OUTPUT_COLOR_ENCODING = OUTPUT_COLOR_ENCODING(1i32);
pub const OUTPUT_COLOR_ENCODING_YCBCR422: OUTPUT_COLOR_ENCODING = OUTPUT_COLOR_ENCODING(2i32);
pub const OUTPUT_COLOR_ENCODING_YCBCR420: OUTPUT_COLOR_ENCODING = OUTPUT_COLOR_ENCODING(3i32);
pub const OUTPUT_COLOR_ENCODING_INTENSITY: OUTPUT_COLOR_ENCODING = OUTPUT_COLOR_ENCODING(4i32);
pub const OUTPUT_COLOR_ENCODING_FORCE_UINT32: OUTPUT_COLOR_ENCODING = OUTPUT_COLOR_ENCODING(-1i32);
impl ::std::convert::From<i32> for OUTPUT_COLOR_ENCODING {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for OUTPUT_COLOR_ENCODING {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct OUTPUT_WIRE_COLOR_SPACE_TYPE(pub i32);
pub const OUTPUT_WIRE_COLOR_SPACE_G22_P709: OUTPUT_WIRE_COLOR_SPACE_TYPE = OUTPUT_WIRE_COLOR_SPACE_TYPE(0i32);
pub const OUTPUT_WIRE_COLOR_SPACE_RESERVED: OUTPUT_WIRE_COLOR_SPACE_TYPE = OUTPUT_WIRE_COLOR_SPACE_TYPE(4i32);
pub const OUTPUT_WIRE_COLOR_SPACE_G2084_P2020: OUTPUT_WIRE_COLOR_SPACE_TYPE = OUTPUT_WIRE_COLOR_SPACE_TYPE(12i32);
pub const OUTPUT_WIRE_COLOR_SPACE_G22_P709_WCG: OUTPUT_WIRE_COLOR_SPACE_TYPE = OUTPUT_WIRE_COLOR_SPACE_TYPE(30i32);
pub const OUTPUT_WIRE_COLOR_SPACE_G22_P2020: OUTPUT_WIRE_COLOR_SPACE_TYPE = OUTPUT_WIRE_COLOR_SPACE_TYPE(31i32);
pub const OUTPUT_WIRE_COLOR_SPACE_G2084_P2020_HDR10PLUS: OUTPUT_WIRE_COLOR_SPACE_TYPE = OUTPUT_WIRE_COLOR_SPACE_TYPE(32i32);
pub const OUTPUT_WIRE_COLOR_SPACE_G2084_P2020_DVLL: OUTPUT_WIRE_COLOR_SPACE_TYPE = OUTPUT_WIRE_COLOR_SPACE_TYPE(33i32);
impl ::std::convert::From<i32> for OUTPUT_WIRE_COLOR_SPACE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for OUTPUT_WIRE_COLOR_SPACE_TYPE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct OUTPUT_WIRE_FORMAT {
    pub ColorEncoding: OUTPUT_COLOR_ENCODING,
    pub BitsPerPixel: u32,
}
impl OUTPUT_WIRE_FORMAT {}
impl ::std::default::Default for OUTPUT_WIRE_FORMAT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for OUTPUT_WIRE_FORMAT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("OUTPUT_WIRE_FORMAT").field("ColorEncoding", &self.ColorEncoding).field("BitsPerPixel", &self.BitsPerPixel).finish()
    }
}
impl ::std::cmp::PartialEq for OUTPUT_WIRE_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.ColorEncoding == other.ColorEncoding && self.BitsPerPixel == other.BitsPerPixel
    }
}
impl ::std::cmp::Eq for OUTPUT_WIRE_FORMAT {}
unsafe impl ::windows::runtime::Abi for OUTPUT_WIRE_FORMAT {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct PANEL_BRIGHTNESS_SENSOR_DATA {
    pub Anonymous: PANEL_BRIGHTNESS_SENSOR_DATA_0,
    pub AlsReading: f32,
    pub ChromaticityCoordinate: CHROMATICITY_COORDINATE,
    pub ColorTemperature: f32,
}
impl PANEL_BRIGHTNESS_SENSOR_DATA {}
impl ::std::default::Default for PANEL_BRIGHTNESS_SENSOR_DATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for PANEL_BRIGHTNESS_SENSOR_DATA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for PANEL_BRIGHTNESS_SENSOR_DATA {}
unsafe impl ::windows::runtime::Abi for PANEL_BRIGHTNESS_SENSOR_DATA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub union PANEL_BRIGHTNESS_SENSOR_DATA_0 {
    pub Anonymous: PANEL_BRIGHTNESS_SENSOR_DATA_0_0,
    pub Value: u32,
}
impl PANEL_BRIGHTNESS_SENSOR_DATA_0 {}
impl ::std::default::Default for PANEL_BRIGHTNESS_SENSOR_DATA_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for PANEL_BRIGHTNESS_SENSOR_DATA_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for PANEL_BRIGHTNESS_SENSOR_DATA_0 {}
unsafe impl ::windows::runtime::Abi for PANEL_BRIGHTNESS_SENSOR_DATA_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct PANEL_BRIGHTNESS_SENSOR_DATA_0_0 {
    pub _bitfield: u32,
}
impl PANEL_BRIGHTNESS_SENSOR_DATA_0_0 {}
impl ::std::default::Default for PANEL_BRIGHTNESS_SENSOR_DATA_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PANEL_BRIGHTNESS_SENSOR_DATA_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::std::cmp::PartialEq for PANEL_BRIGHTNESS_SENSOR_DATA_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for PANEL_BRIGHTNESS_SENSOR_DATA_0_0 {}
unsafe impl ::windows::runtime::Abi for PANEL_BRIGHTNESS_SENSOR_DATA_0_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct PANEL_GET_BACKLIGHT_REDUCTION {
    pub BacklightUsersetting: u16,
    pub BacklightEffective: u16,
    pub GammaRamp: BACKLIGHT_REDUCTION_GAMMA_RAMP,
}
impl PANEL_GET_BACKLIGHT_REDUCTION {}
impl ::std::default::Default for PANEL_GET_BACKLIGHT_REDUCTION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PANEL_GET_BACKLIGHT_REDUCTION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PANEL_GET_BACKLIGHT_REDUCTION").field("BacklightUsersetting", &self.BacklightUsersetting).field("BacklightEffective", &self.BacklightEffective).field("GammaRamp", &self.GammaRamp).finish()
    }
}
impl ::std::cmp::PartialEq for PANEL_GET_BACKLIGHT_REDUCTION {
    fn eq(&self, other: &Self) -> bool {
        self.BacklightUsersetting == other.BacklightUsersetting && self.BacklightEffective == other.BacklightEffective && self.GammaRamp == other.GammaRamp
    }
}
impl ::std::cmp::Eq for PANEL_GET_BACKLIGHT_REDUCTION {}
unsafe impl ::windows::runtime::Abi for PANEL_GET_BACKLIGHT_REDUCTION {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct PANEL_GET_BRIGHTNESS {
    pub Version: BRIGHTNESS_INTERFACE_VERSION,
    pub Anonymous: PANEL_GET_BRIGHTNESS_0,
}
impl PANEL_GET_BRIGHTNESS {}
impl ::std::default::Default for PANEL_GET_BRIGHTNESS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for PANEL_GET_BRIGHTNESS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for PANEL_GET_BRIGHTNESS {}
unsafe impl ::windows::runtime::Abi for PANEL_GET_BRIGHTNESS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub union PANEL_GET_BRIGHTNESS_0 {
    pub Level: u8,
    pub Anonymous: PANEL_GET_BRIGHTNESS_0_0,
}
impl PANEL_GET_BRIGHTNESS_0 {}
impl ::std::default::Default for PANEL_GET_BRIGHTNESS_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for PANEL_GET_BRIGHTNESS_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for PANEL_GET_BRIGHTNESS_0 {}
unsafe impl ::windows::runtime::Abi for PANEL_GET_BRIGHTNESS_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct PANEL_GET_BRIGHTNESS_0_0 {
    pub CurrentInMillinits: u32,
    pub TargetInMillinits: u32,
}
impl PANEL_GET_BRIGHTNESS_0_0 {}
impl ::std::default::Default for PANEL_GET_BRIGHTNESS_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PANEL_GET_BRIGHTNESS_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("CurrentInMillinits", &self.CurrentInMillinits).field("TargetInMillinits", &self.TargetInMillinits).finish()
    }
}
impl ::std::cmp::PartialEq for PANEL_GET_BRIGHTNESS_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.CurrentInMillinits == other.CurrentInMillinits && self.TargetInMillinits == other.TargetInMillinits
    }
}
impl ::std::cmp::Eq for PANEL_GET_BRIGHTNESS_0_0 {}
unsafe impl ::windows::runtime::Abi for PANEL_GET_BRIGHTNESS_0_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct PANEL_QUERY_BRIGHTNESS_CAPS {
    pub Version: BRIGHTNESS_INTERFACE_VERSION,
    pub Anonymous: PANEL_QUERY_BRIGHTNESS_CAPS_0,
}
impl PANEL_QUERY_BRIGHTNESS_CAPS {}
impl ::std::default::Default for PANEL_QUERY_BRIGHTNESS_CAPS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for PANEL_QUERY_BRIGHTNESS_CAPS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for PANEL_QUERY_BRIGHTNESS_CAPS {}
unsafe impl ::windows::runtime::Abi for PANEL_QUERY_BRIGHTNESS_CAPS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub union PANEL_QUERY_BRIGHTNESS_CAPS_0 {
    pub Anonymous: PANEL_QUERY_BRIGHTNESS_CAPS_0_0,
    pub Value: u32,
}
impl PANEL_QUERY_BRIGHTNESS_CAPS_0 {}
impl ::std::default::Default for PANEL_QUERY_BRIGHTNESS_CAPS_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for PANEL_QUERY_BRIGHTNESS_CAPS_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for PANEL_QUERY_BRIGHTNESS_CAPS_0 {}
unsafe impl ::windows::runtime::Abi for PANEL_QUERY_BRIGHTNESS_CAPS_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct PANEL_QUERY_BRIGHTNESS_CAPS_0_0 {
    pub _bitfield: u32,
}
impl PANEL_QUERY_BRIGHTNESS_CAPS_0_0 {}
impl ::std::default::Default for PANEL_QUERY_BRIGHTNESS_CAPS_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PANEL_QUERY_BRIGHTNESS_CAPS_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::std::cmp::PartialEq for PANEL_QUERY_BRIGHTNESS_CAPS_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for PANEL_QUERY_BRIGHTNESS_CAPS_0_0 {}
unsafe impl ::windows::runtime::Abi for PANEL_QUERY_BRIGHTNESS_CAPS_0_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct PANEL_QUERY_BRIGHTNESS_RANGES {
    pub Version: BRIGHTNESS_INTERFACE_VERSION,
    pub Anonymous: PANEL_QUERY_BRIGHTNESS_RANGES_0,
}
impl PANEL_QUERY_BRIGHTNESS_RANGES {}
impl ::std::default::Default for PANEL_QUERY_BRIGHTNESS_RANGES {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for PANEL_QUERY_BRIGHTNESS_RANGES {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for PANEL_QUERY_BRIGHTNESS_RANGES {}
unsafe impl ::windows::runtime::Abi for PANEL_QUERY_BRIGHTNESS_RANGES {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub union PANEL_QUERY_BRIGHTNESS_RANGES_0 {
    pub BrightnessLevel: BRIGHTNESS_LEVEL,
    pub NitRanges: BRIGHTNESS_NIT_RANGES,
}
impl PANEL_QUERY_BRIGHTNESS_RANGES_0 {}
impl ::std::default::Default for PANEL_QUERY_BRIGHTNESS_RANGES_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for PANEL_QUERY_BRIGHTNESS_RANGES_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for PANEL_QUERY_BRIGHTNESS_RANGES_0 {}
unsafe impl ::windows::runtime::Abi for PANEL_QUERY_BRIGHTNESS_RANGES_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct PANEL_SET_BACKLIGHT_OPTIMIZATION {
    pub Level: BACKLIGHT_OPTIMIZATION_LEVEL,
}
impl PANEL_SET_BACKLIGHT_OPTIMIZATION {}
impl ::std::default::Default for PANEL_SET_BACKLIGHT_OPTIMIZATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PANEL_SET_BACKLIGHT_OPTIMIZATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PANEL_SET_BACKLIGHT_OPTIMIZATION").field("Level", &self.Level).finish()
    }
}
impl ::std::cmp::PartialEq for PANEL_SET_BACKLIGHT_OPTIMIZATION {
    fn eq(&self, other: &Self) -> bool {
        self.Level == other.Level
    }
}
impl ::std::cmp::Eq for PANEL_SET_BACKLIGHT_OPTIMIZATION {}
unsafe impl ::windows::runtime::Abi for PANEL_SET_BACKLIGHT_OPTIMIZATION {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct PANEL_SET_BRIGHTNESS {
    pub Version: BRIGHTNESS_INTERFACE_VERSION,
    pub Anonymous: PANEL_SET_BRIGHTNESS_0,
}
impl PANEL_SET_BRIGHTNESS {}
impl ::std::default::Default for PANEL_SET_BRIGHTNESS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for PANEL_SET_BRIGHTNESS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for PANEL_SET_BRIGHTNESS {}
unsafe impl ::windows::runtime::Abi for PANEL_SET_BRIGHTNESS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub union PANEL_SET_BRIGHTNESS_0 {
    pub Level: u8,
    pub Anonymous: PANEL_SET_BRIGHTNESS_0_0,
}
impl PANEL_SET_BRIGHTNESS_0 {}
impl ::std::default::Default for PANEL_SET_BRIGHTNESS_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for PANEL_SET_BRIGHTNESS_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for PANEL_SET_BRIGHTNESS_0 {}
unsafe impl ::windows::runtime::Abi for PANEL_SET_BRIGHTNESS_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct PANEL_SET_BRIGHTNESS_0_0 {
    pub Millinits: u32,
    pub TransitionTimeInMs: u32,
    pub SensorData: PANEL_BRIGHTNESS_SENSOR_DATA,
}
impl PANEL_SET_BRIGHTNESS_0_0 {}
impl ::std::default::Default for PANEL_SET_BRIGHTNESS_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for PANEL_SET_BRIGHTNESS_0_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for PANEL_SET_BRIGHTNESS_0_0 {}
unsafe impl ::windows::runtime::Abi for PANEL_SET_BRIGHTNESS_0_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct PANEL_SET_BRIGHTNESS_STATE {
    pub Anonymous: PANEL_SET_BRIGHTNESS_STATE_0,
}
impl PANEL_SET_BRIGHTNESS_STATE {}
impl ::std::default::Default for PANEL_SET_BRIGHTNESS_STATE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for PANEL_SET_BRIGHTNESS_STATE {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for PANEL_SET_BRIGHTNESS_STATE {}
unsafe impl ::windows::runtime::Abi for PANEL_SET_BRIGHTNESS_STATE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub union PANEL_SET_BRIGHTNESS_STATE_0 {
    pub Anonymous: PANEL_SET_BRIGHTNESS_STATE_0_0,
    pub Value: u32,
}
impl PANEL_SET_BRIGHTNESS_STATE_0 {}
impl ::std::default::Default for PANEL_SET_BRIGHTNESS_STATE_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for PANEL_SET_BRIGHTNESS_STATE_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for PANEL_SET_BRIGHTNESS_STATE_0 {}
unsafe impl ::windows::runtime::Abi for PANEL_SET_BRIGHTNESS_STATE_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct PANEL_SET_BRIGHTNESS_STATE_0_0 {
    pub _bitfield: u32,
}
impl PANEL_SET_BRIGHTNESS_STATE_0_0 {}
impl ::std::default::Default for PANEL_SET_BRIGHTNESS_STATE_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PANEL_SET_BRIGHTNESS_STATE_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::std::cmp::PartialEq for PANEL_SET_BRIGHTNESS_STATE_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for PANEL_SET_BRIGHTNESS_STATE_0_0 {}
unsafe impl ::windows::runtime::Abi for PANEL_SET_BRIGHTNESS_STATE_0_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
pub struct PHYSICAL_MONITOR {
    pub hPhysicalMonitor: super::super::Foundation::HANDLE,
    pub szPhysicalMonitorDescription: [u16; 128],
}
#[cfg(feature = "Win32_Foundation")]
impl PHYSICAL_MONITOR {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for PHYSICAL_MONITOR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for PHYSICAL_MONITOR {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for PHYSICAL_MONITOR {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for PHYSICAL_MONITOR {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const PHYSICAL_MONITOR_DESCRIPTION_SIZE: u32 = 128u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const PLANAR_HC: u32 = 1u32;
pub type PVIDEO_WIN32K_CALLOUT = unsafe extern "system" fn(params: *mut ::std::ffi::c_void);
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RestoreMonitorFactoryColorDefaults<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hmonitor: Param0) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RestoreMonitorFactoryColorDefaults(hmonitor: super::super::Foundation::HANDLE) -> i32;
        }
        ::std::mem::transmute(RestoreMonitorFactoryColorDefaults(hmonitor.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RestoreMonitorFactoryDefaults<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hmonitor: Param0) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RestoreMonitorFactoryDefaults(hmonitor: super::super::Foundation::HANDLE) -> i32;
        }
        ::std::mem::transmute(RestoreMonitorFactoryDefaults(hmonitor.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const SETCONFIGURATION_STATUS_ADDITIONAL: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const SETCONFIGURATION_STATUS_APPLIED: u32 = 0u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const SETCONFIGURATION_STATUS_OVERRIDDEN: u32 = 2u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct SET_ACTIVE_COLOR_PROFILE_NAME {
    pub ColorProfileName: [u16; 1],
}
impl SET_ACTIVE_COLOR_PROFILE_NAME {}
impl ::std::default::Default for SET_ACTIVE_COLOR_PROFILE_NAME {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SET_ACTIVE_COLOR_PROFILE_NAME {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SET_ACTIVE_COLOR_PROFILE_NAME").field("ColorProfileName", &self.ColorProfileName).finish()
    }
}
impl ::std::cmp::PartialEq for SET_ACTIVE_COLOR_PROFILE_NAME {
    fn eq(&self, other: &Self) -> bool {
        self.ColorProfileName == other.ColorProfileName
    }
}
impl ::std::cmp::Eq for SET_ACTIVE_COLOR_PROFILE_NAME {}
unsafe impl ::windows::runtime::Abi for SET_ACTIVE_COLOR_PROFILE_NAME {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct SURFACEALIGNMENT {
    pub Anonymous: SURFACEALIGNMENT_0,
}
impl SURFACEALIGNMENT {}
impl ::std::default::Default for SURFACEALIGNMENT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for SURFACEALIGNMENT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for SURFACEALIGNMENT {}
unsafe impl ::windows::runtime::Abi for SURFACEALIGNMENT {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub union SURFACEALIGNMENT_0 {
    pub Linear: SURFACEALIGNMENT_0_0,
    pub Rectangular: SURFACEALIGNMENT_0_1,
}
impl SURFACEALIGNMENT_0 {}
impl ::std::default::Default for SURFACEALIGNMENT_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for SURFACEALIGNMENT_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for SURFACEALIGNMENT_0 {}
unsafe impl ::windows::runtime::Abi for SURFACEALIGNMENT_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct SURFACEALIGNMENT_0_0 {
    pub dwStartAlignment: u32,
    pub dwPitchAlignment: u32,
    pub dwFlags: u32,
    pub dwReserved2: u32,
}
impl SURFACEALIGNMENT_0_0 {}
impl ::std::default::Default for SURFACEALIGNMENT_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SURFACEALIGNMENT_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Linear_e__Struct").field("dwStartAlignment", &self.dwStartAlignment).field("dwPitchAlignment", &self.dwPitchAlignment).field("dwFlags", &self.dwFlags).field("dwReserved2", &self.dwReserved2).finish()
    }
}
impl ::std::cmp::PartialEq for SURFACEALIGNMENT_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.dwStartAlignment == other.dwStartAlignment && self.dwPitchAlignment == other.dwPitchAlignment && self.dwFlags == other.dwFlags && self.dwReserved2 == other.dwReserved2
    }
}
impl ::std::cmp::Eq for SURFACEALIGNMENT_0_0 {}
unsafe impl ::windows::runtime::Abi for SURFACEALIGNMENT_0_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct SURFACEALIGNMENT_0_1 {
    pub dwXAlignment: u32,
    pub dwYAlignment: u32,
    pub dwFlags: u32,
    pub dwReserved2: u32,
}
impl SURFACEALIGNMENT_0_1 {}
impl ::std::default::Default for SURFACEALIGNMENT_0_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SURFACEALIGNMENT_0_1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Rectangular_e__Struct").field("dwXAlignment", &self.dwXAlignment).field("dwYAlignment", &self.dwYAlignment).field("dwFlags", &self.dwFlags).field("dwReserved2", &self.dwReserved2).finish()
    }
}
impl ::std::cmp::PartialEq for SURFACEALIGNMENT_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.dwXAlignment == other.dwXAlignment && self.dwYAlignment == other.dwYAlignment && self.dwFlags == other.dwFlags && self.dwReserved2 == other.dwReserved2
    }
}
impl ::std::cmp::Eq for SURFACEALIGNMENT_0_1 {}
unsafe impl ::windows::runtime::Abi for SURFACEALIGNMENT_0_1 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const SURFACEALIGN_DISCARDABLE: i32 = 1i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const S_INIT: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SaveCurrentMonitorSettings<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hmonitor: Param0) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SaveCurrentMonitorSettings(hmonitor: super::super::Foundation::HANDLE) -> i32;
        }
        ::std::mem::transmute(SaveCurrentMonitorSettings(hmonitor.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SaveCurrentSettings<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hmonitor: Param0) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SaveCurrentSettings(hmonitor: super::super::Foundation::HANDLE) -> i32;
        }
        ::std::mem::transmute(SaveCurrentSettings(hmonitor.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetDisplayAutoRotationPreferences(orientation: ORIENTATION_PREFERENCE) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetDisplayAutoRotationPreferences(orientation: ORIENTATION_PREFERENCE) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetDisplayAutoRotationPreferences(::std::mem::transmute(orientation)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetMonitorBrightness<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hmonitor: Param0, dwnewbrightness: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetMonitorBrightness(hmonitor: super::super::Foundation::HANDLE, dwnewbrightness: u32) -> i32;
        }
        ::std::mem::transmute(SetMonitorBrightness(hmonitor.into_param().abi(), ::std::mem::transmute(dwnewbrightness)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetMonitorColorTemperature<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hmonitor: Param0, ctcurrentcolortemperature: MC_COLOR_TEMPERATURE) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetMonitorColorTemperature(hmonitor: super::super::Foundation::HANDLE, ctcurrentcolortemperature: MC_COLOR_TEMPERATURE) -> i32;
        }
        ::std::mem::transmute(SetMonitorColorTemperature(hmonitor.into_param().abi(), ::std::mem::transmute(ctcurrentcolortemperature)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetMonitorContrast<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hmonitor: Param0, dwnewcontrast: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetMonitorContrast(hmonitor: super::super::Foundation::HANDLE, dwnewcontrast: u32) -> i32;
        }
        ::std::mem::transmute(SetMonitorContrast(hmonitor.into_param().abi(), ::std::mem::transmute(dwnewcontrast)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetMonitorDisplayAreaPosition<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hmonitor: Param0, ptpositiontype: MC_POSITION_TYPE, dwnewposition: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetMonitorDisplayAreaPosition(hmonitor: super::super::Foundation::HANDLE, ptpositiontype: MC_POSITION_TYPE, dwnewposition: u32) -> i32;
        }
        ::std::mem::transmute(SetMonitorDisplayAreaPosition(hmonitor.into_param().abi(), ::std::mem::transmute(ptpositiontype), ::std::mem::transmute(dwnewposition)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetMonitorDisplayAreaSize<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hmonitor: Param0, stsizetype: MC_SIZE_TYPE, dwnewdisplayareawidthorheight: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetMonitorDisplayAreaSize(hmonitor: super::super::Foundation::HANDLE, stsizetype: MC_SIZE_TYPE, dwnewdisplayareawidthorheight: u32) -> i32;
        }
        ::std::mem::transmute(SetMonitorDisplayAreaSize(hmonitor.into_param().abi(), ::std::mem::transmute(stsizetype), ::std::mem::transmute(dwnewdisplayareawidthorheight)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetMonitorRedGreenOrBlueDrive<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hmonitor: Param0, dtdrivetype: MC_DRIVE_TYPE, dwnewdrive: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetMonitorRedGreenOrBlueDrive(hmonitor: super::super::Foundation::HANDLE, dtdrivetype: MC_DRIVE_TYPE, dwnewdrive: u32) -> i32;
        }
        ::std::mem::transmute(SetMonitorRedGreenOrBlueDrive(hmonitor.into_param().abi(), ::std::mem::transmute(dtdrivetype), ::std::mem::transmute(dwnewdrive)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetMonitorRedGreenOrBlueGain<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hmonitor: Param0, gtgaintype: MC_GAIN_TYPE, dwnewgain: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetMonitorRedGreenOrBlueGain(hmonitor: super::super::Foundation::HANDLE, gtgaintype: MC_GAIN_TYPE, dwnewgain: u32) -> i32;
        }
        ::std::mem::transmute(SetMonitorRedGreenOrBlueGain(hmonitor.into_param().abi(), ::std::mem::transmute(gtgaintype), ::std::mem::transmute(dwnewgain)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetVCPFeature<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hmonitor: Param0, bvcpcode: u8, dwnewvalue: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetVCPFeature(hmonitor: super::super::Foundation::HANDLE, bvcpcode: u8, dwnewvalue: u32) -> i32;
        }
        ::std::mem::transmute(SetVCPFeature(hmonitor.into_param().abi(), ::std::mem::transmute(bvcpcode), ::std::mem::transmute(dwnewvalue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct Sources {
    pub sourceId: u32,
    pub numTargets: i32,
    pub aTargets: [u32; 1],
}
impl Sources {}
impl ::std::default::Default for Sources {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for Sources {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("Sources").field("sourceId", &self.sourceId).field("numTargets", &self.numTargets).field("aTargets", &self.aTargets).finish()
    }
}
impl ::std::cmp::PartialEq for Sources {
    fn eq(&self, other: &Self) -> bool {
        self.sourceId == other.sourceId && self.numTargets == other.numTargets && self.aTargets == other.aTargets
    }
}
impl ::std::cmp::Eq for Sources {}
unsafe impl ::windows::runtime::Abi for Sources {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
pub struct VGA_CHAR {
    pub Char: super::super::Foundation::CHAR,
    pub Attributes: super::super::Foundation::CHAR,
}
#[cfg(feature = "Win32_Foundation")]
impl VGA_CHAR {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for VGA_CHAR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for VGA_CHAR {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VGA_CHAR").field("Char", &self.Char).field("Attributes", &self.Attributes).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for VGA_CHAR {
    fn eq(&self, other: &Self) -> bool {
        self.Char == other.Char && self.Attributes == other.Attributes
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for VGA_CHAR {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for VGA_CHAR {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct VIDEO_BANK_SELECT {
    pub Length: u32,
    pub Size: u32,
    pub BankingFlags: u32,
    pub BankingType: u32,
    pub PlanarHCBankingType: u32,
    pub BitmapWidthInBytes: u32,
    pub BitmapSize: u32,
    pub Granularity: u32,
    pub PlanarHCGranularity: u32,
    pub CodeOffset: u32,
    pub PlanarHCBankCodeOffset: u32,
    pub PlanarHCEnableCodeOffset: u32,
    pub PlanarHCDisableCodeOffset: u32,
}
impl VIDEO_BANK_SELECT {}
impl ::std::default::Default for VIDEO_BANK_SELECT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VIDEO_BANK_SELECT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VIDEO_BANK_SELECT")
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
impl ::std::cmp::PartialEq for VIDEO_BANK_SELECT {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length
            && self.Size == other.Size
            && self.BankingFlags == other.BankingFlags
            && self.BankingType == other.BankingType
            && self.PlanarHCBankingType == other.PlanarHCBankingType
            && self.BitmapWidthInBytes == other.BitmapWidthInBytes
            && self.BitmapSize == other.BitmapSize
            && self.Granularity == other.Granularity
            && self.PlanarHCGranularity == other.PlanarHCGranularity
            && self.CodeOffset == other.CodeOffset
            && self.PlanarHCBankCodeOffset == other.PlanarHCBankCodeOffset
            && self.PlanarHCEnableCodeOffset == other.PlanarHCEnableCodeOffset
            && self.PlanarHCDisableCodeOffset == other.PlanarHCDisableCodeOffset
    }
}
impl ::std::cmp::Eq for VIDEO_BANK_SELECT {}
unsafe impl ::windows::runtime::Abi for VIDEO_BANK_SELECT {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct VIDEO_BANK_TYPE(pub i32);
pub const VideoNotBanked: VIDEO_BANK_TYPE = VIDEO_BANK_TYPE(0i32);
pub const VideoBanked1RW: VIDEO_BANK_TYPE = VIDEO_BANK_TYPE(1i32);
pub const VideoBanked1R1W: VIDEO_BANK_TYPE = VIDEO_BANK_TYPE(2i32);
pub const VideoBanked2RW: VIDEO_BANK_TYPE = VIDEO_BANK_TYPE(3i32);
pub const NumVideoBankTypes: VIDEO_BANK_TYPE = VIDEO_BANK_TYPE(4i32);
impl ::std::convert::From<i32> for VIDEO_BANK_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VIDEO_BANK_TYPE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
pub struct VIDEO_BRIGHTNESS_POLICY {
    pub DefaultToBiosPolicy: super::super::Foundation::BOOLEAN,
    pub LevelCount: u8,
    pub Level: [VIDEO_BRIGHTNESS_POLICY_0; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl VIDEO_BRIGHTNESS_POLICY {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for VIDEO_BRIGHTNESS_POLICY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for VIDEO_BRIGHTNESS_POLICY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VIDEO_BRIGHTNESS_POLICY").field("DefaultToBiosPolicy", &self.DefaultToBiosPolicy).field("LevelCount", &self.LevelCount).field("Level", &self.Level).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for VIDEO_BRIGHTNESS_POLICY {
    fn eq(&self, other: &Self) -> bool {
        self.DefaultToBiosPolicy == other.DefaultToBiosPolicy && self.LevelCount == other.LevelCount && self.Level == other.Level
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for VIDEO_BRIGHTNESS_POLICY {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for VIDEO_BRIGHTNESS_POLICY {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct VIDEO_BRIGHTNESS_POLICY_0 {
    pub BatteryLevel: u8,
    pub Brightness: u8,
}
impl VIDEO_BRIGHTNESS_POLICY_0 {}
impl ::std::default::Default for VIDEO_BRIGHTNESS_POLICY_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VIDEO_BRIGHTNESS_POLICY_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("BatteryLevel", &self.BatteryLevel).field("Brightness", &self.Brightness).finish()
    }
}
impl ::std::cmp::PartialEq for VIDEO_BRIGHTNESS_POLICY_0 {
    fn eq(&self, other: &Self) -> bool {
        self.BatteryLevel == other.BatteryLevel && self.Brightness == other.Brightness
    }
}
impl ::std::cmp::Eq for VIDEO_BRIGHTNESS_POLICY_0 {}
unsafe impl ::windows::runtime::Abi for VIDEO_BRIGHTNESS_POLICY_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct VIDEO_CLUT {
    pub NumEntries: u16,
    pub FirstEntry: u16,
    pub LookupTable: [VIDEO_CLUT_0; 1],
}
impl VIDEO_CLUT {}
impl ::std::default::Default for VIDEO_CLUT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for VIDEO_CLUT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for VIDEO_CLUT {}
unsafe impl ::windows::runtime::Abi for VIDEO_CLUT {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub union VIDEO_CLUT_0 {
    pub RgbArray: VIDEO_CLUTDATA,
    pub RgbLong: u32,
}
impl VIDEO_CLUT_0 {}
impl ::std::default::Default for VIDEO_CLUT_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for VIDEO_CLUT_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for VIDEO_CLUT_0 {}
unsafe impl ::windows::runtime::Abi for VIDEO_CLUT_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct VIDEO_CLUTDATA {
    pub Red: u8,
    pub Green: u8,
    pub Blue: u8,
    pub Unused: u8,
}
impl VIDEO_CLUTDATA {}
impl ::std::default::Default for VIDEO_CLUTDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VIDEO_CLUTDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VIDEO_CLUTDATA").field("Red", &self.Red).field("Green", &self.Green).field("Blue", &self.Blue).field("Unused", &self.Unused).finish()
    }
}
impl ::std::cmp::PartialEq for VIDEO_CLUTDATA {
    fn eq(&self, other: &Self) -> bool {
        self.Red == other.Red && self.Green == other.Green && self.Blue == other.Blue && self.Unused == other.Unused
    }
}
impl ::std::cmp::Eq for VIDEO_CLUTDATA {}
unsafe impl ::windows::runtime::Abi for VIDEO_CLUTDATA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct VIDEO_COLOR_CAPABILITIES {
    pub Length: u32,
    pub AttributeFlags: u32,
    pub RedPhosphoreDecay: i32,
    pub GreenPhosphoreDecay: i32,
    pub BluePhosphoreDecay: i32,
    pub WhiteChromaticity_x: i32,
    pub WhiteChromaticity_y: i32,
    pub WhiteChromaticity_Y: i32,
    pub RedChromaticity_x: i32,
    pub RedChromaticity_y: i32,
    pub GreenChromaticity_x: i32,
    pub GreenChromaticity_y: i32,
    pub BlueChromaticity_x: i32,
    pub BlueChromaticity_y: i32,
    pub WhiteGamma: i32,
    pub RedGamma: i32,
    pub GreenGamma: i32,
    pub BlueGamma: i32,
}
impl VIDEO_COLOR_CAPABILITIES {}
impl ::std::default::Default for VIDEO_COLOR_CAPABILITIES {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VIDEO_COLOR_CAPABILITIES {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VIDEO_COLOR_CAPABILITIES")
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
impl ::std::cmp::PartialEq for VIDEO_COLOR_CAPABILITIES {
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
impl ::std::cmp::Eq for VIDEO_COLOR_CAPABILITIES {}
unsafe impl ::windows::runtime::Abi for VIDEO_COLOR_CAPABILITIES {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct VIDEO_COLOR_LUT_DATA {
    pub Length: u32,
    pub LutDataFormat: u32,
    pub LutData: [u8; 1],
}
impl VIDEO_COLOR_LUT_DATA {}
impl ::std::default::Default for VIDEO_COLOR_LUT_DATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VIDEO_COLOR_LUT_DATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VIDEO_COLOR_LUT_DATA").field("Length", &self.Length).field("LutDataFormat", &self.LutDataFormat).field("LutData", &self.LutData).finish()
    }
}
impl ::std::cmp::PartialEq for VIDEO_COLOR_LUT_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.LutDataFormat == other.LutDataFormat && self.LutData == other.LutData
    }
}
impl ::std::cmp::Eq for VIDEO_COLOR_LUT_DATA {}
unsafe impl ::windows::runtime::Abi for VIDEO_COLOR_LUT_DATA {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const VIDEO_COLOR_LUT_DATA_FORMAT_PRIVATEFORMAT: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const VIDEO_COLOR_LUT_DATA_FORMAT_RGB256WORDS: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct VIDEO_CURSOR_ATTRIBUTES {
    pub Width: u16,
    pub Height: u16,
    pub Column: i16,
    pub Row: i16,
    pub Rate: u8,
    pub Enable: u8,
}
impl VIDEO_CURSOR_ATTRIBUTES {}
impl ::std::default::Default for VIDEO_CURSOR_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VIDEO_CURSOR_ATTRIBUTES {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VIDEO_CURSOR_ATTRIBUTES").field("Width", &self.Width).field("Height", &self.Height).field("Column", &self.Column).field("Row", &self.Row).field("Rate", &self.Rate).field("Enable", &self.Enable).finish()
    }
}
impl ::std::cmp::PartialEq for VIDEO_CURSOR_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        self.Width == other.Width && self.Height == other.Height && self.Column == other.Column && self.Row == other.Row && self.Rate == other.Rate && self.Enable == other.Enable
    }
}
impl ::std::cmp::Eq for VIDEO_CURSOR_ATTRIBUTES {}
unsafe impl ::windows::runtime::Abi for VIDEO_CURSOR_ATTRIBUTES {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct VIDEO_CURSOR_POSITION {
    pub Column: i16,
    pub Row: i16,
}
impl VIDEO_CURSOR_POSITION {}
impl ::std::default::Default for VIDEO_CURSOR_POSITION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VIDEO_CURSOR_POSITION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VIDEO_CURSOR_POSITION").field("Column", &self.Column).field("Row", &self.Row).finish()
    }
}
impl ::std::cmp::PartialEq for VIDEO_CURSOR_POSITION {
    fn eq(&self, other: &Self) -> bool {
        self.Column == other.Column && self.Row == other.Row
    }
}
impl ::std::cmp::Eq for VIDEO_CURSOR_POSITION {}
unsafe impl ::windows::runtime::Abi for VIDEO_CURSOR_POSITION {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const VIDEO_DEVICE_COLOR: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct VIDEO_DEVICE_SESSION_STATUS {
    pub bEnable: u32,
    pub bSuccess: u32,
}
impl VIDEO_DEVICE_SESSION_STATUS {}
impl ::std::default::Default for VIDEO_DEVICE_SESSION_STATUS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VIDEO_DEVICE_SESSION_STATUS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VIDEO_DEVICE_SESSION_STATUS").field("bEnable", &self.bEnable).field("bSuccess", &self.bSuccess).finish()
    }
}
impl ::std::cmp::PartialEq for VIDEO_DEVICE_SESSION_STATUS {
    fn eq(&self, other: &Self) -> bool {
        self.bEnable == other.bEnable && self.bSuccess == other.bSuccess
    }
}
impl ::std::cmp::Eq for VIDEO_DEVICE_SESSION_STATUS {}
unsafe impl ::windows::runtime::Abi for VIDEO_DEVICE_SESSION_STATUS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const VIDEO_DUALVIEW_PRIMARY: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const VIDEO_DUALVIEW_REMOVABLE: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const VIDEO_DUALVIEW_SECONDARY: u32 = 1073741824u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const VIDEO_DUALVIEW_WDDM_VGA: u32 = 536870912u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct VIDEO_HARDWARE_STATE {
    pub StateHeader: *mut VIDEO_HARDWARE_STATE_HEADER,
    pub StateLength: u32,
}
impl VIDEO_HARDWARE_STATE {}
impl ::std::default::Default for VIDEO_HARDWARE_STATE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VIDEO_HARDWARE_STATE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VIDEO_HARDWARE_STATE").field("StateHeader", &self.StateHeader).field("StateLength", &self.StateLength).finish()
    }
}
impl ::std::cmp::PartialEq for VIDEO_HARDWARE_STATE {
    fn eq(&self, other: &Self) -> bool {
        self.StateHeader == other.StateHeader && self.StateLength == other.StateLength
    }
}
impl ::std::cmp::Eq for VIDEO_HARDWARE_STATE {}
unsafe impl ::windows::runtime::Abi for VIDEO_HARDWARE_STATE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct VIDEO_HARDWARE_STATE_HEADER {
    pub Length: u32,
    pub PortValue: [u8; 48],
    pub AttribIndexDataState: u32,
    pub BasicSequencerOffset: u32,
    pub BasicCrtContOffset: u32,
    pub BasicGraphContOffset: u32,
    pub BasicAttribContOffset: u32,
    pub BasicDacOffset: u32,
    pub BasicLatchesOffset: u32,
    pub ExtendedSequencerOffset: u32,
    pub ExtendedCrtContOffset: u32,
    pub ExtendedGraphContOffset: u32,
    pub ExtendedAttribContOffset: u32,
    pub ExtendedDacOffset: u32,
    pub ExtendedValidatorStateOffset: u32,
    pub ExtendedMiscDataOffset: u32,
    pub PlaneLength: u32,
    pub Plane1Offset: u32,
    pub Plane2Offset: u32,
    pub Plane3Offset: u32,
    pub Plane4Offset: u32,
    pub VGAStateFlags: u32,
    pub DIBOffset: u32,
    pub DIBBitsPerPixel: u32,
    pub DIBXResolution: u32,
    pub DIBYResolution: u32,
    pub DIBXlatOffset: u32,
    pub DIBXlatLength: u32,
    pub VesaInfoOffset: u32,
    pub FrameBufferData: *mut ::std::ffi::c_void,
}
impl VIDEO_HARDWARE_STATE_HEADER {}
impl ::std::default::Default for VIDEO_HARDWARE_STATE_HEADER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VIDEO_HARDWARE_STATE_HEADER {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VIDEO_HARDWARE_STATE_HEADER")
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
impl ::std::cmp::PartialEq for VIDEO_HARDWARE_STATE_HEADER {
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
impl ::std::cmp::Eq for VIDEO_HARDWARE_STATE_HEADER {}
unsafe impl ::windows::runtime::Abi for VIDEO_HARDWARE_STATE_HEADER {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct VIDEO_LOAD_FONT_INFORMATION {
    pub WidthInPixels: u16,
    pub HeightInPixels: u16,
    pub FontSize: u32,
    pub Font: [u8; 1],
}
impl VIDEO_LOAD_FONT_INFORMATION {}
impl ::std::default::Default for VIDEO_LOAD_FONT_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VIDEO_LOAD_FONT_INFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VIDEO_LOAD_FONT_INFORMATION").field("WidthInPixels", &self.WidthInPixels).field("HeightInPixels", &self.HeightInPixels).field("FontSize", &self.FontSize).field("Font", &self.Font).finish()
    }
}
impl ::std::cmp::PartialEq for VIDEO_LOAD_FONT_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.WidthInPixels == other.WidthInPixels && self.HeightInPixels == other.HeightInPixels && self.FontSize == other.FontSize && self.Font == other.Font
    }
}
impl ::std::cmp::Eq for VIDEO_LOAD_FONT_INFORMATION {}
unsafe impl ::windows::runtime::Abi for VIDEO_LOAD_FONT_INFORMATION {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct VIDEO_LUT_RGB256WORDS {
    pub Red: [u16; 256],
    pub Green: [u16; 256],
    pub Blue: [u16; 256],
}
impl VIDEO_LUT_RGB256WORDS {}
impl ::std::default::Default for VIDEO_LUT_RGB256WORDS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VIDEO_LUT_RGB256WORDS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VIDEO_LUT_RGB256WORDS").field("Red", &self.Red).field("Green", &self.Green).field("Blue", &self.Blue).finish()
    }
}
impl ::std::cmp::PartialEq for VIDEO_LUT_RGB256WORDS {
    fn eq(&self, other: &Self) -> bool {
        self.Red == other.Red && self.Green == other.Green && self.Blue == other.Blue
    }
}
impl ::std::cmp::Eq for VIDEO_LUT_RGB256WORDS {}
unsafe impl ::windows::runtime::Abi for VIDEO_LUT_RGB256WORDS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const VIDEO_MAX_REASON: u32 = 9u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct VIDEO_MEMORY {
    pub RequestedVirtualAddress: *mut ::std::ffi::c_void,
}
impl VIDEO_MEMORY {}
impl ::std::default::Default for VIDEO_MEMORY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VIDEO_MEMORY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VIDEO_MEMORY").field("RequestedVirtualAddress", &self.RequestedVirtualAddress).finish()
    }
}
impl ::std::cmp::PartialEq for VIDEO_MEMORY {
    fn eq(&self, other: &Self) -> bool {
        self.RequestedVirtualAddress == other.RequestedVirtualAddress
    }
}
impl ::std::cmp::Eq for VIDEO_MEMORY {}
unsafe impl ::windows::runtime::Abi for VIDEO_MEMORY {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct VIDEO_MEMORY_INFORMATION {
    pub VideoRamBase: *mut ::std::ffi::c_void,
    pub VideoRamLength: u32,
    pub FrameBufferBase: *mut ::std::ffi::c_void,
    pub FrameBufferLength: u32,
}
impl VIDEO_MEMORY_INFORMATION {}
impl ::std::default::Default for VIDEO_MEMORY_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VIDEO_MEMORY_INFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VIDEO_MEMORY_INFORMATION").field("VideoRamBase", &self.VideoRamBase).field("VideoRamLength", &self.VideoRamLength).field("FrameBufferBase", &self.FrameBufferBase).field("FrameBufferLength", &self.FrameBufferLength).finish()
    }
}
impl ::std::cmp::PartialEq for VIDEO_MEMORY_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.VideoRamBase == other.VideoRamBase && self.VideoRamLength == other.VideoRamLength && self.FrameBufferBase == other.FrameBufferBase && self.FrameBufferLength == other.FrameBufferLength
    }
}
impl ::std::cmp::Eq for VIDEO_MEMORY_INFORMATION {}
unsafe impl ::windows::runtime::Abi for VIDEO_MEMORY_INFORMATION {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct VIDEO_MODE {
    pub RequestedMode: u32,
}
impl VIDEO_MODE {}
impl ::std::default::Default for VIDEO_MODE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VIDEO_MODE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VIDEO_MODE").field("RequestedMode", &self.RequestedMode).finish()
    }
}
impl ::std::cmp::PartialEq for VIDEO_MODE {
    fn eq(&self, other: &Self) -> bool {
        self.RequestedMode == other.RequestedMode
    }
}
impl ::std::cmp::Eq for VIDEO_MODE {}
unsafe impl ::windows::runtime::Abi for VIDEO_MODE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const VIDEO_MODE_ANIMATE_START: u32 = 8u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const VIDEO_MODE_ANIMATE_UPDATE: u32 = 16u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const VIDEO_MODE_ASYNC_POINTER: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const VIDEO_MODE_BANKED: u32 = 128u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const VIDEO_MODE_COLOR: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const VIDEO_MODE_COLOR_POINTER: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const VIDEO_MODE_GRAPHICS: u32 = 2u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct VIDEO_MODE_INFORMATION {
    pub Length: u32,
    pub ModeIndex: u32,
    pub VisScreenWidth: u32,
    pub VisScreenHeight: u32,
    pub ScreenStride: u32,
    pub NumberOfPlanes: u32,
    pub BitsPerPlane: u32,
    pub Frequency: u32,
    pub XMillimeter: u32,
    pub YMillimeter: u32,
    pub NumberRedBits: u32,
    pub NumberGreenBits: u32,
    pub NumberBlueBits: u32,
    pub RedMask: u32,
    pub GreenMask: u32,
    pub BlueMask: u32,
    pub AttributeFlags: u32,
    pub VideoMemoryBitmapWidth: u32,
    pub VideoMemoryBitmapHeight: u32,
    pub DriverSpecificAttributeFlags: u32,
}
impl VIDEO_MODE_INFORMATION {}
impl ::std::default::Default for VIDEO_MODE_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VIDEO_MODE_INFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VIDEO_MODE_INFORMATION")
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
impl ::std::cmp::PartialEq for VIDEO_MODE_INFORMATION {
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
impl ::std::cmp::Eq for VIDEO_MODE_INFORMATION {}
unsafe impl ::windows::runtime::Abi for VIDEO_MODE_INFORMATION {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const VIDEO_MODE_INTERLACED: u32 = 16u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const VIDEO_MODE_LINEAR: u32 = 256u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const VIDEO_MODE_MANAGED_PALETTE: u32 = 8u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const VIDEO_MODE_MAP_MEM_LINEAR: u32 = 1073741824u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const VIDEO_MODE_MONO_POINTER: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const VIDEO_MODE_NO_64_BIT_ACCESS: u32 = 64u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const VIDEO_MODE_NO_OFF_SCREEN: u32 = 32u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const VIDEO_MODE_NO_ZERO_MEMORY: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const VIDEO_MODE_PALETTE_DRIVEN: u32 = 4u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct VIDEO_MONITOR_DESCRIPTOR {
    pub DescriptorSize: u32,
    pub Descriptor: [u8; 1],
}
impl VIDEO_MONITOR_DESCRIPTOR {}
impl ::std::default::Default for VIDEO_MONITOR_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VIDEO_MONITOR_DESCRIPTOR {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VIDEO_MONITOR_DESCRIPTOR").field("DescriptorSize", &self.DescriptorSize).field("Descriptor", &self.Descriptor).finish()
    }
}
impl ::std::cmp::PartialEq for VIDEO_MONITOR_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.DescriptorSize == other.DescriptorSize && self.Descriptor == other.Descriptor
    }
}
impl ::std::cmp::Eq for VIDEO_MONITOR_DESCRIPTOR {}
unsafe impl ::windows::runtime::Abi for VIDEO_MONITOR_DESCRIPTOR {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct VIDEO_NUM_MODES {
    pub NumModes: u32,
    pub ModeInformationLength: u32,
}
impl VIDEO_NUM_MODES {}
impl ::std::default::Default for VIDEO_NUM_MODES {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VIDEO_NUM_MODES {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VIDEO_NUM_MODES").field("NumModes", &self.NumModes).field("ModeInformationLength", &self.ModeInformationLength).finish()
    }
}
impl ::std::cmp::PartialEq for VIDEO_NUM_MODES {
    fn eq(&self, other: &Self) -> bool {
        self.NumModes == other.NumModes && self.ModeInformationLength == other.ModeInformationLength
    }
}
impl ::std::cmp::Eq for VIDEO_NUM_MODES {}
unsafe impl ::windows::runtime::Abi for VIDEO_NUM_MODES {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const VIDEO_OPTIONAL_GAMMET_TABLE: u32 = 2u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct VIDEO_PALETTE_DATA {
    pub NumEntries: u16,
    pub FirstEntry: u16,
    pub Colors: [u16; 1],
}
impl VIDEO_PALETTE_DATA {}
impl ::std::default::Default for VIDEO_PALETTE_DATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VIDEO_PALETTE_DATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VIDEO_PALETTE_DATA").field("NumEntries", &self.NumEntries).field("FirstEntry", &self.FirstEntry).field("Colors", &self.Colors).finish()
    }
}
impl ::std::cmp::PartialEq for VIDEO_PALETTE_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.NumEntries == other.NumEntries && self.FirstEntry == other.FirstEntry && self.Colors == other.Colors
    }
}
impl ::std::cmp::Eq for VIDEO_PALETTE_DATA {}
unsafe impl ::windows::runtime::Abi for VIDEO_PALETTE_DATA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct VIDEO_PERFORMANCE_COUNTER {
    pub NbOfAllocationEvicted: [u64; 10],
    pub NbOfAllocationMarked: [u64; 10],
    pub NbOfAllocationRestored: [u64; 10],
    pub KBytesEvicted: [u64; 10],
    pub KBytesMarked: [u64; 10],
    pub KBytesRestored: [u64; 10],
    pub NbProcessCommited: u64,
    pub NbAllocationCommited: u64,
    pub NbAllocationMarked: u64,
    pub KBytesAllocated: u64,
    pub KBytesAvailable: u64,
    pub KBytesCurMarked: u64,
    pub Reference: u64,
    pub Unreference: u64,
    pub TrueReference: u64,
    pub NbOfPageIn: u64,
    pub KBytesPageIn: u64,
    pub NbOfPageOut: u64,
    pub KBytesPageOut: u64,
    pub NbOfRotateOut: u64,
    pub KBytesRotateOut: u64,
}
impl VIDEO_PERFORMANCE_COUNTER {}
impl ::std::default::Default for VIDEO_PERFORMANCE_COUNTER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VIDEO_PERFORMANCE_COUNTER {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VIDEO_PERFORMANCE_COUNTER")
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
impl ::std::cmp::PartialEq for VIDEO_PERFORMANCE_COUNTER {
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
impl ::std::cmp::Eq for VIDEO_PERFORMANCE_COUNTER {}
unsafe impl ::windows::runtime::Abi for VIDEO_PERFORMANCE_COUNTER {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct VIDEO_POINTER_ATTRIBUTES {
    pub Flags: u32,
    pub Width: u32,
    pub Height: u32,
    pub WidthInBytes: u32,
    pub Enable: u32,
    pub Column: i16,
    pub Row: i16,
    pub Pixels: [u8; 1],
}
impl VIDEO_POINTER_ATTRIBUTES {}
impl ::std::default::Default for VIDEO_POINTER_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VIDEO_POINTER_ATTRIBUTES {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VIDEO_POINTER_ATTRIBUTES").field("Flags", &self.Flags).field("Width", &self.Width).field("Height", &self.Height).field("WidthInBytes", &self.WidthInBytes).field("Enable", &self.Enable).field("Column", &self.Column).field("Row", &self.Row).field("Pixels", &self.Pixels).finish()
    }
}
impl ::std::cmp::PartialEq for VIDEO_POINTER_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.Width == other.Width && self.Height == other.Height && self.WidthInBytes == other.WidthInBytes && self.Enable == other.Enable && self.Column == other.Column && self.Row == other.Row && self.Pixels == other.Pixels
    }
}
impl ::std::cmp::Eq for VIDEO_POINTER_ATTRIBUTES {}
unsafe impl ::windows::runtime::Abi for VIDEO_POINTER_ATTRIBUTES {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct VIDEO_POINTER_CAPABILITIES {
    pub Flags: u32,
    pub MaxWidth: u32,
    pub MaxHeight: u32,
    pub HWPtrBitmapStart: u32,
    pub HWPtrBitmapEnd: u32,
}
impl VIDEO_POINTER_CAPABILITIES {}
impl ::std::default::Default for VIDEO_POINTER_CAPABILITIES {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VIDEO_POINTER_CAPABILITIES {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VIDEO_POINTER_CAPABILITIES").field("Flags", &self.Flags).field("MaxWidth", &self.MaxWidth).field("MaxHeight", &self.MaxHeight).field("HWPtrBitmapStart", &self.HWPtrBitmapStart).field("HWPtrBitmapEnd", &self.HWPtrBitmapEnd).finish()
    }
}
impl ::std::cmp::PartialEq for VIDEO_POINTER_CAPABILITIES {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.MaxWidth == other.MaxWidth && self.MaxHeight == other.MaxHeight && self.HWPtrBitmapStart == other.HWPtrBitmapStart && self.HWPtrBitmapEnd == other.HWPtrBitmapEnd
    }
}
impl ::std::cmp::Eq for VIDEO_POINTER_CAPABILITIES {}
unsafe impl ::windows::runtime::Abi for VIDEO_POINTER_CAPABILITIES {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct VIDEO_POINTER_POSITION {
    pub Column: i16,
    pub Row: i16,
}
impl VIDEO_POINTER_POSITION {}
impl ::std::default::Default for VIDEO_POINTER_POSITION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VIDEO_POINTER_POSITION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VIDEO_POINTER_POSITION").field("Column", &self.Column).field("Row", &self.Row).finish()
    }
}
impl ::std::cmp::PartialEq for VIDEO_POINTER_POSITION {
    fn eq(&self, other: &Self) -> bool {
        self.Column == other.Column && self.Row == other.Row
    }
}
impl ::std::cmp::Eq for VIDEO_POINTER_POSITION {}
unsafe impl ::windows::runtime::Abi for VIDEO_POINTER_POSITION {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct VIDEO_POWER_MANAGEMENT {
    pub Length: u32,
    pub DPMSVersion: u32,
    pub PowerState: u32,
}
impl VIDEO_POWER_MANAGEMENT {}
impl ::std::default::Default for VIDEO_POWER_MANAGEMENT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VIDEO_POWER_MANAGEMENT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VIDEO_POWER_MANAGEMENT").field("Length", &self.Length).field("DPMSVersion", &self.DPMSVersion).field("PowerState", &self.PowerState).finish()
    }
}
impl ::std::cmp::PartialEq for VIDEO_POWER_MANAGEMENT {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.DPMSVersion == other.DPMSVersion && self.PowerState == other.PowerState
    }
}
impl ::std::cmp::Eq for VIDEO_POWER_MANAGEMENT {}
unsafe impl ::windows::runtime::Abi for VIDEO_POWER_MANAGEMENT {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct VIDEO_POWER_STATE(pub i32);
pub const VideoPowerUnspecified: VIDEO_POWER_STATE = VIDEO_POWER_STATE(0i32);
pub const VideoPowerOn: VIDEO_POWER_STATE = VIDEO_POWER_STATE(1i32);
pub const VideoPowerStandBy: VIDEO_POWER_STATE = VIDEO_POWER_STATE(2i32);
pub const VideoPowerSuspend: VIDEO_POWER_STATE = VIDEO_POWER_STATE(3i32);
pub const VideoPowerOff: VIDEO_POWER_STATE = VIDEO_POWER_STATE(4i32);
pub const VideoPowerHibernate: VIDEO_POWER_STATE = VIDEO_POWER_STATE(5i32);
pub const VideoPowerShutdown: VIDEO_POWER_STATE = VIDEO_POWER_STATE(6i32);
pub const VideoPowerMaximum: VIDEO_POWER_STATE = VIDEO_POWER_STATE(7i32);
impl ::std::convert::From<i32> for VIDEO_POWER_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VIDEO_POWER_STATE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct VIDEO_PUBLIC_ACCESS_RANGES {
    pub InIoSpace: u32,
    pub MappedInIoSpace: u32,
    pub VirtualAddress: *mut ::std::ffi::c_void,
}
impl VIDEO_PUBLIC_ACCESS_RANGES {}
impl ::std::default::Default for VIDEO_PUBLIC_ACCESS_RANGES {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VIDEO_PUBLIC_ACCESS_RANGES {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VIDEO_PUBLIC_ACCESS_RANGES").field("InIoSpace", &self.InIoSpace).field("MappedInIoSpace", &self.MappedInIoSpace).field("VirtualAddress", &self.VirtualAddress).finish()
    }
}
impl ::std::cmp::PartialEq for VIDEO_PUBLIC_ACCESS_RANGES {
    fn eq(&self, other: &Self) -> bool {
        self.InIoSpace == other.InIoSpace && self.MappedInIoSpace == other.MappedInIoSpace && self.VirtualAddress == other.VirtualAddress
    }
}
impl ::std::cmp::Eq for VIDEO_PUBLIC_ACCESS_RANGES {}
unsafe impl ::windows::runtime::Abi for VIDEO_PUBLIC_ACCESS_RANGES {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct VIDEO_QUERY_PERFORMANCE_COUNTER {
    pub BufferSize: u32,
    pub Buffer: *mut VIDEO_PERFORMANCE_COUNTER,
}
impl VIDEO_QUERY_PERFORMANCE_COUNTER {}
impl ::std::default::Default for VIDEO_QUERY_PERFORMANCE_COUNTER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VIDEO_QUERY_PERFORMANCE_COUNTER {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VIDEO_QUERY_PERFORMANCE_COUNTER").field("BufferSize", &self.BufferSize).field("Buffer", &self.Buffer).finish()
    }
}
impl ::std::cmp::PartialEq for VIDEO_QUERY_PERFORMANCE_COUNTER {
    fn eq(&self, other: &Self) -> bool {
        self.BufferSize == other.BufferSize && self.Buffer == other.Buffer
    }
}
impl ::std::cmp::Eq for VIDEO_QUERY_PERFORMANCE_COUNTER {}
unsafe impl ::windows::runtime::Abi for VIDEO_QUERY_PERFORMANCE_COUNTER {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const VIDEO_REASON_ALLOCATION: u32 = 6u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const VIDEO_REASON_CONFIGURATION: u32 = 9u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const VIDEO_REASON_FAILED_ROTATION: u32 = 5u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const VIDEO_REASON_LOCK: u32 = 5u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const VIDEO_REASON_NONE: u32 = 0u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const VIDEO_REASON_POLICY1: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const VIDEO_REASON_POLICY2: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const VIDEO_REASON_POLICY3: u32 = 3u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const VIDEO_REASON_POLICY4: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const VIDEO_REASON_SCRATCH: u32 = 8u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct VIDEO_REGISTER_VDM {
    pub MinimumStateSize: u32,
}
impl VIDEO_REGISTER_VDM {}
impl ::std::default::Default for VIDEO_REGISTER_VDM {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VIDEO_REGISTER_VDM {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VIDEO_REGISTER_VDM").field("MinimumStateSize", &self.MinimumStateSize).finish()
    }
}
impl ::std::cmp::PartialEq for VIDEO_REGISTER_VDM {
    fn eq(&self, other: &Self) -> bool {
        self.MinimumStateSize == other.MinimumStateSize
    }
}
impl ::std::cmp::Eq for VIDEO_REGISTER_VDM {}
unsafe impl ::windows::runtime::Abi for VIDEO_REGISTER_VDM {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
pub struct VIDEO_SHARE_MEMORY {
    pub ProcessHandle: super::super::Foundation::HANDLE,
    pub ViewOffset: u32,
    pub ViewSize: u32,
    pub RequestedVirtualAddress: *mut ::std::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl VIDEO_SHARE_MEMORY {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for VIDEO_SHARE_MEMORY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for VIDEO_SHARE_MEMORY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VIDEO_SHARE_MEMORY").field("ProcessHandle", &self.ProcessHandle).field("ViewOffset", &self.ViewOffset).field("ViewSize", &self.ViewSize).field("RequestedVirtualAddress", &self.RequestedVirtualAddress).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for VIDEO_SHARE_MEMORY {
    fn eq(&self, other: &Self) -> bool {
        self.ProcessHandle == other.ProcessHandle && self.ViewOffset == other.ViewOffset && self.ViewSize == other.ViewSize && self.RequestedVirtualAddress == other.RequestedVirtualAddress
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for VIDEO_SHARE_MEMORY {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for VIDEO_SHARE_MEMORY {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct VIDEO_SHARE_MEMORY_INFORMATION {
    pub SharedViewOffset: u32,
    pub SharedViewSize: u32,
    pub VirtualAddress: *mut ::std::ffi::c_void,
}
impl VIDEO_SHARE_MEMORY_INFORMATION {}
impl ::std::default::Default for VIDEO_SHARE_MEMORY_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VIDEO_SHARE_MEMORY_INFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VIDEO_SHARE_MEMORY_INFORMATION").field("SharedViewOffset", &self.SharedViewOffset).field("SharedViewSize", &self.SharedViewSize).field("VirtualAddress", &self.VirtualAddress).finish()
    }
}
impl ::std::cmp::PartialEq for VIDEO_SHARE_MEMORY_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.SharedViewOffset == other.SharedViewOffset && self.SharedViewSize == other.SharedViewSize && self.VirtualAddress == other.VirtualAddress
    }
}
impl ::std::cmp::Eq for VIDEO_SHARE_MEMORY_INFORMATION {}
unsafe impl ::windows::runtime::Abi for VIDEO_SHARE_MEMORY_INFORMATION {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const VIDEO_STATE_NON_STANDARD_VGA: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const VIDEO_STATE_PACKED_CHAIN4_MODE: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const VIDEO_STATE_UNEMULATED_VGA_STATE: u32 = 2u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
pub struct VIDEO_VDM {
    pub ProcessHandle: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl VIDEO_VDM {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for VIDEO_VDM {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for VIDEO_VDM {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VIDEO_VDM").field("ProcessHandle", &self.ProcessHandle).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for VIDEO_VDM {
    fn eq(&self, other: &Self) -> bool {
        self.ProcessHandle == other.ProcessHandle
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for VIDEO_VDM {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for VIDEO_VDM {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
pub struct VIDEO_WIN32K_CALLBACKS {
    pub PhysDisp: *mut ::std::ffi::c_void,
    pub Callout: ::std::option::Option<PVIDEO_WIN32K_CALLOUT>,
    pub bACPI: u32,
    pub pPhysDeviceObject: super::super::Foundation::HANDLE,
    pub DualviewFlags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl VIDEO_WIN32K_CALLBACKS {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for VIDEO_WIN32K_CALLBACKS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for VIDEO_WIN32K_CALLBACKS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VIDEO_WIN32K_CALLBACKS").field("PhysDisp", &self.PhysDisp).field("bACPI", &self.bACPI).field("pPhysDeviceObject", &self.pPhysDeviceObject).field("DualviewFlags", &self.DualviewFlags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for VIDEO_WIN32K_CALLBACKS {
    fn eq(&self, other: &Self) -> bool {
        self.PhysDisp == other.PhysDisp && self.Callout.map(|f| f as usize) == other.Callout.map(|f| f as usize) && self.bACPI == other.bACPI && self.pPhysDeviceObject == other.pPhysDeviceObject && self.DualviewFlags == other.DualviewFlags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for VIDEO_WIN32K_CALLBACKS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for VIDEO_WIN32K_CALLBACKS {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
pub struct VIDEO_WIN32K_CALLBACKS_PARAMS {
    pub CalloutType: VIDEO_WIN32K_CALLBACKS_PARAMS_TYPE,
    pub PhysDisp: *mut ::std::ffi::c_void,
    pub Param: usize,
    pub Status: i32,
    pub LockUserSession: super::super::Foundation::BOOLEAN,
    pub IsPostDevice: super::super::Foundation::BOOLEAN,
    pub SurpriseRemoval: super::super::Foundation::BOOLEAN,
    pub WaitForQueueReady: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl VIDEO_WIN32K_CALLBACKS_PARAMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for VIDEO_WIN32K_CALLBACKS_PARAMS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for VIDEO_WIN32K_CALLBACKS_PARAMS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VIDEO_WIN32K_CALLBACKS_PARAMS")
            .field("CalloutType", &self.CalloutType)
            .field("PhysDisp", &self.PhysDisp)
            .field("Param", &self.Param)
            .field("Status", &self.Status)
            .field("LockUserSession", &self.LockUserSession)
            .field("IsPostDevice", &self.IsPostDevice)
            .field("SurpriseRemoval", &self.SurpriseRemoval)
            .field("WaitForQueueReady", &self.WaitForQueueReady)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for VIDEO_WIN32K_CALLBACKS_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.CalloutType == other.CalloutType && self.PhysDisp == other.PhysDisp && self.Param == other.Param && self.Status == other.Status && self.LockUserSession == other.LockUserSession && self.IsPostDevice == other.IsPostDevice && self.SurpriseRemoval == other.SurpriseRemoval && self.WaitForQueueReady == other.WaitForQueueReady
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for VIDEO_WIN32K_CALLBACKS_PARAMS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for VIDEO_WIN32K_CALLBACKS_PARAMS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct VIDEO_WIN32K_CALLBACKS_PARAMS_TYPE(pub i32);
pub const VideoPowerNotifyCallout: VIDEO_WIN32K_CALLBACKS_PARAMS_TYPE = VIDEO_WIN32K_CALLBACKS_PARAMS_TYPE(1i32);
pub const VideoEnumChildPdoNotifyCallout: VIDEO_WIN32K_CALLBACKS_PARAMS_TYPE = VIDEO_WIN32K_CALLBACKS_PARAMS_TYPE(3i32);
pub const VideoFindAdapterCallout: VIDEO_WIN32K_CALLBACKS_PARAMS_TYPE = VIDEO_WIN32K_CALLBACKS_PARAMS_TYPE(4i32);
pub const VideoPnpNotifyCallout: VIDEO_WIN32K_CALLBACKS_PARAMS_TYPE = VIDEO_WIN32K_CALLBACKS_PARAMS_TYPE(7i32);
pub const VideoDxgkDisplaySwitchCallout: VIDEO_WIN32K_CALLBACKS_PARAMS_TYPE = VIDEO_WIN32K_CALLBACKS_PARAMS_TYPE(8i32);
pub const VideoDxgkFindAdapterTdrCallout: VIDEO_WIN32K_CALLBACKS_PARAMS_TYPE = VIDEO_WIN32K_CALLBACKS_PARAMS_TYPE(10i32);
pub const VideoDxgkHardwareProtectionTeardown: VIDEO_WIN32K_CALLBACKS_PARAMS_TYPE = VIDEO_WIN32K_CALLBACKS_PARAMS_TYPE(11i32);
pub const VideoRepaintDesktop: VIDEO_WIN32K_CALLBACKS_PARAMS_TYPE = VIDEO_WIN32K_CALLBACKS_PARAMS_TYPE(12i32);
pub const VideoUpdateCursor: VIDEO_WIN32K_CALLBACKS_PARAMS_TYPE = VIDEO_WIN32K_CALLBACKS_PARAMS_TYPE(13i32);
pub const VideoDisableMultiPlaneOverlay: VIDEO_WIN32K_CALLBACKS_PARAMS_TYPE = VIDEO_WIN32K_CALLBACKS_PARAMS_TYPE(14i32);
pub const VideoDesktopDuplicationChange: VIDEO_WIN32K_CALLBACKS_PARAMS_TYPE = VIDEO_WIN32K_CALLBACKS_PARAMS_TYPE(15i32);
pub const VideoBlackScreenDiagnostics: VIDEO_WIN32K_CALLBACKS_PARAMS_TYPE = VIDEO_WIN32K_CALLBACKS_PARAMS_TYPE(16i32);
impl ::std::convert::From<i32> for VIDEO_WIN32K_CALLBACKS_PARAMS_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VIDEO_WIN32K_CALLBACKS_PARAMS_TYPE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_DirectDraw`*"]
pub struct VMEMHEAP {
    pub dwFlags: u32,
    pub stride: u32,
    pub freeList: *mut ::std::ffi::c_void,
    pub allocList: *mut ::std::ffi::c_void,
    pub dwTotalSize: u32,
    pub fpGARTLin: usize,
    pub fpGARTDev: usize,
    pub dwCommitedSize: u32,
    pub dwCoalesceCount: u32,
    pub Alignment: HEAPALIGNMENT,
    pub ddsCapsEx: super::super::Graphics::DirectDraw::DDSCAPSEX,
    pub ddsCapsExAlt: super::super::Graphics::DirectDraw::DDSCAPSEX,
    pub liPhysAGPBase: i64,
    pub hdevAGP: super::super::Foundation::HANDLE,
    pub pvPhysRsrv: *mut ::std::ffi::c_void,
    pub pAgpCommitMask: *mut u8,
    pub dwAgpCommitMaskSize: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl VMEMHEAP {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::default::Default for VMEMHEAP {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::PartialEq for VMEMHEAP {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::Eq for VMEMHEAP {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
unsafe impl ::windows::runtime::Abi for VMEMHEAP {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const VMEMHEAP_ALIGNMENT: i32 = 4i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const VMEMHEAP_LINEAR: i32 = 1i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const VMEMHEAP_RECTANGULAR: i32 = 2i32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
pub struct VMEML {
    pub next: *mut VMEML,
    pub ptr: usize,
    pub size: u32,
    pub bDiscardable: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl VMEML {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for VMEML {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for VMEML {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VMEML").field("next", &self.next).field("ptr", &self.ptr).field("size", &self.size).field("bDiscardable", &self.bDiscardable).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for VMEML {
    fn eq(&self, other: &Self) -> bool {
        self.next == other.next && self.ptr == other.ptr && self.size == other.size && self.bDiscardable == other.bDiscardable
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for VMEML {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for VMEML {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
pub struct VMEMR {
    pub next: *mut VMEMR,
    pub prev: *mut VMEMR,
    pub pUp: *mut VMEMR,
    pub pDown: *mut VMEMR,
    pub pLeft: *mut VMEMR,
    pub pRight: *mut VMEMR,
    pub ptr: usize,
    pub size: u32,
    pub x: u32,
    pub y: u32,
    pub cx: u32,
    pub cy: u32,
    pub flags: u32,
    pub pBits: usize,
    pub bDiscardable: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl VMEMR {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for VMEMR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for VMEMR {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VMEMR")
            .field("next", &self.next)
            .field("prev", &self.prev)
            .field("pUp", &self.pUp)
            .field("pDown", &self.pDown)
            .field("pLeft", &self.pLeft)
            .field("pRight", &self.pRight)
            .field("ptr", &self.ptr)
            .field("size", &self.size)
            .field("x", &self.x)
            .field("y", &self.y)
            .field("cx", &self.cx)
            .field("cy", &self.cy)
            .field("flags", &self.flags)
            .field("pBits", &self.pBits)
            .field("bDiscardable", &self.bDiscardable)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for VMEMR {
    fn eq(&self, other: &Self) -> bool {
        self.next == other.next && self.prev == other.prev && self.pUp == other.pUp && self.pDown == other.pDown && self.pLeft == other.pLeft && self.pRight == other.pRight && self.ptr == other.ptr && self.size == other.size && self.x == other.x && self.y == other.y && self.cx == other.cx && self.cy == other.cy && self.flags == other.flags && self.pBits == other.pBits && self.bDiscardable == other.bDiscardable
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for VMEMR {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for VMEMR {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const _FACDXCORE: u32 = 2176u32;
