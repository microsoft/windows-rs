#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn BrowseForGPO();
    fn CommandLineFromMsiDescriptor();
    fn CreateGPOLink();
    fn DeleteAllGPOLinks();
    fn DeleteGPOLink();
    fn EnterCriticalPolicySection();
    fn ExportRSoPData();
    fn FreeGPOListA();
    fn FreeGPOListW();
    fn GenerateGPNotification();
    fn GetAppliedGPOListA();
    fn GetAppliedGPOListW();
    fn GetGPOListA();
    fn GetGPOListW();
    fn GetLocalManagedApplicationData();
    fn GetLocalManagedApplications();
    fn GetManagedApplicationCategories();
    fn GetManagedApplications();
    fn ImportRSoPData();
    fn InstallApplication();
    fn LeaveCriticalPolicySection();
    fn ProcessGroupPolicyCompleted();
    fn ProcessGroupPolicyCompletedEx();
    fn RefreshPolicy();
    fn RefreshPolicyEx();
    fn RegisterGPNotification();
    fn RsopAccessCheckByType();
    fn RsopFileAccessCheck();
    fn RsopResetPolicySettingStatus();
    fn RsopSetPolicySettingStatus();
    fn UninstallApplication();
    fn UnregisterGPNotification();
}
