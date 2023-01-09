#[doc = "*Required features: `\"Win32_System_Diagnostics_ToolHelp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateToolhelp32Snapshot(dwflags: CREATE_TOOLHELP_SNAPSHOT_FLAGS, th32processid: u32) -> ::windows::core::Result<super::super::super::Foundation::HANDLE> {
    ::windows::core::link ! ( "kernel32.dll""system" fn CreateToolhelp32Snapshot ( dwflags : CREATE_TOOLHELP_SNAPSHOT_FLAGS , th32processid : u32 ) -> super::super::super::Foundation:: HANDLE );
    let result__ = CreateToolhelp32Snapshot(dwflags, th32processid);
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_ToolHelp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn Heap32First(lphe: *mut HEAPENTRY32, th32processid: u32, th32heapid: usize) -> super::super::super::Foundation::BOOL {
    ::windows::core::link ! ( "kernel32.dll""system" fn Heap32First ( lphe : *mut HEAPENTRY32 , th32processid : u32 , th32heapid : usize ) -> super::super::super::Foundation:: BOOL );
    Heap32First(lphe, th32processid, th32heapid)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_ToolHelp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn Heap32ListFirst<P0>(hsnapshot: P0, lphl: *mut HEAPLIST32) -> super::super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "kernel32.dll""system" fn Heap32ListFirst ( hsnapshot : super::super::super::Foundation:: HANDLE , lphl : *mut HEAPLIST32 ) -> super::super::super::Foundation:: BOOL );
    Heap32ListFirst(hsnapshot.into(), lphl)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_ToolHelp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn Heap32ListNext<P0>(hsnapshot: P0, lphl: *mut HEAPLIST32) -> super::super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "kernel32.dll""system" fn Heap32ListNext ( hsnapshot : super::super::super::Foundation:: HANDLE , lphl : *mut HEAPLIST32 ) -> super::super::super::Foundation:: BOOL );
    Heap32ListNext(hsnapshot.into(), lphl)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_ToolHelp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn Heap32Next(lphe: *mut HEAPENTRY32) -> super::super::super::Foundation::BOOL {
    ::windows::core::link ! ( "kernel32.dll""system" fn Heap32Next ( lphe : *mut HEAPENTRY32 ) -> super::super::super::Foundation:: BOOL );
    Heap32Next(lphe)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_ToolHelp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn Module32First<P0>(hsnapshot: P0, lpme: *mut MODULEENTRY32) -> super::super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "kernel32.dll""system" fn Module32First ( hsnapshot : super::super::super::Foundation:: HANDLE , lpme : *mut MODULEENTRY32 ) -> super::super::super::Foundation:: BOOL );
    Module32First(hsnapshot.into(), lpme)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_ToolHelp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn Module32FirstW<P0>(hsnapshot: P0, lpme: *mut MODULEENTRY32W) -> super::super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "kernel32.dll""system" fn Module32FirstW ( hsnapshot : super::super::super::Foundation:: HANDLE , lpme : *mut MODULEENTRY32W ) -> super::super::super::Foundation:: BOOL );
    Module32FirstW(hsnapshot.into(), lpme)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_ToolHelp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn Module32Next<P0>(hsnapshot: P0, lpme: *mut MODULEENTRY32) -> super::super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "kernel32.dll""system" fn Module32Next ( hsnapshot : super::super::super::Foundation:: HANDLE , lpme : *mut MODULEENTRY32 ) -> super::super::super::Foundation:: BOOL );
    Module32Next(hsnapshot.into(), lpme)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_ToolHelp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn Module32NextW<P0>(hsnapshot: P0, lpme: *mut MODULEENTRY32W) -> super::super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "kernel32.dll""system" fn Module32NextW ( hsnapshot : super::super::super::Foundation:: HANDLE , lpme : *mut MODULEENTRY32W ) -> super::super::super::Foundation:: BOOL );
    Module32NextW(hsnapshot.into(), lpme)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_ToolHelp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn Process32First<P0>(hsnapshot: P0, lppe: *mut PROCESSENTRY32) -> super::super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "kernel32.dll""system" fn Process32First ( hsnapshot : super::super::super::Foundation:: HANDLE , lppe : *mut PROCESSENTRY32 ) -> super::super::super::Foundation:: BOOL );
    Process32First(hsnapshot.into(), lppe)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_ToolHelp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn Process32FirstW<P0>(hsnapshot: P0, lppe: *mut PROCESSENTRY32W) -> super::super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "kernel32.dll""system" fn Process32FirstW ( hsnapshot : super::super::super::Foundation:: HANDLE , lppe : *mut PROCESSENTRY32W ) -> super::super::super::Foundation:: BOOL );
    Process32FirstW(hsnapshot.into(), lppe)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_ToolHelp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn Process32Next<P0>(hsnapshot: P0, lppe: *mut PROCESSENTRY32) -> super::super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "kernel32.dll""system" fn Process32Next ( hsnapshot : super::super::super::Foundation:: HANDLE , lppe : *mut PROCESSENTRY32 ) -> super::super::super::Foundation:: BOOL );
    Process32Next(hsnapshot.into(), lppe)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_ToolHelp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn Process32NextW<P0>(hsnapshot: P0, lppe: *mut PROCESSENTRY32W) -> super::super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "kernel32.dll""system" fn Process32NextW ( hsnapshot : super::super::super::Foundation:: HANDLE , lppe : *mut PROCESSENTRY32W ) -> super::super::super::Foundation:: BOOL );
    Process32NextW(hsnapshot.into(), lppe)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_ToolHelp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn Thread32First<P0>(hsnapshot: P0, lpte: *mut THREADENTRY32) -> super::super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "kernel32.dll""system" fn Thread32First ( hsnapshot : super::super::super::Foundation:: HANDLE , lpte : *mut THREADENTRY32 ) -> super::super::super::Foundation:: BOOL );
    Thread32First(hsnapshot.into(), lpte)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_ToolHelp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn Thread32Next<P0>(hsnapshot: P0, lpte: *mut THREADENTRY32) -> super::super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "kernel32.dll""system" fn Thread32Next ( hsnapshot : super::super::super::Foundation:: HANDLE , lpte : *mut THREADENTRY32 ) -> super::super::super::Foundation:: BOOL );
    Thread32Next(hsnapshot.into(), lpte)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_ToolHelp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn Toolhelp32ReadProcessMemory(th32processid: u32, lpbaseaddress: *const ::core::ffi::c_void, lpbuffer: *mut ::core::ffi::c_void, cbread: usize, lpnumberofbytesread: *mut usize) -> super::super::super::Foundation::BOOL {
    ::windows::core::link ! ( "kernel32.dll""system" fn Toolhelp32ReadProcessMemory ( th32processid : u32 , lpbaseaddress : *const ::core::ffi::c_void , lpbuffer : *mut ::core::ffi::c_void , cbread : usize , lpnumberofbytesread : *mut usize ) -> super::super::super::Foundation:: BOOL );
    Toolhelp32ReadProcessMemory(th32processid, lpbaseaddress, lpbuffer, cbread, lpnumberofbytesread)
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_ToolHelp\"`*"]
pub const HF32_DEFAULT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ToolHelp\"`*"]
pub const HF32_SHARED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ToolHelp\"`*"]
pub const MAX_MODULE_NAME32: u32 = 255u32;
#[doc = "*Required features: `\"Win32_System_Diagnostics_ToolHelp\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CREATE_TOOLHELP_SNAPSHOT_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ToolHelp\"`*"]
pub const TH32CS_INHERIT: CREATE_TOOLHELP_SNAPSHOT_FLAGS = CREATE_TOOLHELP_SNAPSHOT_FLAGS(2147483648u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ToolHelp\"`*"]
pub const TH32CS_SNAPALL: CREATE_TOOLHELP_SNAPSHOT_FLAGS = CREATE_TOOLHELP_SNAPSHOT_FLAGS(15u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ToolHelp\"`*"]
pub const TH32CS_SNAPHEAPLIST: CREATE_TOOLHELP_SNAPSHOT_FLAGS = CREATE_TOOLHELP_SNAPSHOT_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ToolHelp\"`*"]
pub const TH32CS_SNAPMODULE: CREATE_TOOLHELP_SNAPSHOT_FLAGS = CREATE_TOOLHELP_SNAPSHOT_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ToolHelp\"`*"]
pub const TH32CS_SNAPMODULE32: CREATE_TOOLHELP_SNAPSHOT_FLAGS = CREATE_TOOLHELP_SNAPSHOT_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ToolHelp\"`*"]
pub const TH32CS_SNAPPROCESS: CREATE_TOOLHELP_SNAPSHOT_FLAGS = CREATE_TOOLHELP_SNAPSHOT_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ToolHelp\"`*"]
pub const TH32CS_SNAPTHREAD: CREATE_TOOLHELP_SNAPSHOT_FLAGS = CREATE_TOOLHELP_SNAPSHOT_FLAGS(4u32);
impl ::core::marker::Copy for CREATE_TOOLHELP_SNAPSHOT_FLAGS {}
impl ::core::clone::Clone for CREATE_TOOLHELP_SNAPSHOT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CREATE_TOOLHELP_SNAPSHOT_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_ToolHelp\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HEAPENTRY32_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ToolHelp\"`*"]
pub const LF32_FIXED: HEAPENTRY32_FLAGS = HEAPENTRY32_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ToolHelp\"`*"]
pub const LF32_FREE: HEAPENTRY32_FLAGS = HEAPENTRY32_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_System_Diagnostics_ToolHelp\"`*"]
pub const LF32_MOVEABLE: HEAPENTRY32_FLAGS = HEAPENTRY32_FLAGS(4u32);
impl ::core::marker::Copy for HEAPENTRY32_FLAGS {}
impl ::core::clone::Clone for HEAPENTRY32_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for HEAPENTRY32_FLAGS {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_ToolHelp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HEAPENTRY32 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HEAPENTRY32 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for HEAPENTRY32 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_ToolHelp\"`*"]
pub struct HEAPLIST32 {
    pub dwSize: usize,
    pub th32ProcessID: u32,
    pub th32HeapID: usize,
    pub dwFlags: u32,
}
impl ::core::marker::Copy for HEAPLIST32 {}
impl ::core::clone::Clone for HEAPLIST32 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for HEAPLIST32 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_ToolHelp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MODULEENTRY32 {
    pub dwSize: u32,
    pub th32ModuleID: u32,
    pub th32ProcessID: u32,
    pub GlblcntUsage: u32,
    pub ProccntUsage: u32,
    pub modBaseAddr: *mut u8,
    pub modBaseSize: u32,
    pub hModule: super::super::super::Foundation::HINSTANCE,
    pub szModule: [super::super::super::Foundation::CHAR; 256],
    pub szExePath: [super::super::super::Foundation::CHAR; 260],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MODULEENTRY32 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MODULEENTRY32 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MODULEENTRY32 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_ToolHelp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MODULEENTRY32W {
    pub dwSize: u32,
    pub th32ModuleID: u32,
    pub th32ProcessID: u32,
    pub GlblcntUsage: u32,
    pub ProccntUsage: u32,
    pub modBaseAddr: *mut u8,
    pub modBaseSize: u32,
    pub hModule: super::super::super::Foundation::HINSTANCE,
    pub szModule: [u16; 256],
    pub szExePath: [u16; 260],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MODULEENTRY32W {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MODULEENTRY32W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MODULEENTRY32W {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_ToolHelp\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
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
    pub szExeFile: [super::super::super::Foundation::CHAR; 260],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PROCESSENTRY32 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PROCESSENTRY32 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PROCESSENTRY32 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_ToolHelp\"`*"]
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
impl ::core::marker::Copy for PROCESSENTRY32W {}
impl ::core::clone::Clone for PROCESSENTRY32W {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PROCESSENTRY32W {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Diagnostics_ToolHelp\"`*"]
pub struct THREADENTRY32 {
    pub dwSize: u32,
    pub cntUsage: u32,
    pub th32ThreadID: u32,
    pub th32OwnerProcessID: u32,
    pub tpBasePri: i32,
    pub tpDeltaPri: i32,
    pub dwFlags: u32,
}
impl ::core::marker::Copy for THREADENTRY32 {}
impl ::core::clone::Clone for THREADENTRY32 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for THREADENTRY32 {
    type Abi = Self;
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
#[cfg(feature = "default")]
::core::include!("default.rs");
