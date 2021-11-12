#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct ISocialDashboardItemUpdater(pub *mut ::core::ffi::c_void);
pub struct ISocialFeedUpdater(pub *mut ::core::ffi::c_void);
pub struct ISocialInfoProviderManagerStatics(pub *mut ::core::ffi::c_void);
pub struct SocialDashboardItemUpdater(i32);
pub struct SocialFeedUpdater(i32);
pub struct SocialInfoProviderManager(i32);
