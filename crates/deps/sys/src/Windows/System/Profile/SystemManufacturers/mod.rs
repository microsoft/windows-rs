#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn IOemSupportInfo();
    fn ISmbiosInformationStatics();
    fn ISystemSupportDeviceInfo();
    fn ISystemSupportInfoStatics();
    fn ISystemSupportInfoStatics2();
    fn OemSupportInfo();
    fn SmbiosInformation();
    fn SystemManufacturersContract();
    fn SystemSupportDeviceInfo();
    fn SystemSupportInfo();
}
