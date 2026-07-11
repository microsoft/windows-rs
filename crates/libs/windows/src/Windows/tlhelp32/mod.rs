#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CreateToolhelp32Snapshot(dwflags: u32, th32processid: u32) -> super::winnt::HANDLE {
    windows_core::link!("kernel32.dll" "system" fn CreateToolhelp32Snapshot(dwflags : u32, th32processid : u32) -> super::winnt::HANDLE);
    unsafe { CreateToolhelp32Snapshot(dwflags, th32processid) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn Heap32First(lphe: *mut HEAPENTRY32, th32processid: u32, th32heapid: usize) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn Heap32First(lphe : *mut HEAPENTRY32, th32processid : u32, th32heapid : usize) -> windows_core::BOOL);
    unsafe { Heap32First(lphe as _, th32processid, th32heapid) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn Heap32ListFirst(hsnapshot: super::winnt::HANDLE, lphl: *mut HEAPLIST32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn Heap32ListFirst(hsnapshot : super::winnt::HANDLE, lphl : *mut HEAPLIST32) -> windows_core::BOOL);
    unsafe { Heap32ListFirst(hsnapshot, lphl as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn Heap32ListNext(hsnapshot: super::winnt::HANDLE, lphl: *mut HEAPLIST32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn Heap32ListNext(hsnapshot : super::winnt::HANDLE, lphl : *mut HEAPLIST32) -> windows_core::BOOL);
    unsafe { Heap32ListNext(hsnapshot, lphl as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn Heap32Next(lphe: *mut HEAPENTRY32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn Heap32Next(lphe : *mut HEAPENTRY32) -> windows_core::BOOL);
    unsafe { Heap32Next(lphe as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn Module32First(hsnapshot: super::winnt::HANDLE, lpme: *mut MODULEENTRY32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn Module32First(hsnapshot : super::winnt::HANDLE, lpme : *mut MODULEENTRY32) -> windows_core::BOOL);
    unsafe { Module32First(hsnapshot, lpme as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn Module32FirstW(hsnapshot: super::winnt::HANDLE, lpme: *mut MODULEENTRY32W) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn Module32FirstW(hsnapshot : super::winnt::HANDLE, lpme : *mut MODULEENTRY32W) -> windows_core::BOOL);
    unsafe { Module32FirstW(hsnapshot, lpme as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn Module32Next(hsnapshot: super::winnt::HANDLE, lpme: *mut MODULEENTRY32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn Module32Next(hsnapshot : super::winnt::HANDLE, lpme : *mut MODULEENTRY32) -> windows_core::BOOL);
    unsafe { Module32Next(hsnapshot, lpme as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn Module32NextW(hsnapshot: super::winnt::HANDLE, lpme: *mut MODULEENTRY32W) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn Module32NextW(hsnapshot : super::winnt::HANDLE, lpme : *mut MODULEENTRY32W) -> windows_core::BOOL);
    unsafe { Module32NextW(hsnapshot, lpme as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn Process32First(hsnapshot: super::winnt::HANDLE, lppe: *mut PROCESSENTRY32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn Process32First(hsnapshot : super::winnt::HANDLE, lppe : *mut PROCESSENTRY32) -> windows_core::BOOL);
    unsafe { Process32First(hsnapshot, lppe as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn Process32FirstW(hsnapshot: super::winnt::HANDLE, lppe: *mut PROCESSENTRY32W) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn Process32FirstW(hsnapshot : super::winnt::HANDLE, lppe : *mut PROCESSENTRY32W) -> windows_core::BOOL);
    unsafe { Process32FirstW(hsnapshot, lppe as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn Process32Next(hsnapshot: super::winnt::HANDLE, lppe: *mut PROCESSENTRY32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn Process32Next(hsnapshot : super::winnt::HANDLE, lppe : *mut PROCESSENTRY32) -> windows_core::BOOL);
    unsafe { Process32Next(hsnapshot, lppe as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn Process32NextW(hsnapshot: super::winnt::HANDLE, lppe: *mut PROCESSENTRY32W) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn Process32NextW(hsnapshot : super::winnt::HANDLE, lppe : *mut PROCESSENTRY32W) -> windows_core::BOOL);
    unsafe { Process32NextW(hsnapshot, lppe as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn Thread32First(hsnapshot: super::winnt::HANDLE, lpte: *mut THREADENTRY32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn Thread32First(hsnapshot : super::winnt::HANDLE, lpte : *mut THREADENTRY32) -> windows_core::BOOL);
    unsafe { Thread32First(hsnapshot, lpte as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn Thread32Next(hsnapshot: super::winnt::HANDLE, lpte: *mut THREADENTRY32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn Thread32Next(hsnapshot : super::winnt::HANDLE, lpte : *mut THREADENTRY32) -> windows_core::BOOL);
    unsafe { Thread32Next(hsnapshot, lpte as _) }
}
#[inline]
pub unsafe fn Toolhelp32ReadProcessMemory(th32processid: u32, lpbaseaddress: *const core::ffi::c_void, lpbuffer: *mut core::ffi::c_void, cbread: usize, lpnumberofbytesread: *mut usize) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn Toolhelp32ReadProcessMemory(th32processid : u32, lpbaseaddress : *const core::ffi::c_void, lpbuffer : *mut core::ffi::c_void, cbread : usize, lpnumberofbytesread : *mut usize) -> windows_core::BOOL);
    unsafe { Toolhelp32ReadProcessMemory(th32processid, lpbaseaddress, lpbuffer as _, cbread, lpnumberofbytesread as _) }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HEAPENTRY32 {
    pub dwSize: usize,
    pub hHandle: super::winnt::HANDLE,
    pub dwAddress: usize,
    pub dwBlockSize: usize,
    pub dwFlags: u32,
    pub dwLockCount: u32,
    pub dwResvd: u32,
    pub th32ProcessID: u32,
    pub th32HeapID: usize,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HEAPLIST32 {
    pub dwSize: usize,
    pub th32ProcessID: u32,
    pub th32HeapID: usize,
    pub dwFlags: u32,
}
pub const HF32_DEFAULT: u32 = 1;
pub const HF32_SHARED: u32 = 2;
pub const LF32_FIXED: u32 = 1;
pub const LF32_FREE: u32 = 2;
pub const LF32_MOVEABLE: u32 = 4;
#[cfg(feature = "winnt")]
pub type LPHEAPENTRY32 = *mut HEAPENTRY32;
pub type LPHEAPLIST32 = *mut HEAPLIST32;
#[cfg(feature = "minwindef")]
pub type LPMODULEENTRY32 = *mut MODULEENTRY32;
#[cfg(feature = "minwindef")]
pub type LPMODULEENTRY32W = *mut MODULEENTRY32W;
pub type LPPROCESSENTRY32 = *mut PROCESSENTRY32;
pub type LPPROCESSENTRY32W = *mut PROCESSENTRY32W;
pub type LPTHREADENTRY32 = *mut THREADENTRY32;
pub const MAX_MODULE_NAME32: u32 = 255;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MODULEENTRY32 {
    pub dwSize: u32,
    pub th32ModuleID: u32,
    pub th32ProcessID: u32,
    pub GlblcntUsage: u32,
    pub ProccntUsage: u32,
    pub modBaseAddr: *mut u8,
    pub modBaseSize: u32,
    pub hModule: super::minwindef::HMODULE,
    pub szModule: [i8; 256],
    pub szExePath: [i8; 260],
}
#[cfg(feature = "minwindef")]
impl Default for MODULEENTRY32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MODULEENTRY32W {
    pub dwSize: u32,
    pub th32ModuleID: u32,
    pub th32ProcessID: u32,
    pub GlblcntUsage: u32,
    pub ProccntUsage: u32,
    pub modBaseAddr: *mut u8,
    pub modBaseSize: u32,
    pub hModule: super::minwindef::HMODULE,
    pub szModule: [u16; 256],
    pub szExePath: [u16; 260],
}
#[cfg(feature = "minwindef")]
impl Default for MODULEENTRY32W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "winnt")]
pub type PHEAPENTRY32 = *mut HEAPENTRY32;
pub type PHEAPLIST32 = *mut HEAPLIST32;
#[cfg(feature = "minwindef")]
pub type PMODULEENTRY32 = *mut MODULEENTRY32;
#[cfg(feature = "minwindef")]
pub type PMODULEENTRY32W = *mut MODULEENTRY32W;
pub type PPROCESSENTRY32 = *mut PROCESSENTRY32;
pub type PPROCESSENTRY32W = *mut PROCESSENTRY32W;
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
pub type PTHREADENTRY32 = *mut THREADENTRY32;
pub const TH32CS_INHERIT: u32 = 2147483648;
pub const TH32CS_SNAPALL: u32 = 15;
pub const TH32CS_SNAPHEAPLIST: u32 = 1;
pub const TH32CS_SNAPMODULE: u32 = 8;
pub const TH32CS_SNAPMODULE32: u32 = 16;
pub const TH32CS_SNAPPROCESS: u32 = 2;
pub const TH32CS_SNAPTHREAD: u32 = 4;
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
