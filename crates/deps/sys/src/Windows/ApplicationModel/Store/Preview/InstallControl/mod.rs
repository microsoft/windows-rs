#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn AppInstallItem();
    fn AppInstallManager();
    fn AppInstallManagerItemEventArgs();
    fn AppInstallOptions();
    fn AppInstallState();
    fn AppInstallStatus();
    fn AppInstallType();
    fn AppInstallationToastNotificationMode();
    fn AppUpdateOptions();
    fn AutoUpdateSetting();
    fn GetEntitlementResult();
    fn GetEntitlementStatus();
    fn IAppInstallItem();
    fn IAppInstallItem2();
    fn IAppInstallItem3();
    fn IAppInstallItem4();
    fn IAppInstallItem5();
    fn IAppInstallManager();
    fn IAppInstallManager2();
    fn IAppInstallManager3();
    fn IAppInstallManager4();
    fn IAppInstallManager5();
    fn IAppInstallManager6();
    fn IAppInstallManager7();
    fn IAppInstallManagerItemEventArgs();
    fn IAppInstallOptions();
    fn IAppInstallOptions2();
    fn IAppInstallStatus();
    fn IAppInstallStatus2();
    fn IAppInstallStatus3();
    fn IAppUpdateOptions();
    fn IAppUpdateOptions2();
    fn IGetEntitlementResult();
}
