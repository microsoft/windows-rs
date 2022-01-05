#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ISocialDashboardItemUpdaterImpl: Sized {
    fn OwnerRemoteId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Content(&self) -> ::windows::core::Result<super::SocialFeedContent>;
    fn Timestamp(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime>;
    fn SetTimestamp(&self, value: &super::super::super::Foundation::DateTime) -> ::windows::core::Result<()>;
    fn SetThumbnail(&self, value: &::core::option::Option<super::SocialItemThumbnail>) -> ::windows::core::Result<()>;
    fn Thumbnail(&self) -> ::windows::core::Result<super::SocialItemThumbnail>;
    fn CommitAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn TargetUri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
    fn SetTargetUri(&self, value: &::core::option::Option<super::super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ISocialFeedUpdaterImpl: Sized {
    fn OwnerRemoteId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Kind(&self) -> ::windows::core::Result<super::SocialFeedKind>;
    fn Items(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<super::SocialFeedItem>>;
    fn CommitAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ISocialInfoProviderManagerStaticsImpl: Sized {
    fn CreateSocialFeedUpdaterAsync(&self, kind: super::SocialFeedKind, mode: super::SocialFeedUpdateMode, ownerremoteid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<SocialFeedUpdater>>;
    fn CreateDashboardItemUpdaterAsync(&self, ownerremoteid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<SocialDashboardItemUpdater>>;
    fn UpdateBadgeCountValue(&self, itemremoteid: &::windows::core::HSTRING, newcount: i32) -> ::windows::core::Result<()>;
    fn ReportNewContentAvailable(&self, contactremoteid: &::windows::core::HSTRING, kind: super::SocialFeedKind) -> ::windows::core::Result<()>;
    fn ProvisionAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<bool>>;
    fn DeprovisionAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
