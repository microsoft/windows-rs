windows_link::link!("mscms.dll" "system" fn AssociateColorProfileWithDeviceA(pmachinename : windows_sys::core::PCSTR, pprofilename : windows_sys::core::PCSTR, pdevicename : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("mscms.dll" "system" fn AssociateColorProfileWithDeviceW(pmachinename : windows_sys::core::PCWSTR, pprofilename : windows_sys::core::PCWSTR, pdevicename : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("icm32.dll" "system" fn CMCheckColors(hcmtransform : HCMTRANSFORM, lpainputcolors : LPCOLOR, ncolors : super::DWORD, ctinput : COLORTYPE, lparesult : super::LPBYTE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "wingdi"))]
windows_link::link!("icm32.dll" "system" fn CMCheckColorsInGamut(hcmtransform : HCMTRANSFORM, lpargbtriple : *const super::RGBTRIPLE, lparesult : super::LPBYTE, ncount : super::UINT) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("icm32.dll" "system" fn CMCheckRGBs(hcmtransform : HCMTRANSFORM, lpsrcbits : super::LPVOID, bminput : BMFORMAT, dwwidth : super::DWORD, dwheight : super::DWORD, dwstride : super::DWORD, lparesult : super::LPBYTE, pfncallback : PBMCALLBACKFN, ulcallbackdata : super::LPARAM) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("icm32.dll" "system" fn CMConvertColorNameToIndex(hprofile : HPROFILE, pacolorname : PCOLOR_NAME, paindex : super::PDWORD, dwcount : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("icm32.dll" "system" fn CMConvertIndexToColorName(hprofile : HPROFILE, paindex : super::PDWORD, pacolorname : PCOLOR_NAME, dwcount : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("icm32.dll" "system" fn CMCreateDeviceLinkProfile(pahprofiles : PHPROFILE, nprofiles : super::DWORD, padwintents : super::PDWORD, nintents : super::DWORD, dwflags : super::DWORD, lpprofiledata : *mut super::LPBYTE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("icm32.dll" "system" fn CMCreateMultiProfileTransform(pahprofiles : PHPROFILE, nprofiles : super::DWORD, padwintents : super::PDWORD, nintents : super::DWORD, dwflags : super::DWORD) -> HCMTRANSFORM);
#[cfg(all(feature = "shared", feature = "wingdi"))]
windows_link::link!("icm32.dll" "system" fn CMCreateProfile(lpcolorspace : super::LPLOGCOLORSPACEA, lpprofiledata : *mut LPDEVCHARACTER) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "wingdi", feature = "winnt"))]
windows_link::link!("icm32.dll" "system" fn CMCreateProfileW(lpcolorspace : super::LPLOGCOLORSPACEW, lpprofiledata : *mut LPDEVCHARACTER) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "wingdi"))]
windows_link::link!("icm32.dll" "system" fn CMCreateTransform(lpcolorspace : super::LPLOGCOLORSPACEA, lpdevcharacter : LPDEVCHARACTER, lptargetdevcharacter : LPDEVCHARACTER) -> HCMTRANSFORM);
#[cfg(all(feature = "shared", feature = "wingdi"))]
windows_link::link!("icm32.dll" "system" fn CMCreateTransformExt(lpcolorspace : super::LPLOGCOLORSPACEA, lpdevcharacter : LPDEVCHARACTER, lptargetdevcharacter : LPDEVCHARACTER, dwflags : super::DWORD) -> HCMTRANSFORM);
#[cfg(all(feature = "shared", feature = "wingdi", feature = "winnt"))]
windows_link::link!("icm32.dll" "system" fn CMCreateTransformExtW(lpcolorspace : super::LPLOGCOLORSPACEW, lpdevcharacter : LPDEVCHARACTER, lptargetdevcharacter : LPDEVCHARACTER, dwflags : super::DWORD) -> HCMTRANSFORM);
#[cfg(all(feature = "shared", feature = "wingdi", feature = "winnt"))]
windows_link::link!("icm32.dll" "system" fn CMCreateTransformW(lpcolorspace : super::LPLOGCOLORSPACEW, lpdevcharacter : LPDEVCHARACTER, lptargetdevcharacter : LPDEVCHARACTER) -> HCMTRANSFORM);
windows_link::link!("icm32.dll" "system" fn CMDeleteTransform(hcmtransform : HCMTRANSFORM) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("icm32.dll" "system" fn CMGetInfo(dwinfo : super::DWORD) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("icm32.dll" "system" fn CMGetNamedProfileInfo(hprofile : HPROFILE, pnamedprofileinfo : PNAMED_PROFILE_INFO) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("icm32.dll" "system" fn CMIsProfileValid(hprofile : HPROFILE, lpbvalid : super::LPBOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("icm32.dll" "system" fn CMTranslateColors(hcmtransform : HCMTRANSFORM, lpainputcolors : LPCOLOR, ncolors : super::DWORD, ctinput : COLORTYPE, lpaoutputcolors : LPCOLOR, ctoutput : COLORTYPE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "windef"))]
windows_link::link!("icm32.dll" "system" fn CMTranslateRGB(hcmtransform : HCMTRANSFORM, colorref : super::COLORREF, lpcolorref : super::LPCOLORREF, dwflags : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("icm32.dll" "system" fn CMTranslateRGBs(hcmtransform : HCMTRANSFORM, lpsrcbits : super::LPVOID, bminput : BMFORMAT, dwwidth : super::DWORD, dwheight : super::DWORD, dwstride : super::DWORD, lpdestbits : super::LPVOID, bmoutput : BMFORMAT, dwtranslatedirection : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("icm32.dll" "system" fn CMTranslateRGBsExt(hcmtransform : HCMTRANSFORM, lpsrcbits : super::LPVOID, bminput : BMFORMAT, dwwidth : super::DWORD, dwheight : super::DWORD, dwinputstride : super::DWORD, lpdestbits : super::LPVOID, bmoutput : BMFORMAT, dwoutputstride : super::DWORD, lpfncallback : LPBMCALLBACKFN, ulcallbackdata : super::LPARAM) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("mscms.dll" "system" fn CheckBitmapBits(hcolortransform : HTRANSFORM, psrcbits : super::PVOID, bminput : BMFORMAT, dwwidth : super::DWORD, dwheight : super::DWORD, dwstride : super::DWORD, paresult : super::PBYTE, pfncallback : PBMCALLBACKFN, lpcallbackdata : super::LPARAM) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("mscms.dll" "system" fn CheckColors(hcolortransform : HTRANSFORM, painputcolors : PCOLOR, ncolors : super::DWORD, ctinput : COLORTYPE, paresult : super::PBYTE) -> windows_sys::core::BOOL);
windows_link::link!("mscms.dll" "system" fn CloseColorProfile(hprofile : HPROFILE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("mscms.dll" "system" fn ColorProfileAddDisplayAssociation(scope : WCS_PROFILE_MANAGEMENT_SCOPE, profilename : windows_sys::core::PCWSTR, targetadapterid : super::LUID, sourceid : super::UINT32, setasdefault : windows_sys::core::BOOL, associateasadvancedcolor : windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("mscms.dll" "system" fn ColorProfileGetDeviceCapabilities(scope : WCS_PROFILE_MANAGEMENT_SCOPE, targetadapterid : super::LUID, sourceid : super::UINT32, capstype : WCS_DEVICE_CAPABILITIES_TYPE, outputcapabilities : super::PVOID) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("mscms.dll" "system" fn ColorProfileGetDisplayDefault(scope : WCS_PROFILE_MANAGEMENT_SCOPE, targetadapterid : super::LUID, sourceid : super::UINT32, profiletype : COLORPROFILETYPE, profilesubtype : COLORPROFILESUBTYPE, profilename : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("mscms.dll" "system" fn ColorProfileGetDisplayList(scope : WCS_PROFILE_MANAGEMENT_SCOPE, targetadapterid : super::LUID, sourceid : super::UINT32, profilelist : *mut *mut windows_sys::core::PWSTR, profilecount : super::PDWORD) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("mscms.dll" "system" fn ColorProfileGetDisplayUserScope(targetadapterid : super::LUID, sourceid : super::UINT32, scope : *mut WCS_PROFILE_MANAGEMENT_SCOPE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("mscms.dll" "system" fn ColorProfileRemoveDisplayAssociation(scope : WCS_PROFILE_MANAGEMENT_SCOPE, profilename : windows_sys::core::PCWSTR, targetadapterid : super::LUID, sourceid : super::UINT32, dissociateadvancedcolor : windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("mscms.dll" "system" fn ColorProfileSetDisplayDefaultAssociation(scope : WCS_PROFILE_MANAGEMENT_SCOPE, profilename : windows_sys::core::PCWSTR, profiletype : COLORPROFILETYPE, profilesubtype : COLORPROFILESUBTYPE, targetadapterid : super::LUID, sourceid : super::UINT32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("mscms.dll" "system" fn ConvertColorNameToIndex(hprofile : HPROFILE, pacolorname : PCOLOR_NAME, paindex : super::PDWORD, dwcount : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("mscms.dll" "system" fn ConvertIndexToColorName(hprofile : HPROFILE, paindex : super::PDWORD, pacolorname : PCOLOR_NAME, dwcount : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "wingdi"))]
windows_link::link!("mscms.dll" "system" fn CreateColorTransformA(plogcolorspace : super::LPLOGCOLORSPACEA, hdestprofile : HPROFILE, htargetprofile : HPROFILE, dwflags : super::DWORD) -> HTRANSFORM);
#[cfg(all(feature = "shared", feature = "wingdi", feature = "winnt"))]
windows_link::link!("mscms.dll" "system" fn CreateColorTransformW(plogcolorspace : super::LPLOGCOLORSPACEW, hdestprofile : HPROFILE, htargetprofile : HPROFILE, dwflags : super::DWORD) -> HTRANSFORM);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("mscms.dll" "system" fn CreateDeviceLinkProfile(hprofile : PHPROFILE, nprofiles : super::DWORD, padwintent : super::PDWORD, nintents : super::DWORD, dwflags : super::DWORD, pprofiledata : *mut super::PBYTE, indexpreferredcmm : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("mscms.dll" "system" fn CreateMultiProfileTransform(pahprofiles : PHPROFILE, nprofiles : super::DWORD, padwintent : super::PDWORD, nintents : super::DWORD, dwflags : super::DWORD, indexpreferredcmm : super::DWORD) -> HTRANSFORM);
#[cfg(all(feature = "shared", feature = "wingdi"))]
windows_link::link!("mscms.dll" "system" fn CreateProfileFromLogColorSpaceA(plogcolorspace : super::LPLOGCOLORSPACEA, pprofile : *mut super::PBYTE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "wingdi", feature = "winnt"))]
windows_link::link!("mscms.dll" "system" fn CreateProfileFromLogColorSpaceW(plogcolorspace : super::LPLOGCOLORSPACEW, pprofile : *mut super::PBYTE) -> windows_sys::core::BOOL);
windows_link::link!("mscms.dll" "system" fn DeleteColorTransform(hxform : HTRANSFORM) -> windows_sys::core::BOOL);
windows_link::link!("mscms.dll" "system" fn DisassociateColorProfileFromDeviceA(pmachinename : windows_sys::core::PCSTR, pprofilename : windows_sys::core::PCSTR, pdevicename : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("mscms.dll" "system" fn DisassociateColorProfileFromDeviceW(pmachinename : windows_sys::core::PCWSTR, pprofilename : windows_sys::core::PCWSTR, pdevicename : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("mscms.dll" "system" fn EnumColorProfilesA(pmachinename : windows_sys::core::PCSTR, penumrecord : PENUMTYPEA, penumerationbuffer : super::PBYTE, pdwsizeofenumerationbuffer : super::PDWORD, pnprofiles : super::PDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("mscms.dll" "system" fn EnumColorProfilesW(pmachinename : windows_sys::core::PCWSTR, penumrecord : PENUMTYPEW, penumerationbuffer : super::PBYTE, pdwsizeofenumerationbuffer : super::PDWORD, pnprofiles : super::PDWORD) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("mscms.dll" "system" fn GetCMMInfo(hcolortransform : HTRANSFORM, param1 : super::DWORD) -> super::DWORD);
#[cfg(feature = "minwindef")]
windows_link::link!("mscms.dll" "system" fn GetColorDirectoryA(pmachinename : windows_sys::core::PCSTR, pbuffer : windows_sys::core::PSTR, pdwsize : super::PDWORD) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("mscms.dll" "system" fn GetColorDirectoryW(pmachinename : windows_sys::core::PCWSTR, pbuffer : windows_sys::core::PWSTR, pdwsize : super::PDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("mscms.dll" "system" fn GetColorProfileElement(hprofile : HPROFILE, tag : TAGTYPE, dwoffset : super::DWORD, pcbelement : super::PDWORD, pelement : super::PVOID, pbreference : super::PBOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("mscms.dll" "system" fn GetColorProfileElementTag(hprofile : HPROFILE, dwindex : super::DWORD, ptag : PTAGTYPE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("mscms.dll" "system" fn GetColorProfileFromHandle(hprofile : HPROFILE, pprofile : super::PBYTE, pcbprofile : super::PDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "wingdi"))]
windows_link::link!("mscms.dll" "system" fn GetColorProfileHeader(hprofile : HPROFILE, pheader : PPROFILEHEADER) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("mscms.dll" "system" fn GetCountColorProfileElements(hprofile : HPROFILE, pnelementcount : super::PDWORD) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("mscms.dll" "system" fn GetNamedProfileInfo(hprofile : HPROFILE, pnamedprofileinfo : PNAMED_PROFILE_INFO) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("mscms.dll" "system" fn GetPS2ColorRenderingDictionary(hprofile : HPROFILE, dwintent : super::DWORD, pps2colorrenderingdictionary : super::PBYTE, pcbps2colorrenderingdictionary : super::PDWORD, pbbinary : super::PBOOL) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("mscms.dll" "system" fn GetPS2ColorRenderingIntent(hprofile : HPROFILE, dwintent : super::DWORD, pbuffer : super::PBYTE, pcbps2colorrenderingintent : super::PDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("mscms.dll" "system" fn GetPS2ColorSpaceArray(hprofile : HPROFILE, dwintent : super::DWORD, dwcsatype : super::DWORD, pps2colorspacearray : super::PBYTE, pcbps2colorspacearray : super::PDWORD, pbbinary : super::PBOOL) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("mscms.dll" "system" fn GetStandardColorSpaceProfileA(pmachinename : windows_sys::core::PCSTR, dwscs : super::DWORD, pbuffer : windows_sys::core::PSTR, pcbsize : super::PDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("mscms.dll" "system" fn GetStandardColorSpaceProfileW(pmachinename : windows_sys::core::PCWSTR, dwscs : super::DWORD, pbuffer : windows_sys::core::PWSTR, pcbsize : super::PDWORD) -> windows_sys::core::BOOL);
windows_link::link!("mscms.dll" "system" fn InstallColorProfileA(pmachinename : windows_sys::core::PCSTR, pprofilename : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("mscms.dll" "system" fn InstallColorProfileW(pmachinename : windows_sys::core::PCWSTR, pprofilename : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("mscms.dll" "system" fn IsColorProfileTagPresent(hprofile : HPROFILE, tag : TAGTYPE, pbpresent : super::PBOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("mscms.dll" "system" fn IsColorProfileValid(hprofile : HPROFILE, pbvalid : super::PBOOL) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("mscms.dll" "system" fn OpenColorProfileA(pprofile : PPROFILE, dwdesiredaccess : super::DWORD, dwsharemode : super::DWORD, dwcreationmode : super::DWORD) -> HPROFILE);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("mscms.dll" "system" fn OpenColorProfileW(pprofile : PPROFILE, dwdesiredaccess : super::DWORD, dwsharemode : super::DWORD, dwcreationmode : super::DWORD) -> HPROFILE);
#[cfg(feature = "shared")]
windows_link::link!("mscms.dll" "system" fn RegisterCMMA(pmachinename : windows_sys::core::PCSTR, cmmid : super::DWORD, pcmmdll : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("mscms.dll" "system" fn RegisterCMMW(pmachinename : windows_sys::core::PCWSTR, cmmid : super::DWORD, pcmmdll : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("mscms.dll" "system" fn SelectCMM(dwcmmtype : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("mscms.dll" "system" fn SetColorProfileElement(hprofile : HPROFILE, tag : TAGTYPE, dwoffset : super::DWORD, pcbelement : super::PDWORD, pelement : super::PVOID) -> windows_sys::core::BOOL);
windows_link::link!("mscms.dll" "system" fn SetColorProfileElementReference(hprofile : HPROFILE, newtag : TAGTYPE, reftag : TAGTYPE) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("mscms.dll" "system" fn SetColorProfileElementSize(hprofile : HPROFILE, tagtype : TAGTYPE, pcbelement : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "wingdi"))]
windows_link::link!("mscms.dll" "system" fn SetColorProfileHeader(hprofile : HPROFILE, pheader : PPROFILEHEADER) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("mscms.dll" "system" fn SetStandardColorSpaceProfileA(pmachinename : windows_sys::core::PCSTR, dwprofileid : super::DWORD, pprofilename : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("mscms.dll" "system" fn SetStandardColorSpaceProfileW(pmachinename : windows_sys::core::PCWSTR, dwprofileid : super::DWORD, pprofilename : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef", feature = "winuser"))]
windows_link::link!("icmui.dll" "system" fn SetupColorMatchingA(pcms : PCOLORMATCHSETUPA) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef", feature = "winuser"))]
windows_link::link!("icmui.dll" "system" fn SetupColorMatchingW(pcms : PCOLORMATCHSETUPW) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("mscms.dll" "system" fn TranslateBitmapBits(hcolortransform : HTRANSFORM, psrcbits : super::PVOID, bminput : BMFORMAT, dwwidth : super::DWORD, dwheight : super::DWORD, dwinputstride : super::DWORD, pdestbits : super::PVOID, bmoutput : BMFORMAT, dwoutputstride : super::DWORD, pfncallback : PBMCALLBACKFN, ulcallbackdata : super::LPARAM) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("mscms.dll" "system" fn TranslateColors(hcolortransform : HTRANSFORM, painputcolors : PCOLOR, ncolors : super::DWORD, ctinput : COLORTYPE, paoutputcolors : PCOLOR, ctoutput : COLORTYPE) -> windows_sys::core::BOOL);
windows_link::link!("mscms.dll" "system" fn UninstallColorProfileA(pmachinename : windows_sys::core::PCSTR, pprofilename : windows_sys::core::PCSTR, bdelete : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
windows_link::link!("mscms.dll" "system" fn UninstallColorProfileW(pmachinename : windows_sys::core::PCWSTR, pprofilename : windows_sys::core::PCWSTR, bdelete : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("mscms.dll" "system" fn UnregisterCMMA(pmachinename : windows_sys::core::PCSTR, cmmid : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("mscms.dll" "system" fn UnregisterCMMW(pmachinename : windows_sys::core::PCWSTR, cmmid : super::DWORD) -> windows_sys::core::BOOL);
windows_link::link!("mscms.dll" "system" fn WcsAssociateColorProfileWithDevice(scope : WCS_PROFILE_MANAGEMENT_SCOPE, pprofilename : windows_sys::core::PCWSTR, pdevicename : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("mscms.dll" "system" fn WcsCheckColors(hcolortransform : HTRANSFORM, ncolors : super::DWORD, ninputchannels : super::DWORD, cdtinput : COLORDATATYPE, cbinput : super::DWORD, pinputdata : super::PVOID, paresult : super::PBYTE) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("mscms.dll" "system" fn WcsCreateIccProfile(hwcsprofile : HPROFILE, dwoptions : super::DWORD) -> HPROFILE);
windows_link::link!("mscms.dll" "system" fn WcsDisassociateColorProfileFromDevice(scope : WCS_PROFILE_MANAGEMENT_SCOPE, pprofilename : windows_sys::core::PCWSTR, pdevicename : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("mscms.dll" "system" fn WcsEnumColorProfiles(scope : WCS_PROFILE_MANAGEMENT_SCOPE, penumrecord : PENUMTYPEW, pbuffer : super::PBYTE, dwsize : super::DWORD, pnprofiles : super::PDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("mscms.dll" "system" fn WcsEnumColorProfilesSize(scope : WCS_PROFILE_MANAGEMENT_SCOPE, penumrecord : PENUMTYPEW, pdwsize : super::PDWORD) -> windows_sys::core::BOOL);
windows_link::link!("mscms.dll" "system" fn WcsGetCalibrationManagementState(pbisenabled : *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("mscms.dll" "system" fn WcsGetDefaultColorProfile(scope : WCS_PROFILE_MANAGEMENT_SCOPE, pdevicename : windows_sys::core::PCWSTR, cptcolorprofiletype : COLORPROFILETYPE, cpstcolorprofilesubtype : COLORPROFILESUBTYPE, dwprofileid : super::DWORD, cbprofilename : super::DWORD, pprofilename : windows_sys::core::PWSTR) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("mscms.dll" "system" fn WcsGetDefaultColorProfileSize(scope : WCS_PROFILE_MANAGEMENT_SCOPE, pdevicename : windows_sys::core::PCWSTR, cptcolorprofiletype : COLORPROFILETYPE, cpstcolorprofilesubtype : COLORPROFILESUBTYPE, dwprofileid : super::DWORD, pcbprofilename : super::PDWORD) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("mscms.dll" "system" fn WcsGetDefaultRenderingIntent(scope : WCS_PROFILE_MANAGEMENT_SCOPE, pdwrenderingintent : super::PDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("mscms.dll" "system" fn WcsGetUsePerUserProfiles(pdevicename : windows_sys::core::PCWSTR, dwdeviceclass : super::DWORD, puseperuserprofiles : super::PBOOL) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("mscms.dll" "system" fn WcsOpenColorProfileA(pcdmpprofile : PPROFILE, pcampprofile : PPROFILE, pgmmpprofile : PPROFILE, dwdesireaccess : super::DWORD, dwsharemode : super::DWORD, dwcreationmode : super::DWORD, dwflags : super::DWORD) -> HPROFILE);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("mscms.dll" "system" fn WcsOpenColorProfileW(pcdmpprofile : PPROFILE, pcampprofile : PPROFILE, pgmmpprofile : PPROFILE, dwdesireaccess : super::DWORD, dwsharemode : super::DWORD, dwcreationmode : super::DWORD, dwflags : super::DWORD) -> HPROFILE);
windows_link::link!("mscms.dll" "system" fn WcsSetCalibrationManagementState(bisenabled : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("mscms.dll" "system" fn WcsSetDefaultColorProfile(scope : WCS_PROFILE_MANAGEMENT_SCOPE, pdevicename : windows_sys::core::PCWSTR, cptcolorprofiletype : COLORPROFILETYPE, cpstcolorprofilesubtype : COLORPROFILESUBTYPE, dwprofileid : super::DWORD, pprofilename : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("mscms.dll" "system" fn WcsSetDefaultRenderingIntent(scope : WCS_PROFILE_MANAGEMENT_SCOPE, dwrenderingintent : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("mscms.dll" "system" fn WcsSetUsePerUserProfiles(pdevicename : windows_sys::core::PCWSTR, dwdeviceclass : super::DWORD, useperuserprofiles : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("mscms.dll" "system" fn WcsTranslateColors(hcolortransform : HTRANSFORM, ncolors : super::DWORD, ninputchannels : super::DWORD, cdtinput : COLORDATATYPE, cbinput : super::DWORD, pinputdata : super::PVOID, noutputchannels : super::DWORD, cdtoutput : COLORDATATYPE, cboutput : super::DWORD, poutputdata : super::PVOID) -> windows_sys::core::BOOL);
pub const ATTRIB_MATTE: i32 = 2;
pub const ATTRIB_TRANSPARENCY: i32 = 1;
pub const BEST_MODE: i32 = 3;
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
pub const CLASS_ABSTRACT: i32 = 1633842036;
pub const CLASS_CAMP: i32 = 1667329392;
pub const CLASS_COLORSPACE: i32 = 1936744803;
pub const CLASS_GMMP: i32 = 1735224688;
pub const CLASS_LINK: i32 = 1818848875;
pub const CLASS_MONITOR: i32 = 1835955314;
pub const CLASS_NAMED: i32 = 1852662636;
pub const CLASS_PRINTER: i32 = 1886549106;
pub const CLASS_SCANNER: i32 = 1935896178;
pub const CMM_DESCRIPTION: i32 = 5;
pub const CMM_DLL_VERSION: i32 = 3;
pub const CMM_DRIVER_VERSION: i32 = 2;
pub const CMM_FROM_PROFILE: i32 = 0;
pub const CMM_IDENT: i32 = 1;
pub const CMM_LOGOICON: i32 = 6;
pub const CMM_VERSION: i32 = 4;
pub const CMM_WINDOWS_DEFAULT: i32 = 1466527264;
pub const CMM_WIN_VERSION: i32 = 0;
pub const CMS_BACKWARD: i32 = 1;
pub const CMS_DISABLEICM: i32 = 1;
pub const CMS_DISABLEINTENT: i32 = 1024;
pub const CMS_DISABLERENDERINTENT: i32 = 2048;
pub const CMS_ENABLEPROOFING: i32 = 2;
pub const CMS_FORWARD: i32 = 0;
pub const CMS_MONITOROVERFLOW: u32 = 2147483648;
pub const CMS_PRINTEROVERFLOW: i32 = 1073741824;
pub const CMS_SETMONITORPROFILE: i32 = 16;
pub const CMS_SETPRINTERPROFILE: i32 = 32;
pub const CMS_SETPROOFINTENT: i32 = 8;
pub const CMS_SETRENDERINTENT: i32 = 4;
pub const CMS_SETTARGETPROFILE: i32 = 64;
pub const CMS_TARGETOVERFLOW: i32 = 536870912;
pub const CMS_USEAPPLYCALLBACK: i32 = 256;
pub const CMS_USEDESCRIPTION: i32 = 512;
pub const CMS_USEHOOK: i32 = 128;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct CMYKCOLOR {
    pub cyan: super::WORD,
    pub magenta: super::WORD,
    pub yellow: super::WORD,
    pub black: super::WORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
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
#[cfg(feature = "shared")]
impl Default for COLOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct COLOR_0 {
    pub reserved1: super::DWORD,
    pub reserved2: *mut core::ffi::c_void,
}
pub type COLORDATATYPE = i32;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Default)]
pub struct COLORMATCHSETUPA {
    pub dwSize: super::DWORD,
    pub dwVersion: super::DWORD,
    pub dwFlags: super::DWORD,
    pub hwndOwner: super::HWND,
    pub pSourceName: windows_sys::core::PCSTR,
    pub pDisplayName: windows_sys::core::PCSTR,
    pub pPrinterName: windows_sys::core::PCSTR,
    pub dwRenderIntent: super::DWORD,
    pub dwProofingIntent: super::DWORD,
    pub pMonitorProfile: windows_sys::core::PSTR,
    pub ccMonitorProfile: super::DWORD,
    pub pPrinterProfile: windows_sys::core::PSTR,
    pub ccPrinterProfile: super::DWORD,
    pub pTargetProfile: windows_sys::core::PSTR,
    pub ccTargetProfile: super::DWORD,
    pub lpfnHook: super::DLGPROC,
    pub lParam: super::LPARAM,
    pub lpfnApplyCallback: PCMSCALLBACKA,
    pub lParamApplyCallback: super::LPARAM,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Default)]
pub struct COLORMATCHSETUPW {
    pub dwSize: super::DWORD,
    pub dwVersion: super::DWORD,
    pub dwFlags: super::DWORD,
    pub hwndOwner: super::HWND,
    pub pSourceName: windows_sys::core::PCWSTR,
    pub pDisplayName: windows_sys::core::PCWSTR,
    pub pPrinterName: windows_sys::core::PCWSTR,
    pub dwRenderIntent: super::DWORD,
    pub dwProofingIntent: super::DWORD,
    pub pMonitorProfile: windows_sys::core::PWSTR,
    pub ccMonitorProfile: super::DWORD,
    pub pPrinterProfile: windows_sys::core::PWSTR,
    pub ccPrinterProfile: super::DWORD,
    pub pTargetProfile: windows_sys::core::PWSTR,
    pub ccTargetProfile: super::DWORD,
    pub lpfnHook: super::DLGPROC,
    pub lParam: super::LPARAM,
    pub lpfnApplyCallback: PCMSCALLBACKW,
    pub lParamApplyCallback: super::LPARAM,
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
pub const COLOR_MATCH_VERSION: i32 = 512;
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
pub const CSA_A: i32 = 1;
pub const CSA_ABC: i32 = 2;
pub const CSA_CMYK: i32 = 7;
pub const CSA_DEF: i32 = 3;
pub const CSA_DEFG: i32 = 4;
pub const CSA_GRAY: i32 = 5;
pub const CSA_Lab: i32 = 8;
pub const CSA_RGB: i32 = 6;
pub const DONT_USE_EMBEDDED_WCS_PROFILES: i32 = 1;
pub const ENABLE_GAMUT_CHECKING: i32 = 65536;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct ENUMTYPEA {
    pub dwSize: super::DWORD,
    pub dwVersion: super::DWORD,
    pub dwFields: super::DWORD,
    pub pDeviceName: windows_sys::core::PCSTR,
    pub dwMediaType: super::DWORD,
    pub dwDitheringMode: super::DWORD,
    pub dwResolution: [super::DWORD; 2],
    pub dwCMMType: super::DWORD,
    pub dwClass: super::DWORD,
    pub dwDataColorSpace: super::DWORD,
    pub dwConnectionSpace: super::DWORD,
    pub dwSignature: super::DWORD,
    pub dwPlatform: super::DWORD,
    pub dwProfileFlags: super::DWORD,
    pub dwManufacturer: super::DWORD,
    pub dwModel: super::DWORD,
    pub dwAttributes: [super::DWORD; 2],
    pub dwRenderingIntent: super::DWORD,
    pub dwCreator: super::DWORD,
    pub dwDeviceClass: super::DWORD,
}
#[cfg(feature = "shared")]
impl Default for ENUMTYPEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct ENUMTYPEW {
    pub dwSize: super::DWORD,
    pub dwVersion: super::DWORD,
    pub dwFields: super::DWORD,
    pub pDeviceName: windows_sys::core::PCWSTR,
    pub dwMediaType: super::DWORD,
    pub dwDitheringMode: super::DWORD,
    pub dwResolution: [super::DWORD; 2],
    pub dwCMMType: super::DWORD,
    pub dwClass: super::DWORD,
    pub dwDataColorSpace: super::DWORD,
    pub dwConnectionSpace: super::DWORD,
    pub dwSignature: super::DWORD,
    pub dwPlatform: super::DWORD,
    pub dwProfileFlags: super::DWORD,
    pub dwManufacturer: super::DWORD,
    pub dwModel: super::DWORD,
    pub dwAttributes: [super::DWORD; 2],
    pub dwRenderingIntent: super::DWORD,
    pub dwCreator: super::DWORD,
    pub dwDeviceClass: super::DWORD,
}
#[cfg(feature = "shared")]
impl Default for ENUMTYPEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ENUM_TYPE_VERSION: i32 = 768;
pub const ET_ATTRIBUTES: i32 = 8192;
pub const ET_CLASS: i32 = 32;
pub const ET_CMMTYPE: i32 = 16;
pub const ET_CONNECTIONSPACE: i32 = 128;
pub const ET_CREATOR: i32 = 32768;
pub const ET_DATACOLORSPACE: i32 = 64;
pub const ET_DEVICECLASS: i32 = 65536;
pub const ET_DEVICENAME: i32 = 1;
pub const ET_DITHERMODE: i32 = 4;
pub const ET_EXTENDEDDISPLAYCOLOR: i32 = 262144;
pub const ET_MANUFACTURER: i32 = 2048;
pub const ET_MEDIATYPE: i32 = 2;
pub const ET_MODEL: i32 = 4096;
pub const ET_PLATFORM: i32 = 512;
pub const ET_PROFILEFLAGS: i32 = 1024;
pub const ET_RENDERINGINTENT: i32 = 16384;
pub const ET_RESOLUTION: i32 = 8;
pub const ET_SIGNATURE: i32 = 256;
pub const ET_STANDARDDISPLAYCOLOR: i32 = 131072;
pub const FAST_TRANSLATE: i32 = 262144;
pub const FLAG_DEPENDENTONDATA: i32 = 2;
pub const FLAG_EMBEDDEDPROFILE: i32 = 1;
pub const FLAG_ENABLE_CHROMATIC_ADAPTATION: i32 = 33554432;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct GENERIC3CHANNEL {
    pub ch1: super::WORD,
    pub ch2: super::WORD,
    pub ch3: super::WORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct GRAYCOLOR {
    pub gray: super::WORD,
}
pub type HCMTRANSFORM = *mut core::ffi::c_void;
pub type HPROFILE = *mut core::ffi::c_void;
pub type HTRANSFORM = *mut core::ffi::c_void;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct HiFiCOLOR {
    pub channel: [super::BYTE; 8],
}
#[cfg(feature = "shared")]
impl Default for HiFiCOLOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const INDEX_DONT_CARE: i32 = 0;
pub const INTENT_ABSOLUTE_COLORIMETRIC: i32 = 3;
pub const INTENT_PERCEPTUAL: i32 = 0;
pub const INTENT_RELATIVE_COLORIMETRIC: i32 = 1;
pub const INTENT_SATURATION: i32 = 2;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type LPBMCALLBACKFN = Option<unsafe extern "system" fn(param0: super::ULONG, param1: super::ULONG, param2: super::LPARAM) -> windows_sys::core::BOOL>;
pub type LPBMFORMAT = *mut BMFORMAT;
#[cfg(feature = "shared")]
pub type LPCOLOR = *mut COLOR;
pub type LPCOLORDATATYPE = *mut COLORDATATYPE;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef", feature = "winuser"))]
pub type LPCOLORMATCHSETUPA = *mut COLORMATCHSETUPA;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef", feature = "winuser"))]
pub type LPCOLORMATCHSETUPW = *mut COLORMATCHSETUPW;
pub type LPCOLORPROFILESUBTYPE = *mut COLORPROFILESUBTYPE;
pub type LPCOLORPROFILETYPE = *mut COLORPROFILETYPE;
pub type LPCOLORTYPE = *mut COLORTYPE;
pub type LPCOLOR_NAME = *mut [i8; 32];
pub type LPDEVCHARACTER = *mut core::ffi::c_void;
#[cfg(feature = "shared")]
pub type LPENUMTYPEA = *mut ENUMTYPEA;
#[cfg(feature = "shared")]
pub type LPENUMTYPEW = *mut ENUMTYPEW;
#[cfg(feature = "shared")]
pub type LPNAMED_PROFILE_INFO = *mut NAMED_PROFILE_INFO;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPPROFILE = *mut PROFILE;
#[cfg(all(feature = "shared", feature = "wingdi"))]
pub type LPPROFILEHEADER = *mut PROFILEHEADER;
pub type LPTAGTYPE = *mut u32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct LabCOLOR {
    pub L: super::WORD,
    pub a: super::WORD,
    pub b: super::WORD,
}
pub const MAX_COLOR_CHANNELS: i32 = 8;
pub const MicrosoftHardwareColorV2: WCS_DEVICE_CAPABILITIES_TYPE = 2;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct NAMEDCOLOR {
    pub dwIndex: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct NAMED_PROFILE_INFO {
    pub dwFlags: super::DWORD,
    pub dwCount: super::DWORD,
    pub dwCountDevCoordinates: super::DWORD,
    pub szPrefix: COLOR_NAME,
    pub szSuffix: COLOR_NAME,
}
#[cfg(feature = "shared")]
impl Default for NAMED_PROFILE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NORMAL_MODE: i32 = 2;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PBMCALLBACKFN = Option<unsafe extern "system" fn(param0: super::ULONG, param1: super::ULONG, param2: super::LPARAM) -> windows_sys::core::BOOL>;
pub type PBMFORMAT = *mut BMFORMAT;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef", feature = "winuser"))]
pub type PCMSCALLBACKA = Option<unsafe extern "system" fn(param0: *mut COLORMATCHSETUPA, param1: super::LPARAM) -> windows_sys::core::BOOL>;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef", feature = "winuser"))]
pub type PCMSCALLBACKW = Option<unsafe extern "system" fn(param0: *mut COLORMATCHSETUPW, param1: super::LPARAM) -> windows_sys::core::BOOL>;
#[cfg(feature = "shared")]
pub type PCOLOR = *mut COLOR;
pub type PCOLORDATATYPE = *mut COLORDATATYPE;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef", feature = "winuser"))]
pub type PCOLORMATCHSETUPA = *mut COLORMATCHSETUPA;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef", feature = "winuser"))]
pub type PCOLORMATCHSETUPW = *mut COLORMATCHSETUPW;
pub type PCOLORPROFILESUBTYPE = *mut COLORPROFILESUBTYPE;
pub type PCOLORPROFILETYPE = *mut COLORPROFILETYPE;
pub type PCOLORTYPE = *mut COLORTYPE;
pub type PCOLOR_NAME = *mut [i8; 32];
#[cfg(feature = "shared")]
pub type PENUMTYPEA = *mut ENUMTYPEA;
#[cfg(feature = "shared")]
pub type PENUMTYPEW = *mut ENUMTYPEW;
pub type PHPROFILE = *mut *mut core::ffi::c_void;
#[cfg(feature = "shared")]
pub type PNAMED_PROFILE_INFO = *mut NAMED_PROFILE_INFO;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PPROFILE = *mut PROFILE;
#[cfg(all(feature = "shared", feature = "wingdi"))]
pub type PPROFILEHEADER = *mut PROFILEHEADER;
pub const PRESERVEBLACK: i32 = 1048576;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct PROFILE {
    pub dwType: super::DWORD,
    pub pProfileData: super::PVOID,
    pub cbDataSize: super::DWORD,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wingdi"))]
#[derive(Clone, Copy)]
pub struct PROFILEHEADER {
    pub phSize: super::DWORD,
    pub phCMMType: super::DWORD,
    pub phVersion: super::DWORD,
    pub phClass: super::DWORD,
    pub phDataColorSpace: super::DWORD,
    pub phConnectionSpace: super::DWORD,
    pub phDateTime: [super::DWORD; 3],
    pub phSignature: super::DWORD,
    pub phPlatform: super::DWORD,
    pub phProfileFlags: super::DWORD,
    pub phManufacturer: super::DWORD,
    pub phModel: super::DWORD,
    pub phAttributes: [super::DWORD; 2],
    pub phRenderingIntent: super::DWORD,
    pub phIlluminant: super::CIEXYZ,
    pub phCreator: super::DWORD,
    pub phReserved: [super::BYTE; 44],
}
#[cfg(all(feature = "shared", feature = "wingdi"))]
impl Default for PROFILEHEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PROFILE_FILENAME: i32 = 1;
pub const PROFILE_MEMBUFFER: i32 = 2;
pub const PROFILE_READ: i32 = 1;
pub const PROFILE_READWRITE: i32 = 2;
pub const PROOF_MODE: i32 = 1;
pub type PTAGTYPE = *mut u32;
pub const RESERVED: u32 = 2147483648;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct RGBCOLOR {
    pub red: super::WORD,
    pub green: super::WORD,
    pub blue: super::WORD,
}
pub const SEQUENTIAL_TRANSFORM: u32 = 2155872256;
pub const SPACE_2_CHANNEL: i32 = 843271250;
pub const SPACE_3_CHANNEL: i32 = 860048466;
pub const SPACE_4_CHANNEL: i32 = 876825682;
pub const SPACE_5_CHANNEL: i32 = 893602898;
pub const SPACE_6_CHANNEL: i32 = 910380114;
pub const SPACE_7_CHANNEL: i32 = 927157330;
pub const SPACE_8_CHANNEL: i32 = 943934546;
pub const SPACE_CMY: i32 = 1129142560;
pub const SPACE_CMYK: i32 = 1129142603;
pub const SPACE_GRAY: i32 = 1196573017;
pub const SPACE_HLS: i32 = 1212961568;
pub const SPACE_HSV: i32 = 1213421088;
pub const SPACE_Lab: i32 = 1281450528;
pub const SPACE_Luv: i32 = 1282766368;
pub const SPACE_RGB: i32 = 1380401696;
pub const SPACE_XYZ: i32 = 1482250784;
pub const SPACE_YCbCr: i32 = 1497588338;
pub const SPACE_Yxy: i32 = 1501067552;
pub type TAGTYPE = u32;
pub const USE_RELATIVE_COLORIMETRIC: i32 = 131072;
pub const VideoCardGammaTable: WCS_DEVICE_CAPABILITIES_TYPE = 1;
pub const WCS_ALWAYS: i32 = 2097152;
pub const WCS_DEFAULT: i32 = 0;
pub type WCS_DEVICE_CAPABILITIES_TYPE = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WCS_DEVICE_MHC2_CAPABILITIES {
    pub Size: super::UINT,
    pub SupportsMhc2: windows_sys::core::BOOL,
    pub RegammaLutEntryCount: super::UINT,
    pub CscXyzMatrixRows: super::UINT,
    pub CscXyzMatrixColumns: super::UINT,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WCS_DEVICE_VCGT_CAPABILITIES {
    pub Size: super::UINT,
    pub SupportsVcgt: windows_sys::core::BOOL,
}
pub const WCS_EMBEDDED_TAG_SIGNATURE: i32 = 1297297456;
pub const WCS_EMBEDDED_TAG_TYPE_SIGNATURE: i32 = 1297297712;
pub const WCS_ICCONLY: i32 = 65536;
pub type WCS_PROFILE_MANAGEMENT_SCOPE = i32;
pub const WCS_PROFILE_MANAGEMENT_SCOPE_CURRENT_USER: WCS_PROFILE_MANAGEMENT_SCOPE = 1;
pub const WCS_PROFILE_MANAGEMENT_SCOPE_SYSTEM_WIDE: WCS_PROFILE_MANAGEMENT_SCOPE = 0;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct XYZCOLOR {
    pub X: super::WORD,
    pub Y: super::WORD,
    pub Z: super::WORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct YxyCOLOR {
    pub Y: super::WORD,
    pub x: super::WORD,
    pub y: super::WORD,
}
