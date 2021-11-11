#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn IUserDataAvailabilityStateChangedEventArgs();
    fn IUserDataBufferUnprotectResult();
    fn IUserDataProtectionManager();
    fn IUserDataProtectionManagerStatics();
    fn IUserDataStorageItemProtectionInfo();
    fn UserDataAvailability();
    fn UserDataAvailabilityStateChangedEventArgs();
    fn UserDataBufferUnprotectResult();
    fn UserDataBufferUnprotectStatus();
    fn UserDataProtectionManager();
    fn UserDataStorageItemProtectionInfo();
    fn UserDataStorageItemProtectionStatus();
}
