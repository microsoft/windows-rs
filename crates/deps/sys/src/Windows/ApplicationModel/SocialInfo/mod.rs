#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "ApplicationModel_SocialInfo_Provider")]
pub mod Provider;
#[link(name = "windows")]
extern "system" {
    fn ISocialFeedChildItem();
    fn ISocialFeedContent();
    fn ISocialFeedItem();
    fn ISocialFeedSharedItem();
    fn ISocialItemThumbnail();
    fn ISocialUserInfo();
    fn SocialFeedChildItem();
    fn SocialFeedContent();
    fn SocialFeedItem();
    fn SocialFeedItemStyle();
    fn SocialFeedKind();
    fn SocialFeedSharedItem();
    fn SocialFeedUpdateMode();
    fn SocialInfoContract();
    fn SocialItemBadgeStyle();
    fn SocialItemThumbnail();
    fn SocialUserInfo();
}
