#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn CheckGamingPrivilegeSilently();
    fn CheckGamingPrivilegeSilentlyForUser();
    fn CheckGamingPrivilegeWithUI();
    fn CheckGamingPrivilegeWithUIForUser();
    fn GetExpandedResourceExclusiveCpuCount();
    fn GetGamingDeviceModelInformation();
    fn HasExpandedResources();
    fn ProcessPendingGameUI();
    fn ReleaseExclusiveCpuSets();
    fn ShowChangeFriendRelationshipUI();
    fn ShowChangeFriendRelationshipUIForUser();
    fn ShowCustomizeUserProfileUI();
    fn ShowCustomizeUserProfileUIForUser();
    fn ShowFindFriendsUI();
    fn ShowFindFriendsUIForUser();
    fn ShowGameInfoUI();
    fn ShowGameInfoUIForUser();
    fn ShowGameInviteUI();
    fn ShowGameInviteUIForUser();
    fn ShowGameInviteUIWithContext();
    fn ShowGameInviteUIWithContextForUser();
    fn ShowPlayerPickerUI();
    fn ShowPlayerPickerUIForUser();
    fn ShowProfileCardUI();
    fn ShowProfileCardUIForUser();
    fn ShowTitleAchievementsUI();
    fn ShowTitleAchievementsUIForUser();
    fn ShowUserSettingsUI();
    fn ShowUserSettingsUIForUser();
    fn TryCancelPendingGameUI();
}
