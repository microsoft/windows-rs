#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ISocialDashboardItemUpdater(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISocialDashboardItemUpdater {}
impl ::core::clone::Clone for ISocialDashboardItemUpdater {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISocialFeedUpdater(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISocialFeedUpdater {}
impl ::core::clone::Clone for ISocialFeedUpdater {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISocialInfoProviderManagerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISocialInfoProviderManagerStatics {}
impl ::core::clone::Clone for ISocialInfoProviderManagerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SocialDashboardItemUpdater(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SocialDashboardItemUpdater {}
impl ::core::clone::Clone for SocialDashboardItemUpdater {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SocialFeedUpdater(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SocialFeedUpdater {}
impl ::core::clone::Clone for SocialFeedUpdater {
    fn clone(&self) -> Self {
        *self
    }
}
