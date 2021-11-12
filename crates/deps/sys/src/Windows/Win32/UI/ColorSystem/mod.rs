#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn AssociateColorProfileWithDeviceA(pmachinename: super::super::Foundation::PSTR, pprofilename: super::super::Foundation::PSTR, pdevicename: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn AssociateColorProfileWithDeviceW(pmachinename: super::super::Foundation::PWSTR, pprofilename: super::super::Foundation::PWSTR, pdevicename: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CMCheckColors(hcmtransform: isize, lpainputcolors: *const COLOR, ncolors: u32, ctinput: COLORTYPE, lparesult: *mut u8) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn CMCheckColorsInGamut(hcmtransform: isize, lpargbtriple: *const super::super::Graphics::Gdi::RGBTRIPLE, lparesult: *mut u8, ncount: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CMCheckRGBs(hcmtransform: isize, lpsrcbits: *const ::core::ffi::c_void, bminput: BMFORMAT, dwwidth: u32, dwheight: u32, dwstride: u32, lparesult: *mut u8, pfncallback: LPBMCALLBACKFN, ulcallbackdata: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CMConvertColorNameToIndex(hprofile: isize, pacolorname: *const *const i8, paindex: *mut u32, dwcount: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CMConvertIndexToColorName(hprofile: isize, paindex: *const u32, pacolorname: *mut *mut i8, dwcount: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CMCreateDeviceLinkProfile(pahprofiles: *const isize, nprofiles: u32, padwintents: *const u32, nintents: u32, dwflags: u32, lpprofiledata: *mut *mut u8) -> super::super::Foundation::BOOL;
    pub fn CMCreateMultiProfileTransform(pahprofiles: *const isize, nprofiles: u32, padwintents: *const u32, nintents: u32, dwflags: u32) -> isize;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn CMCreateProfile(lpcolorspace: *mut LOGCOLORSPACEA, lpprofiledata: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn CMCreateProfileW(lpcolorspace: *mut LOGCOLORSPACEW, lpprofiledata: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn CMCreateTransform(lpcolorspace: *const LOGCOLORSPACEA, lpdevcharacter: *const ::core::ffi::c_void, lptargetdevcharacter: *const ::core::ffi::c_void) -> isize;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn CMCreateTransformExt(lpcolorspace: *const LOGCOLORSPACEA, lpdevcharacter: *const ::core::ffi::c_void, lptargetdevcharacter: *const ::core::ffi::c_void, dwflags: u32) -> isize;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn CMCreateTransformExtW(lpcolorspace: *const LOGCOLORSPACEW, lpdevcharacter: *const ::core::ffi::c_void, lptargetdevcharacter: *const ::core::ffi::c_void, dwflags: u32) -> isize;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn CMCreateTransformW(lpcolorspace: *const LOGCOLORSPACEW, lpdevcharacter: *const ::core::ffi::c_void, lptargetdevcharacter: *const ::core::ffi::c_void) -> isize;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CMDeleteTransform(hcmtransform: isize) -> super::super::Foundation::BOOL;
    pub fn CMGetInfo(dwinfo: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CMGetNamedProfileInfo(hprofile: isize, pnamedprofileinfo: *mut NAMED_PROFILE_INFO) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CMIsProfileValid(hprofile: isize, lpbvalid: *mut i32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CMTranslateColors(hcmtransform: isize, lpainputcolors: *const COLOR, ncolors: u32, ctinput: COLORTYPE, lpaoutputcolors: *mut COLOR, ctoutput: COLORTYPE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CMTranslateRGB(hcmtransform: isize, colorref: u32, lpcolorref: *mut u32, dwflags: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CMTranslateRGBs(hcmtransform: isize, lpsrcbits: *const ::core::ffi::c_void, bminput: BMFORMAT, dwwidth: u32, dwheight: u32, dwstride: u32, lpdestbits: *mut ::core::ffi::c_void, bmoutput: BMFORMAT, dwtranslatedirection: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CMTranslateRGBsExt(hcmtransform: isize, lpsrcbits: *const ::core::ffi::c_void, bminput: BMFORMAT, dwwidth: u32, dwheight: u32, dwinputstride: u32, lpdestbits: *mut ::core::ffi::c_void, bmoutput: BMFORMAT, dwoutputstride: u32, lpfncallback: LPBMCALLBACKFN, ulcallbackdata: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CheckBitmapBits(hcolortransform: isize, psrcbits: *const ::core::ffi::c_void, bminput: BMFORMAT, dwwidth: u32, dwheight: u32, dwstride: u32, paresult: *mut u8, pfncallback: LPBMCALLBACKFN, lpcallbackdata: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CheckColors(hcolortransform: isize, painputcolors: *const COLOR, ncolors: u32, ctinput: COLORTYPE, paresult: *mut u8) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn CheckColorsInGamut(hdc: super::super::Graphics::Gdi::HDC, lprgbtriple: *const super::super::Graphics::Gdi::RGBTRIPLE, dlpbuffer: *mut ::core::ffi::c_void, ncount: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CloseColorProfile(hprofile: isize) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn ColorCorrectPalette(hdc: super::super::Graphics::Gdi::HDC, hpal: super::super::Graphics::Gdi::HPALETTE, defirst: u32, num: u32) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn ColorMatchToTarget(hdc: super::super::Graphics::Gdi::HDC, hdctarget: super::super::Graphics::Gdi::HDC, action: COLOR_MATCH_TO_TARGET_ACTION) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ColorProfileAddDisplayAssociation(scope: WCS_PROFILE_MANAGEMENT_SCOPE, profilename: super::super::Foundation::PWSTR, targetadapterid: super::super::Foundation::LUID, sourceid: u32, setasdefault: super::super::Foundation::BOOL, associateasadvancedcolor: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ColorProfileGetDisplayDefault(scope: WCS_PROFILE_MANAGEMENT_SCOPE, targetadapterid: super::super::Foundation::LUID, sourceid: u32, profiletype: COLORPROFILETYPE, profilesubtype: COLORPROFILESUBTYPE, profilename: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ColorProfileGetDisplayList(scope: WCS_PROFILE_MANAGEMENT_SCOPE, targetadapterid: super::super::Foundation::LUID, sourceid: u32, profilelist: *mut *mut super::super::Foundation::PWSTR, profilecount: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ColorProfileGetDisplayUserScope(targetadapterid: super::super::Foundation::LUID, sourceid: u32, scope: *mut WCS_PROFILE_MANAGEMENT_SCOPE) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ColorProfileRemoveDisplayAssociation(scope: WCS_PROFILE_MANAGEMENT_SCOPE, profilename: super::super::Foundation::PWSTR, targetadapterid: super::super::Foundation::LUID, sourceid: u32, dissociateadvancedcolor: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ColorProfileSetDisplayDefaultAssociation(scope: WCS_PROFILE_MANAGEMENT_SCOPE, profilename: super::super::Foundation::PWSTR, profiletype: COLORPROFILETYPE, profilesubtype: COLORPROFILESUBTYPE, targetadapterid: super::super::Foundation::LUID, sourceid: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConvertColorNameToIndex(hprofile: isize, pacolorname: *const *const i8, paindex: *mut u32, dwcount: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConvertIndexToColorName(hprofile: isize, paindex: *const u32, pacolorname: *mut *mut i8, dwcount: u32) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn CreateColorSpaceA(lplcs: *const LOGCOLORSPACEA) -> HCOLORSPACE;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn CreateColorSpaceW(lplcs: *const LOGCOLORSPACEW) -> HCOLORSPACE;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn CreateColorTransformA(plogcolorspace: *const LOGCOLORSPACEA, hdestprofile: isize, htargetprofile: isize, dwflags: u32) -> isize;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn CreateColorTransformW(plogcolorspace: *const LOGCOLORSPACEW, hdestprofile: isize, htargetprofile: isize, dwflags: u32) -> isize;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateDeviceLinkProfile(hprofile: *const isize, nprofiles: u32, padwintent: *const u32, nintents: u32, dwflags: u32, pprofiledata: *mut *mut u8, indexpreferredcmm: u32) -> super::super::Foundation::BOOL;
    pub fn CreateMultiProfileTransform(pahprofiles: *const isize, nprofiles: u32, padwintent: *const u32, nintents: u32, dwflags: u32, indexpreferredcmm: u32) -> isize;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn CreateProfileFromLogColorSpaceA(plogcolorspace: *const LOGCOLORSPACEA, pprofile: *mut *mut u8) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn CreateProfileFromLogColorSpaceW(plogcolorspace: *const LOGCOLORSPACEW, pprofile: *mut *mut u8) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteColorSpace(hcs: HCOLORSPACE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteColorTransform(hxform: isize) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DisassociateColorProfileFromDeviceA(pmachinename: super::super::Foundation::PSTR, pprofilename: super::super::Foundation::PSTR, pdevicename: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DisassociateColorProfileFromDeviceW(pmachinename: super::super::Foundation::PWSTR, pprofilename: super::super::Foundation::PWSTR, pdevicename: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumColorProfilesA(pmachinename: super::super::Foundation::PSTR, penumrecord: *const ENUMTYPEA, penumerationbuffer: *mut u8, pdwsizeofenumerationbuffer: *mut u32, pnprofiles: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumColorProfilesW(pmachinename: super::super::Foundation::PWSTR, penumrecord: *const ENUMTYPEW, penumerationbuffer: *mut u8, pdwsizeofenumerationbuffer: *mut u32, pnprofiles: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn EnumICMProfilesA(hdc: super::super::Graphics::Gdi::HDC, proc: ICMENUMPROCA, param2: super::super::Foundation::LPARAM) -> i32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn EnumICMProfilesW(hdc: super::super::Graphics::Gdi::HDC, proc: ICMENUMPROCW, param2: super::super::Foundation::LPARAM) -> i32;
    pub fn GetCMMInfo(hcolortransform: isize, param1: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetColorDirectoryA(pmachinename: super::super::Foundation::PSTR, pbuffer: super::super::Foundation::PSTR, pdwsize: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetColorDirectoryW(pmachinename: super::super::Foundation::PWSTR, pbuffer: super::super::Foundation::PWSTR, pdwsize: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetColorProfileElement(hprofile: isize, tag: u32, dwoffset: u32, pcbelement: *mut u32, pelement: *mut ::core::ffi::c_void, pbreference: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetColorProfileElementTag(hprofile: isize, dwindex: u32, ptag: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetColorProfileFromHandle(hprofile: isize, pprofile: *mut u8, pcbprofile: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn GetColorProfileHeader(hprofile: isize, pheader: *mut PROFILEHEADER) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn GetColorSpace(hdc: super::super::Graphics::Gdi::HDC) -> HCOLORSPACE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCountColorProfileElements(hprofile: isize, pnelementcount: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn GetDeviceGammaRamp(hdc: super::super::Graphics::Gdi::HDC, lpramp: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn GetICMProfileA(hdc: super::super::Graphics::Gdi::HDC, pbufsize: *mut u32, pszfilename: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn GetICMProfileW(hdc: super::super::Graphics::Gdi::HDC, pbufsize: *mut u32, pszfilename: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn GetLogColorSpaceA(hcolorspace: HCOLORSPACE, lpbuffer: *mut LOGCOLORSPACEA, nsize: u32) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn GetLogColorSpaceW(hcolorspace: HCOLORSPACE, lpbuffer: *mut LOGCOLORSPACEW, nsize: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNamedProfileInfo(hprofile: isize, pnamedprofileinfo: *mut NAMED_PROFILE_INFO) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPS2ColorRenderingDictionary(hprofile: isize, dwintent: u32, pps2colorrenderingdictionary: *mut u8, pcbps2colorrenderingdictionary: *mut u32, pbbinary: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPS2ColorRenderingIntent(hprofile: isize, dwintent: u32, pbuffer: *mut u8, pcbps2colorrenderingintent: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPS2ColorSpaceArray(hprofile: isize, dwintent: u32, dwcsatype: u32, pps2colorspacearray: *mut u8, pcbps2colorspacearray: *mut u32, pbbinary: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetStandardColorSpaceProfileA(pmachinename: super::super::Foundation::PSTR, dwscs: u32, pbuffer: super::super::Foundation::PSTR, pcbsize: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetStandardColorSpaceProfileW(pmachinename: super::super::Foundation::PWSTR, dwscs: u32, pbuffer: super::super::Foundation::PWSTR, pcbsize: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn InstallColorProfileA(pmachinename: super::super::Foundation::PSTR, pprofilename: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn InstallColorProfileW(pmachinename: super::super::Foundation::PWSTR, pprofilename: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsColorProfileTagPresent(hprofile: isize, tag: u32, pbpresent: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsColorProfileValid(hprofile: isize, pbvalid: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    pub fn OpenColorProfileA(pprofile: *const PROFILE, dwdesiredaccess: u32, dwsharemode: u32, dwcreationmode: u32) -> isize;
    pub fn OpenColorProfileW(pprofile: *const PROFILE, dwdesiredaccess: u32, dwsharemode: u32, dwcreationmode: u32) -> isize;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterCMMA(pmachinename: super::super::Foundation::PSTR, cmmid: u32, pcmmdll: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterCMMW(pmachinename: super::super::Foundation::PWSTR, cmmid: u32, pcmmdll: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SelectCMM(dwcmmtype: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetColorProfileElement(hprofile: isize, tag: u32, dwoffset: u32, pcbelement: *const u32, pelement: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetColorProfileElementReference(hprofile: isize, newtag: u32, reftag: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetColorProfileElementSize(hprofile: isize, tagtype: u32, pcbelement: u32) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn SetColorProfileHeader(hprofile: isize, pheader: *const PROFILEHEADER) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn SetColorSpace(hdc: super::super::Graphics::Gdi::HDC, hcs: HCOLORSPACE) -> HCOLORSPACE;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn SetDeviceGammaRamp(hdc: super::super::Graphics::Gdi::HDC, lpramp: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn SetICMMode(hdc: super::super::Graphics::Gdi::HDC, mode: i32) -> i32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn SetICMProfileA(hdc: super::super::Graphics::Gdi::HDC, lpfilename: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn SetICMProfileW(hdc: super::super::Graphics::Gdi::HDC, lpfilename: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetStandardColorSpaceProfileA(pmachinename: super::super::Foundation::PSTR, dwprofileid: u32, pprofilename: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetStandardColorSpaceProfileW(pmachinename: super::super::Foundation::PWSTR, dwprofileid: u32, pprofilename: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn SetupColorMatchingA(pcms: *mut COLORMATCHSETUPA) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn SetupColorMatchingW(pcms: *mut COLORMATCHSETUPW) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn TranslateBitmapBits(hcolortransform: isize, psrcbits: *const ::core::ffi::c_void, bminput: BMFORMAT, dwwidth: u32, dwheight: u32, dwinputstride: u32, pdestbits: *mut ::core::ffi::c_void, bmoutput: BMFORMAT, dwoutputstride: u32, pfncallback: LPBMCALLBACKFN, ulcallbackdata: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn TranslateColors(hcolortransform: isize, painputcolors: *const COLOR, ncolors: u32, ctinput: COLORTYPE, paoutputcolors: *mut COLOR, ctoutput: COLORTYPE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn UninstallColorProfileA(pmachinename: super::super::Foundation::PSTR, pprofilename: super::super::Foundation::PSTR, bdelete: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn UninstallColorProfileW(pmachinename: super::super::Foundation::PWSTR, pprofilename: super::super::Foundation::PWSTR, bdelete: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnregisterCMMA(pmachinename: super::super::Foundation::PSTR, cmmid: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnregisterCMMW(pmachinename: super::super::Foundation::PWSTR, cmmid: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn UpdateICMRegKeyA(reserved: u32, lpszcmid: super::super::Foundation::PSTR, lpszfilename: super::super::Foundation::PSTR, command: ICM_COMMAND) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn UpdateICMRegKeyW(reserved: u32, lpszcmid: super::super::Foundation::PWSTR, lpszfilename: super::super::Foundation::PWSTR, command: ICM_COMMAND) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WcsAssociateColorProfileWithDevice(scope: WCS_PROFILE_MANAGEMENT_SCOPE, pprofilename: super::super::Foundation::PWSTR, pdevicename: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WcsCheckColors(hcolortransform: isize, ncolors: u32, ninputchannels: u32, cdtinput: COLORDATATYPE, cbinput: u32, pinputdata: *const ::core::ffi::c_void, paresult: *mut u8) -> super::super::Foundation::BOOL;
    pub fn WcsCreateIccProfile(hwcsprofile: isize, dwoptions: u32) -> isize;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WcsDisassociateColorProfileFromDevice(scope: WCS_PROFILE_MANAGEMENT_SCOPE, pprofilename: super::super::Foundation::PWSTR, pdevicename: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WcsEnumColorProfiles(scope: WCS_PROFILE_MANAGEMENT_SCOPE, penumrecord: *const ENUMTYPEW, pbuffer: *mut u8, dwsize: u32, pnprofiles: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WcsEnumColorProfilesSize(scope: WCS_PROFILE_MANAGEMENT_SCOPE, penumrecord: *const ENUMTYPEW, pdwsize: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WcsGetCalibrationManagementState(pbisenabled: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WcsGetDefaultColorProfile(scope: WCS_PROFILE_MANAGEMENT_SCOPE, pdevicename: super::super::Foundation::PWSTR, cptcolorprofiletype: COLORPROFILETYPE, cpstcolorprofilesubtype: COLORPROFILESUBTYPE, dwprofileid: u32, cbprofilename: u32, pprofilename: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WcsGetDefaultColorProfileSize(scope: WCS_PROFILE_MANAGEMENT_SCOPE, pdevicename: super::super::Foundation::PWSTR, cptcolorprofiletype: COLORPROFILETYPE, cpstcolorprofilesubtype: COLORPROFILESUBTYPE, dwprofileid: u32, pcbprofilename: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WcsGetDefaultRenderingIntent(scope: WCS_PROFILE_MANAGEMENT_SCOPE, pdwrenderingintent: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WcsGetUsePerUserProfiles(pdevicename: super::super::Foundation::PWSTR, dwdeviceclass: u32, puseperuserprofiles: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    pub fn WcsOpenColorProfileA(pcdmpprofile: *const PROFILE, pcampprofile: *const PROFILE, pgmmpprofile: *const PROFILE, dwdesireaccess: u32, dwsharemode: u32, dwcreationmode: u32, dwflags: u32) -> isize;
    pub fn WcsOpenColorProfileW(pcdmpprofile: *const PROFILE, pcampprofile: *const PROFILE, pgmmpprofile: *const PROFILE, dwdesireaccess: u32, dwsharemode: u32, dwcreationmode: u32, dwflags: u32) -> isize;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WcsSetCalibrationManagementState(bisenabled: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WcsSetDefaultColorProfile(scope: WCS_PROFILE_MANAGEMENT_SCOPE, pdevicename: super::super::Foundation::PWSTR, cptcolorprofiletype: COLORPROFILETYPE, cpstcolorprofilesubtype: COLORPROFILESUBTYPE, dwprofileid: u32, pprofilename: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WcsSetDefaultRenderingIntent(scope: WCS_PROFILE_MANAGEMENT_SCOPE, dwrenderingintent: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WcsSetUsePerUserProfiles(pdevicename: super::super::Foundation::PWSTR, dwdeviceclass: u32, useperuserprofiles: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WcsTranslateColors(hcolortransform: isize, ncolors: u32, ninputchannels: u32, cdtinput: COLORDATATYPE, cbinput: u32, pinputdata: *const ::core::ffi::c_void, noutputchannels: u32, cdtoutput: COLORDATATYPE, cboutput: u32, poutputdata: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
}
pub const ATTRIB_MATTE: u32 = 2u32;
pub const ATTRIB_TRANSPARENCY: u32 = 1u32;
pub const BEST_MODE: u32 = 3u32;
#[repr(C)]
pub struct BMFORMAT(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct BlackInformation(i32);
pub const CATID_WcsPlugin: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2696151776,
    data2: 33344,
    data3: 16479,
    data4: [138, 22, 138, 91, 77, 242, 240, 221],
};
pub const CMM_DESCRIPTION: u32 = 5u32;
pub const CMM_DLL_VERSION: u32 = 3u32;
pub const CMM_DRIVER_VERSION: u32 = 2u32;
pub const CMM_FROM_PROFILE: u32 = 0u32;
pub const CMM_IDENT: u32 = 1u32;
pub const CMM_LOGOICON: u32 = 6u32;
pub const CMM_VERSION: u32 = 4u32;
pub const CMM_WIN_VERSION: u32 = 0u32;
pub const CMS_BACKWARD: u32 = 1u32;
pub const CMS_DISABLEICM: u32 = 1u32;
pub const CMS_DISABLEINTENT: u32 = 1024u32;
pub const CMS_DISABLERENDERINTENT: u32 = 2048u32;
pub const CMS_ENABLEPROOFING: u32 = 2u32;
pub const CMS_FORWARD: u32 = 0u32;
pub const CMS_MONITOROVERFLOW: i32 = -2147483648i32;
pub const CMS_PRINTEROVERFLOW: i32 = 1073741824i32;
pub const CMS_SETMONITORPROFILE: u32 = 16u32;
pub const CMS_SETPRINTERPROFILE: u32 = 32u32;
pub const CMS_SETPROOFINTENT: u32 = 8u32;
pub const CMS_SETRENDERINTENT: u32 = 4u32;
pub const CMS_SETTARGETPROFILE: u32 = 64u32;
pub const CMS_TARGETOVERFLOW: i32 = 536870912i32;
pub const CMS_USEAPPLYCALLBACK: u32 = 256u32;
pub const CMS_USEDESCRIPTION: u32 = 512u32;
pub const CMS_USEHOOK: u32 = 128u32;
#[repr(C)]
pub struct CMYKCOLOR(i32);
#[repr(C)]
pub struct COLOR(i32);
#[repr(C)]
pub struct COLORDATATYPE(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[repr(C)]
pub struct COLORMATCHSETUPA(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[repr(C)]
pub struct COLORMATCHSETUPW(i32);
#[repr(C)]
pub struct COLORPROFILESUBTYPE(i32);
#[repr(C)]
pub struct COLORPROFILETYPE(i32);
#[repr(C)]
pub struct COLORTYPE(i32);
#[repr(C)]
pub struct COLOR_MATCH_TO_TARGET_ACTION(i32);
pub const COLOR_MATCH_VERSION: u32 = 512u32;
pub const CSA_A: u32 = 1u32;
pub const CSA_ABC: u32 = 2u32;
pub const CSA_CMYK: u32 = 7u32;
pub const CSA_DEF: u32 = 3u32;
pub const CSA_DEFG: u32 = 4u32;
pub const CSA_GRAY: u32 = 5u32;
pub const CSA_Lab: u32 = 8u32;
pub const CSA_RGB: u32 = 6u32;
pub const DONT_USE_EMBEDDED_WCS_PROFILES: i32 = 1i32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct EMRCREATECOLORSPACE(i32);
#[cfg(feature = "Win32_Graphics_Gdi")]
#[repr(C)]
pub struct EMRCREATECOLORSPACEW(i32);
pub const ENABLE_GAMUT_CHECKING: u32 = 65536u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct ENUMTYPEA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct ENUMTYPEW(i32);
pub const ENUM_TYPE_VERSION: u32 = 768u32;
pub const ET_ATTRIBUTES: u32 = 8192u32;
pub const ET_CLASS: u32 = 32u32;
pub const ET_CMMTYPE: u32 = 16u32;
pub const ET_CONNECTIONSPACE: u32 = 128u32;
pub const ET_CREATOR: u32 = 32768u32;
pub const ET_DATACOLORSPACE: u32 = 64u32;
pub const ET_DEVICECLASS: u32 = 65536u32;
pub const ET_DEVICENAME: u32 = 1u32;
pub const ET_DITHERMODE: u32 = 4u32;
pub const ET_EXTENDEDDISPLAYCOLOR: u32 = 262144u32;
pub const ET_MANUFACTURER: u32 = 2048u32;
pub const ET_MEDIATYPE: u32 = 2u32;
pub const ET_MODEL: u32 = 4096u32;
pub const ET_PLATFORM: u32 = 512u32;
pub const ET_PROFILEFLAGS: u32 = 1024u32;
pub const ET_RENDERINGINTENT: u32 = 16384u32;
pub const ET_RESOLUTION: u32 = 8u32;
pub const ET_SIGNATURE: u32 = 256u32;
pub const ET_STANDARDDISPLAYCOLOR: u32 = 131072u32;
pub const FAST_TRANSLATE: u32 = 262144u32;
pub const FLAG_DEPENDENTONDATA: u32 = 2u32;
pub const FLAG_EMBEDDEDPROFILE: u32 = 1u32;
pub const FLAG_ENABLE_CHROMATIC_ADAPTATION: u32 = 33554432u32;
#[repr(C)]
pub struct GENERIC3CHANNEL(i32);
#[repr(C)]
pub struct GRAYCOLOR(i32);
#[repr(C)]
pub struct GamutBoundaryDescription(i32);
#[repr(C)]
pub struct GamutShell(i32);
#[repr(C)]
pub struct GamutShellTriangle(i32);
#[repr(C)]
pub struct HCOLORSPACE(i32);
#[repr(C)]
pub struct HiFiCOLOR(i32);
#[repr(C)]
pub struct ICMENUMPROCA(i32);
#[repr(C)]
pub struct ICMENUMPROCW(i32);
#[repr(C)]
pub struct ICM_COMMAND(i32);
#[repr(transparent)]
pub struct IDeviceModelPlugIn(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGamutMapModelPlugIn(pub *mut ::core::ffi::c_void);
pub const INDEX_DONT_CARE: u32 = 0u32;
pub const INTENT_ABSOLUTE_COLORIMETRIC: u32 = 3u32;
pub const INTENT_PERCEPTUAL: u32 = 0u32;
pub const INTENT_RELATIVE_COLORIMETRIC: u32 = 1u32;
pub const INTENT_SATURATION: u32 = 2u32;
#[repr(C)]
pub struct JChColorF(i32);
#[repr(C)]
pub struct JabColorF(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct LOGCOLORSPACEA(i32);
#[cfg(feature = "Win32_Graphics_Gdi")]
#[repr(C)]
pub struct LOGCOLORSPACEW(i32);
#[repr(C)]
pub struct LPBMCALLBACKFN(i32);
#[repr(C)]
pub struct LabCOLOR(i32);
pub const MAX_COLOR_CHANNELS: u32 = 8u32;
#[repr(C)]
pub struct NAMEDCOLOR(i32);
#[repr(C)]
pub struct NAMED_PROFILE_INFO(i32);
pub const NORMAL_MODE: u32 = 2u32;
#[repr(C)]
pub struct PCMSCALLBACKA(i32);
#[repr(C)]
pub struct PCMSCALLBACKW(i32);
pub const PRESERVEBLACK: u32 = 1048576u32;
#[repr(C)]
pub struct PROFILE(i32);
#[cfg(feature = "Win32_Graphics_Gdi")]
#[repr(C)]
pub struct PROFILEHEADER(i32);
pub const PROFILE_FILENAME: u32 = 1u32;
pub const PROFILE_MEMBUFFER: u32 = 2u32;
pub const PROFILE_READ: u32 = 1u32;
pub const PROFILE_READWRITE: u32 = 2u32;
pub const PROOF_MODE: u32 = 1u32;
#[repr(C)]
pub struct PrimaryJabColors(i32);
#[repr(C)]
pub struct PrimaryXYZColors(i32);
pub const RESERVED: u32 = 2147483648u32;
#[repr(C)]
pub struct RGBCOLOR(i32);
pub const SEQUENTIAL_TRANSFORM: u32 = 2155872256u32;
pub const USE_RELATIVE_COLORIMETRIC: u32 = 131072u32;
pub const WCS_ALWAYS: u32 = 2097152u32;
pub const WCS_DEFAULT: i32 = 0i32;
#[repr(C)]
pub struct WCS_DEVICE_CAPABILITIES_TYPE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WCS_DEVICE_MHC2_CAPABILITIES(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WCS_DEVICE_VCGT_CAPABILITIES(i32);
pub const WCS_ICCONLY: i32 = 65536i32;
#[repr(C)]
pub struct WCS_PROFILE_MANAGEMENT_SCOPE(i32);
#[repr(C)]
pub struct XYZCOLOR(i32);
#[repr(C)]
pub struct XYZColorF(i32);
#[repr(C)]
pub struct YxyCOLOR(i32);
