#[cfg(feature = "implement_exclusive")]
pub trait IOfflineMapPackageImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<OfflineMapPackageStatus>;
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn EnclosingRegionName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn EstimatedSizeInBytes(&self) -> ::windows::core::Result<u64>;
    fn RemoveStatusChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn StatusChanged(&self, value: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<OfflineMapPackage, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RequestStartDownloadAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<OfflineMapPackageStartDownloadResult>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IOfflineMapPackageQueryResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<OfflineMapPackageQueryStatus>;
    fn Packages(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<OfflineMapPackage>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IOfflineMapPackageStartDownloadResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<OfflineMapPackageStartDownloadStatus>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IOfflineMapPackageStaticsImpl: Sized {
    fn FindPackagesAsync(&self, querypoint: &::core::option::Option<super::super::super::Devices::Geolocation::Geopoint>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<OfflineMapPackageQueryResult>>;
    fn FindPackagesInBoundingBoxAsync(&self, queryboundingbox: &::core::option::Option<super::super::super::Devices::Geolocation::GeoboundingBox>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<OfflineMapPackageQueryResult>>;
    fn FindPackagesInGeocircleAsync(&self, querycircle: &::core::option::Option<super::super::super::Devices::Geolocation::Geocircle>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<OfflineMapPackageQueryResult>>;
}
