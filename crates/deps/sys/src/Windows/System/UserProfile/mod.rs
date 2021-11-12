#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct AccountPictureKind(i32);
pub struct AdvertisingManager(i32);
pub struct AdvertisingManagerForUser(i32);
pub struct AssignedAccessSettings(i32);
pub struct DiagnosticsSettings(i32);
pub struct FirstSignInSettings(i32);
pub struct GlobalizationPreferences(i32);
pub struct GlobalizationPreferencesForUser(i32);
pub struct IAdvertisingManagerForUser(pub *mut ::core::ffi::c_void);
pub struct IAdvertisingManagerStatics(pub *mut ::core::ffi::c_void);
pub struct IAdvertisingManagerStatics2(pub *mut ::core::ffi::c_void);
pub struct IAssignedAccessSettings(pub *mut ::core::ffi::c_void);
pub struct IAssignedAccessSettingsStatics(pub *mut ::core::ffi::c_void);
pub struct IDiagnosticsSettings(pub *mut ::core::ffi::c_void);
pub struct IDiagnosticsSettingsStatics(pub *mut ::core::ffi::c_void);
pub struct IFirstSignInSettings(pub *mut ::core::ffi::c_void);
pub struct IFirstSignInSettingsStatics(pub *mut ::core::ffi::c_void);
pub struct IGlobalizationPreferencesForUser(pub *mut ::core::ffi::c_void);
pub struct IGlobalizationPreferencesStatics(pub *mut ::core::ffi::c_void);
pub struct IGlobalizationPreferencesStatics2(pub *mut ::core::ffi::c_void);
pub struct IGlobalizationPreferencesStatics3(pub *mut ::core::ffi::c_void);
pub struct ILockScreenImageFeedStatics(pub *mut ::core::ffi::c_void);
pub struct ILockScreenStatics(pub *mut ::core::ffi::c_void);
pub struct IUserInformationStatics(pub *mut ::core::ffi::c_void);
pub struct IUserProfilePersonalizationSettings(pub *mut ::core::ffi::c_void);
pub struct IUserProfilePersonalizationSettingsStatics(pub *mut ::core::ffi::c_void);
pub struct LockScreen(i32);
pub struct SetAccountPictureResult(i32);
pub struct SetImageFeedResult(i32);
pub struct UserInformation(i32);
pub struct UserProfileContract(i32);
pub struct UserProfileLockScreenContract(i32);
pub struct UserProfilePersonalizationSettings(i32);
