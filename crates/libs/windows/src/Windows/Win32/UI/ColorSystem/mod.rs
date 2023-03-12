#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AssociateColorProfileWithDeviceA<P0, P1, P2>(pmachinename: P0, pprofilename: P1, pdevicename: P2) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "mscms.dll""system" fn AssociateColorProfileWithDeviceA ( pmachinename : :: windows::core::PCSTR , pprofilename : :: windows::core::PCSTR , pdevicename : :: windows::core::PCSTR ) -> super::super::Foundation:: BOOL );
    AssociateColorProfileWithDeviceA(pmachinename.into_param().abi(), pprofilename.into_param().abi(), pdevicename.into_param().abi())
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AssociateColorProfileWithDeviceW<P0, P1, P2>(pmachinename: P0, pprofilename: P1, pdevicename: P2) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "mscms.dll""system" fn AssociateColorProfileWithDeviceW ( pmachinename : :: windows::core::PCWSTR , pprofilename : :: windows::core::PCWSTR , pdevicename : :: windows::core::PCWSTR ) -> super::super::Foundation:: BOOL );
    AssociateColorProfileWithDeviceW(pmachinename.into_param().abi(), pprofilename.into_param().abi(), pdevicename.into_param().abi())
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CMCheckColors(hcmtransform: isize, lpainputcolors: *const COLOR, ncolors: u32, ctinput: COLORTYPE, lparesult: *mut u8) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "icm32.dll""system" fn CMCheckColors ( hcmtransform : isize , lpainputcolors : *const COLOR , ncolors : u32 , ctinput : COLORTYPE , lparesult : *mut u8 ) -> super::super::Foundation:: BOOL );
    CMCheckColors(hcmtransform, lpainputcolors, ncolors, ctinput, lparesult)
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn CMCheckColorsInGamut(hcmtransform: isize, lpargbtriple: *const super::super::Graphics::Gdi::RGBTRIPLE, lparesult: *mut u8, ncount: u32) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "icm32.dll""system" fn CMCheckColorsInGamut ( hcmtransform : isize , lpargbtriple : *const super::super::Graphics::Gdi:: RGBTRIPLE , lparesult : *mut u8 , ncount : u32 ) -> super::super::Foundation:: BOOL );
    CMCheckColorsInGamut(hcmtransform, lpargbtriple, lparesult, ncount)
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CMCheckRGBs<P0>(hcmtransform: isize, lpsrcbits: *const ::core::ffi::c_void, bminput: BMFORMAT, dwwidth: u32, dwheight: u32, dwstride: u32, lparesult: *mut u8, pfncallback: LPBMCALLBACKFN, ulcallbackdata: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::LPARAM>,
{
    ::windows::imp::link ! ( "icm32.dll""system" fn CMCheckRGBs ( hcmtransform : isize , lpsrcbits : *const ::core::ffi::c_void , bminput : BMFORMAT , dwwidth : u32 , dwheight : u32 , dwstride : u32 , lparesult : *mut u8 , pfncallback : LPBMCALLBACKFN , ulcallbackdata : super::super::Foundation:: LPARAM ) -> super::super::Foundation:: BOOL );
    CMCheckRGBs(hcmtransform, lpsrcbits, bminput, dwwidth, dwheight, dwstride, lparesult, pfncallback, ulcallbackdata.into_param().abi())
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CMConvertColorNameToIndex(hprofile: isize, pacolorname: *const *const i8, paindex: *mut u32, dwcount: u32) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "icm32.dll""system" fn CMConvertColorNameToIndex ( hprofile : isize , pacolorname : *const *const i8 , paindex : *mut u32 , dwcount : u32 ) -> super::super::Foundation:: BOOL );
    CMConvertColorNameToIndex(hprofile, pacolorname, paindex, dwcount)
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CMConvertIndexToColorName(hprofile: isize, paindex: *const u32, pacolorname: *mut *mut i8, dwcount: u32) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "icm32.dll""system" fn CMConvertIndexToColorName ( hprofile : isize , paindex : *const u32 , pacolorname : *mut *mut i8 , dwcount : u32 ) -> super::super::Foundation:: BOOL );
    CMConvertIndexToColorName(hprofile, paindex, pacolorname, dwcount)
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CMCreateDeviceLinkProfile(pahprofiles: &[isize], padwintents: &[u32], dwflags: u32, lpprofiledata: *mut *mut u8) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "icm32.dll""system" fn CMCreateDeviceLinkProfile ( pahprofiles : *const isize , nprofiles : u32 , padwintents : *const u32 , nintents : u32 , dwflags : u32 , lpprofiledata : *mut *mut u8 ) -> super::super::Foundation:: BOOL );
    CMCreateDeviceLinkProfile(::core::mem::transmute(pahprofiles.as_ptr()), pahprofiles.len() as _, ::core::mem::transmute(padwintents.as_ptr()), padwintents.len() as _, dwflags, lpprofiledata)
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
#[inline]
pub unsafe fn CMCreateMultiProfileTransform(pahprofiles: &[isize], padwintents: &[u32], dwflags: u32) -> isize {
    ::windows::imp::link ! ( "icm32.dll""system" fn CMCreateMultiProfileTransform ( pahprofiles : *const isize , nprofiles : u32 , padwintents : *const u32 , nintents : u32 , dwflags : u32 ) -> isize );
    CMCreateMultiProfileTransform(::core::mem::transmute(pahprofiles.as_ptr()), pahprofiles.len() as _, ::core::mem::transmute(padwintents.as_ptr()), padwintents.len() as _, dwflags)
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn CMCreateProfile(lpcolorspace: *mut LOGCOLORSPACEA, lpprofiledata: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "icm32.dll""system" fn CMCreateProfile ( lpcolorspace : *mut LOGCOLORSPACEA , lpprofiledata : *mut *mut ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    CMCreateProfile(lpcolorspace, lpprofiledata)
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn CMCreateProfileW(lpcolorspace: *mut LOGCOLORSPACEW, lpprofiledata: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "icm32.dll""system" fn CMCreateProfileW ( lpcolorspace : *mut LOGCOLORSPACEW , lpprofiledata : *mut *mut ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    CMCreateProfileW(lpcolorspace, lpprofiledata)
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn CMCreateTransform(lpcolorspace: *const LOGCOLORSPACEA, lpdevcharacter: *const ::core::ffi::c_void, lptargetdevcharacter: *const ::core::ffi::c_void) -> isize {
    ::windows::imp::link ! ( "icm32.dll""system" fn CMCreateTransform ( lpcolorspace : *const LOGCOLORSPACEA , lpdevcharacter : *const ::core::ffi::c_void , lptargetdevcharacter : *const ::core::ffi::c_void ) -> isize );
    CMCreateTransform(lpcolorspace, lpdevcharacter, lptargetdevcharacter)
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn CMCreateTransformExt(lpcolorspace: *const LOGCOLORSPACEA, lpdevcharacter: *const ::core::ffi::c_void, lptargetdevcharacter: *const ::core::ffi::c_void, dwflags: u32) -> isize {
    ::windows::imp::link ! ( "icm32.dll""system" fn CMCreateTransformExt ( lpcolorspace : *const LOGCOLORSPACEA , lpdevcharacter : *const ::core::ffi::c_void , lptargetdevcharacter : *const ::core::ffi::c_void , dwflags : u32 ) -> isize );
    CMCreateTransformExt(lpcolorspace, lpdevcharacter, lptargetdevcharacter, dwflags)
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn CMCreateTransformExtW(lpcolorspace: *const LOGCOLORSPACEW, lpdevcharacter: *const ::core::ffi::c_void, lptargetdevcharacter: *const ::core::ffi::c_void, dwflags: u32) -> isize {
    ::windows::imp::link ! ( "icm32.dll""system" fn CMCreateTransformExtW ( lpcolorspace : *const LOGCOLORSPACEW , lpdevcharacter : *const ::core::ffi::c_void , lptargetdevcharacter : *const ::core::ffi::c_void , dwflags : u32 ) -> isize );
    CMCreateTransformExtW(lpcolorspace, lpdevcharacter, lptargetdevcharacter, dwflags)
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn CMCreateTransformW(lpcolorspace: *const LOGCOLORSPACEW, lpdevcharacter: *const ::core::ffi::c_void, lptargetdevcharacter: *const ::core::ffi::c_void) -> isize {
    ::windows::imp::link ! ( "icm32.dll""system" fn CMCreateTransformW ( lpcolorspace : *const LOGCOLORSPACEW , lpdevcharacter : *const ::core::ffi::c_void , lptargetdevcharacter : *const ::core::ffi::c_void ) -> isize );
    CMCreateTransformW(lpcolorspace, lpdevcharacter, lptargetdevcharacter)
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CMDeleteTransform(hcmtransform: isize) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "icm32.dll""system" fn CMDeleteTransform ( hcmtransform : isize ) -> super::super::Foundation:: BOOL );
    CMDeleteTransform(hcmtransform)
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
#[inline]
pub unsafe fn CMGetInfo(dwinfo: u32) -> u32 {
    ::windows::imp::link ! ( "icm32.dll""system" fn CMGetInfo ( dwinfo : u32 ) -> u32 );
    CMGetInfo(dwinfo)
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CMGetNamedProfileInfo(hprofile: isize, pnamedprofileinfo: *mut NAMED_PROFILE_INFO) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "icm32.dll""system" fn CMGetNamedProfileInfo ( hprofile : isize , pnamedprofileinfo : *mut NAMED_PROFILE_INFO ) -> super::super::Foundation:: BOOL );
    CMGetNamedProfileInfo(hprofile, pnamedprofileinfo)
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CMIsProfileValid(hprofile: isize, lpbvalid: *mut i32) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "icm32.dll""system" fn CMIsProfileValid ( hprofile : isize , lpbvalid : *mut i32 ) -> super::super::Foundation:: BOOL );
    CMIsProfileValid(hprofile, lpbvalid)
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CMTranslateColors(hcmtransform: isize, lpainputcolors: *const COLOR, ncolors: u32, ctinput: COLORTYPE, lpaoutputcolors: *mut COLOR, ctoutput: COLORTYPE) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "icm32.dll""system" fn CMTranslateColors ( hcmtransform : isize , lpainputcolors : *const COLOR , ncolors : u32 , ctinput : COLORTYPE , lpaoutputcolors : *mut COLOR , ctoutput : COLORTYPE ) -> super::super::Foundation:: BOOL );
    CMTranslateColors(hcmtransform, lpainputcolors, ncolors, ctinput, lpaoutputcolors, ctoutput)
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CMTranslateRGB<P0>(hcmtransform: isize, colorref: P0, lpcolorref: *mut u32, dwflags: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::COLORREF>,
{
    ::windows::imp::link ! ( "icm32.dll""system" fn CMTranslateRGB ( hcmtransform : isize , colorref : super::super::Foundation:: COLORREF , lpcolorref : *mut u32 , dwflags : u32 ) -> super::super::Foundation:: BOOL );
    CMTranslateRGB(hcmtransform, colorref.into_param().abi(), lpcolorref, dwflags)
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CMTranslateRGBs(hcmtransform: isize, lpsrcbits: *const ::core::ffi::c_void, bminput: BMFORMAT, dwwidth: u32, dwheight: u32, dwstride: u32, lpdestbits: *mut ::core::ffi::c_void, bmoutput: BMFORMAT, dwtranslatedirection: u32) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "icm32.dll""system" fn CMTranslateRGBs ( hcmtransform : isize , lpsrcbits : *const ::core::ffi::c_void , bminput : BMFORMAT , dwwidth : u32 , dwheight : u32 , dwstride : u32 , lpdestbits : *mut ::core::ffi::c_void , bmoutput : BMFORMAT , dwtranslatedirection : u32 ) -> super::super::Foundation:: BOOL );
    CMTranslateRGBs(hcmtransform, lpsrcbits, bminput, dwwidth, dwheight, dwstride, lpdestbits, bmoutput, dwtranslatedirection)
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CMTranslateRGBsExt<P0>(hcmtransform: isize, lpsrcbits: *const ::core::ffi::c_void, bminput: BMFORMAT, dwwidth: u32, dwheight: u32, dwinputstride: u32, lpdestbits: *mut ::core::ffi::c_void, bmoutput: BMFORMAT, dwoutputstride: u32, lpfncallback: LPBMCALLBACKFN, ulcallbackdata: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::LPARAM>,
{
    ::windows::imp::link ! ( "icm32.dll""system" fn CMTranslateRGBsExt ( hcmtransform : isize , lpsrcbits : *const ::core::ffi::c_void , bminput : BMFORMAT , dwwidth : u32 , dwheight : u32 , dwinputstride : u32 , lpdestbits : *mut ::core::ffi::c_void , bmoutput : BMFORMAT , dwoutputstride : u32 , lpfncallback : LPBMCALLBACKFN , ulcallbackdata : super::super::Foundation:: LPARAM ) -> super::super::Foundation:: BOOL );
    CMTranslateRGBsExt(hcmtransform, lpsrcbits, bminput, dwwidth, dwheight, dwinputstride, lpdestbits, bmoutput, dwoutputstride, lpfncallback, ulcallbackdata.into_param().abi())
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CheckBitmapBits<P0>(hcolortransform: isize, psrcbits: *const ::core::ffi::c_void, bminput: BMFORMAT, dwwidth: u32, dwheight: u32, dwstride: u32, paresult: *mut u8, pfncallback: LPBMCALLBACKFN, lpcallbackdata: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::LPARAM>,
{
    ::windows::imp::link ! ( "mscms.dll""system" fn CheckBitmapBits ( hcolortransform : isize , psrcbits : *const ::core::ffi::c_void , bminput : BMFORMAT , dwwidth : u32 , dwheight : u32 , dwstride : u32 , paresult : *mut u8 , pfncallback : LPBMCALLBACKFN , lpcallbackdata : super::super::Foundation:: LPARAM ) -> super::super::Foundation:: BOOL );
    CheckBitmapBits(hcolortransform, psrcbits, bminput, dwwidth, dwheight, dwstride, paresult, pfncallback, lpcallbackdata.into_param().abi())
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CheckColors(hcolortransform: isize, painputcolors: *const COLOR, ncolors: u32, ctinput: COLORTYPE, paresult: *mut u8) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "mscms.dll""system" fn CheckColors ( hcolortransform : isize , painputcolors : *const COLOR , ncolors : u32 , ctinput : COLORTYPE , paresult : *mut u8 ) -> super::super::Foundation:: BOOL );
    CheckColors(hcolortransform, painputcolors, ncolors, ctinput, paresult)
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn CheckColorsInGamut<P0>(hdc: P0, lprgbtriple: *const super::super::Graphics::Gdi::RGBTRIPLE, dlpbuffer: *mut ::core::ffi::c_void, ncount: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Graphics::Gdi::HDC>,
{
    ::windows::imp::link ! ( "gdi32.dll""system" fn CheckColorsInGamut ( hdc : super::super::Graphics::Gdi:: HDC , lprgbtriple : *const super::super::Graphics::Gdi:: RGBTRIPLE , dlpbuffer : *mut ::core::ffi::c_void , ncount : u32 ) -> super::super::Foundation:: BOOL );
    CheckColorsInGamut(hdc.into_param().abi(), lprgbtriple, dlpbuffer, ncount)
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CloseColorProfile(hprofile: isize) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "mscms.dll""system" fn CloseColorProfile ( hprofile : isize ) -> super::super::Foundation:: BOOL );
    CloseColorProfile(hprofile)
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn ColorCorrectPalette<P0, P1>(hdc: P0, hpal: P1, defirst: u32, num: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Graphics::Gdi::HDC>,
    P1: ::windows::core::IntoParam<super::super::Graphics::Gdi::HPALETTE>,
{
    ::windows::imp::link ! ( "gdi32.dll""system" fn ColorCorrectPalette ( hdc : super::super::Graphics::Gdi:: HDC , hpal : super::super::Graphics::Gdi:: HPALETTE , defirst : u32 , num : u32 ) -> super::super::Foundation:: BOOL );
    ColorCorrectPalette(hdc.into_param().abi(), hpal.into_param().abi(), defirst, num)
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn ColorMatchToTarget<P0, P1>(hdc: P0, hdctarget: P1, action: COLOR_MATCH_TO_TARGET_ACTION) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Graphics::Gdi::HDC>,
    P1: ::windows::core::IntoParam<super::super::Graphics::Gdi::HDC>,
{
    ::windows::imp::link ! ( "gdi32.dll""system" fn ColorMatchToTarget ( hdc : super::super::Graphics::Gdi:: HDC , hdctarget : super::super::Graphics::Gdi:: HDC , action : COLOR_MATCH_TO_TARGET_ACTION ) -> super::super::Foundation:: BOOL );
    ColorMatchToTarget(hdc.into_param().abi(), hdctarget.into_param().abi(), action)
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ColorProfileAddDisplayAssociation<P0, P1, P2>(scope: WCS_PROFILE_MANAGEMENT_SCOPE, profilename: P0, targetadapterid: super::super::Foundation::LUID, sourceid: u32, setasdefault: P1, associateasadvancedcolor: P2) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    P2: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows::imp::link ! ( "mscms.dll""system" fn ColorProfileAddDisplayAssociation ( scope : WCS_PROFILE_MANAGEMENT_SCOPE , profilename : :: windows::core::PCWSTR , targetadapterid : super::super::Foundation:: LUID , sourceid : u32 , setasdefault : super::super::Foundation:: BOOL , associateasadvancedcolor : super::super::Foundation:: BOOL ) -> :: windows::core::HRESULT );
    ColorProfileAddDisplayAssociation(scope, profilename.into_param().abi(), ::core::mem::transmute(targetadapterid), sourceid, setasdefault.into_param().abi(), associateasadvancedcolor.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ColorProfileGetDisplayDefault(scope: WCS_PROFILE_MANAGEMENT_SCOPE, targetadapterid: super::super::Foundation::LUID, sourceid: u32, profiletype: COLORPROFILETYPE, profilesubtype: COLORPROFILESUBTYPE) -> ::windows::core::Result<::windows::core::PWSTR> {
    ::windows::imp::link ! ( "mscms.dll""system" fn ColorProfileGetDisplayDefault ( scope : WCS_PROFILE_MANAGEMENT_SCOPE , targetadapterid : super::super::Foundation:: LUID , sourceid : u32 , profiletype : COLORPROFILETYPE , profilesubtype : COLORPROFILESUBTYPE , profilename : *mut :: windows::core::PWSTR ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
    ColorProfileGetDisplayDefault(scope, ::core::mem::transmute(targetadapterid), sourceid, profiletype, profilesubtype, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ColorProfileGetDisplayList(scope: WCS_PROFILE_MANAGEMENT_SCOPE, targetadapterid: super::super::Foundation::LUID, sourceid: u32, profilelist: *mut *mut ::windows::core::PWSTR, profilecount: *mut u32) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "mscms.dll""system" fn ColorProfileGetDisplayList ( scope : WCS_PROFILE_MANAGEMENT_SCOPE , targetadapterid : super::super::Foundation:: LUID , sourceid : u32 , profilelist : *mut *mut :: windows::core::PWSTR , profilecount : *mut u32 ) -> :: windows::core::HRESULT );
    ColorProfileGetDisplayList(scope, ::core::mem::transmute(targetadapterid), sourceid, profilelist, profilecount).ok()
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ColorProfileGetDisplayUserScope(targetadapterid: super::super::Foundation::LUID, sourceid: u32) -> ::windows::core::Result<WCS_PROFILE_MANAGEMENT_SCOPE> {
    ::windows::imp::link ! ( "mscms.dll""system" fn ColorProfileGetDisplayUserScope ( targetadapterid : super::super::Foundation:: LUID , sourceid : u32 , scope : *mut WCS_PROFILE_MANAGEMENT_SCOPE ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<WCS_PROFILE_MANAGEMENT_SCOPE>();
    ColorProfileGetDisplayUserScope(::core::mem::transmute(targetadapterid), sourceid, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ColorProfileRemoveDisplayAssociation<P0, P1>(scope: WCS_PROFILE_MANAGEMENT_SCOPE, profilename: P0, targetadapterid: super::super::Foundation::LUID, sourceid: u32, dissociateadvancedcolor: P1) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows::imp::link ! ( "mscms.dll""system" fn ColorProfileRemoveDisplayAssociation ( scope : WCS_PROFILE_MANAGEMENT_SCOPE , profilename : :: windows::core::PCWSTR , targetadapterid : super::super::Foundation:: LUID , sourceid : u32 , dissociateadvancedcolor : super::super::Foundation:: BOOL ) -> :: windows::core::HRESULT );
    ColorProfileRemoveDisplayAssociation(scope, profilename.into_param().abi(), ::core::mem::transmute(targetadapterid), sourceid, dissociateadvancedcolor.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ColorProfileSetDisplayDefaultAssociation<P0>(scope: WCS_PROFILE_MANAGEMENT_SCOPE, profilename: P0, profiletype: COLORPROFILETYPE, profilesubtype: COLORPROFILESUBTYPE, targetadapterid: super::super::Foundation::LUID, sourceid: u32) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "mscms.dll""system" fn ColorProfileSetDisplayDefaultAssociation ( scope : WCS_PROFILE_MANAGEMENT_SCOPE , profilename : :: windows::core::PCWSTR , profiletype : COLORPROFILETYPE , profilesubtype : COLORPROFILESUBTYPE , targetadapterid : super::super::Foundation:: LUID , sourceid : u32 ) -> :: windows::core::HRESULT );
    ColorProfileSetDisplayDefaultAssociation(scope, profilename.into_param().abi(), profiletype, profilesubtype, ::core::mem::transmute(targetadapterid), sourceid).ok()
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ConvertColorNameToIndex(hprofile: isize, pacolorname: *const *const i8, paindex: *mut u32, dwcount: u32) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "mscms.dll""system" fn ConvertColorNameToIndex ( hprofile : isize , pacolorname : *const *const i8 , paindex : *mut u32 , dwcount : u32 ) -> super::super::Foundation:: BOOL );
    ConvertColorNameToIndex(hprofile, pacolorname, paindex, dwcount)
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ConvertIndexToColorName(hprofile: isize, paindex: *const u32, pacolorname: *mut *mut i8, dwcount: u32) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "mscms.dll""system" fn ConvertIndexToColorName ( hprofile : isize , paindex : *const u32 , pacolorname : *mut *mut i8 , dwcount : u32 ) -> super::super::Foundation:: BOOL );
    ConvertIndexToColorName(hprofile, paindex, pacolorname, dwcount)
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn CreateColorSpaceA(lplcs: *const LOGCOLORSPACEA) -> HCOLORSPACE {
    ::windows::imp::link ! ( "gdi32.dll""system" fn CreateColorSpaceA ( lplcs : *const LOGCOLORSPACEA ) -> HCOLORSPACE );
    CreateColorSpaceA(lplcs)
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn CreateColorSpaceW(lplcs: *const LOGCOLORSPACEW) -> HCOLORSPACE {
    ::windows::imp::link ! ( "gdi32.dll""system" fn CreateColorSpaceW ( lplcs : *const LOGCOLORSPACEW ) -> HCOLORSPACE );
    CreateColorSpaceW(lplcs)
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn CreateColorTransformA(plogcolorspace: *const LOGCOLORSPACEA, hdestprofile: isize, htargetprofile: isize, dwflags: u32) -> isize {
    ::windows::imp::link ! ( "mscms.dll""system" fn CreateColorTransformA ( plogcolorspace : *const LOGCOLORSPACEA , hdestprofile : isize , htargetprofile : isize , dwflags : u32 ) -> isize );
    CreateColorTransformA(plogcolorspace, hdestprofile, htargetprofile, dwflags)
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn CreateColorTransformW(plogcolorspace: *const LOGCOLORSPACEW, hdestprofile: isize, htargetprofile: isize, dwflags: u32) -> isize {
    ::windows::imp::link ! ( "mscms.dll""system" fn CreateColorTransformW ( plogcolorspace : *const LOGCOLORSPACEW , hdestprofile : isize , htargetprofile : isize , dwflags : u32 ) -> isize );
    CreateColorTransformW(plogcolorspace, hdestprofile, htargetprofile, dwflags)
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateDeviceLinkProfile(hprofile: &[isize], padwintent: &[u32], dwflags: u32, pprofiledata: *mut *mut u8, indexpreferredcmm: u32) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "mscms.dll""system" fn CreateDeviceLinkProfile ( hprofile : *const isize , nprofiles : u32 , padwintent : *const u32 , nintents : u32 , dwflags : u32 , pprofiledata : *mut *mut u8 , indexpreferredcmm : u32 ) -> super::super::Foundation:: BOOL );
    CreateDeviceLinkProfile(::core::mem::transmute(hprofile.as_ptr()), hprofile.len() as _, ::core::mem::transmute(padwintent.as_ptr()), padwintent.len() as _, dwflags, pprofiledata, indexpreferredcmm)
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
#[inline]
pub unsafe fn CreateMultiProfileTransform(pahprofiles: &[isize], padwintent: &[u32], dwflags: u32, indexpreferredcmm: u32) -> isize {
    ::windows::imp::link ! ( "mscms.dll""system" fn CreateMultiProfileTransform ( pahprofiles : *const isize , nprofiles : u32 , padwintent : *const u32 , nintents : u32 , dwflags : u32 , indexpreferredcmm : u32 ) -> isize );
    CreateMultiProfileTransform(::core::mem::transmute(pahprofiles.as_ptr()), pahprofiles.len() as _, ::core::mem::transmute(padwintent.as_ptr()), padwintent.len() as _, dwflags, indexpreferredcmm)
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn CreateProfileFromLogColorSpaceA(plogcolorspace: *const LOGCOLORSPACEA, pprofile: *mut *mut u8) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "mscms.dll""system" fn CreateProfileFromLogColorSpaceA ( plogcolorspace : *const LOGCOLORSPACEA , pprofile : *mut *mut u8 ) -> super::super::Foundation:: BOOL );
    CreateProfileFromLogColorSpaceA(plogcolorspace, pprofile)
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn CreateProfileFromLogColorSpaceW(plogcolorspace: *const LOGCOLORSPACEW, pprofile: *mut *mut u8) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "mscms.dll""system" fn CreateProfileFromLogColorSpaceW ( plogcolorspace : *const LOGCOLORSPACEW , pprofile : *mut *mut u8 ) -> super::super::Foundation:: BOOL );
    CreateProfileFromLogColorSpaceW(plogcolorspace, pprofile)
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeleteColorSpace<P0>(hcs: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<HCOLORSPACE>,
{
    ::windows::imp::link ! ( "gdi32.dll""system" fn DeleteColorSpace ( hcs : HCOLORSPACE ) -> super::super::Foundation:: BOOL );
    DeleteColorSpace(hcs.into_param().abi())
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeleteColorTransform(hxform: isize) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "mscms.dll""system" fn DeleteColorTransform ( hxform : isize ) -> super::super::Foundation:: BOOL );
    DeleteColorTransform(hxform)
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DisassociateColorProfileFromDeviceA<P0, P1, P2>(pmachinename: P0, pprofilename: P1, pdevicename: P2) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "mscms.dll""system" fn DisassociateColorProfileFromDeviceA ( pmachinename : :: windows::core::PCSTR , pprofilename : :: windows::core::PCSTR , pdevicename : :: windows::core::PCSTR ) -> super::super::Foundation:: BOOL );
    DisassociateColorProfileFromDeviceA(pmachinename.into_param().abi(), pprofilename.into_param().abi(), pdevicename.into_param().abi())
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DisassociateColorProfileFromDeviceW<P0, P1, P2>(pmachinename: P0, pprofilename: P1, pdevicename: P2) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "mscms.dll""system" fn DisassociateColorProfileFromDeviceW ( pmachinename : :: windows::core::PCWSTR , pprofilename : :: windows::core::PCWSTR , pdevicename : :: windows::core::PCWSTR ) -> super::super::Foundation:: BOOL );
    DisassociateColorProfileFromDeviceW(pmachinename.into_param().abi(), pprofilename.into_param().abi(), pdevicename.into_param().abi())
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumColorProfilesA<P0>(pmachinename: P0, penumrecord: *const ENUMTYPEA, penumerationbuffer: ::core::option::Option<*mut u8>, pdwsizeofenumerationbuffer: *mut u32, pnprofiles: ::core::option::Option<*mut u32>) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "mscms.dll""system" fn EnumColorProfilesA ( pmachinename : :: windows::core::PCSTR , penumrecord : *const ENUMTYPEA , penumerationbuffer : *mut u8 , pdwsizeofenumerationbuffer : *mut u32 , pnprofiles : *mut u32 ) -> super::super::Foundation:: BOOL );
    EnumColorProfilesA(pmachinename.into_param().abi(), penumrecord, ::core::mem::transmute(penumerationbuffer.unwrap_or(::std::ptr::null_mut())), pdwsizeofenumerationbuffer, ::core::mem::transmute(pnprofiles.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumColorProfilesW<P0>(pmachinename: P0, penumrecord: *const ENUMTYPEW, penumerationbuffer: ::core::option::Option<*mut u8>, pdwsizeofenumerationbuffer: *mut u32, pnprofiles: ::core::option::Option<*mut u32>) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "mscms.dll""system" fn EnumColorProfilesW ( pmachinename : :: windows::core::PCWSTR , penumrecord : *const ENUMTYPEW , penumerationbuffer : *mut u8 , pdwsizeofenumerationbuffer : *mut u32 , pnprofiles : *mut u32 ) -> super::super::Foundation:: BOOL );
    EnumColorProfilesW(pmachinename.into_param().abi(), penumrecord, ::core::mem::transmute(penumerationbuffer.unwrap_or(::std::ptr::null_mut())), pdwsizeofenumerationbuffer, ::core::mem::transmute(pnprofiles.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn EnumICMProfilesA<P0, P1>(hdc: P0, proc: ICMENUMPROCA, param2: P1) -> i32
where
    P0: ::windows::core::IntoParam<super::super::Graphics::Gdi::HDC>,
    P1: ::windows::core::IntoParam<super::super::Foundation::LPARAM>,
{
    ::windows::imp::link ! ( "gdi32.dll""system" fn EnumICMProfilesA ( hdc : super::super::Graphics::Gdi:: HDC , proc : ICMENUMPROCA , param2 : super::super::Foundation:: LPARAM ) -> i32 );
    EnumICMProfilesA(hdc.into_param().abi(), proc, param2.into_param().abi())
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn EnumICMProfilesW<P0, P1>(hdc: P0, proc: ICMENUMPROCW, param2: P1) -> i32
where
    P0: ::windows::core::IntoParam<super::super::Graphics::Gdi::HDC>,
    P1: ::windows::core::IntoParam<super::super::Foundation::LPARAM>,
{
    ::windows::imp::link ! ( "gdi32.dll""system" fn EnumICMProfilesW ( hdc : super::super::Graphics::Gdi:: HDC , proc : ICMENUMPROCW , param2 : super::super::Foundation:: LPARAM ) -> i32 );
    EnumICMProfilesW(hdc.into_param().abi(), proc, param2.into_param().abi())
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
#[inline]
pub unsafe fn GetCMMInfo(hcolortransform: isize, param1: u32) -> u32 {
    ::windows::imp::link ! ( "mscms.dll""system" fn GetCMMInfo ( hcolortransform : isize , param1 : u32 ) -> u32 );
    GetCMMInfo(hcolortransform, param1)
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetColorDirectoryA<P0>(pmachinename: P0, pbuffer: ::windows::core::PSTR, pdwsize: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "mscms.dll""system" fn GetColorDirectoryA ( pmachinename : :: windows::core::PCSTR , pbuffer : :: windows::core::PSTR , pdwsize : *mut u32 ) -> super::super::Foundation:: BOOL );
    GetColorDirectoryA(pmachinename.into_param().abi(), ::core::mem::transmute(pbuffer), pdwsize)
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetColorDirectoryW<P0>(pmachinename: P0, pbuffer: ::windows::core::PWSTR, pdwsize: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "mscms.dll""system" fn GetColorDirectoryW ( pmachinename : :: windows::core::PCWSTR , pbuffer : :: windows::core::PWSTR , pdwsize : *mut u32 ) -> super::super::Foundation:: BOOL );
    GetColorDirectoryW(pmachinename.into_param().abi(), ::core::mem::transmute(pbuffer), pdwsize)
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetColorProfileElement(hprofile: isize, tag: u32, dwoffset: u32, pcbelement: *mut u32, pelement: ::core::option::Option<*mut ::core::ffi::c_void>, pbreference: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "mscms.dll""system" fn GetColorProfileElement ( hprofile : isize , tag : u32 , dwoffset : u32 , pcbelement : *mut u32 , pelement : *mut ::core::ffi::c_void , pbreference : *mut super::super::Foundation:: BOOL ) -> super::super::Foundation:: BOOL );
    GetColorProfileElement(hprofile, tag, dwoffset, pcbelement, ::core::mem::transmute(pelement.unwrap_or(::std::ptr::null_mut())), pbreference)
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetColorProfileElementTag(hprofile: isize, dwindex: u32, ptag: *mut u32) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "mscms.dll""system" fn GetColorProfileElementTag ( hprofile : isize , dwindex : u32 , ptag : *mut u32 ) -> super::super::Foundation:: BOOL );
    GetColorProfileElementTag(hprofile, dwindex, ptag)
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetColorProfileFromHandle(hprofile: isize, pprofile: ::core::option::Option<*mut u8>, pcbprofile: *mut u32) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "mscms.dll""system" fn GetColorProfileFromHandle ( hprofile : isize , pprofile : *mut u8 , pcbprofile : *mut u32 ) -> super::super::Foundation:: BOOL );
    GetColorProfileFromHandle(hprofile, ::core::mem::transmute(pprofile.unwrap_or(::std::ptr::null_mut())), pcbprofile)
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn GetColorProfileHeader(hprofile: isize, pheader: *mut PROFILEHEADER) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "mscms.dll""system" fn GetColorProfileHeader ( hprofile : isize , pheader : *mut PROFILEHEADER ) -> super::super::Foundation:: BOOL );
    GetColorProfileHeader(hprofile, pheader)
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetColorSpace<P0>(hdc: P0) -> HCOLORSPACE
where
    P0: ::windows::core::IntoParam<super::super::Graphics::Gdi::HDC>,
{
    ::windows::imp::link ! ( "gdi32.dll""system" fn GetColorSpace ( hdc : super::super::Graphics::Gdi:: HDC ) -> HCOLORSPACE );
    GetColorSpace(hdc.into_param().abi())
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetCountColorProfileElements(hprofile: isize, pnelementcount: *mut u32) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "mscms.dll""system" fn GetCountColorProfileElements ( hprofile : isize , pnelementcount : *mut u32 ) -> super::super::Foundation:: BOOL );
    GetCountColorProfileElements(hprofile, pnelementcount)
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn GetDeviceGammaRamp<P0>(hdc: P0, lpramp: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Graphics::Gdi::HDC>,
{
    ::windows::imp::link ! ( "gdi32.dll""system" fn GetDeviceGammaRamp ( hdc : super::super::Graphics::Gdi:: HDC , lpramp : *mut ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    GetDeviceGammaRamp(hdc.into_param().abi(), lpramp)
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn GetICMProfileA<P0>(hdc: P0, pbufsize: *mut u32, pszfilename: ::windows::core::PSTR) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Graphics::Gdi::HDC>,
{
    ::windows::imp::link ! ( "gdi32.dll""system" fn GetICMProfileA ( hdc : super::super::Graphics::Gdi:: HDC , pbufsize : *mut u32 , pszfilename : :: windows::core::PSTR ) -> super::super::Foundation:: BOOL );
    GetICMProfileA(hdc.into_param().abi(), pbufsize, ::core::mem::transmute(pszfilename))
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn GetICMProfileW<P0>(hdc: P0, pbufsize: *mut u32, pszfilename: ::windows::core::PWSTR) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Graphics::Gdi::HDC>,
{
    ::windows::imp::link ! ( "gdi32.dll""system" fn GetICMProfileW ( hdc : super::super::Graphics::Gdi:: HDC , pbufsize : *mut u32 , pszfilename : :: windows::core::PWSTR ) -> super::super::Foundation:: BOOL );
    GetICMProfileW(hdc.into_param().abi(), pbufsize, ::core::mem::transmute(pszfilename))
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn GetLogColorSpaceA<P0>(hcolorspace: P0, lpbuffer: *mut LOGCOLORSPACEA, nsize: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<HCOLORSPACE>,
{
    ::windows::imp::link ! ( "gdi32.dll""system" fn GetLogColorSpaceA ( hcolorspace : HCOLORSPACE , lpbuffer : *mut LOGCOLORSPACEA , nsize : u32 ) -> super::super::Foundation:: BOOL );
    GetLogColorSpaceA(hcolorspace.into_param().abi(), lpbuffer, nsize)
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn GetLogColorSpaceW<P0>(hcolorspace: P0, lpbuffer: *mut LOGCOLORSPACEW, nsize: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<HCOLORSPACE>,
{
    ::windows::imp::link ! ( "gdi32.dll""system" fn GetLogColorSpaceW ( hcolorspace : HCOLORSPACE , lpbuffer : *mut LOGCOLORSPACEW , nsize : u32 ) -> super::super::Foundation:: BOOL );
    GetLogColorSpaceW(hcolorspace.into_param().abi(), lpbuffer, nsize)
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetNamedProfileInfo(hprofile: isize, pnamedprofileinfo: *mut NAMED_PROFILE_INFO) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "mscms.dll""system" fn GetNamedProfileInfo ( hprofile : isize , pnamedprofileinfo : *mut NAMED_PROFILE_INFO ) -> super::super::Foundation:: BOOL );
    GetNamedProfileInfo(hprofile, pnamedprofileinfo)
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetPS2ColorRenderingDictionary(hprofile: isize, dwintent: u32, pps2colorrenderingdictionary: ::core::option::Option<*mut u8>, pcbps2colorrenderingdictionary: *mut u32, pbbinary: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "mscms.dll""system" fn GetPS2ColorRenderingDictionary ( hprofile : isize , dwintent : u32 , pps2colorrenderingdictionary : *mut u8 , pcbps2colorrenderingdictionary : *mut u32 , pbbinary : *mut super::super::Foundation:: BOOL ) -> super::super::Foundation:: BOOL );
    GetPS2ColorRenderingDictionary(hprofile, dwintent, ::core::mem::transmute(pps2colorrenderingdictionary.unwrap_or(::std::ptr::null_mut())), pcbps2colorrenderingdictionary, pbbinary)
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetPS2ColorRenderingIntent(hprofile: isize, dwintent: u32, pbuffer: ::core::option::Option<*mut u8>, pcbps2colorrenderingintent: *mut u32) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "mscms.dll""system" fn GetPS2ColorRenderingIntent ( hprofile : isize , dwintent : u32 , pbuffer : *mut u8 , pcbps2colorrenderingintent : *mut u32 ) -> super::super::Foundation:: BOOL );
    GetPS2ColorRenderingIntent(hprofile, dwintent, ::core::mem::transmute(pbuffer.unwrap_or(::std::ptr::null_mut())), pcbps2colorrenderingintent)
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetPS2ColorSpaceArray(hprofile: isize, dwintent: u32, dwcsatype: u32, pps2colorspacearray: ::core::option::Option<*mut u8>, pcbps2colorspacearray: *mut u32, pbbinary: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "mscms.dll""system" fn GetPS2ColorSpaceArray ( hprofile : isize , dwintent : u32 , dwcsatype : u32 , pps2colorspacearray : *mut u8 , pcbps2colorspacearray : *mut u32 , pbbinary : *mut super::super::Foundation:: BOOL ) -> super::super::Foundation:: BOOL );
    GetPS2ColorSpaceArray(hprofile, dwintent, dwcsatype, ::core::mem::transmute(pps2colorspacearray.unwrap_or(::std::ptr::null_mut())), pcbps2colorspacearray, pbbinary)
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetStandardColorSpaceProfileA<P0>(pmachinename: P0, dwscs: u32, pbuffer: ::windows::core::PSTR, pcbsize: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "mscms.dll""system" fn GetStandardColorSpaceProfileA ( pmachinename : :: windows::core::PCSTR , dwscs : u32 , pbuffer : :: windows::core::PSTR , pcbsize : *mut u32 ) -> super::super::Foundation:: BOOL );
    GetStandardColorSpaceProfileA(pmachinename.into_param().abi(), dwscs, ::core::mem::transmute(pbuffer), pcbsize)
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetStandardColorSpaceProfileW<P0>(pmachinename: P0, dwscs: u32, pbuffer: ::windows::core::PWSTR, pcbsize: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "mscms.dll""system" fn GetStandardColorSpaceProfileW ( pmachinename : :: windows::core::PCWSTR , dwscs : u32 , pbuffer : :: windows::core::PWSTR , pcbsize : *mut u32 ) -> super::super::Foundation:: BOOL );
    GetStandardColorSpaceProfileW(pmachinename.into_param().abi(), dwscs, ::core::mem::transmute(pbuffer), pcbsize)
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InstallColorProfileA<P0, P1>(pmachinename: P0, pprofilename: P1) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "mscms.dll""system" fn InstallColorProfileA ( pmachinename : :: windows::core::PCSTR , pprofilename : :: windows::core::PCSTR ) -> super::super::Foundation:: BOOL );
    InstallColorProfileA(pmachinename.into_param().abi(), pprofilename.into_param().abi())
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InstallColorProfileW<P0, P1>(pmachinename: P0, pprofilename: P1) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "mscms.dll""system" fn InstallColorProfileW ( pmachinename : :: windows::core::PCWSTR , pprofilename : :: windows::core::PCWSTR ) -> super::super::Foundation:: BOOL );
    InstallColorProfileW(pmachinename.into_param().abi(), pprofilename.into_param().abi())
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsColorProfileTagPresent(hprofile: isize, tag: u32, pbpresent: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "mscms.dll""system" fn IsColorProfileTagPresent ( hprofile : isize , tag : u32 , pbpresent : *mut super::super::Foundation:: BOOL ) -> super::super::Foundation:: BOOL );
    IsColorProfileTagPresent(hprofile, tag, pbpresent)
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsColorProfileValid(hprofile: isize, pbvalid: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "mscms.dll""system" fn IsColorProfileValid ( hprofile : isize , pbvalid : *mut super::super::Foundation:: BOOL ) -> super::super::Foundation:: BOOL );
    IsColorProfileValid(hprofile, pbvalid)
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
#[inline]
pub unsafe fn OpenColorProfileA(pprofile: *const PROFILE, dwdesiredaccess: u32, dwsharemode: u32, dwcreationmode: u32) -> isize {
    ::windows::imp::link ! ( "mscms.dll""system" fn OpenColorProfileA ( pprofile : *const PROFILE , dwdesiredaccess : u32 , dwsharemode : u32 , dwcreationmode : u32 ) -> isize );
    OpenColorProfileA(pprofile, dwdesiredaccess, dwsharemode, dwcreationmode)
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
#[inline]
pub unsafe fn OpenColorProfileW(pprofile: *const PROFILE, dwdesiredaccess: u32, dwsharemode: u32, dwcreationmode: u32) -> isize {
    ::windows::imp::link ! ( "mscms.dll""system" fn OpenColorProfileW ( pprofile : *const PROFILE , dwdesiredaccess : u32 , dwsharemode : u32 , dwcreationmode : u32 ) -> isize );
    OpenColorProfileW(pprofile, dwdesiredaccess, dwsharemode, dwcreationmode)
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegisterCMMA<P0, P1>(pmachinename: P0, cmmid: u32, pcmmdll: P1) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "mscms.dll""system" fn RegisterCMMA ( pmachinename : :: windows::core::PCSTR , cmmid : u32 , pcmmdll : :: windows::core::PCSTR ) -> super::super::Foundation:: BOOL );
    RegisterCMMA(pmachinename.into_param().abi(), cmmid, pcmmdll.into_param().abi())
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegisterCMMW<P0, P1>(pmachinename: P0, cmmid: u32, pcmmdll: P1) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "mscms.dll""system" fn RegisterCMMW ( pmachinename : :: windows::core::PCWSTR , cmmid : u32 , pcmmdll : :: windows::core::PCWSTR ) -> super::super::Foundation:: BOOL );
    RegisterCMMW(pmachinename.into_param().abi(), cmmid, pcmmdll.into_param().abi())
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SelectCMM(dwcmmtype: u32) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "mscms.dll""system" fn SelectCMM ( dwcmmtype : u32 ) -> super::super::Foundation:: BOOL );
    SelectCMM(dwcmmtype)
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetColorProfileElement(hprofile: isize, tag: u32, dwoffset: u32, pcbelement: *const u32, pelement: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "mscms.dll""system" fn SetColorProfileElement ( hprofile : isize , tag : u32 , dwoffset : u32 , pcbelement : *const u32 , pelement : *const ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    SetColorProfileElement(hprofile, tag, dwoffset, pcbelement, pelement)
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetColorProfileElementReference(hprofile: isize, newtag: u32, reftag: u32) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "mscms.dll""system" fn SetColorProfileElementReference ( hprofile : isize , newtag : u32 , reftag : u32 ) -> super::super::Foundation:: BOOL );
    SetColorProfileElementReference(hprofile, newtag, reftag)
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetColorProfileElementSize(hprofile: isize, tagtype: u32, pcbelement: u32) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "mscms.dll""system" fn SetColorProfileElementSize ( hprofile : isize , tagtype : u32 , pcbelement : u32 ) -> super::super::Foundation:: BOOL );
    SetColorProfileElementSize(hprofile, tagtype, pcbelement)
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn SetColorProfileHeader(hprofile: isize, pheader: *const PROFILEHEADER) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "mscms.dll""system" fn SetColorProfileHeader ( hprofile : isize , pheader : *const PROFILEHEADER ) -> super::super::Foundation:: BOOL );
    SetColorProfileHeader(hprofile, pheader)
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn SetColorSpace<P0, P1>(hdc: P0, hcs: P1) -> HCOLORSPACE
where
    P0: ::windows::core::IntoParam<super::super::Graphics::Gdi::HDC>,
    P1: ::windows::core::IntoParam<HCOLORSPACE>,
{
    ::windows::imp::link ! ( "gdi32.dll""system" fn SetColorSpace ( hdc : super::super::Graphics::Gdi:: HDC , hcs : HCOLORSPACE ) -> HCOLORSPACE );
    SetColorSpace(hdc.into_param().abi(), hcs.into_param().abi())
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn SetDeviceGammaRamp<P0>(hdc: P0, lpramp: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Graphics::Gdi::HDC>,
{
    ::windows::imp::link ! ( "gdi32.dll""system" fn SetDeviceGammaRamp ( hdc : super::super::Graphics::Gdi:: HDC , lpramp : *const ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    SetDeviceGammaRamp(hdc.into_param().abi(), lpramp)
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn SetICMMode<P0>(hdc: P0, mode: ICM_MODE) -> i32
where
    P0: ::windows::core::IntoParam<super::super::Graphics::Gdi::HDC>,
{
    ::windows::imp::link ! ( "gdi32.dll""system" fn SetICMMode ( hdc : super::super::Graphics::Gdi:: HDC , mode : ICM_MODE ) -> i32 );
    SetICMMode(hdc.into_param().abi(), mode)
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn SetICMProfileA<P0, P1>(hdc: P0, lpfilename: P1) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Graphics::Gdi::HDC>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "gdi32.dll""system" fn SetICMProfileA ( hdc : super::super::Graphics::Gdi:: HDC , lpfilename : :: windows::core::PCSTR ) -> super::super::Foundation:: BOOL );
    SetICMProfileA(hdc.into_param().abi(), lpfilename.into_param().abi())
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn SetICMProfileW<P0, P1>(hdc: P0, lpfilename: P1) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Graphics::Gdi::HDC>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "gdi32.dll""system" fn SetICMProfileW ( hdc : super::super::Graphics::Gdi:: HDC , lpfilename : :: windows::core::PCWSTR ) -> super::super::Foundation:: BOOL );
    SetICMProfileW(hdc.into_param().abi(), lpfilename.into_param().abi())
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetStandardColorSpaceProfileA<P0, P1>(pmachinename: P0, dwprofileid: u32, pprofilename: P1) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "mscms.dll""system" fn SetStandardColorSpaceProfileA ( pmachinename : :: windows::core::PCSTR , dwprofileid : u32 , pprofilename : :: windows::core::PCSTR ) -> super::super::Foundation:: BOOL );
    SetStandardColorSpaceProfileA(pmachinename.into_param().abi(), dwprofileid, pprofilename.into_param().abi())
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetStandardColorSpaceProfileW<P0, P1>(pmachinename: P0, dwprofileid: u32, pprofilename: P1) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "mscms.dll""system" fn SetStandardColorSpaceProfileW ( pmachinename : :: windows::core::PCWSTR , dwprofileid : u32 , pprofilename : :: windows::core::PCWSTR ) -> super::super::Foundation:: BOOL );
    SetStandardColorSpaceProfileW(pmachinename.into_param().abi(), dwprofileid, pprofilename.into_param().abi())
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn SetupColorMatchingA(pcms: *mut COLORMATCHSETUPA) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "icmui.dll""system" fn SetupColorMatchingA ( pcms : *mut COLORMATCHSETUPA ) -> super::super::Foundation:: BOOL );
    SetupColorMatchingA(pcms)
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn SetupColorMatchingW(pcms: *mut COLORMATCHSETUPW) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "icmui.dll""system" fn SetupColorMatchingW ( pcms : *mut COLORMATCHSETUPW ) -> super::super::Foundation:: BOOL );
    SetupColorMatchingW(pcms)
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TranslateBitmapBits<P0>(hcolortransform: isize, psrcbits: *const ::core::ffi::c_void, bminput: BMFORMAT, dwwidth: u32, dwheight: u32, dwinputstride: u32, pdestbits: *mut ::core::ffi::c_void, bmoutput: BMFORMAT, dwoutputstride: u32, pfncallback: LPBMCALLBACKFN, ulcallbackdata: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::LPARAM>,
{
    ::windows::imp::link ! ( "mscms.dll""system" fn TranslateBitmapBits ( hcolortransform : isize , psrcbits : *const ::core::ffi::c_void , bminput : BMFORMAT , dwwidth : u32 , dwheight : u32 , dwinputstride : u32 , pdestbits : *mut ::core::ffi::c_void , bmoutput : BMFORMAT , dwoutputstride : u32 , pfncallback : LPBMCALLBACKFN , ulcallbackdata : super::super::Foundation:: LPARAM ) -> super::super::Foundation:: BOOL );
    TranslateBitmapBits(hcolortransform, psrcbits, bminput, dwwidth, dwheight, dwinputstride, pdestbits, bmoutput, dwoutputstride, pfncallback, ulcallbackdata.into_param().abi())
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TranslateColors(hcolortransform: isize, painputcolors: *const COLOR, ncolors: u32, ctinput: COLORTYPE, paoutputcolors: *mut COLOR, ctoutput: COLORTYPE) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "mscms.dll""system" fn TranslateColors ( hcolortransform : isize , painputcolors : *const COLOR , ncolors : u32 , ctinput : COLORTYPE , paoutputcolors : *mut COLOR , ctoutput : COLORTYPE ) -> super::super::Foundation:: BOOL );
    TranslateColors(hcolortransform, painputcolors, ncolors, ctinput, paoutputcolors, ctoutput)
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UninstallColorProfileA<P0, P1, P2>(pmachinename: P0, pprofilename: P1, bdelete: P2) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows::imp::link ! ( "mscms.dll""system" fn UninstallColorProfileA ( pmachinename : :: windows::core::PCSTR , pprofilename : :: windows::core::PCSTR , bdelete : super::super::Foundation:: BOOL ) -> super::super::Foundation:: BOOL );
    UninstallColorProfileA(pmachinename.into_param().abi(), pprofilename.into_param().abi(), bdelete.into_param().abi())
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UninstallColorProfileW<P0, P1, P2>(pmachinename: P0, pprofilename: P1, bdelete: P2) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows::imp::link ! ( "mscms.dll""system" fn UninstallColorProfileW ( pmachinename : :: windows::core::PCWSTR , pprofilename : :: windows::core::PCWSTR , bdelete : super::super::Foundation:: BOOL ) -> super::super::Foundation:: BOOL );
    UninstallColorProfileW(pmachinename.into_param().abi(), pprofilename.into_param().abi(), bdelete.into_param().abi())
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UnregisterCMMA<P0>(pmachinename: P0, cmmid: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "mscms.dll""system" fn UnregisterCMMA ( pmachinename : :: windows::core::PCSTR , cmmid : u32 ) -> super::super::Foundation:: BOOL );
    UnregisterCMMA(pmachinename.into_param().abi(), cmmid)
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UnregisterCMMW<P0>(pmachinename: P0, cmmid: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "mscms.dll""system" fn UnregisterCMMW ( pmachinename : :: windows::core::PCWSTR , cmmid : u32 ) -> super::super::Foundation:: BOOL );
    UnregisterCMMW(pmachinename.into_param().abi(), cmmid)
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UpdateICMRegKeyA<P0, P1>(reserved: u32, lpszcmid: P0, lpszfilename: P1, command: ICM_COMMAND) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "gdi32.dll""system" fn UpdateICMRegKeyA ( reserved : u32 , lpszcmid : :: windows::core::PCSTR , lpszfilename : :: windows::core::PCSTR , command : ICM_COMMAND ) -> super::super::Foundation:: BOOL );
    UpdateICMRegKeyA(reserved, lpszcmid.into_param().abi(), lpszfilename.into_param().abi(), command)
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UpdateICMRegKeyW<P0, P1>(reserved: u32, lpszcmid: P0, lpszfilename: P1, command: ICM_COMMAND) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "gdi32.dll""system" fn UpdateICMRegKeyW ( reserved : u32 , lpszcmid : :: windows::core::PCWSTR , lpszfilename : :: windows::core::PCWSTR , command : ICM_COMMAND ) -> super::super::Foundation:: BOOL );
    UpdateICMRegKeyW(reserved, lpszcmid.into_param().abi(), lpszfilename.into_param().abi(), command)
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WcsAssociateColorProfileWithDevice<P0, P1>(scope: WCS_PROFILE_MANAGEMENT_SCOPE, pprofilename: P0, pdevicename: P1) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "mscms.dll""system" fn WcsAssociateColorProfileWithDevice ( scope : WCS_PROFILE_MANAGEMENT_SCOPE , pprofilename : :: windows::core::PCWSTR , pdevicename : :: windows::core::PCWSTR ) -> super::super::Foundation:: BOOL );
    WcsAssociateColorProfileWithDevice(scope, pprofilename.into_param().abi(), pdevicename.into_param().abi())
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WcsCheckColors(hcolortransform: isize, ninputchannels: u32, cdtinput: COLORDATATYPE, cbinput: u32, pinputdata: *const ::core::ffi::c_void, paresult: &mut [u8]) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "mscms.dll""system" fn WcsCheckColors ( hcolortransform : isize , ncolors : u32 , ninputchannels : u32 , cdtinput : COLORDATATYPE , cbinput : u32 , pinputdata : *const ::core::ffi::c_void , paresult : *mut u8 ) -> super::super::Foundation:: BOOL );
    WcsCheckColors(hcolortransform, paresult.len() as _, ninputchannels, cdtinput, cbinput, pinputdata, ::core::mem::transmute(paresult.as_ptr()))
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
#[inline]
pub unsafe fn WcsCreateIccProfile(hwcsprofile: isize, dwoptions: u32) -> isize {
    ::windows::imp::link ! ( "mscms.dll""system" fn WcsCreateIccProfile ( hwcsprofile : isize , dwoptions : u32 ) -> isize );
    WcsCreateIccProfile(hwcsprofile, dwoptions)
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WcsDisassociateColorProfileFromDevice<P0, P1>(scope: WCS_PROFILE_MANAGEMENT_SCOPE, pprofilename: P0, pdevicename: P1) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "mscms.dll""system" fn WcsDisassociateColorProfileFromDevice ( scope : WCS_PROFILE_MANAGEMENT_SCOPE , pprofilename : :: windows::core::PCWSTR , pdevicename : :: windows::core::PCWSTR ) -> super::super::Foundation:: BOOL );
    WcsDisassociateColorProfileFromDevice(scope, pprofilename.into_param().abi(), pdevicename.into_param().abi())
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WcsEnumColorProfiles(scope: WCS_PROFILE_MANAGEMENT_SCOPE, penumrecord: *const ENUMTYPEW, pbuffer: &mut [u8], pnprofiles: ::core::option::Option<*mut u32>) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "mscms.dll""system" fn WcsEnumColorProfiles ( scope : WCS_PROFILE_MANAGEMENT_SCOPE , penumrecord : *const ENUMTYPEW , pbuffer : *mut u8 , dwsize : u32 , pnprofiles : *mut u32 ) -> super::super::Foundation:: BOOL );
    WcsEnumColorProfiles(scope, penumrecord, ::core::mem::transmute(pbuffer.as_ptr()), pbuffer.len() as _, ::core::mem::transmute(pnprofiles.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WcsEnumColorProfilesSize(scope: WCS_PROFILE_MANAGEMENT_SCOPE, penumrecord: *const ENUMTYPEW, pdwsize: *mut u32) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "mscms.dll""system" fn WcsEnumColorProfilesSize ( scope : WCS_PROFILE_MANAGEMENT_SCOPE , penumrecord : *const ENUMTYPEW , pdwsize : *mut u32 ) -> super::super::Foundation:: BOOL );
    WcsEnumColorProfilesSize(scope, penumrecord, pdwsize)
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WcsGetCalibrationManagementState(pbisenabled: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "mscms.dll""system" fn WcsGetCalibrationManagementState ( pbisenabled : *mut super::super::Foundation:: BOOL ) -> super::super::Foundation:: BOOL );
    WcsGetCalibrationManagementState(pbisenabled)
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WcsGetDefaultColorProfile<P0>(scope: WCS_PROFILE_MANAGEMENT_SCOPE, pdevicename: P0, cptcolorprofiletype: COLORPROFILETYPE, cpstcolorprofilesubtype: COLORPROFILESUBTYPE, dwprofileid: u32, cbprofilename: u32, pprofilename: ::windows::core::PWSTR) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "mscms.dll""system" fn WcsGetDefaultColorProfile ( scope : WCS_PROFILE_MANAGEMENT_SCOPE , pdevicename : :: windows::core::PCWSTR , cptcolorprofiletype : COLORPROFILETYPE , cpstcolorprofilesubtype : COLORPROFILESUBTYPE , dwprofileid : u32 , cbprofilename : u32 , pprofilename : :: windows::core::PWSTR ) -> super::super::Foundation:: BOOL );
    WcsGetDefaultColorProfile(scope, pdevicename.into_param().abi(), cptcolorprofiletype, cpstcolorprofilesubtype, dwprofileid, cbprofilename, ::core::mem::transmute(pprofilename))
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WcsGetDefaultColorProfileSize<P0>(scope: WCS_PROFILE_MANAGEMENT_SCOPE, pdevicename: P0, cptcolorprofiletype: COLORPROFILETYPE, cpstcolorprofilesubtype: COLORPROFILESUBTYPE, dwprofileid: u32, pcbprofilename: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "mscms.dll""system" fn WcsGetDefaultColorProfileSize ( scope : WCS_PROFILE_MANAGEMENT_SCOPE , pdevicename : :: windows::core::PCWSTR , cptcolorprofiletype : COLORPROFILETYPE , cpstcolorprofilesubtype : COLORPROFILESUBTYPE , dwprofileid : u32 , pcbprofilename : *mut u32 ) -> super::super::Foundation:: BOOL );
    WcsGetDefaultColorProfileSize(scope, pdevicename.into_param().abi(), cptcolorprofiletype, cpstcolorprofilesubtype, dwprofileid, pcbprofilename)
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WcsGetDefaultRenderingIntent(scope: WCS_PROFILE_MANAGEMENT_SCOPE, pdwrenderingintent: *mut u32) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "mscms.dll""system" fn WcsGetDefaultRenderingIntent ( scope : WCS_PROFILE_MANAGEMENT_SCOPE , pdwrenderingintent : *mut u32 ) -> super::super::Foundation:: BOOL );
    WcsGetDefaultRenderingIntent(scope, pdwrenderingintent)
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WcsGetUsePerUserProfiles<P0>(pdevicename: P0, dwdeviceclass: u32, puseperuserprofiles: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "mscms.dll""system" fn WcsGetUsePerUserProfiles ( pdevicename : :: windows::core::PCWSTR , dwdeviceclass : u32 , puseperuserprofiles : *mut super::super::Foundation:: BOOL ) -> super::super::Foundation:: BOOL );
    WcsGetUsePerUserProfiles(pdevicename.into_param().abi(), dwdeviceclass, puseperuserprofiles)
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
#[inline]
pub unsafe fn WcsOpenColorProfileA(pcdmpprofile: *const PROFILE, pcampprofile: ::core::option::Option<*const PROFILE>, pgmmpprofile: ::core::option::Option<*const PROFILE>, dwdesireaccess: u32, dwsharemode: u32, dwcreationmode: u32, dwflags: u32) -> isize {
    ::windows::imp::link ! ( "mscms.dll""system" fn WcsOpenColorProfileA ( pcdmpprofile : *const PROFILE , pcampprofile : *const PROFILE , pgmmpprofile : *const PROFILE , dwdesireaccess : u32 , dwsharemode : u32 , dwcreationmode : u32 , dwflags : u32 ) -> isize );
    WcsOpenColorProfileA(pcdmpprofile, ::core::mem::transmute(pcampprofile.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pgmmpprofile.unwrap_or(::std::ptr::null())), dwdesireaccess, dwsharemode, dwcreationmode, dwflags)
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
#[inline]
pub unsafe fn WcsOpenColorProfileW(pcdmpprofile: *const PROFILE, pcampprofile: ::core::option::Option<*const PROFILE>, pgmmpprofile: ::core::option::Option<*const PROFILE>, dwdesireaccess: u32, dwsharemode: u32, dwcreationmode: u32, dwflags: u32) -> isize {
    ::windows::imp::link ! ( "mscms.dll""system" fn WcsOpenColorProfileW ( pcdmpprofile : *const PROFILE , pcampprofile : *const PROFILE , pgmmpprofile : *const PROFILE , dwdesireaccess : u32 , dwsharemode : u32 , dwcreationmode : u32 , dwflags : u32 ) -> isize );
    WcsOpenColorProfileW(pcdmpprofile, ::core::mem::transmute(pcampprofile.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pgmmpprofile.unwrap_or(::std::ptr::null())), dwdesireaccess, dwsharemode, dwcreationmode, dwflags)
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WcsSetCalibrationManagementState<P0>(bisenabled: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows::imp::link ! ( "mscms.dll""system" fn WcsSetCalibrationManagementState ( bisenabled : super::super::Foundation:: BOOL ) -> super::super::Foundation:: BOOL );
    WcsSetCalibrationManagementState(bisenabled.into_param().abi())
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WcsSetDefaultColorProfile<P0, P1>(scope: WCS_PROFILE_MANAGEMENT_SCOPE, pdevicename: P0, cptcolorprofiletype: COLORPROFILETYPE, cpstcolorprofilesubtype: COLORPROFILESUBTYPE, dwprofileid: u32, pprofilename: P1) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "mscms.dll""system" fn WcsSetDefaultColorProfile ( scope : WCS_PROFILE_MANAGEMENT_SCOPE , pdevicename : :: windows::core::PCWSTR , cptcolorprofiletype : COLORPROFILETYPE , cpstcolorprofilesubtype : COLORPROFILESUBTYPE , dwprofileid : u32 , pprofilename : :: windows::core::PCWSTR ) -> super::super::Foundation:: BOOL );
    WcsSetDefaultColorProfile(scope, pdevicename.into_param().abi(), cptcolorprofiletype, cpstcolorprofilesubtype, dwprofileid, pprofilename.into_param().abi())
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WcsSetDefaultRenderingIntent(scope: WCS_PROFILE_MANAGEMENT_SCOPE, dwrenderingintent: u32) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "mscms.dll""system" fn WcsSetDefaultRenderingIntent ( scope : WCS_PROFILE_MANAGEMENT_SCOPE , dwrenderingintent : u32 ) -> super::super::Foundation:: BOOL );
    WcsSetDefaultRenderingIntent(scope, dwrenderingintent)
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WcsSetUsePerUserProfiles<P0, P1>(pdevicename: P0, dwdeviceclass: u32, useperuserprofiles: P1) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows::imp::link ! ( "mscms.dll""system" fn WcsSetUsePerUserProfiles ( pdevicename : :: windows::core::PCWSTR , dwdeviceclass : u32 , useperuserprofiles : super::super::Foundation:: BOOL ) -> super::super::Foundation:: BOOL );
    WcsSetUsePerUserProfiles(pdevicename.into_param().abi(), dwdeviceclass, useperuserprofiles.into_param().abi())
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WcsTranslateColors(hcolortransform: isize, ncolors: u32, ninputchannels: u32, cdtinput: COLORDATATYPE, cbinput: u32, pinputdata: *const ::core::ffi::c_void, noutputchannels: u32, cdtoutput: COLORDATATYPE, cboutput: u32, poutputdata: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "mscms.dll""system" fn WcsTranslateColors ( hcolortransform : isize , ncolors : u32 , ninputchannels : u32 , cdtinput : COLORDATATYPE , cbinput : u32 , pinputdata : *const ::core::ffi::c_void , noutputchannels : u32 , cdtoutput : COLORDATATYPE , cboutput : u32 , poutputdata : *mut ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    WcsTranslateColors(hcolortransform, ncolors, ninputchannels, cdtinput, cbinput, pinputdata, noutputchannels, cdtoutput, cboutput, poutputdata)
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
#[repr(transparent)]
pub struct IDeviceModelPlugIn(::windows::core::IUnknown);
impl IDeviceModelPlugIn {
    pub unsafe fn Initialize<P0>(&self, bstrxml: P0, cnummodels: u32, imodelposition: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).Initialize)(::windows::core::Interface::as_raw(self), bstrxml.into_param().abi(), cnummodels, imodelposition).ok()
    }
    pub unsafe fn GetNumChannels(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetNumChannels)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn DeviceToColorimetricColors(&self, cchannels: u32, pdevicevalues: *const f32, pxyzcolors: &mut [XYZColorF]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DeviceToColorimetricColors)(::windows::core::Interface::as_raw(self), pxyzcolors.len() as _, cchannels, pdevicevalues, ::core::mem::transmute(pxyzcolors.as_ptr())).ok()
    }
    pub unsafe fn ColorimetricToDeviceColors(&self, cchannels: u32, pxyzcolors: &[XYZColorF]) -> ::windows::core::Result<f32> {
        let mut result__ = ::windows::core::zeroed::<f32>();
        (::windows::core::Interface::vtable(self).ColorimetricToDeviceColors)(::windows::core::Interface::as_raw(self), pxyzcolors.len() as _, cchannels, ::core::mem::transmute(pxyzcolors.as_ptr()), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ColorimetricToDeviceColorsWithBlack(&self, ccolors: u32, cchannels: u32, pxyzcolors: *const XYZColorF, pblackinformation: *const BlackInformation) -> ::windows::core::Result<f32> {
        let mut result__ = ::windows::core::zeroed::<f32>();
        (::windows::core::Interface::vtable(self).ColorimetricToDeviceColorsWithBlack)(::windows::core::Interface::as_raw(self), ccolors, cchannels, pxyzcolors, pblackinformation, &mut result__).from_abi(result__)
    }
    pub unsafe fn SetTransformDeviceModelInfo<P0>(&self, imodelposition: u32, pidevicemodelother: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDeviceModelPlugIn>,
    {
        (::windows::core::Interface::vtable(self).SetTransformDeviceModelInfo)(::windows::core::Interface::as_raw(self), imodelposition, pidevicemodelother.into_param().abi()).ok()
    }
    pub unsafe fn GetPrimarySamples(&self, pprimarycolor: *mut PrimaryXYZColors) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPrimarySamples)(::windows::core::Interface::as_raw(self), pprimarycolor).ok()
    }
    pub unsafe fn GetGamutBoundaryMeshSize(&self, pnumvertices: *mut u32, pnumtriangles: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetGamutBoundaryMeshSize)(::windows::core::Interface::as_raw(self), pnumvertices, pnumtriangles).ok()
    }
    pub unsafe fn GetGamutBoundaryMesh(&self, cchannels: u32, cvertices: u32, pvertices: *mut f32, ptriangles: &mut [GamutShellTriangle]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetGamutBoundaryMesh)(::windows::core::Interface::as_raw(self), cchannels, cvertices, ptriangles.len() as _, pvertices, ::core::mem::transmute(ptriangles.as_ptr())).ok()
    }
    pub unsafe fn GetNeutralAxisSize(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetNeutralAxisSize)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetNeutralAxis(&self, pxyzcolors: &mut [XYZColorF]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetNeutralAxis)(::windows::core::Interface::as_raw(self), pxyzcolors.len() as _, ::core::mem::transmute(pxyzcolors.as_ptr())).ok()
    }
}
::windows::imp::interface_hierarchy!(IDeviceModelPlugIn, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDeviceModelPlugIn {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDeviceModelPlugIn {}
impl ::core::fmt::Debug for IDeviceModelPlugIn {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDeviceModelPlugIn").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDeviceModelPlugIn {
    type Vtable = IDeviceModelPlugIn_Vtbl;
}
impl ::core::clone::Clone for IDeviceModelPlugIn {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDeviceModelPlugIn {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1cd63475_07c4_46fe_a903_d655316d11fd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceModelPlugIn_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrxml: ::std::mem::MaybeUninit<::windows::core::BSTR>, cnummodels: u32, imodelposition: u32) -> ::windows::core::HRESULT,
    pub GetNumChannels: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnumchannels: *mut u32) -> ::windows::core::HRESULT,
    pub DeviceToColorimetricColors: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ccolors: u32, cchannels: u32, pdevicevalues: *const f32, pxyzcolors: *mut XYZColorF) -> ::windows::core::HRESULT,
    pub ColorimetricToDeviceColors: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ccolors: u32, cchannels: u32, pxyzcolors: *const XYZColorF, pdevicevalues: *mut f32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ColorimetricToDeviceColorsWithBlack: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ccolors: u32, cchannels: u32, pxyzcolors: *const XYZColorF, pblackinformation: *const BlackInformation, pdevicevalues: *mut f32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ColorimetricToDeviceColorsWithBlack: usize,
    pub SetTransformDeviceModelInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imodelposition: u32, pidevicemodelother: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetPrimarySamples: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprimarycolor: *mut PrimaryXYZColors) -> ::windows::core::HRESULT,
    pub GetGamutBoundaryMeshSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnumvertices: *mut u32, pnumtriangles: *mut u32) -> ::windows::core::HRESULT,
    pub GetGamutBoundaryMesh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchannels: u32, cvertices: u32, ctriangles: u32, pvertices: *mut f32, ptriangles: *mut GamutShellTriangle) -> ::windows::core::HRESULT,
    pub GetNeutralAxisSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pccolors: *mut u32) -> ::windows::core::HRESULT,
    pub GetNeutralAxis: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ccolors: u32, pxyzcolors: *mut XYZColorF) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
#[repr(transparent)]
pub struct IGamutMapModelPlugIn(::windows::core::IUnknown);
impl IGamutMapModelPlugIn {
    pub unsafe fn Initialize<P0, P1, P2>(&self, bstrxml: P0, psrcplugin: P1, pdestplugin: P2, psrcgbd: *const GamutBoundaryDescription, pdestgbd: *const GamutBoundaryDescription) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<IDeviceModelPlugIn>,
        P2: ::windows::core::IntoParam<IDeviceModelPlugIn>,
    {
        (::windows::core::Interface::vtable(self).Initialize)(::windows::core::Interface::as_raw(self), bstrxml.into_param().abi(), psrcplugin.into_param().abi(), pdestplugin.into_param().abi(), psrcgbd, pdestgbd).ok()
    }
    pub unsafe fn SourceToDestinationAppearanceColors(&self, ccolors: u32, pinputcolors: *const JChColorF, poutputcolors: *mut JChColorF) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SourceToDestinationAppearanceColors)(::windows::core::Interface::as_raw(self), ccolors, pinputcolors, poutputcolors).ok()
    }
}
::windows::imp::interface_hierarchy!(IGamutMapModelPlugIn, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IGamutMapModelPlugIn {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGamutMapModelPlugIn {}
impl ::core::fmt::Debug for IGamutMapModelPlugIn {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGamutMapModelPlugIn").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IGamutMapModelPlugIn {
    type Vtable = IGamutMapModelPlugIn_Vtbl;
}
impl ::core::clone::Clone for IGamutMapModelPlugIn {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGamutMapModelPlugIn {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2dd80115_ad1e_41f6_a219_a4f4b583d1f9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGamutMapModelPlugIn_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrxml: ::std::mem::MaybeUninit<::windows::core::BSTR>, psrcplugin: *mut ::core::ffi::c_void, pdestplugin: *mut ::core::ffi::c_void, psrcgbd: *const GamutBoundaryDescription, pdestgbd: *const GamutBoundaryDescription) -> ::windows::core::HRESULT,
    pub SourceToDestinationAppearanceColors: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ccolors: u32, pinputcolors: *const JChColorF, poutputcolors: *mut JChColorF) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const ATTRIB_MATTE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const ATTRIB_TRANSPARENCY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const BEST_MODE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const CATID_WcsPlugin: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa0b402e0_8240_405f_8a16_8a5b4df2f0dd);
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
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const ENABLE_GAMUT_CHECKING: u32 = 65536u32;
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
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const MAX_COLOR_CHANNELS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const NORMAL_MODE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const PRESERVEBLACK: u32 = 1048576u32;
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
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const RESERVED: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const SEQUENTIAL_TRANSFORM: u32 = 2155872256u32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const USE_RELATIVE_COLORIMETRIC: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const WCS_ALWAYS: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const WCS_DEFAULT: i32 = 0i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const WCS_ICCONLY: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct BMFORMAT(pub i32);
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const BM_x555RGB: BMFORMAT = BMFORMAT(0i32);
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const BM_x555XYZ: BMFORMAT = BMFORMAT(257i32);
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const BM_x555Yxy: BMFORMAT = BMFORMAT(258i32);
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const BM_x555Lab: BMFORMAT = BMFORMAT(259i32);
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const BM_x555G3CH: BMFORMAT = BMFORMAT(260i32);
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const BM_RGBTRIPLETS: BMFORMAT = BMFORMAT(2i32);
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const BM_BGRTRIPLETS: BMFORMAT = BMFORMAT(4i32);
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const BM_XYZTRIPLETS: BMFORMAT = BMFORMAT(513i32);
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const BM_YxyTRIPLETS: BMFORMAT = BMFORMAT(514i32);
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const BM_LabTRIPLETS: BMFORMAT = BMFORMAT(515i32);
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const BM_G3CHTRIPLETS: BMFORMAT = BMFORMAT(516i32);
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const BM_5CHANNEL: BMFORMAT = BMFORMAT(517i32);
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const BM_6CHANNEL: BMFORMAT = BMFORMAT(518i32);
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const BM_7CHANNEL: BMFORMAT = BMFORMAT(519i32);
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const BM_8CHANNEL: BMFORMAT = BMFORMAT(520i32);
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const BM_GRAY: BMFORMAT = BMFORMAT(521i32);
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const BM_xRGBQUADS: BMFORMAT = BMFORMAT(8i32);
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const BM_xBGRQUADS: BMFORMAT = BMFORMAT(16i32);
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const BM_xG3CHQUADS: BMFORMAT = BMFORMAT(772i32);
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const BM_KYMCQUADS: BMFORMAT = BMFORMAT(773i32);
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const BM_CMYKQUADS: BMFORMAT = BMFORMAT(32i32);
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const BM_10b_RGB: BMFORMAT = BMFORMAT(9i32);
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const BM_10b_XYZ: BMFORMAT = BMFORMAT(1025i32);
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const BM_10b_Yxy: BMFORMAT = BMFORMAT(1026i32);
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const BM_10b_Lab: BMFORMAT = BMFORMAT(1027i32);
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const BM_10b_G3CH: BMFORMAT = BMFORMAT(1028i32);
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const BM_NAMED_INDEX: BMFORMAT = BMFORMAT(1029i32);
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const BM_16b_RGB: BMFORMAT = BMFORMAT(10i32);
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const BM_16b_XYZ: BMFORMAT = BMFORMAT(1281i32);
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const BM_16b_Yxy: BMFORMAT = BMFORMAT(1282i32);
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const BM_16b_Lab: BMFORMAT = BMFORMAT(1283i32);
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const BM_16b_G3CH: BMFORMAT = BMFORMAT(1284i32);
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const BM_16b_GRAY: BMFORMAT = BMFORMAT(1285i32);
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const BM_565RGB: BMFORMAT = BMFORMAT(1i32);
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const BM_32b_scRGB: BMFORMAT = BMFORMAT(1537i32);
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const BM_32b_scARGB: BMFORMAT = BMFORMAT(1538i32);
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const BM_S2DOT13FIXED_scRGB: BMFORMAT = BMFORMAT(1539i32);
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const BM_S2DOT13FIXED_scARGB: BMFORMAT = BMFORMAT(1540i32);
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const BM_R10G10B10A2: BMFORMAT = BMFORMAT(1793i32);
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const BM_R10G10B10A2_XR: BMFORMAT = BMFORMAT(1794i32);
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const BM_R16G16B16A16_FLOAT: BMFORMAT = BMFORMAT(1795i32);
impl ::core::marker::Copy for BMFORMAT {}
impl ::core::clone::Clone for BMFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BMFORMAT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for BMFORMAT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for BMFORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BMFORMAT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct COLORDATATYPE(pub i32);
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const COLOR_BYTE: COLORDATATYPE = COLORDATATYPE(1i32);
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const COLOR_WORD: COLORDATATYPE = COLORDATATYPE(2i32);
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const COLOR_FLOAT: COLORDATATYPE = COLORDATATYPE(3i32);
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const COLOR_S2DOT13FIXED: COLORDATATYPE = COLORDATATYPE(4i32);
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const COLOR_10b_R10G10B10A2: COLORDATATYPE = COLORDATATYPE(5i32);
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const COLOR_10b_R10G10B10A2_XR: COLORDATATYPE = COLORDATATYPE(6i32);
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const COLOR_FLOAT16: COLORDATATYPE = COLORDATATYPE(7i32);
impl ::core::marker::Copy for COLORDATATYPE {}
impl ::core::clone::Clone for COLORDATATYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COLORDATATYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for COLORDATATYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for COLORDATATYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COLORDATATYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct COLORPROFILESUBTYPE(pub i32);
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const CPST_PERCEPTUAL: COLORPROFILESUBTYPE = COLORPROFILESUBTYPE(0i32);
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const CPST_RELATIVE_COLORIMETRIC: COLORPROFILESUBTYPE = COLORPROFILESUBTYPE(1i32);
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const CPST_SATURATION: COLORPROFILESUBTYPE = COLORPROFILESUBTYPE(2i32);
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const CPST_ABSOLUTE_COLORIMETRIC: COLORPROFILESUBTYPE = COLORPROFILESUBTYPE(3i32);
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const CPST_NONE: COLORPROFILESUBTYPE = COLORPROFILESUBTYPE(4i32);
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const CPST_RGB_WORKING_SPACE: COLORPROFILESUBTYPE = COLORPROFILESUBTYPE(5i32);
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const CPST_CUSTOM_WORKING_SPACE: COLORPROFILESUBTYPE = COLORPROFILESUBTYPE(6i32);
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const CPST_STANDARD_DISPLAY_COLOR_MODE: COLORPROFILESUBTYPE = COLORPROFILESUBTYPE(7i32);
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const CPST_EXTENDED_DISPLAY_COLOR_MODE: COLORPROFILESUBTYPE = COLORPROFILESUBTYPE(8i32);
impl ::core::marker::Copy for COLORPROFILESUBTYPE {}
impl ::core::clone::Clone for COLORPROFILESUBTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COLORPROFILESUBTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for COLORPROFILESUBTYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for COLORPROFILESUBTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COLORPROFILESUBTYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct COLORPROFILETYPE(pub i32);
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const CPT_ICC: COLORPROFILETYPE = COLORPROFILETYPE(0i32);
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const CPT_DMP: COLORPROFILETYPE = COLORPROFILETYPE(1i32);
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const CPT_CAMP: COLORPROFILETYPE = COLORPROFILETYPE(2i32);
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const CPT_GMMP: COLORPROFILETYPE = COLORPROFILETYPE(3i32);
impl ::core::marker::Copy for COLORPROFILETYPE {}
impl ::core::clone::Clone for COLORPROFILETYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COLORPROFILETYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for COLORPROFILETYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for COLORPROFILETYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COLORPROFILETYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct COLORTYPE(pub i32);
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const COLOR_GRAY: COLORTYPE = COLORTYPE(1i32);
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const COLOR_RGB: COLORTYPE = COLORTYPE(2i32);
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const COLOR_XYZ: COLORTYPE = COLORTYPE(3i32);
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const COLOR_Yxy: COLORTYPE = COLORTYPE(4i32);
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const COLOR_Lab: COLORTYPE = COLORTYPE(5i32);
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const COLOR_3_CHANNEL: COLORTYPE = COLORTYPE(6i32);
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const COLOR_CMYK: COLORTYPE = COLORTYPE(7i32);
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const COLOR_5_CHANNEL: COLORTYPE = COLORTYPE(8i32);
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const COLOR_6_CHANNEL: COLORTYPE = COLORTYPE(9i32);
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const COLOR_7_CHANNEL: COLORTYPE = COLORTYPE(10i32);
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const COLOR_8_CHANNEL: COLORTYPE = COLORTYPE(11i32);
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const COLOR_NAMED: COLORTYPE = COLORTYPE(12i32);
impl ::core::marker::Copy for COLORTYPE {}
impl ::core::clone::Clone for COLORTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COLORTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for COLORTYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for COLORTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COLORTYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct COLOR_MATCH_TO_TARGET_ACTION(pub i32);
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const CS_ENABLE: COLOR_MATCH_TO_TARGET_ACTION = COLOR_MATCH_TO_TARGET_ACTION(1i32);
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const CS_DISABLE: COLOR_MATCH_TO_TARGET_ACTION = COLOR_MATCH_TO_TARGET_ACTION(2i32);
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const CS_DELETE_TRANSFORM: COLOR_MATCH_TO_TARGET_ACTION = COLOR_MATCH_TO_TARGET_ACTION(3i32);
impl ::core::marker::Copy for COLOR_MATCH_TO_TARGET_ACTION {}
impl ::core::clone::Clone for COLOR_MATCH_TO_TARGET_ACTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COLOR_MATCH_TO_TARGET_ACTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for COLOR_MATCH_TO_TARGET_ACTION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for COLOR_MATCH_TO_TARGET_ACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COLOR_MATCH_TO_TARGET_ACTION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ICM_COMMAND(pub u32);
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const ICM_ADDPROFILE: ICM_COMMAND = ICM_COMMAND(1u32);
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const ICM_DELETEPROFILE: ICM_COMMAND = ICM_COMMAND(2u32);
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const ICM_QUERYPROFILE: ICM_COMMAND = ICM_COMMAND(3u32);
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const ICM_SETDEFAULTPROFILE: ICM_COMMAND = ICM_COMMAND(4u32);
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const ICM_REGISTERICMATCHER: ICM_COMMAND = ICM_COMMAND(5u32);
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const ICM_UNREGISTERICMATCHER: ICM_COMMAND = ICM_COMMAND(6u32);
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const ICM_QUERYMATCH: ICM_COMMAND = ICM_COMMAND(7u32);
impl ::core::marker::Copy for ICM_COMMAND {}
impl ::core::clone::Clone for ICM_COMMAND {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ICM_COMMAND {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for ICM_COMMAND {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for ICM_COMMAND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICM_COMMAND").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ICM_MODE(pub i32);
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const ICM_OFF: ICM_MODE = ICM_MODE(1i32);
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const ICM_ON: ICM_MODE = ICM_MODE(2i32);
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const ICM_QUERY: ICM_MODE = ICM_MODE(3i32);
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const ICM_DONE_OUTSIDEDC: ICM_MODE = ICM_MODE(4i32);
impl ::core::marker::Copy for ICM_MODE {}
impl ::core::clone::Clone for ICM_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ICM_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for ICM_MODE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for ICM_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICM_MODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WCS_DEVICE_CAPABILITIES_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const VideoCardGammaTable: WCS_DEVICE_CAPABILITIES_TYPE = WCS_DEVICE_CAPABILITIES_TYPE(1i32);
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const MicrosoftHardwareColorV2: WCS_DEVICE_CAPABILITIES_TYPE = WCS_DEVICE_CAPABILITIES_TYPE(2i32);
impl ::core::marker::Copy for WCS_DEVICE_CAPABILITIES_TYPE {}
impl ::core::clone::Clone for WCS_DEVICE_CAPABILITIES_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WCS_DEVICE_CAPABILITIES_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WCS_DEVICE_CAPABILITIES_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WCS_DEVICE_CAPABILITIES_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WCS_DEVICE_CAPABILITIES_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WCS_PROFILE_MANAGEMENT_SCOPE(pub i32);
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const WCS_PROFILE_MANAGEMENT_SCOPE_SYSTEM_WIDE: WCS_PROFILE_MANAGEMENT_SCOPE = WCS_PROFILE_MANAGEMENT_SCOPE(0i32);
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub const WCS_PROFILE_MANAGEMENT_SCOPE_CURRENT_USER: WCS_PROFILE_MANAGEMENT_SCOPE = WCS_PROFILE_MANAGEMENT_SCOPE(1i32);
impl ::core::marker::Copy for WCS_PROFILE_MANAGEMENT_SCOPE {}
impl ::core::clone::Clone for WCS_PROFILE_MANAGEMENT_SCOPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WCS_PROFILE_MANAGEMENT_SCOPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WCS_PROFILE_MANAGEMENT_SCOPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WCS_PROFILE_MANAGEMENT_SCOPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WCS_PROFILE_MANAGEMENT_SCOPE").field(&self.0).finish()
    }
}
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for BlackInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BlackInformation").field("fBlackOnly", &self.fBlackOnly).field("blackWeight", &self.blackWeight).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for BlackInformation {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for BlackInformation {
    fn eq(&self, other: &Self) -> bool {
        self.fBlackOnly == other.fBlackOnly && self.blackWeight == other.blackWeight
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for BlackInformation {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for BlackInformation {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
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
impl ::core::fmt::Debug for CMYKCOLOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CMYKCOLOR").field("cyan", &self.cyan).field("magenta", &self.magenta).field("yellow", &self.yellow).field("black", &self.black).finish()
    }
}
impl ::windows::core::TypeKind for CMYKCOLOR {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for CMYKCOLOR {
    fn eq(&self, other: &Self) -> bool {
        self.cyan == other.cyan && self.magenta == other.magenta && self.yellow == other.yellow && self.black == other.black
    }
}
impl ::core::cmp::Eq for CMYKCOLOR {}
impl ::core::default::Default for CMYKCOLOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::windows::core::TypeKind for COLOR {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for COLOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for COLOR_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COLOR_0").field("reserved1", &self.reserved1).field("reserved2", &self.reserved2).finish()
    }
}
impl ::windows::core::TypeKind for COLOR_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for COLOR_0 {
    fn eq(&self, other: &Self) -> bool {
        self.reserved1 == other.reserved1 && self.reserved2 == other.reserved2
    }
}
impl ::core::cmp::Eq for COLOR_0 {}
impl ::core::default::Default for COLOR_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct COLORMATCHSETUPA {
    pub dwSize: u32,
    pub dwVersion: u32,
    pub dwFlags: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub pSourceName: ::windows::core::PCSTR,
    pub pDisplayName: ::windows::core::PCSTR,
    pub pPrinterName: ::windows::core::PCSTR,
    pub dwRenderIntent: u32,
    pub dwProofingIntent: u32,
    pub pMonitorProfile: ::windows::core::PSTR,
    pub ccMonitorProfile: u32,
    pub pPrinterProfile: ::windows::core::PSTR,
    pub ccPrinterProfile: u32,
    pub pTargetProfile: ::windows::core::PSTR,
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::fmt::Debug for COLORMATCHSETUPA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COLORMATCHSETUPA")
            .field("dwSize", &self.dwSize)
            .field("dwVersion", &self.dwVersion)
            .field("dwFlags", &self.dwFlags)
            .field("hwndOwner", &self.hwndOwner)
            .field("pSourceName", &self.pSourceName)
            .field("pDisplayName", &self.pDisplayName)
            .field("pPrinterName", &self.pPrinterName)
            .field("dwRenderIntent", &self.dwRenderIntent)
            .field("dwProofingIntent", &self.dwProofingIntent)
            .field("pMonitorProfile", &self.pMonitorProfile)
            .field("ccMonitorProfile", &self.ccMonitorProfile)
            .field("pPrinterProfile", &self.pPrinterProfile)
            .field("ccPrinterProfile", &self.ccPrinterProfile)
            .field("pTargetProfile", &self.pTargetProfile)
            .field("ccTargetProfile", &self.ccTargetProfile)
            .field("lParam", &self.lParam)
            .field("lParamApplyCallback", &self.lParamApplyCallback)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::windows::core::TypeKind for COLORMATCHSETUPA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for COLORMATCHSETUPA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
    pub pSourceName: ::windows::core::PCWSTR,
    pub pDisplayName: ::windows::core::PCWSTR,
    pub pPrinterName: ::windows::core::PCWSTR,
    pub dwRenderIntent: u32,
    pub dwProofingIntent: u32,
    pub pMonitorProfile: ::windows::core::PWSTR,
    pub ccMonitorProfile: u32,
    pub pPrinterProfile: ::windows::core::PWSTR,
    pub ccPrinterProfile: u32,
    pub pTargetProfile: ::windows::core::PWSTR,
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::fmt::Debug for COLORMATCHSETUPW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COLORMATCHSETUPW")
            .field("dwSize", &self.dwSize)
            .field("dwVersion", &self.dwVersion)
            .field("dwFlags", &self.dwFlags)
            .field("hwndOwner", &self.hwndOwner)
            .field("pSourceName", &self.pSourceName)
            .field("pDisplayName", &self.pDisplayName)
            .field("pPrinterName", &self.pPrinterName)
            .field("dwRenderIntent", &self.dwRenderIntent)
            .field("dwProofingIntent", &self.dwProofingIntent)
            .field("pMonitorProfile", &self.pMonitorProfile)
            .field("ccMonitorProfile", &self.ccMonitorProfile)
            .field("pPrinterProfile", &self.pPrinterProfile)
            .field("ccPrinterProfile", &self.ccPrinterProfile)
            .field("pTargetProfile", &self.pTargetProfile)
            .field("ccTargetProfile", &self.ccTargetProfile)
            .field("lParam", &self.lParam)
            .field("lParamApplyCallback", &self.lParamApplyCallback)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::windows::core::TypeKind for COLORMATCHSETUPW {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for COLORMATCHSETUPW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub struct EMRCREATECOLORSPACE {
    pub emr: super::super::Graphics::Gdi::EMR,
    pub ihCS: u32,
    pub lcs: LOGCOLORSPACEA,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::marker::Copy for EMRCREATECOLORSPACE {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::clone::Clone for EMRCREATECOLORSPACE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::fmt::Debug for EMRCREATECOLORSPACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRCREATECOLORSPACE").field("emr", &self.emr).field("ihCS", &self.ihCS).field("lcs", &self.lcs).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::windows::core::TypeKind for EMRCREATECOLORSPACE {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::PartialEq for EMRCREATECOLORSPACE {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.ihCS == other.ihCS && self.lcs == other.lcs
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::Eq for EMRCREATECOLORSPACE {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for EMRCREATECOLORSPACE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::fmt::Debug for EMRCREATECOLORSPACEW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRCREATECOLORSPACEW").field("emr", &self.emr).field("ihCS", &self.ihCS).field("lcs", &self.lcs).field("dwFlags", &self.dwFlags).field("cbData", &self.cbData).field("Data", &self.Data).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::windows::core::TypeKind for EMRCREATECOLORSPACEW {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::PartialEq for EMRCREATECOLORSPACEW {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.ihCS == other.ihCS && self.lcs == other.lcs && self.dwFlags == other.dwFlags && self.cbData == other.cbData && self.Data == other.Data
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::Eq for EMRCREATECOLORSPACEW {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for EMRCREATECOLORSPACEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub struct ENUMTYPEA {
    pub dwSize: u32,
    pub dwVersion: u32,
    pub dwFields: u32,
    pub pDeviceName: ::windows::core::PCSTR,
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
impl ::core::fmt::Debug for ENUMTYPEA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENUMTYPEA")
            .field("dwSize", &self.dwSize)
            .field("dwVersion", &self.dwVersion)
            .field("dwFields", &self.dwFields)
            .field("pDeviceName", &self.pDeviceName)
            .field("dwMediaType", &self.dwMediaType)
            .field("dwDitheringMode", &self.dwDitheringMode)
            .field("dwResolution", &self.dwResolution)
            .field("dwCMMType", &self.dwCMMType)
            .field("dwClass", &self.dwClass)
            .field("dwDataColorSpace", &self.dwDataColorSpace)
            .field("dwConnectionSpace", &self.dwConnectionSpace)
            .field("dwSignature", &self.dwSignature)
            .field("dwPlatform", &self.dwPlatform)
            .field("dwProfileFlags", &self.dwProfileFlags)
            .field("dwManufacturer", &self.dwManufacturer)
            .field("dwModel", &self.dwModel)
            .field("dwAttributes", &self.dwAttributes)
            .field("dwRenderingIntent", &self.dwRenderingIntent)
            .field("dwCreator", &self.dwCreator)
            .field("dwDeviceClass", &self.dwDeviceClass)
            .finish()
    }
}
impl ::windows::core::TypeKind for ENUMTYPEA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for ENUMTYPEA {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
            && self.dwVersion == other.dwVersion
            && self.dwFields == other.dwFields
            && self.pDeviceName == other.pDeviceName
            && self.dwMediaType == other.dwMediaType
            && self.dwDitheringMode == other.dwDitheringMode
            && self.dwResolution == other.dwResolution
            && self.dwCMMType == other.dwCMMType
            && self.dwClass == other.dwClass
            && self.dwDataColorSpace == other.dwDataColorSpace
            && self.dwConnectionSpace == other.dwConnectionSpace
            && self.dwSignature == other.dwSignature
            && self.dwPlatform == other.dwPlatform
            && self.dwProfileFlags == other.dwProfileFlags
            && self.dwManufacturer == other.dwManufacturer
            && self.dwModel == other.dwModel
            && self.dwAttributes == other.dwAttributes
            && self.dwRenderingIntent == other.dwRenderingIntent
            && self.dwCreator == other.dwCreator
            && self.dwDeviceClass == other.dwDeviceClass
    }
}
impl ::core::cmp::Eq for ENUMTYPEA {}
impl ::core::default::Default for ENUMTYPEA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`*"]
pub struct ENUMTYPEW {
    pub dwSize: u32,
    pub dwVersion: u32,
    pub dwFields: u32,
    pub pDeviceName: ::windows::core::PCWSTR,
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
impl ::core::fmt::Debug for ENUMTYPEW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENUMTYPEW")
            .field("dwSize", &self.dwSize)
            .field("dwVersion", &self.dwVersion)
            .field("dwFields", &self.dwFields)
            .field("pDeviceName", &self.pDeviceName)
            .field("dwMediaType", &self.dwMediaType)
            .field("dwDitheringMode", &self.dwDitheringMode)
            .field("dwResolution", &self.dwResolution)
            .field("dwCMMType", &self.dwCMMType)
            .field("dwClass", &self.dwClass)
            .field("dwDataColorSpace", &self.dwDataColorSpace)
            .field("dwConnectionSpace", &self.dwConnectionSpace)
            .field("dwSignature", &self.dwSignature)
            .field("dwPlatform", &self.dwPlatform)
            .field("dwProfileFlags", &self.dwProfileFlags)
            .field("dwManufacturer", &self.dwManufacturer)
            .field("dwModel", &self.dwModel)
            .field("dwAttributes", &self.dwAttributes)
            .field("dwRenderingIntent", &self.dwRenderingIntent)
            .field("dwCreator", &self.dwCreator)
            .field("dwDeviceClass", &self.dwDeviceClass)
            .finish()
    }
}
impl ::windows::core::TypeKind for ENUMTYPEW {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for ENUMTYPEW {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
            && self.dwVersion == other.dwVersion
            && self.dwFields == other.dwFields
            && self.pDeviceName == other.pDeviceName
            && self.dwMediaType == other.dwMediaType
            && self.dwDitheringMode == other.dwDitheringMode
            && self.dwResolution == other.dwResolution
            && self.dwCMMType == other.dwCMMType
            && self.dwClass == other.dwClass
            && self.dwDataColorSpace == other.dwDataColorSpace
            && self.dwConnectionSpace == other.dwConnectionSpace
            && self.dwSignature == other.dwSignature
            && self.dwPlatform == other.dwPlatform
            && self.dwProfileFlags == other.dwProfileFlags
            && self.dwManufacturer == other.dwManufacturer
            && self.dwModel == other.dwModel
            && self.dwAttributes == other.dwAttributes
            && self.dwRenderingIntent == other.dwRenderingIntent
            && self.dwCreator == other.dwCreator
            && self.dwDeviceClass == other.dwDeviceClass
    }
}
impl ::core::cmp::Eq for ENUMTYPEW {}
impl ::core::default::Default for ENUMTYPEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
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
impl ::core::fmt::Debug for GENERIC3CHANNEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GENERIC3CHANNEL").field("ch1", &self.ch1).field("ch2", &self.ch2).field("ch3", &self.ch3).finish()
    }
}
impl ::windows::core::TypeKind for GENERIC3CHANNEL {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for GENERIC3CHANNEL {
    fn eq(&self, other: &Self) -> bool {
        self.ch1 == other.ch1 && self.ch2 == other.ch2 && self.ch3 == other.ch3
    }
}
impl ::core::cmp::Eq for GENERIC3CHANNEL {}
impl ::core::default::Default for GENERIC3CHANNEL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for GRAYCOLOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GRAYCOLOR").field("gray", &self.gray).finish()
    }
}
impl ::windows::core::TypeKind for GRAYCOLOR {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for GRAYCOLOR {
    fn eq(&self, other: &Self) -> bool {
        self.gray == other.gray
    }
}
impl ::core::cmp::Eq for GRAYCOLOR {}
impl ::core::default::Default for GRAYCOLOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for GamutBoundaryDescription {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GamutBoundaryDescription").field("pPrimaries", &self.pPrimaries).field("cNeutralSamples", &self.cNeutralSamples).field("pNeutralSamples", &self.pNeutralSamples).field("pReferenceShell", &self.pReferenceShell).field("pPlausibleShell", &self.pPlausibleShell).field("pPossibleShell", &self.pPossibleShell).finish()
    }
}
impl ::windows::core::TypeKind for GamutBoundaryDescription {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for GamutBoundaryDescription {
    fn eq(&self, other: &Self) -> bool {
        self.pPrimaries == other.pPrimaries && self.cNeutralSamples == other.cNeutralSamples && self.pNeutralSamples == other.pNeutralSamples && self.pReferenceShell == other.pReferenceShell && self.pPlausibleShell == other.pPlausibleShell && self.pPossibleShell == other.pPossibleShell
    }
}
impl ::core::cmp::Eq for GamutBoundaryDescription {}
impl ::core::default::Default for GamutBoundaryDescription {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for GamutShell {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GamutShell").field("JMin", &self.JMin).field("JMax", &self.JMax).field("cVertices", &self.cVertices).field("cTriangles", &self.cTriangles).field("pVertices", &self.pVertices).field("pTriangles", &self.pTriangles).finish()
    }
}
impl ::windows::core::TypeKind for GamutShell {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for GamutShell {
    fn eq(&self, other: &Self) -> bool {
        self.JMin == other.JMin && self.JMax == other.JMax && self.cVertices == other.cVertices && self.cTriangles == other.cTriangles && self.pVertices == other.pVertices && self.pTriangles == other.pTriangles
    }
}
impl ::core::cmp::Eq for GamutShell {}
impl ::core::default::Default for GamutShell {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for GamutShellTriangle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GamutShellTriangle").field("aVertexIndex", &self.aVertexIndex).finish()
    }
}
impl ::windows::core::TypeKind for GamutShellTriangle {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for GamutShellTriangle {
    fn eq(&self, other: &Self) -> bool {
        self.aVertexIndex == other.aVertexIndex
    }
}
impl ::core::cmp::Eq for GamutShellTriangle {}
impl ::core::default::Default for GamutShellTriangle {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HCOLORSPACE(pub isize);
impl HCOLORSPACE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HCOLORSPACE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HCOLORSPACE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HCOLORSPACE {}
impl ::core::fmt::Debug for HCOLORSPACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HCOLORSPACE").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for HCOLORSPACE {
    type TypeKind = ::windows::core::CopyType;
}
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
impl ::core::fmt::Debug for HiFiCOLOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HiFiCOLOR").field("channel", &self.channel).finish()
    }
}
impl ::windows::core::TypeKind for HiFiCOLOR {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for HiFiCOLOR {
    fn eq(&self, other: &Self) -> bool {
        self.channel == other.channel
    }
}
impl ::core::cmp::Eq for HiFiCOLOR {}
impl ::core::default::Default for HiFiCOLOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
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
impl ::core::fmt::Debug for JChColorF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JChColorF").field("J", &self.J).field("C", &self.C).field("h", &self.h).finish()
    }
}
impl ::windows::core::TypeKind for JChColorF {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for JChColorF {
    fn eq(&self, other: &Self) -> bool {
        self.J == other.J && self.C == other.C && self.h == other.h
    }
}
impl ::core::cmp::Eq for JChColorF {}
impl ::core::default::Default for JChColorF {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for JabColorF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JabColorF").field("J", &self.J).field("a", &self.a).field("b", &self.b).finish()
    }
}
impl ::windows::core::TypeKind for JabColorF {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for JabColorF {
    fn eq(&self, other: &Self) -> bool {
        self.J == other.J && self.a == other.a && self.b == other.b
    }
}
impl ::core::cmp::Eq for JabColorF {}
impl ::core::default::Default for JabColorF {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
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
    pub lcsFilename: [u8; 260],
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::marker::Copy for LOGCOLORSPACEA {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::clone::Clone for LOGCOLORSPACEA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::fmt::Debug for LOGCOLORSPACEA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LOGCOLORSPACEA").field("lcsSignature", &self.lcsSignature).field("lcsVersion", &self.lcsVersion).field("lcsSize", &self.lcsSize).field("lcsCSType", &self.lcsCSType).field("lcsIntent", &self.lcsIntent).field("lcsEndpoints", &self.lcsEndpoints).field("lcsGammaRed", &self.lcsGammaRed).field("lcsGammaGreen", &self.lcsGammaGreen).field("lcsGammaBlue", &self.lcsGammaBlue).field("lcsFilename", &self.lcsFilename).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::windows::core::TypeKind for LOGCOLORSPACEA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::PartialEq for LOGCOLORSPACEA {
    fn eq(&self, other: &Self) -> bool {
        self.lcsSignature == other.lcsSignature && self.lcsVersion == other.lcsVersion && self.lcsSize == other.lcsSize && self.lcsCSType == other.lcsCSType && self.lcsIntent == other.lcsIntent && self.lcsEndpoints == other.lcsEndpoints && self.lcsGammaRed == other.lcsGammaRed && self.lcsGammaGreen == other.lcsGammaGreen && self.lcsGammaBlue == other.lcsGammaBlue && self.lcsFilename == other.lcsFilename
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::Eq for LOGCOLORSPACEA {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for LOGCOLORSPACEA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::fmt::Debug for LOGCOLORSPACEW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LOGCOLORSPACEW").field("lcsSignature", &self.lcsSignature).field("lcsVersion", &self.lcsVersion).field("lcsSize", &self.lcsSize).field("lcsCSType", &self.lcsCSType).field("lcsIntent", &self.lcsIntent).field("lcsEndpoints", &self.lcsEndpoints).field("lcsGammaRed", &self.lcsGammaRed).field("lcsGammaGreen", &self.lcsGammaGreen).field("lcsGammaBlue", &self.lcsGammaBlue).field("lcsFilename", &self.lcsFilename).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::windows::core::TypeKind for LOGCOLORSPACEW {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::PartialEq for LOGCOLORSPACEW {
    fn eq(&self, other: &Self) -> bool {
        self.lcsSignature == other.lcsSignature && self.lcsVersion == other.lcsVersion && self.lcsSize == other.lcsSize && self.lcsCSType == other.lcsCSType && self.lcsIntent == other.lcsIntent && self.lcsEndpoints == other.lcsEndpoints && self.lcsGammaRed == other.lcsGammaRed && self.lcsGammaGreen == other.lcsGammaGreen && self.lcsGammaBlue == other.lcsGammaBlue && self.lcsFilename == other.lcsFilename
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::Eq for LOGCOLORSPACEW {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for LOGCOLORSPACEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
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
impl ::core::fmt::Debug for LabCOLOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LabCOLOR").field("L", &self.L).field("a", &self.a).field("b", &self.b).finish()
    }
}
impl ::windows::core::TypeKind for LabCOLOR {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for LabCOLOR {
    fn eq(&self, other: &Self) -> bool {
        self.L == other.L && self.a == other.a && self.b == other.b
    }
}
impl ::core::cmp::Eq for LabCOLOR {}
impl ::core::default::Default for LabCOLOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
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
impl ::core::fmt::Debug for NAMEDCOLOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NAMEDCOLOR").field("dwIndex", &self.dwIndex).finish()
    }
}
impl ::windows::core::TypeKind for NAMEDCOLOR {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NAMEDCOLOR {
    fn eq(&self, other: &Self) -> bool {
        self.dwIndex == other.dwIndex
    }
}
impl ::core::cmp::Eq for NAMEDCOLOR {}
impl ::core::default::Default for NAMEDCOLOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for NAMED_PROFILE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NAMED_PROFILE_INFO").field("dwFlags", &self.dwFlags).field("dwCount", &self.dwCount).field("dwCountDevCoordinates", &self.dwCountDevCoordinates).field("szPrefix", &self.szPrefix).field("szSuffix", &self.szSuffix).finish()
    }
}
impl ::windows::core::TypeKind for NAMED_PROFILE_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NAMED_PROFILE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags && self.dwCount == other.dwCount && self.dwCountDevCoordinates == other.dwCountDevCoordinates && self.szPrefix == other.szPrefix && self.szSuffix == other.szSuffix
    }
}
impl ::core::cmp::Eq for NAMED_PROFILE_INFO {}
impl ::core::default::Default for NAMED_PROFILE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
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
impl ::core::fmt::Debug for PROFILE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROFILE").field("dwType", &self.dwType).field("pProfileData", &self.pProfileData).field("cbDataSize", &self.cbDataSize).finish()
    }
}
impl ::windows::core::TypeKind for PROFILE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for PROFILE {
    fn eq(&self, other: &Self) -> bool {
        self.dwType == other.dwType && self.pProfileData == other.pProfileData && self.cbDataSize == other.cbDataSize
    }
}
impl ::core::cmp::Eq for PROFILE {}
impl ::core::default::Default for PROFILE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::fmt::Debug for PROFILEHEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROFILEHEADER")
            .field("phSize", &self.phSize)
            .field("phCMMType", &self.phCMMType)
            .field("phVersion", &self.phVersion)
            .field("phClass", &self.phClass)
            .field("phDataColorSpace", &self.phDataColorSpace)
            .field("phConnectionSpace", &self.phConnectionSpace)
            .field("phDateTime", &self.phDateTime)
            .field("phSignature", &self.phSignature)
            .field("phPlatform", &self.phPlatform)
            .field("phProfileFlags", &self.phProfileFlags)
            .field("phManufacturer", &self.phManufacturer)
            .field("phModel", &self.phModel)
            .field("phAttributes", &self.phAttributes)
            .field("phRenderingIntent", &self.phRenderingIntent)
            .field("phIlluminant", &self.phIlluminant)
            .field("phCreator", &self.phCreator)
            .field("phReserved", &self.phReserved)
            .finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::windows::core::TypeKind for PROFILEHEADER {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::PartialEq for PROFILEHEADER {
    fn eq(&self, other: &Self) -> bool {
        self.phSize == other.phSize && self.phCMMType == other.phCMMType && self.phVersion == other.phVersion && self.phClass == other.phClass && self.phDataColorSpace == other.phDataColorSpace && self.phConnectionSpace == other.phConnectionSpace && self.phDateTime == other.phDateTime && self.phSignature == other.phSignature && self.phPlatform == other.phPlatform && self.phProfileFlags == other.phProfileFlags && self.phManufacturer == other.phManufacturer && self.phModel == other.phModel && self.phAttributes == other.phAttributes && self.phRenderingIntent == other.phRenderingIntent && self.phIlluminant == other.phIlluminant && self.phCreator == other.phCreator && self.phReserved == other.phReserved
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::Eq for PROFILEHEADER {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for PROFILEHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
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
impl ::core::fmt::Debug for PrimaryJabColors {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PrimaryJabColors").field("red", &self.red).field("yellow", &self.yellow).field("green", &self.green).field("cyan", &self.cyan).field("blue", &self.blue).field("magenta", &self.magenta).field("black", &self.black).field("white", &self.white).finish()
    }
}
impl ::windows::core::TypeKind for PrimaryJabColors {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for PrimaryJabColors {
    fn eq(&self, other: &Self) -> bool {
        self.red == other.red && self.yellow == other.yellow && self.green == other.green && self.cyan == other.cyan && self.blue == other.blue && self.magenta == other.magenta && self.black == other.black && self.white == other.white
    }
}
impl ::core::cmp::Eq for PrimaryJabColors {}
impl ::core::default::Default for PrimaryJabColors {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for PrimaryXYZColors {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PrimaryXYZColors").field("red", &self.red).field("yellow", &self.yellow).field("green", &self.green).field("cyan", &self.cyan).field("blue", &self.blue).field("magenta", &self.magenta).field("black", &self.black).field("white", &self.white).finish()
    }
}
impl ::windows::core::TypeKind for PrimaryXYZColors {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for PrimaryXYZColors {
    fn eq(&self, other: &Self) -> bool {
        self.red == other.red && self.yellow == other.yellow && self.green == other.green && self.cyan == other.cyan && self.blue == other.blue && self.magenta == other.magenta && self.black == other.black && self.white == other.white
    }
}
impl ::core::cmp::Eq for PrimaryXYZColors {}
impl ::core::default::Default for PrimaryXYZColors {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
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
impl ::core::fmt::Debug for RGBCOLOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RGBCOLOR").field("red", &self.red).field("green", &self.green).field("blue", &self.blue).finish()
    }
}
impl ::windows::core::TypeKind for RGBCOLOR {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for RGBCOLOR {
    fn eq(&self, other: &Self) -> bool {
        self.red == other.red && self.green == other.green && self.blue == other.blue
    }
}
impl ::core::cmp::Eq for RGBCOLOR {}
impl ::core::default::Default for RGBCOLOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WCS_DEVICE_MHC2_CAPABILITIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WCS_DEVICE_MHC2_CAPABILITIES").field("Size", &self.Size).field("SupportsMhc2", &self.SupportsMhc2).field("RegammaLutEntryCount", &self.RegammaLutEntryCount).field("CscXyzMatrixRows", &self.CscXyzMatrixRows).field("CscXyzMatrixColumns", &self.CscXyzMatrixColumns).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WCS_DEVICE_MHC2_CAPABILITIES {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WCS_DEVICE_MHC2_CAPABILITIES {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.SupportsMhc2 == other.SupportsMhc2 && self.RegammaLutEntryCount == other.RegammaLutEntryCount && self.CscXyzMatrixRows == other.CscXyzMatrixRows && self.CscXyzMatrixColumns == other.CscXyzMatrixColumns
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WCS_DEVICE_MHC2_CAPABILITIES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WCS_DEVICE_MHC2_CAPABILITIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WCS_DEVICE_VCGT_CAPABILITIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WCS_DEVICE_VCGT_CAPABILITIES").field("Size", &self.Size).field("SupportsVcgt", &self.SupportsVcgt).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WCS_DEVICE_VCGT_CAPABILITIES {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WCS_DEVICE_VCGT_CAPABILITIES {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.SupportsVcgt == other.SupportsVcgt
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WCS_DEVICE_VCGT_CAPABILITIES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WCS_DEVICE_VCGT_CAPABILITIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
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
impl ::core::fmt::Debug for XYZCOLOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("XYZCOLOR").field("X", &self.X).field("Y", &self.Y).field("Z", &self.Z).finish()
    }
}
impl ::windows::core::TypeKind for XYZCOLOR {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for XYZCOLOR {
    fn eq(&self, other: &Self) -> bool {
        self.X == other.X && self.Y == other.Y && self.Z == other.Z
    }
}
impl ::core::cmp::Eq for XYZCOLOR {}
impl ::core::default::Default for XYZCOLOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for XYZColorF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("XYZColorF").field("X", &self.X).field("Y", &self.Y).field("Z", &self.Z).finish()
    }
}
impl ::windows::core::TypeKind for XYZColorF {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for XYZColorF {
    fn eq(&self, other: &Self) -> bool {
        self.X == other.X && self.Y == other.Y && self.Z == other.Z
    }
}
impl ::core::cmp::Eq for XYZColorF {}
impl ::core::default::Default for XYZColorF {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for YxyCOLOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("YxyCOLOR").field("Y", &self.Y).field("x", &self.x).field("y", &self.y).finish()
    }
}
impl ::windows::core::TypeKind for YxyCOLOR {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for YxyCOLOR {
    fn eq(&self, other: &Self) -> bool {
        self.Y == other.Y && self.x == other.x && self.y == other.y
    }
}
impl ::core::cmp::Eq for YxyCOLOR {}
impl ::core::default::Default for YxyCOLOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type ICMENUMPROCA = ::core::option::Option<unsafe extern "system" fn(param0: ::windows::core::PCSTR, param1: super::super::Foundation::LPARAM) -> i32>;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type ICMENUMPROCW = ::core::option::Option<unsafe extern "system" fn(param0: ::windows::core::PCWSTR, param1: super::super::Foundation::LPARAM) -> i32>;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPBMCALLBACKFN = ::core::option::Option<unsafe extern "system" fn(param0: u32, param1: u32, param2: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub type PCMSCALLBACKA = ::core::option::Option<unsafe extern "system" fn(param0: *mut COLORMATCHSETUPA, param1: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub type PCMSCALLBACKW = ::core::option::Option<unsafe extern "system" fn(param0: *mut COLORMATCHSETUPW, param1: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
