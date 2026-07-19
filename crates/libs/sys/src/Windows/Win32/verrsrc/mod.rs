pub const FILE_VER_GET_LOCALISED: u32 = 1;
pub const FILE_VER_GET_NEUTRAL: u32 = 2;
pub const FILE_VER_GET_PREFETCHED: u32 = 4;
pub const VFFF_ISSHAREDFILE: u32 = 1;
pub const VFF_BUFFTOOSMALL: u32 = 4;
pub const VFF_CURNEDEST: u32 = 1;
pub const VFF_FILEINUSE: u32 = 2;
pub const VFT2_DRV_COMM: u32 = 10;
pub const VFT2_DRV_DISPLAY: u32 = 4;
pub const VFT2_DRV_INPUTMETHOD: u32 = 11;
pub const VFT2_DRV_INSTALLABLE: u32 = 8;
pub const VFT2_DRV_KEYBOARD: u32 = 2;
pub const VFT2_DRV_LANGUAGE: u32 = 3;
pub const VFT2_DRV_MOUSE: u32 = 5;
pub const VFT2_DRV_NETWORK: u32 = 6;
pub const VFT2_DRV_PRINTER: u32 = 1;
pub const VFT2_DRV_SOUND: u32 = 9;
pub const VFT2_DRV_SYSTEM: u32 = 7;
pub const VFT2_DRV_VERSIONED_PRINTER: u32 = 12;
pub const VFT2_FONT_RASTER: u32 = 1;
pub const VFT2_FONT_TRUETYPE: u32 = 3;
pub const VFT2_FONT_VECTOR: u32 = 2;
pub const VFT2_UNKNOWN: u32 = 0;
pub const VFT_APP: u32 = 1;
pub const VFT_DLL: u32 = 2;
pub const VFT_DRV: u32 = 3;
pub const VFT_FONT: u32 = 4;
pub const VFT_STATIC_LIB: u32 = 7;
pub const VFT_UNKNOWN: u32 = 0;
pub const VFT_VXD: u32 = 5;
pub const VIFF_DONTDELETEOLD: u32 = 2;
pub const VIFF_FORCEINSTALL: u32 = 1;
pub const VIF_ACCESSVIOLATION: u32 = 512;
pub const VIF_BUFFTOOSMALL: u32 = 262144;
pub const VIF_CANNOTCREATE: u32 = 2048;
pub const VIF_CANNOTDELETE: u32 = 4096;
pub const VIF_CANNOTDELETECUR: u32 = 16384;
pub const VIF_CANNOTLOADCABINET: u32 = 1048576;
pub const VIF_CANNOTLOADLZ32: u32 = 524288;
pub const VIF_CANNOTREADDST: u32 = 131072;
pub const VIF_CANNOTREADSRC: u32 = 65536;
pub const VIF_CANNOTRENAME: u32 = 8192;
pub const VIF_DIFFCODEPG: u32 = 16;
pub const VIF_DIFFLANG: u32 = 8;
pub const VIF_DIFFTYPE: u32 = 32;
pub const VIF_FILEINUSE: u32 = 128;
pub const VIF_MISMATCH: u32 = 2;
pub const VIF_OUTOFMEMORY: u32 = 32768;
pub const VIF_OUTOFSPACE: u32 = 256;
pub const VIF_SHARINGVIOLATION: u32 = 1024;
pub const VIF_SRCOLD: u32 = 4;
pub const VIF_TEMPFILE: u32 = 1;
pub const VIF_WRITEPROT: u32 = 64;
pub const VOS_DOS: u32 = 65536;
pub const VOS_DOS_WINDOWS16: u32 = 65537;
pub const VOS_DOS_WINDOWS32: u32 = 65540;
pub const VOS_NT: u32 = 262144;
pub const VOS_NT_WINDOWS32: u32 = 262148;
pub const VOS_OS216: u32 = 131072;
pub const VOS_OS216_PM16: u32 = 131074;
pub const VOS_OS232: u32 = 196608;
pub const VOS_OS232_PM32: u32 = 196611;
pub const VOS_UNKNOWN: u32 = 0;
pub const VOS_WINCE: u32 = 327680;
pub const VOS__BASE: u32 = 0;
pub const VOS__PM16: u32 = 2;
pub const VOS__PM32: u32 = 3;
pub const VOS__WINDOWS16: u32 = 1;
pub const VOS__WINDOWS32: u32 = 4;
pub const VS_FFI_FILEFLAGSMASK: u32 = 63;
pub const VS_FFI_SIGNATURE: u32 = 4277077181;
pub const VS_FFI_STRUCVERSION: u32 = 65536;
pub const VS_FF_DEBUG: u32 = 1;
pub const VS_FF_INFOINFERRED: u32 = 16;
pub const VS_FF_PATCHED: u32 = 4;
pub const VS_FF_PRERELEASE: u32 = 2;
pub const VS_FF_PRIVATEBUILD: u32 = 8;
pub const VS_FF_SPECIALBUILD: u32 = 32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct VS_FIXEDFILEINFO {
    pub dwSignature: u32,
    pub dwStrucVersion: u32,
    pub dwFileVersionMS: u32,
    pub dwFileVersionLS: u32,
    pub dwProductVersionMS: u32,
    pub dwProductVersionLS: u32,
    pub dwFileFlagsMask: u32,
    pub dwFileFlags: u32,
    pub dwFileOS: u32,
    pub dwFileType: u32,
    pub dwFileSubtype: u32,
    pub dwFileDateMS: u32,
    pub dwFileDateLS: u32,
}
pub const VS_USER_DEFINED: u32 = 100;
pub const VS_VERSION_INFO: u32 = 1;
