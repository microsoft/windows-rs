#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "ApplicationModel_UserActivities_Core")]
pub mod Core;
#[link(name = "windows")]
extern "system" {}
pub struct IUserActivity(pub *mut ::core::ffi::c_void);
pub struct IUserActivity2(pub *mut ::core::ffi::c_void);
pub struct IUserActivity3(pub *mut ::core::ffi::c_void);
pub struct IUserActivityAttribution(pub *mut ::core::ffi::c_void);
pub struct IUserActivityAttributionFactory(pub *mut ::core::ffi::c_void);
pub struct IUserActivityChannel(pub *mut ::core::ffi::c_void);
pub struct IUserActivityChannel2(pub *mut ::core::ffi::c_void);
pub struct IUserActivityChannelStatics(pub *mut ::core::ffi::c_void);
pub struct IUserActivityChannelStatics2(pub *mut ::core::ffi::c_void);
pub struct IUserActivityChannelStatics3(pub *mut ::core::ffi::c_void);
pub struct IUserActivityContentInfo(pub *mut ::core::ffi::c_void);
pub struct IUserActivityContentInfoStatics(pub *mut ::core::ffi::c_void);
pub struct IUserActivityFactory(pub *mut ::core::ffi::c_void);
pub struct IUserActivityRequest(pub *mut ::core::ffi::c_void);
pub struct IUserActivityRequestManager(pub *mut ::core::ffi::c_void);
pub struct IUserActivityRequestManagerStatics(pub *mut ::core::ffi::c_void);
pub struct IUserActivityRequestedEventArgs(pub *mut ::core::ffi::c_void);
pub struct IUserActivitySession(pub *mut ::core::ffi::c_void);
pub struct IUserActivitySessionHistoryItem(pub *mut ::core::ffi::c_void);
pub struct IUserActivityStatics(pub *mut ::core::ffi::c_void);
pub struct IUserActivityVisualElements(pub *mut ::core::ffi::c_void);
pub struct IUserActivityVisualElements2(pub *mut ::core::ffi::c_void);
pub struct UserActivity(i32);
pub struct UserActivityAttribution(i32);
pub struct UserActivityChannel(i32);
pub struct UserActivityContentInfo(i32);
pub struct UserActivityRequest(i32);
pub struct UserActivityRequestManager(i32);
pub struct UserActivityRequestedEventArgs(i32);
pub struct UserActivitySession(i32);
pub struct UserActivitySessionHistoryItem(i32);
pub struct UserActivityState(i32);
pub struct UserActivityVisualElements(i32);
