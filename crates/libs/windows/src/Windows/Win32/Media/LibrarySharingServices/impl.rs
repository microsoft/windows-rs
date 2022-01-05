#[cfg(feature = "Win32_System_Com")]
pub trait IWindowsMediaLibrarySharingDeviceImpl: Sized + IDispatchImpl {
    fn DeviceID();
    fn Authorization();
    fn SetAuthorization();
    fn Properties();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWindowsMediaLibrarySharingDevicePropertiesImpl: Sized + IDispatchImpl {
    fn Item();
    fn Count();
    fn GetProperty();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWindowsMediaLibrarySharingDevicePropertyImpl: Sized + IDispatchImpl {
    fn Name();
    fn Value();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWindowsMediaLibrarySharingDevicesImpl: Sized + IDispatchImpl {
    fn Item();
    fn Count();
    fn GetDevice();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWindowsMediaLibrarySharingServicesImpl: Sized + IDispatchImpl {
    fn showShareMediaCPL();
    fn userHomeMediaSharingState();
    fn SetuserHomeMediaSharingState();
    fn userHomeMediaSharingLibraryName();
    fn SetuserHomeMediaSharingLibraryName();
    fn computerHomeMediaSharingAllowedState();
    fn SetcomputerHomeMediaSharingAllowedState();
    fn userInternetMediaSharingState();
    fn SetuserInternetMediaSharingState();
    fn computerInternetMediaSharingAllowedState();
    fn SetcomputerInternetMediaSharingAllowedState();
    fn internetMediaSharingSecurityGroup();
    fn SetinternetMediaSharingSecurityGroup();
    fn allowSharingToAllDevices();
    fn SetallowSharingToAllDevices();
    fn setDefaultAuthorization();
    fn setAuthorizationState();
    fn getAllDevices();
    fn customSettingsApplied();
}
