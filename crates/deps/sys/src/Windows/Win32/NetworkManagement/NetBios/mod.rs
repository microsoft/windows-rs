#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_NetworkManagement_NetBios`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Netbios(pncb: *mut NCB) -> u8;
}
