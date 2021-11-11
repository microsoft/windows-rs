#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn DirectSoundCaptureCreate();
    fn DirectSoundCaptureCreate8();
    fn DirectSoundCaptureEnumerateA();
    fn DirectSoundCaptureEnumerateW();
    fn DirectSoundCreate();
    fn DirectSoundCreate8();
    fn DirectSoundEnumerateA();
    fn DirectSoundEnumerateW();
    fn DirectSoundFullDuplexCreate();
    fn GetDeviceID();
}
