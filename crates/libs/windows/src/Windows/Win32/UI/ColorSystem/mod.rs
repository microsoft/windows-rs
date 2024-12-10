#[inline]
pub unsafe fn AssociateColorProfileWithDeviceA<P0, P1, P2>(pmachinename: P0, pprofilename: P1, pdevicename: P2) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("mscms.dll" "system" fn AssociateColorProfileWithDeviceA(pmachinename : windows_core::PCSTR, pprofilename : windows_core::PCSTR, pdevicename : windows_core::PCSTR) -> super::super::Foundation:: BOOL);
    AssociateColorProfileWithDeviceA(pmachinename.param().abi(), pprofilename.param().abi(), pdevicename.param().abi())
}
#[inline]
pub unsafe fn AssociateColorProfileWithDeviceW<P0, P1, P2>(pmachinename: P0, pprofilename: P1, pdevicename: P2) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mscms.dll" "system" fn AssociateColorProfileWithDeviceW(pmachinename : windows_core::PCWSTR, pprofilename : windows_core::PCWSTR, pdevicename : windows_core::PCWSTR) -> super::super::Foundation:: BOOL);
    AssociateColorProfileWithDeviceW(pmachinename.param().abi(), pprofilename.param().abi(), pdevicename.param().abi())
}
#[inline]
pub unsafe fn CMCheckColors(hcmtransform: isize, lpainputcolors: *const COLOR, ncolors: u32, ctinput: COLORTYPE, lparesult: *mut u8) -> super::super::Foundation::BOOL {
    windows_targets::link!("icm32.dll" "system" fn CMCheckColors(hcmtransform : isize, lpainputcolors : *const COLOR, ncolors : u32, ctinput : COLORTYPE, lparesult : *mut u8) -> super::super::Foundation:: BOOL);
    CMCheckColors(core::mem::transmute(hcmtransform), core::mem::transmute(lpainputcolors), core::mem::transmute(ncolors), core::mem::transmute(ctinput), core::mem::transmute(lparesult))
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn CMCheckColorsInGamut(hcmtransform: isize, lpargbtriple: *const super::super::Graphics::Gdi::RGBTRIPLE, lparesult: *mut u8, ncount: u32) -> super::super::Foundation::BOOL {
    windows_targets::link!("icm32.dll" "system" fn CMCheckColorsInGamut(hcmtransform : isize, lpargbtriple : *const super::super::Graphics::Gdi:: RGBTRIPLE, lparesult : *mut u8, ncount : u32) -> super::super::Foundation:: BOOL);
    CMCheckColorsInGamut(core::mem::transmute(hcmtransform), core::mem::transmute(lpargbtriple), core::mem::transmute(lparesult), core::mem::transmute(ncount))
}
#[inline]
pub unsafe fn CMCheckRGBs(hcmtransform: isize, lpsrcbits: *const core::ffi::c_void, bminput: BMFORMAT, dwwidth: u32, dwheight: u32, dwstride: u32, lparesult: *mut u8, pfncallback: LPBMCALLBACKFN, ulcallbackdata: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL {
    windows_targets::link!("icm32.dll" "system" fn CMCheckRGBs(hcmtransform : isize, lpsrcbits : *const core::ffi::c_void, bminput : BMFORMAT, dwwidth : u32, dwheight : u32, dwstride : u32, lparesult : *mut u8, pfncallback : LPBMCALLBACKFN, ulcallbackdata : super::super::Foundation:: LPARAM) -> super::super::Foundation:: BOOL);
    CMCheckRGBs(core::mem::transmute(hcmtransform), core::mem::transmute(lpsrcbits), core::mem::transmute(bminput), core::mem::transmute(dwwidth), core::mem::transmute(dwheight), core::mem::transmute(dwstride), core::mem::transmute(lparesult), core::mem::transmute(pfncallback), core::mem::transmute(ulcallbackdata))
}
#[inline]
pub unsafe fn CMConvertColorNameToIndex(hprofile: isize, pacolorname: *const *const i8, paindex: *mut u32, dwcount: u32) -> super::super::Foundation::BOOL {
    windows_targets::link!("icm32.dll" "system" fn CMConvertColorNameToIndex(hprofile : isize, pacolorname : *const *const i8, paindex : *mut u32, dwcount : u32) -> super::super::Foundation:: BOOL);
    CMConvertColorNameToIndex(core::mem::transmute(hprofile), core::mem::transmute(pacolorname), core::mem::transmute(paindex), core::mem::transmute(dwcount))
}
#[inline]
pub unsafe fn CMConvertIndexToColorName(hprofile: isize, paindex: *const u32, pacolorname: *mut *mut i8, dwcount: u32) -> super::super::Foundation::BOOL {
    windows_targets::link!("icm32.dll" "system" fn CMConvertIndexToColorName(hprofile : isize, paindex : *const u32, pacolorname : *mut *mut i8, dwcount : u32) -> super::super::Foundation:: BOOL);
    CMConvertIndexToColorName(core::mem::transmute(hprofile), core::mem::transmute(paindex), core::mem::transmute(pacolorname), core::mem::transmute(dwcount))
}
#[inline]
pub unsafe fn CMCreateDeviceLinkProfile(pahprofiles: &[isize], padwintents: &[u32], dwflags: u32, lpprofiledata: *mut *mut u8) -> super::super::Foundation::BOOL {
    windows_targets::link!("icm32.dll" "system" fn CMCreateDeviceLinkProfile(pahprofiles : *const isize, nprofiles : u32, padwintents : *const u32, nintents : u32, dwflags : u32, lpprofiledata : *mut *mut u8) -> super::super::Foundation:: BOOL);
    CMCreateDeviceLinkProfile(core::mem::transmute(pahprofiles.as_ptr()), pahprofiles.len().try_into().unwrap(), core::mem::transmute(padwintents.as_ptr()), padwintents.len().try_into().unwrap(), core::mem::transmute(dwflags), core::mem::transmute(lpprofiledata))
}
#[inline]
pub unsafe fn CMCreateMultiProfileTransform(pahprofiles: &[isize], padwintents: &[u32], dwflags: u32) -> isize {
    windows_targets::link!("icm32.dll" "system" fn CMCreateMultiProfileTransform(pahprofiles : *const isize, nprofiles : u32, padwintents : *const u32, nintents : u32, dwflags : u32) -> isize);
    CMCreateMultiProfileTransform(core::mem::transmute(pahprofiles.as_ptr()), pahprofiles.len().try_into().unwrap(), core::mem::transmute(padwintents.as_ptr()), padwintents.len().try_into().unwrap(), core::mem::transmute(dwflags))
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn CMCreateProfile(lpcolorspace: *mut LOGCOLORSPACEA, lpprofiledata: *mut *mut core::ffi::c_void) -> super::super::Foundation::BOOL {
    windows_targets::link!("icm32.dll" "system" fn CMCreateProfile(lpcolorspace : *mut LOGCOLORSPACEA, lpprofiledata : *mut *mut core::ffi::c_void) -> super::super::Foundation:: BOOL);
    CMCreateProfile(core::mem::transmute(lpcolorspace), core::mem::transmute(lpprofiledata))
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn CMCreateProfileW(lpcolorspace: *mut LOGCOLORSPACEW, lpprofiledata: *mut *mut core::ffi::c_void) -> super::super::Foundation::BOOL {
    windows_targets::link!("icm32.dll" "system" fn CMCreateProfileW(lpcolorspace : *mut LOGCOLORSPACEW, lpprofiledata : *mut *mut core::ffi::c_void) -> super::super::Foundation:: BOOL);
    CMCreateProfileW(core::mem::transmute(lpcolorspace), core::mem::transmute(lpprofiledata))
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn CMCreateTransform(lpcolorspace: *const LOGCOLORSPACEA, lpdevcharacter: *const core::ffi::c_void, lptargetdevcharacter: *const core::ffi::c_void) -> isize {
    windows_targets::link!("icm32.dll" "system" fn CMCreateTransform(lpcolorspace : *const LOGCOLORSPACEA, lpdevcharacter : *const core::ffi::c_void, lptargetdevcharacter : *const core::ffi::c_void) -> isize);
    CMCreateTransform(core::mem::transmute(lpcolorspace), core::mem::transmute(lpdevcharacter), core::mem::transmute(lptargetdevcharacter))
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn CMCreateTransformExt(lpcolorspace: *const LOGCOLORSPACEA, lpdevcharacter: *const core::ffi::c_void, lptargetdevcharacter: *const core::ffi::c_void, dwflags: u32) -> isize {
    windows_targets::link!("icm32.dll" "system" fn CMCreateTransformExt(lpcolorspace : *const LOGCOLORSPACEA, lpdevcharacter : *const core::ffi::c_void, lptargetdevcharacter : *const core::ffi::c_void, dwflags : u32) -> isize);
    CMCreateTransformExt(core::mem::transmute(lpcolorspace), core::mem::transmute(lpdevcharacter), core::mem::transmute(lptargetdevcharacter), core::mem::transmute(dwflags))
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn CMCreateTransformExtW(lpcolorspace: *const LOGCOLORSPACEW, lpdevcharacter: *const core::ffi::c_void, lptargetdevcharacter: *const core::ffi::c_void, dwflags: u32) -> isize {
    windows_targets::link!("icm32.dll" "system" fn CMCreateTransformExtW(lpcolorspace : *const LOGCOLORSPACEW, lpdevcharacter : *const core::ffi::c_void, lptargetdevcharacter : *const core::ffi::c_void, dwflags : u32) -> isize);
    CMCreateTransformExtW(core::mem::transmute(lpcolorspace), core::mem::transmute(lpdevcharacter), core::mem::transmute(lptargetdevcharacter), core::mem::transmute(dwflags))
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn CMCreateTransformW(lpcolorspace: *const LOGCOLORSPACEW, lpdevcharacter: *const core::ffi::c_void, lptargetdevcharacter: *const core::ffi::c_void) -> isize {
    windows_targets::link!("icm32.dll" "system" fn CMCreateTransformW(lpcolorspace : *const LOGCOLORSPACEW, lpdevcharacter : *const core::ffi::c_void, lptargetdevcharacter : *const core::ffi::c_void) -> isize);
    CMCreateTransformW(core::mem::transmute(lpcolorspace), core::mem::transmute(lpdevcharacter), core::mem::transmute(lptargetdevcharacter))
}
#[inline]
pub unsafe fn CMDeleteTransform(hcmtransform: isize) -> super::super::Foundation::BOOL {
    windows_targets::link!("icm32.dll" "system" fn CMDeleteTransform(hcmtransform : isize) -> super::super::Foundation:: BOOL);
    CMDeleteTransform(core::mem::transmute(hcmtransform))
}
#[inline]
pub unsafe fn CMGetInfo(dwinfo: u32) -> u32 {
    windows_targets::link!("icm32.dll" "system" fn CMGetInfo(dwinfo : u32) -> u32);
    CMGetInfo(core::mem::transmute(dwinfo))
}
#[inline]
pub unsafe fn CMGetNamedProfileInfo(hprofile: isize, pnamedprofileinfo: *mut NAMED_PROFILE_INFO) -> super::super::Foundation::BOOL {
    windows_targets::link!("icm32.dll" "system" fn CMGetNamedProfileInfo(hprofile : isize, pnamedprofileinfo : *mut NAMED_PROFILE_INFO) -> super::super::Foundation:: BOOL);
    CMGetNamedProfileInfo(core::mem::transmute(hprofile), core::mem::transmute(pnamedprofileinfo))
}
#[inline]
pub unsafe fn CMIsProfileValid(hprofile: isize, lpbvalid: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL {
    windows_targets::link!("icm32.dll" "system" fn CMIsProfileValid(hprofile : isize, lpbvalid : *mut super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
    CMIsProfileValid(core::mem::transmute(hprofile), core::mem::transmute(lpbvalid))
}
#[inline]
pub unsafe fn CMTranslateColors(hcmtransform: isize, lpainputcolors: *const COLOR, ncolors: u32, ctinput: COLORTYPE, lpaoutputcolors: *mut COLOR, ctoutput: COLORTYPE) -> super::super::Foundation::BOOL {
    windows_targets::link!("icm32.dll" "system" fn CMTranslateColors(hcmtransform : isize, lpainputcolors : *const COLOR, ncolors : u32, ctinput : COLORTYPE, lpaoutputcolors : *mut COLOR, ctoutput : COLORTYPE) -> super::super::Foundation:: BOOL);
    CMTranslateColors(core::mem::transmute(hcmtransform), core::mem::transmute(lpainputcolors), core::mem::transmute(ncolors), core::mem::transmute(ctinput), core::mem::transmute(lpaoutputcolors), core::mem::transmute(ctoutput))
}
#[inline]
pub unsafe fn CMTranslateRGB(hcmtransform: isize, colorref: super::super::Foundation::COLORREF, lpcolorref: *mut u32, dwflags: u32) -> super::super::Foundation::BOOL {
    windows_targets::link!("icm32.dll" "system" fn CMTranslateRGB(hcmtransform : isize, colorref : super::super::Foundation:: COLORREF, lpcolorref : *mut u32, dwflags : u32) -> super::super::Foundation:: BOOL);
    CMTranslateRGB(core::mem::transmute(hcmtransform), core::mem::transmute(colorref), core::mem::transmute(lpcolorref), core::mem::transmute(dwflags))
}
#[inline]
pub unsafe fn CMTranslateRGBs(hcmtransform: isize, lpsrcbits: *const core::ffi::c_void, bminput: BMFORMAT, dwwidth: u32, dwheight: u32, dwstride: u32, lpdestbits: *mut core::ffi::c_void, bmoutput: BMFORMAT, dwtranslatedirection: u32) -> super::super::Foundation::BOOL {
    windows_targets::link!("icm32.dll" "system" fn CMTranslateRGBs(hcmtransform : isize, lpsrcbits : *const core::ffi::c_void, bminput : BMFORMAT, dwwidth : u32, dwheight : u32, dwstride : u32, lpdestbits : *mut core::ffi::c_void, bmoutput : BMFORMAT, dwtranslatedirection : u32) -> super::super::Foundation:: BOOL);
    CMTranslateRGBs(core::mem::transmute(hcmtransform), core::mem::transmute(lpsrcbits), core::mem::transmute(bminput), core::mem::transmute(dwwidth), core::mem::transmute(dwheight), core::mem::transmute(dwstride), core::mem::transmute(lpdestbits), core::mem::transmute(bmoutput), core::mem::transmute(dwtranslatedirection))
}
#[inline]
pub unsafe fn CMTranslateRGBsExt(hcmtransform: isize, lpsrcbits: *const core::ffi::c_void, bminput: BMFORMAT, dwwidth: u32, dwheight: u32, dwinputstride: u32, lpdestbits: *mut core::ffi::c_void, bmoutput: BMFORMAT, dwoutputstride: u32, lpfncallback: LPBMCALLBACKFN, ulcallbackdata: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL {
    windows_targets::link!("icm32.dll" "system" fn CMTranslateRGBsExt(hcmtransform : isize, lpsrcbits : *const core::ffi::c_void, bminput : BMFORMAT, dwwidth : u32, dwheight : u32, dwinputstride : u32, lpdestbits : *mut core::ffi::c_void, bmoutput : BMFORMAT, dwoutputstride : u32, lpfncallback : LPBMCALLBACKFN, ulcallbackdata : super::super::Foundation:: LPARAM) -> super::super::Foundation:: BOOL);
    CMTranslateRGBsExt(core::mem::transmute(hcmtransform), core::mem::transmute(lpsrcbits), core::mem::transmute(bminput), core::mem::transmute(dwwidth), core::mem::transmute(dwheight), core::mem::transmute(dwinputstride), core::mem::transmute(lpdestbits), core::mem::transmute(bmoutput), core::mem::transmute(dwoutputstride), core::mem::transmute(lpfncallback), core::mem::transmute(ulcallbackdata))
}
#[inline]
pub unsafe fn CheckBitmapBits(hcolortransform: isize, psrcbits: *const core::ffi::c_void, bminput: BMFORMAT, dwwidth: u32, dwheight: u32, dwstride: u32, paresult: *mut u8, pfncallback: Option<LPBMCALLBACKFN>, lpcallbackdata: Option<super::super::Foundation::LPARAM>) -> super::super::Foundation::BOOL {
    windows_targets::link!("mscms.dll" "system" fn CheckBitmapBits(hcolortransform : isize, psrcbits : *const core::ffi::c_void, bminput : BMFORMAT, dwwidth : u32, dwheight : u32, dwstride : u32, paresult : *mut u8, pfncallback : LPBMCALLBACKFN, lpcallbackdata : super::super::Foundation:: LPARAM) -> super::super::Foundation:: BOOL);
    CheckBitmapBits(core::mem::transmute(hcolortransform), core::mem::transmute(psrcbits), core::mem::transmute(bminput), core::mem::transmute(dwwidth), core::mem::transmute(dwheight), core::mem::transmute(dwstride), core::mem::transmute(paresult), core::mem::transmute(pfncallback.unwrap_or(core::mem::zeroed())), core::mem::transmute(lpcallbackdata.unwrap_or(core::mem::zeroed())))
}
#[inline]
pub unsafe fn CheckColors(hcolortransform: isize, painputcolors: *const COLOR, ncolors: u32, ctinput: COLORTYPE, paresult: *mut u8) -> super::super::Foundation::BOOL {
    windows_targets::link!("mscms.dll" "system" fn CheckColors(hcolortransform : isize, painputcolors : *const COLOR, ncolors : u32, ctinput : COLORTYPE, paresult : *mut u8) -> super::super::Foundation:: BOOL);
    CheckColors(core::mem::transmute(hcolortransform), core::mem::transmute(painputcolors), core::mem::transmute(ncolors), core::mem::transmute(ctinput), core::mem::transmute(paresult))
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn CheckColorsInGamut(hdc: super::super::Graphics::Gdi::HDC, lprgbtriple: *const super::super::Graphics::Gdi::RGBTRIPLE, dlpbuffer: *mut core::ffi::c_void, ncount: u32) -> super::super::Foundation::BOOL {
    windows_targets::link!("gdi32.dll" "system" fn CheckColorsInGamut(hdc : super::super::Graphics::Gdi:: HDC, lprgbtriple : *const super::super::Graphics::Gdi:: RGBTRIPLE, dlpbuffer : *mut core::ffi::c_void, ncount : u32) -> super::super::Foundation:: BOOL);
    CheckColorsInGamut(core::mem::transmute(hdc), core::mem::transmute(lprgbtriple), core::mem::transmute(dlpbuffer), core::mem::transmute(ncount))
}
#[inline]
pub unsafe fn CloseColorProfile(hprofile: Option<isize>) -> super::super::Foundation::BOOL {
    windows_targets::link!("mscms.dll" "system" fn CloseColorProfile(hprofile : isize) -> super::super::Foundation:: BOOL);
    CloseColorProfile(core::mem::transmute(hprofile.unwrap_or(core::mem::zeroed())))
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ColorCorrectPalette(hdc: super::super::Graphics::Gdi::HDC, hpal: super::super::Graphics::Gdi::HPALETTE, defirst: u32, num: u32) -> super::super::Foundation::BOOL {
    windows_targets::link!("gdi32.dll" "system" fn ColorCorrectPalette(hdc : super::super::Graphics::Gdi:: HDC, hpal : super::super::Graphics::Gdi:: HPALETTE, defirst : u32, num : u32) -> super::super::Foundation:: BOOL);
    ColorCorrectPalette(core::mem::transmute(hdc), core::mem::transmute(hpal), core::mem::transmute(defirst), core::mem::transmute(num))
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ColorMatchToTarget(hdc: super::super::Graphics::Gdi::HDC, hdctarget: super::super::Graphics::Gdi::HDC, action: COLOR_MATCH_TO_TARGET_ACTION) -> super::super::Foundation::BOOL {
    windows_targets::link!("gdi32.dll" "system" fn ColorMatchToTarget(hdc : super::super::Graphics::Gdi:: HDC, hdctarget : super::super::Graphics::Gdi:: HDC, action : COLOR_MATCH_TO_TARGET_ACTION) -> super::super::Foundation:: BOOL);
    ColorMatchToTarget(core::mem::transmute(hdc), core::mem::transmute(hdctarget), core::mem::transmute(action))
}
#[inline]
pub unsafe fn ColorProfileAddDisplayAssociation<P1>(scope: WCS_PROFILE_MANAGEMENT_SCOPE, profilename: P1, targetadapterid: super::super::Foundation::LUID, sourceid: u32, setasdefault: bool, associateasadvancedcolor: bool) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mscms.dll" "system" fn ColorProfileAddDisplayAssociation(scope : WCS_PROFILE_MANAGEMENT_SCOPE, profilename : windows_core::PCWSTR, targetadapterid : super::super::Foundation:: LUID, sourceid : u32, setasdefault : super::super::Foundation:: BOOL, associateasadvancedcolor : super::super::Foundation:: BOOL) -> windows_core::HRESULT);
    ColorProfileAddDisplayAssociation(core::mem::transmute(scope), profilename.param().abi(), core::mem::transmute(targetadapterid), core::mem::transmute(sourceid), setasdefault.into(), associateasadvancedcolor.into()).ok()
}
#[inline]
pub unsafe fn ColorProfileGetDisplayDefault(scope: WCS_PROFILE_MANAGEMENT_SCOPE, targetadapterid: super::super::Foundation::LUID, sourceid: u32, profiletype: COLORPROFILETYPE, profilesubtype: COLORPROFILESUBTYPE) -> windows_core::Result<windows_core::PWSTR> {
    windows_targets::link!("mscms.dll" "system" fn ColorProfileGetDisplayDefault(scope : WCS_PROFILE_MANAGEMENT_SCOPE, targetadapterid : super::super::Foundation:: LUID, sourceid : u32, profiletype : COLORPROFILETYPE, profilesubtype : COLORPROFILESUBTYPE, profilename : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    ColorProfileGetDisplayDefault(core::mem::transmute(scope), core::mem::transmute(targetadapterid), core::mem::transmute(sourceid), core::mem::transmute(profiletype), core::mem::transmute(profilesubtype), &mut result__).map(|| core::mem::transmute(result__))
}
#[inline]
pub unsafe fn ColorProfileGetDisplayList(scope: WCS_PROFILE_MANAGEMENT_SCOPE, targetadapterid: super::super::Foundation::LUID, sourceid: u32, profilelist: *mut *mut windows_core::PWSTR, profilecount: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("mscms.dll" "system" fn ColorProfileGetDisplayList(scope : WCS_PROFILE_MANAGEMENT_SCOPE, targetadapterid : super::super::Foundation:: LUID, sourceid : u32, profilelist : *mut *mut windows_core::PWSTR, profilecount : *mut u32) -> windows_core::HRESULT);
    ColorProfileGetDisplayList(core::mem::transmute(scope), core::mem::transmute(targetadapterid), core::mem::transmute(sourceid), core::mem::transmute(profilelist), core::mem::transmute(profilecount)).ok()
}
#[inline]
pub unsafe fn ColorProfileGetDisplayUserScope(targetadapterid: super::super::Foundation::LUID, sourceid: u32) -> windows_core::Result<WCS_PROFILE_MANAGEMENT_SCOPE> {
    windows_targets::link!("mscms.dll" "system" fn ColorProfileGetDisplayUserScope(targetadapterid : super::super::Foundation:: LUID, sourceid : u32, scope : *mut WCS_PROFILE_MANAGEMENT_SCOPE) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    ColorProfileGetDisplayUserScope(core::mem::transmute(targetadapterid), core::mem::transmute(sourceid), &mut result__).map(|| core::mem::transmute(result__))
}
#[inline]
pub unsafe fn ColorProfileRemoveDisplayAssociation<P1>(scope: WCS_PROFILE_MANAGEMENT_SCOPE, profilename: P1, targetadapterid: super::super::Foundation::LUID, sourceid: u32, dissociateadvancedcolor: bool) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mscms.dll" "system" fn ColorProfileRemoveDisplayAssociation(scope : WCS_PROFILE_MANAGEMENT_SCOPE, profilename : windows_core::PCWSTR, targetadapterid : super::super::Foundation:: LUID, sourceid : u32, dissociateadvancedcolor : super::super::Foundation:: BOOL) -> windows_core::HRESULT);
    ColorProfileRemoveDisplayAssociation(core::mem::transmute(scope), profilename.param().abi(), core::mem::transmute(targetadapterid), core::mem::transmute(sourceid), dissociateadvancedcolor.into()).ok()
}
#[inline]
pub unsafe fn ColorProfileSetDisplayDefaultAssociation<P1>(scope: WCS_PROFILE_MANAGEMENT_SCOPE, profilename: P1, profiletype: COLORPROFILETYPE, profilesubtype: COLORPROFILESUBTYPE, targetadapterid: super::super::Foundation::LUID, sourceid: u32) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mscms.dll" "system" fn ColorProfileSetDisplayDefaultAssociation(scope : WCS_PROFILE_MANAGEMENT_SCOPE, profilename : windows_core::PCWSTR, profiletype : COLORPROFILETYPE, profilesubtype : COLORPROFILESUBTYPE, targetadapterid : super::super::Foundation:: LUID, sourceid : u32) -> windows_core::HRESULT);
    ColorProfileSetDisplayDefaultAssociation(core::mem::transmute(scope), profilename.param().abi(), core::mem::transmute(profiletype), core::mem::transmute(profilesubtype), core::mem::transmute(targetadapterid), core::mem::transmute(sourceid)).ok()
}
#[inline]
pub unsafe fn ConvertColorNameToIndex(hprofile: isize, pacolorname: *const *const i8, paindex: *mut u32, dwcount: u32) -> super::super::Foundation::BOOL {
    windows_targets::link!("mscms.dll" "system" fn ConvertColorNameToIndex(hprofile : isize, pacolorname : *const *const i8, paindex : *mut u32, dwcount : u32) -> super::super::Foundation:: BOOL);
    ConvertColorNameToIndex(core::mem::transmute(hprofile), core::mem::transmute(pacolorname), core::mem::transmute(paindex), core::mem::transmute(dwcount))
}
#[inline]
pub unsafe fn ConvertIndexToColorName(hprofile: isize, paindex: *const u32, pacolorname: *mut *mut i8, dwcount: u32) -> super::super::Foundation::BOOL {
    windows_targets::link!("mscms.dll" "system" fn ConvertIndexToColorName(hprofile : isize, paindex : *const u32, pacolorname : *mut *mut i8, dwcount : u32) -> super::super::Foundation:: BOOL);
    ConvertIndexToColorName(core::mem::transmute(hprofile), core::mem::transmute(paindex), core::mem::transmute(pacolorname), core::mem::transmute(dwcount))
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn CreateColorSpaceA(lplcs: *const LOGCOLORSPACEA) -> HCOLORSPACE {
    windows_targets::link!("gdi32.dll" "system" fn CreateColorSpaceA(lplcs : *const LOGCOLORSPACEA) -> HCOLORSPACE);
    CreateColorSpaceA(core::mem::transmute(lplcs))
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn CreateColorSpaceW(lplcs: *const LOGCOLORSPACEW) -> HCOLORSPACE {
    windows_targets::link!("gdi32.dll" "system" fn CreateColorSpaceW(lplcs : *const LOGCOLORSPACEW) -> HCOLORSPACE);
    CreateColorSpaceW(core::mem::transmute(lplcs))
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn CreateColorTransformA(plogcolorspace: *const LOGCOLORSPACEA, hdestprofile: isize, htargetprofile: isize, dwflags: u32) -> isize {
    windows_targets::link!("mscms.dll" "system" fn CreateColorTransformA(plogcolorspace : *const LOGCOLORSPACEA, hdestprofile : isize, htargetprofile : isize, dwflags : u32) -> isize);
    CreateColorTransformA(core::mem::transmute(plogcolorspace), core::mem::transmute(hdestprofile), core::mem::transmute(htargetprofile), core::mem::transmute(dwflags))
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn CreateColorTransformW(plogcolorspace: *const LOGCOLORSPACEW, hdestprofile: isize, htargetprofile: isize, dwflags: u32) -> isize {
    windows_targets::link!("mscms.dll" "system" fn CreateColorTransformW(plogcolorspace : *const LOGCOLORSPACEW, hdestprofile : isize, htargetprofile : isize, dwflags : u32) -> isize);
    CreateColorTransformW(core::mem::transmute(plogcolorspace), core::mem::transmute(hdestprofile), core::mem::transmute(htargetprofile), core::mem::transmute(dwflags))
}
#[inline]
pub unsafe fn CreateDeviceLinkProfile(hprofile: &[isize], padwintent: &[u32], dwflags: u32, pprofiledata: *mut *mut u8, indexpreferredcmm: u32) -> super::super::Foundation::BOOL {
    windows_targets::link!("mscms.dll" "system" fn CreateDeviceLinkProfile(hprofile : *const isize, nprofiles : u32, padwintent : *const u32, nintents : u32, dwflags : u32, pprofiledata : *mut *mut u8, indexpreferredcmm : u32) -> super::super::Foundation:: BOOL);
    CreateDeviceLinkProfile(core::mem::transmute(hprofile.as_ptr()), hprofile.len().try_into().unwrap(), core::mem::transmute(padwintent.as_ptr()), padwintent.len().try_into().unwrap(), core::mem::transmute(dwflags), core::mem::transmute(pprofiledata), core::mem::transmute(indexpreferredcmm))
}
#[inline]
pub unsafe fn CreateMultiProfileTransform(pahprofiles: &[isize], padwintent: &[u32], dwflags: u32, indexpreferredcmm: u32) -> isize {
    windows_targets::link!("mscms.dll" "system" fn CreateMultiProfileTransform(pahprofiles : *const isize, nprofiles : u32, padwintent : *const u32, nintents : u32, dwflags : u32, indexpreferredcmm : u32) -> isize);
    CreateMultiProfileTransform(core::mem::transmute(pahprofiles.as_ptr()), pahprofiles.len().try_into().unwrap(), core::mem::transmute(padwintent.as_ptr()), padwintent.len().try_into().unwrap(), core::mem::transmute(dwflags), core::mem::transmute(indexpreferredcmm))
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn CreateProfileFromLogColorSpaceA(plogcolorspace: *const LOGCOLORSPACEA, pprofile: *mut *mut u8) -> super::super::Foundation::BOOL {
    windows_targets::link!("mscms.dll" "system" fn CreateProfileFromLogColorSpaceA(plogcolorspace : *const LOGCOLORSPACEA, pprofile : *mut *mut u8) -> super::super::Foundation:: BOOL);
    CreateProfileFromLogColorSpaceA(core::mem::transmute(plogcolorspace), core::mem::transmute(pprofile))
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn CreateProfileFromLogColorSpaceW(plogcolorspace: *const LOGCOLORSPACEW, pprofile: *mut *mut u8) -> super::super::Foundation::BOOL {
    windows_targets::link!("mscms.dll" "system" fn CreateProfileFromLogColorSpaceW(plogcolorspace : *const LOGCOLORSPACEW, pprofile : *mut *mut u8) -> super::super::Foundation:: BOOL);
    CreateProfileFromLogColorSpaceW(core::mem::transmute(plogcolorspace), core::mem::transmute(pprofile))
}
#[inline]
pub unsafe fn DeleteColorSpace(hcs: HCOLORSPACE) -> super::super::Foundation::BOOL {
    windows_targets::link!("gdi32.dll" "system" fn DeleteColorSpace(hcs : HCOLORSPACE) -> super::super::Foundation:: BOOL);
    DeleteColorSpace(core::mem::transmute(hcs))
}
#[inline]
pub unsafe fn DeleteColorTransform(hxform: isize) -> super::super::Foundation::BOOL {
    windows_targets::link!("mscms.dll" "system" fn DeleteColorTransform(hxform : isize) -> super::super::Foundation:: BOOL);
    DeleteColorTransform(core::mem::transmute(hxform))
}
#[inline]
pub unsafe fn DisassociateColorProfileFromDeviceA<P0, P1, P2>(pmachinename: P0, pprofilename: P1, pdevicename: P2) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("mscms.dll" "system" fn DisassociateColorProfileFromDeviceA(pmachinename : windows_core::PCSTR, pprofilename : windows_core::PCSTR, pdevicename : windows_core::PCSTR) -> super::super::Foundation:: BOOL);
    DisassociateColorProfileFromDeviceA(pmachinename.param().abi(), pprofilename.param().abi(), pdevicename.param().abi())
}
#[inline]
pub unsafe fn DisassociateColorProfileFromDeviceW<P0, P1, P2>(pmachinename: P0, pprofilename: P1, pdevicename: P2) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mscms.dll" "system" fn DisassociateColorProfileFromDeviceW(pmachinename : windows_core::PCWSTR, pprofilename : windows_core::PCWSTR, pdevicename : windows_core::PCWSTR) -> super::super::Foundation:: BOOL);
    DisassociateColorProfileFromDeviceW(pmachinename.param().abi(), pprofilename.param().abi(), pdevicename.param().abi())
}
#[inline]
pub unsafe fn EnumColorProfilesA<P0>(pmachinename: P0, penumrecord: *const ENUMTYPEA, penumerationbuffer: Option<*mut u8>, pdwsizeofenumerationbuffer: *mut u32, pnprofiles: Option<*mut u32>) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("mscms.dll" "system" fn EnumColorProfilesA(pmachinename : windows_core::PCSTR, penumrecord : *const ENUMTYPEA, penumerationbuffer : *mut u8, pdwsizeofenumerationbuffer : *mut u32, pnprofiles : *mut u32) -> super::super::Foundation:: BOOL);
    EnumColorProfilesA(pmachinename.param().abi(), core::mem::transmute(penumrecord), core::mem::transmute(penumerationbuffer.unwrap_or(core::mem::zeroed())), core::mem::transmute(pdwsizeofenumerationbuffer), core::mem::transmute(pnprofiles.unwrap_or(core::mem::zeroed())))
}
#[inline]
pub unsafe fn EnumColorProfilesW<P0>(pmachinename: P0, penumrecord: *const ENUMTYPEW, penumerationbuffer: Option<*mut u8>, pdwsizeofenumerationbuffer: *mut u32, pnprofiles: Option<*mut u32>) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mscms.dll" "system" fn EnumColorProfilesW(pmachinename : windows_core::PCWSTR, penumrecord : *const ENUMTYPEW, penumerationbuffer : *mut u8, pdwsizeofenumerationbuffer : *mut u32, pnprofiles : *mut u32) -> super::super::Foundation:: BOOL);
    EnumColorProfilesW(pmachinename.param().abi(), core::mem::transmute(penumrecord), core::mem::transmute(penumerationbuffer.unwrap_or(core::mem::zeroed())), core::mem::transmute(pdwsizeofenumerationbuffer), core::mem::transmute(pnprofiles.unwrap_or(core::mem::zeroed())))
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn EnumICMProfilesA(hdc: super::super::Graphics::Gdi::HDC, proc: ICMENUMPROCA, param2: Option<super::super::Foundation::LPARAM>) -> i32 {
    windows_targets::link!("gdi32.dll" "system" fn EnumICMProfilesA(hdc : super::super::Graphics::Gdi:: HDC, proc : ICMENUMPROCA, param2 : super::super::Foundation:: LPARAM) -> i32);
    EnumICMProfilesA(core::mem::transmute(hdc), core::mem::transmute(proc), core::mem::transmute(param2.unwrap_or(core::mem::zeroed())))
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn EnumICMProfilesW(hdc: super::super::Graphics::Gdi::HDC, proc: ICMENUMPROCW, param2: Option<super::super::Foundation::LPARAM>) -> i32 {
    windows_targets::link!("gdi32.dll" "system" fn EnumICMProfilesW(hdc : super::super::Graphics::Gdi:: HDC, proc : ICMENUMPROCW, param2 : super::super::Foundation:: LPARAM) -> i32);
    EnumICMProfilesW(core::mem::transmute(hdc), core::mem::transmute(proc), core::mem::transmute(param2.unwrap_or(core::mem::zeroed())))
}
#[inline]
pub unsafe fn GetCMMInfo(hcolortransform: isize, param1: u32) -> u32 {
    windows_targets::link!("mscms.dll" "system" fn GetCMMInfo(hcolortransform : isize, param1 : u32) -> u32);
    GetCMMInfo(core::mem::transmute(hcolortransform), core::mem::transmute(param1))
}
#[inline]
pub unsafe fn GetColorDirectoryA<P0>(pmachinename: P0, pbuffer: Option<windows_core::PSTR>, pdwsize: *mut u32) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("mscms.dll" "system" fn GetColorDirectoryA(pmachinename : windows_core::PCSTR, pbuffer : windows_core::PSTR, pdwsize : *mut u32) -> super::super::Foundation:: BOOL);
    GetColorDirectoryA(pmachinename.param().abi(), core::mem::transmute(pbuffer.unwrap_or(core::mem::zeroed())), core::mem::transmute(pdwsize))
}
#[inline]
pub unsafe fn GetColorDirectoryW<P0>(pmachinename: P0, pbuffer: Option<windows_core::PWSTR>, pdwsize: *mut u32) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mscms.dll" "system" fn GetColorDirectoryW(pmachinename : windows_core::PCWSTR, pbuffer : windows_core::PWSTR, pdwsize : *mut u32) -> super::super::Foundation:: BOOL);
    GetColorDirectoryW(pmachinename.param().abi(), core::mem::transmute(pbuffer.unwrap_or(core::mem::zeroed())), core::mem::transmute(pdwsize))
}
#[inline]
pub unsafe fn GetColorProfileElement(hprofile: isize, tag: u32, dwoffset: u32, pcbelement: *mut u32, pelement: Option<*mut core::ffi::c_void>, pbreference: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL {
    windows_targets::link!("mscms.dll" "system" fn GetColorProfileElement(hprofile : isize, tag : u32, dwoffset : u32, pcbelement : *mut u32, pelement : *mut core::ffi::c_void, pbreference : *mut super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
    GetColorProfileElement(core::mem::transmute(hprofile), core::mem::transmute(tag), core::mem::transmute(dwoffset), core::mem::transmute(pcbelement), core::mem::transmute(pelement.unwrap_or(core::mem::zeroed())), core::mem::transmute(pbreference))
}
#[inline]
pub unsafe fn GetColorProfileElementTag(hprofile: isize, dwindex: u32, ptag: *mut u32) -> super::super::Foundation::BOOL {
    windows_targets::link!("mscms.dll" "system" fn GetColorProfileElementTag(hprofile : isize, dwindex : u32, ptag : *mut u32) -> super::super::Foundation:: BOOL);
    GetColorProfileElementTag(core::mem::transmute(hprofile), core::mem::transmute(dwindex), core::mem::transmute(ptag))
}
#[inline]
pub unsafe fn GetColorProfileFromHandle(hprofile: isize, pprofile: Option<*mut u8>, pcbprofile: *mut u32) -> super::super::Foundation::BOOL {
    windows_targets::link!("mscms.dll" "system" fn GetColorProfileFromHandle(hprofile : isize, pprofile : *mut u8, pcbprofile : *mut u32) -> super::super::Foundation:: BOOL);
    GetColorProfileFromHandle(core::mem::transmute(hprofile), core::mem::transmute(pprofile.unwrap_or(core::mem::zeroed())), core::mem::transmute(pcbprofile))
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetColorProfileHeader(hprofile: isize, pheader: *mut PROFILEHEADER) -> super::super::Foundation::BOOL {
    windows_targets::link!("mscms.dll" "system" fn GetColorProfileHeader(hprofile : isize, pheader : *mut PROFILEHEADER) -> super::super::Foundation:: BOOL);
    GetColorProfileHeader(core::mem::transmute(hprofile), core::mem::transmute(pheader))
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetColorSpace(hdc: super::super::Graphics::Gdi::HDC) -> HCOLORSPACE {
    windows_targets::link!("gdi32.dll" "system" fn GetColorSpace(hdc : super::super::Graphics::Gdi:: HDC) -> HCOLORSPACE);
    GetColorSpace(core::mem::transmute(hdc))
}
#[inline]
pub unsafe fn GetCountColorProfileElements(hprofile: isize, pnelementcount: *mut u32) -> super::super::Foundation::BOOL {
    windows_targets::link!("mscms.dll" "system" fn GetCountColorProfileElements(hprofile : isize, pnelementcount : *mut u32) -> super::super::Foundation:: BOOL);
    GetCountColorProfileElements(core::mem::transmute(hprofile), core::mem::transmute(pnelementcount))
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetDeviceGammaRamp(hdc: super::super::Graphics::Gdi::HDC, lpramp: *mut core::ffi::c_void) -> super::super::Foundation::BOOL {
    windows_targets::link!("gdi32.dll" "system" fn GetDeviceGammaRamp(hdc : super::super::Graphics::Gdi:: HDC, lpramp : *mut core::ffi::c_void) -> super::super::Foundation:: BOOL);
    GetDeviceGammaRamp(core::mem::transmute(hdc), core::mem::transmute(lpramp))
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetICMProfileA(hdc: super::super::Graphics::Gdi::HDC, pbufsize: *mut u32, pszfilename: Option<windows_core::PSTR>) -> super::super::Foundation::BOOL {
    windows_targets::link!("gdi32.dll" "system" fn GetICMProfileA(hdc : super::super::Graphics::Gdi:: HDC, pbufsize : *mut u32, pszfilename : windows_core::PSTR) -> super::super::Foundation:: BOOL);
    GetICMProfileA(core::mem::transmute(hdc), core::mem::transmute(pbufsize), core::mem::transmute(pszfilename.unwrap_or(core::mem::zeroed())))
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetICMProfileW(hdc: super::super::Graphics::Gdi::HDC, pbufsize: *mut u32, pszfilename: Option<windows_core::PWSTR>) -> super::super::Foundation::BOOL {
    windows_targets::link!("gdi32.dll" "system" fn GetICMProfileW(hdc : super::super::Graphics::Gdi:: HDC, pbufsize : *mut u32, pszfilename : windows_core::PWSTR) -> super::super::Foundation:: BOOL);
    GetICMProfileW(core::mem::transmute(hdc), core::mem::transmute(pbufsize), core::mem::transmute(pszfilename.unwrap_or(core::mem::zeroed())))
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetLogColorSpaceA(hcolorspace: HCOLORSPACE, lpbuffer: *mut LOGCOLORSPACEA, nsize: u32) -> super::super::Foundation::BOOL {
    windows_targets::link!("gdi32.dll" "system" fn GetLogColorSpaceA(hcolorspace : HCOLORSPACE, lpbuffer : *mut LOGCOLORSPACEA, nsize : u32) -> super::super::Foundation:: BOOL);
    GetLogColorSpaceA(core::mem::transmute(hcolorspace), core::mem::transmute(lpbuffer), core::mem::transmute(nsize))
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetLogColorSpaceW(hcolorspace: HCOLORSPACE, lpbuffer: *mut LOGCOLORSPACEW, nsize: u32) -> super::super::Foundation::BOOL {
    windows_targets::link!("gdi32.dll" "system" fn GetLogColorSpaceW(hcolorspace : HCOLORSPACE, lpbuffer : *mut LOGCOLORSPACEW, nsize : u32) -> super::super::Foundation:: BOOL);
    GetLogColorSpaceW(core::mem::transmute(hcolorspace), core::mem::transmute(lpbuffer), core::mem::transmute(nsize))
}
#[inline]
pub unsafe fn GetNamedProfileInfo(hprofile: isize, pnamedprofileinfo: *mut NAMED_PROFILE_INFO) -> super::super::Foundation::BOOL {
    windows_targets::link!("mscms.dll" "system" fn GetNamedProfileInfo(hprofile : isize, pnamedprofileinfo : *mut NAMED_PROFILE_INFO) -> super::super::Foundation:: BOOL);
    GetNamedProfileInfo(core::mem::transmute(hprofile), core::mem::transmute(pnamedprofileinfo))
}
#[inline]
pub unsafe fn GetPS2ColorRenderingDictionary(hprofile: isize, dwintent: u32, pps2colorrenderingdictionary: Option<*mut u8>, pcbps2colorrenderingdictionary: *mut u32, pbbinary: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL {
    windows_targets::link!("mscms.dll" "system" fn GetPS2ColorRenderingDictionary(hprofile : isize, dwintent : u32, pps2colorrenderingdictionary : *mut u8, pcbps2colorrenderingdictionary : *mut u32, pbbinary : *mut super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
    GetPS2ColorRenderingDictionary(core::mem::transmute(hprofile), core::mem::transmute(dwintent), core::mem::transmute(pps2colorrenderingdictionary.unwrap_or(core::mem::zeroed())), core::mem::transmute(pcbps2colorrenderingdictionary), core::mem::transmute(pbbinary))
}
#[inline]
pub unsafe fn GetPS2ColorRenderingIntent(hprofile: isize, dwintent: u32, pbuffer: Option<*mut u8>, pcbps2colorrenderingintent: *mut u32) -> super::super::Foundation::BOOL {
    windows_targets::link!("mscms.dll" "system" fn GetPS2ColorRenderingIntent(hprofile : isize, dwintent : u32, pbuffer : *mut u8, pcbps2colorrenderingintent : *mut u32) -> super::super::Foundation:: BOOL);
    GetPS2ColorRenderingIntent(core::mem::transmute(hprofile), core::mem::transmute(dwintent), core::mem::transmute(pbuffer.unwrap_or(core::mem::zeroed())), core::mem::transmute(pcbps2colorrenderingintent))
}
#[inline]
pub unsafe fn GetPS2ColorSpaceArray(hprofile: isize, dwintent: u32, dwcsatype: u32, pps2colorspacearray: Option<*mut u8>, pcbps2colorspacearray: *mut u32, pbbinary: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL {
    windows_targets::link!("mscms.dll" "system" fn GetPS2ColorSpaceArray(hprofile : isize, dwintent : u32, dwcsatype : u32, pps2colorspacearray : *mut u8, pcbps2colorspacearray : *mut u32, pbbinary : *mut super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
    GetPS2ColorSpaceArray(core::mem::transmute(hprofile), core::mem::transmute(dwintent), core::mem::transmute(dwcsatype), core::mem::transmute(pps2colorspacearray.unwrap_or(core::mem::zeroed())), core::mem::transmute(pcbps2colorspacearray), core::mem::transmute(pbbinary))
}
#[inline]
pub unsafe fn GetStandardColorSpaceProfileA<P0>(pmachinename: P0, dwscs: u32, pbuffer: Option<windows_core::PSTR>, pcbsize: *mut u32) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("mscms.dll" "system" fn GetStandardColorSpaceProfileA(pmachinename : windows_core::PCSTR, dwscs : u32, pbuffer : windows_core::PSTR, pcbsize : *mut u32) -> super::super::Foundation:: BOOL);
    GetStandardColorSpaceProfileA(pmachinename.param().abi(), core::mem::transmute(dwscs), core::mem::transmute(pbuffer.unwrap_or(core::mem::zeroed())), core::mem::transmute(pcbsize))
}
#[inline]
pub unsafe fn GetStandardColorSpaceProfileW<P0>(pmachinename: P0, dwscs: u32, pbuffer: Option<windows_core::PWSTR>, pcbsize: *mut u32) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mscms.dll" "system" fn GetStandardColorSpaceProfileW(pmachinename : windows_core::PCWSTR, dwscs : u32, pbuffer : windows_core::PWSTR, pcbsize : *mut u32) -> super::super::Foundation:: BOOL);
    GetStandardColorSpaceProfileW(pmachinename.param().abi(), core::mem::transmute(dwscs), core::mem::transmute(pbuffer.unwrap_or(core::mem::zeroed())), core::mem::transmute(pcbsize))
}
#[inline]
pub unsafe fn InstallColorProfileA<P0, P1>(pmachinename: P0, pprofilename: P1) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("mscms.dll" "system" fn InstallColorProfileA(pmachinename : windows_core::PCSTR, pprofilename : windows_core::PCSTR) -> super::super::Foundation:: BOOL);
    InstallColorProfileA(pmachinename.param().abi(), pprofilename.param().abi())
}
#[inline]
pub unsafe fn InstallColorProfileW<P0, P1>(pmachinename: P0, pprofilename: P1) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mscms.dll" "system" fn InstallColorProfileW(pmachinename : windows_core::PCWSTR, pprofilename : windows_core::PCWSTR) -> super::super::Foundation:: BOOL);
    InstallColorProfileW(pmachinename.param().abi(), pprofilename.param().abi())
}
#[inline]
pub unsafe fn IsColorProfileTagPresent(hprofile: isize, tag: u32, pbpresent: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL {
    windows_targets::link!("mscms.dll" "system" fn IsColorProfileTagPresent(hprofile : isize, tag : u32, pbpresent : *mut super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
    IsColorProfileTagPresent(core::mem::transmute(hprofile), core::mem::transmute(tag), core::mem::transmute(pbpresent))
}
#[inline]
pub unsafe fn IsColorProfileValid(hprofile: isize, pbvalid: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL {
    windows_targets::link!("mscms.dll" "system" fn IsColorProfileValid(hprofile : isize, pbvalid : *mut super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
    IsColorProfileValid(core::mem::transmute(hprofile), core::mem::transmute(pbvalid))
}
#[inline]
pub unsafe fn OpenColorProfileA(pprofile: *const PROFILE, dwdesiredaccess: u32, dwsharemode: u32, dwcreationmode: u32) -> isize {
    windows_targets::link!("mscms.dll" "system" fn OpenColorProfileA(pprofile : *const PROFILE, dwdesiredaccess : u32, dwsharemode : u32, dwcreationmode : u32) -> isize);
    OpenColorProfileA(core::mem::transmute(pprofile), core::mem::transmute(dwdesiredaccess), core::mem::transmute(dwsharemode), core::mem::transmute(dwcreationmode))
}
#[inline]
pub unsafe fn OpenColorProfileW(pprofile: *const PROFILE, dwdesiredaccess: u32, dwsharemode: u32, dwcreationmode: u32) -> isize {
    windows_targets::link!("mscms.dll" "system" fn OpenColorProfileW(pprofile : *const PROFILE, dwdesiredaccess : u32, dwsharemode : u32, dwcreationmode : u32) -> isize);
    OpenColorProfileW(core::mem::transmute(pprofile), core::mem::transmute(dwdesiredaccess), core::mem::transmute(dwsharemode), core::mem::transmute(dwcreationmode))
}
#[inline]
pub unsafe fn RegisterCMMA<P0, P2>(pmachinename: P0, cmmid: u32, pcmmdll: P2) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("mscms.dll" "system" fn RegisterCMMA(pmachinename : windows_core::PCSTR, cmmid : u32, pcmmdll : windows_core::PCSTR) -> super::super::Foundation:: BOOL);
    RegisterCMMA(pmachinename.param().abi(), core::mem::transmute(cmmid), pcmmdll.param().abi())
}
#[inline]
pub unsafe fn RegisterCMMW<P0, P2>(pmachinename: P0, cmmid: u32, pcmmdll: P2) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mscms.dll" "system" fn RegisterCMMW(pmachinename : windows_core::PCWSTR, cmmid : u32, pcmmdll : windows_core::PCWSTR) -> super::super::Foundation:: BOOL);
    RegisterCMMW(pmachinename.param().abi(), core::mem::transmute(cmmid), pcmmdll.param().abi())
}
#[inline]
pub unsafe fn SelectCMM(dwcmmtype: u32) -> super::super::Foundation::BOOL {
    windows_targets::link!("mscms.dll" "system" fn SelectCMM(dwcmmtype : u32) -> super::super::Foundation:: BOOL);
    SelectCMM(core::mem::transmute(dwcmmtype))
}
#[inline]
pub unsafe fn SetColorProfileElement(hprofile: isize, tag: u32, dwoffset: u32, pcbelement: *const u32, pelement: *const core::ffi::c_void) -> super::super::Foundation::BOOL {
    windows_targets::link!("mscms.dll" "system" fn SetColorProfileElement(hprofile : isize, tag : u32, dwoffset : u32, pcbelement : *const u32, pelement : *const core::ffi::c_void) -> super::super::Foundation:: BOOL);
    SetColorProfileElement(core::mem::transmute(hprofile), core::mem::transmute(tag), core::mem::transmute(dwoffset), core::mem::transmute(pcbelement), core::mem::transmute(pelement))
}
#[inline]
pub unsafe fn SetColorProfileElementReference(hprofile: isize, newtag: u32, reftag: u32) -> super::super::Foundation::BOOL {
    windows_targets::link!("mscms.dll" "system" fn SetColorProfileElementReference(hprofile : isize, newtag : u32, reftag : u32) -> super::super::Foundation:: BOOL);
    SetColorProfileElementReference(core::mem::transmute(hprofile), core::mem::transmute(newtag), core::mem::transmute(reftag))
}
#[inline]
pub unsafe fn SetColorProfileElementSize(hprofile: isize, tagtype: u32, pcbelement: u32) -> super::super::Foundation::BOOL {
    windows_targets::link!("mscms.dll" "system" fn SetColorProfileElementSize(hprofile : isize, tagtype : u32, pcbelement : u32) -> super::super::Foundation:: BOOL);
    SetColorProfileElementSize(core::mem::transmute(hprofile), core::mem::transmute(tagtype), core::mem::transmute(pcbelement))
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn SetColorProfileHeader(hprofile: isize, pheader: *const PROFILEHEADER) -> super::super::Foundation::BOOL {
    windows_targets::link!("mscms.dll" "system" fn SetColorProfileHeader(hprofile : isize, pheader : *const PROFILEHEADER) -> super::super::Foundation:: BOOL);
    SetColorProfileHeader(core::mem::transmute(hprofile), core::mem::transmute(pheader))
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn SetColorSpace(hdc: super::super::Graphics::Gdi::HDC, hcs: HCOLORSPACE) -> HCOLORSPACE {
    windows_targets::link!("gdi32.dll" "system" fn SetColorSpace(hdc : super::super::Graphics::Gdi:: HDC, hcs : HCOLORSPACE) -> HCOLORSPACE);
    SetColorSpace(core::mem::transmute(hdc), core::mem::transmute(hcs))
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn SetDeviceGammaRamp(hdc: super::super::Graphics::Gdi::HDC, lpramp: *const core::ffi::c_void) -> super::super::Foundation::BOOL {
    windows_targets::link!("gdi32.dll" "system" fn SetDeviceGammaRamp(hdc : super::super::Graphics::Gdi:: HDC, lpramp : *const core::ffi::c_void) -> super::super::Foundation:: BOOL);
    SetDeviceGammaRamp(core::mem::transmute(hdc), core::mem::transmute(lpramp))
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn SetICMMode(hdc: super::super::Graphics::Gdi::HDC, mode: ICM_MODE) -> i32 {
    windows_targets::link!("gdi32.dll" "system" fn SetICMMode(hdc : super::super::Graphics::Gdi:: HDC, mode : ICM_MODE) -> i32);
    SetICMMode(core::mem::transmute(hdc), core::mem::transmute(mode))
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn SetICMProfileA<P1>(hdc: super::super::Graphics::Gdi::HDC, lpfilename: P1) -> super::super::Foundation::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("gdi32.dll" "system" fn SetICMProfileA(hdc : super::super::Graphics::Gdi:: HDC, lpfilename : windows_core::PCSTR) -> super::super::Foundation:: BOOL);
    SetICMProfileA(core::mem::transmute(hdc), lpfilename.param().abi())
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn SetICMProfileW<P1>(hdc: super::super::Graphics::Gdi::HDC, lpfilename: P1) -> super::super::Foundation::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("gdi32.dll" "system" fn SetICMProfileW(hdc : super::super::Graphics::Gdi:: HDC, lpfilename : windows_core::PCWSTR) -> super::super::Foundation:: BOOL);
    SetICMProfileW(core::mem::transmute(hdc), lpfilename.param().abi())
}
#[inline]
pub unsafe fn SetStandardColorSpaceProfileA<P0, P2>(pmachinename: P0, dwprofileid: u32, pprofilename: P2) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("mscms.dll" "system" fn SetStandardColorSpaceProfileA(pmachinename : windows_core::PCSTR, dwprofileid : u32, pprofilename : windows_core::PCSTR) -> super::super::Foundation:: BOOL);
    SetStandardColorSpaceProfileA(pmachinename.param().abi(), core::mem::transmute(dwprofileid), pprofilename.param().abi())
}
#[inline]
pub unsafe fn SetStandardColorSpaceProfileW<P0, P2>(pmachinename: P0, dwprofileid: u32, pprofilename: P2) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mscms.dll" "system" fn SetStandardColorSpaceProfileW(pmachinename : windows_core::PCWSTR, dwprofileid : u32, pprofilename : windows_core::PCWSTR) -> super::super::Foundation:: BOOL);
    SetStandardColorSpaceProfileW(pmachinename.param().abi(), core::mem::transmute(dwprofileid), pprofilename.param().abi())
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn SetupColorMatchingA(pcms: *mut COLORMATCHSETUPA) -> super::super::Foundation::BOOL {
    windows_targets::link!("icmui.dll" "system" fn SetupColorMatchingA(pcms : *mut COLORMATCHSETUPA) -> super::super::Foundation:: BOOL);
    SetupColorMatchingA(core::mem::transmute(pcms))
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn SetupColorMatchingW(pcms: *mut COLORMATCHSETUPW) -> super::super::Foundation::BOOL {
    windows_targets::link!("icmui.dll" "system" fn SetupColorMatchingW(pcms : *mut COLORMATCHSETUPW) -> super::super::Foundation:: BOOL);
    SetupColorMatchingW(core::mem::transmute(pcms))
}
#[inline]
pub unsafe fn TranslateBitmapBits(hcolortransform: isize, psrcbits: *const core::ffi::c_void, bminput: BMFORMAT, dwwidth: u32, dwheight: u32, dwinputstride: u32, pdestbits: *mut core::ffi::c_void, bmoutput: BMFORMAT, dwoutputstride: u32, pfncallback: Option<LPBMCALLBACKFN>, ulcallbackdata: Option<super::super::Foundation::LPARAM>) -> super::super::Foundation::BOOL {
    windows_targets::link!("mscms.dll" "system" fn TranslateBitmapBits(hcolortransform : isize, psrcbits : *const core::ffi::c_void, bminput : BMFORMAT, dwwidth : u32, dwheight : u32, dwinputstride : u32, pdestbits : *mut core::ffi::c_void, bmoutput : BMFORMAT, dwoutputstride : u32, pfncallback : LPBMCALLBACKFN, ulcallbackdata : super::super::Foundation:: LPARAM) -> super::super::Foundation:: BOOL);
    TranslateBitmapBits(core::mem::transmute(hcolortransform), core::mem::transmute(psrcbits), core::mem::transmute(bminput), core::mem::transmute(dwwidth), core::mem::transmute(dwheight), core::mem::transmute(dwinputstride), core::mem::transmute(pdestbits), core::mem::transmute(bmoutput), core::mem::transmute(dwoutputstride), core::mem::transmute(pfncallback.unwrap_or(core::mem::zeroed())), core::mem::transmute(ulcallbackdata.unwrap_or(core::mem::zeroed())))
}
#[inline]
pub unsafe fn TranslateColors(hcolortransform: isize, painputcolors: *const COLOR, ncolors: u32, ctinput: COLORTYPE, paoutputcolors: *mut COLOR, ctoutput: COLORTYPE) -> super::super::Foundation::BOOL {
    windows_targets::link!("mscms.dll" "system" fn TranslateColors(hcolortransform : isize, painputcolors : *const COLOR, ncolors : u32, ctinput : COLORTYPE, paoutputcolors : *mut COLOR, ctoutput : COLORTYPE) -> super::super::Foundation:: BOOL);
    TranslateColors(core::mem::transmute(hcolortransform), core::mem::transmute(painputcolors), core::mem::transmute(ncolors), core::mem::transmute(ctinput), core::mem::transmute(paoutputcolors), core::mem::transmute(ctoutput))
}
#[inline]
pub unsafe fn UninstallColorProfileA<P0, P1>(pmachinename: P0, pprofilename: P1, bdelete: bool) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("mscms.dll" "system" fn UninstallColorProfileA(pmachinename : windows_core::PCSTR, pprofilename : windows_core::PCSTR, bdelete : super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
    UninstallColorProfileA(pmachinename.param().abi(), pprofilename.param().abi(), bdelete.into())
}
#[inline]
pub unsafe fn UninstallColorProfileW<P0, P1>(pmachinename: P0, pprofilename: P1, bdelete: bool) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mscms.dll" "system" fn UninstallColorProfileW(pmachinename : windows_core::PCWSTR, pprofilename : windows_core::PCWSTR, bdelete : super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
    UninstallColorProfileW(pmachinename.param().abi(), pprofilename.param().abi(), bdelete.into())
}
#[inline]
pub unsafe fn UnregisterCMMA<P0>(pmachinename: P0, cmmid: u32) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("mscms.dll" "system" fn UnregisterCMMA(pmachinename : windows_core::PCSTR, cmmid : u32) -> super::super::Foundation:: BOOL);
    UnregisterCMMA(pmachinename.param().abi(), core::mem::transmute(cmmid))
}
#[inline]
pub unsafe fn UnregisterCMMW<P0>(pmachinename: P0, cmmid: u32) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mscms.dll" "system" fn UnregisterCMMW(pmachinename : windows_core::PCWSTR, cmmid : u32) -> super::super::Foundation:: BOOL);
    UnregisterCMMW(pmachinename.param().abi(), core::mem::transmute(cmmid))
}
#[inline]
pub unsafe fn UpdateICMRegKeyA<P1, P2>(reserved: Option<u32>, lpszcmid: P1, lpszfilename: P2, command: ICM_COMMAND) -> super::super::Foundation::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("gdi32.dll" "system" fn UpdateICMRegKeyA(reserved : u32, lpszcmid : windows_core::PCSTR, lpszfilename : windows_core::PCSTR, command : ICM_COMMAND) -> super::super::Foundation:: BOOL);
    UpdateICMRegKeyA(core::mem::transmute(reserved.unwrap_or(core::mem::zeroed())), lpszcmid.param().abi(), lpszfilename.param().abi(), core::mem::transmute(command))
}
#[inline]
pub unsafe fn UpdateICMRegKeyW<P1, P2>(reserved: Option<u32>, lpszcmid: P1, lpszfilename: P2, command: ICM_COMMAND) -> super::super::Foundation::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("gdi32.dll" "system" fn UpdateICMRegKeyW(reserved : u32, lpszcmid : windows_core::PCWSTR, lpszfilename : windows_core::PCWSTR, command : ICM_COMMAND) -> super::super::Foundation:: BOOL);
    UpdateICMRegKeyW(core::mem::transmute(reserved.unwrap_or(core::mem::zeroed())), lpszcmid.param().abi(), lpszfilename.param().abi(), core::mem::transmute(command))
}
#[inline]
pub unsafe fn WcsAssociateColorProfileWithDevice<P1, P2>(scope: WCS_PROFILE_MANAGEMENT_SCOPE, pprofilename: P1, pdevicename: P2) -> super::super::Foundation::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mscms.dll" "system" fn WcsAssociateColorProfileWithDevice(scope : WCS_PROFILE_MANAGEMENT_SCOPE, pprofilename : windows_core::PCWSTR, pdevicename : windows_core::PCWSTR) -> super::super::Foundation:: BOOL);
    WcsAssociateColorProfileWithDevice(core::mem::transmute(scope), pprofilename.param().abi(), pdevicename.param().abi())
}
#[inline]
pub unsafe fn WcsCheckColors(hcolortransform: isize, ninputchannels: u32, cdtinput: COLORDATATYPE, cbinput: u32, pinputdata: *const core::ffi::c_void, paresult: &mut [u8]) -> super::super::Foundation::BOOL {
    windows_targets::link!("mscms.dll" "system" fn WcsCheckColors(hcolortransform : isize, ncolors : u32, ninputchannels : u32, cdtinput : COLORDATATYPE, cbinput : u32, pinputdata : *const core::ffi::c_void, paresult : *mut u8) -> super::super::Foundation:: BOOL);
    WcsCheckColors(core::mem::transmute(hcolortransform), paresult.len().try_into().unwrap(), core::mem::transmute(ninputchannels), core::mem::transmute(cdtinput), core::mem::transmute(cbinput), core::mem::transmute(pinputdata), core::mem::transmute(paresult.as_ptr()))
}
#[inline]
pub unsafe fn WcsCreateIccProfile(hwcsprofile: isize, dwoptions: u32) -> isize {
    windows_targets::link!("mscms.dll" "system" fn WcsCreateIccProfile(hwcsprofile : isize, dwoptions : u32) -> isize);
    WcsCreateIccProfile(core::mem::transmute(hwcsprofile), core::mem::transmute(dwoptions))
}
#[inline]
pub unsafe fn WcsDisassociateColorProfileFromDevice<P1, P2>(scope: WCS_PROFILE_MANAGEMENT_SCOPE, pprofilename: P1, pdevicename: P2) -> super::super::Foundation::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mscms.dll" "system" fn WcsDisassociateColorProfileFromDevice(scope : WCS_PROFILE_MANAGEMENT_SCOPE, pprofilename : windows_core::PCWSTR, pdevicename : windows_core::PCWSTR) -> super::super::Foundation:: BOOL);
    WcsDisassociateColorProfileFromDevice(core::mem::transmute(scope), pprofilename.param().abi(), pdevicename.param().abi())
}
#[inline]
pub unsafe fn WcsEnumColorProfiles(scope: WCS_PROFILE_MANAGEMENT_SCOPE, penumrecord: *const ENUMTYPEW, pbuffer: &mut [u8], pnprofiles: Option<*mut u32>) -> super::super::Foundation::BOOL {
    windows_targets::link!("mscms.dll" "system" fn WcsEnumColorProfiles(scope : WCS_PROFILE_MANAGEMENT_SCOPE, penumrecord : *const ENUMTYPEW, pbuffer : *mut u8, dwsize : u32, pnprofiles : *mut u32) -> super::super::Foundation:: BOOL);
    WcsEnumColorProfiles(core::mem::transmute(scope), core::mem::transmute(penumrecord), core::mem::transmute(pbuffer.as_ptr()), pbuffer.len().try_into().unwrap(), core::mem::transmute(pnprofiles.unwrap_or(core::mem::zeroed())))
}
#[inline]
pub unsafe fn WcsEnumColorProfilesSize(scope: WCS_PROFILE_MANAGEMENT_SCOPE, penumrecord: *const ENUMTYPEW, pdwsize: *mut u32) -> super::super::Foundation::BOOL {
    windows_targets::link!("mscms.dll" "system" fn WcsEnumColorProfilesSize(scope : WCS_PROFILE_MANAGEMENT_SCOPE, penumrecord : *const ENUMTYPEW, pdwsize : *mut u32) -> super::super::Foundation:: BOOL);
    WcsEnumColorProfilesSize(core::mem::transmute(scope), core::mem::transmute(penumrecord), core::mem::transmute(pdwsize))
}
#[inline]
pub unsafe fn WcsGetCalibrationManagementState(pbisenabled: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL {
    windows_targets::link!("mscms.dll" "system" fn WcsGetCalibrationManagementState(pbisenabled : *mut super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
    WcsGetCalibrationManagementState(core::mem::transmute(pbisenabled))
}
#[inline]
pub unsafe fn WcsGetDefaultColorProfile<P1>(scope: WCS_PROFILE_MANAGEMENT_SCOPE, pdevicename: P1, cptcolorprofiletype: COLORPROFILETYPE, cpstcolorprofilesubtype: COLORPROFILESUBTYPE, dwprofileid: u32, cbprofilename: u32, pprofilename: windows_core::PWSTR) -> super::super::Foundation::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mscms.dll" "system" fn WcsGetDefaultColorProfile(scope : WCS_PROFILE_MANAGEMENT_SCOPE, pdevicename : windows_core::PCWSTR, cptcolorprofiletype : COLORPROFILETYPE, cpstcolorprofilesubtype : COLORPROFILESUBTYPE, dwprofileid : u32, cbprofilename : u32, pprofilename : windows_core::PWSTR) -> super::super::Foundation:: BOOL);
    WcsGetDefaultColorProfile(core::mem::transmute(scope), pdevicename.param().abi(), core::mem::transmute(cptcolorprofiletype), core::mem::transmute(cpstcolorprofilesubtype), core::mem::transmute(dwprofileid), core::mem::transmute(cbprofilename), core::mem::transmute(pprofilename))
}
#[inline]
pub unsafe fn WcsGetDefaultColorProfileSize<P1>(scope: WCS_PROFILE_MANAGEMENT_SCOPE, pdevicename: P1, cptcolorprofiletype: COLORPROFILETYPE, cpstcolorprofilesubtype: COLORPROFILESUBTYPE, dwprofileid: u32, pcbprofilename: *mut u32) -> super::super::Foundation::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mscms.dll" "system" fn WcsGetDefaultColorProfileSize(scope : WCS_PROFILE_MANAGEMENT_SCOPE, pdevicename : windows_core::PCWSTR, cptcolorprofiletype : COLORPROFILETYPE, cpstcolorprofilesubtype : COLORPROFILESUBTYPE, dwprofileid : u32, pcbprofilename : *mut u32) -> super::super::Foundation:: BOOL);
    WcsGetDefaultColorProfileSize(core::mem::transmute(scope), pdevicename.param().abi(), core::mem::transmute(cptcolorprofiletype), core::mem::transmute(cpstcolorprofilesubtype), core::mem::transmute(dwprofileid), core::mem::transmute(pcbprofilename))
}
#[inline]
pub unsafe fn WcsGetDefaultRenderingIntent(scope: WCS_PROFILE_MANAGEMENT_SCOPE, pdwrenderingintent: *mut u32) -> super::super::Foundation::BOOL {
    windows_targets::link!("mscms.dll" "system" fn WcsGetDefaultRenderingIntent(scope : WCS_PROFILE_MANAGEMENT_SCOPE, pdwrenderingintent : *mut u32) -> super::super::Foundation:: BOOL);
    WcsGetDefaultRenderingIntent(core::mem::transmute(scope), core::mem::transmute(pdwrenderingintent))
}
#[inline]
pub unsafe fn WcsGetUsePerUserProfiles<P0>(pdevicename: P0, dwdeviceclass: u32, puseperuserprofiles: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mscms.dll" "system" fn WcsGetUsePerUserProfiles(pdevicename : windows_core::PCWSTR, dwdeviceclass : u32, puseperuserprofiles : *mut super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
    WcsGetUsePerUserProfiles(pdevicename.param().abi(), core::mem::transmute(dwdeviceclass), core::mem::transmute(puseperuserprofiles))
}
#[inline]
pub unsafe fn WcsOpenColorProfileA(pcdmpprofile: *const PROFILE, pcampprofile: Option<*const PROFILE>, pgmmpprofile: Option<*const PROFILE>, dwdesireaccess: u32, dwsharemode: u32, dwcreationmode: u32, dwflags: u32) -> isize {
    windows_targets::link!("mscms.dll" "system" fn WcsOpenColorProfileA(pcdmpprofile : *const PROFILE, pcampprofile : *const PROFILE, pgmmpprofile : *const PROFILE, dwdesireaccess : u32, dwsharemode : u32, dwcreationmode : u32, dwflags : u32) -> isize);
    WcsOpenColorProfileA(core::mem::transmute(pcdmpprofile), core::mem::transmute(pcampprofile.unwrap_or(core::mem::zeroed())), core::mem::transmute(pgmmpprofile.unwrap_or(core::mem::zeroed())), core::mem::transmute(dwdesireaccess), core::mem::transmute(dwsharemode), core::mem::transmute(dwcreationmode), core::mem::transmute(dwflags))
}
#[inline]
pub unsafe fn WcsOpenColorProfileW(pcdmpprofile: *const PROFILE, pcampprofile: Option<*const PROFILE>, pgmmpprofile: Option<*const PROFILE>, dwdesireaccess: u32, dwsharemode: u32, dwcreationmode: u32, dwflags: u32) -> isize {
    windows_targets::link!("mscms.dll" "system" fn WcsOpenColorProfileW(pcdmpprofile : *const PROFILE, pcampprofile : *const PROFILE, pgmmpprofile : *const PROFILE, dwdesireaccess : u32, dwsharemode : u32, dwcreationmode : u32, dwflags : u32) -> isize);
    WcsOpenColorProfileW(core::mem::transmute(pcdmpprofile), core::mem::transmute(pcampprofile.unwrap_or(core::mem::zeroed())), core::mem::transmute(pgmmpprofile.unwrap_or(core::mem::zeroed())), core::mem::transmute(dwdesireaccess), core::mem::transmute(dwsharemode), core::mem::transmute(dwcreationmode), core::mem::transmute(dwflags))
}
#[inline]
pub unsafe fn WcsSetCalibrationManagementState(bisenabled: bool) -> super::super::Foundation::BOOL {
    windows_targets::link!("mscms.dll" "system" fn WcsSetCalibrationManagementState(bisenabled : super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
    WcsSetCalibrationManagementState(bisenabled.into())
}
#[inline]
pub unsafe fn WcsSetDefaultColorProfile<P1, P5>(scope: WCS_PROFILE_MANAGEMENT_SCOPE, pdevicename: P1, cptcolorprofiletype: COLORPROFILETYPE, cpstcolorprofilesubtype: COLORPROFILESUBTYPE, dwprofileid: u32, pprofilename: P5) -> super::super::Foundation::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mscms.dll" "system" fn WcsSetDefaultColorProfile(scope : WCS_PROFILE_MANAGEMENT_SCOPE, pdevicename : windows_core::PCWSTR, cptcolorprofiletype : COLORPROFILETYPE, cpstcolorprofilesubtype : COLORPROFILESUBTYPE, dwprofileid : u32, pprofilename : windows_core::PCWSTR) -> super::super::Foundation:: BOOL);
    WcsSetDefaultColorProfile(core::mem::transmute(scope), pdevicename.param().abi(), core::mem::transmute(cptcolorprofiletype), core::mem::transmute(cpstcolorprofilesubtype), core::mem::transmute(dwprofileid), pprofilename.param().abi())
}
#[inline]
pub unsafe fn WcsSetDefaultRenderingIntent(scope: WCS_PROFILE_MANAGEMENT_SCOPE, dwrenderingintent: u32) -> super::super::Foundation::BOOL {
    windows_targets::link!("mscms.dll" "system" fn WcsSetDefaultRenderingIntent(scope : WCS_PROFILE_MANAGEMENT_SCOPE, dwrenderingintent : u32) -> super::super::Foundation:: BOOL);
    WcsSetDefaultRenderingIntent(core::mem::transmute(scope), core::mem::transmute(dwrenderingintent))
}
#[inline]
pub unsafe fn WcsSetUsePerUserProfiles<P0>(pdevicename: P0, dwdeviceclass: u32, useperuserprofiles: bool) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("mscms.dll" "system" fn WcsSetUsePerUserProfiles(pdevicename : windows_core::PCWSTR, dwdeviceclass : u32, useperuserprofiles : super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
    WcsSetUsePerUserProfiles(pdevicename.param().abi(), core::mem::transmute(dwdeviceclass), useperuserprofiles.into())
}
#[inline]
pub unsafe fn WcsTranslateColors(hcolortransform: isize, ncolors: u32, ninputchannels: u32, cdtinput: COLORDATATYPE, cbinput: u32, pinputdata: *const core::ffi::c_void, noutputchannels: u32, cdtoutput: COLORDATATYPE, cboutput: u32, poutputdata: *mut core::ffi::c_void) -> super::super::Foundation::BOOL {
    windows_targets::link!("mscms.dll" "system" fn WcsTranslateColors(hcolortransform : isize, ncolors : u32, ninputchannels : u32, cdtinput : COLORDATATYPE, cbinput : u32, pinputdata : *const core::ffi::c_void, noutputchannels : u32, cdtoutput : COLORDATATYPE, cboutput : u32, poutputdata : *mut core::ffi::c_void) -> super::super::Foundation:: BOOL);
    WcsTranslateColors(core::mem::transmute(hcolortransform), core::mem::transmute(ncolors), core::mem::transmute(ninputchannels), core::mem::transmute(cdtinput), core::mem::transmute(cbinput), core::mem::transmute(pinputdata), core::mem::transmute(noutputchannels), core::mem::transmute(cdtoutput), core::mem::transmute(cboutput), core::mem::transmute(poutputdata))
}
pub const ATTRIB_MATTE: u32 = 2u32;
pub const ATTRIB_TRANSPARENCY: u32 = 1u32;
pub const BEST_MODE: u32 = 3u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BMFORMAT(pub i32);
pub const BM_10b_G3CH: BMFORMAT = BMFORMAT(1028i32);
pub const BM_10b_Lab: BMFORMAT = BMFORMAT(1027i32);
pub const BM_10b_RGB: BMFORMAT = BMFORMAT(9i32);
pub const BM_10b_XYZ: BMFORMAT = BMFORMAT(1025i32);
pub const BM_10b_Yxy: BMFORMAT = BMFORMAT(1026i32);
pub const BM_16b_G3CH: BMFORMAT = BMFORMAT(1284i32);
pub const BM_16b_GRAY: BMFORMAT = BMFORMAT(1285i32);
pub const BM_16b_Lab: BMFORMAT = BMFORMAT(1283i32);
pub const BM_16b_RGB: BMFORMAT = BMFORMAT(10i32);
pub const BM_16b_XYZ: BMFORMAT = BMFORMAT(1281i32);
pub const BM_16b_Yxy: BMFORMAT = BMFORMAT(1282i32);
pub const BM_32b_scARGB: BMFORMAT = BMFORMAT(1538i32);
pub const BM_32b_scRGB: BMFORMAT = BMFORMAT(1537i32);
pub const BM_565RGB: BMFORMAT = BMFORMAT(1i32);
pub const BM_5CHANNEL: BMFORMAT = BMFORMAT(517i32);
pub const BM_6CHANNEL: BMFORMAT = BMFORMAT(518i32);
pub const BM_7CHANNEL: BMFORMAT = BMFORMAT(519i32);
pub const BM_8CHANNEL: BMFORMAT = BMFORMAT(520i32);
pub const BM_BGRTRIPLETS: BMFORMAT = BMFORMAT(4i32);
pub const BM_CMYKQUADS: BMFORMAT = BMFORMAT(32i32);
pub const BM_G3CHTRIPLETS: BMFORMAT = BMFORMAT(516i32);
pub const BM_GRAY: BMFORMAT = BMFORMAT(521i32);
pub const BM_KYMCQUADS: BMFORMAT = BMFORMAT(773i32);
pub const BM_LabTRIPLETS: BMFORMAT = BMFORMAT(515i32);
pub const BM_NAMED_INDEX: BMFORMAT = BMFORMAT(1029i32);
pub const BM_R10G10B10A2: BMFORMAT = BMFORMAT(1793i32);
pub const BM_R10G10B10A2_XR: BMFORMAT = BMFORMAT(1794i32);
pub const BM_R16G16B16A16_FLOAT: BMFORMAT = BMFORMAT(1795i32);
pub const BM_RGBTRIPLETS: BMFORMAT = BMFORMAT(2i32);
pub const BM_S2DOT13FIXED_scARGB: BMFORMAT = BMFORMAT(1540i32);
pub const BM_S2DOT13FIXED_scRGB: BMFORMAT = BMFORMAT(1539i32);
pub const BM_XYZTRIPLETS: BMFORMAT = BMFORMAT(513i32);
pub const BM_YxyTRIPLETS: BMFORMAT = BMFORMAT(514i32);
pub const BM_x555G3CH: BMFORMAT = BMFORMAT(260i32);
pub const BM_x555Lab: BMFORMAT = BMFORMAT(259i32);
pub const BM_x555RGB: BMFORMAT = BMFORMAT(0i32);
pub const BM_x555XYZ: BMFORMAT = BMFORMAT(257i32);
pub const BM_x555Yxy: BMFORMAT = BMFORMAT(258i32);
pub const BM_xBGRQUADS: BMFORMAT = BMFORMAT(16i32);
pub const BM_xG3CHQUADS: BMFORMAT = BMFORMAT(772i32);
pub const BM_xRGBQUADS: BMFORMAT = BMFORMAT(8i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BlackInformation {
    pub fBlackOnly: super::super::Foundation::BOOL,
    pub blackWeight: f32,
}
impl Default for BlackInformation {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CATID_WcsPlugin: windows_core::GUID = windows_core::GUID::from_u128(0xa0b402e0_8240_405f_8a16_8a5b4df2f0dd);
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
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CMYKCOLOR {
    pub cyan: u16,
    pub magenta: u16,
    pub yellow: u16,
    pub black: u16,
}
impl Default for CMYKCOLOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
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
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct COLOR_0 {
    pub reserved1: u32,
    pub reserved2: *mut core::ffi::c_void,
}
impl Default for COLOR_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COLORDATATYPE(pub i32);
#[repr(C)]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct COLORMATCHSETUPA {
    pub dwSize: u32,
    pub dwVersion: u32,
    pub dwFlags: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub pSourceName: windows_core::PCSTR,
    pub pDisplayName: windows_core::PCSTR,
    pub pPrinterName: windows_core::PCSTR,
    pub dwRenderIntent: u32,
    pub dwProofingIntent: u32,
    pub pMonitorProfile: windows_core::PSTR,
    pub ccMonitorProfile: u32,
    pub pPrinterProfile: windows_core::PSTR,
    pub ccPrinterProfile: u32,
    pub pTargetProfile: windows_core::PSTR,
    pub ccTargetProfile: u32,
    pub lpfnHook: super::WindowsAndMessaging::DLGPROC,
    pub lParam: super::super::Foundation::LPARAM,
    pub lpfnApplyCallback: PCMSCALLBACKA,
    pub lParamApplyCallback: super::super::Foundation::LPARAM,
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl Default for COLORMATCHSETUPA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct COLORMATCHSETUPW {
    pub dwSize: u32,
    pub dwVersion: u32,
    pub dwFlags: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub pSourceName: windows_core::PCWSTR,
    pub pDisplayName: windows_core::PCWSTR,
    pub pPrinterName: windows_core::PCWSTR,
    pub dwRenderIntent: u32,
    pub dwProofingIntent: u32,
    pub pMonitorProfile: windows_core::PWSTR,
    pub ccMonitorProfile: u32,
    pub pPrinterProfile: windows_core::PWSTR,
    pub ccPrinterProfile: u32,
    pub pTargetProfile: windows_core::PWSTR,
    pub ccTargetProfile: u32,
    pub lpfnHook: super::WindowsAndMessaging::DLGPROC,
    pub lParam: super::super::Foundation::LPARAM,
    pub lpfnApplyCallback: PCMSCALLBACKW,
    pub lParamApplyCallback: super::super::Foundation::LPARAM,
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl Default for COLORMATCHSETUPW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COLORPROFILESUBTYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COLORPROFILETYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COLORTYPE(pub i32);
pub const COLOR_10b_R10G10B10A2: COLORDATATYPE = COLORDATATYPE(5i32);
pub const COLOR_10b_R10G10B10A2_XR: COLORDATATYPE = COLORDATATYPE(6i32);
pub const COLOR_3_CHANNEL: COLORTYPE = COLORTYPE(6i32);
pub const COLOR_5_CHANNEL: COLORTYPE = COLORTYPE(8i32);
pub const COLOR_6_CHANNEL: COLORTYPE = COLORTYPE(9i32);
pub const COLOR_7_CHANNEL: COLORTYPE = COLORTYPE(10i32);
pub const COLOR_8_CHANNEL: COLORTYPE = COLORTYPE(11i32);
pub const COLOR_BYTE: COLORDATATYPE = COLORDATATYPE(1i32);
pub const COLOR_CMYK: COLORTYPE = COLORTYPE(7i32);
pub const COLOR_FLOAT: COLORDATATYPE = COLORDATATYPE(3i32);
pub const COLOR_FLOAT16: COLORDATATYPE = COLORDATATYPE(7i32);
pub const COLOR_GRAY: COLORTYPE = COLORTYPE(1i32);
pub const COLOR_Lab: COLORTYPE = COLORTYPE(5i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COLOR_MATCH_TO_TARGET_ACTION(pub u32);
pub const COLOR_MATCH_VERSION: u32 = 512u32;
pub const COLOR_NAMED: COLORTYPE = COLORTYPE(12i32);
pub const COLOR_RGB: COLORTYPE = COLORTYPE(2i32);
pub const COLOR_S2DOT13FIXED: COLORDATATYPE = COLORDATATYPE(4i32);
pub const COLOR_WORD: COLORDATATYPE = COLORDATATYPE(2i32);
pub const COLOR_XYZ: COLORTYPE = COLORTYPE(3i32);
pub const COLOR_Yxy: COLORTYPE = COLORTYPE(4i32);
pub const CPST_ABSOLUTE_COLORIMETRIC: COLORPROFILESUBTYPE = COLORPROFILESUBTYPE(3i32);
pub const CPST_CUSTOM_WORKING_SPACE: COLORPROFILESUBTYPE = COLORPROFILESUBTYPE(6i32);
pub const CPST_EXTENDED_DISPLAY_COLOR_MODE: COLORPROFILESUBTYPE = COLORPROFILESUBTYPE(8i32);
pub const CPST_NONE: COLORPROFILESUBTYPE = COLORPROFILESUBTYPE(4i32);
pub const CPST_PERCEPTUAL: COLORPROFILESUBTYPE = COLORPROFILESUBTYPE(0i32);
pub const CPST_RELATIVE_COLORIMETRIC: COLORPROFILESUBTYPE = COLORPROFILESUBTYPE(1i32);
pub const CPST_RGB_WORKING_SPACE: COLORPROFILESUBTYPE = COLORPROFILESUBTYPE(5i32);
pub const CPST_SATURATION: COLORPROFILESUBTYPE = COLORPROFILESUBTYPE(2i32);
pub const CPST_STANDARD_DISPLAY_COLOR_MODE: COLORPROFILESUBTYPE = COLORPROFILESUBTYPE(7i32);
pub const CPT_CAMP: COLORPROFILETYPE = COLORPROFILETYPE(2i32);
pub const CPT_DMP: COLORPROFILETYPE = COLORPROFILETYPE(1i32);
pub const CPT_GMMP: COLORPROFILETYPE = COLORPROFILETYPE(3i32);
pub const CPT_ICC: COLORPROFILETYPE = COLORPROFILETYPE(0i32);
pub const CSA_A: u32 = 1u32;
pub const CSA_ABC: u32 = 2u32;
pub const CSA_CMYK: u32 = 7u32;
pub const CSA_DEF: u32 = 3u32;
pub const CSA_DEFG: u32 = 4u32;
pub const CSA_GRAY: u32 = 5u32;
pub const CSA_Lab: u32 = 8u32;
pub const CSA_RGB: u32 = 6u32;
pub const CS_DELETE_TRANSFORM: COLOR_MATCH_TO_TARGET_ACTION = COLOR_MATCH_TO_TARGET_ACTION(3u32);
pub const CS_DISABLE: COLOR_MATCH_TO_TARGET_ACTION = COLOR_MATCH_TO_TARGET_ACTION(2u32);
pub const CS_ENABLE: COLOR_MATCH_TO_TARGET_ACTION = COLOR_MATCH_TO_TARGET_ACTION(1u32);
pub const DONT_USE_EMBEDDED_WCS_PROFILES: i32 = 1i32;
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EMRCREATECOLORSPACE {
    pub emr: super::super::Graphics::Gdi::EMR,
    pub ihCS: u32,
    pub lcs: LOGCOLORSPACEA,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for EMRCREATECOLORSPACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EMRCREATECOLORSPACEW {
    pub emr: super::super::Graphics::Gdi::EMR,
    pub ihCS: u32,
    pub lcs: LOGCOLORSPACEW,
    pub dwFlags: u32,
    pub cbData: u32,
    pub Data: [u8; 1],
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for EMRCREATECOLORSPACEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ENABLE_GAMUT_CHECKING: u32 = 65536u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ENUMTYPEA {
    pub dwSize: u32,
    pub dwVersion: u32,
    pub dwFields: u32,
    pub pDeviceName: windows_core::PCSTR,
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
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ENUMTYPEW {
    pub dwSize: u32,
    pub dwVersion: u32,
    pub dwFields: u32,
    pub pDeviceName: windows_core::PCWSTR,
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
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GENERIC3CHANNEL {
    pub ch1: u16,
    pub ch2: u16,
    pub ch3: u16,
}
impl Default for GENERIC3CHANNEL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GRAYCOLOR {
    pub gray: u16,
}
impl Default for GRAYCOLOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GamutBoundaryDescription {
    pub pPrimaries: *mut PrimaryJabColors,
    pub cNeutralSamples: u32,
    pub pNeutralSamples: *mut JabColorF,
    pub pReferenceShell: *mut GamutShell,
    pub pPlausibleShell: *mut GamutShell,
    pub pPossibleShell: *mut GamutShell,
}
impl Default for GamutBoundaryDescription {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GamutShell {
    pub JMin: f32,
    pub JMax: f32,
    pub cVertices: u32,
    pub cTriangles: u32,
    pub pVertices: *mut JabColorF,
    pub pTriangles: *mut GamutShellTriangle,
}
impl Default for GamutShell {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GamutShellTriangle {
    pub aVertexIndex: [u32; 3],
}
impl Default for GamutShellTriangle {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HCOLORSPACE(pub *mut core::ffi::c_void);
impl HCOLORSPACE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl windows_core::Free for HCOLORSPACE {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            windows_targets::link!("gdi32.dll" "system" fn DeleteColorSpace(hcs : *mut core::ffi::c_void) -> i32);
            DeleteColorSpace(self.0);
        }
    }
}
impl Default for HCOLORSPACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HiFiCOLOR {
    pub channel: [u8; 8],
}
impl Default for HiFiCOLOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type ICMENUMPROCA = Option<unsafe extern "system" fn(param0: windows_core::PCSTR, param1: super::super::Foundation::LPARAM) -> i32>;
pub type ICMENUMPROCW = Option<unsafe extern "system" fn(param0: windows_core::PCWSTR, param1: super::super::Foundation::LPARAM) -> i32>;
pub const ICM_ADDPROFILE: ICM_COMMAND = ICM_COMMAND(1u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ICM_COMMAND(pub u32);
pub const ICM_DELETEPROFILE: ICM_COMMAND = ICM_COMMAND(2u32);
pub const ICM_DONE_OUTSIDEDC: ICM_MODE = ICM_MODE(4i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ICM_MODE(pub i32);
pub const ICM_OFF: ICM_MODE = ICM_MODE(1i32);
pub const ICM_ON: ICM_MODE = ICM_MODE(2i32);
pub const ICM_QUERY: ICM_MODE = ICM_MODE(3i32);
pub const ICM_QUERYMATCH: ICM_COMMAND = ICM_COMMAND(7u32);
pub const ICM_QUERYPROFILE: ICM_COMMAND = ICM_COMMAND(3u32);
pub const ICM_REGISTERICMATCHER: ICM_COMMAND = ICM_COMMAND(5u32);
pub const ICM_SETDEFAULTPROFILE: ICM_COMMAND = ICM_COMMAND(4u32);
pub const ICM_UNREGISTERICMATCHER: ICM_COMMAND = ICM_COMMAND(6u32);
windows_core::imp::define_interface!(IDeviceModelPlugIn, IDeviceModelPlugIn_Vtbl, 0x1cd63475_07c4_46fe_a903_d655316d11fd);
windows_core::imp::interface_hierarchy!(IDeviceModelPlugIn, windows_core::IUnknown);
impl IDeviceModelPlugIn {
    pub unsafe fn Initialize(&self, bstrxml: &windows_core::BSTR, cnummodels: u32, imodelposition: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrxml), core::mem::transmute(cnummodels), core::mem::transmute(imodelposition)).ok()
    }
    pub unsafe fn GetNumChannels(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetNumChannels)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn DeviceToColorimetricColors(&self, cchannels: u32, pdevicevalues: *const f32, pxyzcolors: &mut [XYZColorF]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DeviceToColorimetricColors)(windows_core::Interface::as_raw(self), pxyzcolors.len().try_into().unwrap(), core::mem::transmute(cchannels), core::mem::transmute(pdevicevalues), core::mem::transmute(pxyzcolors.as_ptr())).ok()
    }
    pub unsafe fn ColorimetricToDeviceColors(&self, cchannels: u32, pxyzcolors: &[XYZColorF]) -> windows_core::Result<f32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ColorimetricToDeviceColors)(windows_core::Interface::as_raw(self), pxyzcolors.len().try_into().unwrap(), core::mem::transmute(cchannels), core::mem::transmute(pxyzcolors.as_ptr()), &mut result__).map(|| result__)
    }
    pub unsafe fn ColorimetricToDeviceColorsWithBlack(&self, ccolors: u32, cchannels: u32, pxyzcolors: *const XYZColorF, pblackinformation: *const BlackInformation) -> windows_core::Result<f32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ColorimetricToDeviceColorsWithBlack)(windows_core::Interface::as_raw(self), core::mem::transmute(ccolors), core::mem::transmute(cchannels), core::mem::transmute(pxyzcolors), core::mem::transmute(pblackinformation), &mut result__).map(|| result__)
    }
    pub unsafe fn SetTransformDeviceModelInfo<P1>(&self, imodelposition: u32, pidevicemodelother: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IDeviceModelPlugIn>,
    {
        (windows_core::Interface::vtable(self).SetTransformDeviceModelInfo)(windows_core::Interface::as_raw(self), core::mem::transmute(imodelposition), pidevicemodelother.param().abi()).ok()
    }
    pub unsafe fn GetPrimarySamples(&self, pprimarycolor: *mut PrimaryXYZColors) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetPrimarySamples)(windows_core::Interface::as_raw(self), core::mem::transmute(pprimarycolor)).ok()
    }
    pub unsafe fn GetGamutBoundaryMeshSize(&self, pnumvertices: *mut u32, pnumtriangles: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetGamutBoundaryMeshSize)(windows_core::Interface::as_raw(self), core::mem::transmute(pnumvertices), core::mem::transmute(pnumtriangles)).ok()
    }
    pub unsafe fn GetGamutBoundaryMesh(&self, cchannels: u32, cvertices: u32, pvertices: *mut f32, ptriangles: &mut [GamutShellTriangle]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetGamutBoundaryMesh)(windows_core::Interface::as_raw(self), core::mem::transmute(cchannels), core::mem::transmute(cvertices), ptriangles.len().try_into().unwrap(), core::mem::transmute(pvertices), core::mem::transmute(ptriangles.as_ptr())).ok()
    }
    pub unsafe fn GetNeutralAxisSize(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetNeutralAxisSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetNeutralAxis(&self, pxyzcolors: &mut [XYZColorF]) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetNeutralAxis)(windows_core::Interface::as_raw(self), pxyzcolors.len().try_into().unwrap(), core::mem::transmute(pxyzcolors.as_ptr())).ok()
    }
}
#[repr(C)]
pub struct IDeviceModelPlugIn_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub GetNumChannels: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub DeviceToColorimetricColors: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const f32, *mut XYZColorF) -> windows_core::HRESULT,
    pub ColorimetricToDeviceColors: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const XYZColorF, *mut f32) -> windows_core::HRESULT,
    pub ColorimetricToDeviceColorsWithBlack: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const XYZColorF, *const BlackInformation, *mut f32) -> windows_core::HRESULT,
    pub SetTransformDeviceModelInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPrimarySamples: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PrimaryXYZColors) -> windows_core::HRESULT,
    pub GetGamutBoundaryMeshSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub GetGamutBoundaryMesh: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, *mut f32, *mut GamutShellTriangle) -> windows_core::HRESULT,
    pub GetNeutralAxisSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetNeutralAxis: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut XYZColorF) -> windows_core::HRESULT,
}
pub trait IDeviceModelPlugIn_Impl: windows_core::IUnknownImpl {
    fn Initialize(&self, bstrxml: &windows_core::BSTR, cnummodels: u32, imodelposition: u32) -> windows_core::Result<()>;
    fn GetNumChannels(&self) -> windows_core::Result<u32>;
    fn DeviceToColorimetricColors(&self, ccolors: u32, cchannels: u32, pdevicevalues: *const f32, pxyzcolors: *mut XYZColorF) -> windows_core::Result<()>;
    fn ColorimetricToDeviceColors(&self, ccolors: u32, cchannels: u32, pxyzcolors: *const XYZColorF) -> windows_core::Result<f32>;
    fn ColorimetricToDeviceColorsWithBlack(&self, ccolors: u32, cchannels: u32, pxyzcolors: *const XYZColorF, pblackinformation: *const BlackInformation) -> windows_core::Result<f32>;
    fn SetTransformDeviceModelInfo(&self, imodelposition: u32, pidevicemodelother: Option<&IDeviceModelPlugIn>) -> windows_core::Result<()>;
    fn GetPrimarySamples(&self, pprimarycolor: *mut PrimaryXYZColors) -> windows_core::Result<()>;
    fn GetGamutBoundaryMeshSize(&self, pnumvertices: *mut u32, pnumtriangles: *mut u32) -> windows_core::Result<()>;
    fn GetGamutBoundaryMesh(&self, cchannels: u32, cvertices: u32, ctriangles: u32, pvertices: *mut f32, ptriangles: *mut GamutShellTriangle) -> windows_core::Result<()>;
    fn GetNeutralAxisSize(&self) -> windows_core::Result<u32>;
    fn GetNeutralAxis(&self, ccolors: u32, pxyzcolors: *mut XYZColorF) -> windows_core::Result<()>;
}
impl IDeviceModelPlugIn_Vtbl {
    pub const fn new<Identity: IDeviceModelPlugIn_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Initialize<Identity: IDeviceModelPlugIn_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrxml: *mut core::ffi::c_void, cnummodels: u32, imodelposition: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDeviceModelPlugIn_Impl::Initialize(this, core::mem::transmute(&bstrxml), core::mem::transmute_copy(&cnummodels), core::mem::transmute_copy(&imodelposition)).into()
        }
        unsafe extern "system" fn GetNumChannels<Identity: IDeviceModelPlugIn_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnumchannels: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDeviceModelPlugIn_Impl::GetNumChannels(this) {
                Ok(ok__) => {
                    pnumchannels.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceToColorimetricColors<Identity: IDeviceModelPlugIn_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ccolors: u32, cchannels: u32, pdevicevalues: *const f32, pxyzcolors: *mut XYZColorF) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDeviceModelPlugIn_Impl::DeviceToColorimetricColors(this, core::mem::transmute_copy(&ccolors), core::mem::transmute_copy(&cchannels), core::mem::transmute_copy(&pdevicevalues), core::mem::transmute_copy(&pxyzcolors)).into()
        }
        unsafe extern "system" fn ColorimetricToDeviceColors<Identity: IDeviceModelPlugIn_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ccolors: u32, cchannels: u32, pxyzcolors: *const XYZColorF, pdevicevalues: *mut f32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDeviceModelPlugIn_Impl::ColorimetricToDeviceColors(this, core::mem::transmute_copy(&ccolors), core::mem::transmute_copy(&cchannels), core::mem::transmute_copy(&pxyzcolors)) {
                Ok(ok__) => {
                    pdevicevalues.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ColorimetricToDeviceColorsWithBlack<Identity: IDeviceModelPlugIn_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ccolors: u32, cchannels: u32, pxyzcolors: *const XYZColorF, pblackinformation: *const BlackInformation, pdevicevalues: *mut f32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDeviceModelPlugIn_Impl::ColorimetricToDeviceColorsWithBlack(this, core::mem::transmute_copy(&ccolors), core::mem::transmute_copy(&cchannels), core::mem::transmute_copy(&pxyzcolors), core::mem::transmute_copy(&pblackinformation)) {
                Ok(ok__) => {
                    pdevicevalues.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransformDeviceModelInfo<Identity: IDeviceModelPlugIn_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, imodelposition: u32, pidevicemodelother: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDeviceModelPlugIn_Impl::SetTransformDeviceModelInfo(this, core::mem::transmute_copy(&imodelposition), windows_core::from_raw_borrowed(&pidevicemodelother)).into()
        }
        unsafe extern "system" fn GetPrimarySamples<Identity: IDeviceModelPlugIn_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprimarycolor: *mut PrimaryXYZColors) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDeviceModelPlugIn_Impl::GetPrimarySamples(this, core::mem::transmute_copy(&pprimarycolor)).into()
        }
        unsafe extern "system" fn GetGamutBoundaryMeshSize<Identity: IDeviceModelPlugIn_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnumvertices: *mut u32, pnumtriangles: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDeviceModelPlugIn_Impl::GetGamutBoundaryMeshSize(this, core::mem::transmute_copy(&pnumvertices), core::mem::transmute_copy(&pnumtriangles)).into()
        }
        unsafe extern "system" fn GetGamutBoundaryMesh<Identity: IDeviceModelPlugIn_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cchannels: u32, cvertices: u32, ctriangles: u32, pvertices: *mut f32, ptriangles: *mut GamutShellTriangle) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDeviceModelPlugIn_Impl::GetGamutBoundaryMesh(this, core::mem::transmute_copy(&cchannels), core::mem::transmute_copy(&cvertices), core::mem::transmute_copy(&ctriangles), core::mem::transmute_copy(&pvertices), core::mem::transmute_copy(&ptriangles)).into()
        }
        unsafe extern "system" fn GetNeutralAxisSize<Identity: IDeviceModelPlugIn_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pccolors: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDeviceModelPlugIn_Impl::GetNeutralAxisSize(this) {
                Ok(ok__) => {
                    pccolors.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNeutralAxis<Identity: IDeviceModelPlugIn_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ccolors: u32, pxyzcolors: *mut XYZColorF) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDeviceModelPlugIn_Impl::GetNeutralAxis(this, core::mem::transmute_copy(&ccolors), core::mem::transmute_copy(&pxyzcolors)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            GetNumChannels: GetNumChannels::<Identity, OFFSET>,
            DeviceToColorimetricColors: DeviceToColorimetricColors::<Identity, OFFSET>,
            ColorimetricToDeviceColors: ColorimetricToDeviceColors::<Identity, OFFSET>,
            ColorimetricToDeviceColorsWithBlack: ColorimetricToDeviceColorsWithBlack::<Identity, OFFSET>,
            SetTransformDeviceModelInfo: SetTransformDeviceModelInfo::<Identity, OFFSET>,
            GetPrimarySamples: GetPrimarySamples::<Identity, OFFSET>,
            GetGamutBoundaryMeshSize: GetGamutBoundaryMeshSize::<Identity, OFFSET>,
            GetGamutBoundaryMesh: GetGamutBoundaryMesh::<Identity, OFFSET>,
            GetNeutralAxisSize: GetNeutralAxisSize::<Identity, OFFSET>,
            GetNeutralAxis: GetNeutralAxis::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDeviceModelPlugIn as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDeviceModelPlugIn {}
windows_core::imp::define_interface!(IGamutMapModelPlugIn, IGamutMapModelPlugIn_Vtbl, 0x2dd80115_ad1e_41f6_a219_a4f4b583d1f9);
windows_core::imp::interface_hierarchy!(IGamutMapModelPlugIn, windows_core::IUnknown);
impl IGamutMapModelPlugIn {
    pub unsafe fn Initialize<P1, P2>(&self, bstrxml: &windows_core::BSTR, psrcplugin: P1, pdestplugin: P2, psrcgbd: *const GamutBoundaryDescription, pdestgbd: *const GamutBoundaryDescription) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IDeviceModelPlugIn>,
        P2: windows_core::Param<IDeviceModelPlugIn>,
    {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrxml), psrcplugin.param().abi(), pdestplugin.param().abi(), core::mem::transmute(psrcgbd), core::mem::transmute(pdestgbd)).ok()
    }
    pub unsafe fn SourceToDestinationAppearanceColors(&self, ccolors: u32, pinputcolors: *const JChColorF) -> windows_core::Result<JChColorF> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SourceToDestinationAppearanceColors)(windows_core::Interface::as_raw(self), core::mem::transmute(ccolors), core::mem::transmute(pinputcolors), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IGamutMapModelPlugIn_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const GamutBoundaryDescription, *const GamutBoundaryDescription) -> windows_core::HRESULT,
    pub SourceToDestinationAppearanceColors: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const JChColorF, *mut JChColorF) -> windows_core::HRESULT,
}
pub trait IGamutMapModelPlugIn_Impl: windows_core::IUnknownImpl {
    fn Initialize(&self, bstrxml: &windows_core::BSTR, psrcplugin: Option<&IDeviceModelPlugIn>, pdestplugin: Option<&IDeviceModelPlugIn>, psrcgbd: *const GamutBoundaryDescription, pdestgbd: *const GamutBoundaryDescription) -> windows_core::Result<()>;
    fn SourceToDestinationAppearanceColors(&self, ccolors: u32, pinputcolors: *const JChColorF) -> windows_core::Result<JChColorF>;
}
impl IGamutMapModelPlugIn_Vtbl {
    pub const fn new<Identity: IGamutMapModelPlugIn_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Initialize<Identity: IGamutMapModelPlugIn_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrxml: *mut core::ffi::c_void, psrcplugin: *mut core::ffi::c_void, pdestplugin: *mut core::ffi::c_void, psrcgbd: *const GamutBoundaryDescription, pdestgbd: *const GamutBoundaryDescription) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IGamutMapModelPlugIn_Impl::Initialize(this, core::mem::transmute(&bstrxml), windows_core::from_raw_borrowed(&psrcplugin), windows_core::from_raw_borrowed(&pdestplugin), core::mem::transmute_copy(&psrcgbd), core::mem::transmute_copy(&pdestgbd)).into()
        }
        unsafe extern "system" fn SourceToDestinationAppearanceColors<Identity: IGamutMapModelPlugIn_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ccolors: u32, pinputcolors: *const JChColorF, poutputcolors: *mut JChColorF) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IGamutMapModelPlugIn_Impl::SourceToDestinationAppearanceColors(this, core::mem::transmute_copy(&ccolors), core::mem::transmute_copy(&pinputcolors)) {
                Ok(ok__) => {
                    poutputcolors.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            SourceToDestinationAppearanceColors: SourceToDestinationAppearanceColors::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGamutMapModelPlugIn as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IGamutMapModelPlugIn {}
pub const INDEX_DONT_CARE: u32 = 0u32;
pub const INTENT_ABSOLUTE_COLORIMETRIC: u32 = 3u32;
pub const INTENT_PERCEPTUAL: u32 = 0u32;
pub const INTENT_RELATIVE_COLORIMETRIC: u32 = 1u32;
pub const INTENT_SATURATION: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct JChColorF {
    pub J: f32,
    pub C: f32,
    pub h: f32,
}
impl Default for JChColorF {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct JabColorF {
    pub J: f32,
    pub a: f32,
    pub b: f32,
}
impl Default for JabColorF {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LCSCSTYPE(pub i32);
pub const LCS_CALIBRATED_RGB: LCSCSTYPE = LCSCSTYPE(0i32);
pub const LCS_WINDOWS_COLOR_SPACE: LCSCSTYPE = LCSCSTYPE(1466527264i32);
pub const LCS_sRGB: LCSCSTYPE = LCSCSTYPE(1934772034i32);
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LOGCOLORSPACEA {
    pub lcsSignature: u32,
    pub lcsVersion: u32,
    pub lcsSize: u32,
    pub lcsCSType: LCSCSTYPE,
    pub lcsIntent: i32,
    pub lcsEndpoints: super::super::Graphics::Gdi::CIEXYZTRIPLE,
    pub lcsGammaRed: u32,
    pub lcsGammaGreen: u32,
    pub lcsGammaBlue: u32,
    pub lcsFilename: [i8; 260],
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for LOGCOLORSPACEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LOGCOLORSPACEW {
    pub lcsSignature: u32,
    pub lcsVersion: u32,
    pub lcsSize: u32,
    pub lcsCSType: LCSCSTYPE,
    pub lcsIntent: i32,
    pub lcsEndpoints: super::super::Graphics::Gdi::CIEXYZTRIPLE,
    pub lcsGammaRed: u32,
    pub lcsGammaGreen: u32,
    pub lcsGammaBlue: u32,
    pub lcsFilename: [u16; 260],
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for LOGCOLORSPACEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type LPBMCALLBACKFN = Option<unsafe extern "system" fn(param0: u32, param1: u32, param2: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL>;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LabCOLOR {
    pub L: u16,
    pub a: u16,
    pub b: u16,
}
impl Default for LabCOLOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MAX_COLOR_CHANNELS: u32 = 8u32;
pub const MicrosoftHardwareColorV2: WCS_DEVICE_CAPABILITIES_TYPE = WCS_DEVICE_CAPABILITIES_TYPE(2i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NAMEDCOLOR {
    pub dwIndex: u32,
}
impl Default for NAMEDCOLOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NAMED_PROFILE_INFO {
    pub dwFlags: u32,
    pub dwCount: u32,
    pub dwCountDevCoordinates: u32,
    pub szPrefix: [i8; 32],
    pub szSuffix: [i8; 32],
}
impl Default for NAMED_PROFILE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NORMAL_MODE: u32 = 2u32;
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub type PCMSCALLBACKA = Option<unsafe extern "system" fn(param0: *mut COLORMATCHSETUPA, param1: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL>;
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub type PCMSCALLBACKW = Option<unsafe extern "system" fn(param0: *mut COLORMATCHSETUPW, param1: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL>;
pub const PRESERVEBLACK: u32 = 1048576u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, PartialEq)]
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
impl Default for PROFILEHEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PROFILE_FILENAME: u32 = 1u32;
pub const PROFILE_MEMBUFFER: u32 = 2u32;
pub const PROFILE_READ: u32 = 1u32;
pub const PROFILE_READWRITE: u32 = 2u32;
pub const PROOF_MODE: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
impl Default for PrimaryJabColors {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
impl Default for PrimaryXYZColors {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const RESERVED: u32 = 2147483648u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RGBCOLOR {
    pub red: u16,
    pub green: u16,
    pub blue: u16,
}
impl Default for RGBCOLOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SEQUENTIAL_TRANSFORM: u32 = 2155872256u32;
pub const USE_RELATIVE_COLORIMETRIC: u32 = 131072u32;
pub const VideoCardGammaTable: WCS_DEVICE_CAPABILITIES_TYPE = WCS_DEVICE_CAPABILITIES_TYPE(1i32);
pub const WCS_ALWAYS: u32 = 2097152u32;
pub const WCS_DEFAULT: i32 = 0i32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WCS_DEVICE_CAPABILITIES_TYPE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WCS_DEVICE_MHC2_CAPABILITIES {
    pub Size: u32,
    pub SupportsMhc2: super::super::Foundation::BOOL,
    pub RegammaLutEntryCount: u32,
    pub CscXyzMatrixRows: u32,
    pub CscXyzMatrixColumns: u32,
}
impl Default for WCS_DEVICE_MHC2_CAPABILITIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WCS_DEVICE_VCGT_CAPABILITIES {
    pub Size: u32,
    pub SupportsVcgt: super::super::Foundation::BOOL,
}
impl Default for WCS_DEVICE_VCGT_CAPABILITIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WCS_ICCONLY: i32 = 65536i32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WCS_PROFILE_MANAGEMENT_SCOPE(pub i32);
pub const WCS_PROFILE_MANAGEMENT_SCOPE_CURRENT_USER: WCS_PROFILE_MANAGEMENT_SCOPE = WCS_PROFILE_MANAGEMENT_SCOPE(1i32);
pub const WCS_PROFILE_MANAGEMENT_SCOPE_SYSTEM_WIDE: WCS_PROFILE_MANAGEMENT_SCOPE = WCS_PROFILE_MANAGEMENT_SCOPE(0i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct XYZCOLOR {
    pub X: u16,
    pub Y: u16,
    pub Z: u16,
}
impl Default for XYZCOLOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct XYZColorF {
    pub X: f32,
    pub Y: f32,
    pub Z: f32,
}
impl Default for XYZColorF {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct YxyCOLOR {
    pub Y: u16,
    pub x: u16,
    pub y: u16,
}
impl Default for YxyCOLOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
