#[cfg(all(feature = "minwindef", feature = "mmsyscom"))]
windows_link::link!("winmm.dll" "system" fn CloseDriver(hdriver : super::HDRVR, lparam1 : super::LPARAM, lparam2 : super::LPARAM) -> super::LRESULT);
#[cfg(all(feature = "minwindef", feature = "mmsyscom", feature = "shared"))]
windows_link::link!("winmm.dll" "system" fn DefDriverProc(dwdriveridentifier : super::DWORD_PTR, hdrvr : super::HDRVR, umsg : super::UINT, lparam1 : super::LPARAM, lparam2 : super::LPARAM) -> super::LRESULT);
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
windows_link::link!("winmm.dll" "system" fn DriverCallback(dwcallback : super::DWORD_PTR, dwflags : super::DWORD, hdevice : super::HDRVR, dwmsg : super::DWORD, dwuser : super::DWORD_PTR, dwparam1 : super::DWORD_PTR, dwparam2 : super::DWORD_PTR) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "mmsyscom"))]
windows_link::link!("winmm.dll" "system" fn DrvGetModuleHandle(hdriver : super::HDRVR) -> super::HMODULE);
#[cfg(all(feature = "minwindef", feature = "mmsyscom"))]
windows_link::link!("winmm.dll" "system" fn GetDriverModuleHandle(hdriver : super::HDRVR) -> super::HMODULE);
#[cfg(all(feature = "minwindef", feature = "mmsyscom"))]
windows_link::link!("winmm.dll" "system" fn OpenDriver(szdrivername : windows_sys::core::PCWSTR, szsectionname : windows_sys::core::PCWSTR, lparam2 : super::LPARAM) -> super::HDRVR);
#[cfg(all(feature = "minwindef", feature = "mmsyscom", feature = "shared"))]
windows_link::link!("winmm.dll" "system" fn SendDriverMessage(hdriver : super::HDRVR, message : super::UINT, lparam1 : super::LPARAM, lparam2 : super::LPARAM) -> super::LRESULT);
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
windows_link::link!("winmm.dll" "system" fn mmDrvInstall(hdriver : super::HDRVR, wszdrventry : windows_sys::core::PCWSTR, drvmessage : DRIVERMSGPROC, wflags : super::UINT) -> super::UINT);
#[cfg(all(feature = "minwindef", feature = "mmsyscom", feature = "shared"))]
windows_link::link!("winmm.dll" "system" fn mmioAdvance(hmmio : HMMIO, pmmioinfo : LPMMIOINFO, fuadvance : super::UINT) -> super::MMRESULT);
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
windows_link::link!("winmm.dll" "system" fn mmioAscend(hmmio : HMMIO, pmmcki : LPMMCKINFO, fuascend : super::UINT) -> super::MMRESULT);
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
windows_link::link!("winmm.dll" "system" fn mmioClose(hmmio : HMMIO, fuclose : super::UINT) -> super::MMRESULT);
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
windows_link::link!("winmm.dll" "system" fn mmioCreateChunk(hmmio : HMMIO, pmmcki : LPMMCKINFO, fucreate : super::UINT) -> super::MMRESULT);
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
windows_link::link!("winmm.dll" "system" fn mmioDescend(hmmio : HMMIO, pmmcki : LPMMCKINFO, pmmckiparent : *const MMCKINFO, fudescend : super::UINT) -> super::MMRESULT);
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
windows_link::link!("winmm.dll" "system" fn mmioFlush(hmmio : HMMIO, fuflush : super::UINT) -> super::MMRESULT);
#[cfg(all(feature = "minwindef", feature = "mmsyscom", feature = "shared"))]
windows_link::link!("winmm.dll" "system" fn mmioGetInfo(hmmio : HMMIO, pmmioinfo : LPMMIOINFO, fuinfo : super::UINT) -> super::MMRESULT);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("winmm.dll" "system" fn mmioInstallIOProcA(fccioproc : FOURCC, pioproc : LPMMIOPROC, dwflags : super::DWORD) -> LPMMIOPROC);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("winmm.dll" "system" fn mmioInstallIOProcW(fccioproc : FOURCC, pioproc : LPMMIOPROC, dwflags : super::DWORD) -> LPMMIOPROC);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("winmm.dll" "system" fn mmioOpenA(pszfilename : windows_sys::core::PSTR, pmmioinfo : LPMMIOINFO, fdwopen : super::DWORD) -> HMMIO);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("winmm.dll" "system" fn mmioOpenW(pszfilename : windows_sys::core::PWSTR, pmmioinfo : LPMMIOINFO, fdwopen : super::DWORD) -> HMMIO);
#[cfg(feature = "shared")]
windows_link::link!("winmm.dll" "system" fn mmioRead(hmmio : HMMIO, pch : HPSTR, cch : super::LONG) -> super::LONG);
#[cfg(all(feature = "minwindef", feature = "mmsyscom", feature = "shared"))]
windows_link::link!("winmm.dll" "system" fn mmioRenameA(pszfilename : windows_sys::core::PCSTR, psznewfilename : windows_sys::core::PCSTR, pmmioinfo : LPCMMIOINFO, fdwrename : super::DWORD) -> super::MMRESULT);
#[cfg(all(feature = "minwindef", feature = "mmsyscom", feature = "shared"))]
windows_link::link!("winmm.dll" "system" fn mmioRenameW(pszfilename : windows_sys::core::PCWSTR, psznewfilename : windows_sys::core::PCWSTR, pmmioinfo : LPCMMIOINFO, fdwrename : super::DWORD) -> super::MMRESULT);
#[cfg(feature = "shared")]
windows_link::link!("winmm.dll" "system" fn mmioSeek(hmmio : HMMIO, loffset : super::LONG, iorigin : i32) -> super::LONG);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("winmm.dll" "system" fn mmioSendMessage(hmmio : HMMIO, umsg : super::UINT, lparam1 : super::LPARAM, lparam2 : super::LPARAM) -> super::LRESULT);
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
windows_link::link!("winmm.dll" "system" fn mmioSetBuffer(hmmio : HMMIO, pchbuffer : windows_sys::core::PSTR, cchbuffer : super::LONG, fubuffer : super::UINT) -> super::MMRESULT);
#[cfg(all(feature = "minwindef", feature = "mmsyscom", feature = "shared"))]
windows_link::link!("winmm.dll" "system" fn mmioSetInfo(hmmio : HMMIO, pmmioinfo : LPCMMIOINFO, fuinfo : super::UINT) -> super::MMRESULT);
#[cfg(feature = "shared")]
windows_link::link!("winmm.dll" "system" fn mmioStringToFOURCCA(sz : windows_sys::core::PCSTR, uflags : super::UINT) -> FOURCC);
#[cfg(feature = "shared")]
windows_link::link!("winmm.dll" "system" fn mmioStringToFOURCCW(sz : windows_sys::core::PCWSTR, uflags : super::UINT) -> FOURCC);
#[cfg(feature = "shared")]
windows_link::link!("winmm.dll" "system" fn mmioWrite(hmmio : HMMIO, pch : *const i8, cch : super::LONG) -> super::LONG);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("api-ms-win-mm-misc-l1-1-1.dll" "system" fn sndOpenSound(eventname : windows_sys::core::PCWSTR, appname : windows_sys::core::PCWSTR, flags : super::INT32, filehandle : super::PHANDLE) -> super::LONG);
pub const CFSEPCHAR: i8 = 43;
#[cfg(feature = "shared")]
pub type DRIVERMSGPROC = Option<unsafe extern "system" fn(param0: super::DWORD, param1: super::DWORD, param2: super::DWORD_PTR, param3: super::DWORD_PTR, param4: super::DWORD_PTR) -> super::DWORD>;
#[cfg(all(feature = "minwindef", feature = "mmsyscom", feature = "shared"))]
pub type DRIVERPROC = Option<unsafe extern "system" fn(param0: super::DWORD_PTR, param1: super::HDRVR, param2: super::UINT, param3: super::LPARAM, param4: super::LPARAM) -> super::LRESULT>;
pub const DRVCNF_CANCEL: i32 = 0;
pub const DRVCNF_OK: i32 = 1;
pub const DRVCNF_RESTART: i32 = 2;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DRVCONFIGINFO {
    pub dwDCISize: super::DWORD,
    pub lpszDCISectionName: windows_sys::core::PCWSTR,
    pub lpszDCIAliasName: windows_sys::core::PCWSTR,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DRVCONFIGINFOEX {
    pub dwDCISize: super::DWORD,
    pub lpszDCISectionName: windows_sys::core::PCWSTR,
    pub lpszDCIAliasName: windows_sys::core::PCWSTR,
    pub dnDevNode: super::DWORD,
}
pub const DRV_CANCEL: i32 = 0;
pub const DRV_CLOSE: i32 = 4;
pub const DRV_CONFIGURE: i32 = 7;
pub const DRV_DISABLE: i32 = 5;
pub const DRV_ENABLE: i32 = 2;
pub const DRV_EXITSESSION: i32 = 11;
pub const DRV_FREE: i32 = 6;
pub const DRV_INSTALL: i32 = 9;
pub const DRV_LOAD: i32 = 1;
pub const DRV_MCI_FIRST: i32 = 2048;
pub const DRV_MCI_LAST: i32 = 6143;
pub const DRV_OK: i32 = 1;
pub const DRV_OPEN: i32 = 3;
pub const DRV_POWER: i32 = 15;
pub const DRV_QUERYCONFIGURE: i32 = 8;
pub const DRV_REMOVE: i32 = 10;
pub const DRV_RESERVED: i32 = 2048;
pub const DRV_RESTART: i32 = 2;
pub const DRV_USER: i32 = 16384;
pub type FOURCC = u32;
#[cfg(feature = "shared")]
pub const FOURCC_DOS: super::DWORD = 542330692;
#[cfg(feature = "shared")]
pub const FOURCC_LIST: super::DWORD = 1414744396;
#[cfg(feature = "shared")]
pub const FOURCC_MEM: super::DWORD = 541934925;
#[cfg(feature = "shared")]
pub const FOURCC_RIFF: super::DWORD = 1179011410;
pub type HMMIO = *mut HMMIO__;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct HMMIO__ {
    pub unused: i32,
}
pub type HPSTR = *mut i8;
#[cfg(feature = "shared")]
pub type LPCMMCKINFO = *const MMCKINFO;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type LPCMMIOINFO = *const MMIOINFO;
#[cfg(feature = "shared")]
pub type LPDRVCONFIGINFO = *mut DRVCONFIGINFO;
#[cfg(feature = "shared")]
pub type LPDRVCONFIGINFOEX = *mut DRVCONFIGINFOEX;
#[cfg(feature = "shared")]
pub type LPMMCKINFO = *mut MMCKINFO;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type LPMMIOINFO = *mut MMIOINFO;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type LPMMIOPROC = Option<unsafe extern "system" fn(lpmmioinfo: windows_sys::core::PSTR, umsg: super::UINT, lparam1: super::LPARAM, lparam2: super::LPARAM) -> super::LRESULT>;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct MMCKINFO {
    pub ckid: FOURCC,
    pub cksize: super::DWORD,
    pub fccType: FOURCC,
    pub dwDataOffset: super::DWORD,
    pub dwFlags: super::DWORD,
}
pub const MMIOERR_ACCESSDENIED: i32 = 268;
pub const MMIOERR_BASE: i32 = 256;
pub const MMIOERR_CANNOTCLOSE: i32 = 260;
pub const MMIOERR_CANNOTEXPAND: i32 = 264;
pub const MMIOERR_CANNOTOPEN: i32 = 259;
pub const MMIOERR_CANNOTREAD: i32 = 261;
pub const MMIOERR_CANNOTSEEK: i32 = 263;
pub const MMIOERR_CANNOTWRITE: i32 = 262;
pub const MMIOERR_CHUNKNOTFOUND: i32 = 265;
pub const MMIOERR_FILENOTFOUND: i32 = 257;
pub const MMIOERR_INVALIDFILE: i32 = 272;
pub const MMIOERR_NETWORKERROR: i32 = 270;
pub const MMIOERR_OUTOFMEMORY: i32 = 258;
pub const MMIOERR_PATHNOTFOUND: i32 = 267;
pub const MMIOERR_SHARINGVIOLATION: i32 = 269;
pub const MMIOERR_TOOMANYOPENFILES: i32 = 271;
pub const MMIOERR_UNBUFFERED: i32 = 266;
#[repr(C, packed(1))]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct MMIOINFO {
    pub dwFlags: super::DWORD,
    pub fccIOProc: FOURCC,
    pub pIOProc: LPMMIOPROC,
    pub wErrorRet: super::UINT,
    pub htask: super::HTASK,
    pub cchBuffer: super::LONG,
    pub pchBuffer: HPSTR,
    pub pchNext: HPSTR,
    pub pchEndRead: HPSTR,
    pub pchEndWrite: HPSTR,
    pub lBufOffset: super::LONG,
    pub lDiskOffset: super::LONG,
    pub adwInfo: [super::DWORD; 3],
    pub dwReserved1: super::DWORD,
    pub dwReserved2: super::DWORD,
    pub hmmio: HMMIO,
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
impl Default for MMIOINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MMIOM_CLOSE: i32 = 4;
pub const MMIOM_OPEN: i32 = 3;
pub const MMIOM_READ: i32 = 0;
pub const MMIOM_RENAME: i32 = 6;
pub const MMIOM_SEEK: i32 = 2;
pub const MMIOM_USER: i32 = 32768;
pub const MMIOM_WRITE: i32 = 1;
pub const MMIOM_WRITEFLUSH: i32 = 5;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type MMIOPROC = Option<unsafe extern "system" fn(lpmmioinfo: windows_sys::core::PSTR, umsg: super::UINT, lparam1: super::LPARAM, lparam2: super::LPARAM) -> super::LRESULT>;
pub const MMIO_ALLOCBUF: i32 = 65536;
pub const MMIO_COMPAT: i32 = 0;
pub const MMIO_CREATE: i32 = 4096;
pub const MMIO_CREATELIST: i32 = 64;
pub const MMIO_CREATERIFF: i32 = 32;
pub const MMIO_DEFAULTBUFFER: i32 = 8192;
pub const MMIO_DELETE: i32 = 512;
pub const MMIO_DENYNONE: i32 = 64;
pub const MMIO_DENYREAD: i32 = 48;
pub const MMIO_DENYWRITE: i32 = 32;
pub const MMIO_DIRTY: i32 = 268435456;
pub const MMIO_EMPTYBUF: i32 = 16;
pub const MMIO_EXCLUSIVE: i32 = 16;
pub const MMIO_EXIST: i32 = 16384;
pub const MMIO_FHOPEN: i32 = 16;
pub const MMIO_FINDCHUNK: i32 = 16;
pub const MMIO_FINDLIST: i32 = 64;
pub const MMIO_FINDPROC: i32 = 262144;
pub const MMIO_FINDRIFF: i32 = 32;
pub const MMIO_GETTEMP: i32 = 131072;
pub const MMIO_GLOBALPROC: i32 = 268435456;
pub const MMIO_INSTALLPROC: i32 = 65536;
pub const MMIO_PARSE: i32 = 256;
pub const MMIO_READ: i32 = 0;
pub const MMIO_READWRITE: i32 = 2;
pub const MMIO_REMOVEPROC: i32 = 131072;
pub const MMIO_RWMODE: i32 = 3;
pub const MMIO_SHAREMODE: i32 = 112;
pub const MMIO_TOUPPER: i32 = 16;
pub const MMIO_UNICODEPROC: i32 = 16777216;
pub const MMIO_WRITE: i32 = 1;
#[cfg(feature = "shared")]
pub type NPDRVCONFIGINFO = *mut DRVCONFIGINFO;
#[cfg(feature = "shared")]
pub type NPDRVCONFIGINFOEX = *mut DRVCONFIGINFOEX;
#[cfg(feature = "shared")]
pub type NPMMCKINFO = *mut MMCKINFO;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type NPMMIOINFO = *mut MMIOINFO;
#[cfg(feature = "shared")]
pub type PDRVCONFIGINFO = *mut DRVCONFIGINFO;
#[cfg(feature = "shared")]
pub type PDRVCONFIGINFOEX = *mut DRVCONFIGINFOEX;
#[cfg(feature = "shared")]
pub type PMMCKINFO = *mut MMCKINFO;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PMMIOINFO = *mut MMIOINFO;
pub const SEEK_CUR: i32 = 1;
pub const SEEK_END: i32 = 2;
pub const SEEK_SET: i32 = 0;
