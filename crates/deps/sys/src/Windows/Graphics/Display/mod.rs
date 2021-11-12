#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Graphics_Display_Core")]
pub mod Core;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AdvancedColorInfo(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct AdvancedColorKind(i32);
#[repr(transparent)]
pub struct BrightnessOverride(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BrightnessOverrideSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ColorOverrideSettings(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct DisplayBrightnessOverrideOptions(i32);
#[repr(C)]
pub struct DisplayBrightnessOverrideScenario(i32);
#[repr(C)]
pub struct DisplayBrightnessScenario(i32);
#[repr(C)]
pub struct DisplayColorOverrideScenario(i32);
#[repr(transparent)]
pub struct DisplayEnhancementOverride(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DisplayEnhancementOverrideCapabilities(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DisplayEnhancementOverrideCapabilitiesChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DisplayInformation(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct DisplayOrientations(i32);
#[repr(transparent)]
pub struct DisplayPropertiesEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DisplayServices(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct HdrMetadataFormat(i32);
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
#[repr(C)]
pub struct ResolutionScale(i32);
