pub const AMDDS_ALL: i32 = 255;
pub const AMDDS_DCIPS: i32 = 1;
pub const AMDDS_DEFAULT: i32 = 255;
pub const AMDDS_NONE: i32 = 0;
pub const AMDDS_PRIMARY: i32 = 3;
pub const AMDDS_PS: i32 = 2;
pub const AMDDS_RGB: i32 = 84;
pub const AMDDS_RGBFLP: i32 = 64;
pub const AMDDS_RGBOFF: i32 = 16;
pub const AMDDS_RGBOVR: i32 = 4;
pub const AMDDS_YUV: i32 = 168;
pub const AMDDS_YUVFLP: i32 = 128;
pub const AMDDS_YUVOFF: i32 = 32;
pub const AMDDS_YUVOVR: i32 = 8;
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
#[cfg(all(feature = "mediaobj", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct ANALOGVIDEOINFO {
    pub rcSource: super::RECT,
    pub rcTarget: super::RECT,
    pub dwActiveWidth: u32,
    pub dwActiveHeight: u32,
    pub AvgTimePerFrame: super::REFERENCE_TIME,
}
pub const MAX_SIZE_MPEG1_SEQUENCE_INFO: i32 = 140;
#[repr(C)]
#[cfg(all(feature = "mediaobj", feature = "windef", feature = "wingdi"))]
#[derive(Clone, Copy)]
pub struct MPEG1VIDEOINFO {
    pub hdr: VIDEOINFOHEADER,
    pub dwStartTimeCode: u32,
    pub cbSequenceHeader: u32,
    pub bSequenceHeader: [u8; 1],
}
#[cfg(all(feature = "mediaobj", feature = "windef", feature = "wingdi"))]
impl Default for MPEG1VIDEOINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SIZE_PREHEADER: i32 = 48;
#[repr(C)]
#[cfg(feature = "wingdi")]
#[derive(Clone, Copy)]
pub struct TRUECOLORINFO {
    pub dwBitMasks: [u32; 3],
    pub bmiColors: [super::RGBQUAD; 256],
}
#[cfg(feature = "wingdi")]
impl Default for TRUECOLORINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "mediaobj", feature = "windef", feature = "wingdi"))]
#[derive(Clone, Copy)]
pub struct VIDEOINFO {
    pub rcSource: super::RECT,
    pub rcTarget: super::RECT,
    pub dwBitRate: u32,
    pub dwBitErrorRate: u32,
    pub AvgTimePerFrame: super::REFERENCE_TIME,
    pub bmiHeader: super::BITMAPINFOHEADER,
    pub Anonymous: VIDEOINFO_0,
}
#[cfg(all(feature = "mediaobj", feature = "windef", feature = "wingdi"))]
impl Default for VIDEOINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "mediaobj", feature = "windef", feature = "wingdi"))]
#[derive(Clone, Copy)]
pub union VIDEOINFO_0 {
    pub bmiColors: [super::RGBQUAD; 256],
    pub dwBitMasks: [u32; 3],
    pub TrueColorInfo: TRUECOLORINFO,
}
#[cfg(all(feature = "mediaobj", feature = "windef", feature = "wingdi"))]
impl Default for VIDEOINFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "mediaobj", feature = "windef", feature = "wingdi"))]
#[derive(Clone, Copy, Default)]
pub struct VIDEOINFOHEADER {
    pub rcSource: super::RECT,
    pub rcTarget: super::RECT,
    pub dwBitRate: u32,
    pub dwBitErrorRate: u32,
    pub AvgTimePerFrame: super::REFERENCE_TIME,
    pub bmiHeader: super::BITMAPINFOHEADER,
}
pub const iBLUE: i32 = 2;
pub const iEGA_COLORS: i32 = 16;
pub const iGREEN: i32 = 1;
pub const iMASK_COLORS: i32 = 3;
pub const iMAXBITS: i32 = 8;
pub const iPALETTE: i32 = 8;
pub const iPALETTE_COLORS: i32 = 256;
pub const iRED: i32 = 0;
pub const iTRUECOLOR: i32 = 16;
