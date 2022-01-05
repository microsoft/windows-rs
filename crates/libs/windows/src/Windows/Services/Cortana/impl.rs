#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ICortanaActionableInsightsImpl: Sized {
    fn User(&self) -> ::windows::core::Result<super::super::System::User>;
    fn IsAvailableAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn ShowInsightsForImageAsync(&self, imagestream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ShowInsightsForImageWithOptionsAsync(&self, imagestream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStreamReference>, options: &::core::option::Option<CortanaActionableInsightsOptions>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ShowInsightsForTextAsync(&self, text: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ShowInsightsForTextWithOptionsAsync(&self, text: &::windows::core::HSTRING, options: &::core::option::Option<CortanaActionableInsightsOptions>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ShowInsightsAsync(&self, datapackage: &::core::option::Option<super::super::ApplicationModel::DataTransfer::DataPackage>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ShowInsightsWithOptionsAsync(&self, datapackage: &::core::option::Option<super::super::ApplicationModel::DataTransfer::DataPackage>, options: &::core::option::Option<CortanaActionableInsightsOptions>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ICortanaActionableInsightsOptionsImpl: Sized {
    fn ContentSourceWebLink(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetContentSourceWebLink(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn SurroundingText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSurroundingText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ICortanaActionableInsightsStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<CortanaActionableInsights>;
    fn GetForUser(&self, user: &::core::option::Option<super::super::System::User>) -> ::windows::core::Result<CortanaActionableInsights>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ICortanaPermissionsManagerImpl: Sized {
    fn IsSupported(&self) -> ::windows::core::Result<bool>;
    fn ArePermissionsGrantedAsync(&self, permissions: &::core::option::Option<super::super::Foundation::Collections::IIterable<CortanaPermission>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn GrantPermissionsAsync(&self, permissions: &::core::option::Option<super::super::Foundation::Collections::IIterable<CortanaPermission>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CortanaPermissionsChangeResult>>;
    fn RevokePermissionsAsync(&self, permissions: &::core::option::Option<super::super::Foundation::Collections::IIterable<CortanaPermission>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CortanaPermissionsChangeResult>>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ICortanaPermissionsManagerStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<CortanaPermissionsManager>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ICortanaSettingsImpl: Sized {
    fn HasUserConsentToVoiceActivation(&self) -> ::windows::core::Result<bool>;
    fn IsVoiceActivationEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsVoiceActivationEnabled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ICortanaSettingsStaticsImpl: Sized {
    fn IsSupported(&self) -> ::windows::core::Result<bool>;
    fn GetDefault(&self) -> ::windows::core::Result<CortanaSettings>;
}
