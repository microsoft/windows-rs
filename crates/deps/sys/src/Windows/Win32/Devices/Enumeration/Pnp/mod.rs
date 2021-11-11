#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn SwDeviceClose();
    fn SwDeviceCreate();
    fn SwDeviceGetLifetime();
    fn SwDeviceInterfacePropertySet();
    fn SwDeviceInterfaceRegister();
    fn SwDeviceInterfaceSetState();
    fn SwDevicePropertySet();
    fn SwDeviceSetLifetime();
    fn SwMemFree();
}
