#[cfg(all(feature = "minwindef", feature = "mmsyscom"))]
windows_link::link!("winmm.dll" "system" fn CloseDriver(hdriver : super::HDRVR, lparam1 : super::LPARAM, lparam2 : super::LPARAM) -> super::LRESULT);
#[cfg(all(feature = "minwindef", feature = "mmsyscom"))]
windows_link::link!("winmm.dll" "system" fn DefDriverProc(dwdriveridentifier : usize, hdrvr : super::HDRVR, umsg : u32, lparam1 : super::LPARAM, lparam2 : super::LPARAM) -> super::LRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn DriverCallback(dwcallback : usize, dwflags : u32, hdevice : super::HDRVR, dwmsg : u32, dwuser : usize, dwparam1 : usize, dwparam2 : usize) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "mmsyscom"))]
windows_link::link!("winmm.dll" "system" fn DrvGetModuleHandle(hdriver : super::HDRVR) -> super::HMODULE);
#[cfg(all(feature = "minwindef", feature = "mmsyscom"))]
windows_link::link!("winmm.dll" "system" fn GetDriverModuleHandle(hdriver : super::HDRVR) -> super::HMODULE);
#[cfg(all(feature = "minwindef", feature = "mmsyscom"))]
windows_link::link!("winmm.dll" "system" fn OpenDriver(szdrivername : windows_sys::core::PCWSTR, szsectionname : windows_sys::core::PCWSTR, lparam2 : super::LPARAM) -> super::HDRVR);
#[cfg(all(feature = "minwindef", feature = "mmsyscom"))]
windows_link::link!("winmm.dll" "system" fn SendDriverMessage(hdriver : super::HDRVR, message : u32, lparam1 : super::LPARAM, lparam2 : super::LPARAM) -> super::LRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn mmDrvInstall(hdriver : super::HDRVR, wszdrventry : windows_sys::core::PCWSTR, drvmessage : DRIVERMSGPROC, wflags : u32) -> u32);
#[cfg(all(feature = "minwindef", feature = "mmsyscom"))]
windows_link::link!("winmm.dll" "system" fn mmioAdvance(hmmio : HMMIO, pmmioinfo : *const MMIOINFO, fuadvance : u32) -> super::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn mmioAscend(hmmio : HMMIO, pmmcki : *const MMCKINFO, fuascend : u32) -> super::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn mmioClose(hmmio : HMMIO, fuclose : u32) -> super::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn mmioCreateChunk(hmmio : HMMIO, pmmcki : *const MMCKINFO, fucreate : u32) -> super::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn mmioDescend(hmmio : HMMIO, pmmcki : *mut MMCKINFO, pmmckiparent : *const MMCKINFO, fudescend : u32) -> super::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn mmioFlush(hmmio : HMMIO, fuflush : u32) -> super::MMRESULT);
#[cfg(all(feature = "minwindef", feature = "mmsyscom"))]
windows_link::link!("winmm.dll" "system" fn mmioGetInfo(hmmio : HMMIO, pmmioinfo : *mut MMIOINFO, fuinfo : u32) -> super::MMRESULT);
#[cfg(feature = "minwindef")]
windows_link::link!("winmm.dll" "system" fn mmioInstallIOProcA(fccioproc : FOURCC, pioproc : LPMMIOPROC, dwflags : u32) -> LPMMIOPROC);
#[cfg(feature = "minwindef")]
windows_link::link!("winmm.dll" "system" fn mmioInstallIOProcW(fccioproc : FOURCC, pioproc : LPMMIOPROC, dwflags : u32) -> LPMMIOPROC);
#[cfg(feature = "minwindef")]
windows_link::link!("winmm.dll" "system" fn mmioOpenA(pszfilename : windows_sys::core::PSTR, pmmioinfo : *mut MMIOINFO, fdwopen : u32) -> HMMIO);
#[cfg(feature = "minwindef")]
windows_link::link!("winmm.dll" "system" fn mmioOpenW(pszfilename : windows_sys::core::PWSTR, pmmioinfo : *mut MMIOINFO, fdwopen : u32) -> HMMIO);
windows_link::link!("winmm.dll" "system" fn mmioRead(hmmio : HMMIO, pch : *mut i8, cch : i32) -> i32);
#[cfg(all(feature = "minwindef", feature = "mmsyscom"))]
windows_link::link!("winmm.dll" "system" fn mmioRenameA(pszfilename : windows_sys::core::PCSTR, psznewfilename : windows_sys::core::PCSTR, pmmioinfo : *const MMIOINFO, fdwrename : u32) -> super::MMRESULT);
#[cfg(all(feature = "minwindef", feature = "mmsyscom"))]
windows_link::link!("winmm.dll" "system" fn mmioRenameW(pszfilename : windows_sys::core::PCWSTR, psznewfilename : windows_sys::core::PCWSTR, pmmioinfo : *const MMIOINFO, fdwrename : u32) -> super::MMRESULT);
windows_link::link!("winmm.dll" "system" fn mmioSeek(hmmio : HMMIO, loffset : i32, iorigin : i32) -> i32);
#[cfg(feature = "minwindef")]
windows_link::link!("winmm.dll" "system" fn mmioSendMessage(hmmio : HMMIO, umsg : u32, lparam1 : super::LPARAM, lparam2 : super::LPARAM) -> super::LRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("winmm.dll" "system" fn mmioSetBuffer(hmmio : HMMIO, pchbuffer : windows_sys::core::PSTR, cchbuffer : i32, fubuffer : u32) -> super::MMRESULT);
#[cfg(all(feature = "minwindef", feature = "mmsyscom"))]
windows_link::link!("winmm.dll" "system" fn mmioSetInfo(hmmio : HMMIO, pmmioinfo : *const MMIOINFO, fuinfo : u32) -> super::MMRESULT);
windows_link::link!("winmm.dll" "system" fn mmioStringToFOURCCA(sz : windows_sys::core::PCSTR, uflags : u32) -> FOURCC);
windows_link::link!("winmm.dll" "system" fn mmioStringToFOURCCW(sz : windows_sys::core::PCWSTR, uflags : u32) -> FOURCC);
windows_link::link!("winmm.dll" "system" fn mmioWrite(hmmio : HMMIO, pch : *const i8, cch : i32) -> i32);
#[cfg(feature = "winnt")]
windows_link::link!("api-ms-win-mm-misc-l1-1-1.dll" "system" fn sndOpenSound(eventname : windows_sys::core::PCWSTR, appname : windows_sys::core::PCWSTR, flags : i32, filehandle : *mut super::HANDLE) -> i32);
pub const CFSEPCHAR: u32 = 43;
pub type DRIVERMSGPROC = Option<unsafe extern "system" fn(param0: u32, param1: u32, param2: usize, param3: usize, param4: usize) -> u32>;
#[cfg(all(feature = "minwindef", feature = "mmsyscom"))]
pub type DRIVERPROC = Option<unsafe extern "system" fn(param0: usize, param1: super::HDRVR, param2: u32, param3: super::LPARAM, param4: super::LPARAM) -> super::LRESULT>;
pub const DRVCNF_CANCEL: i32 = 0;
pub const DRVCNF_OK: i32 = 1;
pub const DRVCNF_RESTART: i32 = 2;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DRVCONFIGINFO {
    pub dwDCISize: u32,
    pub lpszDCISectionName: windows_sys::core::PCWSTR,
    pub lpszDCIAliasName: windows_sys::core::PCWSTR,
}
impl Default for DRVCONFIGINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DRVCONFIGINFOEX {
    pub dwDCISize: u32,
    pub lpszDCISectionName: windows_sys::core::PCWSTR,
    pub lpszDCIAliasName: windows_sys::core::PCWSTR,
    pub dnDevNode: u32,
}
impl Default for DRVCONFIGINFOEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
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
pub const FOURCC_DOS: u32 = 542330692;
pub const FOURCC_LIST: u32 = 1414744396;
pub const FOURCC_MEM: u32 = 541934925;
pub const FOURCC_RIFF: u32 = 1179011410;
pub type HMMIO = *mut core::ffi::c_void;
pub type HPSTR = *mut i8;
pub type LPCMMCKINFO = *const MMCKINFO;
#[cfg(feature = "minwindef")]
pub type LPCMMIOINFO = *const MMIOINFO;
pub type LPDRVCONFIGINFO = *mut DRVCONFIGINFO;
pub type LPDRVCONFIGINFOEX = *mut DRVCONFIGINFOEX;
pub type LPMMCKINFO = *mut MMCKINFO;
#[cfg(feature = "minwindef")]
pub type LPMMIOINFO = *mut MMIOINFO;
#[cfg(feature = "minwindef")]
pub type LPMMIOPROC = Option<unsafe extern "system" fn(lpmmioinfo: windows_sys::core::PCSTR, umsg: u32, lparam1: super::LPARAM, lparam2: super::LPARAM) -> super::LRESULT>;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct MMCKINFO {
    pub ckid: FOURCC,
    pub cksize: u32,
    pub fccType: FOURCC,
    pub dwDataOffset: u32,
    pub dwFlags: u32,
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
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct MMIOINFO {
    pub dwFlags: u32,
    pub fccIOProc: FOURCC,
    pub pIOProc: LPMMIOPROC,
    pub wErrorRet: u32,
    pub htask: super::HTASK,
    pub cchBuffer: i32,
    pub pchBuffer: HPSTR,
    pub pchNext: HPSTR,
    pub pchEndRead: HPSTR,
    pub pchEndWrite: HPSTR,
    pub lBufOffset: i32,
    pub lDiskOffset: i32,
    pub adwInfo: [u32; 3],
    pub dwReserved1: u32,
    pub dwReserved2: u32,
    pub hmmio: HMMIO,
}
#[cfg(feature = "minwindef")]
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
#[cfg(feature = "minwindef")]
pub type MMIOPROC = Option<unsafe extern "system" fn(lpmmioinfo: windows_sys::core::PCSTR, umsg: u32, lparam1: super::LPARAM, lparam2: super::LPARAM) -> super::LRESULT>;
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
pub type NPDRVCONFIGINFO = *mut DRVCONFIGINFO;
pub type NPDRVCONFIGINFOEX = *mut DRVCONFIGINFOEX;
pub type NPMMCKINFO = *mut MMCKINFO;
#[cfg(feature = "minwindef")]
pub type NPMMIOINFO = *mut MMIOINFO;
pub type PDRVCONFIGINFO = *mut DRVCONFIGINFO;
pub type PDRVCONFIGINFOEX = *mut DRVCONFIGINFOEX;
pub type PMMCKINFO = *mut MMCKINFO;
#[cfg(feature = "minwindef")]
pub type PMMIOINFO = *mut MMIOINFO;
pub const SEEK_CUR: i32 = 1;
pub const SEEK_END: i32 = 2;
pub const SEEK_SET: i32 = 0;
