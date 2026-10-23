#[cfg(feature = "shared")]
pub type LPVIDEOPARAMETERS = *mut VIDEOPARAMETERS;
#[cfg(feature = "shared")]
pub type PVIDEOPARAMETERS = *mut VIDEOPARAMETERS;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct VIDEOPARAMETERS {
    pub Guid: windows_sys::core::GUID,
    pub dwOffset: super::ULONG,
    pub dwCommand: super::ULONG,
    pub dwFlags: super::ULONG,
    pub dwMode: super::ULONG,
    pub dwTVStandard: super::ULONG,
    pub dwAvailableModes: super::ULONG,
    pub dwAvailableTVStandard: super::ULONG,
    pub dwFlickerFilter: super::ULONG,
    pub dwOverScanX: super::ULONG,
    pub dwOverScanY: super::ULONG,
    pub dwMaxUnscaledX: super::ULONG,
    pub dwMaxUnscaledY: super::ULONG,
    pub dwPositionX: super::ULONG,
    pub dwPositionY: super::ULONG,
    pub dwBrightness: super::ULONG,
    pub dwContrast: super::ULONG,
    pub dwCPType: super::ULONG,
    pub dwCPCommand: super::ULONG,
    pub dwCPStandard: super::ULONG,
    pub dwCPKey: super::ULONG,
    pub bCP_APSTriggerBits: super::ULONG,
    pub bOEMCopyProtection: [super::UCHAR; 256],
}
#[cfg(feature = "shared")]
impl Default for VIDEOPARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const VP_COMMAND_GET: i32 = 1;
pub const VP_COMMAND_SET: i32 = 2;
pub const VP_CP_CMD_ACTIVATE: i32 = 1;
pub const VP_CP_CMD_CHANGE: i32 = 4;
pub const VP_CP_CMD_DEACTIVATE: i32 = 2;
pub const VP_CP_TYPE_APS_TRIGGER: i32 = 1;
pub const VP_CP_TYPE_MACROVISION: i32 = 2;
pub const VP_FLAGS_BRIGHTNESS: i32 = 64;
pub const VP_FLAGS_CONTRAST: i32 = 128;
pub const VP_FLAGS_COPYPROTECT: i32 = 256;
pub const VP_FLAGS_FLICKER: i32 = 4;
pub const VP_FLAGS_MAX_UNSCALED: i32 = 16;
pub const VP_FLAGS_OVERSCAN: i32 = 8;
pub const VP_FLAGS_POSITION: i32 = 32;
pub const VP_FLAGS_TV_MODE: i32 = 1;
pub const VP_FLAGS_TV_STANDARD: i32 = 2;
pub const VP_MODE_TV_PLAYBACK: i32 = 2;
pub const VP_MODE_WIN_GRAPHICS: i32 = 1;
pub const VP_TV_STANDARD_NTSC_433: i32 = 65536;
pub const VP_TV_STANDARD_NTSC_M: i32 = 1;
pub const VP_TV_STANDARD_NTSC_M_J: i32 = 2;
pub const VP_TV_STANDARD_PAL_60: i32 = 262144;
pub const VP_TV_STANDARD_PAL_B: i32 = 4;
pub const VP_TV_STANDARD_PAL_D: i32 = 8;
pub const VP_TV_STANDARD_PAL_G: i32 = 131072;
pub const VP_TV_STANDARD_PAL_H: i32 = 16;
pub const VP_TV_STANDARD_PAL_I: i32 = 32;
pub const VP_TV_STANDARD_PAL_M: i32 = 64;
pub const VP_TV_STANDARD_PAL_N: i32 = 128;
pub const VP_TV_STANDARD_SECAM_B: i32 = 256;
pub const VP_TV_STANDARD_SECAM_D: i32 = 512;
pub const VP_TV_STANDARD_SECAM_G: i32 = 1024;
pub const VP_TV_STANDARD_SECAM_H: i32 = 2048;
pub const VP_TV_STANDARD_SECAM_K: i32 = 4096;
pub const VP_TV_STANDARD_SECAM_K1: i32 = 8192;
pub const VP_TV_STANDARD_SECAM_L: i32 = 16384;
pub const VP_TV_STANDARD_SECAM_L1: i32 = 524288;
pub const VP_TV_STANDARD_WIN_VGA: i32 = 32768;
