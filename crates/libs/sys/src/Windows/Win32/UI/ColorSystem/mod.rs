#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AssociateColorProfileWithDeviceA(pmachinename: ::windows_sys::core::PCSTR, pprofilename: ::windows_sys::core::PCSTR, pdevicename: ::windows_sys::core::PCSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AssociateColorProfileWithDeviceW(pmachinename: ::windows_sys::core::PCWSTR, pprofilename: ::windows_sys::core::PCWSTR, pdevicename: ::windows_sys::core::PCWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CMCheckColors(hcmtransform: isize, lpainputcolors: *const COLOR, ncolors: u32, ctinput: COLORTYPE, lparesult: *mut u8) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn CMCheckColorsInGamut(hcmtransform: isize, lpargbtriple: *const super::super::Graphics::Gdi::RGBTRIPLE, lparesult: *mut u8, ncount: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CMCheckRGBs(hcmtransform: isize, lpsrcbits: *const ::core::ffi::c_void, bminput: BMFORMAT, dwwidth: u32, dwheight: u32, dwstride: u32, lparesult: *mut u8, pfncallback: LPBMCALLBACKFN, ulcallbackdata: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CMConvertColorNameToIndex(hprofile: isize, pacolorname: *const *const i8, paindex: *mut u32, dwcount: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CMConvertIndexToColorName(hprofile: isize, paindex: *const u32, pacolorname: *mut *mut i8, dwcount: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CMCreateDeviceLinkProfile(pahprofiles: *const isize, nprofiles: u32, padwintents: *const u32, nintents: u32, dwflags: u32, lpprofiledata: *mut *mut u8) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
    pub fn CMCreateMultiProfileTransform(pahprofiles: *const isize, nprofiles: u32, padwintents: *const u32, nintents: u32, dwflags: u32) -> isize;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn CMCreateProfile(lpcolorspace: *mut LOGCOLORSPACEA, lpprofiledata: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn CMCreateProfileW(lpcolorspace: *mut LOGCOLORSPACEW, lpprofiledata: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn CMCreateTransform(lpcolorspace: *const LOGCOLORSPACEA, lpdevcharacter: *const ::core::ffi::c_void, lptargetdevcharacter: *const ::core::ffi::c_void) -> isize;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn CMCreateTransformExt(lpcolorspace: *const LOGCOLORSPACEA, lpdevcharacter: *const ::core::ffi::c_void, lptargetdevcharacter: *const ::core::ffi::c_void, dwflags: u32) -> isize;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn CMCreateTransformExtW(lpcolorspace: *const LOGCOLORSPACEW, lpdevcharacter: *const ::core::ffi::c_void, lptargetdevcharacter: *const ::core::ffi::c_void, dwflags: u32) -> isize;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn CMCreateTransformW(lpcolorspace: *const LOGCOLORSPACEW, lpdevcharacter: *const ::core::ffi::c_void, lptargetdevcharacter: *const ::core::ffi::c_void) -> isize;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CMDeleteTransform(hcmtransform: isize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
    pub fn CMGetInfo(dwinfo: u32) -> u32;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CMGetNamedProfileInfo(hprofile: isize, pnamedprofileinfo: *mut NAMED_PROFILE_INFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CMIsProfileValid(hprofile: isize, lpbvalid: *mut i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CMTranslateColors(hcmtransform: isize, lpainputcolors: *const COLOR, ncolors: u32, ctinput: COLORTYPE, lpaoutputcolors: *mut COLOR, ctoutput: COLORTYPE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CMTranslateRGB(hcmtransform: isize, colorref: u32, lpcolorref: *mut u32, dwflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CMTranslateRGBs(hcmtransform: isize, lpsrcbits: *const ::core::ffi::c_void, bminput: BMFORMAT, dwwidth: u32, dwheight: u32, dwstride: u32, lpdestbits: *mut ::core::ffi::c_void, bmoutput: BMFORMAT, dwtranslatedirection: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CMTranslateRGBsExt(hcmtransform: isize, lpsrcbits: *const ::core::ffi::c_void, bminput: BMFORMAT, dwwidth: u32, dwheight: u32, dwinputstride: u32, lpdestbits: *mut ::core::ffi::c_void, bmoutput: BMFORMAT, dwoutputstride: u32, lpfncallback: LPBMCALLBACKFN, ulcallbackdata: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CheckBitmapBits(hcolortransform: isize, psrcbits: *const ::core::ffi::c_void, bminput: BMFORMAT, dwwidth: u32, dwheight: u32, dwstride: u32, paresult: *mut u8, pfncallback: LPBMCALLBACKFN, lpcallbackdata: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CheckColors(hcolortransform: isize, painputcolors: *const COLOR, ncolors: u32, ctinput: COLORTYPE, paresult: *mut u8) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn CheckColorsInGamut(hdc: super::super::Graphics::Gdi::HDC, lprgbtriple: *const super::super::Graphics::Gdi::RGBTRIPLE, dlpbuffer: *mut ::core::ffi::c_void, ncount: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CloseColorProfile(hprofile: isize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn ColorCorrectPalette(hdc: super::super::Graphics::Gdi::HDC, hpal: super::super::Graphics::Gdi::HPALETTE, defirst: u32, num: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn ColorMatchToTarget(hdc: super::super::Graphics::Gdi::HDC, hdctarget: super::super::Graphics::Gdi::HDC, action: COLOR_MATCH_TO_TARGET_ACTION) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ColorProfileAddDisplayAssociation(scope: WCS_PROFILE_MANAGEMENT_SCOPE, profilename: ::windows_sys::core::PCWSTR, targetadapterid: super::super::Foundation::LUID, sourceid: u32, setasdefault: super::super::Foundation::BOOL, associateasadvancedcolor: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ColorProfileGetDisplayDefault(scope: WCS_PROFILE_MANAGEMENT_SCOPE, targetadapterid: super::super::Foundation::LUID, sourceid: u32, profiletype: COLORPROFILETYPE, profilesubtype: COLORPROFILESUBTYPE, profilename: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ColorProfileGetDisplayList(scope: WCS_PROFILE_MANAGEMENT_SCOPE, targetadapterid: super::super::Foundation::LUID, sourceid: u32, profilelist: *mut *mut ::windows_sys::core::PWSTR, profilecount: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ColorProfileGetDisplayUserScope(targetadapterid: super::super::Foundation::LUID, sourceid: u32, scope: *mut WCS_PROFILE_MANAGEMENT_SCOPE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ColorProfileRemoveDisplayAssociation(scope: WCS_PROFILE_MANAGEMENT_SCOPE, profilename: ::windows_sys::core::PCWSTR, targetadapterid: super::super::Foundation::LUID, sourceid: u32, dissociateadvancedcolor: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ColorProfileSetDisplayDefaultAssociation(scope: WCS_PROFILE_MANAGEMENT_SCOPE, profilename: ::windows_sys::core::PCWSTR, profiletype: COLORPROFILETYPE, profilesubtype: COLORPROFILESUBTYPE, targetadapterid: super::super::Foundation::LUID, sourceid: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConvertColorNameToIndex(hprofile: isize, pacolorname: *const *const i8, paindex: *mut u32, dwcount: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ConvertIndexToColorName(hprofile: isize, paindex: *const u32, pacolorname: *mut *mut i8, dwcount: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn CreateColorSpaceA(lplcs: *const LOGCOLORSPACEA) -> HCOLORSPACE;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn CreateColorSpaceW(lplcs: *const LOGCOLORSPACEW) -> HCOLORSPACE;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn CreateColorTransformA(plogcolorspace: *const LOGCOLORSPACEA, hdestprofile: isize, htargetprofile: isize, dwflags: u32) -> isize;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn CreateColorTransformW(plogcolorspace: *const LOGCOLORSPACEW, hdestprofile: isize, htargetprofile: isize, dwflags: u32) -> isize;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateDeviceLinkProfile(hprofile: *const isize, nprofiles: u32, padwintent: *const u32, nintents: u32, dwflags: u32, pprofiledata: *mut *mut u8, indexpreferredcmm: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
    pub fn CreateMultiProfileTransform(pahprofiles: *const isize, nprofiles: u32, padwintent: *const u32, nintents: u32, dwflags: u32, indexpreferredcmm: u32) -> isize;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn CreateProfileFromLogColorSpaceA(plogcolorspace: *const LOGCOLORSPACEA, pprofile: *mut *mut u8) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn CreateProfileFromLogColorSpaceW(plogcolorspace: *const LOGCOLORSPACEW, pprofile: *mut *mut u8) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteColorSpace(hcs: HCOLORSPACE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteColorTransform(hxform: isize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DisassociateColorProfileFromDeviceA(pmachinename: ::windows_sys::core::PCSTR, pprofilename: ::windows_sys::core::PCSTR, pdevicename: ::windows_sys::core::PCSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DisassociateColorProfileFromDeviceW(pmachinename: ::windows_sys::core::PCWSTR, pprofilename: ::windows_sys::core::PCWSTR, pdevicename: ::windows_sys::core::PCWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumColorProfilesA(pmachinename: ::windows_sys::core::PCSTR, penumrecord: *const ENUMTYPEA, penumerationbuffer: *mut u8, pdwsizeofenumerationbuffer: *mut u32, pnprofiles: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumColorProfilesW(pmachinename: ::windows_sys::core::PCWSTR, penumrecord: *const ENUMTYPEW, penumerationbuffer: *mut u8, pdwsizeofenumerationbuffer: *mut u32, pnprofiles: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn EnumICMProfilesA(hdc: super::super::Graphics::Gdi::HDC, proc: ICMENUMPROCA, param2: super::super::Foundation::LPARAM) -> i32;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn EnumICMProfilesW(hdc: super::super::Graphics::Gdi::HDC, proc: ICMENUMPROCW, param2: super::super::Foundation::LPARAM) -> i32;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
    pub fn GetCMMInfo(hcolortransform: isize, param1: u32) -> u32;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetColorDirectoryA(pmachinename: ::windows_sys::core::PCSTR, pbuffer: ::windows_sys::core::PSTR, pdwsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetColorDirectoryW(pmachinename: ::windows_sys::core::PCWSTR, pbuffer: ::windows_sys::core::PWSTR, pdwsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetColorProfileElement(hprofile: isize, tag: u32, dwoffset: u32, pcbelement: *mut u32, pelement: *mut ::core::ffi::c_void, pbreference: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetColorProfileElementTag(hprofile: isize, dwindex: u32, ptag: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetColorProfileFromHandle(hprofile: isize, pprofile: *mut u8, pcbprofile: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn GetColorProfileHeader(hprofile: isize, pheader: *mut PROFILEHEADER) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn GetColorSpace(hdc: super::super::Graphics::Gdi::HDC) -> HCOLORSPACE;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCountColorProfileElements(hprofile: isize, pnelementcount: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn GetDeviceGammaRamp(hdc: super::super::Graphics::Gdi::HDC, lpramp: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn GetICMProfileA(hdc: super::super::Graphics::Gdi::HDC, pbufsize: *mut u32, pszfilename: ::windows_sys::core::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn GetICMProfileW(hdc: super::super::Graphics::Gdi::HDC, pbufsize: *mut u32, pszfilename: ::windows_sys::core::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn GetLogColorSpaceA(hcolorspace: HCOLORSPACE, lpbuffer: *mut LOGCOLORSPACEA, nsize: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn GetLogColorSpaceW(hcolorspace: HCOLORSPACE, lpbuffer: *mut LOGCOLORSPACEW, nsize: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNamedProfileInfo(hprofile: isize, pnamedprofileinfo: *mut NAMED_PROFILE_INFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPS2ColorRenderingDictionary(hprofile: isize, dwintent: u32, pps2colorrenderingdictionary: *mut u8, pcbps2colorrenderingdictionary: *mut u32, pbbinary: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPS2ColorRenderingIntent(hprofile: isize, dwintent: u32, pbuffer: *mut u8, pcbps2colorrenderingintent: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPS2ColorSpaceArray(hprofile: isize, dwintent: u32, dwcsatype: u32, pps2colorspacearray: *mut u8, pcbps2colorspacearray: *mut u32, pbbinary: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetStandardColorSpaceProfileA(pmachinename: ::windows_sys::core::PCSTR, dwscs: u32, pbuffer: ::windows_sys::core::PSTR, pcbsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetStandardColorSpaceProfileW(pmachinename: ::windows_sys::core::PCWSTR, dwscs: u32, pbuffer: ::windows_sys::core::PWSTR, pcbsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InstallColorProfileA(pmachinename: ::windows_sys::core::PCSTR, pprofilename: ::windows_sys::core::PCSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InstallColorProfileW(pmachinename: ::windows_sys::core::PCWSTR, pprofilename: ::windows_sys::core::PCWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsColorProfileTagPresent(hprofile: isize, tag: u32, pbpresent: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsColorProfileValid(hprofile: isize, pbvalid: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
    pub fn OpenColorProfileA(pprofile: *const PROFILE, dwdesiredaccess: u32, dwsharemode: u32, dwcreationmode: u32) -> isize;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
    pub fn OpenColorProfileW(pprofile: *const PROFILE, dwdesiredaccess: u32, dwsharemode: u32, dwcreationmode: u32) -> isize;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterCMMA(pmachinename: ::windows_sys::core::PCSTR, cmmid: u32, pcmmdll: ::windows_sys::core::PCSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterCMMW(pmachinename: ::windows_sys::core::PCWSTR, cmmid: u32, pcmmdll: ::windows_sys::core::PCWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SelectCMM(dwcmmtype: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetColorProfileElement(hprofile: isize, tag: u32, dwoffset: u32, pcbelement: *const u32, pelement: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetColorProfileElementReference(hprofile: isize, newtag: u32, reftag: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetColorProfileElementSize(hprofile: isize, tagtype: u32, pcbelement: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn SetColorProfileHeader(hprofile: isize, pheader: *const PROFILEHEADER) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn SetColorSpace(hdc: super::super::Graphics::Gdi::HDC, hcs: HCOLORSPACE) -> HCOLORSPACE;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn SetDeviceGammaRamp(hdc: super::super::Graphics::Gdi::HDC, lpramp: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn SetICMMode(hdc: super::super::Graphics::Gdi::HDC, mode: ICM_MODE) -> i32;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn SetICMProfileA(hdc: super::super::Graphics::Gdi::HDC, lpfilename: ::windows_sys::core::PCSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn SetICMProfileW(hdc: super::super::Graphics::Gdi::HDC, lpfilename: ::windows_sys::core::PCWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetStandardColorSpaceProfileA(pmachinename: ::windows_sys::core::PCSTR, dwprofileid: u32, pprofilename: ::windows_sys::core::PCSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetStandardColorSpaceProfileW(pmachinename: ::windows_sys::core::PCWSTR, dwprofileid: u32, pprofilename: ::windows_sys::core::PCWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn SetupColorMatchingA(pcms: *mut COLORMATCHSETUPA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn SetupColorMatchingW(pcms: *mut COLORMATCHSETUPW) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TranslateBitmapBits(hcolortransform: isize, psrcbits: *const ::core::ffi::c_void, bminput: BMFORMAT, dwwidth: u32, dwheight: u32, dwinputstride: u32, pdestbits: *mut ::core::ffi::c_void, bmoutput: BMFORMAT, dwoutputstride: u32, pfncallback: LPBMCALLBACKFN, ulcallbackdata: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TranslateColors(hcolortransform: isize, painputcolors: *const COLOR, ncolors: u32, ctinput: COLORTYPE, paoutputcolors: *mut COLOR, ctoutput: COLORTYPE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UninstallColorProfileA(pmachinename: ::windows_sys::core::PCSTR, pprofilename: ::windows_sys::core::PCSTR, bdelete: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UninstallColorProfileW(pmachinename: ::windows_sys::core::PCWSTR, pprofilename: ::windows_sys::core::PCWSTR, bdelete: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnregisterCMMA(pmachinename: ::windows_sys::core::PCSTR, cmmid: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnregisterCMMW(pmachinename: ::windows_sys::core::PCWSTR, cmmid: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UpdateICMRegKeyA(reserved: u32, lpszcmid: ::windows_sys::core::PCSTR, lpszfilename: ::windows_sys::core::PCSTR, command: ICM_COMMAND) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UpdateICMRegKeyW(reserved: u32, lpszcmid: ::windows_sys::core::PCWSTR, lpszfilename: ::windows_sys::core::PCWSTR, command: ICM_COMMAND) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WcsAssociateColorProfileWithDevice(scope: WCS_PROFILE_MANAGEMENT_SCOPE, pprofilename: ::windows_sys::core::PCWSTR, pdevicename: ::windows_sys::core::PCWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WcsCheckColors(hcolortransform: isize, ncolors: u32, ninputchannels: u32, cdtinput: COLORDATATYPE, cbinput: u32, pinputdata: *const ::core::ffi::c_void, paresult: *mut u8) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
    pub fn WcsCreateIccProfile(hwcsprofile: isize, dwoptions: u32) -> isize;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WcsDisassociateColorProfileFromDevice(scope: WCS_PROFILE_MANAGEMENT_SCOPE, pprofilename: ::windows_sys::core::PCWSTR, pdevicename: ::windows_sys::core::PCWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WcsEnumColorProfiles(scope: WCS_PROFILE_MANAGEMENT_SCOPE, penumrecord: *const ENUMTYPEW, pbuffer: *mut u8, dwsize: u32, pnprofiles: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WcsEnumColorProfilesSize(scope: WCS_PROFILE_MANAGEMENT_SCOPE, penumrecord: *const ENUMTYPEW, pdwsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WcsGetCalibrationManagementState(pbisenabled: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WcsGetDefaultColorProfile(scope: WCS_PROFILE_MANAGEMENT_SCOPE, pdevicename: ::windows_sys::core::PCWSTR, cptcolorprofiletype: COLORPROFILETYPE, cpstcolorprofilesubtype: COLORPROFILESUBTYPE, dwprofileid: u32, cbprofilename: u32, pprofilename: ::windows_sys::core::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WcsGetDefaultColorProfileSize(scope: WCS_PROFILE_MANAGEMENT_SCOPE, pdevicename: ::windows_sys::core::PCWSTR, cptcolorprofiletype: COLORPROFILETYPE, cpstcolorprofilesubtype: COLORPROFILESUBTYPE, dwprofileid: u32, pcbprofilename: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WcsGetDefaultRenderingIntent(scope: WCS_PROFILE_MANAGEMENT_SCOPE, pdwrenderingintent: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WcsGetUsePerUserProfiles(pdevicename: ::windows_sys::core::PCWSTR, dwdeviceclass: u32, puseperuserprofiles: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
    pub fn WcsOpenColorProfileA(pcdmpprofile: *const PROFILE, pcampprofile: *const PROFILE, pgmmpprofile: *const PROFILE, dwdesireaccess: u32, dwsharemode: u32, dwcreationmode: u32, dwflags: u32) -> isize;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
    pub fn WcsOpenColorProfileW(pcdmpprofile: *const PROFILE, pcampprofile: *const PROFILE, pgmmpprofile: *const PROFILE, dwdesireaccess: u32, dwsharemode: u32, dwcreationmode: u32, dwflags: u32) -> isize;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WcsSetCalibrationManagementState(bisenabled: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WcsSetDefaultColorProfile(scope: WCS_PROFILE_MANAGEMENT_SCOPE, pdevicename: ::windows_sys::core::PCWSTR, cptcolorprofiletype: COLORPROFILETYPE, cpstcolorprofilesubtype: COLORPROFILESUBTYPE, dwprofileid: u32, pprofilename: ::windows_sys::core::PCWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WcsSetDefaultRenderingIntent(scope: WCS_PROFILE_MANAGEMENT_SCOPE, dwrenderingintent: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WcsSetUsePerUserProfiles(pdevicename: ::windows_sys::core::PCWSTR, dwdeviceclass: u32, useperuserprofiles: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WcsTranslateColors(hcolortransform: isize, ncolors: u32, ninputchannels: u32, cdtinput: COLORDATATYPE, cbinput: u32, pinputdata: *const ::core::ffi::c_void, noutputchannels: u32, cdtoutput: COLORDATATYPE, cboutput: u32, poutputdata: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const ATTRIB_MATTE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const ATTRIB_TRANSPARENCY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const BEST_MODE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub type BMFORMAT = i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const BM_x555RGB: BMFORMAT = 0i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const BM_x555XYZ: BMFORMAT = 257i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const BM_x555Yxy: BMFORMAT = 258i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const BM_x555Lab: BMFORMAT = 259i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const BM_x555G3CH: BMFORMAT = 260i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const BM_RGBTRIPLETS: BMFORMAT = 2i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const BM_BGRTRIPLETS: BMFORMAT = 4i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const BM_XYZTRIPLETS: BMFORMAT = 513i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const BM_YxyTRIPLETS: BMFORMAT = 514i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const BM_LabTRIPLETS: BMFORMAT = 515i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const BM_G3CHTRIPLETS: BMFORMAT = 516i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const BM_5CHANNEL: BMFORMAT = 517i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const BM_6CHANNEL: BMFORMAT = 518i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const BM_7CHANNEL: BMFORMAT = 519i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const BM_8CHANNEL: BMFORMAT = 520i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const BM_GRAY: BMFORMAT = 521i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const BM_xRGBQUADS: BMFORMAT = 8i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const BM_xBGRQUADS: BMFORMAT = 16i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const BM_xG3CHQUADS: BMFORMAT = 772i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const BM_KYMCQUADS: BMFORMAT = 773i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const BM_CMYKQUADS: BMFORMAT = 32i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const BM_10b_RGB: BMFORMAT = 9i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const BM_10b_XYZ: BMFORMAT = 1025i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const BM_10b_Yxy: BMFORMAT = 1026i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const BM_10b_Lab: BMFORMAT = 1027i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const BM_10b_G3CH: BMFORMAT = 1028i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const BM_NAMED_INDEX: BMFORMAT = 1029i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const BM_16b_RGB: BMFORMAT = 10i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const BM_16b_XYZ: BMFORMAT = 1281i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const BM_16b_Yxy: BMFORMAT = 1282i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const BM_16b_Lab: BMFORMAT = 1283i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const BM_16b_G3CH: BMFORMAT = 1284i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const BM_16b_GRAY: BMFORMAT = 1285i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const BM_565RGB: BMFORMAT = 1i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const BM_32b_scRGB: BMFORMAT = 1537i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const BM_32b_scARGB: BMFORMAT = 1538i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const BM_S2DOT13FIXED_scRGB: BMFORMAT = 1539i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const BM_S2DOT13FIXED_scARGB: BMFORMAT = 1540i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const BM_R10G10B10A2: BMFORMAT = 1793i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const BM_R10G10B10A2_XR: BMFORMAT = 1794i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const BM_R16G16B16A16_FLOAT: BMFORMAT = 1795i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct BlackInformation {
    pub fBlackOnly: super::super::Foundation::BOOL,
    pub blackWeight: f32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for BlackInformation {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for BlackInformation {
    fn clone(&self) -> Self {
        *self
    }
}
pub const CATID_WcsPlugin: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2696151776, data2: 33344, data3: 16479, data4: [138, 22, 138, 91, 77, 242, 240, 221] };
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const CMM_DESCRIPTION: u32 = 5u32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const CMM_DLL_VERSION: u32 = 3u32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const CMM_DRIVER_VERSION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const CMM_FROM_PROFILE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const CMM_IDENT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const CMM_LOGOICON: u32 = 6u32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const CMM_VERSION: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const CMM_WIN_VERSION: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const CMS_BACKWARD: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const CMS_DISABLEICM: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const CMS_DISABLEINTENT: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const CMS_DISABLERENDERINTENT: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const CMS_ENABLEPROOFING: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const CMS_FORWARD: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const CMS_MONITOROVERFLOW: i32 = -2147483648i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const CMS_PRINTEROVERFLOW: i32 = 1073741824i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const CMS_SETMONITORPROFILE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const CMS_SETPRINTERPROFILE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const CMS_SETPROOFINTENT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const CMS_SETRENDERINTENT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const CMS_SETTARGETPROFILE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const CMS_TARGETOVERFLOW: i32 = 536870912i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const CMS_USEAPPLYCALLBACK: u32 = 256u32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const CMS_USEDESCRIPTION: u32 = 512u32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const CMS_USEHOOK: u32 = 128u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub struct CMYKCOLOR {
    pub cyan: u16,
    pub magenta: u16,
    pub yellow: u16,
    pub black: u16,
}
impl ::core::marker::Copy for CMYKCOLOR {}
impl ::core::clone::Clone for CMYKCOLOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
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
impl ::core::marker::Copy for COLOR {}
impl ::core::clone::Clone for COLOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub struct COLOR_0 {
    pub reserved1: u32,
    pub reserved2: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for COLOR_0 {}
impl ::core::clone::Clone for COLOR_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub type COLORDATATYPE = i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const COLOR_BYTE: COLORDATATYPE = 1i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const COLOR_WORD: COLORDATATYPE = 2i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const COLOR_FLOAT: COLORDATATYPE = 3i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const COLOR_S2DOT13FIXED: COLORDATATYPE = 4i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const COLOR_10b_R10G10B10A2: COLORDATATYPE = 5i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const COLOR_10b_R10G10B10A2_XR: COLORDATATYPE = 6i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const COLOR_FLOAT16: COLORDATATYPE = 7i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct COLORMATCHSETUPA {
    pub dwSize: u32,
    pub dwVersion: u32,
    pub dwFlags: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub pSourceName: ::windows_sys::core::PCSTR,
    pub pDisplayName: ::windows_sys::core::PCSTR,
    pub pPrinterName: ::windows_sys::core::PCSTR,
    pub dwRenderIntent: u32,
    pub dwProofingIntent: u32,
    pub pMonitorProfile: ::windows_sys::core::PSTR,
    pub ccMonitorProfile: u32,
    pub pPrinterProfile: ::windows_sys::core::PSTR,
    pub ccPrinterProfile: u32,
    pub pTargetProfile: ::windows_sys::core::PSTR,
    pub ccTargetProfile: u32,
    pub lpfnHook: super::WindowsAndMessaging::DLGPROC,
    pub lParam: super::super::Foundation::LPARAM,
    pub lpfnApplyCallback: PCMSCALLBACKA,
    pub lParamApplyCallback: super::super::Foundation::LPARAM,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for COLORMATCHSETUPA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for COLORMATCHSETUPA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct COLORMATCHSETUPW {
    pub dwSize: u32,
    pub dwVersion: u32,
    pub dwFlags: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub pSourceName: ::windows_sys::core::PCWSTR,
    pub pDisplayName: ::windows_sys::core::PCWSTR,
    pub pPrinterName: ::windows_sys::core::PCWSTR,
    pub dwRenderIntent: u32,
    pub dwProofingIntent: u32,
    pub pMonitorProfile: ::windows_sys::core::PWSTR,
    pub ccMonitorProfile: u32,
    pub pPrinterProfile: ::windows_sys::core::PWSTR,
    pub ccPrinterProfile: u32,
    pub pTargetProfile: ::windows_sys::core::PWSTR,
    pub ccTargetProfile: u32,
    pub lpfnHook: super::WindowsAndMessaging::DLGPROC,
    pub lParam: super::super::Foundation::LPARAM,
    pub lpfnApplyCallback: PCMSCALLBACKW,
    pub lParamApplyCallback: super::super::Foundation::LPARAM,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::marker::Copy for COLORMATCHSETUPW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for COLORMATCHSETUPW {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub type COLORPROFILESUBTYPE = i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const CPST_PERCEPTUAL: COLORPROFILESUBTYPE = 0i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const CPST_RELATIVE_COLORIMETRIC: COLORPROFILESUBTYPE = 1i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const CPST_SATURATION: COLORPROFILESUBTYPE = 2i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const CPST_ABSOLUTE_COLORIMETRIC: COLORPROFILESUBTYPE = 3i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const CPST_NONE: COLORPROFILESUBTYPE = 4i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const CPST_RGB_WORKING_SPACE: COLORPROFILESUBTYPE = 5i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const CPST_CUSTOM_WORKING_SPACE: COLORPROFILESUBTYPE = 6i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const CPST_STANDARD_DISPLAY_COLOR_MODE: COLORPROFILESUBTYPE = 7i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const CPST_EXTENDED_DISPLAY_COLOR_MODE: COLORPROFILESUBTYPE = 8i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub type COLORPROFILETYPE = i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const CPT_ICC: COLORPROFILETYPE = 0i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const CPT_DMP: COLORPROFILETYPE = 1i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const CPT_CAMP: COLORPROFILETYPE = 2i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const CPT_GMMP: COLORPROFILETYPE = 3i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub type COLORTYPE = i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const COLOR_GRAY: COLORTYPE = 1i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const COLOR_RGB: COLORTYPE = 2i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const COLOR_XYZ: COLORTYPE = 3i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const COLOR_Yxy: COLORTYPE = 4i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const COLOR_Lab: COLORTYPE = 5i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const COLOR_3_CHANNEL: COLORTYPE = 6i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const COLOR_CMYK: COLORTYPE = 7i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const COLOR_5_CHANNEL: COLORTYPE = 8i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const COLOR_6_CHANNEL: COLORTYPE = 9i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const COLOR_7_CHANNEL: COLORTYPE = 10i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const COLOR_8_CHANNEL: COLORTYPE = 11i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const COLOR_NAMED: COLORTYPE = 12i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub type COLOR_MATCH_TO_TARGET_ACTION = i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const CS_ENABLE: COLOR_MATCH_TO_TARGET_ACTION = 1i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const CS_DISABLE: COLOR_MATCH_TO_TARGET_ACTION = 2i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const CS_DELETE_TRANSFORM: COLOR_MATCH_TO_TARGET_ACTION = 3i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const COLOR_MATCH_VERSION: u32 = 512u32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const CSA_A: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const CSA_ABC: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const CSA_CMYK: u32 = 7u32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const CSA_DEF: u32 = 3u32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const CSA_DEFG: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const CSA_GRAY: u32 = 5u32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const CSA_Lab: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const CSA_RGB: u32 = 6u32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const DONT_USE_EMBEDDED_WCS_PROFILES: i32 = 1i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct EMRCREATECOLORSPACE {
    pub emr: super::super::Graphics::Gdi::EMR,
    pub ihCS: u32,
    pub lcs: LOGCOLORSPACEA,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for EMRCREATECOLORSPACE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for EMRCREATECOLORSPACE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub struct EMRCREATECOLORSPACEW {
    pub emr: super::super::Graphics::Gdi::EMR,
    pub ihCS: u32,
    pub lcs: LOGCOLORSPACEW,
    pub dwFlags: u32,
    pub cbData: u32,
    pub Data: [u8; 1],
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::marker::Copy for EMRCREATECOLORSPACEW {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::clone::Clone for EMRCREATECOLORSPACEW {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const ENABLE_GAMUT_CHECKING: u32 = 65536u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub struct ENUMTYPEA {
    pub dwSize: u32,
    pub dwVersion: u32,
    pub dwFields: u32,
    pub pDeviceName: ::windows_sys::core::PCSTR,
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
impl ::core::marker::Copy for ENUMTYPEA {}
impl ::core::clone::Clone for ENUMTYPEA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub struct ENUMTYPEW {
    pub dwSize: u32,
    pub dwVersion: u32,
    pub dwFields: u32,
    pub pDeviceName: ::windows_sys::core::PCWSTR,
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
impl ::core::marker::Copy for ENUMTYPEW {}
impl ::core::clone::Clone for ENUMTYPEW {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const ENUM_TYPE_VERSION: u32 = 768u32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const ET_ATTRIBUTES: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const ET_CLASS: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const ET_CMMTYPE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const ET_CONNECTIONSPACE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const ET_CREATOR: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const ET_DATACOLORSPACE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const ET_DEVICECLASS: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const ET_DEVICENAME: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const ET_DITHERMODE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const ET_EXTENDEDDISPLAYCOLOR: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const ET_MANUFACTURER: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const ET_MEDIATYPE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const ET_MODEL: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const ET_PLATFORM: u32 = 512u32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const ET_PROFILEFLAGS: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const ET_RENDERINGINTENT: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const ET_RESOLUTION: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const ET_SIGNATURE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const ET_STANDARDDISPLAYCOLOR: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const FAST_TRANSLATE: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const FLAG_DEPENDENTONDATA: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const FLAG_EMBEDDEDPROFILE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const FLAG_ENABLE_CHROMATIC_ADAPTATION: u32 = 33554432u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub struct GENERIC3CHANNEL {
    pub ch1: u16,
    pub ch2: u16,
    pub ch3: u16,
}
impl ::core::marker::Copy for GENERIC3CHANNEL {}
impl ::core::clone::Clone for GENERIC3CHANNEL {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub struct GRAYCOLOR {
    pub gray: u16,
}
impl ::core::marker::Copy for GRAYCOLOR {}
impl ::core::clone::Clone for GRAYCOLOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub struct GamutBoundaryDescription {
    pub pPrimaries: *mut PrimaryJabColors,
    pub cNeutralSamples: u32,
    pub pNeutralSamples: *mut JabColorF,
    pub pReferenceShell: *mut GamutShell,
    pub pPlausibleShell: *mut GamutShell,
    pub pPossibleShell: *mut GamutShell,
}
impl ::core::marker::Copy for GamutBoundaryDescription {}
impl ::core::clone::Clone for GamutBoundaryDescription {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub struct GamutShell {
    pub JMin: f32,
    pub JMax: f32,
    pub cVertices: u32,
    pub cTriangles: u32,
    pub pVertices: *mut JabColorF,
    pub pTriangles: *mut GamutShellTriangle,
}
impl ::core::marker::Copy for GamutShell {}
impl ::core::clone::Clone for GamutShell {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub struct GamutShellTriangle {
    pub aVertexIndex: [u32; 3],
}
impl ::core::marker::Copy for GamutShellTriangle {}
impl ::core::clone::Clone for GamutShellTriangle {
    fn clone(&self) -> Self {
        *self
    }
}
pub type HCOLORSPACE = isize;
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub struct HiFiCOLOR {
    pub channel: [u8; 8],
}
impl ::core::marker::Copy for HiFiCOLOR {}
impl ::core::clone::Clone for HiFiCOLOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type ICMENUMPROCA = ::core::option::Option<unsafe extern "system" fn(param0: ::windows_sys::core::PCSTR, param1: super::super::Foundation::LPARAM) -> i32>;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type ICMENUMPROCW = ::core::option::Option<unsafe extern "system" fn(param0: ::windows_sys::core::PCWSTR, param1: super::super::Foundation::LPARAM) -> i32>;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub type ICM_COMMAND = u32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const ICM_ADDPROFILE: ICM_COMMAND = 1u32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const ICM_DELETEPROFILE: ICM_COMMAND = 2u32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const ICM_QUERYPROFILE: ICM_COMMAND = 3u32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const ICM_SETDEFAULTPROFILE: ICM_COMMAND = 4u32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const ICM_REGISTERICMATCHER: ICM_COMMAND = 5u32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const ICM_UNREGISTERICMATCHER: ICM_COMMAND = 6u32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const ICM_QUERYMATCH: ICM_COMMAND = 7u32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub type ICM_MODE = i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const ICM_OFF: ICM_MODE = 1i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const ICM_ON: ICM_MODE = 2i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const ICM_QUERY: ICM_MODE = 3i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const ICM_DONE_OUTSIDEDC: ICM_MODE = 4i32;
pub type IDeviceModelPlugIn = *mut ::core::ffi::c_void;
pub type IGamutMapModelPlugIn = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const INDEX_DONT_CARE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const INTENT_ABSOLUTE_COLORIMETRIC: u32 = 3u32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const INTENT_PERCEPTUAL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const INTENT_RELATIVE_COLORIMETRIC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const INTENT_SATURATION: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub struct JChColorF {
    pub J: f32,
    pub C: f32,
    pub h: f32,
}
impl ::core::marker::Copy for JChColorF {}
impl ::core::clone::Clone for JChColorF {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub struct JabColorF {
    pub J: f32,
    pub a: f32,
    pub b: f32,
}
impl ::core::marker::Copy for JabColorF {}
impl ::core::clone::Clone for JabColorF {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct LOGCOLORSPACEA {
    pub lcsSignature: u32,
    pub lcsVersion: u32,
    pub lcsSize: u32,
    pub lcsCSType: i32,
    pub lcsIntent: i32,
    pub lcsEndpoints: super::super::Graphics::Gdi::CIEXYZTRIPLE,
    pub lcsGammaRed: u32,
    pub lcsGammaGreen: u32,
    pub lcsGammaBlue: u32,
    pub lcsFilename: [super::super::Foundation::CHAR; 260],
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for LOGCOLORSPACEA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for LOGCOLORSPACEA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub struct LOGCOLORSPACEW {
    pub lcsSignature: u32,
    pub lcsVersion: u32,
    pub lcsSize: u32,
    pub lcsCSType: i32,
    pub lcsIntent: i32,
    pub lcsEndpoints: super::super::Graphics::Gdi::CIEXYZTRIPLE,
    pub lcsGammaRed: u32,
    pub lcsGammaGreen: u32,
    pub lcsGammaBlue: u32,
    pub lcsFilename: [u16; 260],
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::marker::Copy for LOGCOLORSPACEW {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::clone::Clone for LOGCOLORSPACEW {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPBMCALLBACKFN = ::core::option::Option<unsafe extern "system" fn(param0: u32, param1: u32, param2: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub struct LabCOLOR {
    pub L: u16,
    pub a: u16,
    pub b: u16,
}
impl ::core::marker::Copy for LabCOLOR {}
impl ::core::clone::Clone for LabCOLOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const MAX_COLOR_CHANNELS: u32 = 8u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub struct NAMEDCOLOR {
    pub dwIndex: u32,
}
impl ::core::marker::Copy for NAMEDCOLOR {}
impl ::core::clone::Clone for NAMEDCOLOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub struct NAMED_PROFILE_INFO {
    pub dwFlags: u32,
    pub dwCount: u32,
    pub dwCountDevCoordinates: u32,
    pub szPrefix: [i8; 32],
    pub szSuffix: [i8; 32],
}
impl ::core::marker::Copy for NAMED_PROFILE_INFO {}
impl ::core::clone::Clone for NAMED_PROFILE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const NORMAL_MODE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub type PCMSCALLBACKA = ::core::option::Option<unsafe extern "system" fn(param0: *mut COLORMATCHSETUPA, param1: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub type PCMSCALLBACKW = ::core::option::Option<unsafe extern "system" fn(param0: *mut COLORMATCHSETUPW, param1: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const PRESERVEBLACK: u32 = 1048576u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub struct PROFILE {
    pub dwType: u32,
    pub pProfileData: *mut ::core::ffi::c_void,
    pub cbDataSize: u32,
}
impl ::core::marker::Copy for PROFILE {}
impl ::core::clone::Clone for PROFILE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
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
    pub phIlluminant: super::super::Graphics::Gdi::CIEXYZ,
    pub phCreator: u32,
    pub phReserved: [u8; 44],
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::marker::Copy for PROFILEHEADER {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::clone::Clone for PROFILEHEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const PROFILE_FILENAME: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const PROFILE_MEMBUFFER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const PROFILE_READ: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const PROFILE_READWRITE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const PROOF_MODE: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub struct PrimaryJabColors {
    pub red: JabColorF,
    pub yellow: JabColorF,
    pub green: JabColorF,
    pub cyan: JabColorF,
    pub blue: JabColorF,
    pub magenta: JabColorF,
    pub black: JabColorF,
    pub white: JabColorF,
}
impl ::core::marker::Copy for PrimaryJabColors {}
impl ::core::clone::Clone for PrimaryJabColors {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub struct PrimaryXYZColors {
    pub red: XYZColorF,
    pub yellow: XYZColorF,
    pub green: XYZColorF,
    pub cyan: XYZColorF,
    pub blue: XYZColorF,
    pub magenta: XYZColorF,
    pub black: XYZColorF,
    pub white: XYZColorF,
}
impl ::core::marker::Copy for PrimaryXYZColors {}
impl ::core::clone::Clone for PrimaryXYZColors {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const RESERVED: u32 = 2147483648u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub struct RGBCOLOR {
    pub red: u16,
    pub green: u16,
    pub blue: u16,
}
impl ::core::marker::Copy for RGBCOLOR {}
impl ::core::clone::Clone for RGBCOLOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const SEQUENTIAL_TRANSFORM: u32 = 2155872256u32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const USE_RELATIVE_COLORIMETRIC: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const WCS_ALWAYS: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const WCS_DEFAULT: i32 = 0i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub type WCS_DEVICE_CAPABILITIES_TYPE = i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const VideoCardGammaTable: WCS_DEVICE_CAPABILITIES_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const MicrosoftHardwareColorV2: WCS_DEVICE_CAPABILITIES_TYPE = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WCS_DEVICE_MHC2_CAPABILITIES {
    pub Size: u32,
    pub SupportsMhc2: super::super::Foundation::BOOL,
    pub RegammaLutEntryCount: u32,
    pub CscXyzMatrixRows: u32,
    pub CscXyzMatrixColumns: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WCS_DEVICE_MHC2_CAPABILITIES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WCS_DEVICE_MHC2_CAPABILITIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WCS_DEVICE_VCGT_CAPABILITIES {
    pub Size: u32,
    pub SupportsVcgt: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WCS_DEVICE_VCGT_CAPABILITIES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WCS_DEVICE_VCGT_CAPABILITIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const WCS_ICCONLY: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub type WCS_PROFILE_MANAGEMENT_SCOPE = i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const WCS_PROFILE_MANAGEMENT_SCOPE_SYSTEM_WIDE: WCS_PROFILE_MANAGEMENT_SCOPE = 0i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const WCS_PROFILE_MANAGEMENT_SCOPE_CURRENT_USER: WCS_PROFILE_MANAGEMENT_SCOPE = 1i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub struct XYZCOLOR {
    pub X: u16,
    pub Y: u16,
    pub Z: u16,
}
impl ::core::marker::Copy for XYZCOLOR {}
impl ::core::clone::Clone for XYZCOLOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub struct XYZColorF {
    pub X: f32,
    pub Y: f32,
    pub Z: f32,
}
impl ::core::marker::Copy for XYZColorF {}
impl ::core::clone::Clone for XYZColorF {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub struct YxyCOLOR {
    pub Y: u16,
    pub x: u16,
    pub y: u16,
}
impl ::core::marker::Copy for YxyCOLOR {}
impl ::core::clone::Clone for YxyCOLOR {
    fn clone(&self) -> Self {
        *self
    }
}
