#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "ApplicationModel_SocialInfo_Provider")]
pub mod Provider;
#[link(name = "windows")]
extern "system" {}
pub struct ISocialFeedChildItem(pub *mut ::core::ffi::c_void);
pub struct ISocialFeedContent(pub *mut ::core::ffi::c_void);
pub struct ISocialFeedItem(pub *mut ::core::ffi::c_void);
pub struct ISocialFeedSharedItem(pub *mut ::core::ffi::c_void);
pub struct ISocialItemThumbnail(pub *mut ::core::ffi::c_void);
pub struct ISocialUserInfo(pub *mut ::core::ffi::c_void);
pub struct SocialFeedChildItem(i32);
pub struct SocialFeedContent(i32);
pub struct SocialFeedItem(i32);
pub struct SocialFeedItemStyle(i32);
pub struct SocialFeedKind(i32);
pub struct SocialFeedSharedItem(i32);
pub struct SocialFeedUpdateMode(i32);
pub struct SocialInfoContract(i32);
pub struct SocialItemBadgeStyle(i32);
pub struct SocialItemThumbnail(i32);
pub struct SocialUserInfo(i32);
