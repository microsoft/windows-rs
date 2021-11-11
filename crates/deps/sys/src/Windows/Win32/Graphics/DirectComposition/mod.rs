#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DCompositionAttachMouseDragToHwnd();
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DCompositionAttachMouseWheelToHwnd();
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DCompositionBoostCompositorClock();
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Graphics_Dxgi`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub fn DCompositionCreateDevice();
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub fn DCompositionCreateDevice2();
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub fn DCompositionCreateDevice3();
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn DCompositionCreateSurfaceHandle();
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub fn DCompositionGetFrameId();
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DCompositionGetStatistics();
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DCompositionGetTargetStatistics();
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DCompositionWaitForCompositorClock();
}
