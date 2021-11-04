#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Graphics_Gdi`*"]
pub struct BLENDOBJ {
    pub BlendFunction: super::super::Graphics::Gdi::BLENDFUNCTION,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl BLENDOBJ {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::std::default::Default for BLENDOBJ {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::std::fmt::Debug for BLENDOBJ {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("BLENDOBJ").field("BlendFunction", &self.BlendFunction).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::std::cmp::PartialEq for BLENDOBJ {
    fn eq(&self, other: &Self) -> bool {
        self.BlendFunction == other.BlendFunction
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::std::cmp::Eq for BLENDOBJ {}
#[cfg(feature = "Win32_Graphics_Gdi")]
unsafe impl ::windows::runtime::Abi for BLENDOBJ {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_DisplayDevices`*"]
pub struct BRUSHOBJ {
    pub iSolidColor: u32,
    pub pvRbrush: *mut ::std::ffi::c_void,
    pub flColorType: u32,
}
impl BRUSHOBJ {}
impl ::std::default::Default for BRUSHOBJ {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for BRUSHOBJ {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("BRUSHOBJ").field("iSolidColor", &self.iSolidColor).field("pvRbrush", &self.pvRbrush).field("flColorType", &self.flColorType).finish()
    }
}
impl ::std::cmp::PartialEq for BRUSHOBJ {
    fn eq(&self, other: &Self) -> bool {
        self.iSolidColor == other.iSolidColor && self.pvRbrush == other.pvRbrush && self.flColorType == other.flColorType
    }
}
impl ::std::cmp::Eq for BRUSHOBJ {}
unsafe impl ::windows::runtime::Abi for BRUSHOBJ {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BRUSHOBJ_hGetColorTransform(pbo: *mut BRUSHOBJ) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BRUSHOBJ_hGetColorTransform(pbo: *mut BRUSHOBJ) -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(BRUSHOBJ_hGetColorTransform(::std::mem::transmute(pbo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_DisplayDevices`*"]
#[inline]
pub unsafe fn BRUSHOBJ_pvAllocRbrush(pbo: *mut BRUSHOBJ, cj: u32) -> *mut ::std::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BRUSHOBJ_pvAllocRbrush(pbo: *mut BRUSHOBJ, cj: u32) -> *mut ::std::ffi::c_void;
        }
        ::std::mem::transmute(BRUSHOBJ_pvAllocRbrush(::std::mem::transmute(pbo), ::std::mem::transmute(cj)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_DisplayDevices`*"]
#[inline]
pub unsafe fn BRUSHOBJ_pvGetRbrush(pbo: *mut BRUSHOBJ) -> *mut ::std::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BRUSHOBJ_pvGetRbrush(pbo: *mut BRUSHOBJ) -> *mut ::std::ffi::c_void;
        }
        ::std::mem::transmute(BRUSHOBJ_pvGetRbrush(::std::mem::transmute(pbo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_DisplayDevices`*"]
#[inline]
pub unsafe fn BRUSHOBJ_ulGetBrushColor(pbo: *mut BRUSHOBJ) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BRUSHOBJ_ulGetBrushColor(pbo: *mut BRUSHOBJ) -> u32;
        }
        ::std::mem::transmute(BRUSHOBJ_ulGetBrushColor(::std::mem::transmute(pbo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_DisplayDevices`*"]
pub struct CIECHROMA {
    pub x: i32,
    pub y: i32,
    pub Y: i32,
}
impl CIECHROMA {}
impl ::std::default::Default for CIECHROMA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CIECHROMA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CIECHROMA").field("x", &self.x).field("y", &self.y).field("Y", &self.Y).finish()
    }
}
impl ::std::cmp::PartialEq for CIECHROMA {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.Y == other.Y
    }
}
impl ::std::cmp::Eq for CIECHROMA {}
unsafe impl ::windows::runtime::Abi for CIECHROMA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_SystemServices")]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_System_SystemServices`*"]
pub struct CLIPLINE {
    pub ptfxA: super::super::System::SystemServices::POINTFIX,
    pub ptfxB: super::super::System::SystemServices::POINTFIX,
    pub lStyleState: i32,
    pub c: u32,
    pub arun: [RUN; 1],
}
#[cfg(feature = "Win32_System_SystemServices")]
impl CLIPLINE {}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::default::Default for CLIPLINE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::fmt::Debug for CLIPLINE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CLIPLINE").field("ptfxA", &self.ptfxA).field("ptfxB", &self.ptfxB).field("lStyleState", &self.lStyleState).field("c", &self.c).field("arun", &self.arun).finish()
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::PartialEq for CLIPLINE {
    fn eq(&self, other: &Self) -> bool {
        self.ptfxA == other.ptfxA && self.ptfxB == other.ptfxB && self.lStyleState == other.lStyleState && self.c == other.c && self.arun == other.arun
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::Eq for CLIPLINE {}
#[cfg(feature = "Win32_System_SystemServices")]
unsafe impl ::windows::runtime::Abi for CLIPLINE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`*"]
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
impl ::std::default::Default for CLIPOBJ {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for CLIPOBJ {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CLIPOBJ").field("iUniq", &self.iUniq).field("rclBounds", &self.rclBounds).field("iDComplexity", &self.iDComplexity).field("iFComplexity", &self.iFComplexity).field("iMode", &self.iMode).field("fjOptions", &self.fjOptions).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for CLIPOBJ {
    fn eq(&self, other: &Self) -> bool {
        self.iUniq == other.iUniq && self.rclBounds == other.rclBounds && self.iDComplexity == other.iDComplexity && self.iFComplexity == other.iFComplexity && self.iMode == other.iMode && self.fjOptions == other.fjOptions
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for CLIPOBJ {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CLIPOBJ {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CLIPOBJ_bEnum(pco: *mut CLIPOBJ, cj: u32, pul: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CLIPOBJ_bEnum(pco: *mut CLIPOBJ, cj: u32, pul: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CLIPOBJ_bEnum(::std::mem::transmute(pco), ::std::mem::transmute(cj), ::std::mem::transmute(pul)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CLIPOBJ_cEnumStart<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(pco: *mut CLIPOBJ, ball: Param1, itype: u32, idirection: u32, climit: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CLIPOBJ_cEnumStart(pco: *mut CLIPOBJ, ball: super::super::Foundation::BOOL, itype: u32, idirection: u32, climit: u32) -> u32;
        }
        ::std::mem::transmute(CLIPOBJ_cEnumStart(::std::mem::transmute(pco), ball.into_param().abi(), ::std::mem::transmute(itype), ::std::mem::transmute(idirection), ::std::mem::transmute(climit)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CLIPOBJ_ppoGetPath(pco: *mut CLIPOBJ) -> *mut PATHOBJ {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CLIPOBJ_ppoGetPath(pco: *mut CLIPOBJ) -> *mut PATHOBJ;
        }
        ::std::mem::transmute(CLIPOBJ_ppoGetPath(::std::mem::transmute(pco)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_DisplayDevices`*"]
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
impl ::std::default::Default for COLORINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for COLORINFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
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
impl ::std::cmp::PartialEq for COLORINFO {
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
impl ::std::cmp::Eq for COLORINFO {}
unsafe impl ::windows::runtime::Abi for COLORINFO {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Graphics_DirectDraw")]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Graphics_DirectDraw`*"]
pub struct DDCOMPBUFFERINFO {
    pub dwSize: u32,
    pub dwNumCompBuffers: u32,
    pub dwWidthToCreate: u32,
    pub dwHeightToCreate: u32,
    pub dwBytesToAllocate: u32,
    pub ddCompCaps: super::super::Graphics::DirectDraw::DDSCAPS2,
    pub ddPixelFormat: super::super::Graphics::DirectDraw::DDPIXELFORMAT,
}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl DDCOMPBUFFERINFO {}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl ::std::default::Default for DDCOMPBUFFERINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl ::std::cmp::PartialEq for DDCOMPBUFFERINFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl ::std::cmp::Eq for DDCOMPBUFFERINFO {}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
unsafe impl ::windows::runtime::Abi for DDCOMPBUFFERINFO {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Graphics_DirectDraw")]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Graphics_DirectDraw`*"]
pub struct DDCORECAPS {
    pub dwSize: u32,
    pub dwCaps: u32,
    pub dwCaps2: u32,
    pub dwCKeyCaps: u32,
    pub dwFXCaps: u32,
    pub dwFXAlphaCaps: u32,
    pub dwPalCaps: u32,
    pub dwSVCaps: u32,
    pub dwAlphaBltConstBitDepths: u32,
    pub dwAlphaBltPixelBitDepths: u32,
    pub dwAlphaBltSurfaceBitDepths: u32,
    pub dwAlphaOverlayConstBitDepths: u32,
    pub dwAlphaOverlayPixelBitDepths: u32,
    pub dwAlphaOverlaySurfaceBitDepths: u32,
    pub dwZBufferBitDepths: u32,
    pub dwVidMemTotal: u32,
    pub dwVidMemFree: u32,
    pub dwMaxVisibleOverlays: u32,
    pub dwCurrVisibleOverlays: u32,
    pub dwNumFourCCCodes: u32,
    pub dwAlignBoundarySrc: u32,
    pub dwAlignSizeSrc: u32,
    pub dwAlignBoundaryDest: u32,
    pub dwAlignSizeDest: u32,
    pub dwAlignStrideAlign: u32,
    pub dwRops: [u32; 8],
    pub ddsCaps: super::super::Graphics::DirectDraw::DDSCAPS,
    pub dwMinOverlayStretch: u32,
    pub dwMaxOverlayStretch: u32,
    pub dwMinLiveVideoStretch: u32,
    pub dwMaxLiveVideoStretch: u32,
    pub dwMinHwCodecStretch: u32,
    pub dwMaxHwCodecStretch: u32,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
    pub dwReserved3: u32,
    pub dwSVBCaps: u32,
    pub dwSVBCKeyCaps: u32,
    pub dwSVBFXCaps: u32,
    pub dwSVBRops: [u32; 8],
    pub dwVSBCaps: u32,
    pub dwVSBCKeyCaps: u32,
    pub dwVSBFXCaps: u32,
    pub dwVSBRops: [u32; 8],
    pub dwSSBCaps: u32,
    pub dwSSBCKeyCaps: u32,
    pub dwSSBFXCaps: u32,
    pub dwSSBRops: [u32; 8],
    pub dwMaxVideoPorts: u32,
    pub dwCurrVideoPorts: u32,
    pub dwSVBCaps2: u32,
}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl DDCORECAPS {}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl ::std::default::Default for DDCORECAPS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl ::std::fmt::Debug for DDCORECAPS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDCORECAPS")
            .field("dwSize", &self.dwSize)
            .field("dwCaps", &self.dwCaps)
            .field("dwCaps2", &self.dwCaps2)
            .field("dwCKeyCaps", &self.dwCKeyCaps)
            .field("dwFXCaps", &self.dwFXCaps)
            .field("dwFXAlphaCaps", &self.dwFXAlphaCaps)
            .field("dwPalCaps", &self.dwPalCaps)
            .field("dwSVCaps", &self.dwSVCaps)
            .field("dwAlphaBltConstBitDepths", &self.dwAlphaBltConstBitDepths)
            .field("dwAlphaBltPixelBitDepths", &self.dwAlphaBltPixelBitDepths)
            .field("dwAlphaBltSurfaceBitDepths", &self.dwAlphaBltSurfaceBitDepths)
            .field("dwAlphaOverlayConstBitDepths", &self.dwAlphaOverlayConstBitDepths)
            .field("dwAlphaOverlayPixelBitDepths", &self.dwAlphaOverlayPixelBitDepths)
            .field("dwAlphaOverlaySurfaceBitDepths", &self.dwAlphaOverlaySurfaceBitDepths)
            .field("dwZBufferBitDepths", &self.dwZBufferBitDepths)
            .field("dwVidMemTotal", &self.dwVidMemTotal)
            .field("dwVidMemFree", &self.dwVidMemFree)
            .field("dwMaxVisibleOverlays", &self.dwMaxVisibleOverlays)
            .field("dwCurrVisibleOverlays", &self.dwCurrVisibleOverlays)
            .field("dwNumFourCCCodes", &self.dwNumFourCCCodes)
            .field("dwAlignBoundarySrc", &self.dwAlignBoundarySrc)
            .field("dwAlignSizeSrc", &self.dwAlignSizeSrc)
            .field("dwAlignBoundaryDest", &self.dwAlignBoundaryDest)
            .field("dwAlignSizeDest", &self.dwAlignSizeDest)
            .field("dwAlignStrideAlign", &self.dwAlignStrideAlign)
            .field("dwRops", &self.dwRops)
            .field("ddsCaps", &self.ddsCaps)
            .field("dwMinOverlayStretch", &self.dwMinOverlayStretch)
            .field("dwMaxOverlayStretch", &self.dwMaxOverlayStretch)
            .field("dwMinLiveVideoStretch", &self.dwMinLiveVideoStretch)
            .field("dwMaxLiveVideoStretch", &self.dwMaxLiveVideoStretch)
            .field("dwMinHwCodecStretch", &self.dwMinHwCodecStretch)
            .field("dwMaxHwCodecStretch", &self.dwMaxHwCodecStretch)
            .field("dwReserved1", &self.dwReserved1)
            .field("dwReserved2", &self.dwReserved2)
            .field("dwReserved3", &self.dwReserved3)
            .field("dwSVBCaps", &self.dwSVBCaps)
            .field("dwSVBCKeyCaps", &self.dwSVBCKeyCaps)
            .field("dwSVBFXCaps", &self.dwSVBFXCaps)
            .field("dwSVBRops", &self.dwSVBRops)
            .field("dwVSBCaps", &self.dwVSBCaps)
            .field("dwVSBCKeyCaps", &self.dwVSBCKeyCaps)
            .field("dwVSBFXCaps", &self.dwVSBFXCaps)
            .field("dwVSBRops", &self.dwVSBRops)
            .field("dwSSBCaps", &self.dwSSBCaps)
            .field("dwSSBCKeyCaps", &self.dwSSBCKeyCaps)
            .field("dwSSBFXCaps", &self.dwSSBFXCaps)
            .field("dwSSBRops", &self.dwSSBRops)
            .field("dwMaxVideoPorts", &self.dwMaxVideoPorts)
            .field("dwCurrVideoPorts", &self.dwCurrVideoPorts)
            .field("dwSVBCaps2", &self.dwSVBCaps2)
            .finish()
    }
}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl ::std::cmp::PartialEq for DDCORECAPS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
            && self.dwCaps == other.dwCaps
            && self.dwCaps2 == other.dwCaps2
            && self.dwCKeyCaps == other.dwCKeyCaps
            && self.dwFXCaps == other.dwFXCaps
            && self.dwFXAlphaCaps == other.dwFXAlphaCaps
            && self.dwPalCaps == other.dwPalCaps
            && self.dwSVCaps == other.dwSVCaps
            && self.dwAlphaBltConstBitDepths == other.dwAlphaBltConstBitDepths
            && self.dwAlphaBltPixelBitDepths == other.dwAlphaBltPixelBitDepths
            && self.dwAlphaBltSurfaceBitDepths == other.dwAlphaBltSurfaceBitDepths
            && self.dwAlphaOverlayConstBitDepths == other.dwAlphaOverlayConstBitDepths
            && self.dwAlphaOverlayPixelBitDepths == other.dwAlphaOverlayPixelBitDepths
            && self.dwAlphaOverlaySurfaceBitDepths == other.dwAlphaOverlaySurfaceBitDepths
            && self.dwZBufferBitDepths == other.dwZBufferBitDepths
            && self.dwVidMemTotal == other.dwVidMemTotal
            && self.dwVidMemFree == other.dwVidMemFree
            && self.dwMaxVisibleOverlays == other.dwMaxVisibleOverlays
            && self.dwCurrVisibleOverlays == other.dwCurrVisibleOverlays
            && self.dwNumFourCCCodes == other.dwNumFourCCCodes
            && self.dwAlignBoundarySrc == other.dwAlignBoundarySrc
            && self.dwAlignSizeSrc == other.dwAlignSizeSrc
            && self.dwAlignBoundaryDest == other.dwAlignBoundaryDest
            && self.dwAlignSizeDest == other.dwAlignSizeDest
            && self.dwAlignStrideAlign == other.dwAlignStrideAlign
            && self.dwRops == other.dwRops
            && self.ddsCaps == other.ddsCaps
            && self.dwMinOverlayStretch == other.dwMinOverlayStretch
            && self.dwMaxOverlayStretch == other.dwMaxOverlayStretch
            && self.dwMinLiveVideoStretch == other.dwMinLiveVideoStretch
            && self.dwMaxLiveVideoStretch == other.dwMaxLiveVideoStretch
            && self.dwMinHwCodecStretch == other.dwMinHwCodecStretch
            && self.dwMaxHwCodecStretch == other.dwMaxHwCodecStretch
            && self.dwReserved1 == other.dwReserved1
            && self.dwReserved2 == other.dwReserved2
            && self.dwReserved3 == other.dwReserved3
            && self.dwSVBCaps == other.dwSVBCaps
            && self.dwSVBCKeyCaps == other.dwSVBCKeyCaps
            && self.dwSVBFXCaps == other.dwSVBFXCaps
            && self.dwSVBRops == other.dwSVBRops
            && self.dwVSBCaps == other.dwVSBCaps
            && self.dwVSBCKeyCaps == other.dwVSBCKeyCaps
            && self.dwVSBFXCaps == other.dwVSBFXCaps
            && self.dwVSBRops == other.dwVSBRops
            && self.dwSSBCaps == other.dwSSBCaps
            && self.dwSSBCKeyCaps == other.dwSSBCKeyCaps
            && self.dwSSBFXCaps == other.dwSSBFXCaps
            && self.dwSSBRops == other.dwSSBRops
            && self.dwMaxVideoPorts == other.dwMaxVideoPorts
            && self.dwCurrVideoPorts == other.dwCurrVideoPorts
            && self.dwSVBCaps2 == other.dwSVBCaps2
    }
}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl ::std::cmp::Eq for DDCORECAPS {}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
unsafe impl ::windows::runtime::Abi for DDCORECAPS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Graphics_Gdi"))]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_DirectDraw`, `Win32_Graphics_Gdi`*"]
pub struct DDHAL_DESTROYDDLOCALDATA {
    pub dwFlags: u32,
    pub pDDLcl: *mut super::super::Graphics::DirectDraw::DDRAWI_DIRECTDRAW_LCL,
    pub ddRVal: ::windows::runtime::HRESULT,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Graphics_Gdi"))]
impl DDHAL_DESTROYDDLOCALDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Graphics_Gdi"))]
impl ::std::default::Default for DDHAL_DESTROYDDLOCALDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Graphics_Gdi"))]
impl ::std::fmt::Debug for DDHAL_DESTROYDDLOCALDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDHAL_DESTROYDDLOCALDATA").field("dwFlags", &self.dwFlags).field("pDDLcl", &self.pDDLcl).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::PartialEq for DDHAL_DESTROYDDLOCALDATA {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags && self.pDDLcl == other.pDDLcl && self.ddRVal == other.ddRVal
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::Eq for DDHAL_DESTROYDDLOCALDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::runtime::Abi for DDHAL_DESTROYDDLOCALDATA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Graphics_Gdi"))]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_DirectDraw`, `Win32_Graphics_Gdi`*"]
pub struct DDHAL_WAITFORVERTICALBLANKDATA {
    pub lpDD: *mut super::super::Graphics::DirectDraw::DDRAWI_DIRECTDRAW_GBL,
    pub dwFlags: u32,
    pub bIsInVB: u32,
    pub hEvent: usize,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub WaitForVerticalBlank: ::std::option::Option<super::super::Graphics::DirectDraw::LPDDHAL_WAITFORVERTICALBLANK>,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Graphics_Gdi"))]
impl DDHAL_WAITFORVERTICALBLANKDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Graphics_Gdi"))]
impl ::std::default::Default for DDHAL_WAITFORVERTICALBLANKDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Graphics_Gdi"))]
impl ::std::fmt::Debug for DDHAL_WAITFORVERTICALBLANKDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDHAL_WAITFORVERTICALBLANKDATA").field("lpDD", &self.lpDD).field("dwFlags", &self.dwFlags).field("bIsInVB", &self.bIsInVB).field("hEvent", &self.hEvent).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::PartialEq for DDHAL_WAITFORVERTICALBLANKDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.dwFlags == other.dwFlags && self.bIsInVB == other.bIsInVB && self.hEvent == other.hEvent && self.ddRVal == other.ddRVal && self.WaitForVerticalBlank.map(|f| f as usize) == other.WaitForVerticalBlank.map(|f| f as usize)
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::Eq for DDHAL_WAITFORVERTICALBLANKDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::runtime::Abi for DDHAL_WAITFORVERTICALBLANKDATA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_DisplayDevices`*"]
pub struct DDKERNELCAPS {
    pub dwSize: u32,
    pub dwCaps: u32,
    pub dwIRQCaps: u32,
}
impl DDKERNELCAPS {}
impl ::std::default::Default for DDKERNELCAPS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DDKERNELCAPS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDKERNELCAPS").field("dwSize", &self.dwSize).field("dwCaps", &self.dwCaps).field("dwIRQCaps", &self.dwIRQCaps).finish()
    }
}
impl ::std::cmp::PartialEq for DDKERNELCAPS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwCaps == other.dwCaps && self.dwIRQCaps == other.dwIRQCaps
    }
}
impl ::std::cmp::Eq for DDKERNELCAPS {}
unsafe impl ::windows::runtime::Abi for DDKERNELCAPS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_DirectDraw`*"]
pub struct DDMOCOMPBUFFERINFO {
    pub dwSize: u32,
    pub lpCompSurface: *mut DD_SURFACE_LOCAL,
    pub dwDataOffset: u32,
    pub dwDataSize: u32,
    pub lpPrivate: *mut ::std::ffi::c_void,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl DDMOCOMPBUFFERINFO {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::default::Default for DDMOCOMPBUFFERINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::fmt::Debug for DDMOCOMPBUFFERINFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDMOCOMPBUFFERINFO").field("dwSize", &self.dwSize).field("lpCompSurface", &self.lpCompSurface).field("dwDataOffset", &self.dwDataOffset).field("dwDataSize", &self.dwDataSize).field("lpPrivate", &self.lpPrivate).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::PartialEq for DDMOCOMPBUFFERINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.lpCompSurface == other.lpCompSurface && self.dwDataOffset == other.dwDataOffset && self.dwDataSize == other.dwDataSize && self.lpPrivate == other.lpPrivate
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::Eq for DDMOCOMPBUFFERINFO {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
unsafe impl ::windows::runtime::Abi for DDMOCOMPBUFFERINFO {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_DisplayDevices`*"]
pub struct DDVIDEOPORTBANDWIDTH {
    pub dwSize: u32,
    pub dwCaps: u32,
    pub dwOverlay: u32,
    pub dwColorkey: u32,
    pub dwYInterpolate: u32,
    pub dwYInterpAndColorkey: u32,
    pub dwReserved1: usize,
    pub dwReserved2: usize,
}
impl DDVIDEOPORTBANDWIDTH {}
impl ::std::default::Default for DDVIDEOPORTBANDWIDTH {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DDVIDEOPORTBANDWIDTH {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDVIDEOPORTBANDWIDTH")
            .field("dwSize", &self.dwSize)
            .field("dwCaps", &self.dwCaps)
            .field("dwOverlay", &self.dwOverlay)
            .field("dwColorkey", &self.dwColorkey)
            .field("dwYInterpolate", &self.dwYInterpolate)
            .field("dwYInterpAndColorkey", &self.dwYInterpAndColorkey)
            .field("dwReserved1", &self.dwReserved1)
            .field("dwReserved2", &self.dwReserved2)
            .finish()
    }
}
impl ::std::cmp::PartialEq for DDVIDEOPORTBANDWIDTH {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwCaps == other.dwCaps && self.dwOverlay == other.dwOverlay && self.dwColorkey == other.dwColorkey && self.dwYInterpolate == other.dwYInterpolate && self.dwYInterpAndColorkey == other.dwYInterpAndColorkey && self.dwReserved1 == other.dwReserved1 && self.dwReserved2 == other.dwReserved2
    }
}
impl ::std::cmp::Eq for DDVIDEOPORTBANDWIDTH {}
unsafe impl ::windows::runtime::Abi for DDVIDEOPORTBANDWIDTH {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_DisplayDevices`*"]
pub struct DDVIDEOPORTCAPS {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwMaxWidth: u32,
    pub dwMaxVBIWidth: u32,
    pub dwMaxHeight: u32,
    pub dwVideoPortID: u32,
    pub dwCaps: u32,
    pub dwFX: u32,
    pub dwNumAutoFlipSurfaces: u32,
    pub dwAlignVideoPortBoundary: u32,
    pub dwAlignVideoPortPrescaleWidth: u32,
    pub dwAlignVideoPortCropBoundary: u32,
    pub dwAlignVideoPortCropWidth: u32,
    pub dwPreshrinkXStep: u32,
    pub dwPreshrinkYStep: u32,
    pub dwNumVBIAutoFlipSurfaces: u32,
    pub dwNumPreferredAutoflip: u32,
    pub wNumFilterTapsX: u16,
    pub wNumFilterTapsY: u16,
}
impl DDVIDEOPORTCAPS {}
impl ::std::default::Default for DDVIDEOPORTCAPS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DDVIDEOPORTCAPS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDVIDEOPORTCAPS")
            .field("dwSize", &self.dwSize)
            .field("dwFlags", &self.dwFlags)
            .field("dwMaxWidth", &self.dwMaxWidth)
            .field("dwMaxVBIWidth", &self.dwMaxVBIWidth)
            .field("dwMaxHeight", &self.dwMaxHeight)
            .field("dwVideoPortID", &self.dwVideoPortID)
            .field("dwCaps", &self.dwCaps)
            .field("dwFX", &self.dwFX)
            .field("dwNumAutoFlipSurfaces", &self.dwNumAutoFlipSurfaces)
            .field("dwAlignVideoPortBoundary", &self.dwAlignVideoPortBoundary)
            .field("dwAlignVideoPortPrescaleWidth", &self.dwAlignVideoPortPrescaleWidth)
            .field("dwAlignVideoPortCropBoundary", &self.dwAlignVideoPortCropBoundary)
            .field("dwAlignVideoPortCropWidth", &self.dwAlignVideoPortCropWidth)
            .field("dwPreshrinkXStep", &self.dwPreshrinkXStep)
            .field("dwPreshrinkYStep", &self.dwPreshrinkYStep)
            .field("dwNumVBIAutoFlipSurfaces", &self.dwNumVBIAutoFlipSurfaces)
            .field("dwNumPreferredAutoflip", &self.dwNumPreferredAutoflip)
            .field("wNumFilterTapsX", &self.wNumFilterTapsX)
            .field("wNumFilterTapsY", &self.wNumFilterTapsY)
            .finish()
    }
}
impl ::std::cmp::PartialEq for DDVIDEOPORTCAPS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
            && self.dwFlags == other.dwFlags
            && self.dwMaxWidth == other.dwMaxWidth
            && self.dwMaxVBIWidth == other.dwMaxVBIWidth
            && self.dwMaxHeight == other.dwMaxHeight
            && self.dwVideoPortID == other.dwVideoPortID
            && self.dwCaps == other.dwCaps
            && self.dwFX == other.dwFX
            && self.dwNumAutoFlipSurfaces == other.dwNumAutoFlipSurfaces
            && self.dwAlignVideoPortBoundary == other.dwAlignVideoPortBoundary
            && self.dwAlignVideoPortPrescaleWidth == other.dwAlignVideoPortPrescaleWidth
            && self.dwAlignVideoPortCropBoundary == other.dwAlignVideoPortCropBoundary
            && self.dwAlignVideoPortCropWidth == other.dwAlignVideoPortCropWidth
            && self.dwPreshrinkXStep == other.dwPreshrinkXStep
            && self.dwPreshrinkYStep == other.dwPreshrinkYStep
            && self.dwNumVBIAutoFlipSurfaces == other.dwNumVBIAutoFlipSurfaces
            && self.dwNumPreferredAutoflip == other.dwNumPreferredAutoflip
            && self.wNumFilterTapsX == other.wNumFilterTapsX
            && self.wNumFilterTapsY == other.wNumFilterTapsY
    }
}
impl ::std::cmp::Eq for DDVIDEOPORTCAPS {}
unsafe impl ::windows::runtime::Abi for DDVIDEOPORTCAPS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Graphics_DirectDraw")]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Graphics_DirectDraw`*"]
pub struct DDVIDEOPORTDESC {
    pub dwSize: u32,
    pub dwFieldWidth: u32,
    pub dwVBIWidth: u32,
    pub dwFieldHeight: u32,
    pub dwMicrosecondsPerField: u32,
    pub dwMaxPixelsPerSecond: u32,
    pub dwVideoPortID: u32,
    pub dwReserved1: u32,
    pub VideoPortType: super::super::Graphics::DirectDraw::DDVIDEOPORTCONNECT,
    pub dwReserved2: usize,
    pub dwReserved3: usize,
}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl DDVIDEOPORTDESC {}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl ::std::default::Default for DDVIDEOPORTDESC {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl ::std::fmt::Debug for DDVIDEOPORTDESC {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDVIDEOPORTDESC")
            .field("dwSize", &self.dwSize)
            .field("dwFieldWidth", &self.dwFieldWidth)
            .field("dwVBIWidth", &self.dwVBIWidth)
            .field("dwFieldHeight", &self.dwFieldHeight)
            .field("dwMicrosecondsPerField", &self.dwMicrosecondsPerField)
            .field("dwMaxPixelsPerSecond", &self.dwMaxPixelsPerSecond)
            .field("dwVideoPortID", &self.dwVideoPortID)
            .field("dwReserved1", &self.dwReserved1)
            .field("VideoPortType", &self.VideoPortType)
            .field("dwReserved2", &self.dwReserved2)
            .field("dwReserved3", &self.dwReserved3)
            .finish()
    }
}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl ::std::cmp::PartialEq for DDVIDEOPORTDESC {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwFieldWidth == other.dwFieldWidth && self.dwVBIWidth == other.dwVBIWidth && self.dwFieldHeight == other.dwFieldHeight && self.dwMicrosecondsPerField == other.dwMicrosecondsPerField && self.dwMaxPixelsPerSecond == other.dwMaxPixelsPerSecond && self.dwVideoPortID == other.dwVideoPortID && self.dwReserved1 == other.dwReserved1 && self.VideoPortType == other.VideoPortType && self.dwReserved2 == other.dwReserved2 && self.dwReserved3 == other.dwReserved3
    }
}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl ::std::cmp::Eq for DDVIDEOPORTDESC {}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
unsafe impl ::windows::runtime::Abi for DDVIDEOPORTDESC {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`, `Win32_Graphics_DirectDraw`*"]
pub struct DDVIDEOPORTINFO {
    pub dwSize: u32,
    pub dwOriginX: u32,
    pub dwOriginY: u32,
    pub dwVPFlags: u32,
    pub rCrop: super::super::Foundation::RECT,
    pub dwPrescaleWidth: u32,
    pub dwPrescaleHeight: u32,
    pub lpddpfInputFormat: *mut super::super::Graphics::DirectDraw::DDPIXELFORMAT,
    pub lpddpfVBIInputFormat: *mut super::super::Graphics::DirectDraw::DDPIXELFORMAT,
    pub lpddpfVBIOutputFormat: *mut super::super::Graphics::DirectDraw::DDPIXELFORMAT,
    pub dwVBIHeight: u32,
    pub dwReserved1: usize,
    pub dwReserved2: usize,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl DDVIDEOPORTINFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::default::Default for DDVIDEOPORTINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::fmt::Debug for DDVIDEOPORTINFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DDVIDEOPORTINFO")
            .field("dwSize", &self.dwSize)
            .field("dwOriginX", &self.dwOriginX)
            .field("dwOriginY", &self.dwOriginY)
            .field("dwVPFlags", &self.dwVPFlags)
            .field("rCrop", &self.rCrop)
            .field("dwPrescaleWidth", &self.dwPrescaleWidth)
            .field("dwPrescaleHeight", &self.dwPrescaleHeight)
            .field("lpddpfInputFormat", &self.lpddpfInputFormat)
            .field("lpddpfVBIInputFormat", &self.lpddpfVBIInputFormat)
            .field("lpddpfVBIOutputFormat", &self.lpddpfVBIOutputFormat)
            .field("dwVBIHeight", &self.dwVBIHeight)
            .field("dwReserved1", &self.dwReserved1)
            .field("dwReserved2", &self.dwReserved2)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::PartialEq for DDVIDEOPORTINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
            && self.dwOriginX == other.dwOriginX
            && self.dwOriginY == other.dwOriginY
            && self.dwVPFlags == other.dwVPFlags
            && self.rCrop == other.rCrop
            && self.dwPrescaleWidth == other.dwPrescaleWidth
            && self.dwPrescaleHeight == other.dwPrescaleHeight
            && self.lpddpfInputFormat == other.lpddpfInputFormat
            && self.lpddpfVBIInputFormat == other.lpddpfVBIInputFormat
            && self.lpddpfVBIOutputFormat == other.lpddpfVBIOutputFormat
            && self.dwVBIHeight == other.dwVBIHeight
            && self.dwReserved1 == other.dwReserved1
            && self.dwReserved2 == other.dwReserved2
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::Eq for DDVIDEOPORTINFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
unsafe impl ::windows::runtime::Abi for DDVIDEOPORTINFO {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_DirectDraw`*"]
pub struct DD_ADDATTACHEDSURFACEDATA {
    pub lpDD: *mut DD_DIRECTDRAW_GLOBAL,
    pub lpDDSurface: *mut DD_SURFACE_LOCAL,
    pub lpSurfAttached: *mut DD_SURFACE_LOCAL,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub AddAttachedSurface: *mut ::std::ffi::c_void,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl DD_ADDATTACHEDSURFACEDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::default::Default for DD_ADDATTACHEDSURFACEDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::fmt::Debug for DD_ADDATTACHEDSURFACEDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DD_ADDATTACHEDSURFACEDATA").field("lpDD", &self.lpDD).field("lpDDSurface", &self.lpDDSurface).field("lpSurfAttached", &self.lpSurfAttached).field("ddRVal", &self.ddRVal).field("AddAttachedSurface", &self.AddAttachedSurface).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::PartialEq for DD_ADDATTACHEDSURFACEDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpDDSurface == other.lpDDSurface && self.lpSurfAttached == other.lpSurfAttached && self.ddRVal == other.ddRVal && self.AddAttachedSurface == other.AddAttachedSurface
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::Eq for DD_ADDATTACHEDSURFACEDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
unsafe impl ::windows::runtime::Abi for DD_ADDATTACHEDSURFACEDATA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_DirectDraw`*"]
pub struct DD_ATTACHLIST {
    pub lpLink: *mut DD_ATTACHLIST,
    pub lpAttached: *mut DD_SURFACE_LOCAL,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl DD_ATTACHLIST {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::default::Default for DD_ATTACHLIST {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::fmt::Debug for DD_ATTACHLIST {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DD_ATTACHLIST").field("lpLink", &self.lpLink).field("lpAttached", &self.lpAttached).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::PartialEq for DD_ATTACHLIST {
    fn eq(&self, other: &Self) -> bool {
        self.lpLink == other.lpLink && self.lpAttached == other.lpAttached
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::Eq for DD_ATTACHLIST {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
unsafe impl ::windows::runtime::Abi for DD_ATTACHLIST {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_DirectDraw`*"]
pub struct DD_BEGINMOCOMPFRAMEDATA {
    pub lpDD: *mut DD_DIRECTDRAW_LOCAL,
    pub lpMoComp: *mut DD_MOTIONCOMP_LOCAL,
    pub lpDestSurface: *mut DD_SURFACE_LOCAL,
    pub dwInputDataSize: u32,
    pub lpInputData: *mut ::std::ffi::c_void,
    pub dwOutputDataSize: u32,
    pub lpOutputData: *mut ::std::ffi::c_void,
    pub ddRVal: ::windows::runtime::HRESULT,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl DD_BEGINMOCOMPFRAMEDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::default::Default for DD_BEGINMOCOMPFRAMEDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::fmt::Debug for DD_BEGINMOCOMPFRAMEDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DD_BEGINMOCOMPFRAMEDATA")
            .field("lpDD", &self.lpDD)
            .field("lpMoComp", &self.lpMoComp)
            .field("lpDestSurface", &self.lpDestSurface)
            .field("dwInputDataSize", &self.dwInputDataSize)
            .field("lpInputData", &self.lpInputData)
            .field("dwOutputDataSize", &self.dwOutputDataSize)
            .field("lpOutputData", &self.lpOutputData)
            .field("ddRVal", &self.ddRVal)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::PartialEq for DD_BEGINMOCOMPFRAMEDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpMoComp == other.lpMoComp && self.lpDestSurface == other.lpDestSurface && self.dwInputDataSize == other.dwInputDataSize && self.lpInputData == other.lpInputData && self.dwOutputDataSize == other.dwOutputDataSize && self.lpOutputData == other.lpOutputData && self.ddRVal == other.ddRVal
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::Eq for DD_BEGINMOCOMPFRAMEDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
unsafe impl ::windows::runtime::Abi for DD_BEGINMOCOMPFRAMEDATA {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::clone::Clone for DD_BLTDATA {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_DirectDraw`*"]
pub struct DD_BLTDATA {
    pub lpDD: *mut DD_DIRECTDRAW_GLOBAL,
    pub lpDDDestSurface: *mut DD_SURFACE_LOCAL,
    pub rDest: super::super::Foundation::RECTL,
    pub lpDDSrcSurface: *mut DD_SURFACE_LOCAL,
    pub rSrc: super::super::Foundation::RECTL,
    pub dwFlags: u32,
    pub dwROPFlags: u32,
    pub bltFX: super::super::Graphics::DirectDraw::DDBLTFX,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub Blt: *mut ::std::ffi::c_void,
    pub IsClipped: super::super::Foundation::BOOL,
    pub rOrigDest: super::super::Foundation::RECTL,
    pub rOrigSrc: super::super::Foundation::RECTL,
    pub dwRectCnt: u32,
    pub prDestRects: *mut super::super::Foundation::RECT,
    pub dwAFlags: u32,
    pub ddargbScaleFactors: super::super::Graphics::DirectDraw::DDARGB,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl DD_BLTDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::default::Default for DD_BLTDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::PartialEq for DD_BLTDATA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::Eq for DD_BLTDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
unsafe impl ::windows::runtime::Abi for DD_BLTDATA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Graphics_Gdi"))]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_DirectDraw`, `Win32_Graphics_Gdi`*"]
pub struct DD_CALLBACKS {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub DestroyDriver: ::std::option::Option<super::super::Graphics::DirectDraw::PDD_DESTROYDRIVER>,
    pub CreateSurface: ::std::option::Option<PDD_CREATESURFACE>,
    pub SetColorKey: ::std::option::Option<super::super::Graphics::DirectDraw::PDD_SETCOLORKEY>,
    pub SetMode: ::std::option::Option<super::super::Graphics::DirectDraw::PDD_SETMODE>,
    pub WaitForVerticalBlank: ::std::option::Option<PDD_WAITFORVERTICALBLANK>,
    pub CanCreateSurface: ::std::option::Option<PDD_CANCREATESURFACE>,
    pub CreatePalette: ::std::option::Option<PDD_CREATEPALETTE>,
    pub GetScanLine: ::std::option::Option<PDD_GETSCANLINE>,
    pub MapMemory: ::std::option::Option<PDD_MAPMEMORY>,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Graphics_Gdi"))]
impl DD_CALLBACKS {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Graphics_Gdi"))]
impl ::std::default::Default for DD_CALLBACKS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Graphics_Gdi"))]
impl ::std::fmt::Debug for DD_CALLBACKS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DD_CALLBACKS").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::PartialEq for DD_CALLBACKS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
            && self.dwFlags == other.dwFlags
            && self.DestroyDriver.map(|f| f as usize) == other.DestroyDriver.map(|f| f as usize)
            && self.CreateSurface.map(|f| f as usize) == other.CreateSurface.map(|f| f as usize)
            && self.SetColorKey.map(|f| f as usize) == other.SetColorKey.map(|f| f as usize)
            && self.SetMode.map(|f| f as usize) == other.SetMode.map(|f| f as usize)
            && self.WaitForVerticalBlank.map(|f| f as usize) == other.WaitForVerticalBlank.map(|f| f as usize)
            && self.CanCreateSurface.map(|f| f as usize) == other.CanCreateSurface.map(|f| f as usize)
            && self.CreatePalette.map(|f| f as usize) == other.CreatePalette.map(|f| f as usize)
            && self.GetScanLine.map(|f| f as usize) == other.GetScanLine.map(|f| f as usize)
            && self.MapMemory.map(|f| f as usize) == other.MapMemory.map(|f| f as usize)
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::Eq for DD_CALLBACKS {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::runtime::Abi for DD_CALLBACKS {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Graphics_DirectDraw")]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Graphics_DirectDraw`*"]
pub struct DD_CANCREATESURFACEDATA {
    pub lpDD: *mut DD_DIRECTDRAW_GLOBAL,
    pub lpDDSurfaceDesc: *mut super::super::Graphics::DirectDraw::DDSURFACEDESC,
    pub bIsDifferentPixelFormat: u32,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub CanCreateSurface: *mut ::std::ffi::c_void,
}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl DD_CANCREATESURFACEDATA {}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl ::std::default::Default for DD_CANCREATESURFACEDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl ::std::fmt::Debug for DD_CANCREATESURFACEDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DD_CANCREATESURFACEDATA").field("lpDD", &self.lpDD).field("lpDDSurfaceDesc", &self.lpDDSurfaceDesc).field("bIsDifferentPixelFormat", &self.bIsDifferentPixelFormat).field("ddRVal", &self.ddRVal).field("CanCreateSurface", &self.CanCreateSurface).finish()
    }
}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl ::std::cmp::PartialEq for DD_CANCREATESURFACEDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpDDSurfaceDesc == other.lpDDSurfaceDesc && self.bIsDifferentPixelFormat == other.bIsDifferentPixelFormat && self.ddRVal == other.ddRVal && self.CanCreateSurface == other.CanCreateSurface
    }
}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl ::std::cmp::Eq for DD_CANCREATESURFACEDATA {}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
unsafe impl ::windows::runtime::Abi for DD_CANCREATESURFACEDATA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Graphics_DirectDraw")]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Graphics_DirectDraw`*"]
pub struct DD_CANCREATEVPORTDATA {
    pub lpDD: *mut DD_DIRECTDRAW_LOCAL,
    pub lpDDVideoPortDesc: *mut DDVIDEOPORTDESC,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub CanCreateVideoPort: *mut ::std::ffi::c_void,
}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl DD_CANCREATEVPORTDATA {}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl ::std::default::Default for DD_CANCREATEVPORTDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl ::std::fmt::Debug for DD_CANCREATEVPORTDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DD_CANCREATEVPORTDATA").field("lpDD", &self.lpDD).field("lpDDVideoPortDesc", &self.lpDDVideoPortDesc).field("ddRVal", &self.ddRVal).field("CanCreateVideoPort", &self.CanCreateVideoPort).finish()
    }
}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl ::std::cmp::PartialEq for DD_CANCREATEVPORTDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpDDVideoPortDesc == other.lpDDVideoPortDesc && self.ddRVal == other.ddRVal && self.CanCreateVideoPort == other.CanCreateVideoPort
    }
}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl ::std::cmp::Eq for DD_CANCREATEVPORTDATA {}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
unsafe impl ::windows::runtime::Abi for DD_CANCREATEVPORTDATA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_DisplayDevices`*"]
pub struct DD_CLIPPER_GLOBAL {
    pub dwReserved1: usize,
}
impl DD_CLIPPER_GLOBAL {}
impl ::std::default::Default for DD_CLIPPER_GLOBAL {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DD_CLIPPER_GLOBAL {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DD_CLIPPER_GLOBAL").field("dwReserved1", &self.dwReserved1).finish()
    }
}
impl ::std::cmp::PartialEq for DD_CLIPPER_GLOBAL {
    fn eq(&self, other: &Self) -> bool {
        self.dwReserved1 == other.dwReserved1
    }
}
impl ::std::cmp::Eq for DD_CLIPPER_GLOBAL {}
unsafe impl ::windows::runtime::Abi for DD_CLIPPER_GLOBAL {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_DisplayDevices`*"]
pub struct DD_CLIPPER_LOCAL {
    pub dwReserved1: usize,
}
impl DD_CLIPPER_LOCAL {}
impl ::std::default::Default for DD_CLIPPER_LOCAL {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DD_CLIPPER_LOCAL {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DD_CLIPPER_LOCAL").field("dwReserved1", &self.dwReserved1).finish()
    }
}
impl ::std::cmp::PartialEq for DD_CLIPPER_LOCAL {
    fn eq(&self, other: &Self) -> bool {
        self.dwReserved1 == other.dwReserved1
    }
}
impl ::std::cmp::Eq for DD_CLIPPER_LOCAL {}
unsafe impl ::windows::runtime::Abi for DD_CLIPPER_LOCAL {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_DirectDraw`*"]
pub struct DD_COLORCONTROLCALLBACKS {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub ColorControl: ::std::option::Option<PDD_COLORCB_COLORCONTROL>,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl DD_COLORCONTROLCALLBACKS {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::default::Default for DD_COLORCONTROLCALLBACKS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::fmt::Debug for DD_COLORCONTROLCALLBACKS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DD_COLORCONTROLCALLBACKS").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::PartialEq for DD_COLORCONTROLCALLBACKS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwFlags == other.dwFlags && self.ColorControl.map(|f| f as usize) == other.ColorControl.map(|f| f as usize)
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::Eq for DD_COLORCONTROLCALLBACKS {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
unsafe impl ::windows::runtime::Abi for DD_COLORCONTROLCALLBACKS {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_DirectDraw`*"]
pub struct DD_COLORCONTROLDATA {
    pub lpDD: *mut DD_DIRECTDRAW_GLOBAL,
    pub lpDDSurface: *mut DD_SURFACE_LOCAL,
    pub lpColorData: *mut super::super::Graphics::DirectDraw::DDCOLORCONTROL,
    pub dwFlags: u32,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub ColorControl: *mut ::std::ffi::c_void,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl DD_COLORCONTROLDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::default::Default for DD_COLORCONTROLDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::fmt::Debug for DD_COLORCONTROLDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DD_COLORCONTROLDATA").field("lpDD", &self.lpDD).field("lpDDSurface", &self.lpDDSurface).field("lpColorData", &self.lpColorData).field("dwFlags", &self.dwFlags).field("ddRVal", &self.ddRVal).field("ColorControl", &self.ColorControl).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::PartialEq for DD_COLORCONTROLDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpDDSurface == other.lpDDSurface && self.lpColorData == other.lpColorData && self.dwFlags == other.dwFlags && self.ddRVal == other.ddRVal && self.ColorControl == other.ColorControl
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::Eq for DD_COLORCONTROLDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
unsafe impl ::windows::runtime::Abi for DD_COLORCONTROLDATA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Graphics_DirectDraw")]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Graphics_DirectDraw`*"]
pub struct DD_CREATEMOCOMPDATA {
    pub lpDD: *mut DD_DIRECTDRAW_LOCAL,
    pub lpMoComp: *mut DD_MOTIONCOMP_LOCAL,
    pub lpGuid: *mut ::windows::runtime::GUID,
    pub dwUncompWidth: u32,
    pub dwUncompHeight: u32,
    pub ddUncompPixelFormat: super::super::Graphics::DirectDraw::DDPIXELFORMAT,
    pub lpData: *mut ::std::ffi::c_void,
    pub dwDataSize: u32,
    pub ddRVal: ::windows::runtime::HRESULT,
}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl DD_CREATEMOCOMPDATA {}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl ::std::default::Default for DD_CREATEMOCOMPDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl ::std::cmp::PartialEq for DD_CREATEMOCOMPDATA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl ::std::cmp::Eq for DD_CREATEMOCOMPDATA {}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
unsafe impl ::windows::runtime::Abi for DD_CREATEMOCOMPDATA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
pub struct DD_CREATEPALETTEDATA {
    pub lpDD: *mut DD_DIRECTDRAW_GLOBAL,
    pub lpDDPalette: *mut DD_PALETTE_GLOBAL,
    pub lpColorTable: *mut super::super::Graphics::Gdi::PALETTEENTRY,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub CreatePalette: *mut ::std::ffi::c_void,
    pub is_excl: super::super::Foundation::BOOL,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl DD_CREATEPALETTEDATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::default::Default for DD_CREATEPALETTEDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::fmt::Debug for DD_CREATEPALETTEDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DD_CREATEPALETTEDATA").field("lpDD", &self.lpDD).field("lpDDPalette", &self.lpDDPalette).field("lpColorTable", &self.lpColorTable).field("ddRVal", &self.ddRVal).field("CreatePalette", &self.CreatePalette).field("is_excl", &self.is_excl).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::PartialEq for DD_CREATEPALETTEDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpDDPalette == other.lpDDPalette && self.lpColorTable == other.lpColorTable && self.ddRVal == other.ddRVal && self.CreatePalette == other.CreatePalette && self.is_excl == other.is_excl
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::Eq for DD_CREATEPALETTEDATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::runtime::Abi for DD_CREATEPALETTEDATA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_DirectDraw`*"]
pub struct DD_CREATESURFACEDATA {
    pub lpDD: *mut DD_DIRECTDRAW_GLOBAL,
    pub lpDDSurfaceDesc: *mut super::super::Graphics::DirectDraw::DDSURFACEDESC,
    pub lplpSList: *mut *mut DD_SURFACE_LOCAL,
    pub dwSCnt: u32,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub CreateSurface: *mut ::std::ffi::c_void,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl DD_CREATESURFACEDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::default::Default for DD_CREATESURFACEDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::fmt::Debug for DD_CREATESURFACEDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DD_CREATESURFACEDATA").field("lpDD", &self.lpDD).field("lpDDSurfaceDesc", &self.lpDDSurfaceDesc).field("lplpSList", &self.lplpSList).field("dwSCnt", &self.dwSCnt).field("ddRVal", &self.ddRVal).field("CreateSurface", &self.CreateSurface).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::PartialEq for DD_CREATESURFACEDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpDDSurfaceDesc == other.lpDDSurfaceDesc && self.lplpSList == other.lplpSList && self.dwSCnt == other.dwSCnt && self.ddRVal == other.ddRVal && self.CreateSurface == other.CreateSurface
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::Eq for DD_CREATESURFACEDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
unsafe impl ::windows::runtime::Abi for DD_CREATESURFACEDATA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_DirectDraw`*"]
pub struct DD_CREATESURFACEEXDATA {
    pub dwFlags: u32,
    pub lpDDLcl: *mut DD_DIRECTDRAW_LOCAL,
    pub lpDDSLcl: *mut DD_SURFACE_LOCAL,
    pub ddRVal: ::windows::runtime::HRESULT,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl DD_CREATESURFACEEXDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::default::Default for DD_CREATESURFACEEXDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::fmt::Debug for DD_CREATESURFACEEXDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DD_CREATESURFACEEXDATA").field("dwFlags", &self.dwFlags).field("lpDDLcl", &self.lpDDLcl).field("lpDDSLcl", &self.lpDDSLcl).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::PartialEq for DD_CREATESURFACEEXDATA {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags && self.lpDDLcl == other.lpDDLcl && self.lpDDSLcl == other.lpDDSLcl && self.ddRVal == other.ddRVal
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::Eq for DD_CREATESURFACEEXDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
unsafe impl ::windows::runtime::Abi for DD_CREATESURFACEEXDATA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_DirectDraw`*"]
pub struct DD_CREATEVPORTDATA {
    pub lpDD: *mut DD_DIRECTDRAW_LOCAL,
    pub lpDDVideoPortDesc: *mut DDVIDEOPORTDESC,
    pub lpVideoPort: *mut DD_VIDEOPORT_LOCAL,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub CreateVideoPort: *mut ::std::ffi::c_void,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl DD_CREATEVPORTDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::default::Default for DD_CREATEVPORTDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::fmt::Debug for DD_CREATEVPORTDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DD_CREATEVPORTDATA").field("lpDD", &self.lpDD).field("lpDDVideoPortDesc", &self.lpDDVideoPortDesc).field("lpVideoPort", &self.lpVideoPort).field("ddRVal", &self.ddRVal).field("CreateVideoPort", &self.CreateVideoPort).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::PartialEq for DD_CREATEVPORTDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpDDVideoPortDesc == other.lpDDVideoPortDesc && self.lpVideoPort == other.lpVideoPort && self.ddRVal == other.ddRVal && self.CreateVideoPort == other.CreateVideoPort
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::Eq for DD_CREATEVPORTDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
unsafe impl ::windows::runtime::Abi for DD_CREATEVPORTDATA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_DirectDraw`*"]
pub struct DD_D3DBUFCALLBACKS {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub CanCreateD3DBuffer: ::std::option::Option<PDD_CANCREATESURFACE>,
    pub CreateD3DBuffer: ::std::option::Option<PDD_CREATESURFACE>,
    pub DestroyD3DBuffer: ::std::option::Option<PDD_SURFCB_DESTROYSURFACE>,
    pub LockD3DBuffer: ::std::option::Option<PDD_SURFCB_LOCK>,
    pub UnlockD3DBuffer: ::std::option::Option<PDD_SURFCB_UNLOCK>,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl DD_D3DBUFCALLBACKS {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::default::Default for DD_D3DBUFCALLBACKS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::fmt::Debug for DD_D3DBUFCALLBACKS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DD_D3DBUFCALLBACKS").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::PartialEq for DD_D3DBUFCALLBACKS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
            && self.dwFlags == other.dwFlags
            && self.CanCreateD3DBuffer.map(|f| f as usize) == other.CanCreateD3DBuffer.map(|f| f as usize)
            && self.CreateD3DBuffer.map(|f| f as usize) == other.CreateD3DBuffer.map(|f| f as usize)
            && self.DestroyD3DBuffer.map(|f| f as usize) == other.DestroyD3DBuffer.map(|f| f as usize)
            && self.LockD3DBuffer.map(|f| f as usize) == other.LockD3DBuffer.map(|f| f as usize)
            && self.UnlockD3DBuffer.map(|f| f as usize) == other.UnlockD3DBuffer.map(|f| f as usize)
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::Eq for DD_D3DBUFCALLBACKS {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
unsafe impl ::windows::runtime::Abi for DD_D3DBUFCALLBACKS {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Graphics_DirectDraw")]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Graphics_DirectDraw`*"]
pub struct DD_DESTROYMOCOMPDATA {
    pub lpDD: *mut DD_DIRECTDRAW_LOCAL,
    pub lpMoComp: *mut DD_MOTIONCOMP_LOCAL,
    pub ddRVal: ::windows::runtime::HRESULT,
}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl DD_DESTROYMOCOMPDATA {}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl ::std::default::Default for DD_DESTROYMOCOMPDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl ::std::fmt::Debug for DD_DESTROYMOCOMPDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DD_DESTROYMOCOMPDATA").field("lpDD", &self.lpDD).field("lpMoComp", &self.lpMoComp).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl ::std::cmp::PartialEq for DD_DESTROYMOCOMPDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpMoComp == other.lpMoComp && self.ddRVal == other.ddRVal
    }
}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl ::std::cmp::Eq for DD_DESTROYMOCOMPDATA {}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
unsafe impl ::windows::runtime::Abi for DD_DESTROYMOCOMPDATA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_DisplayDevices`*"]
pub struct DD_DESTROYPALETTEDATA {
    pub lpDD: *mut DD_DIRECTDRAW_GLOBAL,
    pub lpDDPalette: *mut DD_PALETTE_GLOBAL,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub DestroyPalette: *mut ::std::ffi::c_void,
}
impl DD_DESTROYPALETTEDATA {}
impl ::std::default::Default for DD_DESTROYPALETTEDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DD_DESTROYPALETTEDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DD_DESTROYPALETTEDATA").field("lpDD", &self.lpDD).field("lpDDPalette", &self.lpDDPalette).field("ddRVal", &self.ddRVal).field("DestroyPalette", &self.DestroyPalette).finish()
    }
}
impl ::std::cmp::PartialEq for DD_DESTROYPALETTEDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpDDPalette == other.lpDDPalette && self.ddRVal == other.ddRVal && self.DestroyPalette == other.DestroyPalette
    }
}
impl ::std::cmp::Eq for DD_DESTROYPALETTEDATA {}
unsafe impl ::windows::runtime::Abi for DD_DESTROYPALETTEDATA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_DirectDraw`*"]
pub struct DD_DESTROYSURFACEDATA {
    pub lpDD: *mut DD_DIRECTDRAW_GLOBAL,
    pub lpDDSurface: *mut DD_SURFACE_LOCAL,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub DestroySurface: *mut ::std::ffi::c_void,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl DD_DESTROYSURFACEDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::default::Default for DD_DESTROYSURFACEDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::fmt::Debug for DD_DESTROYSURFACEDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DD_DESTROYSURFACEDATA").field("lpDD", &self.lpDD).field("lpDDSurface", &self.lpDDSurface).field("ddRVal", &self.ddRVal).field("DestroySurface", &self.DestroySurface).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::PartialEq for DD_DESTROYSURFACEDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpDDSurface == other.lpDDSurface && self.ddRVal == other.ddRVal && self.DestroySurface == other.DestroySurface
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::Eq for DD_DESTROYSURFACEDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
unsafe impl ::windows::runtime::Abi for DD_DESTROYSURFACEDATA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_DirectDraw`*"]
pub struct DD_DESTROYVPORTDATA {
    pub lpDD: *mut DD_DIRECTDRAW_LOCAL,
    pub lpVideoPort: *mut DD_VIDEOPORT_LOCAL,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub DestroyVideoPort: *mut ::std::ffi::c_void,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl DD_DESTROYVPORTDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::default::Default for DD_DESTROYVPORTDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::fmt::Debug for DD_DESTROYVPORTDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DD_DESTROYVPORTDATA").field("lpDD", &self.lpDD).field("lpVideoPort", &self.lpVideoPort).field("ddRVal", &self.ddRVal).field("DestroyVideoPort", &self.DestroyVideoPort).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::PartialEq for DD_DESTROYVPORTDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpVideoPort == other.lpVideoPort && self.ddRVal == other.ddRVal && self.DestroyVideoPort == other.DestroyVideoPort
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::Eq for DD_DESTROYVPORTDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
unsafe impl ::windows::runtime::Abi for DD_DESTROYVPORTDATA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_DisplayDevices`*"]
pub struct DD_DIRECTDRAW_GLOBAL {
    pub dhpdev: *mut ::std::ffi::c_void,
    pub dwReserved1: usize,
    pub dwReserved2: usize,
    pub lpDDVideoPortCaps: *mut DDVIDEOPORTCAPS,
}
impl DD_DIRECTDRAW_GLOBAL {}
impl ::std::default::Default for DD_DIRECTDRAW_GLOBAL {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DD_DIRECTDRAW_GLOBAL {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DD_DIRECTDRAW_GLOBAL").field("dhpdev", &self.dhpdev).field("dwReserved1", &self.dwReserved1).field("dwReserved2", &self.dwReserved2).field("lpDDVideoPortCaps", &self.lpDDVideoPortCaps).finish()
    }
}
impl ::std::cmp::PartialEq for DD_DIRECTDRAW_GLOBAL {
    fn eq(&self, other: &Self) -> bool {
        self.dhpdev == other.dhpdev && self.dwReserved1 == other.dwReserved1 && self.dwReserved2 == other.dwReserved2 && self.lpDDVideoPortCaps == other.lpDDVideoPortCaps
    }
}
impl ::std::cmp::Eq for DD_DIRECTDRAW_GLOBAL {}
unsafe impl ::windows::runtime::Abi for DD_DIRECTDRAW_GLOBAL {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_DisplayDevices`*"]
pub struct DD_DIRECTDRAW_LOCAL {
    pub lpGbl: *mut DD_DIRECTDRAW_GLOBAL,
}
impl DD_DIRECTDRAW_LOCAL {}
impl ::std::default::Default for DD_DIRECTDRAW_LOCAL {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DD_DIRECTDRAW_LOCAL {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DD_DIRECTDRAW_LOCAL").field("lpGbl", &self.lpGbl).finish()
    }
}
impl ::std::cmp::PartialEq for DD_DIRECTDRAW_LOCAL {
    fn eq(&self, other: &Self) -> bool {
        self.lpGbl == other.lpGbl
    }
}
impl ::std::cmp::Eq for DD_DIRECTDRAW_LOCAL {}
unsafe impl ::windows::runtime::Abi for DD_DIRECTDRAW_LOCAL {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Graphics_DirectDraw")]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Graphics_DirectDraw`*"]
pub struct DD_ENDMOCOMPFRAMEDATA {
    pub lpDD: *mut DD_DIRECTDRAW_LOCAL,
    pub lpMoComp: *mut DD_MOTIONCOMP_LOCAL,
    pub lpInputData: *mut ::std::ffi::c_void,
    pub dwInputDataSize: u32,
    pub ddRVal: ::windows::runtime::HRESULT,
}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl DD_ENDMOCOMPFRAMEDATA {}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl ::std::default::Default for DD_ENDMOCOMPFRAMEDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl ::std::fmt::Debug for DD_ENDMOCOMPFRAMEDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DD_ENDMOCOMPFRAMEDATA").field("lpDD", &self.lpDD).field("lpMoComp", &self.lpMoComp).field("lpInputData", &self.lpInputData).field("dwInputDataSize", &self.dwInputDataSize).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl ::std::cmp::PartialEq for DD_ENDMOCOMPFRAMEDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpMoComp == other.lpMoComp && self.lpInputData == other.lpInputData && self.dwInputDataSize == other.dwInputDataSize && self.ddRVal == other.ddRVal
    }
}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl ::std::cmp::Eq for DD_ENDMOCOMPFRAMEDATA {}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
unsafe impl ::windows::runtime::Abi for DD_ENDMOCOMPFRAMEDATA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_DirectDraw`*"]
pub struct DD_FLIPDATA {
    pub lpDD: *mut DD_DIRECTDRAW_GLOBAL,
    pub lpSurfCurr: *mut DD_SURFACE_LOCAL,
    pub lpSurfTarg: *mut DD_SURFACE_LOCAL,
    pub dwFlags: u32,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub Flip: *mut ::std::ffi::c_void,
    pub lpSurfCurrLeft: *mut DD_SURFACE_LOCAL,
    pub lpSurfTargLeft: *mut DD_SURFACE_LOCAL,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl DD_FLIPDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::default::Default for DD_FLIPDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::fmt::Debug for DD_FLIPDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DD_FLIPDATA")
            .field("lpDD", &self.lpDD)
            .field("lpSurfCurr", &self.lpSurfCurr)
            .field("lpSurfTarg", &self.lpSurfTarg)
            .field("dwFlags", &self.dwFlags)
            .field("ddRVal", &self.ddRVal)
            .field("Flip", &self.Flip)
            .field("lpSurfCurrLeft", &self.lpSurfCurrLeft)
            .field("lpSurfTargLeft", &self.lpSurfTargLeft)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::PartialEq for DD_FLIPDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpSurfCurr == other.lpSurfCurr && self.lpSurfTarg == other.lpSurfTarg && self.dwFlags == other.dwFlags && self.ddRVal == other.ddRVal && self.Flip == other.Flip && self.lpSurfCurrLeft == other.lpSurfCurrLeft && self.lpSurfTargLeft == other.lpSurfTargLeft
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::Eq for DD_FLIPDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
unsafe impl ::windows::runtime::Abi for DD_FLIPDATA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_DisplayDevices`*"]
pub struct DD_FLIPTOGDISURFACEDATA {
    pub lpDD: *mut DD_DIRECTDRAW_GLOBAL,
    pub dwToGDI: u32,
    pub dwReserved: u32,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub FlipToGDISurface: *mut ::std::ffi::c_void,
}
impl DD_FLIPTOGDISURFACEDATA {}
impl ::std::default::Default for DD_FLIPTOGDISURFACEDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DD_FLIPTOGDISURFACEDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DD_FLIPTOGDISURFACEDATA").field("lpDD", &self.lpDD).field("dwToGDI", &self.dwToGDI).field("dwReserved", &self.dwReserved).field("ddRVal", &self.ddRVal).field("FlipToGDISurface", &self.FlipToGDISurface).finish()
    }
}
impl ::std::cmp::PartialEq for DD_FLIPTOGDISURFACEDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.dwToGDI == other.dwToGDI && self.dwReserved == other.dwReserved && self.ddRVal == other.ddRVal && self.FlipToGDISurface == other.FlipToGDISurface
    }
}
impl ::std::cmp::Eq for DD_FLIPTOGDISURFACEDATA {}
unsafe impl ::windows::runtime::Abi for DD_FLIPTOGDISURFACEDATA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_DirectDraw`*"]
pub struct DD_FLIPVPORTDATA {
    pub lpDD: *mut DD_DIRECTDRAW_LOCAL,
    pub lpVideoPort: *mut DD_VIDEOPORT_LOCAL,
    pub lpSurfCurr: *mut DD_SURFACE_LOCAL,
    pub lpSurfTarg: *mut DD_SURFACE_LOCAL,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub FlipVideoPort: *mut ::std::ffi::c_void,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl DD_FLIPVPORTDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::default::Default for DD_FLIPVPORTDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::fmt::Debug for DD_FLIPVPORTDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DD_FLIPVPORTDATA").field("lpDD", &self.lpDD).field("lpVideoPort", &self.lpVideoPort).field("lpSurfCurr", &self.lpSurfCurr).field("lpSurfTarg", &self.lpSurfTarg).field("ddRVal", &self.ddRVal).field("FlipVideoPort", &self.FlipVideoPort).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::PartialEq for DD_FLIPVPORTDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpVideoPort == other.lpVideoPort && self.lpSurfCurr == other.lpSurfCurr && self.lpSurfTarg == other.lpSurfTarg && self.ddRVal == other.ddRVal && self.FlipVideoPort == other.FlipVideoPort
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::Eq for DD_FLIPVPORTDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
unsafe impl ::windows::runtime::Abi for DD_FLIPVPORTDATA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_DirectDraw`*"]
pub struct DD_FREEDRIVERMEMORYDATA {
    pub lpDD: *mut DD_DIRECTDRAW_GLOBAL,
    pub lpDDSurface: *mut DD_SURFACE_LOCAL,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub FreeDriverMemory: *mut ::std::ffi::c_void,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl DD_FREEDRIVERMEMORYDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::default::Default for DD_FREEDRIVERMEMORYDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::fmt::Debug for DD_FREEDRIVERMEMORYDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DD_FREEDRIVERMEMORYDATA").field("lpDD", &self.lpDD).field("lpDDSurface", &self.lpDDSurface).field("ddRVal", &self.ddRVal).field("FreeDriverMemory", &self.FreeDriverMemory).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::PartialEq for DD_FREEDRIVERMEMORYDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpDDSurface == other.lpDDSurface && self.ddRVal == other.ddRVal && self.FreeDriverMemory == other.FreeDriverMemory
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::Eq for DD_FREEDRIVERMEMORYDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
unsafe impl ::windows::runtime::Abi for DD_FREEDRIVERMEMORYDATA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Graphics_DirectDraw")]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Graphics_DirectDraw`*"]
pub struct DD_GETAVAILDRIVERMEMORYDATA {
    pub lpDD: *mut DD_DIRECTDRAW_GLOBAL,
    pub DDSCaps: super::super::Graphics::DirectDraw::DDSCAPS,
    pub dwTotal: u32,
    pub dwFree: u32,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub GetAvailDriverMemory: *mut ::std::ffi::c_void,
}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl DD_GETAVAILDRIVERMEMORYDATA {}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl ::std::default::Default for DD_GETAVAILDRIVERMEMORYDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl ::std::fmt::Debug for DD_GETAVAILDRIVERMEMORYDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DD_GETAVAILDRIVERMEMORYDATA").field("lpDD", &self.lpDD).field("DDSCaps", &self.DDSCaps).field("dwTotal", &self.dwTotal).field("dwFree", &self.dwFree).field("ddRVal", &self.ddRVal).field("GetAvailDriverMemory", &self.GetAvailDriverMemory).finish()
    }
}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl ::std::cmp::PartialEq for DD_GETAVAILDRIVERMEMORYDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.DDSCaps == other.DDSCaps && self.dwTotal == other.dwTotal && self.dwFree == other.dwFree && self.ddRVal == other.ddRVal && self.GetAvailDriverMemory == other.GetAvailDriverMemory
    }
}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl ::std::cmp::Eq for DD_GETAVAILDRIVERMEMORYDATA {}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
unsafe impl ::windows::runtime::Abi for DD_GETAVAILDRIVERMEMORYDATA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_DirectDraw`*"]
pub struct DD_GETBLTSTATUSDATA {
    pub lpDD: *mut DD_DIRECTDRAW_GLOBAL,
    pub lpDDSurface: *mut DD_SURFACE_LOCAL,
    pub dwFlags: u32,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub GetBltStatus: *mut ::std::ffi::c_void,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl DD_GETBLTSTATUSDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::default::Default for DD_GETBLTSTATUSDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::fmt::Debug for DD_GETBLTSTATUSDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DD_GETBLTSTATUSDATA").field("lpDD", &self.lpDD).field("lpDDSurface", &self.lpDDSurface).field("dwFlags", &self.dwFlags).field("ddRVal", &self.ddRVal).field("GetBltStatus", &self.GetBltStatus).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::PartialEq for DD_GETBLTSTATUSDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpDDSurface == other.lpDDSurface && self.dwFlags == other.dwFlags && self.ddRVal == other.ddRVal && self.GetBltStatus == other.GetBltStatus
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::Eq for DD_GETBLTSTATUSDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
unsafe impl ::windows::runtime::Abi for DD_GETBLTSTATUSDATA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_DisplayDevices`*"]
pub struct DD_GETDRIVERINFODATA {
    pub dhpdev: *mut ::std::ffi::c_void,
    pub dwSize: u32,
    pub dwFlags: u32,
    pub guidInfo: ::windows::runtime::GUID,
    pub dwExpectedSize: u32,
    pub lpvData: *mut ::std::ffi::c_void,
    pub dwActualSize: u32,
    pub ddRVal: ::windows::runtime::HRESULT,
}
impl DD_GETDRIVERINFODATA {}
impl ::std::default::Default for DD_GETDRIVERINFODATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DD_GETDRIVERINFODATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DD_GETDRIVERINFODATA")
            .field("dhpdev", &self.dhpdev)
            .field("dwSize", &self.dwSize)
            .field("dwFlags", &self.dwFlags)
            .field("guidInfo", &self.guidInfo)
            .field("dwExpectedSize", &self.dwExpectedSize)
            .field("lpvData", &self.lpvData)
            .field("dwActualSize", &self.dwActualSize)
            .field("ddRVal", &self.ddRVal)
            .finish()
    }
}
impl ::std::cmp::PartialEq for DD_GETDRIVERINFODATA {
    fn eq(&self, other: &Self) -> bool {
        self.dhpdev == other.dhpdev && self.dwSize == other.dwSize && self.dwFlags == other.dwFlags && self.guidInfo == other.guidInfo && self.dwExpectedSize == other.dwExpectedSize && self.lpvData == other.lpvData && self.dwActualSize == other.dwActualSize && self.ddRVal == other.ddRVal
    }
}
impl ::std::cmp::Eq for DD_GETDRIVERINFODATA {}
unsafe impl ::windows::runtime::Abi for DD_GETDRIVERINFODATA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_DisplayDevices`*"]
pub struct DD_GETDRIVERSTATEDATA {
    pub dwFlags: u32,
    pub Anonymous: DD_GETDRIVERSTATEDATA_0,
    pub lpdwStates: *mut u32,
    pub dwLength: u32,
    pub ddRVal: ::windows::runtime::HRESULT,
}
impl DD_GETDRIVERSTATEDATA {}
impl ::std::default::Default for DD_GETDRIVERSTATEDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for DD_GETDRIVERSTATEDATA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for DD_GETDRIVERSTATEDATA {}
unsafe impl ::windows::runtime::Abi for DD_GETDRIVERSTATEDATA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_DisplayDevices`*"]
pub union DD_GETDRIVERSTATEDATA_0 {
    pub lpDD: *mut DD_DIRECTDRAW_GLOBAL,
    pub dwhContext: usize,
}
impl DD_GETDRIVERSTATEDATA_0 {}
impl ::std::default::Default for DD_GETDRIVERSTATEDATA_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for DD_GETDRIVERSTATEDATA_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for DD_GETDRIVERSTATEDATA_0 {}
unsafe impl ::windows::runtime::Abi for DD_GETDRIVERSTATEDATA_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_DirectDraw`*"]
pub struct DD_GETFLIPSTATUSDATA {
    pub lpDD: *mut DD_DIRECTDRAW_GLOBAL,
    pub lpDDSurface: *mut DD_SURFACE_LOCAL,
    pub dwFlags: u32,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub GetFlipStatus: *mut ::std::ffi::c_void,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl DD_GETFLIPSTATUSDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::default::Default for DD_GETFLIPSTATUSDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::fmt::Debug for DD_GETFLIPSTATUSDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DD_GETFLIPSTATUSDATA").field("lpDD", &self.lpDD).field("lpDDSurface", &self.lpDDSurface).field("dwFlags", &self.dwFlags).field("ddRVal", &self.ddRVal).field("GetFlipStatus", &self.GetFlipStatus).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::PartialEq for DD_GETFLIPSTATUSDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpDDSurface == other.lpDDSurface && self.dwFlags == other.dwFlags && self.ddRVal == other.ddRVal && self.GetFlipStatus == other.GetFlipStatus
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::Eq for DD_GETFLIPSTATUSDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
unsafe impl ::windows::runtime::Abi for DD_GETFLIPSTATUSDATA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Graphics_DirectDraw")]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Graphics_DirectDraw`*"]
pub struct DD_GETINTERNALMOCOMPDATA {
    pub lpDD: *mut DD_DIRECTDRAW_LOCAL,
    pub lpGuid: *mut ::windows::runtime::GUID,
    pub dwWidth: u32,
    pub dwHeight: u32,
    pub ddPixelFormat: super::super::Graphics::DirectDraw::DDPIXELFORMAT,
    pub dwScratchMemAlloc: u32,
    pub ddRVal: ::windows::runtime::HRESULT,
}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl DD_GETINTERNALMOCOMPDATA {}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl ::std::default::Default for DD_GETINTERNALMOCOMPDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl ::std::cmp::PartialEq for DD_GETINTERNALMOCOMPDATA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl ::std::cmp::Eq for DD_GETINTERNALMOCOMPDATA {}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
unsafe impl ::windows::runtime::Abi for DD_GETINTERNALMOCOMPDATA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Graphics_DirectDraw")]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Graphics_DirectDraw`*"]
pub struct DD_GETMOCOMPCOMPBUFFDATA {
    pub lpDD: *mut DD_DIRECTDRAW_LOCAL,
    pub lpGuid: *mut ::windows::runtime::GUID,
    pub dwWidth: u32,
    pub dwHeight: u32,
    pub ddPixelFormat: super::super::Graphics::DirectDraw::DDPIXELFORMAT,
    pub dwNumTypesCompBuffs: u32,
    pub lpCompBuffInfo: *mut DDCOMPBUFFERINFO,
    pub ddRVal: ::windows::runtime::HRESULT,
}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl DD_GETMOCOMPCOMPBUFFDATA {}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl ::std::default::Default for DD_GETMOCOMPCOMPBUFFDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl ::std::cmp::PartialEq for DD_GETMOCOMPCOMPBUFFDATA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl ::std::cmp::Eq for DD_GETMOCOMPCOMPBUFFDATA {}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
unsafe impl ::windows::runtime::Abi for DD_GETMOCOMPCOMPBUFFDATA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Graphics_DirectDraw")]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Graphics_DirectDraw`*"]
pub struct DD_GETMOCOMPFORMATSDATA {
    pub lpDD: *mut DD_DIRECTDRAW_LOCAL,
    pub lpGuid: *mut ::windows::runtime::GUID,
    pub dwNumFormats: u32,
    pub lpFormats: *mut super::super::Graphics::DirectDraw::DDPIXELFORMAT,
    pub ddRVal: ::windows::runtime::HRESULT,
}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl DD_GETMOCOMPFORMATSDATA {}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl ::std::default::Default for DD_GETMOCOMPFORMATSDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl ::std::fmt::Debug for DD_GETMOCOMPFORMATSDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DD_GETMOCOMPFORMATSDATA").field("lpDD", &self.lpDD).field("lpGuid", &self.lpGuid).field("dwNumFormats", &self.dwNumFormats).field("lpFormats", &self.lpFormats).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl ::std::cmp::PartialEq for DD_GETMOCOMPFORMATSDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpGuid == other.lpGuid && self.dwNumFormats == other.dwNumFormats && self.lpFormats == other.lpFormats && self.ddRVal == other.ddRVal
    }
}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl ::std::cmp::Eq for DD_GETMOCOMPFORMATSDATA {}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
unsafe impl ::windows::runtime::Abi for DD_GETMOCOMPFORMATSDATA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_DisplayDevices`*"]
pub struct DD_GETMOCOMPGUIDSDATA {
    pub lpDD: *mut DD_DIRECTDRAW_LOCAL,
    pub dwNumGuids: u32,
    pub lpGuids: *mut ::windows::runtime::GUID,
    pub ddRVal: ::windows::runtime::HRESULT,
}
impl DD_GETMOCOMPGUIDSDATA {}
impl ::std::default::Default for DD_GETMOCOMPGUIDSDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DD_GETMOCOMPGUIDSDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DD_GETMOCOMPGUIDSDATA").field("lpDD", &self.lpDD).field("dwNumGuids", &self.dwNumGuids).field("lpGuids", &self.lpGuids).field("ddRVal", &self.ddRVal).finish()
    }
}
impl ::std::cmp::PartialEq for DD_GETMOCOMPGUIDSDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.dwNumGuids == other.dwNumGuids && self.lpGuids == other.lpGuids && self.ddRVal == other.ddRVal
    }
}
impl ::std::cmp::Eq for DD_GETMOCOMPGUIDSDATA {}
unsafe impl ::windows::runtime::Abi for DD_GETMOCOMPGUIDSDATA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_DisplayDevices`*"]
pub struct DD_GETSCANLINEDATA {
    pub lpDD: *mut DD_DIRECTDRAW_GLOBAL,
    pub dwScanLine: u32,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub GetScanLine: *mut ::std::ffi::c_void,
}
impl DD_GETSCANLINEDATA {}
impl ::std::default::Default for DD_GETSCANLINEDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DD_GETSCANLINEDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DD_GETSCANLINEDATA").field("lpDD", &self.lpDD).field("dwScanLine", &self.dwScanLine).field("ddRVal", &self.ddRVal).field("GetScanLine", &self.GetScanLine).finish()
    }
}
impl ::std::cmp::PartialEq for DD_GETSCANLINEDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.dwScanLine == other.dwScanLine && self.ddRVal == other.ddRVal && self.GetScanLine == other.GetScanLine
    }
}
impl ::std::cmp::Eq for DD_GETSCANLINEDATA {}
unsafe impl ::windows::runtime::Abi for DD_GETSCANLINEDATA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_DirectDraw`*"]
pub struct DD_GETVPORTBANDWIDTHDATA {
    pub lpDD: *mut DD_DIRECTDRAW_LOCAL,
    pub lpVideoPort: *mut DD_VIDEOPORT_LOCAL,
    pub lpddpfFormat: *mut super::super::Graphics::DirectDraw::DDPIXELFORMAT,
    pub dwWidth: u32,
    pub dwHeight: u32,
    pub dwFlags: u32,
    pub lpBandwidth: *mut DDVIDEOPORTBANDWIDTH,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub GetVideoPortBandwidth: *mut ::std::ffi::c_void,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl DD_GETVPORTBANDWIDTHDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::default::Default for DD_GETVPORTBANDWIDTHDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::fmt::Debug for DD_GETVPORTBANDWIDTHDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DD_GETVPORTBANDWIDTHDATA")
            .field("lpDD", &self.lpDD)
            .field("lpVideoPort", &self.lpVideoPort)
            .field("lpddpfFormat", &self.lpddpfFormat)
            .field("dwWidth", &self.dwWidth)
            .field("dwHeight", &self.dwHeight)
            .field("dwFlags", &self.dwFlags)
            .field("lpBandwidth", &self.lpBandwidth)
            .field("ddRVal", &self.ddRVal)
            .field("GetVideoPortBandwidth", &self.GetVideoPortBandwidth)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::PartialEq for DD_GETVPORTBANDWIDTHDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpVideoPort == other.lpVideoPort && self.lpddpfFormat == other.lpddpfFormat && self.dwWidth == other.dwWidth && self.dwHeight == other.dwHeight && self.dwFlags == other.dwFlags && self.lpBandwidth == other.lpBandwidth && self.ddRVal == other.ddRVal && self.GetVideoPortBandwidth == other.GetVideoPortBandwidth
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::Eq for DD_GETVPORTBANDWIDTHDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
unsafe impl ::windows::runtime::Abi for DD_GETVPORTBANDWIDTHDATA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Graphics_DirectDraw")]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Graphics_DirectDraw`*"]
pub struct DD_GETVPORTCONNECTDATA {
    pub lpDD: *mut DD_DIRECTDRAW_LOCAL,
    pub dwPortId: u32,
    pub lpConnect: *mut super::super::Graphics::DirectDraw::DDVIDEOPORTCONNECT,
    pub dwNumEntries: u32,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub GetVideoPortConnectInfo: *mut ::std::ffi::c_void,
}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl DD_GETVPORTCONNECTDATA {}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl ::std::default::Default for DD_GETVPORTCONNECTDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl ::std::fmt::Debug for DD_GETVPORTCONNECTDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DD_GETVPORTCONNECTDATA").field("lpDD", &self.lpDD).field("dwPortId", &self.dwPortId).field("lpConnect", &self.lpConnect).field("dwNumEntries", &self.dwNumEntries).field("ddRVal", &self.ddRVal).field("GetVideoPortConnectInfo", &self.GetVideoPortConnectInfo).finish()
    }
}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl ::std::cmp::PartialEq for DD_GETVPORTCONNECTDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.dwPortId == other.dwPortId && self.lpConnect == other.lpConnect && self.dwNumEntries == other.dwNumEntries && self.ddRVal == other.ddRVal && self.GetVideoPortConnectInfo == other.GetVideoPortConnectInfo
    }
}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl ::std::cmp::Eq for DD_GETVPORTCONNECTDATA {}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
unsafe impl ::windows::runtime::Abi for DD_GETVPORTCONNECTDATA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_DirectDraw`*"]
pub struct DD_GETVPORTFIELDDATA {
    pub lpDD: *mut DD_DIRECTDRAW_LOCAL,
    pub lpVideoPort: *mut DD_VIDEOPORT_LOCAL,
    pub bField: super::super::Foundation::BOOL,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub GetVideoPortField: *mut ::std::ffi::c_void,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl DD_GETVPORTFIELDDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::default::Default for DD_GETVPORTFIELDDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::fmt::Debug for DD_GETVPORTFIELDDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DD_GETVPORTFIELDDATA").field("lpDD", &self.lpDD).field("lpVideoPort", &self.lpVideoPort).field("bField", &self.bField).field("ddRVal", &self.ddRVal).field("GetVideoPortField", &self.GetVideoPortField).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::PartialEq for DD_GETVPORTFIELDDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpVideoPort == other.lpVideoPort && self.bField == other.bField && self.ddRVal == other.ddRVal && self.GetVideoPortField == other.GetVideoPortField
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::Eq for DD_GETVPORTFIELDDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
unsafe impl ::windows::runtime::Abi for DD_GETVPORTFIELDDATA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_DisplayDevices`*"]
pub struct DD_GETVPORTFLIPSTATUSDATA {
    pub lpDD: *mut DD_DIRECTDRAW_LOCAL,
    pub fpSurface: usize,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub GetVideoPortFlipStatus: *mut ::std::ffi::c_void,
}
impl DD_GETVPORTFLIPSTATUSDATA {}
impl ::std::default::Default for DD_GETVPORTFLIPSTATUSDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DD_GETVPORTFLIPSTATUSDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DD_GETVPORTFLIPSTATUSDATA").field("lpDD", &self.lpDD).field("fpSurface", &self.fpSurface).field("ddRVal", &self.ddRVal).field("GetVideoPortFlipStatus", &self.GetVideoPortFlipStatus).finish()
    }
}
impl ::std::cmp::PartialEq for DD_GETVPORTFLIPSTATUSDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.fpSurface == other.fpSurface && self.ddRVal == other.ddRVal && self.GetVideoPortFlipStatus == other.GetVideoPortFlipStatus
    }
}
impl ::std::cmp::Eq for DD_GETVPORTFLIPSTATUSDATA {}
unsafe impl ::windows::runtime::Abi for DD_GETVPORTFLIPSTATUSDATA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_DirectDraw`*"]
pub struct DD_GETVPORTINPUTFORMATDATA {
    pub lpDD: *mut DD_DIRECTDRAW_LOCAL,
    pub lpVideoPort: *mut DD_VIDEOPORT_LOCAL,
    pub dwFlags: u32,
    pub lpddpfFormat: *mut super::super::Graphics::DirectDraw::DDPIXELFORMAT,
    pub dwNumFormats: u32,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub GetVideoPortInputFormats: *mut ::std::ffi::c_void,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl DD_GETVPORTINPUTFORMATDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::default::Default for DD_GETVPORTINPUTFORMATDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::fmt::Debug for DD_GETVPORTINPUTFORMATDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DD_GETVPORTINPUTFORMATDATA")
            .field("lpDD", &self.lpDD)
            .field("lpVideoPort", &self.lpVideoPort)
            .field("dwFlags", &self.dwFlags)
            .field("lpddpfFormat", &self.lpddpfFormat)
            .field("dwNumFormats", &self.dwNumFormats)
            .field("ddRVal", &self.ddRVal)
            .field("GetVideoPortInputFormats", &self.GetVideoPortInputFormats)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::PartialEq for DD_GETVPORTINPUTFORMATDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpVideoPort == other.lpVideoPort && self.dwFlags == other.dwFlags && self.lpddpfFormat == other.lpddpfFormat && self.dwNumFormats == other.dwNumFormats && self.ddRVal == other.ddRVal && self.GetVideoPortInputFormats == other.GetVideoPortInputFormats
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::Eq for DD_GETVPORTINPUTFORMATDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
unsafe impl ::windows::runtime::Abi for DD_GETVPORTINPUTFORMATDATA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_DirectDraw`*"]
pub struct DD_GETVPORTLINEDATA {
    pub lpDD: *mut DD_DIRECTDRAW_LOCAL,
    pub lpVideoPort: *mut DD_VIDEOPORT_LOCAL,
    pub dwLine: u32,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub GetVideoPortLine: *mut ::std::ffi::c_void,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl DD_GETVPORTLINEDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::default::Default for DD_GETVPORTLINEDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::fmt::Debug for DD_GETVPORTLINEDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DD_GETVPORTLINEDATA").field("lpDD", &self.lpDD).field("lpVideoPort", &self.lpVideoPort).field("dwLine", &self.dwLine).field("ddRVal", &self.ddRVal).field("GetVideoPortLine", &self.GetVideoPortLine).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::PartialEq for DD_GETVPORTLINEDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpVideoPort == other.lpVideoPort && self.dwLine == other.dwLine && self.ddRVal == other.ddRVal && self.GetVideoPortLine == other.GetVideoPortLine
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::Eq for DD_GETVPORTLINEDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
unsafe impl ::windows::runtime::Abi for DD_GETVPORTLINEDATA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_DirectDraw`*"]
pub struct DD_GETVPORTOUTPUTFORMATDATA {
    pub lpDD: *mut DD_DIRECTDRAW_LOCAL,
    pub lpVideoPort: *mut DD_VIDEOPORT_LOCAL,
    pub dwFlags: u32,
    pub lpddpfInputFormat: *mut super::super::Graphics::DirectDraw::DDPIXELFORMAT,
    pub lpddpfOutputFormats: *mut super::super::Graphics::DirectDraw::DDPIXELFORMAT,
    pub dwNumFormats: u32,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub GetVideoPortInputFormats: *mut ::std::ffi::c_void,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl DD_GETVPORTOUTPUTFORMATDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::default::Default for DD_GETVPORTOUTPUTFORMATDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::fmt::Debug for DD_GETVPORTOUTPUTFORMATDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DD_GETVPORTOUTPUTFORMATDATA")
            .field("lpDD", &self.lpDD)
            .field("lpVideoPort", &self.lpVideoPort)
            .field("dwFlags", &self.dwFlags)
            .field("lpddpfInputFormat", &self.lpddpfInputFormat)
            .field("lpddpfOutputFormats", &self.lpddpfOutputFormats)
            .field("dwNumFormats", &self.dwNumFormats)
            .field("ddRVal", &self.ddRVal)
            .field("GetVideoPortInputFormats", &self.GetVideoPortInputFormats)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::PartialEq for DD_GETVPORTOUTPUTFORMATDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpVideoPort == other.lpVideoPort && self.dwFlags == other.dwFlags && self.lpddpfInputFormat == other.lpddpfInputFormat && self.lpddpfOutputFormats == other.lpddpfOutputFormats && self.dwNumFormats == other.dwNumFormats && self.ddRVal == other.ddRVal && self.GetVideoPortInputFormats == other.GetVideoPortInputFormats
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::Eq for DD_GETVPORTOUTPUTFORMATDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
unsafe impl ::windows::runtime::Abi for DD_GETVPORTOUTPUTFORMATDATA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_DirectDraw`*"]
pub struct DD_GETVPORTSIGNALDATA {
    pub lpDD: *mut DD_DIRECTDRAW_LOCAL,
    pub lpVideoPort: *mut DD_VIDEOPORT_LOCAL,
    pub dwStatus: u32,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub GetVideoSignalStatus: *mut ::std::ffi::c_void,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl DD_GETVPORTSIGNALDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::default::Default for DD_GETVPORTSIGNALDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::fmt::Debug for DD_GETVPORTSIGNALDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DD_GETVPORTSIGNALDATA").field("lpDD", &self.lpDD).field("lpVideoPort", &self.lpVideoPort).field("dwStatus", &self.dwStatus).field("ddRVal", &self.ddRVal).field("GetVideoSignalStatus", &self.GetVideoSignalStatus).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::PartialEq for DD_GETVPORTSIGNALDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpVideoPort == other.lpVideoPort && self.dwStatus == other.dwStatus && self.ddRVal == other.ddRVal && self.GetVideoSignalStatus == other.GetVideoSignalStatus
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::Eq for DD_GETVPORTSIGNALDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
unsafe impl ::windows::runtime::Abi for DD_GETVPORTSIGNALDATA {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::clone::Clone for DD_HALINFO {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_DirectDraw`*"]
pub struct DD_HALINFO {
    pub dwSize: u32,
    pub vmiData: VIDEOMEMORYINFO,
    pub ddCaps: super::super::Graphics::DirectDraw::DDNTCORECAPS,
    pub GetDriverInfo: ::std::option::Option<PDD_GETDRIVERINFO>,
    pub dwFlags: u32,
    pub lpD3DGlobalDriverData: *mut ::std::ffi::c_void,
    pub lpD3DHALCallbacks: *mut ::std::ffi::c_void,
    pub lpD3DBufCallbacks: *mut DD_D3DBUFCALLBACKS,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl DD_HALINFO {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::default::Default for DD_HALINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::PartialEq for DD_HALINFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::Eq for DD_HALINFO {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
unsafe impl ::windows::runtime::Abi for DD_HALINFO {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_DirectDraw`*"]
pub struct DD_KERNELCALLBACKS {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub SyncSurfaceData: ::std::option::Option<PDD_KERNELCB_SYNCSURFACE>,
    pub SyncVideoPortData: ::std::option::Option<PDD_KERNELCB_SYNCVIDEOPORT>,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl DD_KERNELCALLBACKS {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::default::Default for DD_KERNELCALLBACKS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::fmt::Debug for DD_KERNELCALLBACKS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DD_KERNELCALLBACKS").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::PartialEq for DD_KERNELCALLBACKS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwFlags == other.dwFlags && self.SyncSurfaceData.map(|f| f as usize) == other.SyncSurfaceData.map(|f| f as usize) && self.SyncVideoPortData.map(|f| f as usize) == other.SyncVideoPortData.map(|f| f as usize)
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::Eq for DD_KERNELCALLBACKS {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
unsafe impl ::windows::runtime::Abi for DD_KERNELCALLBACKS {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_DirectDraw`*"]
pub struct DD_LOCKDATA {
    pub lpDD: *mut DD_DIRECTDRAW_GLOBAL,
    pub lpDDSurface: *mut DD_SURFACE_LOCAL,
    pub bHasRect: u32,
    pub rArea: super::super::Foundation::RECTL,
    pub lpSurfData: *mut ::std::ffi::c_void,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub Lock: *mut ::std::ffi::c_void,
    pub dwFlags: u32,
    pub fpProcess: usize,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl DD_LOCKDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::default::Default for DD_LOCKDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::fmt::Debug for DD_LOCKDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DD_LOCKDATA")
            .field("lpDD", &self.lpDD)
            .field("lpDDSurface", &self.lpDDSurface)
            .field("bHasRect", &self.bHasRect)
            .field("rArea", &self.rArea)
            .field("lpSurfData", &self.lpSurfData)
            .field("ddRVal", &self.ddRVal)
            .field("Lock", &self.Lock)
            .field("dwFlags", &self.dwFlags)
            .field("fpProcess", &self.fpProcess)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::PartialEq for DD_LOCKDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpDDSurface == other.lpDDSurface && self.bHasRect == other.bHasRect && self.rArea == other.rArea && self.lpSurfData == other.lpSurfData && self.ddRVal == other.ddRVal && self.Lock == other.Lock && self.dwFlags == other.dwFlags && self.fpProcess == other.fpProcess
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::Eq for DD_LOCKDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
unsafe impl ::windows::runtime::Abi for DD_LOCKDATA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`*"]
pub struct DD_MAPMEMORYDATA {
    pub lpDD: *mut DD_DIRECTDRAW_GLOBAL,
    pub bMap: super::super::Foundation::BOOL,
    pub hProcess: super::super::Foundation::HANDLE,
    pub fpProcess: usize,
    pub ddRVal: ::windows::runtime::HRESULT,
}
#[cfg(feature = "Win32_Foundation")]
impl DD_MAPMEMORYDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DD_MAPMEMORYDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for DD_MAPMEMORYDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DD_MAPMEMORYDATA").field("lpDD", &self.lpDD).field("bMap", &self.bMap).field("hProcess", &self.hProcess).field("fpProcess", &self.fpProcess).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DD_MAPMEMORYDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.bMap == other.bMap && self.hProcess == other.hProcess && self.fpProcess == other.fpProcess && self.ddRVal == other.ddRVal
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DD_MAPMEMORYDATA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DD_MAPMEMORYDATA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_DirectDraw`*"]
pub struct DD_MISCELLANEOUS2CALLBACKS {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub AlphaBlt: ::std::option::Option<super::super::Graphics::DirectDraw::PDD_ALPHABLT>,
    pub CreateSurfaceEx: ::std::option::Option<PDD_CREATESURFACEEX>,
    pub GetDriverState: ::std::option::Option<PDD_GETDRIVERSTATE>,
    pub DestroyDDLocal: ::std::option::Option<PDD_DESTROYDDLOCAL>,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl DD_MISCELLANEOUS2CALLBACKS {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::default::Default for DD_MISCELLANEOUS2CALLBACKS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::fmt::Debug for DD_MISCELLANEOUS2CALLBACKS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DD_MISCELLANEOUS2CALLBACKS").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::PartialEq for DD_MISCELLANEOUS2CALLBACKS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwFlags == other.dwFlags && self.AlphaBlt.map(|f| f as usize) == other.AlphaBlt.map(|f| f as usize) && self.CreateSurfaceEx.map(|f| f as usize) == other.CreateSurfaceEx.map(|f| f as usize) && self.GetDriverState.map(|f| f as usize) == other.GetDriverState.map(|f| f as usize) && self.DestroyDDLocal.map(|f| f as usize) == other.DestroyDDLocal.map(|f| f as usize)
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::Eq for DD_MISCELLANEOUS2CALLBACKS {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
unsafe impl ::windows::runtime::Abi for DD_MISCELLANEOUS2CALLBACKS {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Graphics_DirectDraw")]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Graphics_DirectDraw`*"]
pub struct DD_MISCELLANEOUSCALLBACKS {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub GetAvailDriverMemory: ::std::option::Option<PDD_GETAVAILDRIVERMEMORY>,
}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl DD_MISCELLANEOUSCALLBACKS {}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl ::std::default::Default for DD_MISCELLANEOUSCALLBACKS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl ::std::fmt::Debug for DD_MISCELLANEOUSCALLBACKS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DD_MISCELLANEOUSCALLBACKS").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).finish()
    }
}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl ::std::cmp::PartialEq for DD_MISCELLANEOUSCALLBACKS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwFlags == other.dwFlags && self.GetAvailDriverMemory.map(|f| f as usize) == other.GetAvailDriverMemory.map(|f| f as usize)
    }
}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl ::std::cmp::Eq for DD_MISCELLANEOUSCALLBACKS {}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
unsafe impl ::windows::runtime::Abi for DD_MISCELLANEOUSCALLBACKS {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Graphics_DirectDraw")]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Graphics_DirectDraw`*"]
pub struct DD_MORESURFACECAPS {
    pub dwSize: u32,
    pub ddsCapsMore: super::super::Graphics::DirectDraw::DDSCAPSEX,
    pub ddsExtendedHeapRestrictions: [DD_MORESURFACECAPS_0; 1],
}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl DD_MORESURFACECAPS {}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl ::std::default::Default for DD_MORESURFACECAPS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl ::std::cmp::PartialEq for DD_MORESURFACECAPS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl ::std::cmp::Eq for DD_MORESURFACECAPS {}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
unsafe impl ::windows::runtime::Abi for DD_MORESURFACECAPS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Graphics_DirectDraw")]
pub struct DD_MORESURFACECAPS_0 {
    pub ddsCapsEx: super::super::Graphics::DirectDraw::DDSCAPSEX,
    pub ddsCapsExAlt: super::super::Graphics::DirectDraw::DDSCAPSEX,
}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl DD_MORESURFACECAPS_0 {}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl ::std::default::Default for DD_MORESURFACECAPS_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl ::std::cmp::PartialEq for DD_MORESURFACECAPS_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl ::std::cmp::Eq for DD_MORESURFACECAPS_0 {}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
unsafe impl ::windows::runtime::Abi for DD_MORESURFACECAPS_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_DirectDraw`*"]
pub struct DD_MOTIONCOMPCALLBACKS {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub GetMoCompGuids: ::std::option::Option<PDD_MOCOMPCB_GETGUIDS>,
    pub GetMoCompFormats: ::std::option::Option<PDD_MOCOMPCB_GETFORMATS>,
    pub CreateMoComp: ::std::option::Option<PDD_MOCOMPCB_CREATE>,
    pub GetMoCompBuffInfo: ::std::option::Option<PDD_MOCOMPCB_GETCOMPBUFFINFO>,
    pub GetInternalMoCompInfo: ::std::option::Option<PDD_MOCOMPCB_GETINTERNALINFO>,
    pub BeginMoCompFrame: ::std::option::Option<PDD_MOCOMPCB_BEGINFRAME>,
    pub EndMoCompFrame: ::std::option::Option<PDD_MOCOMPCB_ENDFRAME>,
    pub RenderMoComp: ::std::option::Option<PDD_MOCOMPCB_RENDER>,
    pub QueryMoCompStatus: ::std::option::Option<PDD_MOCOMPCB_QUERYSTATUS>,
    pub DestroyMoComp: ::std::option::Option<PDD_MOCOMPCB_DESTROY>,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl DD_MOTIONCOMPCALLBACKS {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::default::Default for DD_MOTIONCOMPCALLBACKS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::fmt::Debug for DD_MOTIONCOMPCALLBACKS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DD_MOTIONCOMPCALLBACKS").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::PartialEq for DD_MOTIONCOMPCALLBACKS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
            && self.dwFlags == other.dwFlags
            && self.GetMoCompGuids.map(|f| f as usize) == other.GetMoCompGuids.map(|f| f as usize)
            && self.GetMoCompFormats.map(|f| f as usize) == other.GetMoCompFormats.map(|f| f as usize)
            && self.CreateMoComp.map(|f| f as usize) == other.CreateMoComp.map(|f| f as usize)
            && self.GetMoCompBuffInfo.map(|f| f as usize) == other.GetMoCompBuffInfo.map(|f| f as usize)
            && self.GetInternalMoCompInfo.map(|f| f as usize) == other.GetInternalMoCompInfo.map(|f| f as usize)
            && self.BeginMoCompFrame.map(|f| f as usize) == other.BeginMoCompFrame.map(|f| f as usize)
            && self.EndMoCompFrame.map(|f| f as usize) == other.EndMoCompFrame.map(|f| f as usize)
            && self.RenderMoComp.map(|f| f as usize) == other.RenderMoComp.map(|f| f as usize)
            && self.QueryMoCompStatus.map(|f| f as usize) == other.QueryMoCompStatus.map(|f| f as usize)
            && self.DestroyMoComp.map(|f| f as usize) == other.DestroyMoComp.map(|f| f as usize)
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::Eq for DD_MOTIONCOMPCALLBACKS {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
unsafe impl ::windows::runtime::Abi for DD_MOTIONCOMPCALLBACKS {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Graphics_DirectDraw")]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Graphics_DirectDraw`*"]
pub struct DD_MOTIONCOMP_LOCAL {
    pub lpDD: *mut DD_DIRECTDRAW_LOCAL,
    pub guid: ::windows::runtime::GUID,
    pub dwUncompWidth: u32,
    pub dwUncompHeight: u32,
    pub ddUncompPixelFormat: super::super::Graphics::DirectDraw::DDPIXELFORMAT,
    pub dwDriverReserved1: u32,
    pub dwDriverReserved2: u32,
    pub dwDriverReserved3: u32,
    pub lpDriverReserved1: *mut ::std::ffi::c_void,
    pub lpDriverReserved2: *mut ::std::ffi::c_void,
    pub lpDriverReserved3: *mut ::std::ffi::c_void,
}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl DD_MOTIONCOMP_LOCAL {}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl ::std::default::Default for DD_MOTIONCOMP_LOCAL {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl ::std::cmp::PartialEq for DD_MOTIONCOMP_LOCAL {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl ::std::cmp::Eq for DD_MOTIONCOMP_LOCAL {}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
unsafe impl ::windows::runtime::Abi for DD_MOTIONCOMP_LOCAL {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_DisplayDevices`*"]
pub struct DD_NONLOCALVIDMEMCAPS {
    pub dwSize: u32,
    pub dwNLVBCaps: u32,
    pub dwNLVBCaps2: u32,
    pub dwNLVBCKeyCaps: u32,
    pub dwNLVBFXCaps: u32,
    pub dwNLVBRops: [u32; 8],
}
impl DD_NONLOCALVIDMEMCAPS {}
impl ::std::default::Default for DD_NONLOCALVIDMEMCAPS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DD_NONLOCALVIDMEMCAPS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DD_NONLOCALVIDMEMCAPS").field("dwSize", &self.dwSize).field("dwNLVBCaps", &self.dwNLVBCaps).field("dwNLVBCaps2", &self.dwNLVBCaps2).field("dwNLVBCKeyCaps", &self.dwNLVBCKeyCaps).field("dwNLVBFXCaps", &self.dwNLVBFXCaps).field("dwNLVBRops", &self.dwNLVBRops).finish()
    }
}
impl ::std::cmp::PartialEq for DD_NONLOCALVIDMEMCAPS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwNLVBCaps == other.dwNLVBCaps && self.dwNLVBCaps2 == other.dwNLVBCaps2 && self.dwNLVBCKeyCaps == other.dwNLVBCKeyCaps && self.dwNLVBFXCaps == other.dwNLVBFXCaps && self.dwNLVBRops == other.dwNLVBRops
    }
}
impl ::std::cmp::Eq for DD_NONLOCALVIDMEMCAPS {}
unsafe impl ::windows::runtime::Abi for DD_NONLOCALVIDMEMCAPS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_DirectDraw`*"]
pub struct DD_NTCALLBACKS {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub FreeDriverMemory: ::std::option::Option<PDD_FREEDRIVERMEMORY>,
    pub SetExclusiveMode: ::std::option::Option<PDD_SETEXCLUSIVEMODE>,
    pub FlipToGDISurface: ::std::option::Option<PDD_FLIPTOGDISURFACE>,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl DD_NTCALLBACKS {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::default::Default for DD_NTCALLBACKS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::fmt::Debug for DD_NTCALLBACKS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DD_NTCALLBACKS").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::PartialEq for DD_NTCALLBACKS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwFlags == other.dwFlags && self.FreeDriverMemory.map(|f| f as usize) == other.FreeDriverMemory.map(|f| f as usize) && self.SetExclusiveMode.map(|f| f as usize) == other.SetExclusiveMode.map(|f| f as usize) && self.FlipToGDISurface.map(|f| f as usize) == other.FlipToGDISurface.map(|f| f as usize)
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::Eq for DD_NTCALLBACKS {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
unsafe impl ::windows::runtime::Abi for DD_NTCALLBACKS {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_DisplayDevices`*"]
pub struct DD_NTPRIVATEDRIVERCAPS {
    pub dwSize: u32,
    pub dwPrivateCaps: u32,
}
impl DD_NTPRIVATEDRIVERCAPS {}
impl ::std::default::Default for DD_NTPRIVATEDRIVERCAPS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DD_NTPRIVATEDRIVERCAPS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DD_NTPRIVATEDRIVERCAPS").field("dwSize", &self.dwSize).field("dwPrivateCaps", &self.dwPrivateCaps).finish()
    }
}
impl ::std::cmp::PartialEq for DD_NTPRIVATEDRIVERCAPS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwPrivateCaps == other.dwPrivateCaps
    }
}
impl ::std::cmp::Eq for DD_NTPRIVATEDRIVERCAPS {}
unsafe impl ::windows::runtime::Abi for DD_NTPRIVATEDRIVERCAPS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Graphics_Gdi`*"]
pub struct DD_PALETTECALLBACKS {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub DestroyPalette: ::std::option::Option<PDD_PALCB_DESTROYPALETTE>,
    pub SetEntries: ::std::option::Option<PDD_PALCB_SETENTRIES>,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl DD_PALETTECALLBACKS {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::std::default::Default for DD_PALETTECALLBACKS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::std::fmt::Debug for DD_PALETTECALLBACKS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DD_PALETTECALLBACKS").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::std::cmp::PartialEq for DD_PALETTECALLBACKS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwFlags == other.dwFlags && self.DestroyPalette.map(|f| f as usize) == other.DestroyPalette.map(|f| f as usize) && self.SetEntries.map(|f| f as usize) == other.SetEntries.map(|f| f as usize)
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::std::cmp::Eq for DD_PALETTECALLBACKS {}
#[cfg(feature = "Win32_Graphics_Gdi")]
unsafe impl ::windows::runtime::Abi for DD_PALETTECALLBACKS {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_DisplayDevices`*"]
pub struct DD_PALETTE_GLOBAL {
    pub dwReserved1: usize,
}
impl DD_PALETTE_GLOBAL {}
impl ::std::default::Default for DD_PALETTE_GLOBAL {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DD_PALETTE_GLOBAL {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DD_PALETTE_GLOBAL").field("dwReserved1", &self.dwReserved1).finish()
    }
}
impl ::std::cmp::PartialEq for DD_PALETTE_GLOBAL {
    fn eq(&self, other: &Self) -> bool {
        self.dwReserved1 == other.dwReserved1
    }
}
impl ::std::cmp::Eq for DD_PALETTE_GLOBAL {}
unsafe impl ::windows::runtime::Abi for DD_PALETTE_GLOBAL {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_DisplayDevices`*"]
pub struct DD_PALETTE_LOCAL {
    pub dwReserved0: u32,
    pub dwReserved1: usize,
}
impl DD_PALETTE_LOCAL {}
impl ::std::default::Default for DD_PALETTE_LOCAL {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DD_PALETTE_LOCAL {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DD_PALETTE_LOCAL").field("dwReserved0", &self.dwReserved0).field("dwReserved1", &self.dwReserved1).finish()
    }
}
impl ::std::cmp::PartialEq for DD_PALETTE_LOCAL {
    fn eq(&self, other: &Self) -> bool {
        self.dwReserved0 == other.dwReserved0 && self.dwReserved1 == other.dwReserved1
    }
}
impl ::std::cmp::Eq for DD_PALETTE_LOCAL {}
unsafe impl ::windows::runtime::Abi for DD_PALETTE_LOCAL {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_DirectDraw`*"]
pub struct DD_QUERYMOCOMPSTATUSDATA {
    pub lpDD: *mut DD_DIRECTDRAW_LOCAL,
    pub lpMoComp: *mut DD_MOTIONCOMP_LOCAL,
    pub lpSurface: *mut DD_SURFACE_LOCAL,
    pub dwFlags: u32,
    pub ddRVal: ::windows::runtime::HRESULT,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl DD_QUERYMOCOMPSTATUSDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::default::Default for DD_QUERYMOCOMPSTATUSDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::fmt::Debug for DD_QUERYMOCOMPSTATUSDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DD_QUERYMOCOMPSTATUSDATA").field("lpDD", &self.lpDD).field("lpMoComp", &self.lpMoComp).field("lpSurface", &self.lpSurface).field("dwFlags", &self.dwFlags).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::PartialEq for DD_QUERYMOCOMPSTATUSDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpMoComp == other.lpMoComp && self.lpSurface == other.lpSurface && self.dwFlags == other.dwFlags && self.ddRVal == other.ddRVal
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::Eq for DD_QUERYMOCOMPSTATUSDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
unsafe impl ::windows::runtime::Abi for DD_QUERYMOCOMPSTATUSDATA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_DirectDraw`*"]
pub struct DD_RENDERMOCOMPDATA {
    pub lpDD: *mut DD_DIRECTDRAW_LOCAL,
    pub lpMoComp: *mut DD_MOTIONCOMP_LOCAL,
    pub dwNumBuffers: u32,
    pub lpBufferInfo: *mut DDMOCOMPBUFFERINFO,
    pub dwFunction: u32,
    pub lpInputData: *mut ::std::ffi::c_void,
    pub dwInputDataSize: u32,
    pub lpOutputData: *mut ::std::ffi::c_void,
    pub dwOutputDataSize: u32,
    pub ddRVal: ::windows::runtime::HRESULT,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl DD_RENDERMOCOMPDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::default::Default for DD_RENDERMOCOMPDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::fmt::Debug for DD_RENDERMOCOMPDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DD_RENDERMOCOMPDATA")
            .field("lpDD", &self.lpDD)
            .field("lpMoComp", &self.lpMoComp)
            .field("dwNumBuffers", &self.dwNumBuffers)
            .field("lpBufferInfo", &self.lpBufferInfo)
            .field("dwFunction", &self.dwFunction)
            .field("lpInputData", &self.lpInputData)
            .field("dwInputDataSize", &self.dwInputDataSize)
            .field("lpOutputData", &self.lpOutputData)
            .field("dwOutputDataSize", &self.dwOutputDataSize)
            .field("ddRVal", &self.ddRVal)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::PartialEq for DD_RENDERMOCOMPDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpMoComp == other.lpMoComp && self.dwNumBuffers == other.dwNumBuffers && self.lpBufferInfo == other.lpBufferInfo && self.dwFunction == other.dwFunction && self.lpInputData == other.lpInputData && self.dwInputDataSize == other.dwInputDataSize && self.lpOutputData == other.lpOutputData && self.dwOutputDataSize == other.dwOutputDataSize && self.ddRVal == other.ddRVal
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::Eq for DD_RENDERMOCOMPDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
unsafe impl ::windows::runtime::Abi for DD_RENDERMOCOMPDATA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_DirectDraw`*"]
pub struct DD_SETCOLORKEYDATA {
    pub lpDD: *mut DD_DIRECTDRAW_GLOBAL,
    pub lpDDSurface: *mut DD_SURFACE_LOCAL,
    pub dwFlags: u32,
    pub ckNew: super::super::Graphics::DirectDraw::DDCOLORKEY,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub SetColorKey: *mut ::std::ffi::c_void,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl DD_SETCOLORKEYDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::default::Default for DD_SETCOLORKEYDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::fmt::Debug for DD_SETCOLORKEYDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DD_SETCOLORKEYDATA").field("lpDD", &self.lpDD).field("lpDDSurface", &self.lpDDSurface).field("dwFlags", &self.dwFlags).field("ckNew", &self.ckNew).field("ddRVal", &self.ddRVal).field("SetColorKey", &self.SetColorKey).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::PartialEq for DD_SETCOLORKEYDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpDDSurface == other.lpDDSurface && self.dwFlags == other.dwFlags && self.ckNew == other.ckNew && self.ddRVal == other.ddRVal && self.SetColorKey == other.SetColorKey
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::Eq for DD_SETCOLORKEYDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
unsafe impl ::windows::runtime::Abi for DD_SETCOLORKEYDATA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Graphics_Gdi`*"]
pub struct DD_SETENTRIESDATA {
    pub lpDD: *mut DD_DIRECTDRAW_GLOBAL,
    pub lpDDPalette: *mut DD_PALETTE_GLOBAL,
    pub dwBase: u32,
    pub dwNumEntries: u32,
    pub lpEntries: *mut super::super::Graphics::Gdi::PALETTEENTRY,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub SetEntries: *mut ::std::ffi::c_void,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl DD_SETENTRIESDATA {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::std::default::Default for DD_SETENTRIESDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::std::fmt::Debug for DD_SETENTRIESDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DD_SETENTRIESDATA").field("lpDD", &self.lpDD).field("lpDDPalette", &self.lpDDPalette).field("dwBase", &self.dwBase).field("dwNumEntries", &self.dwNumEntries).field("lpEntries", &self.lpEntries).field("ddRVal", &self.ddRVal).field("SetEntries", &self.SetEntries).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::std::cmp::PartialEq for DD_SETENTRIESDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpDDPalette == other.lpDDPalette && self.dwBase == other.dwBase && self.dwNumEntries == other.dwNumEntries && self.lpEntries == other.lpEntries && self.ddRVal == other.ddRVal && self.SetEntries == other.SetEntries
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::std::cmp::Eq for DD_SETENTRIESDATA {}
#[cfg(feature = "Win32_Graphics_Gdi")]
unsafe impl ::windows::runtime::Abi for DD_SETENTRIESDATA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_DisplayDevices`*"]
pub struct DD_SETEXCLUSIVEMODEDATA {
    pub lpDD: *mut DD_DIRECTDRAW_GLOBAL,
    pub dwEnterExcl: u32,
    pub dwReserved: u32,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub SetExclusiveMode: *mut ::std::ffi::c_void,
}
impl DD_SETEXCLUSIVEMODEDATA {}
impl ::std::default::Default for DD_SETEXCLUSIVEMODEDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DD_SETEXCLUSIVEMODEDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DD_SETEXCLUSIVEMODEDATA").field("lpDD", &self.lpDD).field("dwEnterExcl", &self.dwEnterExcl).field("dwReserved", &self.dwReserved).field("ddRVal", &self.ddRVal).field("SetExclusiveMode", &self.SetExclusiveMode).finish()
    }
}
impl ::std::cmp::PartialEq for DD_SETEXCLUSIVEMODEDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.dwEnterExcl == other.dwEnterExcl && self.dwReserved == other.dwReserved && self.ddRVal == other.ddRVal && self.SetExclusiveMode == other.SetExclusiveMode
    }
}
impl ::std::cmp::Eq for DD_SETEXCLUSIVEMODEDATA {}
unsafe impl ::windows::runtime::Abi for DD_SETEXCLUSIVEMODEDATA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_DirectDraw`*"]
pub struct DD_SETOVERLAYPOSITIONDATA {
    pub lpDD: *mut DD_DIRECTDRAW_GLOBAL,
    pub lpDDSrcSurface: *mut DD_SURFACE_LOCAL,
    pub lpDDDestSurface: *mut DD_SURFACE_LOCAL,
    pub lXPos: i32,
    pub lYPos: i32,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub SetOverlayPosition: *mut ::std::ffi::c_void,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl DD_SETOVERLAYPOSITIONDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::default::Default for DD_SETOVERLAYPOSITIONDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::fmt::Debug for DD_SETOVERLAYPOSITIONDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DD_SETOVERLAYPOSITIONDATA")
            .field("lpDD", &self.lpDD)
            .field("lpDDSrcSurface", &self.lpDDSrcSurface)
            .field("lpDDDestSurface", &self.lpDDDestSurface)
            .field("lXPos", &self.lXPos)
            .field("lYPos", &self.lYPos)
            .field("ddRVal", &self.ddRVal)
            .field("SetOverlayPosition", &self.SetOverlayPosition)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::PartialEq for DD_SETOVERLAYPOSITIONDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpDDSrcSurface == other.lpDDSrcSurface && self.lpDDDestSurface == other.lpDDDestSurface && self.lXPos == other.lXPos && self.lYPos == other.lYPos && self.ddRVal == other.ddRVal && self.SetOverlayPosition == other.SetOverlayPosition
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::Eq for DD_SETOVERLAYPOSITIONDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
unsafe impl ::windows::runtime::Abi for DD_SETOVERLAYPOSITIONDATA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_DirectDraw`*"]
pub struct DD_SETPALETTEDATA {
    pub lpDD: *mut DD_DIRECTDRAW_GLOBAL,
    pub lpDDSurface: *mut DD_SURFACE_LOCAL,
    pub lpDDPalette: *mut DD_PALETTE_GLOBAL,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub SetPalette: *mut ::std::ffi::c_void,
    pub Attach: super::super::Foundation::BOOL,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl DD_SETPALETTEDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::default::Default for DD_SETPALETTEDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::fmt::Debug for DD_SETPALETTEDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DD_SETPALETTEDATA").field("lpDD", &self.lpDD).field("lpDDSurface", &self.lpDDSurface).field("lpDDPalette", &self.lpDDPalette).field("ddRVal", &self.ddRVal).field("SetPalette", &self.SetPalette).field("Attach", &self.Attach).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::PartialEq for DD_SETPALETTEDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpDDSurface == other.lpDDSurface && self.lpDDPalette == other.lpDDPalette && self.ddRVal == other.ddRVal && self.SetPalette == other.SetPalette && self.Attach == other.Attach
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::Eq for DD_SETPALETTEDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
unsafe impl ::windows::runtime::Abi for DD_SETPALETTEDATA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`*"]
pub struct DD_STEREOMODE {
    pub dwSize: u32,
    pub dwHeight: u32,
    pub dwWidth: u32,
    pub dwBpp: u32,
    pub dwRefreshRate: u32,
    pub bSupported: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl DD_STEREOMODE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DD_STEREOMODE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for DD_STEREOMODE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DD_STEREOMODE").field("dwSize", &self.dwSize).field("dwHeight", &self.dwHeight).field("dwWidth", &self.dwWidth).field("dwBpp", &self.dwBpp).field("dwRefreshRate", &self.dwRefreshRate).field("bSupported", &self.bSupported).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DD_STEREOMODE {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwHeight == other.dwHeight && self.dwWidth == other.dwWidth && self.dwBpp == other.dwBpp && self.dwRefreshRate == other.dwRefreshRate && self.bSupported == other.bSupported
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DD_STEREOMODE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DD_STEREOMODE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_DirectDraw`*"]
pub struct DD_SURFACECALLBACKS {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub DestroySurface: ::std::option::Option<PDD_SURFCB_DESTROYSURFACE>,
    pub Flip: ::std::option::Option<PDD_SURFCB_FLIP>,
    pub SetClipList: ::std::option::Option<super::super::Graphics::DirectDraw::PDD_SURFCB_SETCLIPLIST>,
    pub Lock: ::std::option::Option<PDD_SURFCB_LOCK>,
    pub Unlock: ::std::option::Option<PDD_SURFCB_UNLOCK>,
    pub Blt: ::std::option::Option<PDD_SURFCB_BLT>,
    pub SetColorKey: ::std::option::Option<PDD_SURFCB_SETCOLORKEY>,
    pub AddAttachedSurface: ::std::option::Option<PDD_SURFCB_ADDATTACHEDSURFACE>,
    pub GetBltStatus: ::std::option::Option<PDD_SURFCB_GETBLTSTATUS>,
    pub GetFlipStatus: ::std::option::Option<PDD_SURFCB_GETFLIPSTATUS>,
    pub UpdateOverlay: ::std::option::Option<PDD_SURFCB_UPDATEOVERLAY>,
    pub SetOverlayPosition: ::std::option::Option<PDD_SURFCB_SETOVERLAYPOSITION>,
    pub reserved4: *mut ::std::ffi::c_void,
    pub SetPalette: ::std::option::Option<PDD_SURFCB_SETPALETTE>,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl DD_SURFACECALLBACKS {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::default::Default for DD_SURFACECALLBACKS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::fmt::Debug for DD_SURFACECALLBACKS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DD_SURFACECALLBACKS").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).field("reserved4", &self.reserved4).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::PartialEq for DD_SURFACECALLBACKS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
            && self.dwFlags == other.dwFlags
            && self.DestroySurface.map(|f| f as usize) == other.DestroySurface.map(|f| f as usize)
            && self.Flip.map(|f| f as usize) == other.Flip.map(|f| f as usize)
            && self.SetClipList.map(|f| f as usize) == other.SetClipList.map(|f| f as usize)
            && self.Lock.map(|f| f as usize) == other.Lock.map(|f| f as usize)
            && self.Unlock.map(|f| f as usize) == other.Unlock.map(|f| f as usize)
            && self.Blt.map(|f| f as usize) == other.Blt.map(|f| f as usize)
            && self.SetColorKey.map(|f| f as usize) == other.SetColorKey.map(|f| f as usize)
            && self.AddAttachedSurface.map(|f| f as usize) == other.AddAttachedSurface.map(|f| f as usize)
            && self.GetBltStatus.map(|f| f as usize) == other.GetBltStatus.map(|f| f as usize)
            && self.GetFlipStatus.map(|f| f as usize) == other.GetFlipStatus.map(|f| f as usize)
            && self.UpdateOverlay.map(|f| f as usize) == other.UpdateOverlay.map(|f| f as usize)
            && self.SetOverlayPosition.map(|f| f as usize) == other.SetOverlayPosition.map(|f| f as usize)
            && self.reserved4 == other.reserved4
            && self.SetPalette.map(|f| f as usize) == other.SetPalette.map(|f| f as usize)
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::Eq for DD_SURFACECALLBACKS {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
unsafe impl ::windows::runtime::Abi for DD_SURFACECALLBACKS {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_DirectDraw`*"]
pub struct DD_SURFACE_GLOBAL {
    pub Anonymous1: DD_SURFACE_GLOBAL_0,
    pub Anonymous2: DD_SURFACE_GLOBAL_1,
    pub fpVidMem: usize,
    pub Anonymous3: DD_SURFACE_GLOBAL_2,
    pub yHint: i32,
    pub xHint: i32,
    pub wHeight: u32,
    pub wWidth: u32,
    pub dwReserved1: usize,
    pub ddpfSurface: super::super::Graphics::DirectDraw::DDPIXELFORMAT,
    pub fpHeapOffset: usize,
    pub hCreatorProcess: super::super::Foundation::HANDLE,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl DD_SURFACE_GLOBAL {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::default::Default for DD_SURFACE_GLOBAL {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::PartialEq for DD_SURFACE_GLOBAL {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::Eq for DD_SURFACE_GLOBAL {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
unsafe impl ::windows::runtime::Abi for DD_SURFACE_GLOBAL {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
pub union DD_SURFACE_GLOBAL_0 {
    pub dwBlockSizeY: u32,
    pub lSlicePitch: i32,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl DD_SURFACE_GLOBAL_0 {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::default::Default for DD_SURFACE_GLOBAL_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::PartialEq for DD_SURFACE_GLOBAL_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::Eq for DD_SURFACE_GLOBAL_0 {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
unsafe impl ::windows::runtime::Abi for DD_SURFACE_GLOBAL_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
pub union DD_SURFACE_GLOBAL_1 {
    pub lpVidMemHeap: *mut VIDEOMEMORY,
    pub dwBlockSizeX: u32,
    pub dwUserMemSize: u32,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl DD_SURFACE_GLOBAL_1 {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::default::Default for DD_SURFACE_GLOBAL_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::PartialEq for DD_SURFACE_GLOBAL_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::Eq for DD_SURFACE_GLOBAL_1 {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
unsafe impl ::windows::runtime::Abi for DD_SURFACE_GLOBAL_1 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
pub union DD_SURFACE_GLOBAL_2 {
    pub lPitch: i32,
    pub dwLinearSize: u32,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl DD_SURFACE_GLOBAL_2 {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::default::Default for DD_SURFACE_GLOBAL_2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::PartialEq for DD_SURFACE_GLOBAL_2 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::Eq for DD_SURFACE_GLOBAL_2 {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
unsafe impl ::windows::runtime::Abi for DD_SURFACE_GLOBAL_2 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_DirectDraw`*"]
pub struct DD_SURFACE_INT {
    pub lpLcl: *mut DD_SURFACE_LOCAL,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl DD_SURFACE_INT {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::default::Default for DD_SURFACE_INT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::fmt::Debug for DD_SURFACE_INT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DD_SURFACE_INT").field("lpLcl", &self.lpLcl).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::PartialEq for DD_SURFACE_INT {
    fn eq(&self, other: &Self) -> bool {
        self.lpLcl == other.lpLcl
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::Eq for DD_SURFACE_INT {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
unsafe impl ::windows::runtime::Abi for DD_SURFACE_INT {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_DirectDraw`*"]
pub struct DD_SURFACE_LOCAL {
    pub lpGbl: *mut DD_SURFACE_GLOBAL,
    pub dwFlags: u32,
    pub ddsCaps: super::super::Graphics::DirectDraw::DDSCAPS,
    pub dwReserved1: usize,
    pub Anonymous1: DD_SURFACE_LOCAL_0,
    pub Anonymous2: DD_SURFACE_LOCAL_1,
    pub lpSurfMore: *mut DD_SURFACE_MORE,
    pub lpAttachList: *mut DD_ATTACHLIST,
    pub lpAttachListFrom: *mut DD_ATTACHLIST,
    pub rcOverlaySrc: super::super::Foundation::RECT,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl DD_SURFACE_LOCAL {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::default::Default for DD_SURFACE_LOCAL {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::PartialEq for DD_SURFACE_LOCAL {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::Eq for DD_SURFACE_LOCAL {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
unsafe impl ::windows::runtime::Abi for DD_SURFACE_LOCAL {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
pub union DD_SURFACE_LOCAL_0 {
    pub ddckCKSrcOverlay: super::super::Graphics::DirectDraw::DDCOLORKEY,
    pub ddckCKSrcBlt: super::super::Graphics::DirectDraw::DDCOLORKEY,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl DD_SURFACE_LOCAL_0 {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::default::Default for DD_SURFACE_LOCAL_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::PartialEq for DD_SURFACE_LOCAL_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::Eq for DD_SURFACE_LOCAL_0 {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
unsafe impl ::windows::runtime::Abi for DD_SURFACE_LOCAL_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
pub union DD_SURFACE_LOCAL_1 {
    pub ddckCKDestOverlay: super::super::Graphics::DirectDraw::DDCOLORKEY,
    pub ddckCKDestBlt: super::super::Graphics::DirectDraw::DDCOLORKEY,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl DD_SURFACE_LOCAL_1 {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::default::Default for DD_SURFACE_LOCAL_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::PartialEq for DD_SURFACE_LOCAL_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::Eq for DD_SURFACE_LOCAL_1 {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
unsafe impl ::windows::runtime::Abi for DD_SURFACE_LOCAL_1 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_DirectDraw`*"]
pub struct DD_SURFACE_MORE {
    pub dwMipMapCount: u32,
    pub lpVideoPort: *mut DD_VIDEOPORT_LOCAL,
    pub dwOverlayFlags: u32,
    pub ddsCapsEx: super::super::Graphics::DirectDraw::DDSCAPSEX,
    pub dwSurfaceHandle: u32,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl DD_SURFACE_MORE {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::default::Default for DD_SURFACE_MORE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::PartialEq for DD_SURFACE_MORE {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::Eq for DD_SURFACE_MORE {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
unsafe impl ::windows::runtime::Abi for DD_SURFACE_MORE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_DirectDraw`*"]
pub struct DD_SYNCSURFACEDATA {
    pub lpDD: *mut DD_DIRECTDRAW_LOCAL,
    pub lpDDSurface: *mut DD_SURFACE_LOCAL,
    pub dwSurfaceOffset: u32,
    pub fpLockPtr: usize,
    pub lPitch: i32,
    pub dwOverlayOffset: u32,
    pub dwDriverReserved1: u32,
    pub dwDriverReserved2: u32,
    pub dwDriverReserved3: u32,
    pub dwDriverReserved4: u32,
    pub ddRVal: ::windows::runtime::HRESULT,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl DD_SYNCSURFACEDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::default::Default for DD_SYNCSURFACEDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::fmt::Debug for DD_SYNCSURFACEDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DD_SYNCSURFACEDATA")
            .field("lpDD", &self.lpDD)
            .field("lpDDSurface", &self.lpDDSurface)
            .field("dwSurfaceOffset", &self.dwSurfaceOffset)
            .field("fpLockPtr", &self.fpLockPtr)
            .field("lPitch", &self.lPitch)
            .field("dwOverlayOffset", &self.dwOverlayOffset)
            .field("dwDriverReserved1", &self.dwDriverReserved1)
            .field("dwDriverReserved2", &self.dwDriverReserved2)
            .field("dwDriverReserved3", &self.dwDriverReserved3)
            .field("dwDriverReserved4", &self.dwDriverReserved4)
            .field("ddRVal", &self.ddRVal)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::PartialEq for DD_SYNCSURFACEDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpDDSurface == other.lpDDSurface && self.dwSurfaceOffset == other.dwSurfaceOffset && self.fpLockPtr == other.fpLockPtr && self.lPitch == other.lPitch && self.dwOverlayOffset == other.dwOverlayOffset && self.dwDriverReserved1 == other.dwDriverReserved1 && self.dwDriverReserved2 == other.dwDriverReserved2 && self.dwDriverReserved3 == other.dwDriverReserved3 && self.dwDriverReserved4 == other.dwDriverReserved4 && self.ddRVal == other.ddRVal
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::Eq for DD_SYNCSURFACEDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
unsafe impl ::windows::runtime::Abi for DD_SYNCSURFACEDATA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_DirectDraw`*"]
pub struct DD_SYNCVIDEOPORTDATA {
    pub lpDD: *mut DD_DIRECTDRAW_LOCAL,
    pub lpVideoPort: *mut DD_VIDEOPORT_LOCAL,
    pub dwOriginOffset: u32,
    pub dwHeight: u32,
    pub dwVBIHeight: u32,
    pub dwDriverReserved1: u32,
    pub dwDriverReserved2: u32,
    pub dwDriverReserved3: u32,
    pub ddRVal: ::windows::runtime::HRESULT,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl DD_SYNCVIDEOPORTDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::default::Default for DD_SYNCVIDEOPORTDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::fmt::Debug for DD_SYNCVIDEOPORTDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DD_SYNCVIDEOPORTDATA")
            .field("lpDD", &self.lpDD)
            .field("lpVideoPort", &self.lpVideoPort)
            .field("dwOriginOffset", &self.dwOriginOffset)
            .field("dwHeight", &self.dwHeight)
            .field("dwVBIHeight", &self.dwVBIHeight)
            .field("dwDriverReserved1", &self.dwDriverReserved1)
            .field("dwDriverReserved2", &self.dwDriverReserved2)
            .field("dwDriverReserved3", &self.dwDriverReserved3)
            .field("ddRVal", &self.ddRVal)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::PartialEq for DD_SYNCVIDEOPORTDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpVideoPort == other.lpVideoPort && self.dwOriginOffset == other.dwOriginOffset && self.dwHeight == other.dwHeight && self.dwVBIHeight == other.dwVBIHeight && self.dwDriverReserved1 == other.dwDriverReserved1 && self.dwDriverReserved2 == other.dwDriverReserved2 && self.dwDriverReserved3 == other.dwDriverReserved3 && self.ddRVal == other.ddRVal
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::Eq for DD_SYNCVIDEOPORTDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
unsafe impl ::windows::runtime::Abi for DD_SYNCVIDEOPORTDATA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_DirectDraw`*"]
pub struct DD_UNLOCKDATA {
    pub lpDD: *mut DD_DIRECTDRAW_GLOBAL,
    pub lpDDSurface: *mut DD_SURFACE_LOCAL,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub Unlock: *mut ::std::ffi::c_void,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl DD_UNLOCKDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::default::Default for DD_UNLOCKDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::fmt::Debug for DD_UNLOCKDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DD_UNLOCKDATA").field("lpDD", &self.lpDD).field("lpDDSurface", &self.lpDDSurface).field("ddRVal", &self.ddRVal).field("Unlock", &self.Unlock).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::PartialEq for DD_UNLOCKDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpDDSurface == other.lpDDSurface && self.ddRVal == other.ddRVal && self.Unlock == other.Unlock
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::Eq for DD_UNLOCKDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
unsafe impl ::windows::runtime::Abi for DD_UNLOCKDATA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_DisplayDevices`*"]
pub struct DD_UPDATENONLOCALHEAPDATA {
    pub lpDD: *mut DD_DIRECTDRAW_GLOBAL,
    pub dwHeap: u32,
    pub fpGARTLin: usize,
    pub fpGARTDev: usize,
    pub ulPolicyMaxBytes: usize,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub UpdateNonLocalHeap: *mut ::std::ffi::c_void,
}
impl DD_UPDATENONLOCALHEAPDATA {}
impl ::std::default::Default for DD_UPDATENONLOCALHEAPDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DD_UPDATENONLOCALHEAPDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DD_UPDATENONLOCALHEAPDATA")
            .field("lpDD", &self.lpDD)
            .field("dwHeap", &self.dwHeap)
            .field("fpGARTLin", &self.fpGARTLin)
            .field("fpGARTDev", &self.fpGARTDev)
            .field("ulPolicyMaxBytes", &self.ulPolicyMaxBytes)
            .field("ddRVal", &self.ddRVal)
            .field("UpdateNonLocalHeap", &self.UpdateNonLocalHeap)
            .finish()
    }
}
impl ::std::cmp::PartialEq for DD_UPDATENONLOCALHEAPDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.dwHeap == other.dwHeap && self.fpGARTLin == other.fpGARTLin && self.fpGARTDev == other.fpGARTDev && self.ulPolicyMaxBytes == other.ulPolicyMaxBytes && self.ddRVal == other.ddRVal && self.UpdateNonLocalHeap == other.UpdateNonLocalHeap
    }
}
impl ::std::cmp::Eq for DD_UPDATENONLOCALHEAPDATA {}
unsafe impl ::windows::runtime::Abi for DD_UPDATENONLOCALHEAPDATA {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::clone::Clone for DD_UPDATEOVERLAYDATA {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_DirectDraw`*"]
pub struct DD_UPDATEOVERLAYDATA {
    pub lpDD: *mut DD_DIRECTDRAW_GLOBAL,
    pub lpDDDestSurface: *mut DD_SURFACE_LOCAL,
    pub rDest: super::super::Foundation::RECTL,
    pub lpDDSrcSurface: *mut DD_SURFACE_LOCAL,
    pub rSrc: super::super::Foundation::RECTL,
    pub dwFlags: u32,
    pub overlayFX: super::super::Graphics::DirectDraw::DDOVERLAYFX,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub UpdateOverlay: *mut ::std::ffi::c_void,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl DD_UPDATEOVERLAYDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::default::Default for DD_UPDATEOVERLAYDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::PartialEq for DD_UPDATEOVERLAYDATA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::Eq for DD_UPDATEOVERLAYDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
unsafe impl ::windows::runtime::Abi for DD_UPDATEOVERLAYDATA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_DirectDraw`*"]
pub struct DD_UPDATEVPORTDATA {
    pub lpDD: *mut DD_DIRECTDRAW_LOCAL,
    pub lpVideoPort: *mut DD_VIDEOPORT_LOCAL,
    pub lplpDDSurface: *mut *mut DD_SURFACE_INT,
    pub lplpDDVBISurface: *mut *mut DD_SURFACE_INT,
    pub lpVideoInfo: *mut DDVIDEOPORTINFO,
    pub dwFlags: u32,
    pub dwNumAutoflip: u32,
    pub dwNumVBIAutoflip: u32,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub UpdateVideoPort: *mut ::std::ffi::c_void,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl DD_UPDATEVPORTDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::default::Default for DD_UPDATEVPORTDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::fmt::Debug for DD_UPDATEVPORTDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DD_UPDATEVPORTDATA")
            .field("lpDD", &self.lpDD)
            .field("lpVideoPort", &self.lpVideoPort)
            .field("lplpDDSurface", &self.lplpDDSurface)
            .field("lplpDDVBISurface", &self.lplpDDVBISurface)
            .field("lpVideoInfo", &self.lpVideoInfo)
            .field("dwFlags", &self.dwFlags)
            .field("dwNumAutoflip", &self.dwNumAutoflip)
            .field("dwNumVBIAutoflip", &self.dwNumVBIAutoflip)
            .field("ddRVal", &self.ddRVal)
            .field("UpdateVideoPort", &self.UpdateVideoPort)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::PartialEq for DD_UPDATEVPORTDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpVideoPort == other.lpVideoPort && self.lplpDDSurface == other.lplpDDSurface && self.lplpDDVBISurface == other.lplpDDVBISurface && self.lpVideoInfo == other.lpVideoInfo && self.dwFlags == other.dwFlags && self.dwNumAutoflip == other.dwNumAutoflip && self.dwNumVBIAutoflip == other.dwNumVBIAutoflip && self.ddRVal == other.ddRVal && self.UpdateVideoPort == other.UpdateVideoPort
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::Eq for DD_UPDATEVPORTDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
unsafe impl ::windows::runtime::Abi for DD_UPDATEVPORTDATA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_DirectDraw`*"]
pub struct DD_VIDEOPORTCALLBACKS {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub CanCreateVideoPort: ::std::option::Option<PDD_VPORTCB_CANCREATEVIDEOPORT>,
    pub CreateVideoPort: ::std::option::Option<PDD_VPORTCB_CREATEVIDEOPORT>,
    pub FlipVideoPort: ::std::option::Option<PDD_VPORTCB_FLIP>,
    pub GetVideoPortBandwidth: ::std::option::Option<PDD_VPORTCB_GETBANDWIDTH>,
    pub GetVideoPortInputFormats: ::std::option::Option<PDD_VPORTCB_GETINPUTFORMATS>,
    pub GetVideoPortOutputFormats: ::std::option::Option<PDD_VPORTCB_GETOUTPUTFORMATS>,
    pub lpReserved1: *mut ::std::ffi::c_void,
    pub GetVideoPortField: ::std::option::Option<PDD_VPORTCB_GETFIELD>,
    pub GetVideoPortLine: ::std::option::Option<PDD_VPORTCB_GETLINE>,
    pub GetVideoPortConnectInfo: ::std::option::Option<PDD_VPORTCB_GETVPORTCONNECT>,
    pub DestroyVideoPort: ::std::option::Option<PDD_VPORTCB_DESTROYVPORT>,
    pub GetVideoPortFlipStatus: ::std::option::Option<PDD_VPORTCB_GETFLIPSTATUS>,
    pub UpdateVideoPort: ::std::option::Option<PDD_VPORTCB_UPDATE>,
    pub WaitForVideoPortSync: ::std::option::Option<PDD_VPORTCB_WAITFORSYNC>,
    pub GetVideoSignalStatus: ::std::option::Option<PDD_VPORTCB_GETSIGNALSTATUS>,
    pub ColorControl: ::std::option::Option<PDD_VPORTCB_COLORCONTROL>,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl DD_VIDEOPORTCALLBACKS {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::default::Default for DD_VIDEOPORTCALLBACKS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::fmt::Debug for DD_VIDEOPORTCALLBACKS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DD_VIDEOPORTCALLBACKS").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).field("lpReserved1", &self.lpReserved1).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::PartialEq for DD_VIDEOPORTCALLBACKS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
            && self.dwFlags == other.dwFlags
            && self.CanCreateVideoPort.map(|f| f as usize) == other.CanCreateVideoPort.map(|f| f as usize)
            && self.CreateVideoPort.map(|f| f as usize) == other.CreateVideoPort.map(|f| f as usize)
            && self.FlipVideoPort.map(|f| f as usize) == other.FlipVideoPort.map(|f| f as usize)
            && self.GetVideoPortBandwidth.map(|f| f as usize) == other.GetVideoPortBandwidth.map(|f| f as usize)
            && self.GetVideoPortInputFormats.map(|f| f as usize) == other.GetVideoPortInputFormats.map(|f| f as usize)
            && self.GetVideoPortOutputFormats.map(|f| f as usize) == other.GetVideoPortOutputFormats.map(|f| f as usize)
            && self.lpReserved1 == other.lpReserved1
            && self.GetVideoPortField.map(|f| f as usize) == other.GetVideoPortField.map(|f| f as usize)
            && self.GetVideoPortLine.map(|f| f as usize) == other.GetVideoPortLine.map(|f| f as usize)
            && self.GetVideoPortConnectInfo.map(|f| f as usize) == other.GetVideoPortConnectInfo.map(|f| f as usize)
            && self.DestroyVideoPort.map(|f| f as usize) == other.DestroyVideoPort.map(|f| f as usize)
            && self.GetVideoPortFlipStatus.map(|f| f as usize) == other.GetVideoPortFlipStatus.map(|f| f as usize)
            && self.UpdateVideoPort.map(|f| f as usize) == other.UpdateVideoPort.map(|f| f as usize)
            && self.WaitForVideoPortSync.map(|f| f as usize) == other.WaitForVideoPortSync.map(|f| f as usize)
            && self.GetVideoSignalStatus.map(|f| f as usize) == other.GetVideoSignalStatus.map(|f| f as usize)
            && self.ColorControl.map(|f| f as usize) == other.ColorControl.map(|f| f as usize)
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::Eq for DD_VIDEOPORTCALLBACKS {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
unsafe impl ::windows::runtime::Abi for DD_VIDEOPORTCALLBACKS {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_DirectDraw`*"]
pub struct DD_VIDEOPORT_LOCAL {
    pub lpDD: *mut DD_DIRECTDRAW_LOCAL,
    pub ddvpDesc: DDVIDEOPORTDESC,
    pub ddvpInfo: DDVIDEOPORTINFO,
    pub lpSurface: *mut DD_SURFACE_INT,
    pub lpVBISurface: *mut DD_SURFACE_INT,
    pub dwNumAutoflip: u32,
    pub dwNumVBIAutoflip: u32,
    pub dwReserved1: usize,
    pub dwReserved2: usize,
    pub dwReserved3: usize,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl DD_VIDEOPORT_LOCAL {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::default::Default for DD_VIDEOPORT_LOCAL {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::fmt::Debug for DD_VIDEOPORT_LOCAL {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DD_VIDEOPORT_LOCAL")
            .field("lpDD", &self.lpDD)
            .field("ddvpDesc", &self.ddvpDesc)
            .field("ddvpInfo", &self.ddvpInfo)
            .field("lpSurface", &self.lpSurface)
            .field("lpVBISurface", &self.lpVBISurface)
            .field("dwNumAutoflip", &self.dwNumAutoflip)
            .field("dwNumVBIAutoflip", &self.dwNumVBIAutoflip)
            .field("dwReserved1", &self.dwReserved1)
            .field("dwReserved2", &self.dwReserved2)
            .field("dwReserved3", &self.dwReserved3)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::PartialEq for DD_VIDEOPORT_LOCAL {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.ddvpDesc == other.ddvpDesc && self.ddvpInfo == other.ddvpInfo && self.lpSurface == other.lpSurface && self.lpVBISurface == other.lpVBISurface && self.dwNumAutoflip == other.dwNumAutoflip && self.dwNumVBIAutoflip == other.dwNumVBIAutoflip && self.dwReserved1 == other.dwReserved1 && self.dwReserved2 == other.dwReserved2 && self.dwReserved3 == other.dwReserved3
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::Eq for DD_VIDEOPORT_LOCAL {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
unsafe impl ::windows::runtime::Abi for DD_VIDEOPORT_LOCAL {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_DirectDraw`*"]
pub struct DD_VPORTCOLORDATA {
    pub lpDD: *mut DD_DIRECTDRAW_LOCAL,
    pub lpVideoPort: *mut DD_VIDEOPORT_LOCAL,
    pub dwFlags: u32,
    pub lpColorData: *mut super::super::Graphics::DirectDraw::DDCOLORCONTROL,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub ColorControl: *mut ::std::ffi::c_void,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl DD_VPORTCOLORDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::default::Default for DD_VPORTCOLORDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::fmt::Debug for DD_VPORTCOLORDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DD_VPORTCOLORDATA").field("lpDD", &self.lpDD).field("lpVideoPort", &self.lpVideoPort).field("dwFlags", &self.dwFlags).field("lpColorData", &self.lpColorData).field("ddRVal", &self.ddRVal).field("ColorControl", &self.ColorControl).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::PartialEq for DD_VPORTCOLORDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpVideoPort == other.lpVideoPort && self.dwFlags == other.dwFlags && self.lpColorData == other.lpColorData && self.ddRVal == other.ddRVal && self.ColorControl == other.ColorControl
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::Eq for DD_VPORTCOLORDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
unsafe impl ::windows::runtime::Abi for DD_VPORTCOLORDATA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_DisplayDevices`*"]
pub struct DD_WAITFORVERTICALBLANKDATA {
    pub lpDD: *mut DD_DIRECTDRAW_GLOBAL,
    pub dwFlags: u32,
    pub bIsInVB: u32,
    pub hEvent: usize,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub WaitForVerticalBlank: *mut ::std::ffi::c_void,
}
impl DD_WAITFORVERTICALBLANKDATA {}
impl ::std::default::Default for DD_WAITFORVERTICALBLANKDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DD_WAITFORVERTICALBLANKDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DD_WAITFORVERTICALBLANKDATA").field("lpDD", &self.lpDD).field("dwFlags", &self.dwFlags).field("bIsInVB", &self.bIsInVB).field("hEvent", &self.hEvent).field("ddRVal", &self.ddRVal).field("WaitForVerticalBlank", &self.WaitForVerticalBlank).finish()
    }
}
impl ::std::cmp::PartialEq for DD_WAITFORVERTICALBLANKDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.dwFlags == other.dwFlags && self.bIsInVB == other.bIsInVB && self.hEvent == other.hEvent && self.ddRVal == other.ddRVal && self.WaitForVerticalBlank == other.WaitForVerticalBlank
    }
}
impl ::std::cmp::Eq for DD_WAITFORVERTICALBLANKDATA {}
unsafe impl ::windows::runtime::Abi for DD_WAITFORVERTICALBLANKDATA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_DirectDraw`*"]
pub struct DD_WAITFORVPORTSYNCDATA {
    pub lpDD: *mut DD_DIRECTDRAW_LOCAL,
    pub lpVideoPort: *mut DD_VIDEOPORT_LOCAL,
    pub dwFlags: u32,
    pub dwLine: u32,
    pub dwTimeOut: u32,
    pub ddRVal: ::windows::runtime::HRESULT,
    pub UpdateVideoPort: *mut ::std::ffi::c_void,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl DD_WAITFORVPORTSYNCDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::default::Default for DD_WAITFORVPORTSYNCDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::fmt::Debug for DD_WAITFORVPORTSYNCDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DD_WAITFORVPORTSYNCDATA").field("lpDD", &self.lpDD).field("lpVideoPort", &self.lpVideoPort).field("dwFlags", &self.dwFlags).field("dwLine", &self.dwLine).field("dwTimeOut", &self.dwTimeOut).field("ddRVal", &self.ddRVal).field("UpdateVideoPort", &self.UpdateVideoPort).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::PartialEq for DD_WAITFORVPORTSYNCDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpVideoPort == other.lpVideoPort && self.dwFlags == other.dwFlags && self.dwLine == other.dwLine && self.dwTimeOut == other.dwTimeOut && self.ddRVal == other.ddRVal && self.UpdateVideoPort == other.UpdateVideoPort
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::Eq for DD_WAITFORVPORTSYNCDATA {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
unsafe impl ::windows::runtime::Abi for DD_WAITFORVPORTSYNCDATA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_DisplayDevices`*"]
pub struct DEVHTADJDATA {
    pub DeviceFlags: u32,
    pub DeviceXDPI: u32,
    pub DeviceYDPI: u32,
    pub pDefHTInfo: *mut DEVHTINFO,
    pub pAdjHTInfo: *mut DEVHTINFO,
}
impl DEVHTADJDATA {}
impl ::std::default::Default for DEVHTADJDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DEVHTADJDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DEVHTADJDATA").field("DeviceFlags", &self.DeviceFlags).field("DeviceXDPI", &self.DeviceXDPI).field("DeviceYDPI", &self.DeviceYDPI).field("pDefHTInfo", &self.pDefHTInfo).field("pAdjHTInfo", &self.pAdjHTInfo).finish()
    }
}
impl ::std::cmp::PartialEq for DEVHTADJDATA {
    fn eq(&self, other: &Self) -> bool {
        self.DeviceFlags == other.DeviceFlags && self.DeviceXDPI == other.DeviceXDPI && self.DeviceYDPI == other.DeviceYDPI && self.pDefHTInfo == other.pDefHTInfo && self.pAdjHTInfo == other.pAdjHTInfo
    }
}
impl ::std::cmp::Eq for DEVHTADJDATA {}
unsafe impl ::windows::runtime::Abi for DEVHTADJDATA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_DisplayDevices`*"]
pub struct DEVHTINFO {
    pub HTFlags: u32,
    pub HTPatternSize: u32,
    pub DevPelsDPI: u32,
    pub ColorInfo: COLORINFO,
}
impl DEVHTINFO {}
impl ::std::default::Default for DEVHTINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DEVHTINFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DEVHTINFO").field("HTFlags", &self.HTFlags).field("HTPatternSize", &self.HTPatternSize).field("DevPelsDPI", &self.DevPelsDPI).field("ColorInfo", &self.ColorInfo).finish()
    }
}
impl ::std::cmp::PartialEq for DEVHTINFO {
    fn eq(&self, other: &Self) -> bool {
        self.HTFlags == other.HTFlags && self.HTPatternSize == other.HTPatternSize && self.DevPelsDPI == other.DevPelsDPI && self.ColorInfo == other.ColorInfo
    }
}
impl ::std::cmp::Eq for DEVHTINFO {}
unsafe impl ::windows::runtime::Abi for DEVHTINFO {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Graphics_Gdi`*"]
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
impl ::std::default::Default for DEVINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::std::fmt::Debug for DEVINFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
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
impl ::std::cmp::PartialEq for DEVINFO {
    fn eq(&self, other: &Self) -> bool {
        self.flGraphicsCaps == other.flGraphicsCaps && self.lfDefaultFont == other.lfDefaultFont && self.lfAnsiVarFont == other.lfAnsiVarFont && self.lfAnsiFixFont == other.lfAnsiFixFont && self.cFonts == other.cFonts && self.iDitherFormat == other.iDitherFormat && self.cxDither == other.cxDither && self.cyDither == other.cyDither && self.hpalDefault == other.hpalDefault && self.flGraphicsCaps2 == other.flGraphicsCaps2
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::std::cmp::Eq for DEVINFO {}
#[cfg(feature = "Win32_Graphics_Gdi")]
unsafe impl ::windows::runtime::Abi for DEVINFO {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_DisplayDevices`*"]
pub struct DISPLAYCONFIG_2DREGION {
    pub cx: u32,
    pub cy: u32,
}
impl DISPLAYCONFIG_2DREGION {}
impl ::std::default::Default for DISPLAYCONFIG_2DREGION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DISPLAYCONFIG_2DREGION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DISPLAYCONFIG_2DREGION").field("cx", &self.cx).field("cy", &self.cy).finish()
    }
}
impl ::std::cmp::PartialEq for DISPLAYCONFIG_2DREGION {
    fn eq(&self, other: &Self) -> bool {
        self.cx == other.cx && self.cy == other.cy
    }
}
impl ::std::cmp::Eq for DISPLAYCONFIG_2DREGION {}
unsafe impl ::windows::runtime::Abi for DISPLAYCONFIG_2DREGION {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`*"]
pub struct DISPLAYCONFIG_ADAPTER_NAME {
    pub header: DISPLAYCONFIG_DEVICE_INFO_HEADER,
    pub adapterDevicePath: [u16; 128],
}
#[cfg(feature = "Win32_Foundation")]
impl DISPLAYCONFIG_ADAPTER_NAME {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DISPLAYCONFIG_ADAPTER_NAME {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for DISPLAYCONFIG_ADAPTER_NAME {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DISPLAYCONFIG_ADAPTER_NAME").field("header", &self.header).field("adapterDevicePath", &self.adapterDevicePath).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DISPLAYCONFIG_ADAPTER_NAME {
    fn eq(&self, other: &Self) -> bool {
        self.header == other.header && self.adapterDevicePath == other.adapterDevicePath
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DISPLAYCONFIG_ADAPTER_NAME {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DISPLAYCONFIG_ADAPTER_NAME {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`*"]
pub struct DISPLAYCONFIG_DESKTOP_IMAGE_INFO {
    pub PathSourceSize: super::super::Foundation::POINTL,
    pub DesktopImageRegion: super::super::Foundation::RECTL,
    pub DesktopImageClip: super::super::Foundation::RECTL,
}
#[cfg(feature = "Win32_Foundation")]
impl DISPLAYCONFIG_DESKTOP_IMAGE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DISPLAYCONFIG_DESKTOP_IMAGE_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for DISPLAYCONFIG_DESKTOP_IMAGE_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DISPLAYCONFIG_DESKTOP_IMAGE_INFO").field("PathSourceSize", &self.PathSourceSize).field("DesktopImageRegion", &self.DesktopImageRegion).field("DesktopImageClip", &self.DesktopImageClip).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DISPLAYCONFIG_DESKTOP_IMAGE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.PathSourceSize == other.PathSourceSize && self.DesktopImageRegion == other.DesktopImageRegion && self.DesktopImageClip == other.DesktopImageClip
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DISPLAYCONFIG_DESKTOP_IMAGE_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DISPLAYCONFIG_DESKTOP_IMAGE_INFO {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`*"]
pub struct DISPLAYCONFIG_DEVICE_INFO_HEADER {
    pub r#type: DISPLAYCONFIG_DEVICE_INFO_TYPE,
    pub size: u32,
    pub adapterId: super::super::Foundation::LUID,
    pub id: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl DISPLAYCONFIG_DEVICE_INFO_HEADER {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DISPLAYCONFIG_DEVICE_INFO_HEADER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for DISPLAYCONFIG_DEVICE_INFO_HEADER {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DISPLAYCONFIG_DEVICE_INFO_HEADER").field("r#type", &self.r#type).field("size", &self.size).field("adapterId", &self.adapterId).field("id", &self.id).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DISPLAYCONFIG_DEVICE_INFO_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.r#type == other.r#type && self.size == other.size && self.adapterId == other.adapterId && self.id == other.id
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DISPLAYCONFIG_DEVICE_INFO_HEADER {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DISPLAYCONFIG_DEVICE_INFO_HEADER {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_DisplayDevices`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for DISPLAYCONFIG_DEVICE_INFO_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DISPLAYCONFIG_DEVICE_INFO_TYPE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
pub struct DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO {
    pub header: DISPLAYCONFIG_DEVICE_INFO_HEADER,
    pub Anonymous: DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO_0,
    pub colorEncoding: super::super::Graphics::Gdi::DISPLAYCONFIG_COLOR_ENCODING,
    pub bitsPerColorChannel: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::default::Default for DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::PartialEq for DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::Eq for DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::runtime::Abi for DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub union DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO_0 {
    pub Anonymous: DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO_0_0,
    pub value: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::default::Default for DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::PartialEq for DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::Eq for DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::runtime::Abi for DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO_0_0 {
    pub _bitfield: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::default::Default for DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::fmt::Debug for DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("_bitfield", &self._bitfield).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::PartialEq for DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::Eq for DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::runtime::Abi for DISPLAYCONFIG_GET_ADVANCED_COLOR_INFO_0_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`*"]
pub struct DISPLAYCONFIG_GET_MONITOR_SPECIALIZATION {
    pub header: DISPLAYCONFIG_DEVICE_INFO_HEADER,
    pub Anonymous: DISPLAYCONFIG_GET_MONITOR_SPECIALIZATION_0,
}
#[cfg(feature = "Win32_Foundation")]
impl DISPLAYCONFIG_GET_MONITOR_SPECIALIZATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DISPLAYCONFIG_GET_MONITOR_SPECIALIZATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DISPLAYCONFIG_GET_MONITOR_SPECIALIZATION {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DISPLAYCONFIG_GET_MONITOR_SPECIALIZATION {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DISPLAYCONFIG_GET_MONITOR_SPECIALIZATION {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union DISPLAYCONFIG_GET_MONITOR_SPECIALIZATION_0 {
    pub Anonymous: DISPLAYCONFIG_GET_MONITOR_SPECIALIZATION_0_0,
    pub value: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl DISPLAYCONFIG_GET_MONITOR_SPECIALIZATION_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DISPLAYCONFIG_GET_MONITOR_SPECIALIZATION_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DISPLAYCONFIG_GET_MONITOR_SPECIALIZATION_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DISPLAYCONFIG_GET_MONITOR_SPECIALIZATION_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DISPLAYCONFIG_GET_MONITOR_SPECIALIZATION_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DISPLAYCONFIG_GET_MONITOR_SPECIALIZATION_0_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl DISPLAYCONFIG_GET_MONITOR_SPECIALIZATION_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DISPLAYCONFIG_GET_MONITOR_SPECIALIZATION_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for DISPLAYCONFIG_GET_MONITOR_SPECIALIZATION_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("_bitfield", &self._bitfield).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DISPLAYCONFIG_GET_MONITOR_SPECIALIZATION_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DISPLAYCONFIG_GET_MONITOR_SPECIALIZATION_0_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DISPLAYCONFIG_GET_MONITOR_SPECIALIZATION_0_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`*"]
pub struct DISPLAYCONFIG_MODE_INFO {
    pub infoType: DISPLAYCONFIG_MODE_INFO_TYPE,
    pub id: u32,
    pub adapterId: super::super::Foundation::LUID,
    pub Anonymous: DISPLAYCONFIG_MODE_INFO_0,
}
#[cfg(feature = "Win32_Foundation")]
impl DISPLAYCONFIG_MODE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DISPLAYCONFIG_MODE_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DISPLAYCONFIG_MODE_INFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DISPLAYCONFIG_MODE_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DISPLAYCONFIG_MODE_INFO {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
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
impl ::std::default::Default for DISPLAYCONFIG_MODE_INFO_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DISPLAYCONFIG_MODE_INFO_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DISPLAYCONFIG_MODE_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DISPLAYCONFIG_MODE_INFO_0 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_DisplayDevices`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DISPLAYCONFIG_MODE_INFO_TYPE(pub i32);
pub const DISPLAYCONFIG_MODE_INFO_TYPE_SOURCE: DISPLAYCONFIG_MODE_INFO_TYPE = DISPLAYCONFIG_MODE_INFO_TYPE(1i32);
pub const DISPLAYCONFIG_MODE_INFO_TYPE_TARGET: DISPLAYCONFIG_MODE_INFO_TYPE = DISPLAYCONFIG_MODE_INFO_TYPE(2i32);
pub const DISPLAYCONFIG_MODE_INFO_TYPE_DESKTOP_IMAGE: DISPLAYCONFIG_MODE_INFO_TYPE = DISPLAYCONFIG_MODE_INFO_TYPE(3i32);
pub const DISPLAYCONFIG_MODE_INFO_TYPE_FORCE_UINT32: DISPLAYCONFIG_MODE_INFO_TYPE = DISPLAYCONFIG_MODE_INFO_TYPE(-1i32);
impl ::std::convert::From<i32> for DISPLAYCONFIG_MODE_INFO_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DISPLAYCONFIG_MODE_INFO_TYPE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`*"]
pub struct DISPLAYCONFIG_PATH_INFO {
    pub sourceInfo: DISPLAYCONFIG_PATH_SOURCE_INFO,
    pub targetInfo: DISPLAYCONFIG_PATH_TARGET_INFO,
    pub flags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl DISPLAYCONFIG_PATH_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DISPLAYCONFIG_PATH_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DISPLAYCONFIG_PATH_INFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DISPLAYCONFIG_PATH_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DISPLAYCONFIG_PATH_INFO {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`*"]
pub struct DISPLAYCONFIG_PATH_SOURCE_INFO {
    pub adapterId: super::super::Foundation::LUID,
    pub id: u32,
    pub Anonymous: DISPLAYCONFIG_PATH_SOURCE_INFO_0,
    pub statusFlags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl DISPLAYCONFIG_PATH_SOURCE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DISPLAYCONFIG_PATH_SOURCE_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DISPLAYCONFIG_PATH_SOURCE_INFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DISPLAYCONFIG_PATH_SOURCE_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DISPLAYCONFIG_PATH_SOURCE_INFO {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union DISPLAYCONFIG_PATH_SOURCE_INFO_0 {
    pub modeInfoIdx: u32,
    pub Anonymous: DISPLAYCONFIG_PATH_SOURCE_INFO_0_0,
}
#[cfg(feature = "Win32_Foundation")]
impl DISPLAYCONFIG_PATH_SOURCE_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DISPLAYCONFIG_PATH_SOURCE_INFO_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DISPLAYCONFIG_PATH_SOURCE_INFO_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DISPLAYCONFIG_PATH_SOURCE_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DISPLAYCONFIG_PATH_SOURCE_INFO_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DISPLAYCONFIG_PATH_SOURCE_INFO_0_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl DISPLAYCONFIG_PATH_SOURCE_INFO_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DISPLAYCONFIG_PATH_SOURCE_INFO_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for DISPLAYCONFIG_PATH_SOURCE_INFO_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("_bitfield", &self._bitfield).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DISPLAYCONFIG_PATH_SOURCE_INFO_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DISPLAYCONFIG_PATH_SOURCE_INFO_0_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DISPLAYCONFIG_PATH_SOURCE_INFO_0_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`*"]
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
impl ::std::default::Default for DISPLAYCONFIG_PATH_TARGET_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DISPLAYCONFIG_PATH_TARGET_INFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DISPLAYCONFIG_PATH_TARGET_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DISPLAYCONFIG_PATH_TARGET_INFO {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union DISPLAYCONFIG_PATH_TARGET_INFO_0 {
    pub modeInfoIdx: u32,
    pub Anonymous: DISPLAYCONFIG_PATH_TARGET_INFO_0_0,
}
#[cfg(feature = "Win32_Foundation")]
impl DISPLAYCONFIG_PATH_TARGET_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DISPLAYCONFIG_PATH_TARGET_INFO_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DISPLAYCONFIG_PATH_TARGET_INFO_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DISPLAYCONFIG_PATH_TARGET_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DISPLAYCONFIG_PATH_TARGET_INFO_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DISPLAYCONFIG_PATH_TARGET_INFO_0_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl DISPLAYCONFIG_PATH_TARGET_INFO_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DISPLAYCONFIG_PATH_TARGET_INFO_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for DISPLAYCONFIG_PATH_TARGET_INFO_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("_bitfield", &self._bitfield).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DISPLAYCONFIG_PATH_TARGET_INFO_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DISPLAYCONFIG_PATH_TARGET_INFO_0_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DISPLAYCONFIG_PATH_TARGET_INFO_0_0 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_DisplayDevices`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DISPLAYCONFIG_PIXELFORMAT(pub i32);
pub const DISPLAYCONFIG_PIXELFORMAT_8BPP: DISPLAYCONFIG_PIXELFORMAT = DISPLAYCONFIG_PIXELFORMAT(1i32);
pub const DISPLAYCONFIG_PIXELFORMAT_16BPP: DISPLAYCONFIG_PIXELFORMAT = DISPLAYCONFIG_PIXELFORMAT(2i32);
pub const DISPLAYCONFIG_PIXELFORMAT_24BPP: DISPLAYCONFIG_PIXELFORMAT = DISPLAYCONFIG_PIXELFORMAT(3i32);
pub const DISPLAYCONFIG_PIXELFORMAT_32BPP: DISPLAYCONFIG_PIXELFORMAT = DISPLAYCONFIG_PIXELFORMAT(4i32);
pub const DISPLAYCONFIG_PIXELFORMAT_NONGDI: DISPLAYCONFIG_PIXELFORMAT = DISPLAYCONFIG_PIXELFORMAT(5i32);
pub const DISPLAYCONFIG_PIXELFORMAT_FORCE_UINT32: DISPLAYCONFIG_PIXELFORMAT = DISPLAYCONFIG_PIXELFORMAT(-1i32);
impl ::std::convert::From<i32> for DISPLAYCONFIG_PIXELFORMAT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DISPLAYCONFIG_PIXELFORMAT {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_DisplayDevices`*"]
pub struct DISPLAYCONFIG_RATIONAL {
    pub Numerator: u32,
    pub Denominator: u32,
}
impl DISPLAYCONFIG_RATIONAL {}
impl ::std::default::Default for DISPLAYCONFIG_RATIONAL {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DISPLAYCONFIG_RATIONAL {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DISPLAYCONFIG_RATIONAL").field("Numerator", &self.Numerator).field("Denominator", &self.Denominator).finish()
    }
}
impl ::std::cmp::PartialEq for DISPLAYCONFIG_RATIONAL {
    fn eq(&self, other: &Self) -> bool {
        self.Numerator == other.Numerator && self.Denominator == other.Denominator
    }
}
impl ::std::cmp::Eq for DISPLAYCONFIG_RATIONAL {}
unsafe impl ::windows::runtime::Abi for DISPLAYCONFIG_RATIONAL {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_DisplayDevices`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DISPLAYCONFIG_ROTATION(pub i32);
pub const DISPLAYCONFIG_ROTATION_IDENTITY: DISPLAYCONFIG_ROTATION = DISPLAYCONFIG_ROTATION(1i32);
pub const DISPLAYCONFIG_ROTATION_ROTATE90: DISPLAYCONFIG_ROTATION = DISPLAYCONFIG_ROTATION(2i32);
pub const DISPLAYCONFIG_ROTATION_ROTATE180: DISPLAYCONFIG_ROTATION = DISPLAYCONFIG_ROTATION(3i32);
pub const DISPLAYCONFIG_ROTATION_ROTATE270: DISPLAYCONFIG_ROTATION = DISPLAYCONFIG_ROTATION(4i32);
pub const DISPLAYCONFIG_ROTATION_FORCE_UINT32: DISPLAYCONFIG_ROTATION = DISPLAYCONFIG_ROTATION(-1i32);
impl ::std::convert::From<i32> for DISPLAYCONFIG_ROTATION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DISPLAYCONFIG_ROTATION {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_DisplayDevices`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DISPLAYCONFIG_SCALING(pub i32);
pub const DISPLAYCONFIG_SCALING_IDENTITY: DISPLAYCONFIG_SCALING = DISPLAYCONFIG_SCALING(1i32);
pub const DISPLAYCONFIG_SCALING_CENTERED: DISPLAYCONFIG_SCALING = DISPLAYCONFIG_SCALING(2i32);
pub const DISPLAYCONFIG_SCALING_STRETCHED: DISPLAYCONFIG_SCALING = DISPLAYCONFIG_SCALING(3i32);
pub const DISPLAYCONFIG_SCALING_ASPECTRATIOCENTEREDMAX: DISPLAYCONFIG_SCALING = DISPLAYCONFIG_SCALING(4i32);
pub const DISPLAYCONFIG_SCALING_CUSTOM: DISPLAYCONFIG_SCALING = DISPLAYCONFIG_SCALING(5i32);
pub const DISPLAYCONFIG_SCALING_PREFERRED: DISPLAYCONFIG_SCALING = DISPLAYCONFIG_SCALING(128i32);
pub const DISPLAYCONFIG_SCALING_FORCE_UINT32: DISPLAYCONFIG_SCALING = DISPLAYCONFIG_SCALING(-1i32);
impl ::std::convert::From<i32> for DISPLAYCONFIG_SCALING {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DISPLAYCONFIG_SCALING {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_DisplayDevices`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DISPLAYCONFIG_SCANLINE_ORDERING(pub i32);
pub const DISPLAYCONFIG_SCANLINE_ORDERING_UNSPECIFIED: DISPLAYCONFIG_SCANLINE_ORDERING = DISPLAYCONFIG_SCANLINE_ORDERING(0i32);
pub const DISPLAYCONFIG_SCANLINE_ORDERING_PROGRESSIVE: DISPLAYCONFIG_SCANLINE_ORDERING = DISPLAYCONFIG_SCANLINE_ORDERING(1i32);
pub const DISPLAYCONFIG_SCANLINE_ORDERING_INTERLACED: DISPLAYCONFIG_SCANLINE_ORDERING = DISPLAYCONFIG_SCANLINE_ORDERING(2i32);
pub const DISPLAYCONFIG_SCANLINE_ORDERING_INTERLACED_UPPERFIELDFIRST: DISPLAYCONFIG_SCANLINE_ORDERING = DISPLAYCONFIG_SCANLINE_ORDERING(2i32);
pub const DISPLAYCONFIG_SCANLINE_ORDERING_INTERLACED_LOWERFIELDFIRST: DISPLAYCONFIG_SCANLINE_ORDERING = DISPLAYCONFIG_SCANLINE_ORDERING(3i32);
pub const DISPLAYCONFIG_SCANLINE_ORDERING_FORCE_UINT32: DISPLAYCONFIG_SCANLINE_ORDERING = DISPLAYCONFIG_SCANLINE_ORDERING(-1i32);
impl ::std::convert::From<i32> for DISPLAYCONFIG_SCANLINE_ORDERING {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DISPLAYCONFIG_SCANLINE_ORDERING {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`*"]
pub struct DISPLAYCONFIG_SDR_WHITE_LEVEL {
    pub header: DISPLAYCONFIG_DEVICE_INFO_HEADER,
    pub SDRWhiteLevel: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl DISPLAYCONFIG_SDR_WHITE_LEVEL {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DISPLAYCONFIG_SDR_WHITE_LEVEL {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for DISPLAYCONFIG_SDR_WHITE_LEVEL {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DISPLAYCONFIG_SDR_WHITE_LEVEL").field("header", &self.header).field("SDRWhiteLevel", &self.SDRWhiteLevel).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DISPLAYCONFIG_SDR_WHITE_LEVEL {
    fn eq(&self, other: &Self) -> bool {
        self.header == other.header && self.SDRWhiteLevel == other.SDRWhiteLevel
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DISPLAYCONFIG_SDR_WHITE_LEVEL {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DISPLAYCONFIG_SDR_WHITE_LEVEL {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`*"]
pub struct DISPLAYCONFIG_SET_ADVANCED_COLOR_STATE {
    pub header: DISPLAYCONFIG_DEVICE_INFO_HEADER,
    pub Anonymous: DISPLAYCONFIG_SET_ADVANCED_COLOR_STATE_0,
}
#[cfg(feature = "Win32_Foundation")]
impl DISPLAYCONFIG_SET_ADVANCED_COLOR_STATE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DISPLAYCONFIG_SET_ADVANCED_COLOR_STATE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DISPLAYCONFIG_SET_ADVANCED_COLOR_STATE {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DISPLAYCONFIG_SET_ADVANCED_COLOR_STATE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DISPLAYCONFIG_SET_ADVANCED_COLOR_STATE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union DISPLAYCONFIG_SET_ADVANCED_COLOR_STATE_0 {
    pub Anonymous: DISPLAYCONFIG_SET_ADVANCED_COLOR_STATE_0_0,
    pub value: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl DISPLAYCONFIG_SET_ADVANCED_COLOR_STATE_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DISPLAYCONFIG_SET_ADVANCED_COLOR_STATE_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DISPLAYCONFIG_SET_ADVANCED_COLOR_STATE_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DISPLAYCONFIG_SET_ADVANCED_COLOR_STATE_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DISPLAYCONFIG_SET_ADVANCED_COLOR_STATE_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DISPLAYCONFIG_SET_ADVANCED_COLOR_STATE_0_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl DISPLAYCONFIG_SET_ADVANCED_COLOR_STATE_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DISPLAYCONFIG_SET_ADVANCED_COLOR_STATE_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for DISPLAYCONFIG_SET_ADVANCED_COLOR_STATE_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("_bitfield", &self._bitfield).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DISPLAYCONFIG_SET_ADVANCED_COLOR_STATE_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DISPLAYCONFIG_SET_ADVANCED_COLOR_STATE_0_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DISPLAYCONFIG_SET_ADVANCED_COLOR_STATE_0_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`*"]
pub struct DISPLAYCONFIG_SET_MONITOR_SPECIALIZATION {
    pub header: DISPLAYCONFIG_DEVICE_INFO_HEADER,
    pub Anonymous: DISPLAYCONFIG_SET_MONITOR_SPECIALIZATION_0,
    pub specializationType: ::windows::runtime::GUID,
    pub specializationSubType: ::windows::runtime::GUID,
    pub specializationApplicationName: [u16; 128],
}
#[cfg(feature = "Win32_Foundation")]
impl DISPLAYCONFIG_SET_MONITOR_SPECIALIZATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DISPLAYCONFIG_SET_MONITOR_SPECIALIZATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DISPLAYCONFIG_SET_MONITOR_SPECIALIZATION {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DISPLAYCONFIG_SET_MONITOR_SPECIALIZATION {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DISPLAYCONFIG_SET_MONITOR_SPECIALIZATION {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union DISPLAYCONFIG_SET_MONITOR_SPECIALIZATION_0 {
    pub Anonymous: DISPLAYCONFIG_SET_MONITOR_SPECIALIZATION_0_0,
    pub value: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl DISPLAYCONFIG_SET_MONITOR_SPECIALIZATION_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DISPLAYCONFIG_SET_MONITOR_SPECIALIZATION_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DISPLAYCONFIG_SET_MONITOR_SPECIALIZATION_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DISPLAYCONFIG_SET_MONITOR_SPECIALIZATION_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DISPLAYCONFIG_SET_MONITOR_SPECIALIZATION_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DISPLAYCONFIG_SET_MONITOR_SPECIALIZATION_0_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl DISPLAYCONFIG_SET_MONITOR_SPECIALIZATION_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DISPLAYCONFIG_SET_MONITOR_SPECIALIZATION_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for DISPLAYCONFIG_SET_MONITOR_SPECIALIZATION_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("_bitfield", &self._bitfield).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DISPLAYCONFIG_SET_MONITOR_SPECIALIZATION_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DISPLAYCONFIG_SET_MONITOR_SPECIALIZATION_0_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DISPLAYCONFIG_SET_MONITOR_SPECIALIZATION_0_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`*"]
pub struct DISPLAYCONFIG_SET_TARGET_PERSISTENCE {
    pub header: DISPLAYCONFIG_DEVICE_INFO_HEADER,
    pub Anonymous: DISPLAYCONFIG_SET_TARGET_PERSISTENCE_0,
}
#[cfg(feature = "Win32_Foundation")]
impl DISPLAYCONFIG_SET_TARGET_PERSISTENCE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DISPLAYCONFIG_SET_TARGET_PERSISTENCE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DISPLAYCONFIG_SET_TARGET_PERSISTENCE {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DISPLAYCONFIG_SET_TARGET_PERSISTENCE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DISPLAYCONFIG_SET_TARGET_PERSISTENCE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union DISPLAYCONFIG_SET_TARGET_PERSISTENCE_0 {
    pub Anonymous: DISPLAYCONFIG_SET_TARGET_PERSISTENCE_0_0,
    pub value: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl DISPLAYCONFIG_SET_TARGET_PERSISTENCE_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DISPLAYCONFIG_SET_TARGET_PERSISTENCE_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DISPLAYCONFIG_SET_TARGET_PERSISTENCE_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DISPLAYCONFIG_SET_TARGET_PERSISTENCE_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DISPLAYCONFIG_SET_TARGET_PERSISTENCE_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DISPLAYCONFIG_SET_TARGET_PERSISTENCE_0_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl DISPLAYCONFIG_SET_TARGET_PERSISTENCE_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DISPLAYCONFIG_SET_TARGET_PERSISTENCE_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for DISPLAYCONFIG_SET_TARGET_PERSISTENCE_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("_bitfield", &self._bitfield).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DISPLAYCONFIG_SET_TARGET_PERSISTENCE_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DISPLAYCONFIG_SET_TARGET_PERSISTENCE_0_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DISPLAYCONFIG_SET_TARGET_PERSISTENCE_0_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`*"]
pub struct DISPLAYCONFIG_SOURCE_DEVICE_NAME {
    pub header: DISPLAYCONFIG_DEVICE_INFO_HEADER,
    pub viewGdiDeviceName: [u16; 32],
}
#[cfg(feature = "Win32_Foundation")]
impl DISPLAYCONFIG_SOURCE_DEVICE_NAME {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DISPLAYCONFIG_SOURCE_DEVICE_NAME {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for DISPLAYCONFIG_SOURCE_DEVICE_NAME {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DISPLAYCONFIG_SOURCE_DEVICE_NAME").field("header", &self.header).field("viewGdiDeviceName", &self.viewGdiDeviceName).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DISPLAYCONFIG_SOURCE_DEVICE_NAME {
    fn eq(&self, other: &Self) -> bool {
        self.header == other.header && self.viewGdiDeviceName == other.viewGdiDeviceName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DISPLAYCONFIG_SOURCE_DEVICE_NAME {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DISPLAYCONFIG_SOURCE_DEVICE_NAME {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`*"]
pub struct DISPLAYCONFIG_SOURCE_MODE {
    pub width: u32,
    pub height: u32,
    pub pixelFormat: DISPLAYCONFIG_PIXELFORMAT,
    pub position: super::super::Foundation::POINTL,
}
#[cfg(feature = "Win32_Foundation")]
impl DISPLAYCONFIG_SOURCE_MODE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DISPLAYCONFIG_SOURCE_MODE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for DISPLAYCONFIG_SOURCE_MODE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DISPLAYCONFIG_SOURCE_MODE").field("width", &self.width).field("height", &self.height).field("pixelFormat", &self.pixelFormat).field("position", &self.position).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DISPLAYCONFIG_SOURCE_MODE {
    fn eq(&self, other: &Self) -> bool {
        self.width == other.width && self.height == other.height && self.pixelFormat == other.pixelFormat && self.position == other.position
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DISPLAYCONFIG_SOURCE_MODE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DISPLAYCONFIG_SOURCE_MODE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`*"]
pub struct DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION {
    pub header: DISPLAYCONFIG_DEVICE_INFO_HEADER,
    pub Anonymous: DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION_0,
}
#[cfg(feature = "Win32_Foundation")]
impl DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION_0 {
    pub Anonymous: DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION_0_0,
    pub value: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION_0_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("_bitfield", &self._bitfield).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION_0_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION_0_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`*"]
pub struct DISPLAYCONFIG_TARGET_BASE_TYPE {
    pub header: DISPLAYCONFIG_DEVICE_INFO_HEADER,
    pub baseOutputTechnology: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY,
}
#[cfg(feature = "Win32_Foundation")]
impl DISPLAYCONFIG_TARGET_BASE_TYPE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DISPLAYCONFIG_TARGET_BASE_TYPE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for DISPLAYCONFIG_TARGET_BASE_TYPE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DISPLAYCONFIG_TARGET_BASE_TYPE").field("header", &self.header).field("baseOutputTechnology", &self.baseOutputTechnology).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DISPLAYCONFIG_TARGET_BASE_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.header == other.header && self.baseOutputTechnology == other.baseOutputTechnology
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DISPLAYCONFIG_TARGET_BASE_TYPE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DISPLAYCONFIG_TARGET_BASE_TYPE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`*"]
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
impl ::std::default::Default for DISPLAYCONFIG_TARGET_DEVICE_NAME {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DISPLAYCONFIG_TARGET_DEVICE_NAME {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DISPLAYCONFIG_TARGET_DEVICE_NAME {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DISPLAYCONFIG_TARGET_DEVICE_NAME {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_DisplayDevices`*"]
pub struct DISPLAYCONFIG_TARGET_DEVICE_NAME_FLAGS {
    pub Anonymous: DISPLAYCONFIG_TARGET_DEVICE_NAME_FLAGS_0,
}
impl DISPLAYCONFIG_TARGET_DEVICE_NAME_FLAGS {}
impl ::std::default::Default for DISPLAYCONFIG_TARGET_DEVICE_NAME_FLAGS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for DISPLAYCONFIG_TARGET_DEVICE_NAME_FLAGS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for DISPLAYCONFIG_TARGET_DEVICE_NAME_FLAGS {}
unsafe impl ::windows::runtime::Abi for DISPLAYCONFIG_TARGET_DEVICE_NAME_FLAGS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_DisplayDevices`*"]
pub union DISPLAYCONFIG_TARGET_DEVICE_NAME_FLAGS_0 {
    pub Anonymous: DISPLAYCONFIG_TARGET_DEVICE_NAME_FLAGS_0_0,
    pub value: u32,
}
impl DISPLAYCONFIG_TARGET_DEVICE_NAME_FLAGS_0 {}
impl ::std::default::Default for DISPLAYCONFIG_TARGET_DEVICE_NAME_FLAGS_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for DISPLAYCONFIG_TARGET_DEVICE_NAME_FLAGS_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for DISPLAYCONFIG_TARGET_DEVICE_NAME_FLAGS_0 {}
unsafe impl ::windows::runtime::Abi for DISPLAYCONFIG_TARGET_DEVICE_NAME_FLAGS_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_DisplayDevices`*"]
pub struct DISPLAYCONFIG_TARGET_DEVICE_NAME_FLAGS_0_0 {
    pub _bitfield: u32,
}
impl DISPLAYCONFIG_TARGET_DEVICE_NAME_FLAGS_0_0 {}
impl ::std::default::Default for DISPLAYCONFIG_TARGET_DEVICE_NAME_FLAGS_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DISPLAYCONFIG_TARGET_DEVICE_NAME_FLAGS_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::std::cmp::PartialEq for DISPLAYCONFIG_TARGET_DEVICE_NAME_FLAGS_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for DISPLAYCONFIG_TARGET_DEVICE_NAME_FLAGS_0_0 {}
unsafe impl ::windows::runtime::Abi for DISPLAYCONFIG_TARGET_DEVICE_NAME_FLAGS_0_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_DisplayDevices`*"]
pub struct DISPLAYCONFIG_TARGET_MODE {
    pub targetVideoSignalInfo: DISPLAYCONFIG_VIDEO_SIGNAL_INFO,
}
impl DISPLAYCONFIG_TARGET_MODE {}
impl ::std::default::Default for DISPLAYCONFIG_TARGET_MODE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for DISPLAYCONFIG_TARGET_MODE {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for DISPLAYCONFIG_TARGET_MODE {}
unsafe impl ::windows::runtime::Abi for DISPLAYCONFIG_TARGET_MODE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`*"]
pub struct DISPLAYCONFIG_TARGET_PREFERRED_MODE {
    pub header: DISPLAYCONFIG_DEVICE_INFO_HEADER,
    pub width: u32,
    pub height: u32,
    pub targetMode: DISPLAYCONFIG_TARGET_MODE,
}
#[cfg(feature = "Win32_Foundation")]
impl DISPLAYCONFIG_TARGET_PREFERRED_MODE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DISPLAYCONFIG_TARGET_PREFERRED_MODE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DISPLAYCONFIG_TARGET_PREFERRED_MODE {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DISPLAYCONFIG_TARGET_PREFERRED_MODE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DISPLAYCONFIG_TARGET_PREFERRED_MODE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_DisplayDevices`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DISPLAYCONFIG_TOPOLOGY_ID(pub i32);
pub const DISPLAYCONFIG_TOPOLOGY_INTERNAL: DISPLAYCONFIG_TOPOLOGY_ID = DISPLAYCONFIG_TOPOLOGY_ID(1i32);
pub const DISPLAYCONFIG_TOPOLOGY_CLONE: DISPLAYCONFIG_TOPOLOGY_ID = DISPLAYCONFIG_TOPOLOGY_ID(2i32);
pub const DISPLAYCONFIG_TOPOLOGY_EXTEND: DISPLAYCONFIG_TOPOLOGY_ID = DISPLAYCONFIG_TOPOLOGY_ID(4i32);
pub const DISPLAYCONFIG_TOPOLOGY_EXTERNAL: DISPLAYCONFIG_TOPOLOGY_ID = DISPLAYCONFIG_TOPOLOGY_ID(8i32);
pub const DISPLAYCONFIG_TOPOLOGY_FORCE_UINT32: DISPLAYCONFIG_TOPOLOGY_ID = DISPLAYCONFIG_TOPOLOGY_ID(-1i32);
impl ::std::convert::From<i32> for DISPLAYCONFIG_TOPOLOGY_ID {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DISPLAYCONFIG_TOPOLOGY_ID {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_DisplayDevices`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_DisplayDevices`*"]
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
impl ::std::default::Default for DISPLAYCONFIG_VIDEO_SIGNAL_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for DISPLAYCONFIG_VIDEO_SIGNAL_INFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for DISPLAYCONFIG_VIDEO_SIGNAL_INFO {}
unsafe impl ::windows::runtime::Abi for DISPLAYCONFIG_VIDEO_SIGNAL_INFO {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_DisplayDevices`*"]
pub union DISPLAYCONFIG_VIDEO_SIGNAL_INFO_0 {
    pub AdditionalSignalInfo: DISPLAYCONFIG_VIDEO_SIGNAL_INFO_0_0,
    pub videoStandard: u32,
}
impl DISPLAYCONFIG_VIDEO_SIGNAL_INFO_0 {}
impl ::std::default::Default for DISPLAYCONFIG_VIDEO_SIGNAL_INFO_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for DISPLAYCONFIG_VIDEO_SIGNAL_INFO_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for DISPLAYCONFIG_VIDEO_SIGNAL_INFO_0 {}
unsafe impl ::windows::runtime::Abi for DISPLAYCONFIG_VIDEO_SIGNAL_INFO_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_DisplayDevices`*"]
pub struct DISPLAYCONFIG_VIDEO_SIGNAL_INFO_0_0 {
    pub _bitfield: u32,
}
impl DISPLAYCONFIG_VIDEO_SIGNAL_INFO_0_0 {}
impl ::std::default::Default for DISPLAYCONFIG_VIDEO_SIGNAL_INFO_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DISPLAYCONFIG_VIDEO_SIGNAL_INFO_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_AdditionalSignalInfo_e__Struct").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::std::cmp::PartialEq for DISPLAYCONFIG_VIDEO_SIGNAL_INFO_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for DISPLAYCONFIG_VIDEO_SIGNAL_INFO_0_0 {}
unsafe impl ::windows::runtime::Abi for DISPLAYCONFIG_VIDEO_SIGNAL_INFO_0_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`, `Win32_System_SystemServices`*"]
pub struct DRIVEROBJ {
    pub pvObj: *mut ::std::ffi::c_void,
    pub pFreeProc: ::std::option::Option<super::super::System::SystemServices::FREEOBJPROC>,
    pub hdev: super::super::System::SystemServices::HDEV,
    pub dhpdev: super::super::System::SystemServices::DHPDEV,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl DRIVEROBJ {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::default::Default for DRIVEROBJ {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::fmt::Debug for DRIVEROBJ {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DRIVEROBJ").field("pvObj", &self.pvObj).field("hdev", &self.hdev).field("dhpdev", &self.dhpdev).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::cmp::PartialEq for DRIVEROBJ {
    fn eq(&self, other: &Self) -> bool {
        self.pvObj == other.pvObj && self.pFreeProc.map(|f| f as usize) == other.pFreeProc.map(|f| f as usize) && self.hdev == other.hdev && self.dhpdev == other.dhpdev
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::cmp::Eq for DRIVEROBJ {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
unsafe impl ::windows::runtime::Abi for DRIVEROBJ {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_SystemServices")]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_System_SystemServices`*"]
pub struct DRVENABLEDATA {
    pub iDriverVersion: u32,
    pub c: u32,
    pub pdrvfn: *mut DRVFN,
}
#[cfg(feature = "Win32_System_SystemServices")]
impl DRVENABLEDATA {}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::default::Default for DRVENABLEDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::fmt::Debug for DRVENABLEDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DRVENABLEDATA").field("iDriverVersion", &self.iDriverVersion).field("c", &self.c).field("pdrvfn", &self.pdrvfn).finish()
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::PartialEq for DRVENABLEDATA {
    fn eq(&self, other: &Self) -> bool {
        self.iDriverVersion == other.iDriverVersion && self.c == other.c && self.pdrvfn == other.pdrvfn
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::Eq for DRVENABLEDATA {}
#[cfg(feature = "Win32_System_SystemServices")]
unsafe impl ::windows::runtime::Abi for DRVENABLEDATA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_System_SystemServices")]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_System_SystemServices`*"]
pub struct DRVFN {
    pub iFunc: u32,
    pub pfn: ::std::option::Option<super::super::System::SystemServices::PFN>,
}
#[cfg(feature = "Win32_System_SystemServices")]
impl DRVFN {}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::default::Default for DRVFN {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::fmt::Debug for DRVFN {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DRVFN").field("iFunc", &self.iFunc).finish()
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::PartialEq for DRVFN {
    fn eq(&self, other: &Self) -> bool {
        self.iFunc == other.iFunc && self.pfn.map(|f| f as usize) == other.pfn.map(|f| f as usize)
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::Eq for DRVFN {}
#[cfg(feature = "Win32_System_SystemServices")]
unsafe impl ::windows::runtime::Abi for DRVFN {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DisplayConfigGetDeviceInfo(requestpacket: *mut DISPLAYCONFIG_DEVICE_INFO_HEADER) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DisplayConfigGetDeviceInfo(requestpacket: *mut DISPLAYCONFIG_DEVICE_INFO_HEADER) -> i32;
        }
        ::std::mem::transmute(DisplayConfigGetDeviceInfo(::std::mem::transmute(requestpacket)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DisplayConfigSetDeviceInfo(setpacket: *const DISPLAYCONFIG_DEVICE_INFO_HEADER) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DisplayConfigSetDeviceInfo(setpacket: *const DISPLAYCONFIG_DEVICE_INFO_HEADER) -> i32;
        }
        ::std::mem::transmute(DisplayConfigSetDeviceInfo(::std::mem::transmute(setpacket)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_SystemServices")]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_System_SystemServices`*"]
pub struct ENGSAFESEMAPHORE {
    pub hsem: *mut super::super::System::SystemServices::HSEMAPHORE__,
    pub lCount: i32,
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ENGSAFESEMAPHORE {}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::default::Default for ENGSAFESEMAPHORE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::fmt::Debug for ENGSAFESEMAPHORE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ENGSAFESEMAPHORE").field("hsem", &self.hsem).field("lCount", &self.lCount).finish()
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::PartialEq for ENGSAFESEMAPHORE {
    fn eq(&self, other: &Self) -> bool {
        self.hsem == other.hsem && self.lCount == other.lCount
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::Eq for ENGSAFESEMAPHORE {}
#[cfg(feature = "Win32_System_SystemServices")]
unsafe impl ::windows::runtime::Abi for ENGSAFESEMAPHORE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_DisplayDevices`*"]
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
impl ::std::default::Default for ENG_TIME_FIELDS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for ENG_TIME_FIELDS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
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
impl ::std::cmp::PartialEq for ENG_TIME_FIELDS {
    fn eq(&self, other: &Self) -> bool {
        self.usYear == other.usYear && self.usMonth == other.usMonth && self.usDay == other.usDay && self.usHour == other.usHour && self.usMinute == other.usMinute && self.usSecond == other.usSecond && self.usMilliseconds == other.usMilliseconds && self.usWeekday == other.usWeekday
    }
}
impl ::std::cmp::Eq for ENG_TIME_FIELDS {}
unsafe impl ::windows::runtime::Abi for ENG_TIME_FIELDS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`*"]
pub struct ENUMRECTS {
    pub c: u32,
    pub arcl: [super::super::Foundation::RECTL; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ENUMRECTS {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for ENUMRECTS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for ENUMRECTS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ENUMRECTS").field("c", &self.c).field("arcl", &self.arcl).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for ENUMRECTS {
    fn eq(&self, other: &Self) -> bool {
        self.c == other.c && self.arcl == other.arcl
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for ENUMRECTS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for ENUMRECTS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_System_SystemServices`*"]
#[cfg(feature = "Win32_System_SystemServices")]
#[inline]
pub unsafe fn EngAcquireSemaphore(hsem: *const super::super::System::SystemServices::HSEMAPHORE__) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EngAcquireSemaphore(hsem: *const super::super::System::SystemServices::HSEMAPHORE__);
        }
        ::std::mem::transmute(EngAcquireSemaphore(::std::mem::transmute(hsem)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_System_SystemServices`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
#[inline]
pub unsafe fn EngAlphaBlend(psodest: *mut SURFOBJ, psosrc: *mut SURFOBJ, pco: *mut CLIPOBJ, pxlo: *mut XLATEOBJ, prcldest: *mut super::super::Foundation::RECTL, prclsrc: *mut super::super::Foundation::RECTL, pblendobj: *mut BLENDOBJ) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EngAlphaBlend(psodest: *mut SURFOBJ, psosrc: *mut SURFOBJ, pco: *mut CLIPOBJ, pxlo: *mut XLATEOBJ, prcldest: *mut super::super::Foundation::RECTL, prclsrc: *mut super::super::Foundation::RECTL, pblendobj: *mut BLENDOBJ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(EngAlphaBlend(::std::mem::transmute(psodest), ::std::mem::transmute(psosrc), ::std::mem::transmute(pco), ::std::mem::transmute(pxlo), ::std::mem::transmute(prcldest), ::std::mem::transmute(prclsrc), ::std::mem::transmute(pblendobj)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`, `Win32_System_SystemServices`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
#[inline]
pub unsafe fn EngAssociateSurface<'a, Param0: ::windows::runtime::IntoParam<'a, HSURF>, Param1: ::windows::runtime::IntoParam<'a, super::super::System::SystemServices::HDEV>>(hsurf: Param0, hdev: Param1, flhooks: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EngAssociateSurface(hsurf: HSURF, hdev: super::super::System::SystemServices::HDEV, flhooks: u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(EngAssociateSurface(hsurf.into_param().abi(), hdev.into_param().abi(), ::std::mem::transmute(flhooks)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`, `Win32_System_SystemServices`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
#[inline]
pub unsafe fn EngBitBlt(psotrg: *const SURFOBJ, psosrc: *const SURFOBJ, psomask: *const SURFOBJ, pco: *const CLIPOBJ, pxlo: *const XLATEOBJ, prcltrg: *const super::super::Foundation::RECTL, pptlsrc: *const super::super::Foundation::POINTL, pptlmask: *const super::super::Foundation::POINTL, pbo: *const BRUSHOBJ, pptlbrush: *const super::super::Foundation::POINTL, rop4: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EngBitBlt(psotrg: *const SURFOBJ, psosrc: *const SURFOBJ, psomask: *const SURFOBJ, pco: *const CLIPOBJ, pxlo: *const XLATEOBJ, prcltrg: *const super::super::Foundation::RECTL, pptlsrc: *const super::super::Foundation::POINTL, pptlmask: *const super::super::Foundation::POINTL, pbo: *const BRUSHOBJ, pptlbrush: *const super::super::Foundation::POINTL, rop4: u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(EngBitBlt(
            ::std::mem::transmute(psotrg),
            ::std::mem::transmute(psosrc),
            ::std::mem::transmute(psomask),
            ::std::mem::transmute(pco),
            ::std::mem::transmute(pxlo),
            ::std::mem::transmute(prcltrg),
            ::std::mem::transmute(pptlsrc),
            ::std::mem::transmute(pptlmask),
            ::std::mem::transmute(pbo),
            ::std::mem::transmute(pptlbrush),
            ::std::mem::transmute(rop4),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`, `Win32_System_SystemServices`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
#[inline]
pub unsafe fn EngCheckAbort(pso: *mut SURFOBJ) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EngCheckAbort(pso: *mut SURFOBJ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(EngCheckAbort(::std::mem::transmute(pso)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_DisplayDevices`*"]
#[inline]
pub unsafe fn EngComputeGlyphSet(ncodepage: i32, nfirstchar: i32, cchars: i32) -> *mut FD_GLYPHSET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EngComputeGlyphSet(ncodepage: i32, nfirstchar: i32, cchars: i32) -> *mut FD_GLYPHSET;
        }
        ::std::mem::transmute(EngComputeGlyphSet(::std::mem::transmute(ncodepage), ::std::mem::transmute(nfirstchar), ::std::mem::transmute(cchars)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`, `Win32_System_SystemServices`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
#[inline]
pub unsafe fn EngCopyBits(psodest: *mut SURFOBJ, psosrc: *mut SURFOBJ, pco: *mut CLIPOBJ, pxlo: *mut XLATEOBJ, prcldest: *mut super::super::Foundation::RECTL, pptlsrc: *mut super::super::Foundation::POINTL) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EngCopyBits(psodest: *mut SURFOBJ, psosrc: *mut SURFOBJ, pco: *mut CLIPOBJ, pxlo: *mut XLATEOBJ, prcldest: *mut super::super::Foundation::RECTL, pptlsrc: *mut super::super::Foundation::POINTL) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(EngCopyBits(::std::mem::transmute(psodest), ::std::mem::transmute(psosrc), ::std::mem::transmute(pco), ::std::mem::transmute(pxlo), ::std::mem::transmute(prcldest), ::std::mem::transmute(pptlsrc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn EngCreateBitmap<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::SIZE>>(sizl: Param0, lwidth: i32, iformat: u32, fl: u32, pvbits: *mut ::std::ffi::c_void) -> super::super::Graphics::Gdi::HBITMAP {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EngCreateBitmap(sizl: super::super::Foundation::SIZE, lwidth: i32, iformat: u32, fl: u32, pvbits: *mut ::std::ffi::c_void) -> super::super::Graphics::Gdi::HBITMAP;
        }
        ::std::mem::transmute(EngCreateBitmap(sizl.into_param().abi(), ::std::mem::transmute(lwidth), ::std::mem::transmute(iformat), ::std::mem::transmute(fl), ::std::mem::transmute(pvbits)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EngCreateClip() -> *mut CLIPOBJ {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EngCreateClip() -> *mut CLIPOBJ;
        }
        ::std::mem::transmute(EngCreateClip())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_System_SystemServices`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
#[inline]
pub unsafe fn EngCreateDeviceBitmap<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::SystemServices::DHSURF>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::SIZE>>(dhsurf: Param0, sizl: Param1, iformatcompat: u32) -> super::super::Graphics::Gdi::HBITMAP {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EngCreateDeviceBitmap(dhsurf: super::super::System::SystemServices::DHSURF, sizl: super::super::Foundation::SIZE, iformatcompat: u32) -> super::super::Graphics::Gdi::HBITMAP;
        }
        ::std::mem::transmute(EngCreateDeviceBitmap(dhsurf.into_param().abi(), sizl.into_param().abi(), ::std::mem::transmute(iformatcompat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`, `Win32_System_SystemServices`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
#[inline]
pub unsafe fn EngCreateDeviceSurface<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::SystemServices::DHSURF>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::SIZE>>(dhsurf: Param0, sizl: Param1, iformatcompat: u32) -> HSURF {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EngCreateDeviceSurface(dhsurf: super::super::System::SystemServices::DHSURF, sizl: super::super::Foundation::SIZE, iformatcompat: u32) -> HSURF;
        }
        ::std::mem::transmute(EngCreateDeviceSurface(dhsurf.into_param().abi(), sizl.into_param().abi(), ::std::mem::transmute(iformatcompat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Graphics_Gdi`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn EngCreatePalette(imode: u32, ccolors: u32, pulcolors: *mut u32, flred: u32, flgreen: u32, flblue: u32) -> super::super::Graphics::Gdi::HPALETTE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EngCreatePalette(imode: u32, ccolors: u32, pulcolors: *mut u32, flred: u32, flgreen: u32, flblue: u32) -> super::super::Graphics::Gdi::HPALETTE;
        }
        ::std::mem::transmute(EngCreatePalette(::std::mem::transmute(imode), ::std::mem::transmute(ccolors), ::std::mem::transmute(pulcolors), ::std::mem::transmute(flred), ::std::mem::transmute(flgreen), ::std::mem::transmute(flblue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_System_SystemServices`*"]
#[cfg(feature = "Win32_System_SystemServices")]
#[inline]
pub unsafe fn EngCreateSemaphore() -> *mut super::super::System::SystemServices::HSEMAPHORE__ {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EngCreateSemaphore() -> *mut super::super::System::SystemServices::HSEMAPHORE__;
        }
        ::std::mem::transmute(EngCreateSemaphore())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EngDeleteClip(pco: *const CLIPOBJ) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EngDeleteClip(pco: *const CLIPOBJ);
        }
        ::std::mem::transmute(EngDeleteClip(::std::mem::transmute(pco)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn EngDeletePalette<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HPALETTE>>(hpal: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EngDeletePalette(hpal: super::super::Graphics::Gdi::HPALETTE) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(EngDeletePalette(hpal.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_DisplayDevices`*"]
#[inline]
pub unsafe fn EngDeletePath(ppo: *mut PATHOBJ) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EngDeletePath(ppo: *mut PATHOBJ);
        }
        ::std::mem::transmute(EngDeletePath(::std::mem::transmute(ppo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_System_SystemServices`*"]
#[cfg(feature = "Win32_System_SystemServices")]
#[inline]
pub unsafe fn EngDeleteSemaphore(hsem: *const super::super::System::SystemServices::HSEMAPHORE__) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EngDeleteSemaphore(hsem: *const super::super::System::SystemServices::HSEMAPHORE__);
        }
        ::std::mem::transmute(EngDeleteSemaphore(::std::mem::transmute(hsem)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EngDeleteSurface<'a, Param0: ::windows::runtime::IntoParam<'a, HSURF>>(hsurf: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EngDeleteSurface(hsurf: HSURF) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(EngDeleteSurface(hsurf.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`, `Win32_System_SystemServices`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
#[inline]
pub unsafe fn EngEraseSurface(pso: *mut SURFOBJ, prcl: *mut super::super::Foundation::RECTL, icolor: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EngEraseSurface(pso: *mut SURFOBJ, prcl: *mut super::super::Foundation::RECTL, icolor: u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(EngEraseSurface(::std::mem::transmute(pso), ::std::mem::transmute(prcl), ::std::mem::transmute(icolor)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`, `Win32_System_SystemServices`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
#[inline]
pub unsafe fn EngFillPath(pso: *mut SURFOBJ, ppo: *mut PATHOBJ, pco: *mut CLIPOBJ, pbo: *mut BRUSHOBJ, pptlbrushorg: *mut super::super::Foundation::POINTL, mix: u32, floptions: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EngFillPath(pso: *mut SURFOBJ, ppo: *mut PATHOBJ, pco: *mut CLIPOBJ, pbo: *mut BRUSHOBJ, pptlbrushorg: *mut super::super::Foundation::POINTL, mix: u32, floptions: u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(EngFillPath(::std::mem::transmute(pso), ::std::mem::transmute(ppo), ::std::mem::transmute(pco), ::std::mem::transmute(pbo), ::std::mem::transmute(pptlbrushorg), ::std::mem::transmute(mix), ::std::mem::transmute(floptions)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EngFindResource<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(h: Param0, iname: i32, itype: i32, pulsize: *mut u32) -> *mut ::std::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EngFindResource(h: super::super::Foundation::HANDLE, iname: i32, itype: i32, pulsize: *mut u32) -> *mut ::std::ffi::c_void;
        }
        ::std::mem::transmute(EngFindResource(h.into_param().abi(), ::std::mem::transmute(iname), ::std::mem::transmute(itype), ::std::mem::transmute(pulsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EngFreeModule<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(h: Param0) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EngFreeModule(h: super::super::Foundation::HANDLE);
        }
        ::std::mem::transmute(EngFreeModule(h.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_DisplayDevices`*"]
#[inline]
pub unsafe fn EngGetCurrentCodePage(oemcodepage: *mut u16, ansicodepage: *mut u16) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EngGetCurrentCodePage(oemcodepage: *mut u16, ansicodepage: *mut u16);
        }
        ::std::mem::transmute(EngGetCurrentCodePage(::std::mem::transmute(oemcodepage), ::std::mem::transmute(ansicodepage)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`, `Win32_System_SystemServices`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
#[inline]
pub unsafe fn EngGetDriverName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::SystemServices::HDEV>>(hdev: Param0) -> super::super::Foundation::PWSTR {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EngGetDriverName(hdev: super::super::System::SystemServices::HDEV) -> super::super::Foundation::PWSTR;
        }
        ::std::mem::transmute(EngGetDriverName(hdev.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`, `Win32_System_SystemServices`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
#[inline]
pub unsafe fn EngGetPrinterDataFileName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::SystemServices::HDEV>>(hdev: Param0) -> super::super::Foundation::PWSTR {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EngGetPrinterDataFileName(hdev: super::super::System::SystemServices::HDEV) -> super::super::Foundation::PWSTR;
        }
        ::std::mem::transmute(EngGetPrinterDataFileName(hdev.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_System_SystemServices`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
#[inline]
pub unsafe fn EngGradientFill(psodest: *mut SURFOBJ, pco: *mut CLIPOBJ, pxlo: *mut XLATEOBJ, pvertex: *mut super::super::Graphics::Gdi::TRIVERTEX, nvertex: u32, pmesh: *mut ::std::ffi::c_void, nmesh: u32, prclextents: *mut super::super::Foundation::RECTL, pptlditherorg: *mut super::super::Foundation::POINTL, ulmode: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EngGradientFill(psodest: *mut SURFOBJ, pco: *mut CLIPOBJ, pxlo: *mut XLATEOBJ, pvertex: *mut super::super::Graphics::Gdi::TRIVERTEX, nvertex: u32, pmesh: *mut ::std::ffi::c_void, nmesh: u32, prclextents: *mut super::super::Foundation::RECTL, pptlditherorg: *mut super::super::Foundation::POINTL, ulmode: u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(EngGradientFill(
            ::std::mem::transmute(psodest),
            ::std::mem::transmute(pco),
            ::std::mem::transmute(pxlo),
            ::std::mem::transmute(pvertex),
            ::std::mem::transmute(nvertex),
            ::std::mem::transmute(pmesh),
            ::std::mem::transmute(nmesh),
            ::std::mem::transmute(prclextents),
            ::std::mem::transmute(pptlditherorg),
            ::std::mem::transmute(ulmode),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`, `Win32_System_SystemServices`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
#[inline]
pub unsafe fn EngLineTo(pso: *mut SURFOBJ, pco: *mut CLIPOBJ, pbo: *mut BRUSHOBJ, x1: i32, y1: i32, x2: i32, y2: i32, prclbounds: *mut super::super::Foundation::RECTL, mix: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EngLineTo(pso: *mut SURFOBJ, pco: *mut CLIPOBJ, pbo: *mut BRUSHOBJ, x1: i32, y1: i32, x2: i32, y2: i32, prclbounds: *mut super::super::Foundation::RECTL, mix: u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(EngLineTo(::std::mem::transmute(pso), ::std::mem::transmute(pco), ::std::mem::transmute(pbo), ::std::mem::transmute(x1), ::std::mem::transmute(y1), ::std::mem::transmute(x2), ::std::mem::transmute(y2), ::std::mem::transmute(prclbounds), ::std::mem::transmute(mix)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EngLoadModule<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(pwsz: Param0) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EngLoadModule(pwsz: super::super::Foundation::PWSTR) -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(EngLoadModule(pwsz.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`, `Win32_System_SystemServices`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
#[inline]
pub unsafe fn EngLockSurface<'a, Param0: ::windows::runtime::IntoParam<'a, HSURF>>(hsurf: Param0) -> *mut SURFOBJ {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EngLockSurface(hsurf: HSURF) -> *mut SURFOBJ;
        }
        ::std::mem::transmute(EngLockSurface(hsurf.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EngMarkBandingSurface<'a, Param0: ::windows::runtime::IntoParam<'a, HSURF>>(hsurf: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EngMarkBandingSurface(hsurf: HSURF) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(EngMarkBandingSurface(hsurf.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EngMultiByteToUnicodeN<'a, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(unicodestring: super::super::Foundation::PWSTR, maxbytesinunicodestring: u32, bytesinunicodestring: *mut u32, multibytestring: Param3, bytesinmultibytestring: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EngMultiByteToUnicodeN(unicodestring: super::super::Foundation::PWSTR, maxbytesinunicodestring: u32, bytesinunicodestring: *mut u32, multibytestring: super::super::Foundation::PSTR, bytesinmultibytestring: u32);
        }
        ::std::mem::transmute(EngMultiByteToUnicodeN(::std::mem::transmute(unicodestring), ::std::mem::transmute(maxbytesinunicodestring), ::std::mem::transmute(bytesinunicodestring), multibytestring.into_param().abi(), ::std::mem::transmute(bytesinmultibytestring)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EngMultiByteToWideChar<'a, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(codepage: u32, widecharstring: super::super::Foundation::PWSTR, bytesinwidecharstring: i32, multibytestring: Param3, bytesinmultibytestring: i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EngMultiByteToWideChar(codepage: u32, widecharstring: super::super::Foundation::PWSTR, bytesinwidecharstring: i32, multibytestring: super::super::Foundation::PSTR, bytesinmultibytestring: i32) -> i32;
        }
        ::std::mem::transmute(EngMultiByteToWideChar(::std::mem::transmute(codepage), ::std::mem::transmute(widecharstring), ::std::mem::transmute(bytesinwidecharstring), multibytestring.into_param().abi(), ::std::mem::transmute(bytesinmultibytestring)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`, `Win32_System_SystemServices`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
#[inline]
pub unsafe fn EngPaint(pso: *mut SURFOBJ, pco: *mut CLIPOBJ, pbo: *mut BRUSHOBJ, pptlbrushorg: *mut super::super::Foundation::POINTL, mix: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EngPaint(pso: *mut SURFOBJ, pco: *mut CLIPOBJ, pbo: *mut BRUSHOBJ, pptlbrushorg: *mut super::super::Foundation::POINTL, mix: u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(EngPaint(::std::mem::transmute(pso), ::std::mem::transmute(pco), ::std::mem::transmute(pbo), ::std::mem::transmute(pptlbrushorg), ::std::mem::transmute(mix)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_System_SystemServices`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
#[inline]
pub unsafe fn EngPlgBlt(psotrg: *mut SURFOBJ, psosrc: *mut SURFOBJ, psomsk: *mut SURFOBJ, pco: *mut CLIPOBJ, pxlo: *mut XLATEOBJ, pca: *mut super::super::Graphics::Gdi::COLORADJUSTMENT, pptlbrushorg: *mut super::super::Foundation::POINTL, pptfx: *mut super::super::System::SystemServices::POINTFIX, prcl: *mut super::super::Foundation::RECTL, pptl: *mut super::super::Foundation::POINTL, imode: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EngPlgBlt(psotrg: *mut SURFOBJ, psosrc: *mut SURFOBJ, psomsk: *mut SURFOBJ, pco: *mut CLIPOBJ, pxlo: *mut XLATEOBJ, pca: *mut super::super::Graphics::Gdi::COLORADJUSTMENT, pptlbrushorg: *mut super::super::Foundation::POINTL, pptfx: *mut super::super::System::SystemServices::POINTFIX, prcl: *mut super::super::Foundation::RECTL, pptl: *mut super::super::Foundation::POINTL, imode: u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(EngPlgBlt(
            ::std::mem::transmute(psotrg),
            ::std::mem::transmute(psosrc),
            ::std::mem::transmute(psomsk),
            ::std::mem::transmute(pco),
            ::std::mem::transmute(pxlo),
            ::std::mem::transmute(pca),
            ::std::mem::transmute(pptlbrushorg),
            ::std::mem::transmute(pptfx),
            ::std::mem::transmute(prcl),
            ::std::mem::transmute(pptl),
            ::std::mem::transmute(imode),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_DisplayDevices`*"]
#[inline]
pub unsafe fn EngQueryLocalTime(param0: *mut ENG_TIME_FIELDS) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EngQueryLocalTime(param0: *mut ENG_TIME_FIELDS);
        }
        ::std::mem::transmute(EngQueryLocalTime(::std::mem::transmute(param0)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_System_SystemServices`*"]
#[cfg(feature = "Win32_System_SystemServices")]
#[inline]
pub unsafe fn EngReleaseSemaphore(hsem: *const super::super::System::SystemServices::HSEMAPHORE__) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EngReleaseSemaphore(hsem: *const super::super::System::SystemServices::HSEMAPHORE__);
        }
        ::std::mem::transmute(EngReleaseSemaphore(::std::mem::transmute(hsem)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_System_SystemServices`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
#[inline]
pub unsafe fn EngStretchBlt(psodest: *mut SURFOBJ, psosrc: *mut SURFOBJ, psomask: *mut SURFOBJ, pco: *mut CLIPOBJ, pxlo: *mut XLATEOBJ, pca: *mut super::super::Graphics::Gdi::COLORADJUSTMENT, pptlhtorg: *mut super::super::Foundation::POINTL, prcldest: *mut super::super::Foundation::RECTL, prclsrc: *mut super::super::Foundation::RECTL, pptlmask: *mut super::super::Foundation::POINTL, imode: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EngStretchBlt(psodest: *mut SURFOBJ, psosrc: *mut SURFOBJ, psomask: *mut SURFOBJ, pco: *mut CLIPOBJ, pxlo: *mut XLATEOBJ, pca: *mut super::super::Graphics::Gdi::COLORADJUSTMENT, pptlhtorg: *mut super::super::Foundation::POINTL, prcldest: *mut super::super::Foundation::RECTL, prclsrc: *mut super::super::Foundation::RECTL, pptlmask: *mut super::super::Foundation::POINTL, imode: u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(EngStretchBlt(
            ::std::mem::transmute(psodest),
            ::std::mem::transmute(psosrc),
            ::std::mem::transmute(psomask),
            ::std::mem::transmute(pco),
            ::std::mem::transmute(pxlo),
            ::std::mem::transmute(pca),
            ::std::mem::transmute(pptlhtorg),
            ::std::mem::transmute(prcldest),
            ::std::mem::transmute(prclsrc),
            ::std::mem::transmute(pptlmask),
            ::std::mem::transmute(imode),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_System_SystemServices`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
#[inline]
pub unsafe fn EngStretchBltROP(psodest: *mut SURFOBJ, psosrc: *mut SURFOBJ, psomask: *mut SURFOBJ, pco: *mut CLIPOBJ, pxlo: *mut XLATEOBJ, pca: *mut super::super::Graphics::Gdi::COLORADJUSTMENT, pptlhtorg: *mut super::super::Foundation::POINTL, prcldest: *mut super::super::Foundation::RECTL, prclsrc: *mut super::super::Foundation::RECTL, pptlmask: *mut super::super::Foundation::POINTL, imode: u32, pbo: *mut BRUSHOBJ, rop4: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EngStretchBltROP(psodest: *mut SURFOBJ, psosrc: *mut SURFOBJ, psomask: *mut SURFOBJ, pco: *mut CLIPOBJ, pxlo: *mut XLATEOBJ, pca: *mut super::super::Graphics::Gdi::COLORADJUSTMENT, pptlhtorg: *mut super::super::Foundation::POINTL, prcldest: *mut super::super::Foundation::RECTL, prclsrc: *mut super::super::Foundation::RECTL, pptlmask: *mut super::super::Foundation::POINTL, imode: u32, pbo: *mut BRUSHOBJ, rop4: u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(EngStretchBltROP(
            ::std::mem::transmute(psodest),
            ::std::mem::transmute(psosrc),
            ::std::mem::transmute(psomask),
            ::std::mem::transmute(pco),
            ::std::mem::transmute(pxlo),
            ::std::mem::transmute(pca),
            ::std::mem::transmute(pptlhtorg),
            ::std::mem::transmute(prcldest),
            ::std::mem::transmute(prclsrc),
            ::std::mem::transmute(pptlmask),
            ::std::mem::transmute(imode),
            ::std::mem::transmute(pbo),
            ::std::mem::transmute(rop4),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`, `Win32_System_SystemServices`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
#[inline]
pub unsafe fn EngStrokeAndFillPath(pso: *mut SURFOBJ, ppo: *mut PATHOBJ, pco: *mut CLIPOBJ, pxo: *mut super::super::System::SystemServices::XFORMOBJ, pbostroke: *mut BRUSHOBJ, plineattrs: *mut LINEATTRS, pbofill: *mut BRUSHOBJ, pptlbrushorg: *mut super::super::Foundation::POINTL, mixfill: u32, floptions: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EngStrokeAndFillPath(pso: *mut SURFOBJ, ppo: *mut PATHOBJ, pco: *mut CLIPOBJ, pxo: *mut super::super::System::SystemServices::XFORMOBJ, pbostroke: *mut BRUSHOBJ, plineattrs: *mut LINEATTRS, pbofill: *mut BRUSHOBJ, pptlbrushorg: *mut super::super::Foundation::POINTL, mixfill: u32, floptions: u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(EngStrokeAndFillPath(
            ::std::mem::transmute(pso),
            ::std::mem::transmute(ppo),
            ::std::mem::transmute(pco),
            ::std::mem::transmute(pxo),
            ::std::mem::transmute(pbostroke),
            ::std::mem::transmute(plineattrs),
            ::std::mem::transmute(pbofill),
            ::std::mem::transmute(pptlbrushorg),
            ::std::mem::transmute(mixfill),
            ::std::mem::transmute(floptions),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`, `Win32_System_SystemServices`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
#[inline]
pub unsafe fn EngStrokePath(pso: *mut SURFOBJ, ppo: *mut PATHOBJ, pco: *mut CLIPOBJ, pxo: *mut super::super::System::SystemServices::XFORMOBJ, pbo: *mut BRUSHOBJ, pptlbrushorg: *mut super::super::Foundation::POINTL, plineattrs: *mut LINEATTRS, mix: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EngStrokePath(pso: *mut SURFOBJ, ppo: *mut PATHOBJ, pco: *mut CLIPOBJ, pxo: *mut super::super::System::SystemServices::XFORMOBJ, pbo: *mut BRUSHOBJ, pptlbrushorg: *mut super::super::Foundation::POINTL, plineattrs: *mut LINEATTRS, mix: u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(EngStrokePath(::std::mem::transmute(pso), ::std::mem::transmute(ppo), ::std::mem::transmute(pco), ::std::mem::transmute(pxo), ::std::mem::transmute(pbo), ::std::mem::transmute(pptlbrushorg), ::std::mem::transmute(plineattrs), ::std::mem::transmute(mix)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`, `Win32_System_SystemServices`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
#[inline]
pub unsafe fn EngTextOut(pso: *mut SURFOBJ, pstro: *mut STROBJ, pfo: *mut FONTOBJ, pco: *mut CLIPOBJ, prclextra: *mut super::super::Foundation::RECTL, prclopaque: *mut super::super::Foundation::RECTL, pbofore: *mut BRUSHOBJ, pboopaque: *mut BRUSHOBJ, pptlorg: *mut super::super::Foundation::POINTL, mix: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EngTextOut(pso: *mut SURFOBJ, pstro: *mut STROBJ, pfo: *mut FONTOBJ, pco: *mut CLIPOBJ, prclextra: *mut super::super::Foundation::RECTL, prclopaque: *mut super::super::Foundation::RECTL, pbofore: *mut BRUSHOBJ, pboopaque: *mut BRUSHOBJ, pptlorg: *mut super::super::Foundation::POINTL, mix: u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(EngTextOut(
            ::std::mem::transmute(pso),
            ::std::mem::transmute(pstro),
            ::std::mem::transmute(pfo),
            ::std::mem::transmute(pco),
            ::std::mem::transmute(prclextra),
            ::std::mem::transmute(prclopaque),
            ::std::mem::transmute(pbofore),
            ::std::mem::transmute(pboopaque),
            ::std::mem::transmute(pptlorg),
            ::std::mem::transmute(mix),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`, `Win32_System_SystemServices`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
#[inline]
pub unsafe fn EngTransparentBlt(psodst: *const SURFOBJ, psosrc: *const SURFOBJ, pco: *const CLIPOBJ, pxlo: *const XLATEOBJ, prcldst: *const super::super::Foundation::RECTL, prclsrc: *const super::super::Foundation::RECTL, transcolor: u32, bcalledfrombitblt: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EngTransparentBlt(psodst: *const SURFOBJ, psosrc: *const SURFOBJ, pco: *const CLIPOBJ, pxlo: *const XLATEOBJ, prcldst: *const super::super::Foundation::RECTL, prclsrc: *const super::super::Foundation::RECTL, transcolor: u32, bcalledfrombitblt: u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(EngTransparentBlt(::std::mem::transmute(psodst), ::std::mem::transmute(psosrc), ::std::mem::transmute(pco), ::std::mem::transmute(pxlo), ::std::mem::transmute(prcldst), ::std::mem::transmute(prclsrc), ::std::mem::transmute(transcolor), ::std::mem::transmute(bcalledfrombitblt)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EngUnicodeToMultiByteN<'a, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(multibytestring: super::super::Foundation::PSTR, maxbytesinmultibytestring: u32, bytesinmultibytestring: *mut u32, unicodestring: Param3, bytesinunicodestring: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EngUnicodeToMultiByteN(multibytestring: super::super::Foundation::PSTR, maxbytesinmultibytestring: u32, bytesinmultibytestring: *mut u32, unicodestring: super::super::Foundation::PWSTR, bytesinunicodestring: u32);
        }
        ::std::mem::transmute(EngUnicodeToMultiByteN(::std::mem::transmute(multibytestring), ::std::mem::transmute(maxbytesinmultibytestring), ::std::mem::transmute(bytesinmultibytestring), unicodestring.into_param().abi(), ::std::mem::transmute(bytesinunicodestring)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`, `Win32_System_SystemServices`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
#[inline]
pub unsafe fn EngUnlockSurface(pso: *mut SURFOBJ) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EngUnlockSurface(pso: *mut SURFOBJ);
        }
        ::std::mem::transmute(EngUnlockSurface(::std::mem::transmute(pso)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EngWideCharToMultiByte<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(codepage: u32, widecharstring: Param1, bytesinwidecharstring: i32, multibytestring: super::super::Foundation::PSTR, bytesinmultibytestring: i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EngWideCharToMultiByte(codepage: u32, widecharstring: super::super::Foundation::PWSTR, bytesinwidecharstring: i32, multibytestring: super::super::Foundation::PSTR, bytesinmultibytestring: i32) -> i32;
        }
        ::std::mem::transmute(EngWideCharToMultiByte(::std::mem::transmute(codepage), widecharstring.into_param().abi(), ::std::mem::transmute(bytesinwidecharstring), ::std::mem::transmute(multibytestring), ::std::mem::transmute(bytesinmultibytestring)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`, `Win32_System_SystemServices`*"]
pub struct FD_DEVICEMETRICS {
    pub flRealizedType: u32,
    pub pteBase: super::super::System::SystemServices::POINTE,
    pub pteSide: super::super::System::SystemServices::POINTE,
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl FD_DEVICEMETRICS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::default::Default for FD_DEVICEMETRICS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::fmt::Debug for FD_DEVICEMETRICS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::cmp::PartialEq for FD_DEVICEMETRICS {
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::cmp::Eq for FD_DEVICEMETRICS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
unsafe impl ::windows::runtime::Abi for FD_DEVICEMETRICS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_DisplayDevices`*"]
pub struct FD_GLYPHATTR {
    pub cjThis: u32,
    pub cGlyphs: u32,
    pub iMode: u32,
    pub aGlyphAttr: [u8; 1],
}
impl FD_GLYPHATTR {}
impl ::std::default::Default for FD_GLYPHATTR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FD_GLYPHATTR {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FD_GLYPHATTR").field("cjThis", &self.cjThis).field("cGlyphs", &self.cGlyphs).field("iMode", &self.iMode).field("aGlyphAttr", &self.aGlyphAttr).finish()
    }
}
impl ::std::cmp::PartialEq for FD_GLYPHATTR {
    fn eq(&self, other: &Self) -> bool {
        self.cjThis == other.cjThis && self.cGlyphs == other.cGlyphs && self.iMode == other.iMode && self.aGlyphAttr == other.aGlyphAttr
    }
}
impl ::std::cmp::Eq for FD_GLYPHATTR {}
unsafe impl ::windows::runtime::Abi for FD_GLYPHATTR {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_DisplayDevices`*"]
pub struct FD_GLYPHSET {
    pub cjThis: u32,
    pub flAccel: u32,
    pub cGlyphsSupported: u32,
    pub cRuns: u32,
    pub awcrun: [WCRUN; 1],
}
impl FD_GLYPHSET {}
impl ::std::default::Default for FD_GLYPHSET {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FD_GLYPHSET {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FD_GLYPHSET").field("cjThis", &self.cjThis).field("flAccel", &self.flAccel).field("cGlyphsSupported", &self.cGlyphsSupported).field("cRuns", &self.cRuns).field("awcrun", &self.awcrun).finish()
    }
}
impl ::std::cmp::PartialEq for FD_GLYPHSET {
    fn eq(&self, other: &Self) -> bool {
        self.cjThis == other.cjThis && self.flAccel == other.flAccel && self.cGlyphsSupported == other.cGlyphsSupported && self.cRuns == other.cRuns && self.awcrun == other.awcrun
    }
}
impl ::std::cmp::Eq for FD_GLYPHSET {}
unsafe impl ::windows::runtime::Abi for FD_GLYPHSET {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_DisplayDevices`*"]
pub struct FD_KERNINGPAIR {
    pub wcFirst: u16,
    pub wcSecond: u16,
    pub fwdKern: i16,
}
impl FD_KERNINGPAIR {}
impl ::std::default::Default for FD_KERNINGPAIR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FD_KERNINGPAIR {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FD_KERNINGPAIR").field("wcFirst", &self.wcFirst).field("wcSecond", &self.wcSecond).field("fwdKern", &self.fwdKern).finish()
    }
}
impl ::std::cmp::PartialEq for FD_KERNINGPAIR {
    fn eq(&self, other: &Self) -> bool {
        self.wcFirst == other.wcFirst && self.wcSecond == other.wcSecond && self.fwdKern == other.fwdKern
    }
}
impl ::std::cmp::Eq for FD_KERNINGPAIR {}
unsafe impl ::windows::runtime::Abi for FD_KERNINGPAIR {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_DisplayDevices`*"]
pub struct FD_XFORM {
    pub eXX: f32,
    pub eXY: f32,
    pub eYX: f32,
    pub eYY: f32,
}
impl FD_XFORM {}
impl ::std::default::Default for FD_XFORM {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FD_XFORM {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FD_XFORM").field("eXX", &self.eXX).field("eXY", &self.eXY).field("eYX", &self.eYX).field("eYY", &self.eYY).finish()
    }
}
impl ::std::cmp::PartialEq for FD_XFORM {
    fn eq(&self, other: &Self) -> bool {
        self.eXX == other.eXX && self.eXY == other.eXY && self.eYX == other.eYX && self.eYY == other.eYY
    }
}
impl ::std::cmp::Eq for FD_XFORM {}
unsafe impl ::windows::runtime::Abi for FD_XFORM {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`*"]
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
impl ::std::default::Default for FONTDIFF {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for FONTDIFF {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
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
impl ::std::cmp::PartialEq for FONTDIFF {
    fn eq(&self, other: &Self) -> bool {
        self.jReserved1 == other.jReserved1 && self.jReserved2 == other.jReserved2 && self.jReserved3 == other.jReserved3 && self.bWeight == other.bWeight && self.usWinWeight == other.usWinWeight && self.fsSelection == other.fsSelection && self.fwdAveCharWidth == other.fwdAveCharWidth && self.fwdMaxCharInc == other.fwdMaxCharInc && self.ptlCaret == other.ptlCaret
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for FONTDIFF {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for FONTDIFF {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_DisplayDevices`*"]
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
impl ::std::default::Default for FONTINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FONTINFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
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
impl ::std::cmp::PartialEq for FONTINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cjThis == other.cjThis && self.flCaps == other.flCaps && self.cGlyphsSupported == other.cGlyphsSupported && self.cjMaxGlyph1 == other.cjMaxGlyph1 && self.cjMaxGlyph4 == other.cjMaxGlyph4 && self.cjMaxGlyph8 == other.cjMaxGlyph8 && self.cjMaxGlyph32 == other.cjMaxGlyph32
    }
}
impl ::std::cmp::Eq for FONTINFO {}
unsafe impl ::windows::runtime::Abi for FONTINFO {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`*"]
pub struct FONTOBJ {
    pub iUniq: u32,
    pub iFace: u32,
    pub cxMax: u32,
    pub flFontType: u32,
    pub iTTUniq: usize,
    pub iFile: usize,
    pub sizLogResPpi: super::super::Foundation::SIZE,
    pub ulStyleSize: u32,
    pub pvConsumer: *mut ::std::ffi::c_void,
    pub pvProducer: *mut ::std::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl FONTOBJ {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for FONTOBJ {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for FONTOBJ {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
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
impl ::std::cmp::PartialEq for FONTOBJ {
    fn eq(&self, other: &Self) -> bool {
        self.iUniq == other.iUniq && self.iFace == other.iFace && self.cxMax == other.cxMax && self.flFontType == other.flFontType && self.iTTUniq == other.iTTUniq && self.iFile == other.iFile && self.sizLogResPpi == other.sizLogResPpi && self.ulStyleSize == other.ulStyleSize && self.pvConsumer == other.pvConsumer && self.pvProducer == other.pvProducer
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for FONTOBJ {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for FONTOBJ {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FONTOBJ_cGetAllGlyphHandles(pfo: *mut FONTOBJ, phg: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FONTOBJ_cGetAllGlyphHandles(pfo: *mut FONTOBJ, phg: *mut u32) -> u32;
        }
        ::std::mem::transmute(FONTOBJ_cGetAllGlyphHandles(::std::mem::transmute(pfo), ::std::mem::transmute(phg)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FONTOBJ_cGetGlyphs(pfo: *mut FONTOBJ, imode: u32, cglyph: u32, phg: *mut u32, ppvglyph: *mut *mut ::std::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FONTOBJ_cGetGlyphs(pfo: *mut FONTOBJ, imode: u32, cglyph: u32, phg: *mut u32, ppvglyph: *mut *mut ::std::ffi::c_void) -> u32;
        }
        ::std::mem::transmute(FONTOBJ_cGetGlyphs(::std::mem::transmute(pfo), ::std::mem::transmute(imode), ::std::mem::transmute(cglyph), ::std::mem::transmute(phg), ::std::mem::transmute(ppvglyph)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FONTOBJ_pQueryGlyphAttrs(pfo: *mut FONTOBJ, imode: u32) -> *mut FD_GLYPHATTR {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FONTOBJ_pQueryGlyphAttrs(pfo: *mut FONTOBJ, imode: u32) -> *mut FD_GLYPHATTR;
        }
        ::std::mem::transmute(FONTOBJ_pQueryGlyphAttrs(::std::mem::transmute(pfo), ::std::mem::transmute(imode)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FONTOBJ_pfdg(pfo: *mut FONTOBJ) -> *mut FD_GLYPHSET {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FONTOBJ_pfdg(pfo: *mut FONTOBJ) -> *mut FD_GLYPHSET;
        }
        ::std::mem::transmute(FONTOBJ_pfdg(::std::mem::transmute(pfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn FONTOBJ_pifi(pfo: *const FONTOBJ) -> *mut IFIMETRICS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FONTOBJ_pifi(pfo: *const FONTOBJ) -> *mut IFIMETRICS;
        }
        ::std::mem::transmute(FONTOBJ_pifi(::std::mem::transmute(pfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FONTOBJ_pvTrueTypeFontFile(pfo: *mut FONTOBJ, pcjfile: *mut u32) -> *mut ::std::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FONTOBJ_pvTrueTypeFontFile(pfo: *mut FONTOBJ, pcjfile: *mut u32) -> *mut ::std::ffi::c_void;
        }
        ::std::mem::transmute(FONTOBJ_pvTrueTypeFontFile(::std::mem::transmute(pfo), ::std::mem::transmute(pcjfile)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`, `Win32_System_SystemServices`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
#[inline]
pub unsafe fn FONTOBJ_pxoGetXform(pfo: *const FONTOBJ) -> *mut super::super::System::SystemServices::XFORMOBJ {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FONTOBJ_pxoGetXform(pfo: *const FONTOBJ) -> *mut super::super::System::SystemServices::XFORMOBJ;
        }
        ::std::mem::transmute(FONTOBJ_pxoGetXform(::std::mem::transmute(pfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FONTOBJ_vGetInfo(pfo: *mut FONTOBJ, cjsize: u32, pfi: *mut FONTINFO) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FONTOBJ_vGetInfo(pfo: *mut FONTOBJ, cjsize: u32, pfi: *mut FONTINFO);
        }
        ::std::mem::transmute(FONTOBJ_vGetInfo(::std::mem::transmute(pfo), ::std::mem::transmute(cjsize), ::std::mem::transmute(pfi)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_DisplayDevices`*"]
pub struct FONTSIM {
    pub dpBold: i32,
    pub dpItalic: i32,
    pub dpBoldItalic: i32,
}
impl FONTSIM {}
impl ::std::default::Default for FONTSIM {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FONTSIM {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FONTSIM").field("dpBold", &self.dpBold).field("dpItalic", &self.dpItalic).field("dpBoldItalic", &self.dpBoldItalic).finish()
    }
}
impl ::std::cmp::PartialEq for FONTSIM {
    fn eq(&self, other: &Self) -> bool {
        self.dpBold == other.dpBold && self.dpItalic == other.dpItalic && self.dpBoldItalic == other.dpBoldItalic
    }
}
impl ::std::cmp::Eq for FONTSIM {}
unsafe impl ::windows::runtime::Abi for FONTSIM {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_DisplayDevices`*"]
pub struct GAMMARAMP {
    pub Red: [u16; 256],
    pub Green: [u16; 256],
    pub Blue: [u16; 256],
}
impl GAMMARAMP {}
impl ::std::default::Default for GAMMARAMP {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for GAMMARAMP {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("GAMMARAMP").field("Red", &self.Red).field("Green", &self.Green).field("Blue", &self.Blue).finish()
    }
}
impl ::std::cmp::PartialEq for GAMMARAMP {
    fn eq(&self, other: &Self) -> bool {
        self.Red == other.Red && self.Green == other.Green && self.Blue == other.Blue
    }
}
impl ::std::cmp::Eq for GAMMARAMP {}
unsafe impl ::windows::runtime::Abi for GAMMARAMP {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`*"]
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
impl ::std::default::Default for GDIINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for GDIINFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
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
impl ::std::cmp::PartialEq for GDIINFO {
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
impl ::std::cmp::Eq for GDIINFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for GDIINFO {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`*"]
pub struct GLYPHBITS {
    pub ptlOrigin: super::super::Foundation::POINTL,
    pub sizlBitmap: super::super::Foundation::SIZE,
    pub aj: [u8; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl GLYPHBITS {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for GLYPHBITS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for GLYPHBITS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("GLYPHBITS").field("ptlOrigin", &self.ptlOrigin).field("sizlBitmap", &self.sizlBitmap).field("aj", &self.aj).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for GLYPHBITS {
    fn eq(&self, other: &Self) -> bool {
        self.ptlOrigin == other.ptlOrigin && self.sizlBitmap == other.sizlBitmap && self.aj == other.aj
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for GLYPHBITS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for GLYPHBITS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`, `Win32_System_SystemServices`*"]
pub struct GLYPHDATA {
    pub gdf: GLYPHDEF,
    pub hg: u32,
    pub fxD: i32,
    pub fxA: i32,
    pub fxAB: i32,
    pub fxInkTop: i32,
    pub fxInkBottom: i32,
    pub rclInk: super::super::Foundation::RECTL,
    pub ptqD: super::super::System::SystemServices::POINTQF,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl GLYPHDATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::default::Default for GLYPHDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::cmp::PartialEq for GLYPHDATA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::cmp::Eq for GLYPHDATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
unsafe impl ::windows::runtime::Abi for GLYPHDATA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`*"]
pub union GLYPHDEF {
    pub pgb: *mut GLYPHBITS,
    pub ppo: *mut PATHOBJ,
}
#[cfg(feature = "Win32_Foundation")]
impl GLYPHDEF {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for GLYPHDEF {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for GLYPHDEF {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for GLYPHDEF {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for GLYPHDEF {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`*"]
pub struct GLYPHPOS {
    pub hg: u32,
    pub pgdf: *mut GLYPHDEF,
    pub ptl: super::super::Foundation::POINTL,
}
#[cfg(feature = "Win32_Foundation")]
impl GLYPHPOS {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for GLYPHPOS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for GLYPHPOS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("GLYPHPOS").field("hg", &self.hg).field("pgdf", &self.pgdf).field("ptl", &self.ptl).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for GLYPHPOS {
    fn eq(&self, other: &Self) -> bool {
        self.hg == other.hg && self.pgdf == other.pgdf && self.ptl == other.ptl
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for GLYPHPOS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for GLYPHPOS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_DisplayDevices`*"]
#[inline]
pub unsafe fn GetDisplayConfigBufferSizes(flags: u32, numpatharrayelements: *mut u32, nummodeinfoarrayelements: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetDisplayConfigBufferSizes(flags: u32, numpatharrayelements: *mut u32, nummodeinfoarrayelements: *mut u32) -> i32;
        }
        ::std::mem::transmute(GetDisplayConfigBufferSizes(::std::mem::transmute(flags), ::std::mem::transmute(numpatharrayelements), ::std::mem::transmute(nummodeinfoarrayelements)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq)]
#[repr(transparent)]
pub struct HSEMAPHORE(pub isize);
impl ::std::default::Default for HSEMAPHORE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for HSEMAPHORE {}
unsafe impl ::windows::runtime::Abi for HSEMAPHORE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq)]
#[repr(transparent)]
pub struct HSURF(pub isize);
impl ::std::default::Default for HSURF {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for HSURF {}
unsafe impl ::windows::runtime::Abi for HSURF {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Graphics_Gdi`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HT_Get8BPPFormatPalette(ppaletteentry: *mut super::super::Graphics::Gdi::PALETTEENTRY, redgamma: u16, greengamma: u16, bluegamma: u16) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HT_Get8BPPFormatPalette(ppaletteentry: *mut super::super::Graphics::Gdi::PALETTEENTRY, redgamma: u16, greengamma: u16, bluegamma: u16) -> i32;
        }
        ::std::mem::transmute(HT_Get8BPPFormatPalette(::std::mem::transmute(ppaletteentry), ::std::mem::transmute(redgamma), ::std::mem::transmute(greengamma), ::std::mem::transmute(bluegamma)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn HT_Get8BPPMaskPalette<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(ppaletteentry: *mut super::super::Graphics::Gdi::PALETTEENTRY, use8bppmaskpal: Param1, cmymask: u8, redgamma: u16, greengamma: u16, bluegamma: u16) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HT_Get8BPPMaskPalette(ppaletteentry: *mut super::super::Graphics::Gdi::PALETTEENTRY, use8bppmaskpal: super::super::Foundation::BOOL, cmymask: u8, redgamma: u16, greengamma: u16, bluegamma: u16) -> i32;
        }
        ::std::mem::transmute(HT_Get8BPPMaskPalette(::std::mem::transmute(ppaletteentry), use8bppmaskpal.into_param().abi(), ::std::mem::transmute(cmymask), ::std::mem::transmute(redgamma), ::std::mem::transmute(greengamma), ::std::mem::transmute(bluegamma)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_DisplayDevices`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDirectDrawKernel(pub ::windows::runtime::IUnknown);
impl IDirectDrawKernel {
    #[doc = "*Required features: `Win32_UI_DisplayDevices`*"]
    pub unsafe fn GetCaps(&self, param0: *mut DDKERNELCAPS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: `Win32_UI_DisplayDevices`*"]
    pub unsafe fn GetKernelHandle(&self, param0: *mut usize) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: `Win32_UI_DisplayDevices`*"]
    pub unsafe fn ReleaseKernelHandle(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDirectDrawKernel {
    type Vtable = IDirectDrawKernel_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2371272992, 27144, 4560, [155, 6, 0, 160, 201, 3, 163, 184]);
}
impl ::std::convert::From<IDirectDrawKernel> for ::windows::runtime::IUnknown {
    fn from(value: IDirectDrawKernel) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IDirectDrawKernel> for ::windows::runtime::IUnknown {
    fn from(value: &IDirectDrawKernel) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirectDrawKernel {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDirectDrawKernel {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectDrawKernel_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut DDKERNELCAPS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut usize) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_DisplayDevices`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDirectDrawSurfaceKernel(pub ::windows::runtime::IUnknown);
impl IDirectDrawSurfaceKernel {
    #[doc = "*Required features: `Win32_UI_DisplayDevices`*"]
    pub unsafe fn GetKernelHandle(&self, param0: *mut usize) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(param0)).ok()
    }
    #[doc = "*Required features: `Win32_UI_DisplayDevices`*"]
    pub unsafe fn ReleaseKernelHandle(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDirectDrawSurfaceKernel {
    type Vtable = IDirectDrawSurfaceKernel_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1618304416, 27200, 4560, [155, 6, 0, 160, 201, 3, 163, 184]);
}
impl ::std::convert::From<IDirectDrawSurfaceKernel> for ::windows::runtime::IUnknown {
    fn from(value: IDirectDrawSurfaceKernel) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IDirectDrawSurfaceKernel> for ::windows::runtime::IUnknown {
    fn from(value: &IDirectDrawSurfaceKernel) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirectDrawSurfaceKernel {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDirectDrawSurfaceKernel {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectDrawSurfaceKernel_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, param0: *mut usize) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_DisplayDevices`*"]
pub struct IFIEXTRA {
    pub ulIdentifier: u32,
    pub dpFontSig: i32,
    pub cig: u32,
    pub dpDesignVector: i32,
    pub dpAxesInfoW: i32,
    pub aulReserved: [u32; 1],
}
impl IFIEXTRA {}
impl ::std::default::Default for IFIEXTRA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IFIEXTRA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IFIEXTRA").field("ulIdentifier", &self.ulIdentifier).field("dpFontSig", &self.dpFontSig).field("cig", &self.cig).field("dpDesignVector", &self.dpDesignVector).field("dpAxesInfoW", &self.dpAxesInfoW).field("aulReserved", &self.aulReserved).finish()
    }
}
impl ::std::cmp::PartialEq for IFIEXTRA {
    fn eq(&self, other: &Self) -> bool {
        self.ulIdentifier == other.ulIdentifier && self.dpFontSig == other.dpFontSig && self.cig == other.cig && self.dpDesignVector == other.dpDesignVector && self.dpAxesInfoW == other.dpAxesInfoW && self.aulReserved == other.aulReserved
    }
}
impl ::std::cmp::Eq for IFIEXTRA {}
unsafe impl ::windows::runtime::Abi for IFIEXTRA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
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
    pub Align: *mut ::std::ffi::c_void,
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl IFIMETRICS {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::default::Default for IFIMETRICS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::fmt::Debug for IFIMETRICS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
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
impl ::std::cmp::PartialEq for IFIMETRICS {
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
impl ::std::cmp::Eq for IFIMETRICS {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::runtime::Abi for IFIMETRICS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(any(target_arch = "x86",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
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
impl ::std::default::Default for IFIMETRICS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "x86",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::fmt::Debug for IFIMETRICS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
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
impl ::std::cmp::PartialEq for IFIMETRICS {
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
impl ::std::cmp::Eq for IFIMETRICS {}
#[cfg(any(target_arch = "x86",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::runtime::Abi for IFIMETRICS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_SystemServices")]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_System_SystemServices`*"]
pub struct LINEATTRS {
    pub fl: u32,
    pub iJoin: u32,
    pub iEndCap: u32,
    pub elWidth: super::super::System::SystemServices::FLOAT_LONG,
    pub eMiterLimit: f32,
    pub cstyle: u32,
    pub pstyle: *mut super::super::System::SystemServices::FLOAT_LONG,
    pub elStyleState: super::super::System::SystemServices::FLOAT_LONG,
}
#[cfg(feature = "Win32_System_SystemServices")]
impl LINEATTRS {}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::default::Default for LINEATTRS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::PartialEq for LINEATTRS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::Eq for LINEATTRS {}
#[cfg(feature = "Win32_System_SystemServices")]
unsafe impl ::windows::runtime::Abi for LINEATTRS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_DisplayDevices`*"]
pub struct PALOBJ {
    pub ulReserved: u32,
}
impl PALOBJ {}
impl ::std::default::Default for PALOBJ {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PALOBJ {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PALOBJ").field("ulReserved", &self.ulReserved).finish()
    }
}
impl ::std::cmp::PartialEq for PALOBJ {
    fn eq(&self, other: &Self) -> bool {
        self.ulReserved == other.ulReserved
    }
}
impl ::std::cmp::Eq for PALOBJ {}
unsafe impl ::windows::runtime::Abi for PALOBJ {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_SystemServices")]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_System_SystemServices`*"]
pub struct PATHDATA {
    pub flags: u32,
    pub count: u32,
    pub pptfx: *mut super::super::System::SystemServices::POINTFIX,
}
#[cfg(feature = "Win32_System_SystemServices")]
impl PATHDATA {}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::default::Default for PATHDATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::fmt::Debug for PATHDATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PATHDATA").field("flags", &self.flags).field("count", &self.count).field("pptfx", &self.pptfx).finish()
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::PartialEq for PATHDATA {
    fn eq(&self, other: &Self) -> bool {
        self.flags == other.flags && self.count == other.count && self.pptfx == other.pptfx
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::Eq for PATHDATA {}
#[cfg(feature = "Win32_System_SystemServices")]
unsafe impl ::windows::runtime::Abi for PATHDATA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_DisplayDevices`*"]
pub struct PATHOBJ {
    pub fl: u32,
    pub cCurves: u32,
}
impl PATHOBJ {}
impl ::std::default::Default for PATHOBJ {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PATHOBJ {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PATHOBJ").field("fl", &self.fl).field("cCurves", &self.cCurves).finish()
    }
}
impl ::std::cmp::PartialEq for PATHOBJ {
    fn eq(&self, other: &Self) -> bool {
        self.fl == other.fl && self.cCurves == other.cCurves
    }
}
impl ::std::cmp::Eq for PATHOBJ {}
unsafe impl ::windows::runtime::Abi for PATHOBJ {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`, `Win32_System_SystemServices`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
#[inline]
pub unsafe fn PATHOBJ_bEnum(ppo: *mut PATHOBJ, ppd: *mut PATHDATA) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PATHOBJ_bEnum(ppo: *mut PATHOBJ, ppd: *mut PATHDATA) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(PATHOBJ_bEnum(::std::mem::transmute(ppo), ::std::mem::transmute(ppd)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`, `Win32_System_SystemServices`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
#[inline]
pub unsafe fn PATHOBJ_bEnumClipLines(ppo: *mut PATHOBJ, cb: u32, pcl: *mut CLIPLINE) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PATHOBJ_bEnumClipLines(ppo: *mut PATHOBJ, cb: u32, pcl: *mut CLIPLINE) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(PATHOBJ_bEnumClipLines(::std::mem::transmute(ppo), ::std::mem::transmute(cb), ::std::mem::transmute(pcl)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_DisplayDevices`*"]
#[inline]
pub unsafe fn PATHOBJ_vEnumStart(ppo: *mut PATHOBJ) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PATHOBJ_vEnumStart(ppo: *mut PATHOBJ);
        }
        ::std::mem::transmute(PATHOBJ_vEnumStart(::std::mem::transmute(ppo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`, `Win32_System_SystemServices`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
#[inline]
pub unsafe fn PATHOBJ_vEnumStartClipLines(ppo: *mut PATHOBJ, pco: *mut CLIPOBJ, pso: *mut SURFOBJ, pla: *mut LINEATTRS) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PATHOBJ_vEnumStartClipLines(ppo: *mut PATHOBJ, pco: *mut CLIPOBJ, pso: *mut SURFOBJ, pla: *mut LINEATTRS);
        }
        ::std::mem::transmute(PATHOBJ_vEnumStartClipLines(::std::mem::transmute(ppo), ::std::mem::transmute(pco), ::std::mem::transmute(pso), ::std::mem::transmute(pla)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_System_SystemServices`*"]
#[cfg(feature = "Win32_System_SystemServices")]
#[inline]
pub unsafe fn PATHOBJ_vGetBounds(ppo: *mut PATHOBJ, prectfx: *mut super::super::System::SystemServices::RECTFX) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PATHOBJ_vGetBounds(ppo: *mut PATHOBJ, prectfx: *mut super::super::System::SystemServices::RECTFX);
        }
        ::std::mem::transmute(PATHOBJ_vGetBounds(::std::mem::transmute(ppo), ::std::mem::transmute(prectfx)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
pub type PDD_CANCREATESURFACE = unsafe extern "system" fn(param0: *mut DD_CANCREATESURFACEDATA) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
pub type PDD_COLORCB_COLORCONTROL = unsafe extern "system" fn(param0: *mut DD_COLORCONTROLDATA) -> u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type PDD_CREATEPALETTE = unsafe extern "system" fn(param0: *mut DD_CREATEPALETTEDATA) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
pub type PDD_CREATESURFACE = unsafe extern "system" fn(param0: *mut DD_CREATESURFACEDATA) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
pub type PDD_CREATESURFACEEX = unsafe extern "system" fn(param0: *mut DD_CREATESURFACEEXDATA) -> u32;
#[cfg(feature = "Win32_Graphics_DirectDraw")]
pub type PDD_DESTROYDDLOCAL = unsafe extern "system" fn(param0: *mut super::super::Graphics::DirectDraw::DD_DESTROYDDLOCALDATA) -> u32;
pub type PDD_FLIPTOGDISURFACE = unsafe extern "system" fn(param0: *mut DD_FLIPTOGDISURFACEDATA) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
pub type PDD_FREEDRIVERMEMORY = unsafe extern "system" fn(param0: *mut DD_FREEDRIVERMEMORYDATA) -> u32;
#[cfg(feature = "Win32_Graphics_DirectDraw")]
pub type PDD_GETAVAILDRIVERMEMORY = unsafe extern "system" fn(param0: *mut DD_GETAVAILDRIVERMEMORYDATA) -> u32;
pub type PDD_GETDRIVERINFO = unsafe extern "system" fn(param0: *mut DD_GETDRIVERINFODATA) -> u32;
pub type PDD_GETDRIVERSTATE = unsafe extern "system" fn(param0: *mut DD_GETDRIVERSTATEDATA) -> u32;
pub type PDD_GETSCANLINE = unsafe extern "system" fn(param0: *mut DD_GETSCANLINEDATA) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
pub type PDD_KERNELCB_SYNCSURFACE = unsafe extern "system" fn(param0: *mut DD_SYNCSURFACEDATA) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
pub type PDD_KERNELCB_SYNCVIDEOPORT = unsafe extern "system" fn(param0: *mut DD_SYNCVIDEOPORTDATA) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PDD_MAPMEMORY = unsafe extern "system" fn(param0: *mut DD_MAPMEMORYDATA) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
pub type PDD_MOCOMPCB_BEGINFRAME = unsafe extern "system" fn(param0: *mut DD_BEGINMOCOMPFRAMEDATA) -> u32;
#[cfg(feature = "Win32_Graphics_DirectDraw")]
pub type PDD_MOCOMPCB_CREATE = unsafe extern "system" fn(param0: *mut DD_CREATEMOCOMPDATA) -> u32;
#[cfg(feature = "Win32_Graphics_DirectDraw")]
pub type PDD_MOCOMPCB_DESTROY = unsafe extern "system" fn(param0: *mut DD_DESTROYMOCOMPDATA) -> u32;
#[cfg(feature = "Win32_Graphics_DirectDraw")]
pub type PDD_MOCOMPCB_ENDFRAME = unsafe extern "system" fn(param0: *mut DD_ENDMOCOMPFRAMEDATA) -> u32;
#[cfg(feature = "Win32_Graphics_DirectDraw")]
pub type PDD_MOCOMPCB_GETCOMPBUFFINFO = unsafe extern "system" fn(param0: *mut DD_GETMOCOMPCOMPBUFFDATA) -> u32;
#[cfg(feature = "Win32_Graphics_DirectDraw")]
pub type PDD_MOCOMPCB_GETFORMATS = unsafe extern "system" fn(param0: *mut DD_GETMOCOMPFORMATSDATA) -> u32;
pub type PDD_MOCOMPCB_GETGUIDS = unsafe extern "system" fn(param0: *mut DD_GETMOCOMPGUIDSDATA) -> u32;
#[cfg(feature = "Win32_Graphics_DirectDraw")]
pub type PDD_MOCOMPCB_GETINTERNALINFO = unsafe extern "system" fn(param0: *mut DD_GETINTERNALMOCOMPDATA) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
pub type PDD_MOCOMPCB_QUERYSTATUS = unsafe extern "system" fn(param0: *mut DD_QUERYMOCOMPSTATUSDATA) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
pub type PDD_MOCOMPCB_RENDER = unsafe extern "system" fn(param0: *mut DD_RENDERMOCOMPDATA) -> u32;
pub type PDD_PALCB_DESTROYPALETTE = unsafe extern "system" fn(param0: *mut DD_DESTROYPALETTEDATA) -> u32;
#[cfg(feature = "Win32_Graphics_Gdi")]
pub type PDD_PALCB_SETENTRIES = unsafe extern "system" fn(param0: *mut DD_SETENTRIESDATA) -> u32;
pub type PDD_SETEXCLUSIVEMODE = unsafe extern "system" fn(param0: *mut DD_SETEXCLUSIVEMODEDATA) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
pub type PDD_SURFCB_ADDATTACHEDSURFACE = unsafe extern "system" fn(param0: *mut DD_ADDATTACHEDSURFACEDATA) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
pub type PDD_SURFCB_BLT = unsafe extern "system" fn(param0: *mut ::std::mem::ManuallyDrop<DD_BLTDATA>) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
pub type PDD_SURFCB_DESTROYSURFACE = unsafe extern "system" fn(param0: *mut DD_DESTROYSURFACEDATA) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
pub type PDD_SURFCB_FLIP = unsafe extern "system" fn(param0: *mut DD_FLIPDATA) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
pub type PDD_SURFCB_GETBLTSTATUS = unsafe extern "system" fn(param0: *mut DD_GETBLTSTATUSDATA) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
pub type PDD_SURFCB_GETFLIPSTATUS = unsafe extern "system" fn(param0: *mut DD_GETFLIPSTATUSDATA) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
pub type PDD_SURFCB_LOCK = unsafe extern "system" fn(param0: *mut DD_LOCKDATA) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
pub type PDD_SURFCB_SETCOLORKEY = unsafe extern "system" fn(param0: *mut DD_SETCOLORKEYDATA) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
pub type PDD_SURFCB_SETOVERLAYPOSITION = unsafe extern "system" fn(param0: *mut DD_SETOVERLAYPOSITIONDATA) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
pub type PDD_SURFCB_SETPALETTE = unsafe extern "system" fn(param0: *mut DD_SETPALETTEDATA) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
pub type PDD_SURFCB_UNLOCK = unsafe extern "system" fn(param0: *mut DD_UNLOCKDATA) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
pub type PDD_SURFCB_UPDATEOVERLAY = unsafe extern "system" fn(param0: *mut ::std::mem::ManuallyDrop<DD_UPDATEOVERLAYDATA>) -> u32;
#[cfg(feature = "Win32_Graphics_DirectDraw")]
pub type PDD_VPORTCB_CANCREATEVIDEOPORT = unsafe extern "system" fn(param0: *mut DD_CANCREATEVPORTDATA) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
pub type PDD_VPORTCB_COLORCONTROL = unsafe extern "system" fn(param0: *mut DD_VPORTCOLORDATA) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
pub type PDD_VPORTCB_CREATEVIDEOPORT = unsafe extern "system" fn(param0: *mut DD_CREATEVPORTDATA) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
pub type PDD_VPORTCB_DESTROYVPORT = unsafe extern "system" fn(param0: *mut DD_DESTROYVPORTDATA) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
pub type PDD_VPORTCB_FLIP = unsafe extern "system" fn(param0: *mut DD_FLIPVPORTDATA) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
pub type PDD_VPORTCB_GETBANDWIDTH = unsafe extern "system" fn(param0: *mut DD_GETVPORTBANDWIDTHDATA) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
pub type PDD_VPORTCB_GETFIELD = unsafe extern "system" fn(param0: *mut DD_GETVPORTFIELDDATA) -> u32;
pub type PDD_VPORTCB_GETFLIPSTATUS = unsafe extern "system" fn(param0: *mut DD_GETVPORTFLIPSTATUSDATA) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
pub type PDD_VPORTCB_GETINPUTFORMATS = unsafe extern "system" fn(param0: *mut DD_GETVPORTINPUTFORMATDATA) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
pub type PDD_VPORTCB_GETLINE = unsafe extern "system" fn(param0: *mut DD_GETVPORTLINEDATA) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
pub type PDD_VPORTCB_GETOUTPUTFORMATS = unsafe extern "system" fn(param0: *mut DD_GETVPORTOUTPUTFORMATDATA) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
pub type PDD_VPORTCB_GETSIGNALSTATUS = unsafe extern "system" fn(param0: *mut DD_GETVPORTSIGNALDATA) -> u32;
#[cfg(feature = "Win32_Graphics_DirectDraw")]
pub type PDD_VPORTCB_GETVPORTCONNECT = unsafe extern "system" fn(param0: *mut DD_GETVPORTCONNECTDATA) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
pub type PDD_VPORTCB_UPDATE = unsafe extern "system" fn(param0: *mut DD_UPDATEVPORTDATA) -> u32;
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
pub type PDD_VPORTCB_WAITFORSYNC = unsafe extern "system" fn(param0: *mut DD_WAITFORVPORTSYNCDATA) -> u32;
pub type PDD_WAITFORVERTICALBLANK = unsafe extern "system" fn(param0: *mut DD_WAITFORVERTICALBLANKDATA) -> u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`*"]
pub struct PERBANDINFO {
    pub bRepeatThisBand: super::super::Foundation::BOOL,
    pub szlBand: super::super::Foundation::SIZE,
    pub ulHorzRes: u32,
    pub ulVertRes: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl PERBANDINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for PERBANDINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for PERBANDINFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PERBANDINFO").field("bRepeatThisBand", &self.bRepeatThisBand).field("szlBand", &self.szlBand).field("ulHorzRes", &self.ulHorzRes).field("ulVertRes", &self.ulVertRes).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for PERBANDINFO {
    fn eq(&self, other: &Self) -> bool {
        self.bRepeatThisBand == other.bRepeatThisBand && self.szlBand == other.szlBand && self.ulHorzRes == other.ulHorzRes && self.ulVertRes == other.ulVertRes
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for PERBANDINFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for PERBANDINFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
pub type PFN_DrvQueryGlyphAttrs = unsafe extern "system" fn(param0: *mut FONTOBJ, param1: u32) -> *mut FD_GLYPHATTR;
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn QueryDisplayConfig(flags: u32, numpatharrayelements: *mut u32, patharray: *mut DISPLAYCONFIG_PATH_INFO, nummodeinfoarrayelements: *mut u32, modeinfoarray: *mut DISPLAYCONFIG_MODE_INFO, currenttopologyid: *mut DISPLAYCONFIG_TOPOLOGY_ID) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryDisplayConfig(flags: u32, numpatharrayelements: *mut u32, patharray: *mut DISPLAYCONFIG_PATH_INFO, nummodeinfoarrayelements: *mut u32, modeinfoarray: *mut DISPLAYCONFIG_MODE_INFO, currenttopologyid: *mut DISPLAYCONFIG_TOPOLOGY_ID) -> i32;
        }
        ::std::mem::transmute(QueryDisplayConfig(::std::mem::transmute(flags), ::std::mem::transmute(numpatharrayelements), ::std::mem::transmute(patharray), ::std::mem::transmute(nummodeinfoarrayelements), ::std::mem::transmute(modeinfoarray), ::std::mem::transmute(currenttopologyid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_DisplayDevices`*"]
pub struct RUN {
    pub iStart: i32,
    pub iStop: i32,
}
impl RUN {}
impl ::std::default::Default for RUN {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RUN {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RUN").field("iStart", &self.iStart).field("iStop", &self.iStop).finish()
    }
}
impl ::std::cmp::PartialEq for RUN {
    fn eq(&self, other: &Self) -> bool {
        self.iStart == other.iStart && self.iStop == other.iStop
    }
}
impl ::std::cmp::Eq for RUN {}
unsafe impl ::windows::runtime::Abi for RUN {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`*"]
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
impl ::std::default::Default for STROBJ {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for STROBJ {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("STROBJ").field("cGlyphs", &self.cGlyphs).field("flAccel", &self.flAccel).field("ulCharInc", &self.ulCharInc).field("rclBkGround", &self.rclBkGround).field("pgp", &self.pgp).field("pwszOrg", &self.pwszOrg).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for STROBJ {
    fn eq(&self, other: &Self) -> bool {
        self.cGlyphs == other.cGlyphs && self.flAccel == other.flAccel && self.ulCharInc == other.ulCharInc && self.rclBkGround == other.rclBkGround && self.pgp == other.pgp && self.pwszOrg == other.pwszOrg
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for STROBJ {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for STROBJ {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn STROBJ_bEnum(pstro: *mut STROBJ, pc: *mut u32, ppgpos: *mut *mut GLYPHPOS) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn STROBJ_bEnum(pstro: *mut STROBJ, pc: *mut u32, ppgpos: *mut *mut GLYPHPOS) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(STROBJ_bEnum(::std::mem::transmute(pstro), ::std::mem::transmute(pc), ::std::mem::transmute(ppgpos)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn STROBJ_bEnumPositionsOnly(pstro: *mut STROBJ, pc: *mut u32, ppgpos: *mut *mut GLYPHPOS) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn STROBJ_bEnumPositionsOnly(pstro: *mut STROBJ, pc: *mut u32, ppgpos: *mut *mut GLYPHPOS) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(STROBJ_bEnumPositionsOnly(::std::mem::transmute(pstro), ::std::mem::transmute(pc), ::std::mem::transmute(ppgpos)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`, `Win32_System_SystemServices`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
#[inline]
pub unsafe fn STROBJ_bGetAdvanceWidths(pso: *mut STROBJ, ifirst: u32, c: u32, pptqd: *mut super::super::System::SystemServices::POINTQF) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn STROBJ_bGetAdvanceWidths(pso: *mut STROBJ, ifirst: u32, c: u32, pptqd: *mut super::super::System::SystemServices::POINTQF) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(STROBJ_bGetAdvanceWidths(::std::mem::transmute(pso), ::std::mem::transmute(ifirst), ::std::mem::transmute(c), ::std::mem::transmute(pptqd)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn STROBJ_dwGetCodePage(pstro: *mut STROBJ) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn STROBJ_dwGetCodePage(pstro: *mut STROBJ) -> u32;
        }
        ::std::mem::transmute(STROBJ_dwGetCodePage(::std::mem::transmute(pstro)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn STROBJ_vEnumStart(pstro: *mut STROBJ) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn STROBJ_vEnumStart(pstro: *mut STROBJ);
        }
        ::std::mem::transmute(STROBJ_vEnumStart(::std::mem::transmute(pstro)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`, `Win32_System_SystemServices`*"]
pub struct SURFOBJ {
    pub dhsurf: super::super::System::SystemServices::DHSURF,
    pub hsurf: HSURF,
    pub dhpdev: super::super::System::SystemServices::DHPDEV,
    pub hdev: super::super::System::SystemServices::HDEV,
    pub sizlBitmap: super::super::Foundation::SIZE,
    pub cjBits: u32,
    pub pvBits: *mut ::std::ffi::c_void,
    pub pvScan0: *mut ::std::ffi::c_void,
    pub lDelta: i32,
    pub iUniq: u32,
    pub iBitmapFormat: u32,
    pub iType: u16,
    pub fjBitmap: u16,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl SURFOBJ {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::default::Default for SURFOBJ {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::fmt::Debug for SURFOBJ {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::cmp::PartialEq for SURFOBJ {
    fn eq(&self, other: &Self) -> bool {
        self.dhsurf == other.dhsurf && self.hsurf == other.hsurf && self.dhpdev == other.dhpdev && self.hdev == other.hdev && self.sizlBitmap == other.sizlBitmap && self.cjBits == other.cjBits && self.pvBits == other.pvBits && self.pvScan0 == other.pvScan0 && self.lDelta == other.lDelta && self.iUniq == other.iUniq && self.iBitmapFormat == other.iBitmapFormat && self.iType == other.iType && self.fjBitmap == other.fjBitmap
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::cmp::Eq for SURFOBJ {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
unsafe impl ::windows::runtime::Abi for SURFOBJ {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetDisplayConfig(numpatharrayelements: u32, patharray: *const DISPLAYCONFIG_PATH_INFO, nummodeinfoarrayelements: u32, modeinfoarray: *const DISPLAYCONFIG_MODE_INFO, flags: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetDisplayConfig(numpatharrayelements: u32, patharray: *const DISPLAYCONFIG_PATH_INFO, nummodeinfoarrayelements: u32, modeinfoarray: *const DISPLAYCONFIG_MODE_INFO, flags: u32) -> i32;
        }
        ::std::mem::transmute(SetDisplayConfig(::std::mem::transmute(numpatharrayelements), ::std::mem::transmute(patharray), ::std::mem::transmute(nummodeinfoarrayelements), ::std::mem::transmute(modeinfoarray), ::std::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`*"]
pub struct TYPE1_FONT {
    pub hPFM: super::super::Foundation::HANDLE,
    pub hPFB: super::super::Foundation::HANDLE,
    pub ulIdentifier: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl TYPE1_FONT {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for TYPE1_FONT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for TYPE1_FONT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TYPE1_FONT").field("hPFM", &self.hPFM).field("hPFB", &self.hPFB).field("ulIdentifier", &self.ulIdentifier).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for TYPE1_FONT {
    fn eq(&self, other: &Self) -> bool {
        self.hPFM == other.hPFM && self.hPFB == other.hPFB && self.ulIdentifier == other.ulIdentifier
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for TYPE1_FONT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for TYPE1_FONT {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Devices_Display`, `Win32_Foundation`, `Win32_Graphics_DirectDraw`*"]
pub struct VIDEOMEMORY {
    pub dwFlags: u32,
    pub fpStart: usize,
    pub Anonymous1: VIDEOMEMORY_0,
    pub ddsCaps: super::super::Graphics::DirectDraw::DDSCAPS,
    pub ddsCapsAlt: super::super::Graphics::DirectDraw::DDSCAPS,
    pub Anonymous2: VIDEOMEMORY_1,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl VIDEOMEMORY {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::default::Default for VIDEOMEMORY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::PartialEq for VIDEOMEMORY {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::Eq for VIDEOMEMORY {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
unsafe impl ::windows::runtime::Abi for VIDEOMEMORY {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
pub union VIDEOMEMORY_0 {
    pub fpEnd: usize,
    pub dwWidth: u32,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl VIDEOMEMORY_0 {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::default::Default for VIDEOMEMORY_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::PartialEq for VIDEOMEMORY_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::Eq for VIDEOMEMORY_0 {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
unsafe impl ::windows::runtime::Abi for VIDEOMEMORY_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
pub union VIDEOMEMORY_1 {
    pub lpHeap: *mut super::super::Devices::Display::VMEMHEAP,
    pub dwHeight: u32,
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl VIDEOMEMORY_1 {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::default::Default for VIDEOMEMORY_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::PartialEq for VIDEOMEMORY_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
impl ::std::cmp::Eq for VIDEOMEMORY_1 {}
#[cfg(all(feature = "Win32_Devices_Display", feature = "Win32_Foundation", feature = "Win32_Graphics_DirectDraw"))]
unsafe impl ::windows::runtime::Abi for VIDEOMEMORY_1 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Graphics_DirectDraw")]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Graphics_DirectDraw`*"]
pub struct VIDEOMEMORYINFO {
    pub fpPrimary: usize,
    pub dwFlags: u32,
    pub dwDisplayWidth: u32,
    pub dwDisplayHeight: u32,
    pub lDisplayPitch: i32,
    pub ddpfDisplay: super::super::Graphics::DirectDraw::DDPIXELFORMAT,
    pub dwOffscreenAlign: u32,
    pub dwOverlayAlign: u32,
    pub dwTextureAlign: u32,
    pub dwZBufferAlign: u32,
    pub dwAlphaAlign: u32,
    pub pvPrimary: *mut ::std::ffi::c_void,
}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl VIDEOMEMORYINFO {}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl ::std::default::Default for VIDEOMEMORYINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl ::std::cmp::PartialEq for VIDEOMEMORYINFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
impl ::std::cmp::Eq for VIDEOMEMORYINFO {}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
unsafe impl ::windows::runtime::Abi for VIDEOMEMORYINFO {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_DisplayDevices`*"]
pub struct VIDEOPARAMETERS {
    pub Guid: ::windows::runtime::GUID,
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
impl ::std::default::Default for VIDEOPARAMETERS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VIDEOPARAMETERS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
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
impl ::std::cmp::PartialEq for VIDEOPARAMETERS {
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
impl ::std::cmp::Eq for VIDEOPARAMETERS {}
unsafe impl ::windows::runtime::Abi for VIDEOPARAMETERS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_DisplayDevices`*"]
pub struct WCRUN {
    pub wcLow: u16,
    pub cGlyphs: u16,
    pub phg: *mut u32,
}
impl WCRUN {}
impl ::std::default::Default for WCRUN {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WCRUN {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WCRUN").field("wcLow", &self.wcLow).field("cGlyphs", &self.cGlyphs).field("phg", &self.phg).finish()
    }
}
impl ::std::cmp::PartialEq for WCRUN {
    fn eq(&self, other: &Self) -> bool {
        self.wcLow == other.wcLow && self.cGlyphs == other.cGlyphs && self.phg == other.phg
    }
}
impl ::std::cmp::Eq for WCRUN {}
unsafe impl ::windows::runtime::Abi for WCRUN {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`, `Win32_System_SystemServices`*"]
pub struct WNDOBJ {
    pub coClient: CLIPOBJ,
    pub pvConsumer: *mut ::std::ffi::c_void,
    pub rclClient: super::super::Foundation::RECTL,
    pub psoOwner: *mut SURFOBJ,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl WNDOBJ {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::default::Default for WNDOBJ {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::fmt::Debug for WNDOBJ {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WNDOBJ").field("coClient", &self.coClient).field("pvConsumer", &self.pvConsumer).field("rclClient", &self.rclClient).field("psoOwner", &self.psoOwner).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::cmp::PartialEq for WNDOBJ {
    fn eq(&self, other: &Self) -> bool {
        self.coClient == other.coClient && self.pvConsumer == other.pvConsumer && self.rclClient == other.rclClient && self.psoOwner == other.psoOwner
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::cmp::Eq for WNDOBJ {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
unsafe impl ::windows::runtime::Abi for WNDOBJ {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_DisplayDevices`*"]
pub struct XFORML {
    pub eM11: f32,
    pub eM12: f32,
    pub eM21: f32,
    pub eM22: f32,
    pub eDx: f32,
    pub eDy: f32,
}
impl XFORML {}
impl ::std::default::Default for XFORML {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for XFORML {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("XFORML").field("eM11", &self.eM11).field("eM12", &self.eM12).field("eM21", &self.eM21).field("eM22", &self.eM22).field("eDx", &self.eDx).field("eDy", &self.eDy).finish()
    }
}
impl ::std::cmp::PartialEq for XFORML {
    fn eq(&self, other: &Self) -> bool {
        self.eM11 == other.eM11 && self.eM12 == other.eM12 && self.eM21 == other.eM21 && self.eM22 == other.eM22 && self.eDx == other.eDx && self.eDy == other.eDy
    }
}
impl ::std::cmp::Eq for XFORML {}
unsafe impl ::windows::runtime::Abi for XFORML {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`, `Win32_System_SystemServices`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
#[inline]
pub unsafe fn XFORMOBJ_bApplyXform(pxo: *mut super::super::System::SystemServices::XFORMOBJ, imode: u32, cpoints: u32, pvin: *mut ::std::ffi::c_void, pvout: *mut ::std::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn XFORMOBJ_bApplyXform(pxo: *mut super::super::System::SystemServices::XFORMOBJ, imode: u32, cpoints: u32, pvin: *mut ::std::ffi::c_void, pvout: *mut ::std::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(XFORMOBJ_bApplyXform(::std::mem::transmute(pxo), ::std::mem::transmute(imode), ::std::mem::transmute(cpoints), ::std::mem::transmute(pvin), ::std::mem::transmute(pvout)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_System_SystemServices`*"]
#[cfg(feature = "Win32_System_SystemServices")]
#[inline]
pub unsafe fn XFORMOBJ_iGetXform(pxo: *const super::super::System::SystemServices::XFORMOBJ, pxform: *mut XFORML) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn XFORMOBJ_iGetXform(pxo: *const super::super::System::SystemServices::XFORMOBJ, pxform: *mut XFORML) -> u32;
        }
        ::std::mem::transmute(XFORMOBJ_iGetXform(::std::mem::transmute(pxo), ::std::mem::transmute(pxform)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_DisplayDevices`*"]
pub struct XLATEOBJ {
    pub iUniq: u32,
    pub flXlate: u32,
    pub iSrcType: u16,
    pub iDstType: u16,
    pub cEntries: u32,
    pub pulXlate: *mut u32,
}
impl XLATEOBJ {}
impl ::std::default::Default for XLATEOBJ {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for XLATEOBJ {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("XLATEOBJ").field("iUniq", &self.iUniq).field("flXlate", &self.flXlate).field("iSrcType", &self.iSrcType).field("iDstType", &self.iDstType).field("cEntries", &self.cEntries).field("pulXlate", &self.pulXlate).finish()
    }
}
impl ::std::cmp::PartialEq for XLATEOBJ {
    fn eq(&self, other: &Self) -> bool {
        self.iUniq == other.iUniq && self.flXlate == other.flXlate && self.iSrcType == other.iSrcType && self.iDstType == other.iDstType && self.cEntries == other.cEntries && self.pulXlate == other.pulXlate
    }
}
impl ::std::cmp::Eq for XLATEOBJ {}
unsafe impl ::windows::runtime::Abi for XLATEOBJ {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_DisplayDevices`*"]
#[inline]
pub unsafe fn XLATEOBJ_cGetPalette(pxlo: *mut XLATEOBJ, ipal: u32, cpal: u32, ppal: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn XLATEOBJ_cGetPalette(pxlo: *mut XLATEOBJ, ipal: u32, cpal: u32, ppal: *mut u32) -> u32;
        }
        ::std::mem::transmute(XLATEOBJ_cGetPalette(::std::mem::transmute(pxlo), ::std::mem::transmute(ipal), ::std::mem::transmute(cpal), ::std::mem::transmute(ppal)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_DisplayDevices`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn XLATEOBJ_hGetColorTransform(pxlo: *mut XLATEOBJ) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn XLATEOBJ_hGetColorTransform(pxlo: *mut XLATEOBJ) -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(XLATEOBJ_hGetColorTransform(::std::mem::transmute(pxlo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_DisplayDevices`*"]
#[inline]
pub unsafe fn XLATEOBJ_iXlate(pxlo: *mut XLATEOBJ, icolor: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn XLATEOBJ_iXlate(pxlo: *mut XLATEOBJ, icolor: u32) -> u32;
        }
        ::std::mem::transmute(XLATEOBJ_iXlate(::std::mem::transmute(pxlo), ::std::mem::transmute(icolor)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_DisplayDevices`*"]
#[inline]
pub unsafe fn XLATEOBJ_piVector(pxlo: *mut XLATEOBJ) -> *mut u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn XLATEOBJ_piVector(pxlo: *mut XLATEOBJ) -> *mut u32;
        }
        ::std::mem::transmute(XLATEOBJ_piVector(::std::mem::transmute(pxlo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
