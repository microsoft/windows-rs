#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Media_Audio_XAudio2`*"]
    pub fn CreateAudioReverb(ppapo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_Audio_XAudio2`*"]
    pub fn CreateAudioVolumeMeter(ppapo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_Audio_XAudio2`*"]
    pub fn CreateFX(clsid: *const ::windows::runtime::GUID, peffect: *mut ::windows::runtime::RawPtr, pinitdat: *const ::core::ffi::c_void, initdatabytesize: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_Audio_XAudio2`*"]
    pub fn CreateHrtfApo(init: *const HrtfApoInit, xapo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_Audio_XAudio2`*"]
    pub fn XAudio2CreateWithVersionInfo(ppxaudio2: *mut ::windows::runtime::RawPtr, flags: u32, xaudio2processor: u32, ntddiversion: u32) -> ::windows::runtime::HRESULT;
}
