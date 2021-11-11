#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_UI_TextServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DoMsCtfMonitor();
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub fn InitLocalMsCtfMonitor();
    #[doc = "*Required features: `Win32_UI_TextServices`*"]
    pub fn UninitLocalMsCtfMonitor();
}
