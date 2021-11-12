#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AccountPictureKind(pub i32);
impl AccountPictureKind {
    pub const SmallImage: Self = Self(0i32);
    pub const LargeImage: Self = Self(1i32);
    pub const Video: Self = Self(2i32);
}
impl ::core::marker::Copy for AccountPictureKind {}
impl ::core::clone::Clone for AccountPictureKind {
    fn clone(&self) -> Self {
        *self
    }
}
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
#[repr(transparent)]
pub struct SetAccountPictureResult(pub i32);
impl SetAccountPictureResult {
    pub const Success: Self = Self(0i32);
    pub const ChangeDisabled: Self = Self(1i32);
    pub const LargeOrDynamicError: Self = Self(2i32);
    pub const VideoFrameSizeError: Self = Self(3i32);
    pub const FileSizeError: Self = Self(4i32);
    pub const Failure: Self = Self(5i32);
}
impl ::core::marker::Copy for SetAccountPictureResult {}
impl ::core::clone::Clone for SetAccountPictureResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SetImageFeedResult(pub i32);
impl SetImageFeedResult {
    pub const Success: Self = Self(0i32);
    pub const ChangeDisabled: Self = Self(1i32);
    pub const UserCanceled: Self = Self(2i32);
}
impl ::core::marker::Copy for SetImageFeedResult {}
impl ::core::clone::Clone for SetImageFeedResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UserProfilePersonalizationSettings(pub *mut ::core::ffi::c_void);
