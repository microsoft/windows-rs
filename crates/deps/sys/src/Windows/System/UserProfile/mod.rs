#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(C)]
pub struct AccountPictureKind(i32);
#[repr(transparent)]
pub struct AdvertisingManagerForUser(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AssignedAccessSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DiagnosticsSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FirstSignInSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GlobalizationPreferencesForUser(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAdvertisingManagerForUser(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAdvertisingManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAdvertisingManagerStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAssignedAccessSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAssignedAccessSettingsStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDiagnosticsSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDiagnosticsSettingsStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFirstSignInSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFirstSignInSettingsStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGlobalizationPreferencesForUser(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGlobalizationPreferencesStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGlobalizationPreferencesStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGlobalizationPreferencesStatics3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILockScreenImageFeedStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILockScreenStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserInformationStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserProfilePersonalizationSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserProfilePersonalizationSettingsStatics(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct SetAccountPictureResult(i32);
#[repr(C)]
pub struct SetImageFeedResult(i32);
#[repr(C)]
pub struct UserProfileContract(i32);
#[repr(C)]
pub struct UserProfileLockScreenContract(i32);
#[repr(transparent)]
pub struct UserProfilePersonalizationSettings(pub *mut ::core::ffi::c_void);
