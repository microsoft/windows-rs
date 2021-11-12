#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "Graphics_Display_Core")]
pub mod Core;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AdvancedColorInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AdvancedColorKind(pub i32);
impl AdvancedColorKind {
    pub const StandardDynamicRange: AdvancedColorKind = AdvancedColorKind(0i32);
    pub const WideColorGamut: AdvancedColorKind = AdvancedColorKind(1i32);
    pub const HighDynamicRange: AdvancedColorKind = AdvancedColorKind(2i32);
}
#[repr(transparent)]
pub struct BrightnessOverride(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BrightnessOverrideSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ColorOverrideSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DisplayBrightnessOverrideOptions(pub u32);
impl DisplayBrightnessOverrideOptions {
    pub const None: DisplayBrightnessOverrideOptions = DisplayBrightnessOverrideOptions(0u32);
    pub const UseDimmedPolicyWhenBatteryIsLow: DisplayBrightnessOverrideOptions = DisplayBrightnessOverrideOptions(1u32);
}
#[repr(transparent)]
pub struct DisplayBrightnessOverrideScenario(pub i32);
impl DisplayBrightnessOverrideScenario {
    pub const IdleBrightness: DisplayBrightnessOverrideScenario = DisplayBrightnessOverrideScenario(0i32);
    pub const BarcodeReadingBrightness: DisplayBrightnessOverrideScenario = DisplayBrightnessOverrideScenario(1i32);
    pub const FullBrightness: DisplayBrightnessOverrideScenario = DisplayBrightnessOverrideScenario(2i32);
}
#[repr(transparent)]
pub struct DisplayBrightnessScenario(pub i32);
impl DisplayBrightnessScenario {
    pub const DefaultBrightness: DisplayBrightnessScenario = DisplayBrightnessScenario(0i32);
    pub const IdleBrightness: DisplayBrightnessScenario = DisplayBrightnessScenario(1i32);
    pub const BarcodeReadingBrightness: DisplayBrightnessScenario = DisplayBrightnessScenario(2i32);
    pub const FullBrightness: DisplayBrightnessScenario = DisplayBrightnessScenario(3i32);
}
#[repr(transparent)]
pub struct DisplayColorOverrideScenario(pub i32);
impl DisplayColorOverrideScenario {
    pub const Accurate: DisplayColorOverrideScenario = DisplayColorOverrideScenario(0i32);
}
#[repr(transparent)]
pub struct DisplayEnhancementOverride(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DisplayEnhancementOverrideCapabilities(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DisplayEnhancementOverrideCapabilitiesChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DisplayInformation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DisplayOrientations(pub u32);
impl DisplayOrientations {
    pub const None: DisplayOrientations = DisplayOrientations(0u32);
    pub const Landscape: DisplayOrientations = DisplayOrientations(1u32);
    pub const Portrait: DisplayOrientations = DisplayOrientations(2u32);
    pub const LandscapeFlipped: DisplayOrientations = DisplayOrientations(4u32);
    pub const PortraitFlipped: DisplayOrientations = DisplayOrientations(8u32);
}
#[repr(transparent)]
pub struct DisplayPropertiesEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DisplayServices(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HdrMetadataFormat(pub i32);
impl HdrMetadataFormat {
    pub const Hdr10: HdrMetadataFormat = HdrMetadataFormat(0i32);
    pub const Hdr10Plus: HdrMetadataFormat = HdrMetadataFormat(1i32);
}
#[repr(transparent)]
pub struct IAdvancedColorInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBrightnessOverride(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBrightnessOverrideSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBrightnessOverrideSettingsStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBrightnessOverrideStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IColorOverrideSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IColorOverrideSettingsStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDisplayEnhancementOverride(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDisplayEnhancementOverrideCapabilities(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDisplayEnhancementOverrideCapabilitiesChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDisplayEnhancementOverrideStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDisplayInformation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDisplayInformation2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDisplayInformation3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDisplayInformation4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDisplayInformation5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDisplayInformationStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDisplayPropertiesStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDisplayServices(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDisplayServicesStatics(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct NitRange(i32);
#[repr(transparent)]
pub struct ResolutionScale(pub i32);
impl ResolutionScale {
    pub const Invalid: ResolutionScale = ResolutionScale(0i32);
    pub const Scale100Percent: ResolutionScale = ResolutionScale(100i32);
    pub const Scale120Percent: ResolutionScale = ResolutionScale(120i32);
    pub const Scale125Percent: ResolutionScale = ResolutionScale(125i32);
    pub const Scale140Percent: ResolutionScale = ResolutionScale(140i32);
    pub const Scale150Percent: ResolutionScale = ResolutionScale(150i32);
    pub const Scale160Percent: ResolutionScale = ResolutionScale(160i32);
    pub const Scale175Percent: ResolutionScale = ResolutionScale(175i32);
    pub const Scale180Percent: ResolutionScale = ResolutionScale(180i32);
    pub const Scale200Percent: ResolutionScale = ResolutionScale(200i32);
    pub const Scale225Percent: ResolutionScale = ResolutionScale(225i32);
    pub const Scale250Percent: ResolutionScale = ResolutionScale(250i32);
    pub const Scale300Percent: ResolutionScale = ResolutionScale(300i32);
    pub const Scale350Percent: ResolutionScale = ResolutionScale(350i32);
    pub const Scale400Percent: ResolutionScale = ResolutionScale(400i32);
    pub const Scale450Percent: ResolutionScale = ResolutionScale(450i32);
    pub const Scale500Percent: ResolutionScale = ResolutionScale(500i32);
}
