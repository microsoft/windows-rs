#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CreateToolhelp32Snapshot(dwflags: super::DWORD, th32processid: super::DWORD) -> super::HANDLE {
    windows_core::link!("kernel32.dll" "system" fn CreateToolhelp32Snapshot(dwflags : super::DWORD, th32processid : super::DWORD) -> super::HANDLE);
    unsafe { CreateToolhelp32Snapshot(dwflags, th32processid) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn Heap32First(lphe: LPHEAPENTRY32, th32processid: super::DWORD, th32heapid: super::ULONG_PTR) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn Heap32First(lphe : LPHEAPENTRY32, th32processid : super::DWORD, th32heapid : super::ULONG_PTR) -> windows_core::BOOL);
    unsafe { Heap32First(lphe as _, th32processid, th32heapid) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn Heap32ListFirst(hsnapshot: super::HANDLE, lphl: LPHEAPLIST32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn Heap32ListFirst(hsnapshot : super::HANDLE, lphl : LPHEAPLIST32) -> windows_core::BOOL);
    unsafe { Heap32ListFirst(hsnapshot as _, lphl as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn Heap32ListNext(hsnapshot: super::HANDLE, lphl: LPHEAPLIST32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn Heap32ListNext(hsnapshot : super::HANDLE, lphl : LPHEAPLIST32) -> windows_core::BOOL);
    unsafe { Heap32ListNext(hsnapshot as _, lphl as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn Heap32Next(lphe: LPHEAPENTRY32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn Heap32Next(lphe : LPHEAPENTRY32) -> windows_core::BOOL);
    unsafe { Heap32Next(lphe as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn Module32First(hsnapshot: super::HANDLE, lpme: LPMODULEENTRY32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn Module32First(hsnapshot : super::HANDLE, lpme : LPMODULEENTRY32) -> windows_core::BOOL);
    unsafe { Module32First(hsnapshot as _, lpme as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn Module32FirstW(hsnapshot: super::HANDLE, lpme: LPMODULEENTRY32W) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn Module32FirstW(hsnapshot : super::HANDLE, lpme : LPMODULEENTRY32W) -> windows_core::BOOL);
    unsafe { Module32FirstW(hsnapshot as _, lpme as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn Module32Next(hsnapshot: super::HANDLE, lpme: LPMODULEENTRY32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn Module32Next(hsnapshot : super::HANDLE, lpme : LPMODULEENTRY32) -> windows_core::BOOL);
    unsafe { Module32Next(hsnapshot as _, lpme as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn Module32NextW(hsnapshot: super::HANDLE, lpme: LPMODULEENTRY32W) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn Module32NextW(hsnapshot : super::HANDLE, lpme : LPMODULEENTRY32W) -> windows_core::BOOL);
    unsafe { Module32NextW(hsnapshot as _, lpme as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn Process32First(hsnapshot: super::HANDLE, lppe: LPPROCESSENTRY32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn Process32First(hsnapshot : super::HANDLE, lppe : LPPROCESSENTRY32) -> windows_core::BOOL);
    unsafe { Process32First(hsnapshot as _, lppe as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn Process32FirstW(hsnapshot: super::HANDLE, lppe: LPPROCESSENTRY32W) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn Process32FirstW(hsnapshot : super::HANDLE, lppe : LPPROCESSENTRY32W) -> windows_core::BOOL);
    unsafe { Process32FirstW(hsnapshot as _, lppe as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn Process32Next(hsnapshot: super::HANDLE, lppe: LPPROCESSENTRY32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn Process32Next(hsnapshot : super::HANDLE, lppe : LPPROCESSENTRY32) -> windows_core::BOOL);
    unsafe { Process32Next(hsnapshot as _, lppe as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn Process32NextW(hsnapshot: super::HANDLE, lppe: LPPROCESSENTRY32W) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn Process32NextW(hsnapshot : super::HANDLE, lppe : LPPROCESSENTRY32W) -> windows_core::BOOL);
    unsafe { Process32NextW(hsnapshot as _, lppe as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn Thread32First(hsnapshot: super::HANDLE, lpte: LPTHREADENTRY32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn Thread32First(hsnapshot : super::HANDLE, lpte : LPTHREADENTRY32) -> windows_core::BOOL);
    unsafe { Thread32First(hsnapshot as _, lpte as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn Thread32Next(hsnapshot: super::HANDLE, lpte: LPTHREADENTRY32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn Thread32Next(hsnapshot : super::HANDLE, lpte : LPTHREADENTRY32) -> windows_core::BOOL);
    unsafe { Thread32Next(hsnapshot as _, lpte as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn Toolhelp32ReadProcessMemory(th32processid: super::DWORD, lpbaseaddress: super::LPCVOID, lpbuffer: super::LPVOID, cbread: super::SIZE_T, lpnumberofbytesread: *mut super::SIZE_T) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn Toolhelp32ReadProcessMemory(th32processid : super::DWORD, lpbaseaddress : super::LPCVOID, lpbuffer : super::LPVOID, cbread : super::SIZE_T, lpnumberofbytesread : *mut super::SIZE_T) -> windows_core::BOOL);
    unsafe { Toolhelp32ReadProcessMemory(th32processid, lpbaseaddress as _, lpbuffer as _, cbread, lpnumberofbytesread as _) }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HEAPENTRY32 {
    pub dwSize: super::SIZE_T,
    pub hHandle: super::HANDLE,
    pub dwAddress: super::ULONG_PTR,
    pub dwBlockSize: super::SIZE_T,
    pub dwFlags: super::DWORD,
    pub dwLockCount: super::DWORD,
    pub dwResvd: super::DWORD,
    pub th32ProcessID: super::DWORD,
    pub th32HeapID: super::ULONG_PTR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HEAPLIST32 {
    pub dwSize: super::SIZE_T,
    pub th32ProcessID: super::DWORD,
    pub th32HeapID: super::ULONG_PTR,
    pub dwFlags: super::DWORD,
}
pub const HF32_DEFAULT: i32 = 1;
pub const HF32_SHARED: i32 = 2;
pub const LF32_FIXED: i32 = 1;
pub const LF32_FREE: i32 = 2;
pub const LF32_MOVEABLE: i32 = 4;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPHEAPENTRY32 = *mut HEAPENTRY32;
#[cfg(feature = "shared")]
pub type LPHEAPLIST32 = *mut HEAPLIST32;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type LPMODULEENTRY32 = *mut MODULEENTRY32;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type LPMODULEENTRY32W = *mut MODULEENTRY32W;
#[cfg(feature = "shared")]
pub type LPPROCESSENTRY32 = *mut PROCESSENTRY32;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPPROCESSENTRY32W = *mut PROCESSENTRY32W;
#[cfg(feature = "shared")]
pub type LPTHREADENTRY32 = *mut THREADENTRY32;
pub const MAX_MODULE_NAME32: i32 = 255;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MODULEENTRY32 {
    pub dwSize: super::DWORD,
    pub th32ModuleID: super::DWORD,
    pub th32ProcessID: super::DWORD,
    pub GlblcntUsage: super::DWORD,
    pub ProccntUsage: super::DWORD,
    pub modBaseAddr: *mut super::BYTE,
    pub modBaseSize: super::DWORD,
    pub hModule: super::HMODULE,
    pub szModule: [i8; 256],
    pub szExePath: [i8; 260],
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
impl Default for MODULEENTRY32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MODULEENTRY32W {
    pub dwSize: super::DWORD,
    pub th32ModuleID: super::DWORD,
    pub th32ProcessID: super::DWORD,
    pub GlblcntUsage: super::DWORD,
    pub ProccntUsage: super::DWORD,
    pub modBaseAddr: *mut super::BYTE,
    pub modBaseSize: super::DWORD,
    pub hModule: super::HMODULE,
    pub szModule: [super::WCHAR; 256],
    pub szExePath: [super::WCHAR; 260],
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
impl Default for MODULEENTRY32W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PHEAPENTRY32 = *mut HEAPENTRY32;
#[cfg(feature = "shared")]
pub type PHEAPLIST32 = *mut HEAPLIST32;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PMODULEENTRY32 = *mut MODULEENTRY32;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type PMODULEENTRY32W = *mut MODULEENTRY32W;
#[cfg(feature = "shared")]
pub type PPROCESSENTRY32 = *mut PROCESSENTRY32;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PPROCESSENTRY32W = *mut PROCESSENTRY32W;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PROCESSENTRY32 {
    pub dwSize: super::DWORD,
    pub cntUsage: super::DWORD,
    pub th32ProcessID: super::DWORD,
    pub th32DefaultHeapID: super::ULONG_PTR,
    pub th32ModuleID: super::DWORD,
    pub cntThreads: super::DWORD,
    pub th32ParentProcessID: super::DWORD,
    pub pcPriClassBase: super::LONG,
    pub dwFlags: super::DWORD,
    pub szExeFile: [super::CHAR; 260],
}
#[cfg(feature = "shared")]
impl Default for PROCESSENTRY32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PROCESSENTRY32W {
    pub dwSize: super::DWORD,
    pub cntUsage: super::DWORD,
    pub th32ProcessID: super::DWORD,
    pub th32DefaultHeapID: super::ULONG_PTR,
    pub th32ModuleID: super::DWORD,
    pub cntThreads: super::DWORD,
    pub th32ParentProcessID: super::DWORD,
    pub pcPriClassBase: super::LONG,
    pub dwFlags: super::DWORD,
    pub szExeFile: [super::WCHAR; 260],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for PROCESSENTRY32W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "shared")]
pub type PTHREADENTRY32 = *mut THREADENTRY32;
pub const TH32CS_INHERIT: u32 = 2147483648;
pub const TH32CS_SNAPALL: i32 = 15;
pub const TH32CS_SNAPHEAPLIST: i32 = 1;
pub const TH32CS_SNAPMODULE: i32 = 8;
pub const TH32CS_SNAPMODULE32: i32 = 16;
pub const TH32CS_SNAPPROCESS: i32 = 2;
pub const TH32CS_SNAPTHREAD: i32 = 4;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct THREADENTRY32 {
    pub dwSize: super::DWORD,
    pub cntUsage: super::DWORD,
    pub th32ThreadID: super::DWORD,
    pub th32OwnerProcessID: super::DWORD,
    pub tpBasePri: super::LONG,
    pub tpDeltaPri: super::LONG,
    pub dwFlags: super::DWORD,
}
