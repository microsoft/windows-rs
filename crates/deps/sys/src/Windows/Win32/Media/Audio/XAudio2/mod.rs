#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Media_Audio_XAudio2`*"]
    pub fn CreateAudioReverb();
    #[doc = "*Required features: `Win32_Media_Audio_XAudio2`*"]
    pub fn CreateAudioVolumeMeter();
    #[doc = "*Required features: `Win32_Media_Audio_XAudio2`*"]
    pub fn CreateFX();
    #[doc = "*Required features: `Win32_Media_Audio_XAudio2`*"]
    pub fn CreateHrtfApo();
    #[doc = "*Required features: `Win32_Media_Audio_XAudio2`*"]
    pub fn XAudio2CreateWithVersionInfo();
}
