#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Graphics_Direct3D11on12`, `Win32_Graphics_Direct3D`, `Win32_Graphics_Direct3D11`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Direct3D11"))]
    pub fn D3D11On12CreateDevice();
}
