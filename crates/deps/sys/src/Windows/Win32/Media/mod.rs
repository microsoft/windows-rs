#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Win32_Media_Audio")]
pub mod Audio;
#[cfg(feature = "Win32_Media_DeviceManager")]
pub mod DeviceManager;
#[cfg(feature = "Win32_Media_DirectShow")]
pub mod DirectShow;
#[cfg(feature = "Win32_Media_DxMediaObjects")]
pub mod DxMediaObjects;
#[cfg(feature = "Win32_Media_KernelStreaming")]
pub mod KernelStreaming;
#[cfg(feature = "Win32_Media_LibrarySharingServices")]
pub mod LibrarySharingServices;
#[cfg(feature = "Win32_Media_MediaFoundation")]
pub mod MediaFoundation;
#[cfg(feature = "Win32_Media_MediaPlayer")]
pub mod MediaPlayer;
#[cfg(feature = "Win32_Media_Multimedia")]
pub mod Multimedia;
#[cfg(feature = "Win32_Media_PictureAcquisition")]
pub mod PictureAcquisition;
#[cfg(feature = "Win32_Media_Speech")]
pub mod Speech;
#[cfg(feature = "Win32_Media_Streaming")]
pub mod Streaming;
#[cfg(feature = "Win32_Media_WindowsMediaFormat")]
pub mod WindowsMediaFormat;
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Media`*"]
    pub fn timeBeginPeriod();
    #[doc = "*Required features: `Win32_Media`*"]
    pub fn timeEndPeriod();
    #[doc = "*Required features: `Win32_Media`*"]
    pub fn timeGetDevCaps();
    #[doc = "*Required features: `Win32_Media`*"]
    pub fn timeGetSystemTime();
    #[doc = "*Required features: `Win32_Media`*"]
    pub fn timeGetTime();
    #[doc = "*Required features: `Win32_Media`*"]
    pub fn timeKillEvent();
    #[doc = "*Required features: `Win32_Media`*"]
    pub fn timeSetEvent();
}
