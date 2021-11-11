#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
    pub fn DMOEnum();
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DMOGetName();
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
    pub fn DMOGetTypes();
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DMORegister();
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`*"]
    pub fn DMOUnregister();
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MoCopyMediaType();
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MoCreateMediaType();
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MoDeleteMediaType();
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MoDuplicateMediaType();
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MoFreeMediaType();
    #[doc = "*Required features: `Win32_Media_DxMediaObjects`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MoInitMediaType();
}
