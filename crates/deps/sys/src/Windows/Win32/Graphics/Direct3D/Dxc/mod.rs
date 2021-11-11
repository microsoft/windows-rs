#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Graphics_Direct3D_Dxc`*"]
    pub fn DxcCreateInstance();
    #[doc = "*Required features: `Win32_Graphics_Direct3D_Dxc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn DxcCreateInstance2();
}
