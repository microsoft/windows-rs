pub const FILE_VER_GET_LOCALISED: i32 = 1;
pub const FILE_VER_GET_NEUTRAL: i32 = 2;
pub const FILE_VER_GET_PREFETCHED: i32 = 4;
pub const VFFF_ISSHAREDFILE: i32 = 1;
pub const VFF_BUFFTOOSMALL: i32 = 4;
pub const VFF_CURNEDEST: i32 = 1;
pub const VFF_FILEINUSE: i32 = 2;
pub const VFT2_DRV_COMM: i32 = 10;
pub const VFT2_DRV_DISPLAY: i32 = 4;
pub const VFT2_DRV_INPUTMETHOD: i32 = 11;
pub const VFT2_DRV_INSTALLABLE: i32 = 8;
pub const VFT2_DRV_KEYBOARD: i32 = 2;
pub const VFT2_DRV_LANGUAGE: i32 = 3;
pub const VFT2_DRV_MOUSE: i32 = 5;
pub const VFT2_DRV_NETWORK: i32 = 6;
pub const VFT2_DRV_PRINTER: i32 = 1;
pub const VFT2_DRV_SOUND: i32 = 9;
pub const VFT2_DRV_SYSTEM: i32 = 7;
pub const VFT2_DRV_VERSIONED_PRINTER: i32 = 12;
pub const VFT2_FONT_RASTER: i32 = 1;
pub const VFT2_FONT_TRUETYPE: i32 = 3;
pub const VFT2_FONT_VECTOR: i32 = 2;
pub const VFT2_UNKNOWN: i32 = 0;
pub const VFT_APP: i32 = 1;
pub const VFT_DLL: i32 = 2;
pub const VFT_DRV: i32 = 3;
pub const VFT_FONT: i32 = 4;
pub const VFT_STATIC_LIB: i32 = 7;
pub const VFT_UNKNOWN: i32 = 0;
pub const VFT_VXD: i32 = 5;
pub const VIFF_DONTDELETEOLD: i32 = 2;
pub const VIFF_FORCEINSTALL: i32 = 1;
pub const VIF_ACCESSVIOLATION: i32 = 512;
pub const VIF_BUFFTOOSMALL: i32 = 262144;
pub const VIF_CANNOTCREATE: i32 = 2048;
pub const VIF_CANNOTDELETE: i32 = 4096;
pub const VIF_CANNOTDELETECUR: i32 = 16384;
pub const VIF_CANNOTLOADCABINET: i32 = 1048576;
pub const VIF_CANNOTLOADLZ32: i32 = 524288;
pub const VIF_CANNOTREADDST: i32 = 131072;
pub const VIF_CANNOTREADSRC: i32 = 65536;
pub const VIF_CANNOTRENAME: i32 = 8192;
pub const VIF_DIFFCODEPG: i32 = 16;
pub const VIF_DIFFLANG: i32 = 8;
pub const VIF_DIFFTYPE: i32 = 32;
pub const VIF_FILEINUSE: i32 = 128;
pub const VIF_MISMATCH: i32 = 2;
pub const VIF_OUTOFMEMORY: i32 = 32768;
pub const VIF_OUTOFSPACE: i32 = 256;
pub const VIF_SHARINGVIOLATION: i32 = 1024;
pub const VIF_SRCOLD: i32 = 4;
pub const VIF_TEMPFILE: i32 = 1;
pub const VIF_WRITEPROT: i32 = 64;
pub const VOS_DOS: i32 = 65536;
pub const VOS_DOS_WINDOWS16: i32 = 65537;
pub const VOS_DOS_WINDOWS32: i32 = 65540;
pub const VOS_NT: i32 = 262144;
pub const VOS_NT_WINDOWS32: i32 = 262148;
pub const VOS_OS216: i32 = 131072;
pub const VOS_OS216_PM16: i32 = 131074;
pub const VOS_OS232: i32 = 196608;
pub const VOS_OS232_PM32: i32 = 196611;
pub const VOS_UNKNOWN: i32 = 0;
pub const VOS_WINCE: i32 = 327680;
pub const VOS__BASE: i32 = 0;
pub const VOS__PM16: i32 = 2;
pub const VOS__PM32: i32 = 3;
pub const VOS__WINDOWS16: i32 = 1;
pub const VOS__WINDOWS32: i32 = 4;
pub const VS_FFI_FILEFLAGSMASK: i32 = 63;
pub const VS_FFI_SIGNATURE: u32 = 4277077181;
pub const VS_FFI_STRUCVERSION: i32 = 65536;
pub const VS_FF_DEBUG: i32 = 1;
pub const VS_FF_INFOINFERRED: i32 = 16;
pub const VS_FF_PATCHED: i32 = 4;
pub const VS_FF_PRERELEASE: i32 = 2;
pub const VS_FF_PRIVATEBUILD: i32 = 8;
pub const VS_FF_SPECIALBUILD: i32 = 32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
pub const VS_USER_DEFINED: i32 = 100;
pub const VS_VERSION_INFO: i32 = 1;
