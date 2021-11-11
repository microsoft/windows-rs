#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub fn DirectDrawCreate();
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub fn DirectDrawCreateClipper();
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub fn DirectDrawCreateEx();
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DirectDrawEnumerateA();
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn DirectDrawEnumerateExA();
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn DirectDrawEnumerateExW();
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DirectDrawEnumerateW();
}
