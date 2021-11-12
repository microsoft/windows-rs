#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "Graphics_Display_Core")]
pub mod Core;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AdvancedColorInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AdvancedColorInfo {}
impl ::core::clone::Clone for AdvancedColorInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AdvancedColorKind(pub i32);
impl AdvancedColorKind {
    pub const StandardDynamicRange: Self = Self(0i32);
    pub const WideColorGamut: Self = Self(1i32);
    pub const HighDynamicRange: Self = Self(2i32);
}
impl ::core::marker::Copy for AdvancedColorKind {}
impl ::core::clone::Clone for AdvancedColorKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BrightnessOverride(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BrightnessOverride {}
impl ::core::clone::Clone for BrightnessOverride {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BrightnessOverrideSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BrightnessOverrideSettings {}
impl ::core::clone::Clone for BrightnessOverrideSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ColorOverrideSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ColorOverrideSettings {}
impl ::core::clone::Clone for ColorOverrideSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DisplayBrightnessOverrideOptions(pub u32);
impl DisplayBrightnessOverrideOptions {
    pub const None: Self = Self(0u32);
    pub const UseDimmedPolicyWhenBatteryIsLow: Self = Self(1u32);
}
impl ::core::marker::Copy for DisplayBrightnessOverrideOptions {}
impl ::core::clone::Clone for DisplayBrightnessOverrideOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DisplayBrightnessOverrideScenario(pub i32);
impl DisplayBrightnessOverrideScenario {
    pub const IdleBrightness: Self = Self(0i32);
    pub const BarcodeReadingBrightness: Self = Self(1i32);
    pub const FullBrightness: Self = Self(2i32);
}
impl ::core::marker::Copy for DisplayBrightnessOverrideScenario {}
impl ::core::clone::Clone for DisplayBrightnessOverrideScenario {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DisplayBrightnessScenario(pub i32);
impl DisplayBrightnessScenario {
    pub const DefaultBrightness: Self = Self(0i32);
    pub const IdleBrightness: Self = Self(1i32);
    pub const BarcodeReadingBrightness: Self = Self(2i32);
    pub const FullBrightness: Self = Self(3i32);
}
impl ::core::marker::Copy for DisplayBrightnessScenario {}
impl ::core::clone::Clone for DisplayBrightnessScenario {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DisplayColorOverrideScenario(pub i32);
impl DisplayColorOverrideScenario {
    pub const Accurate: Self = Self(0i32);
}
impl ::core::marker::Copy for DisplayColorOverrideScenario {}
impl ::core::clone::Clone for DisplayColorOverrideScenario {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DisplayEnhancementOverride(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DisplayEnhancementOverride {}
impl ::core::clone::Clone for DisplayEnhancementOverride {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DisplayEnhancementOverrideCapabilities(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DisplayEnhancementOverrideCapabilities {}
impl ::core::clone::Clone for DisplayEnhancementOverrideCapabilities {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DisplayEnhancementOverrideCapabilitiesChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DisplayEnhancementOverrideCapabilitiesChangedEventArgs {}
impl ::core::clone::Clone for DisplayEnhancementOverrideCapabilitiesChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DisplayInformation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DisplayInformation {}
impl ::core::clone::Clone for DisplayInformation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DisplayOrientations(pub u32);
impl DisplayOrientations {
    pub const None: Self = Self(0u32);
    pub const Landscape: Self = Self(1u32);
    pub const Portrait: Self = Self(2u32);
    pub const LandscapeFlipped: Self = Self(4u32);
    pub const PortraitFlipped: Self = Self(8u32);
}
impl ::core::marker::Copy for DisplayOrientations {}
impl ::core::clone::Clone for DisplayOrientations {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DisplayPropertiesEventHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DisplayPropertiesEventHandler {}
impl ::core::clone::Clone for DisplayPropertiesEventHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DisplayServices(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DisplayServices {}
impl ::core::clone::Clone for DisplayServices {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HdrMetadataFormat(pub i32);
impl HdrMetadataFormat {
    pub const Hdr10: Self = Self(0i32);
    pub const Hdr10Plus: Self = Self(1i32);
}
impl ::core::marker::Copy for HdrMetadataFormat {}
impl ::core::clone::Clone for HdrMetadataFormat {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAdvancedColorInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAdvancedColorInfo {}
impl ::core::clone::Clone for IAdvancedColorInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBrightnessOverride(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBrightnessOverride {}
impl ::core::clone::Clone for IBrightnessOverride {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBrightnessOverrideSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBrightnessOverrideSettings {}
impl ::core::clone::Clone for IBrightnessOverrideSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBrightnessOverrideSettingsStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBrightnessOverrideSettingsStatics {}
impl ::core::clone::Clone for IBrightnessOverrideSettingsStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBrightnessOverrideStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBrightnessOverrideStatics {}
impl ::core::clone::Clone for IBrightnessOverrideStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IColorOverrideSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IColorOverrideSettings {}
impl ::core::clone::Clone for IColorOverrideSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IColorOverrideSettingsStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IColorOverrideSettingsStatics {}
impl ::core::clone::Clone for IColorOverrideSettingsStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDisplayEnhancementOverride(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDisplayEnhancementOverride {}
impl ::core::clone::Clone for IDisplayEnhancementOverride {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDisplayEnhancementOverrideCapabilities(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDisplayEnhancementOverrideCapabilities {}
impl ::core::clone::Clone for IDisplayEnhancementOverrideCapabilities {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDisplayEnhancementOverrideCapabilitiesChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDisplayEnhancementOverrideCapabilitiesChangedEventArgs {}
impl ::core::clone::Clone for IDisplayEnhancementOverrideCapabilitiesChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDisplayEnhancementOverrideStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDisplayEnhancementOverrideStatics {}
impl ::core::clone::Clone for IDisplayEnhancementOverrideStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDisplayInformation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDisplayInformation {}
impl ::core::clone::Clone for IDisplayInformation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDisplayInformation2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDisplayInformation2 {}
impl ::core::clone::Clone for IDisplayInformation2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDisplayInformation3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDisplayInformation3 {}
impl ::core::clone::Clone for IDisplayInformation3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDisplayInformation4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDisplayInformation4 {}
impl ::core::clone::Clone for IDisplayInformation4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDisplayInformation5(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDisplayInformation5 {}
impl ::core::clone::Clone for IDisplayInformation5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDisplayInformationStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDisplayInformationStatics {}
impl ::core::clone::Clone for IDisplayInformationStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDisplayPropertiesStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDisplayPropertiesStatics {}
impl ::core::clone::Clone for IDisplayPropertiesStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDisplayServices(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDisplayServices {}
impl ::core::clone::Clone for IDisplayServices {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDisplayServicesStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDisplayServicesStatics {}
impl ::core::clone::Clone for IDisplayServicesStatics {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for ResolutionScale {}
impl ::core::clone::Clone for ResolutionScale {
    fn clone(&self) -> Self {
        *self
    }
}
