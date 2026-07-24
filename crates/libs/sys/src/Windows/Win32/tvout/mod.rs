pub type LPVIDEOPARAMETERS = *mut VIDEOPARAMETERS;
pub type PVIDEOPARAMETERS = *mut VIDEOPARAMETERS;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VIDEOPARAMETERS {
    pub Guid: windows_sys::core::GUID,
    pub dwOffset: u32,
    pub dwCommand: u32,
    pub dwFlags: u32,
    pub dwMode: u32,
    pub dwTVStandard: u32,
    pub dwAvailableModes: u32,
    pub dwAvailableTVStandard: u32,
    pub dwFlickerFilter: u32,
    pub dwOverScanX: u32,
    pub dwOverScanY: u32,
    pub dwMaxUnscaledX: u32,
    pub dwMaxUnscaledY: u32,
    pub dwPositionX: u32,
    pub dwPositionY: u32,
    pub dwBrightness: u32,
    pub dwContrast: u32,
    pub dwCPType: u32,
    pub dwCPCommand: u32,
    pub dwCPStandard: u32,
    pub dwCPKey: u32,
    pub bCP_APSTriggerBits: u32,
    pub bOEMCopyProtection: [u8; 256],
}
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
