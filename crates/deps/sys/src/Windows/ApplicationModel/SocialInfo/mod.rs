#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "ApplicationModel_SocialInfo_Provider")]
pub mod Provider;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ISocialFeedChildItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISocialFeedContent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISocialFeedItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISocialFeedSharedItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISocialItemThumbnail(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISocialUserInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SocialFeedChildItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SocialFeedContent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SocialFeedItem(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct SocialFeedItemStyle(i32);
#[repr(C)]
pub struct SocialFeedKind(i32);
#[repr(transparent)]
pub struct SocialFeedSharedItem(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct SocialFeedUpdateMode(i32);
#[repr(C)]
pub struct SocialInfoContract(i32);
#[repr(C)]
pub struct SocialItemBadgeStyle(i32);
#[repr(transparent)]
pub struct SocialItemThumbnail(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SocialUserInfo(pub *mut ::core::ffi::c_void);
