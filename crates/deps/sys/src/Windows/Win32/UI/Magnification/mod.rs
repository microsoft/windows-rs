#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_UI_Magnification`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MagGetColorEffect();
    #[doc = "*Required features: `Win32_UI_Magnification`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MagGetFullscreenColorEffect();
    #[doc = "*Required features: `Win32_UI_Magnification`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MagGetFullscreenTransform();
    #[doc = "*Required features: `Win32_UI_Magnification`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn MagGetImageScalingCallback();
    #[doc = "*Required features: `Win32_UI_Magnification`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MagGetInputTransform();
    #[doc = "*Required features: `Win32_UI_Magnification`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MagGetWindowFilterList();
    #[doc = "*Required features: `Win32_UI_Magnification`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MagGetWindowSource();
    #[doc = "*Required features: `Win32_UI_Magnification`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MagGetWindowTransform();
    #[doc = "*Required features: `Win32_UI_Magnification`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MagInitialize();
    #[doc = "*Required features: `Win32_UI_Magnification`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MagSetColorEffect();
    #[doc = "*Required features: `Win32_UI_Magnification`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MagSetFullscreenColorEffect();
    #[doc = "*Required features: `Win32_UI_Magnification`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MagSetFullscreenTransform();
    #[doc = "*Required features: `Win32_UI_Magnification`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn MagSetImageScalingCallback();
    #[doc = "*Required features: `Win32_UI_Magnification`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MagSetInputTransform();
    #[doc = "*Required features: `Win32_UI_Magnification`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MagSetWindowFilterList();
    #[doc = "*Required features: `Win32_UI_Magnification`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MagSetWindowSource();
    #[doc = "*Required features: `Win32_UI_Magnification`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MagSetWindowTransform();
    #[doc = "*Required features: `Win32_UI_Magnification`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MagShowSystemCursor();
    #[doc = "*Required features: `Win32_UI_Magnification`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MagUninitialize();
}
