pub trait ICameraUIControlImpl: Sized {
    fn Show();
    fn Close();
    fn Suspend();
    fn Resume();
    fn GetCurrentViewType();
    fn GetActiveItem();
    fn GetSelectedItems();
    fn RemoveCapturedItem();
}
pub trait ICameraUIControlEventCallbackImpl: Sized {
    fn OnStartupComplete();
    fn OnSuspendComplete();
    fn OnItemCaptured();
    fn OnItemDeleted();
    fn OnClosed();
}
pub trait IClipServiceNotificationHelperImpl: Sized {
    fn ShowToast();
}
pub trait IContainerActivationHelperImpl: Sized {
    fn CanActivateClientVM();
}
pub trait IDefaultBrowserSyncSettingsImpl: Sized {
    fn IsEnabled();
}
pub trait IDeleteBrowsingHistoryImpl: Sized {
    fn DeleteBrowsingHistory();
}
pub trait IEditionUpgradeBrokerImpl: Sized {
    fn InitializeParentWindow();
    fn UpdateOperatingSystem();
    fn ShowProductKeyUI();
    fn CanUpgrade();
}
pub trait IEditionUpgradeHelperImpl: Sized {
    fn CanUpgrade();
    fn UpdateOperatingSystem();
    fn ShowProductKeyUI();
    fn GetOsProductContentId();
    fn GetGenuineLocalStatus();
}
pub trait IWindowsLockModeHelperImpl: Sized {
    fn GetSMode();
}
