#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Gaming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CheckGamingPrivilegeSilently();
    #[doc = "*Required features: `Win32_Gaming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CheckGamingPrivilegeSilentlyForUser();
    #[doc = "*Required features: `Win32_Gaming`*"]
    pub fn CheckGamingPrivilegeWithUI();
    #[doc = "*Required features: `Win32_Gaming`*"]
    pub fn CheckGamingPrivilegeWithUIForUser();
    #[doc = "*Required features: `Win32_Gaming`*"]
    pub fn GetExpandedResourceExclusiveCpuCount();
    #[doc = "*Required features: `Win32_Gaming`*"]
    pub fn GetGamingDeviceModelInformation();
    #[doc = "*Required features: `Win32_Gaming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HasExpandedResources();
    #[doc = "*Required features: `Win32_Gaming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ProcessPendingGameUI();
    #[doc = "*Required features: `Win32_Gaming`*"]
    pub fn ReleaseExclusiveCpuSets();
    #[doc = "*Required features: `Win32_Gaming`*"]
    pub fn ShowChangeFriendRelationshipUI();
    #[doc = "*Required features: `Win32_Gaming`*"]
    pub fn ShowChangeFriendRelationshipUIForUser();
    #[doc = "*Required features: `Win32_Gaming`*"]
    pub fn ShowCustomizeUserProfileUI();
    #[doc = "*Required features: `Win32_Gaming`*"]
    pub fn ShowCustomizeUserProfileUIForUser();
    #[doc = "*Required features: `Win32_Gaming`*"]
    pub fn ShowFindFriendsUI();
    #[doc = "*Required features: `Win32_Gaming`*"]
    pub fn ShowFindFriendsUIForUser();
    #[doc = "*Required features: `Win32_Gaming`*"]
    pub fn ShowGameInfoUI();
    #[doc = "*Required features: `Win32_Gaming`*"]
    pub fn ShowGameInfoUIForUser();
    #[doc = "*Required features: `Win32_Gaming`*"]
    pub fn ShowGameInviteUI();
    #[doc = "*Required features: `Win32_Gaming`*"]
    pub fn ShowGameInviteUIForUser();
    #[doc = "*Required features: `Win32_Gaming`*"]
    pub fn ShowGameInviteUIWithContext();
    #[doc = "*Required features: `Win32_Gaming`*"]
    pub fn ShowGameInviteUIWithContextForUser();
    #[doc = "*Required features: `Win32_Gaming`*"]
    pub fn ShowPlayerPickerUI();
    #[doc = "*Required features: `Win32_Gaming`*"]
    pub fn ShowPlayerPickerUIForUser();
    #[doc = "*Required features: `Win32_Gaming`*"]
    pub fn ShowProfileCardUI();
    #[doc = "*Required features: `Win32_Gaming`*"]
    pub fn ShowProfileCardUIForUser();
    #[doc = "*Required features: `Win32_Gaming`*"]
    pub fn ShowTitleAchievementsUI();
    #[doc = "*Required features: `Win32_Gaming`*"]
    pub fn ShowTitleAchievementsUIForUser();
    #[doc = "*Required features: `Win32_Gaming`*"]
    pub fn ShowUserSettingsUI();
    #[doc = "*Required features: `Win32_Gaming`*"]
    pub fn ShowUserSettingsUIForUser();
    #[doc = "*Required features: `Win32_Gaming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TryCancelPendingGameUI();
}
