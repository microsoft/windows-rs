pub trait IProtectionPolicyManagerInteropImpl: Sized {
    fn RequestAccessForWindowAsync();
    fn GetForWindow();
}
pub trait IProtectionPolicyManagerInterop2Impl: Sized {
    fn RequestAccessForAppWithWindowAsync();
    fn RequestAccessWithAuditingInfoForWindowAsync();
    fn RequestAccessWithMessageForWindowAsync();
    fn RequestAccessForAppWithAuditingInfoForWindowAsync();
    fn RequestAccessForAppWithMessageForWindowAsync();
}
pub trait IProtectionPolicyManagerInterop3Impl: Sized {
    fn RequestAccessWithBehaviorForWindowAsync();
    fn RequestAccessForAppWithBehaviorForWindowAsync();
    fn RequestAccessToFilesForAppForWindowAsync();
    fn RequestAccessToFilesForAppWithMessageAndBehaviorForWindowAsync();
    fn RequestAccessToFilesForProcessForWindowAsync();
    fn RequestAccessToFilesForProcessWithMessageAndBehaviorForWindowAsync();
}
