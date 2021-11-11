#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Graphics_Direct3D9on12`, `Win32_Foundation`, `Win32_Graphics_Direct3D9`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9"))]
    pub fn Direct3DCreate9On12();
    #[doc = "*Required features: `Win32_Graphics_Direct3D9on12`, `Win32_Foundation`, `Win32_Graphics_Direct3D9`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9"))]
    pub fn Direct3DCreate9On12Ex();
}
