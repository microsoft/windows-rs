#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ISocialFeedChildItemImpl: Sized {
    fn Author(&self) -> ::windows::core::Result<SocialUserInfo>;
    fn PrimaryContent(&self) -> ::windows::core::Result<SocialFeedContent>;
    fn SecondaryContent(&self) -> ::windows::core::Result<SocialFeedContent>;
    fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn SetTimestamp(&self, value: &super::super::Foundation::DateTime) -> ::windows::core::Result<()>;
    fn TargetUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetTargetUri(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn Thumbnails(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<SocialItemThumbnail>>;
    fn SharedItem(&self) -> ::windows::core::Result<SocialFeedSharedItem>;
    fn SetSharedItem(&self, value: &::core::option::Option<SocialFeedSharedItem>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ISocialFeedContentImpl: Sized {
    fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTitle(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Message(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetMessage(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TargetUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetTargetUri(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ISocialFeedItemImpl: Sized {
    fn Author(&self) -> ::windows::core::Result<SocialUserInfo>;
    fn PrimaryContent(&self) -> ::windows::core::Result<SocialFeedContent>;
    fn SecondaryContent(&self) -> ::windows::core::Result<SocialFeedContent>;
    fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn SetTimestamp(&self, value: &super::super::Foundation::DateTime) -> ::windows::core::Result<()>;
    fn TargetUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetTargetUri(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn Thumbnails(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<SocialItemThumbnail>>;
    fn SharedItem(&self) -> ::windows::core::Result<SocialFeedSharedItem>;
    fn SetSharedItem(&self, value: &::core::option::Option<SocialFeedSharedItem>) -> ::windows::core::Result<()>;
    fn BadgeStyle(&self) -> ::windows::core::Result<SocialItemBadgeStyle>;
    fn SetBadgeStyle(&self, value: SocialItemBadgeStyle) -> ::windows::core::Result<()>;
    fn BadgeCountValue(&self) -> ::windows::core::Result<i32>;
    fn SetBadgeCountValue(&self, value: i32) -> ::windows::core::Result<()>;
    fn RemoteId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetRemoteId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ChildItem(&self) -> ::windows::core::Result<SocialFeedChildItem>;
    fn SetChildItem(&self, value: &::core::option::Option<SocialFeedChildItem>) -> ::windows::core::Result<()>;
    fn Style(&self) -> ::windows::core::Result<SocialFeedItemStyle>;
    fn SetStyle(&self, value: SocialFeedItemStyle) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ISocialFeedSharedItemImpl: Sized {
    fn OriginalSource(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetOriginalSource(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn Content(&self) -> ::windows::core::Result<SocialFeedContent>;
    fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn SetTimestamp(&self, value: &super::super::Foundation::DateTime) -> ::windows::core::Result<()>;
    fn TargetUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetTargetUri(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn SetThumbnail(&self, value: &::core::option::Option<SocialItemThumbnail>) -> ::windows::core::Result<()>;
    fn Thumbnail(&self) -> ::windows::core::Result<SocialItemThumbnail>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ISocialItemThumbnailImpl: Sized {
    fn TargetUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetTargetUri(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn ImageUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetImageUri(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn BitmapSize(&self) -> ::windows::core::Result<super::super::Graphics::Imaging::BitmapSize>;
    fn SetBitmapSize(&self, value: &super::super::Graphics::Imaging::BitmapSize) -> ::windows::core::Result<()>;
    fn SetImageAsync(&self, image: &::core::option::Option<super::super::Storage::Streams::IInputStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ISocialUserInfoImpl: Sized {
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDisplayName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn UserName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetUserName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn RemoteId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetRemoteId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TargetUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetTargetUri(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
}
