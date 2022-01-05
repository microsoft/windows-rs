#[cfg(feature = "implement_exclusive")]
pub trait ILicenseManagerStaticsImpl: Sized {
    fn AddLicenseAsync(&self, license: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn GetSatisfactionInfosAsync(&self, contentids: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, keyids: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<LicenseSatisfactionResult>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILicenseManagerStatics2Impl: Sized {
    fn RefreshLicensesAsync(&self, refreshoption: LicenseRefreshOption) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILicenseSatisfactionInfoImpl: Sized {
    fn SatisfiedByDevice(&self) -> ::windows::core::Result<bool>;
    fn SatisfiedByOpenLicense(&self) -> ::windows::core::Result<bool>;
    fn SatisfiedByTrial(&self) -> ::windows::core::Result<bool>;
    fn SatisfiedByPass(&self) -> ::windows::core::Result<bool>;
    fn SatisfiedByInstallMedia(&self) -> ::windows::core::Result<bool>;
    fn SatisfiedBySignedInUser(&self) -> ::windows::core::Result<bool>;
    fn IsSatisfied(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILicenseSatisfactionResultImpl: Sized {
    fn LicenseSatisfactionInfos(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, LicenseSatisfactionInfo>>;
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
