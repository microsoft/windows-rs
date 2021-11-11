#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn AccountPictureKind();
    fn AdvertisingManager();
    fn AdvertisingManagerForUser();
    fn AssignedAccessSettings();
    fn DiagnosticsSettings();
    fn FirstSignInSettings();
    fn GlobalizationPreferences();
    fn GlobalizationPreferencesForUser();
    fn IAdvertisingManagerForUser();
    fn IAdvertisingManagerStatics();
    fn IAdvertisingManagerStatics2();
    fn IAssignedAccessSettings();
    fn IAssignedAccessSettingsStatics();
    fn IDiagnosticsSettings();
    fn IDiagnosticsSettingsStatics();
    fn IFirstSignInSettings();
    fn IFirstSignInSettingsStatics();
    fn IGlobalizationPreferencesForUser();
    fn IGlobalizationPreferencesStatics();
    fn IGlobalizationPreferencesStatics2();
    fn IGlobalizationPreferencesStatics3();
    fn ILockScreenImageFeedStatics();
    fn ILockScreenStatics();
    fn IUserInformationStatics();
    fn IUserProfilePersonalizationSettings();
    fn IUserProfilePersonalizationSettingsStatics();
    fn LockScreen();
    fn SetAccountPictureResult();
    fn SetImageFeedResult();
    fn UserInformation();
    fn UserProfileContract();
    fn UserProfileLockScreenContract();
    fn UserProfilePersonalizationSettings();
}
