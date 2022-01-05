#[cfg(feature = "implement_exclusive")]
pub trait ILocalCategoriesStaticsImpl: Sized {
    fn BankAndCreditUnions(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn EatDrink(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Hospitals(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn HotelsAndMotels(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn All(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Parking(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SeeDo(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Shop(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILocalLocationImpl: Sized {
    fn Address(&self) -> ::windows::core::Result<super::MapAddress>;
    fn Identifier(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Point(&self) -> ::windows::core::Result<super::super::super::Devices::Geolocation::Geopoint>;
    fn PhoneNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DataAttribution(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILocalLocation2Impl: Sized {
    fn Category(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RatingInfo(&self) -> ::windows::core::Result<LocalLocationRatingInfo>;
    fn HoursOfOperation(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<LocalLocationHoursOfOperationItem>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILocalLocationFinderResultImpl: Sized {
    fn LocalLocations(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<LocalLocation>>;
    fn Status(&self) -> ::windows::core::Result<LocalLocationFinderStatus>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILocalLocationFinderStaticsImpl: Sized {
    fn FindLocalLocationsAsync(&self, searchterm: &::windows::core::HSTRING, searcharea: &::core::option::Option<super::super::super::Devices::Geolocation::Geocircle>, localcategory: &::windows::core::HSTRING, maxresults: u32) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<LocalLocationFinderResult>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILocalLocationHoursOfOperationItemImpl: Sized {
    fn Day(&self) -> ::windows::core::Result<super::super::super::Globalization::DayOfWeek>;
    fn Start(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn Span(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILocalLocationRatingInfoImpl: Sized {
    fn AggregateRating(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<f64>>;
    fn RatingCount(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<i32>>;
    fn ProviderIdentifier(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPlaceInfoHelperStaticsImpl: Sized {
    fn CreateFromLocalLocation(&self, location: &::core::option::Option<LocalLocation>) -> ::windows::core::Result<super::PlaceInfo>;
}
