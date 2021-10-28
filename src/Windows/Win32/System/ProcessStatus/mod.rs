#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct ENUM_PAGE_FILE_INFORMATION {
    pub cb: u32,
    pub Reserved: u32,
    pub TotalSize: usize,
    pub TotalInUse: usize,
    pub PeakUsage: usize,
}
impl ENUM_PAGE_FILE_INFORMATION {}
impl ::std::default::Default for ENUM_PAGE_FILE_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for ENUM_PAGE_FILE_INFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ENUM_PAGE_FILE_INFORMATION")
            .field("cb", &self.cb)
            .field("Reserved", &self.Reserved)
            .field("TotalSize", &self.TotalSize)
            .field("TotalInUse", &self.TotalInUse)
            .field("PeakUsage", &self.PeakUsage)
            .finish()
    }
}
impl ::std::cmp::PartialEq for ENUM_PAGE_FILE_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.cb == other.cb
            && self.Reserved == other.Reserved
            && self.TotalSize == other.TotalSize
            && self.TotalInUse == other.TotalInUse
            && self.PeakUsage == other.PeakUsage
    }
}
impl ::std::cmp::Eq for ENUM_PAGE_FILE_INFORMATION {}
unsafe impl ::windows::runtime::Abi for ENUM_PAGE_FILE_INFORMATION {
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
pub struct ENUM_PROCESS_MODULES_EX_FLAGS(pub u32);
pub const LIST_MODULES_ALL: ENUM_PROCESS_MODULES_EX_FLAGS = ENUM_PROCESS_MODULES_EX_FLAGS(3u32);
pub const LIST_MODULES_DEFAULT: ENUM_PROCESS_MODULES_EX_FLAGS = ENUM_PROCESS_MODULES_EX_FLAGS(0u32);
pub const LIST_MODULES_32BIT: ENUM_PROCESS_MODULES_EX_FLAGS = ENUM_PROCESS_MODULES_EX_FLAGS(1u32);
pub const LIST_MODULES_64BIT: ENUM_PROCESS_MODULES_EX_FLAGS = ENUM_PROCESS_MODULES_EX_FLAGS(2u32);
impl ::std::convert::From<u32> for ENUM_PROCESS_MODULES_EX_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ENUM_PROCESS_MODULES_EX_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for ENUM_PROCESS_MODULES_EX_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for ENUM_PROCESS_MODULES_EX_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for ENUM_PROCESS_MODULES_EX_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for ENUM_PROCESS_MODULES_EX_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for ENUM_PROCESS_MODULES_EX_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn K32EmptyWorkingSet<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hprocess: Param0,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn K32EmptyWorkingSet(
                hprocess: super::super::Foundation::HANDLE,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(K32EmptyWorkingSet(hprocess.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn K32EnumDeviceDrivers(
    lpimagebase: *mut *mut ::std::ffi::c_void,
    cb: u32,
    lpcbneeded: *mut u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn K32EnumDeviceDrivers(
                lpimagebase: *mut *mut ::std::ffi::c_void,
                cb: u32,
                lpcbneeded: *mut u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(K32EnumDeviceDrivers(
            ::std::mem::transmute(lpimagebase),
            ::std::mem::transmute(cb),
            ::std::mem::transmute(lpcbneeded),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn K32EnumPageFilesA(
    pcallbackroutine: ::std::option::Option<PENUM_PAGE_FILE_CALLBACKA>,
    pcontext: *mut ::std::ffi::c_void,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn K32EnumPageFilesA(
                pcallbackroutine: ::windows::runtime::RawPtr,
                pcontext: *mut ::std::ffi::c_void,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(K32EnumPageFilesA(
            ::std::mem::transmute(pcallbackroutine),
            ::std::mem::transmute(pcontext),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn K32EnumPageFilesW(
    pcallbackroutine: ::std::option::Option<PENUM_PAGE_FILE_CALLBACKW>,
    pcontext: *mut ::std::ffi::c_void,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn K32EnumPageFilesW(
                pcallbackroutine: ::windows::runtime::RawPtr,
                pcontext: *mut ::std::ffi::c_void,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(K32EnumPageFilesW(
            ::std::mem::transmute(pcallbackroutine),
            ::std::mem::transmute(pcontext),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn K32EnumProcessModules<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hprocess: Param0,
    lphmodule: *mut super::super::Foundation::HINSTANCE,
    cb: u32,
    lpcbneeded: *mut u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn K32EnumProcessModules(
                hprocess: super::super::Foundation::HANDLE,
                lphmodule: *mut super::super::Foundation::HINSTANCE,
                cb: u32,
                lpcbneeded: *mut u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(K32EnumProcessModules(
            hprocess.into_param().abi(),
            ::std::mem::transmute(lphmodule),
            ::std::mem::transmute(cb),
            ::std::mem::transmute(lpcbneeded),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn K32EnumProcessModulesEx<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hprocess: Param0,
    lphmodule: *mut super::super::Foundation::HINSTANCE,
    cb: u32,
    lpcbneeded: *mut u32,
    dwfilterflag: ENUM_PROCESS_MODULES_EX_FLAGS,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn K32EnumProcessModulesEx(
                hprocess: super::super::Foundation::HANDLE,
                lphmodule: *mut super::super::Foundation::HINSTANCE,
                cb: u32,
                lpcbneeded: *mut u32,
                dwfilterflag: ENUM_PROCESS_MODULES_EX_FLAGS,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(K32EnumProcessModulesEx(
            hprocess.into_param().abi(),
            ::std::mem::transmute(lphmodule),
            ::std::mem::transmute(cb),
            ::std::mem::transmute(lpcbneeded),
            ::std::mem::transmute(dwfilterflag),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn K32EnumProcesses(
    lpidprocess: *mut u32,
    cb: u32,
    lpcbneeded: *mut u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn K32EnumProcesses(
                lpidprocess: *mut u32,
                cb: u32,
                lpcbneeded: *mut u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(K32EnumProcesses(
            ::std::mem::transmute(lpidprocess),
            ::std::mem::transmute(cb),
            ::std::mem::transmute(lpcbneeded),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn K32GetDeviceDriverBaseNameA(
    imagebase: *const ::std::ffi::c_void,
    lpfilename: super::super::Foundation::PSTR,
    nsize: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn K32GetDeviceDriverBaseNameA(
                imagebase: *const ::std::ffi::c_void,
                lpfilename: super::super::Foundation::PSTR,
                nsize: u32,
            ) -> u32;
        }
        ::std::mem::transmute(K32GetDeviceDriverBaseNameA(
            ::std::mem::transmute(imagebase),
            ::std::mem::transmute(lpfilename),
            ::std::mem::transmute(nsize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn K32GetDeviceDriverBaseNameW(
    imagebase: *const ::std::ffi::c_void,
    lpbasename: super::super::Foundation::PWSTR,
    nsize: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn K32GetDeviceDriverBaseNameW(
                imagebase: *const ::std::ffi::c_void,
                lpbasename: super::super::Foundation::PWSTR,
                nsize: u32,
            ) -> u32;
        }
        ::std::mem::transmute(K32GetDeviceDriverBaseNameW(
            ::std::mem::transmute(imagebase),
            ::std::mem::transmute(lpbasename),
            ::std::mem::transmute(nsize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn K32GetDeviceDriverFileNameA(
    imagebase: *const ::std::ffi::c_void,
    lpfilename: super::super::Foundation::PSTR,
    nsize: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn K32GetDeviceDriverFileNameA(
                imagebase: *const ::std::ffi::c_void,
                lpfilename: super::super::Foundation::PSTR,
                nsize: u32,
            ) -> u32;
        }
        ::std::mem::transmute(K32GetDeviceDriverFileNameA(
            ::std::mem::transmute(imagebase),
            ::std::mem::transmute(lpfilename),
            ::std::mem::transmute(nsize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn K32GetDeviceDriverFileNameW(
    imagebase: *const ::std::ffi::c_void,
    lpfilename: super::super::Foundation::PWSTR,
    nsize: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn K32GetDeviceDriverFileNameW(
                imagebase: *const ::std::ffi::c_void,
                lpfilename: super::super::Foundation::PWSTR,
                nsize: u32,
            ) -> u32;
        }
        ::std::mem::transmute(K32GetDeviceDriverFileNameW(
            ::std::mem::transmute(imagebase),
            ::std::mem::transmute(lpfilename),
            ::std::mem::transmute(nsize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn K32GetMappedFileNameA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hprocess: Param0,
    lpv: *const ::std::ffi::c_void,
    lpfilename: super::super::Foundation::PSTR,
    nsize: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn K32GetMappedFileNameA(
                hprocess: super::super::Foundation::HANDLE,
                lpv: *const ::std::ffi::c_void,
                lpfilename: super::super::Foundation::PSTR,
                nsize: u32,
            ) -> u32;
        }
        ::std::mem::transmute(K32GetMappedFileNameA(
            hprocess.into_param().abi(),
            ::std::mem::transmute(lpv),
            ::std::mem::transmute(lpfilename),
            ::std::mem::transmute(nsize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn K32GetMappedFileNameW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hprocess: Param0,
    lpv: *const ::std::ffi::c_void,
    lpfilename: super::super::Foundation::PWSTR,
    nsize: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn K32GetMappedFileNameW(
                hprocess: super::super::Foundation::HANDLE,
                lpv: *const ::std::ffi::c_void,
                lpfilename: super::super::Foundation::PWSTR,
                nsize: u32,
            ) -> u32;
        }
        ::std::mem::transmute(K32GetMappedFileNameW(
            hprocess.into_param().abi(),
            ::std::mem::transmute(lpv),
            ::std::mem::transmute(lpfilename),
            ::std::mem::transmute(nsize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn K32GetModuleBaseNameA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HINSTANCE>,
>(
    hprocess: Param0,
    hmodule: Param1,
    lpbasename: super::super::Foundation::PSTR,
    nsize: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn K32GetModuleBaseNameA(
                hprocess: super::super::Foundation::HANDLE,
                hmodule: super::super::Foundation::HINSTANCE,
                lpbasename: super::super::Foundation::PSTR,
                nsize: u32,
            ) -> u32;
        }
        ::std::mem::transmute(K32GetModuleBaseNameA(
            hprocess.into_param().abi(),
            hmodule.into_param().abi(),
            ::std::mem::transmute(lpbasename),
            ::std::mem::transmute(nsize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn K32GetModuleBaseNameW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HINSTANCE>,
>(
    hprocess: Param0,
    hmodule: Param1,
    lpbasename: super::super::Foundation::PWSTR,
    nsize: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn K32GetModuleBaseNameW(
                hprocess: super::super::Foundation::HANDLE,
                hmodule: super::super::Foundation::HINSTANCE,
                lpbasename: super::super::Foundation::PWSTR,
                nsize: u32,
            ) -> u32;
        }
        ::std::mem::transmute(K32GetModuleBaseNameW(
            hprocess.into_param().abi(),
            hmodule.into_param().abi(),
            ::std::mem::transmute(lpbasename),
            ::std::mem::transmute(nsize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn K32GetModuleFileNameExA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HINSTANCE>,
>(
    hprocess: Param0,
    hmodule: Param1,
    lpfilename: super::super::Foundation::PSTR,
    nsize: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn K32GetModuleFileNameExA(
                hprocess: super::super::Foundation::HANDLE,
                hmodule: super::super::Foundation::HINSTANCE,
                lpfilename: super::super::Foundation::PSTR,
                nsize: u32,
            ) -> u32;
        }
        ::std::mem::transmute(K32GetModuleFileNameExA(
            hprocess.into_param().abi(),
            hmodule.into_param().abi(),
            ::std::mem::transmute(lpfilename),
            ::std::mem::transmute(nsize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn K32GetModuleFileNameExW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HINSTANCE>,
>(
    hprocess: Param0,
    hmodule: Param1,
    lpfilename: super::super::Foundation::PWSTR,
    nsize: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn K32GetModuleFileNameExW(
                hprocess: super::super::Foundation::HANDLE,
                hmodule: super::super::Foundation::HINSTANCE,
                lpfilename: super::super::Foundation::PWSTR,
                nsize: u32,
            ) -> u32;
        }
        ::std::mem::transmute(K32GetModuleFileNameExW(
            hprocess.into_param().abi(),
            hmodule.into_param().abi(),
            ::std::mem::transmute(lpfilename),
            ::std::mem::transmute(nsize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn K32GetModuleInformation<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HINSTANCE>,
>(
    hprocess: Param0,
    hmodule: Param1,
    lpmodinfo: *mut MODULEINFO,
    cb: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn K32GetModuleInformation(
                hprocess: super::super::Foundation::HANDLE,
                hmodule: super::super::Foundation::HINSTANCE,
                lpmodinfo: *mut MODULEINFO,
                cb: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(K32GetModuleInformation(
            hprocess.into_param().abi(),
            hmodule.into_param().abi(),
            ::std::mem::transmute(lpmodinfo),
            ::std::mem::transmute(cb),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn K32GetPerformanceInfo(
    pperformanceinformation: *mut PERFORMANCE_INFORMATION,
    cb: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn K32GetPerformanceInfo(
                pperformanceinformation: *mut PERFORMANCE_INFORMATION,
                cb: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(K32GetPerformanceInfo(
            ::std::mem::transmute(pperformanceinformation),
            ::std::mem::transmute(cb),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn K32GetProcessImageFileNameA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hprocess: Param0,
    lpimagefilename: super::super::Foundation::PSTR,
    nsize: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn K32GetProcessImageFileNameA(
                hprocess: super::super::Foundation::HANDLE,
                lpimagefilename: super::super::Foundation::PSTR,
                nsize: u32,
            ) -> u32;
        }
        ::std::mem::transmute(K32GetProcessImageFileNameA(
            hprocess.into_param().abi(),
            ::std::mem::transmute(lpimagefilename),
            ::std::mem::transmute(nsize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn K32GetProcessImageFileNameW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hprocess: Param0,
    lpimagefilename: super::super::Foundation::PWSTR,
    nsize: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn K32GetProcessImageFileNameW(
                hprocess: super::super::Foundation::HANDLE,
                lpimagefilename: super::super::Foundation::PWSTR,
                nsize: u32,
            ) -> u32;
        }
        ::std::mem::transmute(K32GetProcessImageFileNameW(
            hprocess.into_param().abi(),
            ::std::mem::transmute(lpimagefilename),
            ::std::mem::transmute(nsize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn K32GetProcessMemoryInfo<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    process: Param0,
    ppsmemcounters: *mut PROCESS_MEMORY_COUNTERS,
    cb: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn K32GetProcessMemoryInfo(
                process: super::super::Foundation::HANDLE,
                ppsmemcounters: *mut PROCESS_MEMORY_COUNTERS,
                cb: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(K32GetProcessMemoryInfo(
            process.into_param().abi(),
            ::std::mem::transmute(ppsmemcounters),
            ::std::mem::transmute(cb),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn K32GetWsChanges<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hprocess: Param0,
    lpwatchinfo: *mut PSAPI_WS_WATCH_INFORMATION,
    cb: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn K32GetWsChanges(
                hprocess: super::super::Foundation::HANDLE,
                lpwatchinfo: *mut PSAPI_WS_WATCH_INFORMATION,
                cb: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(K32GetWsChanges(
            hprocess.into_param().abi(),
            ::std::mem::transmute(lpwatchinfo),
            ::std::mem::transmute(cb),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn K32GetWsChangesEx<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hprocess: Param0,
    lpwatchinfoex: *mut PSAPI_WS_WATCH_INFORMATION_EX,
    cb: *mut u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn K32GetWsChangesEx(
                hprocess: super::super::Foundation::HANDLE,
                lpwatchinfoex: *mut PSAPI_WS_WATCH_INFORMATION_EX,
                cb: *mut u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(K32GetWsChangesEx(
            hprocess.into_param().abi(),
            ::std::mem::transmute(lpwatchinfoex),
            ::std::mem::transmute(cb),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn K32InitializeProcessForWsWatch<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hprocess: Param0,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn K32InitializeProcessForWsWatch(
                hprocess: super::super::Foundation::HANDLE,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(K32InitializeProcessForWsWatch(hprocess.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn K32QueryWorkingSet<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hprocess: Param0,
    pv: *mut ::std::ffi::c_void,
    cb: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn K32QueryWorkingSet(
                hprocess: super::super::Foundation::HANDLE,
                pv: *mut ::std::ffi::c_void,
                cb: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(K32QueryWorkingSet(
            hprocess.into_param().abi(),
            ::std::mem::transmute(pv),
            ::std::mem::transmute(cb),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn K32QueryWorkingSetEx<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hprocess: Param0,
    pv: *mut ::std::ffi::c_void,
    cb: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn K32QueryWorkingSetEx(
                hprocess: super::super::Foundation::HANDLE,
                pv: *mut ::std::ffi::c_void,
                cb: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(K32QueryWorkingSetEx(
            hprocess.into_param().abi(),
            ::std::mem::transmute(pv),
            ::std::mem::transmute(cb),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct MODULEINFO {
    pub lpBaseOfDll: *mut ::std::ffi::c_void,
    pub SizeOfImage: u32,
    pub EntryPoint: *mut ::std::ffi::c_void,
}
impl MODULEINFO {}
impl ::std::default::Default for MODULEINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for MODULEINFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MODULEINFO")
            .field("lpBaseOfDll", &self.lpBaseOfDll)
            .field("SizeOfImage", &self.SizeOfImage)
            .field("EntryPoint", &self.EntryPoint)
            .finish()
    }
}
impl ::std::cmp::PartialEq for MODULEINFO {
    fn eq(&self, other: &Self) -> bool {
        self.lpBaseOfDll == other.lpBaseOfDll
            && self.SizeOfImage == other.SizeOfImage
            && self.EntryPoint == other.EntryPoint
    }
}
impl ::std::cmp::Eq for MODULEINFO {}
unsafe impl ::windows::runtime::Abi for MODULEINFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
pub type PENUM_PAGE_FILE_CALLBACKA = unsafe extern "system" fn(
    pcontext: *mut ::std::ffi::c_void,
    ppagefileinfo: *mut ENUM_PAGE_FILE_INFORMATION,
    lpfilename: super::super::Foundation::PSTR,
) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PENUM_PAGE_FILE_CALLBACKW = unsafe extern "system" fn(
    pcontext: *mut ::std::ffi::c_void,
    ppagefileinfo: *mut ENUM_PAGE_FILE_INFORMATION,
    lpfilename: super::super::Foundation::PWSTR,
) -> super::super::Foundation::BOOL;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct PERFORMANCE_INFORMATION {
    pub cb: u32,
    pub CommitTotal: usize,
    pub CommitLimit: usize,
    pub CommitPeak: usize,
    pub PhysicalTotal: usize,
    pub PhysicalAvailable: usize,
    pub SystemCache: usize,
    pub KernelTotal: usize,
    pub KernelPaged: usize,
    pub KernelNonpaged: usize,
    pub PageSize: usize,
    pub HandleCount: u32,
    pub ProcessCount: u32,
    pub ThreadCount: u32,
}
impl PERFORMANCE_INFORMATION {}
impl ::std::default::Default for PERFORMANCE_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PERFORMANCE_INFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PERFORMANCE_INFORMATION")
            .field("cb", &self.cb)
            .field("CommitTotal", &self.CommitTotal)
            .field("CommitLimit", &self.CommitLimit)
            .field("CommitPeak", &self.CommitPeak)
            .field("PhysicalTotal", &self.PhysicalTotal)
            .field("PhysicalAvailable", &self.PhysicalAvailable)
            .field("SystemCache", &self.SystemCache)
            .field("KernelTotal", &self.KernelTotal)
            .field("KernelPaged", &self.KernelPaged)
            .field("KernelNonpaged", &self.KernelNonpaged)
            .field("PageSize", &self.PageSize)
            .field("HandleCount", &self.HandleCount)
            .field("ProcessCount", &self.ProcessCount)
            .field("ThreadCount", &self.ThreadCount)
            .finish()
    }
}
impl ::std::cmp::PartialEq for PERFORMANCE_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.cb == other.cb
            && self.CommitTotal == other.CommitTotal
            && self.CommitLimit == other.CommitLimit
            && self.CommitPeak == other.CommitPeak
            && self.PhysicalTotal == other.PhysicalTotal
            && self.PhysicalAvailable == other.PhysicalAvailable
            && self.SystemCache == other.SystemCache
            && self.KernelTotal == other.KernelTotal
            && self.KernelPaged == other.KernelPaged
            && self.KernelNonpaged == other.KernelNonpaged
            && self.PageSize == other.PageSize
            && self.HandleCount == other.HandleCount
            && self.ProcessCount == other.ProcessCount
            && self.ThreadCount == other.ThreadCount
    }
}
impl ::std::cmp::Eq for PERFORMANCE_INFORMATION {}
unsafe impl ::windows::runtime::Abi for PERFORMANCE_INFORMATION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct PROCESS_MEMORY_COUNTERS {
    pub cb: u32,
    pub PageFaultCount: u32,
    pub PeakWorkingSetSize: usize,
    pub WorkingSetSize: usize,
    pub QuotaPeakPagedPoolUsage: usize,
    pub QuotaPagedPoolUsage: usize,
    pub QuotaPeakNonPagedPoolUsage: usize,
    pub QuotaNonPagedPoolUsage: usize,
    pub PagefileUsage: usize,
    pub PeakPagefileUsage: usize,
}
impl PROCESS_MEMORY_COUNTERS {}
impl ::std::default::Default for PROCESS_MEMORY_COUNTERS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PROCESS_MEMORY_COUNTERS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PROCESS_MEMORY_COUNTERS")
            .field("cb", &self.cb)
            .field("PageFaultCount", &self.PageFaultCount)
            .field("PeakWorkingSetSize", &self.PeakWorkingSetSize)
            .field("WorkingSetSize", &self.WorkingSetSize)
            .field("QuotaPeakPagedPoolUsage", &self.QuotaPeakPagedPoolUsage)
            .field("QuotaPagedPoolUsage", &self.QuotaPagedPoolUsage)
            .field(
                "QuotaPeakNonPagedPoolUsage",
                &self.QuotaPeakNonPagedPoolUsage,
            )
            .field("QuotaNonPagedPoolUsage", &self.QuotaNonPagedPoolUsage)
            .field("PagefileUsage", &self.PagefileUsage)
            .field("PeakPagefileUsage", &self.PeakPagefileUsage)
            .finish()
    }
}
impl ::std::cmp::PartialEq for PROCESS_MEMORY_COUNTERS {
    fn eq(&self, other: &Self) -> bool {
        self.cb == other.cb
            && self.PageFaultCount == other.PageFaultCount
            && self.PeakWorkingSetSize == other.PeakWorkingSetSize
            && self.WorkingSetSize == other.WorkingSetSize
            && self.QuotaPeakPagedPoolUsage == other.QuotaPeakPagedPoolUsage
            && self.QuotaPagedPoolUsage == other.QuotaPagedPoolUsage
            && self.QuotaPeakNonPagedPoolUsage == other.QuotaPeakNonPagedPoolUsage
            && self.QuotaNonPagedPoolUsage == other.QuotaNonPagedPoolUsage
            && self.PagefileUsage == other.PagefileUsage
            && self.PeakPagefileUsage == other.PeakPagefileUsage
    }
}
impl ::std::cmp::Eq for PROCESS_MEMORY_COUNTERS {}
unsafe impl ::windows::runtime::Abi for PROCESS_MEMORY_COUNTERS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct PROCESS_MEMORY_COUNTERS_EX {
    pub cb: u32,
    pub PageFaultCount: u32,
    pub PeakWorkingSetSize: usize,
    pub WorkingSetSize: usize,
    pub QuotaPeakPagedPoolUsage: usize,
    pub QuotaPagedPoolUsage: usize,
    pub QuotaPeakNonPagedPoolUsage: usize,
    pub QuotaNonPagedPoolUsage: usize,
    pub PagefileUsage: usize,
    pub PeakPagefileUsage: usize,
    pub PrivateUsage: usize,
}
impl PROCESS_MEMORY_COUNTERS_EX {}
impl ::std::default::Default for PROCESS_MEMORY_COUNTERS_EX {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PROCESS_MEMORY_COUNTERS_EX {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PROCESS_MEMORY_COUNTERS_EX")
            .field("cb", &self.cb)
            .field("PageFaultCount", &self.PageFaultCount)
            .field("PeakWorkingSetSize", &self.PeakWorkingSetSize)
            .field("WorkingSetSize", &self.WorkingSetSize)
            .field("QuotaPeakPagedPoolUsage", &self.QuotaPeakPagedPoolUsage)
            .field("QuotaPagedPoolUsage", &self.QuotaPagedPoolUsage)
            .field(
                "QuotaPeakNonPagedPoolUsage",
                &self.QuotaPeakNonPagedPoolUsage,
            )
            .field("QuotaNonPagedPoolUsage", &self.QuotaNonPagedPoolUsage)
            .field("PagefileUsage", &self.PagefileUsage)
            .field("PeakPagefileUsage", &self.PeakPagefileUsage)
            .field("PrivateUsage", &self.PrivateUsage)
            .finish()
    }
}
impl ::std::cmp::PartialEq for PROCESS_MEMORY_COUNTERS_EX {
    fn eq(&self, other: &Self) -> bool {
        self.cb == other.cb
            && self.PageFaultCount == other.PageFaultCount
            && self.PeakWorkingSetSize == other.PeakWorkingSetSize
            && self.WorkingSetSize == other.WorkingSetSize
            && self.QuotaPeakPagedPoolUsage == other.QuotaPeakPagedPoolUsage
            && self.QuotaPagedPoolUsage == other.QuotaPagedPoolUsage
            && self.QuotaPeakNonPagedPoolUsage == other.QuotaPeakNonPagedPoolUsage
            && self.QuotaNonPagedPoolUsage == other.QuotaNonPagedPoolUsage
            && self.PagefileUsage == other.PagefileUsage
            && self.PeakPagefileUsage == other.PeakPagefileUsage
            && self.PrivateUsage == other.PrivateUsage
    }
}
impl ::std::cmp::Eq for PROCESS_MEMORY_COUNTERS_EX {}
unsafe impl ::windows::runtime::Abi for PROCESS_MEMORY_COUNTERS_EX {
    type Abi = Self;
    type DefaultType = Self;
}
pub const PSAPI_VERSION: u32 = 2u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union PSAPI_WORKING_SET_BLOCK {
    pub Flags: usize,
    pub Anonymous: PSAPI_WORKING_SET_BLOCK_0,
}
impl PSAPI_WORKING_SET_BLOCK {}
impl ::std::default::Default for PSAPI_WORKING_SET_BLOCK {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for PSAPI_WORKING_SET_BLOCK {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for PSAPI_WORKING_SET_BLOCK {}
unsafe impl ::windows::runtime::Abi for PSAPI_WORKING_SET_BLOCK {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct PSAPI_WORKING_SET_BLOCK_0 {
    pub _bitfield: usize,
}
impl PSAPI_WORKING_SET_BLOCK_0 {}
impl ::std::default::Default for PSAPI_WORKING_SET_BLOCK_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PSAPI_WORKING_SET_BLOCK_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct")
            .field("_bitfield", &self._bitfield)
            .finish()
    }
}
impl ::std::cmp::PartialEq for PSAPI_WORKING_SET_BLOCK_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for PSAPI_WORKING_SET_BLOCK_0 {}
unsafe impl ::windows::runtime::Abi for PSAPI_WORKING_SET_BLOCK_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union PSAPI_WORKING_SET_EX_BLOCK {
    pub Flags: usize,
    pub Anonymous: PSAPI_WORKING_SET_EX_BLOCK_0,
}
impl PSAPI_WORKING_SET_EX_BLOCK {}
impl ::std::default::Default for PSAPI_WORKING_SET_EX_BLOCK {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for PSAPI_WORKING_SET_EX_BLOCK {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for PSAPI_WORKING_SET_EX_BLOCK {}
unsafe impl ::windows::runtime::Abi for PSAPI_WORKING_SET_EX_BLOCK {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union PSAPI_WORKING_SET_EX_BLOCK_0 {
    pub Anonymous: PSAPI_WORKING_SET_EX_BLOCK_0_0,
    pub Invalid: PSAPI_WORKING_SET_EX_BLOCK_0_1,
}
impl PSAPI_WORKING_SET_EX_BLOCK_0 {}
impl ::std::default::Default for PSAPI_WORKING_SET_EX_BLOCK_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for PSAPI_WORKING_SET_EX_BLOCK_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for PSAPI_WORKING_SET_EX_BLOCK_0 {}
unsafe impl ::windows::runtime::Abi for PSAPI_WORKING_SET_EX_BLOCK_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct PSAPI_WORKING_SET_EX_BLOCK_0_0 {
    pub _bitfield: usize,
}
impl PSAPI_WORKING_SET_EX_BLOCK_0_0 {}
impl ::std::default::Default for PSAPI_WORKING_SET_EX_BLOCK_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PSAPI_WORKING_SET_EX_BLOCK_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct")
            .field("_bitfield", &self._bitfield)
            .finish()
    }
}
impl ::std::cmp::PartialEq for PSAPI_WORKING_SET_EX_BLOCK_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for PSAPI_WORKING_SET_EX_BLOCK_0_0 {}
unsafe impl ::windows::runtime::Abi for PSAPI_WORKING_SET_EX_BLOCK_0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct PSAPI_WORKING_SET_EX_BLOCK_0_1 {
    pub _bitfield: usize,
}
impl PSAPI_WORKING_SET_EX_BLOCK_0_1 {}
impl ::std::default::Default for PSAPI_WORKING_SET_EX_BLOCK_0_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PSAPI_WORKING_SET_EX_BLOCK_0_1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Invalid_e__Struct")
            .field("_bitfield", &self._bitfield)
            .finish()
    }
}
impl ::std::cmp::PartialEq for PSAPI_WORKING_SET_EX_BLOCK_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for PSAPI_WORKING_SET_EX_BLOCK_0_1 {}
unsafe impl ::windows::runtime::Abi for PSAPI_WORKING_SET_EX_BLOCK_0_1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct PSAPI_WORKING_SET_EX_INFORMATION {
    pub VirtualAddress: *mut ::std::ffi::c_void,
    pub VirtualAttributes: PSAPI_WORKING_SET_EX_BLOCK,
}
impl PSAPI_WORKING_SET_EX_INFORMATION {}
impl ::std::default::Default for PSAPI_WORKING_SET_EX_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for PSAPI_WORKING_SET_EX_INFORMATION {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for PSAPI_WORKING_SET_EX_INFORMATION {}
unsafe impl ::windows::runtime::Abi for PSAPI_WORKING_SET_EX_INFORMATION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct PSAPI_WORKING_SET_INFORMATION {
    pub NumberOfEntries: usize,
    pub WorkingSetInfo: [PSAPI_WORKING_SET_BLOCK; 1],
}
impl PSAPI_WORKING_SET_INFORMATION {}
impl ::std::default::Default for PSAPI_WORKING_SET_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for PSAPI_WORKING_SET_INFORMATION {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for PSAPI_WORKING_SET_INFORMATION {}
unsafe impl ::windows::runtime::Abi for PSAPI_WORKING_SET_INFORMATION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct PSAPI_WS_WATCH_INFORMATION {
    pub FaultingPc: *mut ::std::ffi::c_void,
    pub FaultingVa: *mut ::std::ffi::c_void,
}
impl PSAPI_WS_WATCH_INFORMATION {}
impl ::std::default::Default for PSAPI_WS_WATCH_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PSAPI_WS_WATCH_INFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PSAPI_WS_WATCH_INFORMATION")
            .field("FaultingPc", &self.FaultingPc)
            .field("FaultingVa", &self.FaultingVa)
            .finish()
    }
}
impl ::std::cmp::PartialEq for PSAPI_WS_WATCH_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.FaultingPc == other.FaultingPc && self.FaultingVa == other.FaultingVa
    }
}
impl ::std::cmp::Eq for PSAPI_WS_WATCH_INFORMATION {}
unsafe impl ::windows::runtime::Abi for PSAPI_WS_WATCH_INFORMATION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct PSAPI_WS_WATCH_INFORMATION_EX {
    pub BasicInfo: PSAPI_WS_WATCH_INFORMATION,
    pub FaultingThreadId: usize,
    pub Flags: usize,
}
impl PSAPI_WS_WATCH_INFORMATION_EX {}
impl ::std::default::Default for PSAPI_WS_WATCH_INFORMATION_EX {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PSAPI_WS_WATCH_INFORMATION_EX {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PSAPI_WS_WATCH_INFORMATION_EX")
            .field("BasicInfo", &self.BasicInfo)
            .field("FaultingThreadId", &self.FaultingThreadId)
            .field("Flags", &self.Flags)
            .finish()
    }
}
impl ::std::cmp::PartialEq for PSAPI_WS_WATCH_INFORMATION_EX {
    fn eq(&self, other: &Self) -> bool {
        self.BasicInfo == other.BasicInfo
            && self.FaultingThreadId == other.FaultingThreadId
            && self.Flags == other.Flags
    }
}
impl ::std::cmp::Eq for PSAPI_WS_WATCH_INFORMATION_EX {}
unsafe impl ::windows::runtime::Abi for PSAPI_WS_WATCH_INFORMATION_EX {
    type Abi = Self;
    type DefaultType = Self;
}
