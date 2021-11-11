#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AssociateColorProfileWithDeviceA();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AssociateColorProfileWithDeviceW();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CMCheckColors();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn CMCheckColorsInGamut();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CMCheckRGBs();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CMConvertColorNameToIndex();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CMConvertIndexToColorName();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CMCreateDeviceLinkProfile();
    #[doc = "*Required features: `Win32_UI_ColorSystem`*"]
    pub fn CMCreateMultiProfileTransform();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn CMCreateProfile();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn CMCreateProfileW();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn CMCreateTransform();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn CMCreateTransformExt();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn CMCreateTransformExtW();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn CMCreateTransformW();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CMDeleteTransform();
    #[doc = "*Required features: `Win32_UI_ColorSystem`*"]
    pub fn CMGetInfo();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CMGetNamedProfileInfo();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CMIsProfileValid();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CMTranslateColors();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CMTranslateRGB();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CMTranslateRGBs();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CMTranslateRGBsExt();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CheckBitmapBits();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CheckColors();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn CheckColorsInGamut();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CloseColorProfile();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn ColorCorrectPalette();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn ColorMatchToTarget();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ColorProfileAddDisplayAssociation();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ColorProfileGetDisplayDefault();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ColorProfileGetDisplayList();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ColorProfileGetDisplayUserScope();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ColorProfileRemoveDisplayAssociation();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ColorProfileSetDisplayDefaultAssociation();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConvertColorNameToIndex();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConvertIndexToColorName();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn CreateColorSpaceA();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn CreateColorSpaceW();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn CreateColorTransformA();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn CreateColorTransformW();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateDeviceLinkProfile();
    #[doc = "*Required features: `Win32_UI_ColorSystem`*"]
    pub fn CreateMultiProfileTransform();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn CreateProfileFromLogColorSpaceA();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn CreateProfileFromLogColorSpaceW();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteColorSpace();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteColorTransform();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DisassociateColorProfileFromDeviceA();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DisassociateColorProfileFromDeviceW();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumColorProfilesA();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumColorProfilesW();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn EnumICMProfilesA();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn EnumICMProfilesW();
    #[doc = "*Required features: `Win32_UI_ColorSystem`*"]
    pub fn GetCMMInfo();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetColorDirectoryA();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetColorDirectoryW();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetColorProfileElement();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetColorProfileElementTag();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetColorProfileFromHandle();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn GetColorProfileHeader();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn GetColorSpace();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCountColorProfileElements();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn GetDeviceGammaRamp();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn GetICMProfileA();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn GetICMProfileW();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn GetLogColorSpaceA();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn GetLogColorSpaceW();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNamedProfileInfo();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPS2ColorRenderingDictionary();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPS2ColorRenderingIntent();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPS2ColorSpaceArray();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetStandardColorSpaceProfileA();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetStandardColorSpaceProfileW();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InstallColorProfileA();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InstallColorProfileW();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsColorProfileTagPresent();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsColorProfileValid();
    #[doc = "*Required features: `Win32_UI_ColorSystem`*"]
    pub fn OpenColorProfileA();
    #[doc = "*Required features: `Win32_UI_ColorSystem`*"]
    pub fn OpenColorProfileW();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterCMMA();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterCMMW();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SelectCMM();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetColorProfileElement();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetColorProfileElementReference();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetColorProfileElementSize();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn SetColorProfileHeader();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn SetColorSpace();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn SetDeviceGammaRamp();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn SetICMMode();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn SetICMProfileA();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn SetICMProfileW();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetStandardColorSpaceProfileA();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetStandardColorSpaceProfileW();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn SetupColorMatchingA();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn SetupColorMatchingW();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TranslateBitmapBits();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TranslateColors();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UninstallColorProfileA();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UninstallColorProfileW();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnregisterCMMA();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnregisterCMMW();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UpdateICMRegKeyA();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UpdateICMRegKeyW();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WcsAssociateColorProfileWithDevice();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WcsCheckColors();
    #[doc = "*Required features: `Win32_UI_ColorSystem`*"]
    pub fn WcsCreateIccProfile();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WcsDisassociateColorProfileFromDevice();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WcsEnumColorProfiles();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WcsEnumColorProfilesSize();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WcsGetCalibrationManagementState();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WcsGetDefaultColorProfile();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WcsGetDefaultColorProfileSize();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WcsGetDefaultRenderingIntent();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WcsGetUsePerUserProfiles();
    #[doc = "*Required features: `Win32_UI_ColorSystem`*"]
    pub fn WcsOpenColorProfileA();
    #[doc = "*Required features: `Win32_UI_ColorSystem`*"]
    pub fn WcsOpenColorProfileW();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WcsSetCalibrationManagementState();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WcsSetDefaultColorProfile();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WcsSetDefaultRenderingIntent();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WcsSetUsePerUserProfiles();
    #[doc = "*Required features: `Win32_UI_ColorSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WcsTranslateColors();
}
