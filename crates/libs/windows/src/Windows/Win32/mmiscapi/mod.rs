#[cfg(all(feature = "minwindef", feature = "mmsyscom"))]
#[inline]
pub unsafe fn CloseDriver(hdriver: super::HDRVR, lparam1: super::LPARAM, lparam2: super::LPARAM) -> super::LRESULT {
    windows_core::link!("winmm.dll" "system" fn CloseDriver(hdriver : super::HDRVR, lparam1 : super::LPARAM, lparam2 : super::LPARAM) -> super::LRESULT);
    unsafe { CloseDriver(hdriver, lparam1, lparam2) }
}
#[cfg(all(feature = "minwindef", feature = "mmsyscom", feature = "shared"))]
#[inline]
pub unsafe fn DefDriverProc(dwdriveridentifier: super::DWORD_PTR, hdrvr: super::HDRVR, umsg: super::UINT, lparam1: super::LPARAM, lparam2: super::LPARAM) -> super::LRESULT {
    windows_core::link!("winmm.dll" "system" fn DefDriverProc(dwdriveridentifier : super::DWORD_PTR, hdrvr : super::HDRVR, umsg : super::UINT, lparam1 : super::LPARAM, lparam2 : super::LPARAM) -> super::LRESULT);
    unsafe { DefDriverProc(dwdriveridentifier, hdrvr, umsg, lparam1, lparam2) }
}
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
#[inline]
pub unsafe fn DriverCallback(dwcallback: super::DWORD_PTR, dwflags: super::DWORD, hdevice: super::HDRVR, dwmsg: super::DWORD, dwuser: super::DWORD_PTR, dwparam1: super::DWORD_PTR, dwparam2: super::DWORD_PTR) -> windows_core::BOOL {
    windows_core::link!("winmm.dll" "system" fn DriverCallback(dwcallback : super::DWORD_PTR, dwflags : super::DWORD, hdevice : super::HDRVR, dwmsg : super::DWORD, dwuser : super::DWORD_PTR, dwparam1 : super::DWORD_PTR, dwparam2 : super::DWORD_PTR) -> windows_core::BOOL);
    unsafe { DriverCallback(dwcallback, dwflags, hdevice as _, dwmsg, dwuser, dwparam1, dwparam2) }
}
#[cfg(all(feature = "minwindef", feature = "mmsyscom"))]
#[inline]
pub unsafe fn DrvGetModuleHandle(hdriver: super::HDRVR) -> super::HMODULE {
    windows_core::link!("winmm.dll" "system" fn DrvGetModuleHandle(hdriver : super::HDRVR) -> super::HMODULE);
    unsafe { DrvGetModuleHandle(hdriver) }
}
#[cfg(all(feature = "minwindef", feature = "mmsyscom"))]
#[inline]
pub unsafe fn GetDriverModuleHandle(hdriver: super::HDRVR) -> super::HMODULE {
    windows_core::link!("winmm.dll" "system" fn GetDriverModuleHandle(hdriver : super::HDRVR) -> super::HMODULE);
    unsafe { GetDriverModuleHandle(hdriver) }
}
#[cfg(all(feature = "minwindef", feature = "mmsyscom"))]
#[inline]
pub unsafe fn OpenDriver<P0, P1>(szdrivername: P0, szsectionname: P1, lparam2: super::LPARAM) -> super::HDRVR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winmm.dll" "system" fn OpenDriver(szdrivername : windows_core::PCWSTR, szsectionname : windows_core::PCWSTR, lparam2 : super::LPARAM) -> super::HDRVR);
    unsafe { OpenDriver(szdrivername.param().abi(), szsectionname.param().abi(), lparam2) }
}
#[cfg(all(feature = "minwindef", feature = "mmsyscom", feature = "shared"))]
#[inline]
pub unsafe fn SendDriverMessage(hdriver: super::HDRVR, message: super::UINT, lparam1: super::LPARAM, lparam2: super::LPARAM) -> super::LRESULT {
    windows_core::link!("winmm.dll" "system" fn SendDriverMessage(hdriver : super::HDRVR, message : super::UINT, lparam1 : super::LPARAM, lparam2 : super::LPARAM) -> super::LRESULT);
    unsafe { SendDriverMessage(hdriver, message, lparam1, lparam2) }
}
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
#[inline]
pub unsafe fn mmDrvInstall(hdriver: super::HDRVR, wszdrventry: &windows_core::PCWSTR, drvmessage: DRIVERMSGPROC, wflags: super::UINT) -> super::UINT {
    windows_core::link!("winmm.dll" "system" fn mmDrvInstall(hdriver : super::HDRVR, wszdrventry : windows_core::PCWSTR, drvmessage : DRIVERMSGPROC, wflags : super::UINT) -> super::UINT);
    unsafe { mmDrvInstall(hdriver as _, core::mem::transmute_copy(wszdrventry), drvmessage, wflags) }
}
#[cfg(all(feature = "minwindef", feature = "mmsyscom", feature = "shared"))]
#[inline]
pub unsafe fn mmioAdvance(hmmio: HMMIO, pmmioinfo: Option<LPMMIOINFO>, fuadvance: super::UINT) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn mmioAdvance(hmmio : HMMIO, pmmioinfo : LPMMIOINFO, fuadvance : super::UINT) -> super::MMRESULT);
    unsafe { mmioAdvance(hmmio, pmmioinfo.unwrap_or(core::mem::zeroed()) as _, fuadvance) }
}
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
#[inline]
pub unsafe fn mmioAscend(hmmio: HMMIO, pmmcki: LPMMCKINFO, fuascend: super::UINT) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn mmioAscend(hmmio : HMMIO, pmmcki : LPMMCKINFO, fuascend : super::UINT) -> super::MMRESULT);
    unsafe { mmioAscend(hmmio, pmmcki, fuascend) }
}
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
#[inline]
pub unsafe fn mmioClose(hmmio: HMMIO, fuclose: super::UINT) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn mmioClose(hmmio : HMMIO, fuclose : super::UINT) -> super::MMRESULT);
    unsafe { mmioClose(hmmio, fuclose) }
}
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
#[inline]
pub unsafe fn mmioCreateChunk(hmmio: HMMIO, pmmcki: LPMMCKINFO, fucreate: super::UINT) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn mmioCreateChunk(hmmio : HMMIO, pmmcki : LPMMCKINFO, fucreate : super::UINT) -> super::MMRESULT);
    unsafe { mmioCreateChunk(hmmio, pmmcki, fucreate) }
}
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
#[inline]
pub unsafe fn mmioDescend(hmmio: HMMIO, pmmcki: LPMMCKINFO, pmmckiparent: Option<*const MMCKINFO>, fudescend: super::UINT) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn mmioDescend(hmmio : HMMIO, pmmcki : LPMMCKINFO, pmmckiparent : *const MMCKINFO, fudescend : super::UINT) -> super::MMRESULT);
    unsafe { mmioDescend(hmmio, pmmcki as _, pmmckiparent.unwrap_or(core::mem::zeroed()) as _, fudescend) }
}
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
#[inline]
pub unsafe fn mmioFlush(hmmio: HMMIO, fuflush: super::UINT) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn mmioFlush(hmmio : HMMIO, fuflush : super::UINT) -> super::MMRESULT);
    unsafe { mmioFlush(hmmio, fuflush) }
}
#[cfg(all(feature = "minwindef", feature = "mmsyscom", feature = "shared"))]
#[inline]
pub unsafe fn mmioGetInfo(hmmio: HMMIO, pmmioinfo: LPMMIOINFO, fuinfo: super::UINT) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn mmioGetInfo(hmmio : HMMIO, pmmioinfo : LPMMIOINFO, fuinfo : super::UINT) -> super::MMRESULT);
    unsafe { mmioGetInfo(hmmio, pmmioinfo as _, fuinfo) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn mmioInstallIOProcA(fccioproc: FOURCC, pioproc: LPMMIOPROC, dwflags: super::DWORD) -> LPMMIOPROC {
    windows_core::link!("winmm.dll" "system" fn mmioInstallIOProcA(fccioproc : FOURCC, pioproc : LPMMIOPROC, dwflags : super::DWORD) -> LPMMIOPROC);
    unsafe { mmioInstallIOProcA(fccioproc, pioproc, dwflags) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn mmioInstallIOProcW(fccioproc: FOURCC, pioproc: LPMMIOPROC, dwflags: super::DWORD) -> LPMMIOPROC {
    windows_core::link!("winmm.dll" "system" fn mmioInstallIOProcW(fccioproc : FOURCC, pioproc : LPMMIOPROC, dwflags : super::DWORD) -> LPMMIOPROC);
    unsafe { mmioInstallIOProcW(fccioproc, pioproc, dwflags) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn mmioOpenA(pszfilename: Option<windows_core::PSTR>, pmmioinfo: Option<LPMMIOINFO>, fdwopen: super::DWORD) -> HMMIO {
    windows_core::link!("winmm.dll" "system" fn mmioOpenA(pszfilename : windows_core::PSTR, pmmioinfo : LPMMIOINFO, fdwopen : super::DWORD) -> HMMIO);
    unsafe { mmioOpenA(pszfilename.unwrap_or(core::mem::zeroed()) as _, pmmioinfo.unwrap_or(core::mem::zeroed()) as _, fdwopen) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn mmioOpenW(pszfilename: Option<windows_core::PWSTR>, pmmioinfo: Option<LPMMIOINFO>, fdwopen: super::DWORD) -> HMMIO {
    windows_core::link!("winmm.dll" "system" fn mmioOpenW(pszfilename : windows_core::PWSTR, pmmioinfo : LPMMIOINFO, fdwopen : super::DWORD) -> HMMIO);
    unsafe { mmioOpenW(pszfilename.unwrap_or(core::mem::zeroed()) as _, pmmioinfo.unwrap_or(core::mem::zeroed()) as _, fdwopen) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn mmioRead(hmmio: HMMIO, pch: HPSTR, cch: super::LONG) -> super::LONG {
    windows_core::link!("winmm.dll" "system" fn mmioRead(hmmio : HMMIO, pch : HPSTR, cch : super::LONG) -> super::LONG);
    unsafe { mmioRead(hmmio, pch as _, cch) }
}
#[cfg(all(feature = "minwindef", feature = "mmsyscom", feature = "shared"))]
#[inline]
pub unsafe fn mmioRenameA<P0, P1>(pszfilename: P0, psznewfilename: P1, pmmioinfo: Option<LPCMMIOINFO>, fdwrename: super::DWORD) -> super::MMRESULT
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winmm.dll" "system" fn mmioRenameA(pszfilename : windows_core::PCSTR, psznewfilename : windows_core::PCSTR, pmmioinfo : LPCMMIOINFO, fdwrename : super::DWORD) -> super::MMRESULT);
    unsafe { mmioRenameA(pszfilename.param().abi(), psznewfilename.param().abi(), pmmioinfo.unwrap_or(core::mem::zeroed()) as _, fdwrename) }
}
#[cfg(all(feature = "minwindef", feature = "mmsyscom", feature = "shared"))]
#[inline]
pub unsafe fn mmioRenameW<P0, P1>(pszfilename: P0, psznewfilename: P1, pmmioinfo: Option<LPCMMIOINFO>, fdwrename: super::DWORD) -> super::MMRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winmm.dll" "system" fn mmioRenameW(pszfilename : windows_core::PCWSTR, psznewfilename : windows_core::PCWSTR, pmmioinfo : LPCMMIOINFO, fdwrename : super::DWORD) -> super::MMRESULT);
    unsafe { mmioRenameW(pszfilename.param().abi(), psznewfilename.param().abi(), pmmioinfo.unwrap_or(core::mem::zeroed()) as _, fdwrename) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn mmioSeek(hmmio: HMMIO, loffset: super::LONG, iorigin: i32) -> super::LONG {
    windows_core::link!("winmm.dll" "system" fn mmioSeek(hmmio : HMMIO, loffset : super::LONG, iorigin : i32) -> super::LONG);
    unsafe { mmioSeek(hmmio, loffset, iorigin) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn mmioSendMessage(hmmio: HMMIO, umsg: super::UINT, lparam1: Option<super::LPARAM>, lparam2: Option<super::LPARAM>) -> super::LRESULT {
    windows_core::link!("winmm.dll" "system" fn mmioSendMessage(hmmio : HMMIO, umsg : super::UINT, lparam1 : super::LPARAM, lparam2 : super::LPARAM) -> super::LRESULT);
    unsafe { mmioSendMessage(hmmio, umsg, lparam1.unwrap_or(core::mem::zeroed()) as _, lparam2.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
#[inline]
pub unsafe fn mmioSetBuffer(hmmio: HMMIO, pchbuffer: Option<windows_core::PSTR>, cchbuffer: super::LONG, fubuffer: super::UINT) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn mmioSetBuffer(hmmio : HMMIO, pchbuffer : windows_core::PSTR, cchbuffer : super::LONG, fubuffer : super::UINT) -> super::MMRESULT);
    unsafe { mmioSetBuffer(hmmio, pchbuffer.unwrap_or(core::mem::zeroed()) as _, cchbuffer, fubuffer) }
}
#[cfg(all(feature = "minwindef", feature = "mmsyscom", feature = "shared"))]
#[inline]
pub unsafe fn mmioSetInfo(hmmio: HMMIO, pmmioinfo: LPCMMIOINFO, fuinfo: super::UINT) -> super::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn mmioSetInfo(hmmio : HMMIO, pmmioinfo : LPCMMIOINFO, fuinfo : super::UINT) -> super::MMRESULT);
    unsafe { mmioSetInfo(hmmio, pmmioinfo, fuinfo) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn mmioStringToFOURCCA(sz: &windows_core::PCSTR, uflags: super::UINT) -> FOURCC {
    windows_core::link!("winmm.dll" "system" fn mmioStringToFOURCCA(sz : windows_core::PCSTR, uflags : super::UINT) -> FOURCC);
    unsafe { mmioStringToFOURCCA(core::mem::transmute_copy(sz), uflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn mmioStringToFOURCCW(sz: &windows_core::PCWSTR, uflags: super::UINT) -> FOURCC {
    windows_core::link!("winmm.dll" "system" fn mmioStringToFOURCCW(sz : windows_core::PCWSTR, uflags : super::UINT) -> FOURCC);
    unsafe { mmioStringToFOURCCW(core::mem::transmute_copy(sz), uflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn mmioWrite(hmmio: HMMIO, pch: &[u8]) -> super::LONG {
    windows_core::link!("winmm.dll" "system" fn mmioWrite(hmmio : HMMIO, pch : *const i8, cch : super::LONG) -> super::LONG);
    unsafe { mmioWrite(hmmio, core::mem::transmute(pch.as_ptr()), pch.len().try_into().unwrap()) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn sndOpenSound<P0, P1>(eventname: P0, appname: P1, flags: super::INT32, filehandle: super::PHANDLE) -> super::LONG
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-mm-misc-l1-1-1.dll" "system" fn sndOpenSound(eventname : windows_core::PCWSTR, appname : windows_core::PCWSTR, flags : super::INT32, filehandle : super::PHANDLE) -> super::LONG);
    unsafe { sndOpenSound(eventname.param().abi(), appname.param().abi(), flags, filehandle as _) }
}
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
    pub lpszDCISectionName: windows_core::PCWSTR,
    pub lpszDCIAliasName: windows_core::PCWSTR,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DRVCONFIGINFOEX {
    pub dwDCISize: super::DWORD,
    pub lpszDCISectionName: windows_core::PCWSTR,
    pub lpszDCIAliasName: windows_core::PCWSTR,
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
pub type LPMMIOPROC = Option<unsafe extern "system" fn(lpmmioinfo: windows_core::PSTR, umsg: super::UINT, lparam1: super::LPARAM, lparam2: super::LPARAM) -> super::LRESULT>;
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
pub type MMIOPROC = Option<unsafe extern "system" fn(lpmmioinfo: windows_core::PSTR, umsg: super::UINT, lparam1: super::LPARAM, lparam2: super::LPARAM) -> super::LRESULT>;
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
