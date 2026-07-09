#[cfg(all(feature = "Win32_minwindef", feature = "Win32_mmsyscom"))]
#[inline]
pub unsafe fn CloseDriver(hdriver: super::mmsyscom::HDRVR, lparam1: super::minwindef::LPARAM, lparam2: super::minwindef::LPARAM) -> super::minwindef::LRESULT {
    windows_core::link!("winmm.dll" "system" fn CloseDriver(hdriver : super::mmsyscom::HDRVR, lparam1 : super::minwindef::LPARAM, lparam2 : super::minwindef::LPARAM) -> super::minwindef::LRESULT);
    unsafe { CloseDriver(hdriver, lparam1, lparam2) }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_mmsyscom"))]
#[inline]
pub unsafe fn DefDriverProc(dwdriveridentifier: usize, hdrvr: super::mmsyscom::HDRVR, umsg: u32, lparam1: super::minwindef::LPARAM, lparam2: super::minwindef::LPARAM) -> super::minwindef::LRESULT {
    windows_core::link!("winmm.dll" "system" fn DefDriverProc(dwdriveridentifier : usize, hdrvr : super::mmsyscom::HDRVR, umsg : u32, lparam1 : super::minwindef::LPARAM, lparam2 : super::minwindef::LPARAM) -> super::minwindef::LRESULT);
    unsafe { DefDriverProc(dwdriveridentifier, hdrvr, umsg, lparam1, lparam2) }
}
#[cfg(feature = "Win32_mmsyscom")]
#[inline]
pub unsafe fn DriverCallback(dwcallback: usize, dwflags: u32, hdevice: super::mmsyscom::HDRVR, dwmsg: u32, dwuser: usize, dwparam1: usize, dwparam2: usize) -> windows_core::BOOL {
    windows_core::link!("winmm.dll" "system" fn DriverCallback(dwcallback : usize, dwflags : u32, hdevice : super::mmsyscom::HDRVR, dwmsg : u32, dwuser : usize, dwparam1 : usize, dwparam2 : usize) -> windows_core::BOOL);
    unsafe { DriverCallback(dwcallback, dwflags, hdevice, dwmsg, dwuser, dwparam1, dwparam2) }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_mmsyscom"))]
#[inline]
pub unsafe fn DrvGetModuleHandle(hdriver: super::mmsyscom::HDRVR) -> super::minwindef::HMODULE {
    windows_core::link!("winmm.dll" "system" fn DrvGetModuleHandle(hdriver : super::mmsyscom::HDRVR) -> super::minwindef::HMODULE);
    unsafe { DrvGetModuleHandle(hdriver) }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_mmsyscom"))]
#[inline]
pub unsafe fn GetDriverModuleHandle(hdriver: super::mmsyscom::HDRVR) -> super::minwindef::HMODULE {
    windows_core::link!("winmm.dll" "system" fn GetDriverModuleHandle(hdriver : super::mmsyscom::HDRVR) -> super::minwindef::HMODULE);
    unsafe { GetDriverModuleHandle(hdriver) }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_mmsyscom"))]
#[inline]
pub unsafe fn OpenDriver<P0, P1>(szdrivername: P0, szsectionname: P1, lparam2: super::minwindef::LPARAM) -> super::mmsyscom::HDRVR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winmm.dll" "system" fn OpenDriver(szdrivername : windows_core::PCWSTR, szsectionname : windows_core::PCWSTR, lparam2 : super::minwindef::LPARAM) -> super::mmsyscom::HDRVR);
    unsafe { OpenDriver(szdrivername.param().abi(), szsectionname.param().abi(), lparam2) }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_mmsyscom"))]
#[inline]
pub unsafe fn SendDriverMessage(hdriver: super::mmsyscom::HDRVR, message: u32, lparam1: super::minwindef::LPARAM, lparam2: super::minwindef::LPARAM) -> super::minwindef::LRESULT {
    windows_core::link!("winmm.dll" "system" fn SendDriverMessage(hdriver : super::mmsyscom::HDRVR, message : u32, lparam1 : super::minwindef::LPARAM, lparam2 : super::minwindef::LPARAM) -> super::minwindef::LRESULT);
    unsafe { SendDriverMessage(hdriver, message, lparam1, lparam2) }
}
#[cfg(feature = "Win32_mmsyscom")]
#[inline]
pub unsafe fn mmDrvInstall<P1>(hdriver: super::mmsyscom::HDRVR, wszdrventry: P1, drvmessage: DRIVERMSGPROC, wflags: u32) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winmm.dll" "system" fn mmDrvInstall(hdriver : super::mmsyscom::HDRVR, wszdrventry : windows_core::PCWSTR, drvmessage : DRIVERMSGPROC, wflags : u32) -> u32);
    unsafe { mmDrvInstall(hdriver, wszdrventry.param().abi(), drvmessage, wflags) }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_mmsyscom"))]
#[inline]
pub unsafe fn mmioAdvance(hmmio: HMMIO, pmmioinfo: Option<*const MMIOINFO>, fuadvance: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn mmioAdvance(hmmio : HMMIO, pmmioinfo : *const MMIOINFO, fuadvance : u32) -> super::mmsyscom::MMRESULT);
    unsafe { mmioAdvance(hmmio, pmmioinfo.unwrap_or(core::mem::zeroed()) as _, fuadvance) }
}
#[cfg(feature = "Win32_mmsyscom")]
#[inline]
pub unsafe fn mmioAscend(hmmio: HMMIO, pmmcki: *const MMCKINFO, fuascend: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn mmioAscend(hmmio : HMMIO, pmmcki : *const MMCKINFO, fuascend : u32) -> super::mmsyscom::MMRESULT);
    unsafe { mmioAscend(hmmio, pmmcki, fuascend) }
}
#[cfg(feature = "Win32_mmsyscom")]
#[inline]
pub unsafe fn mmioClose(hmmio: HMMIO, fuclose: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn mmioClose(hmmio : HMMIO, fuclose : u32) -> super::mmsyscom::MMRESULT);
    unsafe { mmioClose(hmmio, fuclose) }
}
#[cfg(feature = "Win32_mmsyscom")]
#[inline]
pub unsafe fn mmioCreateChunk(hmmio: HMMIO, pmmcki: *const MMCKINFO, fucreate: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn mmioCreateChunk(hmmio : HMMIO, pmmcki : *const MMCKINFO, fucreate : u32) -> super::mmsyscom::MMRESULT);
    unsafe { mmioCreateChunk(hmmio, pmmcki, fucreate) }
}
#[cfg(feature = "Win32_mmsyscom")]
#[inline]
pub unsafe fn mmioDescend(hmmio: HMMIO, pmmcki: *mut MMCKINFO, pmmckiparent: Option<*const MMCKINFO>, fudescend: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn mmioDescend(hmmio : HMMIO, pmmcki : *mut MMCKINFO, pmmckiparent : *const MMCKINFO, fudescend : u32) -> super::mmsyscom::MMRESULT);
    unsafe { mmioDescend(hmmio, pmmcki as _, pmmckiparent.unwrap_or(core::mem::zeroed()) as _, fudescend) }
}
#[cfg(feature = "Win32_mmsyscom")]
#[inline]
pub unsafe fn mmioFlush(hmmio: HMMIO, fuflush: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn mmioFlush(hmmio : HMMIO, fuflush : u32) -> super::mmsyscom::MMRESULT);
    unsafe { mmioFlush(hmmio, fuflush) }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_mmsyscom"))]
#[inline]
pub unsafe fn mmioGetInfo(hmmio: HMMIO, pmmioinfo: *mut MMIOINFO, fuinfo: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn mmioGetInfo(hmmio : HMMIO, pmmioinfo : *mut MMIOINFO, fuinfo : u32) -> super::mmsyscom::MMRESULT);
    unsafe { mmioGetInfo(hmmio, pmmioinfo as _, fuinfo) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn mmioInstallIOProcA(fccioproc: FOURCC, pioproc: LPMMIOPROC, dwflags: u32) -> LPMMIOPROC {
    windows_core::link!("winmm.dll" "system" fn mmioInstallIOProcA(fccioproc : FOURCC, pioproc : LPMMIOPROC, dwflags : u32) -> LPMMIOPROC);
    unsafe { mmioInstallIOProcA(fccioproc, pioproc, dwflags) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn mmioInstallIOProcW(fccioproc: FOURCC, pioproc: LPMMIOPROC, dwflags: u32) -> LPMMIOPROC {
    windows_core::link!("winmm.dll" "system" fn mmioInstallIOProcW(fccioproc : FOURCC, pioproc : LPMMIOPROC, dwflags : u32) -> LPMMIOPROC);
    unsafe { mmioInstallIOProcW(fccioproc, pioproc, dwflags) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn mmioOpenA(pszfilename: Option<windows_core::PSTR>, pmmioinfo: Option<*mut MMIOINFO>, fdwopen: u32) -> HMMIO {
    windows_core::link!("winmm.dll" "system" fn mmioOpenA(pszfilename : windows_core::PSTR, pmmioinfo : *mut MMIOINFO, fdwopen : u32) -> HMMIO);
    unsafe { mmioOpenA(pszfilename.unwrap_or(core::mem::zeroed()) as _, pmmioinfo.unwrap_or(core::mem::zeroed()) as _, fdwopen) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn mmioOpenW(pszfilename: Option<windows_core::PWSTR>, pmmioinfo: Option<*mut MMIOINFO>, fdwopen: u32) -> HMMIO {
    windows_core::link!("winmm.dll" "system" fn mmioOpenW(pszfilename : windows_core::PWSTR, pmmioinfo : *mut MMIOINFO, fdwopen : u32) -> HMMIO);
    unsafe { mmioOpenW(pszfilename.unwrap_or(core::mem::zeroed()) as _, pmmioinfo.unwrap_or(core::mem::zeroed()) as _, fdwopen) }
}
#[inline]
pub unsafe fn mmioRead(hmmio: HMMIO, pch: &mut [u8]) -> i32 {
    windows_core::link!("winmm.dll" "system" fn mmioRead(hmmio : HMMIO, pch : *mut i8, cch : i32) -> i32);
    unsafe { mmioRead(hmmio, core::mem::transmute(pch.as_ptr()), pch.len().try_into().unwrap()) }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_mmsyscom"))]
#[inline]
pub unsafe fn mmioRenameA<P0, P1>(pszfilename: P0, psznewfilename: P1, pmmioinfo: Option<*const MMIOINFO>, fdwrename: u32) -> super::mmsyscom::MMRESULT
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winmm.dll" "system" fn mmioRenameA(pszfilename : windows_core::PCSTR, psznewfilename : windows_core::PCSTR, pmmioinfo : *const MMIOINFO, fdwrename : u32) -> super::mmsyscom::MMRESULT);
    unsafe { mmioRenameA(pszfilename.param().abi(), psznewfilename.param().abi(), pmmioinfo.unwrap_or(core::mem::zeroed()) as _, fdwrename) }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_mmsyscom"))]
#[inline]
pub unsafe fn mmioRenameW<P0, P1>(pszfilename: P0, psznewfilename: P1, pmmioinfo: Option<*const MMIOINFO>, fdwrename: u32) -> super::mmsyscom::MMRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winmm.dll" "system" fn mmioRenameW(pszfilename : windows_core::PCWSTR, psznewfilename : windows_core::PCWSTR, pmmioinfo : *const MMIOINFO, fdwrename : u32) -> super::mmsyscom::MMRESULT);
    unsafe { mmioRenameW(pszfilename.param().abi(), psznewfilename.param().abi(), pmmioinfo.unwrap_or(core::mem::zeroed()) as _, fdwrename) }
}
#[inline]
pub unsafe fn mmioSeek(hmmio: HMMIO, loffset: i32, iorigin: i32) -> i32 {
    windows_core::link!("winmm.dll" "system" fn mmioSeek(hmmio : HMMIO, loffset : i32, iorigin : i32) -> i32);
    unsafe { mmioSeek(hmmio, loffset, iorigin) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn mmioSendMessage(hmmio: HMMIO, umsg: u32, lparam1: Option<super::minwindef::LPARAM>, lparam2: Option<super::minwindef::LPARAM>) -> super::minwindef::LRESULT {
    windows_core::link!("winmm.dll" "system" fn mmioSendMessage(hmmio : HMMIO, umsg : u32, lparam1 : super::minwindef::LPARAM, lparam2 : super::minwindef::LPARAM) -> super::minwindef::LRESULT);
    unsafe { mmioSendMessage(hmmio, umsg, lparam1.unwrap_or(core::mem::zeroed()) as _, lparam2.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_mmsyscom")]
#[inline]
pub unsafe fn mmioSetBuffer(hmmio: HMMIO, pchbuffer: Option<&mut [u8]>, fubuffer: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn mmioSetBuffer(hmmio : HMMIO, pchbuffer : windows_core::PSTR, cchbuffer : i32, fubuffer : u32) -> super::mmsyscom::MMRESULT);
    unsafe { mmioSetBuffer(hmmio, core::mem::transmute(pchbuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pchbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), fubuffer) }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_mmsyscom"))]
#[inline]
pub unsafe fn mmioSetInfo(hmmio: HMMIO, pmmioinfo: *const MMIOINFO, fuinfo: u32) -> super::mmsyscom::MMRESULT {
    windows_core::link!("winmm.dll" "system" fn mmioSetInfo(hmmio : HMMIO, pmmioinfo : *const MMIOINFO, fuinfo : u32) -> super::mmsyscom::MMRESULT);
    unsafe { mmioSetInfo(hmmio, pmmioinfo, fuinfo) }
}
#[inline]
pub unsafe fn mmioStringToFOURCCA<P0>(sz: P0, uflags: u32) -> FOURCC
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("winmm.dll" "system" fn mmioStringToFOURCCA(sz : windows_core::PCSTR, uflags : u32) -> FOURCC);
    unsafe { mmioStringToFOURCCA(sz.param().abi(), uflags) }
}
#[inline]
pub unsafe fn mmioStringToFOURCCW<P0>(sz: P0, uflags: u32) -> FOURCC
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("winmm.dll" "system" fn mmioStringToFOURCCW(sz : windows_core::PCWSTR, uflags : u32) -> FOURCC);
    unsafe { mmioStringToFOURCCW(sz.param().abi(), uflags) }
}
#[inline]
pub unsafe fn mmioWrite(hmmio: HMMIO, pch: &[u8]) -> i32 {
    windows_core::link!("winmm.dll" "system" fn mmioWrite(hmmio : HMMIO, pch : *const i8, cch : i32) -> i32);
    unsafe { mmioWrite(hmmio, core::mem::transmute(pch.as_ptr()), pch.len().try_into().unwrap()) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn sndOpenSound<P0, P1>(eventname: P0, appname: P1, flags: i32, filehandle: *mut super::winnt::HANDLE) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-mm-misc-l1-1-1.dll" "system" fn sndOpenSound(eventname : windows_core::PCWSTR, appname : windows_core::PCWSTR, flags : i32, filehandle : *mut super::winnt::HANDLE) -> i32);
    unsafe { sndOpenSound(eventname.param().abi(), appname.param().abi(), flags, filehandle as _) }
}
pub const CFSEPCHAR: u32 = 43;
pub type DRIVERMSGPROC = Option<unsafe extern "system" fn(param0: u32, param1: u32, param2: usize, param3: usize, param4: usize) -> u32>;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_mmsyscom"))]
pub type DRIVERPROC = Option<unsafe extern "system" fn(param0: usize, param1: super::mmsyscom::HDRVR, param2: u32, param3: super::minwindef::LPARAM, param4: super::minwindef::LPARAM) -> super::minwindef::LRESULT>;
pub const DRVCNF_CANCEL: u32 = 0;
pub const DRVCNF_OK: u32 = 1;
pub const DRVCNF_RESTART: u32 = 2;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct DRVCONFIGINFO {
    pub dwDCISize: u32,
    pub lpszDCISectionName: windows_core::PCWSTR,
    pub lpszDCIAliasName: windows_core::PCWSTR,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct DRVCONFIGINFOEX {
    pub dwDCISize: u32,
    pub lpszDCISectionName: windows_core::PCWSTR,
    pub lpszDCIAliasName: windows_core::PCWSTR,
    pub dnDevNode: u32,
}
pub const DRV_CANCEL: u32 = 0;
pub const DRV_CLOSE: u32 = 4;
pub const DRV_CONFIGURE: u32 = 7;
pub const DRV_DISABLE: u32 = 5;
pub const DRV_ENABLE: u32 = 2;
pub const DRV_EXITSESSION: u32 = 11;
pub const DRV_FREE: u32 = 6;
pub const DRV_INSTALL: u32 = 9;
pub const DRV_LOAD: u32 = 1;
pub const DRV_MCI_FIRST: u32 = 2048;
pub const DRV_MCI_LAST: u32 = 6143;
pub const DRV_OK: u32 = 1;
pub const DRV_OPEN: u32 = 3;
pub const DRV_POWER: u32 = 15;
pub const DRV_QUERYCONFIGURE: u32 = 8;
pub const DRV_REMOVE: u32 = 10;
pub const DRV_RESERVED: u32 = 2048;
pub const DRV_RESTART: u32 = 2;
pub const DRV_USER: u32 = 16384;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct FOURCC(pub u32);
pub const FOURCC_DOS: u32 = 542330692;
pub const FOURCC_LIST: u32 = 1414744396;
pub const FOURCC_MEM: u32 = 541934925;
pub const FOURCC_RIFF: u32 = 1179011410;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HMMIO(pub *mut core::ffi::c_void);
impl HMMIO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for HMMIO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HPSTR(pub *mut i8);
impl HPSTR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for HPSTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPCMMCKINFO(pub *const MMCKINFO);
impl LPCMMCKINFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPCMMCKINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_minwindef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPCMMIOINFO(pub *const MMIOINFO);
#[cfg(feature = "Win32_minwindef")]
impl LPCMMIOINFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_minwindef")]
impl Default for LPCMMIOINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPDRVCONFIGINFO(pub *mut DRVCONFIGINFO);
impl LPDRVCONFIGINFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPDRVCONFIGINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPDRVCONFIGINFOEX(pub *mut DRVCONFIGINFOEX);
impl LPDRVCONFIGINFOEX {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPDRVCONFIGINFOEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPMMCKINFO(pub *mut MMCKINFO);
impl LPMMCKINFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPMMCKINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_minwindef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPMMIOINFO(pub *mut MMIOINFO);
#[cfg(feature = "Win32_minwindef")]
impl LPMMIOINFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_minwindef")]
impl Default for LPMMIOINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_minwindef")]
pub type LPMMIOPROC = Option<unsafe extern "system" fn(lpmmioinfo: windows_core::PCSTR, umsg: u32, lparam1: super::minwindef::LPARAM, lparam2: super::minwindef::LPARAM) -> super::minwindef::LRESULT>;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct MMCKINFO {
    pub ckid: FOURCC,
    pub cksize: u32,
    pub fccType: FOURCC,
    pub dwDataOffset: u32,
    pub dwFlags: u32,
}
pub const MMIOERR_ACCESSDENIED: u32 = 268;
pub const MMIOERR_BASE: u32 = 256;
pub const MMIOERR_CANNOTCLOSE: u32 = 260;
pub const MMIOERR_CANNOTEXPAND: u32 = 264;
pub const MMIOERR_CANNOTOPEN: u32 = 259;
pub const MMIOERR_CANNOTREAD: u32 = 261;
pub const MMIOERR_CANNOTSEEK: u32 = 263;
pub const MMIOERR_CANNOTWRITE: u32 = 262;
pub const MMIOERR_CHUNKNOTFOUND: u32 = 265;
pub const MMIOERR_FILENOTFOUND: u32 = 257;
pub const MMIOERR_INVALIDFILE: u32 = 272;
pub const MMIOERR_NETWORKERROR: u32 = 270;
pub const MMIOERR_OUTOFMEMORY: u32 = 258;
pub const MMIOERR_PATHNOTFOUND: u32 = 267;
pub const MMIOERR_SHARINGVIOLATION: u32 = 269;
pub const MMIOERR_TOOMANYOPENFILES: u32 = 271;
pub const MMIOERR_UNBUFFERED: u32 = 266;
#[repr(C, packed(1))]
#[cfg(feature = "Win32_minwindef")]
#[derive(Clone, Copy)]
pub struct MMIOINFO {
    pub dwFlags: u32,
    pub fccIOProc: FOURCC,
    pub pIOProc: LPMMIOPROC,
    pub wErrorRet: u32,
    pub htask: super::minwindef::HTASK,
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
#[cfg(feature = "Win32_minwindef")]
impl Default for MMIOINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MMIOM_CLOSE: u32 = 4;
pub const MMIOM_OPEN: u32 = 3;
pub const MMIOM_READ: u32 = 0;
pub const MMIOM_RENAME: u32 = 6;
pub const MMIOM_SEEK: u32 = 2;
pub const MMIOM_USER: u32 = 32768;
pub const MMIOM_WRITE: u32 = 1;
pub const MMIOM_WRITEFLUSH: u32 = 5;
#[cfg(feature = "Win32_minwindef")]
pub type MMIOPROC = Option<unsafe extern "system" fn(lpmmioinfo: windows_core::PCSTR, umsg: u32, lparam1: super::minwindef::LPARAM, lparam2: super::minwindef::LPARAM) -> super::minwindef::LRESULT>;
pub const MMIO_ALLOCBUF: u32 = 65536;
pub const MMIO_COMPAT: u32 = 0;
pub const MMIO_CREATE: u32 = 4096;
pub const MMIO_CREATELIST: u32 = 64;
pub const MMIO_CREATERIFF: u32 = 32;
pub const MMIO_DEFAULTBUFFER: u32 = 8192;
pub const MMIO_DELETE: u32 = 512;
pub const MMIO_DENYNONE: u32 = 64;
pub const MMIO_DENYREAD: u32 = 48;
pub const MMIO_DENYWRITE: u32 = 32;
pub const MMIO_DIRTY: u32 = 268435456;
pub const MMIO_EMPTYBUF: u32 = 16;
pub const MMIO_EXCLUSIVE: u32 = 16;
pub const MMIO_EXIST: u32 = 16384;
pub const MMIO_FHOPEN: u32 = 16;
pub const MMIO_FINDCHUNK: u32 = 16;
pub const MMIO_FINDLIST: u32 = 64;
pub const MMIO_FINDPROC: u32 = 262144;
pub const MMIO_FINDRIFF: u32 = 32;
pub const MMIO_GETTEMP: u32 = 131072;
pub const MMIO_GLOBALPROC: u32 = 268435456;
pub const MMIO_INSTALLPROC: u32 = 65536;
pub const MMIO_PARSE: u32 = 256;
pub const MMIO_READ: u32 = 0;
pub const MMIO_READWRITE: u32 = 2;
pub const MMIO_REMOVEPROC: u32 = 131072;
pub const MMIO_RWMODE: u32 = 3;
pub const MMIO_SHAREMODE: u32 = 112;
pub const MMIO_TOUPPER: u32 = 16;
pub const MMIO_UNICODEPROC: u32 = 16777216;
pub const MMIO_WRITE: u32 = 1;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NPDRVCONFIGINFO(pub *mut DRVCONFIGINFO);
impl NPDRVCONFIGINFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for NPDRVCONFIGINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NPDRVCONFIGINFOEX(pub *mut DRVCONFIGINFOEX);
impl NPDRVCONFIGINFOEX {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for NPDRVCONFIGINFOEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NPMMCKINFO(pub *mut MMCKINFO);
impl NPMMCKINFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for NPMMCKINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_minwindef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NPMMIOINFO(pub *mut MMIOINFO);
#[cfg(feature = "Win32_minwindef")]
impl NPMMIOINFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_minwindef")]
impl Default for NPMMIOINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDRVCONFIGINFO(pub *mut DRVCONFIGINFO);
impl PDRVCONFIGINFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDRVCONFIGINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDRVCONFIGINFOEX(pub *mut DRVCONFIGINFOEX);
impl PDRVCONFIGINFOEX {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDRVCONFIGINFOEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PMMCKINFO(pub *mut MMCKINFO);
impl PMMCKINFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PMMCKINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_minwindef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PMMIOINFO(pub *mut MMIOINFO);
#[cfg(feature = "Win32_minwindef")]
impl PMMIOINFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_minwindef")]
impl Default for PMMIOINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SEEK_CUR: u32 = 1;
pub const SEEK_END: u32 = 2;
pub const SEEK_SET: u32 = 0;
