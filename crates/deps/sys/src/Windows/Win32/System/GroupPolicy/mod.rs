#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BrowseForGPO();
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CommandLineFromMsiDescriptor();
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateGPOLink();
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteAllGPOLinks();
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteGPOLink();
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnterCriticalPolicySection();
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ExportRSoPData();
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FreeGPOListA();
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FreeGPOListW();
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GenerateGPNotification();
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetAppliedGPOListA();
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetAppliedGPOListW();
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetGPOListA();
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetGPOListW();
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetLocalManagedApplicationData();
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetLocalManagedApplications();
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`, `Win32_UI_Shell`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell"))]
    pub fn GetManagedApplicationCategories();
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetManagedApplications();
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImportRSoPData();
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InstallApplication();
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LeaveCriticalPolicySection();
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub fn ProcessGroupPolicyCompleted();
    #[doc = "*Required features: `Win32_System_GroupPolicy`*"]
    pub fn ProcessGroupPolicyCompletedEx();
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RefreshPolicy();
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RefreshPolicyEx();
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterGPNotification();
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn RsopAccessCheckByType();
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RsopFileAccessCheck();
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_System_Wmi`*"]
    #[cfg(feature = "Win32_System_Wmi")]
    pub fn RsopResetPolicySettingStatus();
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`, `Win32_System_Wmi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Wmi"))]
    pub fn RsopSetPolicySettingStatus();
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UninstallApplication();
    #[doc = "*Required features: `Win32_System_GroupPolicy`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnregisterGPNotification();
}
