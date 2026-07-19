windows_link::link!("mscms.dll" "system" fn AssociateColorProfileWithDeviceA(pmachinename : windows_sys::core::PCSTR, pprofilename : windows_sys::core::PCSTR, pdevicename : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("mscms.dll" "system" fn AssociateColorProfileWithDeviceW(pmachinename : windows_sys::core::PCWSTR, pprofilename : windows_sys::core::PCWSTR, pdevicename : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("icm32.dll" "system" fn CMCheckColors(hcmtransform : HCMTRANSFORM, lpainputcolors : *const COLOR, ncolors : u32, ctinput : COLORTYPE, lparesult : *mut u8) -> windows_sys::core::BOOL);
#[cfg(all(feature = "wingdi", feature = "winnt"))]
windows_link::link!("icm32.dll" "system" fn CMCheckColorsInGamut(hcmtransform : HCMTRANSFORM, lpargbtriple : *const super::RGBTRIPLE, lparesult : *mut u8, ncount : u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("icm32.dll" "system" fn CMCheckRGBs(hcmtransform : HCMTRANSFORM, lpsrcbits : *const core::ffi::c_void, bminput : BMFORMAT, dwwidth : u32, dwheight : u32, dwstride : u32, lparesult : *mut u8, pfncallback : PBMCALLBACKFN, ulcallbackdata : super::LPARAM) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("icm32.dll" "system" fn CMConvertColorNameToIndex(hprofile : HPROFILE, pacolorname : *const COLOR_NAME, paindex : *mut u32, dwcount : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("icm32.dll" "system" fn CMConvertIndexToColorName(hprofile : HPROFILE, paindex : *const u32, pacolorname : *mut COLOR_NAME, dwcount : u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("icm32.dll" "system" fn CMCreateDeviceLinkProfile(pahprofiles : *const HPROFILE, nprofiles : u32, padwintents : *const u32, nintents : u32, dwflags : u32, lpprofiledata : *mut super::LPBYTE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("icm32.dll" "system" fn CMCreateMultiProfileTransform(pahprofiles : *const HPROFILE, nprofiles : u32, padwintents : *const u32, nintents : u32, dwflags : u32) -> HCMTRANSFORM);
#[cfg(feature = "wingdi")]
windows_link::link!("icm32.dll" "system" fn CMCreateProfile(lpcolorspace : *mut super::LOGCOLORSPACEA, lpprofiledata : *mut LPDEVCHARACTER) -> windows_sys::core::BOOL);
#[cfg(feature = "wingdi")]
windows_link::link!("icm32.dll" "system" fn CMCreateProfileW(lpcolorspace : *mut super::LOGCOLORSPACEW, lpprofiledata : *mut LPDEVCHARACTER) -> windows_sys::core::BOOL);
#[cfg(all(feature = "wingdi", feature = "winnt"))]
windows_link::link!("icm32.dll" "system" fn CMCreateTransform(lpcolorspace : *const super::LOGCOLORSPACEA, lpdevcharacter : LPDEVCHARACTER, lptargetdevcharacter : LPDEVCHARACTER) -> HCMTRANSFORM);
#[cfg(all(feature = "wingdi", feature = "winnt"))]
windows_link::link!("icm32.dll" "system" fn CMCreateTransformExt(lpcolorspace : *const super::LOGCOLORSPACEA, lpdevcharacter : LPDEVCHARACTER, lptargetdevcharacter : LPDEVCHARACTER, dwflags : u32) -> HCMTRANSFORM);
#[cfg(all(feature = "wingdi", feature = "winnt"))]
windows_link::link!("icm32.dll" "system" fn CMCreateTransformExtW(lpcolorspace : *const super::LOGCOLORSPACEW, lpdevcharacter : LPDEVCHARACTER, lptargetdevcharacter : LPDEVCHARACTER, dwflags : u32) -> HCMTRANSFORM);
#[cfg(all(feature = "wingdi", feature = "winnt"))]
windows_link::link!("icm32.dll" "system" fn CMCreateTransformW(lpcolorspace : *const super::LOGCOLORSPACEW, lpdevcharacter : LPDEVCHARACTER, lptargetdevcharacter : LPDEVCHARACTER) -> HCMTRANSFORM);
#[cfg(feature = "winnt")]
windows_link::link!("icm32.dll" "system" fn CMDeleteTransform(hcmtransform : HCMTRANSFORM) -> windows_sys::core::BOOL);
windows_link::link!("icm32.dll" "system" fn CMGetInfo(dwinfo : u32) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("icm32.dll" "system" fn CMGetNamedProfileInfo(hprofile : HPROFILE, pnamedprofileinfo : *mut NAMED_PROFILE_INFO) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("icm32.dll" "system" fn CMIsProfileValid(hprofile : HPROFILE, lpbvalid : *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("icm32.dll" "system" fn CMTranslateColors(hcmtransform : HCMTRANSFORM, lpainputcolors : *const COLOR, ncolors : u32, ctinput : COLORTYPE, lpaoutputcolors : *mut COLOR, ctoutput : COLORTYPE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "windef", feature = "winnt"))]
windows_link::link!("icm32.dll" "system" fn CMTranslateRGB(hcmtransform : HCMTRANSFORM, colorref : super::COLORREF, lpcolorref : *mut u32, dwflags : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("icm32.dll" "system" fn CMTranslateRGBs(hcmtransform : HCMTRANSFORM, lpsrcbits : *const core::ffi::c_void, bminput : BMFORMAT, dwwidth : u32, dwheight : u32, dwstride : u32, lpdestbits : *mut core::ffi::c_void, bmoutput : BMFORMAT, dwtranslatedirection : u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("icm32.dll" "system" fn CMTranslateRGBsExt(hcmtransform : HCMTRANSFORM, lpsrcbits : *const core::ffi::c_void, bminput : BMFORMAT, dwwidth : u32, dwheight : u32, dwinputstride : u32, lpdestbits : *mut core::ffi::c_void, bmoutput : BMFORMAT, dwoutputstride : u32, lpfncallback : LPBMCALLBACKFN, ulcallbackdata : super::LPARAM) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("mscms.dll" "system" fn CheckBitmapBits(hcolortransform : HTRANSFORM, psrcbits : *const core::ffi::c_void, bminput : BMFORMAT, dwwidth : u32, dwheight : u32, dwstride : u32, paresult : *mut u8, pfncallback : PBMCALLBACKFN, lpcallbackdata : super::LPARAM) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("mscms.dll" "system" fn CheckColors(hcolortransform : HTRANSFORM, painputcolors : *const COLOR, ncolors : u32, ctinput : COLORTYPE, paresult : *mut u8) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("mscms.dll" "system" fn CloseColorProfile(hprofile : HPROFILE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("mscms.dll" "system" fn ColorProfileAddDisplayAssociation(scope : WCS_PROFILE_MANAGEMENT_SCOPE, profilename : windows_sys::core::PCWSTR, targetadapterid : super::LUID, sourceid : u32, setasdefault : windows_sys::core::BOOL, associateasadvancedcolor : windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
#[cfg(feature = "winnt")]
windows_link::link!("mscms.dll" "system" fn ColorProfileGetDeviceCapabilities(scope : WCS_PROFILE_MANAGEMENT_SCOPE, targetadapterid : super::LUID, sourceid : u32, capstype : WCS_DEVICE_CAPABILITIES_TYPE, outputcapabilities : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "winnt")]
windows_link::link!("mscms.dll" "system" fn ColorProfileGetDisplayDefault(scope : WCS_PROFILE_MANAGEMENT_SCOPE, targetadapterid : super::LUID, sourceid : u32, profiletype : COLORPROFILETYPE, profilesubtype : COLORPROFILESUBTYPE, profilename : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "winnt")]
windows_link::link!("mscms.dll" "system" fn ColorProfileGetDisplayList(scope : WCS_PROFILE_MANAGEMENT_SCOPE, targetadapterid : super::LUID, sourceid : u32, profilelist : *mut *mut windows_sys::core::PWSTR, profilecount : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(feature = "winnt")]
windows_link::link!("mscms.dll" "system" fn ColorProfileGetDisplayUserScope(targetadapterid : super::LUID, sourceid : u32, scope : *mut WCS_PROFILE_MANAGEMENT_SCOPE) -> windows_sys::core::HRESULT);
#[cfg(feature = "winnt")]
windows_link::link!("mscms.dll" "system" fn ColorProfileRemoveDisplayAssociation(scope : WCS_PROFILE_MANAGEMENT_SCOPE, profilename : windows_sys::core::PCWSTR, targetadapterid : super::LUID, sourceid : u32, dissociateadvancedcolor : windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
#[cfg(feature = "winnt")]
windows_link::link!("mscms.dll" "system" fn ColorProfileSetDisplayDefaultAssociation(scope : WCS_PROFILE_MANAGEMENT_SCOPE, profilename : windows_sys::core::PCWSTR, profiletype : COLORPROFILETYPE, profilesubtype : COLORPROFILESUBTYPE, targetadapterid : super::LUID, sourceid : u32) -> windows_sys::core::HRESULT);
#[cfg(feature = "winnt")]
windows_link::link!("mscms.dll" "system" fn ConvertColorNameToIndex(hprofile : HPROFILE, pacolorname : *const COLOR_NAME, paindex : *mut u32, dwcount : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("mscms.dll" "system" fn ConvertIndexToColorName(hprofile : HPROFILE, paindex : *const u32, pacolorname : *mut COLOR_NAME, dwcount : u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "wingdi", feature = "winnt"))]
windows_link::link!("mscms.dll" "system" fn CreateColorTransformA(plogcolorspace : *const super::LOGCOLORSPACEA, hdestprofile : HPROFILE, htargetprofile : HPROFILE, dwflags : u32) -> HTRANSFORM);
#[cfg(all(feature = "wingdi", feature = "winnt"))]
windows_link::link!("mscms.dll" "system" fn CreateColorTransformW(plogcolorspace : *const super::LOGCOLORSPACEW, hdestprofile : HPROFILE, htargetprofile : HPROFILE, dwflags : u32) -> HTRANSFORM);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("mscms.dll" "system" fn CreateDeviceLinkProfile(hprofile : *const HPROFILE, nprofiles : u32, padwintent : *const u32, nintents : u32, dwflags : u32, pprofiledata : *mut super::PBYTE, indexpreferredcmm : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("mscms.dll" "system" fn CreateMultiProfileTransform(pahprofiles : *const HPROFILE, nprofiles : u32, padwintent : *const u32, nintents : u32, dwflags : u32, indexpreferredcmm : u32) -> HTRANSFORM);
#[cfg(all(feature = "minwindef", feature = "wingdi"))]
windows_link::link!("mscms.dll" "system" fn CreateProfileFromLogColorSpaceA(plogcolorspace : *const super::LOGCOLORSPACEA, pprofile : *mut super::PBYTE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "wingdi"))]
windows_link::link!("mscms.dll" "system" fn CreateProfileFromLogColorSpaceW(plogcolorspace : *const super::LOGCOLORSPACEW, pprofile : *mut super::PBYTE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("mscms.dll" "system" fn DeleteColorTransform(hxform : HTRANSFORM) -> windows_sys::core::BOOL);
windows_link::link!("mscms.dll" "system" fn DisassociateColorProfileFromDeviceA(pmachinename : windows_sys::core::PCSTR, pprofilename : windows_sys::core::PCSTR, pdevicename : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("mscms.dll" "system" fn DisassociateColorProfileFromDeviceW(pmachinename : windows_sys::core::PCWSTR, pprofilename : windows_sys::core::PCWSTR, pdevicename : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("mscms.dll" "system" fn EnumColorProfilesA(pmachinename : windows_sys::core::PCSTR, penumrecord : *const ENUMTYPEA, penumerationbuffer : *mut u8, pdwsizeofenumerationbuffer : *mut u32, pnprofiles : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("mscms.dll" "system" fn EnumColorProfilesW(pmachinename : windows_sys::core::PCWSTR, penumrecord : *const ENUMTYPEW, penumerationbuffer : *mut u8, pdwsizeofenumerationbuffer : *mut u32, pnprofiles : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("mscms.dll" "system" fn GetCMMInfo(hcolortransform : HTRANSFORM, param1 : u32) -> u32);
windows_link::link!("mscms.dll" "system" fn GetColorDirectoryA(pmachinename : windows_sys::core::PCSTR, pbuffer : windows_sys::core::PSTR, pdwsize : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("mscms.dll" "system" fn GetColorDirectoryW(pmachinename : windows_sys::core::PCWSTR, pbuffer : windows_sys::core::PWSTR, pdwsize : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("mscms.dll" "system" fn GetColorProfileElement(hprofile : HPROFILE, tag : TAGTYPE, dwoffset : u32, pcbelement : *mut u32, pelement : *mut core::ffi::c_void, pbreference : *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("mscms.dll" "system" fn GetColorProfileElementTag(hprofile : HPROFILE, dwindex : u32, ptag : *mut TAGTYPE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("mscms.dll" "system" fn GetColorProfileFromHandle(hprofile : HPROFILE, pprofile : *mut u8, pcbprofile : *mut u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "wingdi", feature = "winnt"))]
windows_link::link!("mscms.dll" "system" fn GetColorProfileHeader(hprofile : HPROFILE, pheader : *mut PROFILEHEADER) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("mscms.dll" "system" fn GetCountColorProfileElements(hprofile : HPROFILE, pnelementcount : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("mscms.dll" "system" fn GetNamedProfileInfo(hprofile : HPROFILE, pnamedprofileinfo : *mut NAMED_PROFILE_INFO) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("mscms.dll" "system" fn GetPS2ColorRenderingDictionary(hprofile : HPROFILE, dwintent : u32, pps2colorrenderingdictionary : *mut u8, pcbps2colorrenderingdictionary : *mut u32, pbbinary : *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("mscms.dll" "system" fn GetPS2ColorRenderingIntent(hprofile : HPROFILE, dwintent : u32, pbuffer : *mut u8, pcbps2colorrenderingintent : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("mscms.dll" "system" fn GetPS2ColorSpaceArray(hprofile : HPROFILE, dwintent : u32, dwcsatype : u32, pps2colorspacearray : *mut u8, pcbps2colorspacearray : *mut u32, pbbinary : *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL);
windows_link::link!("mscms.dll" "system" fn GetStandardColorSpaceProfileA(pmachinename : windows_sys::core::PCSTR, dwscs : u32, pbuffer : windows_sys::core::PSTR, pcbsize : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("mscms.dll" "system" fn GetStandardColorSpaceProfileW(pmachinename : windows_sys::core::PCWSTR, dwscs : u32, pbuffer : windows_sys::core::PWSTR, pcbsize : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("mscms.dll" "system" fn InstallColorProfileA(pmachinename : windows_sys::core::PCSTR, pprofilename : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("mscms.dll" "system" fn InstallColorProfileW(pmachinename : windows_sys::core::PCWSTR, pprofilename : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("mscms.dll" "system" fn IsColorProfileTagPresent(hprofile : HPROFILE, tag : TAGTYPE, pbpresent : *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("mscms.dll" "system" fn IsColorProfileValid(hprofile : HPROFILE, pbvalid : *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("mscms.dll" "system" fn OpenColorProfileA(pprofile : *const PROFILE, dwdesiredaccess : u32, dwsharemode : u32, dwcreationmode : u32) -> HPROFILE);
#[cfg(feature = "winnt")]
windows_link::link!("mscms.dll" "system" fn OpenColorProfileW(pprofile : *const PROFILE, dwdesiredaccess : u32, dwsharemode : u32, dwcreationmode : u32) -> HPROFILE);
windows_link::link!("mscms.dll" "system" fn RegisterCMMA(pmachinename : windows_sys::core::PCSTR, cmmid : u32, pcmmdll : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("mscms.dll" "system" fn RegisterCMMW(pmachinename : windows_sys::core::PCWSTR, cmmid : u32, pcmmdll : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("mscms.dll" "system" fn SelectCMM(dwcmmtype : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("mscms.dll" "system" fn SetColorProfileElement(hprofile : HPROFILE, tag : TAGTYPE, dwoffset : u32, pcbelement : *const u32, pelement : *const core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("mscms.dll" "system" fn SetColorProfileElementReference(hprofile : HPROFILE, newtag : TAGTYPE, reftag : TAGTYPE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("mscms.dll" "system" fn SetColorProfileElementSize(hprofile : HPROFILE, tagtype : TAGTYPE, pcbelement : u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "wingdi", feature = "winnt"))]
windows_link::link!("mscms.dll" "system" fn SetColorProfileHeader(hprofile : HPROFILE, pheader : *const PROFILEHEADER) -> windows_sys::core::BOOL);
windows_link::link!("mscms.dll" "system" fn SetStandardColorSpaceProfileA(pmachinename : windows_sys::core::PCSTR, dwprofileid : u32, pprofilename : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("mscms.dll" "system" fn SetStandardColorSpaceProfileW(pmachinename : windows_sys::core::PCWSTR, dwprofileid : u32, pprofilename : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
windows_link::link!("icmui.dll" "system" fn SetupColorMatchingA(pcms : *mut COLORMATCHSETUPA) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
windows_link::link!("icmui.dll" "system" fn SetupColorMatchingW(pcms : *mut COLORMATCHSETUPW) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("mscms.dll" "system" fn TranslateBitmapBits(hcolortransform : HTRANSFORM, psrcbits : *const core::ffi::c_void, bminput : BMFORMAT, dwwidth : u32, dwheight : u32, dwinputstride : u32, pdestbits : *mut core::ffi::c_void, bmoutput : BMFORMAT, dwoutputstride : u32, pfncallback : PBMCALLBACKFN, ulcallbackdata : super::LPARAM) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("mscms.dll" "system" fn TranslateColors(hcolortransform : HTRANSFORM, painputcolors : *const COLOR, ncolors : u32, ctinput : COLORTYPE, paoutputcolors : *mut COLOR, ctoutput : COLORTYPE) -> windows_sys::core::BOOL);
windows_link::link!("mscms.dll" "system" fn UninstallColorProfileA(pmachinename : windows_sys::core::PCSTR, pprofilename : windows_sys::core::PCSTR, bdelete : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
windows_link::link!("mscms.dll" "system" fn UninstallColorProfileW(pmachinename : windows_sys::core::PCWSTR, pprofilename : windows_sys::core::PCWSTR, bdelete : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
windows_link::link!("mscms.dll" "system" fn UnregisterCMMA(pmachinename : windows_sys::core::PCSTR, cmmid : u32) -> windows_sys::core::BOOL);
windows_link::link!("mscms.dll" "system" fn UnregisterCMMW(pmachinename : windows_sys::core::PCWSTR, cmmid : u32) -> windows_sys::core::BOOL);
windows_link::link!("mscms.dll" "system" fn WcsAssociateColorProfileWithDevice(scope : WCS_PROFILE_MANAGEMENT_SCOPE, pprofilename : windows_sys::core::PCWSTR, pdevicename : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("mscms.dll" "system" fn WcsCheckColors(hcolortransform : HTRANSFORM, ncolors : u32, ninputchannels : u32, cdtinput : COLORDATATYPE, cbinput : u32, pinputdata : *const core::ffi::c_void, paresult : *mut u8) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("mscms.dll" "system" fn WcsCreateIccProfile(hwcsprofile : HPROFILE, dwoptions : u32) -> HPROFILE);
windows_link::link!("mscms.dll" "system" fn WcsDisassociateColorProfileFromDevice(scope : WCS_PROFILE_MANAGEMENT_SCOPE, pprofilename : windows_sys::core::PCWSTR, pdevicename : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("mscms.dll" "system" fn WcsEnumColorProfiles(scope : WCS_PROFILE_MANAGEMENT_SCOPE, penumrecord : *const ENUMTYPEW, pbuffer : *mut u8, dwsize : u32, pnprofiles : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("mscms.dll" "system" fn WcsEnumColorProfilesSize(scope : WCS_PROFILE_MANAGEMENT_SCOPE, penumrecord : *const ENUMTYPEW, pdwsize : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("mscms.dll" "system" fn WcsGetCalibrationManagementState(pbisenabled : *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL);
windows_link::link!("mscms.dll" "system" fn WcsGetDefaultColorProfile(scope : WCS_PROFILE_MANAGEMENT_SCOPE, pdevicename : windows_sys::core::PCWSTR, cptcolorprofiletype : COLORPROFILETYPE, cpstcolorprofilesubtype : COLORPROFILESUBTYPE, dwprofileid : u32, cbprofilename : u32, pprofilename : windows_sys::core::PWSTR) -> windows_sys::core::BOOL);
windows_link::link!("mscms.dll" "system" fn WcsGetDefaultColorProfileSize(scope : WCS_PROFILE_MANAGEMENT_SCOPE, pdevicename : windows_sys::core::PCWSTR, cptcolorprofiletype : COLORPROFILETYPE, cpstcolorprofilesubtype : COLORPROFILESUBTYPE, dwprofileid : u32, pcbprofilename : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("mscms.dll" "system" fn WcsGetDefaultRenderingIntent(scope : WCS_PROFILE_MANAGEMENT_SCOPE, pdwrenderingintent : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("mscms.dll" "system" fn WcsGetUsePerUserProfiles(pdevicename : windows_sys::core::PCWSTR, dwdeviceclass : u32, puseperuserprofiles : *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("mscms.dll" "system" fn WcsOpenColorProfileA(pcdmpprofile : *const PROFILE, pcampprofile : *const PROFILE, pgmmpprofile : *const PROFILE, dwdesireaccess : u32, dwsharemode : u32, dwcreationmode : u32, dwflags : u32) -> HPROFILE);
#[cfg(feature = "winnt")]
windows_link::link!("mscms.dll" "system" fn WcsOpenColorProfileW(pcdmpprofile : *const PROFILE, pcampprofile : *const PROFILE, pgmmpprofile : *const PROFILE, dwdesireaccess : u32, dwsharemode : u32, dwcreationmode : u32, dwflags : u32) -> HPROFILE);
windows_link::link!("mscms.dll" "system" fn WcsSetCalibrationManagementState(bisenabled : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
windows_link::link!("mscms.dll" "system" fn WcsSetDefaultColorProfile(scope : WCS_PROFILE_MANAGEMENT_SCOPE, pdevicename : windows_sys::core::PCWSTR, cptcolorprofiletype : COLORPROFILETYPE, cpstcolorprofilesubtype : COLORPROFILESUBTYPE, dwprofileid : u32, pprofilename : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("mscms.dll" "system" fn WcsSetDefaultRenderingIntent(scope : WCS_PROFILE_MANAGEMENT_SCOPE, dwrenderingintent : u32) -> windows_sys::core::BOOL);
windows_link::link!("mscms.dll" "system" fn WcsSetUsePerUserProfiles(pdevicename : windows_sys::core::PCWSTR, dwdeviceclass : u32, useperuserprofiles : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("mscms.dll" "system" fn WcsTranslateColors(hcolortransform : HTRANSFORM, ncolors : u32, ninputchannels : u32, cdtinput : COLORDATATYPE, cbinput : u32, pinputdata : *const core::ffi::c_void, noutputchannels : u32, cdtoutput : COLORDATATYPE, cboutput : u32, poutputdata : *mut core::ffi::c_void) -> windows_sys::core::BOOL);
pub const ATTRIB_MATTE: u32 = 2;
pub const ATTRIB_TRANSPARENCY: u32 = 1;
pub const BEST_MODE: u32 = 3;
pub type BMFORMAT = i32;
pub const BM_10b_G3CH: BMFORMAT = 1028;
pub const BM_10b_Lab: BMFORMAT = 1027;
pub const BM_10b_RGB: BMFORMAT = 9;
pub const BM_10b_XYZ: BMFORMAT = 1025;
pub const BM_10b_Yxy: BMFORMAT = 1026;
pub const BM_16b_G3CH: BMFORMAT = 1284;
pub const BM_16b_GRAY: BMFORMAT = 1285;
pub const BM_16b_Lab: BMFORMAT = 1283;
pub const BM_16b_RGB: BMFORMAT = 10;
pub const BM_16b_XYZ: BMFORMAT = 1281;
pub const BM_16b_Yxy: BMFORMAT = 1282;
pub const BM_32b_scARGB: BMFORMAT = 1538;
pub const BM_32b_scRGB: BMFORMAT = 1537;
pub const BM_565RGB: BMFORMAT = 1;
pub const BM_5CHANNEL: BMFORMAT = 517;
pub const BM_6CHANNEL: BMFORMAT = 518;
pub const BM_7CHANNEL: BMFORMAT = 519;
pub const BM_8CHANNEL: BMFORMAT = 520;
pub const BM_BGRTRIPLETS: BMFORMAT = 4;
pub const BM_CMYKQUADS: BMFORMAT = 32;
pub const BM_G3CHTRIPLETS: BMFORMAT = 516;
pub const BM_GRAY: BMFORMAT = 521;
pub const BM_KYMCQUADS: BMFORMAT = 773;
pub const BM_LabTRIPLETS: BMFORMAT = 515;
pub const BM_NAMED_INDEX: BMFORMAT = 1029;
pub const BM_R10G10B10A2: BMFORMAT = 1793;
pub const BM_R10G10B10A2_XR: BMFORMAT = 1794;
pub const BM_R16G16B16A16_FLOAT: BMFORMAT = 1795;
pub const BM_RGBTRIPLETS: BMFORMAT = 2;
pub const BM_S2DOT13FIXED_scARGB: BMFORMAT = 1540;
pub const BM_S2DOT13FIXED_scRGB: BMFORMAT = 1539;
pub const BM_XYZTRIPLETS: BMFORMAT = 513;
pub const BM_YxyTRIPLETS: BMFORMAT = 514;
pub const BM_x555G3CH: BMFORMAT = 260;
pub const BM_x555Lab: BMFORMAT = 259;
pub const BM_x555RGB: BMFORMAT = 0;
pub const BM_x555XYZ: BMFORMAT = 257;
pub const BM_x555Yxy: BMFORMAT = 258;
pub const BM_xBGRQUADS: BMFORMAT = 16;
pub const BM_xG3CHQUADS: BMFORMAT = 772;
pub const BM_xRGBQUADS: BMFORMAT = 8;
pub const CLASS_ABSTRACT: u32 = 1633842036;
pub const CLASS_CAMP: u32 = 1667329392;
pub const CLASS_COLORSPACE: u32 = 1936744803;
pub const CLASS_GMMP: u32 = 1735224688;
pub const CLASS_LINK: u32 = 1818848875;
pub const CLASS_MONITOR: u32 = 1835955314;
pub const CLASS_NAMED: u32 = 1852662636;
pub const CLASS_PRINTER: u32 = 1886549106;
pub const CLASS_SCANNER: u32 = 1935896178;
pub const CMM_DESCRIPTION: u32 = 5;
pub const CMM_DLL_VERSION: u32 = 3;
pub const CMM_DRIVER_VERSION: u32 = 2;
pub const CMM_FROM_PROFILE: u32 = 0;
pub const CMM_IDENT: u32 = 1;
pub const CMM_LOGOICON: u32 = 6;
pub const CMM_VERSION: u32 = 4;
pub const CMM_WINDOWS_DEFAULT: u32 = 1466527264;
pub const CMM_WIN_VERSION: u32 = 0;
pub const CMS_BACKWARD: u32 = 1;
pub const CMS_DISABLEICM: u32 = 1;
pub const CMS_DISABLEINTENT: u32 = 1024;
pub const CMS_DISABLERENDERINTENT: u32 = 2048;
pub const CMS_ENABLEPROOFING: u32 = 2;
pub const CMS_FORWARD: u32 = 0;
pub const CMS_MONITOROVERFLOW: u32 = 2147483648;
pub const CMS_PRINTEROVERFLOW: u32 = 1073741824;
pub const CMS_SETMONITORPROFILE: u32 = 16;
pub const CMS_SETPRINTERPROFILE: u32 = 32;
pub const CMS_SETPROOFINTENT: u32 = 8;
pub const CMS_SETRENDERINTENT: u32 = 4;
pub const CMS_SETTARGETPROFILE: u32 = 64;
pub const CMS_TARGETOVERFLOW: u32 = 536870912;
pub const CMS_USEAPPLYCALLBACK: u32 = 256;
pub const CMS_USEDESCRIPTION: u32 = 512;
pub const CMS_USEHOOK: u32 = 128;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CMYKCOLOR {
    pub cyan: u16,
    pub magenta: u16,
    pub yellow: u16,
    pub black: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union COLOR {
    pub gray: GRAYCOLOR,
    pub rgb: RGBCOLOR,
    pub cmyk: CMYKCOLOR,
    pub XYZ: XYZCOLOR,
    pub Yxy: YxyCOLOR,
    pub Lab: LabCOLOR,
    pub gen3ch: GENERIC3CHANNEL,
    pub named: NAMEDCOLOR,
    pub hifi: HiFiCOLOR,
    pub Anonymous: COLOR_0,
}
impl Default for COLOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct COLOR_0 {
    pub reserved1: u32,
    pub reserved2: *mut core::ffi::c_void,
}
impl Default for COLOR_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type COLORDATATYPE = i32;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy)]
pub struct COLORMATCHSETUPA {
    pub dwSize: u32,
    pub dwVersion: u32,
    pub dwFlags: u32,
    pub hwndOwner: super::HWND,
    pub pSourceName: windows_sys::core::PCSTR,
    pub pDisplayName: windows_sys::core::PCSTR,
    pub pPrinterName: windows_sys::core::PCSTR,
    pub dwRenderIntent: u32,
    pub dwProofingIntent: u32,
    pub pMonitorProfile: windows_sys::core::PSTR,
    pub ccMonitorProfile: u32,
    pub pPrinterProfile: windows_sys::core::PSTR,
    pub ccPrinterProfile: u32,
    pub pTargetProfile: windows_sys::core::PSTR,
    pub ccTargetProfile: u32,
    pub lpfnHook: super::DLGPROC,
    pub lParam: super::LPARAM,
    pub lpfnApplyCallback: PCMSCALLBACKA,
    pub lParamApplyCallback: super::LPARAM,
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl Default for COLORMATCHSETUPA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy)]
pub struct COLORMATCHSETUPW {
    pub dwSize: u32,
    pub dwVersion: u32,
    pub dwFlags: u32,
    pub hwndOwner: super::HWND,
    pub pSourceName: windows_sys::core::PCWSTR,
    pub pDisplayName: windows_sys::core::PCWSTR,
    pub pPrinterName: windows_sys::core::PCWSTR,
    pub dwRenderIntent: u32,
    pub dwProofingIntent: u32,
    pub pMonitorProfile: windows_sys::core::PWSTR,
    pub ccMonitorProfile: u32,
    pub pPrinterProfile: windows_sys::core::PWSTR,
    pub ccPrinterProfile: u32,
    pub pTargetProfile: windows_sys::core::PWSTR,
    pub ccTargetProfile: u32,
    pub lpfnHook: super::DLGPROC,
    pub lParam: super::LPARAM,
    pub lpfnApplyCallback: PCMSCALLBACKW,
    pub lParamApplyCallback: super::LPARAM,
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl Default for COLORMATCHSETUPW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type COLORPROFILESUBTYPE = i32;
pub type COLORPROFILETYPE = i32;
pub type COLORTYPE = i32;
pub const COLOR_10b_R10G10B10A2: COLORDATATYPE = 5;
pub const COLOR_10b_R10G10B10A2_XR: COLORDATATYPE = 6;
pub const COLOR_3_CHANNEL: COLORTYPE = 6;
pub const COLOR_5_CHANNEL: COLORTYPE = 8;
pub const COLOR_6_CHANNEL: COLORTYPE = 9;
pub const COLOR_7_CHANNEL: COLORTYPE = 10;
pub const COLOR_8_CHANNEL: COLORTYPE = 11;
pub const COLOR_BYTE: COLORDATATYPE = 1;
pub const COLOR_CMYK: COLORTYPE = 7;
pub const COLOR_FLOAT: COLORDATATYPE = 3;
pub const COLOR_FLOAT16: COLORDATATYPE = 7;
pub const COLOR_GRAY: COLORTYPE = 1;
pub const COLOR_Lab: COLORTYPE = 5;
pub const COLOR_MATCH_VERSION: u32 = 512;
pub type COLOR_NAME = [i8; 32];
pub const COLOR_NAMED: COLORTYPE = 12;
pub const COLOR_RGB: COLORTYPE = 2;
pub const COLOR_S2DOT13FIXED: COLORDATATYPE = 4;
pub const COLOR_WORD: COLORDATATYPE = 2;
pub const COLOR_XYZ: COLORTYPE = 3;
pub const COLOR_Yxy: COLORTYPE = 4;
pub const CPST_ABSOLUTE_COLORIMETRIC: COLORPROFILESUBTYPE = 3;
pub const CPST_CUSTOM_WORKING_SPACE: COLORPROFILESUBTYPE = 6;
pub const CPST_EXTENDED_DISPLAY_COLOR_MODE: COLORPROFILESUBTYPE = 8;
pub const CPST_NONE: COLORPROFILESUBTYPE = 4;
pub const CPST_PERCEPTUAL: COLORPROFILESUBTYPE = 0;
pub const CPST_RELATIVE_COLORIMETRIC: COLORPROFILESUBTYPE = 1;
pub const CPST_RGB_WORKING_SPACE: COLORPROFILESUBTYPE = 5;
pub const CPST_SATURATION: COLORPROFILESUBTYPE = 2;
pub const CPST_STANDARD_DISPLAY_COLOR_MODE: COLORPROFILESUBTYPE = 7;
pub const CPT_CAMP: COLORPROFILETYPE = 2;
pub const CPT_DMP: COLORPROFILETYPE = 1;
pub const CPT_GMMP: COLORPROFILETYPE = 3;
pub const CPT_ICC: COLORPROFILETYPE = 0;
pub const CSA_A: u32 = 1;
pub const CSA_ABC: u32 = 2;
pub const CSA_CMYK: u32 = 7;
pub const CSA_DEF: u32 = 3;
pub const CSA_DEFG: u32 = 4;
pub const CSA_GRAY: u32 = 5;
pub const CSA_Lab: u32 = 8;
pub const CSA_RGB: u32 = 6;
pub const DONT_USE_EMBEDDED_WCS_PROFILES: u32 = 1;
pub const ENABLE_GAMUT_CHECKING: u32 = 65536;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ENUMTYPEA {
    pub dwSize: u32,
    pub dwVersion: u32,
    pub dwFields: u32,
    pub pDeviceName: windows_sys::core::PCSTR,
    pub dwMediaType: u32,
    pub dwDitheringMode: u32,
    pub dwResolution: [u32; 2],
    pub dwCMMType: u32,
    pub dwClass: u32,
    pub dwDataColorSpace: u32,
    pub dwConnectionSpace: u32,
    pub dwSignature: u32,
    pub dwPlatform: u32,
    pub dwProfileFlags: u32,
    pub dwManufacturer: u32,
    pub dwModel: u32,
    pub dwAttributes: [u32; 2],
    pub dwRenderingIntent: u32,
    pub dwCreator: u32,
    pub dwDeviceClass: u32,
}
impl Default for ENUMTYPEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ENUMTYPEW {
    pub dwSize: u32,
    pub dwVersion: u32,
    pub dwFields: u32,
    pub pDeviceName: windows_sys::core::PCWSTR,
    pub dwMediaType: u32,
    pub dwDitheringMode: u32,
    pub dwResolution: [u32; 2],
    pub dwCMMType: u32,
    pub dwClass: u32,
    pub dwDataColorSpace: u32,
    pub dwConnectionSpace: u32,
    pub dwSignature: u32,
    pub dwPlatform: u32,
    pub dwProfileFlags: u32,
    pub dwManufacturer: u32,
    pub dwModel: u32,
    pub dwAttributes: [u32; 2],
    pub dwRenderingIntent: u32,
    pub dwCreator: u32,
    pub dwDeviceClass: u32,
}
impl Default for ENUMTYPEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ENUM_TYPE_VERSION: u32 = 768;
pub const ET_ATTRIBUTES: u32 = 8192;
pub const ET_CLASS: u32 = 32;
pub const ET_CMMTYPE: u32 = 16;
pub const ET_CONNECTIONSPACE: u32 = 128;
pub const ET_CREATOR: u32 = 32768;
pub const ET_DATACOLORSPACE: u32 = 64;
pub const ET_DEVICECLASS: u32 = 65536;
pub const ET_DEVICENAME: u32 = 1;
pub const ET_DITHERMODE: u32 = 4;
pub const ET_EXTENDEDDISPLAYCOLOR: u32 = 262144;
pub const ET_MANUFACTURER: u32 = 2048;
pub const ET_MEDIATYPE: u32 = 2;
pub const ET_MODEL: u32 = 4096;
pub const ET_PLATFORM: u32 = 512;
pub const ET_PROFILEFLAGS: u32 = 1024;
pub const ET_RENDERINGINTENT: u32 = 16384;
pub const ET_RESOLUTION: u32 = 8;
pub const ET_SIGNATURE: u32 = 256;
pub const ET_STANDARDDISPLAYCOLOR: u32 = 131072;
pub const FAST_TRANSLATE: u32 = 262144;
pub const FLAG_DEPENDENTONDATA: u32 = 2;
pub const FLAG_EMBEDDEDPROFILE: u32 = 1;
pub const FLAG_ENABLE_CHROMATIC_ADAPTATION: u32 = 33554432;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct GENERIC3CHANNEL {
    pub ch1: u16,
    pub ch2: u16,
    pub ch3: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct GRAYCOLOR {
    pub gray: u16,
}
#[cfg(feature = "winnt")]
pub type HCMTRANSFORM = super::HANDLE;
#[cfg(feature = "winnt")]
pub type HPROFILE = super::HANDLE;
#[cfg(feature = "winnt")]
pub type HTRANSFORM = super::HANDLE;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HiFiCOLOR {
    pub channel: [u8; 8],
}
impl Default for HiFiCOLOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const INDEX_DONT_CARE: u32 = 0;
pub const INTENT_ABSOLUTE_COLORIMETRIC: u32 = 3;
pub const INTENT_PERCEPTUAL: u32 = 0;
pub const INTENT_RELATIVE_COLORIMETRIC: u32 = 1;
pub const INTENT_SATURATION: u32 = 2;
#[cfg(feature = "minwindef")]
pub type LPBMCALLBACKFN = PBMCALLBACKFN;
pub type LPBMFORMAT = *mut BMFORMAT;
pub type LPCOLOR = *mut COLOR;
pub type LPCOLORDATATYPE = *mut COLORDATATYPE;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
pub type LPCOLORMATCHSETUPA = *mut COLORMATCHSETUPA;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
pub type LPCOLORMATCHSETUPW = *mut COLORMATCHSETUPW;
pub type LPCOLORPROFILESUBTYPE = *mut COLORPROFILESUBTYPE;
pub type LPCOLORPROFILETYPE = *mut COLORPROFILETYPE;
pub type LPCOLORTYPE = *mut COLORTYPE;
pub type LPCOLOR_NAME = *mut COLOR_NAME;
pub type LPDEVCHARACTER = *mut core::ffi::c_void;
pub type LPENUMTYPEA = *mut ENUMTYPEA;
pub type LPENUMTYPEW = *mut ENUMTYPEW;
pub type LPNAMED_PROFILE_INFO = *mut NAMED_PROFILE_INFO;
pub type LPPROFILE = *mut PROFILE;
#[cfg(feature = "wingdi")]
pub type LPPROFILEHEADER = *mut PROFILEHEADER;
pub type LPTAGTYPE = *mut TAGTYPE;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct LabCOLOR {
    pub L: u16,
    pub a: u16,
    pub b: u16,
}
pub const MAX_COLOR_CHANNELS: u32 = 8;
pub const MicrosoftHardwareColorV2: WCS_DEVICE_CAPABILITIES_TYPE = 2;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NAMEDCOLOR {
    pub dwIndex: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NAMED_PROFILE_INFO {
    pub dwFlags: u32,
    pub dwCount: u32,
    pub dwCountDevCoordinates: u32,
    pub szPrefix: COLOR_NAME,
    pub szSuffix: COLOR_NAME,
}
impl Default for NAMED_PROFILE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NORMAL_MODE: u32 = 2;
#[cfg(feature = "minwindef")]
pub type PBMCALLBACKFN = Option<unsafe extern "system" fn(param0: u32, param1: u32, param2: super::LPARAM) -> windows_sys::core::BOOL>;
pub type PBMFORMAT = *mut BMFORMAT;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
pub type PCMSCALLBACKA = Option<unsafe extern "system" fn(param0: *mut COLORMATCHSETUPA, param1: super::LPARAM) -> windows_sys::core::BOOL>;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
pub type PCMSCALLBACKW = Option<unsafe extern "system" fn(param0: *mut COLORMATCHSETUPW, param1: super::LPARAM) -> windows_sys::core::BOOL>;
pub type PCOLOR = *mut COLOR;
pub type PCOLORDATATYPE = *mut COLORDATATYPE;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
pub type PCOLORMATCHSETUPA = *mut COLORMATCHSETUPA;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
pub type PCOLORMATCHSETUPW = *mut COLORMATCHSETUPW;
pub type PCOLORPROFILESUBTYPE = *mut COLORPROFILESUBTYPE;
pub type PCOLORPROFILETYPE = *mut COLORPROFILETYPE;
pub type PCOLORTYPE = *mut COLORTYPE;
pub type PCOLOR_NAME = *mut COLOR_NAME;
pub type PENUMTYPEA = *mut ENUMTYPEA;
pub type PENUMTYPEW = *mut ENUMTYPEW;
#[cfg(feature = "winnt")]
pub type PHPROFILE = *mut HPROFILE;
pub type PNAMED_PROFILE_INFO = *mut NAMED_PROFILE_INFO;
pub type PPROFILE = *mut PROFILE;
#[cfg(feature = "wingdi")]
pub type PPROFILEHEADER = *mut PROFILEHEADER;
pub const PRESERVEBLACK: u32 = 1048576;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROFILE {
    pub dwType: u32,
    pub pProfileData: *mut core::ffi::c_void,
    pub cbDataSize: u32,
}
impl Default for PROFILE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "wingdi")]
#[derive(Clone, Copy)]
pub struct PROFILEHEADER {
    pub phSize: u32,
    pub phCMMType: u32,
    pub phVersion: u32,
    pub phClass: u32,
    pub phDataColorSpace: u32,
    pub phConnectionSpace: u32,
    pub phDateTime: [u32; 3],
    pub phSignature: u32,
    pub phPlatform: u32,
    pub phProfileFlags: u32,
    pub phManufacturer: u32,
    pub phModel: u32,
    pub phAttributes: [u32; 2],
    pub phRenderingIntent: u32,
    pub phIlluminant: super::CIEXYZ,
    pub phCreator: u32,
    pub phReserved: [u8; 44],
}
#[cfg(feature = "wingdi")]
impl Default for PROFILEHEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PROFILE_FILENAME: u32 = 1;
pub const PROFILE_MEMBUFFER: u32 = 2;
pub const PROFILE_READ: u32 = 1;
pub const PROFILE_READWRITE: u32 = 2;
pub const PROOF_MODE: u32 = 1;
pub type PTAGTYPE = *mut TAGTYPE;
pub const RESERVED: u32 = 2147483648;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct RGBCOLOR {
    pub red: u16,
    pub green: u16,
    pub blue: u16,
}
pub const SEQUENTIAL_TRANSFORM: u32 = 2155872256;
pub const SPACE_2_CHANNEL: u32 = 843271250;
pub const SPACE_3_CHANNEL: u32 = 860048466;
pub const SPACE_4_CHANNEL: u32 = 876825682;
pub const SPACE_5_CHANNEL: u32 = 893602898;
pub const SPACE_6_CHANNEL: u32 = 910380114;
pub const SPACE_7_CHANNEL: u32 = 927157330;
pub const SPACE_8_CHANNEL: u32 = 943934546;
pub const SPACE_CMY: u32 = 1129142560;
pub const SPACE_CMYK: u32 = 1129142603;
pub const SPACE_GRAY: u32 = 1196573017;
pub const SPACE_HLS: u32 = 1212961568;
pub const SPACE_HSV: u32 = 1213421088;
pub const SPACE_Lab: u32 = 1281450528;
pub const SPACE_Luv: u32 = 1282766368;
pub const SPACE_RGB: u32 = 1380401696;
pub const SPACE_XYZ: u32 = 1482250784;
pub const SPACE_YCbCr: u32 = 1497588338;
pub const SPACE_Yxy: u32 = 1501067552;
pub type TAGTYPE = u32;
pub const USE_RELATIVE_COLORIMETRIC: u32 = 131072;
pub const VideoCardGammaTable: WCS_DEVICE_CAPABILITIES_TYPE = 1;
pub const WCS_ALWAYS: u32 = 2097152;
pub const WCS_DEFAULT: u32 = 0;
pub type WCS_DEVICE_CAPABILITIES_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WCS_DEVICE_MHC2_CAPABILITIES {
    pub Size: u32,
    pub SupportsMhc2: windows_sys::core::BOOL,
    pub RegammaLutEntryCount: u32,
    pub CscXyzMatrixRows: u32,
    pub CscXyzMatrixColumns: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WCS_DEVICE_VCGT_CAPABILITIES {
    pub Size: u32,
    pub SupportsVcgt: windows_sys::core::BOOL,
}
pub const WCS_EMBEDDED_TAG_SIGNATURE: u32 = 1297297456;
pub const WCS_EMBEDDED_TAG_TYPE_SIGNATURE: u32 = 1297297712;
pub const WCS_ICCONLY: u32 = 65536;
pub type WCS_PROFILE_MANAGEMENT_SCOPE = i32;
pub const WCS_PROFILE_MANAGEMENT_SCOPE_CURRENT_USER: WCS_PROFILE_MANAGEMENT_SCOPE = 1;
pub const WCS_PROFILE_MANAGEMENT_SCOPE_SYSTEM_WIDE: WCS_PROFILE_MANAGEMENT_SCOPE = 0;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct XYZCOLOR {
    pub X: u16,
    pub Y: u16,
    pub Z: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct YxyCOLOR {
    pub Y: u16,
    pub x: u16,
    pub y: u16,
}
