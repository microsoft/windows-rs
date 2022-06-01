#[cfg(feature = "Win32_System_Memory_NonVolatile")]
pub mod NonVolatile;
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AddSecureMemoryCacheCallback(pfncallback: PSECURE_MEMORY_CACHE_CALLBACK) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AddSecureMemoryCacheCallback(pfncallback: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(AddSecureMemoryCacheCallback(::core::mem::transmute(pfncallback)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AllocateUserPhysicalPages<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hprocess: Param0, numberofpages: *mut usize, pagearray: *mut usize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AllocateUserPhysicalPages(hprocess: super::super::Foundation::HANDLE, numberofpages: *mut usize, pagearray: *mut usize) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(AllocateUserPhysicalPages(hprocess.into_param().abi(), ::core::mem::transmute(numberofpages), ::core::mem::transmute(pagearray)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AllocateUserPhysicalPages2<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(objecthandle: Param0, numberofpages: *mut usize, pagearray: *mut usize, extendedparameters: &mut [MEM_EXTENDED_PARAMETER]) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AllocateUserPhysicalPages2(objecthandle: super::super::Foundation::HANDLE, numberofpages: *mut usize, pagearray: *mut usize, extendedparameters: *mut MEM_EXTENDED_PARAMETER, extendedparametercount: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(AllocateUserPhysicalPages2(objecthandle.into_param().abi(), ::core::mem::transmute(numberofpages), ::core::mem::transmute(pagearray), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(extendedparameters)), extendedparameters.len() as _))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AllocateUserPhysicalPagesNuma<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hprocess: Param0, numberofpages: *mut usize, pagearray: *mut usize, nndpreferred: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AllocateUserPhysicalPagesNuma(hprocess: super::super::Foundation::HANDLE, numberofpages: *mut usize, pagearray: *mut usize, nndpreferred: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(AllocateUserPhysicalPagesNuma(hprocess.into_param().abi(), ::core::mem::transmute(numberofpages), ::core::mem::transmute(pagearray), ::core::mem::transmute(nndpreferred)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub struct CFG_CALL_TARGET_INFO {
    pub Offset: usize,
    pub Flags: usize,
}
impl ::core::marker::Copy for CFG_CALL_TARGET_INFO {}
impl ::core::clone::Clone for CFG_CALL_TARGET_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CFG_CALL_TARGET_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CFG_CALL_TARGET_INFO").field("Offset", &self.Offset).field("Flags", &self.Flags).finish()
    }
}
unsafe impl ::windows::core::Abi for CFG_CALL_TARGET_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CFG_CALL_TARGET_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CFG_CALL_TARGET_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for CFG_CALL_TARGET_INFO {}
impl ::core::default::Default for CFG_CALL_TARGET_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateFileMapping2<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param6: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(file: Param0, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, desiredaccess: u32, pageprotection: PAGE_PROTECTION_FLAGS, allocationattributes: u32, maximumsize: u64, name: Param6, extendedparameters: &mut [MEM_EXTENDED_PARAMETER]) -> ::windows::core::Result<super::super::Foundation::HANDLE> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateFileMapping2(file: super::super::Foundation::HANDLE, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, desiredaccess: u32, pageprotection: PAGE_PROTECTION_FLAGS, allocationattributes: u32, maximumsize: u64, name: ::windows::core::PCWSTR, extendedparameters: *mut MEM_EXTENDED_PARAMETER, parametercount: u32) -> super::super::Foundation::HANDLE;
        }
        let result__ = CreateFileMapping2(file.into_param().abi(), ::core::mem::transmute(securityattributes), ::core::mem::transmute(desiredaccess), ::core::mem::transmute(pageprotection), ::core::mem::transmute(allocationattributes), ::core::mem::transmute(maximumsize), name.into_param().abi(), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(extendedparameters)), extendedparameters.len() as _);
        (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateFileMappingA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param5: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(hfile: Param0, lpfilemappingattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flprotect: PAGE_PROTECTION_FLAGS, dwmaximumsizehigh: u32, dwmaximumsizelow: u32, lpname: Param5) -> ::windows::core::Result<super::super::Foundation::HANDLE> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateFileMappingA(hfile: super::super::Foundation::HANDLE, lpfilemappingattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flprotect: PAGE_PROTECTION_FLAGS, dwmaximumsizehigh: u32, dwmaximumsizelow: u32, lpname: ::windows::core::PCSTR) -> super::super::Foundation::HANDLE;
        }
        let result__ = CreateFileMappingA(hfile.into_param().abi(), ::core::mem::transmute(lpfilemappingattributes), ::core::mem::transmute(flprotect), ::core::mem::transmute(dwmaximumsizehigh), ::core::mem::transmute(dwmaximumsizelow), lpname.into_param().abi());
        (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateFileMappingFromApp<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param4: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(hfile: Param0, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, pageprotection: PAGE_PROTECTION_FLAGS, maximumsize: u64, name: Param4) -> ::windows::core::Result<super::super::Foundation::HANDLE> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateFileMappingFromApp(hfile: super::super::Foundation::HANDLE, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, pageprotection: PAGE_PROTECTION_FLAGS, maximumsize: u64, name: ::windows::core::PCWSTR) -> super::super::Foundation::HANDLE;
        }
        let result__ = CreateFileMappingFromApp(hfile.into_param().abi(), ::core::mem::transmute(securityattributes), ::core::mem::transmute(pageprotection), ::core::mem::transmute(maximumsize), name.into_param().abi());
        (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateFileMappingNumaA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param5: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(hfile: Param0, lpfilemappingattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flprotect: PAGE_PROTECTION_FLAGS, dwmaximumsizehigh: u32, dwmaximumsizelow: u32, lpname: Param5, nndpreferred: u32) -> ::windows::core::Result<super::super::Foundation::HANDLE> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateFileMappingNumaA(hfile: super::super::Foundation::HANDLE, lpfilemappingattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flprotect: PAGE_PROTECTION_FLAGS, dwmaximumsizehigh: u32, dwmaximumsizelow: u32, lpname: ::windows::core::PCSTR, nndpreferred: u32) -> super::super::Foundation::HANDLE;
        }
        let result__ = CreateFileMappingNumaA(hfile.into_param().abi(), ::core::mem::transmute(lpfilemappingattributes), ::core::mem::transmute(flprotect), ::core::mem::transmute(dwmaximumsizehigh), ::core::mem::transmute(dwmaximumsizelow), lpname.into_param().abi(), ::core::mem::transmute(nndpreferred));
        (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateFileMappingNumaW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param5: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(hfile: Param0, lpfilemappingattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flprotect: PAGE_PROTECTION_FLAGS, dwmaximumsizehigh: u32, dwmaximumsizelow: u32, lpname: Param5, nndpreferred: u32) -> ::windows::core::Result<super::super::Foundation::HANDLE> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateFileMappingNumaW(hfile: super::super::Foundation::HANDLE, lpfilemappingattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flprotect: PAGE_PROTECTION_FLAGS, dwmaximumsizehigh: u32, dwmaximumsizelow: u32, lpname: ::windows::core::PCWSTR, nndpreferred: u32) -> super::super::Foundation::HANDLE;
        }
        let result__ = CreateFileMappingNumaW(hfile.into_param().abi(), ::core::mem::transmute(lpfilemappingattributes), ::core::mem::transmute(flprotect), ::core::mem::transmute(dwmaximumsizehigh), ::core::mem::transmute(dwmaximumsizelow), lpname.into_param().abi(), ::core::mem::transmute(nndpreferred));
        (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateFileMappingW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param5: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(hfile: Param0, lpfilemappingattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flprotect: PAGE_PROTECTION_FLAGS, dwmaximumsizehigh: u32, dwmaximumsizelow: u32, lpname: Param5) -> ::windows::core::Result<super::super::Foundation::HANDLE> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateFileMappingW(hfile: super::super::Foundation::HANDLE, lpfilemappingattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flprotect: PAGE_PROTECTION_FLAGS, dwmaximumsizehigh: u32, dwmaximumsizelow: u32, lpname: ::windows::core::PCWSTR) -> super::super::Foundation::HANDLE;
        }
        let result__ = CreateFileMappingW(hfile.into_param().abi(), ::core::mem::transmute(lpfilemappingattributes), ::core::mem::transmute(flprotect), ::core::mem::transmute(dwmaximumsizehigh), ::core::mem::transmute(dwmaximumsizelow), lpname.into_param().abi());
        (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateMemoryResourceNotification(notificationtype: MEMORY_RESOURCE_NOTIFICATION_TYPE) -> ::windows::core::Result<super::super::Foundation::HANDLE> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateMemoryResourceNotification(notificationtype: MEMORY_RESOURCE_NOTIFICATION_TYPE) -> super::super::Foundation::HANDLE;
        }
        let result__ = CreateMemoryResourceNotification(::core::mem::transmute(notificationtype));
        (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
#[inline]
pub unsafe fn DiscardVirtualMemory(virtualaddress: &mut [u8]) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DiscardVirtualMemory(virtualaddress: *mut ::core::ffi::c_void, size: usize) -> u32;
        }
        ::core::mem::transmute(DiscardVirtualMemory(::core::mem::transmute(::windows::core::as_mut_ptr_or_null(virtualaddress)), virtualaddress.len() as _))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const FILE_CACHE_MAX_HARD_DISABLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const FILE_CACHE_MAX_HARD_ENABLE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const FILE_CACHE_MIN_HARD_DISABLE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const FILE_CACHE_MIN_HARD_ENABLE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FILE_MAP(pub u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const FILE_MAP_WRITE: FILE_MAP = FILE_MAP(2u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const FILE_MAP_READ: FILE_MAP = FILE_MAP(4u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const FILE_MAP_ALL_ACCESS: FILE_MAP = FILE_MAP(983071u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const FILE_MAP_EXECUTE: FILE_MAP = FILE_MAP(32u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const FILE_MAP_COPY: FILE_MAP = FILE_MAP(1u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const FILE_MAP_RESERVE: FILE_MAP = FILE_MAP(2147483648u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const FILE_MAP_TARGETS_INVALID: FILE_MAP = FILE_MAP(1073741824u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const FILE_MAP_LARGE_PAGES: FILE_MAP = FILE_MAP(536870912u32);
impl ::core::marker::Copy for FILE_MAP {}
impl ::core::clone::Clone for FILE_MAP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FILE_MAP {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FILE_MAP {
    type Abi = Self;
}
impl ::core::fmt::Debug for FILE_MAP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FILE_MAP").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for FILE_MAP {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for FILE_MAP {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for FILE_MAP {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for FILE_MAP {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for FILE_MAP {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FlushViewOfFile(lpbaseaddress: *const ::core::ffi::c_void, dwnumberofbytestoflush: usize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FlushViewOfFile(lpbaseaddress: *const ::core::ffi::c_void, dwnumberofbytestoflush: usize) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FlushViewOfFile(::core::mem::transmute(lpbaseaddress), ::core::mem::transmute(dwnumberofbytestoflush)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FreeUserPhysicalPages<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hprocess: Param0, numberofpages: *mut usize, pagearray: *const usize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FreeUserPhysicalPages(hprocess: super::super::Foundation::HANDLE, numberofpages: *mut usize, pagearray: *const usize) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FreeUserPhysicalPages(hprocess.into_param().abi(), ::core::mem::transmute(numberofpages), ::core::mem::transmute(pagearray)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GLOBAL_ALLOC_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const GHND: GLOBAL_ALLOC_FLAGS = GLOBAL_ALLOC_FLAGS(66u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const GMEM_FIXED: GLOBAL_ALLOC_FLAGS = GLOBAL_ALLOC_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const GMEM_MOVEABLE: GLOBAL_ALLOC_FLAGS = GLOBAL_ALLOC_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const GMEM_ZEROINIT: GLOBAL_ALLOC_FLAGS = GLOBAL_ALLOC_FLAGS(64u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const GPTR: GLOBAL_ALLOC_FLAGS = GLOBAL_ALLOC_FLAGS(64u32);
impl ::core::marker::Copy for GLOBAL_ALLOC_FLAGS {}
impl ::core::clone::Clone for GLOBAL_ALLOC_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GLOBAL_ALLOC_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for GLOBAL_ALLOC_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for GLOBAL_ALLOC_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GLOBAL_ALLOC_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for GLOBAL_ALLOC_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for GLOBAL_ALLOC_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for GLOBAL_ALLOC_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for GLOBAL_ALLOC_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for GLOBAL_ALLOC_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
#[inline]
pub unsafe fn GetLargePageMinimum() -> usize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetLargePageMinimum() -> usize;
        }
        ::core::mem::transmute(GetLargePageMinimum())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetMemoryErrorHandlingCapabilities(capabilities: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetMemoryErrorHandlingCapabilities(capabilities: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetMemoryErrorHandlingCapabilities(::core::mem::transmute(capabilities)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
#[inline]
pub unsafe fn GetProcessHeap() -> ::windows::core::Result<HeapHandle> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetProcessHeap() -> HeapHandle;
        }
        let result__ = GetProcessHeap();
        (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
#[inline]
pub unsafe fn GetProcessHeaps(processheaps: &mut [HeapHandle]) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetProcessHeaps(numberofheaps: u32, processheaps: *mut HeapHandle) -> u32;
        }
        ::core::mem::transmute(GetProcessHeaps(processheaps.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(processheaps))))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetProcessWorkingSetSizeEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hprocess: Param0, lpminimumworkingsetsize: *mut usize, lpmaximumworkingsetsize: *mut usize, flags: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetProcessWorkingSetSizeEx(hprocess: super::super::Foundation::HANDLE, lpminimumworkingsetsize: *mut usize, lpmaximumworkingsetsize: *mut usize, flags: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetProcessWorkingSetSizeEx(hprocess.into_param().abi(), ::core::mem::transmute(lpminimumworkingsetsize), ::core::mem::transmute(lpmaximumworkingsetsize), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetSystemFileCacheSize(lpminimumfilecachesize: *mut usize, lpmaximumfilecachesize: *mut usize, lpflags: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetSystemFileCacheSize(lpminimumfilecachesize: *mut usize, lpmaximumfilecachesize: *mut usize, lpflags: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetSystemFileCacheSize(::core::mem::transmute(lpminimumfilecachesize), ::core::mem::transmute(lpmaximumfilecachesize), ::core::mem::transmute(lpflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
#[inline]
pub unsafe fn GetWriteWatch(dwflags: u32, lpbaseaddress: *const ::core::ffi::c_void, dwregionsize: usize, lpaddresses: *mut *mut ::core::ffi::c_void, lpdwcount: *mut usize, lpdwgranularity: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetWriteWatch(dwflags: u32, lpbaseaddress: *const ::core::ffi::c_void, dwregionsize: usize, lpaddresses: *mut *mut ::core::ffi::c_void, lpdwcount: *mut usize, lpdwgranularity: *mut u32) -> u32;
        }
        ::core::mem::transmute(GetWriteWatch(::core::mem::transmute(dwflags), ::core::mem::transmute(lpbaseaddress), ::core::mem::transmute(dwregionsize), ::core::mem::transmute(lpaddresses), ::core::mem::transmute(lpdwcount), ::core::mem::transmute(lpdwgranularity)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
#[inline]
pub unsafe fn GlobalAlloc(uflags: GLOBAL_ALLOC_FLAGS, dwbytes: usize) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GlobalAlloc(uflags: GLOBAL_ALLOC_FLAGS, dwbytes: usize) -> isize;
        }
        ::core::mem::transmute(GlobalAlloc(::core::mem::transmute(uflags), ::core::mem::transmute(dwbytes)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
#[inline]
pub unsafe fn GlobalFlags(hmem: isize) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GlobalFlags(hmem: isize) -> u32;
        }
        ::core::mem::transmute(GlobalFlags(::core::mem::transmute(hmem)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
#[inline]
pub unsafe fn GlobalFree(hmem: isize) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GlobalFree(hmem: isize) -> isize;
        }
        ::core::mem::transmute(GlobalFree(::core::mem::transmute(hmem)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
#[inline]
pub unsafe fn GlobalHandle(pmem: *const ::core::ffi::c_void) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GlobalHandle(pmem: *const ::core::ffi::c_void) -> isize;
        }
        ::core::mem::transmute(GlobalHandle(::core::mem::transmute(pmem)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
#[inline]
pub unsafe fn GlobalLock(hmem: isize) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GlobalLock(hmem: isize) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(GlobalLock(::core::mem::transmute(hmem)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
#[inline]
pub unsafe fn GlobalReAlloc(hmem: isize, dwbytes: usize, uflags: u32) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GlobalReAlloc(hmem: isize, dwbytes: usize, uflags: u32) -> isize;
        }
        ::core::mem::transmute(GlobalReAlloc(::core::mem::transmute(hmem), ::core::mem::transmute(dwbytes), ::core::mem::transmute(uflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
#[inline]
pub unsafe fn GlobalSize(hmem: isize) -> usize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GlobalSize(hmem: isize) -> usize;
        }
        ::core::mem::transmute(GlobalSize(::core::mem::transmute(hmem)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GlobalUnlock(hmem: isize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GlobalUnlock(hmem: isize) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GlobalUnlock(::core::mem::transmute(hmem)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HEAP_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const HEAP_NONE: HEAP_FLAGS = HEAP_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const HEAP_NO_SERIALIZE: HEAP_FLAGS = HEAP_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const HEAP_GROWABLE: HEAP_FLAGS = HEAP_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const HEAP_GENERATE_EXCEPTIONS: HEAP_FLAGS = HEAP_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const HEAP_ZERO_MEMORY: HEAP_FLAGS = HEAP_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const HEAP_REALLOC_IN_PLACE_ONLY: HEAP_FLAGS = HEAP_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const HEAP_TAIL_CHECKING_ENABLED: HEAP_FLAGS = HEAP_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const HEAP_FREE_CHECKING_ENABLED: HEAP_FLAGS = HEAP_FLAGS(64u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const HEAP_DISABLE_COALESCE_ON_FREE: HEAP_FLAGS = HEAP_FLAGS(128u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const HEAP_CREATE_ALIGN_16: HEAP_FLAGS = HEAP_FLAGS(65536u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const HEAP_CREATE_ENABLE_TRACING: HEAP_FLAGS = HEAP_FLAGS(131072u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const HEAP_CREATE_ENABLE_EXECUTE: HEAP_FLAGS = HEAP_FLAGS(262144u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const HEAP_MAXIMUM_TAG: HEAP_FLAGS = HEAP_FLAGS(4095u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const HEAP_PSEUDO_TAG_FLAG: HEAP_FLAGS = HEAP_FLAGS(32768u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const HEAP_TAG_SHIFT: HEAP_FLAGS = HEAP_FLAGS(18u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const HEAP_CREATE_SEGMENT_HEAP: HEAP_FLAGS = HEAP_FLAGS(256u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const HEAP_CREATE_HARDENED: HEAP_FLAGS = HEAP_FLAGS(512u32);
impl ::core::marker::Copy for HEAP_FLAGS {}
impl ::core::clone::Clone for HEAP_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HEAP_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for HEAP_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for HEAP_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HEAP_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for HEAP_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for HEAP_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for HEAP_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for HEAP_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for HEAP_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HEAP_INFORMATION_CLASS(pub i32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const HeapCompatibilityInformation: HEAP_INFORMATION_CLASS = HEAP_INFORMATION_CLASS(0i32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const HeapEnableTerminationOnCorruption: HEAP_INFORMATION_CLASS = HEAP_INFORMATION_CLASS(1i32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const HeapOptimizeResources: HEAP_INFORMATION_CLASS = HEAP_INFORMATION_CLASS(3i32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const HeapTag: HEAP_INFORMATION_CLASS = HEAP_INFORMATION_CLASS(7i32);
impl ::core::marker::Copy for HEAP_INFORMATION_CLASS {}
impl ::core::clone::Clone for HEAP_INFORMATION_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HEAP_INFORMATION_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for HEAP_INFORMATION_CLASS {
    type Abi = Self;
}
impl ::core::fmt::Debug for HEAP_INFORMATION_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HEAP_INFORMATION_CLASS").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub struct HEAP_SUMMARY {
    pub cb: u32,
    pub cbAllocated: usize,
    pub cbCommitted: usize,
    pub cbReserved: usize,
    pub cbMaxReserve: usize,
}
impl ::core::marker::Copy for HEAP_SUMMARY {}
impl ::core::clone::Clone for HEAP_SUMMARY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HEAP_SUMMARY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HEAP_SUMMARY").field("cb", &self.cb).field("cbAllocated", &self.cbAllocated).field("cbCommitted", &self.cbCommitted).field("cbReserved", &self.cbReserved).field("cbMaxReserve", &self.cbMaxReserve).finish()
    }
}
unsafe impl ::windows::core::Abi for HEAP_SUMMARY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HEAP_SUMMARY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HEAP_SUMMARY>()) == 0 }
    }
}
impl ::core::cmp::Eq for HEAP_SUMMARY {}
impl ::core::default::Default for HEAP_SUMMARY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
#[inline]
pub unsafe fn HeapAlloc<'a, Param0: ::windows::core::IntoParam<'a, HeapHandle>>(hheap: Param0, dwflags: HEAP_FLAGS, dwbytes: usize) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HeapAlloc(hheap: HeapHandle, dwflags: HEAP_FLAGS, dwbytes: usize) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(HeapAlloc(hheap.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwbytes)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
#[inline]
pub unsafe fn HeapCompact<'a, Param0: ::windows::core::IntoParam<'a, HeapHandle>>(hheap: Param0, dwflags: HEAP_FLAGS) -> usize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HeapCompact(hheap: HeapHandle, dwflags: HEAP_FLAGS) -> usize;
        }
        ::core::mem::transmute(HeapCompact(hheap.into_param().abi(), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
#[inline]
pub unsafe fn HeapCreate(floptions: HEAP_FLAGS, dwinitialsize: usize, dwmaximumsize: usize) -> ::windows::core::Result<HeapHandle> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HeapCreate(floptions: HEAP_FLAGS, dwinitialsize: usize, dwmaximumsize: usize) -> HeapHandle;
        }
        let result__ = HeapCreate(::core::mem::transmute(floptions), ::core::mem::transmute(dwinitialsize), ::core::mem::transmute(dwmaximumsize));
        (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HeapDestroy<'a, Param0: ::windows::core::IntoParam<'a, HeapHandle>>(hheap: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HeapDestroy(hheap: HeapHandle) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(HeapDestroy(hheap.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HeapFree<'a, Param0: ::windows::core::IntoParam<'a, HeapHandle>>(hheap: Param0, dwflags: HEAP_FLAGS, lpmem: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HeapFree(hheap: HeapHandle, dwflags: HEAP_FLAGS, lpmem: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(HeapFree(hheap.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(lpmem)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HeapHandle(pub isize);
impl HeapHandle {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HeapHandle {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HeapHandle {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HeapHandle {}
impl ::core::fmt::Debug for HeapHandle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HeapHandle").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Abi for HeapHandle {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HeapLock<'a, Param0: ::windows::core::IntoParam<'a, HeapHandle>>(hheap: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HeapLock(hheap: HeapHandle) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(HeapLock(hheap.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HeapQueryInformation<'a, Param0: ::windows::core::IntoParam<'a, HeapHandle>>(heaphandle: Param0, heapinformationclass: HEAP_INFORMATION_CLASS, heapinformation: *mut ::core::ffi::c_void, heapinformationlength: usize, returnlength: *mut usize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HeapQueryInformation(heaphandle: HeapHandle, heapinformationclass: HEAP_INFORMATION_CLASS, heapinformation: *mut ::core::ffi::c_void, heapinformationlength: usize, returnlength: *mut usize) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(HeapQueryInformation(heaphandle.into_param().abi(), ::core::mem::transmute(heapinformationclass), ::core::mem::transmute(heapinformation), ::core::mem::transmute(heapinformationlength), ::core::mem::transmute(returnlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
#[inline]
pub unsafe fn HeapReAlloc<'a, Param0: ::windows::core::IntoParam<'a, HeapHandle>>(hheap: Param0, dwflags: HEAP_FLAGS, lpmem: *const ::core::ffi::c_void, dwbytes: usize) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HeapReAlloc(hheap: HeapHandle, dwflags: HEAP_FLAGS, lpmem: *const ::core::ffi::c_void, dwbytes: usize) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(HeapReAlloc(hheap.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(lpmem), ::core::mem::transmute(dwbytes)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HeapSetInformation<'a, Param0: ::windows::core::IntoParam<'a, HeapHandle>>(heaphandle: Param0, heapinformationclass: HEAP_INFORMATION_CLASS, heapinformation: *const ::core::ffi::c_void, heapinformationlength: usize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HeapSetInformation(heaphandle: HeapHandle, heapinformationclass: HEAP_INFORMATION_CLASS, heapinformation: *const ::core::ffi::c_void, heapinformationlength: usize) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(HeapSetInformation(heaphandle.into_param().abi(), ::core::mem::transmute(heapinformationclass), ::core::mem::transmute(heapinformation), ::core::mem::transmute(heapinformationlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
#[inline]
pub unsafe fn HeapSize<'a, Param0: ::windows::core::IntoParam<'a, HeapHandle>>(hheap: Param0, dwflags: HEAP_FLAGS, lpmem: *const ::core::ffi::c_void) -> usize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HeapSize(hheap: HeapHandle, dwflags: HEAP_FLAGS, lpmem: *const ::core::ffi::c_void) -> usize;
        }
        ::core::mem::transmute(HeapSize(hheap.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(lpmem)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HeapSummary<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hheap: Param0, dwflags: u32, lpsummary: *mut HEAP_SUMMARY) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HeapSummary(hheap: super::super::Foundation::HANDLE, dwflags: u32, lpsummary: *mut HEAP_SUMMARY) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(HeapSummary(hheap.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(lpsummary)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HeapUnlock<'a, Param0: ::windows::core::IntoParam<'a, HeapHandle>>(hheap: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HeapUnlock(hheap: HeapHandle) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(HeapUnlock(hheap.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HeapValidate<'a, Param0: ::windows::core::IntoParam<'a, HeapHandle>>(hheap: Param0, dwflags: HEAP_FLAGS, lpmem: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HeapValidate(hheap: HeapHandle, dwflags: HEAP_FLAGS, lpmem: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(HeapValidate(hheap.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(lpmem)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HeapWalk<'a, Param0: ::windows::core::IntoParam<'a, HeapHandle>>(hheap: Param0, lpentry: *mut PROCESS_HEAP_ENTRY) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HeapWalk(hheap: HeapHandle, lpentry: *mut PROCESS_HEAP_ENTRY) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(HeapWalk(hheap.into_param().abi(), ::core::mem::transmute(lpentry)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsBadCodePtr(lpfn: super::super::Foundation::FARPROC) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsBadCodePtr(lpfn: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(IsBadCodePtr(::core::mem::transmute(lpfn)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsBadReadPtr(lp: *const ::core::ffi::c_void, ucb: usize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsBadReadPtr(lp: *const ::core::ffi::c_void, ucb: usize) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(IsBadReadPtr(::core::mem::transmute(lp), ::core::mem::transmute(ucb)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsBadStringPtrA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(lpsz: Param0, ucchmax: usize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsBadStringPtrA(lpsz: ::windows::core::PCSTR, ucchmax: usize) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(IsBadStringPtrA(lpsz.into_param().abi(), ::core::mem::transmute(ucchmax)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsBadStringPtrW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(lpsz: Param0, ucchmax: usize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsBadStringPtrW(lpsz: ::windows::core::PCWSTR, ucchmax: usize) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(IsBadStringPtrW(lpsz.into_param().abi(), ::core::mem::transmute(ucchmax)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsBadWritePtr(lp: *const ::core::ffi::c_void, ucb: usize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsBadWritePtr(lp: *const ::core::ffi::c_void, ucb: usize) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(IsBadWritePtr(::core::mem::transmute(lp), ::core::mem::transmute(ucb)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct LOCAL_ALLOC_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const LHND: LOCAL_ALLOC_FLAGS = LOCAL_ALLOC_FLAGS(66u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const LMEM_FIXED: LOCAL_ALLOC_FLAGS = LOCAL_ALLOC_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const LMEM_MOVEABLE: LOCAL_ALLOC_FLAGS = LOCAL_ALLOC_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const LMEM_ZEROINIT: LOCAL_ALLOC_FLAGS = LOCAL_ALLOC_FLAGS(64u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const LPTR: LOCAL_ALLOC_FLAGS = LOCAL_ALLOC_FLAGS(64u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const NONZEROLHND: LOCAL_ALLOC_FLAGS = LOCAL_ALLOC_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const NONZEROLPTR: LOCAL_ALLOC_FLAGS = LOCAL_ALLOC_FLAGS(0u32);
impl ::core::marker::Copy for LOCAL_ALLOC_FLAGS {}
impl ::core::clone::Clone for LOCAL_ALLOC_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LOCAL_ALLOC_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for LOCAL_ALLOC_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for LOCAL_ALLOC_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LOCAL_ALLOC_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for LOCAL_ALLOC_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for LOCAL_ALLOC_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for LOCAL_ALLOC_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for LOCAL_ALLOC_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for LOCAL_ALLOC_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
#[inline]
pub unsafe fn LocalAlloc(uflags: LOCAL_ALLOC_FLAGS, ubytes: usize) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LocalAlloc(uflags: LOCAL_ALLOC_FLAGS, ubytes: usize) -> isize;
        }
        ::core::mem::transmute(LocalAlloc(::core::mem::transmute(uflags), ::core::mem::transmute(ubytes)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
#[inline]
pub unsafe fn LocalFlags(hmem: isize) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LocalFlags(hmem: isize) -> u32;
        }
        ::core::mem::transmute(LocalFlags(::core::mem::transmute(hmem)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
#[inline]
pub unsafe fn LocalFree(hmem: isize) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LocalFree(hmem: isize) -> isize;
        }
        ::core::mem::transmute(LocalFree(::core::mem::transmute(hmem)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
#[inline]
pub unsafe fn LocalHandle(pmem: *const ::core::ffi::c_void) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LocalHandle(pmem: *const ::core::ffi::c_void) -> isize;
        }
        ::core::mem::transmute(LocalHandle(::core::mem::transmute(pmem)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
#[inline]
pub unsafe fn LocalLock(hmem: isize) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LocalLock(hmem: isize) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(LocalLock(::core::mem::transmute(hmem)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
#[inline]
pub unsafe fn LocalReAlloc(hmem: isize, ubytes: usize, uflags: u32) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LocalReAlloc(hmem: isize, ubytes: usize, uflags: u32) -> isize;
        }
        ::core::mem::transmute(LocalReAlloc(::core::mem::transmute(hmem), ::core::mem::transmute(ubytes), ::core::mem::transmute(uflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
#[inline]
pub unsafe fn LocalSize(hmem: isize) -> usize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LocalSize(hmem: isize) -> usize;
        }
        ::core::mem::transmute(LocalSize(::core::mem::transmute(hmem)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LocalUnlock(hmem: isize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LocalUnlock(hmem: isize) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(LocalUnlock(::core::mem::transmute(hmem)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const MEHC_PATROL_SCRUBBER_PRESENT: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct MEMORY_BASIC_INFORMATION {
    pub BaseAddress: *mut ::core::ffi::c_void,
    pub AllocationBase: *mut ::core::ffi::c_void,
    pub AllocationProtect: PAGE_PROTECTION_FLAGS,
    pub PartitionId: u16,
    pub RegionSize: usize,
    pub State: VIRTUAL_ALLOCATION_TYPE,
    pub Protect: PAGE_PROTECTION_FLAGS,
    pub Type: PAGE_TYPE,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for MEMORY_BASIC_INFORMATION {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for MEMORY_BASIC_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::fmt::Debug for MEMORY_BASIC_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MEMORY_BASIC_INFORMATION").field("BaseAddress", &self.BaseAddress).field("AllocationBase", &self.AllocationBase).field("AllocationProtect", &self.AllocationProtect).field("PartitionId", &self.PartitionId).field("RegionSize", &self.RegionSize).field("State", &self.State).field("Protect", &self.Protect).field("Type", &self.Type).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
unsafe impl ::windows::core::Abi for MEMORY_BASIC_INFORMATION {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for MEMORY_BASIC_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MEMORY_BASIC_INFORMATION>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for MEMORY_BASIC_INFORMATION {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for MEMORY_BASIC_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
#[cfg(target_arch = "x86")]
pub struct MEMORY_BASIC_INFORMATION {
    pub BaseAddress: *mut ::core::ffi::c_void,
    pub AllocationBase: *mut ::core::ffi::c_void,
    pub AllocationProtect: PAGE_PROTECTION_FLAGS,
    pub RegionSize: usize,
    pub State: VIRTUAL_ALLOCATION_TYPE,
    pub Protect: PAGE_PROTECTION_FLAGS,
    pub Type: PAGE_TYPE,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for MEMORY_BASIC_INFORMATION {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for MEMORY_BASIC_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
impl ::core::fmt::Debug for MEMORY_BASIC_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MEMORY_BASIC_INFORMATION").field("BaseAddress", &self.BaseAddress).field("AllocationBase", &self.AllocationBase).field("AllocationProtect", &self.AllocationProtect).field("RegionSize", &self.RegionSize).field("State", &self.State).field("Protect", &self.Protect).field("Type", &self.Type).finish()
    }
}
#[cfg(target_arch = "x86")]
unsafe impl ::windows::core::Abi for MEMORY_BASIC_INFORMATION {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::PartialEq for MEMORY_BASIC_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MEMORY_BASIC_INFORMATION>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::Eq for MEMORY_BASIC_INFORMATION {}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for MEMORY_BASIC_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub struct MEMORY_BASIC_INFORMATION32 {
    pub BaseAddress: u32,
    pub AllocationBase: u32,
    pub AllocationProtect: PAGE_PROTECTION_FLAGS,
    pub RegionSize: u32,
    pub State: VIRTUAL_ALLOCATION_TYPE,
    pub Protect: PAGE_PROTECTION_FLAGS,
    pub Type: PAGE_TYPE,
}
impl ::core::marker::Copy for MEMORY_BASIC_INFORMATION32 {}
impl ::core::clone::Clone for MEMORY_BASIC_INFORMATION32 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MEMORY_BASIC_INFORMATION32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MEMORY_BASIC_INFORMATION32").field("BaseAddress", &self.BaseAddress).field("AllocationBase", &self.AllocationBase).field("AllocationProtect", &self.AllocationProtect).field("RegionSize", &self.RegionSize).field("State", &self.State).field("Protect", &self.Protect).field("Type", &self.Type).finish()
    }
}
unsafe impl ::windows::core::Abi for MEMORY_BASIC_INFORMATION32 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MEMORY_BASIC_INFORMATION32 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MEMORY_BASIC_INFORMATION32>()) == 0 }
    }
}
impl ::core::cmp::Eq for MEMORY_BASIC_INFORMATION32 {}
impl ::core::default::Default for MEMORY_BASIC_INFORMATION32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub struct MEMORY_BASIC_INFORMATION64 {
    pub BaseAddress: u64,
    pub AllocationBase: u64,
    pub AllocationProtect: PAGE_PROTECTION_FLAGS,
    pub __alignment1: u32,
    pub RegionSize: u64,
    pub State: VIRTUAL_ALLOCATION_TYPE,
    pub Protect: PAGE_PROTECTION_FLAGS,
    pub Type: PAGE_TYPE,
    pub __alignment2: u32,
}
impl ::core::marker::Copy for MEMORY_BASIC_INFORMATION64 {}
impl ::core::clone::Clone for MEMORY_BASIC_INFORMATION64 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MEMORY_BASIC_INFORMATION64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MEMORY_BASIC_INFORMATION64").field("BaseAddress", &self.BaseAddress).field("AllocationBase", &self.AllocationBase).field("AllocationProtect", &self.AllocationProtect).field("__alignment1", &self.__alignment1).field("RegionSize", &self.RegionSize).field("State", &self.State).field("Protect", &self.Protect).field("Type", &self.Type).field("__alignment2", &self.__alignment2).finish()
    }
}
unsafe impl ::windows::core::Abi for MEMORY_BASIC_INFORMATION64 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MEMORY_BASIC_INFORMATION64 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MEMORY_BASIC_INFORMATION64>()) == 0 }
    }
}
impl ::core::cmp::Eq for MEMORY_BASIC_INFORMATION64 {}
impl ::core::default::Default for MEMORY_BASIC_INFORMATION64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MEMORY_RESOURCE_NOTIFICATION_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const LowMemoryResourceNotification: MEMORY_RESOURCE_NOTIFICATION_TYPE = MEMORY_RESOURCE_NOTIFICATION_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const HighMemoryResourceNotification: MEMORY_RESOURCE_NOTIFICATION_TYPE = MEMORY_RESOURCE_NOTIFICATION_TYPE(1i32);
impl ::core::marker::Copy for MEMORY_RESOURCE_NOTIFICATION_TYPE {}
impl ::core::clone::Clone for MEMORY_RESOURCE_NOTIFICATION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MEMORY_RESOURCE_NOTIFICATION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MEMORY_RESOURCE_NOTIFICATION_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for MEMORY_RESOURCE_NOTIFICATION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MEMORY_RESOURCE_NOTIFICATION_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub struct MEM_ADDRESS_REQUIREMENTS {
    pub LowestStartingAddress: *mut ::core::ffi::c_void,
    pub HighestEndingAddress: *mut ::core::ffi::c_void,
    pub Alignment: usize,
}
impl ::core::marker::Copy for MEM_ADDRESS_REQUIREMENTS {}
impl ::core::clone::Clone for MEM_ADDRESS_REQUIREMENTS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MEM_ADDRESS_REQUIREMENTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MEM_ADDRESS_REQUIREMENTS").field("LowestStartingAddress", &self.LowestStartingAddress).field("HighestEndingAddress", &self.HighestEndingAddress).field("Alignment", &self.Alignment).finish()
    }
}
unsafe impl ::windows::core::Abi for MEM_ADDRESS_REQUIREMENTS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MEM_ADDRESS_REQUIREMENTS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MEM_ADDRESS_REQUIREMENTS>()) == 0 }
    }
}
impl ::core::cmp::Eq for MEM_ADDRESS_REQUIREMENTS {}
impl ::core::default::Default for MEM_ADDRESS_REQUIREMENTS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MEM_EXTENDED_PARAMETER {
    pub Anonymous1: MEM_EXTENDED_PARAMETER_0,
    pub Anonymous2: MEM_EXTENDED_PARAMETER_1,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MEM_EXTENDED_PARAMETER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MEM_EXTENDED_PARAMETER {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MEM_EXTENDED_PARAMETER {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MEM_EXTENDED_PARAMETER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MEM_EXTENDED_PARAMETER>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MEM_EXTENDED_PARAMETER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MEM_EXTENDED_PARAMETER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MEM_EXTENDED_PARAMETER_0 {
    pub _bitfield: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MEM_EXTENDED_PARAMETER_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MEM_EXTENDED_PARAMETER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MEM_EXTENDED_PARAMETER_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MEM_EXTENDED_PARAMETER_0").field("_bitfield", &self._bitfield).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MEM_EXTENDED_PARAMETER_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MEM_EXTENDED_PARAMETER_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MEM_EXTENDED_PARAMETER_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MEM_EXTENDED_PARAMETER_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MEM_EXTENDED_PARAMETER_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union MEM_EXTENDED_PARAMETER_1 {
    pub ULong64: u64,
    pub Pointer: *mut ::core::ffi::c_void,
    pub Size: usize,
    pub Handle: super::super::Foundation::HANDLE,
    pub ULong: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MEM_EXTENDED_PARAMETER_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MEM_EXTENDED_PARAMETER_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MEM_EXTENDED_PARAMETER_1 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MEM_EXTENDED_PARAMETER_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MEM_EXTENDED_PARAMETER_1>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MEM_EXTENDED_PARAMETER_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MEM_EXTENDED_PARAMETER_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MEM_EXTENDED_PARAMETER_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const MemExtendedParameterInvalidType: MEM_EXTENDED_PARAMETER_TYPE = MEM_EXTENDED_PARAMETER_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const MemExtendedParameterAddressRequirements: MEM_EXTENDED_PARAMETER_TYPE = MEM_EXTENDED_PARAMETER_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const MemExtendedParameterNumaNode: MEM_EXTENDED_PARAMETER_TYPE = MEM_EXTENDED_PARAMETER_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const MemExtendedParameterPartitionHandle: MEM_EXTENDED_PARAMETER_TYPE = MEM_EXTENDED_PARAMETER_TYPE(3i32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const MemExtendedParameterUserPhysicalHandle: MEM_EXTENDED_PARAMETER_TYPE = MEM_EXTENDED_PARAMETER_TYPE(4i32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const MemExtendedParameterAttributeFlags: MEM_EXTENDED_PARAMETER_TYPE = MEM_EXTENDED_PARAMETER_TYPE(5i32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const MemExtendedParameterImageMachine: MEM_EXTENDED_PARAMETER_TYPE = MEM_EXTENDED_PARAMETER_TYPE(6i32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const MemExtendedParameterMax: MEM_EXTENDED_PARAMETER_TYPE = MEM_EXTENDED_PARAMETER_TYPE(7i32);
impl ::core::marker::Copy for MEM_EXTENDED_PARAMETER_TYPE {}
impl ::core::clone::Clone for MEM_EXTENDED_PARAMETER_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MEM_EXTENDED_PARAMETER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MEM_EXTENDED_PARAMETER_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for MEM_EXTENDED_PARAMETER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MEM_EXTENDED_PARAMETER_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MapUserPhysicalPages(virtualaddress: *const ::core::ffi::c_void, pagearray: &[usize]) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MapUserPhysicalPages(virtualaddress: *const ::core::ffi::c_void, numberofpages: usize, pagearray: *const usize) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(MapUserPhysicalPages(::core::mem::transmute(virtualaddress), pagearray.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(pagearray))))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MapUserPhysicalPagesScatter(virtualaddresses: *const *const ::core::ffi::c_void, numberofpages: usize, pagearray: *const usize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MapUserPhysicalPagesScatter(virtualaddresses: *const *const ::core::ffi::c_void, numberofpages: usize, pagearray: *const usize) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(MapUserPhysicalPagesScatter(::core::mem::transmute(virtualaddresses), ::core::mem::transmute(numberofpages), ::core::mem::transmute(pagearray)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MapViewOfFile<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hfilemappingobject: Param0, dwdesiredaccess: FILE_MAP, dwfileoffsethigh: u32, dwfileoffsetlow: u32, dwnumberofbytestomap: usize) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MapViewOfFile(hfilemappingobject: super::super::Foundation::HANDLE, dwdesiredaccess: FILE_MAP, dwfileoffsethigh: u32, dwfileoffsetlow: u32, dwnumberofbytestomap: usize) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(MapViewOfFile(hfilemappingobject.into_param().abi(), ::core::mem::transmute(dwdesiredaccess), ::core::mem::transmute(dwfileoffsethigh), ::core::mem::transmute(dwfileoffsetlow), ::core::mem::transmute(dwnumberofbytestomap)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MapViewOfFile3<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(filemapping: Param0, process: Param1, baseaddress: *const ::core::ffi::c_void, offset: u64, viewsize: usize, allocationtype: VIRTUAL_ALLOCATION_TYPE, pageprotection: u32, extendedparameters: &mut [MEM_EXTENDED_PARAMETER]) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MapViewOfFile3(filemapping: super::super::Foundation::HANDLE, process: super::super::Foundation::HANDLE, baseaddress: *const ::core::ffi::c_void, offset: u64, viewsize: usize, allocationtype: VIRTUAL_ALLOCATION_TYPE, pageprotection: u32, extendedparameters: *mut MEM_EXTENDED_PARAMETER, parametercount: u32) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(MapViewOfFile3(filemapping.into_param().abi(), process.into_param().abi(), ::core::mem::transmute(baseaddress), ::core::mem::transmute(offset), ::core::mem::transmute(viewsize), ::core::mem::transmute(allocationtype), ::core::mem::transmute(pageprotection), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(extendedparameters)), extendedparameters.len() as _))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MapViewOfFile3FromApp<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(filemapping: Param0, process: Param1, baseaddress: *const ::core::ffi::c_void, offset: u64, viewsize: usize, allocationtype: VIRTUAL_ALLOCATION_TYPE, pageprotection: u32, extendedparameters: &mut [MEM_EXTENDED_PARAMETER]) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MapViewOfFile3FromApp(filemapping: super::super::Foundation::HANDLE, process: super::super::Foundation::HANDLE, baseaddress: *const ::core::ffi::c_void, offset: u64, viewsize: usize, allocationtype: VIRTUAL_ALLOCATION_TYPE, pageprotection: u32, extendedparameters: *mut MEM_EXTENDED_PARAMETER, parametercount: u32) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(MapViewOfFile3FromApp(filemapping.into_param().abi(), process.into_param().abi(), ::core::mem::transmute(baseaddress), ::core::mem::transmute(offset), ::core::mem::transmute(viewsize), ::core::mem::transmute(allocationtype), ::core::mem::transmute(pageprotection), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(extendedparameters)), extendedparameters.len() as _))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MapViewOfFileEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hfilemappingobject: Param0, dwdesiredaccess: FILE_MAP, dwfileoffsethigh: u32, dwfileoffsetlow: u32, dwnumberofbytestomap: usize, lpbaseaddress: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MapViewOfFileEx(hfilemappingobject: super::super::Foundation::HANDLE, dwdesiredaccess: FILE_MAP, dwfileoffsethigh: u32, dwfileoffsetlow: u32, dwnumberofbytestomap: usize, lpbaseaddress: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(MapViewOfFileEx(hfilemappingobject.into_param().abi(), ::core::mem::transmute(dwdesiredaccess), ::core::mem::transmute(dwfileoffsethigh), ::core::mem::transmute(dwfileoffsetlow), ::core::mem::transmute(dwnumberofbytestomap), ::core::mem::transmute(lpbaseaddress)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MapViewOfFileExNuma<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hfilemappingobject: Param0, dwdesiredaccess: FILE_MAP, dwfileoffsethigh: u32, dwfileoffsetlow: u32, dwnumberofbytestomap: usize, lpbaseaddress: *const ::core::ffi::c_void, nndpreferred: u32) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MapViewOfFileExNuma(hfilemappingobject: super::super::Foundation::HANDLE, dwdesiredaccess: FILE_MAP, dwfileoffsethigh: u32, dwfileoffsetlow: u32, dwnumberofbytestomap: usize, lpbaseaddress: *const ::core::ffi::c_void, nndpreferred: u32) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(MapViewOfFileExNuma(hfilemappingobject.into_param().abi(), ::core::mem::transmute(dwdesiredaccess), ::core::mem::transmute(dwfileoffsethigh), ::core::mem::transmute(dwfileoffsetlow), ::core::mem::transmute(dwnumberofbytestomap), ::core::mem::transmute(lpbaseaddress), ::core::mem::transmute(nndpreferred)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MapViewOfFileFromApp<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hfilemappingobject: Param0, desiredaccess: FILE_MAP, fileoffset: u64, numberofbytestomap: usize) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MapViewOfFileFromApp(hfilemappingobject: super::super::Foundation::HANDLE, desiredaccess: FILE_MAP, fileoffset: u64, numberofbytestomap: usize) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(MapViewOfFileFromApp(hfilemappingobject.into_param().abi(), ::core::mem::transmute(desiredaccess), ::core::mem::transmute(fileoffset), ::core::mem::transmute(numberofbytestomap)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MapViewOfFileNuma2<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(filemappinghandle: Param0, processhandle: Param1, offset: u64, baseaddress: *const ::core::ffi::c_void, viewsize: usize, allocationtype: u32, pageprotection: u32, preferrednode: u32) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MapViewOfFileNuma2(filemappinghandle: super::super::Foundation::HANDLE, processhandle: super::super::Foundation::HANDLE, offset: u64, baseaddress: *const ::core::ffi::c_void, viewsize: usize, allocationtype: u32, pageprotection: u32, preferrednode: u32) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(MapViewOfFileNuma2(filemappinghandle.into_param().abi(), processhandle.into_param().abi(), ::core::mem::transmute(offset), ::core::mem::transmute(baseaddress), ::core::mem::transmute(viewsize), ::core::mem::transmute(allocationtype), ::core::mem::transmute(pageprotection), ::core::mem::transmute(preferrednode)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct OFFER_PRIORITY(pub i32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const VmOfferPriorityVeryLow: OFFER_PRIORITY = OFFER_PRIORITY(1i32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const VmOfferPriorityLow: OFFER_PRIORITY = OFFER_PRIORITY(2i32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const VmOfferPriorityBelowNormal: OFFER_PRIORITY = OFFER_PRIORITY(3i32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const VmOfferPriorityNormal: OFFER_PRIORITY = OFFER_PRIORITY(4i32);
impl ::core::marker::Copy for OFFER_PRIORITY {}
impl ::core::clone::Clone for OFFER_PRIORITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for OFFER_PRIORITY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for OFFER_PRIORITY {
    type Abi = Self;
}
impl ::core::fmt::Debug for OFFER_PRIORITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OFFER_PRIORITY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
#[inline]
pub unsafe fn OfferVirtualMemory(virtualaddress: &mut [u8], priority: OFFER_PRIORITY) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OfferVirtualMemory(virtualaddress: *mut ::core::ffi::c_void, size: usize, priority: OFFER_PRIORITY) -> u32;
        }
        ::core::mem::transmute(OfferVirtualMemory(::core::mem::transmute(::windows::core::as_mut_ptr_or_null(virtualaddress)), virtualaddress.len() as _, ::core::mem::transmute(priority)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenDedicatedMemoryPartition<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(partition: Param0, dedicatedmemorytypeid: u64, desiredaccess: u32, inherithandle: Param3) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenDedicatedMemoryPartition(partition: super::super::Foundation::HANDLE, dedicatedmemorytypeid: u64, desiredaccess: u32, inherithandle: super::super::Foundation::BOOL) -> super::super::Foundation::HANDLE;
        }
        ::core::mem::transmute(OpenDedicatedMemoryPartition(partition.into_param().abi(), ::core::mem::transmute(dedicatedmemorytypeid), ::core::mem::transmute(desiredaccess), inherithandle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenFileMappingA<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(dwdesiredaccess: u32, binherithandle: Param1, lpname: Param2) -> ::windows::core::Result<super::super::Foundation::HANDLE> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenFileMappingA(dwdesiredaccess: u32, binherithandle: super::super::Foundation::BOOL, lpname: ::windows::core::PCSTR) -> super::super::Foundation::HANDLE;
        }
        let result__ = OpenFileMappingA(::core::mem::transmute(dwdesiredaccess), binherithandle.into_param().abi(), lpname.into_param().abi());
        (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenFileMappingFromApp<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(desiredaccess: u32, inherithandle: Param1, name: Param2) -> ::windows::core::Result<super::super::Foundation::HANDLE> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenFileMappingFromApp(desiredaccess: u32, inherithandle: super::super::Foundation::BOOL, name: ::windows::core::PCWSTR) -> super::super::Foundation::HANDLE;
        }
        let result__ = OpenFileMappingFromApp(::core::mem::transmute(desiredaccess), inherithandle.into_param().abi(), name.into_param().abi());
        (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenFileMappingW<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(dwdesiredaccess: u32, binherithandle: Param1, lpname: Param2) -> ::windows::core::Result<super::super::Foundation::HANDLE> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenFileMappingW(dwdesiredaccess: u32, binherithandle: super::super::Foundation::BOOL, lpname: ::windows::core::PCWSTR) -> super::super::Foundation::HANDLE;
        }
        let result__ = OpenFileMappingW(::core::mem::transmute(dwdesiredaccess), binherithandle.into_param().abi(), lpname.into_param().abi());
        (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PAGE_PROTECTION_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const PAGE_NOACCESS: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const PAGE_READONLY: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const PAGE_READWRITE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const PAGE_WRITECOPY: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const PAGE_EXECUTE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const PAGE_EXECUTE_READ: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const PAGE_EXECUTE_READWRITE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(64u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const PAGE_EXECUTE_WRITECOPY: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(128u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const PAGE_GUARD: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(256u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const PAGE_NOCACHE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(512u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const PAGE_WRITECOMBINE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(1024u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const PAGE_GRAPHICS_NOACCESS: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(2048u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const PAGE_GRAPHICS_READONLY: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(4096u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const PAGE_GRAPHICS_READWRITE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(8192u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const PAGE_GRAPHICS_EXECUTE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(16384u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const PAGE_GRAPHICS_EXECUTE_READ: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(32768u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const PAGE_GRAPHICS_EXECUTE_READWRITE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(65536u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const PAGE_GRAPHICS_COHERENT: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(131072u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const PAGE_GRAPHICS_NOCACHE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(262144u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const PAGE_ENCLAVE_THREAD_CONTROL: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(2147483648u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const PAGE_REVERT_TO_FILE_MAP: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(2147483648u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const PAGE_TARGETS_NO_UPDATE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(1073741824u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const PAGE_TARGETS_INVALID: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(1073741824u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const PAGE_ENCLAVE_UNVALIDATED: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(536870912u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const PAGE_ENCLAVE_MASK: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(268435456u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const PAGE_ENCLAVE_DECOMMIT: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(268435456u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const PAGE_ENCLAVE_SS_FIRST: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(268435457u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const PAGE_ENCLAVE_SS_REST: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(268435458u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const SEC_PARTITION_OWNER_HANDLE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(262144u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const SEC_64K_PAGES: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(524288u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const SEC_FILE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(8388608u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const SEC_IMAGE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(16777216u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const SEC_PROTECTED_IMAGE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(33554432u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const SEC_RESERVE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(67108864u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const SEC_COMMIT: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(134217728u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const SEC_NOCACHE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(268435456u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const SEC_WRITECOMBINE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(1073741824u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const SEC_LARGE_PAGES: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(2147483648u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const SEC_IMAGE_NO_EXECUTE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(285212672u32);
impl ::core::marker::Copy for PAGE_PROTECTION_FLAGS {}
impl ::core::clone::Clone for PAGE_PROTECTION_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PAGE_PROTECTION_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PAGE_PROTECTION_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for PAGE_PROTECTION_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PAGE_PROTECTION_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PAGE_PROTECTION_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PAGE_PROTECTION_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PAGE_PROTECTION_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PAGE_PROTECTION_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PAGE_PROTECTION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PAGE_TYPE(pub u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const MEM_PRIVATE: PAGE_TYPE = PAGE_TYPE(131072u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const MEM_MAPPED: PAGE_TYPE = PAGE_TYPE(262144u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const MEM_IMAGE: PAGE_TYPE = PAGE_TYPE(16777216u32);
impl ::core::marker::Copy for PAGE_TYPE {}
impl ::core::clone::Clone for PAGE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PAGE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PAGE_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for PAGE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PAGE_TYPE").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PAGE_TYPE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PAGE_TYPE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PAGE_TYPE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PAGE_TYPE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PAGE_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub type PBAD_MEMORY_CALLBACK_ROUTINE = ::core::option::Option<unsafe extern "system" fn()>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PROCESS_HEAP_ENTRY {
    pub lpData: *mut ::core::ffi::c_void,
    pub cbData: u32,
    pub cbOverhead: u8,
    pub iRegionIndex: u8,
    pub wFlags: u16,
    pub Anonymous: PROCESS_HEAP_ENTRY_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PROCESS_HEAP_ENTRY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PROCESS_HEAP_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PROCESS_HEAP_ENTRY {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PROCESS_HEAP_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PROCESS_HEAP_ENTRY>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PROCESS_HEAP_ENTRY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PROCESS_HEAP_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union PROCESS_HEAP_ENTRY_0 {
    pub Block: PROCESS_HEAP_ENTRY_0_0,
    pub Region: PROCESS_HEAP_ENTRY_0_1,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PROCESS_HEAP_ENTRY_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PROCESS_HEAP_ENTRY_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PROCESS_HEAP_ENTRY_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PROCESS_HEAP_ENTRY_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PROCESS_HEAP_ENTRY_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PROCESS_HEAP_ENTRY_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PROCESS_HEAP_ENTRY_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PROCESS_HEAP_ENTRY_0_0 {
    pub hMem: super::super::Foundation::HANDLE,
    pub dwReserved: [u32; 3],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PROCESS_HEAP_ENTRY_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PROCESS_HEAP_ENTRY_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PROCESS_HEAP_ENTRY_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROCESS_HEAP_ENTRY_0_0").field("hMem", &self.hMem).field("dwReserved", &self.dwReserved).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PROCESS_HEAP_ENTRY_0_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PROCESS_HEAP_ENTRY_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PROCESS_HEAP_ENTRY_0_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PROCESS_HEAP_ENTRY_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PROCESS_HEAP_ENTRY_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PROCESS_HEAP_ENTRY_0_1 {
    pub dwCommittedSize: u32,
    pub dwUnCommittedSize: u32,
    pub lpFirstBlock: *mut ::core::ffi::c_void,
    pub lpLastBlock: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PROCESS_HEAP_ENTRY_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PROCESS_HEAP_ENTRY_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PROCESS_HEAP_ENTRY_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROCESS_HEAP_ENTRY_0_1").field("dwCommittedSize", &self.dwCommittedSize).field("dwUnCommittedSize", &self.dwUnCommittedSize).field("lpFirstBlock", &self.lpFirstBlock).field("lpLastBlock", &self.lpLastBlock).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PROCESS_HEAP_ENTRY_0_1 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PROCESS_HEAP_ENTRY_0_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PROCESS_HEAP_ENTRY_0_1>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PROCESS_HEAP_ENTRY_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PROCESS_HEAP_ENTRY_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PSECURE_MEMORY_CACHE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(addr: *const ::core::ffi::c_void, range: usize) -> super::super::Foundation::BOOLEAN>;
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PrefetchVirtualMemory<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hprocess: Param0, virtualaddresses: &[WIN32_MEMORY_RANGE_ENTRY], flags: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PrefetchVirtualMemory(hprocess: super::super::Foundation::HANDLE, numberofentries: usize, virtualaddresses: *const WIN32_MEMORY_RANGE_ENTRY, flags: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(PrefetchVirtualMemory(hprocess.into_param().abi(), virtualaddresses.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(virtualaddresses)), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn QueryMemoryResourceNotification<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(resourcenotificationhandle: Param0, resourcestate: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryMemoryResourceNotification(resourcenotificationhandle: super::super::Foundation::HANDLE, resourcestate: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(QueryMemoryResourceNotification(resourcenotificationhandle.into_param().abi(), ::core::mem::transmute(resourcestate)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn QueryPartitionInformation<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(partition: Param0, partitioninformationclass: WIN32_MEMORY_PARTITION_INFORMATION_CLASS, partitioninformation: *mut ::core::ffi::c_void, partitioninformationlength: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryPartitionInformation(partition: super::super::Foundation::HANDLE, partitioninformationclass: WIN32_MEMORY_PARTITION_INFORMATION_CLASS, partitioninformation: *mut ::core::ffi::c_void, partitioninformationlength: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(QueryPartitionInformation(partition.into_param().abi(), ::core::mem::transmute(partitioninformationclass), ::core::mem::transmute(partitioninformation), ::core::mem::transmute(partitioninformationlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn QueryVirtualMemoryInformation<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(process: Param0, virtualaddress: *const ::core::ffi::c_void, memoryinformationclass: WIN32_MEMORY_INFORMATION_CLASS, memoryinformation: *mut ::core::ffi::c_void, memoryinformationsize: usize, returnsize: *mut usize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryVirtualMemoryInformation(process: super::super::Foundation::HANDLE, virtualaddress: *const ::core::ffi::c_void, memoryinformationclass: WIN32_MEMORY_INFORMATION_CLASS, memoryinformation: *mut ::core::ffi::c_void, memoryinformationsize: usize, returnsize: *mut usize) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(QueryVirtualMemoryInformation(process.into_param().abi(), ::core::mem::transmute(virtualaddress), ::core::mem::transmute(memoryinformationclass), ::core::mem::transmute(memoryinformation), ::core::mem::transmute(memoryinformationsize), ::core::mem::transmute(returnsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
#[inline]
pub unsafe fn ReclaimVirtualMemory(virtualaddress: &[u8]) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReclaimVirtualMemory(virtualaddress: *const ::core::ffi::c_void, size: usize) -> u32;
        }
        ::core::mem::transmute(ReclaimVirtualMemory(::core::mem::transmute(::windows::core::as_ptr_or_null(virtualaddress)), virtualaddress.len() as _))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
#[inline]
pub unsafe fn RegisterBadMemoryNotification(callback: PBAD_MEMORY_CALLBACK_ROUTINE) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegisterBadMemoryNotification(callback: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(RegisterBadMemoryNotification(::core::mem::transmute(callback)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RemoveSecureMemoryCacheCallback(pfncallback: PSECURE_MEMORY_CACHE_CALLBACK) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RemoveSecureMemoryCacheCallback(pfncallback: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(RemoveSecureMemoryCacheCallback(::core::mem::transmute(pfncallback)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
#[inline]
pub unsafe fn ResetWriteWatch(lpbaseaddress: *const ::core::ffi::c_void, dwregionsize: usize) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ResetWriteWatch(lpbaseaddress: *const ::core::ffi::c_void, dwregionsize: usize) -> u32;
        }
        ::core::mem::transmute(ResetWriteWatch(::core::mem::transmute(lpbaseaddress), ::core::mem::transmute(dwregionsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
#[inline]
pub unsafe fn RtlCompareMemory(source1: *const ::core::ffi::c_void, source2: *const ::core::ffi::c_void, length: usize) -> usize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtlCompareMemory(source1: *const ::core::ffi::c_void, source2: *const ::core::ffi::c_void, length: usize) -> usize;
        }
        ::core::mem::transmute(RtlCompareMemory(::core::mem::transmute(source1), ::core::mem::transmute(source2), ::core::mem::transmute(length)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
#[inline]
pub unsafe fn RtlCrc32(buffer: *const ::core::ffi::c_void, size: usize, initialcrc: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtlCrc32(buffer: *const ::core::ffi::c_void, size: usize, initialcrc: u32) -> u32;
        }
        ::core::mem::transmute(RtlCrc32(::core::mem::transmute(buffer), ::core::mem::transmute(size), ::core::mem::transmute(initialcrc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
#[inline]
pub unsafe fn RtlCrc64(buffer: *const ::core::ffi::c_void, size: usize, initialcrc: u64) -> u64 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtlCrc64(buffer: *const ::core::ffi::c_void, size: usize, initialcrc: u64) -> u64;
        }
        ::core::mem::transmute(RtlCrc64(::core::mem::transmute(buffer), ::core::mem::transmute(size), ::core::mem::transmute(initialcrc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RtlIsZeroMemory(buffer: *const ::core::ffi::c_void, length: usize) -> super::super::Foundation::BOOLEAN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtlIsZeroMemory(buffer: *const ::core::ffi::c_void, length: usize) -> super::super::Foundation::BOOLEAN;
        }
        ::core::mem::transmute(RtlIsZeroMemory(::core::mem::transmute(buffer), ::core::mem::transmute(length)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetProcessValidCallTargets<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hprocess: Param0, virtualaddress: *const ::core::ffi::c_void, regionsize: usize, offsetinformation: &mut [CFG_CALL_TARGET_INFO]) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetProcessValidCallTargets(hprocess: super::super::Foundation::HANDLE, virtualaddress: *const ::core::ffi::c_void, regionsize: usize, numberofoffsets: u32, offsetinformation: *mut CFG_CALL_TARGET_INFO) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetProcessValidCallTargets(hprocess.into_param().abi(), ::core::mem::transmute(virtualaddress), ::core::mem::transmute(regionsize), offsetinformation.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(offsetinformation))))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetProcessValidCallTargetsForMappedView<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(process: Param0, virtualaddress: *const ::core::ffi::c_void, regionsize: usize, offsetinformation: &mut [CFG_CALL_TARGET_INFO], section: Param5, expectedfileoffset: u64) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetProcessValidCallTargetsForMappedView(process: super::super::Foundation::HANDLE, virtualaddress: *const ::core::ffi::c_void, regionsize: usize, numberofoffsets: u32, offsetinformation: *mut CFG_CALL_TARGET_INFO, section: super::super::Foundation::HANDLE, expectedfileoffset: u64) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetProcessValidCallTargetsForMappedView(process.into_param().abi(), ::core::mem::transmute(virtualaddress), ::core::mem::transmute(regionsize), offsetinformation.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(offsetinformation)), section.into_param().abi(), ::core::mem::transmute(expectedfileoffset)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetProcessWorkingSetSizeEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hprocess: Param0, dwminimumworkingsetsize: usize, dwmaximumworkingsetsize: usize, flags: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetProcessWorkingSetSizeEx(hprocess: super::super::Foundation::HANDLE, dwminimumworkingsetsize: usize, dwmaximumworkingsetsize: usize, flags: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetProcessWorkingSetSizeEx(hprocess.into_param().abi(), ::core::mem::transmute(dwminimumworkingsetsize), ::core::mem::transmute(dwmaximumworkingsetsize), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetSystemFileCacheSize(minimumfilecachesize: usize, maximumfilecachesize: usize, flags: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetSystemFileCacheSize(minimumfilecachesize: usize, maximumfilecachesize: usize, flags: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetSystemFileCacheSize(::core::mem::transmute(minimumfilecachesize), ::core::mem::transmute(maximumfilecachesize), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UNMAP_VIEW_OF_FILE_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const MEM_UNMAP_NONE: UNMAP_VIEW_OF_FILE_FLAGS = UNMAP_VIEW_OF_FILE_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const MEM_UNMAP_WITH_TRANSIENT_BOOST: UNMAP_VIEW_OF_FILE_FLAGS = UNMAP_VIEW_OF_FILE_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const MEM_PRESERVE_PLACEHOLDER: UNMAP_VIEW_OF_FILE_FLAGS = UNMAP_VIEW_OF_FILE_FLAGS(2u32);
impl ::core::marker::Copy for UNMAP_VIEW_OF_FILE_FLAGS {}
impl ::core::clone::Clone for UNMAP_VIEW_OF_FILE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UNMAP_VIEW_OF_FILE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UNMAP_VIEW_OF_FILE_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for UNMAP_VIEW_OF_FILE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UNMAP_VIEW_OF_FILE_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UnmapViewOfFile(lpbaseaddress: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UnmapViewOfFile(lpbaseaddress: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(UnmapViewOfFile(::core::mem::transmute(lpbaseaddress)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UnmapViewOfFile2<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(process: Param0, baseaddress: *const ::core::ffi::c_void, unmapflags: UNMAP_VIEW_OF_FILE_FLAGS) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UnmapViewOfFile2(process: super::super::Foundation::HANDLE, baseaddress: *const ::core::ffi::c_void, unmapflags: UNMAP_VIEW_OF_FILE_FLAGS) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(UnmapViewOfFile2(process.into_param().abi(), ::core::mem::transmute(baseaddress), ::core::mem::transmute(unmapflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UnmapViewOfFileEx(baseaddress: *const ::core::ffi::c_void, unmapflags: UNMAP_VIEW_OF_FILE_FLAGS) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UnmapViewOfFileEx(baseaddress: *const ::core::ffi::c_void, unmapflags: UNMAP_VIEW_OF_FILE_FLAGS) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(UnmapViewOfFileEx(::core::mem::transmute(baseaddress), ::core::mem::transmute(unmapflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UnregisterBadMemoryNotification(registrationhandle: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UnregisterBadMemoryNotification(registrationhandle: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(UnregisterBadMemoryNotification(::core::mem::transmute(registrationhandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct VIRTUAL_ALLOCATION_TYPE(pub u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const MEM_COMMIT: VIRTUAL_ALLOCATION_TYPE = VIRTUAL_ALLOCATION_TYPE(4096u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const MEM_RESERVE: VIRTUAL_ALLOCATION_TYPE = VIRTUAL_ALLOCATION_TYPE(8192u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const MEM_RESET: VIRTUAL_ALLOCATION_TYPE = VIRTUAL_ALLOCATION_TYPE(524288u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const MEM_RESET_UNDO: VIRTUAL_ALLOCATION_TYPE = VIRTUAL_ALLOCATION_TYPE(16777216u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const MEM_REPLACE_PLACEHOLDER: VIRTUAL_ALLOCATION_TYPE = VIRTUAL_ALLOCATION_TYPE(16384u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const MEM_LARGE_PAGES: VIRTUAL_ALLOCATION_TYPE = VIRTUAL_ALLOCATION_TYPE(536870912u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const MEM_RESERVE_PLACEHOLDER: VIRTUAL_ALLOCATION_TYPE = VIRTUAL_ALLOCATION_TYPE(262144u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const MEM_FREE: VIRTUAL_ALLOCATION_TYPE = VIRTUAL_ALLOCATION_TYPE(65536u32);
impl ::core::marker::Copy for VIRTUAL_ALLOCATION_TYPE {}
impl ::core::clone::Clone for VIRTUAL_ALLOCATION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VIRTUAL_ALLOCATION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for VIRTUAL_ALLOCATION_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for VIRTUAL_ALLOCATION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VIRTUAL_ALLOCATION_TYPE").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for VIRTUAL_ALLOCATION_TYPE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for VIRTUAL_ALLOCATION_TYPE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for VIRTUAL_ALLOCATION_TYPE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for VIRTUAL_ALLOCATION_TYPE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for VIRTUAL_ALLOCATION_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct VIRTUAL_FREE_TYPE(pub u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const MEM_DECOMMIT: VIRTUAL_FREE_TYPE = VIRTUAL_FREE_TYPE(16384u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const MEM_RELEASE: VIRTUAL_FREE_TYPE = VIRTUAL_FREE_TYPE(32768u32);
impl ::core::marker::Copy for VIRTUAL_FREE_TYPE {}
impl ::core::clone::Clone for VIRTUAL_FREE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VIRTUAL_FREE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for VIRTUAL_FREE_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for VIRTUAL_FREE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VIRTUAL_FREE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
#[inline]
pub unsafe fn VirtualAlloc(lpaddress: *const ::core::ffi::c_void, dwsize: usize, flallocationtype: VIRTUAL_ALLOCATION_TYPE, flprotect: PAGE_PROTECTION_FLAGS) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VirtualAlloc(lpaddress: *const ::core::ffi::c_void, dwsize: usize, flallocationtype: VIRTUAL_ALLOCATION_TYPE, flprotect: PAGE_PROTECTION_FLAGS) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(VirtualAlloc(::core::mem::transmute(lpaddress), ::core::mem::transmute(dwsize), ::core::mem::transmute(flallocationtype), ::core::mem::transmute(flprotect)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VirtualAlloc2<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(process: Param0, baseaddress: *const ::core::ffi::c_void, size: usize, allocationtype: VIRTUAL_ALLOCATION_TYPE, pageprotection: u32, extendedparameters: &mut [MEM_EXTENDED_PARAMETER]) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VirtualAlloc2(process: super::super::Foundation::HANDLE, baseaddress: *const ::core::ffi::c_void, size: usize, allocationtype: VIRTUAL_ALLOCATION_TYPE, pageprotection: u32, extendedparameters: *mut MEM_EXTENDED_PARAMETER, parametercount: u32) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(VirtualAlloc2(process.into_param().abi(), ::core::mem::transmute(baseaddress), ::core::mem::transmute(size), ::core::mem::transmute(allocationtype), ::core::mem::transmute(pageprotection), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(extendedparameters)), extendedparameters.len() as _))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VirtualAlloc2FromApp<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(process: Param0, baseaddress: *const ::core::ffi::c_void, size: usize, allocationtype: VIRTUAL_ALLOCATION_TYPE, pageprotection: u32, extendedparameters: &mut [MEM_EXTENDED_PARAMETER]) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VirtualAlloc2FromApp(process: super::super::Foundation::HANDLE, baseaddress: *const ::core::ffi::c_void, size: usize, allocationtype: VIRTUAL_ALLOCATION_TYPE, pageprotection: u32, extendedparameters: *mut MEM_EXTENDED_PARAMETER, parametercount: u32) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(VirtualAlloc2FromApp(process.into_param().abi(), ::core::mem::transmute(baseaddress), ::core::mem::transmute(size), ::core::mem::transmute(allocationtype), ::core::mem::transmute(pageprotection), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(extendedparameters)), extendedparameters.len() as _))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VirtualAllocEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hprocess: Param0, lpaddress: *const ::core::ffi::c_void, dwsize: usize, flallocationtype: VIRTUAL_ALLOCATION_TYPE, flprotect: PAGE_PROTECTION_FLAGS) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VirtualAllocEx(hprocess: super::super::Foundation::HANDLE, lpaddress: *const ::core::ffi::c_void, dwsize: usize, flallocationtype: VIRTUAL_ALLOCATION_TYPE, flprotect: PAGE_PROTECTION_FLAGS) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(VirtualAllocEx(hprocess.into_param().abi(), ::core::mem::transmute(lpaddress), ::core::mem::transmute(dwsize), ::core::mem::transmute(flallocationtype), ::core::mem::transmute(flprotect)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VirtualAllocExNuma<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hprocess: Param0, lpaddress: *const ::core::ffi::c_void, dwsize: usize, flallocationtype: VIRTUAL_ALLOCATION_TYPE, flprotect: u32, nndpreferred: u32) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VirtualAllocExNuma(hprocess: super::super::Foundation::HANDLE, lpaddress: *const ::core::ffi::c_void, dwsize: usize, flallocationtype: VIRTUAL_ALLOCATION_TYPE, flprotect: u32, nndpreferred: u32) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(VirtualAllocExNuma(hprocess.into_param().abi(), ::core::mem::transmute(lpaddress), ::core::mem::transmute(dwsize), ::core::mem::transmute(flallocationtype), ::core::mem::transmute(flprotect), ::core::mem::transmute(nndpreferred)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
#[inline]
pub unsafe fn VirtualAllocFromApp(baseaddress: *const ::core::ffi::c_void, size: usize, allocationtype: VIRTUAL_ALLOCATION_TYPE, protection: u32) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VirtualAllocFromApp(baseaddress: *const ::core::ffi::c_void, size: usize, allocationtype: VIRTUAL_ALLOCATION_TYPE, protection: u32) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(VirtualAllocFromApp(::core::mem::transmute(baseaddress), ::core::mem::transmute(size), ::core::mem::transmute(allocationtype), ::core::mem::transmute(protection)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VirtualFree(lpaddress: *mut ::core::ffi::c_void, dwsize: usize, dwfreetype: VIRTUAL_FREE_TYPE) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VirtualFree(lpaddress: *mut ::core::ffi::c_void, dwsize: usize, dwfreetype: VIRTUAL_FREE_TYPE) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(VirtualFree(::core::mem::transmute(lpaddress), ::core::mem::transmute(dwsize), ::core::mem::transmute(dwfreetype)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VirtualFreeEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hprocess: Param0, lpaddress: *mut ::core::ffi::c_void, dwsize: usize, dwfreetype: VIRTUAL_FREE_TYPE) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VirtualFreeEx(hprocess: super::super::Foundation::HANDLE, lpaddress: *mut ::core::ffi::c_void, dwsize: usize, dwfreetype: VIRTUAL_FREE_TYPE) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(VirtualFreeEx(hprocess.into_param().abi(), ::core::mem::transmute(lpaddress), ::core::mem::transmute(dwsize), ::core::mem::transmute(dwfreetype)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VirtualLock(lpaddress: *const ::core::ffi::c_void, dwsize: usize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VirtualLock(lpaddress: *const ::core::ffi::c_void, dwsize: usize) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(VirtualLock(::core::mem::transmute(lpaddress), ::core::mem::transmute(dwsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VirtualProtect(lpaddress: *const ::core::ffi::c_void, dwsize: usize, flnewprotect: PAGE_PROTECTION_FLAGS, lpfloldprotect: *mut PAGE_PROTECTION_FLAGS) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VirtualProtect(lpaddress: *const ::core::ffi::c_void, dwsize: usize, flnewprotect: PAGE_PROTECTION_FLAGS, lpfloldprotect: *mut PAGE_PROTECTION_FLAGS) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(VirtualProtect(::core::mem::transmute(lpaddress), ::core::mem::transmute(dwsize), ::core::mem::transmute(flnewprotect), ::core::mem::transmute(lpfloldprotect)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VirtualProtectEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hprocess: Param0, lpaddress: *const ::core::ffi::c_void, dwsize: usize, flnewprotect: PAGE_PROTECTION_FLAGS, lpfloldprotect: *mut PAGE_PROTECTION_FLAGS) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VirtualProtectEx(hprocess: super::super::Foundation::HANDLE, lpaddress: *const ::core::ffi::c_void, dwsize: usize, flnewprotect: PAGE_PROTECTION_FLAGS, lpfloldprotect: *mut PAGE_PROTECTION_FLAGS) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(VirtualProtectEx(hprocess.into_param().abi(), ::core::mem::transmute(lpaddress), ::core::mem::transmute(dwsize), ::core::mem::transmute(flnewprotect), ::core::mem::transmute(lpfloldprotect)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VirtualProtectFromApp(address: *const ::core::ffi::c_void, size: usize, newprotection: u32, oldprotection: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VirtualProtectFromApp(address: *const ::core::ffi::c_void, size: usize, newprotection: u32, oldprotection: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(VirtualProtectFromApp(::core::mem::transmute(address), ::core::mem::transmute(size), ::core::mem::transmute(newprotection), ::core::mem::transmute(oldprotection)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
#[inline]
pub unsafe fn VirtualQuery(lpaddress: *const ::core::ffi::c_void, lpbuffer: *mut MEMORY_BASIC_INFORMATION, dwlength: usize) -> usize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VirtualQuery(lpaddress: *const ::core::ffi::c_void, lpbuffer: *mut MEMORY_BASIC_INFORMATION, dwlength: usize) -> usize;
        }
        ::core::mem::transmute(VirtualQuery(::core::mem::transmute(lpaddress), ::core::mem::transmute(lpbuffer), ::core::mem::transmute(dwlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VirtualQueryEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hprocess: Param0, lpaddress: *const ::core::ffi::c_void, lpbuffer: *mut MEMORY_BASIC_INFORMATION, dwlength: usize) -> usize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VirtualQueryEx(hprocess: super::super::Foundation::HANDLE, lpaddress: *const ::core::ffi::c_void, lpbuffer: *mut MEMORY_BASIC_INFORMATION, dwlength: usize) -> usize;
        }
        ::core::mem::transmute(VirtualQueryEx(hprocess.into_param().abi(), ::core::mem::transmute(lpaddress), ::core::mem::transmute(lpbuffer), ::core::mem::transmute(dwlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VirtualUnlock(lpaddress: *const ::core::ffi::c_void, dwsize: usize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VirtualUnlock(lpaddress: *const ::core::ffi::c_void, dwsize: usize) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(VirtualUnlock(::core::mem::transmute(lpaddress), ::core::mem::transmute(dwsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VirtualUnlockEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(process: Param0, address: *const ::core::ffi::c_void, size: usize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VirtualUnlockEx(process: super::super::Foundation::HANDLE, address: *const ::core::ffi::c_void, size: usize) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(VirtualUnlockEx(process.into_param().abi(), ::core::mem::transmute(address), ::core::mem::transmute(size)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WIN32_MEMORY_INFORMATION_CLASS(pub i32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const MemoryRegionInfo: WIN32_MEMORY_INFORMATION_CLASS = WIN32_MEMORY_INFORMATION_CLASS(0i32);
impl ::core::marker::Copy for WIN32_MEMORY_INFORMATION_CLASS {}
impl ::core::clone::Clone for WIN32_MEMORY_INFORMATION_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WIN32_MEMORY_INFORMATION_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WIN32_MEMORY_INFORMATION_CLASS {
    type Abi = Self;
}
impl ::core::fmt::Debug for WIN32_MEMORY_INFORMATION_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WIN32_MEMORY_INFORMATION_CLASS").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub struct WIN32_MEMORY_PARTITION_INFORMATION {
    pub Flags: u32,
    pub NumaNode: u32,
    pub Channel: u32,
    pub NumberOfNumaNodes: u32,
    pub ResidentAvailablePages: u64,
    pub CommittedPages: u64,
    pub CommitLimit: u64,
    pub PeakCommitment: u64,
    pub TotalNumberOfPages: u64,
    pub AvailablePages: u64,
    pub ZeroPages: u64,
    pub FreePages: u64,
    pub StandbyPages: u64,
    pub Reserved: [u64; 16],
    pub MaximumCommitLimit: u64,
    pub Reserved2: u64,
    pub PartitionId: u32,
}
impl ::core::marker::Copy for WIN32_MEMORY_PARTITION_INFORMATION {}
impl ::core::clone::Clone for WIN32_MEMORY_PARTITION_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WIN32_MEMORY_PARTITION_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIN32_MEMORY_PARTITION_INFORMATION")
            .field("Flags", &self.Flags)
            .field("NumaNode", &self.NumaNode)
            .field("Channel", &self.Channel)
            .field("NumberOfNumaNodes", &self.NumberOfNumaNodes)
            .field("ResidentAvailablePages", &self.ResidentAvailablePages)
            .field("CommittedPages", &self.CommittedPages)
            .field("CommitLimit", &self.CommitLimit)
            .field("PeakCommitment", &self.PeakCommitment)
            .field("TotalNumberOfPages", &self.TotalNumberOfPages)
            .field("AvailablePages", &self.AvailablePages)
            .field("ZeroPages", &self.ZeroPages)
            .field("FreePages", &self.FreePages)
            .field("StandbyPages", &self.StandbyPages)
            .field("Reserved", &self.Reserved)
            .field("MaximumCommitLimit", &self.MaximumCommitLimit)
            .field("Reserved2", &self.Reserved2)
            .field("PartitionId", &self.PartitionId)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for WIN32_MEMORY_PARTITION_INFORMATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WIN32_MEMORY_PARTITION_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WIN32_MEMORY_PARTITION_INFORMATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for WIN32_MEMORY_PARTITION_INFORMATION {}
impl ::core::default::Default for WIN32_MEMORY_PARTITION_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WIN32_MEMORY_PARTITION_INFORMATION_CLASS(pub i32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const MemoryPartitionInfo: WIN32_MEMORY_PARTITION_INFORMATION_CLASS = WIN32_MEMORY_PARTITION_INFORMATION_CLASS(0i32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const MemoryPartitionDedicatedMemoryInfo: WIN32_MEMORY_PARTITION_INFORMATION_CLASS = WIN32_MEMORY_PARTITION_INFORMATION_CLASS(1i32);
impl ::core::marker::Copy for WIN32_MEMORY_PARTITION_INFORMATION_CLASS {}
impl ::core::clone::Clone for WIN32_MEMORY_PARTITION_INFORMATION_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WIN32_MEMORY_PARTITION_INFORMATION_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WIN32_MEMORY_PARTITION_INFORMATION_CLASS {
    type Abi = Self;
}
impl ::core::fmt::Debug for WIN32_MEMORY_PARTITION_INFORMATION_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WIN32_MEMORY_PARTITION_INFORMATION_CLASS").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub struct WIN32_MEMORY_RANGE_ENTRY {
    pub VirtualAddress: *mut ::core::ffi::c_void,
    pub NumberOfBytes: usize,
}
impl ::core::marker::Copy for WIN32_MEMORY_RANGE_ENTRY {}
impl ::core::clone::Clone for WIN32_MEMORY_RANGE_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WIN32_MEMORY_RANGE_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIN32_MEMORY_RANGE_ENTRY").field("VirtualAddress", &self.VirtualAddress).field("NumberOfBytes", &self.NumberOfBytes).finish()
    }
}
unsafe impl ::windows::core::Abi for WIN32_MEMORY_RANGE_ENTRY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WIN32_MEMORY_RANGE_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WIN32_MEMORY_RANGE_ENTRY>()) == 0 }
    }
}
impl ::core::cmp::Eq for WIN32_MEMORY_RANGE_ENTRY {}
impl ::core::default::Default for WIN32_MEMORY_RANGE_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub struct WIN32_MEMORY_REGION_INFORMATION {
    pub AllocationBase: *mut ::core::ffi::c_void,
    pub AllocationProtect: u32,
    pub Anonymous: WIN32_MEMORY_REGION_INFORMATION_0,
    pub RegionSize: usize,
    pub CommitSize: usize,
}
impl ::core::marker::Copy for WIN32_MEMORY_REGION_INFORMATION {}
impl ::core::clone::Clone for WIN32_MEMORY_REGION_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WIN32_MEMORY_REGION_INFORMATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WIN32_MEMORY_REGION_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WIN32_MEMORY_REGION_INFORMATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for WIN32_MEMORY_REGION_INFORMATION {}
impl ::core::default::Default for WIN32_MEMORY_REGION_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub union WIN32_MEMORY_REGION_INFORMATION_0 {
    pub Flags: u32,
    pub Anonymous: WIN32_MEMORY_REGION_INFORMATION_0_0,
}
impl ::core::marker::Copy for WIN32_MEMORY_REGION_INFORMATION_0 {}
impl ::core::clone::Clone for WIN32_MEMORY_REGION_INFORMATION_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WIN32_MEMORY_REGION_INFORMATION_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WIN32_MEMORY_REGION_INFORMATION_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WIN32_MEMORY_REGION_INFORMATION_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for WIN32_MEMORY_REGION_INFORMATION_0 {}
impl ::core::default::Default for WIN32_MEMORY_REGION_INFORMATION_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub struct WIN32_MEMORY_REGION_INFORMATION_0_0 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for WIN32_MEMORY_REGION_INFORMATION_0_0 {}
impl ::core::clone::Clone for WIN32_MEMORY_REGION_INFORMATION_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WIN32_MEMORY_REGION_INFORMATION_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIN32_MEMORY_REGION_INFORMATION_0_0").field("_bitfield", &self._bitfield).finish()
    }
}
unsafe impl ::windows::core::Abi for WIN32_MEMORY_REGION_INFORMATION_0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WIN32_MEMORY_REGION_INFORMATION_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WIN32_MEMORY_REGION_INFORMATION_0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for WIN32_MEMORY_REGION_INFORMATION_0_0 {}
impl ::core::default::Default for WIN32_MEMORY_REGION_INFORMATION_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
