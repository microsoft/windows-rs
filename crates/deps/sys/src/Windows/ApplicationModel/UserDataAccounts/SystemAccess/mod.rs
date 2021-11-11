#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn DeviceAccountAuthenticationType();
    fn DeviceAccountConfiguration();
    fn DeviceAccountIconId();
    fn DeviceAccountMailAgeFilter();
    fn DeviceAccountServerType();
    fn DeviceAccountSyncScheduleKind();
    fn IDeviceAccountConfiguration();
    fn IDeviceAccountConfiguration2();
    fn IUserDataAccountSystemAccessManagerStatics();
    fn IUserDataAccountSystemAccessManagerStatics2();
    fn UserDataAccountSystemAccessManager();
}
