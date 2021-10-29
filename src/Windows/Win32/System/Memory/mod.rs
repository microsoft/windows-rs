#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AddSecureMemoryCacheCallback(
    pfncallback: ::std::option::Option<PSECURE_MEMORY_CACHE_CALLBACK>,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AddSecureMemoryCacheCallback(
                pfncallback: ::windows::runtime::RawPtr,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(AddSecureMemoryCacheCallback(::std::mem::transmute(
            pfncallback,
        )))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AllocateUserPhysicalPages<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hprocess: Param0,
    numberofpages: *mut usize,
    pagearray: *mut usize,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AllocateUserPhysicalPages(
                hprocess: super::super::Foundation::HANDLE,
                numberofpages: *mut usize,
                pagearray: *mut usize,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(AllocateUserPhysicalPages(
            hprocess.into_param().abi(),
            ::std::mem::transmute(numberofpages),
            ::std::mem::transmute(pagearray),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
#[inline]
pub unsafe fn AllocateUserPhysicalPages2<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    objecthandle: Param0,
    numberofpages: *mut usize,
    pagearray: *mut usize,
    extendedparameters: *mut super::SystemServices::MEM_EXTENDED_PARAMETER,
    extendedparametercount: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AllocateUserPhysicalPages2(
                objecthandle: super::super::Foundation::HANDLE,
                numberofpages: *mut usize,
                pagearray: *mut usize,
                extendedparameters: *mut super::SystemServices::MEM_EXTENDED_PARAMETER,
                extendedparametercount: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(AllocateUserPhysicalPages2(
            objecthandle.into_param().abi(),
            ::std::mem::transmute(numberofpages),
            ::std::mem::transmute(pagearray),
            ::std::mem::transmute(extendedparameters),
            ::std::mem::transmute(extendedparametercount),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AllocateUserPhysicalPagesNuma<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hprocess: Param0,
    numberofpages: *mut usize,
    pagearray: *mut usize,
    nndpreferred: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AllocateUserPhysicalPagesNuma(
                hprocess: super::super::Foundation::HANDLE,
                numberofpages: *mut usize,
                pagearray: *mut usize,
                nndpreferred: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(AllocateUserPhysicalPagesNuma(
            hprocess.into_param().abi(),
            ::std::mem::transmute(numberofpages),
            ::std::mem::transmute(pagearray),
            ::std::mem::transmute(nndpreferred),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security",
    feature = "Win32_System_SystemServices"
))]
#[inline]
pub unsafe fn CreateFileMapping2<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param6: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    file: Param0,
    securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
    desiredaccess: u32,
    pageprotection: PAGE_PROTECTION_FLAGS,
    allocationattributes: u32,
    maximumsize: u64,
    name: Param6,
    extendedparameters: *mut super::SystemServices::MEM_EXTENDED_PARAMETER,
    parametercount: u32,
) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateFileMapping2(
                file: super::super::Foundation::HANDLE,
                securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
                desiredaccess: u32,
                pageprotection: PAGE_PROTECTION_FLAGS,
                allocationattributes: u32,
                maximumsize: u64,
                name: super::super::Foundation::PWSTR,
                extendedparameters: *mut super::SystemServices::MEM_EXTENDED_PARAMETER,
                parametercount: u32,
            ) -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(CreateFileMapping2(
            file.into_param().abi(),
            ::std::mem::transmute(securityattributes),
            ::std::mem::transmute(desiredaccess),
            ::std::mem::transmute(pageprotection),
            ::std::mem::transmute(allocationattributes),
            ::std::mem::transmute(maximumsize),
            name.into_param().abi(),
            ::std::mem::transmute(extendedparameters),
            ::std::mem::transmute(parametercount),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateFileMappingA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hfile: Param0,
    lpfilemappingattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
    flprotect: PAGE_PROTECTION_FLAGS,
    dwmaximumsizehigh: u32,
    dwmaximumsizelow: u32,
    lpname: Param5,
) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateFileMappingA(
                hfile: super::super::Foundation::HANDLE,
                lpfilemappingattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
                flprotect: PAGE_PROTECTION_FLAGS,
                dwmaximumsizehigh: u32,
                dwmaximumsizelow: u32,
                lpname: super::super::Foundation::PSTR,
            ) -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(CreateFileMappingA(
            hfile.into_param().abi(),
            ::std::mem::transmute(lpfilemappingattributes),
            ::std::mem::transmute(flprotect),
            ::std::mem::transmute(dwmaximumsizehigh),
            ::std::mem::transmute(dwmaximumsizelow),
            lpname.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateFileMappingFromApp<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hfile: Param0,
    securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
    pageprotection: PAGE_PROTECTION_FLAGS,
    maximumsize: u64,
    name: Param4,
) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateFileMappingFromApp(
                hfile: super::super::Foundation::HANDLE,
                securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
                pageprotection: PAGE_PROTECTION_FLAGS,
                maximumsize: u64,
                name: super::super::Foundation::PWSTR,
            ) -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(CreateFileMappingFromApp(
            hfile.into_param().abi(),
            ::std::mem::transmute(securityattributes),
            ::std::mem::transmute(pageprotection),
            ::std::mem::transmute(maximumsize),
            name.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateFileMappingNumaA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hfile: Param0,
    lpfilemappingattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
    flprotect: PAGE_PROTECTION_FLAGS,
    dwmaximumsizehigh: u32,
    dwmaximumsizelow: u32,
    lpname: Param5,
    nndpreferred: u32,
) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateFileMappingNumaA(
                hfile: super::super::Foundation::HANDLE,
                lpfilemappingattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
                flprotect: PAGE_PROTECTION_FLAGS,
                dwmaximumsizehigh: u32,
                dwmaximumsizelow: u32,
                lpname: super::super::Foundation::PSTR,
                nndpreferred: u32,
            ) -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(CreateFileMappingNumaA(
            hfile.into_param().abi(),
            ::std::mem::transmute(lpfilemappingattributes),
            ::std::mem::transmute(flprotect),
            ::std::mem::transmute(dwmaximumsizehigh),
            ::std::mem::transmute(dwmaximumsizelow),
            lpname.into_param().abi(),
            ::std::mem::transmute(nndpreferred),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateFileMappingNumaW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hfile: Param0,
    lpfilemappingattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
    flprotect: PAGE_PROTECTION_FLAGS,
    dwmaximumsizehigh: u32,
    dwmaximumsizelow: u32,
    lpname: Param5,
    nndpreferred: u32,
) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateFileMappingNumaW(
                hfile: super::super::Foundation::HANDLE,
                lpfilemappingattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
                flprotect: PAGE_PROTECTION_FLAGS,
                dwmaximumsizehigh: u32,
                dwmaximumsizelow: u32,
                lpname: super::super::Foundation::PWSTR,
                nndpreferred: u32,
            ) -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(CreateFileMappingNumaW(
            hfile.into_param().abi(),
            ::std::mem::transmute(lpfilemappingattributes),
            ::std::mem::transmute(flprotect),
            ::std::mem::transmute(dwmaximumsizehigh),
            ::std::mem::transmute(dwmaximumsizelow),
            lpname.into_param().abi(),
            ::std::mem::transmute(nndpreferred),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateFileMappingW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hfile: Param0,
    lpfilemappingattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
    flprotect: PAGE_PROTECTION_FLAGS,
    dwmaximumsizehigh: u32,
    dwmaximumsizelow: u32,
    lpname: Param5,
) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateFileMappingW(
                hfile: super::super::Foundation::HANDLE,
                lpfilemappingattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
                flprotect: PAGE_PROTECTION_FLAGS,
                dwmaximumsizehigh: u32,
                dwmaximumsizelow: u32,
                lpname: super::super::Foundation::PWSTR,
            ) -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(CreateFileMappingW(
            hfile.into_param().abi(),
            ::std::mem::transmute(lpfilemappingattributes),
            ::std::mem::transmute(flprotect),
            ::std::mem::transmute(dwmaximumsizehigh),
            ::std::mem::transmute(dwmaximumsizelow),
            lpname.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateMemoryResourceNotification(
    notificationtype: MEMORY_RESOURCE_NOTIFICATION_TYPE,
) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateMemoryResourceNotification(
                notificationtype: MEMORY_RESOURCE_NOTIFICATION_TYPE,
            ) -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(CreateMemoryResourceNotification(::std::mem::transmute(
            notificationtype,
        )))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DiscardVirtualMemory(virtualaddress: *mut ::std::ffi::c_void, size: usize) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DiscardVirtualMemory(virtualaddress: *mut ::std::ffi::c_void, size: usize) -> u32;
        }
        ::std::mem::transmute(DiscardVirtualMemory(
            ::std::mem::transmute(virtualaddress),
            ::std::mem::transmute(size),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const FILE_CACHE_MAX_HARD_DISABLE: u32 = 2u32;
pub const FILE_CACHE_MAX_HARD_ENABLE: u32 = 1u32;
pub const FILE_CACHE_MIN_HARD_DISABLE: u32 = 8u32;
pub const FILE_CACHE_MIN_HARD_ENABLE: u32 = 4u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct FILE_MAP(pub u32);
pub const FILE_MAP_WRITE: FILE_MAP = FILE_MAP(2u32);
pub const FILE_MAP_READ: FILE_MAP = FILE_MAP(4u32);
pub const FILE_MAP_ALL_ACCESS: FILE_MAP = FILE_MAP(983071u32);
pub const FILE_MAP_EXECUTE: FILE_MAP = FILE_MAP(32u32);
pub const FILE_MAP_COPY: FILE_MAP = FILE_MAP(1u32);
pub const FILE_MAP_RESERVE: FILE_MAP = FILE_MAP(2147483648u32);
pub const FILE_MAP_TARGETS_INVALID: FILE_MAP = FILE_MAP(1073741824u32);
pub const FILE_MAP_LARGE_PAGES: FILE_MAP = FILE_MAP(536870912u32);
impl ::std::convert::From<u32> for FILE_MAP {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FILE_MAP {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for FILE_MAP {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for FILE_MAP {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for FILE_MAP {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for FILE_MAP {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for FILE_MAP {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FlushViewOfFile(
    lpbaseaddress: *const ::std::ffi::c_void,
    dwnumberofbytestoflush: usize,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FlushViewOfFile(
                lpbaseaddress: *const ::std::ffi::c_void,
                dwnumberofbytestoflush: usize,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FlushViewOfFile(
            ::std::mem::transmute(lpbaseaddress),
            ::std::mem::transmute(dwnumberofbytestoflush),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FreeUserPhysicalPages<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hprocess: Param0,
    numberofpages: *mut usize,
    pagearray: *const usize,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FreeUserPhysicalPages(
                hprocess: super::super::Foundation::HANDLE,
                numberofpages: *mut usize,
                pagearray: *const usize,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FreeUserPhysicalPages(
            hprocess.into_param().abi(),
            ::std::mem::transmute(numberofpages),
            ::std::mem::transmute(pagearray),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct GLOBAL_ALLOC_FLAGS(pub u32);
pub const GHND: GLOBAL_ALLOC_FLAGS = GLOBAL_ALLOC_FLAGS(66u32);
pub const GMEM_FIXED: GLOBAL_ALLOC_FLAGS = GLOBAL_ALLOC_FLAGS(0u32);
pub const GMEM_MOVEABLE: GLOBAL_ALLOC_FLAGS = GLOBAL_ALLOC_FLAGS(2u32);
pub const GMEM_ZEROINIT: GLOBAL_ALLOC_FLAGS = GLOBAL_ALLOC_FLAGS(64u32);
pub const GPTR: GLOBAL_ALLOC_FLAGS = GLOBAL_ALLOC_FLAGS(64u32);
impl ::std::convert::From<u32> for GLOBAL_ALLOC_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GLOBAL_ALLOC_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for GLOBAL_ALLOC_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for GLOBAL_ALLOC_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for GLOBAL_ALLOC_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for GLOBAL_ALLOC_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for GLOBAL_ALLOC_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[inline]
pub unsafe fn GetLargePageMinimum() -> usize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetLargePageMinimum() -> usize;
        }
        ::std::mem::transmute(GetLargePageMinimum())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetMemoryErrorHandlingCapabilities(
    capabilities: *mut u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetMemoryErrorHandlingCapabilities(
                capabilities: *mut u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetMemoryErrorHandlingCapabilities(::std::mem::transmute(
            capabilities,
        )))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetProcessHeap() -> HeapHandle {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetProcessHeap() -> HeapHandle;
        }
        ::std::mem::transmute(GetProcessHeap())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetProcessHeaps(numberofheaps: u32, processheaps: *mut HeapHandle) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetProcessHeaps(numberofheaps: u32, processheaps: *mut HeapHandle) -> u32;
        }
        ::std::mem::transmute(GetProcessHeaps(
            ::std::mem::transmute(numberofheaps),
            ::std::mem::transmute(processheaps),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetProcessWorkingSetSizeEx<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hprocess: Param0,
    lpminimumworkingsetsize: *mut usize,
    lpmaximumworkingsetsize: *mut usize,
    flags: *mut u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetProcessWorkingSetSizeEx(
                hprocess: super::super::Foundation::HANDLE,
                lpminimumworkingsetsize: *mut usize,
                lpmaximumworkingsetsize: *mut usize,
                flags: *mut u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetProcessWorkingSetSizeEx(
            hprocess.into_param().abi(),
            ::std::mem::transmute(lpminimumworkingsetsize),
            ::std::mem::transmute(lpmaximumworkingsetsize),
            ::std::mem::transmute(flags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetSystemFileCacheSize(
    lpminimumfilecachesize: *mut usize,
    lpmaximumfilecachesize: *mut usize,
    lpflags: *mut u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetSystemFileCacheSize(
                lpminimumfilecachesize: *mut usize,
                lpmaximumfilecachesize: *mut usize,
                lpflags: *mut u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetSystemFileCacheSize(
            ::std::mem::transmute(lpminimumfilecachesize),
            ::std::mem::transmute(lpmaximumfilecachesize),
            ::std::mem::transmute(lpflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetWriteWatch(
    dwflags: u32,
    lpbaseaddress: *const ::std::ffi::c_void,
    dwregionsize: usize,
    lpaddresses: *mut *mut ::std::ffi::c_void,
    lpdwcount: *mut usize,
    lpdwgranularity: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetWriteWatch(
                dwflags: u32,
                lpbaseaddress: *const ::std::ffi::c_void,
                dwregionsize: usize,
                lpaddresses: *mut *mut ::std::ffi::c_void,
                lpdwcount: *mut usize,
                lpdwgranularity: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(GetWriteWatch(
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(lpbaseaddress),
            ::std::mem::transmute(dwregionsize),
            ::std::mem::transmute(lpaddresses),
            ::std::mem::transmute(lpdwcount),
            ::std::mem::transmute(lpdwgranularity),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GlobalAlloc(uflags: GLOBAL_ALLOC_FLAGS, dwbytes: usize) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GlobalAlloc(uflags: GLOBAL_ALLOC_FLAGS, dwbytes: usize) -> isize;
        }
        ::std::mem::transmute(GlobalAlloc(
            ::std::mem::transmute(uflags),
            ::std::mem::transmute(dwbytes),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GlobalFlags(hmem: isize) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GlobalFlags(hmem: isize) -> u32;
        }
        ::std::mem::transmute(GlobalFlags(::std::mem::transmute(hmem)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GlobalFree(hmem: isize) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GlobalFree(hmem: isize) -> isize;
        }
        ::std::mem::transmute(GlobalFree(::std::mem::transmute(hmem)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GlobalHandle(pmem: *const ::std::ffi::c_void) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GlobalHandle(pmem: *const ::std::ffi::c_void) -> isize;
        }
        ::std::mem::transmute(GlobalHandle(::std::mem::transmute(pmem)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GlobalLock(hmem: isize) -> *mut ::std::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GlobalLock(hmem: isize) -> *mut ::std::ffi::c_void;
        }
        ::std::mem::transmute(GlobalLock(::std::mem::transmute(hmem)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GlobalReAlloc(hmem: isize, dwbytes: usize, uflags: u32) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GlobalReAlloc(hmem: isize, dwbytes: usize, uflags: u32) -> isize;
        }
        ::std::mem::transmute(GlobalReAlloc(
            ::std::mem::transmute(hmem),
            ::std::mem::transmute(dwbytes),
            ::std::mem::transmute(uflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GlobalSize(hmem: isize) -> usize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GlobalSize(hmem: isize) -> usize;
        }
        ::std::mem::transmute(GlobalSize(::std::mem::transmute(hmem)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GlobalUnlock(hmem: isize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GlobalUnlock(hmem: isize) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GlobalUnlock(::std::mem::transmute(hmem)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct HEAP_FLAGS(pub u32);
pub const HEAP_NONE: HEAP_FLAGS = HEAP_FLAGS(0u32);
pub const HEAP_NO_SERIALIZE: HEAP_FLAGS = HEAP_FLAGS(1u32);
pub const HEAP_GROWABLE: HEAP_FLAGS = HEAP_FLAGS(2u32);
pub const HEAP_GENERATE_EXCEPTIONS: HEAP_FLAGS = HEAP_FLAGS(4u32);
pub const HEAP_ZERO_MEMORY: HEAP_FLAGS = HEAP_FLAGS(8u32);
pub const HEAP_REALLOC_IN_PLACE_ONLY: HEAP_FLAGS = HEAP_FLAGS(16u32);
pub const HEAP_TAIL_CHECKING_ENABLED: HEAP_FLAGS = HEAP_FLAGS(32u32);
pub const HEAP_FREE_CHECKING_ENABLED: HEAP_FLAGS = HEAP_FLAGS(64u32);
pub const HEAP_DISABLE_COALESCE_ON_FREE: HEAP_FLAGS = HEAP_FLAGS(128u32);
pub const HEAP_CREATE_ALIGN_16: HEAP_FLAGS = HEAP_FLAGS(65536u32);
pub const HEAP_CREATE_ENABLE_TRACING: HEAP_FLAGS = HEAP_FLAGS(131072u32);
pub const HEAP_CREATE_ENABLE_EXECUTE: HEAP_FLAGS = HEAP_FLAGS(262144u32);
pub const HEAP_MAXIMUM_TAG: HEAP_FLAGS = HEAP_FLAGS(4095u32);
pub const HEAP_PSEUDO_TAG_FLAG: HEAP_FLAGS = HEAP_FLAGS(32768u32);
pub const HEAP_TAG_SHIFT: HEAP_FLAGS = HEAP_FLAGS(18u32);
pub const HEAP_CREATE_SEGMENT_HEAP: HEAP_FLAGS = HEAP_FLAGS(256u32);
pub const HEAP_CREATE_HARDENED: HEAP_FLAGS = HEAP_FLAGS(512u32);
impl ::std::convert::From<u32> for HEAP_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HEAP_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for HEAP_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for HEAP_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for HEAP_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for HEAP_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for HEAP_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct HEAP_SUMMARY {
    pub cb: u32,
    pub cbAllocated: usize,
    pub cbCommitted: usize,
    pub cbReserved: usize,
    pub cbMaxReserve: usize,
}
impl HEAP_SUMMARY {}
impl ::std::default::Default for HEAP_SUMMARY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for HEAP_SUMMARY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HEAP_SUMMARY")
            .field("cb", &self.cb)
            .field("cbAllocated", &self.cbAllocated)
            .field("cbCommitted", &self.cbCommitted)
            .field("cbReserved", &self.cbReserved)
            .field("cbMaxReserve", &self.cbMaxReserve)
            .finish()
    }
}
impl ::std::cmp::PartialEq for HEAP_SUMMARY {
    fn eq(&self, other: &Self) -> bool {
        self.cb == other.cb
            && self.cbAllocated == other.cbAllocated
            && self.cbCommitted == other.cbCommitted
            && self.cbReserved == other.cbReserved
            && self.cbMaxReserve == other.cbMaxReserve
    }
}
impl ::std::cmp::Eq for HEAP_SUMMARY {}
unsafe impl ::windows::runtime::Abi for HEAP_SUMMARY {
    type Abi = Self;
    type DefaultType = Self;
}
#[inline]
pub unsafe fn HeapAlloc<'a, Param0: ::windows::runtime::IntoParam<'a, HeapHandle>>(
    hheap: Param0,
    dwflags: HEAP_FLAGS,
    dwbytes: usize,
) -> *mut ::std::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HeapAlloc(
                hheap: HeapHandle,
                dwflags: HEAP_FLAGS,
                dwbytes: usize,
            ) -> *mut ::std::ffi::c_void;
        }
        ::std::mem::transmute(HeapAlloc(
            hheap.into_param().abi(),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(dwbytes),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn HeapCompact<'a, Param0: ::windows::runtime::IntoParam<'a, HeapHandle>>(
    hheap: Param0,
    dwflags: HEAP_FLAGS,
) -> usize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HeapCompact(hheap: HeapHandle, dwflags: HEAP_FLAGS) -> usize;
        }
        ::std::mem::transmute(HeapCompact(
            hheap.into_param().abi(),
            ::std::mem::transmute(dwflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn HeapCreate(
    floptions: HEAP_FLAGS,
    dwinitialsize: usize,
    dwmaximumsize: usize,
) -> HeapHandle {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HeapCreate(
                floptions: HEAP_FLAGS,
                dwinitialsize: usize,
                dwmaximumsize: usize,
            ) -> HeapHandle;
        }
        ::std::mem::transmute(HeapCreate(
            ::std::mem::transmute(floptions),
            ::std::mem::transmute(dwinitialsize),
            ::std::mem::transmute(dwmaximumsize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HeapDestroy<'a, Param0: ::windows::runtime::IntoParam<'a, HeapHandle>>(
    hheap: Param0,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HeapDestroy(hheap: HeapHandle) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(HeapDestroy(hheap.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HeapFree<'a, Param0: ::windows::runtime::IntoParam<'a, HeapHandle>>(
    hheap: Param0,
    dwflags: HEAP_FLAGS,
    lpmem: *const ::std::ffi::c_void,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HeapFree(
                hheap: HeapHandle,
                dwflags: HEAP_FLAGS,
                lpmem: *const ::std::ffi::c_void,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(HeapFree(
            hheap.into_param().abi(),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(lpmem),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(
    :: std :: clone :: Clone,
    :: std :: marker :: Copy,
    :: std :: fmt :: Debug,
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
)]
#[repr(transparent)]
pub struct HeapHandle(pub isize);
impl ::std::default::Default for HeapHandle {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for HeapHandle {}
unsafe impl ::windows::runtime::Abi for HeapHandle {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HeapLock<'a, Param0: ::windows::runtime::IntoParam<'a, HeapHandle>>(
    hheap: Param0,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HeapLock(hheap: HeapHandle) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(HeapLock(hheap.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
#[inline]
pub unsafe fn HeapQueryInformation<'a, Param0: ::windows::runtime::IntoParam<'a, HeapHandle>>(
    heaphandle: Param0,
    heapinformationclass: super::SystemServices::HEAP_INFORMATION_CLASS,
    heapinformation: *mut ::std::ffi::c_void,
    heapinformationlength: usize,
    returnlength: *mut usize,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HeapQueryInformation(
                heaphandle: HeapHandle,
                heapinformationclass: super::SystemServices::HEAP_INFORMATION_CLASS,
                heapinformation: *mut ::std::ffi::c_void,
                heapinformationlength: usize,
                returnlength: *mut usize,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(HeapQueryInformation(
            heaphandle.into_param().abi(),
            ::std::mem::transmute(heapinformationclass),
            ::std::mem::transmute(heapinformation),
            ::std::mem::transmute(heapinformationlength),
            ::std::mem::transmute(returnlength),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn HeapReAlloc<'a, Param0: ::windows::runtime::IntoParam<'a, HeapHandle>>(
    hheap: Param0,
    dwflags: HEAP_FLAGS,
    lpmem: *const ::std::ffi::c_void,
    dwbytes: usize,
) -> *mut ::std::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HeapReAlloc(
                hheap: HeapHandle,
                dwflags: HEAP_FLAGS,
                lpmem: *const ::std::ffi::c_void,
                dwbytes: usize,
            ) -> *mut ::std::ffi::c_void;
        }
        ::std::mem::transmute(HeapReAlloc(
            hheap.into_param().abi(),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(lpmem),
            ::std::mem::transmute(dwbytes),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
#[inline]
pub unsafe fn HeapSetInformation<'a, Param0: ::windows::runtime::IntoParam<'a, HeapHandle>>(
    heaphandle: Param0,
    heapinformationclass: super::SystemServices::HEAP_INFORMATION_CLASS,
    heapinformation: *const ::std::ffi::c_void,
    heapinformationlength: usize,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HeapSetInformation(
                heaphandle: HeapHandle,
                heapinformationclass: super::SystemServices::HEAP_INFORMATION_CLASS,
                heapinformation: *const ::std::ffi::c_void,
                heapinformationlength: usize,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(HeapSetInformation(
            heaphandle.into_param().abi(),
            ::std::mem::transmute(heapinformationclass),
            ::std::mem::transmute(heapinformation),
            ::std::mem::transmute(heapinformationlength),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn HeapSize<'a, Param0: ::windows::runtime::IntoParam<'a, HeapHandle>>(
    hheap: Param0,
    dwflags: HEAP_FLAGS,
    lpmem: *const ::std::ffi::c_void,
) -> usize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HeapSize(
                hheap: HeapHandle,
                dwflags: HEAP_FLAGS,
                lpmem: *const ::std::ffi::c_void,
            ) -> usize;
        }
        ::std::mem::transmute(HeapSize(
            hheap.into_param().abi(),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(lpmem),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HeapSummary<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hheap: Param0,
    dwflags: u32,
    lpsummary: *mut HEAP_SUMMARY,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HeapSummary(
                hheap: super::super::Foundation::HANDLE,
                dwflags: u32,
                lpsummary: *mut HEAP_SUMMARY,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(HeapSummary(
            hheap.into_param().abi(),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(lpsummary),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HeapUnlock<'a, Param0: ::windows::runtime::IntoParam<'a, HeapHandle>>(
    hheap: Param0,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HeapUnlock(hheap: HeapHandle) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(HeapUnlock(hheap.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HeapValidate<'a, Param0: ::windows::runtime::IntoParam<'a, HeapHandle>>(
    hheap: Param0,
    dwflags: HEAP_FLAGS,
    lpmem: *const ::std::ffi::c_void,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HeapValidate(
                hheap: HeapHandle,
                dwflags: HEAP_FLAGS,
                lpmem: *const ::std::ffi::c_void,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(HeapValidate(
            hheap.into_param().abi(),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(lpmem),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
#[inline]
pub unsafe fn HeapWalk<'a, Param0: ::windows::runtime::IntoParam<'a, HeapHandle>>(
    hheap: Param0,
    lpentry: *mut super::SystemServices::PROCESS_HEAP_ENTRY,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HeapWalk(
                hheap: HeapHandle,
                lpentry: *mut super::SystemServices::PROCESS_HEAP_ENTRY,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(HeapWalk(
            hheap.into_param().abi(),
            ::std::mem::transmute(lpentry),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsBadCodePtr(
    lpfn: ::std::option::Option<super::super::Foundation::FARPROC>,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsBadCodePtr(lpfn: ::windows::runtime::RawPtr) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(IsBadCodePtr(::std::mem::transmute(lpfn)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsBadReadPtr(
    lp: *const ::std::ffi::c_void,
    ucb: usize,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsBadReadPtr(
                lp: *const ::std::ffi::c_void,
                ucb: usize,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(IsBadReadPtr(
            ::std::mem::transmute(lp),
            ::std::mem::transmute(ucb),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsBadStringPtrA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    lpsz: Param0,
    ucchmax: usize,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsBadStringPtrA(
                lpsz: super::super::Foundation::PSTR,
                ucchmax: usize,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(IsBadStringPtrA(
            lpsz.into_param().abi(),
            ::std::mem::transmute(ucchmax),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsBadStringPtrW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpsz: Param0,
    ucchmax: usize,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsBadStringPtrW(
                lpsz: super::super::Foundation::PWSTR,
                ucchmax: usize,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(IsBadStringPtrW(
            lpsz.into_param().abi(),
            ::std::mem::transmute(ucchmax),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsBadWritePtr(
    lp: *const ::std::ffi::c_void,
    ucb: usize,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsBadWritePtr(
                lp: *const ::std::ffi::c_void,
                ucb: usize,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(IsBadWritePtr(
            ::std::mem::transmute(lp),
            ::std::mem::transmute(ucb),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct LOCAL_ALLOC_FLAGS(pub u32);
pub const LHND: LOCAL_ALLOC_FLAGS = LOCAL_ALLOC_FLAGS(66u32);
pub const LMEM_FIXED: LOCAL_ALLOC_FLAGS = LOCAL_ALLOC_FLAGS(0u32);
pub const LMEM_MOVEABLE: LOCAL_ALLOC_FLAGS = LOCAL_ALLOC_FLAGS(2u32);
pub const LMEM_ZEROINIT: LOCAL_ALLOC_FLAGS = LOCAL_ALLOC_FLAGS(64u32);
pub const LPTR: LOCAL_ALLOC_FLAGS = LOCAL_ALLOC_FLAGS(64u32);
pub const NONZEROLHND: LOCAL_ALLOC_FLAGS = LOCAL_ALLOC_FLAGS(2u32);
pub const NONZEROLPTR: LOCAL_ALLOC_FLAGS = LOCAL_ALLOC_FLAGS(0u32);
impl ::std::convert::From<u32> for LOCAL_ALLOC_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for LOCAL_ALLOC_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for LOCAL_ALLOC_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for LOCAL_ALLOC_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for LOCAL_ALLOC_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for LOCAL_ALLOC_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for LOCAL_ALLOC_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[inline]
pub unsafe fn LocalAlloc(uflags: LOCAL_ALLOC_FLAGS, ubytes: usize) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LocalAlloc(uflags: LOCAL_ALLOC_FLAGS, ubytes: usize) -> isize;
        }
        ::std::mem::transmute(LocalAlloc(
            ::std::mem::transmute(uflags),
            ::std::mem::transmute(ubytes),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn LocalFlags(hmem: isize) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LocalFlags(hmem: isize) -> u32;
        }
        ::std::mem::transmute(LocalFlags(::std::mem::transmute(hmem)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn LocalFree(hmem: isize) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LocalFree(hmem: isize) -> isize;
        }
        ::std::mem::transmute(LocalFree(::std::mem::transmute(hmem)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn LocalHandle(pmem: *const ::std::ffi::c_void) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LocalHandle(pmem: *const ::std::ffi::c_void) -> isize;
        }
        ::std::mem::transmute(LocalHandle(::std::mem::transmute(pmem)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn LocalLock(hmem: isize) -> *mut ::std::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LocalLock(hmem: isize) -> *mut ::std::ffi::c_void;
        }
        ::std::mem::transmute(LocalLock(::std::mem::transmute(hmem)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn LocalReAlloc(hmem: isize, ubytes: usize, uflags: u32) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LocalReAlloc(hmem: isize, ubytes: usize, uflags: u32) -> isize;
        }
        ::std::mem::transmute(LocalReAlloc(
            ::std::mem::transmute(hmem),
            ::std::mem::transmute(ubytes),
            ::std::mem::transmute(uflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn LocalSize(hmem: isize) -> usize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LocalSize(hmem: isize) -> usize;
        }
        ::std::mem::transmute(LocalSize(::std::mem::transmute(hmem)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LocalUnlock(hmem: isize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LocalUnlock(hmem: isize) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(LocalUnlock(::std::mem::transmute(hmem)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const MEHC_PATROL_SCRUBBER_PRESENT: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct MEMORY_BASIC_INFORMATION {
    pub BaseAddress: *mut ::std::ffi::c_void,
    pub AllocationBase: *mut ::std::ffi::c_void,
    pub AllocationProtect: PAGE_PROTECTION_FLAGS,
    pub PartitionId: u16,
    pub RegionSize: usize,
    pub State: VIRTUAL_ALLOCATION_TYPE,
    pub Protect: PAGE_PROTECTION_FLAGS,
    pub Type: PAGE_TYPE,
}
impl MEMORY_BASIC_INFORMATION {}
impl ::std::default::Default for MEMORY_BASIC_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for MEMORY_BASIC_INFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MEMORY_BASIC_INFORMATION")
            .field("BaseAddress", &self.BaseAddress)
            .field("AllocationBase", &self.AllocationBase)
            .field("AllocationProtect", &self.AllocationProtect)
            .field("PartitionId", &self.PartitionId)
            .field("RegionSize", &self.RegionSize)
            .field("State", &self.State)
            .field("Protect", &self.Protect)
            .field("Type", &self.Type)
            .finish()
    }
}
impl ::std::cmp::PartialEq for MEMORY_BASIC_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.BaseAddress == other.BaseAddress
            && self.AllocationBase == other.AllocationBase
            && self.AllocationProtect == other.AllocationProtect
            && self.PartitionId == other.PartitionId
            && self.RegionSize == other.RegionSize
            && self.State == other.State
            && self.Protect == other.Protect
            && self.Type == other.Type
    }
}
impl ::std::cmp::Eq for MEMORY_BASIC_INFORMATION {}
unsafe impl ::windows::runtime::Abi for MEMORY_BASIC_INFORMATION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct MEMORY_BASIC_INFORMATION32 {
    pub BaseAddress: u32,
    pub AllocationBase: u32,
    pub AllocationProtect: PAGE_PROTECTION_FLAGS,
    pub RegionSize: u32,
    pub State: VIRTUAL_ALLOCATION_TYPE,
    pub Protect: PAGE_PROTECTION_FLAGS,
    pub Type: PAGE_TYPE,
}
impl MEMORY_BASIC_INFORMATION32 {}
impl ::std::default::Default for MEMORY_BASIC_INFORMATION32 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for MEMORY_BASIC_INFORMATION32 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MEMORY_BASIC_INFORMATION32")
            .field("BaseAddress", &self.BaseAddress)
            .field("AllocationBase", &self.AllocationBase)
            .field("AllocationProtect", &self.AllocationProtect)
            .field("RegionSize", &self.RegionSize)
            .field("State", &self.State)
            .field("Protect", &self.Protect)
            .field("Type", &self.Type)
            .finish()
    }
}
impl ::std::cmp::PartialEq for MEMORY_BASIC_INFORMATION32 {
    fn eq(&self, other: &Self) -> bool {
        self.BaseAddress == other.BaseAddress
            && self.AllocationBase == other.AllocationBase
            && self.AllocationProtect == other.AllocationProtect
            && self.RegionSize == other.RegionSize
            && self.State == other.State
            && self.Protect == other.Protect
            && self.Type == other.Type
    }
}
impl ::std::cmp::Eq for MEMORY_BASIC_INFORMATION32 {}
unsafe impl ::windows::runtime::Abi for MEMORY_BASIC_INFORMATION32 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl MEMORY_BASIC_INFORMATION64 {}
impl ::std::default::Default for MEMORY_BASIC_INFORMATION64 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for MEMORY_BASIC_INFORMATION64 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MEMORY_BASIC_INFORMATION64")
            .field("BaseAddress", &self.BaseAddress)
            .field("AllocationBase", &self.AllocationBase)
            .field("AllocationProtect", &self.AllocationProtect)
            .field("__alignment1", &self.__alignment1)
            .field("RegionSize", &self.RegionSize)
            .field("State", &self.State)
            .field("Protect", &self.Protect)
            .field("Type", &self.Type)
            .field("__alignment2", &self.__alignment2)
            .finish()
    }
}
impl ::std::cmp::PartialEq for MEMORY_BASIC_INFORMATION64 {
    fn eq(&self, other: &Self) -> bool {
        self.BaseAddress == other.BaseAddress
            && self.AllocationBase == other.AllocationBase
            && self.AllocationProtect == other.AllocationProtect
            && self.__alignment1 == other.__alignment1
            && self.RegionSize == other.RegionSize
            && self.State == other.State
            && self.Protect == other.Protect
            && self.Type == other.Type
            && self.__alignment2 == other.__alignment2
    }
}
impl ::std::cmp::Eq for MEMORY_BASIC_INFORMATION64 {}
unsafe impl ::windows::runtime::Abi for MEMORY_BASIC_INFORMATION64 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct MEMORY_RESOURCE_NOTIFICATION_TYPE(pub i32);
pub const LowMemoryResourceNotification: MEMORY_RESOURCE_NOTIFICATION_TYPE =
    MEMORY_RESOURCE_NOTIFICATION_TYPE(0i32);
pub const HighMemoryResourceNotification: MEMORY_RESOURCE_NOTIFICATION_TYPE =
    MEMORY_RESOURCE_NOTIFICATION_TYPE(1i32);
impl ::std::convert::From<i32> for MEMORY_RESOURCE_NOTIFICATION_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MEMORY_RESOURCE_NOTIFICATION_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MapUserPhysicalPages(
    virtualaddress: *const ::std::ffi::c_void,
    numberofpages: usize,
    pagearray: *const usize,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MapUserPhysicalPages(
                virtualaddress: *const ::std::ffi::c_void,
                numberofpages: usize,
                pagearray: *const usize,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(MapUserPhysicalPages(
            ::std::mem::transmute(virtualaddress),
            ::std::mem::transmute(numberofpages),
            ::std::mem::transmute(pagearray),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MapUserPhysicalPagesScatter(
    virtualaddresses: *const *const ::std::ffi::c_void,
    numberofpages: usize,
    pagearray: *const usize,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MapUserPhysicalPagesScatter(
                virtualaddresses: *const *const ::std::ffi::c_void,
                numberofpages: usize,
                pagearray: *const usize,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(MapUserPhysicalPagesScatter(
            ::std::mem::transmute(virtualaddresses),
            ::std::mem::transmute(numberofpages),
            ::std::mem::transmute(pagearray),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MapViewOfFile<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hfilemappingobject: Param0,
    dwdesiredaccess: FILE_MAP,
    dwfileoffsethigh: u32,
    dwfileoffsetlow: u32,
    dwnumberofbytestomap: usize,
) -> *mut ::std::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MapViewOfFile(
                hfilemappingobject: super::super::Foundation::HANDLE,
                dwdesiredaccess: FILE_MAP,
                dwfileoffsethigh: u32,
                dwfileoffsetlow: u32,
                dwnumberofbytestomap: usize,
            ) -> *mut ::std::ffi::c_void;
        }
        ::std::mem::transmute(MapViewOfFile(
            hfilemappingobject.into_param().abi(),
            ::std::mem::transmute(dwdesiredaccess),
            ::std::mem::transmute(dwfileoffsethigh),
            ::std::mem::transmute(dwfileoffsetlow),
            ::std::mem::transmute(dwnumberofbytestomap),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
#[inline]
pub unsafe fn MapViewOfFile3<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    filemapping: Param0,
    process: Param1,
    baseaddress: *const ::std::ffi::c_void,
    offset: u64,
    viewsize: usize,
    allocationtype: VIRTUAL_ALLOCATION_TYPE,
    pageprotection: u32,
    extendedparameters: *mut super::SystemServices::MEM_EXTENDED_PARAMETER,
    parametercount: u32,
) -> *mut ::std::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MapViewOfFile3(
                filemapping: super::super::Foundation::HANDLE,
                process: super::super::Foundation::HANDLE,
                baseaddress: *const ::std::ffi::c_void,
                offset: u64,
                viewsize: usize,
                allocationtype: VIRTUAL_ALLOCATION_TYPE,
                pageprotection: u32,
                extendedparameters: *mut super::SystemServices::MEM_EXTENDED_PARAMETER,
                parametercount: u32,
            ) -> *mut ::std::ffi::c_void;
        }
        ::std::mem::transmute(MapViewOfFile3(
            filemapping.into_param().abi(),
            process.into_param().abi(),
            ::std::mem::transmute(baseaddress),
            ::std::mem::transmute(offset),
            ::std::mem::transmute(viewsize),
            ::std::mem::transmute(allocationtype),
            ::std::mem::transmute(pageprotection),
            ::std::mem::transmute(extendedparameters),
            ::std::mem::transmute(parametercount),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
#[inline]
pub unsafe fn MapViewOfFile3FromApp<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    filemapping: Param0,
    process: Param1,
    baseaddress: *const ::std::ffi::c_void,
    offset: u64,
    viewsize: usize,
    allocationtype: VIRTUAL_ALLOCATION_TYPE,
    pageprotection: u32,
    extendedparameters: *mut super::SystemServices::MEM_EXTENDED_PARAMETER,
    parametercount: u32,
) -> *mut ::std::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MapViewOfFile3FromApp(
                filemapping: super::super::Foundation::HANDLE,
                process: super::super::Foundation::HANDLE,
                baseaddress: *const ::std::ffi::c_void,
                offset: u64,
                viewsize: usize,
                allocationtype: VIRTUAL_ALLOCATION_TYPE,
                pageprotection: u32,
                extendedparameters: *mut super::SystemServices::MEM_EXTENDED_PARAMETER,
                parametercount: u32,
            ) -> *mut ::std::ffi::c_void;
        }
        ::std::mem::transmute(MapViewOfFile3FromApp(
            filemapping.into_param().abi(),
            process.into_param().abi(),
            ::std::mem::transmute(baseaddress),
            ::std::mem::transmute(offset),
            ::std::mem::transmute(viewsize),
            ::std::mem::transmute(allocationtype),
            ::std::mem::transmute(pageprotection),
            ::std::mem::transmute(extendedparameters),
            ::std::mem::transmute(parametercount),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MapViewOfFileEx<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hfilemappingobject: Param0,
    dwdesiredaccess: FILE_MAP,
    dwfileoffsethigh: u32,
    dwfileoffsetlow: u32,
    dwnumberofbytestomap: usize,
    lpbaseaddress: *const ::std::ffi::c_void,
) -> *mut ::std::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MapViewOfFileEx(
                hfilemappingobject: super::super::Foundation::HANDLE,
                dwdesiredaccess: FILE_MAP,
                dwfileoffsethigh: u32,
                dwfileoffsetlow: u32,
                dwnumberofbytestomap: usize,
                lpbaseaddress: *const ::std::ffi::c_void,
            ) -> *mut ::std::ffi::c_void;
        }
        ::std::mem::transmute(MapViewOfFileEx(
            hfilemappingobject.into_param().abi(),
            ::std::mem::transmute(dwdesiredaccess),
            ::std::mem::transmute(dwfileoffsethigh),
            ::std::mem::transmute(dwfileoffsetlow),
            ::std::mem::transmute(dwnumberofbytestomap),
            ::std::mem::transmute(lpbaseaddress),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MapViewOfFileExNuma<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hfilemappingobject: Param0,
    dwdesiredaccess: FILE_MAP,
    dwfileoffsethigh: u32,
    dwfileoffsetlow: u32,
    dwnumberofbytestomap: usize,
    lpbaseaddress: *const ::std::ffi::c_void,
    nndpreferred: u32,
) -> *mut ::std::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MapViewOfFileExNuma(
                hfilemappingobject: super::super::Foundation::HANDLE,
                dwdesiredaccess: FILE_MAP,
                dwfileoffsethigh: u32,
                dwfileoffsetlow: u32,
                dwnumberofbytestomap: usize,
                lpbaseaddress: *const ::std::ffi::c_void,
                nndpreferred: u32,
            ) -> *mut ::std::ffi::c_void;
        }
        ::std::mem::transmute(MapViewOfFileExNuma(
            hfilemappingobject.into_param().abi(),
            ::std::mem::transmute(dwdesiredaccess),
            ::std::mem::transmute(dwfileoffsethigh),
            ::std::mem::transmute(dwfileoffsetlow),
            ::std::mem::transmute(dwnumberofbytestomap),
            ::std::mem::transmute(lpbaseaddress),
            ::std::mem::transmute(nndpreferred),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MapViewOfFileFromApp<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hfilemappingobject: Param0,
    desiredaccess: FILE_MAP,
    fileoffset: u64,
    numberofbytestomap: usize,
) -> *mut ::std::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MapViewOfFileFromApp(
                hfilemappingobject: super::super::Foundation::HANDLE,
                desiredaccess: FILE_MAP,
                fileoffset: u64,
                numberofbytestomap: usize,
            ) -> *mut ::std::ffi::c_void;
        }
        ::std::mem::transmute(MapViewOfFileFromApp(
            hfilemappingobject.into_param().abi(),
            ::std::mem::transmute(desiredaccess),
            ::std::mem::transmute(fileoffset),
            ::std::mem::transmute(numberofbytestomap),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MapViewOfFileNuma2<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    filemappinghandle: Param0,
    processhandle: Param1,
    offset: u64,
    baseaddress: *const ::std::ffi::c_void,
    viewsize: usize,
    allocationtype: u32,
    pageprotection: u32,
    preferrednode: u32,
) -> *mut ::std::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MapViewOfFileNuma2(
                filemappinghandle: super::super::Foundation::HANDLE,
                processhandle: super::super::Foundation::HANDLE,
                offset: u64,
                baseaddress: *const ::std::ffi::c_void,
                viewsize: usize,
                allocationtype: u32,
                pageprotection: u32,
                preferrednode: u32,
            ) -> *mut ::std::ffi::c_void;
        }
        ::std::mem::transmute(MapViewOfFileNuma2(
            filemappinghandle.into_param().abi(),
            processhandle.into_param().abi(),
            ::std::mem::transmute(offset),
            ::std::mem::transmute(baseaddress),
            ::std::mem::transmute(viewsize),
            ::std::mem::transmute(allocationtype),
            ::std::mem::transmute(pageprotection),
            ::std::mem::transmute(preferrednode),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct OFFER_PRIORITY(pub i32);
pub const VmOfferPriorityVeryLow: OFFER_PRIORITY = OFFER_PRIORITY(1i32);
pub const VmOfferPriorityLow: OFFER_PRIORITY = OFFER_PRIORITY(2i32);
pub const VmOfferPriorityBelowNormal: OFFER_PRIORITY = OFFER_PRIORITY(3i32);
pub const VmOfferPriorityNormal: OFFER_PRIORITY = OFFER_PRIORITY(4i32);
impl ::std::convert::From<i32> for OFFER_PRIORITY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for OFFER_PRIORITY {
    type Abi = Self;
    type DefaultType = Self;
}
#[inline]
pub unsafe fn OfferVirtualMemory(
    virtualaddress: *mut ::std::ffi::c_void,
    size: usize,
    priority: OFFER_PRIORITY,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OfferVirtualMemory(
                virtualaddress: *mut ::std::ffi::c_void,
                size: usize,
                priority: OFFER_PRIORITY,
            ) -> u32;
        }
        ::std::mem::transmute(OfferVirtualMemory(
            ::std::mem::transmute(virtualaddress),
            ::std::mem::transmute(size),
            ::std::mem::transmute(priority),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenDedicatedMemoryPartition<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    partition: Param0,
    dedicatedmemorytypeid: u64,
    desiredaccess: u32,
    inherithandle: Param3,
) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenDedicatedMemoryPartition(
                partition: super::super::Foundation::HANDLE,
                dedicatedmemorytypeid: u64,
                desiredaccess: u32,
                inherithandle: super::super::Foundation::BOOL,
            ) -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(OpenDedicatedMemoryPartition(
            partition.into_param().abi(),
            ::std::mem::transmute(dedicatedmemorytypeid),
            ::std::mem::transmute(desiredaccess),
            inherithandle.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenFileMappingA<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    dwdesiredaccess: u32,
    binherithandle: Param1,
    lpname: Param2,
) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenFileMappingA(
                dwdesiredaccess: u32,
                binherithandle: super::super::Foundation::BOOL,
                lpname: super::super::Foundation::PSTR,
            ) -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(OpenFileMappingA(
            ::std::mem::transmute(dwdesiredaccess),
            binherithandle.into_param().abi(),
            lpname.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenFileMappingFromApp<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    desiredaccess: u32,
    inherithandle: Param1,
    name: Param2,
) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenFileMappingFromApp(
                desiredaccess: u32,
                inherithandle: super::super::Foundation::BOOL,
                name: super::super::Foundation::PWSTR,
            ) -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(OpenFileMappingFromApp(
            ::std::mem::transmute(desiredaccess),
            inherithandle.into_param().abi(),
            name.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenFileMappingW<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    dwdesiredaccess: u32,
    binherithandle: Param1,
    lpname: Param2,
) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenFileMappingW(
                dwdesiredaccess: u32,
                binherithandle: super::super::Foundation::BOOL,
                lpname: super::super::Foundation::PWSTR,
            ) -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(OpenFileMappingW(
            ::std::mem::transmute(dwdesiredaccess),
            binherithandle.into_param().abi(),
            lpname.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct PAGE_PROTECTION_FLAGS(pub u32);
pub const PAGE_NOACCESS: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(1u32);
pub const PAGE_READONLY: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(2u32);
pub const PAGE_READWRITE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(4u32);
pub const PAGE_WRITECOPY: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(8u32);
pub const PAGE_EXECUTE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(16u32);
pub const PAGE_EXECUTE_READ: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(32u32);
pub const PAGE_EXECUTE_READWRITE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(64u32);
pub const PAGE_EXECUTE_WRITECOPY: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(128u32);
pub const PAGE_GUARD: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(256u32);
pub const PAGE_NOCACHE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(512u32);
pub const PAGE_WRITECOMBINE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(1024u32);
pub const PAGE_GRAPHICS_NOACCESS: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(2048u32);
pub const PAGE_GRAPHICS_READONLY: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(4096u32);
pub const PAGE_GRAPHICS_READWRITE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(8192u32);
pub const PAGE_GRAPHICS_EXECUTE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(16384u32);
pub const PAGE_GRAPHICS_EXECUTE_READ: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(32768u32);
pub const PAGE_GRAPHICS_EXECUTE_READWRITE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(65536u32);
pub const PAGE_GRAPHICS_COHERENT: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(131072u32);
pub const PAGE_GRAPHICS_NOCACHE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(262144u32);
pub const PAGE_ENCLAVE_THREAD_CONTROL: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(2147483648u32);
pub const PAGE_REVERT_TO_FILE_MAP: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(2147483648u32);
pub const PAGE_TARGETS_NO_UPDATE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(1073741824u32);
pub const PAGE_TARGETS_INVALID: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(1073741824u32);
pub const PAGE_ENCLAVE_UNVALIDATED: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(536870912u32);
pub const PAGE_ENCLAVE_MASK: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(268435456u32);
pub const PAGE_ENCLAVE_DECOMMIT: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(268435456u32);
pub const PAGE_ENCLAVE_SS_FIRST: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(268435457u32);
pub const PAGE_ENCLAVE_SS_REST: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(268435458u32);
pub const SEC_PARTITION_OWNER_HANDLE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(262144u32);
pub const SEC_64K_PAGES: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(524288u32);
pub const SEC_FILE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(8388608u32);
pub const SEC_IMAGE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(16777216u32);
pub const SEC_PROTECTED_IMAGE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(33554432u32);
pub const SEC_RESERVE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(67108864u32);
pub const SEC_COMMIT: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(134217728u32);
pub const SEC_NOCACHE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(268435456u32);
pub const SEC_WRITECOMBINE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(1073741824u32);
pub const SEC_LARGE_PAGES: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(2147483648u32);
pub const SEC_IMAGE_NO_EXECUTE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(285212672u32);
impl ::std::convert::From<u32> for PAGE_PROTECTION_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PAGE_PROTECTION_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for PAGE_PROTECTION_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for PAGE_PROTECTION_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for PAGE_PROTECTION_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for PAGE_PROTECTION_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for PAGE_PROTECTION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct PAGE_TYPE(pub u32);
pub const MEM_PRIVATE: PAGE_TYPE = PAGE_TYPE(131072u32);
pub const MEM_MAPPED: PAGE_TYPE = PAGE_TYPE(262144u32);
pub const MEM_IMAGE: PAGE_TYPE = PAGE_TYPE(16777216u32);
impl ::std::convert::From<u32> for PAGE_TYPE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PAGE_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for PAGE_TYPE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for PAGE_TYPE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for PAGE_TYPE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for PAGE_TYPE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for PAGE_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub type PBAD_MEMORY_CALLBACK_ROUTINE = unsafe extern "system" fn();
#[cfg(feature = "Win32_Foundation")]
pub type PSECURE_MEMORY_CACHE_CALLBACK =
    unsafe extern "system" fn(
        addr: *const ::std::ffi::c_void,
        range: usize,
    ) -> super::super::Foundation::BOOLEAN;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PrefetchVirtualMemory<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hprocess: Param0,
    numberofentries: usize,
    virtualaddresses: *const WIN32_MEMORY_RANGE_ENTRY,
    flags: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PrefetchVirtualMemory(
                hprocess: super::super::Foundation::HANDLE,
                numberofentries: usize,
                virtualaddresses: *const WIN32_MEMORY_RANGE_ENTRY,
                flags: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(PrefetchVirtualMemory(
            hprocess.into_param().abi(),
            ::std::mem::transmute(numberofentries),
            ::std::mem::transmute(virtualaddresses),
            ::std::mem::transmute(flags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn QueryMemoryResourceNotification<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    resourcenotificationhandle: Param0,
    resourcestate: *mut super::super::Foundation::BOOL,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryMemoryResourceNotification(
                resourcenotificationhandle: super::super::Foundation::HANDLE,
                resourcestate: *mut super::super::Foundation::BOOL,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(QueryMemoryResourceNotification(
            resourcenotificationhandle.into_param().abi(),
            ::std::mem::transmute(resourcestate),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn QueryPartitionInformation<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    partition: Param0,
    partitioninformationclass: WIN32_MEMORY_PARTITION_INFORMATION_CLASS,
    partitioninformation: *mut ::std::ffi::c_void,
    partitioninformationlength: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryPartitionInformation(
                partition: super::super::Foundation::HANDLE,
                partitioninformationclass: WIN32_MEMORY_PARTITION_INFORMATION_CLASS,
                partitioninformation: *mut ::std::ffi::c_void,
                partitioninformationlength: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(QueryPartitionInformation(
            partition.into_param().abi(),
            ::std::mem::transmute(partitioninformationclass),
            ::std::mem::transmute(partitioninformation),
            ::std::mem::transmute(partitioninformationlength),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn QueryVirtualMemoryInformation<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    process: Param0,
    virtualaddress: *const ::std::ffi::c_void,
    memoryinformationclass: WIN32_MEMORY_INFORMATION_CLASS,
    memoryinformation: *mut ::std::ffi::c_void,
    memoryinformationsize: usize,
    returnsize: *mut usize,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryVirtualMemoryInformation(
                process: super::super::Foundation::HANDLE,
                virtualaddress: *const ::std::ffi::c_void,
                memoryinformationclass: WIN32_MEMORY_INFORMATION_CLASS,
                memoryinformation: *mut ::std::ffi::c_void,
                memoryinformationsize: usize,
                returnsize: *mut usize,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(QueryVirtualMemoryInformation(
            process.into_param().abi(),
            ::std::mem::transmute(virtualaddress),
            ::std::mem::transmute(memoryinformationclass),
            ::std::mem::transmute(memoryinformation),
            ::std::mem::transmute(memoryinformationsize),
            ::std::mem::transmute(returnsize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ReclaimVirtualMemory(virtualaddress: *const ::std::ffi::c_void, size: usize) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReclaimVirtualMemory(virtualaddress: *const ::std::ffi::c_void, size: usize) -> u32;
        }
        ::std::mem::transmute(ReclaimVirtualMemory(
            ::std::mem::transmute(virtualaddress),
            ::std::mem::transmute(size),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RegisterBadMemoryNotification(
    callback: ::std::option::Option<PBAD_MEMORY_CALLBACK_ROUTINE>,
) -> *mut ::std::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegisterBadMemoryNotification(
                callback: ::windows::runtime::RawPtr,
            ) -> *mut ::std::ffi::c_void;
        }
        ::std::mem::transmute(RegisterBadMemoryNotification(::std::mem::transmute(
            callback,
        )))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RemoveSecureMemoryCacheCallback(
    pfncallback: ::std::option::Option<PSECURE_MEMORY_CACHE_CALLBACK>,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RemoveSecureMemoryCacheCallback(
                pfncallback: ::windows::runtime::RawPtr,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(RemoveSecureMemoryCacheCallback(::std::mem::transmute(
            pfncallback,
        )))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ResetWriteWatch(
    lpbaseaddress: *const ::std::ffi::c_void,
    dwregionsize: usize,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ResetWriteWatch(
                lpbaseaddress: *const ::std::ffi::c_void,
                dwregionsize: usize,
            ) -> u32;
        }
        ::std::mem::transmute(ResetWriteWatch(
            ::std::mem::transmute(lpbaseaddress),
            ::std::mem::transmute(dwregionsize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
#[inline]
pub unsafe fn SetProcessValidCallTargets<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hprocess: Param0,
    virtualaddress: *const ::std::ffi::c_void,
    regionsize: usize,
    numberofoffsets: u32,
    offsetinformation: *mut super::SystemServices::CFG_CALL_TARGET_INFO,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetProcessValidCallTargets(
                hprocess: super::super::Foundation::HANDLE,
                virtualaddress: *const ::std::ffi::c_void,
                regionsize: usize,
                numberofoffsets: u32,
                offsetinformation: *mut super::SystemServices::CFG_CALL_TARGET_INFO,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetProcessValidCallTargets(
            hprocess.into_param().abi(),
            ::std::mem::transmute(virtualaddress),
            ::std::mem::transmute(regionsize),
            ::std::mem::transmute(numberofoffsets),
            ::std::mem::transmute(offsetinformation),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
#[inline]
pub unsafe fn SetProcessValidCallTargetsForMappedView<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    process: Param0,
    virtualaddress: *const ::std::ffi::c_void,
    regionsize: usize,
    numberofoffsets: u32,
    offsetinformation: *mut super::SystemServices::CFG_CALL_TARGET_INFO,
    section: Param5,
    expectedfileoffset: u64,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetProcessValidCallTargetsForMappedView(
                process: super::super::Foundation::HANDLE,
                virtualaddress: *const ::std::ffi::c_void,
                regionsize: usize,
                numberofoffsets: u32,
                offsetinformation: *mut super::SystemServices::CFG_CALL_TARGET_INFO,
                section: super::super::Foundation::HANDLE,
                expectedfileoffset: u64,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetProcessValidCallTargetsForMappedView(
            process.into_param().abi(),
            ::std::mem::transmute(virtualaddress),
            ::std::mem::transmute(regionsize),
            ::std::mem::transmute(numberofoffsets),
            ::std::mem::transmute(offsetinformation),
            section.into_param().abi(),
            ::std::mem::transmute(expectedfileoffset),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetProcessWorkingSetSizeEx<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hprocess: Param0,
    dwminimumworkingsetsize: usize,
    dwmaximumworkingsetsize: usize,
    flags: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetProcessWorkingSetSizeEx(
                hprocess: super::super::Foundation::HANDLE,
                dwminimumworkingsetsize: usize,
                dwmaximumworkingsetsize: usize,
                flags: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetProcessWorkingSetSizeEx(
            hprocess.into_param().abi(),
            ::std::mem::transmute(dwminimumworkingsetsize),
            ::std::mem::transmute(dwmaximumworkingsetsize),
            ::std::mem::transmute(flags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetSystemFileCacheSize(
    minimumfilecachesize: usize,
    maximumfilecachesize: usize,
    flags: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetSystemFileCacheSize(
                minimumfilecachesize: usize,
                maximumfilecachesize: usize,
                flags: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetSystemFileCacheSize(
            ::std::mem::transmute(minimumfilecachesize),
            ::std::mem::transmute(maximumfilecachesize),
            ::std::mem::transmute(flags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct UNMAP_VIEW_OF_FILE_FLAGS(pub u32);
pub const MEM_UNMAP_NONE: UNMAP_VIEW_OF_FILE_FLAGS = UNMAP_VIEW_OF_FILE_FLAGS(0u32);
pub const MEM_UNMAP_WITH_TRANSIENT_BOOST: UNMAP_VIEW_OF_FILE_FLAGS = UNMAP_VIEW_OF_FILE_FLAGS(1u32);
pub const MEM_PRESERVE_PLACEHOLDER: UNMAP_VIEW_OF_FILE_FLAGS = UNMAP_VIEW_OF_FILE_FLAGS(2u32);
impl ::std::convert::From<u32> for UNMAP_VIEW_OF_FILE_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for UNMAP_VIEW_OF_FILE_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for UNMAP_VIEW_OF_FILE_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for UNMAP_VIEW_OF_FILE_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for UNMAP_VIEW_OF_FILE_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for UNMAP_VIEW_OF_FILE_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for UNMAP_VIEW_OF_FILE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UnmapViewOfFile(
    lpbaseaddress: *const ::std::ffi::c_void,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UnmapViewOfFile(
                lpbaseaddress: *const ::std::ffi::c_void,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(UnmapViewOfFile(::std::mem::transmute(lpbaseaddress)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UnmapViewOfFile2<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    process: Param0,
    baseaddress: *const ::std::ffi::c_void,
    unmapflags: UNMAP_VIEW_OF_FILE_FLAGS,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UnmapViewOfFile2(
                process: super::super::Foundation::HANDLE,
                baseaddress: *const ::std::ffi::c_void,
                unmapflags: UNMAP_VIEW_OF_FILE_FLAGS,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(UnmapViewOfFile2(
            process.into_param().abi(),
            ::std::mem::transmute(baseaddress),
            ::std::mem::transmute(unmapflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UnmapViewOfFileEx(
    baseaddress: *const ::std::ffi::c_void,
    unmapflags: UNMAP_VIEW_OF_FILE_FLAGS,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UnmapViewOfFileEx(
                baseaddress: *const ::std::ffi::c_void,
                unmapflags: UNMAP_VIEW_OF_FILE_FLAGS,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(UnmapViewOfFileEx(
            ::std::mem::transmute(baseaddress),
            ::std::mem::transmute(unmapflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UnregisterBadMemoryNotification(
    registrationhandle: *const ::std::ffi::c_void,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UnregisterBadMemoryNotification(
                registrationhandle: *const ::std::ffi::c_void,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(UnregisterBadMemoryNotification(::std::mem::transmute(
            registrationhandle,
        )))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct VIRTUAL_ALLOCATION_TYPE(pub u32);
pub const MEM_COMMIT: VIRTUAL_ALLOCATION_TYPE = VIRTUAL_ALLOCATION_TYPE(4096u32);
pub const MEM_RESERVE: VIRTUAL_ALLOCATION_TYPE = VIRTUAL_ALLOCATION_TYPE(8192u32);
pub const MEM_RESET: VIRTUAL_ALLOCATION_TYPE = VIRTUAL_ALLOCATION_TYPE(524288u32);
pub const MEM_RESET_UNDO: VIRTUAL_ALLOCATION_TYPE = VIRTUAL_ALLOCATION_TYPE(16777216u32);
pub const MEM_REPLACE_PLACEHOLDER: VIRTUAL_ALLOCATION_TYPE = VIRTUAL_ALLOCATION_TYPE(16384u32);
pub const MEM_LARGE_PAGES: VIRTUAL_ALLOCATION_TYPE = VIRTUAL_ALLOCATION_TYPE(536870912u32);
pub const MEM_RESERVE_PLACEHOLDER: VIRTUAL_ALLOCATION_TYPE = VIRTUAL_ALLOCATION_TYPE(262144u32);
pub const MEM_FREE: VIRTUAL_ALLOCATION_TYPE = VIRTUAL_ALLOCATION_TYPE(65536u32);
impl ::std::convert::From<u32> for VIRTUAL_ALLOCATION_TYPE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VIRTUAL_ALLOCATION_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for VIRTUAL_ALLOCATION_TYPE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for VIRTUAL_ALLOCATION_TYPE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for VIRTUAL_ALLOCATION_TYPE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for VIRTUAL_ALLOCATION_TYPE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for VIRTUAL_ALLOCATION_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct VIRTUAL_FREE_TYPE(pub u32);
pub const MEM_DECOMMIT: VIRTUAL_FREE_TYPE = VIRTUAL_FREE_TYPE(16384u32);
pub const MEM_RELEASE: VIRTUAL_FREE_TYPE = VIRTUAL_FREE_TYPE(32768u32);
impl ::std::convert::From<u32> for VIRTUAL_FREE_TYPE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VIRTUAL_FREE_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for VIRTUAL_FREE_TYPE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for VIRTUAL_FREE_TYPE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for VIRTUAL_FREE_TYPE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for VIRTUAL_FREE_TYPE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for VIRTUAL_FREE_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[inline]
pub unsafe fn VirtualAlloc(
    lpaddress: *const ::std::ffi::c_void,
    dwsize: usize,
    flallocationtype: VIRTUAL_ALLOCATION_TYPE,
    flprotect: PAGE_PROTECTION_FLAGS,
) -> *mut ::std::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VirtualAlloc(
                lpaddress: *const ::std::ffi::c_void,
                dwsize: usize,
                flallocationtype: VIRTUAL_ALLOCATION_TYPE,
                flprotect: PAGE_PROTECTION_FLAGS,
            ) -> *mut ::std::ffi::c_void;
        }
        ::std::mem::transmute(VirtualAlloc(
            ::std::mem::transmute(lpaddress),
            ::std::mem::transmute(dwsize),
            ::std::mem::transmute(flallocationtype),
            ::std::mem::transmute(flprotect),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
#[inline]
pub unsafe fn VirtualAlloc2<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    process: Param0,
    baseaddress: *const ::std::ffi::c_void,
    size: usize,
    allocationtype: VIRTUAL_ALLOCATION_TYPE,
    pageprotection: u32,
    extendedparameters: *mut super::SystemServices::MEM_EXTENDED_PARAMETER,
    parametercount: u32,
) -> *mut ::std::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VirtualAlloc2(
                process: super::super::Foundation::HANDLE,
                baseaddress: *const ::std::ffi::c_void,
                size: usize,
                allocationtype: VIRTUAL_ALLOCATION_TYPE,
                pageprotection: u32,
                extendedparameters: *mut super::SystemServices::MEM_EXTENDED_PARAMETER,
                parametercount: u32,
            ) -> *mut ::std::ffi::c_void;
        }
        ::std::mem::transmute(VirtualAlloc2(
            process.into_param().abi(),
            ::std::mem::transmute(baseaddress),
            ::std::mem::transmute(size),
            ::std::mem::transmute(allocationtype),
            ::std::mem::transmute(pageprotection),
            ::std::mem::transmute(extendedparameters),
            ::std::mem::transmute(parametercount),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
#[inline]
pub unsafe fn VirtualAlloc2FromApp<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    process: Param0,
    baseaddress: *const ::std::ffi::c_void,
    size: usize,
    allocationtype: VIRTUAL_ALLOCATION_TYPE,
    pageprotection: u32,
    extendedparameters: *mut super::SystemServices::MEM_EXTENDED_PARAMETER,
    parametercount: u32,
) -> *mut ::std::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VirtualAlloc2FromApp(
                process: super::super::Foundation::HANDLE,
                baseaddress: *const ::std::ffi::c_void,
                size: usize,
                allocationtype: VIRTUAL_ALLOCATION_TYPE,
                pageprotection: u32,
                extendedparameters: *mut super::SystemServices::MEM_EXTENDED_PARAMETER,
                parametercount: u32,
            ) -> *mut ::std::ffi::c_void;
        }
        ::std::mem::transmute(VirtualAlloc2FromApp(
            process.into_param().abi(),
            ::std::mem::transmute(baseaddress),
            ::std::mem::transmute(size),
            ::std::mem::transmute(allocationtype),
            ::std::mem::transmute(pageprotection),
            ::std::mem::transmute(extendedparameters),
            ::std::mem::transmute(parametercount),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VirtualAllocEx<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hprocess: Param0,
    lpaddress: *const ::std::ffi::c_void,
    dwsize: usize,
    flallocationtype: VIRTUAL_ALLOCATION_TYPE,
    flprotect: PAGE_PROTECTION_FLAGS,
) -> *mut ::std::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VirtualAllocEx(
                hprocess: super::super::Foundation::HANDLE,
                lpaddress: *const ::std::ffi::c_void,
                dwsize: usize,
                flallocationtype: VIRTUAL_ALLOCATION_TYPE,
                flprotect: PAGE_PROTECTION_FLAGS,
            ) -> *mut ::std::ffi::c_void;
        }
        ::std::mem::transmute(VirtualAllocEx(
            hprocess.into_param().abi(),
            ::std::mem::transmute(lpaddress),
            ::std::mem::transmute(dwsize),
            ::std::mem::transmute(flallocationtype),
            ::std::mem::transmute(flprotect),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VirtualAllocExNuma<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hprocess: Param0,
    lpaddress: *const ::std::ffi::c_void,
    dwsize: usize,
    flallocationtype: VIRTUAL_ALLOCATION_TYPE,
    flprotect: u32,
    nndpreferred: u32,
) -> *mut ::std::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VirtualAllocExNuma(
                hprocess: super::super::Foundation::HANDLE,
                lpaddress: *const ::std::ffi::c_void,
                dwsize: usize,
                flallocationtype: VIRTUAL_ALLOCATION_TYPE,
                flprotect: u32,
                nndpreferred: u32,
            ) -> *mut ::std::ffi::c_void;
        }
        ::std::mem::transmute(VirtualAllocExNuma(
            hprocess.into_param().abi(),
            ::std::mem::transmute(lpaddress),
            ::std::mem::transmute(dwsize),
            ::std::mem::transmute(flallocationtype),
            ::std::mem::transmute(flprotect),
            ::std::mem::transmute(nndpreferred),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn VirtualAllocFromApp(
    baseaddress: *const ::std::ffi::c_void,
    size: usize,
    allocationtype: VIRTUAL_ALLOCATION_TYPE,
    protection: u32,
) -> *mut ::std::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VirtualAllocFromApp(
                baseaddress: *const ::std::ffi::c_void,
                size: usize,
                allocationtype: VIRTUAL_ALLOCATION_TYPE,
                protection: u32,
            ) -> *mut ::std::ffi::c_void;
        }
        ::std::mem::transmute(VirtualAllocFromApp(
            ::std::mem::transmute(baseaddress),
            ::std::mem::transmute(size),
            ::std::mem::transmute(allocationtype),
            ::std::mem::transmute(protection),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VirtualFree(
    lpaddress: *mut ::std::ffi::c_void,
    dwsize: usize,
    dwfreetype: VIRTUAL_FREE_TYPE,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VirtualFree(
                lpaddress: *mut ::std::ffi::c_void,
                dwsize: usize,
                dwfreetype: VIRTUAL_FREE_TYPE,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(VirtualFree(
            ::std::mem::transmute(lpaddress),
            ::std::mem::transmute(dwsize),
            ::std::mem::transmute(dwfreetype),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VirtualFreeEx<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hprocess: Param0,
    lpaddress: *mut ::std::ffi::c_void,
    dwsize: usize,
    dwfreetype: VIRTUAL_FREE_TYPE,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VirtualFreeEx(
                hprocess: super::super::Foundation::HANDLE,
                lpaddress: *mut ::std::ffi::c_void,
                dwsize: usize,
                dwfreetype: VIRTUAL_FREE_TYPE,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(VirtualFreeEx(
            hprocess.into_param().abi(),
            ::std::mem::transmute(lpaddress),
            ::std::mem::transmute(dwsize),
            ::std::mem::transmute(dwfreetype),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VirtualLock(
    lpaddress: *const ::std::ffi::c_void,
    dwsize: usize,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VirtualLock(
                lpaddress: *const ::std::ffi::c_void,
                dwsize: usize,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(VirtualLock(
            ::std::mem::transmute(lpaddress),
            ::std::mem::transmute(dwsize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VirtualProtect(
    lpaddress: *const ::std::ffi::c_void,
    dwsize: usize,
    flnewprotect: PAGE_PROTECTION_FLAGS,
    lpfloldprotect: *mut PAGE_PROTECTION_FLAGS,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VirtualProtect(
                lpaddress: *const ::std::ffi::c_void,
                dwsize: usize,
                flnewprotect: PAGE_PROTECTION_FLAGS,
                lpfloldprotect: *mut PAGE_PROTECTION_FLAGS,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(VirtualProtect(
            ::std::mem::transmute(lpaddress),
            ::std::mem::transmute(dwsize),
            ::std::mem::transmute(flnewprotect),
            ::std::mem::transmute(lpfloldprotect),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VirtualProtectEx<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hprocess: Param0,
    lpaddress: *const ::std::ffi::c_void,
    dwsize: usize,
    flnewprotect: PAGE_PROTECTION_FLAGS,
    lpfloldprotect: *mut PAGE_PROTECTION_FLAGS,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VirtualProtectEx(
                hprocess: super::super::Foundation::HANDLE,
                lpaddress: *const ::std::ffi::c_void,
                dwsize: usize,
                flnewprotect: PAGE_PROTECTION_FLAGS,
                lpfloldprotect: *mut PAGE_PROTECTION_FLAGS,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(VirtualProtectEx(
            hprocess.into_param().abi(),
            ::std::mem::transmute(lpaddress),
            ::std::mem::transmute(dwsize),
            ::std::mem::transmute(flnewprotect),
            ::std::mem::transmute(lpfloldprotect),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VirtualProtectFromApp(
    address: *const ::std::ffi::c_void,
    size: usize,
    newprotection: u32,
    oldprotection: *mut u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VirtualProtectFromApp(
                address: *const ::std::ffi::c_void,
                size: usize,
                newprotection: u32,
                oldprotection: *mut u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(VirtualProtectFromApp(
            ::std::mem::transmute(address),
            ::std::mem::transmute(size),
            ::std::mem::transmute(newprotection),
            ::std::mem::transmute(oldprotection),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn VirtualQuery(
    lpaddress: *const ::std::ffi::c_void,
    lpbuffer: *mut MEMORY_BASIC_INFORMATION,
    dwlength: usize,
) -> usize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VirtualQuery(
                lpaddress: *const ::std::ffi::c_void,
                lpbuffer: *mut MEMORY_BASIC_INFORMATION,
                dwlength: usize,
            ) -> usize;
        }
        ::std::mem::transmute(VirtualQuery(
            ::std::mem::transmute(lpaddress),
            ::std::mem::transmute(lpbuffer),
            ::std::mem::transmute(dwlength),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VirtualQueryEx<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hprocess: Param0,
    lpaddress: *const ::std::ffi::c_void,
    lpbuffer: *mut MEMORY_BASIC_INFORMATION,
    dwlength: usize,
) -> usize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VirtualQueryEx(
                hprocess: super::super::Foundation::HANDLE,
                lpaddress: *const ::std::ffi::c_void,
                lpbuffer: *mut MEMORY_BASIC_INFORMATION,
                dwlength: usize,
            ) -> usize;
        }
        ::std::mem::transmute(VirtualQueryEx(
            hprocess.into_param().abi(),
            ::std::mem::transmute(lpaddress),
            ::std::mem::transmute(lpbuffer),
            ::std::mem::transmute(dwlength),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VirtualUnlock(
    lpaddress: *const ::std::ffi::c_void,
    dwsize: usize,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VirtualUnlock(
                lpaddress: *const ::std::ffi::c_void,
                dwsize: usize,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(VirtualUnlock(
            ::std::mem::transmute(lpaddress),
            ::std::mem::transmute(dwsize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VirtualUnlockEx<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    process: Param0,
    address: *const ::std::ffi::c_void,
    size: usize,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VirtualUnlockEx(
                process: super::super::Foundation::HANDLE,
                address: *const ::std::ffi::c_void,
                size: usize,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(VirtualUnlockEx(
            process.into_param().abi(),
            ::std::mem::transmute(address),
            ::std::mem::transmute(size),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct WIN32_MEMORY_INFORMATION_CLASS(pub i32);
pub const MemoryRegionInfo: WIN32_MEMORY_INFORMATION_CLASS = WIN32_MEMORY_INFORMATION_CLASS(0i32);
impl ::std::convert::From<i32> for WIN32_MEMORY_INFORMATION_CLASS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WIN32_MEMORY_INFORMATION_CLASS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl WIN32_MEMORY_PARTITION_INFORMATION {}
impl ::std::default::Default for WIN32_MEMORY_PARTITION_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WIN32_MEMORY_PARTITION_INFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WIN32_MEMORY_PARTITION_INFORMATION")
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
impl ::std::cmp::PartialEq for WIN32_MEMORY_PARTITION_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags
            && self.NumaNode == other.NumaNode
            && self.Channel == other.Channel
            && self.NumberOfNumaNodes == other.NumberOfNumaNodes
            && self.ResidentAvailablePages == other.ResidentAvailablePages
            && self.CommittedPages == other.CommittedPages
            && self.CommitLimit == other.CommitLimit
            && self.PeakCommitment == other.PeakCommitment
            && self.TotalNumberOfPages == other.TotalNumberOfPages
            && self.AvailablePages == other.AvailablePages
            && self.ZeroPages == other.ZeroPages
            && self.FreePages == other.FreePages
            && self.StandbyPages == other.StandbyPages
            && self.Reserved == other.Reserved
            && self.MaximumCommitLimit == other.MaximumCommitLimit
            && self.Reserved2 == other.Reserved2
            && self.PartitionId == other.PartitionId
    }
}
impl ::std::cmp::Eq for WIN32_MEMORY_PARTITION_INFORMATION {}
unsafe impl ::windows::runtime::Abi for WIN32_MEMORY_PARTITION_INFORMATION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct WIN32_MEMORY_PARTITION_INFORMATION_CLASS(pub i32);
pub const MemoryPartitionInfo: WIN32_MEMORY_PARTITION_INFORMATION_CLASS =
    WIN32_MEMORY_PARTITION_INFORMATION_CLASS(0i32);
pub const MemoryPartitionDedicatedMemoryInfo: WIN32_MEMORY_PARTITION_INFORMATION_CLASS =
    WIN32_MEMORY_PARTITION_INFORMATION_CLASS(1i32);
impl ::std::convert::From<i32> for WIN32_MEMORY_PARTITION_INFORMATION_CLASS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WIN32_MEMORY_PARTITION_INFORMATION_CLASS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WIN32_MEMORY_RANGE_ENTRY {
    pub VirtualAddress: *mut ::std::ffi::c_void,
    pub NumberOfBytes: usize,
}
impl WIN32_MEMORY_RANGE_ENTRY {}
impl ::std::default::Default for WIN32_MEMORY_RANGE_ENTRY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WIN32_MEMORY_RANGE_ENTRY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WIN32_MEMORY_RANGE_ENTRY")
            .field("VirtualAddress", &self.VirtualAddress)
            .field("NumberOfBytes", &self.NumberOfBytes)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WIN32_MEMORY_RANGE_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.VirtualAddress == other.VirtualAddress && self.NumberOfBytes == other.NumberOfBytes
    }
}
impl ::std::cmp::Eq for WIN32_MEMORY_RANGE_ENTRY {}
unsafe impl ::windows::runtime::Abi for WIN32_MEMORY_RANGE_ENTRY {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WIN32_MEMORY_REGION_INFORMATION {
    pub AllocationBase: *mut ::std::ffi::c_void,
    pub AllocationProtect: u32,
    pub Anonymous: WIN32_MEMORY_REGION_INFORMATION_0,
    pub RegionSize: usize,
    pub CommitSize: usize,
}
impl WIN32_MEMORY_REGION_INFORMATION {}
impl ::std::default::Default for WIN32_MEMORY_REGION_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WIN32_MEMORY_REGION_INFORMATION {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WIN32_MEMORY_REGION_INFORMATION {}
unsafe impl ::windows::runtime::Abi for WIN32_MEMORY_REGION_INFORMATION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union WIN32_MEMORY_REGION_INFORMATION_0 {
    pub Flags: u32,
    pub Anonymous: WIN32_MEMORY_REGION_INFORMATION_0_0,
}
impl WIN32_MEMORY_REGION_INFORMATION_0 {}
impl ::std::default::Default for WIN32_MEMORY_REGION_INFORMATION_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WIN32_MEMORY_REGION_INFORMATION_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WIN32_MEMORY_REGION_INFORMATION_0 {}
unsafe impl ::windows::runtime::Abi for WIN32_MEMORY_REGION_INFORMATION_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WIN32_MEMORY_REGION_INFORMATION_0_0 {
    pub _bitfield: u32,
}
impl WIN32_MEMORY_REGION_INFORMATION_0_0 {}
impl ::std::default::Default for WIN32_MEMORY_REGION_INFORMATION_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WIN32_MEMORY_REGION_INFORMATION_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct")
            .field("_bitfield", &self._bitfield)
            .finish()
    }
}
impl ::std::cmp::PartialEq for WIN32_MEMORY_REGION_INFORMATION_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for WIN32_MEMORY_REGION_INFORMATION_0_0 {}
unsafe impl ::windows::runtime::Abi for WIN32_MEMORY_REGION_INFORMATION_0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
