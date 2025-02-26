#[inline]
pub unsafe fn CreateToolhelp32Snapshot(dwflags: CREATE_TOOLHELP_SNAPSHOT_FLAGS, th32processid: u32) -> windows_core::Result<super::super::super::Foundation::HANDLE> {
    windows_link::link!("kernel32.dll" "system" fn CreateToolhelp32Snapshot(dwflags : CREATE_TOOLHELP_SNAPSHOT_FLAGS, th32processid : u32) -> super::super::super::Foundation:: HANDLE);
    let result__ = unsafe { CreateToolhelp32Snapshot(dwflags, th32processid) };
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn Heap32First(lphe: *mut HEAPENTRY32, th32processid: u32, th32heapid: usize) -> windows_core::Result<()> {
    windows_link::link!("kernel32.dll" "system" fn Heap32First(lphe : *mut HEAPENTRY32, th32processid : u32, th32heapid : usize) -> windows_core::BOOL);
    unsafe { Heap32First(lphe as _, th32processid, th32heapid).ok() }
}
#[inline]
pub unsafe fn Heap32ListFirst(hsnapshot: super::super::super::Foundation::HANDLE, lphl: *mut HEAPLIST32) -> windows_core::Result<()> {
    windows_link::link!("kernel32.dll" "system" fn Heap32ListFirst(hsnapshot : super::super::super::Foundation:: HANDLE, lphl : *mut HEAPLIST32) -> windows_core::BOOL);
    unsafe { Heap32ListFirst(hsnapshot, lphl as _).ok() }
}
#[inline]
pub unsafe fn Heap32ListNext(hsnapshot: super::super::super::Foundation::HANDLE, lphl: *mut HEAPLIST32) -> windows_core::Result<()> {
    windows_link::link!("kernel32.dll" "system" fn Heap32ListNext(hsnapshot : super::super::super::Foundation:: HANDLE, lphl : *mut HEAPLIST32) -> windows_core::BOOL);
    unsafe { Heap32ListNext(hsnapshot, lphl as _).ok() }
}
#[inline]
pub unsafe fn Heap32Next(lphe: *mut HEAPENTRY32) -> windows_core::Result<()> {
    windows_link::link!("kernel32.dll" "system" fn Heap32Next(lphe : *mut HEAPENTRY32) -> windows_core::BOOL);
    unsafe { Heap32Next(lphe as _).ok() }
}
#[inline]
pub unsafe fn Module32First(hsnapshot: super::super::super::Foundation::HANDLE, lpme: *mut MODULEENTRY32) -> windows_core::Result<()> {
    windows_link::link!("kernel32.dll" "system" fn Module32First(hsnapshot : super::super::super::Foundation:: HANDLE, lpme : *mut MODULEENTRY32) -> windows_core::BOOL);
    unsafe { Module32First(hsnapshot, lpme as _).ok() }
}
#[inline]
pub unsafe fn Module32FirstW(hsnapshot: super::super::super::Foundation::HANDLE, lpme: *mut MODULEENTRY32W) -> windows_core::Result<()> {
    windows_link::link!("kernel32.dll" "system" fn Module32FirstW(hsnapshot : super::super::super::Foundation:: HANDLE, lpme : *mut MODULEENTRY32W) -> windows_core::BOOL);
    unsafe { Module32FirstW(hsnapshot, lpme as _).ok() }
}
#[inline]
pub unsafe fn Module32Next(hsnapshot: super::super::super::Foundation::HANDLE, lpme: *mut MODULEENTRY32) -> windows_core::Result<()> {
    windows_link::link!("kernel32.dll" "system" fn Module32Next(hsnapshot : super::super::super::Foundation:: HANDLE, lpme : *mut MODULEENTRY32) -> windows_core::BOOL);
    unsafe { Module32Next(hsnapshot, lpme as _).ok() }
}
#[inline]
pub unsafe fn Module32NextW(hsnapshot: super::super::super::Foundation::HANDLE, lpme: *mut MODULEENTRY32W) -> windows_core::Result<()> {
    windows_link::link!("kernel32.dll" "system" fn Module32NextW(hsnapshot : super::super::super::Foundation:: HANDLE, lpme : *mut MODULEENTRY32W) -> windows_core::BOOL);
    unsafe { Module32NextW(hsnapshot, lpme as _).ok() }
}
#[inline]
pub unsafe fn Process32First(hsnapshot: super::super::super::Foundation::HANDLE, lppe: *mut PROCESSENTRY32) -> windows_core::Result<()> {
    windows_link::link!("kernel32.dll" "system" fn Process32First(hsnapshot : super::super::super::Foundation:: HANDLE, lppe : *mut PROCESSENTRY32) -> windows_core::BOOL);
    unsafe { Process32First(hsnapshot, lppe as _).ok() }
}
#[inline]
pub unsafe fn Process32FirstW(hsnapshot: super::super::super::Foundation::HANDLE, lppe: *mut PROCESSENTRY32W) -> windows_core::Result<()> {
    windows_link::link!("kernel32.dll" "system" fn Process32FirstW(hsnapshot : super::super::super::Foundation:: HANDLE, lppe : *mut PROCESSENTRY32W) -> windows_core::BOOL);
    unsafe { Process32FirstW(hsnapshot, lppe as _).ok() }
}
#[inline]
pub unsafe fn Process32Next(hsnapshot: super::super::super::Foundation::HANDLE, lppe: *mut PROCESSENTRY32) -> windows_core::Result<()> {
    windows_link::link!("kernel32.dll" "system" fn Process32Next(hsnapshot : super::super::super::Foundation:: HANDLE, lppe : *mut PROCESSENTRY32) -> windows_core::BOOL);
    unsafe { Process32Next(hsnapshot, lppe as _).ok() }
}
#[inline]
pub unsafe fn Process32NextW(hsnapshot: super::super::super::Foundation::HANDLE, lppe: *mut PROCESSENTRY32W) -> windows_core::Result<()> {
    windows_link::link!("kernel32.dll" "system" fn Process32NextW(hsnapshot : super::super::super::Foundation:: HANDLE, lppe : *mut PROCESSENTRY32W) -> windows_core::BOOL);
    unsafe { Process32NextW(hsnapshot, lppe as _).ok() }
}
#[inline]
pub unsafe fn Thread32First(hsnapshot: super::super::super::Foundation::HANDLE, lpte: *mut THREADENTRY32) -> windows_core::Result<()> {
    windows_link::link!("kernel32.dll" "system" fn Thread32First(hsnapshot : super::super::super::Foundation:: HANDLE, lpte : *mut THREADENTRY32) -> windows_core::BOOL);
    unsafe { Thread32First(hsnapshot, lpte as _).ok() }
}
#[inline]
pub unsafe fn Thread32Next(hsnapshot: super::super::super::Foundation::HANDLE, lpte: *mut THREADENTRY32) -> windows_core::Result<()> {
    windows_link::link!("kernel32.dll" "system" fn Thread32Next(hsnapshot : super::super::super::Foundation:: HANDLE, lpte : *mut THREADENTRY32) -> windows_core::BOOL);
    unsafe { Thread32Next(hsnapshot, lpte as _).ok() }
}
#[inline]
pub unsafe fn Toolhelp32ReadProcessMemory(th32processid: u32, lpbaseaddress: *const core::ffi::c_void, lpbuffer: *mut core::ffi::c_void, cbread: usize, lpnumberofbytesread: *mut usize) -> windows_core::BOOL {
    windows_link::link!("kernel32.dll" "system" fn Toolhelp32ReadProcessMemory(th32processid : u32, lpbaseaddress : *const core::ffi::c_void, lpbuffer : *mut core::ffi::c_void, cbread : usize, lpnumberofbytesread : *mut usize) -> windows_core::BOOL);
    unsafe { Toolhelp32ReadProcessMemory(th32processid, lpbaseaddress, lpbuffer as _, cbread, lpnumberofbytesread as _) }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CREATE_TOOLHELP_SNAPSHOT_FLAGS(pub u32);
impl CREATE_TOOLHELP_SNAPSHOT_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CREATE_TOOLHELP_SNAPSHOT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CREATE_TOOLHELP_SNAPSHOT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CREATE_TOOLHELP_SNAPSHOT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CREATE_TOOLHELP_SNAPSHOT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CREATE_TOOLHELP_SNAPSHOT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HEAPENTRY32 {
    pub dwSize: usize,
    pub hHandle: super::super::super::Foundation::HANDLE,
    pub dwAddress: usize,
    pub dwBlockSize: usize,
    pub dwFlags: HEAPENTRY32_FLAGS,
    pub dwLockCount: u32,
    pub dwResvd: u32,
    pub th32ProcessID: u32,
    pub th32HeapID: usize,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HEAPENTRY32_FLAGS(pub u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HEAPLIST32 {
    pub dwSize: usize,
    pub th32ProcessID: u32,
    pub th32HeapID: usize,
    pub dwFlags: u32,
}
pub const HF32_DEFAULT: u32 = 1u32;
pub const HF32_SHARED: u32 = 2u32;
pub const LF32_FIXED: HEAPENTRY32_FLAGS = HEAPENTRY32_FLAGS(1u32);
pub const LF32_FREE: HEAPENTRY32_FLAGS = HEAPENTRY32_FLAGS(2u32);
pub const LF32_MOVEABLE: HEAPENTRY32_FLAGS = HEAPENTRY32_FLAGS(4u32);
pub const MAX_MODULE_NAME32: u32 = 255u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MODULEENTRY32 {
    pub dwSize: u32,
    pub th32ModuleID: u32,
    pub th32ProcessID: u32,
    pub GlblcntUsage: u32,
    pub ProccntUsage: u32,
    pub modBaseAddr: *mut u8,
    pub modBaseSize: u32,
    pub hModule: super::super::super::Foundation::HMODULE,
    pub szModule: [i8; 256],
    pub szExePath: [i8; 260],
}
impl Default for MODULEENTRY32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MODULEENTRY32W {
    pub dwSize: u32,
    pub th32ModuleID: u32,
    pub th32ProcessID: u32,
    pub GlblcntUsage: u32,
    pub ProccntUsage: u32,
    pub modBaseAddr: *mut u8,
    pub modBaseSize: u32,
    pub hModule: super::super::super::Foundation::HMODULE,
    pub szModule: [u16; 256],
    pub szExePath: [u16; 260],
}
impl Default for MODULEENTRY32W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PROCESSENTRY32 {
    pub dwSize: u32,
    pub cntUsage: u32,
    pub th32ProcessID: u32,
    pub th32DefaultHeapID: usize,
    pub th32ModuleID: u32,
    pub cntThreads: u32,
    pub th32ParentProcessID: u32,
    pub pcPriClassBase: i32,
    pub dwFlags: u32,
    pub szExeFile: [i8; 260],
}
impl Default for PROCESSENTRY32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PROCESSENTRY32W {
    pub dwSize: u32,
    pub cntUsage: u32,
    pub th32ProcessID: u32,
    pub th32DefaultHeapID: usize,
    pub th32ModuleID: u32,
    pub cntThreads: u32,
    pub th32ParentProcessID: u32,
    pub pcPriClassBase: i32,
    pub dwFlags: u32,
    pub szExeFile: [u16; 260],
}
impl Default for PROCESSENTRY32W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const TH32CS_INHERIT: CREATE_TOOLHELP_SNAPSHOT_FLAGS = CREATE_TOOLHELP_SNAPSHOT_FLAGS(2147483648u32);
pub const TH32CS_SNAPALL: CREATE_TOOLHELP_SNAPSHOT_FLAGS = CREATE_TOOLHELP_SNAPSHOT_FLAGS(15u32);
pub const TH32CS_SNAPHEAPLIST: CREATE_TOOLHELP_SNAPSHOT_FLAGS = CREATE_TOOLHELP_SNAPSHOT_FLAGS(1u32);
pub const TH32CS_SNAPMODULE: CREATE_TOOLHELP_SNAPSHOT_FLAGS = CREATE_TOOLHELP_SNAPSHOT_FLAGS(8u32);
pub const TH32CS_SNAPMODULE32: CREATE_TOOLHELP_SNAPSHOT_FLAGS = CREATE_TOOLHELP_SNAPSHOT_FLAGS(16u32);
pub const TH32CS_SNAPPROCESS: CREATE_TOOLHELP_SNAPSHOT_FLAGS = CREATE_TOOLHELP_SNAPSHOT_FLAGS(2u32);
pub const TH32CS_SNAPTHREAD: CREATE_TOOLHELP_SNAPSHOT_FLAGS = CREATE_TOOLHELP_SNAPSHOT_FLAGS(4u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct THREADENTRY32 {
    pub dwSize: u32,
    pub cntUsage: u32,
    pub th32ThreadID: u32,
    pub th32OwnerProcessID: u32,
    pub tpBasePri: i32,
    pub tpDeltaPri: i32,
    pub dwFlags: u32,
}
