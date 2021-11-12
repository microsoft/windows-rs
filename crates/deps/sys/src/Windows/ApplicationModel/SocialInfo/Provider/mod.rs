#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ISocialDashboardItemUpdater(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISocialFeedUpdater(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISocialInfoProviderManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SocialDashboardItemUpdater(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SocialFeedUpdater(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SocialInfoProviderManager(pub *mut ::core::ffi::c_void);
