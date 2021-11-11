#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_Diagnostics_Ceip`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CeipIsOptedIn();
}
