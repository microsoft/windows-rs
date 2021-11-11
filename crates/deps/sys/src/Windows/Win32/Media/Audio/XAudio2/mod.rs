#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn CreateAudioReverb();
    fn CreateAudioVolumeMeter();
    fn CreateFX();
    fn CreateHrtfApo();
    fn XAudio2CreateWithVersionInfo();
}
