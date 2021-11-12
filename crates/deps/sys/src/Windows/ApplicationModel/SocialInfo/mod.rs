#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
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
#[repr(transparent)]
pub struct SocialFeedItemStyle(pub i32);
impl SocialFeedItemStyle {
    pub const Default: SocialFeedItemStyle = SocialFeedItemStyle(0i32);
    pub const Photo: SocialFeedItemStyle = SocialFeedItemStyle(1i32);
}
#[repr(transparent)]
pub struct SocialFeedKind(pub i32);
impl SocialFeedKind {
    pub const HomeFeed: SocialFeedKind = SocialFeedKind(0i32);
    pub const ContactFeed: SocialFeedKind = SocialFeedKind(1i32);
    pub const Dashboard: SocialFeedKind = SocialFeedKind(2i32);
}
#[repr(transparent)]
pub struct SocialFeedSharedItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SocialFeedUpdateMode(pub i32);
impl SocialFeedUpdateMode {
    pub const Append: SocialFeedUpdateMode = SocialFeedUpdateMode(0i32);
    pub const Replace: SocialFeedUpdateMode = SocialFeedUpdateMode(1i32);
}
#[repr(C)]
pub struct SocialInfoContract(i32);
#[repr(transparent)]
pub struct SocialItemBadgeStyle(pub i32);
impl SocialItemBadgeStyle {
    pub const Hidden: SocialItemBadgeStyle = SocialItemBadgeStyle(0i32);
    pub const Visible: SocialItemBadgeStyle = SocialItemBadgeStyle(1i32);
    pub const VisibleWithCount: SocialItemBadgeStyle = SocialItemBadgeStyle(2i32);
}
#[repr(transparent)]
pub struct SocialItemThumbnail(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SocialUserInfo(pub *mut ::core::ffi::c_void);
