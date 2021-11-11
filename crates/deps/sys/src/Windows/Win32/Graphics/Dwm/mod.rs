#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Graphics_Dwm`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DwmAttachMilContent();
    #[doc = "*Required features: `Win32_Graphics_Dwm`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DwmDefWindowProc();
    #[doc = "*Required features: `Win32_Graphics_Dwm`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DwmDetachMilContent();
    #[doc = "*Required features: `Win32_Graphics_Dwm`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn DwmEnableBlurBehindWindow();
    #[doc = "*Required features: `Win32_Graphics_Dwm`*"]
    pub fn DwmEnableComposition();
    #[doc = "*Required features: `Win32_Graphics_Dwm`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DwmEnableMMCSS();
    #[doc = "*Required features: `Win32_Graphics_Dwm`, `Win32_Foundation`, `Win32_UI_Controls`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
    pub fn DwmExtendFrameIntoClientArea();
    #[doc = "*Required features: `Win32_Graphics_Dwm`*"]
    pub fn DwmFlush();
    #[doc = "*Required features: `Win32_Graphics_Dwm`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DwmGetColorizationColor();
    #[doc = "*Required features: `Win32_Graphics_Dwm`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DwmGetCompositionTimingInfo();
    #[doc = "*Required features: `Win32_Graphics_Dwm`*"]
    pub fn DwmGetGraphicsStreamClient();
    #[doc = "*Required features: `Win32_Graphics_Dwm`*"]
    pub fn DwmGetGraphicsStreamTransformHint();
    #[doc = "*Required features: `Win32_Graphics_Dwm`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DwmGetTransportAttributes();
    #[doc = "*Required features: `Win32_Graphics_Dwm`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DwmGetUnmetTabRequirements();
    #[doc = "*Required features: `Win32_Graphics_Dwm`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DwmGetWindowAttribute();
    #[doc = "*Required features: `Win32_Graphics_Dwm`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DwmInvalidateIconicBitmaps();
    #[doc = "*Required features: `Win32_Graphics_Dwm`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DwmIsCompositionEnabled();
    #[doc = "*Required features: `Win32_Graphics_Dwm`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DwmModifyPreviousDxFrameDuration();
    #[doc = "*Required features: `Win32_Graphics_Dwm`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DwmQueryThumbnailSourceSize();
    #[doc = "*Required features: `Win32_Graphics_Dwm`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DwmRegisterThumbnail();
    #[doc = "*Required features: `Win32_Graphics_Dwm`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DwmRenderGesture();
    #[doc = "*Required features: `Win32_Graphics_Dwm`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DwmSetDxFrameDuration();
    #[doc = "*Required features: `Win32_Graphics_Dwm`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn DwmSetIconicLivePreviewBitmap();
    #[doc = "*Required features: `Win32_Graphics_Dwm`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn DwmSetIconicThumbnail();
    #[doc = "*Required features: `Win32_Graphics_Dwm`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DwmSetPresentParameters();
    #[doc = "*Required features: `Win32_Graphics_Dwm`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DwmSetWindowAttribute();
    #[doc = "*Required features: `Win32_Graphics_Dwm`*"]
    pub fn DwmShowContact();
    #[doc = "*Required features: `Win32_Graphics_Dwm`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DwmTetherContact();
    #[doc = "*Required features: `Win32_Graphics_Dwm`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DwmTransitionOwnedWindow();
    #[doc = "*Required features: `Win32_Graphics_Dwm`*"]
    pub fn DwmUnregisterThumbnail();
    #[doc = "*Required features: `Win32_Graphics_Dwm`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DwmUpdateThumbnailProperties();
}
