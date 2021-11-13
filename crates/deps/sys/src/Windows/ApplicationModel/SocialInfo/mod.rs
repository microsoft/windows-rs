#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "ApplicationModel_SocialInfo_Provider")]
pub mod Provider;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ISocialFeedChildItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISocialFeedChildItem {}
impl ::core::clone::Clone for ISocialFeedChildItem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISocialFeedContent(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISocialFeedContent {}
impl ::core::clone::Clone for ISocialFeedContent {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISocialFeedItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISocialFeedItem {}
impl ::core::clone::Clone for ISocialFeedItem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISocialFeedSharedItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISocialFeedSharedItem {}
impl ::core::clone::Clone for ISocialFeedSharedItem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISocialItemThumbnail(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISocialItemThumbnail {}
impl ::core::clone::Clone for ISocialItemThumbnail {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISocialUserInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISocialUserInfo {}
impl ::core::clone::Clone for ISocialUserInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SocialFeedChildItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SocialFeedChildItem {}
impl ::core::clone::Clone for SocialFeedChildItem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SocialFeedContent(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SocialFeedContent {}
impl ::core::clone::Clone for SocialFeedContent {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SocialFeedItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SocialFeedItem {}
impl ::core::clone::Clone for SocialFeedItem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SocialFeedItemStyle(pub i32);
impl SocialFeedItemStyle {
    pub const Default: Self = Self(0i32);
    pub const Photo: Self = Self(1i32);
}
impl ::core::marker::Copy for SocialFeedItemStyle {}
impl ::core::clone::Clone for SocialFeedItemStyle {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SocialFeedKind(pub i32);
impl SocialFeedKind {
    pub const HomeFeed: Self = Self(0i32);
    pub const ContactFeed: Self = Self(1i32);
    pub const Dashboard: Self = Self(2i32);
}
impl ::core::marker::Copy for SocialFeedKind {}
impl ::core::clone::Clone for SocialFeedKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SocialFeedSharedItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SocialFeedSharedItem {}
impl ::core::clone::Clone for SocialFeedSharedItem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SocialFeedUpdateMode(pub i32);
impl SocialFeedUpdateMode {
    pub const Append: Self = Self(0i32);
    pub const Replace: Self = Self(1i32);
}
impl ::core::marker::Copy for SocialFeedUpdateMode {}
impl ::core::clone::Clone for SocialFeedUpdateMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SocialItemBadgeStyle(pub i32);
impl SocialItemBadgeStyle {
    pub const Hidden: Self = Self(0i32);
    pub const Visible: Self = Self(1i32);
    pub const VisibleWithCount: Self = Self(2i32);
}
impl ::core::marker::Copy for SocialItemBadgeStyle {}
impl ::core::clone::Clone for SocialItemBadgeStyle {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SocialItemThumbnail(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SocialItemThumbnail {}
impl ::core::clone::Clone for SocialItemThumbnail {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SocialUserInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SocialUserInfo {}
impl ::core::clone::Clone for SocialUserInfo {
    fn clone(&self) -> Self {
        *self
    }
}
