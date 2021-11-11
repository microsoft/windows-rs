#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn IServiceDeviceStatics();
    fn IStorageDeviceStatics();
    fn PortableDeviceContract();
    fn ServiceDevice();
    fn ServiceDeviceType();
    fn StorageDevice();
}
