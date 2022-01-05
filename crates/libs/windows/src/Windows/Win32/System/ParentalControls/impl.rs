pub trait IWPCGamesSettingsImpl: Sized + IWPCSettingsImpl {
    fn IsBlocked();
}
pub trait IWPCProviderConfigImpl: Sized {
    fn GetUserSummary();
    fn Configure();
    fn RequestOverride();
}
pub trait IWPCProviderStateImpl: Sized {
    fn Enable();
    fn Disable();
}
pub trait IWPCProviderSupportImpl: Sized {
    fn GetCurrent();
}
pub trait IWPCSettingsImpl: Sized {
    fn IsLoggingRequired();
    fn GetLastSettingsChangeTime();
    fn GetRestrictions();
}
pub trait IWPCWebSettingsImpl: Sized + IWPCSettingsImpl {
    fn GetSettings();
    fn RequestURLOverride();
}
pub trait IWindowsParentalControlsImpl: Sized + IWindowsParentalControlsCoreImpl {
    fn GetGamesSettings();
}
pub trait IWindowsParentalControlsCoreImpl: Sized {
    fn GetVisibility();
    fn GetUserSettings();
    fn GetWebSettings();
    fn GetWebFilterInfo();
}
