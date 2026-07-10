#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPVIDEOPARAMETERS(pub *mut VIDEOPARAMETERS);
impl LPVIDEOPARAMETERS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPVIDEOPARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PVIDEOPARAMETERS(pub *mut VIDEOPARAMETERS);
impl PVIDEOPARAMETERS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PVIDEOPARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VIDEOPARAMETERS {
    pub Guid: windows_core::GUID,
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
pub const VP_COMMAND_GET: u32 = 1;
pub const VP_COMMAND_SET: u32 = 2;
pub const VP_CP_CMD_ACTIVATE: u32 = 1;
pub const VP_CP_CMD_CHANGE: u32 = 4;
pub const VP_CP_CMD_DEACTIVATE: u32 = 2;
pub const VP_CP_TYPE_APS_TRIGGER: u32 = 1;
pub const VP_CP_TYPE_MACROVISION: u32 = 2;
pub const VP_FLAGS_BRIGHTNESS: u32 = 64;
pub const VP_FLAGS_CONTRAST: u32 = 128;
pub const VP_FLAGS_COPYPROTECT: u32 = 256;
pub const VP_FLAGS_FLICKER: u32 = 4;
pub const VP_FLAGS_MAX_UNSCALED: u32 = 16;
pub const VP_FLAGS_OVERSCAN: u32 = 8;
pub const VP_FLAGS_POSITION: u32 = 32;
pub const VP_FLAGS_TV_MODE: u32 = 1;
pub const VP_FLAGS_TV_STANDARD: u32 = 2;
pub const VP_MODE_TV_PLAYBACK: u32 = 2;
pub const VP_MODE_WIN_GRAPHICS: u32 = 1;
pub const VP_TV_STANDARD_NTSC_433: u32 = 65536;
pub const VP_TV_STANDARD_NTSC_M: u32 = 1;
pub const VP_TV_STANDARD_NTSC_M_J: u32 = 2;
pub const VP_TV_STANDARD_PAL_60: u32 = 262144;
pub const VP_TV_STANDARD_PAL_B: u32 = 4;
pub const VP_TV_STANDARD_PAL_D: u32 = 8;
pub const VP_TV_STANDARD_PAL_G: u32 = 131072;
pub const VP_TV_STANDARD_PAL_H: u32 = 16;
pub const VP_TV_STANDARD_PAL_I: u32 = 32;
pub const VP_TV_STANDARD_PAL_M: u32 = 64;
pub const VP_TV_STANDARD_PAL_N: u32 = 128;
pub const VP_TV_STANDARD_SECAM_B: u32 = 256;
pub const VP_TV_STANDARD_SECAM_D: u32 = 512;
pub const VP_TV_STANDARD_SECAM_G: u32 = 1024;
pub const VP_TV_STANDARD_SECAM_H: u32 = 2048;
pub const VP_TV_STANDARD_SECAM_K: u32 = 4096;
pub const VP_TV_STANDARD_SECAM_K1: u32 = 8192;
pub const VP_TV_STANDARD_SECAM_L: u32 = 16384;
pub const VP_TV_STANDARD_SECAM_L1: u32 = 524288;
pub const VP_TV_STANDARD_WIN_VGA: u32 = 32768;
