#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Graphics_Display_Core")]
pub mod Core;
#[link(name = "windows")]
extern "system" {
    fn AdvancedColorInfo();
    fn AdvancedColorKind();
    fn BrightnessOverride();
    fn BrightnessOverrideSettings();
    fn ColorOverrideSettings();
    fn DisplayBrightnessOverrideOptions();
    fn DisplayBrightnessOverrideScenario();
    fn DisplayBrightnessScenario();
    fn DisplayColorOverrideScenario();
    fn DisplayEnhancementOverride();
    fn DisplayEnhancementOverrideCapabilities();
    fn DisplayEnhancementOverrideCapabilitiesChangedEventArgs();
    fn DisplayInformation();
    fn DisplayOrientations();
    fn DisplayProperties();
    fn DisplayPropertiesEventHandler();
    fn DisplayServices();
    fn HdrMetadataFormat();
    fn IAdvancedColorInfo();
    fn IBrightnessOverride();
    fn IBrightnessOverrideSettings();
    fn IBrightnessOverrideSettingsStatics();
    fn IBrightnessOverrideStatics();
    fn IColorOverrideSettings();
    fn IColorOverrideSettingsStatics();
    fn IDisplayEnhancementOverride();
    fn IDisplayEnhancementOverrideCapabilities();
    fn IDisplayEnhancementOverrideCapabilitiesChangedEventArgs();
    fn IDisplayEnhancementOverrideStatics();
    fn IDisplayInformation();
    fn IDisplayInformation2();
    fn IDisplayInformation3();
    fn IDisplayInformation4();
    fn IDisplayInformation5();
    fn IDisplayInformationStatics();
    fn IDisplayPropertiesStatics();
    fn IDisplayServices();
    fn IDisplayServicesStatics();
    fn NitRange();
    fn ResolutionScale();
}
