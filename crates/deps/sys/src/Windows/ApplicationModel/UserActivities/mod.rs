#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "ApplicationModel_UserActivities_Core")]
pub mod Core;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IUserActivity(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserActivity2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserActivity3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserActivityAttribution(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserActivityAttributionFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserActivityChannel(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserActivityChannel2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserActivityChannelStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserActivityChannelStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserActivityChannelStatics3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserActivityContentInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserActivityContentInfoStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserActivityFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserActivityRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserActivityRequestManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserActivityRequestManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserActivityRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserActivitySession(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserActivitySessionHistoryItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserActivityStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserActivityVisualElements(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserActivityVisualElements2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UserActivity(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UserActivityAttribution(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UserActivityChannel(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UserActivityContentInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UserActivityRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UserActivityRequestManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UserActivityRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UserActivitySession(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UserActivitySessionHistoryItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UserActivityState(pub i32);
impl UserActivityState {
    pub const New: Self = Self(0i32);
    pub const Published: Self = Self(1i32);
}
impl ::core::marker::Copy for UserActivityState {}
impl ::core::clone::Clone for UserActivityState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UserActivityVisualElements(pub *mut ::core::ffi::c_void);
