#[cfg(feature = "implement_exclusive")]
pub trait IAdvancedColorInfoImpl: Sized {
    fn CurrentAdvancedColorKind(&self) -> ::windows::core::Result<AdvancedColorKind>;
    fn RedPrimary(&self) -> ::windows::core::Result<super::super::Foundation::Point>;
    fn GreenPrimary(&self) -> ::windows::core::Result<super::super::Foundation::Point>;
    fn BluePrimary(&self) -> ::windows::core::Result<super::super::Foundation::Point>;
    fn WhitePoint(&self) -> ::windows::core::Result<super::super::Foundation::Point>;
    fn MaxLuminanceInNits(&self) -> ::windows::core::Result<f32>;
    fn MinLuminanceInNits(&self) -> ::windows::core::Result<f32>;
    fn MaxAverageFullFrameLuminanceInNits(&self) -> ::windows::core::Result<f32>;
    fn SdrWhiteLevelInNits(&self) -> ::windows::core::Result<f32>;
    fn IsHdrMetadataFormatCurrentlySupported(&self, format: HdrMetadataFormat) -> ::windows::core::Result<bool>;
    fn IsAdvancedColorKindAvailable(&self, kind: AdvancedColorKind) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBrightnessOverrideImpl: Sized {
    fn IsSupported(&self) -> ::windows::core::Result<bool>;
    fn IsOverrideActive(&self) -> ::windows::core::Result<bool>;
    fn BrightnessLevel(&self) -> ::windows::core::Result<f64>;
    fn SetBrightnessLevel(&self, brightnesslevel: f64, options: DisplayBrightnessOverrideOptions) -> ::windows::core::Result<()>;
    fn SetBrightnessScenario(&self, scenario: DisplayBrightnessScenario, options: DisplayBrightnessOverrideOptions) -> ::windows::core::Result<()>;
    fn GetLevelForScenario(&self, scenario: DisplayBrightnessScenario) -> ::windows::core::Result<f64>;
    fn StartOverride(&self) -> ::windows::core::Result<()>;
    fn StopOverride(&self) -> ::windows::core::Result<()>;
    fn IsSupportedChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<BrightnessOverride, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveIsSupportedChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn IsOverrideActiveChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<BrightnessOverride, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveIsOverrideActiveChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn BrightnessLevelChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<BrightnessOverride, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveBrightnessLevelChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBrightnessOverrideSettingsImpl: Sized {
    fn DesiredLevel(&self) -> ::windows::core::Result<f64>;
    fn DesiredNits(&self) -> ::windows::core::Result<f32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBrightnessOverrideSettingsStaticsImpl: Sized {
    fn CreateFromLevel(&self, level: f64) -> ::windows::core::Result<BrightnessOverrideSettings>;
    fn CreateFromNits(&self, nits: f32) -> ::windows::core::Result<BrightnessOverrideSettings>;
    fn CreateFromDisplayBrightnessOverrideScenario(&self, overridescenario: DisplayBrightnessOverrideScenario) -> ::windows::core::Result<BrightnessOverrideSettings>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBrightnessOverrideStaticsImpl: Sized {
    fn GetDefaultForSystem(&self) -> ::windows::core::Result<BrightnessOverride>;
    fn GetForCurrentView(&self) -> ::windows::core::Result<BrightnessOverride>;
    fn SaveForSystemAsync(&self, value: &::core::option::Option<BrightnessOverride>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IColorOverrideSettingsImpl: Sized {
    fn DesiredDisplayColorOverrideScenario(&self) -> ::windows::core::Result<DisplayColorOverrideScenario>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IColorOverrideSettingsStaticsImpl: Sized {
    fn CreateFromDisplayColorOverrideScenario(&self, overridescenario: DisplayColorOverrideScenario) -> ::windows::core::Result<ColorOverrideSettings>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplayEnhancementOverrideImpl: Sized {
    fn ColorOverrideSettings(&self) -> ::windows::core::Result<ColorOverrideSettings>;
    fn SetColorOverrideSettings(&self, value: &::core::option::Option<ColorOverrideSettings>) -> ::windows::core::Result<()>;
    fn BrightnessOverrideSettings(&self) -> ::windows::core::Result<BrightnessOverrideSettings>;
    fn SetBrightnessOverrideSettings(&self, value: &::core::option::Option<BrightnessOverrideSettings>) -> ::windows::core::Result<()>;
    fn CanOverride(&self) -> ::windows::core::Result<bool>;
    fn IsOverrideActive(&self) -> ::windows::core::Result<bool>;
    fn GetCurrentDisplayEnhancementOverrideCapabilities(&self) -> ::windows::core::Result<DisplayEnhancementOverrideCapabilities>;
    fn RequestOverride(&self) -> ::windows::core::Result<()>;
    fn StopOverride(&self) -> ::windows::core::Result<()>;
    fn CanOverrideChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<DisplayEnhancementOverride, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCanOverrideChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn IsOverrideActiveChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<DisplayEnhancementOverride, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveIsOverrideActiveChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DisplayEnhancementOverrideCapabilitiesChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<DisplayEnhancementOverride, DisplayEnhancementOverrideCapabilitiesChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDisplayEnhancementOverrideCapabilitiesChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplayEnhancementOverrideCapabilitiesImpl: Sized {
    fn IsBrightnessControlSupported(&self) -> ::windows::core::Result<bool>;
    fn IsBrightnessNitsControlSupported(&self) -> ::windows::core::Result<bool>;
    fn GetSupportedNitRanges(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<NitRange>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplayEnhancementOverrideCapabilitiesChangedEventArgsImpl: Sized {
    fn Capabilities(&self) -> ::windows::core::Result<DisplayEnhancementOverrideCapabilities>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplayEnhancementOverrideStaticsImpl: Sized {
    fn GetForCurrentView(&self) -> ::windows::core::Result<DisplayEnhancementOverride>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplayInformationImpl: Sized {
    fn CurrentOrientation(&self) -> ::windows::core::Result<DisplayOrientations>;
    fn NativeOrientation(&self) -> ::windows::core::Result<DisplayOrientations>;
    fn OrientationChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<DisplayInformation, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveOrientationChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ResolutionScale(&self) -> ::windows::core::Result<ResolutionScale>;
    fn LogicalDpi(&self) -> ::windows::core::Result<f32>;
    fn RawDpiX(&self) -> ::windows::core::Result<f32>;
    fn RawDpiY(&self) -> ::windows::core::Result<f32>;
    fn DpiChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<DisplayInformation, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDpiChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn StereoEnabled(&self) -> ::windows::core::Result<bool>;
    fn StereoEnabledChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<DisplayInformation, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStereoEnabledChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GetColorProfileAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStream>>;
    fn ColorProfileChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<DisplayInformation, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveColorProfileChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplayInformation2Impl: Sized + IDisplayInformationImpl {
    fn RawPixelsPerViewPixel(&self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplayInformation3Impl: Sized {
    fn DiagonalSizeInInches(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplayInformation4Impl: Sized {
    fn ScreenWidthInRawPixels(&self) -> ::windows::core::Result<u32>;
    fn ScreenHeightInRawPixels(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplayInformation5Impl: Sized {
    fn GetAdvancedColorInfo(&self) -> ::windows::core::Result<AdvancedColorInfo>;
    fn AdvancedColorInfoChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<DisplayInformation, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAdvancedColorInfoChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplayInformationStaticsImpl: Sized {
    fn GetForCurrentView(&self) -> ::windows::core::Result<DisplayInformation>;
    fn AutoRotationPreferences(&self) -> ::windows::core::Result<DisplayOrientations>;
    fn SetAutoRotationPreferences(&self, value: DisplayOrientations) -> ::windows::core::Result<()>;
    fn DisplayContentsInvalidated(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<DisplayInformation, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDisplayContentsInvalidated(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IDisplayPropertiesStaticsImpl: Sized {
    fn CurrentOrientation(&self) -> ::windows::core::Result<DisplayOrientations>;
    fn NativeOrientation(&self) -> ::windows::core::Result<DisplayOrientations>;
    fn AutoRotationPreferences(&self) -> ::windows::core::Result<DisplayOrientations>;
    fn SetAutoRotationPreferences(&self, value: DisplayOrientations) -> ::windows::core::Result<()>;
    fn OrientationChanged(&self, handler: &::core::option::Option<DisplayPropertiesEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveOrientationChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ResolutionScale(&self) -> ::windows::core::Result<ResolutionScale>;
    fn LogicalDpi(&self) -> ::windows::core::Result<f32>;
    fn LogicalDpiChanged(&self, handler: &::core::option::Option<DisplayPropertiesEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveLogicalDpiChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn StereoEnabled(&self) -> ::windows::core::Result<bool>;
    fn StereoEnabledChanged(&self, handler: &::core::option::Option<DisplayPropertiesEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStereoEnabledChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GetColorProfileAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStream>>;
    fn ColorProfileChanged(&self, handler: &::core::option::Option<DisplayPropertiesEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveColorProfileChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DisplayContentsInvalidated(&self, handler: &::core::option::Option<DisplayPropertiesEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDisplayContentsInvalidated(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplayServicesImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplayServicesStaticsImpl: Sized {
    fn FindAll(&self) -> ::windows::core::Result<::windows::core::Array<super::DisplayId>>;
}
