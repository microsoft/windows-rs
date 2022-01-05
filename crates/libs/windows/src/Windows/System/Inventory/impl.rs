#[cfg(feature = "implement_exclusive")]
pub trait IInstalledDesktopAppImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Publisher(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayVersion(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInstalledDesktopAppStaticsImpl: Sized {
    fn GetInventoryAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<InstalledDesktopApp>>>;
}
