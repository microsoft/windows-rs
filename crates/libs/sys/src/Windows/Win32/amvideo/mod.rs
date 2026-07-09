pub const AMDDS_ALL: u32 = 255;
pub const AMDDS_DCIPS: u32 = 1;
pub const AMDDS_DEFAULT: u32 = 255;
pub const AMDDS_NONE: u32 = 0;
pub const AMDDS_PRIMARY: u32 = 3;
pub const AMDDS_PS: u32 = 2;
pub const AMDDS_RGB: u32 = 84;
pub const AMDDS_RGBFLP: u32 = 64;
pub const AMDDS_RGBOFF: u32 = 16;
pub const AMDDS_RGBOVR: u32 = 4;
pub const AMDDS_YUV: u32 = 168;
pub const AMDDS_YUVFLP: u32 = 128;
pub const AMDDS_YUVOFF: u32 = 32;
pub const AMDDS_YUVOVR: u32 = 8;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct AM_FRAMESTEP_STEP {
    pub dwFramesToStep: u32,
}
pub type AM_PROPERTY_FRAMESTEP = i32;
pub const AM_PROPERTY_FRAMESTEP_CANCEL: AM_PROPERTY_FRAMESTEP = 2;
pub const AM_PROPERTY_FRAMESTEP_CANSTEP: AM_PROPERTY_FRAMESTEP = 3;
pub const AM_PROPERTY_FRAMESTEP_CANSTEPMULTIPLE: AM_PROPERTY_FRAMESTEP = 4;
pub const AM_PROPERTY_FRAMESTEP_STEP: AM_PROPERTY_FRAMESTEP = 1;
#[repr(C)]
#[cfg(all(feature = "Win32_mediaobj", feature = "Win32_windef"))]
#[derive(Clone, Copy, Default)]
pub struct ANALOGVIDEOINFO {
    pub rcSource: super::windef::RECT,
    pub rcTarget: super::windef::RECT,
    pub dwActiveWidth: u32,
    pub dwActiveHeight: u32,
    pub AvgTimePerFrame: super::mediaobj::REFERENCE_TIME,
}
pub const MAX_SIZE_MPEG1_SEQUENCE_INFO: u32 = 140;
#[repr(C)]
#[cfg(all(feature = "Win32_mediaobj", feature = "Win32_windef", feature = "Win32_wingdi"))]
#[derive(Clone, Copy)]
pub struct MPEG1VIDEOINFO {
    pub hdr: VIDEOINFOHEADER,
    pub dwStartTimeCode: u32,
    pub cbSequenceHeader: u32,
    pub bSequenceHeader: [u8; 1],
}
#[cfg(all(feature = "Win32_mediaobj", feature = "Win32_windef", feature = "Win32_wingdi"))]
impl Default for MPEG1VIDEOINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SIZE_PREHEADER: u32 = 48;
#[repr(C)]
#[cfg(feature = "Win32_wingdi")]
#[derive(Clone, Copy)]
pub struct TRUECOLORINFO {
    pub dwBitMasks: [u32; 3],
    pub bmiColors: [super::wingdi::RGBQUAD; 256],
}
#[cfg(feature = "Win32_wingdi")]
impl Default for TRUECOLORINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_mediaobj", feature = "Win32_windef", feature = "Win32_wingdi"))]
#[derive(Clone, Copy)]
pub struct VIDEOINFO {
    pub rcSource: super::windef::RECT,
    pub rcTarget: super::windef::RECT,
    pub dwBitRate: u32,
    pub dwBitErrorRate: u32,
    pub AvgTimePerFrame: super::mediaobj::REFERENCE_TIME,
    pub bmiHeader: super::wingdi::BITMAPINFOHEADER,
    pub Anonymous: VIDEOINFO_0,
}
#[cfg(all(feature = "Win32_mediaobj", feature = "Win32_windef", feature = "Win32_wingdi"))]
impl Default for VIDEOINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_mediaobj", feature = "Win32_windef", feature = "Win32_wingdi"))]
#[derive(Clone, Copy)]
pub union VIDEOINFO_0 {
    pub bmiColors: [super::wingdi::RGBQUAD; 256],
    pub dwBitMasks: [u32; 3],
    pub TrueColorInfo: TRUECOLORINFO,
}
#[cfg(all(feature = "Win32_mediaobj", feature = "Win32_windef", feature = "Win32_wingdi"))]
impl Default for VIDEOINFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_mediaobj", feature = "Win32_windef", feature = "Win32_wingdi"))]
#[derive(Clone, Copy, Default)]
pub struct VIDEOINFOHEADER {
    pub rcSource: super::windef::RECT,
    pub rcTarget: super::windef::RECT,
    pub dwBitRate: u32,
    pub dwBitErrorRate: u32,
    pub AvgTimePerFrame: super::mediaobj::REFERENCE_TIME,
    pub bmiHeader: super::wingdi::BITMAPINFOHEADER,
}
pub const iBLUE: u32 = 2;
pub const iEGA_COLORS: u32 = 16;
pub const iGREEN: u32 = 1;
pub const iMASK_COLORS: u32 = 3;
pub const iMAXBITS: u32 = 8;
pub const iPALETTE: u32 = 8;
pub const iPALETTE_COLORS: u32 = 256;
pub const iRED: u32 = 0;
pub const iTRUECOLOR: u32 = 16;
