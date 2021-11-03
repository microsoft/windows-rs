#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Win32_System_Diagnostics_ToolHelp`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CREATE_TOOLHELP_SNAPSHOT_FLAGS(pub u32);
pub const TH32CS_INHERIT: CREATE_TOOLHELP_SNAPSHOT_FLAGS = CREATE_TOOLHELP_SNAPSHOT_FLAGS(2147483648u32);
pub const TH32CS_SNAPALL: CREATE_TOOLHELP_SNAPSHOT_FLAGS = CREATE_TOOLHELP_SNAPSHOT_FLAGS(15u32);
pub const TH32CS_SNAPHEAPLIST: CREATE_TOOLHELP_SNAPSHOT_FLAGS = CREATE_TOOLHELP_SNAPSHOT_FLAGS(1u32);
pub const TH32CS_SNAPMODULE: CREATE_TOOLHELP_SNAPSHOT_FLAGS = CREATE_TOOLHELP_SNAPSHOT_FLAGS(8u32);
pub const TH32CS_SNAPMODULE32: CREATE_TOOLHELP_SNAPSHOT_FLAGS = CREATE_TOOLHELP_SNAPSHOT_FLAGS(16u32);
pub const TH32CS_SNAPPROCESS: CREATE_TOOLHELP_SNAPSHOT_FLAGS = CREATE_TOOLHELP_SNAPSHOT_FLAGS(2u32);
pub const TH32CS_SNAPTHREAD: CREATE_TOOLHELP_SNAPSHOT_FLAGS = CREATE_TOOLHELP_SNAPSHOT_FLAGS(4u32);
impl ::std::convert::From<u32> for CREATE_TOOLHELP_SNAPSHOT_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CREATE_TOOLHELP_SNAPSHOT_FLAGS {
    type Abi = Self;
}
impl ::std::ops::BitOr for CREATE_TOOLHELP_SNAPSHOT_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for CREATE_TOOLHELP_SNAPSHOT_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for CREATE_TOOLHELP_SNAPSHOT_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for CREATE_TOOLHELP_SNAPSHOT_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for CREATE_TOOLHELP_SNAPSHOT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Diagnostics_ToolHelp`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn CreateToolhelp32Snapshot(dwflags: CREATE_TOOLHELP_SNAPSHOT_FLAGS, th32processid: u32) -> super::super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateToolhelp32Snapshot(dwflags: CREATE_TOOLHELP_SNAPSHOT_FLAGS, th32processid: u32) -> super::super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(CreateToolhelp32Snapshot(::std::mem::transmute(dwflags), ::std::mem::transmute(th32processid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Diagnostics_ToolHelp`, `Win32_Foundation`*"]
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
impl HEAPENTRY32 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for HEAPENTRY32 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for HEAPENTRY32 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HEAPENTRY32")
            .field("dwSize", &self.dwSize)
            .field("hHandle", &self.hHandle)
            .field("dwAddress", &self.dwAddress)
            .field("dwBlockSize", &self.dwBlockSize)
            .field("dwFlags", &self.dwFlags)
            .field("dwLockCount", &self.dwLockCount)
            .field("dwResvd", &self.dwResvd)
            .field("th32ProcessID", &self.th32ProcessID)
            .field("th32HeapID", &self.th32HeapID)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for HEAPENTRY32 {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.hHandle == other.hHandle && self.dwAddress == other.dwAddress && self.dwBlockSize == other.dwBlockSize && self.dwFlags == other.dwFlags && self.dwLockCount == other.dwLockCount && self.dwResvd == other.dwResvd && self.th32ProcessID == other.th32ProcessID && self.th32HeapID == other.th32HeapID
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for HEAPENTRY32 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for HEAPENTRY32 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Diagnostics_ToolHelp`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct HEAPENTRY32_FLAGS(pub u32);
pub const LF32_FIXED: HEAPENTRY32_FLAGS = HEAPENTRY32_FLAGS(1u32);
pub const LF32_FREE: HEAPENTRY32_FLAGS = HEAPENTRY32_FLAGS(2u32);
pub const LF32_MOVEABLE: HEAPENTRY32_FLAGS = HEAPENTRY32_FLAGS(4u32);
impl ::std::convert::From<u32> for HEAPENTRY32_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HEAPENTRY32_FLAGS {
    type Abi = Self;
}
impl ::std::ops::BitOr for HEAPENTRY32_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for HEAPENTRY32_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for HEAPENTRY32_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for HEAPENTRY32_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for HEAPENTRY32_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_ToolHelp`*"]
pub struct HEAPLIST32 {
    pub dwSize: usize,
    pub th32ProcessID: u32,
    pub th32HeapID: usize,
    pub dwFlags: u32,
}
impl HEAPLIST32 {}
impl ::std::default::Default for HEAPLIST32 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for HEAPLIST32 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HEAPLIST32").field("dwSize", &self.dwSize).field("th32ProcessID", &self.th32ProcessID).field("th32HeapID", &self.th32HeapID).field("dwFlags", &self.dwFlags).finish()
    }
}
impl ::std::cmp::PartialEq for HEAPLIST32 {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.th32ProcessID == other.th32ProcessID && self.th32HeapID == other.th32HeapID && self.dwFlags == other.dwFlags
    }
}
impl ::std::cmp::Eq for HEAPLIST32 {}
unsafe impl ::windows::runtime::Abi for HEAPLIST32 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Diagnostics_ToolHelp`*"]
pub const HF32_DEFAULT: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_ToolHelp`*"]
pub const HF32_SHARED: u32 = 2u32;
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Diagnostics_ToolHelp`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn Heap32First(lphe: *mut HEAPENTRY32, th32processid: u32, th32heapid: usize) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn Heap32First(lphe: *mut HEAPENTRY32, th32processid: u32, th32heapid: usize) -> super::super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(Heap32First(::std::mem::transmute(lphe), ::std::mem::transmute(th32processid), ::std::mem::transmute(th32heapid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Diagnostics_ToolHelp`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn Heap32ListFirst<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HANDLE>>(hsnapshot: Param0, lphl: *mut HEAPLIST32) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn Heap32ListFirst(hsnapshot: super::super::super::Foundation::HANDLE, lphl: *mut HEAPLIST32) -> super::super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(Heap32ListFirst(hsnapshot.into_param().abi(), ::std::mem::transmute(lphl)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Diagnostics_ToolHelp`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn Heap32ListNext<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HANDLE>>(hsnapshot: Param0, lphl: *mut HEAPLIST32) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn Heap32ListNext(hsnapshot: super::super::super::Foundation::HANDLE, lphl: *mut HEAPLIST32) -> super::super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(Heap32ListNext(hsnapshot.into_param().abi(), ::std::mem::transmute(lphl)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Diagnostics_ToolHelp`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn Heap32Next(lphe: *mut HEAPENTRY32) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn Heap32Next(lphe: *mut HEAPENTRY32) -> super::super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(Heap32Next(::std::mem::transmute(lphe)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Diagnostics_ToolHelp`*"]
pub const MAX_MODULE_NAME32: u32 = 255u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Diagnostics_ToolHelp`, `Win32_Foundation`*"]
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
impl MODULEENTRY32 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MODULEENTRY32 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for MODULEENTRY32 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MODULEENTRY32")
            .field("dwSize", &self.dwSize)
            .field("th32ModuleID", &self.th32ModuleID)
            .field("th32ProcessID", &self.th32ProcessID)
            .field("GlblcntUsage", &self.GlblcntUsage)
            .field("ProccntUsage", &self.ProccntUsage)
            .field("modBaseAddr", &self.modBaseAddr)
            .field("modBaseSize", &self.modBaseSize)
            .field("hModule", &self.hModule)
            .field("szModule", &self.szModule)
            .field("szExePath", &self.szExePath)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MODULEENTRY32 {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.th32ModuleID == other.th32ModuleID && self.th32ProcessID == other.th32ProcessID && self.GlblcntUsage == other.GlblcntUsage && self.ProccntUsage == other.ProccntUsage && self.modBaseAddr == other.modBaseAddr && self.modBaseSize == other.modBaseSize && self.hModule == other.hModule && self.szModule == other.szModule && self.szExePath == other.szExePath
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MODULEENTRY32 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MODULEENTRY32 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Diagnostics_ToolHelp`, `Win32_Foundation`*"]
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
impl MODULEENTRY32W {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MODULEENTRY32W {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for MODULEENTRY32W {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MODULEENTRY32W")
            .field("dwSize", &self.dwSize)
            .field("th32ModuleID", &self.th32ModuleID)
            .field("th32ProcessID", &self.th32ProcessID)
            .field("GlblcntUsage", &self.GlblcntUsage)
            .field("ProccntUsage", &self.ProccntUsage)
            .field("modBaseAddr", &self.modBaseAddr)
            .field("modBaseSize", &self.modBaseSize)
            .field("hModule", &self.hModule)
            .field("szModule", &self.szModule)
            .field("szExePath", &self.szExePath)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MODULEENTRY32W {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.th32ModuleID == other.th32ModuleID && self.th32ProcessID == other.th32ProcessID && self.GlblcntUsage == other.GlblcntUsage && self.ProccntUsage == other.ProccntUsage && self.modBaseAddr == other.modBaseAddr && self.modBaseSize == other.modBaseSize && self.hModule == other.hModule && self.szModule == other.szModule && self.szExePath == other.szExePath
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MODULEENTRY32W {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MODULEENTRY32W {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Diagnostics_ToolHelp`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn Module32First<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HANDLE>>(hsnapshot: Param0, lpme: *mut MODULEENTRY32) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn Module32First(hsnapshot: super::super::super::Foundation::HANDLE, lpme: *mut MODULEENTRY32) -> super::super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(Module32First(hsnapshot.into_param().abi(), ::std::mem::transmute(lpme)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Diagnostics_ToolHelp`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn Module32FirstW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HANDLE>>(hsnapshot: Param0, lpme: *mut MODULEENTRY32W) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn Module32FirstW(hsnapshot: super::super::super::Foundation::HANDLE, lpme: *mut MODULEENTRY32W) -> super::super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(Module32FirstW(hsnapshot.into_param().abi(), ::std::mem::transmute(lpme)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Diagnostics_ToolHelp`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn Module32Next<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HANDLE>>(hsnapshot: Param0, lpme: *mut MODULEENTRY32) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn Module32Next(hsnapshot: super::super::super::Foundation::HANDLE, lpme: *mut MODULEENTRY32) -> super::super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(Module32Next(hsnapshot.into_param().abi(), ::std::mem::transmute(lpme)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Diagnostics_ToolHelp`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn Module32NextW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HANDLE>>(hsnapshot: Param0, lpme: *mut MODULEENTRY32W) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn Module32NextW(hsnapshot: super::super::super::Foundation::HANDLE, lpme: *mut MODULEENTRY32W) -> super::super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(Module32NextW(hsnapshot.into_param().abi(), ::std::mem::transmute(lpme)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Diagnostics_ToolHelp`, `Win32_Foundation`*"]
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
impl PROCESSENTRY32 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for PROCESSENTRY32 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for PROCESSENTRY32 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PROCESSENTRY32")
            .field("dwSize", &self.dwSize)
            .field("cntUsage", &self.cntUsage)
            .field("th32ProcessID", &self.th32ProcessID)
            .field("th32DefaultHeapID", &self.th32DefaultHeapID)
            .field("th32ModuleID", &self.th32ModuleID)
            .field("cntThreads", &self.cntThreads)
            .field("th32ParentProcessID", &self.th32ParentProcessID)
            .field("pcPriClassBase", &self.pcPriClassBase)
            .field("dwFlags", &self.dwFlags)
            .field("szExeFile", &self.szExeFile)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for PROCESSENTRY32 {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.cntUsage == other.cntUsage && self.th32ProcessID == other.th32ProcessID && self.th32DefaultHeapID == other.th32DefaultHeapID && self.th32ModuleID == other.th32ModuleID && self.cntThreads == other.cntThreads && self.th32ParentProcessID == other.th32ParentProcessID && self.pcPriClassBase == other.pcPriClassBase && self.dwFlags == other.dwFlags && self.szExeFile == other.szExeFile
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for PROCESSENTRY32 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for PROCESSENTRY32 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_ToolHelp`*"]
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
impl PROCESSENTRY32W {}
impl ::std::default::Default for PROCESSENTRY32W {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PROCESSENTRY32W {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PROCESSENTRY32W")
            .field("dwSize", &self.dwSize)
            .field("cntUsage", &self.cntUsage)
            .field("th32ProcessID", &self.th32ProcessID)
            .field("th32DefaultHeapID", &self.th32DefaultHeapID)
            .field("th32ModuleID", &self.th32ModuleID)
            .field("cntThreads", &self.cntThreads)
            .field("th32ParentProcessID", &self.th32ParentProcessID)
            .field("pcPriClassBase", &self.pcPriClassBase)
            .field("dwFlags", &self.dwFlags)
            .field("szExeFile", &self.szExeFile)
            .finish()
    }
}
impl ::std::cmp::PartialEq for PROCESSENTRY32W {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.cntUsage == other.cntUsage && self.th32ProcessID == other.th32ProcessID && self.th32DefaultHeapID == other.th32DefaultHeapID && self.th32ModuleID == other.th32ModuleID && self.cntThreads == other.cntThreads && self.th32ParentProcessID == other.th32ParentProcessID && self.pcPriClassBase == other.pcPriClassBase && self.dwFlags == other.dwFlags && self.szExeFile == other.szExeFile
    }
}
impl ::std::cmp::Eq for PROCESSENTRY32W {}
unsafe impl ::windows::runtime::Abi for PROCESSENTRY32W {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Diagnostics_ToolHelp`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn Process32First<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HANDLE>>(hsnapshot: Param0, lppe: *mut PROCESSENTRY32) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn Process32First(hsnapshot: super::super::super::Foundation::HANDLE, lppe: *mut PROCESSENTRY32) -> super::super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(Process32First(hsnapshot.into_param().abi(), ::std::mem::transmute(lppe)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Diagnostics_ToolHelp`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn Process32FirstW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HANDLE>>(hsnapshot: Param0, lppe: *mut PROCESSENTRY32W) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn Process32FirstW(hsnapshot: super::super::super::Foundation::HANDLE, lppe: *mut PROCESSENTRY32W) -> super::super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(Process32FirstW(hsnapshot.into_param().abi(), ::std::mem::transmute(lppe)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Diagnostics_ToolHelp`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn Process32Next<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HANDLE>>(hsnapshot: Param0, lppe: *mut PROCESSENTRY32) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn Process32Next(hsnapshot: super::super::super::Foundation::HANDLE, lppe: *mut PROCESSENTRY32) -> super::super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(Process32Next(hsnapshot.into_param().abi(), ::std::mem::transmute(lppe)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Diagnostics_ToolHelp`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn Process32NextW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HANDLE>>(hsnapshot: Param0, lppe: *mut PROCESSENTRY32W) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn Process32NextW(hsnapshot: super::super::super::Foundation::HANDLE, lppe: *mut PROCESSENTRY32W) -> super::super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(Process32NextW(hsnapshot.into_param().abi(), ::std::mem::transmute(lppe)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Diagnostics_ToolHelp`*"]
pub struct THREADENTRY32 {
    pub dwSize: u32,
    pub cntUsage: u32,
    pub th32ThreadID: u32,
    pub th32OwnerProcessID: u32,
    pub tpBasePri: i32,
    pub tpDeltaPri: i32,
    pub dwFlags: u32,
}
impl THREADENTRY32 {}
impl ::std::default::Default for THREADENTRY32 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for THREADENTRY32 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("THREADENTRY32")
            .field("dwSize", &self.dwSize)
            .field("cntUsage", &self.cntUsage)
            .field("th32ThreadID", &self.th32ThreadID)
            .field("th32OwnerProcessID", &self.th32OwnerProcessID)
            .field("tpBasePri", &self.tpBasePri)
            .field("tpDeltaPri", &self.tpDeltaPri)
            .field("dwFlags", &self.dwFlags)
            .finish()
    }
}
impl ::std::cmp::PartialEq for THREADENTRY32 {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.cntUsage == other.cntUsage && self.th32ThreadID == other.th32ThreadID && self.th32OwnerProcessID == other.th32OwnerProcessID && self.tpBasePri == other.tpBasePri && self.tpDeltaPri == other.tpDeltaPri && self.dwFlags == other.dwFlags
    }
}
impl ::std::cmp::Eq for THREADENTRY32 {}
unsafe impl ::windows::runtime::Abi for THREADENTRY32 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Diagnostics_ToolHelp`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn Thread32First<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HANDLE>>(hsnapshot: Param0, lpte: *mut THREADENTRY32) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn Thread32First(hsnapshot: super::super::super::Foundation::HANDLE, lpte: *mut THREADENTRY32) -> super::super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(Thread32First(hsnapshot.into_param().abi(), ::std::mem::transmute(lpte)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Diagnostics_ToolHelp`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn Thread32Next<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HANDLE>>(hsnapshot: Param0, lpte: *mut THREADENTRY32) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn Thread32Next(hsnapshot: super::super::super::Foundation::HANDLE, lpte: *mut THREADENTRY32) -> super::super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(Thread32Next(hsnapshot.into_param().abi(), ::std::mem::transmute(lpte)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Diagnostics_ToolHelp`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn Toolhelp32ReadProcessMemory(th32processid: u32, lpbaseaddress: *const ::std::ffi::c_void, lpbuffer: *mut ::std::ffi::c_void, cbread: usize, lpnumberofbytesread: *mut usize) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn Toolhelp32ReadProcessMemory(th32processid: u32, lpbaseaddress: *const ::std::ffi::c_void, lpbuffer: *mut ::std::ffi::c_void, cbread: usize, lpnumberofbytesread: *mut usize) -> super::super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(Toolhelp32ReadProcessMemory(::std::mem::transmute(th32processid), ::std::mem::transmute(lpbaseaddress), ::std::mem::transmute(lpbuffer), ::std::mem::transmute(cbread), ::std::mem::transmute(lpnumberofbytesread)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
