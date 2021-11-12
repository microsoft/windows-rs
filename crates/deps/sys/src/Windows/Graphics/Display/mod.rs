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
    pub const StandardDynamicRange: Self = Self(0i32);
    pub const WideColorGamut: Self = Self(1i32);
    pub const HighDynamicRange: Self = Self(2i32);
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
    pub const None: Self = Self(0u32);
    pub const UseDimmedPolicyWhenBatteryIsLow: Self = Self(1u32);
}
#[repr(transparent)]
pub struct DisplayBrightnessOverrideScenario(pub i32);
impl DisplayBrightnessOverrideScenario {
    pub const IdleBrightness: Self = Self(0i32);
    pub const BarcodeReadingBrightness: Self = Self(1i32);
    pub const FullBrightness: Self = Self(2i32);
}
#[repr(transparent)]
pub struct DisplayBrightnessScenario(pub i32);
impl DisplayBrightnessScenario {
    pub const DefaultBrightness: Self = Self(0i32);
    pub const IdleBrightness: Self = Self(1i32);
    pub const BarcodeReadingBrightness: Self = Self(2i32);
    pub const FullBrightness: Self = Self(3i32);
}
#[repr(transparent)]
pub struct DisplayColorOverrideScenario(pub i32);
impl DisplayColorOverrideScenario {
    pub const Accurate: Self = Self(0i32);
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
    pub const None: Self = Self(0u32);
    pub const Landscape: Self = Self(1u32);
    pub const Portrait: Self = Self(2u32);
    pub const LandscapeFlipped: Self = Self(4u32);
    pub const PortraitFlipped: Self = Self(8u32);
}
#[repr(transparent)]
pub struct DisplayPropertiesEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DisplayServices(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HdrMetadataFormat(pub i32);
impl HdrMetadataFormat {
    pub const Hdr10: Self = Self(0i32);
    pub const Hdr10Plus: Self = Self(1i32);
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
pub struct NitRange {
    pub MinNits: f32,
    pub MaxNits: f32,
    pub StepSizeNits: f32,
}
impl ::core::marker::Copy for NitRange {}
impl ::core::clone::Clone for NitRange {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ResolutionScale(pub i32);
impl ResolutionScale {
    pub const Invalid: Self = Self(0i32);
    pub const Scale100Percent: Self = Self(100i32);
    pub const Scale120Percent: Self = Self(120i32);
    pub const Scale125Percent: Self = Self(125i32);
    pub const Scale140Percent: Self = Self(140i32);
    pub const Scale150Percent: Self = Self(150i32);
    pub const Scale160Percent: Self = Self(160i32);
    pub const Scale175Percent: Self = Self(175i32);
    pub const Scale180Percent: Self = Self(180i32);
    pub const Scale200Percent: Self = Self(200i32);
    pub const Scale225Percent: Self = Self(225i32);
    pub const Scale250Percent: Self = Self(250i32);
    pub const Scale300Percent: Self = Self(300i32);
    pub const Scale350Percent: Self = Self(350i32);
    pub const Scale400Percent: Self = Self(400i32);
    pub const Scale450Percent: Self = Self(450i32);
    pub const Scale500Percent: Self = Self(500i32);
}
