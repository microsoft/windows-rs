#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn XInputEnable();
    fn XInputGetAudioDeviceIds();
    fn XInputGetBatteryInformation();
    fn XInputGetCapabilities();
    fn XInputGetKeystroke();
    fn XInputGetState();
    fn XInputSetState();
}
