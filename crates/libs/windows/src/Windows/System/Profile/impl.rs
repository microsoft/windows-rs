#[cfg(feature = "implement_exclusive")]
pub trait IAnalyticsInfoStaticsImpl: Sized {
    fn VersionInfo(&self) -> ::windows::core::Result<AnalyticsVersionInfo>;
    fn DeviceForm(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAnalyticsInfoStatics2Impl: Sized {
    fn GetSystemPropertiesAsync(&self, attributenames: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAnalyticsVersionInfoImpl: Sized {
    fn DeviceFamily(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DeviceFamilyVersion(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAnalyticsVersionInfo2Impl: Sized {
    fn ProductName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppApplicabilityStaticsImpl: Sized {
    fn GetUnsupportedAppRequirements(&self, capabilities: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<UnsupportedAppRequirement>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEducationSettingsStaticsImpl: Sized {
    fn IsEducationEnvironment(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHardwareIdentificationStaticsImpl: Sized {
    fn GetPackageSpecificToken(&self, nonce: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<HardwareToken>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHardwareTokenImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn Signature(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn Certificate(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IKnownRetailInfoPropertiesStaticsImpl: Sized {
    fn RetailAccessCode(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ManufacturerName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ModelName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayModelName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Price(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsFeatured(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FormFactor(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ScreenSize(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Weight(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayDescription(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn BatteryLifeDescription(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ProcessorDescription(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Memory(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn StorageDescription(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GraphicsDescription(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FrontCameraDescription(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RearCameraDescription(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn HasNfc(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn HasSdSlot(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn HasOpticalDrive(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsOfficeInstalled(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn WindowsEdition(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPlatformDiagnosticsAndUsageDataSettingsStaticsImpl: Sized {
    fn CollectionLevel(&self) -> ::windows::core::Result<PlatformDataCollectionLevel>;
    fn CollectionLevelChanged(&self, handler: &::core::option::Option<super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCollectionLevelChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CanCollectDiagnostics(&self, level: PlatformDataCollectionLevel) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRetailInfoStaticsImpl: Sized {
    fn IsDemoModeEnabled(&self) -> ::windows::core::Result<bool>;
    fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISharedModeSettingsStaticsImpl: Sized {
    fn IsEnabled(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISharedModeSettingsStatics2Impl: Sized {
    fn ShouldAvoidLocalStorage(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemIdentificationInfoImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn Source(&self) -> ::windows::core::Result<SystemIdentificationSource>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemIdentificationStaticsImpl: Sized {
    fn GetSystemIdForPublisher(&self) -> ::windows::core::Result<SystemIdentificationInfo>;
    fn GetSystemIdForUser(&self, user: &::core::option::Option<super::User>) -> ::windows::core::Result<SystemIdentificationInfo>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemSetupInfoStaticsImpl: Sized {
    fn OutOfBoxExperienceState(&self) -> ::windows::core::Result<SystemOutOfBoxExperienceState>;
    fn OutOfBoxExperienceStateChanged(&self, handler: &::core::option::Option<super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveOutOfBoxExperienceStateChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUnsupportedAppRequirementImpl: Sized {
    fn Requirement(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Reasons(&self) -> ::windows::core::Result<UnsupportedAppRequirementReasons>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWindowsIntegrityPolicyStaticsImpl: Sized {
    fn IsEnabled(&self) -> ::windows::core::Result<bool>;
    fn IsEnabledForTrial(&self) -> ::windows::core::Result<bool>;
    fn CanDisable(&self) -> ::windows::core::Result<bool>;
    fn IsDisableSupported(&self) -> ::windows::core::Result<bool>;
    fn PolicyChanged(&self, handler: &::core::option::Option<super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePolicyChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
