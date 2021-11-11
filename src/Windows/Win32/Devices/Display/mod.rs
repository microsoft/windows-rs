#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
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
impl ::core::convert::From<i32> for AR_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AR_STATE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct Adapter {
    pub AdapterName: [u16; 128],
    pub numSources: i32,
    pub sources: [Sources; 1],
}
impl Adapter {}
impl ::core::default::Default for Adapter {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for Adapter {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("Adapter").field("AdapterName", &self.AdapterName).field("numSources", &self.numSources).field("sources", &self.sources).finish()
    }
}
impl ::core::cmp::PartialEq for Adapter {
    fn eq(&self, other: &Self) -> bool {
        self.AdapterName == other.AdapterName && self.numSources == other.numSources && self.sources == other.sources
    }
}
impl ::core::cmp::Eq for Adapter {}
unsafe impl ::windows::core::Abi for Adapter {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct Adapters {
    pub numAdapters: i32,
    pub adapter: [Adapter; 1],
}
impl Adapters {}
impl ::core::default::Default for Adapters {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for Adapters {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("Adapters").field("numAdapters", &self.numAdapters).field("adapter", &self.adapter).finish()
    }
}
impl ::core::cmp::PartialEq for Adapters {
    fn eq(&self, other: &Self) -> bool {
        self.numAdapters == other.numAdapters && self.adapter == other.adapter
    }
}
impl ::core::cmp::Eq for Adapters {}
unsafe impl ::windows::core::Abi for Adapters {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct BACKLIGHT_OPTIMIZATION_LEVEL(pub i32);
pub const BacklightOptimizationDisable: BACKLIGHT_OPTIMIZATION_LEVEL = BACKLIGHT_OPTIMIZATION_LEVEL(0i32);
pub const BacklightOptimizationDesktop: BACKLIGHT_OPTIMIZATION_LEVEL = BACKLIGHT_OPTIMIZATION_LEVEL(1i32);
pub const BacklightOptimizationDynamic: BACKLIGHT_OPTIMIZATION_LEVEL = BACKLIGHT_OPTIMIZATION_LEVEL(2i32);
pub const BacklightOptimizationDimmed: BACKLIGHT_OPTIMIZATION_LEVEL = BACKLIGHT_OPTIMIZATION_LEVEL(3i32);
pub const BacklightOptimizationEDR: BACKLIGHT_OPTIMIZATION_LEVEL = BACKLIGHT_OPTIMIZATION_LEVEL(4i32);
impl ::core::convert::From<i32> for BACKLIGHT_OPTIMIZATION_LEVEL {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for BACKLIGHT_OPTIMIZATION_LEVEL {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct BACKLIGHT_REDUCTION_GAMMA_RAMP {
    pub R: [u16; 256],
    pub G: [u16; 256],
    pub B: [u16; 256],
}
impl BACKLIGHT_REDUCTION_GAMMA_RAMP {}
impl ::core::default::Default for BACKLIGHT_REDUCTION_GAMMA_RAMP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for BACKLIGHT_REDUCTION_GAMMA_RAMP {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("BACKLIGHT_REDUCTION_GAMMA_RAMP").field("R", &self.R).field("G", &self.G).field("B", &self.B).finish()
    }
}
impl ::core::cmp::PartialEq for BACKLIGHT_REDUCTION_GAMMA_RAMP {
    fn eq(&self, other: &Self) -> bool {
        self.R == other.R && self.G == other.G && self.B == other.B
    }
}
impl ::core::cmp::Eq for BACKLIGHT_REDUCTION_GAMMA_RAMP {}
unsafe impl ::windows::core::Abi for BACKLIGHT_REDUCTION_GAMMA_RAMP {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct BANK_POSITION {
    pub ReadBankPosition: u32,
    pub WriteBankPosition: u32,
}
impl BANK_POSITION {}
impl ::core::default::Default for BANK_POSITION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for BANK_POSITION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("BANK_POSITION").field("ReadBankPosition", &self.ReadBankPosition).field("WriteBankPosition", &self.WriteBankPosition).finish()
    }
}
impl ::core::cmp::PartialEq for BANK_POSITION {
    fn eq(&self, other: &Self) -> bool {
        self.ReadBankPosition == other.ReadBankPosition && self.WriteBankPosition == other.WriteBankPosition
    }
}
impl ::core::cmp::Eq for BANK_POSITION {}
unsafe impl ::windows::core::Abi for BANK_POSITION {
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
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Graphics_Gdi`*"]
pub struct BLENDOBJ {
    pub BlendFunction: super::super::Graphics::Gdi::BLENDFUNCTION,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl BLENDOBJ {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for BLENDOBJ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::fmt::Debug for BLENDOBJ {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("BLENDOBJ").field("BlendFunction", &self.BlendFunction).finish()
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
unsafe impl ::windows::core::Abi for BLENDOBJ {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const BMF_16BPP: i32 = 4i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const BMF_1BPP: i32 = 1i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const BMF_24BPP: i32 = 5i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const BMF_32BPP: i32 = 6i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const BMF_4BPP: i32 = 2i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const BMF_4RLE: i32 = 7i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const BMF_8BPP: i32 = 3i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const BMF_8RLE: i32 = 8i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const BMF_ACC_NOTIFY: u32 = 32768u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const BMF_DONTCACHE: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const BMF_JPEG: i32 = 9i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const BMF_KMSECTION: u32 = 16u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const BMF_NOTSYSMEM: u32 = 32u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const BMF_NOZEROINIT: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const BMF_PNG: i32 = 10i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const BMF_RESERVED: u32 = 15872u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const BMF_RMT_ENTER: u32 = 16384u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const BMF_TEMP_ALPHA: u32 = 256u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const BMF_TOPDOWN: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const BMF_UMPDMEM: u32 = 128u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const BMF_USERMEM: u32 = 8u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const BMF_WINDOW_BLT: u32 = 64u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct BRIGHTNESS_INTERFACE_VERSION(pub i32);
pub const BRIGHTNESS_INTERFACE_VERSION_1: BRIGHTNESS_INTERFACE_VERSION = BRIGHTNESS_INTERFACE_VERSION(1i32);
pub const BRIGHTNESS_INTERFACE_VERSION_2: BRIGHTNESS_INTERFACE_VERSION = BRIGHTNESS_INTERFACE_VERSION(2i32);
pub const BRIGHTNESS_INTERFACE_VERSION_3: BRIGHTNESS_INTERFACE_VERSION = BRIGHTNESS_INTERFACE_VERSION(3i32);
impl ::core::convert::From<i32> for BRIGHTNESS_INTERFACE_VERSION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for BRIGHTNESS_INTERFACE_VERSION {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct BRIGHTNESS_LEVEL {
    pub Count: u8,
    pub Level: [u8; 103],
}
impl BRIGHTNESS_LEVEL {}
impl ::core::default::Default for BRIGHTNESS_LEVEL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for BRIGHTNESS_LEVEL {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("BRIGHTNESS_LEVEL").field("Count", &self.Count).field("Level", &self.Level).finish()
    }
}
impl ::core::cmp::PartialEq for BRIGHTNESS_LEVEL {
    fn eq(&self, other: &Self) -> bool {
        self.Count == other.Count && self.Level == other.Level
    }
}
impl ::core::cmp::Eq for BRIGHTNESS_LEVEL {}
unsafe impl ::windows::core::Abi for BRIGHTNESS_LEVEL {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const BRIGHTNESS_MAX_LEVEL_COUNT: u32 = 103u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const BRIGHTNESS_MAX_NIT_RANGE_COUNT: u32 = 16u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct BRIGHTNESS_NIT_RANGE {
    pub MinLevelInMillinit: u32,
    pub MaxLevelInMillinit: u32,
    pub StepSizeInMillinit: u32,
}
impl BRIGHTNESS_NIT_RANGE {}
impl ::core::default::Default for BRIGHTNESS_NIT_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for BRIGHTNESS_NIT_RANGE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("BRIGHTNESS_NIT_RANGE").field("MinLevelInMillinit", &self.MinLevelInMillinit).field("MaxLevelInMillinit", &self.MaxLevelInMillinit).field("StepSizeInMillinit", &self.StepSizeInMillinit).finish()
    }
}
impl ::core::cmp::PartialEq for BRIGHTNESS_NIT_RANGE {
    fn eq(&self, other: &Self) -> bool {
        self.MinLevelInMillinit == other.MinLevelInMillinit && self.MaxLevelInMillinit == other.MaxLevelInMillinit && self.StepSizeInMillinit == other.StepSizeInMillinit
    }
}
impl ::core::cmp::Eq for BRIGHTNESS_NIT_RANGE {}
unsafe impl ::windows::core::Abi for BRIGHTNESS_NIT_RANGE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct BRIGHTNESS_NIT_RANGES {
    pub NormalRangeCount: u32,
    pub RangeCount: u32,
    pub PreferredMaximumBrightness: u32,
    pub SupportedRanges: [BRIGHTNESS_NIT_RANGE; 16],
}
impl BRIGHTNESS_NIT_RANGES {}
impl ::core::default::Default for BRIGHTNESS_NIT_RANGES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for BRIGHTNESS_NIT_RANGES {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("BRIGHTNESS_NIT_RANGES").field("NormalRangeCount", &self.NormalRangeCount).field("RangeCount", &self.RangeCount).field("PreferredMaximumBrightness", &self.PreferredMaximumBrightness).field("SupportedRanges", &self.SupportedRanges).finish()
    }
}
impl ::core::cmp::PartialEq for BRIGHTNESS_NIT_RANGES {
    fn eq(&self, other: &Self) -> bool {
        self.NormalRangeCount == other.NormalRangeCount && self.RangeCount == other.RangeCount && self.PreferredMaximumBrightness == other.PreferredMaximumBrightness && self.SupportedRanges == other.SupportedRanges
    }
}
impl ::core::cmp::Eq for BRIGHTNESS_NIT_RANGES {}
unsafe impl ::windows::core::Abi for BRIGHTNESS_NIT_RANGES {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct BRUSHOBJ {
    pub iSolidColor: u32,
    pub pvRbrush: *mut ::core::ffi::c_void,
    pub flColorType: u32,
}
impl BRUSHOBJ {}
impl ::core::default::Default for BRUSHOBJ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for BRUSHOBJ {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("BRUSHOBJ").field("iSolidColor", &self.iSolidColor).field("pvRbrush", &self.pvRbrush).field("flColorType", &self.flColorType).finish()
    }
}
impl ::core::cmp::PartialEq for BRUSHOBJ {
    fn eq(&self, other: &Self) -> bool {
        self.iSolidColor == other.iSolidColor && self.pvRbrush == other.pvRbrush && self.flColorType == other.flColorType
    }
}
impl ::core::cmp::Eq for BRUSHOBJ {}
unsafe impl ::windows::core::Abi for BRUSHOBJ {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BRUSHOBJ_hGetColorTransform(pbo: *mut BRUSHOBJ) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BRUSHOBJ_hGetColorTransform(pbo: *mut BRUSHOBJ) -> super::super::Foundation::HANDLE;
        }
        ::core::mem::transmute(BRUSHOBJ_hGetColorTransform(::core::mem::transmute(pbo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
#[inline]
pub unsafe fn BRUSHOBJ_pvAllocRbrush(pbo: *mut BRUSHOBJ, cj: u32) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BRUSHOBJ_pvAllocRbrush(pbo: *mut BRUSHOBJ, cj: u32) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(BRUSHOBJ_pvAllocRbrush(::core::mem::transmute(pbo), ::core::mem::transmute(cj)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
#[inline]
pub unsafe fn BRUSHOBJ_pvGetRbrush(pbo: *mut BRUSHOBJ) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BRUSHOBJ_pvGetRbrush(pbo: *mut BRUSHOBJ) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(BRUSHOBJ_pvGetRbrush(::core::mem::transmute(pbo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
#[inline]
pub unsafe fn BRUSHOBJ_ulGetBrushColor(pbo: *mut BRUSHOBJ) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BRUSHOBJ_ulGetBrushColor(pbo: *mut BRUSHOBJ) -> u32;
        }
        ::core::mem::transmute(BRUSHOBJ_ulGetBrushColor(::core::mem::transmute(pbo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const BR_CMYKCOLOR: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const BR_DEVICE_ICM: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const BR_HOST_ICM: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const BR_ORIGCOLOR: u32 = 8u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct BlackScreenDiagnosticsCalloutParam(pub i32);
pub const BlackScreenDiagnosticsData: BlackScreenDiagnosticsCalloutParam = BlackScreenDiagnosticsCalloutParam(1i32);
pub const BlackScreenDisplayRecovery: BlackScreenDiagnosticsCalloutParam = BlackScreenDiagnosticsCalloutParam(2i32);
impl ::core::convert::From<i32> for BlackScreenDiagnosticsCalloutParam {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for BlackScreenDiagnosticsCalloutParam {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const CDBEX_CROSSADAPTER: u32 = 8u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const CDBEX_DXINTEROP: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const CDBEX_NTSHAREDSURFACEHANDLE: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const CDBEX_REDIRECTION: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const CDBEX_REUSE: u32 = 16u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
pub struct CDDDXGK_REDIRBITMAPPRESENTINFO {
    pub NumDirtyRects: u32,
    pub DirtyRect: *mut super::super::Foundation::RECT,
    pub NumContexts: u32,
    pub hContext: [super::super::Foundation::HANDLE; 65],
    pub bDoNotSynchronizeWithDxContent: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl CDDDXGK_REDIRBITMAPPRESENTINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CDDDXGK_REDIRBITMAPPRESENTINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CDDDXGK_REDIRBITMAPPRESENTINFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("CDDDXGK_REDIRBITMAPPRESENTINFO").field("NumDirtyRects", &self.NumDirtyRects).field("DirtyRect", &self.DirtyRect).field("NumContexts", &self.NumContexts).field("hContext", &self.hContext).field("bDoNotSynchronizeWithDxContent", &self.bDoNotSynchronizeWithDxContent).finish()
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
unsafe impl ::windows::core::Abi for CDDDXGK_REDIRBITMAPPRESENTINFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const CD_ANY: i32 = 4i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const CD_LEFTDOWN: i32 = 1i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const CD_LEFTUP: i32 = 3i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const CD_LEFTWARDS: i32 = 1i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const CD_RIGHTDOWN: i32 = 0i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const CD_RIGHTUP: i32 = 2i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const CD_UPWARDS: i32 = 2i32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
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
impl ::core::default::Default for CHAR_IMAGE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Console"))]
impl ::core::cmp::PartialEq for CHAR_IMAGE_INFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Console"))]
impl ::core::cmp::Eq for CHAR_IMAGE_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Console"))]
unsafe impl ::windows::core::Abi for CHAR_IMAGE_INFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const CHAR_TYPE_LEADING: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const CHAR_TYPE_SBCS: u32 = 0u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const CHAR_TYPE_TRAILING: u32 = 3u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct CHROMATICITY_COORDINATE {
    pub x: f32,
    pub y: f32,
}
impl CHROMATICITY_COORDINATE {}
impl ::core::default::Default for CHROMATICITY_COORDINATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for CHROMATICITY_COORDINATE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("CHROMATICITY_COORDINATE").field("x", &self.x).field("y", &self.y).finish()
    }
}
impl ::core::cmp::PartialEq for CHROMATICITY_COORDINATE {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl ::core::cmp::Eq for CHROMATICITY_COORDINATE {}
unsafe impl ::windows::core::Abi for CHROMATICITY_COORDINATE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct CIECHROMA {
    pub x: i32,
    pub y: i32,
    pub Y: i32,
}
impl CIECHROMA {}
impl ::core::default::Default for CIECHROMA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for CIECHROMA {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("CIECHROMA").field("x", &self.x).field("y", &self.y).field("Y", &self.Y).finish()
    }
}
impl ::core::cmp::PartialEq for CIECHROMA {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.Y == other.Y
    }
}
impl ::core::cmp::Eq for CIECHROMA {}
unsafe impl ::windows::core::Abi for CIECHROMA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct CLIPLINE {
    pub ptfxA: POINTFIX,
    pub ptfxB: POINTFIX,
    pub lStyleState: i32,
    pub c: u32,
    pub arun: [RUN; 1],
}
impl CLIPLINE {}
impl ::core::default::Default for CLIPLINE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for CLIPLINE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("CLIPLINE").field("ptfxA", &self.ptfxA).field("ptfxB", &self.ptfxB).field("lStyleState", &self.lStyleState).field("c", &self.c).field("arun", &self.arun).finish()
    }
}
impl ::core::cmp::PartialEq for CLIPLINE {
    fn eq(&self, other: &Self) -> bool {
        self.ptfxA == other.ptfxA && self.ptfxB == other.ptfxB && self.lStyleState == other.lStyleState && self.c == other.c && self.arun == other.arun
    }
}
impl ::core::cmp::Eq for CLIPLINE {}
unsafe impl ::windows::core::Abi for CLIPLINE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
pub struct CLIPOBJ {
    pub iUniq: u32,
    pub rclBounds: super::super::Foundation::RECTL,
    pub iDComplexity: u8,
    pub iFComplexity: u8,
    pub iMode: u8,
    pub fjOptions: u8,
}
#[cfg(feature = "Win32_Foundation")]
impl CLIPOBJ {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CLIPOBJ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CLIPOBJ {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("CLIPOBJ").field("iUniq", &self.iUniq).field("rclBounds", &self.rclBounds).field("iDComplexity", &self.iDComplexity).field("iFComplexity", &self.iFComplexity).field("iMode", &self.iMode).field("fjOptions", &self.fjOptions).finish()
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
unsafe impl ::windows::core::Abi for CLIPOBJ {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CLIPOBJ_bEnum(pco: *mut CLIPOBJ, cj: u32, pul: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CLIPOBJ_bEnum(pco: *mut CLIPOBJ, cj: u32, pul: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CLIPOBJ_bEnum(::core::mem::transmute(pco), ::core::mem::transmute(cj), ::core::mem::transmute(pul)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CLIPOBJ_cEnumStart<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(pco: *mut CLIPOBJ, ball: Param1, itype: u32, idirection: u32, climit: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CLIPOBJ_cEnumStart(pco: *mut CLIPOBJ, ball: super::super::Foundation::BOOL, itype: u32, idirection: u32, climit: u32) -> u32;
        }
        ::core::mem::transmute(CLIPOBJ_cEnumStart(::core::mem::transmute(pco), ball.into_param().abi(), ::core::mem::transmute(itype), ::core::mem::transmute(idirection), ::core::mem::transmute(climit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CLIPOBJ_ppoGetPath(pco: *mut CLIPOBJ) -> *mut PATHOBJ {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CLIPOBJ_ppoGetPath(pco: *mut CLIPOBJ) -> *mut PATHOBJ;
        }
        ::core::mem::transmute(CLIPOBJ_ppoGetPath(::core::mem::transmute(pco)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct COLORINFO {
    pub Red: CIECHROMA,
    pub Green: CIECHROMA,
    pub Blue: CIECHROMA,
    pub Cyan: CIECHROMA,
    pub Magenta: CIECHROMA,
    pub Yellow: CIECHROMA,
    pub AlignmentWhite: CIECHROMA,
    pub RedGamma: i32,
    pub GreenGamma: i32,
    pub BlueGamma: i32,
    pub MagentaInCyanDye: i32,
    pub YellowInCyanDye: i32,
    pub CyanInMagentaDye: i32,
    pub YellowInMagentaDye: i32,
    pub CyanInYellowDye: i32,
    pub MagentaInYellowDye: i32,
}
impl COLORINFO {}
impl ::core::default::Default for COLORINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for COLORINFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("COLORINFO")
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
impl ::core::cmp::PartialEq for COLORINFO {
    fn eq(&self, other: &Self) -> bool {
        self.Red == other.Red
            && self.Green == other.Green
            && self.Blue == other.Blue
            && self.Cyan == other.Cyan
            && self.Magenta == other.Magenta
            && self.Yellow == other.Yellow
            && self.AlignmentWhite == other.AlignmentWhite
            && self.RedGamma == other.RedGamma
            && self.GreenGamma == other.GreenGamma
            && self.BlueGamma == other.BlueGamma
            && self.MagentaInCyanDye == other.MagentaInCyanDye
            && self.YellowInCyanDye == other.YellowInCyanDye
            && self.CyanInMagentaDye == other.CyanInMagentaDye
            && self.YellowInMagentaDye == other.YellowInMagentaDye
            && self.CyanInYellowDye == other.CyanInYellowDye
            && self.MagentaInYellowDye == other.MagentaInYellowDye
    }
}
impl ::core::cmp::Eq for COLORINFO {}
unsafe impl ::windows::core::Abi for COLORINFO {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct COLORSPACE_TRANSFORM {
    pub Type: COLORSPACE_TRANSFORM_TYPE,
    pub Data: COLORSPACE_TRANSFORM_0,
}
impl COLORSPACE_TRANSFORM {}
impl ::core::default::Default for COLORSPACE_TRANSFORM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for COLORSPACE_TRANSFORM {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for COLORSPACE_TRANSFORM {}
unsafe impl ::windows::core::Abi for COLORSPACE_TRANSFORM {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub union COLORSPACE_TRANSFORM_0 {
    pub Rgb256x3x16: GAMMA_RAMP_RGB256x3x16,
    pub Dxgi1: GAMMA_RAMP_DXGI_1,
    pub T3x4: COLORSPACE_TRANSFORM_3x4,
    pub MatrixV2: COLORSPACE_TRANSFORM_MATRIX_V2,
}
impl COLORSPACE_TRANSFORM_0 {}
impl ::core::default::Default for COLORSPACE_TRANSFORM_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for COLORSPACE_TRANSFORM_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for COLORSPACE_TRANSFORM_0 {}
unsafe impl ::windows::core::Abi for COLORSPACE_TRANSFORM_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct COLORSPACE_TRANSFORM_1DLUT_CAP {
    pub NumberOfLUTEntries: u32,
    pub DataCap: COLORSPACE_TRANSFORM_DATA_CAP,
}
impl COLORSPACE_TRANSFORM_1DLUT_CAP {}
impl ::core::default::Default for COLORSPACE_TRANSFORM_1DLUT_CAP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for COLORSPACE_TRANSFORM_1DLUT_CAP {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for COLORSPACE_TRANSFORM_1DLUT_CAP {}
unsafe impl ::windows::core::Abi for COLORSPACE_TRANSFORM_1DLUT_CAP {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct COLORSPACE_TRANSFORM_3x4 {
    pub ColorMatrix3x4: [f32; 12],
    pub ScalarMultiplier: f32,
    pub LookupTable1D: [GAMMA_RAMP_RGB; 4096],
}
impl COLORSPACE_TRANSFORM_3x4 {}
impl ::core::default::Default for COLORSPACE_TRANSFORM_3x4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for COLORSPACE_TRANSFORM_3x4 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("COLORSPACE_TRANSFORM_3x4").field("ColorMatrix3x4", &self.ColorMatrix3x4).field("ScalarMultiplier", &self.ScalarMultiplier).field("LookupTable1D", &self.LookupTable1D).finish()
    }
}
impl ::core::cmp::PartialEq for COLORSPACE_TRANSFORM_3x4 {
    fn eq(&self, other: &Self) -> bool {
        self.ColorMatrix3x4 == other.ColorMatrix3x4 && self.ScalarMultiplier == other.ScalarMultiplier && self.LookupTable1D == other.LookupTable1D
    }
}
impl ::core::cmp::Eq for COLORSPACE_TRANSFORM_3x4 {}
unsafe impl ::windows::core::Abi for COLORSPACE_TRANSFORM_3x4 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct COLORSPACE_TRANSFORM_DATA_CAP {
    pub DataType: COLORSPACE_TRANSFORM_DATA_TYPE,
    pub Anonymous: COLORSPACE_TRANSFORM_DATA_CAP_0,
    pub NumericRangeMin: f32,
    pub NumericRangeMax: f32,
}
impl COLORSPACE_TRANSFORM_DATA_CAP {}
impl ::core::default::Default for COLORSPACE_TRANSFORM_DATA_CAP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for COLORSPACE_TRANSFORM_DATA_CAP {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for COLORSPACE_TRANSFORM_DATA_CAP {}
unsafe impl ::windows::core::Abi for COLORSPACE_TRANSFORM_DATA_CAP {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub union COLORSPACE_TRANSFORM_DATA_CAP_0 {
    pub Anonymous1: COLORSPACE_TRANSFORM_DATA_CAP_0_0,
    pub Anonymous2: COLORSPACE_TRANSFORM_DATA_CAP_0_1,
    pub Value: u32,
}
impl COLORSPACE_TRANSFORM_DATA_CAP_0 {}
impl ::core::default::Default for COLORSPACE_TRANSFORM_DATA_CAP_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for COLORSPACE_TRANSFORM_DATA_CAP_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for COLORSPACE_TRANSFORM_DATA_CAP_0 {}
unsafe impl ::windows::core::Abi for COLORSPACE_TRANSFORM_DATA_CAP_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct COLORSPACE_TRANSFORM_DATA_CAP_0_0 {
    pub _bitfield: u32,
}
impl COLORSPACE_TRANSFORM_DATA_CAP_0_0 {}
impl ::core::default::Default for COLORSPACE_TRANSFORM_DATA_CAP_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for COLORSPACE_TRANSFORM_DATA_CAP_0_0 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_Anonymous1_e__Struct").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::core::cmp::PartialEq for COLORSPACE_TRANSFORM_DATA_CAP_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for COLORSPACE_TRANSFORM_DATA_CAP_0_0 {}
unsafe impl ::windows::core::Abi for COLORSPACE_TRANSFORM_DATA_CAP_0_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct COLORSPACE_TRANSFORM_DATA_CAP_0_1 {
    pub _bitfield: u32,
}
impl COLORSPACE_TRANSFORM_DATA_CAP_0_1 {}
impl ::core::default::Default for COLORSPACE_TRANSFORM_DATA_CAP_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for COLORSPACE_TRANSFORM_DATA_CAP_0_1 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_Anonymous2_e__Struct").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::core::cmp::PartialEq for COLORSPACE_TRANSFORM_DATA_CAP_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for COLORSPACE_TRANSFORM_DATA_CAP_0_1 {}
unsafe impl ::windows::core::Abi for COLORSPACE_TRANSFORM_DATA_CAP_0_1 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct COLORSPACE_TRANSFORM_DATA_TYPE(pub i32);
pub const COLORSPACE_TRANSFORM_DATA_TYPE_FIXED_POINT: COLORSPACE_TRANSFORM_DATA_TYPE = COLORSPACE_TRANSFORM_DATA_TYPE(0i32);
pub const COLORSPACE_TRANSFORM_DATA_TYPE_FLOAT: COLORSPACE_TRANSFORM_DATA_TYPE = COLORSPACE_TRANSFORM_DATA_TYPE(1i32);
impl ::core::convert::From<i32> for COLORSPACE_TRANSFORM_DATA_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for COLORSPACE_TRANSFORM_DATA_TYPE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct COLORSPACE_TRANSFORM_MATRIX_CAP {
    pub Anonymous: COLORSPACE_TRANSFORM_MATRIX_CAP_0,
    pub DataCap: COLORSPACE_TRANSFORM_DATA_CAP,
}
impl COLORSPACE_TRANSFORM_MATRIX_CAP {}
impl ::core::default::Default for COLORSPACE_TRANSFORM_MATRIX_CAP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for COLORSPACE_TRANSFORM_MATRIX_CAP {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for COLORSPACE_TRANSFORM_MATRIX_CAP {}
unsafe impl ::windows::core::Abi for COLORSPACE_TRANSFORM_MATRIX_CAP {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub union COLORSPACE_TRANSFORM_MATRIX_CAP_0 {
    pub Anonymous: COLORSPACE_TRANSFORM_MATRIX_CAP_0_0,
    pub Value: u32,
}
impl COLORSPACE_TRANSFORM_MATRIX_CAP_0 {}
impl ::core::default::Default for COLORSPACE_TRANSFORM_MATRIX_CAP_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for COLORSPACE_TRANSFORM_MATRIX_CAP_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for COLORSPACE_TRANSFORM_MATRIX_CAP_0 {}
unsafe impl ::windows::core::Abi for COLORSPACE_TRANSFORM_MATRIX_CAP_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct COLORSPACE_TRANSFORM_MATRIX_CAP_0_0 {
    pub _bitfield: u32,
}
impl COLORSPACE_TRANSFORM_MATRIX_CAP_0_0 {}
impl ::core::default::Default for COLORSPACE_TRANSFORM_MATRIX_CAP_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for COLORSPACE_TRANSFORM_MATRIX_CAP_0_0 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::core::cmp::PartialEq for COLORSPACE_TRANSFORM_MATRIX_CAP_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for COLORSPACE_TRANSFORM_MATRIX_CAP_0_0 {}
unsafe impl ::windows::core::Abi for COLORSPACE_TRANSFORM_MATRIX_CAP_0_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
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
impl ::core::default::Default for COLORSPACE_TRANSFORM_MATRIX_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for COLORSPACE_TRANSFORM_MATRIX_V2 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
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
impl ::core::cmp::PartialEq for COLORSPACE_TRANSFORM_MATRIX_V2 {
    fn eq(&self, other: &Self) -> bool {
        self.StageControlLookupTable1DDegamma == other.StageControlLookupTable1DDegamma && self.LookupTable1DDegamma == other.LookupTable1DDegamma && self.StageControlColorMatrix3x3 == other.StageControlColorMatrix3x3 && self.ColorMatrix3x3 == other.ColorMatrix3x3 && self.StageControlLookupTable1DRegamma == other.StageControlLookupTable1DRegamma && self.LookupTable1DRegamma == other.LookupTable1DRegamma
    }
}
impl ::core::cmp::Eq for COLORSPACE_TRANSFORM_MATRIX_V2 {}
unsafe impl ::windows::core::Abi for COLORSPACE_TRANSFORM_MATRIX_V2 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct COLORSPACE_TRANSFORM_SET_INPUT {
    pub OutputWireColorSpaceExpected: OUTPUT_WIRE_COLOR_SPACE_TYPE,
    pub OutputWireFormatExpected: OUTPUT_WIRE_FORMAT,
    pub ColorSpaceTransform: COLORSPACE_TRANSFORM,
}
impl COLORSPACE_TRANSFORM_SET_INPUT {}
impl ::core::default::Default for COLORSPACE_TRANSFORM_SET_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for COLORSPACE_TRANSFORM_SET_INPUT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for COLORSPACE_TRANSFORM_SET_INPUT {}
unsafe impl ::windows::core::Abi for COLORSPACE_TRANSFORM_SET_INPUT {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct COLORSPACE_TRANSFORM_STAGE_CONTROL(pub i32);
pub const ColorSpaceTransformStageControl_No_Change: COLORSPACE_TRANSFORM_STAGE_CONTROL = COLORSPACE_TRANSFORM_STAGE_CONTROL(0i32);
pub const ColorSpaceTransformStageControl_Enable: COLORSPACE_TRANSFORM_STAGE_CONTROL = COLORSPACE_TRANSFORM_STAGE_CONTROL(1i32);
pub const ColorSpaceTransformStageControl_Bypass: COLORSPACE_TRANSFORM_STAGE_CONTROL = COLORSPACE_TRANSFORM_STAGE_CONTROL(2i32);
impl ::core::convert::From<i32> for COLORSPACE_TRANSFORM_STAGE_CONTROL {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for COLORSPACE_TRANSFORM_STAGE_CONTROL {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct COLORSPACE_TRANSFORM_TARGET_CAPS {
    pub Version: COLORSPACE_TRANSFORM_TARGET_CAPS_VERSION,
    pub LookupTable1DDegammaCap: COLORSPACE_TRANSFORM_1DLUT_CAP,
    pub ColorMatrix3x3Cap: COLORSPACE_TRANSFORM_MATRIX_CAP,
    pub LookupTable1DRegammaCap: COLORSPACE_TRANSFORM_1DLUT_CAP,
}
impl COLORSPACE_TRANSFORM_TARGET_CAPS {}
impl ::core::default::Default for COLORSPACE_TRANSFORM_TARGET_CAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for COLORSPACE_TRANSFORM_TARGET_CAPS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for COLORSPACE_TRANSFORM_TARGET_CAPS {}
unsafe impl ::windows::core::Abi for COLORSPACE_TRANSFORM_TARGET_CAPS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct COLORSPACE_TRANSFORM_TARGET_CAPS_VERSION(pub i32);
pub const COLORSPACE_TRANSFORM_VERSION_DEFAULT: COLORSPACE_TRANSFORM_TARGET_CAPS_VERSION = COLORSPACE_TRANSFORM_TARGET_CAPS_VERSION(0i32);
pub const COLORSPACE_TRANSFORM_VERSION_1: COLORSPACE_TRANSFORM_TARGET_CAPS_VERSION = COLORSPACE_TRANSFORM_TARGET_CAPS_VERSION(1i32);
pub const COLORSPACE_TRANSFORM_VERSION_NOT_SUPPORTED: COLORSPACE_TRANSFORM_TARGET_CAPS_VERSION = COLORSPACE_TRANSFORM_TARGET_CAPS_VERSION(0i32);
impl ::core::convert::From<i32> for COLORSPACE_TRANSFORM_TARGET_CAPS_VERSION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for COLORSPACE_TRANSFORM_TARGET_CAPS_VERSION {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct COLORSPACE_TRANSFORM_TYPE(pub i32);
pub const COLORSPACE_TRANSFORM_TYPE_UNINITIALIZED: COLORSPACE_TRANSFORM_TYPE = COLORSPACE_TRANSFORM_TYPE(0i32);
pub const COLORSPACE_TRANSFORM_TYPE_DEFAULT: COLORSPACE_TRANSFORM_TYPE = COLORSPACE_TRANSFORM_TYPE(1i32);
pub const COLORSPACE_TRANSFORM_TYPE_RGB256x3x16: COLORSPACE_TRANSFORM_TYPE = COLORSPACE_TRANSFORM_TYPE(2i32);
pub const COLORSPACE_TRANSFORM_TYPE_DXGI_1: COLORSPACE_TRANSFORM_TYPE = COLORSPACE_TRANSFORM_TYPE(3i32);
pub const COLORSPACE_TRANSFORM_TYPE_MATRIX_3x4: COLORSPACE_TRANSFORM_TYPE = COLORSPACE_TRANSFORM_TYPE(4i32);
pub const COLORSPACE_TRANSFORM_TYPE_MATRIX_V2: COLORSPACE_TRANSFORM_TYPE = COLORSPACE_TRANSFORM_TYPE(5i32);
impl ::core::convert::From<i32> for COLORSPACE_TRANSFORM_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for COLORSPACE_TRANSFORM_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const CT_RECTANGLES: i32 = 0i32;
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CapabilitiesRequestAndCapabilitiesReply<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hmonitor: Param0, pszasciicapabilitiesstring: super::super::Foundation::PSTR, dwcapabilitiesstringlengthincharacters: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CapabilitiesRequestAndCapabilitiesReply(hmonitor: super::super::Foundation::HANDLE, pszasciicapabilitiesstring: super::super::Foundation::PSTR, dwcapabilitiesstringlengthincharacters: u32) -> i32;
        }
        ::core::mem::transmute(CapabilitiesRequestAndCapabilitiesReply(hmonitor.into_param().abi(), ::core::mem::transmute(pszasciicapabilitiesstring), ::core::mem::transmute(dwcapabilitiesstringlengthincharacters)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const DCR_DRIVER: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const DCR_HALFTONE: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const DCR_SOLID: u32 = 0u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const DC_COMPLEX: u32 = 3u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const DC_RECT: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const DC_TRIVIAL: u32 = 0u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const DDI_DRIVER_VERSION_NT4: u32 = 131072u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const DDI_DRIVER_VERSION_NT5: u32 = 196608u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const DDI_DRIVER_VERSION_NT5_01: u32 = 196864u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const DDI_DRIVER_VERSION_NT5_01_SP1: u32 = 196865u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const DDI_DRIVER_VERSION_SP3: u32 = 131075u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const DDI_ERROR: u32 = 4294967295u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct DEVHTADJDATA {
    pub DeviceFlags: u32,
    pub DeviceXDPI: u32,
    pub DeviceYDPI: u32,
    pub pDefHTInfo: *mut DEVHTINFO,
    pub pAdjHTInfo: *mut DEVHTINFO,
}
impl DEVHTADJDATA {}
impl ::core::default::Default for DEVHTADJDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DEVHTADJDATA {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DEVHTADJDATA").field("DeviceFlags", &self.DeviceFlags).field("DeviceXDPI", &self.DeviceXDPI).field("DeviceYDPI", &self.DeviceYDPI).field("pDefHTInfo", &self.pDefHTInfo).field("pAdjHTInfo", &self.pAdjHTInfo).finish()
    }
}
impl ::core::cmp::PartialEq for DEVHTADJDATA {
    fn eq(&self, other: &Self) -> bool {
        self.DeviceFlags == other.DeviceFlags && self.DeviceXDPI == other.DeviceXDPI && self.DeviceYDPI == other.DeviceYDPI && self.pDefHTInfo == other.pDefHTInfo && self.pAdjHTInfo == other.pAdjHTInfo
    }
}
impl ::core::cmp::Eq for DEVHTADJDATA {}
unsafe impl ::windows::core::Abi for DEVHTADJDATA {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const DEVHTADJF_ADDITIVE_DEVICE: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const DEVHTADJF_COLOR_DEVICE: u32 = 1u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct DEVHTINFO {
    pub HTFlags: u32,
    pub HTPatternSize: u32,
    pub DevPelsDPI: u32,
    pub ColorInfo: COLORINFO,
}
impl DEVHTINFO {}
impl ::core::default::Default for DEVHTINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DEVHTINFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DEVHTINFO").field("HTFlags", &self.HTFlags).field("HTPatternSize", &self.HTPatternSize).field("DevPelsDPI", &self.DevPelsDPI).field("ColorInfo", &self.ColorInfo).finish()
    }
}
impl ::core::cmp::PartialEq for DEVHTINFO {
    fn eq(&self, other: &Self) -> bool {
        self.HTFlags == other.HTFlags && self.HTPatternSize == other.HTPatternSize && self.DevPelsDPI == other.DevPelsDPI && self.ColorInfo == other.ColorInfo
    }
}
impl ::core::cmp::Eq for DEVHTINFO {}
unsafe impl ::windows::core::Abi for DEVHTINFO {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Graphics_Gdi`*"]
pub struct DEVINFO {
    pub flGraphicsCaps: u32,
    pub lfDefaultFont: super::super::Graphics::Gdi::LOGFONTW,
    pub lfAnsiVarFont: super::super::Graphics::Gdi::LOGFONTW,
    pub lfAnsiFixFont: super::super::Graphics::Gdi::LOGFONTW,
    pub cFonts: u32,
    pub iDitherFormat: u32,
    pub cxDither: u16,
    pub cyDither: u16,
    pub hpalDefault: super::super::Graphics::Gdi::HPALETTE,
    pub flGraphicsCaps2: u32,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl DEVINFO {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for DEVINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::fmt::Debug for DEVINFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DEVINFO")
            .field("flGraphicsCaps", &self.flGraphicsCaps)
            .field("lfDefaultFont", &self.lfDefaultFont)
            .field("lfAnsiVarFont", &self.lfAnsiVarFont)
            .field("lfAnsiFixFont", &self.lfAnsiFixFont)
            .field("cFonts", &self.cFonts)
            .field("iDitherFormat", &self.iDitherFormat)
            .field("cxDither", &self.cxDither)
            .field("cyDither", &self.cyDither)
            .field("hpalDefault", &self.hpalDefault)
            .field("flGraphicsCaps2", &self.flGraphicsCaps2)
            .finish()
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
unsafe impl ::windows::core::Abi for DEVINFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const DEVPKEY_Device_ActivityId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xc50a3f10_aa5c_4247_b830_d6a6f8eaa310), pid: 4u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const DEVPKEY_Device_AdapterLuid: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xc50a3f10_aa5c_4247_b830_d6a6f8eaa310), pid: 3u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const DEVPKEY_Device_TerminalLuid: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xc50a3f10_aa5c_4247_b830_d6a6f8eaa310), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const DEVPKEY_IndirectDisplay: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xc50a3f10_aa5c_4247_b830_d6a6f8eaa310), pid: 1u32 };
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
#[repr(transparent)]
pub struct DHPDEV(pub isize);
impl ::core::default::Default for DHPDEV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
unsafe impl ::windows::core::Handle for DHPDEV {}
unsafe impl ::windows::core::Abi for DHPDEV {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
#[repr(transparent)]
pub struct DHSURF(pub isize);
impl ::core::default::Default for DHSURF {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
unsafe impl ::windows::core::Handle for DHSURF {}
unsafe impl ::windows::core::Abi for DHSURF {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct DISPLAYCONFIG_2DREGION {
    pub cx: u32,
    pub cy: u32,
}
impl DISPLAYCONFIG_2DREGION {}
impl ::core::default::Default for DISPLAYCONFIG_2DREGION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DISPLAYCONFIG_2DREGION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DISPLAYCONFIG_2DREGION").field("cx", &self.cx).field("cy", &self.cy).finish()
    }
}
impl ::core::cmp::PartialEq for DISPLAYCONFIG_2DREGION {
    fn eq(&self, other: &Self) -> bool {
        self.cx == other.cx && self.cy == other.cy
    }
}
impl ::core::cmp::Eq for DISPLAYCONFIG_2DREGION {}
unsafe impl ::windows::core::Abi for DISPLAYCONFIG_2DREGION {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
pub struct DISPLAYCONFIG_ADAPTER_NAME {
    pub header: DISPLAYCONFIG_DEVICE_INFO_HEADER,
    pub adapterDevicePath: [u16; 128],
}
#[cfg(feature = "Win32_Foundation")]
impl DISPLAYCONFIG_ADAPTER_NAME {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DISPLAYCONFIG_ADAPTER_NAME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DISPLAYCONFIG_ADAPTER_NAME {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DISPLAYCONFIG_ADAPTER_NAME").field("header", &self.header).field("adapterDevicePath", &self.adapterDevicePath).finish()
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
unsafe impl ::windows::core::Abi for DISPLAYCONFIG_ADAPTER_NAME {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
pub struct DISPLAYCONFIG_DESKTOP_IMAGE_INFO {
    pub PathSourceSize: super::super::Foundation::POINTL,
    pub DesktopImageRegion: super::super::Foundation::RECTL,
    pub DesktopImageClip: super::super::Foundation::RECTL,
}
#[cfg(feature = "Win32_Foundation")]
impl DISPLAYCONFIG_DESKTOP_IMAGE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DISPLAYCONFIG_DESKTOP_IMAGE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DISPLAYCONFIG_DESKTOP_IMAGE_INFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DISPLAYCONFIG_DESKTOP_IMAGE_INFO").field("PathSourceSize", &self.PathSourceSize).field("DesktopImageRegion", &self.DesktopImageRegion).field("DesktopImageClip", &self.DesktopImageClip).finish()
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
unsafe impl ::windows::core::Abi for DISPLAYCONFIG_DESKTOP_IMAGE_INFO {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
pub struct DISPLAYCONFIG_DEVICE_INFO_HEADER {
    pub r#type: DISPLAYCONFIG_DEVICE_INFO_TYPE,
    pub size: u32,
    pub adapterId: super::super::Foundation::LUID,
    pub id: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl DISPLAYCONFIG_DEVICE_INFO_HEADER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DISPLAYCONFIG_DEVICE_INFO_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DISPLAYCONFIG_DEVICE_INFO_HEADER {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DISPLAYCONFIG_DEVICE_INFO_HEADER").field("r#type", &self.r#type).field("size", &self.size).field("adapterId", &self.adapterId).field("id", &self.id).finish()
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
unsafe impl ::windows::core::Abi for DISPLAYCONFIG_DEVICE_INFO_HEADER {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DISPLAYCONFIG_DEVICE_INFO_TYPE(pub i32);
pub const DISPLAYCONFIG_DEVICE_INFO_GET_SOURCE_NAME: DISPLAYCONFIG_DEVICE_INFO_TYPE = DISPLAYCONFIG_DEVICE_INFO_TYPE(1i32);
pub const DISPLAYCONFIG_DEVICE_INFO_GET_TARGET_NAME: DISPLAYCONFIG_DEVICE_INFO_TYPE = DISPLAYCONFIG_DEVICE_INFO_TYPE(2i32);
pub const DISPLAYCONFIG_DEVICE_INFO_GET_TARGET_PREFERRED_MODE: DISPLAYCONFIG_DEVICE_INFO_TYPE = DISPLAYCONFIG_DEVICE_INFO_TYPE(3i32);
pub const DISPLAYCONFIG_DEVICE_INFO_GET_ADAPTER_NAME: DISPLAYCONFIG_DEVICE_INFO_TYPE = DISPLAYCONFIG_DEVICE_INFO_TYPE(4i32);
pub const DISPLAYCONFIG_DEVICE_INFO_SET_TARGET_PERSISTENCE: DISPLAYCONFIG_DEVICE_INFO_TYPE = DISPLAYCONFIG_DEVICE_INFO_TYPE(5i32);
pub const DISPLAYCONFIG_DEVICE_INFO_GET_TARGET_BASE_TYPE: DISPLAYCONFIG_DEVICE_INFO_TYPE = DISPLAYCONFIG_DEVICE_INFO_TYPE(6i32);
pub const DISPLAYCONFIG_DEVICE_INFO_GET_SUPPORT_VIRTUAL_RESOLUTION: DISPLAYCONFIG_DEVICE_INFO_TYPE = DISPLAYCONFIG_DEVICE_INFO_TYPE(7i32);
pub const DISPLAYCONFIG_DEVICE_INFO_SET_SUPPORT_VIRTUAL_RESOLUTION: DISPLAYCONFIG_DEVICE_INFO_TYPE = DISPLAYCONFIG_DEVICE_INFO_TYPE(8i32);
pub const DISPLAYCONFIG_DEVICE_INFO_GET_ADVANCED_COLOR_INFO: DISPLAYCONFIG_DEVICE_INFO_TYPE = DISPLAYCONFIG_DEVICE_INFO_TYPE(9i32);
pub const DISPLAYCONFIG_DEVICE_INFO_SET_ADVANCED_COLOR_STATE: DISPLAYCONFIG_DEVICE_INFO_TYPE = DISPLAYCONFIG_DEVICE_INFO_TYPE(10i32);
pub const DISPLAYCONFIG_DEVICE_INFO_GET_SDR_WHITE_LEVEL: DISPLAYCONFIG_DEVICE_INFO_TYPE = DISPLAYCONFIG_DEVICE_INFO_TYPE(11i32);
pub const DISPLAYCONFIG_DEVICE_INFO_GET_MONITOR_SPECIALIZATION: DISPLAYCONFIG_DEVICE_INFO_TYPE = DISPLAYCONFIG_DEVICE_INFO_TYPE(12i32);
pub const DISPLAYCONFIG_DEVICE_INFO_SET_MONITOR_SPECIALIZATION: DISPLAYCONFIG_DEVICE_INFO_TYPE = DISPLAYCONFIG_DEVICE_INFO_TYPE(13i32);
pub const DISPLAYCONFIG_DEVICE_INFO_FORCE_UINT32: DISPLAYCONFIG_DEVICE_INFO_TYPE = DISPLAYCONFIG_DEVICE_INFO_TYPE(-1i32);
impl ::core::convert::From<i32> for DISPLAYCONFIG_DEVICE_INFO_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DISPLAYCONFIG_DEVICE_INFO_TYPE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
pub struct DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO {
    pub header: DISPLAYCONFIG_DEVICE_INFO_HEADER,
    pub Anonymous: DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO_0,
    pub colorEncoding: super::super::Graphics::Gdi::DISPLAYCONFIG_COLOR_ENCODING,
    pub bitsPerColorChannel: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::core::Abi for DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub union DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO_0 {
    pub Anonymous: DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO_0_0,
    pub value: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::core::Abi for DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO_0_0 {
    pub _bitfield: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO_0_0 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("_bitfield", &self._bitfield).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::core::Abi for DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO_0_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
pub struct DISPLAYCONFIG_GET_MONITOR_SPECIALIZATION {
    pub header: DISPLAYCONFIG_DEVICE_INFO_HEADER,
    pub Anonymous: DISPLAYCONFIG_GET_MONITOR_SPECIALIZATION_0,
}
#[cfg(feature = "Win32_Foundation")]
impl DISPLAYCONFIG_GET_MONITOR_SPECIALIZATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DISPLAYCONFIG_GET_MONITOR_SPECIALIZATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DISPLAYCONFIG_GET_MONITOR_SPECIALIZATION {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DISPLAYCONFIG_GET_MONITOR_SPECIALIZATION {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DISPLAYCONFIG_GET_MONITOR_SPECIALIZATION {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union DISPLAYCONFIG_GET_MONITOR_SPECIALIZATION_0 {
    pub Anonymous: DISPLAYCONFIG_GET_MONITOR_SPECIALIZATION_0_0,
    pub value: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl DISPLAYCONFIG_GET_MONITOR_SPECIALIZATION_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DISPLAYCONFIG_GET_MONITOR_SPECIALIZATION_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DISPLAYCONFIG_GET_MONITOR_SPECIALIZATION_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DISPLAYCONFIG_GET_MONITOR_SPECIALIZATION_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DISPLAYCONFIG_GET_MONITOR_SPECIALIZATION_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DISPLAYCONFIG_GET_MONITOR_SPECIALIZATION_0_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl DISPLAYCONFIG_GET_MONITOR_SPECIALIZATION_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DISPLAYCONFIG_GET_MONITOR_SPECIALIZATION_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DISPLAYCONFIG_GET_MONITOR_SPECIALIZATION_0_0 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("_bitfield", &self._bitfield).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DISPLAYCONFIG_GET_MONITOR_SPECIALIZATION_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DISPLAYCONFIG_GET_MONITOR_SPECIALIZATION_0_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DISPLAYCONFIG_GET_MONITOR_SPECIALIZATION_0_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
pub struct DISPLAYCONFIG_MODE_INFO {
    pub infoType: DISPLAYCONFIG_MODE_INFO_TYPE,
    pub id: u32,
    pub adapterId: super::super::Foundation::LUID,
    pub Anonymous: DISPLAYCONFIG_MODE_INFO_0,
}
#[cfg(feature = "Win32_Foundation")]
impl DISPLAYCONFIG_MODE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DISPLAYCONFIG_MODE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DISPLAYCONFIG_MODE_INFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DISPLAYCONFIG_MODE_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DISPLAYCONFIG_MODE_INFO {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union DISPLAYCONFIG_MODE_INFO_0 {
    pub targetMode: DISPLAYCONFIG_TARGET_MODE,
    pub sourceMode: DISPLAYCONFIG_SOURCE_MODE,
    pub desktopImageInfo: DISPLAYCONFIG_DESKTOP_IMAGE_INFO,
}
#[cfg(feature = "Win32_Foundation")]
impl DISPLAYCONFIG_MODE_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DISPLAYCONFIG_MODE_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DISPLAYCONFIG_MODE_INFO_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DISPLAYCONFIG_MODE_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DISPLAYCONFIG_MODE_INFO_0 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DISPLAYCONFIG_MODE_INFO_TYPE(pub i32);
pub const DISPLAYCONFIG_MODE_INFO_TYPE_SOURCE: DISPLAYCONFIG_MODE_INFO_TYPE = DISPLAYCONFIG_MODE_INFO_TYPE(1i32);
pub const DISPLAYCONFIG_MODE_INFO_TYPE_TARGET: DISPLAYCONFIG_MODE_INFO_TYPE = DISPLAYCONFIG_MODE_INFO_TYPE(2i32);
pub const DISPLAYCONFIG_MODE_INFO_TYPE_DESKTOP_IMAGE: DISPLAYCONFIG_MODE_INFO_TYPE = DISPLAYCONFIG_MODE_INFO_TYPE(3i32);
pub const DISPLAYCONFIG_MODE_INFO_TYPE_FORCE_UINT32: DISPLAYCONFIG_MODE_INFO_TYPE = DISPLAYCONFIG_MODE_INFO_TYPE(-1i32);
impl ::core::convert::From<i32> for DISPLAYCONFIG_MODE_INFO_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DISPLAYCONFIG_MODE_INFO_TYPE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
pub struct DISPLAYCONFIG_PATH_INFO {
    pub sourceInfo: DISPLAYCONFIG_PATH_SOURCE_INFO,
    pub targetInfo: DISPLAYCONFIG_PATH_TARGET_INFO,
    pub flags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl DISPLAYCONFIG_PATH_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DISPLAYCONFIG_PATH_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DISPLAYCONFIG_PATH_INFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DISPLAYCONFIG_PATH_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DISPLAYCONFIG_PATH_INFO {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
pub struct DISPLAYCONFIG_PATH_SOURCE_INFO {
    pub adapterId: super::super::Foundation::LUID,
    pub id: u32,
    pub Anonymous: DISPLAYCONFIG_PATH_SOURCE_INFO_0,
    pub statusFlags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl DISPLAYCONFIG_PATH_SOURCE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DISPLAYCONFIG_PATH_SOURCE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DISPLAYCONFIG_PATH_SOURCE_INFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DISPLAYCONFIG_PATH_SOURCE_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DISPLAYCONFIG_PATH_SOURCE_INFO {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union DISPLAYCONFIG_PATH_SOURCE_INFO_0 {
    pub modeInfoIdx: u32,
    pub Anonymous: DISPLAYCONFIG_PATH_SOURCE_INFO_0_0,
}
#[cfg(feature = "Win32_Foundation")]
impl DISPLAYCONFIG_PATH_SOURCE_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DISPLAYCONFIG_PATH_SOURCE_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DISPLAYCONFIG_PATH_SOURCE_INFO_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DISPLAYCONFIG_PATH_SOURCE_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DISPLAYCONFIG_PATH_SOURCE_INFO_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DISPLAYCONFIG_PATH_SOURCE_INFO_0_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl DISPLAYCONFIG_PATH_SOURCE_INFO_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DISPLAYCONFIG_PATH_SOURCE_INFO_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DISPLAYCONFIG_PATH_SOURCE_INFO_0_0 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("_bitfield", &self._bitfield).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DISPLAYCONFIG_PATH_SOURCE_INFO_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DISPLAYCONFIG_PATH_SOURCE_INFO_0_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DISPLAYCONFIG_PATH_SOURCE_INFO_0_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
pub struct DISPLAYCONFIG_PATH_TARGET_INFO {
    pub adapterId: super::super::Foundation::LUID,
    pub id: u32,
    pub Anonymous: DISPLAYCONFIG_PATH_TARGET_INFO_0,
    pub outputTechnology: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY,
    pub rotation: DISPLAYCONFIG_ROTATION,
    pub scaling: DISPLAYCONFIG_SCALING,
    pub refreshRate: DISPLAYCONFIG_RATIONAL,
    pub scanLineOrdering: DISPLAYCONFIG_SCANLINE_ORDERING,
    pub targetAvailable: super::super::Foundation::BOOL,
    pub statusFlags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl DISPLAYCONFIG_PATH_TARGET_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DISPLAYCONFIG_PATH_TARGET_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DISPLAYCONFIG_PATH_TARGET_INFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DISPLAYCONFIG_PATH_TARGET_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DISPLAYCONFIG_PATH_TARGET_INFO {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union DISPLAYCONFIG_PATH_TARGET_INFO_0 {
    pub modeInfoIdx: u32,
    pub Anonymous: DISPLAYCONFIG_PATH_TARGET_INFO_0_0,
}
#[cfg(feature = "Win32_Foundation")]
impl DISPLAYCONFIG_PATH_TARGET_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DISPLAYCONFIG_PATH_TARGET_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DISPLAYCONFIG_PATH_TARGET_INFO_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DISPLAYCONFIG_PATH_TARGET_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DISPLAYCONFIG_PATH_TARGET_INFO_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DISPLAYCONFIG_PATH_TARGET_INFO_0_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl DISPLAYCONFIG_PATH_TARGET_INFO_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DISPLAYCONFIG_PATH_TARGET_INFO_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DISPLAYCONFIG_PATH_TARGET_INFO_0_0 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("_bitfield", &self._bitfield).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DISPLAYCONFIG_PATH_TARGET_INFO_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DISPLAYCONFIG_PATH_TARGET_INFO_0_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DISPLAYCONFIG_PATH_TARGET_INFO_0_0 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DISPLAYCONFIG_PIXELFORMAT(pub i32);
pub const DISPLAYCONFIG_PIXELFORMAT_8BPP: DISPLAYCONFIG_PIXELFORMAT = DISPLAYCONFIG_PIXELFORMAT(1i32);
pub const DISPLAYCONFIG_PIXELFORMAT_16BPP: DISPLAYCONFIG_PIXELFORMAT = DISPLAYCONFIG_PIXELFORMAT(2i32);
pub const DISPLAYCONFIG_PIXELFORMAT_24BPP: DISPLAYCONFIG_PIXELFORMAT = DISPLAYCONFIG_PIXELFORMAT(3i32);
pub const DISPLAYCONFIG_PIXELFORMAT_32BPP: DISPLAYCONFIG_PIXELFORMAT = DISPLAYCONFIG_PIXELFORMAT(4i32);
pub const DISPLAYCONFIG_PIXELFORMAT_NONGDI: DISPLAYCONFIG_PIXELFORMAT = DISPLAYCONFIG_PIXELFORMAT(5i32);
pub const DISPLAYCONFIG_PIXELFORMAT_FORCE_UINT32: DISPLAYCONFIG_PIXELFORMAT = DISPLAYCONFIG_PIXELFORMAT(-1i32);
impl ::core::convert::From<i32> for DISPLAYCONFIG_PIXELFORMAT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DISPLAYCONFIG_PIXELFORMAT {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct DISPLAYCONFIG_RATIONAL {
    pub Numerator: u32,
    pub Denominator: u32,
}
impl DISPLAYCONFIG_RATIONAL {}
impl ::core::default::Default for DISPLAYCONFIG_RATIONAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DISPLAYCONFIG_RATIONAL {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DISPLAYCONFIG_RATIONAL").field("Numerator", &self.Numerator).field("Denominator", &self.Denominator).finish()
    }
}
impl ::core::cmp::PartialEq for DISPLAYCONFIG_RATIONAL {
    fn eq(&self, other: &Self) -> bool {
        self.Numerator == other.Numerator && self.Denominator == other.Denominator
    }
}
impl ::core::cmp::Eq for DISPLAYCONFIG_RATIONAL {}
unsafe impl ::windows::core::Abi for DISPLAYCONFIG_RATIONAL {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DISPLAYCONFIG_ROTATION(pub i32);
pub const DISPLAYCONFIG_ROTATION_IDENTITY: DISPLAYCONFIG_ROTATION = DISPLAYCONFIG_ROTATION(1i32);
pub const DISPLAYCONFIG_ROTATION_ROTATE90: DISPLAYCONFIG_ROTATION = DISPLAYCONFIG_ROTATION(2i32);
pub const DISPLAYCONFIG_ROTATION_ROTATE180: DISPLAYCONFIG_ROTATION = DISPLAYCONFIG_ROTATION(3i32);
pub const DISPLAYCONFIG_ROTATION_ROTATE270: DISPLAYCONFIG_ROTATION = DISPLAYCONFIG_ROTATION(4i32);
pub const DISPLAYCONFIG_ROTATION_FORCE_UINT32: DISPLAYCONFIG_ROTATION = DISPLAYCONFIG_ROTATION(-1i32);
impl ::core::convert::From<i32> for DISPLAYCONFIG_ROTATION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DISPLAYCONFIG_ROTATION {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DISPLAYCONFIG_SCALING(pub i32);
pub const DISPLAYCONFIG_SCALING_IDENTITY: DISPLAYCONFIG_SCALING = DISPLAYCONFIG_SCALING(1i32);
pub const DISPLAYCONFIG_SCALING_CENTERED: DISPLAYCONFIG_SCALING = DISPLAYCONFIG_SCALING(2i32);
pub const DISPLAYCONFIG_SCALING_STRETCHED: DISPLAYCONFIG_SCALING = DISPLAYCONFIG_SCALING(3i32);
pub const DISPLAYCONFIG_SCALING_ASPECTRATIOCENTEREDMAX: DISPLAYCONFIG_SCALING = DISPLAYCONFIG_SCALING(4i32);
pub const DISPLAYCONFIG_SCALING_CUSTOM: DISPLAYCONFIG_SCALING = DISPLAYCONFIG_SCALING(5i32);
pub const DISPLAYCONFIG_SCALING_PREFERRED: DISPLAYCONFIG_SCALING = DISPLAYCONFIG_SCALING(128i32);
pub const DISPLAYCONFIG_SCALING_FORCE_UINT32: DISPLAYCONFIG_SCALING = DISPLAYCONFIG_SCALING(-1i32);
impl ::core::convert::From<i32> for DISPLAYCONFIG_SCALING {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DISPLAYCONFIG_SCALING {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DISPLAYCONFIG_SCANLINE_ORDERING(pub i32);
pub const DISPLAYCONFIG_SCANLINE_ORDERING_UNSPECIFIED: DISPLAYCONFIG_SCANLINE_ORDERING = DISPLAYCONFIG_SCANLINE_ORDERING(0i32);
pub const DISPLAYCONFIG_SCANLINE_ORDERING_PROGRESSIVE: DISPLAYCONFIG_SCANLINE_ORDERING = DISPLAYCONFIG_SCANLINE_ORDERING(1i32);
pub const DISPLAYCONFIG_SCANLINE_ORDERING_INTERLACED: DISPLAYCONFIG_SCANLINE_ORDERING = DISPLAYCONFIG_SCANLINE_ORDERING(2i32);
pub const DISPLAYCONFIG_SCANLINE_ORDERING_INTERLACED_UPPERFIELDFIRST: DISPLAYCONFIG_SCANLINE_ORDERING = DISPLAYCONFIG_SCANLINE_ORDERING(2i32);
pub const DISPLAYCONFIG_SCANLINE_ORDERING_INTERLACED_LOWERFIELDFIRST: DISPLAYCONFIG_SCANLINE_ORDERING = DISPLAYCONFIG_SCANLINE_ORDERING(3i32);
pub const DISPLAYCONFIG_SCANLINE_ORDERING_FORCE_UINT32: DISPLAYCONFIG_SCANLINE_ORDERING = DISPLAYCONFIG_SCANLINE_ORDERING(-1i32);
impl ::core::convert::From<i32> for DISPLAYCONFIG_SCANLINE_ORDERING {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DISPLAYCONFIG_SCANLINE_ORDERING {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
pub struct DISPLAYCONFIG_SDR_WHITE_LEVEL {
    pub header: DISPLAYCONFIG_DEVICE_INFO_HEADER,
    pub SDRWhiteLevel: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl DISPLAYCONFIG_SDR_WHITE_LEVEL {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DISPLAYCONFIG_SDR_WHITE_LEVEL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DISPLAYCONFIG_SDR_WHITE_LEVEL {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DISPLAYCONFIG_SDR_WHITE_LEVEL").field("header", &self.header).field("SDRWhiteLevel", &self.SDRWhiteLevel).finish()
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
unsafe impl ::windows::core::Abi for DISPLAYCONFIG_SDR_WHITE_LEVEL {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
pub struct DISPLAYCONFIG_SET_ADVANCED_COLOR_STATE {
    pub header: DISPLAYCONFIG_DEVICE_INFO_HEADER,
    pub Anonymous: DISPLAYCONFIG_SET_ADVANCED_COLOR_STATE_0,
}
#[cfg(feature = "Win32_Foundation")]
impl DISPLAYCONFIG_SET_ADVANCED_COLOR_STATE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DISPLAYCONFIG_SET_ADVANCED_COLOR_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DISPLAYCONFIG_SET_ADVANCED_COLOR_STATE {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DISPLAYCONFIG_SET_ADVANCED_COLOR_STATE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DISPLAYCONFIG_SET_ADVANCED_COLOR_STATE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union DISPLAYCONFIG_SET_ADVANCED_COLOR_STATE_0 {
    pub Anonymous: DISPLAYCONFIG_SET_ADVANCED_COLOR_STATE_0_0,
    pub value: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl DISPLAYCONFIG_SET_ADVANCED_COLOR_STATE_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DISPLAYCONFIG_SET_ADVANCED_COLOR_STATE_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DISPLAYCONFIG_SET_ADVANCED_COLOR_STATE_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DISPLAYCONFIG_SET_ADVANCED_COLOR_STATE_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DISPLAYCONFIG_SET_ADVANCED_COLOR_STATE_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DISPLAYCONFIG_SET_ADVANCED_COLOR_STATE_0_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl DISPLAYCONFIG_SET_ADVANCED_COLOR_STATE_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DISPLAYCONFIG_SET_ADVANCED_COLOR_STATE_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DISPLAYCONFIG_SET_ADVANCED_COLOR_STATE_0_0 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("_bitfield", &self._bitfield).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DISPLAYCONFIG_SET_ADVANCED_COLOR_STATE_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DISPLAYCONFIG_SET_ADVANCED_COLOR_STATE_0_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DISPLAYCONFIG_SET_ADVANCED_COLOR_STATE_0_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
pub struct DISPLAYCONFIG_SET_MONITOR_SPECIALIZATION {
    pub header: DISPLAYCONFIG_DEVICE_INFO_HEADER,
    pub Anonymous: DISPLAYCONFIG_SET_MONITOR_SPECIALIZATION_0,
    pub specializationType: ::windows::core::GUID,
    pub specializationSubType: ::windows::core::GUID,
    pub specializationApplicationName: [u16; 128],
}
#[cfg(feature = "Win32_Foundation")]
impl DISPLAYCONFIG_SET_MONITOR_SPECIALIZATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DISPLAYCONFIG_SET_MONITOR_SPECIALIZATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DISPLAYCONFIG_SET_MONITOR_SPECIALIZATION {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DISPLAYCONFIG_SET_MONITOR_SPECIALIZATION {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DISPLAYCONFIG_SET_MONITOR_SPECIALIZATION {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union DISPLAYCONFIG_SET_MONITOR_SPECIALIZATION_0 {
    pub Anonymous: DISPLAYCONFIG_SET_MONITOR_SPECIALIZATION_0_0,
    pub value: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl DISPLAYCONFIG_SET_MONITOR_SPECIALIZATION_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DISPLAYCONFIG_SET_MONITOR_SPECIALIZATION_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DISPLAYCONFIG_SET_MONITOR_SPECIALIZATION_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DISPLAYCONFIG_SET_MONITOR_SPECIALIZATION_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DISPLAYCONFIG_SET_MONITOR_SPECIALIZATION_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DISPLAYCONFIG_SET_MONITOR_SPECIALIZATION_0_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl DISPLAYCONFIG_SET_MONITOR_SPECIALIZATION_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DISPLAYCONFIG_SET_MONITOR_SPECIALIZATION_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DISPLAYCONFIG_SET_MONITOR_SPECIALIZATION_0_0 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("_bitfield", &self._bitfield).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DISPLAYCONFIG_SET_MONITOR_SPECIALIZATION_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DISPLAYCONFIG_SET_MONITOR_SPECIALIZATION_0_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DISPLAYCONFIG_SET_MONITOR_SPECIALIZATION_0_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
pub struct DISPLAYCONFIG_SET_TARGET_PERSISTENCE {
    pub header: DISPLAYCONFIG_DEVICE_INFO_HEADER,
    pub Anonymous: DISPLAYCONFIG_SET_TARGET_PERSISTENCE_0,
}
#[cfg(feature = "Win32_Foundation")]
impl DISPLAYCONFIG_SET_TARGET_PERSISTENCE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DISPLAYCONFIG_SET_TARGET_PERSISTENCE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DISPLAYCONFIG_SET_TARGET_PERSISTENCE {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DISPLAYCONFIG_SET_TARGET_PERSISTENCE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DISPLAYCONFIG_SET_TARGET_PERSISTENCE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union DISPLAYCONFIG_SET_TARGET_PERSISTENCE_0 {
    pub Anonymous: DISPLAYCONFIG_SET_TARGET_PERSISTENCE_0_0,
    pub value: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl DISPLAYCONFIG_SET_TARGET_PERSISTENCE_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DISPLAYCONFIG_SET_TARGET_PERSISTENCE_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DISPLAYCONFIG_SET_TARGET_PERSISTENCE_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DISPLAYCONFIG_SET_TARGET_PERSISTENCE_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DISPLAYCONFIG_SET_TARGET_PERSISTENCE_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DISPLAYCONFIG_SET_TARGET_PERSISTENCE_0_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl DISPLAYCONFIG_SET_TARGET_PERSISTENCE_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DISPLAYCONFIG_SET_TARGET_PERSISTENCE_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DISPLAYCONFIG_SET_TARGET_PERSISTENCE_0_0 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("_bitfield", &self._bitfield).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DISPLAYCONFIG_SET_TARGET_PERSISTENCE_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DISPLAYCONFIG_SET_TARGET_PERSISTENCE_0_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DISPLAYCONFIG_SET_TARGET_PERSISTENCE_0_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
pub struct DISPLAYCONFIG_SOURCE_DEVICE_NAME {
    pub header: DISPLAYCONFIG_DEVICE_INFO_HEADER,
    pub viewGdiDeviceName: [u16; 32],
}
#[cfg(feature = "Win32_Foundation")]
impl DISPLAYCONFIG_SOURCE_DEVICE_NAME {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DISPLAYCONFIG_SOURCE_DEVICE_NAME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DISPLAYCONFIG_SOURCE_DEVICE_NAME {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DISPLAYCONFIG_SOURCE_DEVICE_NAME").field("header", &self.header).field("viewGdiDeviceName", &self.viewGdiDeviceName).finish()
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
unsafe impl ::windows::core::Abi for DISPLAYCONFIG_SOURCE_DEVICE_NAME {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
pub struct DISPLAYCONFIG_SOURCE_MODE {
    pub width: u32,
    pub height: u32,
    pub pixelFormat: DISPLAYCONFIG_PIXELFORMAT,
    pub position: super::super::Foundation::POINTL,
}
#[cfg(feature = "Win32_Foundation")]
impl DISPLAYCONFIG_SOURCE_MODE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DISPLAYCONFIG_SOURCE_MODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DISPLAYCONFIG_SOURCE_MODE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DISPLAYCONFIG_SOURCE_MODE").field("width", &self.width).field("height", &self.height).field("pixelFormat", &self.pixelFormat).field("position", &self.position).finish()
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
unsafe impl ::windows::core::Abi for DISPLAYCONFIG_SOURCE_MODE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
pub struct DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION {
    pub header: DISPLAYCONFIG_DEVICE_INFO_HEADER,
    pub Anonymous: DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION_0,
}
#[cfg(feature = "Win32_Foundation")]
impl DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION_0 {
    pub Anonymous: DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION_0_0,
    pub value: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION_0_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION_0_0 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("_bitfield", &self._bitfield).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION_0_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION_0_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
pub struct DISPLAYCONFIG_TARGET_BASE_TYPE {
    pub header: DISPLAYCONFIG_DEVICE_INFO_HEADER,
    pub baseOutputTechnology: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY,
}
#[cfg(feature = "Win32_Foundation")]
impl DISPLAYCONFIG_TARGET_BASE_TYPE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DISPLAYCONFIG_TARGET_BASE_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DISPLAYCONFIG_TARGET_BASE_TYPE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DISPLAYCONFIG_TARGET_BASE_TYPE").field("header", &self.header).field("baseOutputTechnology", &self.baseOutputTechnology).finish()
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
unsafe impl ::windows::core::Abi for DISPLAYCONFIG_TARGET_BASE_TYPE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
pub struct DISPLAYCONFIG_TARGET_DEVICE_NAME {
    pub header: DISPLAYCONFIG_DEVICE_INFO_HEADER,
    pub flags: DISPLAYCONFIG_TARGET_DEVICE_NAME_FLAGS,
    pub outputTechnology: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY,
    pub edidManufactureId: u16,
    pub edidProductCodeId: u16,
    pub connectorInstance: u32,
    pub monitorFriendlyDeviceName: [u16; 64],
    pub monitorDevicePath: [u16; 128],
}
#[cfg(feature = "Win32_Foundation")]
impl DISPLAYCONFIG_TARGET_DEVICE_NAME {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DISPLAYCONFIG_TARGET_DEVICE_NAME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DISPLAYCONFIG_TARGET_DEVICE_NAME {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DISPLAYCONFIG_TARGET_DEVICE_NAME {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DISPLAYCONFIG_TARGET_DEVICE_NAME {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct DISPLAYCONFIG_TARGET_DEVICE_NAME_FLAGS {
    pub Anonymous: DISPLAYCONFIG_TARGET_DEVICE_NAME_FLAGS_0,
}
impl DISPLAYCONFIG_TARGET_DEVICE_NAME_FLAGS {}
impl ::core::default::Default for DISPLAYCONFIG_TARGET_DEVICE_NAME_FLAGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DISPLAYCONFIG_TARGET_DEVICE_NAME_FLAGS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for DISPLAYCONFIG_TARGET_DEVICE_NAME_FLAGS {}
unsafe impl ::windows::core::Abi for DISPLAYCONFIG_TARGET_DEVICE_NAME_FLAGS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub union DISPLAYCONFIG_TARGET_DEVICE_NAME_FLAGS_0 {
    pub Anonymous: DISPLAYCONFIG_TARGET_DEVICE_NAME_FLAGS_0_0,
    pub value: u32,
}
impl DISPLAYCONFIG_TARGET_DEVICE_NAME_FLAGS_0 {}
impl ::core::default::Default for DISPLAYCONFIG_TARGET_DEVICE_NAME_FLAGS_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DISPLAYCONFIG_TARGET_DEVICE_NAME_FLAGS_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for DISPLAYCONFIG_TARGET_DEVICE_NAME_FLAGS_0 {}
unsafe impl ::windows::core::Abi for DISPLAYCONFIG_TARGET_DEVICE_NAME_FLAGS_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct DISPLAYCONFIG_TARGET_DEVICE_NAME_FLAGS_0_0 {
    pub _bitfield: u32,
}
impl DISPLAYCONFIG_TARGET_DEVICE_NAME_FLAGS_0_0 {}
impl ::core::default::Default for DISPLAYCONFIG_TARGET_DEVICE_NAME_FLAGS_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DISPLAYCONFIG_TARGET_DEVICE_NAME_FLAGS_0_0 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::core::cmp::PartialEq for DISPLAYCONFIG_TARGET_DEVICE_NAME_FLAGS_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for DISPLAYCONFIG_TARGET_DEVICE_NAME_FLAGS_0_0 {}
unsafe impl ::windows::core::Abi for DISPLAYCONFIG_TARGET_DEVICE_NAME_FLAGS_0_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct DISPLAYCONFIG_TARGET_MODE {
    pub targetVideoSignalInfo: DISPLAYCONFIG_VIDEO_SIGNAL_INFO,
}
impl DISPLAYCONFIG_TARGET_MODE {}
impl ::core::default::Default for DISPLAYCONFIG_TARGET_MODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DISPLAYCONFIG_TARGET_MODE {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for DISPLAYCONFIG_TARGET_MODE {}
unsafe impl ::windows::core::Abi for DISPLAYCONFIG_TARGET_MODE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
pub struct DISPLAYCONFIG_TARGET_PREFERRED_MODE {
    pub header: DISPLAYCONFIG_DEVICE_INFO_HEADER,
    pub width: u32,
    pub height: u32,
    pub targetMode: DISPLAYCONFIG_TARGET_MODE,
}
#[cfg(feature = "Win32_Foundation")]
impl DISPLAYCONFIG_TARGET_PREFERRED_MODE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DISPLAYCONFIG_TARGET_PREFERRED_MODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DISPLAYCONFIG_TARGET_PREFERRED_MODE {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DISPLAYCONFIG_TARGET_PREFERRED_MODE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DISPLAYCONFIG_TARGET_PREFERRED_MODE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DISPLAYCONFIG_TOPOLOGY_ID(pub i32);
pub const DISPLAYCONFIG_TOPOLOGY_INTERNAL: DISPLAYCONFIG_TOPOLOGY_ID = DISPLAYCONFIG_TOPOLOGY_ID(1i32);
pub const DISPLAYCONFIG_TOPOLOGY_CLONE: DISPLAYCONFIG_TOPOLOGY_ID = DISPLAYCONFIG_TOPOLOGY_ID(2i32);
pub const DISPLAYCONFIG_TOPOLOGY_EXTEND: DISPLAYCONFIG_TOPOLOGY_ID = DISPLAYCONFIG_TOPOLOGY_ID(4i32);
pub const DISPLAYCONFIG_TOPOLOGY_EXTERNAL: DISPLAYCONFIG_TOPOLOGY_ID = DISPLAYCONFIG_TOPOLOGY_ID(8i32);
pub const DISPLAYCONFIG_TOPOLOGY_FORCE_UINT32: DISPLAYCONFIG_TOPOLOGY_ID = DISPLAYCONFIG_TOPOLOGY_ID(-1i32);
impl ::core::convert::From<i32> for DISPLAYCONFIG_TOPOLOGY_ID {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DISPLAYCONFIG_TOPOLOGY_ID {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY(pub i32);
pub const DISPLAYCONFIG_OUTPUT_TECHNOLOGY_OTHER: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY = DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY(-1i32);
pub const DISPLAYCONFIG_OUTPUT_TECHNOLOGY_HD15: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY = DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY(0i32);
pub const DISPLAYCONFIG_OUTPUT_TECHNOLOGY_SVIDEO: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY = DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY(1i32);
pub const DISPLAYCONFIG_OUTPUT_TECHNOLOGY_COMPOSITE_VIDEO: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY = DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY(2i32);
pub const DISPLAYCONFIG_OUTPUT_TECHNOLOGY_COMPONENT_VIDEO: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY = DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY(3i32);
pub const DISPLAYCONFIG_OUTPUT_TECHNOLOGY_DVI: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY = DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY(4i32);
pub const DISPLAYCONFIG_OUTPUT_TECHNOLOGY_HDMI: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY = DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY(5i32);
pub const DISPLAYCONFIG_OUTPUT_TECHNOLOGY_LVDS: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY = DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY(6i32);
pub const DISPLAYCONFIG_OUTPUT_TECHNOLOGY_D_JPN: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY = DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY(8i32);
pub const DISPLAYCONFIG_OUTPUT_TECHNOLOGY_SDI: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY = DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY(9i32);
pub const DISPLAYCONFIG_OUTPUT_TECHNOLOGY_DISPLAYPORT_EXTERNAL: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY = DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY(10i32);
pub const DISPLAYCONFIG_OUTPUT_TECHNOLOGY_DISPLAYPORT_EMBEDDED: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY = DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY(11i32);
pub const DISPLAYCONFIG_OUTPUT_TECHNOLOGY_UDI_EXTERNAL: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY = DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY(12i32);
pub const DISPLAYCONFIG_OUTPUT_TECHNOLOGY_UDI_EMBEDDED: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY = DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY(13i32);
pub const DISPLAYCONFIG_OUTPUT_TECHNOLOGY_SDTVDONGLE: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY = DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY(14i32);
pub const DISPLAYCONFIG_OUTPUT_TECHNOLOGY_MIRACAST: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY = DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY(15i32);
pub const DISPLAYCONFIG_OUTPUT_TECHNOLOGY_INDIRECT_WIRED: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY = DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY(16i32);
pub const DISPLAYCONFIG_OUTPUT_TECHNOLOGY_INDIRECT_VIRTUAL: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY = DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY(17i32);
pub const DISPLAYCONFIG_OUTPUT_TECHNOLOGY_DISPLAYPORT_USB_TUNNEL: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY = DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY(18i32);
pub const DISPLAYCONFIG_OUTPUT_TECHNOLOGY_INTERNAL: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY = DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY(-2147483648i32);
pub const DISPLAYCONFIG_OUTPUT_TECHNOLOGY_FORCE_UINT32: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY = DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY(-1i32);
impl ::core::convert::From<i32> for DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct DISPLAYCONFIG_VIDEO_SIGNAL_INFO {
    pub pixelRate: u64,
    pub hSyncFreq: DISPLAYCONFIG_RATIONAL,
    pub vSyncFreq: DISPLAYCONFIG_RATIONAL,
    pub activeSize: DISPLAYCONFIG_2DREGION,
    pub totalSize: DISPLAYCONFIG_2DREGION,
    pub Anonymous: DISPLAYCONFIG_VIDEO_SIGNAL_INFO_0,
    pub scanLineOrdering: DISPLAYCONFIG_SCANLINE_ORDERING,
}
impl DISPLAYCONFIG_VIDEO_SIGNAL_INFO {}
impl ::core::default::Default for DISPLAYCONFIG_VIDEO_SIGNAL_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DISPLAYCONFIG_VIDEO_SIGNAL_INFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for DISPLAYCONFIG_VIDEO_SIGNAL_INFO {}
unsafe impl ::windows::core::Abi for DISPLAYCONFIG_VIDEO_SIGNAL_INFO {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub union DISPLAYCONFIG_VIDEO_SIGNAL_INFO_0 {
    pub AdditionalSignalInfo: DISPLAYCONFIG_VIDEO_SIGNAL_INFO_0_0,
    pub videoStandard: u32,
}
impl DISPLAYCONFIG_VIDEO_SIGNAL_INFO_0 {}
impl ::core::default::Default for DISPLAYCONFIG_VIDEO_SIGNAL_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DISPLAYCONFIG_VIDEO_SIGNAL_INFO_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for DISPLAYCONFIG_VIDEO_SIGNAL_INFO_0 {}
unsafe impl ::windows::core::Abi for DISPLAYCONFIG_VIDEO_SIGNAL_INFO_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct DISPLAYCONFIG_VIDEO_SIGNAL_INFO_0_0 {
    pub _bitfield: u32,
}
impl DISPLAYCONFIG_VIDEO_SIGNAL_INFO_0_0 {}
impl ::core::default::Default for DISPLAYCONFIG_VIDEO_SIGNAL_INFO_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DISPLAYCONFIG_VIDEO_SIGNAL_INFO_0_0 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_AdditionalSignalInfo_e__Struct").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::core::cmp::PartialEq for DISPLAYCONFIG_VIDEO_SIGNAL_INFO_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for DISPLAYCONFIG_VIDEO_SIGNAL_INFO_0_0 {}
unsafe impl ::windows::core::Abi for DISPLAYCONFIG_VIDEO_SIGNAL_INFO_0_0 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const DISPLAYPOLICY_AC: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const DISPLAYPOLICY_DC: u32 = 2u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct DISPLAY_BRIGHTNESS {
    pub ucDisplayPolicy: u8,
    pub ucACBrightness: u8,
    pub ucDCBrightness: u8,
}
impl DISPLAY_BRIGHTNESS {}
impl ::core::default::Default for DISPLAY_BRIGHTNESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DISPLAY_BRIGHTNESS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DISPLAY_BRIGHTNESS").field("ucDisplayPolicy", &self.ucDisplayPolicy).field("ucACBrightness", &self.ucACBrightness).field("ucDCBrightness", &self.ucDCBrightness).finish()
    }
}
impl ::core::cmp::PartialEq for DISPLAY_BRIGHTNESS {
    fn eq(&self, other: &Self) -> bool {
        self.ucDisplayPolicy == other.ucDisplayPolicy && self.ucACBrightness == other.ucACBrightness && self.ucDCBrightness == other.ucDCBrightness
    }
}
impl ::core::cmp::Eq for DISPLAY_BRIGHTNESS {}
unsafe impl ::windows::core::Abi for DISPLAY_BRIGHTNESS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const DM_DEFAULT: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const DM_MONOCHROME: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const DN_ACCELERATION_LEVEL: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const DN_ASSOCIATE_WINDOW: u32 = 5u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const DN_COMPOSITION_CHANGED: u32 = 6u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const DN_DEVICE_ORIGIN: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const DN_DRAWING_BEGIN: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const DN_DRAWING_BEGIN_APIBITMAP: u32 = 7u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const DN_SLEEP_MODE: u32 = 3u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const DN_SURFOBJ_DESTRUCTION: u32 = 8u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const DRD_ERROR: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const DRD_SUCCESS: u32 = 0u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const DRH_APIBITMAP: u32 = 1u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
pub struct DRH_APIBITMAPDATA {
    pub pso: *mut SURFOBJ,
    pub b: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl DRH_APIBITMAPDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DRH_APIBITMAPDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DRH_APIBITMAPDATA {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DRH_APIBITMAPDATA").field("pso", &self.pso).field("b", &self.b).finish()
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
unsafe impl ::windows::core::Abi for DRH_APIBITMAPDATA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
pub struct DRIVEROBJ {
    pub pvObj: *mut ::core::ffi::c_void,
    pub pFreeProc: ::core::option::Option<FREEOBJPROC>,
    pub hdev: HDEV,
    pub dhpdev: DHPDEV,
}
#[cfg(feature = "Win32_Foundation")]
impl DRIVEROBJ {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DRIVEROBJ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DRIVEROBJ {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DRIVEROBJ").field("pvObj", &self.pvObj).field("hdev", &self.hdev).field("dhpdev", &self.dhpdev).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DRIVEROBJ {
    fn eq(&self, other: &Self) -> bool {
        self.pvObj == other.pvObj && self.pFreeProc.map(|f| f as usize) == other.pFreeProc.map(|f| f as usize) && self.hdev == other.hdev && self.dhpdev == other.dhpdev
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DRIVEROBJ {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DRIVEROBJ {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct DRVENABLEDATA {
    pub iDriverVersion: u32,
    pub c: u32,
    pub pdrvfn: *mut DRVFN,
}
impl DRVENABLEDATA {}
impl ::core::default::Default for DRVENABLEDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DRVENABLEDATA {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DRVENABLEDATA").field("iDriverVersion", &self.iDriverVersion).field("c", &self.c).field("pdrvfn", &self.pdrvfn).finish()
    }
}
impl ::core::cmp::PartialEq for DRVENABLEDATA {
    fn eq(&self, other: &Self) -> bool {
        self.iDriverVersion == other.iDriverVersion && self.c == other.c && self.pdrvfn == other.pdrvfn
    }
}
impl ::core::cmp::Eq for DRVENABLEDATA {}
unsafe impl ::windows::core::Abi for DRVENABLEDATA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct DRVFN {
    pub iFunc: u32,
    pub pfn: ::core::option::Option<PFN>,
}
impl DRVFN {}
impl ::core::default::Default for DRVFN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DRVFN {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DRVFN").field("iFunc", &self.iFunc).finish()
    }
}
impl ::core::cmp::PartialEq for DRVFN {
    fn eq(&self, other: &Self) -> bool {
        self.iFunc == other.iFunc && self.pfn.map(|f| f as usize) == other.pfn.map(|f| f as usize)
    }
}
impl ::core::cmp::Eq for DRVFN {}
unsafe impl ::windows::core::Abi for DRVFN {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const DRVQUERY_USERMODE: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const DSI_CHECKSUM_ERROR_CORRECTED: u32 = 256u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const DSI_CHECKSUM_ERROR_NOT_CORRECTED: u32 = 512u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const DSI_CONTENTION_DETECTED: u32 = 128u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DSI_CONTROL_TRANSMISSION_MODE(pub i32);
pub const DCT_DEFAULT: DSI_CONTROL_TRANSMISSION_MODE = DSI_CONTROL_TRANSMISSION_MODE(0i32);
pub const DCT_FORCE_LOW_POWER: DSI_CONTROL_TRANSMISSION_MODE = DSI_CONTROL_TRANSMISSION_MODE(1i32);
pub const DCT_FORCE_HIGH_PERFORMANCE: DSI_CONTROL_TRANSMISSION_MODE = DSI_CONTROL_TRANSMISSION_MODE(2i32);
impl ::core::convert::From<i32> for DSI_CONTROL_TRANSMISSION_MODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DSI_CONTROL_TRANSMISSION_MODE {
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
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const DSS_FLUSH_EVENT: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const DSS_RESERVED: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const DSS_RESERVED1: u32 = 8u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const DSS_RESERVED2: u32 = 16u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const DSS_TIMER_EVENT: u32 = 1u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct DXGK_WIN32K_PARAM_DATA {
    pub PathsArray: *mut ::core::ffi::c_void,
    pub ModesArray: *mut ::core::ffi::c_void,
    pub NumPathArrayElements: u32,
    pub NumModeArrayElements: u32,
    pub SDCFlags: u32,
}
impl DXGK_WIN32K_PARAM_DATA {}
impl ::core::default::Default for DXGK_WIN32K_PARAM_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DXGK_WIN32K_PARAM_DATA {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DXGK_WIN32K_PARAM_DATA").field("PathsArray", &self.PathsArray).field("ModesArray", &self.ModesArray).field("NumPathArrayElements", &self.NumPathArrayElements).field("NumModeArrayElements", &self.NumModeArrayElements).field("SDCFlags", &self.SDCFlags).finish()
    }
}
impl ::core::cmp::PartialEq for DXGK_WIN32K_PARAM_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.PathsArray == other.PathsArray && self.ModesArray == other.ModesArray && self.NumPathArrayElements == other.NumPathArrayElements && self.NumModeArrayElements == other.NumModeArrayElements && self.SDCFlags == other.SDCFlags
    }
}
impl ::core::cmp::Eq for DXGK_WIN32K_PARAM_DATA {}
unsafe impl ::windows::core::Abi for DXGK_WIN32K_PARAM_DATA {
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
pub unsafe fn DegaussMonitor<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hmonitor: Param0) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DegaussMonitor(hmonitor: super::super::Foundation::HANDLE) -> i32;
        }
        ::core::mem::transmute(DegaussMonitor(hmonitor.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DestroyPhysicalMonitor<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hmonitor: Param0) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DestroyPhysicalMonitor(hmonitor: super::super::Foundation::HANDLE) -> i32;
        }
        ::core::mem::transmute(DestroyPhysicalMonitor(hmonitor.into_param().abi()))
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
        ::core::mem::transmute(DestroyPhysicalMonitors(::core::mem::transmute(dwphysicalmonitorarraysize), ::core::mem::transmute(pphysicalmonitorarray)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DisplayConfigGetDeviceInfo(requestpacket: *mut DISPLAYCONFIG_DEVICE_INFO_HEADER) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DisplayConfigGetDeviceInfo(requestpacket: *mut DISPLAYCONFIG_DEVICE_INFO_HEADER) -> i32;
        }
        ::core::mem::transmute(DisplayConfigGetDeviceInfo(::core::mem::transmute(requestpacket)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DisplayConfigSetDeviceInfo(setpacket: *const DISPLAYCONFIG_DEVICE_INFO_HEADER) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DisplayConfigSetDeviceInfo(setpacket: *const DISPLAYCONFIG_DEVICE_INFO_HEADER) -> i32;
        }
        ::core::mem::transmute(DisplayConfigSetDeviceInfo(::core::mem::transmute(setpacket)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
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
impl ::core::default::Default for DisplayMode {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for DisplayMode {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for DisplayMode {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::core::Abi for DisplayMode {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
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
impl ::core::default::Default for DisplayModes {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for DisplayModes {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for DisplayModes {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::core::Abi for DisplayModes {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const ECS_REDRAW: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const ECS_TEARDOWN: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const ED_ABORTDOC: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const EHN_ERROR: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const EHN_RESTORED: u32 = 0u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Graphics_Gdi`*"]
pub struct EMFINFO {
    pub nSize: u32,
    pub hdc: super::super::Graphics::Gdi::HDC,
    pub pvEMF: *mut u8,
    pub pvCurrentRecord: *mut u8,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl EMFINFO {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for EMFINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::fmt::Debug for EMFINFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("EMFINFO").field("nSize", &self.nSize).field("hdc", &self.hdc).field("pvEMF", &self.pvEMF).field("pvCurrentRecord", &self.pvCurrentRecord).finish()
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
unsafe impl ::windows::core::Abi for EMFINFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const ENDCAP_BUTT: i32 = 2i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const ENDCAP_ROUND: i32 = 0i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const ENDCAP_SQUARE: i32 = 1i32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct ENGSAFESEMAPHORE {
    pub hsem: HSEMAPHORE,
    pub lCount: i32,
}
impl ENGSAFESEMAPHORE {}
impl ::core::default::Default for ENGSAFESEMAPHORE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for ENGSAFESEMAPHORE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("ENGSAFESEMAPHORE").field("hsem", &self.hsem).field("lCount", &self.lCount).finish()
    }
}
impl ::core::cmp::PartialEq for ENGSAFESEMAPHORE {
    fn eq(&self, other: &Self) -> bool {
        self.hsem == other.hsem && self.lCount == other.lCount
    }
}
impl ::core::cmp::Eq for ENGSAFESEMAPHORE {}
unsafe impl ::windows::core::Abi for ENGSAFESEMAPHORE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ENG_DEVICE_ATTRIBUTE(pub i32);
pub const QDA_RESERVED: ENG_DEVICE_ATTRIBUTE = ENG_DEVICE_ATTRIBUTE(0i32);
pub const QDA_ACCELERATION_LEVEL: ENG_DEVICE_ATTRIBUTE = ENG_DEVICE_ATTRIBUTE(1i32);
impl ::core::convert::From<i32> for ENG_DEVICE_ATTRIBUTE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ENG_DEVICE_ATTRIBUTE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct ENG_EVENT {
    pub pKEvent: *mut ::core::ffi::c_void,
    pub fFlags: u32,
}
impl ENG_EVENT {}
impl ::core::default::Default for ENG_EVENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for ENG_EVENT {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("ENG_EVENT").field("pKEvent", &self.pKEvent).field("fFlags", &self.fFlags).finish()
    }
}
impl ::core::cmp::PartialEq for ENG_EVENT {
    fn eq(&self, other: &Self) -> bool {
        self.pKEvent == other.pKEvent && self.fFlags == other.fFlags
    }
}
impl ::core::cmp::Eq for ENG_EVENT {}
unsafe impl ::windows::core::Abi for ENG_EVENT {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const ENG_FNT_CACHE_READ_FAULT: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const ENG_FNT_CACHE_WRITE_FAULT: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ENG_SYSTEM_ATTRIBUTE(pub i32);
pub const EngProcessorFeature: ENG_SYSTEM_ATTRIBUTE = ENG_SYSTEM_ATTRIBUTE(1i32);
pub const EngNumberOfProcessors: ENG_SYSTEM_ATTRIBUTE = ENG_SYSTEM_ATTRIBUTE(2i32);
pub const EngOptimumAvailableUserMemory: ENG_SYSTEM_ATTRIBUTE = ENG_SYSTEM_ATTRIBUTE(3i32);
pub const EngOptimumAvailableSystemMemory: ENG_SYSTEM_ATTRIBUTE = ENG_SYSTEM_ATTRIBUTE(4i32);
impl ::core::convert::From<i32> for ENG_SYSTEM_ATTRIBUTE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ENG_SYSTEM_ATTRIBUTE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct ENG_TIME_FIELDS {
    pub usYear: u16,
    pub usMonth: u16,
    pub usDay: u16,
    pub usHour: u16,
    pub usMinute: u16,
    pub usSecond: u16,
    pub usMilliseconds: u16,
    pub usWeekday: u16,
}
impl ENG_TIME_FIELDS {}
impl ::core::default::Default for ENG_TIME_FIELDS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for ENG_TIME_FIELDS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("ENG_TIME_FIELDS")
            .field("usYear", &self.usYear)
            .field("usMonth", &self.usMonth)
            .field("usDay", &self.usDay)
            .field("usHour", &self.usHour)
            .field("usMinute", &self.usMinute)
            .field("usSecond", &self.usSecond)
            .field("usMilliseconds", &self.usMilliseconds)
            .field("usWeekday", &self.usWeekday)
            .finish()
    }
}
impl ::core::cmp::PartialEq for ENG_TIME_FIELDS {
    fn eq(&self, other: &Self) -> bool {
        self.usYear == other.usYear && self.usMonth == other.usMonth && self.usDay == other.usDay && self.usHour == other.usHour && self.usMinute == other.usMinute && self.usSecond == other.usSecond && self.usMilliseconds == other.usMilliseconds && self.usWeekday == other.usWeekday
    }
}
impl ::core::cmp::Eq for ENG_TIME_FIELDS {}
unsafe impl ::windows::core::Abi for ENG_TIME_FIELDS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
pub struct ENUMRECTS {
    pub c: u32,
    pub arcl: [super::super::Foundation::RECTL; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ENUMRECTS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ENUMRECTS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ENUMRECTS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("ENUMRECTS").field("c", &self.c).field("arcl", &self.arcl).finish()
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
unsafe impl ::windows::core::Abi for ENUMRECTS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
#[inline]
pub unsafe fn EngAcquireSemaphore<'a, Param0: ::windows::core::IntoParam<'a, HSEMAPHORE>>(hsem: Param0) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EngAcquireSemaphore(hsem: HSEMAPHORE);
        }
        ::core::mem::transmute(EngAcquireSemaphore(hsem.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn EngAlphaBlend(psodest: *mut SURFOBJ, psosrc: *mut SURFOBJ, pco: *mut CLIPOBJ, pxlo: *mut XLATEOBJ, prcldest: *mut super::super::Foundation::RECTL, prclsrc: *mut super::super::Foundation::RECTL, pblendobj: *mut BLENDOBJ) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EngAlphaBlend(psodest: *mut SURFOBJ, psosrc: *mut SURFOBJ, pco: *mut CLIPOBJ, pxlo: *mut XLATEOBJ, prcldest: *mut super::super::Foundation::RECTL, prclsrc: *mut super::super::Foundation::RECTL, pblendobj: *mut BLENDOBJ) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(EngAlphaBlend(::core::mem::transmute(psodest), ::core::mem::transmute(psosrc), ::core::mem::transmute(pco), ::core::mem::transmute(pxlo), ::core::mem::transmute(prcldest), ::core::mem::transmute(prclsrc), ::core::mem::transmute(pblendobj)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EngAssociateSurface<'a, Param0: ::windows::core::IntoParam<'a, HSURF>, Param1: ::windows::core::IntoParam<'a, HDEV>>(hsurf: Param0, hdev: Param1, flhooks: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EngAssociateSurface(hsurf: HSURF, hdev: HDEV, flhooks: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(EngAssociateSurface(hsurf.into_param().abi(), hdev.into_param().abi(), ::core::mem::transmute(flhooks)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EngBitBlt(psotrg: *const SURFOBJ, psosrc: *const SURFOBJ, psomask: *const SURFOBJ, pco: *const CLIPOBJ, pxlo: *const XLATEOBJ, prcltrg: *const super::super::Foundation::RECTL, pptlsrc: *const super::super::Foundation::POINTL, pptlmask: *const super::super::Foundation::POINTL, pbo: *const BRUSHOBJ, pptlbrush: *const super::super::Foundation::POINTL, rop4: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EngBitBlt(psotrg: *const SURFOBJ, psosrc: *const SURFOBJ, psomask: *const SURFOBJ, pco: *const CLIPOBJ, pxlo: *const XLATEOBJ, prcltrg: *const super::super::Foundation::RECTL, pptlsrc: *const super::super::Foundation::POINTL, pptlmask: *const super::super::Foundation::POINTL, pbo: *const BRUSHOBJ, pptlbrush: *const super::super::Foundation::POINTL, rop4: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(EngBitBlt(
            ::core::mem::transmute(psotrg),
            ::core::mem::transmute(psosrc),
            ::core::mem::transmute(psomask),
            ::core::mem::transmute(pco),
            ::core::mem::transmute(pxlo),
            ::core::mem::transmute(prcltrg),
            ::core::mem::transmute(pptlsrc),
            ::core::mem::transmute(pptlmask),
            ::core::mem::transmute(pbo),
            ::core::mem::transmute(pptlbrush),
            ::core::mem::transmute(rop4),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EngCheckAbort(pso: *mut SURFOBJ) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EngCheckAbort(pso: *mut SURFOBJ) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(EngCheckAbort(::core::mem::transmute(pso)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
#[inline]
pub unsafe fn EngComputeGlyphSet(ncodepage: i32, nfirstchar: i32, cchars: i32) -> *mut FD_GLYPHSET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EngComputeGlyphSet(ncodepage: i32, nfirstchar: i32, cchars: i32) -> *mut FD_GLYPHSET;
        }
        ::core::mem::transmute(EngComputeGlyphSet(::core::mem::transmute(ncodepage), ::core::mem::transmute(nfirstchar), ::core::mem::transmute(cchars)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EngCopyBits(psodest: *mut SURFOBJ, psosrc: *mut SURFOBJ, pco: *mut CLIPOBJ, pxlo: *mut XLATEOBJ, prcldest: *mut super::super::Foundation::RECTL, pptlsrc: *mut super::super::Foundation::POINTL) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EngCopyBits(psodest: *mut SURFOBJ, psosrc: *mut SURFOBJ, pco: *mut CLIPOBJ, pxlo: *mut XLATEOBJ, prcldest: *mut super::super::Foundation::RECTL, pptlsrc: *mut super::super::Foundation::POINTL) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(EngCopyBits(::core::mem::transmute(psodest), ::core::mem::transmute(psosrc), ::core::mem::transmute(pco), ::core::mem::transmute(pxlo), ::core::mem::transmute(prcldest), ::core::mem::transmute(pptlsrc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn EngCreateBitmap<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::SIZE>>(sizl: Param0, lwidth: i32, iformat: u32, fl: u32, pvbits: *mut ::core::ffi::c_void) -> super::super::Graphics::Gdi::HBITMAP {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EngCreateBitmap(sizl: super::super::Foundation::SIZE, lwidth: i32, iformat: u32, fl: u32, pvbits: *mut ::core::ffi::c_void) -> super::super::Graphics::Gdi::HBITMAP;
        }
        ::core::mem::transmute(EngCreateBitmap(sizl.into_param().abi(), ::core::mem::transmute(lwidth), ::core::mem::transmute(iformat), ::core::mem::transmute(fl), ::core::mem::transmute(pvbits)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EngCreateClip() -> *mut CLIPOBJ {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EngCreateClip() -> *mut CLIPOBJ;
        }
        ::core::mem::transmute(EngCreateClip())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn EngCreateDeviceBitmap<'a, Param0: ::windows::core::IntoParam<'a, DHSURF>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::SIZE>>(dhsurf: Param0, sizl: Param1, iformatcompat: u32) -> super::super::Graphics::Gdi::HBITMAP {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EngCreateDeviceBitmap(dhsurf: DHSURF, sizl: super::super::Foundation::SIZE, iformatcompat: u32) -> super::super::Graphics::Gdi::HBITMAP;
        }
        ::core::mem::transmute(EngCreateDeviceBitmap(dhsurf.into_param().abi(), sizl.into_param().abi(), ::core::mem::transmute(iformatcompat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EngCreateDeviceSurface<'a, Param0: ::windows::core::IntoParam<'a, DHSURF>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::SIZE>>(dhsurf: Param0, sizl: Param1, iformatcompat: u32) -> HSURF {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EngCreateDeviceSurface(dhsurf: DHSURF, sizl: super::super::Foundation::SIZE, iformatcompat: u32) -> HSURF;
        }
        ::core::mem::transmute(EngCreateDeviceSurface(dhsurf.into_param().abi(), sizl.into_param().abi(), ::core::mem::transmute(iformatcompat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Graphics_Gdi`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn EngCreatePalette(imode: u32, ccolors: u32, pulcolors: *mut u32, flred: u32, flgreen: u32, flblue: u32) -> super::super::Graphics::Gdi::HPALETTE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EngCreatePalette(imode: u32, ccolors: u32, pulcolors: *mut u32, flred: u32, flgreen: u32, flblue: u32) -> super::super::Graphics::Gdi::HPALETTE;
        }
        ::core::mem::transmute(EngCreatePalette(::core::mem::transmute(imode), ::core::mem::transmute(ccolors), ::core::mem::transmute(pulcolors), ::core::mem::transmute(flred), ::core::mem::transmute(flgreen), ::core::mem::transmute(flblue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
#[inline]
pub unsafe fn EngCreateSemaphore() -> HSEMAPHORE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EngCreateSemaphore() -> HSEMAPHORE;
        }
        ::core::mem::transmute(EngCreateSemaphore())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EngDeleteClip(pco: *const CLIPOBJ) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EngDeleteClip(pco: *const CLIPOBJ);
        }
        ::core::mem::transmute(EngDeleteClip(::core::mem::transmute(pco)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn EngDeletePalette<'a, Param0: ::windows::core::IntoParam<'a, super::super::Graphics::Gdi::HPALETTE>>(hpal: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EngDeletePalette(hpal: super::super::Graphics::Gdi::HPALETTE) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(EngDeletePalette(hpal.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
#[inline]
pub unsafe fn EngDeletePath(ppo: *mut PATHOBJ) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EngDeletePath(ppo: *mut PATHOBJ);
        }
        ::core::mem::transmute(EngDeletePath(::core::mem::transmute(ppo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
#[inline]
pub unsafe fn EngDeleteSemaphore<'a, Param0: ::windows::core::IntoParam<'a, HSEMAPHORE>>(hsem: Param0) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EngDeleteSemaphore(hsem: HSEMAPHORE);
        }
        ::core::mem::transmute(EngDeleteSemaphore(hsem.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EngDeleteSurface<'a, Param0: ::windows::core::IntoParam<'a, HSURF>>(hsurf: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EngDeleteSurface(hsurf: HSURF) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(EngDeleteSurface(hsurf.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EngEraseSurface(pso: *mut SURFOBJ, prcl: *mut super::super::Foundation::RECTL, icolor: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EngEraseSurface(pso: *mut SURFOBJ, prcl: *mut super::super::Foundation::RECTL, icolor: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(EngEraseSurface(::core::mem::transmute(pso), ::core::mem::transmute(prcl), ::core::mem::transmute(icolor)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EngFillPath(pso: *mut SURFOBJ, ppo: *mut PATHOBJ, pco: *mut CLIPOBJ, pbo: *mut BRUSHOBJ, pptlbrushorg: *mut super::super::Foundation::POINTL, mix: u32, floptions: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EngFillPath(pso: *mut SURFOBJ, ppo: *mut PATHOBJ, pco: *mut CLIPOBJ, pbo: *mut BRUSHOBJ, pptlbrushorg: *mut super::super::Foundation::POINTL, mix: u32, floptions: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(EngFillPath(::core::mem::transmute(pso), ::core::mem::transmute(ppo), ::core::mem::transmute(pco), ::core::mem::transmute(pbo), ::core::mem::transmute(pptlbrushorg), ::core::mem::transmute(mix), ::core::mem::transmute(floptions)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EngFindResource<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(h: Param0, iname: i32, itype: i32, pulsize: *mut u32) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EngFindResource(h: super::super::Foundation::HANDLE, iname: i32, itype: i32, pulsize: *mut u32) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(EngFindResource(h.into_param().abi(), ::core::mem::transmute(iname), ::core::mem::transmute(itype), ::core::mem::transmute(pulsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EngFreeModule<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(h: Param0) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EngFreeModule(h: super::super::Foundation::HANDLE);
        }
        ::core::mem::transmute(EngFreeModule(h.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
#[inline]
pub unsafe fn EngGetCurrentCodePage(oemcodepage: *mut u16, ansicodepage: *mut u16) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EngGetCurrentCodePage(oemcodepage: *mut u16, ansicodepage: *mut u16);
        }
        ::core::mem::transmute(EngGetCurrentCodePage(::core::mem::transmute(oemcodepage), ::core::mem::transmute(ansicodepage)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EngGetDriverName<'a, Param0: ::windows::core::IntoParam<'a, HDEV>>(hdev: Param0) -> super::super::Foundation::PWSTR {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EngGetDriverName(hdev: HDEV) -> super::super::Foundation::PWSTR;
        }
        ::core::mem::transmute(EngGetDriverName(hdev.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EngGetPrinterDataFileName<'a, Param0: ::windows::core::IntoParam<'a, HDEV>>(hdev: Param0) -> super::super::Foundation::PWSTR {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EngGetPrinterDataFileName(hdev: HDEV) -> super::super::Foundation::PWSTR;
        }
        ::core::mem::transmute(EngGetPrinterDataFileName(hdev.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn EngGradientFill(psodest: *mut SURFOBJ, pco: *mut CLIPOBJ, pxlo: *mut XLATEOBJ, pvertex: *mut super::super::Graphics::Gdi::TRIVERTEX, nvertex: u32, pmesh: *mut ::core::ffi::c_void, nmesh: u32, prclextents: *mut super::super::Foundation::RECTL, pptlditherorg: *mut super::super::Foundation::POINTL, ulmode: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EngGradientFill(psodest: *mut SURFOBJ, pco: *mut CLIPOBJ, pxlo: *mut XLATEOBJ, pvertex: *mut super::super::Graphics::Gdi::TRIVERTEX, nvertex: u32, pmesh: *mut ::core::ffi::c_void, nmesh: u32, prclextents: *mut super::super::Foundation::RECTL, pptlditherorg: *mut super::super::Foundation::POINTL, ulmode: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(EngGradientFill(
            ::core::mem::transmute(psodest),
            ::core::mem::transmute(pco),
            ::core::mem::transmute(pxlo),
            ::core::mem::transmute(pvertex),
            ::core::mem::transmute(nvertex),
            ::core::mem::transmute(pmesh),
            ::core::mem::transmute(nmesh),
            ::core::mem::transmute(prclextents),
            ::core::mem::transmute(pptlditherorg),
            ::core::mem::transmute(ulmode),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EngLineTo(pso: *mut SURFOBJ, pco: *mut CLIPOBJ, pbo: *mut BRUSHOBJ, x1: i32, y1: i32, x2: i32, y2: i32, prclbounds: *mut super::super::Foundation::RECTL, mix: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EngLineTo(pso: *mut SURFOBJ, pco: *mut CLIPOBJ, pbo: *mut BRUSHOBJ, x1: i32, y1: i32, x2: i32, y2: i32, prclbounds: *mut super::super::Foundation::RECTL, mix: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(EngLineTo(::core::mem::transmute(pso), ::core::mem::transmute(pco), ::core::mem::transmute(pbo), ::core::mem::transmute(x1), ::core::mem::transmute(y1), ::core::mem::transmute(x2), ::core::mem::transmute(y2), ::core::mem::transmute(prclbounds), ::core::mem::transmute(mix)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EngLoadModule<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(pwsz: Param0) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EngLoadModule(pwsz: super::super::Foundation::PWSTR) -> super::super::Foundation::HANDLE;
        }
        ::core::mem::transmute(EngLoadModule(pwsz.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EngLockSurface<'a, Param0: ::windows::core::IntoParam<'a, HSURF>>(hsurf: Param0) -> *mut SURFOBJ {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EngLockSurface(hsurf: HSURF) -> *mut SURFOBJ;
        }
        ::core::mem::transmute(EngLockSurface(hsurf.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EngMarkBandingSurface<'a, Param0: ::windows::core::IntoParam<'a, HSURF>>(hsurf: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EngMarkBandingSurface(hsurf: HSURF) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(EngMarkBandingSurface(hsurf.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EngMultiByteToUnicodeN<'a, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(unicodestring: super::super::Foundation::PWSTR, maxbytesinunicodestring: u32, bytesinunicodestring: *mut u32, multibytestring: Param3, bytesinmultibytestring: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EngMultiByteToUnicodeN(unicodestring: super::super::Foundation::PWSTR, maxbytesinunicodestring: u32, bytesinunicodestring: *mut u32, multibytestring: super::super::Foundation::PSTR, bytesinmultibytestring: u32);
        }
        ::core::mem::transmute(EngMultiByteToUnicodeN(::core::mem::transmute(unicodestring), ::core::mem::transmute(maxbytesinunicodestring), ::core::mem::transmute(bytesinunicodestring), multibytestring.into_param().abi(), ::core::mem::transmute(bytesinmultibytestring)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EngMultiByteToWideChar<'a, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(codepage: u32, widecharstring: super::super::Foundation::PWSTR, bytesinwidecharstring: i32, multibytestring: Param3, bytesinmultibytestring: i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EngMultiByteToWideChar(codepage: u32, widecharstring: super::super::Foundation::PWSTR, bytesinwidecharstring: i32, multibytestring: super::super::Foundation::PSTR, bytesinmultibytestring: i32) -> i32;
        }
        ::core::mem::transmute(EngMultiByteToWideChar(::core::mem::transmute(codepage), ::core::mem::transmute(widecharstring), ::core::mem::transmute(bytesinwidecharstring), multibytestring.into_param().abi(), ::core::mem::transmute(bytesinmultibytestring)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EngPaint(pso: *mut SURFOBJ, pco: *mut CLIPOBJ, pbo: *mut BRUSHOBJ, pptlbrushorg: *mut super::super::Foundation::POINTL, mix: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EngPaint(pso: *mut SURFOBJ, pco: *mut CLIPOBJ, pbo: *mut BRUSHOBJ, pptlbrushorg: *mut super::super::Foundation::POINTL, mix: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(EngPaint(::core::mem::transmute(pso), ::core::mem::transmute(pco), ::core::mem::transmute(pbo), ::core::mem::transmute(pptlbrushorg), ::core::mem::transmute(mix)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn EngPlgBlt(psotrg: *mut SURFOBJ, psosrc: *mut SURFOBJ, psomsk: *mut SURFOBJ, pco: *mut CLIPOBJ, pxlo: *mut XLATEOBJ, pca: *mut super::super::Graphics::Gdi::COLORADJUSTMENT, pptlbrushorg: *mut super::super::Foundation::POINTL, pptfx: *mut POINTFIX, prcl: *mut super::super::Foundation::RECTL, pptl: *mut super::super::Foundation::POINTL, imode: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EngPlgBlt(psotrg: *mut SURFOBJ, psosrc: *mut SURFOBJ, psomsk: *mut SURFOBJ, pco: *mut CLIPOBJ, pxlo: *mut XLATEOBJ, pca: *mut super::super::Graphics::Gdi::COLORADJUSTMENT, pptlbrushorg: *mut super::super::Foundation::POINTL, pptfx: *mut POINTFIX, prcl: *mut super::super::Foundation::RECTL, pptl: *mut super::super::Foundation::POINTL, imode: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(EngPlgBlt(
            ::core::mem::transmute(psotrg),
            ::core::mem::transmute(psosrc),
            ::core::mem::transmute(psomsk),
            ::core::mem::transmute(pco),
            ::core::mem::transmute(pxlo),
            ::core::mem::transmute(pca),
            ::core::mem::transmute(pptlbrushorg),
            ::core::mem::transmute(pptfx),
            ::core::mem::transmute(prcl),
            ::core::mem::transmute(pptl),
            ::core::mem::transmute(imode),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn EngQueryEMFInfo<'a, Param0: ::windows::core::IntoParam<'a, HDEV>>(hdev: Param0, pemfinfo: *mut EMFINFO) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EngQueryEMFInfo(hdev: HDEV, pemfinfo: *mut EMFINFO) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(EngQueryEMFInfo(hdev.into_param().abi(), ::core::mem::transmute(pemfinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
#[inline]
pub unsafe fn EngQueryLocalTime(param0: *mut ENG_TIME_FIELDS) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EngQueryLocalTime(param0: *mut ENG_TIME_FIELDS);
        }
        ::core::mem::transmute(EngQueryLocalTime(::core::mem::transmute(param0)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
#[inline]
pub unsafe fn EngReleaseSemaphore<'a, Param0: ::windows::core::IntoParam<'a, HSEMAPHORE>>(hsem: Param0) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EngReleaseSemaphore(hsem: HSEMAPHORE);
        }
        ::core::mem::transmute(EngReleaseSemaphore(hsem.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn EngStretchBlt(psodest: *mut SURFOBJ, psosrc: *mut SURFOBJ, psomask: *mut SURFOBJ, pco: *mut CLIPOBJ, pxlo: *mut XLATEOBJ, pca: *mut super::super::Graphics::Gdi::COLORADJUSTMENT, pptlhtorg: *mut super::super::Foundation::POINTL, prcldest: *mut super::super::Foundation::RECTL, prclsrc: *mut super::super::Foundation::RECTL, pptlmask: *mut super::super::Foundation::POINTL, imode: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EngStretchBlt(psodest: *mut SURFOBJ, psosrc: *mut SURFOBJ, psomask: *mut SURFOBJ, pco: *mut CLIPOBJ, pxlo: *mut XLATEOBJ, pca: *mut super::super::Graphics::Gdi::COLORADJUSTMENT, pptlhtorg: *mut super::super::Foundation::POINTL, prcldest: *mut super::super::Foundation::RECTL, prclsrc: *mut super::super::Foundation::RECTL, pptlmask: *mut super::super::Foundation::POINTL, imode: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(EngStretchBlt(
            ::core::mem::transmute(psodest),
            ::core::mem::transmute(psosrc),
            ::core::mem::transmute(psomask),
            ::core::mem::transmute(pco),
            ::core::mem::transmute(pxlo),
            ::core::mem::transmute(pca),
            ::core::mem::transmute(pptlhtorg),
            ::core::mem::transmute(prcldest),
            ::core::mem::transmute(prclsrc),
            ::core::mem::transmute(pptlmask),
            ::core::mem::transmute(imode),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn EngStretchBltROP(psodest: *mut SURFOBJ, psosrc: *mut SURFOBJ, psomask: *mut SURFOBJ, pco: *mut CLIPOBJ, pxlo: *mut XLATEOBJ, pca: *mut super::super::Graphics::Gdi::COLORADJUSTMENT, pptlhtorg: *mut super::super::Foundation::POINTL, prcldest: *mut super::super::Foundation::RECTL, prclsrc: *mut super::super::Foundation::RECTL, pptlmask: *mut super::super::Foundation::POINTL, imode: u32, pbo: *mut BRUSHOBJ, rop4: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EngStretchBltROP(psodest: *mut SURFOBJ, psosrc: *mut SURFOBJ, psomask: *mut SURFOBJ, pco: *mut CLIPOBJ, pxlo: *mut XLATEOBJ, pca: *mut super::super::Graphics::Gdi::COLORADJUSTMENT, pptlhtorg: *mut super::super::Foundation::POINTL, prcldest: *mut super::super::Foundation::RECTL, prclsrc: *mut super::super::Foundation::RECTL, pptlmask: *mut super::super::Foundation::POINTL, imode: u32, pbo: *mut BRUSHOBJ, rop4: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(EngStretchBltROP(
            ::core::mem::transmute(psodest),
            ::core::mem::transmute(psosrc),
            ::core::mem::transmute(psomask),
            ::core::mem::transmute(pco),
            ::core::mem::transmute(pxlo),
            ::core::mem::transmute(pca),
            ::core::mem::transmute(pptlhtorg),
            ::core::mem::transmute(prcldest),
            ::core::mem::transmute(prclsrc),
            ::core::mem::transmute(pptlmask),
            ::core::mem::transmute(imode),
            ::core::mem::transmute(pbo),
            ::core::mem::transmute(rop4),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EngStrokeAndFillPath(pso: *mut SURFOBJ, ppo: *mut PATHOBJ, pco: *mut CLIPOBJ, pxo: *mut XFORMOBJ, pbostroke: *mut BRUSHOBJ, plineattrs: *mut LINEATTRS, pbofill: *mut BRUSHOBJ, pptlbrushorg: *mut super::super::Foundation::POINTL, mixfill: u32, floptions: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EngStrokeAndFillPath(pso: *mut SURFOBJ, ppo: *mut PATHOBJ, pco: *mut CLIPOBJ, pxo: *mut XFORMOBJ, pbostroke: *mut BRUSHOBJ, plineattrs: *mut LINEATTRS, pbofill: *mut BRUSHOBJ, pptlbrushorg: *mut super::super::Foundation::POINTL, mixfill: u32, floptions: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(EngStrokeAndFillPath(
            ::core::mem::transmute(pso),
            ::core::mem::transmute(ppo),
            ::core::mem::transmute(pco),
            ::core::mem::transmute(pxo),
            ::core::mem::transmute(pbostroke),
            ::core::mem::transmute(plineattrs),
            ::core::mem::transmute(pbofill),
            ::core::mem::transmute(pptlbrushorg),
            ::core::mem::transmute(mixfill),
            ::core::mem::transmute(floptions),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EngStrokePath(pso: *mut SURFOBJ, ppo: *mut PATHOBJ, pco: *mut CLIPOBJ, pxo: *mut XFORMOBJ, pbo: *mut BRUSHOBJ, pptlbrushorg: *mut super::super::Foundation::POINTL, plineattrs: *mut LINEATTRS, mix: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EngStrokePath(pso: *mut SURFOBJ, ppo: *mut PATHOBJ, pco: *mut CLIPOBJ, pxo: *mut XFORMOBJ, pbo: *mut BRUSHOBJ, pptlbrushorg: *mut super::super::Foundation::POINTL, plineattrs: *mut LINEATTRS, mix: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(EngStrokePath(::core::mem::transmute(pso), ::core::mem::transmute(ppo), ::core::mem::transmute(pco), ::core::mem::transmute(pxo), ::core::mem::transmute(pbo), ::core::mem::transmute(pptlbrushorg), ::core::mem::transmute(plineattrs), ::core::mem::transmute(mix)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EngTextOut(pso: *mut SURFOBJ, pstro: *mut STROBJ, pfo: *mut FONTOBJ, pco: *mut CLIPOBJ, prclextra: *mut super::super::Foundation::RECTL, prclopaque: *mut super::super::Foundation::RECTL, pbofore: *mut BRUSHOBJ, pboopaque: *mut BRUSHOBJ, pptlorg: *mut super::super::Foundation::POINTL, mix: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EngTextOut(pso: *mut SURFOBJ, pstro: *mut STROBJ, pfo: *mut FONTOBJ, pco: *mut CLIPOBJ, prclextra: *mut super::super::Foundation::RECTL, prclopaque: *mut super::super::Foundation::RECTL, pbofore: *mut BRUSHOBJ, pboopaque: *mut BRUSHOBJ, pptlorg: *mut super::super::Foundation::POINTL, mix: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(EngTextOut(
            ::core::mem::transmute(pso),
            ::core::mem::transmute(pstro),
            ::core::mem::transmute(pfo),
            ::core::mem::transmute(pco),
            ::core::mem::transmute(prclextra),
            ::core::mem::transmute(prclopaque),
            ::core::mem::transmute(pbofore),
            ::core::mem::transmute(pboopaque),
            ::core::mem::transmute(pptlorg),
            ::core::mem::transmute(mix),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EngTransparentBlt(psodst: *const SURFOBJ, psosrc: *const SURFOBJ, pco: *const CLIPOBJ, pxlo: *const XLATEOBJ, prcldst: *const super::super::Foundation::RECTL, prclsrc: *const super::super::Foundation::RECTL, transcolor: u32, bcalledfrombitblt: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EngTransparentBlt(psodst: *const SURFOBJ, psosrc: *const SURFOBJ, pco: *const CLIPOBJ, pxlo: *const XLATEOBJ, prcldst: *const super::super::Foundation::RECTL, prclsrc: *const super::super::Foundation::RECTL, transcolor: u32, bcalledfrombitblt: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(EngTransparentBlt(::core::mem::transmute(psodst), ::core::mem::transmute(psosrc), ::core::mem::transmute(pco), ::core::mem::transmute(pxlo), ::core::mem::transmute(prcldst), ::core::mem::transmute(prclsrc), ::core::mem::transmute(transcolor), ::core::mem::transmute(bcalledfrombitblt)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EngUnicodeToMultiByteN<'a, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(multibytestring: super::super::Foundation::PSTR, maxbytesinmultibytestring: u32, bytesinmultibytestring: *mut u32, unicodestring: Param3, bytesinunicodestring: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EngUnicodeToMultiByteN(multibytestring: super::super::Foundation::PSTR, maxbytesinmultibytestring: u32, bytesinmultibytestring: *mut u32, unicodestring: super::super::Foundation::PWSTR, bytesinunicodestring: u32);
        }
        ::core::mem::transmute(EngUnicodeToMultiByteN(::core::mem::transmute(multibytestring), ::core::mem::transmute(maxbytesinmultibytestring), ::core::mem::transmute(bytesinmultibytestring), unicodestring.into_param().abi(), ::core::mem::transmute(bytesinunicodestring)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EngUnlockSurface(pso: *mut SURFOBJ) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EngUnlockSurface(pso: *mut SURFOBJ);
        }
        ::core::mem::transmute(EngUnlockSurface(::core::mem::transmute(pso)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EngWideCharToMultiByte<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(codepage: u32, widecharstring: Param1, bytesinwidecharstring: i32, multibytestring: super::super::Foundation::PSTR, bytesinmultibytestring: i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EngWideCharToMultiByte(codepage: u32, widecharstring: super::super::Foundation::PWSTR, bytesinwidecharstring: i32, multibytestring: super::super::Foundation::PSTR, bytesinmultibytestring: i32) -> i32;
        }
        ::core::mem::transmute(EngWideCharToMultiByte(::core::mem::transmute(codepage), widecharstring.into_param().abi(), ::core::mem::transmute(bytesinwidecharstring), ::core::mem::transmute(multibytestring), ::core::mem::transmute(bytesinmultibytestring)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const FC_COMPLEX: u32 = 3u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const FC_RECT: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const FC_RECT4: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const FDM_TYPE_BM_SIDE_CONST: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const FDM_TYPE_CHAR_INC_EQUAL_BM_BASE: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const FDM_TYPE_CONST_BEARINGS: u32 = 16u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const FDM_TYPE_MAXEXT_EQUAL_BM_SIDE: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const FDM_TYPE_ZERO_BEARINGS: u32 = 8u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
pub struct FD_DEVICEMETRICS {
    pub flRealizedType: u32,
    pub pteBase: POINTE,
    pub pteSide: POINTE,
    pub lD: i32,
    pub fxMaxAscender: i32,
    pub fxMaxDescender: i32,
    pub ptlUnderline1: super::super::Foundation::POINTL,
    pub ptlStrikeOut: super::super::Foundation::POINTL,
    pub ptlULThickness: super::super::Foundation::POINTL,
    pub ptlSOThickness: super::super::Foundation::POINTL,
    pub cxMax: u32,
    pub cyMax: u32,
    pub cjGlyphMax: u32,
    pub fdxQuantized: FD_XFORM,
    pub lNonLinearExtLeading: i32,
    pub lNonLinearIntLeading: i32,
    pub lNonLinearMaxCharWidth: i32,
    pub lNonLinearAvgCharWidth: i32,
    pub lMinA: i32,
    pub lMinC: i32,
    pub lMinD: i32,
    pub alReserved: [i32; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl FD_DEVICEMETRICS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FD_DEVICEMETRICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FD_DEVICEMETRICS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("FD_DEVICEMETRICS")
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
unsafe impl ::windows::core::Abi for FD_DEVICEMETRICS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const FD_ERROR: u32 = 4294967295u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct FD_GLYPHATTR {
    pub cjThis: u32,
    pub cGlyphs: u32,
    pub iMode: u32,
    pub aGlyphAttr: [u8; 1],
}
impl FD_GLYPHATTR {}
impl ::core::default::Default for FD_GLYPHATTR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for FD_GLYPHATTR {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("FD_GLYPHATTR").field("cjThis", &self.cjThis).field("cGlyphs", &self.cGlyphs).field("iMode", &self.iMode).field("aGlyphAttr", &self.aGlyphAttr).finish()
    }
}
impl ::core::cmp::PartialEq for FD_GLYPHATTR {
    fn eq(&self, other: &Self) -> bool {
        self.cjThis == other.cjThis && self.cGlyphs == other.cGlyphs && self.iMode == other.iMode && self.aGlyphAttr == other.aGlyphAttr
    }
}
impl ::core::cmp::Eq for FD_GLYPHATTR {}
unsafe impl ::windows::core::Abi for FD_GLYPHATTR {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct FD_GLYPHSET {
    pub cjThis: u32,
    pub flAccel: u32,
    pub cGlyphsSupported: u32,
    pub cRuns: u32,
    pub awcrun: [WCRUN; 1],
}
impl FD_GLYPHSET {}
impl ::core::default::Default for FD_GLYPHSET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for FD_GLYPHSET {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("FD_GLYPHSET").field("cjThis", &self.cjThis).field("flAccel", &self.flAccel).field("cGlyphsSupported", &self.cGlyphsSupported).field("cRuns", &self.cRuns).field("awcrun", &self.awcrun).finish()
    }
}
impl ::core::cmp::PartialEq for FD_GLYPHSET {
    fn eq(&self, other: &Self) -> bool {
        self.cjThis == other.cjThis && self.flAccel == other.flAccel && self.cGlyphsSupported == other.cGlyphsSupported && self.cRuns == other.cRuns && self.awcrun == other.awcrun
    }
}
impl ::core::cmp::Eq for FD_GLYPHSET {}
unsafe impl ::windows::core::Abi for FD_GLYPHSET {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct FD_KERNINGPAIR {
    pub wcFirst: u16,
    pub wcSecond: u16,
    pub fwdKern: i16,
}
impl FD_KERNINGPAIR {}
impl ::core::default::Default for FD_KERNINGPAIR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for FD_KERNINGPAIR {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("FD_KERNINGPAIR").field("wcFirst", &self.wcFirst).field("wcSecond", &self.wcSecond).field("fwdKern", &self.fwdKern).finish()
    }
}
impl ::core::cmp::PartialEq for FD_KERNINGPAIR {
    fn eq(&self, other: &Self) -> bool {
        self.wcFirst == other.wcFirst && self.wcSecond == other.wcSecond && self.fwdKern == other.fwdKern
    }
}
impl ::core::cmp::Eq for FD_KERNINGPAIR {}
unsafe impl ::windows::core::Abi for FD_KERNINGPAIR {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
pub struct FD_LIGATURE {
    pub culThis: u32,
    pub ulType: u32,
    pub cLigatures: u32,
    pub alig: [LIGATURE; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl FD_LIGATURE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FD_LIGATURE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FD_LIGATURE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("FD_LIGATURE").field("culThis", &self.culThis).field("ulType", &self.ulType).field("cLigatures", &self.cLigatures).field("alig", &self.alig).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FD_LIGATURE {
    fn eq(&self, other: &Self) -> bool {
        self.culThis == other.culThis && self.ulType == other.ulType && self.cLigatures == other.cLigatures && self.alig == other.alig
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FD_LIGATURE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for FD_LIGATURE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const FD_NEGATIVE_FONT: i32 = 1i32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct FD_XFORM {
    pub eXX: f32,
    pub eXY: f32,
    pub eYX: f32,
    pub eYY: f32,
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl FD_XFORM {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::default::Default for FD_XFORM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::fmt::Debug for FD_XFORM {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("FD_XFORM").field("eXX", &self.eXX).field("eXY", &self.eXY).field("eYX", &self.eYX).field("eYY", &self.eYY).finish()
    }
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::cmp::PartialEq for FD_XFORM {
    fn eq(&self, other: &Self) -> bool {
        self.eXX == other.eXX && self.eXY == other.eXY && self.eYX == other.eYX && self.eYY == other.eYY
    }
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::cmp::Eq for FD_XFORM {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
unsafe impl ::windows::core::Abi for FD_XFORM {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(any(target_arch = "x86",))]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct FD_XFORM {
    pub eXX: u32,
    pub eXY: u32,
    pub eYX: u32,
    pub eYY: u32,
}
#[cfg(any(target_arch = "x86",))]
impl FD_XFORM {}
#[cfg(any(target_arch = "x86",))]
impl ::core::default::Default for FD_XFORM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "x86",))]
impl ::core::fmt::Debug for FD_XFORM {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("FD_XFORM").field("eXX", &self.eXX).field("eXY", &self.eXY).field("eYX", &self.eYX).field("eYY", &self.eYY).finish()
    }
}
#[cfg(any(target_arch = "x86",))]
impl ::core::cmp::PartialEq for FD_XFORM {
    fn eq(&self, other: &Self) -> bool {
        self.eXX == other.eXX && self.eXY == other.eXY && self.eYX == other.eYX && self.eYY == other.eYY
    }
}
#[cfg(any(target_arch = "x86",))]
impl ::core::cmp::Eq for FD_XFORM {}
#[cfg(any(target_arch = "x86",))]
unsafe impl ::windows::core::Abi for FD_XFORM {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const FF_IGNORED_SIGNATURE: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const FF_SIGNATURE_VERIFIED: u32 = 1u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(any(target_arch = "x86",))]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct FLOATOBJ {
    pub ul1: u32,
    pub ul2: u32,
}
#[cfg(any(target_arch = "x86",))]
impl FLOATOBJ {}
#[cfg(any(target_arch = "x86",))]
impl ::core::default::Default for FLOATOBJ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "x86",))]
impl ::core::fmt::Debug for FLOATOBJ {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("FLOATOBJ").field("ul1", &self.ul1).field("ul2", &self.ul2).finish()
    }
}
#[cfg(any(target_arch = "x86",))]
impl ::core::cmp::PartialEq for FLOATOBJ {
    fn eq(&self, other: &Self) -> bool {
        self.ul1 == other.ul1 && self.ul2 == other.ul2
    }
}
#[cfg(any(target_arch = "x86",))]
impl ::core::cmp::Eq for FLOATOBJ {}
#[cfg(any(target_arch = "x86",))]
unsafe impl ::windows::core::Abi for FLOATOBJ {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct FLOATOBJ_XFORM {
    pub eM11: f32,
    pub eM12: f32,
    pub eM21: f32,
    pub eM22: f32,
    pub eDx: f32,
    pub eDy: f32,
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl FLOATOBJ_XFORM {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::default::Default for FLOATOBJ_XFORM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::fmt::Debug for FLOATOBJ_XFORM {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("FLOATOBJ_XFORM").field("eM11", &self.eM11).field("eM12", &self.eM12).field("eM21", &self.eM21).field("eM22", &self.eM22).field("eDx", &self.eDx).field("eDy", &self.eDy).finish()
    }
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::cmp::PartialEq for FLOATOBJ_XFORM {
    fn eq(&self, other: &Self) -> bool {
        self.eM11 == other.eM11 && self.eM12 == other.eM12 && self.eM21 == other.eM21 && self.eM22 == other.eM22 && self.eDx == other.eDx && self.eDy == other.eDy
    }
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::cmp::Eq for FLOATOBJ_XFORM {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
unsafe impl ::windows::core::Abi for FLOATOBJ_XFORM {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(any(target_arch = "x86",))]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct FLOATOBJ_XFORM {
    pub eM11: FLOATOBJ,
    pub eM12: FLOATOBJ,
    pub eM21: FLOATOBJ,
    pub eM22: FLOATOBJ,
    pub eDx: FLOATOBJ,
    pub eDy: FLOATOBJ,
}
#[cfg(any(target_arch = "x86",))]
impl FLOATOBJ_XFORM {}
#[cfg(any(target_arch = "x86",))]
impl ::core::default::Default for FLOATOBJ_XFORM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "x86",))]
impl ::core::fmt::Debug for FLOATOBJ_XFORM {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("FLOATOBJ_XFORM").field("eM11", &self.eM11).field("eM12", &self.eM12).field("eM21", &self.eM21).field("eM22", &self.eM22).field("eDx", &self.eDx).field("eDy", &self.eDy).finish()
    }
}
#[cfg(any(target_arch = "x86",))]
impl ::core::cmp::PartialEq for FLOATOBJ_XFORM {
    fn eq(&self, other: &Self) -> bool {
        self.eM11 == other.eM11 && self.eM12 == other.eM12 && self.eM21 == other.eM21 && self.eM22 == other.eM22 && self.eDx == other.eDx && self.eDy == other.eDy
    }
}
#[cfg(any(target_arch = "x86",))]
impl ::core::cmp::Eq for FLOATOBJ_XFORM {}
#[cfg(any(target_arch = "x86",))]
unsafe impl ::windows::core::Abi for FLOATOBJ_XFORM {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub union FLOAT_LONG {
    pub e: f32,
    pub l: i32,
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl FLOAT_LONG {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::default::Default for FLOAT_LONG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::cmp::PartialEq for FLOAT_LONG {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::cmp::Eq for FLOAT_LONG {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
unsafe impl ::windows::core::Abi for FLOAT_LONG {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(any(target_arch = "x86",))]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub union FLOAT_LONG {
    pub e: u32,
    pub l: i32,
}
#[cfg(any(target_arch = "x86",))]
impl FLOAT_LONG {}
#[cfg(any(target_arch = "x86",))]
impl ::core::default::Default for FLOAT_LONG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "x86",))]
impl ::core::cmp::PartialEq for FLOAT_LONG {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(any(target_arch = "x86",))]
impl ::core::cmp::Eq for FLOAT_LONG {}
#[cfg(any(target_arch = "x86",))]
unsafe impl ::windows::core::Abi for FLOAT_LONG {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const FL_NONPAGED_MEMORY: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const FL_NON_SESSION: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const FL_ZERO_MEMORY: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const FM_EDITABLE_EMBED: u32 = 8u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const FM_INFO_16BPP: u32 = 256u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const FM_INFO_1BPP: u32 = 32u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const FM_INFO_24BPP: u32 = 512u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const FM_INFO_32BPP: u32 = 1024u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const FM_INFO_4BPP: u32 = 64u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const FM_INFO_8BPP: u32 = 128u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const FM_INFO_90DEGREE_ROTATIONS: u32 = 2097152u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const FM_INFO_ANISOTROPIC_SCALING_ONLY: u32 = 33554432u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const FM_INFO_ARB_XFORMS: u32 = 16u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const FM_INFO_CONSTANT_WIDTH: u32 = 4096u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const FM_INFO_DBCS_FIXED_PITCH: u32 = 268435456u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const FM_INFO_DO_NOT_ENUMERATE: u32 = 8388608u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const FM_INFO_DSIG: u32 = 262144u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const FM_INFO_FAMILY_EQUIV: u32 = 134217728u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const FM_INFO_IGNORE_TC_RA_ABLE: u32 = 1073741824u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const FM_INFO_INTEGER_WIDTH: u32 = 2048u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const FM_INFO_INTEGRAL_SCALING: u32 = 1048576u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const FM_INFO_ISOTROPIC_SCALING_ONLY: u32 = 16777216u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const FM_INFO_NONNEGATIVE_AC: u32 = 536870912u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const FM_INFO_NOT_CONTIGUOUS: u32 = 8192u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const FM_INFO_OPTICALLY_FIXED_PITCH: u32 = 4194304u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const FM_INFO_RETURNS_BITMAPS: u32 = 131072u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const FM_INFO_RETURNS_OUTLINES: u32 = 32768u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const FM_INFO_RETURNS_STROKES: u32 = 65536u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const FM_INFO_RIGHT_HANDED: u32 = 524288u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const FM_INFO_TECH_BITMAP: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const FM_INFO_TECH_CFF: u32 = 67108864u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const FM_INFO_TECH_MM: u32 = 16384u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const FM_INFO_TECH_OUTLINE_NOT_TRUETYPE: u32 = 8u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const FM_INFO_TECH_STROKE: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const FM_INFO_TECH_TRUETYPE: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const FM_INFO_TECH_TYPE1: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const FM_NO_EMBEDDING: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const FM_PANOSE_CULTURE_LATIN: u32 = 0u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const FM_READONLY_EMBED: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const FM_SEL_BOLD: u32 = 32u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const FM_SEL_ITALIC: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const FM_SEL_NEGATIVE: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const FM_SEL_OUTLINED: u32 = 8u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const FM_SEL_REGULAR: u32 = 64u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const FM_SEL_STRIKEOUT: u32 = 16u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const FM_SEL_UNDERSCORE: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const FM_TYPE_LICENSED: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const FM_VERSION_NUMBER: u32 = 0u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
pub struct FONTDIFF {
    pub jReserved1: u8,
    pub jReserved2: u8,
    pub jReserved3: u8,
    pub bWeight: u8,
    pub usWinWeight: u16,
    pub fsSelection: u16,
    pub fwdAveCharWidth: i16,
    pub fwdMaxCharInc: i16,
    pub ptlCaret: super::super::Foundation::POINTL,
}
#[cfg(feature = "Win32_Foundation")]
impl FONTDIFF {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FONTDIFF {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FONTDIFF {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("FONTDIFF")
            .field("jReserved1", &self.jReserved1)
            .field("jReserved2", &self.jReserved2)
            .field("jReserved3", &self.jReserved3)
            .field("bWeight", &self.bWeight)
            .field("usWinWeight", &self.usWinWeight)
            .field("fsSelection", &self.fsSelection)
            .field("fwdAveCharWidth", &self.fwdAveCharWidth)
            .field("fwdMaxCharInc", &self.fwdMaxCharInc)
            .field("ptlCaret", &self.ptlCaret)
            .finish()
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
unsafe impl ::windows::core::Abi for FONTDIFF {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct FONTINFO {
    pub cjThis: u32,
    pub flCaps: u32,
    pub cGlyphsSupported: u32,
    pub cjMaxGlyph1: u32,
    pub cjMaxGlyph4: u32,
    pub cjMaxGlyph8: u32,
    pub cjMaxGlyph32: u32,
}
impl FONTINFO {}
impl ::core::default::Default for FONTINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for FONTINFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("FONTINFO")
            .field("cjThis", &self.cjThis)
            .field("flCaps", &self.flCaps)
            .field("cGlyphsSupported", &self.cGlyphsSupported)
            .field("cjMaxGlyph1", &self.cjMaxGlyph1)
            .field("cjMaxGlyph4", &self.cjMaxGlyph4)
            .field("cjMaxGlyph8", &self.cjMaxGlyph8)
            .field("cjMaxGlyph32", &self.cjMaxGlyph32)
            .finish()
    }
}
impl ::core::cmp::PartialEq for FONTINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cjThis == other.cjThis && self.flCaps == other.flCaps && self.cGlyphsSupported == other.cGlyphsSupported && self.cjMaxGlyph1 == other.cjMaxGlyph1 && self.cjMaxGlyph4 == other.cjMaxGlyph4 && self.cjMaxGlyph8 == other.cjMaxGlyph8 && self.cjMaxGlyph32 == other.cjMaxGlyph32
    }
}
impl ::core::cmp::Eq for FONTINFO {}
unsafe impl ::windows::core::Abi for FONTINFO {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
pub struct FONTOBJ {
    pub iUniq: u32,
    pub iFace: u32,
    pub cxMax: u32,
    pub flFontType: u32,
    pub iTTUniq: usize,
    pub iFile: usize,
    pub sizLogResPpi: super::super::Foundation::SIZE,
    pub ulStyleSize: u32,
    pub pvConsumer: *mut ::core::ffi::c_void,
    pub pvProducer: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl FONTOBJ {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FONTOBJ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FONTOBJ {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("FONTOBJ")
            .field("iUniq", &self.iUniq)
            .field("iFace", &self.iFace)
            .field("cxMax", &self.cxMax)
            .field("flFontType", &self.flFontType)
            .field("iTTUniq", &self.iTTUniq)
            .field("iFile", &self.iFile)
            .field("sizLogResPpi", &self.sizLogResPpi)
            .field("ulStyleSize", &self.ulStyleSize)
            .field("pvConsumer", &self.pvConsumer)
            .field("pvProducer", &self.pvProducer)
            .finish()
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
unsafe impl ::windows::core::Abi for FONTOBJ {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FONTOBJ_cGetAllGlyphHandles(pfo: *mut FONTOBJ, phg: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FONTOBJ_cGetAllGlyphHandles(pfo: *mut FONTOBJ, phg: *mut u32) -> u32;
        }
        ::core::mem::transmute(FONTOBJ_cGetAllGlyphHandles(::core::mem::transmute(pfo), ::core::mem::transmute(phg)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FONTOBJ_cGetGlyphs(pfo: *mut FONTOBJ, imode: u32, cglyph: u32, phg: *mut u32, ppvglyph: *mut *mut ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FONTOBJ_cGetGlyphs(pfo: *mut FONTOBJ, imode: u32, cglyph: u32, phg: *mut u32, ppvglyph: *mut *mut ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(FONTOBJ_cGetGlyphs(::core::mem::transmute(pfo), ::core::mem::transmute(imode), ::core::mem::transmute(cglyph), ::core::mem::transmute(phg), ::core::mem::transmute(ppvglyph)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FONTOBJ_pQueryGlyphAttrs(pfo: *mut FONTOBJ, imode: u32) -> *mut FD_GLYPHATTR {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FONTOBJ_pQueryGlyphAttrs(pfo: *mut FONTOBJ, imode: u32) -> *mut FD_GLYPHATTR;
        }
        ::core::mem::transmute(FONTOBJ_pQueryGlyphAttrs(::core::mem::transmute(pfo), ::core::mem::transmute(imode)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FONTOBJ_pfdg(pfo: *mut FONTOBJ) -> *mut FD_GLYPHSET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FONTOBJ_pfdg(pfo: *mut FONTOBJ) -> *mut FD_GLYPHSET;
        }
        ::core::mem::transmute(FONTOBJ_pfdg(::core::mem::transmute(pfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn FONTOBJ_pifi(pfo: *const FONTOBJ) -> *mut IFIMETRICS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FONTOBJ_pifi(pfo: *const FONTOBJ) -> *mut IFIMETRICS;
        }
        ::core::mem::transmute(FONTOBJ_pifi(::core::mem::transmute(pfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FONTOBJ_pvTrueTypeFontFile(pfo: *mut FONTOBJ, pcjfile: *mut u32) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FONTOBJ_pvTrueTypeFontFile(pfo: *mut FONTOBJ, pcjfile: *mut u32) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(FONTOBJ_pvTrueTypeFontFile(::core::mem::transmute(pfo), ::core::mem::transmute(pcjfile)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FONTOBJ_pxoGetXform(pfo: *const FONTOBJ) -> *mut XFORMOBJ {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FONTOBJ_pxoGetXform(pfo: *const FONTOBJ) -> *mut XFORMOBJ;
        }
        ::core::mem::transmute(FONTOBJ_pxoGetXform(::core::mem::transmute(pfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FONTOBJ_vGetInfo(pfo: *mut FONTOBJ, cjsize: u32, pfi: *mut FONTINFO) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FONTOBJ_vGetInfo(pfo: *mut FONTOBJ, cjsize: u32, pfi: *mut FONTINFO);
        }
        ::core::mem::transmute(FONTOBJ_vGetInfo(::core::mem::transmute(pfo), ::core::mem::transmute(cjsize), ::core::mem::transmute(pfi)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct FONTSIM {
    pub dpBold: i32,
    pub dpItalic: i32,
    pub dpBoldItalic: i32,
}
impl FONTSIM {}
impl ::core::default::Default for FONTSIM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for FONTSIM {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("FONTSIM").field("dpBold", &self.dpBold).field("dpItalic", &self.dpItalic).field("dpBoldItalic", &self.dpBoldItalic).finish()
    }
}
impl ::core::cmp::PartialEq for FONTSIM {
    fn eq(&self, other: &Self) -> bool {
        self.dpBold == other.dpBold && self.dpItalic == other.dpItalic && self.dpBoldItalic == other.dpBoldItalic
    }
}
impl ::core::cmp::Eq for FONTSIM {}
unsafe impl ::windows::core::Abi for FONTSIM {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
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
impl ::core::default::Default for FONT_IMAGE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Console")]
impl ::core::fmt::Debug for FONT_IMAGE_INFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("FONT_IMAGE_INFO").field("FontSize", &self.FontSize).field("ImageBits", &self.ImageBits).finish()
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
unsafe impl ::windows::core::Abi for FONT_IMAGE_INFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const FO_ATTR_MODE_ROTATE: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const FO_CFF: u32 = 1048576u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const FO_CLEARTYPENATURAL_X: u32 = 1073741824u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const FO_CLEARTYPE_X: u32 = 268435456u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const FO_CLEARTYPE_Y: u32 = 536870912u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const FO_DBCS_FONT: u32 = 16777216u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const FO_DEVICE_FONT: i32 = 1i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const FO_EM_HEIGHT: u32 = 32768u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const FO_GLYPHBITS: i32 = 1i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const FO_GRAY16: u32 = 65536u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const FO_HGLYPHS: i32 = 0i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const FO_MULTIPLEMASTER: u32 = 4194304u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const FO_NOCLEARTYPE: u32 = 33554432u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const FO_NOGRAY16: u32 = 131072u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const FO_NOHINTS: u32 = 262144u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const FO_NO_CHOICE: u32 = 524288u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const FO_OUTLINE_CAPABLE: i32 = 2i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const FO_PATHOBJ: i32 = 2i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const FO_POSTSCRIPT: u32 = 2097152u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const FO_SIM_BOLD: u32 = 8192u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const FO_SIM_ITALIC: u32 = 16384u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const FO_VERT_FACE: u32 = 8388608u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const FP_ALTERNATEMODE: i32 = 1i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const FP_WINDINGMODE: i32 = 2i32;
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type FREEOBJPROC = unsafe extern "system" fn(pdriverobj: *mut ::core::mem::ManuallyDrop<DRIVEROBJ>) -> super::super::Foundation::BOOL;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
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
impl ::core::default::Default for FSCNTL_SCREEN_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Console")]
impl ::core::fmt::Debug for FSCNTL_SCREEN_INFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("FSCNTL_SCREEN_INFO").field("Position", &self.Position).field("ScreenSize", &self.ScreenSize).field("nNumberOfChars", &self.nNumberOfChars).finish()
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
unsafe impl ::windows::core::Abi for FSCNTL_SCREEN_INFO {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
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
impl ::core::default::Default for FSVIDEO_COPY_FRAME_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Console")]
impl ::core::fmt::Debug for FSVIDEO_COPY_FRAME_BUFFER {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("FSVIDEO_COPY_FRAME_BUFFER").field("SrcScreen", &self.SrcScreen).field("DestScreen", &self.DestScreen).finish()
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
unsafe impl ::windows::core::Abi for FSVIDEO_COPY_FRAME_BUFFER {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct FSVIDEO_CURSOR_POSITION {
    pub Coord: VIDEO_CURSOR_POSITION,
    pub dwType: u32,
}
impl FSVIDEO_CURSOR_POSITION {}
impl ::core::default::Default for FSVIDEO_CURSOR_POSITION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for FSVIDEO_CURSOR_POSITION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("FSVIDEO_CURSOR_POSITION").field("Coord", &self.Coord).field("dwType", &self.dwType).finish()
    }
}
impl ::core::cmp::PartialEq for FSVIDEO_CURSOR_POSITION {
    fn eq(&self, other: &Self) -> bool {
        self.Coord == other.Coord && self.dwType == other.dwType
    }
}
impl ::core::cmp::Eq for FSVIDEO_CURSOR_POSITION {}
unsafe impl ::windows::core::Abi for FSVIDEO_CURSOR_POSITION {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct FSVIDEO_MODE_INFORMATION {
    pub VideoMode: VIDEO_MODE_INFORMATION,
    pub VideoMemory: VIDEO_MEMORY_INFORMATION,
}
impl FSVIDEO_MODE_INFORMATION {}
impl ::core::default::Default for FSVIDEO_MODE_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for FSVIDEO_MODE_INFORMATION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("FSVIDEO_MODE_INFORMATION").field("VideoMode", &self.VideoMode).field("VideoMemory", &self.VideoMemory).finish()
    }
}
impl ::core::cmp::PartialEq for FSVIDEO_MODE_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.VideoMode == other.VideoMode && self.VideoMemory == other.VideoMemory
    }
}
impl ::core::cmp::Eq for FSVIDEO_MODE_INFORMATION {}
unsafe impl ::windows::core::Abi for FSVIDEO_MODE_INFORMATION {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
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
impl ::core::default::Default for FSVIDEO_REVERSE_MOUSE_POINTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Console")]
impl ::core::fmt::Debug for FSVIDEO_REVERSE_MOUSE_POINTER {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("FSVIDEO_REVERSE_MOUSE_POINTER").field("Screen", &self.Screen).field("dwType", &self.dwType).finish()
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
unsafe impl ::windows::core::Abi for FSVIDEO_REVERSE_MOUSE_POINTER {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
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
impl ::core::default::Default for FSVIDEO_SCREEN_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Console")]
impl ::core::fmt::Debug for FSVIDEO_SCREEN_INFORMATION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("FSVIDEO_SCREEN_INFORMATION").field("ScreenSize", &self.ScreenSize).field("FontSize", &self.FontSize).finish()
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
unsafe impl ::windows::core::Abi for FSVIDEO_SCREEN_INFORMATION {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
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
impl ::core::default::Default for FSVIDEO_WRITE_TO_FRAME_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Console"))]
impl ::core::fmt::Debug for FSVIDEO_WRITE_TO_FRAME_BUFFER {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("FSVIDEO_WRITE_TO_FRAME_BUFFER").field("SrcBuffer", &self.SrcBuffer).field("DestScreen", &self.DestScreen).finish()
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
unsafe impl ::windows::core::Abi for FSVIDEO_WRITE_TO_FRAME_BUFFER {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct GAMMARAMP {
    pub Red: [u16; 256],
    pub Green: [u16; 256],
    pub Blue: [u16; 256],
}
impl GAMMARAMP {}
impl ::core::default::Default for GAMMARAMP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for GAMMARAMP {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("GAMMARAMP").field("Red", &self.Red).field("Green", &self.Green).field("Blue", &self.Blue).finish()
    }
}
impl ::core::cmp::PartialEq for GAMMARAMP {
    fn eq(&self, other: &Self) -> bool {
        self.Red == other.Red && self.Green == other.Green && self.Blue == other.Blue
    }
}
impl ::core::cmp::Eq for GAMMARAMP {}
unsafe impl ::windows::core::Abi for GAMMARAMP {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct GAMMA_RAMP_DXGI_1 {
    pub Scale: GAMMA_RAMP_RGB,
    pub Offset: GAMMA_RAMP_RGB,
    pub GammaCurve: [GAMMA_RAMP_RGB; 1025],
}
impl GAMMA_RAMP_DXGI_1 {}
impl ::core::default::Default for GAMMA_RAMP_DXGI_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for GAMMA_RAMP_DXGI_1 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("GAMMA_RAMP_DXGI_1").field("Scale", &self.Scale).field("Offset", &self.Offset).field("GammaCurve", &self.GammaCurve).finish()
    }
}
impl ::core::cmp::PartialEq for GAMMA_RAMP_DXGI_1 {
    fn eq(&self, other: &Self) -> bool {
        self.Scale == other.Scale && self.Offset == other.Offset && self.GammaCurve == other.GammaCurve
    }
}
impl ::core::cmp::Eq for GAMMA_RAMP_DXGI_1 {}
unsafe impl ::windows::core::Abi for GAMMA_RAMP_DXGI_1 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct GAMMA_RAMP_RGB {
    pub Red: f32,
    pub Green: f32,
    pub Blue: f32,
}
impl GAMMA_RAMP_RGB {}
impl ::core::default::Default for GAMMA_RAMP_RGB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for GAMMA_RAMP_RGB {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("GAMMA_RAMP_RGB").field("Red", &self.Red).field("Green", &self.Green).field("Blue", &self.Blue).finish()
    }
}
impl ::core::cmp::PartialEq for GAMMA_RAMP_RGB {
    fn eq(&self, other: &Self) -> bool {
        self.Red == other.Red && self.Green == other.Green && self.Blue == other.Blue
    }
}
impl ::core::cmp::Eq for GAMMA_RAMP_RGB {}
unsafe impl ::windows::core::Abi for GAMMA_RAMP_RGB {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct GAMMA_RAMP_RGB256x3x16 {
    pub Red: [u16; 256],
    pub Green: [u16; 256],
    pub Blue: [u16; 256],
}
impl GAMMA_RAMP_RGB256x3x16 {}
impl ::core::default::Default for GAMMA_RAMP_RGB256x3x16 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for GAMMA_RAMP_RGB256x3x16 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("GAMMA_RAMP_RGB256x3x16").field("Red", &self.Red).field("Green", &self.Green).field("Blue", &self.Blue).finish()
    }
}
impl ::core::cmp::PartialEq for GAMMA_RAMP_RGB256x3x16 {
    fn eq(&self, other: &Self) -> bool {
        self.Red == other.Red && self.Green == other.Green && self.Blue == other.Blue
    }
}
impl ::core::cmp::Eq for GAMMA_RAMP_RGB256x3x16 {}
unsafe impl ::windows::core::Abi for GAMMA_RAMP_RGB256x3x16 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const GCAPS2_ACC_DRIVER: u32 = 32768u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const GCAPS2_ALPHACURSOR: u32 = 32u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const GCAPS2_BITMAPEXREUSE: u32 = 65536u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const GCAPS2_CHANGEGAMMARAMP: u32 = 16u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const GCAPS2_CLEARTYPE: u32 = 16384u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const GCAPS2_EXCLUDELAYERED: u32 = 2048u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const GCAPS2_ICD_MULTIMON: u32 = 256u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const GCAPS2_INCLUDEAPIBITMAPS: u32 = 4096u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const GCAPS2_JPEGSRC: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const GCAPS2_MOUSETRAILS: u32 = 512u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const GCAPS2_PNGSRC: u32 = 8u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const GCAPS2_REMOTEDRIVER: u32 = 1024u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const GCAPS2_RESERVED1: u32 = 1024u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const GCAPS2_SHOWHIDDENPOINTER: u32 = 8192u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const GCAPS2_SYNCFLUSH: u32 = 64u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const GCAPS2_SYNCTIMER: u32 = 128u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const GCAPS2_xxxx: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const GCAPS_ALTERNATEFILL: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const GCAPS_ARBRUSHOPAQUE: u32 = 32768u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const GCAPS_ARBRUSHTEXT: u32 = 268435456u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const GCAPS_ASYNCCHANGE: u32 = 2048u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const GCAPS_ASYNCMOVE: u32 = 4096u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const GCAPS_BEZIERS: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const GCAPS_CMYKCOLOR: u32 = 67108864u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const GCAPS_COLOR_DITHER: u32 = 32u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const GCAPS_DIRECTDRAW: u32 = 16384u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const GCAPS_DITHERONREALIZE: u32 = 2097152u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const GCAPS_DONTJOURNAL: u32 = 8192u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const GCAPS_FONT_RASTERIZER: u32 = 1073741824u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const GCAPS_FORCEDITHER: u32 = 8388608u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const GCAPS_GEOMETRICWIDE: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const GCAPS_GRAY16: u32 = 16777216u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const GCAPS_HALFTONE: u32 = 16u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const GCAPS_HIGHRESTEXT: u32 = 262144u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const GCAPS_HORIZSTRIKE: u32 = 64u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const GCAPS_ICM: u32 = 33554432u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const GCAPS_LAYERED: u32 = 134217728u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const GCAPS_MONO_DITHER: u32 = 1024u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const GCAPS_NO64BITMEMACCESS: u32 = 4194304u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const GCAPS_NUP: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const GCAPS_OPAQUERECT: u32 = 256u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const GCAPS_PALMANAGED: u32 = 524288u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const GCAPS_PANNING: u32 = 65536u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const GCAPS_SCREENPRECISION: u32 = 536870912u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const GCAPS_VECTORFONT: u32 = 512u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const GCAPS_VERTSTRIKE: u32 = 128u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const GCAPS_WINDINGFILL: u32 = 8u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
pub struct GDIINFO {
    pub ulVersion: u32,
    pub ulTechnology: u32,
    pub ulHorzSize: u32,
    pub ulVertSize: u32,
    pub ulHorzRes: u32,
    pub ulVertRes: u32,
    pub cBitsPixel: u32,
    pub cPlanes: u32,
    pub ulNumColors: u32,
    pub flRaster: u32,
    pub ulLogPixelsX: u32,
    pub ulLogPixelsY: u32,
    pub flTextCaps: u32,
    pub ulDACRed: u32,
    pub ulDACGreen: u32,
    pub ulDACBlue: u32,
    pub ulAspectX: u32,
    pub ulAspectY: u32,
    pub ulAspectXY: u32,
    pub xStyleStep: i32,
    pub yStyleStep: i32,
    pub denStyleStep: i32,
    pub ptlPhysOffset: super::super::Foundation::POINTL,
    pub szlPhysSize: super::super::Foundation::SIZE,
    pub ulNumPalReg: u32,
    pub ciDevice: COLORINFO,
    pub ulDevicePelsDPI: u32,
    pub ulPrimaryOrder: u32,
    pub ulHTPatternSize: u32,
    pub ulHTOutputFormat: u32,
    pub flHTFlags: u32,
    pub ulVRefresh: u32,
    pub ulBltAlignment: u32,
    pub ulPanningHorzRes: u32,
    pub ulPanningVertRes: u32,
    pub xPanningAlignment: u32,
    pub yPanningAlignment: u32,
    pub cxHTPat: u32,
    pub cyHTPat: u32,
    pub pHTPatA: *mut u8,
    pub pHTPatB: *mut u8,
    pub pHTPatC: *mut u8,
    pub flShadeBlend: u32,
    pub ulPhysicalPixelCharacteristics: u32,
    pub ulPhysicalPixelGamma: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl GDIINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GDIINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for GDIINFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("GDIINFO")
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
unsafe impl ::windows::core::Abi for GDIINFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const GDI_DRIVER_VERSION: u32 = 16384u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const GETCONNECTEDIDS_SOURCE: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const GETCONNECTEDIDS_TARGET: u32 = 0u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
pub struct GLYPHBITS {
    pub ptlOrigin: super::super::Foundation::POINTL,
    pub sizlBitmap: super::super::Foundation::SIZE,
    pub aj: [u8; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl GLYPHBITS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GLYPHBITS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for GLYPHBITS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("GLYPHBITS").field("ptlOrigin", &self.ptlOrigin).field("sizlBitmap", &self.sizlBitmap).field("aj", &self.aj).finish()
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
unsafe impl ::windows::core::Abi for GLYPHBITS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
pub struct GLYPHDATA {
    pub gdf: GLYPHDEF,
    pub hg: u32,
    pub fxD: i32,
    pub fxA: i32,
    pub fxAB: i32,
    pub fxInkTop: i32,
    pub fxInkBottom: i32,
    pub rclInk: super::super::Foundation::RECTL,
    pub ptqD: POINTQF,
}
#[cfg(feature = "Win32_Foundation")]
impl GLYPHDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GLYPHDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GLYPHDATA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GLYPHDATA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for GLYPHDATA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
pub union GLYPHDEF {
    pub pgb: *mut GLYPHBITS,
    pub ppo: *mut PATHOBJ,
}
#[cfg(feature = "Win32_Foundation")]
impl GLYPHDEF {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GLYPHDEF {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GLYPHDEF {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GLYPHDEF {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for GLYPHDEF {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
pub struct GLYPHPOS {
    pub hg: u32,
    pub pgdf: *mut GLYPHDEF,
    pub ptl: super::super::Foundation::POINTL,
}
#[cfg(feature = "Win32_Foundation")]
impl GLYPHPOS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GLYPHPOS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for GLYPHPOS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("GLYPHPOS").field("hg", &self.hg).field("pgdf", &self.pgdf).field("ptl", &self.ptl).finish()
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
unsafe impl ::windows::core::Abi for GLYPHPOS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const GS_16BIT_HANDLES: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const GS_8BIT_HANDLES: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const GS_UNICODE_HANDLES: u32 = 1u32;
pub const GUID_DEVINTERFACE_DISPLAY_ADAPTER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5b45201d_f2f2_4f3b_85bb_30ff1f953599);
pub const GUID_DEVINTERFACE_MONITOR: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe6f07b5f_ee97_4a90_b076_33f57bf4eaa7);
pub const GUID_DEVINTERFACE_VIDEO_OUTPUT_ARRIVAL: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1ad9e4f0_f88d_4360_bab9_4c2d55e564cd);
pub const GUID_DISPLAY_DEVICE_ARRIVAL: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1ca05180_a699_450a_9a0c_de4fbe3ddd89);
pub const GUID_MONITOR_OVERRIDE_PSEUDO_SPECIALIZED: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf196c02f_f86f_4f9a_aa15_e9cebdfe3b96);
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const GX_GENERAL: i32 = 3i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const GX_IDENTITY: i32 = 0i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const GX_OFFSET: i32 = 1i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const GX_SCALE: i32 = 2i32;
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
        ::core::mem::transmute(GetAutoRotationState(::core::mem::transmute(pstate)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetCapabilitiesStringLength<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hmonitor: Param0, pdwcapabilitiesstringlengthincharacters: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetCapabilitiesStringLength(hmonitor: super::super::Foundation::HANDLE, pdwcapabilitiesstringlengthincharacters: *mut u32) -> i32;
        }
        ::core::mem::transmute(GetCapabilitiesStringLength(hmonitor.into_param().abi(), ::core::mem::transmute(pdwcapabilitiesstringlengthincharacters)))
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
        ::core::mem::transmute(GetDisplayAutoRotationPreferences(::core::mem::transmute(porientation)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
#[inline]
pub unsafe fn GetDisplayConfigBufferSizes(flags: u32, numpatharrayelements: *mut u32, nummodeinfoarrayelements: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetDisplayConfigBufferSizes(flags: u32, numpatharrayelements: *mut u32, nummodeinfoarrayelements: *mut u32) -> i32;
        }
        ::core::mem::transmute(GetDisplayConfigBufferSizes(::core::mem::transmute(flags), ::core::mem::transmute(numpatharrayelements), ::core::mem::transmute(nummodeinfoarrayelements)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetMonitorBrightness<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hmonitor: Param0, pdwminimumbrightness: *mut u32, pdwcurrentbrightness: *mut u32, pdwmaximumbrightness: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetMonitorBrightness(hmonitor: super::super::Foundation::HANDLE, pdwminimumbrightness: *mut u32, pdwcurrentbrightness: *mut u32, pdwmaximumbrightness: *mut u32) -> i32;
        }
        ::core::mem::transmute(GetMonitorBrightness(hmonitor.into_param().abi(), ::core::mem::transmute(pdwminimumbrightness), ::core::mem::transmute(pdwcurrentbrightness), ::core::mem::transmute(pdwmaximumbrightness)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetMonitorCapabilities<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hmonitor: Param0, pdwmonitorcapabilities: *mut u32, pdwsupportedcolortemperatures: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetMonitorCapabilities(hmonitor: super::super::Foundation::HANDLE, pdwmonitorcapabilities: *mut u32, pdwsupportedcolortemperatures: *mut u32) -> i32;
        }
        ::core::mem::transmute(GetMonitorCapabilities(hmonitor.into_param().abi(), ::core::mem::transmute(pdwmonitorcapabilities), ::core::mem::transmute(pdwsupportedcolortemperatures)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetMonitorColorTemperature<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hmonitor: Param0, pctcurrentcolortemperature: *mut MC_COLOR_TEMPERATURE) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetMonitorColorTemperature(hmonitor: super::super::Foundation::HANDLE, pctcurrentcolortemperature: *mut MC_COLOR_TEMPERATURE) -> i32;
        }
        ::core::mem::transmute(GetMonitorColorTemperature(hmonitor.into_param().abi(), ::core::mem::transmute(pctcurrentcolortemperature)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetMonitorContrast<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hmonitor: Param0, pdwminimumcontrast: *mut u32, pdwcurrentcontrast: *mut u32, pdwmaximumcontrast: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetMonitorContrast(hmonitor: super::super::Foundation::HANDLE, pdwminimumcontrast: *mut u32, pdwcurrentcontrast: *mut u32, pdwmaximumcontrast: *mut u32) -> i32;
        }
        ::core::mem::transmute(GetMonitorContrast(hmonitor.into_param().abi(), ::core::mem::transmute(pdwminimumcontrast), ::core::mem::transmute(pdwcurrentcontrast), ::core::mem::transmute(pdwmaximumcontrast)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetMonitorDisplayAreaPosition<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hmonitor: Param0, ptpositiontype: MC_POSITION_TYPE, pdwminimumposition: *mut u32, pdwcurrentposition: *mut u32, pdwmaximumposition: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetMonitorDisplayAreaPosition(hmonitor: super::super::Foundation::HANDLE, ptpositiontype: MC_POSITION_TYPE, pdwminimumposition: *mut u32, pdwcurrentposition: *mut u32, pdwmaximumposition: *mut u32) -> i32;
        }
        ::core::mem::transmute(GetMonitorDisplayAreaPosition(hmonitor.into_param().abi(), ::core::mem::transmute(ptpositiontype), ::core::mem::transmute(pdwminimumposition), ::core::mem::transmute(pdwcurrentposition), ::core::mem::transmute(pdwmaximumposition)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetMonitorDisplayAreaSize<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hmonitor: Param0, stsizetype: MC_SIZE_TYPE, pdwminimumwidthorheight: *mut u32, pdwcurrentwidthorheight: *mut u32, pdwmaximumwidthorheight: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetMonitorDisplayAreaSize(hmonitor: super::super::Foundation::HANDLE, stsizetype: MC_SIZE_TYPE, pdwminimumwidthorheight: *mut u32, pdwcurrentwidthorheight: *mut u32, pdwmaximumwidthorheight: *mut u32) -> i32;
        }
        ::core::mem::transmute(GetMonitorDisplayAreaSize(hmonitor.into_param().abi(), ::core::mem::transmute(stsizetype), ::core::mem::transmute(pdwminimumwidthorheight), ::core::mem::transmute(pdwcurrentwidthorheight), ::core::mem::transmute(pdwmaximumwidthorheight)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetMonitorRedGreenOrBlueDrive<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hmonitor: Param0, dtdrivetype: MC_DRIVE_TYPE, pdwminimumdrive: *mut u32, pdwcurrentdrive: *mut u32, pdwmaximumdrive: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetMonitorRedGreenOrBlueDrive(hmonitor: super::super::Foundation::HANDLE, dtdrivetype: MC_DRIVE_TYPE, pdwminimumdrive: *mut u32, pdwcurrentdrive: *mut u32, pdwmaximumdrive: *mut u32) -> i32;
        }
        ::core::mem::transmute(GetMonitorRedGreenOrBlueDrive(hmonitor.into_param().abi(), ::core::mem::transmute(dtdrivetype), ::core::mem::transmute(pdwminimumdrive), ::core::mem::transmute(pdwcurrentdrive), ::core::mem::transmute(pdwmaximumdrive)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetMonitorRedGreenOrBlueGain<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hmonitor: Param0, gtgaintype: MC_GAIN_TYPE, pdwminimumgain: *mut u32, pdwcurrentgain: *mut u32, pdwmaximumgain: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetMonitorRedGreenOrBlueGain(hmonitor: super::super::Foundation::HANDLE, gtgaintype: MC_GAIN_TYPE, pdwminimumgain: *mut u32, pdwcurrentgain: *mut u32, pdwmaximumgain: *mut u32) -> i32;
        }
        ::core::mem::transmute(GetMonitorRedGreenOrBlueGain(hmonitor.into_param().abi(), ::core::mem::transmute(gtgaintype), ::core::mem::transmute(pdwminimumgain), ::core::mem::transmute(pdwcurrentgain), ::core::mem::transmute(pdwmaximumgain)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetMonitorTechnologyType<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hmonitor: Param0, pdtydisplaytechnologytype: *mut MC_DISPLAY_TECHNOLOGY_TYPE) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetMonitorTechnologyType(hmonitor: super::super::Foundation::HANDLE, pdtydisplaytechnologytype: *mut MC_DISPLAY_TECHNOLOGY_TYPE) -> i32;
        }
        ::core::mem::transmute(GetMonitorTechnologyType(hmonitor.into_param().abi(), ::core::mem::transmute(pdtydisplaytechnologytype)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Graphics_Gdi`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetNumberOfPhysicalMonitorsFromHMONITOR<'a, Param0: ::windows::core::IntoParam<'a, super::super::Graphics::Gdi::HMONITOR>>(hmonitor: Param0, pdwnumberofphysicalmonitors: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetNumberOfPhysicalMonitorsFromHMONITOR(hmonitor: super::super::Graphics::Gdi::HMONITOR, pdwnumberofphysicalmonitors: *mut u32) -> i32;
        }
        ::core::mem::transmute(GetNumberOfPhysicalMonitorsFromHMONITOR(hmonitor.into_param().abi(), ::core::mem::transmute(pdwnumberofphysicalmonitors)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Graphics_Direct3D9`*"]
#[cfg(feature = "Win32_Graphics_Direct3D9")]
#[inline]
pub unsafe fn GetNumberOfPhysicalMonitorsFromIDirect3DDevice9<'a, Param0: ::windows::core::IntoParam<'a, super::super::Graphics::Direct3D9::IDirect3DDevice9>>(pdirect3ddevice9: Param0) -> ::windows::core::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetNumberOfPhysicalMonitorsFromIDirect3DDevice9(pdirect3ddevice9: ::windows::core::RawPtr, pdwnumberofphysicalmonitors: *mut u32) -> ::windows::core::HRESULT;
        }
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        GetNumberOfPhysicalMonitorsFromIDirect3DDevice9(pdirect3ddevice9.into_param().abi(), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn GetPhysicalMonitorsFromHMONITOR<'a, Param0: ::windows::core::IntoParam<'a, super::super::Graphics::Gdi::HMONITOR>>(hmonitor: Param0, dwphysicalmonitorarraysize: u32, pphysicalmonitorarray: *mut PHYSICAL_MONITOR) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPhysicalMonitorsFromHMONITOR(hmonitor: super::super::Graphics::Gdi::HMONITOR, dwphysicalmonitorarraysize: u32, pphysicalmonitorarray: *mut PHYSICAL_MONITOR) -> i32;
        }
        ::core::mem::transmute(GetPhysicalMonitorsFromHMONITOR(hmonitor.into_param().abi(), ::core::mem::transmute(dwphysicalmonitorarraysize), ::core::mem::transmute(pphysicalmonitorarray)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Direct3D9`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9"))]
#[inline]
pub unsafe fn GetPhysicalMonitorsFromIDirect3DDevice9<'a, Param0: ::windows::core::IntoParam<'a, super::super::Graphics::Direct3D9::IDirect3DDevice9>>(pdirect3ddevice9: Param0, dwphysicalmonitorarraysize: u32, pphysicalmonitorarray: *mut PHYSICAL_MONITOR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPhysicalMonitorsFromIDirect3DDevice9(pdirect3ddevice9: ::windows::core::RawPtr, dwphysicalmonitorarraysize: u32, pphysicalmonitorarray: *mut PHYSICAL_MONITOR) -> ::windows::core::HRESULT;
        }
        GetPhysicalMonitorsFromIDirect3DDevice9(pdirect3ddevice9.into_param().abi(), ::core::mem::transmute(dwphysicalmonitorarraysize), ::core::mem::transmute(pphysicalmonitorarray)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetTimingReport<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hmonitor: Param0, pmtrmonitortimingreport: *mut MC_TIMING_REPORT) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetTimingReport(hmonitor: super::super::Foundation::HANDLE, pmtrmonitortimingreport: *mut MC_TIMING_REPORT) -> i32;
        }
        ::core::mem::transmute(GetTimingReport(hmonitor.into_param().abi(), ::core::mem::transmute(pmtrmonitortimingreport)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetVCPFeatureAndVCPFeatureReply<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hmonitor: Param0, bvcpcode: u8, pvct: *mut MC_VCP_CODE_TYPE, pdwcurrentvalue: *mut u32, pdwmaximumvalue: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetVCPFeatureAndVCPFeatureReply(hmonitor: super::super::Foundation::HANDLE, bvcpcode: u8, pvct: *mut MC_VCP_CODE_TYPE, pdwcurrentvalue: *mut u32, pdwmaximumvalue: *mut u32) -> i32;
        }
        ::core::mem::transmute(GetVCPFeatureAndVCPFeatureReply(hmonitor.into_param().abi(), ::core::mem::transmute(bvcpcode), ::core::mem::transmute(pvct), ::core::mem::transmute(pdwcurrentvalue), ::core::mem::transmute(pdwmaximumvalue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
#[repr(transparent)]
pub struct HBM(pub isize);
impl ::core::default::Default for HBM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
unsafe impl ::windows::core::Handle for HBM {}
unsafe impl ::windows::core::Abi for HBM {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
#[repr(transparent)]
pub struct HDEV(pub isize);
impl ::core::default::Default for HDEV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
unsafe impl ::windows::core::Handle for HDEV {}
unsafe impl ::windows::core::Abi for HDEV {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
#[repr(transparent)]
pub struct HDRVOBJ(pub isize);
impl ::core::default::Default for HDRVOBJ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
unsafe impl ::windows::core::Handle for HDRVOBJ {}
unsafe impl ::windows::core::Abi for HDRVOBJ {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
#[repr(transparent)]
pub struct HFASTMUTEX(pub isize);
impl ::core::default::Default for HFASTMUTEX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
unsafe impl ::windows::core::Handle for HFASTMUTEX {}
unsafe impl ::windows::core::Abi for HFASTMUTEX {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const HOOK_ALPHABLEND: u32 = 65536u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const HOOK_BITBLT: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const HOOK_COPYBITS: u32 = 1024u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const HOOK_FILLPATH: u32 = 64u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const HOOK_FLAGS: u32 = 243199u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const HOOK_GRADIENTFILL: u32 = 131072u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const HOOK_LINETO: u32 = 256u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const HOOK_MOVEPANNING: u32 = 2048u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const HOOK_PAINT: u32 = 16u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const HOOK_PLGBLT: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const HOOK_STRETCHBLT: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const HOOK_STRETCHBLTROP: u32 = 8192u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const HOOK_STROKEANDFILLPATH: u32 = 128u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const HOOK_STROKEPATH: u32 = 32u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const HOOK_SYNCHRONIZE: u32 = 4096u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const HOOK_SYNCHRONIZEACCESS: u32 = 16384u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const HOOK_TEXTOUT: u32 = 8u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const HOOK_TRANSPARENTBLT: u32 = 32768u32;
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
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
#[repr(transparent)]
pub struct HSEMAPHORE(pub isize);
impl ::core::default::Default for HSEMAPHORE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
unsafe impl ::windows::core::Handle for HSEMAPHORE {}
unsafe impl ::windows::core::Abi for HSEMAPHORE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
#[repr(transparent)]
pub struct HSURF(pub isize);
impl ::core::default::Default for HSURF {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
unsafe impl ::windows::core::Handle for HSURF {}
unsafe impl ::windows::core::Abi for HSURF {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const HS_DDI_MAX: u32 = 6u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const HT_FLAG_8BPP_CMY332_MASK: u32 = 4278190080u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const HT_FLAG_ADDITIVE_PRIMS: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const HT_FLAG_DO_DEVCLR_XFORM: u32 = 128u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const HT_FLAG_HAS_BLACK_DYE: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const HT_FLAG_INK_ABSORPTION_IDX0: u32 = 0u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const HT_FLAG_INK_ABSORPTION_IDX1: u32 = 32u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const HT_FLAG_INK_ABSORPTION_IDX2: u32 = 64u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const HT_FLAG_INK_ABSORPTION_IDX3: u32 = 96u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const HT_FLAG_INK_ABSORPTION_INDICES: u32 = 96u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const HT_FLAG_INK_HIGH_ABSORPTION: u32 = 16u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const HT_FLAG_INVERT_8BPP_BITMASK_IDX: u32 = 1024u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const HT_FLAG_LOWER_INK_ABSORPTION: u32 = 64u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const HT_FLAG_LOWEST_INK_ABSORPTION: u32 = 96u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const HT_FLAG_LOW_INK_ABSORPTION: u32 = 32u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const HT_FLAG_NORMAL_INK_ABSORPTION: u32 = 0u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const HT_FLAG_OUTPUT_CMY: u32 = 256u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const HT_FLAG_PRINT_DRAFT_MODE: u32 = 512u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const HT_FLAG_SQUARE_DEVICE_PEL: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const HT_FLAG_USE_8BPP_BITMASK: u32 = 8u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const HT_FORMAT_16BPP: u32 = 5u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const HT_FORMAT_1BPP: u32 = 0u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const HT_FORMAT_24BPP: u32 = 6u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const HT_FORMAT_32BPP: u32 = 7u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const HT_FORMAT_4BPP: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const HT_FORMAT_4BPP_IRGB: u32 = 3u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const HT_FORMAT_8BPP: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Graphics_Gdi`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HT_Get8BPPFormatPalette(ppaletteentry: *mut super::super::Graphics::Gdi::PALETTEENTRY, redgamma: u16, greengamma: u16, bluegamma: u16) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HT_Get8BPPFormatPalette(ppaletteentry: *mut super::super::Graphics::Gdi::PALETTEENTRY, redgamma: u16, greengamma: u16, bluegamma: u16) -> i32;
        }
        ::core::mem::transmute(HT_Get8BPPFormatPalette(::core::mem::transmute(ppaletteentry), ::core::mem::transmute(redgamma), ::core::mem::transmute(greengamma), ::core::mem::transmute(bluegamma)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn HT_Get8BPPMaskPalette<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(ppaletteentry: *mut super::super::Graphics::Gdi::PALETTEENTRY, use8bppmaskpal: Param1, cmymask: u8, redgamma: u16, greengamma: u16, bluegamma: u16) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HT_Get8BPPMaskPalette(ppaletteentry: *mut super::super::Graphics::Gdi::PALETTEENTRY, use8bppmaskpal: super::super::Foundation::BOOL, cmymask: u8, redgamma: u16, greengamma: u16, bluegamma: u16) -> i32;
        }
        ::core::mem::transmute(HT_Get8BPPMaskPalette(::core::mem::transmute(ppaletteentry), use8bppmaskpal.into_param().abi(), ::core::mem::transmute(cmymask), ::core::mem::transmute(redgamma), ::core::mem::transmute(greengamma), ::core::mem::transmute(bluegamma)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const HT_PATSIZE_10x10: u32 = 8u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const HT_PATSIZE_10x10_M: u32 = 9u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const HT_PATSIZE_12x12: u32 = 10u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const HT_PATSIZE_12x12_M: u32 = 11u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const HT_PATSIZE_14x14: u32 = 12u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const HT_PATSIZE_14x14_M: u32 = 13u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const HT_PATSIZE_16x16: u32 = 14u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const HT_PATSIZE_16x16_M: u32 = 15u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const HT_PATSIZE_2x2: u32 = 0u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const HT_PATSIZE_2x2_M: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const HT_PATSIZE_4x4: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const HT_PATSIZE_4x4_M: u32 = 3u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const HT_PATSIZE_6x6: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const HT_PATSIZE_6x6_M: u32 = 5u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const HT_PATSIZE_8x8: u32 = 6u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const HT_PATSIZE_8x8_M: u32 = 7u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const HT_PATSIZE_DEFAULT: u32 = 17u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const HT_PATSIZE_MAX_INDEX: u32 = 18u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const HT_PATSIZE_SUPERCELL: u32 = 16u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const HT_PATSIZE_SUPERCELL_M: u32 = 17u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const HT_PATSIZE_USER: u32 = 18u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const HT_USERPAT_CX_MAX: u32 = 256u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const HT_USERPAT_CX_MIN: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const HT_USERPAT_CY_MAX: u32 = 256u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const HT_USERPAT_CY_MIN: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ICloneViewHelper(pub ::windows::core::IUnknown);
impl ICloneViewHelper {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    pub unsafe fn GetConnectedIDs<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wszadaptorname: Param0, pulcount: *mut u32, pulid: *mut u32, ulflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), wszadaptorname.into_param().abi(), ::core::mem::transmute(pulcount), ::core::mem::transmute(pulid), ::core::mem::transmute(ulflags)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    pub unsafe fn GetActiveTopology<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wszadaptorname: Param0, ulsourceid: u32, pulcount: *mut u32, pultargetid: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), wszadaptorname.into_param().abi(), ::core::mem::transmute(ulsourceid), ::core::mem::transmute(pulcount), ::core::mem::transmute(pultargetid)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    pub unsafe fn SetActiveTopology<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wszadaptorname: Param0, ulsourceid: u32, ulcount: u32, pultargetid: *const u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), wszadaptorname.into_param().abi(), ::core::mem::transmute(ulsourceid), ::core::mem::transmute(ulcount), ::core::mem::transmute(pultargetid)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    pub unsafe fn Commit<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, ffinalcall: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ffinalcall.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for ICloneViewHelper {
    type Vtable = ICloneViewHelper_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf6a3d4c4_5632_4d83_b0a1_fb88712b1eb7);
}
impl ::core::convert::From<ICloneViewHelper> for ::windows::core::IUnknown {
    fn from(value: ICloneViewHelper) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ICloneViewHelper> for ::windows::core::IUnknown {
    fn from(value: &ICloneViewHelper) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ICloneViewHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ICloneViewHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICloneViewHelper_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, wszadaptorname: super::super::Foundation::PWSTR, pulcount: *mut u32, pulid: *mut u32, ulflags: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, wszadaptorname: super::super::Foundation::PWSTR, ulsourceid: u32, pulcount: *mut u32, pultargetid: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, wszadaptorname: super::super::Foundation::PWSTR, ulsourceid: u32, ulcount: u32, pultargetid: *const u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ffinalcall: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct IFIEXTRA {
    pub ulIdentifier: u32,
    pub dpFontSig: i32,
    pub cig: u32,
    pub dpDesignVector: i32,
    pub dpAxesInfoW: i32,
    pub aulReserved: [u32; 1],
}
impl IFIEXTRA {}
impl ::core::default::Default for IFIEXTRA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for IFIEXTRA {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("IFIEXTRA").field("ulIdentifier", &self.ulIdentifier).field("dpFontSig", &self.dpFontSig).field("cig", &self.cig).field("dpDesignVector", &self.dpDesignVector).field("dpAxesInfoW", &self.dpAxesInfoW).field("aulReserved", &self.aulReserved).finish()
    }
}
impl ::core::cmp::PartialEq for IFIEXTRA {
    fn eq(&self, other: &Self) -> bool {
        self.ulIdentifier == other.ulIdentifier && self.dpFontSig == other.dpFontSig && self.cig == other.cig && self.dpDesignVector == other.dpDesignVector && self.dpAxesInfoW == other.dpAxesInfoW && self.aulReserved == other.aulReserved
    }
}
impl ::core::cmp::Eq for IFIEXTRA {}
unsafe impl ::windows::core::Abi for IFIEXTRA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
pub struct IFIMETRICS {
    pub cjThis: u32,
    pub cjIfiExtra: u32,
    pub dpwszFamilyName: i32,
    pub dpwszStyleName: i32,
    pub dpwszFaceName: i32,
    pub dpwszUniqueName: i32,
    pub dpFontSim: i32,
    pub lEmbedId: i32,
    pub lItalicAngle: i32,
    pub lCharBias: i32,
    pub dpCharSets: i32,
    pub jWinCharSet: u8,
    pub jWinPitchAndFamily: u8,
    pub usWinWeight: u16,
    pub flInfo: u32,
    pub fsSelection: u16,
    pub fsType: u16,
    pub fwdUnitsPerEm: i16,
    pub fwdLowestPPEm: i16,
    pub fwdWinAscender: i16,
    pub fwdWinDescender: i16,
    pub fwdMacAscender: i16,
    pub fwdMacDescender: i16,
    pub fwdMacLineGap: i16,
    pub fwdTypoAscender: i16,
    pub fwdTypoDescender: i16,
    pub fwdTypoLineGap: i16,
    pub fwdAveCharWidth: i16,
    pub fwdMaxCharInc: i16,
    pub fwdCapHeight: i16,
    pub fwdXHeight: i16,
    pub fwdSubscriptXSize: i16,
    pub fwdSubscriptYSize: i16,
    pub fwdSubscriptXOffset: i16,
    pub fwdSubscriptYOffset: i16,
    pub fwdSuperscriptXSize: i16,
    pub fwdSuperscriptYSize: i16,
    pub fwdSuperscriptXOffset: i16,
    pub fwdSuperscriptYOffset: i16,
    pub fwdUnderscoreSize: i16,
    pub fwdUnderscorePosition: i16,
    pub fwdStrikeoutSize: i16,
    pub fwdStrikeoutPosition: i16,
    pub chFirstChar: u8,
    pub chLastChar: u8,
    pub chDefaultChar: u8,
    pub chBreakChar: u8,
    pub wcFirstChar: u16,
    pub wcLastChar: u16,
    pub wcDefaultChar: u16,
    pub wcBreakChar: u16,
    pub ptlBaseline: super::super::Foundation::POINTL,
    pub ptlAspect: super::super::Foundation::POINTL,
    pub ptlCaret: super::super::Foundation::POINTL,
    pub rclFontBox: super::super::Foundation::RECTL,
    pub achVendId: [u8; 4],
    pub cKerningPairs: u32,
    pub ulPanoseCulture: u32,
    pub panose: super::super::Graphics::Gdi::PANOSE,
    pub Align: *mut ::core::ffi::c_void,
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl IFIMETRICS {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for IFIMETRICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for IFIMETRICS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("IFIMETRICS")
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
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
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
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for IFIMETRICS {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::core::Abi for IFIMETRICS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(any(target_arch = "x86",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
pub struct IFIMETRICS {
    pub cjThis: u32,
    pub cjIfiExtra: u32,
    pub dpwszFamilyName: i32,
    pub dpwszStyleName: i32,
    pub dpwszFaceName: i32,
    pub dpwszUniqueName: i32,
    pub dpFontSim: i32,
    pub lEmbedId: i32,
    pub lItalicAngle: i32,
    pub lCharBias: i32,
    pub dpCharSets: i32,
    pub jWinCharSet: u8,
    pub jWinPitchAndFamily: u8,
    pub usWinWeight: u16,
    pub flInfo: u32,
    pub fsSelection: u16,
    pub fsType: u16,
    pub fwdUnitsPerEm: i16,
    pub fwdLowestPPEm: i16,
    pub fwdWinAscender: i16,
    pub fwdWinDescender: i16,
    pub fwdMacAscender: i16,
    pub fwdMacDescender: i16,
    pub fwdMacLineGap: i16,
    pub fwdTypoAscender: i16,
    pub fwdTypoDescender: i16,
    pub fwdTypoLineGap: i16,
    pub fwdAveCharWidth: i16,
    pub fwdMaxCharInc: i16,
    pub fwdCapHeight: i16,
    pub fwdXHeight: i16,
    pub fwdSubscriptXSize: i16,
    pub fwdSubscriptYSize: i16,
    pub fwdSubscriptXOffset: i16,
    pub fwdSubscriptYOffset: i16,
    pub fwdSuperscriptXSize: i16,
    pub fwdSuperscriptYSize: i16,
    pub fwdSuperscriptXOffset: i16,
    pub fwdSuperscriptYOffset: i16,
    pub fwdUnderscoreSize: i16,
    pub fwdUnderscorePosition: i16,
    pub fwdStrikeoutSize: i16,
    pub fwdStrikeoutPosition: i16,
    pub chFirstChar: u8,
    pub chLastChar: u8,
    pub chDefaultChar: u8,
    pub chBreakChar: u8,
    pub wcFirstChar: u16,
    pub wcLastChar: u16,
    pub wcDefaultChar: u16,
    pub wcBreakChar: u16,
    pub ptlBaseline: super::super::Foundation::POINTL,
    pub ptlAspect: super::super::Foundation::POINTL,
    pub ptlCaret: super::super::Foundation::POINTL,
    pub rclFontBox: super::super::Foundation::RECTL,
    pub achVendId: [u8; 4],
    pub cKerningPairs: u32,
    pub ulPanoseCulture: u32,
    pub panose: super::super::Graphics::Gdi::PANOSE,
}
#[cfg(any(target_arch = "x86",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl IFIMETRICS {}
#[cfg(any(target_arch = "x86",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for IFIMETRICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "x86",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for IFIMETRICS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("IFIMETRICS")
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
#[cfg(any(target_arch = "x86",))]
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
#[cfg(any(target_arch = "x86",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for IFIMETRICS {}
#[cfg(any(target_arch = "x86",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::core::Abi for IFIMETRICS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const IGRF_RGB_256BYTES: u32 = 0u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const IGRF_RGB_256WORDS: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvAccumulateD3DDirtyRect: i32 = 98i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvAlphaBlend: i32 = 71i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvAssertMode: i32 = 5i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvAssociateSharedSurface: i32 = 96i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvBitBlt: i32 = 18i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvCompletePDEV: i32 = 1i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvCopyBits: i32 = 19i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvCreateDeviceBitmap: i32 = 10i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvCreateDeviceBitmapEx: i32 = 94i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvDeleteDeviceBitmap: i32 = 11i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvDeleteDeviceBitmapEx: i32 = 95i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvDeriveSurface: i32 = 85i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvDescribePixelFormat: i32 = 55i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvDestroyFont: i32 = 43i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvDisableDirectDraw: i32 = 61i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvDisableDriver: i32 = 8i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvDisablePDEV: i32 = 2i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvDisableSurface: i32 = 4i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvDitherColor: i32 = 13i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvDrawEscape: i32 = 25i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvEnableDirectDraw: i32 = 60i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvEnablePDEV: i32 = 0i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvEnableSurface: i32 = 3i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvEndDoc: i32 = 34i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvEndDxInterop: i32 = 100i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvEscape: i32 = 24i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvFillPath: i32 = 15i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvFontManagement: i32 = 47i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvFree: i32 = 42i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvGetDirectDrawInfo: i32 = 59i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvGetGlyphMode: i32 = 37i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvGetModes: i32 = 41i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvGetSynthesizedFontFiles: i32 = 73i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvGetTrueTypeFile: i32 = 50i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvGradientFill: i32 = 68i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvIcmCheckBitmapBits: i32 = 66i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvIcmCreateColorTransform: i32 = 64i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvIcmDeleteColorTransform: i32 = 65i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvIcmSetDeviceGammaRamp: i32 = 67i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvLineTo: i32 = 31i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvLoadFontFile: i32 = 45i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvLockDisplayArea: i32 = 101i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvMovePanning: i32 = 52i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvMovePointer: i32 = 30i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvNextBand: i32 = 58i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvNotify: i32 = 87i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvOffset: i32 = 6i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvPaint: i32 = 17i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvPlgBlt: i32 = 70i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvQueryAdvanceWidths: i32 = 53i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvQueryDeviceSupport: i32 = 76i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvQueryFont: i32 = 26i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvQueryFontCaps: i32 = 44i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvQueryFontData: i32 = 28i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvQueryFontFile: i32 = 51i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvQueryFontTree: i32 = 27i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvQueryGlyphAttrs: i32 = 86i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvQueryPerBandInfo: i32 = 75i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvQuerySpoolType: i32 = 62i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvQueryTrueTypeOutline: i32 = 49i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvQueryTrueTypeTable: i32 = 48i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvRealizeBrush: i32 = 12i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvRenderHint: i32 = 93i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvReserved1: i32 = 77i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvReserved10: i32 = 91i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvReserved11: i32 = 92i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvReserved2: i32 = 78i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvReserved3: i32 = 79i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvReserved4: i32 = 80i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvReserved5: i32 = 81i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvReserved6: i32 = 82i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvReserved7: i32 = 83i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvReserved8: i32 = 84i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvReserved9: i32 = 90i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvResetDevice: i32 = 89i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvResetPDEV: i32 = 7i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvSaveScreenBits: i32 = 40i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvSendPage: i32 = 32i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvSetPalette: i32 = 22i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvSetPixelFormat: i32 = 54i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvSetPointerShape: i32 = 29i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvStartBanding: i32 = 57i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvStartDoc: i32 = 35i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvStartDxInterop: i32 = 99i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvStartPage: i32 = 33i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvStretchBlt: i32 = 20i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvStretchBltROP: i32 = 69i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvStrokeAndFillPath: i32 = 16i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvStrokePath: i32 = 14i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvSurfaceComplete: i32 = 103i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvSwapBuffers: i32 = 56i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvSynchronize: i32 = 38i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvSynchronizeRedirectionBitmaps: i32 = 97i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvSynchronizeSurface: i32 = 88i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvSynthesizeFont: i32 = 72i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvTextOut: i32 = 23i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvTransparentBlt: i32 = 74i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvUnloadFontFile: i32 = 46i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_DrvUnlockDisplayArea: i32 = 102i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const INDEX_LAST: i32 = 89i32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
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
impl ::core::default::Default for INDIRECT_DISPLAY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for INDIRECT_DISPLAY_INFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("INDIRECT_DISPLAY_INFO").field("DisplayAdapterLuid", &self.DisplayAdapterLuid).field("Flags", &self.Flags).field("NumMonitors", &self.NumMonitors).field("DisplayAdapterTargetBase", &self.DisplayAdapterTargetBase).finish()
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
unsafe impl ::windows::core::Abi for INDIRECT_DISPLAY_INFO {
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
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IViewHelper(pub ::windows::core::IUnknown);
impl IViewHelper {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    pub unsafe fn GetConnectedIDs<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wszadaptorname: Param0, pulcount: *mut u32, pulid: *mut u32, ulflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), wszadaptorname.into_param().abi(), ::core::mem::transmute(pulcount), ::core::mem::transmute(pulid), ::core::mem::transmute(ulflags)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    pub unsafe fn GetActiveTopology<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wszadaptorname: Param0, ulsourceid: u32, pulcount: *mut u32, pultargetid: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), wszadaptorname.into_param().abi(), ::core::mem::transmute(ulsourceid), ::core::mem::transmute(pulcount), ::core::mem::transmute(pultargetid)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
    pub unsafe fn SetActiveTopology<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, wszadaptorname: Param0, ulsourceid: u32, ulcount: u32, pultargetid: *const u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), wszadaptorname.into_param().abi(), ::core::mem::transmute(ulsourceid), ::core::mem::transmute(ulcount), ::core::mem::transmute(pultargetid)).ok()
    }
    #[doc = "*Required features: `Win32_Devices_Display`*"]
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Devices_Display`, `Win32_System_Com`*"]
    pub unsafe fn SetConfiguration<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IStream>>(&self, pistream: Param0) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pistream.into_param().abi(), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_Display`*"]
    pub unsafe fn GetProceedOnNewConfiguration(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for IViewHelper {
    type Vtable = IViewHelper_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe85ccef5_aaaa_47f0_b5e3_61f7aecdc4c1);
}
impl ::core::convert::From<IViewHelper> for ::windows::core::IUnknown {
    fn from(value: IViewHelper) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IViewHelper> for ::windows::core::IUnknown {
    fn from(value: &IViewHelper) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IViewHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IViewHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IViewHelper_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, wszadaptorname: super::super::Foundation::PWSTR, pulcount: *mut u32, pulid: *mut u32, ulflags: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, wszadaptorname: super::super::Foundation::PWSTR, ulsourceid: u32, pulcount: *mut u32, pultargetid: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, wszadaptorname: super::super::Foundation::PWSTR, ulsourceid: u32, ulcount: u32, pultargetid: *const u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pistream: ::windows::core::RawPtr, pulstatus: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const JOIN_BEVEL: i32 = 1i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const JOIN_MITER: i32 = 2i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const JOIN_ROUND: i32 = 0i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const LA_ALTERNATE: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const LA_GEOMETRIC: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const LA_STARTGAP: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const LA_STYLED: u32 = 8u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
pub struct LIGATURE {
    pub culSize: u32,
    pub pwsz: super::super::Foundation::PWSTR,
    pub chglyph: u32,
    pub ahglyph: [u32; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl LIGATURE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for LIGATURE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for LIGATURE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("LIGATURE").field("culSize", &self.culSize).field("pwsz", &self.pwsz).field("chglyph", &self.chglyph).field("ahglyph", &self.ahglyph).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for LIGATURE {
    fn eq(&self, other: &Self) -> bool {
        self.culSize == other.culSize && self.pwsz == other.pwsz && self.chglyph == other.chglyph && self.ahglyph == other.ahglyph
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for LIGATURE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for LIGATURE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct LINEATTRS {
    pub fl: u32,
    pub iJoin: u32,
    pub iEndCap: u32,
    pub elWidth: FLOAT_LONG,
    pub eMiterLimit: f32,
    pub cstyle: u32,
    pub pstyle: *mut FLOAT_LONG,
    pub elStyleState: FLOAT_LONG,
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl LINEATTRS {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::default::Default for LINEATTRS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::cmp::PartialEq for LINEATTRS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::cmp::Eq for LINEATTRS {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
unsafe impl ::windows::core::Abi for LINEATTRS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(any(target_arch = "x86",))]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct LINEATTRS {
    pub fl: u32,
    pub iJoin: u32,
    pub iEndCap: u32,
    pub elWidth: FLOAT_LONG,
    pub eMiterLimit: u32,
    pub cstyle: u32,
    pub pstyle: *mut FLOAT_LONG,
    pub elStyleState: FLOAT_LONG,
}
#[cfg(any(target_arch = "x86",))]
impl LINEATTRS {}
#[cfg(any(target_arch = "x86",))]
impl ::core::default::Default for LINEATTRS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "x86",))]
impl ::core::cmp::PartialEq for LINEATTRS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(any(target_arch = "x86",))]
impl ::core::cmp::Eq for LINEATTRS {}
#[cfg(any(target_arch = "x86",))]
unsafe impl ::windows::core::Abi for LINEATTRS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const MAXCHARSETS: u32 = 16u32;
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
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
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
impl ::core::convert::From<i32> for MC_COLOR_TEMPERATURE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MC_COLOR_TEMPERATURE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
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
impl ::core::convert::From<i32> for MC_DISPLAY_TECHNOLOGY_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MC_DISPLAY_TECHNOLOGY_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MC_DRIVE_TYPE(pub i32);
pub const MC_RED_DRIVE: MC_DRIVE_TYPE = MC_DRIVE_TYPE(0i32);
pub const MC_GREEN_DRIVE: MC_DRIVE_TYPE = MC_DRIVE_TYPE(1i32);
pub const MC_BLUE_DRIVE: MC_DRIVE_TYPE = MC_DRIVE_TYPE(2i32);
impl ::core::convert::From<i32> for MC_DRIVE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MC_DRIVE_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MC_GAIN_TYPE(pub i32);
pub const MC_RED_GAIN: MC_GAIN_TYPE = MC_GAIN_TYPE(0i32);
pub const MC_GREEN_GAIN: MC_GAIN_TYPE = MC_GAIN_TYPE(1i32);
pub const MC_BLUE_GAIN: MC_GAIN_TYPE = MC_GAIN_TYPE(2i32);
impl ::core::convert::From<i32> for MC_GAIN_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MC_GAIN_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MC_POSITION_TYPE(pub i32);
pub const MC_HORIZONTAL_POSITION: MC_POSITION_TYPE = MC_POSITION_TYPE(0i32);
pub const MC_VERTICAL_POSITION: MC_POSITION_TYPE = MC_POSITION_TYPE(1i32);
impl ::core::convert::From<i32> for MC_POSITION_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MC_POSITION_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const MC_RESTORE_FACTORY_DEFAULTS_ENABLES_MONITOR_SETTINGS: u32 = 4096u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MC_SIZE_TYPE(pub i32);
pub const MC_WIDTH: MC_SIZE_TYPE = MC_SIZE_TYPE(0i32);
pub const MC_HEIGHT: MC_SIZE_TYPE = MC_SIZE_TYPE(1i32);
impl ::core::convert::From<i32> for MC_SIZE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MC_SIZE_TYPE {
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
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct MC_TIMING_REPORT {
    pub dwHorizontalFrequencyInHZ: u32,
    pub dwVerticalFrequencyInHZ: u32,
    pub bTimingStatusByte: u8,
}
impl MC_TIMING_REPORT {}
impl ::core::default::Default for MC_TIMING_REPORT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MC_TIMING_REPORT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for MC_TIMING_REPORT {}
unsafe impl ::windows::core::Abi for MC_TIMING_REPORT {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MC_VCP_CODE_TYPE(pub i32);
pub const MC_MOMENTARY: MC_VCP_CODE_TYPE = MC_VCP_CODE_TYPE(0i32);
pub const MC_SET_PARAMETER: MC_VCP_CODE_TYPE = MC_VCP_CODE_TYPE(1i32);
impl ::core::convert::From<i32> for MC_VCP_CODE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MC_VCP_CODE_TYPE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
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
impl ::core::default::Default for MIPI_DSI_CAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for MIPI_DSI_CAPS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
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
unsafe impl ::windows::core::Abi for MIPI_DSI_CAPS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct MIPI_DSI_PACKET {
    pub Anonymous1: MIPI_DSI_PACKET_0,
    pub Anonymous2: MIPI_DSI_PACKET_1,
    pub EccFiller: u8,
    pub Payload: [u8; 8],
}
impl MIPI_DSI_PACKET {}
impl ::core::default::Default for MIPI_DSI_PACKET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIPI_DSI_PACKET {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for MIPI_DSI_PACKET {}
unsafe impl ::windows::core::Abi for MIPI_DSI_PACKET {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub union MIPI_DSI_PACKET_0 {
    pub DataId: u8,
    pub Anonymous: MIPI_DSI_PACKET_0_0,
}
impl MIPI_DSI_PACKET_0 {}
impl ::core::default::Default for MIPI_DSI_PACKET_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIPI_DSI_PACKET_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for MIPI_DSI_PACKET_0 {}
unsafe impl ::windows::core::Abi for MIPI_DSI_PACKET_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct MIPI_DSI_PACKET_0_0 {
    pub _bitfield: u8,
}
impl MIPI_DSI_PACKET_0_0 {}
impl ::core::default::Default for MIPI_DSI_PACKET_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for MIPI_DSI_PACKET_0_0 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::core::cmp::PartialEq for MIPI_DSI_PACKET_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for MIPI_DSI_PACKET_0_0 {}
unsafe impl ::windows::core::Abi for MIPI_DSI_PACKET_0_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub union MIPI_DSI_PACKET_1 {
    pub Anonymous: MIPI_DSI_PACKET_1_0,
    pub LongWriteWordCount: u16,
}
impl MIPI_DSI_PACKET_1 {}
impl ::core::default::Default for MIPI_DSI_PACKET_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIPI_DSI_PACKET_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for MIPI_DSI_PACKET_1 {}
unsafe impl ::windows::core::Abi for MIPI_DSI_PACKET_1 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct MIPI_DSI_PACKET_1_0 {
    pub Data0: u8,
    pub Data1: u8,
}
impl MIPI_DSI_PACKET_1_0 {}
impl ::core::default::Default for MIPI_DSI_PACKET_1_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for MIPI_DSI_PACKET_1_0 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("Data0", &self.Data0).field("Data1", &self.Data1).finish()
    }
}
impl ::core::cmp::PartialEq for MIPI_DSI_PACKET_1_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Data0 == other.Data0 && self.Data1 == other.Data1
    }
}
impl ::core::cmp::Eq for MIPI_DSI_PACKET_1_0 {}
unsafe impl ::windows::core::Abi for MIPI_DSI_PACKET_1_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct MIPI_DSI_RESET {
    pub Flags: u32,
    pub Anonymous: MIPI_DSI_RESET_0,
}
impl MIPI_DSI_RESET {}
impl ::core::default::Default for MIPI_DSI_RESET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIPI_DSI_RESET {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for MIPI_DSI_RESET {}
unsafe impl ::windows::core::Abi for MIPI_DSI_RESET {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub union MIPI_DSI_RESET_0 {
    pub Anonymous: MIPI_DSI_RESET_0_0,
    pub Results: u32,
}
impl MIPI_DSI_RESET_0 {}
impl ::core::default::Default for MIPI_DSI_RESET_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIPI_DSI_RESET_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for MIPI_DSI_RESET_0 {}
unsafe impl ::windows::core::Abi for MIPI_DSI_RESET_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct MIPI_DSI_RESET_0_0 {
    pub _bitfield: u32,
}
impl MIPI_DSI_RESET_0_0 {}
impl ::core::default::Default for MIPI_DSI_RESET_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for MIPI_DSI_RESET_0_0 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::core::cmp::PartialEq for MIPI_DSI_RESET_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for MIPI_DSI_RESET_0_0 {}
unsafe impl ::windows::core::Abi for MIPI_DSI_RESET_0_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
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
impl ::core::default::Default for MIPI_DSI_TRANSMISSION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIPI_DSI_TRANSMISSION {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for MIPI_DSI_TRANSMISSION {}
unsafe impl ::windows::core::Abi for MIPI_DSI_TRANSMISSION {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct MIPI_DSI_TRANSMISSION_0 {
    pub _bitfield: u16,
}
impl MIPI_DSI_TRANSMISSION_0 {}
impl ::core::default::Default for MIPI_DSI_TRANSMISSION_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for MIPI_DSI_TRANSMISSION_0 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::core::cmp::PartialEq for MIPI_DSI_TRANSMISSION_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for MIPI_DSI_TRANSMISSION_0 {}
unsafe impl ::windows::core::Abi for MIPI_DSI_TRANSMISSION_0 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const MS_CDDDEVICEBITMAP: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const MS_NOTSYSTEMMEMORY: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const MS_REUSEDDEVICEBITMAP: u32 = 8u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const MS_SHAREDACCESS: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const OC_BANK_CLIP: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const OPENGL_CMD: u32 = 4352u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const OPENGL_GETINFO: u32 = 4353u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ORIENTATION_PREFERENCE(pub i32);
pub const ORIENTATION_PREFERENCE_NONE: ORIENTATION_PREFERENCE = ORIENTATION_PREFERENCE(0i32);
pub const ORIENTATION_PREFERENCE_LANDSCAPE: ORIENTATION_PREFERENCE = ORIENTATION_PREFERENCE(1i32);
pub const ORIENTATION_PREFERENCE_PORTRAIT: ORIENTATION_PREFERENCE = ORIENTATION_PREFERENCE(2i32);
pub const ORIENTATION_PREFERENCE_LANDSCAPE_FLIPPED: ORIENTATION_PREFERENCE = ORIENTATION_PREFERENCE(4i32);
pub const ORIENTATION_PREFERENCE_PORTRAIT_FLIPPED: ORIENTATION_PREFERENCE = ORIENTATION_PREFERENCE(8i32);
impl ::core::convert::From<i32> for ORIENTATION_PREFERENCE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ORIENTATION_PREFERENCE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct OUTPUT_COLOR_ENCODING(pub i32);
pub const OUTPUT_COLOR_ENCODING_RGB: OUTPUT_COLOR_ENCODING = OUTPUT_COLOR_ENCODING(0i32);
pub const OUTPUT_COLOR_ENCODING_YCBCR444: OUTPUT_COLOR_ENCODING = OUTPUT_COLOR_ENCODING(1i32);
pub const OUTPUT_COLOR_ENCODING_YCBCR422: OUTPUT_COLOR_ENCODING = OUTPUT_COLOR_ENCODING(2i32);
pub const OUTPUT_COLOR_ENCODING_YCBCR420: OUTPUT_COLOR_ENCODING = OUTPUT_COLOR_ENCODING(3i32);
pub const OUTPUT_COLOR_ENCODING_INTENSITY: OUTPUT_COLOR_ENCODING = OUTPUT_COLOR_ENCODING(4i32);
pub const OUTPUT_COLOR_ENCODING_FORCE_UINT32: OUTPUT_COLOR_ENCODING = OUTPUT_COLOR_ENCODING(-1i32);
impl ::core::convert::From<i32> for OUTPUT_COLOR_ENCODING {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for OUTPUT_COLOR_ENCODING {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct OUTPUT_WIRE_COLOR_SPACE_TYPE(pub i32);
pub const OUTPUT_WIRE_COLOR_SPACE_G22_P709: OUTPUT_WIRE_COLOR_SPACE_TYPE = OUTPUT_WIRE_COLOR_SPACE_TYPE(0i32);
pub const OUTPUT_WIRE_COLOR_SPACE_RESERVED: OUTPUT_WIRE_COLOR_SPACE_TYPE = OUTPUT_WIRE_COLOR_SPACE_TYPE(4i32);
pub const OUTPUT_WIRE_COLOR_SPACE_G2084_P2020: OUTPUT_WIRE_COLOR_SPACE_TYPE = OUTPUT_WIRE_COLOR_SPACE_TYPE(12i32);
pub const OUTPUT_WIRE_COLOR_SPACE_G22_P709_WCG: OUTPUT_WIRE_COLOR_SPACE_TYPE = OUTPUT_WIRE_COLOR_SPACE_TYPE(30i32);
pub const OUTPUT_WIRE_COLOR_SPACE_G22_P2020: OUTPUT_WIRE_COLOR_SPACE_TYPE = OUTPUT_WIRE_COLOR_SPACE_TYPE(31i32);
pub const OUTPUT_WIRE_COLOR_SPACE_G2084_P2020_HDR10PLUS: OUTPUT_WIRE_COLOR_SPACE_TYPE = OUTPUT_WIRE_COLOR_SPACE_TYPE(32i32);
pub const OUTPUT_WIRE_COLOR_SPACE_G2084_P2020_DVLL: OUTPUT_WIRE_COLOR_SPACE_TYPE = OUTPUT_WIRE_COLOR_SPACE_TYPE(33i32);
impl ::core::convert::From<i32> for OUTPUT_WIRE_COLOR_SPACE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for OUTPUT_WIRE_COLOR_SPACE_TYPE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct OUTPUT_WIRE_FORMAT {
    pub ColorEncoding: OUTPUT_COLOR_ENCODING,
    pub BitsPerPixel: u32,
}
impl OUTPUT_WIRE_FORMAT {}
impl ::core::default::Default for OUTPUT_WIRE_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for OUTPUT_WIRE_FORMAT {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("OUTPUT_WIRE_FORMAT").field("ColorEncoding", &self.ColorEncoding).field("BitsPerPixel", &self.BitsPerPixel).finish()
    }
}
impl ::core::cmp::PartialEq for OUTPUT_WIRE_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.ColorEncoding == other.ColorEncoding && self.BitsPerPixel == other.BitsPerPixel
    }
}
impl ::core::cmp::Eq for OUTPUT_WIRE_FORMAT {}
unsafe impl ::windows::core::Abi for OUTPUT_WIRE_FORMAT {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct PALOBJ {
    pub ulReserved: u32,
}
impl PALOBJ {}
impl ::core::default::Default for PALOBJ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for PALOBJ {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("PALOBJ").field("ulReserved", &self.ulReserved).finish()
    }
}
impl ::core::cmp::PartialEq for PALOBJ {
    fn eq(&self, other: &Self) -> bool {
        self.ulReserved == other.ulReserved
    }
}
impl ::core::cmp::Eq for PALOBJ {}
unsafe impl ::windows::core::Abi for PALOBJ {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const PAL_BGR: u32 = 8u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const PAL_BITFIELDS: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const PAL_CMYK: u32 = 16u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const PAL_INDEXED: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const PAL_RGB: u32 = 4u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct PANEL_BRIGHTNESS_SENSOR_DATA {
    pub Anonymous: PANEL_BRIGHTNESS_SENSOR_DATA_0,
    pub AlsReading: f32,
    pub ChromaticityCoordinate: CHROMATICITY_COORDINATE,
    pub ColorTemperature: f32,
}
impl PANEL_BRIGHTNESS_SENSOR_DATA {}
impl ::core::default::Default for PANEL_BRIGHTNESS_SENSOR_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PANEL_BRIGHTNESS_SENSOR_DATA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for PANEL_BRIGHTNESS_SENSOR_DATA {}
unsafe impl ::windows::core::Abi for PANEL_BRIGHTNESS_SENSOR_DATA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub union PANEL_BRIGHTNESS_SENSOR_DATA_0 {
    pub Anonymous: PANEL_BRIGHTNESS_SENSOR_DATA_0_0,
    pub Value: u32,
}
impl PANEL_BRIGHTNESS_SENSOR_DATA_0 {}
impl ::core::default::Default for PANEL_BRIGHTNESS_SENSOR_DATA_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PANEL_BRIGHTNESS_SENSOR_DATA_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for PANEL_BRIGHTNESS_SENSOR_DATA_0 {}
unsafe impl ::windows::core::Abi for PANEL_BRIGHTNESS_SENSOR_DATA_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct PANEL_BRIGHTNESS_SENSOR_DATA_0_0 {
    pub _bitfield: u32,
}
impl PANEL_BRIGHTNESS_SENSOR_DATA_0_0 {}
impl ::core::default::Default for PANEL_BRIGHTNESS_SENSOR_DATA_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for PANEL_BRIGHTNESS_SENSOR_DATA_0_0 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::core::cmp::PartialEq for PANEL_BRIGHTNESS_SENSOR_DATA_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for PANEL_BRIGHTNESS_SENSOR_DATA_0_0 {}
unsafe impl ::windows::core::Abi for PANEL_BRIGHTNESS_SENSOR_DATA_0_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct PANEL_GET_BACKLIGHT_REDUCTION {
    pub BacklightUsersetting: u16,
    pub BacklightEffective: u16,
    pub GammaRamp: BACKLIGHT_REDUCTION_GAMMA_RAMP,
}
impl PANEL_GET_BACKLIGHT_REDUCTION {}
impl ::core::default::Default for PANEL_GET_BACKLIGHT_REDUCTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for PANEL_GET_BACKLIGHT_REDUCTION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("PANEL_GET_BACKLIGHT_REDUCTION").field("BacklightUsersetting", &self.BacklightUsersetting).field("BacklightEffective", &self.BacklightEffective).field("GammaRamp", &self.GammaRamp).finish()
    }
}
impl ::core::cmp::PartialEq for PANEL_GET_BACKLIGHT_REDUCTION {
    fn eq(&self, other: &Self) -> bool {
        self.BacklightUsersetting == other.BacklightUsersetting && self.BacklightEffective == other.BacklightEffective && self.GammaRamp == other.GammaRamp
    }
}
impl ::core::cmp::Eq for PANEL_GET_BACKLIGHT_REDUCTION {}
unsafe impl ::windows::core::Abi for PANEL_GET_BACKLIGHT_REDUCTION {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct PANEL_GET_BRIGHTNESS {
    pub Version: BRIGHTNESS_INTERFACE_VERSION,
    pub Anonymous: PANEL_GET_BRIGHTNESS_0,
}
impl PANEL_GET_BRIGHTNESS {}
impl ::core::default::Default for PANEL_GET_BRIGHTNESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PANEL_GET_BRIGHTNESS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for PANEL_GET_BRIGHTNESS {}
unsafe impl ::windows::core::Abi for PANEL_GET_BRIGHTNESS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub union PANEL_GET_BRIGHTNESS_0 {
    pub Level: u8,
    pub Anonymous: PANEL_GET_BRIGHTNESS_0_0,
}
impl PANEL_GET_BRIGHTNESS_0 {}
impl ::core::default::Default for PANEL_GET_BRIGHTNESS_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PANEL_GET_BRIGHTNESS_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for PANEL_GET_BRIGHTNESS_0 {}
unsafe impl ::windows::core::Abi for PANEL_GET_BRIGHTNESS_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct PANEL_GET_BRIGHTNESS_0_0 {
    pub CurrentInMillinits: u32,
    pub TargetInMillinits: u32,
}
impl PANEL_GET_BRIGHTNESS_0_0 {}
impl ::core::default::Default for PANEL_GET_BRIGHTNESS_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for PANEL_GET_BRIGHTNESS_0_0 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("CurrentInMillinits", &self.CurrentInMillinits).field("TargetInMillinits", &self.TargetInMillinits).finish()
    }
}
impl ::core::cmp::PartialEq for PANEL_GET_BRIGHTNESS_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.CurrentInMillinits == other.CurrentInMillinits && self.TargetInMillinits == other.TargetInMillinits
    }
}
impl ::core::cmp::Eq for PANEL_GET_BRIGHTNESS_0_0 {}
unsafe impl ::windows::core::Abi for PANEL_GET_BRIGHTNESS_0_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct PANEL_QUERY_BRIGHTNESS_CAPS {
    pub Version: BRIGHTNESS_INTERFACE_VERSION,
    pub Anonymous: PANEL_QUERY_BRIGHTNESS_CAPS_0,
}
impl PANEL_QUERY_BRIGHTNESS_CAPS {}
impl ::core::default::Default for PANEL_QUERY_BRIGHTNESS_CAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PANEL_QUERY_BRIGHTNESS_CAPS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for PANEL_QUERY_BRIGHTNESS_CAPS {}
unsafe impl ::windows::core::Abi for PANEL_QUERY_BRIGHTNESS_CAPS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub union PANEL_QUERY_BRIGHTNESS_CAPS_0 {
    pub Anonymous: PANEL_QUERY_BRIGHTNESS_CAPS_0_0,
    pub Value: u32,
}
impl PANEL_QUERY_BRIGHTNESS_CAPS_0 {}
impl ::core::default::Default for PANEL_QUERY_BRIGHTNESS_CAPS_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PANEL_QUERY_BRIGHTNESS_CAPS_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for PANEL_QUERY_BRIGHTNESS_CAPS_0 {}
unsafe impl ::windows::core::Abi for PANEL_QUERY_BRIGHTNESS_CAPS_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct PANEL_QUERY_BRIGHTNESS_CAPS_0_0 {
    pub _bitfield: u32,
}
impl PANEL_QUERY_BRIGHTNESS_CAPS_0_0 {}
impl ::core::default::Default for PANEL_QUERY_BRIGHTNESS_CAPS_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for PANEL_QUERY_BRIGHTNESS_CAPS_0_0 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::core::cmp::PartialEq for PANEL_QUERY_BRIGHTNESS_CAPS_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for PANEL_QUERY_BRIGHTNESS_CAPS_0_0 {}
unsafe impl ::windows::core::Abi for PANEL_QUERY_BRIGHTNESS_CAPS_0_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct PANEL_QUERY_BRIGHTNESS_RANGES {
    pub Version: BRIGHTNESS_INTERFACE_VERSION,
    pub Anonymous: PANEL_QUERY_BRIGHTNESS_RANGES_0,
}
impl PANEL_QUERY_BRIGHTNESS_RANGES {}
impl ::core::default::Default for PANEL_QUERY_BRIGHTNESS_RANGES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PANEL_QUERY_BRIGHTNESS_RANGES {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for PANEL_QUERY_BRIGHTNESS_RANGES {}
unsafe impl ::windows::core::Abi for PANEL_QUERY_BRIGHTNESS_RANGES {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub union PANEL_QUERY_BRIGHTNESS_RANGES_0 {
    pub BrightnessLevel: BRIGHTNESS_LEVEL,
    pub NitRanges: BRIGHTNESS_NIT_RANGES,
}
impl PANEL_QUERY_BRIGHTNESS_RANGES_0 {}
impl ::core::default::Default for PANEL_QUERY_BRIGHTNESS_RANGES_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PANEL_QUERY_BRIGHTNESS_RANGES_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for PANEL_QUERY_BRIGHTNESS_RANGES_0 {}
unsafe impl ::windows::core::Abi for PANEL_QUERY_BRIGHTNESS_RANGES_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct PANEL_SET_BACKLIGHT_OPTIMIZATION {
    pub Level: BACKLIGHT_OPTIMIZATION_LEVEL,
}
impl PANEL_SET_BACKLIGHT_OPTIMIZATION {}
impl ::core::default::Default for PANEL_SET_BACKLIGHT_OPTIMIZATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for PANEL_SET_BACKLIGHT_OPTIMIZATION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("PANEL_SET_BACKLIGHT_OPTIMIZATION").field("Level", &self.Level).finish()
    }
}
impl ::core::cmp::PartialEq for PANEL_SET_BACKLIGHT_OPTIMIZATION {
    fn eq(&self, other: &Self) -> bool {
        self.Level == other.Level
    }
}
impl ::core::cmp::Eq for PANEL_SET_BACKLIGHT_OPTIMIZATION {}
unsafe impl ::windows::core::Abi for PANEL_SET_BACKLIGHT_OPTIMIZATION {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct PANEL_SET_BRIGHTNESS {
    pub Version: BRIGHTNESS_INTERFACE_VERSION,
    pub Anonymous: PANEL_SET_BRIGHTNESS_0,
}
impl PANEL_SET_BRIGHTNESS {}
impl ::core::default::Default for PANEL_SET_BRIGHTNESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PANEL_SET_BRIGHTNESS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for PANEL_SET_BRIGHTNESS {}
unsafe impl ::windows::core::Abi for PANEL_SET_BRIGHTNESS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub union PANEL_SET_BRIGHTNESS_0 {
    pub Level: u8,
    pub Anonymous: PANEL_SET_BRIGHTNESS_0_0,
}
impl PANEL_SET_BRIGHTNESS_0 {}
impl ::core::default::Default for PANEL_SET_BRIGHTNESS_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PANEL_SET_BRIGHTNESS_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for PANEL_SET_BRIGHTNESS_0 {}
unsafe impl ::windows::core::Abi for PANEL_SET_BRIGHTNESS_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct PANEL_SET_BRIGHTNESS_0_0 {
    pub Millinits: u32,
    pub TransitionTimeInMs: u32,
    pub SensorData: PANEL_BRIGHTNESS_SENSOR_DATA,
}
impl PANEL_SET_BRIGHTNESS_0_0 {}
impl ::core::default::Default for PANEL_SET_BRIGHTNESS_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PANEL_SET_BRIGHTNESS_0_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for PANEL_SET_BRIGHTNESS_0_0 {}
unsafe impl ::windows::core::Abi for PANEL_SET_BRIGHTNESS_0_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct PANEL_SET_BRIGHTNESS_STATE {
    pub Anonymous: PANEL_SET_BRIGHTNESS_STATE_0,
}
impl PANEL_SET_BRIGHTNESS_STATE {}
impl ::core::default::Default for PANEL_SET_BRIGHTNESS_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PANEL_SET_BRIGHTNESS_STATE {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for PANEL_SET_BRIGHTNESS_STATE {}
unsafe impl ::windows::core::Abi for PANEL_SET_BRIGHTNESS_STATE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub union PANEL_SET_BRIGHTNESS_STATE_0 {
    pub Anonymous: PANEL_SET_BRIGHTNESS_STATE_0_0,
    pub Value: u32,
}
impl PANEL_SET_BRIGHTNESS_STATE_0 {}
impl ::core::default::Default for PANEL_SET_BRIGHTNESS_STATE_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PANEL_SET_BRIGHTNESS_STATE_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for PANEL_SET_BRIGHTNESS_STATE_0 {}
unsafe impl ::windows::core::Abi for PANEL_SET_BRIGHTNESS_STATE_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct PANEL_SET_BRIGHTNESS_STATE_0_0 {
    pub _bitfield: u32,
}
impl PANEL_SET_BRIGHTNESS_STATE_0_0 {}
impl ::core::default::Default for PANEL_SET_BRIGHTNESS_STATE_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for PANEL_SET_BRIGHTNESS_STATE_0_0 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::core::cmp::PartialEq for PANEL_SET_BRIGHTNESS_STATE_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for PANEL_SET_BRIGHTNESS_STATE_0_0 {}
unsafe impl ::windows::core::Abi for PANEL_SET_BRIGHTNESS_STATE_0_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct PATHDATA {
    pub flags: u32,
    pub count: u32,
    pub pptfx: *mut POINTFIX,
}
impl PATHDATA {}
impl ::core::default::Default for PATHDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for PATHDATA {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("PATHDATA").field("flags", &self.flags).field("count", &self.count).field("pptfx", &self.pptfx).finish()
    }
}
impl ::core::cmp::PartialEq for PATHDATA {
    fn eq(&self, other: &Self) -> bool {
        self.flags == other.flags && self.count == other.count && self.pptfx == other.pptfx
    }
}
impl ::core::cmp::Eq for PATHDATA {}
unsafe impl ::windows::core::Abi for PATHDATA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct PATHOBJ {
    pub fl: u32,
    pub cCurves: u32,
}
impl PATHOBJ {}
impl ::core::default::Default for PATHOBJ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for PATHOBJ {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("PATHOBJ").field("fl", &self.fl).field("cCurves", &self.cCurves).finish()
    }
}
impl ::core::cmp::PartialEq for PATHOBJ {
    fn eq(&self, other: &Self) -> bool {
        self.fl == other.fl && self.cCurves == other.cCurves
    }
}
impl ::core::cmp::Eq for PATHOBJ {}
unsafe impl ::windows::core::Abi for PATHOBJ {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PATHOBJ_bEnum(ppo: *mut PATHOBJ, ppd: *mut PATHDATA) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PATHOBJ_bEnum(ppo: *mut PATHOBJ, ppd: *mut PATHDATA) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(PATHOBJ_bEnum(::core::mem::transmute(ppo), ::core::mem::transmute(ppd)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PATHOBJ_bEnumClipLines(ppo: *mut PATHOBJ, cb: u32, pcl: *mut CLIPLINE) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PATHOBJ_bEnumClipLines(ppo: *mut PATHOBJ, cb: u32, pcl: *mut CLIPLINE) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(PATHOBJ_bEnumClipLines(::core::mem::transmute(ppo), ::core::mem::transmute(cb), ::core::mem::transmute(pcl)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
#[inline]
pub unsafe fn PATHOBJ_vEnumStart(ppo: *mut PATHOBJ) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PATHOBJ_vEnumStart(ppo: *mut PATHOBJ);
        }
        ::core::mem::transmute(PATHOBJ_vEnumStart(::core::mem::transmute(ppo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PATHOBJ_vEnumStartClipLines(ppo: *mut PATHOBJ, pco: *mut CLIPOBJ, pso: *mut SURFOBJ, pla: *mut LINEATTRS) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PATHOBJ_vEnumStartClipLines(ppo: *mut PATHOBJ, pco: *mut CLIPOBJ, pso: *mut SURFOBJ, pla: *mut LINEATTRS);
        }
        ::core::mem::transmute(PATHOBJ_vEnumStartClipLines(::core::mem::transmute(ppo), ::core::mem::transmute(pco), ::core::mem::transmute(pso), ::core::mem::transmute(pla)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
#[inline]
pub unsafe fn PATHOBJ_vGetBounds(ppo: *mut PATHOBJ, prectfx: *mut RECTFX) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PATHOBJ_vGetBounds(ppo: *mut PATHOBJ, prectfx: *mut RECTFX);
        }
        ::core::mem::transmute(PATHOBJ_vGetBounds(::core::mem::transmute(ppo), ::core::mem::transmute(prectfx)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const PD_BEGINSUBPATH: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const PD_BEZIERS: u32 = 16u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const PD_CLOSEFIGURE: u32 = 8u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const PD_ENDSUBPATH: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const PD_RESETSTYLE: u32 = 4u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
pub struct PERBANDINFO {
    pub bRepeatThisBand: super::super::Foundation::BOOL,
    pub szlBand: super::super::Foundation::SIZE,
    pub ulHorzRes: u32,
    pub ulVertRes: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl PERBANDINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PERBANDINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PERBANDINFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("PERBANDINFO").field("bRepeatThisBand", &self.bRepeatThisBand).field("szlBand", &self.szlBand).field("ulHorzRes", &self.ulHorzRes).field("ulVertRes", &self.ulVertRes).finish()
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
unsafe impl ::windows::core::Abi for PERBANDINFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub type PFN = unsafe extern "system" fn() -> isize;
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvAccumulateD3DDirtyRect = unsafe extern "system" fn(param0: *mut SURFOBJ, param1: *mut CDDDXGK_REDIRBITMAPPRESENTINFO) -> super::super::Foundation::BOOL;
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type PFN_DrvAlphaBlend = unsafe extern "system" fn(param0: *mut SURFOBJ, param1: *mut SURFOBJ, param2: *mut CLIPOBJ, param3: *mut XLATEOBJ, param4: *mut super::super::Foundation::RECTL, param5: *mut super::super::Foundation::RECTL, param6: *mut BLENDOBJ) -> super::super::Foundation::BOOL;
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvAssertMode = unsafe extern "system" fn(param0: DHPDEV, param1: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvAssociateSharedSurface = unsafe extern "system" fn(param0: *mut SURFOBJ, param1: super::super::Foundation::HANDLE, param2: super::super::Foundation::HANDLE, param3: super::super::Foundation::SIZE) -> super::super::Foundation::BOOL;
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvBitBlt = unsafe extern "system" fn(param0: *mut SURFOBJ, param1: *mut SURFOBJ, param2: *mut SURFOBJ, param3: *mut CLIPOBJ, param4: *mut XLATEOBJ, param5: *mut super::super::Foundation::RECTL, param6: *mut super::super::Foundation::POINTL, param7: *mut super::super::Foundation::POINTL, param8: *mut BRUSHOBJ, param9: *mut super::super::Foundation::POINTL, param10: u32) -> super::super::Foundation::BOOL;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub type PFN_DrvCompletePDEV = unsafe extern "system" fn(param0: DHPDEV, param1: HDEV);
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvCopyBits = unsafe extern "system" fn(param0: *mut SURFOBJ, param1: *mut SURFOBJ, param2: *mut CLIPOBJ, param3: *mut XLATEOBJ, param4: *mut super::super::Foundation::RECTL, param5: *mut super::super::Foundation::POINTL) -> super::super::Foundation::BOOL;
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type PFN_DrvCreateDeviceBitmap = unsafe extern "system" fn(param0: DHPDEV, param1: super::super::Foundation::SIZE, param2: u32) -> super::super::Graphics::Gdi::HBITMAP;
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type PFN_DrvCreateDeviceBitmapEx = unsafe extern "system" fn(param0: DHPDEV, param1: super::super::Foundation::SIZE, param2: u32, param3: u32, param4: DHSURF, param5: u32, param6: u32, param7: *mut super::super::Foundation::HANDLE) -> super::super::Graphics::Gdi::HBITMAP;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub type PFN_DrvDeleteDeviceBitmap = unsafe extern "system" fn(param0: DHSURF);
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub type PFN_DrvDeleteDeviceBitmapEx = unsafe extern "system" fn(param0: DHSURF);
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_DirectDraw`, `Win32_Graphics_Gdi`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Graphics_Gdi"))]
pub type PFN_DrvDeriveSurface = unsafe extern "system" fn(param0: *mut super::super::Graphics::DirectDraw::DD_DIRECTDRAW_GLOBAL, param1: *mut super::super::Graphics::DirectDraw::DD_SURFACE_LOCAL) -> super::super::Graphics::Gdi::HBITMAP;
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Graphics_OpenGL`*"]
#[cfg(feature = "Win32_Graphics_OpenGL")]
pub type PFN_DrvDescribePixelFormat = unsafe extern "system" fn(param0: DHPDEV, param1: i32, param2: u32, param3: *mut super::super::Graphics::OpenGL::PIXELFORMATDESCRIPTOR) -> i32;
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvDestroyFont = unsafe extern "system" fn(param0: *mut FONTOBJ);
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub type PFN_DrvDisableDirectDraw = unsafe extern "system" fn(param0: DHPDEV);
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub type PFN_DrvDisableDriver = unsafe extern "system" fn();
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub type PFN_DrvDisablePDEV = unsafe extern "system" fn(param0: DHPDEV);
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub type PFN_DrvDisableSurface = unsafe extern "system" fn(param0: DHPDEV);
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub type PFN_DrvDitherColor = unsafe extern "system" fn(param0: DHPDEV, param1: u32, param2: u32, param3: *mut u32) -> u32;
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvDrawEscape = unsafe extern "system" fn(param0: *mut SURFOBJ, param1: u32, param2: *mut CLIPOBJ, param3: *mut super::super::Foundation::RECTL, param4: u32, param5: *mut ::core::ffi::c_void) -> u32;
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_DirectDraw`, `Win32_Graphics_Gdi`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Graphics_Gdi"))]
pub type PFN_DrvEnableDirectDraw = unsafe extern "system" fn(param0: DHPDEV, param1: *mut ::core::mem::ManuallyDrop<super::super::Graphics::DirectDraw::DD_CALLBACKS>, param2: *mut ::core::mem::ManuallyDrop<super::super::Graphics::DirectDraw::DD_SURFACECALLBACKS>, param3: *mut ::core::mem::ManuallyDrop<super::super::Graphics::DirectDraw::DD_PALETTECALLBACKS>) -> super::super::Foundation::BOOL;
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvEnableDriver = unsafe extern "system" fn(param0: u32, param1: u32, param2: *mut DRVENABLEDATA) -> super::super::Foundation::BOOL;
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type PFN_DrvEnablePDEV = unsafe extern "system" fn(param0: *mut super::super::Graphics::Gdi::DEVMODEW, param1: super::super::Foundation::PWSTR, param2: u32, param3: *mut HSURF, param4: u32, param5: *mut GDIINFO, param6: u32, param7: *mut DEVINFO, param8: HDEV, param9: super::super::Foundation::PWSTR, param10: super::super::Foundation::HANDLE) -> DHPDEV;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub type PFN_DrvEnableSurface = unsafe extern "system" fn(param0: DHPDEV) -> HSURF;
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvEndDoc = unsafe extern "system" fn(pso: *mut SURFOBJ, fl: u32) -> super::super::Foundation::BOOL;
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvEndDxInterop = unsafe extern "system" fn(param0: *mut SURFOBJ, param1: super::super::Foundation::BOOL, param2: *mut super::super::Foundation::BOOL, kernelmodedevicehandle: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvEscape = unsafe extern "system" fn(param0: *mut SURFOBJ, param1: u32, param2: u32, param3: *mut ::core::ffi::c_void, param4: u32, param5: *mut ::core::ffi::c_void) -> u32;
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvFillPath = unsafe extern "system" fn(param0: *mut SURFOBJ, param1: *mut PATHOBJ, param2: *mut CLIPOBJ, param3: *mut BRUSHOBJ, param4: *mut super::super::Foundation::POINTL, param5: u32, param6: u32) -> super::super::Foundation::BOOL;
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvFontManagement = unsafe extern "system" fn(param0: *mut SURFOBJ, param1: *mut FONTOBJ, param2: u32, param3: u32, param4: *mut ::core::ffi::c_void, param5: u32, param6: *mut ::core::ffi::c_void) -> u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub type PFN_DrvFree = unsafe extern "system" fn(param0: *mut ::core::ffi::c_void, param1: usize);
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_DirectDraw`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
pub type PFN_DrvGetDirectDrawInfo = unsafe extern "system" fn(param0: DHPDEV, param1: *mut ::core::mem::ManuallyDrop<super::super::Graphics::DirectDraw::DD_HALINFO>, param2: *mut u32, param3: *mut super::super::Graphics::DirectDraw::VIDEOMEMORY, param4: *mut u32, param5: *mut u32) -> super::super::Foundation::BOOL;
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvGetGlyphMode = unsafe extern "system" fn(dhpdev: DHPDEV, pfo: *mut FONTOBJ) -> u32;
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type PFN_DrvGetModes = unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: u32, param2: *mut super::super::Graphics::Gdi::DEVMODEW) -> u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub type PFN_DrvGetTrueTypeFile = unsafe extern "system" fn(param0: usize, param1: *mut u32) -> *mut ::core::ffi::c_void;
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type PFN_DrvGradientFill = unsafe extern "system" fn(param0: *mut SURFOBJ, param1: *mut CLIPOBJ, param2: *mut XLATEOBJ, param3: *mut super::super::Graphics::Gdi::TRIVERTEX, param4: u32, param5: *mut ::core::ffi::c_void, param6: u32, param7: *mut super::super::Foundation::RECTL, param8: *mut super::super::Foundation::POINTL, param9: u32) -> super::super::Foundation::BOOL;
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvIcmCheckBitmapBits = unsafe extern "system" fn(param0: DHPDEV, param1: super::super::Foundation::HANDLE, param2: *mut SURFOBJ, param3: *mut u8) -> super::super::Foundation::BOOL;
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_ColorSystem`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_ColorSystem"))]
pub type PFN_DrvIcmCreateColorTransform = unsafe extern "system" fn(param0: DHPDEV, param1: *mut super::super::UI::ColorSystem::LOGCOLORSPACEW, param2: *mut ::core::ffi::c_void, param3: u32, param4: *mut ::core::ffi::c_void, param5: u32, param6: *mut ::core::ffi::c_void, param7: u32, param8: u32) -> super::super::Foundation::HANDLE;
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvIcmDeleteColorTransform = unsafe extern "system" fn(param0: DHPDEV, param1: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvIcmSetDeviceGammaRamp = unsafe extern "system" fn(param0: DHPDEV, param1: u32, param2: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvLineTo = unsafe extern "system" fn(param0: *mut SURFOBJ, param1: *mut CLIPOBJ, param2: *mut BRUSHOBJ, param3: i32, param4: i32, param5: i32, param6: i32, param7: *mut super::super::Foundation::RECTL, param8: u32) -> super::super::Foundation::BOOL;
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Graphics_Gdi`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub type PFN_DrvLoadFontFile = unsafe extern "system" fn(param0: u32, param1: *mut usize, param2: *mut *mut ::core::ffi::c_void, param3: *mut u32, param4: *mut super::super::Graphics::Gdi::DESIGNVECTOR, param5: u32, param6: u32) -> usize;
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvLockDisplayArea = unsafe extern "system" fn(param0: DHPDEV, param1: *mut super::super::Foundation::RECTL);
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvMovePointer = unsafe extern "system" fn(pso: *mut SURFOBJ, x: i32, y: i32, prcl: *mut super::super::Foundation::RECTL);
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvNextBand = unsafe extern "system" fn(param0: *mut SURFOBJ, ppointl: *mut super::super::Foundation::POINTL) -> super::super::Foundation::BOOL;
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvNotify = unsafe extern "system" fn(param0: *mut SURFOBJ, param1: u32, param2: *mut ::core::ffi::c_void);
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvPaint = unsafe extern "system" fn(param0: *mut SURFOBJ, param1: *mut CLIPOBJ, param2: *mut BRUSHOBJ, param3: *mut super::super::Foundation::POINTL, param4: u32) -> super::super::Foundation::BOOL;
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type PFN_DrvPlgBlt = unsafe extern "system" fn(param0: *mut SURFOBJ, param1: *mut SURFOBJ, param2: *mut SURFOBJ, param3: *mut CLIPOBJ, param4: *mut XLATEOBJ, param5: *mut super::super::Graphics::Gdi::COLORADJUSTMENT, param6: *mut super::super::Foundation::POINTL, param7: *mut POINTFIX, param8: *mut super::super::Foundation::RECTL, param9: *mut super::super::Foundation::POINTL, param10: u32) -> super::super::Foundation::BOOL;
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvQueryAdvanceWidths = unsafe extern "system" fn(param0: DHPDEV, param1: *mut FONTOBJ, param2: u32, param3: *mut u32, param4: *mut ::core::ffi::c_void, param5: u32) -> super::super::Foundation::BOOL;
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvQueryDeviceSupport = unsafe extern "system" fn(param0: *mut SURFOBJ, param1: *mut XLATEOBJ, param2: *mut XFORMOBJ, param3: u32, param4: u32, param5: *mut ::core::ffi::c_void, param6: u32, param7: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type PFN_DrvQueryFont = unsafe extern "system" fn(param0: DHPDEV, param1: usize, param2: u32, param3: *mut usize) -> *mut IFIMETRICS;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub type PFN_DrvQueryFontCaps = unsafe extern "system" fn(param0: u32, param1: *mut u32) -> i32;
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvQueryFontData = unsafe extern "system" fn(param0: DHPDEV, param1: *mut FONTOBJ, param2: u32, param3: u32, param4: *mut GLYPHDATA, param5: *mut ::core::ffi::c_void, param6: u32) -> i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub type PFN_DrvQueryFontFile = unsafe extern "system" fn(param0: usize, param1: u32, param2: u32, param3: *mut u32) -> i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub type PFN_DrvQueryFontTree = unsafe extern "system" fn(param0: DHPDEV, param1: usize, param2: u32, param3: u32, param4: *mut usize) -> *mut ::core::ffi::c_void;
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvQueryGlyphAttrs = unsafe extern "system" fn(param0: *mut FONTOBJ, param1: u32) -> *mut FD_GLYPHATTR;
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvQueryPerBandInfo = unsafe extern "system" fn(param0: *mut SURFOBJ, param1: *mut PERBANDINFO) -> super::super::Foundation::BOOL;
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvQuerySpoolType = unsafe extern "system" fn(dhpdev: DHPDEV, pwchtype: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type PFN_DrvQueryTrueTypeOutline = unsafe extern "system" fn(param0: DHPDEV, param1: *mut FONTOBJ, param2: u32, param3: super::super::Foundation::BOOL, param4: *mut GLYPHDATA, param5: u32, param6: *mut super::super::Graphics::Gdi::TTPOLYGONHEADER) -> i32;
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvQueryTrueTypeSection = unsafe extern "system" fn(param0: u32, param1: u32, param2: u32, param3: *mut super::super::Foundation::HANDLE, param4: *mut i32) -> i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub type PFN_DrvQueryTrueTypeTable = unsafe extern "system" fn(param0: usize, param1: u32, param2: u32, param3: i32, param4: u32, param5: *mut u8, param6: *mut *mut u8, param7: *mut u32) -> i32;
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvRealizeBrush = unsafe extern "system" fn(param0: *mut BRUSHOBJ, param1: *mut SURFOBJ, param2: *mut SURFOBJ, param3: *mut SURFOBJ, param4: *mut XLATEOBJ, param5: u32) -> super::super::Foundation::BOOL;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub type PFN_DrvRenderHint = unsafe extern "system" fn(dhpdev: DHPDEV, notifycode: u32, length: usize, data: *const ::core::ffi::c_void) -> i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub type PFN_DrvResetDevice = unsafe extern "system" fn(param0: DHPDEV, param1: *mut ::core::ffi::c_void) -> u32;
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvResetPDEV = unsafe extern "system" fn(dhpdevold: DHPDEV, dhpdevnew: DHPDEV) -> super::super::Foundation::BOOL;
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvSaveScreenBits = unsafe extern "system" fn(param0: *mut SURFOBJ, param1: u32, param2: usize, param3: *mut super::super::Foundation::RECTL) -> usize;
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvSendPage = unsafe extern "system" fn(param0: *mut SURFOBJ) -> super::super::Foundation::BOOL;
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvSetPalette = unsafe extern "system" fn(param0: DHPDEV, param1: *mut PALOBJ, param2: u32, param3: u32, param4: u32) -> super::super::Foundation::BOOL;
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvSetPixelFormat = unsafe extern "system" fn(param0: *mut SURFOBJ, param1: i32, param2: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvSetPointerShape = unsafe extern "system" fn(param0: *mut SURFOBJ, param1: *mut SURFOBJ, param2: *mut SURFOBJ, param3: *mut XLATEOBJ, param4: i32, param5: i32, param6: i32, param7: i32, param8: *mut super::super::Foundation::RECTL, param9: u32) -> u32;
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvStartBanding = unsafe extern "system" fn(param0: *mut SURFOBJ, ppointl: *mut super::super::Foundation::POINTL) -> super::super::Foundation::BOOL;
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvStartDoc = unsafe extern "system" fn(pso: *mut SURFOBJ, pwszdocname: super::super::Foundation::PWSTR, dwjobid: u32) -> super::super::Foundation::BOOL;
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvStartDxInterop = unsafe extern "system" fn(param0: *mut SURFOBJ, param1: super::super::Foundation::BOOL, kernelmodedevicehandle: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvStartPage = unsafe extern "system" fn(pso: *mut SURFOBJ) -> super::super::Foundation::BOOL;
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type PFN_DrvStretchBlt = unsafe extern "system" fn(param0: *mut SURFOBJ, param1: *mut SURFOBJ, param2: *mut SURFOBJ, param3: *mut CLIPOBJ, param4: *mut XLATEOBJ, param5: *mut super::super::Graphics::Gdi::COLORADJUSTMENT, param6: *mut super::super::Foundation::POINTL, param7: *mut super::super::Foundation::RECTL, param8: *mut super::super::Foundation::RECTL, param9: *mut super::super::Foundation::POINTL, param10: u32) -> super::super::Foundation::BOOL;
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type PFN_DrvStretchBltROP = unsafe extern "system" fn(param0: *mut SURFOBJ, param1: *mut SURFOBJ, param2: *mut SURFOBJ, param3: *mut CLIPOBJ, param4: *mut XLATEOBJ, param5: *mut super::super::Graphics::Gdi::COLORADJUSTMENT, param6: *mut super::super::Foundation::POINTL, param7: *mut super::super::Foundation::RECTL, param8: *mut super::super::Foundation::RECTL, param9: *mut super::super::Foundation::POINTL, param10: u32, param11: *mut BRUSHOBJ, param12: u32) -> super::super::Foundation::BOOL;
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvStrokeAndFillPath = unsafe extern "system" fn(param0: *mut SURFOBJ, param1: *mut PATHOBJ, param2: *mut CLIPOBJ, param3: *mut XFORMOBJ, param4: *mut BRUSHOBJ, param5: *mut LINEATTRS, param6: *mut BRUSHOBJ, param7: *mut super::super::Foundation::POINTL, param8: u32, param9: u32) -> super::super::Foundation::BOOL;
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvStrokePath = unsafe extern "system" fn(param0: *mut SURFOBJ, param1: *mut PATHOBJ, param2: *mut CLIPOBJ, param3: *mut XFORMOBJ, param4: *mut BRUSHOBJ, param5: *mut super::super::Foundation::POINTL, param6: *mut LINEATTRS, param7: u32) -> super::super::Foundation::BOOL;
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvSurfaceComplete = unsafe extern "system" fn(param0: DHPDEV, param1: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvSwapBuffers = unsafe extern "system" fn(param0: *mut SURFOBJ, param1: *mut WNDOBJ) -> super::super::Foundation::BOOL;
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvSynchronize = unsafe extern "system" fn(param0: DHPDEV, param1: *mut super::super::Foundation::RECTL);
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvSynchronizeRedirectionBitmaps = unsafe extern "system" fn(param0: DHPDEV, param1: *mut u64) -> super::super::Foundation::NTSTATUS;
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvSynchronizeSurface = unsafe extern "system" fn(param0: *mut SURFOBJ, param1: *mut super::super::Foundation::RECTL, param2: u32);
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvTextOut = unsafe extern "system" fn(param0: *mut SURFOBJ, param1: *mut STROBJ, param2: *mut FONTOBJ, param3: *mut CLIPOBJ, param4: *mut super::super::Foundation::RECTL, param5: *mut super::super::Foundation::RECTL, param6: *mut BRUSHOBJ, param7: *mut BRUSHOBJ, param8: *mut super::super::Foundation::POINTL, param9: u32) -> super::super::Foundation::BOOL;
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvTransparentBlt = unsafe extern "system" fn(param0: *mut SURFOBJ, param1: *mut SURFOBJ, param2: *mut CLIPOBJ, param3: *mut XLATEOBJ, param4: *mut super::super::Foundation::RECTL, param5: *mut super::super::Foundation::RECTL, param6: u32, param7: u32) -> super::super::Foundation::BOOL;
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvUnloadFontFile = unsafe extern "system" fn(param0: usize) -> super::super::Foundation::BOOL;
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvUnlockDisplayArea = unsafe extern "system" fn(param0: DHPDEV, param1: *mut super::super::Foundation::RECTL);
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_EngCombineRgn = unsafe extern "system" fn(hrgntrg: super::super::Foundation::HANDLE, hrgnsrc1: super::super::Foundation::HANDLE, hrgnsrc2: super::super::Foundation::HANDLE, imode: i32) -> i32;
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_EngCopyRgn = unsafe extern "system" fn(hrgndst: super::super::Foundation::HANDLE, hrgnsrc: super::super::Foundation::HANDLE) -> i32;
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_EngCreateRectRgn = unsafe extern "system" fn(left: i32, top: i32, right: i32, bottom: i32) -> super::super::Foundation::HANDLE;
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_EngDeleteRgn = unsafe extern "system" fn(hrgn: super::super::Foundation::HANDLE);
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_EngIntersectRgn = unsafe extern "system" fn(hrgnresult: super::super::Foundation::HANDLE, hrgna: super::super::Foundation::HANDLE, hrgnb: super::super::Foundation::HANDLE) -> i32;
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_EngSubtractRgn = unsafe extern "system" fn(hrgnresult: super::super::Foundation::HANDLE, hrgna: super::super::Foundation::HANDLE, hrgnb: super::super::Foundation::HANDLE) -> i32;
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_EngUnionRgn = unsafe extern "system" fn(hrgnresult: super::super::Foundation::HANDLE, hrgna: super::super::Foundation::HANDLE, hrgnb: super::super::Foundation::HANDLE) -> i32;
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_EngXorRgn = unsafe extern "system" fn(hrgnresult: super::super::Foundation::HANDLE, hrgna: super::super::Foundation::HANDLE, hrgnb: super::super::Foundation::HANDLE) -> i32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
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
impl ::core::default::Default for PHYSICAL_MONITOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PHYSICAL_MONITOR {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PHYSICAL_MONITOR {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PHYSICAL_MONITOR {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const PHYSICAL_MONITOR_DESCRIPTION_SIZE: u32 = 128u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const PLANAR_HC: u32 = 1u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct POINTE {
    pub x: f32,
    pub y: f32,
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl POINTE {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::default::Default for POINTE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::fmt::Debug for POINTE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("POINTE").field("x", &self.x).field("y", &self.y).finish()
    }
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::cmp::PartialEq for POINTE {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::cmp::Eq for POINTE {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
unsafe impl ::windows::core::Abi for POINTE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(any(target_arch = "x86",))]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct POINTE {
    pub x: u32,
    pub y: u32,
}
#[cfg(any(target_arch = "x86",))]
impl POINTE {}
#[cfg(any(target_arch = "x86",))]
impl ::core::default::Default for POINTE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "x86",))]
impl ::core::fmt::Debug for POINTE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("POINTE").field("x", &self.x).field("y", &self.y).finish()
    }
}
#[cfg(any(target_arch = "x86",))]
impl ::core::cmp::PartialEq for POINTE {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
#[cfg(any(target_arch = "x86",))]
impl ::core::cmp::Eq for POINTE {}
#[cfg(any(target_arch = "x86",))]
unsafe impl ::windows::core::Abi for POINTE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct POINTFIX {
    pub x: i32,
    pub y: i32,
}
impl POINTFIX {}
impl ::core::default::Default for POINTFIX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for POINTFIX {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("POINTFIX").field("x", &self.x).field("y", &self.y).finish()
    }
}
impl ::core::cmp::PartialEq for POINTFIX {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl ::core::cmp::Eq for POINTFIX {}
unsafe impl ::windows::core::Abi for POINTFIX {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct POINTQF {
    pub x: i64,
    pub y: i64,
}
impl POINTQF {}
impl ::core::default::Default for POINTQF {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for POINTQF {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("POINTQF").field("x", &self.x).field("y", &self.y).finish()
    }
}
impl ::core::cmp::PartialEq for POINTQF {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl ::core::cmp::Eq for POINTQF {}
unsafe impl ::windows::core::Abi for POINTQF {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const PO_ALL_INTEGERS: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const PO_BEZIERS: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const PO_ELLIPSE: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const PO_ENUM_AS_INTEGERS: u32 = 8u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const PO_WIDENED: u32 = 16u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const PPC_BGR_ORDER_HORIZONTAL_STRIPES: u32 = 5u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const PPC_BGR_ORDER_VERTICAL_STRIPES: u32 = 3u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const PPC_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const PPC_RGB_ORDER_HORIZONTAL_STRIPES: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const PPC_RGB_ORDER_VERTICAL_STRIPES: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const PPC_UNDEFINED: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const PPG_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const PPG_SRGB: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const PRIMARY_ORDER_ABC: u32 = 0u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const PRIMARY_ORDER_ACB: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const PRIMARY_ORDER_BAC: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const PRIMARY_ORDER_BCA: u32 = 3u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const PRIMARY_ORDER_CAB: u32 = 5u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const PRIMARY_ORDER_CBA: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub type PVIDEO_WIN32K_CALLOUT = unsafe extern "system" fn(params: *mut ::core::ffi::c_void);
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const QAW_GETEASYWIDTHS: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const QAW_GETWIDTHS: u32 = 0u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const QC_1BIT: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const QC_4BIT: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const QC_OUTLINES: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const QDS_CHECKJPEGFORMAT: u32 = 0u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const QDS_CHECKPNGFORMAT: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const QFD_GLYPHANDBITMAP: i32 = 1i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const QFD_GLYPHANDOUTLINE: i32 = 2i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const QFD_MAXEXTENTS: i32 = 3i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const QFD_TT_GLYPHANDBITMAP: i32 = 4i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const QFD_TT_GRAY1_BITMAP: i32 = 5i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const QFD_TT_GRAY2_BITMAP: i32 = 6i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const QFD_TT_GRAY4_BITMAP: i32 = 8i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const QFD_TT_GRAY8_BITMAP: i32 = 9i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const QFD_TT_MONO_BITMAP: i32 = 5i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const QFF_DESCRIPTION: i32 = 1i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const QFF_NUMFACES: i32 = 2i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const QFT_GLYPHSET: i32 = 3i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const QFT_KERNPAIRS: i32 = 2i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const QFT_LIGATURES: i32 = 1i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const QSA_3DNOW: u32 = 16384u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const QSA_MMX: u32 = 256u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const QSA_SSE: u32 = 8192u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const QSA_SSE1: u32 = 8192u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const QSA_SSE2: u32 = 65536u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const QSA_SSE3: u32 = 524288u32;
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn QueryDisplayConfig(flags: u32, numpatharrayelements: *mut u32, patharray: *mut DISPLAYCONFIG_PATH_INFO, nummodeinfoarrayelements: *mut u32, modeinfoarray: *mut DISPLAYCONFIG_MODE_INFO, currenttopologyid: *mut DISPLAYCONFIG_TOPOLOGY_ID) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryDisplayConfig(flags: u32, numpatharrayelements: *mut u32, patharray: *mut DISPLAYCONFIG_PATH_INFO, nummodeinfoarrayelements: *mut u32, modeinfoarray: *mut DISPLAYCONFIG_MODE_INFO, currenttopologyid: *mut DISPLAYCONFIG_TOPOLOGY_ID) -> i32;
        }
        ::core::mem::transmute(QueryDisplayConfig(::core::mem::transmute(flags), ::core::mem::transmute(numpatharrayelements), ::core::mem::transmute(patharray), ::core::mem::transmute(nummodeinfoarrayelements), ::core::mem::transmute(modeinfoarray), ::core::mem::transmute(currenttopologyid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const RB_DITHERCOLOR: i32 = -2147483648i32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct RECTFX {
    pub xLeft: i32,
    pub yTop: i32,
    pub xRight: i32,
    pub yBottom: i32,
}
impl RECTFX {}
impl ::core::default::Default for RECTFX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for RECTFX {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("RECTFX").field("xLeft", &self.xLeft).field("yTop", &self.yTop).field("xRight", &self.xRight).field("yBottom", &self.yBottom).finish()
    }
}
impl ::core::cmp::PartialEq for RECTFX {
    fn eq(&self, other: &Self) -> bool {
        self.xLeft == other.xLeft && self.yTop == other.yTop && self.xRight == other.xRight && self.yBottom == other.yBottom
    }
}
impl ::core::cmp::Eq for RECTFX {}
unsafe impl ::windows::core::Abi for RECTFX {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct RUN {
    pub iStart: i32,
    pub iStop: i32,
}
impl RUN {}
impl ::core::default::Default for RUN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for RUN {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("RUN").field("iStart", &self.iStart).field("iStop", &self.iStop).finish()
    }
}
impl ::core::cmp::PartialEq for RUN {
    fn eq(&self, other: &Self) -> bool {
        self.iStart == other.iStart && self.iStop == other.iStop
    }
}
impl ::core::cmp::Eq for RUN {}
unsafe impl ::windows::core::Abi for RUN {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RestoreMonitorFactoryColorDefaults<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hmonitor: Param0) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RestoreMonitorFactoryColorDefaults(hmonitor: super::super::Foundation::HANDLE) -> i32;
        }
        ::core::mem::transmute(RestoreMonitorFactoryColorDefaults(hmonitor.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RestoreMonitorFactoryDefaults<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hmonitor: Param0) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RestoreMonitorFactoryDefaults(hmonitor: super::super::Foundation::HANDLE) -> i32;
        }
        ::core::mem::transmute(RestoreMonitorFactoryDefaults(hmonitor.into_param().abi()))
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
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct SET_ACTIVE_COLOR_PROFILE_NAME {
    pub ColorProfileName: [u16; 1],
}
impl SET_ACTIVE_COLOR_PROFILE_NAME {}
impl ::core::default::Default for SET_ACTIVE_COLOR_PROFILE_NAME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for SET_ACTIVE_COLOR_PROFILE_NAME {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("SET_ACTIVE_COLOR_PROFILE_NAME").field("ColorProfileName", &self.ColorProfileName).finish()
    }
}
impl ::core::cmp::PartialEq for SET_ACTIVE_COLOR_PROFILE_NAME {
    fn eq(&self, other: &Self) -> bool {
        self.ColorProfileName == other.ColorProfileName
    }
}
impl ::core::cmp::Eq for SET_ACTIVE_COLOR_PROFILE_NAME {}
unsafe impl ::windows::core::Abi for SET_ACTIVE_COLOR_PROFILE_NAME {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const SGI_EXTRASPACE: u32 = 0u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub type SORTCOMP = unsafe extern "system" fn(pv1: *const ::core::ffi::c_void, pv2: *const ::core::ffi::c_void) -> i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const SO_BREAK_EXTRA: u32 = 4096u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const SO_CHARACTER_EXTRA: u32 = 2048u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const SO_CHAR_INC_EQUAL_BM_BASE: u32 = 32u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const SO_DO_NOT_SUBSTITUTE_DEVICE_FONT: u32 = 128u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const SO_DXDY: u32 = 1024u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const SO_ESC_NOT_ORIENT: u32 = 512u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const SO_FLAG_DEFAULT_PLACEMENT: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const SO_GLYPHINDEX_TEXTOUT: u32 = 256u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const SO_HORIZONTAL: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const SO_MAXEXT_EQUAL_BM_SIDE: u32 = 64u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const SO_REVERSED: u32 = 8u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const SO_VERTICAL: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const SO_ZERO_BEARINGS: u32 = 16u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const SPS_ACCEPT_EXCLUDE: u32 = 3u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const SPS_ACCEPT_NOEXCLUDE: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const SPS_ACCEPT_SYNCHRONOUS: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const SPS_ALPHA: i32 = 16i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const SPS_ANIMATESTART: i32 = 4i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const SPS_ANIMATEUPDATE: i32 = 8i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const SPS_ASYNCCHANGE: i32 = 2i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const SPS_CHANGE: i32 = 1i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const SPS_DECLINE: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const SPS_ERROR: u32 = 0u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const SPS_FLAGSMASK: i32 = 255i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const SPS_FREQMASK: i32 = 1044480i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const SPS_LENGTHMASK: i32 = 3840i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const SPS_RESERVED: i32 = 32i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const SPS_RESERVED1: i32 = 64i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const SS_FREE: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const SS_RESTORE: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const SS_SAVE: u32 = 0u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
pub struct STROBJ {
    pub cGlyphs: u32,
    pub flAccel: u32,
    pub ulCharInc: u32,
    pub rclBkGround: super::super::Foundation::RECTL,
    pub pgp: *mut GLYPHPOS,
    pub pwszOrg: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl STROBJ {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for STROBJ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for STROBJ {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("STROBJ").field("cGlyphs", &self.cGlyphs).field("flAccel", &self.flAccel).field("ulCharInc", &self.ulCharInc).field("rclBkGround", &self.rclBkGround).field("pgp", &self.pgp).field("pwszOrg", &self.pwszOrg).finish()
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
unsafe impl ::windows::core::Abi for STROBJ {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn STROBJ_bEnum(pstro: *mut STROBJ, pc: *mut u32, ppgpos: *mut *mut GLYPHPOS) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn STROBJ_bEnum(pstro: *mut STROBJ, pc: *mut u32, ppgpos: *mut *mut GLYPHPOS) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(STROBJ_bEnum(::core::mem::transmute(pstro), ::core::mem::transmute(pc), ::core::mem::transmute(ppgpos)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn STROBJ_bEnumPositionsOnly(pstro: *mut STROBJ, pc: *mut u32, ppgpos: *mut *mut GLYPHPOS) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn STROBJ_bEnumPositionsOnly(pstro: *mut STROBJ, pc: *mut u32, ppgpos: *mut *mut GLYPHPOS) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(STROBJ_bEnumPositionsOnly(::core::mem::transmute(pstro), ::core::mem::transmute(pc), ::core::mem::transmute(ppgpos)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn STROBJ_bGetAdvanceWidths(pso: *mut STROBJ, ifirst: u32, c: u32, pptqd: *mut POINTQF) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn STROBJ_bGetAdvanceWidths(pso: *mut STROBJ, ifirst: u32, c: u32, pptqd: *mut POINTQF) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(STROBJ_bGetAdvanceWidths(::core::mem::transmute(pso), ::core::mem::transmute(ifirst), ::core::mem::transmute(c), ::core::mem::transmute(pptqd)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn STROBJ_dwGetCodePage(pstro: *mut STROBJ) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn STROBJ_dwGetCodePage(pstro: *mut STROBJ) -> u32;
        }
        ::core::mem::transmute(STROBJ_dwGetCodePage(::core::mem::transmute(pstro)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn STROBJ_vEnumStart(pstro: *mut STROBJ) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn STROBJ_vEnumStart(pstro: *mut STROBJ);
        }
        ::core::mem::transmute(STROBJ_vEnumStart(::core::mem::transmute(pstro)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const STYPE_BITMAP: i32 = 0i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const STYPE_DEVBITMAP: i32 = 3i32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
pub struct SURFOBJ {
    pub dhsurf: DHSURF,
    pub hsurf: HSURF,
    pub dhpdev: DHPDEV,
    pub hdev: HDEV,
    pub sizlBitmap: super::super::Foundation::SIZE,
    pub cjBits: u32,
    pub pvBits: *mut ::core::ffi::c_void,
    pub pvScan0: *mut ::core::ffi::c_void,
    pub lDelta: i32,
    pub iUniq: u32,
    pub iBitmapFormat: u32,
    pub iType: u16,
    pub fjBitmap: u16,
}
#[cfg(feature = "Win32_Foundation")]
impl SURFOBJ {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SURFOBJ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SURFOBJ {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("SURFOBJ")
            .field("dhsurf", &self.dhsurf)
            .field("hsurf", &self.hsurf)
            .field("dhpdev", &self.dhpdev)
            .field("hdev", &self.hdev)
            .field("sizlBitmap", &self.sizlBitmap)
            .field("cjBits", &self.cjBits)
            .field("pvBits", &self.pvBits)
            .field("pvScan0", &self.pvScan0)
            .field("lDelta", &self.lDelta)
            .field("iUniq", &self.iUniq)
            .field("iBitmapFormat", &self.iBitmapFormat)
            .field("iType", &self.iType)
            .field("fjBitmap", &self.fjBitmap)
            .finish()
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
unsafe impl ::windows::core::Abi for SURFOBJ {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const S_INIT: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SaveCurrentMonitorSettings<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hmonitor: Param0) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SaveCurrentMonitorSettings(hmonitor: super::super::Foundation::HANDLE) -> i32;
        }
        ::core::mem::transmute(SaveCurrentMonitorSettings(hmonitor.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SaveCurrentSettings<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hmonitor: Param0) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SaveCurrentSettings(hmonitor: super::super::Foundation::HANDLE) -> i32;
        }
        ::core::mem::transmute(SaveCurrentSettings(hmonitor.into_param().abi()))
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
        ::core::mem::transmute(SetDisplayAutoRotationPreferences(::core::mem::transmute(orientation)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetDisplayConfig(numpatharrayelements: u32, patharray: *const DISPLAYCONFIG_PATH_INFO, nummodeinfoarrayelements: u32, modeinfoarray: *const DISPLAYCONFIG_MODE_INFO, flags: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetDisplayConfig(numpatharrayelements: u32, patharray: *const DISPLAYCONFIG_PATH_INFO, nummodeinfoarrayelements: u32, modeinfoarray: *const DISPLAYCONFIG_MODE_INFO, flags: u32) -> i32;
        }
        ::core::mem::transmute(SetDisplayConfig(::core::mem::transmute(numpatharrayelements), ::core::mem::transmute(patharray), ::core::mem::transmute(nummodeinfoarrayelements), ::core::mem::transmute(modeinfoarray), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetMonitorBrightness<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hmonitor: Param0, dwnewbrightness: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetMonitorBrightness(hmonitor: super::super::Foundation::HANDLE, dwnewbrightness: u32) -> i32;
        }
        ::core::mem::transmute(SetMonitorBrightness(hmonitor.into_param().abi(), ::core::mem::transmute(dwnewbrightness)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetMonitorColorTemperature<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hmonitor: Param0, ctcurrentcolortemperature: MC_COLOR_TEMPERATURE) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetMonitorColorTemperature(hmonitor: super::super::Foundation::HANDLE, ctcurrentcolortemperature: MC_COLOR_TEMPERATURE) -> i32;
        }
        ::core::mem::transmute(SetMonitorColorTemperature(hmonitor.into_param().abi(), ::core::mem::transmute(ctcurrentcolortemperature)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetMonitorContrast<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hmonitor: Param0, dwnewcontrast: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetMonitorContrast(hmonitor: super::super::Foundation::HANDLE, dwnewcontrast: u32) -> i32;
        }
        ::core::mem::transmute(SetMonitorContrast(hmonitor.into_param().abi(), ::core::mem::transmute(dwnewcontrast)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetMonitorDisplayAreaPosition<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hmonitor: Param0, ptpositiontype: MC_POSITION_TYPE, dwnewposition: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetMonitorDisplayAreaPosition(hmonitor: super::super::Foundation::HANDLE, ptpositiontype: MC_POSITION_TYPE, dwnewposition: u32) -> i32;
        }
        ::core::mem::transmute(SetMonitorDisplayAreaPosition(hmonitor.into_param().abi(), ::core::mem::transmute(ptpositiontype), ::core::mem::transmute(dwnewposition)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetMonitorDisplayAreaSize<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hmonitor: Param0, stsizetype: MC_SIZE_TYPE, dwnewdisplayareawidthorheight: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetMonitorDisplayAreaSize(hmonitor: super::super::Foundation::HANDLE, stsizetype: MC_SIZE_TYPE, dwnewdisplayareawidthorheight: u32) -> i32;
        }
        ::core::mem::transmute(SetMonitorDisplayAreaSize(hmonitor.into_param().abi(), ::core::mem::transmute(stsizetype), ::core::mem::transmute(dwnewdisplayareawidthorheight)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetMonitorRedGreenOrBlueDrive<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hmonitor: Param0, dtdrivetype: MC_DRIVE_TYPE, dwnewdrive: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetMonitorRedGreenOrBlueDrive(hmonitor: super::super::Foundation::HANDLE, dtdrivetype: MC_DRIVE_TYPE, dwnewdrive: u32) -> i32;
        }
        ::core::mem::transmute(SetMonitorRedGreenOrBlueDrive(hmonitor.into_param().abi(), ::core::mem::transmute(dtdrivetype), ::core::mem::transmute(dwnewdrive)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetMonitorRedGreenOrBlueGain<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hmonitor: Param0, gtgaintype: MC_GAIN_TYPE, dwnewgain: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetMonitorRedGreenOrBlueGain(hmonitor: super::super::Foundation::HANDLE, gtgaintype: MC_GAIN_TYPE, dwnewgain: u32) -> i32;
        }
        ::core::mem::transmute(SetMonitorRedGreenOrBlueGain(hmonitor.into_param().abi(), ::core::mem::transmute(gtgaintype), ::core::mem::transmute(dwnewgain)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetVCPFeature<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hmonitor: Param0, bvcpcode: u8, dwnewvalue: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetVCPFeature(hmonitor: super::super::Foundation::HANDLE, bvcpcode: u8, dwnewvalue: u32) -> i32;
        }
        ::core::mem::transmute(SetVCPFeature(hmonitor.into_param().abi(), ::core::mem::transmute(bvcpcode), ::core::mem::transmute(dwnewvalue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct Sources {
    pub sourceId: u32,
    pub numTargets: i32,
    pub aTargets: [u32; 1],
}
impl Sources {}
impl ::core::default::Default for Sources {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for Sources {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("Sources").field("sourceId", &self.sourceId).field("numTargets", &self.numTargets).field("aTargets", &self.aTargets).finish()
    }
}
impl ::core::cmp::PartialEq for Sources {
    fn eq(&self, other: &Self) -> bool {
        self.sourceId == other.sourceId && self.numTargets == other.numTargets && self.aTargets == other.aTargets
    }
}
impl ::core::cmp::Eq for Sources {}
unsafe impl ::windows::core::Abi for Sources {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const TC_PATHOBJ: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const TC_RECTANGLES: u32 = 0u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const TTO_METRICS_ONLY: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const TTO_QUBICS: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const TTO_UNHINTED: u32 = 4u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
pub struct TYPE1_FONT {
    pub hPFM: super::super::Foundation::HANDLE,
    pub hPFB: super::super::Foundation::HANDLE,
    pub ulIdentifier: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl TYPE1_FONT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TYPE1_FONT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TYPE1_FONT {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("TYPE1_FONT").field("hPFM", &self.hPFM).field("hPFB", &self.hPFB).field("ulIdentifier", &self.ulIdentifier).finish()
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
unsafe impl ::windows::core::Abi for TYPE1_FONT {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
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
impl ::core::default::Default for VGA_CHAR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for VGA_CHAR {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("VGA_CHAR").field("Char", &self.Char).field("Attributes", &self.Attributes).finish()
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
unsafe impl ::windows::core::Abi for VGA_CHAR {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct VIDEOPARAMETERS {
    pub Guid: ::windows::core::GUID,
    pub dwOffset: u32,
    pub dwCommand: u32,
    pub dwFlags: u32,
    pub dwMode: u32,
    pub dwTVStandard: u32,
    pub dwAvailableModes: u32,
    pub dwAvailableTVStandard: u32,
    pub dwFlickerFilter: u32,
    pub dwOverScanX: u32,
    pub dwOverScanY: u32,
    pub dwMaxUnscaledX: u32,
    pub dwMaxUnscaledY: u32,
    pub dwPositionX: u32,
    pub dwPositionY: u32,
    pub dwBrightness: u32,
    pub dwContrast: u32,
    pub dwCPType: u32,
    pub dwCPCommand: u32,
    pub dwCPStandard: u32,
    pub dwCPKey: u32,
    pub bCP_APSTriggerBits: u32,
    pub bOEMCopyProtection: [u8; 256],
}
impl VIDEOPARAMETERS {}
impl ::core::default::Default for VIDEOPARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for VIDEOPARAMETERS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("VIDEOPARAMETERS")
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
unsafe impl ::windows::core::Abi for VIDEOPARAMETERS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
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
impl ::core::default::Default for VIDEO_BANK_SELECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for VIDEO_BANK_SELECT {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
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
impl ::core::cmp::PartialEq for VIDEO_BANK_SELECT {
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
impl ::core::cmp::Eq for VIDEO_BANK_SELECT {}
unsafe impl ::windows::core::Abi for VIDEO_BANK_SELECT {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct VIDEO_BANK_TYPE(pub i32);
pub const VideoNotBanked: VIDEO_BANK_TYPE = VIDEO_BANK_TYPE(0i32);
pub const VideoBanked1RW: VIDEO_BANK_TYPE = VIDEO_BANK_TYPE(1i32);
pub const VideoBanked1R1W: VIDEO_BANK_TYPE = VIDEO_BANK_TYPE(2i32);
pub const VideoBanked2RW: VIDEO_BANK_TYPE = VIDEO_BANK_TYPE(3i32);
pub const NumVideoBankTypes: VIDEO_BANK_TYPE = VIDEO_BANK_TYPE(4i32);
impl ::core::convert::From<i32> for VIDEO_BANK_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for VIDEO_BANK_TYPE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
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
impl ::core::default::Default for VIDEO_BRIGHTNESS_POLICY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for VIDEO_BRIGHTNESS_POLICY {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("VIDEO_BRIGHTNESS_POLICY").field("DefaultToBiosPolicy", &self.DefaultToBiosPolicy).field("LevelCount", &self.LevelCount).field("Level", &self.Level).finish()
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
unsafe impl ::windows::core::Abi for VIDEO_BRIGHTNESS_POLICY {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct VIDEO_BRIGHTNESS_POLICY_0 {
    pub BatteryLevel: u8,
    pub Brightness: u8,
}
#[cfg(feature = "Win32_Foundation")]
impl VIDEO_BRIGHTNESS_POLICY_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for VIDEO_BRIGHTNESS_POLICY_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for VIDEO_BRIGHTNESS_POLICY_0 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("BatteryLevel", &self.BatteryLevel).field("Brightness", &self.Brightness).finish()
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
unsafe impl ::windows::core::Abi for VIDEO_BRIGHTNESS_POLICY_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct VIDEO_CLUT {
    pub NumEntries: u16,
    pub FirstEntry: u16,
    pub LookupTable: [VIDEO_CLUT_0; 1],
}
impl VIDEO_CLUT {}
impl ::core::default::Default for VIDEO_CLUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VIDEO_CLUT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for VIDEO_CLUT {}
unsafe impl ::windows::core::Abi for VIDEO_CLUT {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub union VIDEO_CLUT_0 {
    pub RgbArray: VIDEO_CLUTDATA,
    pub RgbLong: u32,
}
impl VIDEO_CLUT_0 {}
impl ::core::default::Default for VIDEO_CLUT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VIDEO_CLUT_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for VIDEO_CLUT_0 {}
unsafe impl ::windows::core::Abi for VIDEO_CLUT_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct VIDEO_CLUTDATA {
    pub Red: u8,
    pub Green: u8,
    pub Blue: u8,
    pub Unused: u8,
}
impl VIDEO_CLUTDATA {}
impl ::core::default::Default for VIDEO_CLUTDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for VIDEO_CLUTDATA {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("VIDEO_CLUTDATA").field("Red", &self.Red).field("Green", &self.Green).field("Blue", &self.Blue).field("Unused", &self.Unused).finish()
    }
}
impl ::core::cmp::PartialEq for VIDEO_CLUTDATA {
    fn eq(&self, other: &Self) -> bool {
        self.Red == other.Red && self.Green == other.Green && self.Blue == other.Blue && self.Unused == other.Unused
    }
}
impl ::core::cmp::Eq for VIDEO_CLUTDATA {}
unsafe impl ::windows::core::Abi for VIDEO_CLUTDATA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
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
impl ::core::default::Default for VIDEO_COLOR_CAPABILITIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for VIDEO_COLOR_CAPABILITIES {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
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
unsafe impl ::windows::core::Abi for VIDEO_COLOR_CAPABILITIES {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct VIDEO_COLOR_LUT_DATA {
    pub Length: u32,
    pub LutDataFormat: u32,
    pub LutData: [u8; 1],
}
impl VIDEO_COLOR_LUT_DATA {}
impl ::core::default::Default for VIDEO_COLOR_LUT_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for VIDEO_COLOR_LUT_DATA {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("VIDEO_COLOR_LUT_DATA").field("Length", &self.Length).field("LutDataFormat", &self.LutDataFormat).field("LutData", &self.LutData).finish()
    }
}
impl ::core::cmp::PartialEq for VIDEO_COLOR_LUT_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.LutDataFormat == other.LutDataFormat && self.LutData == other.LutData
    }
}
impl ::core::cmp::Eq for VIDEO_COLOR_LUT_DATA {}
unsafe impl ::windows::core::Abi for VIDEO_COLOR_LUT_DATA {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const VIDEO_COLOR_LUT_DATA_FORMAT_PRIVATEFORMAT: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const VIDEO_COLOR_LUT_DATA_FORMAT_RGB256WORDS: u32 = 1u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
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
impl ::core::default::Default for VIDEO_CURSOR_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for VIDEO_CURSOR_ATTRIBUTES {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("VIDEO_CURSOR_ATTRIBUTES").field("Width", &self.Width).field("Height", &self.Height).field("Column", &self.Column).field("Row", &self.Row).field("Rate", &self.Rate).field("Enable", &self.Enable).finish()
    }
}
impl ::core::cmp::PartialEq for VIDEO_CURSOR_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        self.Width == other.Width && self.Height == other.Height && self.Column == other.Column && self.Row == other.Row && self.Rate == other.Rate && self.Enable == other.Enable
    }
}
impl ::core::cmp::Eq for VIDEO_CURSOR_ATTRIBUTES {}
unsafe impl ::windows::core::Abi for VIDEO_CURSOR_ATTRIBUTES {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct VIDEO_CURSOR_POSITION {
    pub Column: i16,
    pub Row: i16,
}
impl VIDEO_CURSOR_POSITION {}
impl ::core::default::Default for VIDEO_CURSOR_POSITION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for VIDEO_CURSOR_POSITION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("VIDEO_CURSOR_POSITION").field("Column", &self.Column).field("Row", &self.Row).finish()
    }
}
impl ::core::cmp::PartialEq for VIDEO_CURSOR_POSITION {
    fn eq(&self, other: &Self) -> bool {
        self.Column == other.Column && self.Row == other.Row
    }
}
impl ::core::cmp::Eq for VIDEO_CURSOR_POSITION {}
unsafe impl ::windows::core::Abi for VIDEO_CURSOR_POSITION {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const VIDEO_DEVICE_COLOR: u32 = 1u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct VIDEO_DEVICE_SESSION_STATUS {
    pub bEnable: u32,
    pub bSuccess: u32,
}
impl VIDEO_DEVICE_SESSION_STATUS {}
impl ::core::default::Default for VIDEO_DEVICE_SESSION_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for VIDEO_DEVICE_SESSION_STATUS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("VIDEO_DEVICE_SESSION_STATUS").field("bEnable", &self.bEnable).field("bSuccess", &self.bSuccess).finish()
    }
}
impl ::core::cmp::PartialEq for VIDEO_DEVICE_SESSION_STATUS {
    fn eq(&self, other: &Self) -> bool {
        self.bEnable == other.bEnable && self.bSuccess == other.bSuccess
    }
}
impl ::core::cmp::Eq for VIDEO_DEVICE_SESSION_STATUS {}
unsafe impl ::windows::core::Abi for VIDEO_DEVICE_SESSION_STATUS {
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
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct VIDEO_HARDWARE_STATE {
    pub StateHeader: *mut VIDEO_HARDWARE_STATE_HEADER,
    pub StateLength: u32,
}
impl VIDEO_HARDWARE_STATE {}
impl ::core::default::Default for VIDEO_HARDWARE_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for VIDEO_HARDWARE_STATE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("VIDEO_HARDWARE_STATE").field("StateHeader", &self.StateHeader).field("StateLength", &self.StateLength).finish()
    }
}
impl ::core::cmp::PartialEq for VIDEO_HARDWARE_STATE {
    fn eq(&self, other: &Self) -> bool {
        self.StateHeader == other.StateHeader && self.StateLength == other.StateLength
    }
}
impl ::core::cmp::Eq for VIDEO_HARDWARE_STATE {}
unsafe impl ::windows::core::Abi for VIDEO_HARDWARE_STATE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
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
    pub FrameBufferData: *mut ::core::ffi::c_void,
}
impl VIDEO_HARDWARE_STATE_HEADER {}
impl ::core::default::Default for VIDEO_HARDWARE_STATE_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for VIDEO_HARDWARE_STATE_HEADER {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
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
unsafe impl ::windows::core::Abi for VIDEO_HARDWARE_STATE_HEADER {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct VIDEO_LOAD_FONT_INFORMATION {
    pub WidthInPixels: u16,
    pub HeightInPixels: u16,
    pub FontSize: u32,
    pub Font: [u8; 1],
}
impl VIDEO_LOAD_FONT_INFORMATION {}
impl ::core::default::Default for VIDEO_LOAD_FONT_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for VIDEO_LOAD_FONT_INFORMATION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("VIDEO_LOAD_FONT_INFORMATION").field("WidthInPixels", &self.WidthInPixels).field("HeightInPixels", &self.HeightInPixels).field("FontSize", &self.FontSize).field("Font", &self.Font).finish()
    }
}
impl ::core::cmp::PartialEq for VIDEO_LOAD_FONT_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.WidthInPixels == other.WidthInPixels && self.HeightInPixels == other.HeightInPixels && self.FontSize == other.FontSize && self.Font == other.Font
    }
}
impl ::core::cmp::Eq for VIDEO_LOAD_FONT_INFORMATION {}
unsafe impl ::windows::core::Abi for VIDEO_LOAD_FONT_INFORMATION {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct VIDEO_LUT_RGB256WORDS {
    pub Red: [u16; 256],
    pub Green: [u16; 256],
    pub Blue: [u16; 256],
}
impl VIDEO_LUT_RGB256WORDS {}
impl ::core::default::Default for VIDEO_LUT_RGB256WORDS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for VIDEO_LUT_RGB256WORDS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("VIDEO_LUT_RGB256WORDS").field("Red", &self.Red).field("Green", &self.Green).field("Blue", &self.Blue).finish()
    }
}
impl ::core::cmp::PartialEq for VIDEO_LUT_RGB256WORDS {
    fn eq(&self, other: &Self) -> bool {
        self.Red == other.Red && self.Green == other.Green && self.Blue == other.Blue
    }
}
impl ::core::cmp::Eq for VIDEO_LUT_RGB256WORDS {}
unsafe impl ::windows::core::Abi for VIDEO_LUT_RGB256WORDS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const VIDEO_MAX_REASON: u32 = 9u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct VIDEO_MEMORY {
    pub RequestedVirtualAddress: *mut ::core::ffi::c_void,
}
impl VIDEO_MEMORY {}
impl ::core::default::Default for VIDEO_MEMORY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for VIDEO_MEMORY {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("VIDEO_MEMORY").field("RequestedVirtualAddress", &self.RequestedVirtualAddress).finish()
    }
}
impl ::core::cmp::PartialEq for VIDEO_MEMORY {
    fn eq(&self, other: &Self) -> bool {
        self.RequestedVirtualAddress == other.RequestedVirtualAddress
    }
}
impl ::core::cmp::Eq for VIDEO_MEMORY {}
unsafe impl ::windows::core::Abi for VIDEO_MEMORY {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct VIDEO_MEMORY_INFORMATION {
    pub VideoRamBase: *mut ::core::ffi::c_void,
    pub VideoRamLength: u32,
    pub FrameBufferBase: *mut ::core::ffi::c_void,
    pub FrameBufferLength: u32,
}
impl VIDEO_MEMORY_INFORMATION {}
impl ::core::default::Default for VIDEO_MEMORY_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for VIDEO_MEMORY_INFORMATION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("VIDEO_MEMORY_INFORMATION").field("VideoRamBase", &self.VideoRamBase).field("VideoRamLength", &self.VideoRamLength).field("FrameBufferBase", &self.FrameBufferBase).field("FrameBufferLength", &self.FrameBufferLength).finish()
    }
}
impl ::core::cmp::PartialEq for VIDEO_MEMORY_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.VideoRamBase == other.VideoRamBase && self.VideoRamLength == other.VideoRamLength && self.FrameBufferBase == other.FrameBufferBase && self.FrameBufferLength == other.FrameBufferLength
    }
}
impl ::core::cmp::Eq for VIDEO_MEMORY_INFORMATION {}
unsafe impl ::windows::core::Abi for VIDEO_MEMORY_INFORMATION {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct VIDEO_MODE {
    pub RequestedMode: u32,
}
impl VIDEO_MODE {}
impl ::core::default::Default for VIDEO_MODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for VIDEO_MODE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("VIDEO_MODE").field("RequestedMode", &self.RequestedMode).finish()
    }
}
impl ::core::cmp::PartialEq for VIDEO_MODE {
    fn eq(&self, other: &Self) -> bool {
        self.RequestedMode == other.RequestedMode
    }
}
impl ::core::cmp::Eq for VIDEO_MODE {}
unsafe impl ::windows::core::Abi for VIDEO_MODE {
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
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
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
impl ::core::default::Default for VIDEO_MODE_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for VIDEO_MODE_INFORMATION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
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
unsafe impl ::windows::core::Abi for VIDEO_MODE_INFORMATION {
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
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct VIDEO_MONITOR_DESCRIPTOR {
    pub DescriptorSize: u32,
    pub Descriptor: [u8; 1],
}
impl VIDEO_MONITOR_DESCRIPTOR {}
impl ::core::default::Default for VIDEO_MONITOR_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for VIDEO_MONITOR_DESCRIPTOR {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("VIDEO_MONITOR_DESCRIPTOR").field("DescriptorSize", &self.DescriptorSize).field("Descriptor", &self.Descriptor).finish()
    }
}
impl ::core::cmp::PartialEq for VIDEO_MONITOR_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.DescriptorSize == other.DescriptorSize && self.Descriptor == other.Descriptor
    }
}
impl ::core::cmp::Eq for VIDEO_MONITOR_DESCRIPTOR {}
unsafe impl ::windows::core::Abi for VIDEO_MONITOR_DESCRIPTOR {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct VIDEO_NUM_MODES {
    pub NumModes: u32,
    pub ModeInformationLength: u32,
}
impl VIDEO_NUM_MODES {}
impl ::core::default::Default for VIDEO_NUM_MODES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for VIDEO_NUM_MODES {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("VIDEO_NUM_MODES").field("NumModes", &self.NumModes).field("ModeInformationLength", &self.ModeInformationLength).finish()
    }
}
impl ::core::cmp::PartialEq for VIDEO_NUM_MODES {
    fn eq(&self, other: &Self) -> bool {
        self.NumModes == other.NumModes && self.ModeInformationLength == other.ModeInformationLength
    }
}
impl ::core::cmp::Eq for VIDEO_NUM_MODES {}
unsafe impl ::windows::core::Abi for VIDEO_NUM_MODES {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const VIDEO_OPTIONAL_GAMMET_TABLE: u32 = 2u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct VIDEO_PALETTE_DATA {
    pub NumEntries: u16,
    pub FirstEntry: u16,
    pub Colors: [u16; 1],
}
impl VIDEO_PALETTE_DATA {}
impl ::core::default::Default for VIDEO_PALETTE_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for VIDEO_PALETTE_DATA {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("VIDEO_PALETTE_DATA").field("NumEntries", &self.NumEntries).field("FirstEntry", &self.FirstEntry).field("Colors", &self.Colors).finish()
    }
}
impl ::core::cmp::PartialEq for VIDEO_PALETTE_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.NumEntries == other.NumEntries && self.FirstEntry == other.FirstEntry && self.Colors == other.Colors
    }
}
impl ::core::cmp::Eq for VIDEO_PALETTE_DATA {}
unsafe impl ::windows::core::Abi for VIDEO_PALETTE_DATA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
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
impl ::core::default::Default for VIDEO_PERFORMANCE_COUNTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for VIDEO_PERFORMANCE_COUNTER {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
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
unsafe impl ::windows::core::Abi for VIDEO_PERFORMANCE_COUNTER {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
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
impl ::core::default::Default for VIDEO_POINTER_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for VIDEO_POINTER_ATTRIBUTES {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("VIDEO_POINTER_ATTRIBUTES").field("Flags", &self.Flags).field("Width", &self.Width).field("Height", &self.Height).field("WidthInBytes", &self.WidthInBytes).field("Enable", &self.Enable).field("Column", &self.Column).field("Row", &self.Row).field("Pixels", &self.Pixels).finish()
    }
}
impl ::core::cmp::PartialEq for VIDEO_POINTER_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.Width == other.Width && self.Height == other.Height && self.WidthInBytes == other.WidthInBytes && self.Enable == other.Enable && self.Column == other.Column && self.Row == other.Row && self.Pixels == other.Pixels
    }
}
impl ::core::cmp::Eq for VIDEO_POINTER_ATTRIBUTES {}
unsafe impl ::windows::core::Abi for VIDEO_POINTER_ATTRIBUTES {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
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
impl ::core::default::Default for VIDEO_POINTER_CAPABILITIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for VIDEO_POINTER_CAPABILITIES {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("VIDEO_POINTER_CAPABILITIES").field("Flags", &self.Flags).field("MaxWidth", &self.MaxWidth).field("MaxHeight", &self.MaxHeight).field("HWPtrBitmapStart", &self.HWPtrBitmapStart).field("HWPtrBitmapEnd", &self.HWPtrBitmapEnd).finish()
    }
}
impl ::core::cmp::PartialEq for VIDEO_POINTER_CAPABILITIES {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.MaxWidth == other.MaxWidth && self.MaxHeight == other.MaxHeight && self.HWPtrBitmapStart == other.HWPtrBitmapStart && self.HWPtrBitmapEnd == other.HWPtrBitmapEnd
    }
}
impl ::core::cmp::Eq for VIDEO_POINTER_CAPABILITIES {}
unsafe impl ::windows::core::Abi for VIDEO_POINTER_CAPABILITIES {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct VIDEO_POINTER_POSITION {
    pub Column: i16,
    pub Row: i16,
}
impl VIDEO_POINTER_POSITION {}
impl ::core::default::Default for VIDEO_POINTER_POSITION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for VIDEO_POINTER_POSITION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("VIDEO_POINTER_POSITION").field("Column", &self.Column).field("Row", &self.Row).finish()
    }
}
impl ::core::cmp::PartialEq for VIDEO_POINTER_POSITION {
    fn eq(&self, other: &Self) -> bool {
        self.Column == other.Column && self.Row == other.Row
    }
}
impl ::core::cmp::Eq for VIDEO_POINTER_POSITION {}
unsafe impl ::windows::core::Abi for VIDEO_POINTER_POSITION {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct VIDEO_POWER_MANAGEMENT {
    pub Length: u32,
    pub DPMSVersion: u32,
    pub PowerState: u32,
}
impl VIDEO_POWER_MANAGEMENT {}
impl ::core::default::Default for VIDEO_POWER_MANAGEMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for VIDEO_POWER_MANAGEMENT {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("VIDEO_POWER_MANAGEMENT").field("Length", &self.Length).field("DPMSVersion", &self.DPMSVersion).field("PowerState", &self.PowerState).finish()
    }
}
impl ::core::cmp::PartialEq for VIDEO_POWER_MANAGEMENT {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.DPMSVersion == other.DPMSVersion && self.PowerState == other.PowerState
    }
}
impl ::core::cmp::Eq for VIDEO_POWER_MANAGEMENT {}
unsafe impl ::windows::core::Abi for VIDEO_POWER_MANAGEMENT {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
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
impl ::core::convert::From<i32> for VIDEO_POWER_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for VIDEO_POWER_STATE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct VIDEO_PUBLIC_ACCESS_RANGES {
    pub InIoSpace: u32,
    pub MappedInIoSpace: u32,
    pub VirtualAddress: *mut ::core::ffi::c_void,
}
impl VIDEO_PUBLIC_ACCESS_RANGES {}
impl ::core::default::Default for VIDEO_PUBLIC_ACCESS_RANGES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for VIDEO_PUBLIC_ACCESS_RANGES {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("VIDEO_PUBLIC_ACCESS_RANGES").field("InIoSpace", &self.InIoSpace).field("MappedInIoSpace", &self.MappedInIoSpace).field("VirtualAddress", &self.VirtualAddress).finish()
    }
}
impl ::core::cmp::PartialEq for VIDEO_PUBLIC_ACCESS_RANGES {
    fn eq(&self, other: &Self) -> bool {
        self.InIoSpace == other.InIoSpace && self.MappedInIoSpace == other.MappedInIoSpace && self.VirtualAddress == other.VirtualAddress
    }
}
impl ::core::cmp::Eq for VIDEO_PUBLIC_ACCESS_RANGES {}
unsafe impl ::windows::core::Abi for VIDEO_PUBLIC_ACCESS_RANGES {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct VIDEO_QUERY_PERFORMANCE_COUNTER {
    pub BufferSize: u32,
    pub Buffer: *mut VIDEO_PERFORMANCE_COUNTER,
}
impl VIDEO_QUERY_PERFORMANCE_COUNTER {}
impl ::core::default::Default for VIDEO_QUERY_PERFORMANCE_COUNTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for VIDEO_QUERY_PERFORMANCE_COUNTER {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("VIDEO_QUERY_PERFORMANCE_COUNTER").field("BufferSize", &self.BufferSize).field("Buffer", &self.Buffer).finish()
    }
}
impl ::core::cmp::PartialEq for VIDEO_QUERY_PERFORMANCE_COUNTER {
    fn eq(&self, other: &Self) -> bool {
        self.BufferSize == other.BufferSize && self.Buffer == other.Buffer
    }
}
impl ::core::cmp::Eq for VIDEO_QUERY_PERFORMANCE_COUNTER {}
unsafe impl ::windows::core::Abi for VIDEO_QUERY_PERFORMANCE_COUNTER {
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
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct VIDEO_REGISTER_VDM {
    pub MinimumStateSize: u32,
}
impl VIDEO_REGISTER_VDM {}
impl ::core::default::Default for VIDEO_REGISTER_VDM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for VIDEO_REGISTER_VDM {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("VIDEO_REGISTER_VDM").field("MinimumStateSize", &self.MinimumStateSize).finish()
    }
}
impl ::core::cmp::PartialEq for VIDEO_REGISTER_VDM {
    fn eq(&self, other: &Self) -> bool {
        self.MinimumStateSize == other.MinimumStateSize
    }
}
impl ::core::cmp::Eq for VIDEO_REGISTER_VDM {}
unsafe impl ::windows::core::Abi for VIDEO_REGISTER_VDM {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
pub struct VIDEO_SHARE_MEMORY {
    pub ProcessHandle: super::super::Foundation::HANDLE,
    pub ViewOffset: u32,
    pub ViewSize: u32,
    pub RequestedVirtualAddress: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl VIDEO_SHARE_MEMORY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for VIDEO_SHARE_MEMORY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for VIDEO_SHARE_MEMORY {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("VIDEO_SHARE_MEMORY").field("ProcessHandle", &self.ProcessHandle).field("ViewOffset", &self.ViewOffset).field("ViewSize", &self.ViewSize).field("RequestedVirtualAddress", &self.RequestedVirtualAddress).finish()
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
unsafe impl ::windows::core::Abi for VIDEO_SHARE_MEMORY {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct VIDEO_SHARE_MEMORY_INFORMATION {
    pub SharedViewOffset: u32,
    pub SharedViewSize: u32,
    pub VirtualAddress: *mut ::core::ffi::c_void,
}
impl VIDEO_SHARE_MEMORY_INFORMATION {}
impl ::core::default::Default for VIDEO_SHARE_MEMORY_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for VIDEO_SHARE_MEMORY_INFORMATION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("VIDEO_SHARE_MEMORY_INFORMATION").field("SharedViewOffset", &self.SharedViewOffset).field("SharedViewSize", &self.SharedViewSize).field("VirtualAddress", &self.VirtualAddress).finish()
    }
}
impl ::core::cmp::PartialEq for VIDEO_SHARE_MEMORY_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.SharedViewOffset == other.SharedViewOffset && self.SharedViewSize == other.SharedViewSize && self.VirtualAddress == other.VirtualAddress
    }
}
impl ::core::cmp::Eq for VIDEO_SHARE_MEMORY_INFORMATION {}
unsafe impl ::windows::core::Abi for VIDEO_SHARE_MEMORY_INFORMATION {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const VIDEO_STATE_NON_STANDARD_VGA: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const VIDEO_STATE_PACKED_CHAIN4_MODE: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const VIDEO_STATE_UNEMULATED_VGA_STATE: u32 = 2u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
pub struct VIDEO_VDM {
    pub ProcessHandle: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl VIDEO_VDM {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for VIDEO_VDM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for VIDEO_VDM {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("VIDEO_VDM").field("ProcessHandle", &self.ProcessHandle).finish()
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
unsafe impl ::windows::core::Abi for VIDEO_VDM {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
pub struct VIDEO_WIN32K_CALLBACKS {
    pub PhysDisp: *mut ::core::ffi::c_void,
    pub Callout: ::core::option::Option<PVIDEO_WIN32K_CALLOUT>,
    pub bACPI: u32,
    pub pPhysDeviceObject: super::super::Foundation::HANDLE,
    pub DualviewFlags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl VIDEO_WIN32K_CALLBACKS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for VIDEO_WIN32K_CALLBACKS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for VIDEO_WIN32K_CALLBACKS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("VIDEO_WIN32K_CALLBACKS").field("PhysDisp", &self.PhysDisp).field("bACPI", &self.bACPI).field("pPhysDeviceObject", &self.pPhysDeviceObject).field("DualviewFlags", &self.DualviewFlags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for VIDEO_WIN32K_CALLBACKS {
    fn eq(&self, other: &Self) -> bool {
        self.PhysDisp == other.PhysDisp && self.Callout.map(|f| f as usize) == other.Callout.map(|f| f as usize) && self.bACPI == other.bACPI && self.pPhysDeviceObject == other.pPhysDeviceObject && self.DualviewFlags == other.DualviewFlags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for VIDEO_WIN32K_CALLBACKS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for VIDEO_WIN32K_CALLBACKS {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
pub struct VIDEO_WIN32K_CALLBACKS_PARAMS {
    pub CalloutType: VIDEO_WIN32K_CALLBACKS_PARAMS_TYPE,
    pub PhysDisp: *mut ::core::ffi::c_void,
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
impl ::core::default::Default for VIDEO_WIN32K_CALLBACKS_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for VIDEO_WIN32K_CALLBACKS_PARAMS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
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
impl ::core::cmp::PartialEq for VIDEO_WIN32K_CALLBACKS_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.CalloutType == other.CalloutType && self.PhysDisp == other.PhysDisp && self.Param == other.Param && self.Status == other.Status && self.LockUserSession == other.LockUserSession && self.IsPostDevice == other.IsPostDevice && self.SurpriseRemoval == other.SurpriseRemoval && self.WaitForQueueReady == other.WaitForQueueReady
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for VIDEO_WIN32K_CALLBACKS_PARAMS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for VIDEO_WIN32K_CALLBACKS_PARAMS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
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
impl ::core::convert::From<i32> for VIDEO_WIN32K_CALLBACKS_PARAMS_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for VIDEO_WIN32K_CALLBACKS_PARAMS_TYPE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct WCRUN {
    pub wcLow: u16,
    pub cGlyphs: u16,
    pub phg: *mut u32,
}
impl WCRUN {}
impl ::core::default::Default for WCRUN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for WCRUN {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WCRUN").field("wcLow", &self.wcLow).field("cGlyphs", &self.cGlyphs).field("phg", &self.phg).finish()
    }
}
impl ::core::cmp::PartialEq for WCRUN {
    fn eq(&self, other: &Self) -> bool {
        self.wcLow == other.wcLow && self.cGlyphs == other.cGlyphs && self.phg == other.phg
    }
}
impl ::core::cmp::Eq for WCRUN {}
unsafe impl ::windows::core::Abi for WCRUN {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const WINDDI_MAXSETPALETTECOLORINDEX: u32 = 255u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const WINDDI_MAXSETPALETTECOLORS: u32 = 256u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const WINDDI_MAX_BROADCAST_CONTEXT: u32 = 64u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
pub struct WNDOBJ {
    pub coClient: CLIPOBJ,
    pub pvConsumer: *mut ::core::ffi::c_void,
    pub rclClient: super::super::Foundation::RECTL,
    pub psoOwner: *mut SURFOBJ,
}
#[cfg(feature = "Win32_Foundation")]
impl WNDOBJ {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WNDOBJ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WNDOBJ {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WNDOBJ").field("coClient", &self.coClient).field("pvConsumer", &self.pvConsumer).field("rclClient", &self.rclClient).field("psoOwner", &self.psoOwner).finish()
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
unsafe impl ::windows::core::Abi for WNDOBJ {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type WNDOBJCHANGEPROC = unsafe extern "system" fn(pwo: *mut WNDOBJ, fl: u32);
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const WNDOBJ_SETUP: u32 = 4354u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const WOC_CHANGED: u32 = 16u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const WOC_DELETE: u32 = 32u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const WOC_DRAWN: u32 = 64u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const WOC_RGN_CLIENT: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const WOC_RGN_CLIENT_DELTA: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const WOC_RGN_SPRITE: u32 = 512u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const WOC_RGN_SURFACE: u32 = 8u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const WOC_RGN_SURFACE_DELTA: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const WOC_SPRITE_NO_OVERLAP: u32 = 256u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const WOC_SPRITE_OVERLAP: u32 = 128u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const WO_DRAW_NOTIFY: u32 = 64u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const WO_RGN_CLIENT: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const WO_RGN_CLIENT_DELTA: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const WO_RGN_DESKTOP_COORD: u32 = 256u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const WO_RGN_SPRITE: u32 = 512u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const WO_RGN_SURFACE: u32 = 8u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const WO_RGN_SURFACE_DELTA: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const WO_RGN_UPDATE_ALL: u32 = 16u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const WO_RGN_WINDOW: u32 = 32u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const WO_SPRITE_NOTIFY: u32 = 128u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct XFORML {
    pub eM11: f32,
    pub eM12: f32,
    pub eM21: f32,
    pub eM22: f32,
    pub eDx: f32,
    pub eDy: f32,
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl XFORML {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::default::Default for XFORML {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::fmt::Debug for XFORML {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("XFORML").field("eM11", &self.eM11).field("eM12", &self.eM12).field("eM21", &self.eM21).field("eM22", &self.eM22).field("eDx", &self.eDx).field("eDy", &self.eDy).finish()
    }
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::cmp::PartialEq for XFORML {
    fn eq(&self, other: &Self) -> bool {
        self.eM11 == other.eM11 && self.eM12 == other.eM12 && self.eM21 == other.eM21 && self.eM22 == other.eM22 && self.eDx == other.eDx && self.eDy == other.eDy
    }
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::cmp::Eq for XFORML {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
unsafe impl ::windows::core::Abi for XFORML {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(any(target_arch = "x86",))]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct XFORML {
    pub eM11: u32,
    pub eM12: u32,
    pub eM21: u32,
    pub eM22: u32,
    pub eDx: u32,
    pub eDy: u32,
}
#[cfg(any(target_arch = "x86",))]
impl XFORML {}
#[cfg(any(target_arch = "x86",))]
impl ::core::default::Default for XFORML {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "x86",))]
impl ::core::fmt::Debug for XFORML {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("XFORML").field("eM11", &self.eM11).field("eM12", &self.eM12).field("eM21", &self.eM21).field("eM22", &self.eM22).field("eDx", &self.eDx).field("eDy", &self.eDy).finish()
    }
}
#[cfg(any(target_arch = "x86",))]
impl ::core::cmp::PartialEq for XFORML {
    fn eq(&self, other: &Self) -> bool {
        self.eM11 == other.eM11 && self.eM12 == other.eM12 && self.eM21 == other.eM21 && self.eM22 == other.eM22 && self.eDx == other.eDx && self.eDy == other.eDy
    }
}
#[cfg(any(target_arch = "x86",))]
impl ::core::cmp::Eq for XFORML {}
#[cfg(any(target_arch = "x86",))]
unsafe impl ::windows::core::Abi for XFORML {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct XFORMOBJ {
    pub ulReserved: u32,
}
impl XFORMOBJ {}
impl ::core::default::Default for XFORMOBJ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for XFORMOBJ {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("XFORMOBJ").field("ulReserved", &self.ulReserved).finish()
    }
}
impl ::core::cmp::PartialEq for XFORMOBJ {
    fn eq(&self, other: &Self) -> bool {
        self.ulReserved == other.ulReserved
    }
}
impl ::core::cmp::Eq for XFORMOBJ {}
unsafe impl ::windows::core::Abi for XFORMOBJ {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn XFORMOBJ_bApplyXform(pxo: *mut XFORMOBJ, imode: u32, cpoints: u32, pvin: *mut ::core::ffi::c_void, pvout: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn XFORMOBJ_bApplyXform(pxo: *mut XFORMOBJ, imode: u32, cpoints: u32, pvin: *mut ::core::ffi::c_void, pvout: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(XFORMOBJ_bApplyXform(::core::mem::transmute(pxo), ::core::mem::transmute(imode), ::core::mem::transmute(cpoints), ::core::mem::transmute(pvin), ::core::mem::transmute(pvout)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
#[inline]
pub unsafe fn XFORMOBJ_iGetXform(pxo: *const XFORMOBJ, pxform: *mut XFORML) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn XFORMOBJ_iGetXform(pxo: *const XFORMOBJ, pxform: *mut XFORML) -> u32;
        }
        ::core::mem::transmute(XFORMOBJ_iGetXform(::core::mem::transmute(pxo), ::core::mem::transmute(pxform)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const XF_INV_FXTOL: i32 = 3i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const XF_INV_LTOL: i32 = 1i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const XF_LTOFX: i32 = 2i32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const XF_LTOL: i32 = 0i32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub struct XLATEOBJ {
    pub iUniq: u32,
    pub flXlate: u32,
    pub iSrcType: u16,
    pub iDstType: u16,
    pub cEntries: u32,
    pub pulXlate: *mut u32,
}
impl XLATEOBJ {}
impl ::core::default::Default for XLATEOBJ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for XLATEOBJ {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("XLATEOBJ").field("iUniq", &self.iUniq).field("flXlate", &self.flXlate).field("iSrcType", &self.iSrcType).field("iDstType", &self.iDstType).field("cEntries", &self.cEntries).field("pulXlate", &self.pulXlate).finish()
    }
}
impl ::core::cmp::PartialEq for XLATEOBJ {
    fn eq(&self, other: &Self) -> bool {
        self.iUniq == other.iUniq && self.flXlate == other.flXlate && self.iSrcType == other.iSrcType && self.iDstType == other.iDstType && self.cEntries == other.cEntries && self.pulXlate == other.pulXlate
    }
}
impl ::core::cmp::Eq for XLATEOBJ {}
unsafe impl ::windows::core::Abi for XLATEOBJ {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
#[inline]
pub unsafe fn XLATEOBJ_cGetPalette(pxlo: *mut XLATEOBJ, ipal: u32, cpal: u32, ppal: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn XLATEOBJ_cGetPalette(pxlo: *mut XLATEOBJ, ipal: u32, cpal: u32, ppal: *mut u32) -> u32;
        }
        ::core::mem::transmute(XLATEOBJ_cGetPalette(::core::mem::transmute(pxlo), ::core::mem::transmute(ipal), ::core::mem::transmute(cpal), ::core::mem::transmute(ppal)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn XLATEOBJ_hGetColorTransform(pxlo: *mut XLATEOBJ) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn XLATEOBJ_hGetColorTransform(pxlo: *mut XLATEOBJ) -> super::super::Foundation::HANDLE;
        }
        ::core::mem::transmute(XLATEOBJ_hGetColorTransform(::core::mem::transmute(pxlo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
#[inline]
pub unsafe fn XLATEOBJ_iXlate(pxlo: *mut XLATEOBJ, icolor: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn XLATEOBJ_iXlate(pxlo: *mut XLATEOBJ, icolor: u32) -> u32;
        }
        ::core::mem::transmute(XLATEOBJ_iXlate(::core::mem::transmute(pxlo), ::core::mem::transmute(icolor)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
#[inline]
pub unsafe fn XLATEOBJ_piVector(pxlo: *mut XLATEOBJ) -> *mut u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn XLATEOBJ_piVector(pxlo: *mut XLATEOBJ) -> *mut u32;
        }
        ::core::mem::transmute(XLATEOBJ_piVector(::core::mem::transmute(pxlo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const XO_DESTBITFIELDS: u32 = 5u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const XO_DESTDCPALETTE: u32 = 3u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const XO_DESTPALETTE: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const XO_DEVICE_ICM: u32 = 16u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const XO_FROM_CMYK: u32 = 8u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const XO_HOST_ICM: u32 = 32u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const XO_SRCBITFIELDS: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const XO_SRCPALETTE: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const XO_TABLE: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const XO_TO_MONO: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Display`*"]
pub const XO_TRIVIAL: u32 = 1u32;
