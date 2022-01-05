#[cfg(feature = "implement_exclusive")]
pub trait IContentRestrictionsBrowsePolicyImpl: Sized {
    fn GeographicRegion(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MaxBrowsableAgeRating(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>>;
    fn PreferredAgeRating(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRatedContentDescriptionImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTitle(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Image(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference>;
    fn SetImage(&self, value: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<()>;
    fn Category(&self) -> ::windows::core::Result<RatedContentCategory>;
    fn SetCategory(&self, value: RatedContentCategory) -> ::windows::core::Result<()>;
    fn Ratings(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn SetRatings(&self, value: &::core::option::Option<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRatedContentDescriptionFactoryImpl: Sized {
    fn Create(&self, id: &::windows::core::HSTRING, title: &::windows::core::HSTRING, category: RatedContentCategory) -> ::windows::core::Result<RatedContentDescription>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRatedContentRestrictionsImpl: Sized {
    fn GetBrowsePolicyAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ContentRestrictionsBrowsePolicy>>;
    fn GetRestrictionLevelAsync(&self, ratedcontentdescription: &::core::option::Option<RatedContentDescription>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ContentAccessRestrictionLevel>>;
    fn RequestContentAccessAsync(&self, ratedcontentdescription: &::core::option::Option<RatedContentDescription>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn RestrictionsChanged(&self, handler: &::core::option::Option<super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRestrictionsChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRatedContentRestrictionsFactoryImpl: Sized {
    fn CreateWithMaxAgeRating(&self, maxagerating: u32) -> ::windows::core::Result<RatedContentRestrictions>;
}
