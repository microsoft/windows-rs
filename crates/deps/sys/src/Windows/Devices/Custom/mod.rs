#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn CustomDevice();
    fn CustomDeviceContract();
    fn DeviceAccessMode();
    fn DeviceSharingMode();
    fn ICustomDevice();
    fn ICustomDeviceStatics();
    fn IIOControlCode();
    fn IIOControlCodeFactory();
    fn IKnownDeviceTypesStatics();
    fn IOControlAccessMode();
    fn IOControlBufferingMethod();
    fn IOControlCode();
    fn KnownDeviceTypes();
}
