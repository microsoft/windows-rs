#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn DeviceServicingDetails();
    fn DeviceUseDetails();
    fn IDeviceServicingDetails();
    fn IDeviceUseDetails();
}
