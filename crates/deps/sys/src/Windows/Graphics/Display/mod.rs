#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Graphics_Display_Core")]
pub mod Core;
#[link(name = "windows")]
extern "system" {}
pub struct AdvancedColorInfo(i32);
pub struct AdvancedColorKind(i32);
pub struct BrightnessOverride(i32);
pub struct BrightnessOverrideSettings(i32);
pub struct ColorOverrideSettings(i32);
pub struct DisplayBrightnessOverrideOptions(i32);
pub struct DisplayBrightnessOverrideScenario(i32);
pub struct DisplayBrightnessScenario(i32);
pub struct DisplayColorOverrideScenario(i32);
pub struct DisplayEnhancementOverride(i32);
pub struct DisplayEnhancementOverrideCapabilities(i32);
pub struct DisplayEnhancementOverrideCapabilitiesChangedEventArgs(i32);
pub struct DisplayInformation(i32);
pub struct DisplayOrientations(i32);
pub struct DisplayProperties(i32);
pub struct DisplayPropertiesEventHandler(pub *mut ::core::ffi::c_void);
pub struct DisplayServices(i32);
pub struct HdrMetadataFormat(i32);
pub struct IAdvancedColorInfo(pub *mut ::core::ffi::c_void);
pub struct IBrightnessOverride(pub *mut ::core::ffi::c_void);
pub struct IBrightnessOverrideSettings(pub *mut ::core::ffi::c_void);
pub struct IBrightnessOverrideSettingsStatics(pub *mut ::core::ffi::c_void);
pub struct IBrightnessOverrideStatics(pub *mut ::core::ffi::c_void);
pub struct IColorOverrideSettings(pub *mut ::core::ffi::c_void);
pub struct IColorOverrideSettingsStatics(pub *mut ::core::ffi::c_void);
pub struct IDisplayEnhancementOverride(pub *mut ::core::ffi::c_void);
pub struct IDisplayEnhancementOverrideCapabilities(pub *mut ::core::ffi::c_void);
pub struct IDisplayEnhancementOverrideCapabilitiesChangedEventArgs(pub *mut ::core::ffi::c_void);
pub struct IDisplayEnhancementOverrideStatics(pub *mut ::core::ffi::c_void);
pub struct IDisplayInformation(pub *mut ::core::ffi::c_void);
pub struct IDisplayInformation2(pub *mut ::core::ffi::c_void);
pub struct IDisplayInformation3(pub *mut ::core::ffi::c_void);
pub struct IDisplayInformation4(pub *mut ::core::ffi::c_void);
pub struct IDisplayInformation5(pub *mut ::core::ffi::c_void);
pub struct IDisplayInformationStatics(pub *mut ::core::ffi::c_void);
pub struct IDisplayPropertiesStatics(pub *mut ::core::ffi::c_void);
pub struct IDisplayServices(pub *mut ::core::ffi::c_void);
pub struct IDisplayServicesStatics(pub *mut ::core::ffi::c_void);
pub struct NitRange(i32);
pub struct ResolutionScale(i32);
