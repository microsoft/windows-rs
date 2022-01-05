pub trait IEnhancedStorageACTImpl: Sized {
    fn Authorize();
    fn Unauthorize();
    fn GetAuthorizationState();
    fn GetMatchingVolume();
    fn GetUniqueIdentity();
    fn GetSilos();
}
pub trait IEnhancedStorageACT2Impl: Sized + IEnhancedStorageACTImpl {
    fn GetDeviceName();
    fn IsDeviceRemovable();
}
pub trait IEnhancedStorageACT3Impl: Sized + IEnhancedStorageACT2Impl + IEnhancedStorageACTImpl {
    fn UnauthorizeEx();
    fn IsQueueFrozen();
    fn GetShellExtSupport();
}
pub trait IEnhancedStorageSiloImpl: Sized {
    fn GetInfo();
    fn GetActions();
    fn SendCommand();
    fn GetPortableDevice();
    fn GetDevicePath();
}
pub trait IEnhancedStorageSiloActionImpl: Sized {
    fn GetName();
    fn GetDescription();
    fn Invoke();
}
pub trait IEnumEnhancedStorageACTImpl: Sized {
    fn GetACTs();
    fn GetMatchingACT();
}
