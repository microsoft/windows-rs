#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn DeviceLockdownContract();
    fn DeviceLockdownProfile();
    fn DeviceLockdownProfileInformation();
    fn IDeviceLockdownProfileInformation();
    fn IDeviceLockdownProfileStatics();
}
