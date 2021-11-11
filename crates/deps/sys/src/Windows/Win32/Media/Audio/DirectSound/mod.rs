#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub fn DirectSoundCaptureCreate();
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub fn DirectSoundCaptureCreate8();
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DirectSoundCaptureEnumerateA();
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DirectSoundCaptureEnumerateW();
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub fn DirectSoundCreate();
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub fn DirectSoundCreate8();
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DirectSoundEnumerateA();
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DirectSoundEnumerateW();
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DirectSoundFullDuplexCreate();
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub fn GetDeviceID();
}
