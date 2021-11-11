#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn ApplyLocalManagementSyncML();
    fn DiscoverManagementService();
    fn DiscoverManagementServiceEx();
    fn GetDeviceManagementConfigInfo();
    fn GetDeviceRegistrationInfo();
    fn GetManagementAppHyperlink();
    fn IsDeviceRegisteredWithManagement();
    fn IsManagementRegistrationAllowed();
    fn IsMdmUxWithoutAadAllowed();
    fn RegisterDeviceWithLocalManagement();
    fn RegisterDeviceWithManagement();
    fn RegisterDeviceWithManagementUsingAADCredentials();
    fn RegisterDeviceWithManagementUsingAADDeviceCredentials();
    fn RegisterDeviceWithManagementUsingAADDeviceCredentials2();
    fn SetDeviceManagementConfigInfo();
    fn SetManagedExternally();
    fn UnregisterDeviceWithLocalManagement();
    fn UnregisterDeviceWithManagement();
}
