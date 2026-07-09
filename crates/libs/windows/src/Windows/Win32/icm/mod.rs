#[inline]
pub unsafe fn AssociateColorProfileWithDeviceA<P0, P1, P2>(pmachinename: P0, pprofilename: P1, pdevicename: P2) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("mscms.dll" "system" fn AssociateColorProfileWithDeviceA(pmachinename : windows_core::PCSTR, pprofilename : windows_core::PCSTR, pdevicename : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { AssociateColorProfileWithDeviceA(pmachinename.param().abi(), pprofilename.param().abi(), pdevicename.param().abi()) }
}
#[inline]
pub unsafe fn AssociateColorProfileWithDeviceW<P0, P1, P2>(pmachinename: P0, pprofilename: P1, pdevicename: P2) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mscms.dll" "system" fn AssociateColorProfileWithDeviceW(pmachinename : windows_core::PCWSTR, pprofilename : windows_core::PCWSTR, pdevicename : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { AssociateColorProfileWithDeviceW(pmachinename.param().abi(), pprofilename.param().abi(), pdevicename.param().abi()) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn CMCheckColors(hcmtransform: HCMTRANSFORM, lpainputcolors: *const COLOR, ncolors: u32, ctinput: COLORTYPE, lparesult: *mut u8) -> windows_core::BOOL {
    windows_core::link!("icm32.dll" "system" fn CMCheckColors(hcmtransform : HCMTRANSFORM, lpainputcolors : *const COLOR, ncolors : u32, ctinput : COLORTYPE, lparesult : *mut u8) -> windows_core::BOOL);
    unsafe { CMCheckColors(hcmtransform, lpainputcolors, ncolors, ctinput, lparesult as _) }
}
#[cfg(all(feature = "Win32_wingdi", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn CMCheckColorsInGamut(hcmtransform: HCMTRANSFORM, lpargbtriple: *const super::wingdi::RGBTRIPLE, lparesult: *mut u8, ncount: u32) -> windows_core::BOOL {
    windows_core::link!("icm32.dll" "system" fn CMCheckColorsInGamut(hcmtransform : HCMTRANSFORM, lpargbtriple : *const super::wingdi::RGBTRIPLE, lparesult : *mut u8, ncount : u32) -> windows_core::BOOL);
    unsafe { CMCheckColorsInGamut(hcmtransform, lpargbtriple, lparesult as _, ncount) }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn CMCheckRGBs(hcmtransform: HCMTRANSFORM, lpsrcbits: *const core::ffi::c_void, bminput: BMFORMAT, dwwidth: u32, dwheight: u32, dwstride: u32, lparesult: *mut u8, pfncallback: PBMCALLBACKFN, ulcallbackdata: super::minwindef::LPARAM) -> windows_core::BOOL {
    windows_core::link!("icm32.dll" "system" fn CMCheckRGBs(hcmtransform : HCMTRANSFORM, lpsrcbits : *const core::ffi::c_void, bminput : BMFORMAT, dwwidth : u32, dwheight : u32, dwstride : u32, lparesult : *mut u8, pfncallback : PBMCALLBACKFN, ulcallbackdata : super::minwindef::LPARAM) -> windows_core::BOOL);
    unsafe { CMCheckRGBs(hcmtransform, lpsrcbits, bminput, dwwidth, dwheight, dwstride, lparesult as _, pfncallback, ulcallbackdata) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn CMConvertColorNameToIndex(hprofile: HPROFILE, pacolorname: *const COLOR_NAME, paindex: *mut u32, dwcount: u32) -> windows_core::BOOL {
    windows_core::link!("icm32.dll" "system" fn CMConvertColorNameToIndex(hprofile : HPROFILE, pacolorname : *const COLOR_NAME, paindex : *mut u32, dwcount : u32) -> windows_core::BOOL);
    unsafe { CMConvertColorNameToIndex(hprofile, pacolorname, paindex as _, dwcount) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn CMConvertIndexToColorName(hprofile: HPROFILE, paindex: *const u32, pacolorname: *mut COLOR_NAME, dwcount: u32) -> windows_core::BOOL {
    windows_core::link!("icm32.dll" "system" fn CMConvertIndexToColorName(hprofile : HPROFILE, paindex : *const u32, pacolorname : *mut COLOR_NAME, dwcount : u32) -> windows_core::BOOL);
    unsafe { CMConvertIndexToColorName(hprofile, paindex, pacolorname as _, dwcount) }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn CMCreateDeviceLinkProfile(pahprofiles: &[HPROFILE], padwintents: &[u32], dwflags: u32, lpprofiledata: *mut super::minwindef::LPBYTE) -> windows_core::BOOL {
    windows_core::link!("icm32.dll" "system" fn CMCreateDeviceLinkProfile(pahprofiles : *const HPROFILE, nprofiles : u32, padwintents : *const u32, nintents : u32, dwflags : u32, lpprofiledata : *mut super::minwindef::LPBYTE) -> windows_core::BOOL);
    unsafe { CMCreateDeviceLinkProfile(core::mem::transmute(pahprofiles.as_ptr()), pahprofiles.len().try_into().unwrap(), core::mem::transmute(padwintents.as_ptr()), padwintents.len().try_into().unwrap(), dwflags, lpprofiledata as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn CMCreateMultiProfileTransform(pahprofiles: &[HPROFILE], padwintents: &[u32], dwflags: u32) -> HCMTRANSFORM {
    windows_core::link!("icm32.dll" "system" fn CMCreateMultiProfileTransform(pahprofiles : *const HPROFILE, nprofiles : u32, padwintents : *const u32, nintents : u32, dwflags : u32) -> HCMTRANSFORM);
    unsafe { CMCreateMultiProfileTransform(core::mem::transmute(pahprofiles.as_ptr()), pahprofiles.len().try_into().unwrap(), core::mem::transmute(padwintents.as_ptr()), padwintents.len().try_into().unwrap(), dwflags) }
}
#[cfg(feature = "Win32_wingdi")]
#[inline]
pub unsafe fn CMCreateProfile(lpcolorspace: *mut super::wingdi::LOGCOLORSPACEA, lpprofiledata: *mut LPDEVCHARACTER) -> windows_core::BOOL {
    windows_core::link!("icm32.dll" "system" fn CMCreateProfile(lpcolorspace : *mut super::wingdi::LOGCOLORSPACEA, lpprofiledata : *mut LPDEVCHARACTER) -> windows_core::BOOL);
    unsafe { CMCreateProfile(lpcolorspace as _, lpprofiledata as _) }
}
#[cfg(feature = "Win32_wingdi")]
#[inline]
pub unsafe fn CMCreateProfileW(lpcolorspace: *mut super::wingdi::LOGCOLORSPACEW, lpprofiledata: *mut LPDEVCHARACTER) -> windows_core::BOOL {
    windows_core::link!("icm32.dll" "system" fn CMCreateProfileW(lpcolorspace : *mut super::wingdi::LOGCOLORSPACEW, lpprofiledata : *mut LPDEVCHARACTER) -> windows_core::BOOL);
    unsafe { CMCreateProfileW(lpcolorspace as _, lpprofiledata as _) }
}
#[cfg(all(feature = "Win32_wingdi", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn CMCreateTransform(lpcolorspace: *const super::wingdi::LOGCOLORSPACEA, lpdevcharacter: LPDEVCHARACTER, lptargetdevcharacter: LPDEVCHARACTER) -> HCMTRANSFORM {
    windows_core::link!("icm32.dll" "system" fn CMCreateTransform(lpcolorspace : *const super::wingdi::LOGCOLORSPACEA, lpdevcharacter : LPDEVCHARACTER, lptargetdevcharacter : LPDEVCHARACTER) -> HCMTRANSFORM);
    unsafe { CMCreateTransform(lpcolorspace, lpdevcharacter, lptargetdevcharacter) }
}
#[cfg(all(feature = "Win32_wingdi", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn CMCreateTransformExt(lpcolorspace: *const super::wingdi::LOGCOLORSPACEA, lpdevcharacter: LPDEVCHARACTER, lptargetdevcharacter: LPDEVCHARACTER, dwflags: u32) -> HCMTRANSFORM {
    windows_core::link!("icm32.dll" "system" fn CMCreateTransformExt(lpcolorspace : *const super::wingdi::LOGCOLORSPACEA, lpdevcharacter : LPDEVCHARACTER, lptargetdevcharacter : LPDEVCHARACTER, dwflags : u32) -> HCMTRANSFORM);
    unsafe { CMCreateTransformExt(lpcolorspace, lpdevcharacter, lptargetdevcharacter, dwflags) }
}
#[cfg(all(feature = "Win32_wingdi", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn CMCreateTransformExtW(lpcolorspace: *const super::wingdi::LOGCOLORSPACEW, lpdevcharacter: LPDEVCHARACTER, lptargetdevcharacter: LPDEVCHARACTER, dwflags: u32) -> HCMTRANSFORM {
    windows_core::link!("icm32.dll" "system" fn CMCreateTransformExtW(lpcolorspace : *const super::wingdi::LOGCOLORSPACEW, lpdevcharacter : LPDEVCHARACTER, lptargetdevcharacter : LPDEVCHARACTER, dwflags : u32) -> HCMTRANSFORM);
    unsafe { CMCreateTransformExtW(lpcolorspace, lpdevcharacter, lptargetdevcharacter, dwflags) }
}
#[cfg(all(feature = "Win32_wingdi", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn CMCreateTransformW(lpcolorspace: *const super::wingdi::LOGCOLORSPACEW, lpdevcharacter: LPDEVCHARACTER, lptargetdevcharacter: LPDEVCHARACTER) -> HCMTRANSFORM {
    windows_core::link!("icm32.dll" "system" fn CMCreateTransformW(lpcolorspace : *const super::wingdi::LOGCOLORSPACEW, lpdevcharacter : LPDEVCHARACTER, lptargetdevcharacter : LPDEVCHARACTER) -> HCMTRANSFORM);
    unsafe { CMCreateTransformW(lpcolorspace, lpdevcharacter, lptargetdevcharacter) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn CMDeleteTransform(hcmtransform: HCMTRANSFORM) -> windows_core::BOOL {
    windows_core::link!("icm32.dll" "system" fn CMDeleteTransform(hcmtransform : HCMTRANSFORM) -> windows_core::BOOL);
    unsafe { CMDeleteTransform(hcmtransform as _) }
}
#[inline]
pub unsafe fn CMGetInfo(dwinfo: u32) -> u32 {
    windows_core::link!("icm32.dll" "system" fn CMGetInfo(dwinfo : u32) -> u32);
    unsafe { CMGetInfo(dwinfo) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn CMGetNamedProfileInfo(hprofile: HPROFILE, pnamedprofileinfo: *mut NAMED_PROFILE_INFO) -> windows_core::BOOL {
    windows_core::link!("icm32.dll" "system" fn CMGetNamedProfileInfo(hprofile : HPROFILE, pnamedprofileinfo : *mut NAMED_PROFILE_INFO) -> windows_core::BOOL);
    unsafe { CMGetNamedProfileInfo(hprofile, pnamedprofileinfo as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn CMIsProfileValid(hprofile: HPROFILE, lpbvalid: *mut windows_core::BOOL) -> windows_core::BOOL {
    windows_core::link!("icm32.dll" "system" fn CMIsProfileValid(hprofile : HPROFILE, lpbvalid : *mut windows_core::BOOL) -> windows_core::BOOL);
    unsafe { CMIsProfileValid(hprofile, lpbvalid as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn CMTranslateColors(hcmtransform: HCMTRANSFORM, lpainputcolors: *const COLOR, ncolors: u32, ctinput: COLORTYPE, lpaoutputcolors: *mut COLOR, ctoutput: COLORTYPE) -> windows_core::BOOL {
    windows_core::link!("icm32.dll" "system" fn CMTranslateColors(hcmtransform : HCMTRANSFORM, lpainputcolors : *const COLOR, ncolors : u32, ctinput : COLORTYPE, lpaoutputcolors : *mut COLOR, ctoutput : COLORTYPE) -> windows_core::BOOL);
    unsafe { CMTranslateColors(hcmtransform, lpainputcolors, ncolors, ctinput, lpaoutputcolors as _, ctoutput) }
}
#[cfg(all(feature = "Win32_windef", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn CMTranslateRGB(hcmtransform: HCMTRANSFORM, colorref: super::windef::COLORREF, lpcolorref: *mut u32, dwflags: u32) -> windows_core::BOOL {
    windows_core::link!("icm32.dll" "system" fn CMTranslateRGB(hcmtransform : HCMTRANSFORM, colorref : super::windef::COLORREF, lpcolorref : *mut u32, dwflags : u32) -> windows_core::BOOL);
    unsafe { CMTranslateRGB(hcmtransform, colorref, lpcolorref as _, dwflags) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn CMTranslateRGBs(hcmtransform: HCMTRANSFORM, lpsrcbits: *const core::ffi::c_void, bminput: BMFORMAT, dwwidth: u32, dwheight: u32, dwstride: u32, lpdestbits: *mut core::ffi::c_void, bmoutput: BMFORMAT, dwtranslatedirection: u32) -> windows_core::BOOL {
    windows_core::link!("icm32.dll" "system" fn CMTranslateRGBs(hcmtransform : HCMTRANSFORM, lpsrcbits : *const core::ffi::c_void, bminput : BMFORMAT, dwwidth : u32, dwheight : u32, dwstride : u32, lpdestbits : *mut core::ffi::c_void, bmoutput : BMFORMAT, dwtranslatedirection : u32) -> windows_core::BOOL);
    unsafe { CMTranslateRGBs(hcmtransform, lpsrcbits, bminput, dwwidth, dwheight, dwstride, lpdestbits as _, bmoutput, dwtranslatedirection) }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn CMTranslateRGBsExt(hcmtransform: HCMTRANSFORM, lpsrcbits: *const core::ffi::c_void, bminput: BMFORMAT, dwwidth: u32, dwheight: u32, dwinputstride: u32, lpdestbits: *mut core::ffi::c_void, bmoutput: BMFORMAT, dwoutputstride: u32, lpfncallback: LPBMCALLBACKFN, ulcallbackdata: super::minwindef::LPARAM) -> windows_core::BOOL {
    windows_core::link!("icm32.dll" "system" fn CMTranslateRGBsExt(hcmtransform : HCMTRANSFORM, lpsrcbits : *const core::ffi::c_void, bminput : BMFORMAT, dwwidth : u32, dwheight : u32, dwinputstride : u32, lpdestbits : *mut core::ffi::c_void, bmoutput : BMFORMAT, dwoutputstride : u32, lpfncallback : LPBMCALLBACKFN, ulcallbackdata : super::minwindef::LPARAM) -> windows_core::BOOL);
    unsafe { CMTranslateRGBsExt(hcmtransform, lpsrcbits, bminput, dwwidth, dwheight, dwinputstride, lpdestbits as _, bmoutput, dwoutputstride, lpfncallback, ulcallbackdata) }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn CheckBitmapBits(hcolortransform: HTRANSFORM, psrcbits: *const core::ffi::c_void, bminput: BMFORMAT, dwwidth: u32, dwheight: u32, dwstride: u32, paresult: *mut u8, pfncallback: PBMCALLBACKFN, lpcallbackdata: Option<super::minwindef::LPARAM>) -> windows_core::BOOL {
    windows_core::link!("mscms.dll" "system" fn CheckBitmapBits(hcolortransform : HTRANSFORM, psrcbits : *const core::ffi::c_void, bminput : BMFORMAT, dwwidth : u32, dwheight : u32, dwstride : u32, paresult : *mut u8, pfncallback : PBMCALLBACKFN, lpcallbackdata : super::minwindef::LPARAM) -> windows_core::BOOL);
    unsafe { CheckBitmapBits(hcolortransform, psrcbits, bminput, dwwidth, dwheight, dwstride, paresult as _, pfncallback, lpcallbackdata.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn CheckColors(hcolortransform: HTRANSFORM, painputcolors: *const COLOR, ncolors: u32, ctinput: COLORTYPE, paresult: *mut u8) -> windows_core::BOOL {
    windows_core::link!("mscms.dll" "system" fn CheckColors(hcolortransform : HTRANSFORM, painputcolors : *const COLOR, ncolors : u32, ctinput : COLORTYPE, paresult : *mut u8) -> windows_core::BOOL);
    unsafe { CheckColors(hcolortransform, painputcolors, ncolors, ctinput, paresult as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn CloseColorProfile(hprofile: Option<HPROFILE>) -> windows_core::BOOL {
    windows_core::link!("mscms.dll" "system" fn CloseColorProfile(hprofile : HPROFILE) -> windows_core::BOOL);
    unsafe { CloseColorProfile(hprofile.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn ColorProfileAddDisplayAssociation<P1>(scope: WCS_PROFILE_MANAGEMENT_SCOPE, profilename: P1, targetadapterid: super::winnt::LUID, sourceid: u32, setasdefault: bool, associateasadvancedcolor: bool) -> windows_core::HRESULT
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mscms.dll" "system" fn ColorProfileAddDisplayAssociation(scope : WCS_PROFILE_MANAGEMENT_SCOPE, profilename : windows_core::PCWSTR, targetadapterid : super::winnt::LUID, sourceid : u32, setasdefault : windows_core::BOOL, associateasadvancedcolor : windows_core::BOOL) -> windows_core::HRESULT);
    unsafe { ColorProfileAddDisplayAssociation(scope, profilename.param().abi(), core::mem::transmute(targetadapterid), sourceid, setasdefault.into(), associateasadvancedcolor.into()) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn ColorProfileGetDeviceCapabilities(scope: WCS_PROFILE_MANAGEMENT_SCOPE, targetadapterid: super::winnt::LUID, sourceid: u32, capstype: WCS_DEVICE_CAPABILITIES_TYPE, outputcapabilities: *mut core::ffi::c_void) -> windows_core::HRESULT {
    windows_core::link!("mscms.dll" "system" fn ColorProfileGetDeviceCapabilities(scope : WCS_PROFILE_MANAGEMENT_SCOPE, targetadapterid : super::winnt::LUID, sourceid : u32, capstype : WCS_DEVICE_CAPABILITIES_TYPE, outputcapabilities : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { ColorProfileGetDeviceCapabilities(scope, core::mem::transmute(targetadapterid), sourceid, capstype, outputcapabilities as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn ColorProfileGetDisplayDefault(scope: WCS_PROFILE_MANAGEMENT_SCOPE, targetadapterid: super::winnt::LUID, sourceid: u32, profiletype: COLORPROFILETYPE, profilesubtype: COLORPROFILESUBTYPE) -> windows_core::Result<windows_core::PWSTR> {
    windows_core::link!("mscms.dll" "system" fn ColorProfileGetDisplayDefault(scope : WCS_PROFILE_MANAGEMENT_SCOPE, targetadapterid : super::winnt::LUID, sourceid : u32, profiletype : COLORPROFILETYPE, profilesubtype : COLORPROFILESUBTYPE, profilename : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        ColorProfileGetDisplayDefault(scope, core::mem::transmute(targetadapterid), sourceid, profiletype, profilesubtype, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn ColorProfileGetDisplayList(scope: WCS_PROFILE_MANAGEMENT_SCOPE, targetadapterid: super::winnt::LUID, sourceid: u32, profilelist: *mut *mut windows_core::PWSTR, profilecount: *mut u32) -> windows_core::HRESULT {
    windows_core::link!("mscms.dll" "system" fn ColorProfileGetDisplayList(scope : WCS_PROFILE_MANAGEMENT_SCOPE, targetadapterid : super::winnt::LUID, sourceid : u32, profilelist : *mut *mut windows_core::PWSTR, profilecount : *mut u32) -> windows_core::HRESULT);
    unsafe { ColorProfileGetDisplayList(scope, core::mem::transmute(targetadapterid), sourceid, profilelist as _, profilecount as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn ColorProfileGetDisplayUserScope(targetadapterid: super::winnt::LUID, sourceid: u32) -> windows_core::Result<WCS_PROFILE_MANAGEMENT_SCOPE> {
    windows_core::link!("mscms.dll" "system" fn ColorProfileGetDisplayUserScope(targetadapterid : super::winnt::LUID, sourceid : u32, scope : *mut WCS_PROFILE_MANAGEMENT_SCOPE) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        ColorProfileGetDisplayUserScope(core::mem::transmute(targetadapterid), sourceid, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn ColorProfileRemoveDisplayAssociation<P1>(scope: WCS_PROFILE_MANAGEMENT_SCOPE, profilename: P1, targetadapterid: super::winnt::LUID, sourceid: u32, dissociateadvancedcolor: bool) -> windows_core::HRESULT
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mscms.dll" "system" fn ColorProfileRemoveDisplayAssociation(scope : WCS_PROFILE_MANAGEMENT_SCOPE, profilename : windows_core::PCWSTR, targetadapterid : super::winnt::LUID, sourceid : u32, dissociateadvancedcolor : windows_core::BOOL) -> windows_core::HRESULT);
    unsafe { ColorProfileRemoveDisplayAssociation(scope, profilename.param().abi(), core::mem::transmute(targetadapterid), sourceid, dissociateadvancedcolor.into()) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn ColorProfileSetDisplayDefaultAssociation<P1>(scope: WCS_PROFILE_MANAGEMENT_SCOPE, profilename: P1, profiletype: COLORPROFILETYPE, profilesubtype: COLORPROFILESUBTYPE, targetadapterid: super::winnt::LUID, sourceid: u32) -> windows_core::HRESULT
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mscms.dll" "system" fn ColorProfileSetDisplayDefaultAssociation(scope : WCS_PROFILE_MANAGEMENT_SCOPE, profilename : windows_core::PCWSTR, profiletype : COLORPROFILETYPE, profilesubtype : COLORPROFILESUBTYPE, targetadapterid : super::winnt::LUID, sourceid : u32) -> windows_core::HRESULT);
    unsafe { ColorProfileSetDisplayDefaultAssociation(scope, profilename.param().abi(), profiletype, profilesubtype, core::mem::transmute(targetadapterid), sourceid) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn ConvertColorNameToIndex(hprofile: HPROFILE, pacolorname: *const COLOR_NAME, paindex: *mut u32, dwcount: u32) -> windows_core::BOOL {
    windows_core::link!("mscms.dll" "system" fn ConvertColorNameToIndex(hprofile : HPROFILE, pacolorname : *const COLOR_NAME, paindex : *mut u32, dwcount : u32) -> windows_core::BOOL);
    unsafe { ConvertColorNameToIndex(hprofile, pacolorname, paindex as _, dwcount) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn ConvertIndexToColorName(hprofile: HPROFILE, paindex: *const u32, pacolorname: *mut COLOR_NAME, dwcount: u32) -> windows_core::BOOL {
    windows_core::link!("mscms.dll" "system" fn ConvertIndexToColorName(hprofile : HPROFILE, paindex : *const u32, pacolorname : *mut COLOR_NAME, dwcount : u32) -> windows_core::BOOL);
    unsafe { ConvertIndexToColorName(hprofile, paindex, pacolorname as _, dwcount) }
}
#[cfg(all(feature = "Win32_wingdi", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn CreateColorTransformA(plogcolorspace: *const super::wingdi::LOGCOLORSPACEA, hdestprofile: HPROFILE, htargetprofile: HPROFILE, dwflags: u32) -> HTRANSFORM {
    windows_core::link!("mscms.dll" "system" fn CreateColorTransformA(plogcolorspace : *const super::wingdi::LOGCOLORSPACEA, hdestprofile : HPROFILE, htargetprofile : HPROFILE, dwflags : u32) -> HTRANSFORM);
    unsafe { CreateColorTransformA(plogcolorspace, hdestprofile, htargetprofile, dwflags) }
}
#[cfg(all(feature = "Win32_wingdi", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn CreateColorTransformW(plogcolorspace: *const super::wingdi::LOGCOLORSPACEW, hdestprofile: HPROFILE, htargetprofile: HPROFILE, dwflags: u32) -> HTRANSFORM {
    windows_core::link!("mscms.dll" "system" fn CreateColorTransformW(plogcolorspace : *const super::wingdi::LOGCOLORSPACEW, hdestprofile : HPROFILE, htargetprofile : HPROFILE, dwflags : u32) -> HTRANSFORM);
    unsafe { CreateColorTransformW(plogcolorspace, hdestprofile, htargetprofile, dwflags) }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn CreateDeviceLinkProfile(hprofile: &[HPROFILE], padwintent: &[u32], dwflags: u32, pprofiledata: *mut super::minwindef::PBYTE, indexpreferredcmm: u32) -> windows_core::BOOL {
    windows_core::link!("mscms.dll" "system" fn CreateDeviceLinkProfile(hprofile : *const HPROFILE, nprofiles : u32, padwintent : *const u32, nintents : u32, dwflags : u32, pprofiledata : *mut super::minwindef::PBYTE, indexpreferredcmm : u32) -> windows_core::BOOL);
    unsafe { CreateDeviceLinkProfile(core::mem::transmute(hprofile.as_ptr()), hprofile.len().try_into().unwrap(), core::mem::transmute(padwintent.as_ptr()), padwintent.len().try_into().unwrap(), dwflags, pprofiledata as _, indexpreferredcmm) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn CreateMultiProfileTransform(pahprofiles: &[HPROFILE], padwintent: &[u32], dwflags: u32, indexpreferredcmm: u32) -> HTRANSFORM {
    windows_core::link!("mscms.dll" "system" fn CreateMultiProfileTransform(pahprofiles : *const HPROFILE, nprofiles : u32, padwintent : *const u32, nintents : u32, dwflags : u32, indexpreferredcmm : u32) -> HTRANSFORM);
    unsafe { CreateMultiProfileTransform(core::mem::transmute(pahprofiles.as_ptr()), pahprofiles.len().try_into().unwrap(), core::mem::transmute(padwintent.as_ptr()), padwintent.len().try_into().unwrap(), dwflags, indexpreferredcmm) }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_wingdi"))]
#[inline]
pub unsafe fn CreateProfileFromLogColorSpaceA(plogcolorspace: *const super::wingdi::LOGCOLORSPACEA, pprofile: *mut super::minwindef::PBYTE) -> windows_core::BOOL {
    windows_core::link!("mscms.dll" "system" fn CreateProfileFromLogColorSpaceA(plogcolorspace : *const super::wingdi::LOGCOLORSPACEA, pprofile : *mut super::minwindef::PBYTE) -> windows_core::BOOL);
    unsafe { CreateProfileFromLogColorSpaceA(plogcolorspace, pprofile as _) }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_wingdi"))]
#[inline]
pub unsafe fn CreateProfileFromLogColorSpaceW(plogcolorspace: *const super::wingdi::LOGCOLORSPACEW, pprofile: *mut super::minwindef::PBYTE) -> windows_core::BOOL {
    windows_core::link!("mscms.dll" "system" fn CreateProfileFromLogColorSpaceW(plogcolorspace : *const super::wingdi::LOGCOLORSPACEW, pprofile : *mut super::minwindef::PBYTE) -> windows_core::BOOL);
    unsafe { CreateProfileFromLogColorSpaceW(plogcolorspace, pprofile as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn DeleteColorTransform(hxform: HTRANSFORM) -> windows_core::BOOL {
    windows_core::link!("mscms.dll" "system" fn DeleteColorTransform(hxform : HTRANSFORM) -> windows_core::BOOL);
    unsafe { DeleteColorTransform(hxform as _) }
}
#[inline]
pub unsafe fn DisassociateColorProfileFromDeviceA<P0, P1, P2>(pmachinename: P0, pprofilename: P1, pdevicename: P2) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("mscms.dll" "system" fn DisassociateColorProfileFromDeviceA(pmachinename : windows_core::PCSTR, pprofilename : windows_core::PCSTR, pdevicename : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { DisassociateColorProfileFromDeviceA(pmachinename.param().abi(), pprofilename.param().abi(), pdevicename.param().abi()) }
}
#[inline]
pub unsafe fn DisassociateColorProfileFromDeviceW<P0, P1, P2>(pmachinename: P0, pprofilename: P1, pdevicename: P2) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mscms.dll" "system" fn DisassociateColorProfileFromDeviceW(pmachinename : windows_core::PCWSTR, pprofilename : windows_core::PCWSTR, pdevicename : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { DisassociateColorProfileFromDeviceW(pmachinename.param().abi(), pprofilename.param().abi(), pdevicename.param().abi()) }
}
#[inline]
pub unsafe fn EnumColorProfilesA<P0>(pmachinename: P0, penumrecord: *const ENUMTYPEA, penumerationbuffer: Option<*mut u8>, pdwsizeofenumerationbuffer: *mut u32, pnprofiles: Option<*mut u32>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("mscms.dll" "system" fn EnumColorProfilesA(pmachinename : windows_core::PCSTR, penumrecord : *const ENUMTYPEA, penumerationbuffer : *mut u8, pdwsizeofenumerationbuffer : *mut u32, pnprofiles : *mut u32) -> windows_core::BOOL);
    unsafe { EnumColorProfilesA(pmachinename.param().abi(), penumrecord, penumerationbuffer.unwrap_or(core::mem::zeroed()) as _, pdwsizeofenumerationbuffer as _, pnprofiles.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn EnumColorProfilesW<P0>(pmachinename: P0, penumrecord: *const ENUMTYPEW, penumerationbuffer: Option<*mut u8>, pdwsizeofenumerationbuffer: *mut u32, pnprofiles: Option<*mut u32>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mscms.dll" "system" fn EnumColorProfilesW(pmachinename : windows_core::PCWSTR, penumrecord : *const ENUMTYPEW, penumerationbuffer : *mut u8, pdwsizeofenumerationbuffer : *mut u32, pnprofiles : *mut u32) -> windows_core::BOOL);
    unsafe { EnumColorProfilesW(pmachinename.param().abi(), penumrecord, penumerationbuffer.unwrap_or(core::mem::zeroed()) as _, pdwsizeofenumerationbuffer as _, pnprofiles.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn GetCMMInfo(hcolortransform: HTRANSFORM, param1: u32) -> u32 {
    windows_core::link!("mscms.dll" "system" fn GetCMMInfo(hcolortransform : HTRANSFORM, param1 : u32) -> u32);
    unsafe { GetCMMInfo(hcolortransform, param1) }
}
#[inline]
pub unsafe fn GetColorDirectoryA<P0>(pmachinename: P0, pbuffer: Option<windows_core::PSTR>, pdwsize: *mut u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("mscms.dll" "system" fn GetColorDirectoryA(pmachinename : windows_core::PCSTR, pbuffer : windows_core::PSTR, pdwsize : *mut u32) -> windows_core::BOOL);
    unsafe { GetColorDirectoryA(pmachinename.param().abi(), pbuffer.unwrap_or(core::mem::zeroed()) as _, pdwsize as _) }
}
#[inline]
pub unsafe fn GetColorDirectoryW<P0>(pmachinename: P0, pbuffer: Option<windows_core::PWSTR>, pdwsize: *mut u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mscms.dll" "system" fn GetColorDirectoryW(pmachinename : windows_core::PCWSTR, pbuffer : windows_core::PWSTR, pdwsize : *mut u32) -> windows_core::BOOL);
    unsafe { GetColorDirectoryW(pmachinename.param().abi(), pbuffer.unwrap_or(core::mem::zeroed()) as _, pdwsize as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn GetColorProfileElement(hprofile: HPROFILE, tag: TAGTYPE, dwoffset: u32, pcbelement: *mut u32, pelement: Option<*mut core::ffi::c_void>, pbreference: *mut windows_core::BOOL) -> windows_core::BOOL {
    windows_core::link!("mscms.dll" "system" fn GetColorProfileElement(hprofile : HPROFILE, tag : TAGTYPE, dwoffset : u32, pcbelement : *mut u32, pelement : *mut core::ffi::c_void, pbreference : *mut windows_core::BOOL) -> windows_core::BOOL);
    unsafe { GetColorProfileElement(hprofile, tag, dwoffset, pcbelement as _, pelement.unwrap_or(core::mem::zeroed()) as _, pbreference as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn GetColorProfileElementTag(hprofile: HPROFILE, dwindex: u32, ptag: *mut TAGTYPE) -> windows_core::BOOL {
    windows_core::link!("mscms.dll" "system" fn GetColorProfileElementTag(hprofile : HPROFILE, dwindex : u32, ptag : *mut TAGTYPE) -> windows_core::BOOL);
    unsafe { GetColorProfileElementTag(hprofile, dwindex, ptag as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn GetColorProfileFromHandle(hprofile: HPROFILE, pprofile: Option<*mut u8>, pcbprofile: *mut u32) -> windows_core::BOOL {
    windows_core::link!("mscms.dll" "system" fn GetColorProfileFromHandle(hprofile : HPROFILE, pprofile : *mut u8, pcbprofile : *mut u32) -> windows_core::BOOL);
    unsafe { GetColorProfileFromHandle(hprofile, pprofile.unwrap_or(core::mem::zeroed()) as _, pcbprofile as _) }
}
#[cfg(all(feature = "Win32_wingdi", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn GetColorProfileHeader(hprofile: HPROFILE, pheader: *mut PROFILEHEADER) -> windows_core::BOOL {
    windows_core::link!("mscms.dll" "system" fn GetColorProfileHeader(hprofile : HPROFILE, pheader : *mut PROFILEHEADER) -> windows_core::BOOL);
    unsafe { GetColorProfileHeader(hprofile, pheader as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn GetCountColorProfileElements(hprofile: HPROFILE, pnelementcount: *mut u32) -> windows_core::BOOL {
    windows_core::link!("mscms.dll" "system" fn GetCountColorProfileElements(hprofile : HPROFILE, pnelementcount : *mut u32) -> windows_core::BOOL);
    unsafe { GetCountColorProfileElements(hprofile, pnelementcount as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn GetNamedProfileInfo(hprofile: HPROFILE, pnamedprofileinfo: *mut NAMED_PROFILE_INFO) -> windows_core::BOOL {
    windows_core::link!("mscms.dll" "system" fn GetNamedProfileInfo(hprofile : HPROFILE, pnamedprofileinfo : *mut NAMED_PROFILE_INFO) -> windows_core::BOOL);
    unsafe { GetNamedProfileInfo(hprofile, pnamedprofileinfo as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn GetPS2ColorRenderingDictionary(hprofile: HPROFILE, dwintent: u32, pps2colorrenderingdictionary: Option<*mut u8>, pcbps2colorrenderingdictionary: *mut u32, pbbinary: *mut windows_core::BOOL) -> windows_core::BOOL {
    windows_core::link!("mscms.dll" "system" fn GetPS2ColorRenderingDictionary(hprofile : HPROFILE, dwintent : u32, pps2colorrenderingdictionary : *mut u8, pcbps2colorrenderingdictionary : *mut u32, pbbinary : *mut windows_core::BOOL) -> windows_core::BOOL);
    unsafe { GetPS2ColorRenderingDictionary(hprofile, dwintent, pps2colorrenderingdictionary.unwrap_or(core::mem::zeroed()) as _, pcbps2colorrenderingdictionary as _, pbbinary as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn GetPS2ColorRenderingIntent(hprofile: HPROFILE, dwintent: u32, pbuffer: Option<*mut u8>, pcbps2colorrenderingintent: *mut u32) -> windows_core::BOOL {
    windows_core::link!("mscms.dll" "system" fn GetPS2ColorRenderingIntent(hprofile : HPROFILE, dwintent : u32, pbuffer : *mut u8, pcbps2colorrenderingintent : *mut u32) -> windows_core::BOOL);
    unsafe { GetPS2ColorRenderingIntent(hprofile, dwintent, pbuffer.unwrap_or(core::mem::zeroed()) as _, pcbps2colorrenderingintent as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn GetPS2ColorSpaceArray(hprofile: HPROFILE, dwintent: u32, dwcsatype: u32, pps2colorspacearray: Option<*mut u8>, pcbps2colorspacearray: *mut u32, pbbinary: *mut windows_core::BOOL) -> windows_core::BOOL {
    windows_core::link!("mscms.dll" "system" fn GetPS2ColorSpaceArray(hprofile : HPROFILE, dwintent : u32, dwcsatype : u32, pps2colorspacearray : *mut u8, pcbps2colorspacearray : *mut u32, pbbinary : *mut windows_core::BOOL) -> windows_core::BOOL);
    unsafe { GetPS2ColorSpaceArray(hprofile, dwintent, dwcsatype, pps2colorspacearray.unwrap_or(core::mem::zeroed()) as _, pcbps2colorspacearray as _, pbbinary as _) }
}
#[inline]
pub unsafe fn GetStandardColorSpaceProfileA<P0>(pmachinename: P0, dwscs: u32, pbuffer: Option<windows_core::PSTR>, pcbsize: *mut u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("mscms.dll" "system" fn GetStandardColorSpaceProfileA(pmachinename : windows_core::PCSTR, dwscs : u32, pbuffer : windows_core::PSTR, pcbsize : *mut u32) -> windows_core::BOOL);
    unsafe { GetStandardColorSpaceProfileA(pmachinename.param().abi(), dwscs, pbuffer.unwrap_or(core::mem::zeroed()) as _, pcbsize as _) }
}
#[inline]
pub unsafe fn GetStandardColorSpaceProfileW<P0>(pmachinename: P0, dwscs: u32, pbuffer: Option<windows_core::PWSTR>, pcbsize: *mut u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mscms.dll" "system" fn GetStandardColorSpaceProfileW(pmachinename : windows_core::PCWSTR, dwscs : u32, pbuffer : windows_core::PWSTR, pcbsize : *mut u32) -> windows_core::BOOL);
    unsafe { GetStandardColorSpaceProfileW(pmachinename.param().abi(), dwscs, pbuffer.unwrap_or(core::mem::zeroed()) as _, pcbsize as _) }
}
#[inline]
pub unsafe fn InstallColorProfileA<P0, P1>(pmachinename: P0, pprofilename: P1) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("mscms.dll" "system" fn InstallColorProfileA(pmachinename : windows_core::PCSTR, pprofilename : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { InstallColorProfileA(pmachinename.param().abi(), pprofilename.param().abi()) }
}
#[inline]
pub unsafe fn InstallColorProfileW<P0, P1>(pmachinename: P0, pprofilename: P1) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mscms.dll" "system" fn InstallColorProfileW(pmachinename : windows_core::PCWSTR, pprofilename : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { InstallColorProfileW(pmachinename.param().abi(), pprofilename.param().abi()) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn IsColorProfileTagPresent(hprofile: HPROFILE, tag: TAGTYPE, pbpresent: *mut windows_core::BOOL) -> windows_core::BOOL {
    windows_core::link!("mscms.dll" "system" fn IsColorProfileTagPresent(hprofile : HPROFILE, tag : TAGTYPE, pbpresent : *mut windows_core::BOOL) -> windows_core::BOOL);
    unsafe { IsColorProfileTagPresent(hprofile, tag, pbpresent as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn IsColorProfileValid(hprofile: HPROFILE, pbvalid: *mut windows_core::BOOL) -> windows_core::BOOL {
    windows_core::link!("mscms.dll" "system" fn IsColorProfileValid(hprofile : HPROFILE, pbvalid : *mut windows_core::BOOL) -> windows_core::BOOL);
    unsafe { IsColorProfileValid(hprofile, pbvalid as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn OpenColorProfileA(pprofile: *const PROFILE, dwdesiredaccess: u32, dwsharemode: u32, dwcreationmode: u32) -> HPROFILE {
    windows_core::link!("mscms.dll" "system" fn OpenColorProfileA(pprofile : *const PROFILE, dwdesiredaccess : u32, dwsharemode : u32, dwcreationmode : u32) -> HPROFILE);
    unsafe { OpenColorProfileA(pprofile, dwdesiredaccess, dwsharemode, dwcreationmode) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn OpenColorProfileW(pprofile: *const PROFILE, dwdesiredaccess: u32, dwsharemode: u32, dwcreationmode: u32) -> HPROFILE {
    windows_core::link!("mscms.dll" "system" fn OpenColorProfileW(pprofile : *const PROFILE, dwdesiredaccess : u32, dwsharemode : u32, dwcreationmode : u32) -> HPROFILE);
    unsafe { OpenColorProfileW(pprofile, dwdesiredaccess, dwsharemode, dwcreationmode) }
}
#[inline]
pub unsafe fn RegisterCMMA<P0, P2>(pmachinename: P0, cmmid: u32, pcmmdll: P2) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("mscms.dll" "system" fn RegisterCMMA(pmachinename : windows_core::PCSTR, cmmid : u32, pcmmdll : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { RegisterCMMA(pmachinename.param().abi(), cmmid, pcmmdll.param().abi()) }
}
#[inline]
pub unsafe fn RegisterCMMW<P0, P2>(pmachinename: P0, cmmid: u32, pcmmdll: P2) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mscms.dll" "system" fn RegisterCMMW(pmachinename : windows_core::PCWSTR, cmmid : u32, pcmmdll : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { RegisterCMMW(pmachinename.param().abi(), cmmid, pcmmdll.param().abi()) }
}
#[inline]
pub unsafe fn SelectCMM(dwcmmtype: u32) -> windows_core::BOOL {
    windows_core::link!("mscms.dll" "system" fn SelectCMM(dwcmmtype : u32) -> windows_core::BOOL);
    unsafe { SelectCMM(dwcmmtype) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn SetColorProfileElement(hprofile: HPROFILE, tag: TAGTYPE, dwoffset: u32, pcbelement: *const u32, pelement: *const core::ffi::c_void) -> windows_core::BOOL {
    windows_core::link!("mscms.dll" "system" fn SetColorProfileElement(hprofile : HPROFILE, tag : TAGTYPE, dwoffset : u32, pcbelement : *const u32, pelement : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetColorProfileElement(hprofile, tag, dwoffset, pcbelement, pelement) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn SetColorProfileElementReference(hprofile: HPROFILE, newtag: TAGTYPE, reftag: TAGTYPE) -> windows_core::BOOL {
    windows_core::link!("mscms.dll" "system" fn SetColorProfileElementReference(hprofile : HPROFILE, newtag : TAGTYPE, reftag : TAGTYPE) -> windows_core::BOOL);
    unsafe { SetColorProfileElementReference(hprofile, newtag, reftag) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn SetColorProfileElementSize(hprofile: HPROFILE, tagtype: TAGTYPE, pcbelement: u32) -> windows_core::BOOL {
    windows_core::link!("mscms.dll" "system" fn SetColorProfileElementSize(hprofile : HPROFILE, tagtype : TAGTYPE, pcbelement : u32) -> windows_core::BOOL);
    unsafe { SetColorProfileElementSize(hprofile, tagtype, pcbelement) }
}
#[cfg(all(feature = "Win32_wingdi", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn SetColorProfileHeader(hprofile: HPROFILE, pheader: *const PROFILEHEADER) -> windows_core::BOOL {
    windows_core::link!("mscms.dll" "system" fn SetColorProfileHeader(hprofile : HPROFILE, pheader : *const PROFILEHEADER) -> windows_core::BOOL);
    unsafe { SetColorProfileHeader(hprofile, pheader) }
}
#[inline]
pub unsafe fn SetStandardColorSpaceProfileA<P0, P2>(pmachinename: P0, dwprofileid: u32, pprofilename: P2) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("mscms.dll" "system" fn SetStandardColorSpaceProfileA(pmachinename : windows_core::PCSTR, dwprofileid : u32, pprofilename : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { SetStandardColorSpaceProfileA(pmachinename.param().abi(), dwprofileid, pprofilename.param().abi()) }
}
#[inline]
pub unsafe fn SetStandardColorSpaceProfileW<P0, P2>(pmachinename: P0, dwprofileid: u32, pprofilename: P2) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mscms.dll" "system" fn SetStandardColorSpaceProfileW(pmachinename : windows_core::PCWSTR, dwprofileid : u32, pprofilename : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { SetStandardColorSpaceProfileW(pmachinename.param().abi(), dwprofileid, pprofilename.param().abi()) }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winuser"))]
#[inline]
pub unsafe fn SetupColorMatchingA(pcms: *mut COLORMATCHSETUPA) -> windows_core::BOOL {
    windows_core::link!("icmui.dll" "system" fn SetupColorMatchingA(pcms : *mut COLORMATCHSETUPA) -> windows_core::BOOL);
    unsafe { SetupColorMatchingA(pcms as _) }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winuser"))]
#[inline]
pub unsafe fn SetupColorMatchingW(pcms: *mut COLORMATCHSETUPW) -> windows_core::BOOL {
    windows_core::link!("icmui.dll" "system" fn SetupColorMatchingW(pcms : *mut COLORMATCHSETUPW) -> windows_core::BOOL);
    unsafe { SetupColorMatchingW(pcms as _) }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn TranslateBitmapBits(hcolortransform: HTRANSFORM, psrcbits: *const core::ffi::c_void, bminput: BMFORMAT, dwwidth: u32, dwheight: u32, dwinputstride: u32, pdestbits: *mut core::ffi::c_void, bmoutput: BMFORMAT, dwoutputstride: u32, pfncallback: PBMCALLBACKFN, ulcallbackdata: Option<super::minwindef::LPARAM>) -> windows_core::BOOL {
    windows_core::link!("mscms.dll" "system" fn TranslateBitmapBits(hcolortransform : HTRANSFORM, psrcbits : *const core::ffi::c_void, bminput : BMFORMAT, dwwidth : u32, dwheight : u32, dwinputstride : u32, pdestbits : *mut core::ffi::c_void, bmoutput : BMFORMAT, dwoutputstride : u32, pfncallback : PBMCALLBACKFN, ulcallbackdata : super::minwindef::LPARAM) -> windows_core::BOOL);
    unsafe { TranslateBitmapBits(hcolortransform, psrcbits, bminput, dwwidth, dwheight, dwinputstride, pdestbits as _, bmoutput, dwoutputstride, pfncallback, ulcallbackdata.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn TranslateColors(hcolortransform: HTRANSFORM, painputcolors: *const COLOR, ncolors: u32, ctinput: COLORTYPE, paoutputcolors: *mut COLOR, ctoutput: COLORTYPE) -> windows_core::BOOL {
    windows_core::link!("mscms.dll" "system" fn TranslateColors(hcolortransform : HTRANSFORM, painputcolors : *const COLOR, ncolors : u32, ctinput : COLORTYPE, paoutputcolors : *mut COLOR, ctoutput : COLORTYPE) -> windows_core::BOOL);
    unsafe { TranslateColors(hcolortransform, painputcolors, ncolors, ctinput, paoutputcolors as _, ctoutput) }
}
#[inline]
pub unsafe fn UninstallColorProfileA<P0, P1>(pmachinename: P0, pprofilename: P1, bdelete: bool) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("mscms.dll" "system" fn UninstallColorProfileA(pmachinename : windows_core::PCSTR, pprofilename : windows_core::PCSTR, bdelete : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { UninstallColorProfileA(pmachinename.param().abi(), pprofilename.param().abi(), bdelete.into()) }
}
#[inline]
pub unsafe fn UninstallColorProfileW<P0, P1>(pmachinename: P0, pprofilename: P1, bdelete: bool) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mscms.dll" "system" fn UninstallColorProfileW(pmachinename : windows_core::PCWSTR, pprofilename : windows_core::PCWSTR, bdelete : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { UninstallColorProfileW(pmachinename.param().abi(), pprofilename.param().abi(), bdelete.into()) }
}
#[inline]
pub unsafe fn UnregisterCMMA<P0>(pmachinename: P0, cmmid: u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("mscms.dll" "system" fn UnregisterCMMA(pmachinename : windows_core::PCSTR, cmmid : u32) -> windows_core::BOOL);
    unsafe { UnregisterCMMA(pmachinename.param().abi(), cmmid) }
}
#[inline]
pub unsafe fn UnregisterCMMW<P0>(pmachinename: P0, cmmid: u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mscms.dll" "system" fn UnregisterCMMW(pmachinename : windows_core::PCWSTR, cmmid : u32) -> windows_core::BOOL);
    unsafe { UnregisterCMMW(pmachinename.param().abi(), cmmid) }
}
#[inline]
pub unsafe fn WcsAssociateColorProfileWithDevice<P1, P2>(scope: WCS_PROFILE_MANAGEMENT_SCOPE, pprofilename: P1, pdevicename: P2) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mscms.dll" "system" fn WcsAssociateColorProfileWithDevice(scope : WCS_PROFILE_MANAGEMENT_SCOPE, pprofilename : windows_core::PCWSTR, pdevicename : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { WcsAssociateColorProfileWithDevice(scope, pprofilename.param().abi(), pdevicename.param().abi()) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn WcsCheckColors(hcolortransform: HTRANSFORM, ninputchannels: u32, cdtinput: COLORDATATYPE, cbinput: u32, pinputdata: *const core::ffi::c_void, paresult: &mut [u8]) -> windows_core::BOOL {
    windows_core::link!("mscms.dll" "system" fn WcsCheckColors(hcolortransform : HTRANSFORM, ncolors : u32, ninputchannels : u32, cdtinput : COLORDATATYPE, cbinput : u32, pinputdata : *const core::ffi::c_void, paresult : *mut u8) -> windows_core::BOOL);
    unsafe { WcsCheckColors(hcolortransform, paresult.len().try_into().unwrap(), ninputchannels, cdtinput, cbinput, pinputdata, core::mem::transmute(paresult.as_ptr())) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn WcsCreateIccProfile(hwcsprofile: HPROFILE, dwoptions: u32) -> HPROFILE {
    windows_core::link!("mscms.dll" "system" fn WcsCreateIccProfile(hwcsprofile : HPROFILE, dwoptions : u32) -> HPROFILE);
    unsafe { WcsCreateIccProfile(hwcsprofile, dwoptions) }
}
#[inline]
pub unsafe fn WcsDisassociateColorProfileFromDevice<P1, P2>(scope: WCS_PROFILE_MANAGEMENT_SCOPE, pprofilename: P1, pdevicename: P2) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mscms.dll" "system" fn WcsDisassociateColorProfileFromDevice(scope : WCS_PROFILE_MANAGEMENT_SCOPE, pprofilename : windows_core::PCWSTR, pdevicename : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { WcsDisassociateColorProfileFromDevice(scope, pprofilename.param().abi(), pdevicename.param().abi()) }
}
#[inline]
pub unsafe fn WcsEnumColorProfiles(scope: WCS_PROFILE_MANAGEMENT_SCOPE, penumrecord: *const ENUMTYPEW, pbuffer: &mut [u8], pnprofiles: Option<*mut u32>) -> windows_core::BOOL {
    windows_core::link!("mscms.dll" "system" fn WcsEnumColorProfiles(scope : WCS_PROFILE_MANAGEMENT_SCOPE, penumrecord : *const ENUMTYPEW, pbuffer : *mut u8, dwsize : u32, pnprofiles : *mut u32) -> windows_core::BOOL);
    unsafe { WcsEnumColorProfiles(scope, penumrecord, core::mem::transmute(pbuffer.as_ptr()), pbuffer.len().try_into().unwrap(), pnprofiles.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn WcsEnumColorProfilesSize(scope: WCS_PROFILE_MANAGEMENT_SCOPE, penumrecord: *const ENUMTYPEW, pdwsize: *mut u32) -> windows_core::BOOL {
    windows_core::link!("mscms.dll" "system" fn WcsEnumColorProfilesSize(scope : WCS_PROFILE_MANAGEMENT_SCOPE, penumrecord : *const ENUMTYPEW, pdwsize : *mut u32) -> windows_core::BOOL);
    unsafe { WcsEnumColorProfilesSize(scope, penumrecord, pdwsize as _) }
}
#[inline]
pub unsafe fn WcsGetCalibrationManagementState(pbisenabled: *mut windows_core::BOOL) -> windows_core::BOOL {
    windows_core::link!("mscms.dll" "system" fn WcsGetCalibrationManagementState(pbisenabled : *mut windows_core::BOOL) -> windows_core::BOOL);
    unsafe { WcsGetCalibrationManagementState(pbisenabled as _) }
}
#[inline]
pub unsafe fn WcsGetDefaultColorProfile<P1>(scope: WCS_PROFILE_MANAGEMENT_SCOPE, pdevicename: P1, cptcolorprofiletype: COLORPROFILETYPE, cpstcolorprofilesubtype: COLORPROFILESUBTYPE, dwprofileid: u32, cbprofilename: u32, pprofilename: windows_core::PWSTR) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mscms.dll" "system" fn WcsGetDefaultColorProfile(scope : WCS_PROFILE_MANAGEMENT_SCOPE, pdevicename : windows_core::PCWSTR, cptcolorprofiletype : COLORPROFILETYPE, cpstcolorprofilesubtype : COLORPROFILESUBTYPE, dwprofileid : u32, cbprofilename : u32, pprofilename : windows_core::PWSTR) -> windows_core::BOOL);
    unsafe { WcsGetDefaultColorProfile(scope, pdevicename.param().abi(), cptcolorprofiletype, cpstcolorprofilesubtype, dwprofileid, cbprofilename, core::mem::transmute(pprofilename)) }
}
#[inline]
pub unsafe fn WcsGetDefaultColorProfileSize<P1>(scope: WCS_PROFILE_MANAGEMENT_SCOPE, pdevicename: P1, cptcolorprofiletype: COLORPROFILETYPE, cpstcolorprofilesubtype: COLORPROFILESUBTYPE, dwprofileid: u32, pcbprofilename: *mut u32) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mscms.dll" "system" fn WcsGetDefaultColorProfileSize(scope : WCS_PROFILE_MANAGEMENT_SCOPE, pdevicename : windows_core::PCWSTR, cptcolorprofiletype : COLORPROFILETYPE, cpstcolorprofilesubtype : COLORPROFILESUBTYPE, dwprofileid : u32, pcbprofilename : *mut u32) -> windows_core::BOOL);
    unsafe { WcsGetDefaultColorProfileSize(scope, pdevicename.param().abi(), cptcolorprofiletype, cpstcolorprofilesubtype, dwprofileid, pcbprofilename as _) }
}
#[inline]
pub unsafe fn WcsGetDefaultRenderingIntent(scope: WCS_PROFILE_MANAGEMENT_SCOPE, pdwrenderingintent: *mut u32) -> windows_core::BOOL {
    windows_core::link!("mscms.dll" "system" fn WcsGetDefaultRenderingIntent(scope : WCS_PROFILE_MANAGEMENT_SCOPE, pdwrenderingintent : *mut u32) -> windows_core::BOOL);
    unsafe { WcsGetDefaultRenderingIntent(scope, pdwrenderingintent as _) }
}
#[inline]
pub unsafe fn WcsGetUsePerUserProfiles<P0>(pdevicename: P0, dwdeviceclass: u32, puseperuserprofiles: *mut windows_core::BOOL) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mscms.dll" "system" fn WcsGetUsePerUserProfiles(pdevicename : windows_core::PCWSTR, dwdeviceclass : u32, puseperuserprofiles : *mut windows_core::BOOL) -> windows_core::BOOL);
    unsafe { WcsGetUsePerUserProfiles(pdevicename.param().abi(), dwdeviceclass, puseperuserprofiles as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn WcsOpenColorProfileA(pcdmpprofile: *const PROFILE, pcampprofile: Option<*const PROFILE>, pgmmpprofile: Option<*const PROFILE>, dwdesireaccess: u32, dwsharemode: u32, dwcreationmode: u32, dwflags: u32) -> HPROFILE {
    windows_core::link!("mscms.dll" "system" fn WcsOpenColorProfileA(pcdmpprofile : *const PROFILE, pcampprofile : *const PROFILE, pgmmpprofile : *const PROFILE, dwdesireaccess : u32, dwsharemode : u32, dwcreationmode : u32, dwflags : u32) -> HPROFILE);
    unsafe { WcsOpenColorProfileA(pcdmpprofile, pcampprofile.unwrap_or(core::mem::zeroed()) as _, pgmmpprofile.unwrap_or(core::mem::zeroed()) as _, dwdesireaccess, dwsharemode, dwcreationmode, dwflags) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn WcsOpenColorProfileW(pcdmpprofile: *const PROFILE, pcampprofile: Option<*const PROFILE>, pgmmpprofile: Option<*const PROFILE>, dwdesireaccess: u32, dwsharemode: u32, dwcreationmode: u32, dwflags: u32) -> HPROFILE {
    windows_core::link!("mscms.dll" "system" fn WcsOpenColorProfileW(pcdmpprofile : *const PROFILE, pcampprofile : *const PROFILE, pgmmpprofile : *const PROFILE, dwdesireaccess : u32, dwsharemode : u32, dwcreationmode : u32, dwflags : u32) -> HPROFILE);
    unsafe { WcsOpenColorProfileW(pcdmpprofile, pcampprofile.unwrap_or(core::mem::zeroed()) as _, pgmmpprofile.unwrap_or(core::mem::zeroed()) as _, dwdesireaccess, dwsharemode, dwcreationmode, dwflags) }
}
#[inline]
pub unsafe fn WcsSetCalibrationManagementState(bisenabled: bool) -> windows_core::BOOL {
    windows_core::link!("mscms.dll" "system" fn WcsSetCalibrationManagementState(bisenabled : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { WcsSetCalibrationManagementState(bisenabled.into()) }
}
#[inline]
pub unsafe fn WcsSetDefaultColorProfile<P1, P5>(scope: WCS_PROFILE_MANAGEMENT_SCOPE, pdevicename: P1, cptcolorprofiletype: COLORPROFILETYPE, cpstcolorprofilesubtype: COLORPROFILESUBTYPE, dwprofileid: u32, pprofilename: P5) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mscms.dll" "system" fn WcsSetDefaultColorProfile(scope : WCS_PROFILE_MANAGEMENT_SCOPE, pdevicename : windows_core::PCWSTR, cptcolorprofiletype : COLORPROFILETYPE, cpstcolorprofilesubtype : COLORPROFILESUBTYPE, dwprofileid : u32, pprofilename : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { WcsSetDefaultColorProfile(scope, pdevicename.param().abi(), cptcolorprofiletype, cpstcolorprofilesubtype, dwprofileid, pprofilename.param().abi()) }
}
#[inline]
pub unsafe fn WcsSetDefaultRenderingIntent(scope: WCS_PROFILE_MANAGEMENT_SCOPE, dwrenderingintent: u32) -> windows_core::BOOL {
    windows_core::link!("mscms.dll" "system" fn WcsSetDefaultRenderingIntent(scope : WCS_PROFILE_MANAGEMENT_SCOPE, dwrenderingintent : u32) -> windows_core::BOOL);
    unsafe { WcsSetDefaultRenderingIntent(scope, dwrenderingintent) }
}
#[inline]
pub unsafe fn WcsSetUsePerUserProfiles<P0>(pdevicename: P0, dwdeviceclass: u32, useperuserprofiles: bool) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mscms.dll" "system" fn WcsSetUsePerUserProfiles(pdevicename : windows_core::PCWSTR, dwdeviceclass : u32, useperuserprofiles : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { WcsSetUsePerUserProfiles(pdevicename.param().abi(), dwdeviceclass, useperuserprofiles.into()) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn WcsTranslateColors(hcolortransform: HTRANSFORM, ncolors: u32, ninputchannels: u32, cdtinput: COLORDATATYPE, cbinput: u32, pinputdata: *const core::ffi::c_void, noutputchannels: u32, cdtoutput: COLORDATATYPE, cboutput: u32, poutputdata: *mut core::ffi::c_void) -> windows_core::BOOL {
    windows_core::link!("mscms.dll" "system" fn WcsTranslateColors(hcolortransform : HTRANSFORM, ncolors : u32, ninputchannels : u32, cdtinput : COLORDATATYPE, cbinput : u32, pinputdata : *const core::ffi::c_void, noutputchannels : u32, cdtoutput : COLORDATATYPE, cboutput : u32, poutputdata : *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { WcsTranslateColors(hcolortransform, ncolors, ninputchannels, cdtinput, cbinput, pinputdata, noutputchannels, cdtoutput, cboutput, poutputdata as _) }
}
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
pub type COLORDATATYPE = i32;
#[repr(C)]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winuser"))]
#[derive(Clone, Copy, Debug, Default)]
pub struct COLORMATCHSETUPA {
    pub dwSize: u32,
    pub dwVersion: u32,
    pub dwFlags: u32,
    pub hwndOwner: super::windef::HWND,
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
    pub lpfnHook: super::winuser::DLGPROC,
    pub lParam: super::minwindef::LPARAM,
    pub lpfnApplyCallback: PCMSCALLBACKA,
    pub lParamApplyCallback: super::minwindef::LPARAM,
}
#[repr(C)]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winuser"))]
#[derive(Clone, Copy, Debug, Default)]
pub struct COLORMATCHSETUPW {
    pub dwSize: u32,
    pub dwVersion: u32,
    pub dwFlags: u32,
    pub hwndOwner: super::windef::HWND,
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
    pub lpfnHook: super::winuser::DLGPROC,
    pub lParam: super::minwindef::LPARAM,
    pub lpfnApplyCallback: PCMSCALLBACKW,
    pub lParamApplyCallback: super::minwindef::LPARAM,
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GENERIC3CHANNEL {
    pub ch1: u16,
    pub ch2: u16,
    pub ch3: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GRAYCOLOR {
    pub gray: u16,
}
#[cfg(feature = "Win32_winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HCMTRANSFORM(pub super::winnt::HANDLE);
#[cfg(feature = "Win32_winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HPROFILE(pub super::winnt::HANDLE);
#[cfg(feature = "Win32_winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HTRANSFORM(pub super::winnt::HANDLE);
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
pub const INDEX_DONT_CARE: u32 = 0;
pub const INTENT_ABSOLUTE_COLORIMETRIC: u32 = 3;
pub const INTENT_PERCEPTUAL: u32 = 0;
pub const INTENT_RELATIVE_COLORIMETRIC: u32 = 1;
pub const INTENT_SATURATION: u32 = 2;
#[cfg(feature = "Win32_minwindef")]
pub type LPBMCALLBACKFN = PBMCALLBACKFN;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPBMFORMAT(pub *mut BMFORMAT);
impl LPBMFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPBMFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPCOLOR(pub *mut COLOR);
impl LPCOLOR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPCOLOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPCOLORDATATYPE(pub *mut COLORDATATYPE);
impl LPCOLORDATATYPE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPCOLORDATATYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPCOLORMATCHSETUPA(pub *mut COLORMATCHSETUPA);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winuser"))]
impl LPCOLORMATCHSETUPA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winuser"))]
impl Default for LPCOLORMATCHSETUPA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPCOLORMATCHSETUPW(pub *mut COLORMATCHSETUPW);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winuser"))]
impl LPCOLORMATCHSETUPW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winuser"))]
impl Default for LPCOLORMATCHSETUPW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPCOLORPROFILESUBTYPE(pub *mut COLORPROFILESUBTYPE);
impl LPCOLORPROFILESUBTYPE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPCOLORPROFILESUBTYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPCOLORPROFILETYPE(pub *mut COLORPROFILETYPE);
impl LPCOLORPROFILETYPE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPCOLORPROFILETYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPCOLORTYPE(pub *mut COLORTYPE);
impl LPCOLORTYPE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPCOLORTYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPCOLOR_NAME(pub *mut COLOR_NAME);
impl LPCOLOR_NAME {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPCOLOR_NAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPDEVCHARACTER(pub *mut core::ffi::c_void);
impl LPDEVCHARACTER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPDEVCHARACTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPENUMTYPEA(pub *mut ENUMTYPEA);
impl LPENUMTYPEA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPENUMTYPEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPENUMTYPEW(pub *mut ENUMTYPEW);
impl LPENUMTYPEW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPENUMTYPEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPNAMED_PROFILE_INFO(pub *mut NAMED_PROFILE_INFO);
impl LPNAMED_PROFILE_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPNAMED_PROFILE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPPROFILE(pub *mut PROFILE);
impl LPPROFILE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPPROFILE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_wingdi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPPROFILEHEADER(pub *mut PROFILEHEADER);
#[cfg(feature = "Win32_wingdi")]
impl LPPROFILEHEADER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_wingdi")]
impl Default for LPPROFILEHEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPTAGTYPE(pub *mut TAGTYPE);
impl LPTAGTYPE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPTAGTYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LabCOLOR {
    pub L: u16,
    pub a: u16,
    pub b: u16,
}
pub const MAX_COLOR_CHANNELS: u32 = 8;
pub const MicrosoftHardwareColorV2: WCS_DEVICE_CAPABILITIES_TYPE = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NAMEDCOLOR {
    pub dwIndex: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[cfg(feature = "Win32_minwindef")]
pub type PBMCALLBACKFN = Option<unsafe extern "system" fn(param0: u32, param1: u32, param2: super::minwindef::LPARAM) -> windows_core::BOOL>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PBMFORMAT(pub *mut BMFORMAT);
impl PBMFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PBMFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winuser"))]
pub type PCMSCALLBACKA = Option<unsafe extern "system" fn(param0: *mut COLORMATCHSETUPA, param1: super::minwindef::LPARAM) -> windows_core::BOOL>;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winuser"))]
pub type PCMSCALLBACKW = Option<unsafe extern "system" fn(param0: *mut COLORMATCHSETUPW, param1: super::minwindef::LPARAM) -> windows_core::BOOL>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCOLOR(pub *mut COLOR);
impl PCOLOR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCOLOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCOLORDATATYPE(pub *mut COLORDATATYPE);
impl PCOLORDATATYPE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCOLORDATATYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCOLORMATCHSETUPA(pub *mut COLORMATCHSETUPA);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winuser"))]
impl PCOLORMATCHSETUPA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winuser"))]
impl Default for PCOLORMATCHSETUPA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCOLORMATCHSETUPW(pub *mut COLORMATCHSETUPW);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winuser"))]
impl PCOLORMATCHSETUPW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winuser"))]
impl Default for PCOLORMATCHSETUPW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCOLORPROFILESUBTYPE(pub *mut COLORPROFILESUBTYPE);
impl PCOLORPROFILESUBTYPE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCOLORPROFILESUBTYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCOLORPROFILETYPE(pub *mut COLORPROFILETYPE);
impl PCOLORPROFILETYPE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCOLORPROFILETYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCOLORTYPE(pub *mut COLORTYPE);
impl PCOLORTYPE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCOLORTYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCOLOR_NAME(pub *mut COLOR_NAME);
impl PCOLOR_NAME {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCOLOR_NAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PENUMTYPEA(pub *mut ENUMTYPEA);
impl PENUMTYPEA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PENUMTYPEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PENUMTYPEW(pub *mut ENUMTYPEW);
impl PENUMTYPEW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PENUMTYPEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PHPROFILE(pub *mut HPROFILE);
#[cfg(feature = "Win32_winnt")]
impl PHPROFILE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_winnt")]
impl Default for PHPROFILE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PNAMED_PROFILE_INFO(pub *mut NAMED_PROFILE_INFO);
impl PNAMED_PROFILE_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PNAMED_PROFILE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPROFILE(pub *mut PROFILE);
impl PPROFILE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPROFILE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_wingdi")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPROFILEHEADER(pub *mut PROFILEHEADER);
#[cfg(feature = "Win32_wingdi")]
impl PPROFILEHEADER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_wingdi")]
impl Default for PPROFILEHEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PRESERVEBLACK: u32 = 1048576;
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
#[cfg(feature = "Win32_wingdi")]
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
    pub phIlluminant: super::wingdi::CIEXYZ,
    pub phCreator: u32,
    pub phReserved: [u8; 44],
}
#[cfg(feature = "Win32_wingdi")]
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PTAGTYPE(pub *mut TAGTYPE);
impl PTAGTYPE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PTAGTYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const RESERVED: u32 = 2147483648;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct TAGTYPE(pub u32);
pub const USE_RELATIVE_COLORIMETRIC: u32 = 131072;
pub const VideoCardGammaTable: WCS_DEVICE_CAPABILITIES_TYPE = 1;
pub const WCS_ALWAYS: u32 = 2097152;
pub const WCS_DEFAULT: u32 = 0;
pub type WCS_DEVICE_CAPABILITIES_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WCS_DEVICE_MHC2_CAPABILITIES {
    pub Size: u32,
    pub SupportsMhc2: windows_core::BOOL,
    pub RegammaLutEntryCount: u32,
    pub CscXyzMatrixRows: u32,
    pub CscXyzMatrixColumns: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WCS_DEVICE_VCGT_CAPABILITIES {
    pub Size: u32,
    pub SupportsVcgt: windows_core::BOOL,
}
pub const WCS_EMBEDDED_TAG_SIGNATURE: u32 = 1297297456;
pub const WCS_EMBEDDED_TAG_TYPE_SIGNATURE: u32 = 1297297712;
pub const WCS_ICCONLY: u32 = 65536;
pub type WCS_PROFILE_MANAGEMENT_SCOPE = i32;
pub const WCS_PROFILE_MANAGEMENT_SCOPE_CURRENT_USER: WCS_PROFILE_MANAGEMENT_SCOPE = 1;
pub const WCS_PROFILE_MANAGEMENT_SCOPE_SYSTEM_WIDE: WCS_PROFILE_MANAGEMENT_SCOPE = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct XYZCOLOR {
    pub X: u16,
    pub Y: u16,
    pub Z: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct YxyCOLOR {
    pub Y: u16,
    pub x: u16,
    pub y: u16,
}
