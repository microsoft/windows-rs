#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DoMsCtfMonitor(dwflags: u32, heventforservicestop: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub fn InitLocalMsCtfMonitor(dwflags: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub fn UninitLocalMsCtfMonitor() -> ::windows::runtime::HRESULT;
}
