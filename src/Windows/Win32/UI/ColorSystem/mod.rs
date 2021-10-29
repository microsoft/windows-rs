#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
pub const ATTRIB_MATTE: u32 = 2u32;
pub const ATTRIB_TRANSPARENCY: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AssociateColorProfileWithDeviceA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    pmachinename: Param0,
    pprofilename: Param1,
    pdevicename: Param2,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AssociateColorProfileWithDeviceA(
                pmachinename: super::super::Foundation::PSTR,
                pprofilename: super::super::Foundation::PSTR,
                pdevicename: super::super::Foundation::PSTR,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(AssociateColorProfileWithDeviceA(
            pmachinename.into_param().abi(),
            pprofilename.into_param().abi(),
            pdevicename.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AssociateColorProfileWithDeviceW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    pmachinename: Param0,
    pprofilename: Param1,
    pdevicename: Param2,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AssociateColorProfileWithDeviceW(
                pmachinename: super::super::Foundation::PWSTR,
                pprofilename: super::super::Foundation::PWSTR,
                pdevicename: super::super::Foundation::PWSTR,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(AssociateColorProfileWithDeviceW(
            pmachinename.into_param().abi(),
            pprofilename.into_param().abi(),
            pdevicename.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const BEST_MODE: u32 = 3u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct BMFORMAT(pub i32);
pub const BM_x555RGB: BMFORMAT = BMFORMAT(0i32);
pub const BM_x555XYZ: BMFORMAT = BMFORMAT(257i32);
pub const BM_x555Yxy: BMFORMAT = BMFORMAT(258i32);
pub const BM_x555Lab: BMFORMAT = BMFORMAT(259i32);
pub const BM_x555G3CH: BMFORMAT = BMFORMAT(260i32);
pub const BM_RGBTRIPLETS: BMFORMAT = BMFORMAT(2i32);
pub const BM_BGRTRIPLETS: BMFORMAT = BMFORMAT(4i32);
pub const BM_XYZTRIPLETS: BMFORMAT = BMFORMAT(513i32);
pub const BM_YxyTRIPLETS: BMFORMAT = BMFORMAT(514i32);
pub const BM_LabTRIPLETS: BMFORMAT = BMFORMAT(515i32);
pub const BM_G3CHTRIPLETS: BMFORMAT = BMFORMAT(516i32);
pub const BM_5CHANNEL: BMFORMAT = BMFORMAT(517i32);
pub const BM_6CHANNEL: BMFORMAT = BMFORMAT(518i32);
pub const BM_7CHANNEL: BMFORMAT = BMFORMAT(519i32);
pub const BM_8CHANNEL: BMFORMAT = BMFORMAT(520i32);
pub const BM_GRAY: BMFORMAT = BMFORMAT(521i32);
pub const BM_xRGBQUADS: BMFORMAT = BMFORMAT(8i32);
pub const BM_xBGRQUADS: BMFORMAT = BMFORMAT(16i32);
pub const BM_xG3CHQUADS: BMFORMAT = BMFORMAT(772i32);
pub const BM_KYMCQUADS: BMFORMAT = BMFORMAT(773i32);
pub const BM_CMYKQUADS: BMFORMAT = BMFORMAT(32i32);
pub const BM_10b_RGB: BMFORMAT = BMFORMAT(9i32);
pub const BM_10b_XYZ: BMFORMAT = BMFORMAT(1025i32);
pub const BM_10b_Yxy: BMFORMAT = BMFORMAT(1026i32);
pub const BM_10b_Lab: BMFORMAT = BMFORMAT(1027i32);
pub const BM_10b_G3CH: BMFORMAT = BMFORMAT(1028i32);
pub const BM_NAMED_INDEX: BMFORMAT = BMFORMAT(1029i32);
pub const BM_16b_RGB: BMFORMAT = BMFORMAT(10i32);
pub const BM_16b_XYZ: BMFORMAT = BMFORMAT(1281i32);
pub const BM_16b_Yxy: BMFORMAT = BMFORMAT(1282i32);
pub const BM_16b_Lab: BMFORMAT = BMFORMAT(1283i32);
pub const BM_16b_G3CH: BMFORMAT = BMFORMAT(1284i32);
pub const BM_16b_GRAY: BMFORMAT = BMFORMAT(1285i32);
pub const BM_565RGB: BMFORMAT = BMFORMAT(1i32);
pub const BM_32b_scRGB: BMFORMAT = BMFORMAT(1537i32);
pub const BM_32b_scARGB: BMFORMAT = BMFORMAT(1538i32);
pub const BM_S2DOT13FIXED_scRGB: BMFORMAT = BMFORMAT(1539i32);
pub const BM_S2DOT13FIXED_scARGB: BMFORMAT = BMFORMAT(1540i32);
pub const BM_R10G10B10A2: BMFORMAT = BMFORMAT(1793i32);
pub const BM_R10G10B10A2_XR: BMFORMAT = BMFORMAT(1794i32);
pub const BM_R16G16B16A16_FLOAT: BMFORMAT = BMFORMAT(1795i32);
impl ::std::convert::From<i32> for BMFORMAT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for BMFORMAT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct BlackInformation {
    pub fBlackOnly: super::super::Foundation::BOOL,
    pub blackWeight: f32,
}
#[cfg(feature = "Win32_Foundation")]
impl BlackInformation {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for BlackInformation {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for BlackInformation {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("BlackInformation")
            .field("fBlackOnly", &self.fBlackOnly)
            .field("blackWeight", &self.blackWeight)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for BlackInformation {
    fn eq(&self, other: &Self) -> bool {
        self.fBlackOnly == other.fBlackOnly && self.blackWeight == other.blackWeight
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for BlackInformation {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for BlackInformation {
    type Abi = Self;
    type DefaultType = Self;
}
pub const CATID_WcsPlugin: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    2696151776,
    33344,
    16479,
    [138, 22, 138, 91, 77, 242, 240, 221],
);
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct CIEXYZ {
    pub ciexyzX: i32,
    pub ciexyzY: i32,
    pub ciexyzZ: i32,
}
impl CIEXYZ {}
impl ::std::default::Default for CIEXYZ {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CIEXYZ {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CIEXYZ")
            .field("ciexyzX", &self.ciexyzX)
            .field("ciexyzY", &self.ciexyzY)
            .field("ciexyzZ", &self.ciexyzZ)
            .finish()
    }
}
impl ::std::cmp::PartialEq for CIEXYZ {
    fn eq(&self, other: &Self) -> bool {
        self.ciexyzX == other.ciexyzX
            && self.ciexyzY == other.ciexyzY
            && self.ciexyzZ == other.ciexyzZ
    }
}
impl ::std::cmp::Eq for CIEXYZ {}
unsafe impl ::windows::runtime::Abi for CIEXYZ {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct CIEXYZTRIPLE {
    pub ciexyzRed: CIEXYZ,
    pub ciexyzGreen: CIEXYZ,
    pub ciexyzBlue: CIEXYZ,
}
impl CIEXYZTRIPLE {}
impl ::std::default::Default for CIEXYZTRIPLE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CIEXYZTRIPLE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CIEXYZTRIPLE")
            .field("ciexyzRed", &self.ciexyzRed)
            .field("ciexyzGreen", &self.ciexyzGreen)
            .field("ciexyzBlue", &self.ciexyzBlue)
            .finish()
    }
}
impl ::std::cmp::PartialEq for CIEXYZTRIPLE {
    fn eq(&self, other: &Self) -> bool {
        self.ciexyzRed == other.ciexyzRed
            && self.ciexyzGreen == other.ciexyzGreen
            && self.ciexyzBlue == other.ciexyzBlue
    }
}
impl ::std::cmp::Eq for CIEXYZTRIPLE {}
unsafe impl ::windows::runtime::Abi for CIEXYZTRIPLE {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CMCheckColors(
    hcmtransform: isize,
    lpainputcolors: *const COLOR,
    ncolors: u32,
    ctinput: COLORTYPE,
    lparesult: *mut u8,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CMCheckColors(
                hcmtransform: isize,
                lpainputcolors: *const COLOR,
                ncolors: u32,
                ctinput: COLORTYPE,
                lparesult: *mut u8,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CMCheckColors(
            ::std::mem::transmute(hcmtransform),
            ::std::mem::transmute(lpainputcolors),
            ::std::mem::transmute(ncolors),
            ::std::mem::transmute(ctinput),
            ::std::mem::transmute(lparesult),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn CMCheckColorsInGamut(
    hcmtransform: isize,
    lpargbtriple: *const super::super::Graphics::Gdi::RGBTRIPLE,
    lparesult: *mut u8,
    ncount: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CMCheckColorsInGamut(
                hcmtransform: isize,
                lpargbtriple: *const super::super::Graphics::Gdi::RGBTRIPLE,
                lparesult: *mut u8,
                ncount: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CMCheckColorsInGamut(
            ::std::mem::transmute(hcmtransform),
            ::std::mem::transmute(lpargbtriple),
            ::std::mem::transmute(lparesult),
            ::std::mem::transmute(ncount),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CMCheckRGBs<
    'a,
    Param8: ::windows::runtime::IntoParam<'a, super::super::Foundation::LPARAM>,
>(
    hcmtransform: isize,
    lpsrcbits: *const ::std::ffi::c_void,
    bminput: BMFORMAT,
    dwwidth: u32,
    dwheight: u32,
    dwstride: u32,
    lparesult: *mut u8,
    pfncallback: ::std::option::Option<LPBMCALLBACKFN>,
    ulcallbackdata: Param8,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CMCheckRGBs(
                hcmtransform: isize,
                lpsrcbits: *const ::std::ffi::c_void,
                bminput: BMFORMAT,
                dwwidth: u32,
                dwheight: u32,
                dwstride: u32,
                lparesult: *mut u8,
                pfncallback: ::windows::runtime::RawPtr,
                ulcallbackdata: super::super::Foundation::LPARAM,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CMCheckRGBs(
            ::std::mem::transmute(hcmtransform),
            ::std::mem::transmute(lpsrcbits),
            ::std::mem::transmute(bminput),
            ::std::mem::transmute(dwwidth),
            ::std::mem::transmute(dwheight),
            ::std::mem::transmute(dwstride),
            ::std::mem::transmute(lparesult),
            ::std::mem::transmute(pfncallback),
            ulcallbackdata.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CMConvertColorNameToIndex(
    hprofile: isize,
    pacolorname: *const *const i8,
    paindex: *mut u32,
    dwcount: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CMConvertColorNameToIndex(
                hprofile: isize,
                pacolorname: *const *const i8,
                paindex: *mut u32,
                dwcount: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CMConvertColorNameToIndex(
            ::std::mem::transmute(hprofile),
            ::std::mem::transmute(pacolorname),
            ::std::mem::transmute(paindex),
            ::std::mem::transmute(dwcount),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CMConvertIndexToColorName(
    hprofile: isize,
    paindex: *const u32,
    pacolorname: *mut *mut i8,
    dwcount: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CMConvertIndexToColorName(
                hprofile: isize,
                paindex: *const u32,
                pacolorname: *mut *mut i8,
                dwcount: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CMConvertIndexToColorName(
            ::std::mem::transmute(hprofile),
            ::std::mem::transmute(paindex),
            ::std::mem::transmute(pacolorname),
            ::std::mem::transmute(dwcount),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CMCreateDeviceLinkProfile(
    pahprofiles: *const isize,
    nprofiles: u32,
    padwintents: *const u32,
    nintents: u32,
    dwflags: u32,
    lpprofiledata: *mut *mut u8,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CMCreateDeviceLinkProfile(
                pahprofiles: *const isize,
                nprofiles: u32,
                padwintents: *const u32,
                nintents: u32,
                dwflags: u32,
                lpprofiledata: *mut *mut u8,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CMCreateDeviceLinkProfile(
            ::std::mem::transmute(pahprofiles),
            ::std::mem::transmute(nprofiles),
            ::std::mem::transmute(padwintents),
            ::std::mem::transmute(nintents),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(lpprofiledata),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CMCreateMultiProfileTransform(
    pahprofiles: *const isize,
    nprofiles: u32,
    padwintents: *const u32,
    nintents: u32,
    dwflags: u32,
) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CMCreateMultiProfileTransform(
                pahprofiles: *const isize,
                nprofiles: u32,
                padwintents: *const u32,
                nintents: u32,
                dwflags: u32,
            ) -> isize;
        }
        ::std::mem::transmute(CMCreateMultiProfileTransform(
            ::std::mem::transmute(pahprofiles),
            ::std::mem::transmute(nprofiles),
            ::std::mem::transmute(padwintents),
            ::std::mem::transmute(nintents),
            ::std::mem::transmute(dwflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
#[inline]
pub unsafe fn CMCreateProfile(
    lpcolorspace: *mut LOGCOLORSPACEA,
    lpprofiledata: *mut *mut ::std::ffi::c_void,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CMCreateProfile(
                lpcolorspace: *mut LOGCOLORSPACEA,
                lpprofiledata: *mut *mut ::std::ffi::c_void,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CMCreateProfile(
            ::std::mem::transmute(lpcolorspace),
            ::std::mem::transmute(lpprofiledata),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CMCreateProfileW(
    lpcolorspace: *mut LOGCOLORSPACEW,
    lpprofiledata: *mut *mut ::std::ffi::c_void,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CMCreateProfileW(
                lpcolorspace: *mut LOGCOLORSPACEW,
                lpprofiledata: *mut *mut ::std::ffi::c_void,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CMCreateProfileW(
            ::std::mem::transmute(lpcolorspace),
            ::std::mem::transmute(lpprofiledata),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
#[inline]
pub unsafe fn CMCreateTransform(
    lpcolorspace: *const LOGCOLORSPACEA,
    lpdevcharacter: *const ::std::ffi::c_void,
    lptargetdevcharacter: *const ::std::ffi::c_void,
) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CMCreateTransform(
                lpcolorspace: *const LOGCOLORSPACEA,
                lpdevcharacter: *const ::std::ffi::c_void,
                lptargetdevcharacter: *const ::std::ffi::c_void,
            ) -> isize;
        }
        ::std::mem::transmute(CMCreateTransform(
            ::std::mem::transmute(lpcolorspace),
            ::std::mem::transmute(lpdevcharacter),
            ::std::mem::transmute(lptargetdevcharacter),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
#[inline]
pub unsafe fn CMCreateTransformExt(
    lpcolorspace: *const LOGCOLORSPACEA,
    lpdevcharacter: *const ::std::ffi::c_void,
    lptargetdevcharacter: *const ::std::ffi::c_void,
    dwflags: u32,
) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CMCreateTransformExt(
                lpcolorspace: *const LOGCOLORSPACEA,
                lpdevcharacter: *const ::std::ffi::c_void,
                lptargetdevcharacter: *const ::std::ffi::c_void,
                dwflags: u32,
            ) -> isize;
        }
        ::std::mem::transmute(CMCreateTransformExt(
            ::std::mem::transmute(lpcolorspace),
            ::std::mem::transmute(lpdevcharacter),
            ::std::mem::transmute(lptargetdevcharacter),
            ::std::mem::transmute(dwflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CMCreateTransformExtW(
    lpcolorspace: *const LOGCOLORSPACEW,
    lpdevcharacter: *const ::std::ffi::c_void,
    lptargetdevcharacter: *const ::std::ffi::c_void,
    dwflags: u32,
) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CMCreateTransformExtW(
                lpcolorspace: *const LOGCOLORSPACEW,
                lpdevcharacter: *const ::std::ffi::c_void,
                lptargetdevcharacter: *const ::std::ffi::c_void,
                dwflags: u32,
            ) -> isize;
        }
        ::std::mem::transmute(CMCreateTransformExtW(
            ::std::mem::transmute(lpcolorspace),
            ::std::mem::transmute(lpdevcharacter),
            ::std::mem::transmute(lptargetdevcharacter),
            ::std::mem::transmute(dwflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CMCreateTransformW(
    lpcolorspace: *const LOGCOLORSPACEW,
    lpdevcharacter: *const ::std::ffi::c_void,
    lptargetdevcharacter: *const ::std::ffi::c_void,
) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CMCreateTransformW(
                lpcolorspace: *const LOGCOLORSPACEW,
                lpdevcharacter: *const ::std::ffi::c_void,
                lptargetdevcharacter: *const ::std::ffi::c_void,
            ) -> isize;
        }
        ::std::mem::transmute(CMCreateTransformW(
            ::std::mem::transmute(lpcolorspace),
            ::std::mem::transmute(lpdevcharacter),
            ::std::mem::transmute(lptargetdevcharacter),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CMDeleteTransform(hcmtransform: isize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CMDeleteTransform(hcmtransform: isize) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CMDeleteTransform(::std::mem::transmute(hcmtransform)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CMGetInfo(dwinfo: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CMGetInfo(dwinfo: u32) -> u32;
        }
        ::std::mem::transmute(CMGetInfo(::std::mem::transmute(dwinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CMGetNamedProfileInfo(
    hprofile: isize,
    pnamedprofileinfo: *mut NAMED_PROFILE_INFO,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CMGetNamedProfileInfo(
                hprofile: isize,
                pnamedprofileinfo: *mut NAMED_PROFILE_INFO,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CMGetNamedProfileInfo(
            ::std::mem::transmute(hprofile),
            ::std::mem::transmute(pnamedprofileinfo),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CMIsProfileValid(
    hprofile: isize,
    lpbvalid: *mut i32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CMIsProfileValid(
                hprofile: isize,
                lpbvalid: *mut i32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CMIsProfileValid(
            ::std::mem::transmute(hprofile),
            ::std::mem::transmute(lpbvalid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
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
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CMTranslateColors(
    hcmtransform: isize,
    lpainputcolors: *const COLOR,
    ncolors: u32,
    ctinput: COLORTYPE,
    lpaoutputcolors: *mut COLOR,
    ctoutput: COLORTYPE,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CMTranslateColors(
                hcmtransform: isize,
                lpainputcolors: *const COLOR,
                ncolors: u32,
                ctinput: COLORTYPE,
                lpaoutputcolors: *mut COLOR,
                ctoutput: COLORTYPE,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CMTranslateColors(
            ::std::mem::transmute(hcmtransform),
            ::std::mem::transmute(lpainputcolors),
            ::std::mem::transmute(ncolors),
            ::std::mem::transmute(ctinput),
            ::std::mem::transmute(lpaoutputcolors),
            ::std::mem::transmute(ctoutput),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CMTranslateRGB(
    hcmtransform: isize,
    colorref: u32,
    lpcolorref: *mut u32,
    dwflags: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CMTranslateRGB(
                hcmtransform: isize,
                colorref: u32,
                lpcolorref: *mut u32,
                dwflags: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CMTranslateRGB(
            ::std::mem::transmute(hcmtransform),
            ::std::mem::transmute(colorref),
            ::std::mem::transmute(lpcolorref),
            ::std::mem::transmute(dwflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CMTranslateRGBs(
    hcmtransform: isize,
    lpsrcbits: *const ::std::ffi::c_void,
    bminput: BMFORMAT,
    dwwidth: u32,
    dwheight: u32,
    dwstride: u32,
    lpdestbits: *mut ::std::ffi::c_void,
    bmoutput: BMFORMAT,
    dwtranslatedirection: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CMTranslateRGBs(
                hcmtransform: isize,
                lpsrcbits: *const ::std::ffi::c_void,
                bminput: BMFORMAT,
                dwwidth: u32,
                dwheight: u32,
                dwstride: u32,
                lpdestbits: *mut ::std::ffi::c_void,
                bmoutput: BMFORMAT,
                dwtranslatedirection: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CMTranslateRGBs(
            ::std::mem::transmute(hcmtransform),
            ::std::mem::transmute(lpsrcbits),
            ::std::mem::transmute(bminput),
            ::std::mem::transmute(dwwidth),
            ::std::mem::transmute(dwheight),
            ::std::mem::transmute(dwstride),
            ::std::mem::transmute(lpdestbits),
            ::std::mem::transmute(bmoutput),
            ::std::mem::transmute(dwtranslatedirection),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CMTranslateRGBsExt<
    'a,
    Param10: ::windows::runtime::IntoParam<'a, super::super::Foundation::LPARAM>,
>(
    hcmtransform: isize,
    lpsrcbits: *const ::std::ffi::c_void,
    bminput: BMFORMAT,
    dwwidth: u32,
    dwheight: u32,
    dwinputstride: u32,
    lpdestbits: *mut ::std::ffi::c_void,
    bmoutput: BMFORMAT,
    dwoutputstride: u32,
    lpfncallback: ::std::option::Option<LPBMCALLBACKFN>,
    ulcallbackdata: Param10,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CMTranslateRGBsExt(
                hcmtransform: isize,
                lpsrcbits: *const ::std::ffi::c_void,
                bminput: BMFORMAT,
                dwwidth: u32,
                dwheight: u32,
                dwinputstride: u32,
                lpdestbits: *mut ::std::ffi::c_void,
                bmoutput: BMFORMAT,
                dwoutputstride: u32,
                lpfncallback: ::windows::runtime::RawPtr,
                ulcallbackdata: super::super::Foundation::LPARAM,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CMTranslateRGBsExt(
            ::std::mem::transmute(hcmtransform),
            ::std::mem::transmute(lpsrcbits),
            ::std::mem::transmute(bminput),
            ::std::mem::transmute(dwwidth),
            ::std::mem::transmute(dwheight),
            ::std::mem::transmute(dwinputstride),
            ::std::mem::transmute(lpdestbits),
            ::std::mem::transmute(bmoutput),
            ::std::mem::transmute(dwoutputstride),
            ::std::mem::transmute(lpfncallback),
            ulcallbackdata.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct CMYKCOLOR {
    pub cyan: u16,
    pub magenta: u16,
    pub yellow: u16,
    pub black: u16,
}
impl CMYKCOLOR {}
impl ::std::default::Default for CMYKCOLOR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CMYKCOLOR {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CMYKCOLOR")
            .field("cyan", &self.cyan)
            .field("magenta", &self.magenta)
            .field("yellow", &self.yellow)
            .field("black", &self.black)
            .finish()
    }
}
impl ::std::cmp::PartialEq for CMYKCOLOR {
    fn eq(&self, other: &Self) -> bool {
        self.cyan == other.cyan
            && self.magenta == other.magenta
            && self.yellow == other.yellow
            && self.black == other.black
    }
}
impl ::std::cmp::Eq for CMYKCOLOR {}
unsafe impl ::windows::runtime::Abi for CMYKCOLOR {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl COLOR {}
impl ::std::default::Default for COLOR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for COLOR {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for COLOR {}
unsafe impl ::windows::runtime::Abi for COLOR {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct COLOR_0 {
    pub reserved1: u32,
    pub reserved2: *mut ::std::ffi::c_void,
}
impl COLOR_0 {}
impl ::std::default::Default for COLOR_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for COLOR_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct")
            .field("reserved1", &self.reserved1)
            .field("reserved2", &self.reserved2)
            .finish()
    }
}
impl ::std::cmp::PartialEq for COLOR_0 {
    fn eq(&self, other: &Self) -> bool {
        self.reserved1 == other.reserved1 && self.reserved2 == other.reserved2
    }
}
impl ::std::cmp::Eq for COLOR_0 {}
unsafe impl ::windows::runtime::Abi for COLOR_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct COLORDATATYPE(pub i32);
pub const COLOR_BYTE: COLORDATATYPE = COLORDATATYPE(1i32);
pub const COLOR_WORD: COLORDATATYPE = COLORDATATYPE(2i32);
pub const COLOR_FLOAT: COLORDATATYPE = COLORDATATYPE(3i32);
pub const COLOR_S2DOT13FIXED: COLORDATATYPE = COLORDATATYPE(4i32);
pub const COLOR_10b_R10G10B10A2: COLORDATATYPE = COLORDATATYPE(5i32);
pub const COLOR_10b_R10G10B10A2_XR: COLORDATATYPE = COLORDATATYPE(6i32);
pub const COLOR_FLOAT16: COLORDATATYPE = COLORDATATYPE(7i32);
impl ::std::convert::From<i32> for COLORDATATYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for COLORDATATYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct COLORMATCHSETUPA {
    pub dwSize: u32,
    pub dwVersion: u32,
    pub dwFlags: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub pSourceName: super::super::Foundation::PSTR,
    pub pDisplayName: super::super::Foundation::PSTR,
    pub pPrinterName: super::super::Foundation::PSTR,
    pub dwRenderIntent: u32,
    pub dwProofingIntent: u32,
    pub pMonitorProfile: super::super::Foundation::PSTR,
    pub ccMonitorProfile: u32,
    pub pPrinterProfile: super::super::Foundation::PSTR,
    pub ccPrinterProfile: u32,
    pub pTargetProfile: super::super::Foundation::PSTR,
    pub ccTargetProfile: u32,
    pub lpfnHook: ::std::option::Option<super::WindowsAndMessaging::DLGPROC>,
    pub lParam: super::super::Foundation::LPARAM,
    pub lpfnApplyCallback: ::std::option::Option<PCMSCALLBACKA>,
    pub lParamApplyCallback: super::super::Foundation::LPARAM,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl COLORMATCHSETUPA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::default::Default for COLORMATCHSETUPA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::fmt::Debug for COLORMATCHSETUPA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("COLORMATCHSETUPA")
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
impl ::std::cmp::PartialEq for COLORMATCHSETUPA {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
            && self.dwVersion == other.dwVersion
            && self.dwFlags == other.dwFlags
            && self.hwndOwner == other.hwndOwner
            && self.pSourceName == other.pSourceName
            && self.pDisplayName == other.pDisplayName
            && self.pPrinterName == other.pPrinterName
            && self.dwRenderIntent == other.dwRenderIntent
            && self.dwProofingIntent == other.dwProofingIntent
            && self.pMonitorProfile == other.pMonitorProfile
            && self.ccMonitorProfile == other.ccMonitorProfile
            && self.pPrinterProfile == other.pPrinterProfile
            && self.ccPrinterProfile == other.ccPrinterProfile
            && self.pTargetProfile == other.pTargetProfile
            && self.ccTargetProfile == other.ccTargetProfile
            && self.lpfnHook.map(|f| f as usize) == other.lpfnHook.map(|f| f as usize)
            && self.lParam == other.lParam
            && self.lpfnApplyCallback.map(|f| f as usize)
                == other.lpfnApplyCallback.map(|f| f as usize)
            && self.lParamApplyCallback == other.lParamApplyCallback
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::Eq for COLORMATCHSETUPA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::runtime::Abi for COLORMATCHSETUPA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct COLORMATCHSETUPW {
    pub dwSize: u32,
    pub dwVersion: u32,
    pub dwFlags: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub pSourceName: super::super::Foundation::PWSTR,
    pub pDisplayName: super::super::Foundation::PWSTR,
    pub pPrinterName: super::super::Foundation::PWSTR,
    pub dwRenderIntent: u32,
    pub dwProofingIntent: u32,
    pub pMonitorProfile: super::super::Foundation::PWSTR,
    pub ccMonitorProfile: u32,
    pub pPrinterProfile: super::super::Foundation::PWSTR,
    pub ccPrinterProfile: u32,
    pub pTargetProfile: super::super::Foundation::PWSTR,
    pub ccTargetProfile: u32,
    pub lpfnHook: ::std::option::Option<super::WindowsAndMessaging::DLGPROC>,
    pub lParam: super::super::Foundation::LPARAM,
    pub lpfnApplyCallback: ::std::option::Option<PCMSCALLBACKW>,
    pub lParamApplyCallback: super::super::Foundation::LPARAM,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl COLORMATCHSETUPW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::default::Default for COLORMATCHSETUPW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::fmt::Debug for COLORMATCHSETUPW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("COLORMATCHSETUPW")
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
impl ::std::cmp::PartialEq for COLORMATCHSETUPW {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
            && self.dwVersion == other.dwVersion
            && self.dwFlags == other.dwFlags
            && self.hwndOwner == other.hwndOwner
            && self.pSourceName == other.pSourceName
            && self.pDisplayName == other.pDisplayName
            && self.pPrinterName == other.pPrinterName
            && self.dwRenderIntent == other.dwRenderIntent
            && self.dwProofingIntent == other.dwProofingIntent
            && self.pMonitorProfile == other.pMonitorProfile
            && self.ccMonitorProfile == other.ccMonitorProfile
            && self.pPrinterProfile == other.pPrinterProfile
            && self.ccPrinterProfile == other.ccPrinterProfile
            && self.pTargetProfile == other.pTargetProfile
            && self.ccTargetProfile == other.ccTargetProfile
            && self.lpfnHook.map(|f| f as usize) == other.lpfnHook.map(|f| f as usize)
            && self.lParam == other.lParam
            && self.lpfnApplyCallback.map(|f| f as usize)
                == other.lpfnApplyCallback.map(|f| f as usize)
            && self.lParamApplyCallback == other.lParamApplyCallback
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::Eq for COLORMATCHSETUPW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::runtime::Abi for COLORMATCHSETUPW {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct COLORPROFILESUBTYPE(pub i32);
pub const CPST_PERCEPTUAL: COLORPROFILESUBTYPE = COLORPROFILESUBTYPE(0i32);
pub const CPST_RELATIVE_COLORIMETRIC: COLORPROFILESUBTYPE = COLORPROFILESUBTYPE(1i32);
pub const CPST_SATURATION: COLORPROFILESUBTYPE = COLORPROFILESUBTYPE(2i32);
pub const CPST_ABSOLUTE_COLORIMETRIC: COLORPROFILESUBTYPE = COLORPROFILESUBTYPE(3i32);
pub const CPST_NONE: COLORPROFILESUBTYPE = COLORPROFILESUBTYPE(4i32);
pub const CPST_RGB_WORKING_SPACE: COLORPROFILESUBTYPE = COLORPROFILESUBTYPE(5i32);
pub const CPST_CUSTOM_WORKING_SPACE: COLORPROFILESUBTYPE = COLORPROFILESUBTYPE(6i32);
pub const CPST_STANDARD_DISPLAY_COLOR_MODE: COLORPROFILESUBTYPE = COLORPROFILESUBTYPE(7i32);
pub const CPST_EXTENDED_DISPLAY_COLOR_MODE: COLORPROFILESUBTYPE = COLORPROFILESUBTYPE(8i32);
impl ::std::convert::From<i32> for COLORPROFILESUBTYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for COLORPROFILESUBTYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct COLORPROFILETYPE(pub i32);
pub const CPT_ICC: COLORPROFILETYPE = COLORPROFILETYPE(0i32);
pub const CPT_DMP: COLORPROFILETYPE = COLORPROFILETYPE(1i32);
pub const CPT_CAMP: COLORPROFILETYPE = COLORPROFILETYPE(2i32);
pub const CPT_GMMP: COLORPROFILETYPE = COLORPROFILETYPE(3i32);
impl ::std::convert::From<i32> for COLORPROFILETYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for COLORPROFILETYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct COLORTYPE(pub i32);
pub const COLOR_GRAY: COLORTYPE = COLORTYPE(1i32);
pub const COLOR_RGB: COLORTYPE = COLORTYPE(2i32);
pub const COLOR_XYZ: COLORTYPE = COLORTYPE(3i32);
pub const COLOR_Yxy: COLORTYPE = COLORTYPE(4i32);
pub const COLOR_Lab: COLORTYPE = COLORTYPE(5i32);
pub const COLOR_3_CHANNEL: COLORTYPE = COLORTYPE(6i32);
pub const COLOR_CMYK: COLORTYPE = COLORTYPE(7i32);
pub const COLOR_5_CHANNEL: COLORTYPE = COLORTYPE(8i32);
pub const COLOR_6_CHANNEL: COLORTYPE = COLORTYPE(9i32);
pub const COLOR_7_CHANNEL: COLORTYPE = COLORTYPE(10i32);
pub const COLOR_8_CHANNEL: COLORTYPE = COLORTYPE(11i32);
pub const COLOR_NAMED: COLORTYPE = COLORTYPE(12i32);
impl ::std::convert::From<i32> for COLORTYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for COLORTYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct COLOR_MATCH_TO_TARGET_ACTION(pub i32);
pub const CS_ENABLE: COLOR_MATCH_TO_TARGET_ACTION = COLOR_MATCH_TO_TARGET_ACTION(1i32);
pub const CS_DISABLE: COLOR_MATCH_TO_TARGET_ACTION = COLOR_MATCH_TO_TARGET_ACTION(2i32);
pub const CS_DELETE_TRANSFORM: COLOR_MATCH_TO_TARGET_ACTION = COLOR_MATCH_TO_TARGET_ACTION(3i32);
impl ::std::convert::From<i32> for COLOR_MATCH_TO_TARGET_ACTION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for COLOR_MATCH_TO_TARGET_ACTION {
    type Abi = Self;
    type DefaultType = Self;
}
pub const COLOR_MATCH_VERSION: u32 = 512u32;
pub const CSA_A: u32 = 1u32;
pub const CSA_ABC: u32 = 2u32;
pub const CSA_CMYK: u32 = 7u32;
pub const CSA_DEF: u32 = 3u32;
pub const CSA_DEFG: u32 = 4u32;
pub const CSA_GRAY: u32 = 5u32;
pub const CSA_Lab: u32 = 8u32;
pub const CSA_RGB: u32 = 6u32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CheckBitmapBits<
    'a,
    Param8: ::windows::runtime::IntoParam<'a, super::super::Foundation::LPARAM>,
>(
    hcolortransform: isize,
    psrcbits: *const ::std::ffi::c_void,
    bminput: BMFORMAT,
    dwwidth: u32,
    dwheight: u32,
    dwstride: u32,
    paresult: *mut u8,
    pfncallback: ::std::option::Option<LPBMCALLBACKFN>,
    lpcallbackdata: Param8,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CheckBitmapBits(
                hcolortransform: isize,
                psrcbits: *const ::std::ffi::c_void,
                bminput: BMFORMAT,
                dwwidth: u32,
                dwheight: u32,
                dwstride: u32,
                paresult: *mut u8,
                pfncallback: ::windows::runtime::RawPtr,
                lpcallbackdata: super::super::Foundation::LPARAM,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CheckBitmapBits(
            ::std::mem::transmute(hcolortransform),
            ::std::mem::transmute(psrcbits),
            ::std::mem::transmute(bminput),
            ::std::mem::transmute(dwwidth),
            ::std::mem::transmute(dwheight),
            ::std::mem::transmute(dwstride),
            ::std::mem::transmute(paresult),
            ::std::mem::transmute(pfncallback),
            lpcallbackdata.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CheckColors(
    hcolortransform: isize,
    painputcolors: *const COLOR,
    ncolors: u32,
    ctinput: COLORTYPE,
    paresult: *mut u8,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CheckColors(
                hcolortransform: isize,
                painputcolors: *const COLOR,
                ncolors: u32,
                ctinput: COLORTYPE,
                paresult: *mut u8,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CheckColors(
            ::std::mem::transmute(hcolortransform),
            ::std::mem::transmute(painputcolors),
            ::std::mem::transmute(ncolors),
            ::std::mem::transmute(ctinput),
            ::std::mem::transmute(paresult),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn CheckColorsInGamut<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HDC>,
>(
    hdc: Param0,
    lprgbtriple: *const super::super::Graphics::Gdi::RGBTRIPLE,
    dlpbuffer: *mut ::std::ffi::c_void,
    ncount: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CheckColorsInGamut(
                hdc: super::super::Graphics::Gdi::HDC,
                lprgbtriple: *const super::super::Graphics::Gdi::RGBTRIPLE,
                dlpbuffer: *mut ::std::ffi::c_void,
                ncount: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CheckColorsInGamut(
            hdc.into_param().abi(),
            ::std::mem::transmute(lprgbtriple),
            ::std::mem::transmute(dlpbuffer),
            ::std::mem::transmute(ncount),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CloseColorProfile(hprofile: isize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CloseColorProfile(hprofile: isize) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CloseColorProfile(::std::mem::transmute(hprofile)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn ColorCorrectPalette<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HDC>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HPALETTE>,
>(
    hdc: Param0,
    hpal: Param1,
    defirst: u32,
    num: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ColorCorrectPalette(
                hdc: super::super::Graphics::Gdi::HDC,
                hpal: super::super::Graphics::Gdi::HPALETTE,
                defirst: u32,
                num: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ColorCorrectPalette(
            hdc.into_param().abi(),
            hpal.into_param().abi(),
            ::std::mem::transmute(defirst),
            ::std::mem::transmute(num),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn ColorMatchToTarget<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HDC>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HDC>,
>(
    hdc: Param0,
    hdctarget: Param1,
    action: COLOR_MATCH_TO_TARGET_ACTION,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ColorMatchToTarget(
                hdc: super::super::Graphics::Gdi::HDC,
                hdctarget: super::super::Graphics::Gdi::HDC,
                action: COLOR_MATCH_TO_TARGET_ACTION,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ColorMatchToTarget(
            hdc.into_param().abi(),
            hdctarget.into_param().abi(),
            ::std::mem::transmute(action),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn ColorProfileAddDisplayAssociation<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::System::SystemServices::LUID>,
    Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    scope: WCS_PROFILE_MANAGEMENT_SCOPE,
    profilename: Param1,
    targetadapterid: Param2,
    sourceid: u32,
    setasdefault: Param4,
    associateasadvancedcolor: Param5,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ColorProfileAddDisplayAssociation(
                scope: WCS_PROFILE_MANAGEMENT_SCOPE,
                profilename: super::super::Foundation::PWSTR,
                targetadapterid: super::super::System::SystemServices::LUID,
                sourceid: u32,
                setasdefault: super::super::Foundation::BOOL,
                associateasadvancedcolor: super::super::Foundation::BOOL,
            ) -> ::windows::runtime::HRESULT;
        }
        ColorProfileAddDisplayAssociation(
            ::std::mem::transmute(scope),
            profilename.into_param().abi(),
            targetadapterid.into_param().abi(),
            ::std::mem::transmute(sourceid),
            setasdefault.into_param().abi(),
            associateasadvancedcolor.into_param().abi(),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn ColorProfileGetDisplayDefault<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::System::SystemServices::LUID>,
>(
    scope: WCS_PROFILE_MANAGEMENT_SCOPE,
    targetadapterid: Param1,
    sourceid: u32,
    profiletype: COLORPROFILETYPE,
    profilesubtype: COLORPROFILESUBTYPE,
) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ColorProfileGetDisplayDefault(
                scope: WCS_PROFILE_MANAGEMENT_SCOPE,
                targetadapterid: super::super::System::SystemServices::LUID,
                sourceid: u32,
                profiletype: COLORPROFILETYPE,
                profilesubtype: COLORPROFILESUBTYPE,
                profilename: *mut super::super::Foundation::PWSTR,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        ColorProfileGetDisplayDefault(
            ::std::mem::transmute(scope),
            targetadapterid.into_param().abi(),
            ::std::mem::transmute(sourceid),
            ::std::mem::transmute(profiletype),
            ::std::mem::transmute(profilesubtype),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn ColorProfileGetDisplayList<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::System::SystemServices::LUID>,
>(
    scope: WCS_PROFILE_MANAGEMENT_SCOPE,
    targetadapterid: Param1,
    sourceid: u32,
    profilelist: *mut *mut super::super::Foundation::PWSTR,
    profilecount: *mut u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ColorProfileGetDisplayList(
                scope: WCS_PROFILE_MANAGEMENT_SCOPE,
                targetadapterid: super::super::System::SystemServices::LUID,
                sourceid: u32,
                profilelist: *mut *mut super::super::Foundation::PWSTR,
                profilecount: *mut u32,
            ) -> ::windows::runtime::HRESULT;
        }
        ColorProfileGetDisplayList(
            ::std::mem::transmute(scope),
            targetadapterid.into_param().abi(),
            ::std::mem::transmute(sourceid),
            ::std::mem::transmute(profilelist),
            ::std::mem::transmute(profilecount),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn ColorProfileGetDisplayUserScope<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::System::SystemServices::LUID>,
>(
    targetadapterid: Param0,
    sourceid: u32,
) -> ::windows::runtime::Result<WCS_PROFILE_MANAGEMENT_SCOPE> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ColorProfileGetDisplayUserScope(
                targetadapterid: super::super::System::SystemServices::LUID,
                sourceid: u32,
                scope: *mut WCS_PROFILE_MANAGEMENT_SCOPE,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <WCS_PROFILE_MANAGEMENT_SCOPE as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        ColorProfileGetDisplayUserScope(
            targetadapterid.into_param().abi(),
            ::std::mem::transmute(sourceid),
            &mut result__,
        )
        .from_abi::<WCS_PROFILE_MANAGEMENT_SCOPE>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn ColorProfileRemoveDisplayAssociation<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::System::SystemServices::LUID>,
    Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    scope: WCS_PROFILE_MANAGEMENT_SCOPE,
    profilename: Param1,
    targetadapterid: Param2,
    sourceid: u32,
    dissociateadvancedcolor: Param4,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ColorProfileRemoveDisplayAssociation(
                scope: WCS_PROFILE_MANAGEMENT_SCOPE,
                profilename: super::super::Foundation::PWSTR,
                targetadapterid: super::super::System::SystemServices::LUID,
                sourceid: u32,
                dissociateadvancedcolor: super::super::Foundation::BOOL,
            ) -> ::windows::runtime::HRESULT;
        }
        ColorProfileRemoveDisplayAssociation(
            ::std::mem::transmute(scope),
            profilename.into_param().abi(),
            targetadapterid.into_param().abi(),
            ::std::mem::transmute(sourceid),
            dissociateadvancedcolor.into_param().abi(),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn ColorProfileSetDisplayDefaultAssociation<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param4: ::windows::runtime::IntoParam<'a, super::super::System::SystemServices::LUID>,
>(
    scope: WCS_PROFILE_MANAGEMENT_SCOPE,
    profilename: Param1,
    profiletype: COLORPROFILETYPE,
    profilesubtype: COLORPROFILESUBTYPE,
    targetadapterid: Param4,
    sourceid: u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ColorProfileSetDisplayDefaultAssociation(
                scope: WCS_PROFILE_MANAGEMENT_SCOPE,
                profilename: super::super::Foundation::PWSTR,
                profiletype: COLORPROFILETYPE,
                profilesubtype: COLORPROFILESUBTYPE,
                targetadapterid: super::super::System::SystemServices::LUID,
                sourceid: u32,
            ) -> ::windows::runtime::HRESULT;
        }
        ColorProfileSetDisplayDefaultAssociation(
            ::std::mem::transmute(scope),
            profilename.into_param().abi(),
            ::std::mem::transmute(profiletype),
            ::std::mem::transmute(profilesubtype),
            targetadapterid.into_param().abi(),
            ::std::mem::transmute(sourceid),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ConvertColorNameToIndex(
    hprofile: isize,
    pacolorname: *const *const i8,
    paindex: *mut u32,
    dwcount: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ConvertColorNameToIndex(
                hprofile: isize,
                pacolorname: *const *const i8,
                paindex: *mut u32,
                dwcount: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ConvertColorNameToIndex(
            ::std::mem::transmute(hprofile),
            ::std::mem::transmute(pacolorname),
            ::std::mem::transmute(paindex),
            ::std::mem::transmute(dwcount),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ConvertIndexToColorName(
    hprofile: isize,
    paindex: *const u32,
    pacolorname: *mut *mut i8,
    dwcount: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ConvertIndexToColorName(
                hprofile: isize,
                paindex: *const u32,
                pacolorname: *mut *mut i8,
                dwcount: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ConvertIndexToColorName(
            ::std::mem::transmute(hprofile),
            ::std::mem::transmute(paindex),
            ::std::mem::transmute(pacolorname),
            ::std::mem::transmute(dwcount),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
#[inline]
pub unsafe fn CreateColorSpaceA(lplcs: *const LOGCOLORSPACEA) -> HCOLORSPACE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateColorSpaceA(lplcs: *const LOGCOLORSPACEA) -> HCOLORSPACE;
        }
        ::std::mem::transmute(CreateColorSpaceA(::std::mem::transmute(lplcs)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CreateColorSpaceW(lplcs: *const LOGCOLORSPACEW) -> HCOLORSPACE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateColorSpaceW(lplcs: *const LOGCOLORSPACEW) -> HCOLORSPACE;
        }
        ::std::mem::transmute(CreateColorSpaceW(::std::mem::transmute(lplcs)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
#[inline]
pub unsafe fn CreateColorTransformA(
    plogcolorspace: *const LOGCOLORSPACEA,
    hdestprofile: isize,
    htargetprofile: isize,
    dwflags: u32,
) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateColorTransformA(
                plogcolorspace: *const LOGCOLORSPACEA,
                hdestprofile: isize,
                htargetprofile: isize,
                dwflags: u32,
            ) -> isize;
        }
        ::std::mem::transmute(CreateColorTransformA(
            ::std::mem::transmute(plogcolorspace),
            ::std::mem::transmute(hdestprofile),
            ::std::mem::transmute(htargetprofile),
            ::std::mem::transmute(dwflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CreateColorTransformW(
    plogcolorspace: *const LOGCOLORSPACEW,
    hdestprofile: isize,
    htargetprofile: isize,
    dwflags: u32,
) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateColorTransformW(
                plogcolorspace: *const LOGCOLORSPACEW,
                hdestprofile: isize,
                htargetprofile: isize,
                dwflags: u32,
            ) -> isize;
        }
        ::std::mem::transmute(CreateColorTransformW(
            ::std::mem::transmute(plogcolorspace),
            ::std::mem::transmute(hdestprofile),
            ::std::mem::transmute(htargetprofile),
            ::std::mem::transmute(dwflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateDeviceLinkProfile(
    hprofile: *const isize,
    nprofiles: u32,
    padwintent: *const u32,
    nintents: u32,
    dwflags: u32,
    pprofiledata: *mut *mut u8,
    indexpreferredcmm: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateDeviceLinkProfile(
                hprofile: *const isize,
                nprofiles: u32,
                padwintent: *const u32,
                nintents: u32,
                dwflags: u32,
                pprofiledata: *mut *mut u8,
                indexpreferredcmm: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CreateDeviceLinkProfile(
            ::std::mem::transmute(hprofile),
            ::std::mem::transmute(nprofiles),
            ::std::mem::transmute(padwintent),
            ::std::mem::transmute(nintents),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(pprofiledata),
            ::std::mem::transmute(indexpreferredcmm),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CreateMultiProfileTransform(
    pahprofiles: *const isize,
    nprofiles: u32,
    padwintent: *const u32,
    nintents: u32,
    dwflags: u32,
    indexpreferredcmm: u32,
) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateMultiProfileTransform(
                pahprofiles: *const isize,
                nprofiles: u32,
                padwintent: *const u32,
                nintents: u32,
                dwflags: u32,
                indexpreferredcmm: u32,
            ) -> isize;
        }
        ::std::mem::transmute(CreateMultiProfileTransform(
            ::std::mem::transmute(pahprofiles),
            ::std::mem::transmute(nprofiles),
            ::std::mem::transmute(padwintent),
            ::std::mem::transmute(nintents),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(indexpreferredcmm),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
#[inline]
pub unsafe fn CreateProfileFromLogColorSpaceA(
    plogcolorspace: *const LOGCOLORSPACEA,
    pprofile: *mut *mut u8,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateProfileFromLogColorSpaceA(
                plogcolorspace: *const LOGCOLORSPACEA,
                pprofile: *mut *mut u8,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CreateProfileFromLogColorSpaceA(
            ::std::mem::transmute(plogcolorspace),
            ::std::mem::transmute(pprofile),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateProfileFromLogColorSpaceW(
    plogcolorspace: *const LOGCOLORSPACEW,
    pprofile: *mut *mut u8,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateProfileFromLogColorSpaceW(
                plogcolorspace: *const LOGCOLORSPACEW,
                pprofile: *mut *mut u8,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CreateProfileFromLogColorSpaceW(
            ::std::mem::transmute(plogcolorspace),
            ::std::mem::transmute(pprofile),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const DONT_USE_EMBEDDED_WCS_PROFILES: i32 = 1i32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeleteColorSpace<'a, Param0: ::windows::runtime::IntoParam<'a, HCOLORSPACE>>(
    hcs: Param0,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeleteColorSpace(hcs: HCOLORSPACE) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(DeleteColorSpace(hcs.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeleteColorTransform(hxform: isize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeleteColorTransform(hxform: isize) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(DeleteColorTransform(::std::mem::transmute(hxform)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DisassociateColorProfileFromDeviceA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    pmachinename: Param0,
    pprofilename: Param1,
    pdevicename: Param2,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DisassociateColorProfileFromDeviceA(
                pmachinename: super::super::Foundation::PSTR,
                pprofilename: super::super::Foundation::PSTR,
                pdevicename: super::super::Foundation::PSTR,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(DisassociateColorProfileFromDeviceA(
            pmachinename.into_param().abi(),
            pprofilename.into_param().abi(),
            pdevicename.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DisassociateColorProfileFromDeviceW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    pmachinename: Param0,
    pprofilename: Param1,
    pdevicename: Param2,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DisassociateColorProfileFromDeviceW(
                pmachinename: super::super::Foundation::PWSTR,
                pprofilename: super::super::Foundation::PWSTR,
                pdevicename: super::super::Foundation::PWSTR,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(DisassociateColorProfileFromDeviceW(
            pmachinename.into_param().abi(),
            pprofilename.into_param().abi(),
            pdevicename.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const ENABLE_GAMUT_CHECKING: u32 = 65536u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct ENUMTYPEA {
    pub dwSize: u32,
    pub dwVersion: u32,
    pub dwFields: u32,
    pub pDeviceName: super::super::Foundation::PSTR,
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
#[cfg(feature = "Win32_Foundation")]
impl ENUMTYPEA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for ENUMTYPEA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for ENUMTYPEA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ENUMTYPEA")
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
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for ENUMTYPEA {
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
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for ENUMTYPEA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for ENUMTYPEA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct ENUMTYPEW {
    pub dwSize: u32,
    pub dwVersion: u32,
    pub dwFields: u32,
    pub pDeviceName: super::super::Foundation::PWSTR,
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
#[cfg(feature = "Win32_Foundation")]
impl ENUMTYPEW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for ENUMTYPEW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for ENUMTYPEW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ENUMTYPEW")
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
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for ENUMTYPEW {
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
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for ENUMTYPEW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for ENUMTYPEW {
    type Abi = Self;
    type DefaultType = Self;
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
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumColorProfilesA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    pmachinename: Param0,
    penumrecord: *const ENUMTYPEA,
    penumerationbuffer: *mut u8,
    pdwsizeofenumerationbuffer: *mut u32,
    pnprofiles: *mut u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnumColorProfilesA(
                pmachinename: super::super::Foundation::PSTR,
                penumrecord: *const ENUMTYPEA,
                penumerationbuffer: *mut u8,
                pdwsizeofenumerationbuffer: *mut u32,
                pnprofiles: *mut u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(EnumColorProfilesA(
            pmachinename.into_param().abi(),
            ::std::mem::transmute(penumrecord),
            ::std::mem::transmute(penumerationbuffer),
            ::std::mem::transmute(pdwsizeofenumerationbuffer),
            ::std::mem::transmute(pnprofiles),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumColorProfilesW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    pmachinename: Param0,
    penumrecord: *const ENUMTYPEW,
    penumerationbuffer: *mut u8,
    pdwsizeofenumerationbuffer: *mut u32,
    pnprofiles: *mut u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnumColorProfilesW(
                pmachinename: super::super::Foundation::PWSTR,
                penumrecord: *const ENUMTYPEW,
                penumerationbuffer: *mut u8,
                pdwsizeofenumerationbuffer: *mut u32,
                pnprofiles: *mut u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(EnumColorProfilesW(
            pmachinename.into_param().abi(),
            ::std::mem::transmute(penumrecord),
            ::std::mem::transmute(penumerationbuffer),
            ::std::mem::transmute(pdwsizeofenumerationbuffer),
            ::std::mem::transmute(pnprofiles),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn EnumICMProfilesA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HDC>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::LPARAM>,
>(
    hdc: Param0,
    proc: ::std::option::Option<ICMENUMPROCA>,
    param2: Param2,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnumICMProfilesA(
                hdc: super::super::Graphics::Gdi::HDC,
                proc: ::windows::runtime::RawPtr,
                param2: super::super::Foundation::LPARAM,
            ) -> i32;
        }
        ::std::mem::transmute(EnumICMProfilesA(
            hdc.into_param().abi(),
            ::std::mem::transmute(proc),
            param2.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn EnumICMProfilesW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HDC>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::LPARAM>,
>(
    hdc: Param0,
    proc: ::std::option::Option<ICMENUMPROCW>,
    param2: Param2,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnumICMProfilesW(
                hdc: super::super::Graphics::Gdi::HDC,
                proc: ::windows::runtime::RawPtr,
                param2: super::super::Foundation::LPARAM,
            ) -> i32;
        }
        ::std::mem::transmute(EnumICMProfilesW(
            hdc.into_param().abi(),
            ::std::mem::transmute(proc),
            param2.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const FAST_TRANSLATE: u32 = 262144u32;
pub const FLAG_DEPENDENTONDATA: u32 = 2u32;
pub const FLAG_EMBEDDEDPROFILE: u32 = 1u32;
pub const FLAG_ENABLE_CHROMATIC_ADAPTATION: u32 = 33554432u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct GENERIC3CHANNEL {
    pub ch1: u16,
    pub ch2: u16,
    pub ch3: u16,
}
impl GENERIC3CHANNEL {}
impl ::std::default::Default for GENERIC3CHANNEL {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for GENERIC3CHANNEL {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("GENERIC3CHANNEL")
            .field("ch1", &self.ch1)
            .field("ch2", &self.ch2)
            .field("ch3", &self.ch3)
            .finish()
    }
}
impl ::std::cmp::PartialEq for GENERIC3CHANNEL {
    fn eq(&self, other: &Self) -> bool {
        self.ch1 == other.ch1 && self.ch2 == other.ch2 && self.ch3 == other.ch3
    }
}
impl ::std::cmp::Eq for GENERIC3CHANNEL {}
unsafe impl ::windows::runtime::Abi for GENERIC3CHANNEL {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct GRAYCOLOR {
    pub gray: u16,
}
impl GRAYCOLOR {}
impl ::std::default::Default for GRAYCOLOR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for GRAYCOLOR {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("GRAYCOLOR")
            .field("gray", &self.gray)
            .finish()
    }
}
impl ::std::cmp::PartialEq for GRAYCOLOR {
    fn eq(&self, other: &Self) -> bool {
        self.gray == other.gray
    }
}
impl ::std::cmp::Eq for GRAYCOLOR {}
unsafe impl ::windows::runtime::Abi for GRAYCOLOR {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct GamutBoundaryDescription {
    pub pPrimaries: *mut PrimaryJabColors,
    pub cNeutralSamples: u32,
    pub pNeutralSamples: *mut JabColorF,
    pub pReferenceShell: *mut GamutShell,
    pub pPlausibleShell: *mut GamutShell,
    pub pPossibleShell: *mut GamutShell,
}
impl GamutBoundaryDescription {}
impl ::std::default::Default for GamutBoundaryDescription {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for GamutBoundaryDescription {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("GamutBoundaryDescription")
            .field("pPrimaries", &self.pPrimaries)
            .field("cNeutralSamples", &self.cNeutralSamples)
            .field("pNeutralSamples", &self.pNeutralSamples)
            .field("pReferenceShell", &self.pReferenceShell)
            .field("pPlausibleShell", &self.pPlausibleShell)
            .field("pPossibleShell", &self.pPossibleShell)
            .finish()
    }
}
impl ::std::cmp::PartialEq for GamutBoundaryDescription {
    fn eq(&self, other: &Self) -> bool {
        self.pPrimaries == other.pPrimaries
            && self.cNeutralSamples == other.cNeutralSamples
            && self.pNeutralSamples == other.pNeutralSamples
            && self.pReferenceShell == other.pReferenceShell
            && self.pPlausibleShell == other.pPlausibleShell
            && self.pPossibleShell == other.pPossibleShell
    }
}
impl ::std::cmp::Eq for GamutBoundaryDescription {}
unsafe impl ::windows::runtime::Abi for GamutBoundaryDescription {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct GamutShell {
    pub JMin: f32,
    pub JMax: f32,
    pub cVertices: u32,
    pub cTriangles: u32,
    pub pVertices: *mut JabColorF,
    pub pTriangles: *mut GamutShellTriangle,
}
impl GamutShell {}
impl ::std::default::Default for GamutShell {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for GamutShell {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("GamutShell")
            .field("JMin", &self.JMin)
            .field("JMax", &self.JMax)
            .field("cVertices", &self.cVertices)
            .field("cTriangles", &self.cTriangles)
            .field("pVertices", &self.pVertices)
            .field("pTriangles", &self.pTriangles)
            .finish()
    }
}
impl ::std::cmp::PartialEq for GamutShell {
    fn eq(&self, other: &Self) -> bool {
        self.JMin == other.JMin
            && self.JMax == other.JMax
            && self.cVertices == other.cVertices
            && self.cTriangles == other.cTriangles
            && self.pVertices == other.pVertices
            && self.pTriangles == other.pTriangles
    }
}
impl ::std::cmp::Eq for GamutShell {}
unsafe impl ::windows::runtime::Abi for GamutShell {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct GamutShellTriangle {
    pub aVertexIndex: [u32; 3],
}
impl GamutShellTriangle {}
impl ::std::default::Default for GamutShellTriangle {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for GamutShellTriangle {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("GamutShellTriangle")
            .field("aVertexIndex", &self.aVertexIndex)
            .finish()
    }
}
impl ::std::cmp::PartialEq for GamutShellTriangle {
    fn eq(&self, other: &Self) -> bool {
        self.aVertexIndex == other.aVertexIndex
    }
}
impl ::std::cmp::Eq for GamutShellTriangle {}
unsafe impl ::windows::runtime::Abi for GamutShellTriangle {
    type Abi = Self;
    type DefaultType = Self;
}
#[inline]
pub unsafe fn GetCMMInfo(hcolortransform: isize, param1: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetCMMInfo(hcolortransform: isize, param1: u32) -> u32;
        }
        ::std::mem::transmute(GetCMMInfo(
            ::std::mem::transmute(hcolortransform),
            ::std::mem::transmute(param1),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetColorDirectoryA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    pmachinename: Param0,
    pbuffer: super::super::Foundation::PSTR,
    pdwsize: *mut u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetColorDirectoryA(
                pmachinename: super::super::Foundation::PSTR,
                pbuffer: super::super::Foundation::PSTR,
                pdwsize: *mut u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetColorDirectoryA(
            pmachinename.into_param().abi(),
            ::std::mem::transmute(pbuffer),
            ::std::mem::transmute(pdwsize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetColorDirectoryW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    pmachinename: Param0,
    pbuffer: super::super::Foundation::PWSTR,
    pdwsize: *mut u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetColorDirectoryW(
                pmachinename: super::super::Foundation::PWSTR,
                pbuffer: super::super::Foundation::PWSTR,
                pdwsize: *mut u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetColorDirectoryW(
            pmachinename.into_param().abi(),
            ::std::mem::transmute(pbuffer),
            ::std::mem::transmute(pdwsize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetColorProfileElement(
    hprofile: isize,
    tag: u32,
    dwoffset: u32,
    pcbelement: *mut u32,
    pelement: *mut ::std::ffi::c_void,
    pbreference: *mut super::super::Foundation::BOOL,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetColorProfileElement(
                hprofile: isize,
                tag: u32,
                dwoffset: u32,
                pcbelement: *mut u32,
                pelement: *mut ::std::ffi::c_void,
                pbreference: *mut super::super::Foundation::BOOL,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetColorProfileElement(
            ::std::mem::transmute(hprofile),
            ::std::mem::transmute(tag),
            ::std::mem::transmute(dwoffset),
            ::std::mem::transmute(pcbelement),
            ::std::mem::transmute(pelement),
            ::std::mem::transmute(pbreference),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetColorProfileElementTag(
    hprofile: isize,
    dwindex: u32,
    ptag: *mut u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetColorProfileElementTag(
                hprofile: isize,
                dwindex: u32,
                ptag: *mut u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetColorProfileElementTag(
            ::std::mem::transmute(hprofile),
            ::std::mem::transmute(dwindex),
            ::std::mem::transmute(ptag),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetColorProfileFromHandle(
    hprofile: isize,
    pprofile: *mut u8,
    pcbprofile: *mut u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetColorProfileFromHandle(
                hprofile: isize,
                pprofile: *mut u8,
                pcbprofile: *mut u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetColorProfileFromHandle(
            ::std::mem::transmute(hprofile),
            ::std::mem::transmute(pprofile),
            ::std::mem::transmute(pcbprofile),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetColorProfileHeader(
    hprofile: isize,
    pheader: *mut PROFILEHEADER,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetColorProfileHeader(
                hprofile: isize,
                pheader: *mut PROFILEHEADER,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetColorProfileHeader(
            ::std::mem::transmute(hprofile),
            ::std::mem::transmute(pheader),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetColorSpace<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HDC>,
>(
    hdc: Param0,
) -> HCOLORSPACE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetColorSpace(hdc: super::super::Graphics::Gdi::HDC) -> HCOLORSPACE;
        }
        ::std::mem::transmute(GetColorSpace(hdc.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetCountColorProfileElements(
    hprofile: isize,
    pnelementcount: *mut u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetCountColorProfileElements(
                hprofile: isize,
                pnelementcount: *mut u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetCountColorProfileElements(
            ::std::mem::transmute(hprofile),
            ::std::mem::transmute(pnelementcount),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn GetDeviceGammaRamp<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HDC>,
>(
    hdc: Param0,
    lpramp: *mut ::std::ffi::c_void,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetDeviceGammaRamp(
                hdc: super::super::Graphics::Gdi::HDC,
                lpramp: *mut ::std::ffi::c_void,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetDeviceGammaRamp(
            hdc.into_param().abi(),
            ::std::mem::transmute(lpramp),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn GetICMProfileA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HDC>,
>(
    hdc: Param0,
    pbufsize: *mut u32,
    pszfilename: super::super::Foundation::PSTR,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetICMProfileA(
                hdc: super::super::Graphics::Gdi::HDC,
                pbufsize: *mut u32,
                pszfilename: super::super::Foundation::PSTR,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetICMProfileA(
            hdc.into_param().abi(),
            ::std::mem::transmute(pbufsize),
            ::std::mem::transmute(pszfilename),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn GetICMProfileW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HDC>,
>(
    hdc: Param0,
    pbufsize: *mut u32,
    pszfilename: super::super::Foundation::PWSTR,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetICMProfileW(
                hdc: super::super::Graphics::Gdi::HDC,
                pbufsize: *mut u32,
                pszfilename: super::super::Foundation::PWSTR,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetICMProfileW(
            hdc.into_param().abi(),
            ::std::mem::transmute(pbufsize),
            ::std::mem::transmute(pszfilename),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
#[inline]
pub unsafe fn GetLogColorSpaceA<'a, Param0: ::windows::runtime::IntoParam<'a, HCOLORSPACE>>(
    hcolorspace: Param0,
    lpbuffer: *mut LOGCOLORSPACEA,
    nsize: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetLogColorSpaceA(
                hcolorspace: HCOLORSPACE,
                lpbuffer: *mut LOGCOLORSPACEA,
                nsize: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetLogColorSpaceA(
            hcolorspace.into_param().abi(),
            ::std::mem::transmute(lpbuffer),
            ::std::mem::transmute(nsize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetLogColorSpaceW<'a, Param0: ::windows::runtime::IntoParam<'a, HCOLORSPACE>>(
    hcolorspace: Param0,
    lpbuffer: *mut LOGCOLORSPACEW,
    nsize: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetLogColorSpaceW(
                hcolorspace: HCOLORSPACE,
                lpbuffer: *mut LOGCOLORSPACEW,
                nsize: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetLogColorSpaceW(
            hcolorspace.into_param().abi(),
            ::std::mem::transmute(lpbuffer),
            ::std::mem::transmute(nsize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetNamedProfileInfo(
    hprofile: isize,
    pnamedprofileinfo: *mut NAMED_PROFILE_INFO,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetNamedProfileInfo(
                hprofile: isize,
                pnamedprofileinfo: *mut NAMED_PROFILE_INFO,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetNamedProfileInfo(
            ::std::mem::transmute(hprofile),
            ::std::mem::transmute(pnamedprofileinfo),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetPS2ColorRenderingDictionary(
    hprofile: isize,
    dwintent: u32,
    pps2colorrenderingdictionary: *mut u8,
    pcbps2colorrenderingdictionary: *mut u32,
    pbbinary: *mut super::super::Foundation::BOOL,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPS2ColorRenderingDictionary(
                hprofile: isize,
                dwintent: u32,
                pps2colorrenderingdictionary: *mut u8,
                pcbps2colorrenderingdictionary: *mut u32,
                pbbinary: *mut super::super::Foundation::BOOL,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetPS2ColorRenderingDictionary(
            ::std::mem::transmute(hprofile),
            ::std::mem::transmute(dwintent),
            ::std::mem::transmute(pps2colorrenderingdictionary),
            ::std::mem::transmute(pcbps2colorrenderingdictionary),
            ::std::mem::transmute(pbbinary),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetPS2ColorRenderingIntent(
    hprofile: isize,
    dwintent: u32,
    pbuffer: *mut u8,
    pcbps2colorrenderingintent: *mut u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPS2ColorRenderingIntent(
                hprofile: isize,
                dwintent: u32,
                pbuffer: *mut u8,
                pcbps2colorrenderingintent: *mut u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetPS2ColorRenderingIntent(
            ::std::mem::transmute(hprofile),
            ::std::mem::transmute(dwintent),
            ::std::mem::transmute(pbuffer),
            ::std::mem::transmute(pcbps2colorrenderingintent),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetPS2ColorSpaceArray(
    hprofile: isize,
    dwintent: u32,
    dwcsatype: u32,
    pps2colorspacearray: *mut u8,
    pcbps2colorspacearray: *mut u32,
    pbbinary: *mut super::super::Foundation::BOOL,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPS2ColorSpaceArray(
                hprofile: isize,
                dwintent: u32,
                dwcsatype: u32,
                pps2colorspacearray: *mut u8,
                pcbps2colorspacearray: *mut u32,
                pbbinary: *mut super::super::Foundation::BOOL,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetPS2ColorSpaceArray(
            ::std::mem::transmute(hprofile),
            ::std::mem::transmute(dwintent),
            ::std::mem::transmute(dwcsatype),
            ::std::mem::transmute(pps2colorspacearray),
            ::std::mem::transmute(pcbps2colorspacearray),
            ::std::mem::transmute(pbbinary),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetStandardColorSpaceProfileA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    pmachinename: Param0,
    dwscs: u32,
    pbuffer: super::super::Foundation::PSTR,
    pcbsize: *mut u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetStandardColorSpaceProfileA(
                pmachinename: super::super::Foundation::PSTR,
                dwscs: u32,
                pbuffer: super::super::Foundation::PSTR,
                pcbsize: *mut u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetStandardColorSpaceProfileA(
            pmachinename.into_param().abi(),
            ::std::mem::transmute(dwscs),
            ::std::mem::transmute(pbuffer),
            ::std::mem::transmute(pcbsize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetStandardColorSpaceProfileW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    pmachinename: Param0,
    dwscs: u32,
    pbuffer: super::super::Foundation::PWSTR,
    pcbsize: *mut u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetStandardColorSpaceProfileW(
                pmachinename: super::super::Foundation::PWSTR,
                dwscs: u32,
                pbuffer: super::super::Foundation::PWSTR,
                pcbsize: *mut u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetStandardColorSpaceProfileW(
            pmachinename.into_param().abi(),
            ::std::mem::transmute(dwscs),
            ::std::mem::transmute(pbuffer),
            ::std::mem::transmute(pcbsize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(
    :: std :: clone :: Clone,
    :: std :: marker :: Copy,
    :: std :: fmt :: Debug,
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
)]
#[repr(transparent)]
pub struct HCOLORSPACE(pub isize);
impl ::std::default::Default for HCOLORSPACE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for HCOLORSPACE {}
unsafe impl ::windows::runtime::Abi for HCOLORSPACE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct HiFiCOLOR {
    pub channel: [u8; 8],
}
impl HiFiCOLOR {}
impl ::std::default::Default for HiFiCOLOR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for HiFiCOLOR {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HiFiCOLOR")
            .field("channel", &self.channel)
            .finish()
    }
}
impl ::std::cmp::PartialEq for HiFiCOLOR {
    fn eq(&self, other: &Self) -> bool {
        self.channel == other.channel
    }
}
impl ::std::cmp::Eq for HiFiCOLOR {}
unsafe impl ::windows::runtime::Abi for HiFiCOLOR {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
pub type ICMENUMPROCA = unsafe extern "system" fn(
    param0: super::super::Foundation::PSTR,
    param1: super::super::Foundation::LPARAM,
) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type ICMENUMPROCW = unsafe extern "system" fn(
    param0: super::super::Foundation::PWSTR,
    param1: super::super::Foundation::LPARAM,
) -> i32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct ICM_COMMAND(pub u32);
pub const ICM_ADDPROFILE: ICM_COMMAND = ICM_COMMAND(1u32);
pub const ICM_DELETEPROFILE: ICM_COMMAND = ICM_COMMAND(2u32);
pub const ICM_QUERYPROFILE: ICM_COMMAND = ICM_COMMAND(3u32);
pub const ICM_SETDEFAULTPROFILE: ICM_COMMAND = ICM_COMMAND(4u32);
pub const ICM_REGISTERICMATCHER: ICM_COMMAND = ICM_COMMAND(5u32);
pub const ICM_UNREGISTERICMATCHER: ICM_COMMAND = ICM_COMMAND(6u32);
pub const ICM_QUERYMATCH: ICM_COMMAND = ICM_COMMAND(7u32);
impl ::std::convert::From<u32> for ICM_COMMAND {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ICM_COMMAND {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for ICM_COMMAND {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for ICM_COMMAND {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for ICM_COMMAND {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for ICM_COMMAND {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for ICM_COMMAND {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDeviceModelPlugIn(::windows::runtime::IUnknown);
impl IDeviceModelPlugIn {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Initialize<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        bstrxml: Param0,
        cnummodels: u32,
        imodelposition: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            bstrxml.into_param().abi(),
            ::std::mem::transmute(cnummodels),
            ::std::mem::transmute(imodelposition),
        )
        .ok()
    }
    pub unsafe fn GetNumChannels(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    pub unsafe fn DeviceToColorimetricColors(
        &self,
        ccolors: u32,
        cchannels: u32,
        pdevicevalues: *const f32,
        pxyzcolors: *mut XYZColorF,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ccolors),
            ::std::mem::transmute(cchannels),
            ::std::mem::transmute(pdevicevalues),
            ::std::mem::transmute(pxyzcolors),
        )
        .ok()
    }
    pub unsafe fn ColorimetricToDeviceColors(
        &self,
        ccolors: u32,
        cchannels: u32,
        pxyzcolors: *const XYZColorF,
    ) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ccolors),
            ::std::mem::transmute(cchannels),
            ::std::mem::transmute(pxyzcolors),
            &mut result__,
        )
        .from_abi::<f32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ColorimetricToDeviceColorsWithBlack(
        &self,
        ccolors: u32,
        cchannels: u32,
        pxyzcolors: *const XYZColorF,
        pblackinformation: *const BlackInformation,
    ) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ccolors),
            ::std::mem::transmute(cchannels),
            ::std::mem::transmute(pxyzcolors),
            ::std::mem::transmute(pblackinformation),
            &mut result__,
        )
        .from_abi::<f32>(result__)
    }
    pub unsafe fn SetTransformDeviceModelInfo<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, IDeviceModelPlugIn>,
    >(
        &self,
        imodelposition: u32,
        pidevicemodelother: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(imodelposition),
            pidevicemodelother.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn GetPrimarySamples(&self) -> ::windows::runtime::Result<PrimaryXYZColors> {
        let mut result__: <PrimaryXYZColors as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<PrimaryXYZColors>(result__)
    }
    pub unsafe fn GetGamutBoundaryMeshSize(
        &self,
        pnumvertices: *mut u32,
        pnumtriangles: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pnumvertices),
            ::std::mem::transmute(pnumtriangles),
        )
        .ok()
    }
    pub unsafe fn GetGamutBoundaryMesh(
        &self,
        cchannels: u32,
        cvertices: u32,
        ctriangles: u32,
        pvertices: *mut f32,
        ptriangles: *mut GamutShellTriangle,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(cchannels),
            ::std::mem::transmute(cvertices),
            ::std::mem::transmute(ctriangles),
            ::std::mem::transmute(pvertices),
            ::std::mem::transmute(ptriangles),
        )
        .ok()
    }
    pub unsafe fn GetNeutralAxisSize(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    pub unsafe fn GetNeutralAxis(
        &self,
        ccolors: u32,
        pxyzcolors: *mut XYZColorF,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ccolors),
            ::std::mem::transmute(pxyzcolors),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDeviceModelPlugIn {
    type Vtable = IDeviceModelPlugIn_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        483800181,
        1988,
        18174,
        [169, 3, 214, 85, 49, 109, 17, 253],
    );
}
impl ::std::convert::From<IDeviceModelPlugIn> for ::windows::runtime::IUnknown {
    fn from(value: IDeviceModelPlugIn) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDeviceModelPlugIn> for ::windows::runtime::IUnknown {
    fn from(value: &IDeviceModelPlugIn) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDeviceModelPlugIn {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDeviceModelPlugIn {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceModelPlugIn_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bstrxml: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        cnummodels: u32,
        imodelposition: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pnumchannels: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ccolors: u32,
        cchannels: u32,
        pdevicevalues: *const f32,
        pxyzcolors: *mut XYZColorF,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ccolors: u32,
        cchannels: u32,
        pxyzcolors: *const XYZColorF,
        pdevicevalues: *mut f32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ccolors: u32,
        cchannels: u32,
        pxyzcolors: *const XYZColorF,
        pblackinformation: *const BlackInformation,
        pdevicevalues: *mut f32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        imodelposition: u32,
        pidevicemodelother: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pprimarycolor: *mut PrimaryXYZColors,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pnumvertices: *mut u32,
        pnumtriangles: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        cchannels: u32,
        cvertices: u32,
        ctriangles: u32,
        pvertices: *mut f32,
        ptriangles: *mut GamutShellTriangle,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pccolors: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ccolors: u32,
        pxyzcolors: *mut XYZColorF,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IGamutMapModelPlugIn(::windows::runtime::IUnknown);
impl IGamutMapModelPlugIn {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Initialize<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
        Param1: ::windows::runtime::IntoParam<'a, IDeviceModelPlugIn>,
        Param2: ::windows::runtime::IntoParam<'a, IDeviceModelPlugIn>,
    >(
        &self,
        bstrxml: Param0,
        psrcplugin: Param1,
        pdestplugin: Param2,
        psrcgbd: *const GamutBoundaryDescription,
        pdestgbd: *const GamutBoundaryDescription,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            bstrxml.into_param().abi(),
            psrcplugin.into_param().abi(),
            pdestplugin.into_param().abi(),
            ::std::mem::transmute(psrcgbd),
            ::std::mem::transmute(pdestgbd),
        )
        .ok()
    }
    pub unsafe fn SourceToDestinationAppearanceColors(
        &self,
        ccolors: u32,
        pinputcolors: *const JChColorF,
        poutputcolors: *mut JChColorF,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ccolors),
            ::std::mem::transmute(pinputcolors),
            ::std::mem::transmute(poutputcolors),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IGamutMapModelPlugIn {
    type Vtable = IGamutMapModelPlugIn_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        769130773,
        44318,
        16886,
        [162, 25, 164, 244, 181, 131, 209, 249],
    );
}
impl ::std::convert::From<IGamutMapModelPlugIn> for ::windows::runtime::IUnknown {
    fn from(value: IGamutMapModelPlugIn) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IGamutMapModelPlugIn> for ::windows::runtime::IUnknown {
    fn from(value: &IGamutMapModelPlugIn) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IGamutMapModelPlugIn {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IGamutMapModelPlugIn {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGamutMapModelPlugIn_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bstrxml: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        psrcplugin: ::windows::runtime::RawPtr,
        pdestplugin: ::windows::runtime::RawPtr,
        psrcgbd: *const GamutBoundaryDescription,
        pdestgbd: *const GamutBoundaryDescription,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ccolors: u32,
        pinputcolors: *const JChColorF,
        poutputcolors: *mut JChColorF,
    ) -> ::windows::runtime::HRESULT,
);
pub const INDEX_DONT_CARE: u32 = 0u32;
pub const INTENT_ABSOLUTE_COLORIMETRIC: u32 = 3u32;
pub const INTENT_PERCEPTUAL: u32 = 0u32;
pub const INTENT_RELATIVE_COLORIMETRIC: u32 = 1u32;
pub const INTENT_SATURATION: u32 = 2u32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InstallColorProfileA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    pmachinename: Param0,
    pprofilename: Param1,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InstallColorProfileA(
                pmachinename: super::super::Foundation::PSTR,
                pprofilename: super::super::Foundation::PSTR,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(InstallColorProfileA(
            pmachinename.into_param().abi(),
            pprofilename.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InstallColorProfileW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    pmachinename: Param0,
    pprofilename: Param1,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InstallColorProfileW(
                pmachinename: super::super::Foundation::PWSTR,
                pprofilename: super::super::Foundation::PWSTR,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(InstallColorProfileW(
            pmachinename.into_param().abi(),
            pprofilename.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsColorProfileTagPresent(
    hprofile: isize,
    tag: u32,
    pbpresent: *mut super::super::Foundation::BOOL,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsColorProfileTagPresent(
                hprofile: isize,
                tag: u32,
                pbpresent: *mut super::super::Foundation::BOOL,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(IsColorProfileTagPresent(
            ::std::mem::transmute(hprofile),
            ::std::mem::transmute(tag),
            ::std::mem::transmute(pbpresent),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsColorProfileValid(
    hprofile: isize,
    pbvalid: *mut super::super::Foundation::BOOL,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsColorProfileValid(
                hprofile: isize,
                pbvalid: *mut super::super::Foundation::BOOL,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(IsColorProfileValid(
            ::std::mem::transmute(hprofile),
            ::std::mem::transmute(pbvalid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct JChColorF {
    pub J: f32,
    pub C: f32,
    pub h: f32,
}
impl JChColorF {}
impl ::std::default::Default for JChColorF {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for JChColorF {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("JChColorF")
            .field("J", &self.J)
            .field("C", &self.C)
            .field("h", &self.h)
            .finish()
    }
}
impl ::std::cmp::PartialEq for JChColorF {
    fn eq(&self, other: &Self) -> bool {
        self.J == other.J && self.C == other.C && self.h == other.h
    }
}
impl ::std::cmp::Eq for JChColorF {}
unsafe impl ::windows::runtime::Abi for JChColorF {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct JabColorF {
    pub J: f32,
    pub a: f32,
    pub b: f32,
}
impl JabColorF {}
impl ::std::default::Default for JabColorF {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for JabColorF {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("JabColorF")
            .field("J", &self.J)
            .field("a", &self.a)
            .field("b", &self.b)
            .finish()
    }
}
impl ::std::cmp::PartialEq for JabColorF {
    fn eq(&self, other: &Self) -> bool {
        self.J == other.J && self.a == other.a && self.b == other.b
    }
}
impl ::std::cmp::Eq for JabColorF {}
unsafe impl ::windows::runtime::Abi for JabColorF {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_SystemServices")]
pub struct LOGCOLORSPACEA {
    pub lcsSignature: u32,
    pub lcsVersion: u32,
    pub lcsSize: u32,
    pub lcsCSType: i32,
    pub lcsIntent: i32,
    pub lcsEndpoints: CIEXYZTRIPLE,
    pub lcsGammaRed: u32,
    pub lcsGammaGreen: u32,
    pub lcsGammaBlue: u32,
    pub lcsFilename: [super::super::System::SystemServices::CHAR; 260],
}
#[cfg(feature = "Win32_System_SystemServices")]
impl LOGCOLORSPACEA {}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::default::Default for LOGCOLORSPACEA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::fmt::Debug for LOGCOLORSPACEA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("LOGCOLORSPACEA")
            .field("lcsSignature", &self.lcsSignature)
            .field("lcsVersion", &self.lcsVersion)
            .field("lcsSize", &self.lcsSize)
            .field("lcsCSType", &self.lcsCSType)
            .field("lcsIntent", &self.lcsIntent)
            .field("lcsEndpoints", &self.lcsEndpoints)
            .field("lcsGammaRed", &self.lcsGammaRed)
            .field("lcsGammaGreen", &self.lcsGammaGreen)
            .field("lcsGammaBlue", &self.lcsGammaBlue)
            .field("lcsFilename", &self.lcsFilename)
            .finish()
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::PartialEq for LOGCOLORSPACEA {
    fn eq(&self, other: &Self) -> bool {
        self.lcsSignature == other.lcsSignature
            && self.lcsVersion == other.lcsVersion
            && self.lcsSize == other.lcsSize
            && self.lcsCSType == other.lcsCSType
            && self.lcsIntent == other.lcsIntent
            && self.lcsEndpoints == other.lcsEndpoints
            && self.lcsGammaRed == other.lcsGammaRed
            && self.lcsGammaGreen == other.lcsGammaGreen
            && self.lcsGammaBlue == other.lcsGammaBlue
            && self.lcsFilename == other.lcsFilename
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::Eq for LOGCOLORSPACEA {}
#[cfg(feature = "Win32_System_SystemServices")]
unsafe impl ::windows::runtime::Abi for LOGCOLORSPACEA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct LOGCOLORSPACEW {
    pub lcsSignature: u32,
    pub lcsVersion: u32,
    pub lcsSize: u32,
    pub lcsCSType: i32,
    pub lcsIntent: i32,
    pub lcsEndpoints: CIEXYZTRIPLE,
    pub lcsGammaRed: u32,
    pub lcsGammaGreen: u32,
    pub lcsGammaBlue: u32,
    pub lcsFilename: [u16; 260],
}
impl LOGCOLORSPACEW {}
impl ::std::default::Default for LOGCOLORSPACEW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for LOGCOLORSPACEW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("LOGCOLORSPACEW")
            .field("lcsSignature", &self.lcsSignature)
            .field("lcsVersion", &self.lcsVersion)
            .field("lcsSize", &self.lcsSize)
            .field("lcsCSType", &self.lcsCSType)
            .field("lcsIntent", &self.lcsIntent)
            .field("lcsEndpoints", &self.lcsEndpoints)
            .field("lcsGammaRed", &self.lcsGammaRed)
            .field("lcsGammaGreen", &self.lcsGammaGreen)
            .field("lcsGammaBlue", &self.lcsGammaBlue)
            .field("lcsFilename", &self.lcsFilename)
            .finish()
    }
}
impl ::std::cmp::PartialEq for LOGCOLORSPACEW {
    fn eq(&self, other: &Self) -> bool {
        self.lcsSignature == other.lcsSignature
            && self.lcsVersion == other.lcsVersion
            && self.lcsSize == other.lcsSize
            && self.lcsCSType == other.lcsCSType
            && self.lcsIntent == other.lcsIntent
            && self.lcsEndpoints == other.lcsEndpoints
            && self.lcsGammaRed == other.lcsGammaRed
            && self.lcsGammaGreen == other.lcsGammaGreen
            && self.lcsGammaBlue == other.lcsGammaBlue
            && self.lcsFilename == other.lcsFilename
    }
}
impl ::std::cmp::Eq for LOGCOLORSPACEW {}
unsafe impl ::windows::runtime::Abi for LOGCOLORSPACEW {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
pub type LPBMCALLBACKFN = unsafe extern "system" fn(
    param0: u32,
    param1: u32,
    param2: super::super::Foundation::LPARAM,
) -> super::super::Foundation::BOOL;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct LabCOLOR {
    pub L: u16,
    pub a: u16,
    pub b: u16,
}
impl LabCOLOR {}
impl ::std::default::Default for LabCOLOR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for LabCOLOR {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("LabCOLOR")
            .field("L", &self.L)
            .field("a", &self.a)
            .field("b", &self.b)
            .finish()
    }
}
impl ::std::cmp::PartialEq for LabCOLOR {
    fn eq(&self, other: &Self) -> bool {
        self.L == other.L && self.a == other.a && self.b == other.b
    }
}
impl ::std::cmp::Eq for LabCOLOR {}
unsafe impl ::windows::runtime::Abi for LabCOLOR {
    type Abi = Self;
    type DefaultType = Self;
}
pub const MAX_COLOR_CHANNELS: u32 = 8u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct NAMEDCOLOR {
    pub dwIndex: u32,
}
impl NAMEDCOLOR {}
impl ::std::default::Default for NAMEDCOLOR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NAMEDCOLOR {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NAMEDCOLOR")
            .field("dwIndex", &self.dwIndex)
            .finish()
    }
}
impl ::std::cmp::PartialEq for NAMEDCOLOR {
    fn eq(&self, other: &Self) -> bool {
        self.dwIndex == other.dwIndex
    }
}
impl ::std::cmp::Eq for NAMEDCOLOR {}
unsafe impl ::windows::runtime::Abi for NAMEDCOLOR {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct NAMED_PROFILE_INFO {
    pub dwFlags: u32,
    pub dwCount: u32,
    pub dwCountDevCoordinates: u32,
    pub szPrefix: [i8; 32],
    pub szSuffix: [i8; 32],
}
impl NAMED_PROFILE_INFO {}
impl ::std::default::Default for NAMED_PROFILE_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NAMED_PROFILE_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NAMED_PROFILE_INFO")
            .field("dwFlags", &self.dwFlags)
            .field("dwCount", &self.dwCount)
            .field("dwCountDevCoordinates", &self.dwCountDevCoordinates)
            .field("szPrefix", &self.szPrefix)
            .field("szSuffix", &self.szSuffix)
            .finish()
    }
}
impl ::std::cmp::PartialEq for NAMED_PROFILE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags
            && self.dwCount == other.dwCount
            && self.dwCountDevCoordinates == other.dwCountDevCoordinates
            && self.szPrefix == other.szPrefix
            && self.szSuffix == other.szSuffix
    }
}
impl ::std::cmp::Eq for NAMED_PROFILE_INFO {}
unsafe impl ::windows::runtime::Abi for NAMED_PROFILE_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
pub const NORMAL_MODE: u32 = 2u32;
#[inline]
pub unsafe fn OpenColorProfileA(
    pprofile: *const PROFILE,
    dwdesiredaccess: u32,
    dwsharemode: u32,
    dwcreationmode: u32,
) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenColorProfileA(
                pprofile: *const PROFILE,
                dwdesiredaccess: u32,
                dwsharemode: u32,
                dwcreationmode: u32,
            ) -> isize;
        }
        ::std::mem::transmute(OpenColorProfileA(
            ::std::mem::transmute(pprofile),
            ::std::mem::transmute(dwdesiredaccess),
            ::std::mem::transmute(dwsharemode),
            ::std::mem::transmute(dwcreationmode),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn OpenColorProfileW(
    pprofile: *const PROFILE,
    dwdesiredaccess: u32,
    dwsharemode: u32,
    dwcreationmode: u32,
) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenColorProfileW(
                pprofile: *const PROFILE,
                dwdesiredaccess: u32,
                dwsharemode: u32,
                dwcreationmode: u32,
            ) -> isize;
        }
        ::std::mem::transmute(OpenColorProfileW(
            ::std::mem::transmute(pprofile),
            ::std::mem::transmute(dwdesiredaccess),
            ::std::mem::transmute(dwsharemode),
            ::std::mem::transmute(dwcreationmode),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub type PCMSCALLBACKA = unsafe extern "system" fn(
    param0: *mut ::std::mem::ManuallyDrop<COLORMATCHSETUPA>,
    param1: super::super::Foundation::LPARAM,
) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub type PCMSCALLBACKW = unsafe extern "system" fn(
    param0: *mut ::std::mem::ManuallyDrop<COLORMATCHSETUPW>,
    param1: super::super::Foundation::LPARAM,
) -> super::super::Foundation::BOOL;
pub const PRESERVEBLACK: u32 = 1048576u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct PROFILE {
    pub dwType: u32,
    pub pProfileData: *mut ::std::ffi::c_void,
    pub cbDataSize: u32,
}
impl PROFILE {}
impl ::std::default::Default for PROFILE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PROFILE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PROFILE")
            .field("dwType", &self.dwType)
            .field("pProfileData", &self.pProfileData)
            .field("cbDataSize", &self.cbDataSize)
            .finish()
    }
}
impl ::std::cmp::PartialEq for PROFILE {
    fn eq(&self, other: &Self) -> bool {
        self.dwType == other.dwType
            && self.pProfileData == other.pProfileData
            && self.cbDataSize == other.cbDataSize
    }
}
impl ::std::cmp::Eq for PROFILE {}
unsafe impl ::windows::runtime::Abi for PROFILE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
    pub phIlluminant: CIEXYZ,
    pub phCreator: u32,
    pub phReserved: [u8; 44],
}
impl PROFILEHEADER {}
impl ::std::default::Default for PROFILEHEADER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PROFILEHEADER {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PROFILEHEADER")
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
impl ::std::cmp::PartialEq for PROFILEHEADER {
    fn eq(&self, other: &Self) -> bool {
        self.phSize == other.phSize
            && self.phCMMType == other.phCMMType
            && self.phVersion == other.phVersion
            && self.phClass == other.phClass
            && self.phDataColorSpace == other.phDataColorSpace
            && self.phConnectionSpace == other.phConnectionSpace
            && self.phDateTime == other.phDateTime
            && self.phSignature == other.phSignature
            && self.phPlatform == other.phPlatform
            && self.phProfileFlags == other.phProfileFlags
            && self.phManufacturer == other.phManufacturer
            && self.phModel == other.phModel
            && self.phAttributes == other.phAttributes
            && self.phRenderingIntent == other.phRenderingIntent
            && self.phIlluminant == other.phIlluminant
            && self.phCreator == other.phCreator
            && self.phReserved == other.phReserved
    }
}
impl ::std::cmp::Eq for PROFILEHEADER {}
unsafe impl ::windows::runtime::Abi for PROFILEHEADER {
    type Abi = Self;
    type DefaultType = Self;
}
pub const PROFILE_FILENAME: u32 = 1u32;
pub const PROFILE_MEMBUFFER: u32 = 2u32;
pub const PROFILE_READ: u32 = 1u32;
pub const PROFILE_READWRITE: u32 = 2u32;
pub const PROOF_MODE: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl PrimaryJabColors {}
impl ::std::default::Default for PrimaryJabColors {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PrimaryJabColors {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PrimaryJabColors")
            .field("red", &self.red)
            .field("yellow", &self.yellow)
            .field("green", &self.green)
            .field("cyan", &self.cyan)
            .field("blue", &self.blue)
            .field("magenta", &self.magenta)
            .field("black", &self.black)
            .field("white", &self.white)
            .finish()
    }
}
impl ::std::cmp::PartialEq for PrimaryJabColors {
    fn eq(&self, other: &Self) -> bool {
        self.red == other.red
            && self.yellow == other.yellow
            && self.green == other.green
            && self.cyan == other.cyan
            && self.blue == other.blue
            && self.magenta == other.magenta
            && self.black == other.black
            && self.white == other.white
    }
}
impl ::std::cmp::Eq for PrimaryJabColors {}
unsafe impl ::windows::runtime::Abi for PrimaryJabColors {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl PrimaryXYZColors {}
impl ::std::default::Default for PrimaryXYZColors {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PrimaryXYZColors {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PrimaryXYZColors")
            .field("red", &self.red)
            .field("yellow", &self.yellow)
            .field("green", &self.green)
            .field("cyan", &self.cyan)
            .field("blue", &self.blue)
            .field("magenta", &self.magenta)
            .field("black", &self.black)
            .field("white", &self.white)
            .finish()
    }
}
impl ::std::cmp::PartialEq for PrimaryXYZColors {
    fn eq(&self, other: &Self) -> bool {
        self.red == other.red
            && self.yellow == other.yellow
            && self.green == other.green
            && self.cyan == other.cyan
            && self.blue == other.blue
            && self.magenta == other.magenta
            && self.black == other.black
            && self.white == other.white
    }
}
impl ::std::cmp::Eq for PrimaryXYZColors {}
unsafe impl ::windows::runtime::Abi for PrimaryXYZColors {
    type Abi = Self;
    type DefaultType = Self;
}
pub const RESERVED: u32 = 2147483648u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct RGBCOLOR {
    pub red: u16,
    pub green: u16,
    pub blue: u16,
}
impl RGBCOLOR {}
impl ::std::default::Default for RGBCOLOR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RGBCOLOR {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RGBCOLOR")
            .field("red", &self.red)
            .field("green", &self.green)
            .field("blue", &self.blue)
            .finish()
    }
}
impl ::std::cmp::PartialEq for RGBCOLOR {
    fn eq(&self, other: &Self) -> bool {
        self.red == other.red && self.green == other.green && self.blue == other.blue
    }
}
impl ::std::cmp::Eq for RGBCOLOR {}
unsafe impl ::windows::runtime::Abi for RGBCOLOR {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegisterCMMA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    pmachinename: Param0,
    cmmid: u32,
    pcmmdll: Param2,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegisterCMMA(
                pmachinename: super::super::Foundation::PSTR,
                cmmid: u32,
                pcmmdll: super::super::Foundation::PSTR,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(RegisterCMMA(
            pmachinename.into_param().abi(),
            ::std::mem::transmute(cmmid),
            pcmmdll.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegisterCMMW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    pmachinename: Param0,
    cmmid: u32,
    pcmmdll: Param2,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegisterCMMW(
                pmachinename: super::super::Foundation::PWSTR,
                cmmid: u32,
                pcmmdll: super::super::Foundation::PWSTR,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(RegisterCMMW(
            pmachinename.into_param().abi(),
            ::std::mem::transmute(cmmid),
            pcmmdll.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const SEQUENTIAL_TRANSFORM: u32 = 2155872256u32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SelectCMM(dwcmmtype: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SelectCMM(dwcmmtype: u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SelectCMM(::std::mem::transmute(dwcmmtype)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetColorProfileElement(
    hprofile: isize,
    tag: u32,
    dwoffset: u32,
    pcbelement: *const u32,
    pelement: *const ::std::ffi::c_void,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetColorProfileElement(
                hprofile: isize,
                tag: u32,
                dwoffset: u32,
                pcbelement: *const u32,
                pelement: *const ::std::ffi::c_void,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetColorProfileElement(
            ::std::mem::transmute(hprofile),
            ::std::mem::transmute(tag),
            ::std::mem::transmute(dwoffset),
            ::std::mem::transmute(pcbelement),
            ::std::mem::transmute(pelement),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetColorProfileElementReference(
    hprofile: isize,
    newtag: u32,
    reftag: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetColorProfileElementReference(
                hprofile: isize,
                newtag: u32,
                reftag: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetColorProfileElementReference(
            ::std::mem::transmute(hprofile),
            ::std::mem::transmute(newtag),
            ::std::mem::transmute(reftag),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetColorProfileElementSize(
    hprofile: isize,
    tagtype: u32,
    pcbelement: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetColorProfileElementSize(
                hprofile: isize,
                tagtype: u32,
                pcbelement: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetColorProfileElementSize(
            ::std::mem::transmute(hprofile),
            ::std::mem::transmute(tagtype),
            ::std::mem::transmute(pcbelement),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetColorProfileHeader(
    hprofile: isize,
    pheader: *const PROFILEHEADER,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetColorProfileHeader(
                hprofile: isize,
                pheader: *const PROFILEHEADER,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetColorProfileHeader(
            ::std::mem::transmute(hprofile),
            ::std::mem::transmute(pheader),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn SetColorSpace<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HDC>,
    Param1: ::windows::runtime::IntoParam<'a, HCOLORSPACE>,
>(
    hdc: Param0,
    hcs: Param1,
) -> HCOLORSPACE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetColorSpace(
                hdc: super::super::Graphics::Gdi::HDC,
                hcs: HCOLORSPACE,
            ) -> HCOLORSPACE;
        }
        ::std::mem::transmute(SetColorSpace(
            hdc.into_param().abi(),
            hcs.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn SetDeviceGammaRamp<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HDC>,
>(
    hdc: Param0,
    lpramp: *const ::std::ffi::c_void,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetDeviceGammaRamp(
                hdc: super::super::Graphics::Gdi::HDC,
                lpramp: *const ::std::ffi::c_void,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetDeviceGammaRamp(
            hdc.into_param().abi(),
            ::std::mem::transmute(lpramp),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn SetICMMode<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HDC>,
>(
    hdc: Param0,
    mode: i32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetICMMode(hdc: super::super::Graphics::Gdi::HDC, mode: i32) -> i32;
        }
        ::std::mem::transmute(SetICMMode(
            hdc.into_param().abi(),
            ::std::mem::transmute(mode),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn SetICMProfileA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HDC>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hdc: Param0,
    lpfilename: Param1,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetICMProfileA(
                hdc: super::super::Graphics::Gdi::HDC,
                lpfilename: super::super::Foundation::PSTR,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetICMProfileA(
            hdc.into_param().abi(),
            lpfilename.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn SetICMProfileW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HDC>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hdc: Param0,
    lpfilename: Param1,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetICMProfileW(
                hdc: super::super::Graphics::Gdi::HDC,
                lpfilename: super::super::Foundation::PWSTR,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetICMProfileW(
            hdc.into_param().abi(),
            lpfilename.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetStandardColorSpaceProfileA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    pmachinename: Param0,
    dwprofileid: u32,
    pprofilename: Param2,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetStandardColorSpaceProfileA(
                pmachinename: super::super::Foundation::PSTR,
                dwprofileid: u32,
                pprofilename: super::super::Foundation::PSTR,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetStandardColorSpaceProfileA(
            pmachinename.into_param().abi(),
            ::std::mem::transmute(dwprofileid),
            pprofilename.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetStandardColorSpaceProfileW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    pmachinename: Param0,
    dwprofileid: u32,
    pprofilename: Param2,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetStandardColorSpaceProfileW(
                pmachinename: super::super::Foundation::PWSTR,
                dwprofileid: u32,
                pprofilename: super::super::Foundation::PWSTR,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetStandardColorSpaceProfileW(
            pmachinename.into_param().abi(),
            ::std::mem::transmute(dwprofileid),
            pprofilename.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn SetupColorMatchingA(pcms: *mut COLORMATCHSETUPA) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupColorMatchingA(
                pcms: *mut ::std::mem::ManuallyDrop<COLORMATCHSETUPA>,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetupColorMatchingA(::std::mem::transmute(pcms)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn SetupColorMatchingW(pcms: *mut COLORMATCHSETUPW) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupColorMatchingW(
                pcms: *mut ::std::mem::ManuallyDrop<COLORMATCHSETUPW>,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetupColorMatchingW(::std::mem::transmute(pcms)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TranslateBitmapBits<
    'a,
    Param10: ::windows::runtime::IntoParam<'a, super::super::Foundation::LPARAM>,
>(
    hcolortransform: isize,
    psrcbits: *const ::std::ffi::c_void,
    bminput: BMFORMAT,
    dwwidth: u32,
    dwheight: u32,
    dwinputstride: u32,
    pdestbits: *mut ::std::ffi::c_void,
    bmoutput: BMFORMAT,
    dwoutputstride: u32,
    pfncallback: ::std::option::Option<LPBMCALLBACKFN>,
    ulcallbackdata: Param10,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TranslateBitmapBits(
                hcolortransform: isize,
                psrcbits: *const ::std::ffi::c_void,
                bminput: BMFORMAT,
                dwwidth: u32,
                dwheight: u32,
                dwinputstride: u32,
                pdestbits: *mut ::std::ffi::c_void,
                bmoutput: BMFORMAT,
                dwoutputstride: u32,
                pfncallback: ::windows::runtime::RawPtr,
                ulcallbackdata: super::super::Foundation::LPARAM,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(TranslateBitmapBits(
            ::std::mem::transmute(hcolortransform),
            ::std::mem::transmute(psrcbits),
            ::std::mem::transmute(bminput),
            ::std::mem::transmute(dwwidth),
            ::std::mem::transmute(dwheight),
            ::std::mem::transmute(dwinputstride),
            ::std::mem::transmute(pdestbits),
            ::std::mem::transmute(bmoutput),
            ::std::mem::transmute(dwoutputstride),
            ::std::mem::transmute(pfncallback),
            ulcallbackdata.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TranslateColors(
    hcolortransform: isize,
    painputcolors: *const COLOR,
    ncolors: u32,
    ctinput: COLORTYPE,
    paoutputcolors: *mut COLOR,
    ctoutput: COLORTYPE,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TranslateColors(
                hcolortransform: isize,
                painputcolors: *const COLOR,
                ncolors: u32,
                ctinput: COLORTYPE,
                paoutputcolors: *mut COLOR,
                ctoutput: COLORTYPE,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(TranslateColors(
            ::std::mem::transmute(hcolortransform),
            ::std::mem::transmute(painputcolors),
            ::std::mem::transmute(ncolors),
            ::std::mem::transmute(ctinput),
            ::std::mem::transmute(paoutputcolors),
            ::std::mem::transmute(ctoutput),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const USE_RELATIVE_COLORIMETRIC: u32 = 131072u32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UninstallColorProfileA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    pmachinename: Param0,
    pprofilename: Param1,
    bdelete: Param2,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UninstallColorProfileA(
                pmachinename: super::super::Foundation::PSTR,
                pprofilename: super::super::Foundation::PSTR,
                bdelete: super::super::Foundation::BOOL,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(UninstallColorProfileA(
            pmachinename.into_param().abi(),
            pprofilename.into_param().abi(),
            bdelete.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UninstallColorProfileW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    pmachinename: Param0,
    pprofilename: Param1,
    bdelete: Param2,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UninstallColorProfileW(
                pmachinename: super::super::Foundation::PWSTR,
                pprofilename: super::super::Foundation::PWSTR,
                bdelete: super::super::Foundation::BOOL,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(UninstallColorProfileW(
            pmachinename.into_param().abi(),
            pprofilename.into_param().abi(),
            bdelete.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UnregisterCMMA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    pmachinename: Param0,
    cmmid: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UnregisterCMMA(
                pmachinename: super::super::Foundation::PSTR,
                cmmid: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(UnregisterCMMA(
            pmachinename.into_param().abi(),
            ::std::mem::transmute(cmmid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UnregisterCMMW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    pmachinename: Param0,
    cmmid: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UnregisterCMMW(
                pmachinename: super::super::Foundation::PWSTR,
                cmmid: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(UnregisterCMMW(
            pmachinename.into_param().abi(),
            ::std::mem::transmute(cmmid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UpdateICMRegKeyA<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    reserved: u32,
    lpszcmid: Param1,
    lpszfilename: Param2,
    command: ICM_COMMAND,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UpdateICMRegKeyA(
                reserved: u32,
                lpszcmid: super::super::Foundation::PSTR,
                lpszfilename: super::super::Foundation::PSTR,
                command: ICM_COMMAND,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(UpdateICMRegKeyA(
            ::std::mem::transmute(reserved),
            lpszcmid.into_param().abi(),
            lpszfilename.into_param().abi(),
            ::std::mem::transmute(command),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UpdateICMRegKeyW<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    reserved: u32,
    lpszcmid: Param1,
    lpszfilename: Param2,
    command: ICM_COMMAND,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UpdateICMRegKeyW(
                reserved: u32,
                lpszcmid: super::super::Foundation::PWSTR,
                lpszfilename: super::super::Foundation::PWSTR,
                command: ICM_COMMAND,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(UpdateICMRegKeyW(
            ::std::mem::transmute(reserved),
            lpszcmid.into_param().abi(),
            lpszfilename.into_param().abi(),
            ::std::mem::transmute(command),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const WCS_ALWAYS: u32 = 2097152u32;
pub const WCS_DEFAULT: i32 = 0i32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct WCS_DEVICE_CAPABILITIES_TYPE(pub i32);
pub const VideoCardGammaTable: WCS_DEVICE_CAPABILITIES_TYPE = WCS_DEVICE_CAPABILITIES_TYPE(1i32);
pub const MicrosoftHardwareColorV2: WCS_DEVICE_CAPABILITIES_TYPE =
    WCS_DEVICE_CAPABILITIES_TYPE(2i32);
impl ::std::convert::From<i32> for WCS_DEVICE_CAPABILITIES_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WCS_DEVICE_CAPABILITIES_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WCS_DEVICE_MHC2_CAPABILITIES {
    pub Size: u32,
    pub SupportsMhc2: super::super::Foundation::BOOL,
    pub RegammaLutEntryCount: u32,
    pub CscXyzMatrixRows: u32,
    pub CscXyzMatrixColumns: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl WCS_DEVICE_MHC2_CAPABILITIES {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WCS_DEVICE_MHC2_CAPABILITIES {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WCS_DEVICE_MHC2_CAPABILITIES {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WCS_DEVICE_MHC2_CAPABILITIES")
            .field("Size", &self.Size)
            .field("SupportsMhc2", &self.SupportsMhc2)
            .field("RegammaLutEntryCount", &self.RegammaLutEntryCount)
            .field("CscXyzMatrixRows", &self.CscXyzMatrixRows)
            .field("CscXyzMatrixColumns", &self.CscXyzMatrixColumns)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WCS_DEVICE_MHC2_CAPABILITIES {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size
            && self.SupportsMhc2 == other.SupportsMhc2
            && self.RegammaLutEntryCount == other.RegammaLutEntryCount
            && self.CscXyzMatrixRows == other.CscXyzMatrixRows
            && self.CscXyzMatrixColumns == other.CscXyzMatrixColumns
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WCS_DEVICE_MHC2_CAPABILITIES {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WCS_DEVICE_MHC2_CAPABILITIES {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WCS_DEVICE_VCGT_CAPABILITIES {
    pub Size: u32,
    pub SupportsVcgt: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl WCS_DEVICE_VCGT_CAPABILITIES {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WCS_DEVICE_VCGT_CAPABILITIES {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WCS_DEVICE_VCGT_CAPABILITIES {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WCS_DEVICE_VCGT_CAPABILITIES")
            .field("Size", &self.Size)
            .field("SupportsVcgt", &self.SupportsVcgt)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WCS_DEVICE_VCGT_CAPABILITIES {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.SupportsVcgt == other.SupportsVcgt
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WCS_DEVICE_VCGT_CAPABILITIES {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WCS_DEVICE_VCGT_CAPABILITIES {
    type Abi = Self;
    type DefaultType = Self;
}
pub const WCS_ICCONLY: i32 = 65536i32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct WCS_PROFILE_MANAGEMENT_SCOPE(pub i32);
pub const WCS_PROFILE_MANAGEMENT_SCOPE_SYSTEM_WIDE: WCS_PROFILE_MANAGEMENT_SCOPE =
    WCS_PROFILE_MANAGEMENT_SCOPE(0i32);
pub const WCS_PROFILE_MANAGEMENT_SCOPE_CURRENT_USER: WCS_PROFILE_MANAGEMENT_SCOPE =
    WCS_PROFILE_MANAGEMENT_SCOPE(1i32);
impl ::std::convert::From<i32> for WCS_PROFILE_MANAGEMENT_SCOPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WCS_PROFILE_MANAGEMENT_SCOPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WcsAssociateColorProfileWithDevice<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    scope: WCS_PROFILE_MANAGEMENT_SCOPE,
    pprofilename: Param1,
    pdevicename: Param2,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WcsAssociateColorProfileWithDevice(
                scope: WCS_PROFILE_MANAGEMENT_SCOPE,
                pprofilename: super::super::Foundation::PWSTR,
                pdevicename: super::super::Foundation::PWSTR,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(WcsAssociateColorProfileWithDevice(
            ::std::mem::transmute(scope),
            pprofilename.into_param().abi(),
            pdevicename.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WcsCheckColors(
    hcolortransform: isize,
    ncolors: u32,
    ninputchannels: u32,
    cdtinput: COLORDATATYPE,
    cbinput: u32,
    pinputdata: *const ::std::ffi::c_void,
    paresult: *mut u8,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WcsCheckColors(
                hcolortransform: isize,
                ncolors: u32,
                ninputchannels: u32,
                cdtinput: COLORDATATYPE,
                cbinput: u32,
                pinputdata: *const ::std::ffi::c_void,
                paresult: *mut u8,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(WcsCheckColors(
            ::std::mem::transmute(hcolortransform),
            ::std::mem::transmute(ncolors),
            ::std::mem::transmute(ninputchannels),
            ::std::mem::transmute(cdtinput),
            ::std::mem::transmute(cbinput),
            ::std::mem::transmute(pinputdata),
            ::std::mem::transmute(paresult),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WcsCreateIccProfile(hwcsprofile: isize, dwoptions: u32) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WcsCreateIccProfile(hwcsprofile: isize, dwoptions: u32) -> isize;
        }
        ::std::mem::transmute(WcsCreateIccProfile(
            ::std::mem::transmute(hwcsprofile),
            ::std::mem::transmute(dwoptions),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WcsDisassociateColorProfileFromDevice<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    scope: WCS_PROFILE_MANAGEMENT_SCOPE,
    pprofilename: Param1,
    pdevicename: Param2,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WcsDisassociateColorProfileFromDevice(
                scope: WCS_PROFILE_MANAGEMENT_SCOPE,
                pprofilename: super::super::Foundation::PWSTR,
                pdevicename: super::super::Foundation::PWSTR,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(WcsDisassociateColorProfileFromDevice(
            ::std::mem::transmute(scope),
            pprofilename.into_param().abi(),
            pdevicename.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WcsEnumColorProfiles(
    scope: WCS_PROFILE_MANAGEMENT_SCOPE,
    penumrecord: *const ENUMTYPEW,
    pbuffer: *mut u8,
    dwsize: u32,
    pnprofiles: *mut u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WcsEnumColorProfiles(
                scope: WCS_PROFILE_MANAGEMENT_SCOPE,
                penumrecord: *const ENUMTYPEW,
                pbuffer: *mut u8,
                dwsize: u32,
                pnprofiles: *mut u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(WcsEnumColorProfiles(
            ::std::mem::transmute(scope),
            ::std::mem::transmute(penumrecord),
            ::std::mem::transmute(pbuffer),
            ::std::mem::transmute(dwsize),
            ::std::mem::transmute(pnprofiles),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WcsEnumColorProfilesSize(
    scope: WCS_PROFILE_MANAGEMENT_SCOPE,
    penumrecord: *const ENUMTYPEW,
    pdwsize: *mut u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WcsEnumColorProfilesSize(
                scope: WCS_PROFILE_MANAGEMENT_SCOPE,
                penumrecord: *const ENUMTYPEW,
                pdwsize: *mut u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(WcsEnumColorProfilesSize(
            ::std::mem::transmute(scope),
            ::std::mem::transmute(penumrecord),
            ::std::mem::transmute(pdwsize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WcsGetCalibrationManagementState(
    pbisenabled: *mut super::super::Foundation::BOOL,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WcsGetCalibrationManagementState(
                pbisenabled: *mut super::super::Foundation::BOOL,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(WcsGetCalibrationManagementState(::std::mem::transmute(
            pbisenabled,
        )))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WcsGetDefaultColorProfile<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    scope: WCS_PROFILE_MANAGEMENT_SCOPE,
    pdevicename: Param1,
    cptcolorprofiletype: COLORPROFILETYPE,
    cpstcolorprofilesubtype: COLORPROFILESUBTYPE,
    dwprofileid: u32,
    cbprofilename: u32,
    pprofilename: super::super::Foundation::PWSTR,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WcsGetDefaultColorProfile(
                scope: WCS_PROFILE_MANAGEMENT_SCOPE,
                pdevicename: super::super::Foundation::PWSTR,
                cptcolorprofiletype: COLORPROFILETYPE,
                cpstcolorprofilesubtype: COLORPROFILESUBTYPE,
                dwprofileid: u32,
                cbprofilename: u32,
                pprofilename: super::super::Foundation::PWSTR,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(WcsGetDefaultColorProfile(
            ::std::mem::transmute(scope),
            pdevicename.into_param().abi(),
            ::std::mem::transmute(cptcolorprofiletype),
            ::std::mem::transmute(cpstcolorprofilesubtype),
            ::std::mem::transmute(dwprofileid),
            ::std::mem::transmute(cbprofilename),
            ::std::mem::transmute(pprofilename),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WcsGetDefaultColorProfileSize<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    scope: WCS_PROFILE_MANAGEMENT_SCOPE,
    pdevicename: Param1,
    cptcolorprofiletype: COLORPROFILETYPE,
    cpstcolorprofilesubtype: COLORPROFILESUBTYPE,
    dwprofileid: u32,
    pcbprofilename: *mut u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WcsGetDefaultColorProfileSize(
                scope: WCS_PROFILE_MANAGEMENT_SCOPE,
                pdevicename: super::super::Foundation::PWSTR,
                cptcolorprofiletype: COLORPROFILETYPE,
                cpstcolorprofilesubtype: COLORPROFILESUBTYPE,
                dwprofileid: u32,
                pcbprofilename: *mut u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(WcsGetDefaultColorProfileSize(
            ::std::mem::transmute(scope),
            pdevicename.into_param().abi(),
            ::std::mem::transmute(cptcolorprofiletype),
            ::std::mem::transmute(cpstcolorprofilesubtype),
            ::std::mem::transmute(dwprofileid),
            ::std::mem::transmute(pcbprofilename),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WcsGetDefaultRenderingIntent(
    scope: WCS_PROFILE_MANAGEMENT_SCOPE,
    pdwrenderingintent: *mut u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WcsGetDefaultRenderingIntent(
                scope: WCS_PROFILE_MANAGEMENT_SCOPE,
                pdwrenderingintent: *mut u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(WcsGetDefaultRenderingIntent(
            ::std::mem::transmute(scope),
            ::std::mem::transmute(pdwrenderingintent),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WcsGetUsePerUserProfiles<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    pdevicename: Param0,
    dwdeviceclass: u32,
    puseperuserprofiles: *mut super::super::Foundation::BOOL,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WcsGetUsePerUserProfiles(
                pdevicename: super::super::Foundation::PWSTR,
                dwdeviceclass: u32,
                puseperuserprofiles: *mut super::super::Foundation::BOOL,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(WcsGetUsePerUserProfiles(
            pdevicename.into_param().abi(),
            ::std::mem::transmute(dwdeviceclass),
            ::std::mem::transmute(puseperuserprofiles),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WcsOpenColorProfileA(
    pcdmpprofile: *const PROFILE,
    pcampprofile: *const PROFILE,
    pgmmpprofile: *const PROFILE,
    dwdesireaccess: u32,
    dwsharemode: u32,
    dwcreationmode: u32,
    dwflags: u32,
) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WcsOpenColorProfileA(
                pcdmpprofile: *const PROFILE,
                pcampprofile: *const PROFILE,
                pgmmpprofile: *const PROFILE,
                dwdesireaccess: u32,
                dwsharemode: u32,
                dwcreationmode: u32,
                dwflags: u32,
            ) -> isize;
        }
        ::std::mem::transmute(WcsOpenColorProfileA(
            ::std::mem::transmute(pcdmpprofile),
            ::std::mem::transmute(pcampprofile),
            ::std::mem::transmute(pgmmpprofile),
            ::std::mem::transmute(dwdesireaccess),
            ::std::mem::transmute(dwsharemode),
            ::std::mem::transmute(dwcreationmode),
            ::std::mem::transmute(dwflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WcsOpenColorProfileW(
    pcdmpprofile: *const PROFILE,
    pcampprofile: *const PROFILE,
    pgmmpprofile: *const PROFILE,
    dwdesireaccess: u32,
    dwsharemode: u32,
    dwcreationmode: u32,
    dwflags: u32,
) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WcsOpenColorProfileW(
                pcdmpprofile: *const PROFILE,
                pcampprofile: *const PROFILE,
                pgmmpprofile: *const PROFILE,
                dwdesireaccess: u32,
                dwsharemode: u32,
                dwcreationmode: u32,
                dwflags: u32,
            ) -> isize;
        }
        ::std::mem::transmute(WcsOpenColorProfileW(
            ::std::mem::transmute(pcdmpprofile),
            ::std::mem::transmute(pcampprofile),
            ::std::mem::transmute(pgmmpprofile),
            ::std::mem::transmute(dwdesireaccess),
            ::std::mem::transmute(dwsharemode),
            ::std::mem::transmute(dwcreationmode),
            ::std::mem::transmute(dwflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WcsSetCalibrationManagementState<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    bisenabled: Param0,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WcsSetCalibrationManagementState(
                bisenabled: super::super::Foundation::BOOL,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(WcsSetCalibrationManagementState(
            bisenabled.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WcsSetDefaultColorProfile<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    scope: WCS_PROFILE_MANAGEMENT_SCOPE,
    pdevicename: Param1,
    cptcolorprofiletype: COLORPROFILETYPE,
    cpstcolorprofilesubtype: COLORPROFILESUBTYPE,
    dwprofileid: u32,
    pprofilename: Param5,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WcsSetDefaultColorProfile(
                scope: WCS_PROFILE_MANAGEMENT_SCOPE,
                pdevicename: super::super::Foundation::PWSTR,
                cptcolorprofiletype: COLORPROFILETYPE,
                cpstcolorprofilesubtype: COLORPROFILESUBTYPE,
                dwprofileid: u32,
                pprofilename: super::super::Foundation::PWSTR,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(WcsSetDefaultColorProfile(
            ::std::mem::transmute(scope),
            pdevicename.into_param().abi(),
            ::std::mem::transmute(cptcolorprofiletype),
            ::std::mem::transmute(cpstcolorprofilesubtype),
            ::std::mem::transmute(dwprofileid),
            pprofilename.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WcsSetDefaultRenderingIntent(
    scope: WCS_PROFILE_MANAGEMENT_SCOPE,
    dwrenderingintent: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WcsSetDefaultRenderingIntent(
                scope: WCS_PROFILE_MANAGEMENT_SCOPE,
                dwrenderingintent: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(WcsSetDefaultRenderingIntent(
            ::std::mem::transmute(scope),
            ::std::mem::transmute(dwrenderingintent),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WcsSetUsePerUserProfiles<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    pdevicename: Param0,
    dwdeviceclass: u32,
    useperuserprofiles: Param2,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WcsSetUsePerUserProfiles(
                pdevicename: super::super::Foundation::PWSTR,
                dwdeviceclass: u32,
                useperuserprofiles: super::super::Foundation::BOOL,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(WcsSetUsePerUserProfiles(
            pdevicename.into_param().abi(),
            ::std::mem::transmute(dwdeviceclass),
            useperuserprofiles.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WcsTranslateColors(
    hcolortransform: isize,
    ncolors: u32,
    ninputchannels: u32,
    cdtinput: COLORDATATYPE,
    cbinput: u32,
    pinputdata: *const ::std::ffi::c_void,
    noutputchannels: u32,
    cdtoutput: COLORDATATYPE,
    cboutput: u32,
    poutputdata: *mut ::std::ffi::c_void,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WcsTranslateColors(
                hcolortransform: isize,
                ncolors: u32,
                ninputchannels: u32,
                cdtinput: COLORDATATYPE,
                cbinput: u32,
                pinputdata: *const ::std::ffi::c_void,
                noutputchannels: u32,
                cdtoutput: COLORDATATYPE,
                cboutput: u32,
                poutputdata: *mut ::std::ffi::c_void,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(WcsTranslateColors(
            ::std::mem::transmute(hcolortransform),
            ::std::mem::transmute(ncolors),
            ::std::mem::transmute(ninputchannels),
            ::std::mem::transmute(cdtinput),
            ::std::mem::transmute(cbinput),
            ::std::mem::transmute(pinputdata),
            ::std::mem::transmute(noutputchannels),
            ::std::mem::transmute(cdtoutput),
            ::std::mem::transmute(cboutput),
            ::std::mem::transmute(poutputdata),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct XYZCOLOR {
    pub X: u16,
    pub Y: u16,
    pub Z: u16,
}
impl XYZCOLOR {}
impl ::std::default::Default for XYZCOLOR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for XYZCOLOR {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("XYZCOLOR")
            .field("X", &self.X)
            .field("Y", &self.Y)
            .field("Z", &self.Z)
            .finish()
    }
}
impl ::std::cmp::PartialEq for XYZCOLOR {
    fn eq(&self, other: &Self) -> bool {
        self.X == other.X && self.Y == other.Y && self.Z == other.Z
    }
}
impl ::std::cmp::Eq for XYZCOLOR {}
unsafe impl ::windows::runtime::Abi for XYZCOLOR {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct XYZColorF {
    pub X: f32,
    pub Y: f32,
    pub Z: f32,
}
impl XYZColorF {}
impl ::std::default::Default for XYZColorF {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for XYZColorF {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("XYZColorF")
            .field("X", &self.X)
            .field("Y", &self.Y)
            .field("Z", &self.Z)
            .finish()
    }
}
impl ::std::cmp::PartialEq for XYZColorF {
    fn eq(&self, other: &Self) -> bool {
        self.X == other.X && self.Y == other.Y && self.Z == other.Z
    }
}
impl ::std::cmp::Eq for XYZColorF {}
unsafe impl ::windows::runtime::Abi for XYZColorF {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct YxyCOLOR {
    pub Y: u16,
    pub x: u16,
    pub y: u16,
}
impl YxyCOLOR {}
impl ::std::default::Default for YxyCOLOR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for YxyCOLOR {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("YxyCOLOR")
            .field("Y", &self.Y)
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}
impl ::std::cmp::PartialEq for YxyCOLOR {
    fn eq(&self, other: &Self) -> bool {
        self.Y == other.Y && self.x == other.x && self.y == other.y
    }
}
impl ::std::cmp::Eq for YxyCOLOR {}
unsafe impl ::windows::runtime::Abi for YxyCOLOR {
    type Abi = Self;
    type DefaultType = Self;
}
