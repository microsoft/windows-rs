#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ProcessStatus\"`*"]
pub struct ENUM_PAGE_FILE_INFORMATION {
    pub cb: u32,
    pub Reserved: u32,
    pub TotalSize: usize,
    pub TotalInUse: usize,
    pub PeakUsage: usize,
}
impl ::core::marker::Copy for ENUM_PAGE_FILE_INFORMATION {}
impl ::core::clone::Clone for ENUM_PAGE_FILE_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ENUM_PAGE_FILE_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENUM_PAGE_FILE_INFORMATION").field("cb", &self.cb).field("Reserved", &self.Reserved).field("TotalSize", &self.TotalSize).field("TotalInUse", &self.TotalInUse).field("PeakUsage", &self.PeakUsage).finish()
    }
}
unsafe impl ::windows::core::Abi for ENUM_PAGE_FILE_INFORMATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ENUM_PAGE_FILE_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ENUM_PAGE_FILE_INFORMATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for ENUM_PAGE_FILE_INFORMATION {}
impl ::core::default::Default for ENUM_PAGE_FILE_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_ProcessStatus\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ENUM_PROCESS_MODULES_EX_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_System_ProcessStatus\"`*"]
pub const LIST_MODULES_ALL: ENUM_PROCESS_MODULES_EX_FLAGS = ENUM_PROCESS_MODULES_EX_FLAGS(3u32);
#[doc = "*Required features: `\"Win32_System_ProcessStatus\"`*"]
pub const LIST_MODULES_DEFAULT: ENUM_PROCESS_MODULES_EX_FLAGS = ENUM_PROCESS_MODULES_EX_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_System_ProcessStatus\"`*"]
pub const LIST_MODULES_32BIT: ENUM_PROCESS_MODULES_EX_FLAGS = ENUM_PROCESS_MODULES_EX_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_System_ProcessStatus\"`*"]
pub const LIST_MODULES_64BIT: ENUM_PROCESS_MODULES_EX_FLAGS = ENUM_PROCESS_MODULES_EX_FLAGS(2u32);
impl ::core::marker::Copy for ENUM_PROCESS_MODULES_EX_FLAGS {}
impl ::core::clone::Clone for ENUM_PROCESS_MODULES_EX_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ENUM_PROCESS_MODULES_EX_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ENUM_PROCESS_MODULES_EX_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for ENUM_PROCESS_MODULES_EX_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ENUM_PROCESS_MODULES_EX_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_ProcessStatus\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn K32EmptyWorkingSet<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hprocess: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn K32EmptyWorkingSet(hprocess: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(K32EmptyWorkingSet(hprocess.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_ProcessStatus\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn K32EnumDeviceDrivers(lpimagebase: *mut *mut ::core::ffi::c_void, cb: u32, lpcbneeded: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn K32EnumDeviceDrivers(lpimagebase: *mut *mut ::core::ffi::c_void, cb: u32, lpcbneeded: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(K32EnumDeviceDrivers(::core::mem::transmute(lpimagebase), ::core::mem::transmute(cb), ::core::mem::transmute(lpcbneeded)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_ProcessStatus\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn K32EnumPageFilesA(pcallbackroutine: PENUM_PAGE_FILE_CALLBACKA, pcontext: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn K32EnumPageFilesA(pcallbackroutine: ::windows::core::RawPtr, pcontext: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(K32EnumPageFilesA(::core::mem::transmute(pcallbackroutine), ::core::mem::transmute(pcontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_ProcessStatus\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn K32EnumPageFilesW(pcallbackroutine: PENUM_PAGE_FILE_CALLBACKW, pcontext: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn K32EnumPageFilesW(pcallbackroutine: ::windows::core::RawPtr, pcontext: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(K32EnumPageFilesW(::core::mem::transmute(pcallbackroutine), ::core::mem::transmute(pcontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_ProcessStatus\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn K32EnumProcessModules<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hprocess: Param0, lphmodule: *mut super::super::Foundation::HINSTANCE, cb: u32, lpcbneeded: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn K32EnumProcessModules(hprocess: super::super::Foundation::HANDLE, lphmodule: *mut super::super::Foundation::HINSTANCE, cb: u32, lpcbneeded: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(K32EnumProcessModules(hprocess.into_param().abi(), ::core::mem::transmute(lphmodule), ::core::mem::transmute(cb), ::core::mem::transmute(lpcbneeded)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_ProcessStatus\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn K32EnumProcessModulesEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hprocess: Param0, lphmodule: *mut super::super::Foundation::HINSTANCE, cb: u32, lpcbneeded: *mut u32, dwfilterflag: ENUM_PROCESS_MODULES_EX_FLAGS) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn K32EnumProcessModulesEx(hprocess: super::super::Foundation::HANDLE, lphmodule: *mut super::super::Foundation::HINSTANCE, cb: u32, lpcbneeded: *mut u32, dwfilterflag: ENUM_PROCESS_MODULES_EX_FLAGS) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(K32EnumProcessModulesEx(hprocess.into_param().abi(), ::core::mem::transmute(lphmodule), ::core::mem::transmute(cb), ::core::mem::transmute(lpcbneeded), ::core::mem::transmute(dwfilterflag)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_ProcessStatus\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn K32EnumProcesses(lpidprocess: *mut u32, cb: u32, lpcbneeded: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn K32EnumProcesses(lpidprocess: *mut u32, cb: u32, lpcbneeded: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(K32EnumProcesses(::core::mem::transmute(lpidprocess), ::core::mem::transmute(cb), ::core::mem::transmute(lpcbneeded)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_ProcessStatus\"`*"]
#[inline]
pub unsafe fn K32GetDeviceDriverBaseNameA(imagebase: *const ::core::ffi::c_void, lpfilename: &mut [u8]) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn K32GetDeviceDriverBaseNameA(imagebase: *const ::core::ffi::c_void, lpfilename: ::windows::core::PSTR, nsize: u32) -> u32;
        }
        ::core::mem::transmute(K32GetDeviceDriverBaseNameA(::core::mem::transmute(imagebase), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(lpfilename)), lpfilename.len() as _))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_ProcessStatus\"`*"]
#[inline]
pub unsafe fn K32GetDeviceDriverBaseNameW(imagebase: *const ::core::ffi::c_void, lpbasename: &mut [u16]) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn K32GetDeviceDriverBaseNameW(imagebase: *const ::core::ffi::c_void, lpbasename: ::windows::core::PWSTR, nsize: u32) -> u32;
        }
        ::core::mem::transmute(K32GetDeviceDriverBaseNameW(::core::mem::transmute(imagebase), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(lpbasename)), lpbasename.len() as _))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_ProcessStatus\"`*"]
#[inline]
pub unsafe fn K32GetDeviceDriverFileNameA(imagebase: *const ::core::ffi::c_void, lpfilename: &mut [u8]) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn K32GetDeviceDriverFileNameA(imagebase: *const ::core::ffi::c_void, lpfilename: ::windows::core::PSTR, nsize: u32) -> u32;
        }
        ::core::mem::transmute(K32GetDeviceDriverFileNameA(::core::mem::transmute(imagebase), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(lpfilename)), lpfilename.len() as _))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_ProcessStatus\"`*"]
#[inline]
pub unsafe fn K32GetDeviceDriverFileNameW(imagebase: *const ::core::ffi::c_void, lpfilename: &mut [u16]) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn K32GetDeviceDriverFileNameW(imagebase: *const ::core::ffi::c_void, lpfilename: ::windows::core::PWSTR, nsize: u32) -> u32;
        }
        ::core::mem::transmute(K32GetDeviceDriverFileNameW(::core::mem::transmute(imagebase), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(lpfilename)), lpfilename.len() as _))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_ProcessStatus\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn K32GetMappedFileNameA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hprocess: Param0, lpv: *const ::core::ffi::c_void, lpfilename: &mut [u8]) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn K32GetMappedFileNameA(hprocess: super::super::Foundation::HANDLE, lpv: *const ::core::ffi::c_void, lpfilename: ::windows::core::PSTR, nsize: u32) -> u32;
        }
        ::core::mem::transmute(K32GetMappedFileNameA(hprocess.into_param().abi(), ::core::mem::transmute(lpv), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(lpfilename)), lpfilename.len() as _))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_ProcessStatus\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn K32GetMappedFileNameW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hprocess: Param0, lpv: *const ::core::ffi::c_void, lpfilename: &mut [u16]) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn K32GetMappedFileNameW(hprocess: super::super::Foundation::HANDLE, lpv: *const ::core::ffi::c_void, lpfilename: ::windows::core::PWSTR, nsize: u32) -> u32;
        }
        ::core::mem::transmute(K32GetMappedFileNameW(hprocess.into_param().abi(), ::core::mem::transmute(lpv), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(lpfilename)), lpfilename.len() as _))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_ProcessStatus\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn K32GetModuleBaseNameA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HINSTANCE>>(hprocess: Param0, hmodule: Param1, lpbasename: &mut [u8]) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn K32GetModuleBaseNameA(hprocess: super::super::Foundation::HANDLE, hmodule: super::super::Foundation::HINSTANCE, lpbasename: ::windows::core::PSTR, nsize: u32) -> u32;
        }
        ::core::mem::transmute(K32GetModuleBaseNameA(hprocess.into_param().abi(), hmodule.into_param().abi(), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(lpbasename)), lpbasename.len() as _))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_ProcessStatus\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn K32GetModuleBaseNameW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HINSTANCE>>(hprocess: Param0, hmodule: Param1, lpbasename: &mut [u16]) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn K32GetModuleBaseNameW(hprocess: super::super::Foundation::HANDLE, hmodule: super::super::Foundation::HINSTANCE, lpbasename: ::windows::core::PWSTR, nsize: u32) -> u32;
        }
        ::core::mem::transmute(K32GetModuleBaseNameW(hprocess.into_param().abi(), hmodule.into_param().abi(), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(lpbasename)), lpbasename.len() as _))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_ProcessStatus\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn K32GetModuleFileNameExA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HINSTANCE>>(hprocess: Param0, hmodule: Param1, lpfilename: &mut [u8]) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn K32GetModuleFileNameExA(hprocess: super::super::Foundation::HANDLE, hmodule: super::super::Foundation::HINSTANCE, lpfilename: ::windows::core::PSTR, nsize: u32) -> u32;
        }
        ::core::mem::transmute(K32GetModuleFileNameExA(hprocess.into_param().abi(), hmodule.into_param().abi(), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(lpfilename)), lpfilename.len() as _))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_ProcessStatus\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn K32GetModuleFileNameExW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HINSTANCE>>(hprocess: Param0, hmodule: Param1, lpfilename: &mut [u16]) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn K32GetModuleFileNameExW(hprocess: super::super::Foundation::HANDLE, hmodule: super::super::Foundation::HINSTANCE, lpfilename: ::windows::core::PWSTR, nsize: u32) -> u32;
        }
        ::core::mem::transmute(K32GetModuleFileNameExW(hprocess.into_param().abi(), hmodule.into_param().abi(), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(lpfilename)), lpfilename.len() as _))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_ProcessStatus\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn K32GetModuleInformation<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HINSTANCE>>(hprocess: Param0, hmodule: Param1, lpmodinfo: *mut MODULEINFO, cb: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn K32GetModuleInformation(hprocess: super::super::Foundation::HANDLE, hmodule: super::super::Foundation::HINSTANCE, lpmodinfo: *mut MODULEINFO, cb: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(K32GetModuleInformation(hprocess.into_param().abi(), hmodule.into_param().abi(), ::core::mem::transmute(lpmodinfo), ::core::mem::transmute(cb)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_ProcessStatus\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn K32GetPerformanceInfo(pperformanceinformation: *mut PERFORMANCE_INFORMATION, cb: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn K32GetPerformanceInfo(pperformanceinformation: *mut PERFORMANCE_INFORMATION, cb: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(K32GetPerformanceInfo(::core::mem::transmute(pperformanceinformation), ::core::mem::transmute(cb)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_ProcessStatus\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn K32GetProcessImageFileNameA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hprocess: Param0, lpimagefilename: &mut [u8]) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn K32GetProcessImageFileNameA(hprocess: super::super::Foundation::HANDLE, lpimagefilename: ::windows::core::PSTR, nsize: u32) -> u32;
        }
        ::core::mem::transmute(K32GetProcessImageFileNameA(hprocess.into_param().abi(), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(lpimagefilename)), lpimagefilename.len() as _))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_ProcessStatus\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn K32GetProcessImageFileNameW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hprocess: Param0, lpimagefilename: &mut [u16]) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn K32GetProcessImageFileNameW(hprocess: super::super::Foundation::HANDLE, lpimagefilename: ::windows::core::PWSTR, nsize: u32) -> u32;
        }
        ::core::mem::transmute(K32GetProcessImageFileNameW(hprocess.into_param().abi(), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(lpimagefilename)), lpimagefilename.len() as _))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_ProcessStatus\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn K32GetProcessMemoryInfo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(process: Param0, ppsmemcounters: *mut PROCESS_MEMORY_COUNTERS, cb: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn K32GetProcessMemoryInfo(process: super::super::Foundation::HANDLE, ppsmemcounters: *mut PROCESS_MEMORY_COUNTERS, cb: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(K32GetProcessMemoryInfo(process.into_param().abi(), ::core::mem::transmute(ppsmemcounters), ::core::mem::transmute(cb)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_ProcessStatus\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn K32GetWsChanges<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hprocess: Param0, lpwatchinfo: *mut PSAPI_WS_WATCH_INFORMATION, cb: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn K32GetWsChanges(hprocess: super::super::Foundation::HANDLE, lpwatchinfo: *mut PSAPI_WS_WATCH_INFORMATION, cb: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(K32GetWsChanges(hprocess.into_param().abi(), ::core::mem::transmute(lpwatchinfo), ::core::mem::transmute(cb)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_ProcessStatus\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn K32GetWsChangesEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hprocess: Param0, lpwatchinfoex: *mut PSAPI_WS_WATCH_INFORMATION_EX, cb: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn K32GetWsChangesEx(hprocess: super::super::Foundation::HANDLE, lpwatchinfoex: *mut PSAPI_WS_WATCH_INFORMATION_EX, cb: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(K32GetWsChangesEx(hprocess.into_param().abi(), ::core::mem::transmute(lpwatchinfoex), ::core::mem::transmute(cb)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_ProcessStatus\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn K32InitializeProcessForWsWatch<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hprocess: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn K32InitializeProcessForWsWatch(hprocess: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(K32InitializeProcessForWsWatch(hprocess.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_ProcessStatus\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn K32QueryWorkingSet<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hprocess: Param0, pv: *mut ::core::ffi::c_void, cb: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn K32QueryWorkingSet(hprocess: super::super::Foundation::HANDLE, pv: *mut ::core::ffi::c_void, cb: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(K32QueryWorkingSet(hprocess.into_param().abi(), ::core::mem::transmute(pv), ::core::mem::transmute(cb)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_ProcessStatus\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn K32QueryWorkingSetEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hprocess: Param0, pv: *mut ::core::ffi::c_void, cb: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn K32QueryWorkingSetEx(hprocess: super::super::Foundation::HANDLE, pv: *mut ::core::ffi::c_void, cb: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(K32QueryWorkingSetEx(hprocess.into_param().abi(), ::core::mem::transmute(pv), ::core::mem::transmute(cb)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ProcessStatus\"`*"]
pub struct MODULEINFO {
    pub lpBaseOfDll: *mut ::core::ffi::c_void,
    pub SizeOfImage: u32,
    pub EntryPoint: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for MODULEINFO {}
impl ::core::clone::Clone for MODULEINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MODULEINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MODULEINFO").field("lpBaseOfDll", &self.lpBaseOfDll).field("SizeOfImage", &self.SizeOfImage).field("EntryPoint", &self.EntryPoint).finish()
    }
}
unsafe impl ::windows::core::Abi for MODULEINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MODULEINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MODULEINFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for MODULEINFO {}
impl ::core::default::Default for MODULEINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_ProcessStatus\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PENUM_PAGE_FILE_CALLBACKA = ::core::option::Option<unsafe extern "system" fn(pcontext: *mut ::core::ffi::c_void, ppagefileinfo: *mut ENUM_PAGE_FILE_INFORMATION, lpfilename: ::windows::core::PCSTR) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_ProcessStatus\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PENUM_PAGE_FILE_CALLBACKW = ::core::option::Option<unsafe extern "system" fn(pcontext: *mut ::core::ffi::c_void, ppagefileinfo: *mut ENUM_PAGE_FILE_INFORMATION, lpfilename: ::windows::core::PCWSTR) -> super::super::Foundation::BOOL>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ProcessStatus\"`*"]
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
impl ::core::marker::Copy for PERFORMANCE_INFORMATION {}
impl ::core::clone::Clone for PERFORMANCE_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PERFORMANCE_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERFORMANCE_INFORMATION")
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
unsafe impl ::windows::core::Abi for PERFORMANCE_INFORMATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PERFORMANCE_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PERFORMANCE_INFORMATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for PERFORMANCE_INFORMATION {}
impl ::core::default::Default for PERFORMANCE_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ProcessStatus\"`*"]
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
impl ::core::marker::Copy for PROCESS_MEMORY_COUNTERS {}
impl ::core::clone::Clone for PROCESS_MEMORY_COUNTERS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PROCESS_MEMORY_COUNTERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROCESS_MEMORY_COUNTERS")
            .field("cb", &self.cb)
            .field("PageFaultCount", &self.PageFaultCount)
            .field("PeakWorkingSetSize", &self.PeakWorkingSetSize)
            .field("WorkingSetSize", &self.WorkingSetSize)
            .field("QuotaPeakPagedPoolUsage", &self.QuotaPeakPagedPoolUsage)
            .field("QuotaPagedPoolUsage", &self.QuotaPagedPoolUsage)
            .field("QuotaPeakNonPagedPoolUsage", &self.QuotaPeakNonPagedPoolUsage)
            .field("QuotaNonPagedPoolUsage", &self.QuotaNonPagedPoolUsage)
            .field("PagefileUsage", &self.PagefileUsage)
            .field("PeakPagefileUsage", &self.PeakPagefileUsage)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for PROCESS_MEMORY_COUNTERS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PROCESS_MEMORY_COUNTERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PROCESS_MEMORY_COUNTERS>()) == 0 }
    }
}
impl ::core::cmp::Eq for PROCESS_MEMORY_COUNTERS {}
impl ::core::default::Default for PROCESS_MEMORY_COUNTERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ProcessStatus\"`*"]
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
impl ::core::marker::Copy for PROCESS_MEMORY_COUNTERS_EX {}
impl ::core::clone::Clone for PROCESS_MEMORY_COUNTERS_EX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PROCESS_MEMORY_COUNTERS_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROCESS_MEMORY_COUNTERS_EX")
            .field("cb", &self.cb)
            .field("PageFaultCount", &self.PageFaultCount)
            .field("PeakWorkingSetSize", &self.PeakWorkingSetSize)
            .field("WorkingSetSize", &self.WorkingSetSize)
            .field("QuotaPeakPagedPoolUsage", &self.QuotaPeakPagedPoolUsage)
            .field("QuotaPagedPoolUsage", &self.QuotaPagedPoolUsage)
            .field("QuotaPeakNonPagedPoolUsage", &self.QuotaPeakNonPagedPoolUsage)
            .field("QuotaNonPagedPoolUsage", &self.QuotaNonPagedPoolUsage)
            .field("PagefileUsage", &self.PagefileUsage)
            .field("PeakPagefileUsage", &self.PeakPagefileUsage)
            .field("PrivateUsage", &self.PrivateUsage)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for PROCESS_MEMORY_COUNTERS_EX {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PROCESS_MEMORY_COUNTERS_EX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PROCESS_MEMORY_COUNTERS_EX>()) == 0 }
    }
}
impl ::core::cmp::Eq for PROCESS_MEMORY_COUNTERS_EX {}
impl ::core::default::Default for PROCESS_MEMORY_COUNTERS_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_ProcessStatus\"`*"]
pub const PSAPI_VERSION: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ProcessStatus\"`*"]
pub union PSAPI_WORKING_SET_BLOCK {
    pub Flags: usize,
    pub Anonymous: PSAPI_WORKING_SET_BLOCK_0,
}
impl ::core::marker::Copy for PSAPI_WORKING_SET_BLOCK {}
impl ::core::clone::Clone for PSAPI_WORKING_SET_BLOCK {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PSAPI_WORKING_SET_BLOCK {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PSAPI_WORKING_SET_BLOCK {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PSAPI_WORKING_SET_BLOCK>()) == 0 }
    }
}
impl ::core::cmp::Eq for PSAPI_WORKING_SET_BLOCK {}
impl ::core::default::Default for PSAPI_WORKING_SET_BLOCK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ProcessStatus\"`*"]
pub struct PSAPI_WORKING_SET_BLOCK_0 {
    pub _bitfield: usize,
}
impl ::core::marker::Copy for PSAPI_WORKING_SET_BLOCK_0 {}
impl ::core::clone::Clone for PSAPI_WORKING_SET_BLOCK_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PSAPI_WORKING_SET_BLOCK_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PSAPI_WORKING_SET_BLOCK_0").field("_bitfield", &self._bitfield).finish()
    }
}
unsafe impl ::windows::core::Abi for PSAPI_WORKING_SET_BLOCK_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PSAPI_WORKING_SET_BLOCK_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PSAPI_WORKING_SET_BLOCK_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for PSAPI_WORKING_SET_BLOCK_0 {}
impl ::core::default::Default for PSAPI_WORKING_SET_BLOCK_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ProcessStatus\"`*"]
pub union PSAPI_WORKING_SET_EX_BLOCK {
    pub Flags: usize,
    pub Anonymous: PSAPI_WORKING_SET_EX_BLOCK_0,
}
impl ::core::marker::Copy for PSAPI_WORKING_SET_EX_BLOCK {}
impl ::core::clone::Clone for PSAPI_WORKING_SET_EX_BLOCK {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PSAPI_WORKING_SET_EX_BLOCK {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PSAPI_WORKING_SET_EX_BLOCK {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PSAPI_WORKING_SET_EX_BLOCK>()) == 0 }
    }
}
impl ::core::cmp::Eq for PSAPI_WORKING_SET_EX_BLOCK {}
impl ::core::default::Default for PSAPI_WORKING_SET_EX_BLOCK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ProcessStatus\"`*"]
pub union PSAPI_WORKING_SET_EX_BLOCK_0 {
    pub Anonymous: PSAPI_WORKING_SET_EX_BLOCK_0_0,
    pub Invalid: PSAPI_WORKING_SET_EX_BLOCK_0_1,
}
impl ::core::marker::Copy for PSAPI_WORKING_SET_EX_BLOCK_0 {}
impl ::core::clone::Clone for PSAPI_WORKING_SET_EX_BLOCK_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PSAPI_WORKING_SET_EX_BLOCK_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PSAPI_WORKING_SET_EX_BLOCK_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PSAPI_WORKING_SET_EX_BLOCK_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for PSAPI_WORKING_SET_EX_BLOCK_0 {}
impl ::core::default::Default for PSAPI_WORKING_SET_EX_BLOCK_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ProcessStatus\"`*"]
pub struct PSAPI_WORKING_SET_EX_BLOCK_0_0 {
    pub _bitfield: usize,
}
impl ::core::marker::Copy for PSAPI_WORKING_SET_EX_BLOCK_0_0 {}
impl ::core::clone::Clone for PSAPI_WORKING_SET_EX_BLOCK_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PSAPI_WORKING_SET_EX_BLOCK_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PSAPI_WORKING_SET_EX_BLOCK_0_0").field("_bitfield", &self._bitfield).finish()
    }
}
unsafe impl ::windows::core::Abi for PSAPI_WORKING_SET_EX_BLOCK_0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PSAPI_WORKING_SET_EX_BLOCK_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PSAPI_WORKING_SET_EX_BLOCK_0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for PSAPI_WORKING_SET_EX_BLOCK_0_0 {}
impl ::core::default::Default for PSAPI_WORKING_SET_EX_BLOCK_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ProcessStatus\"`*"]
pub struct PSAPI_WORKING_SET_EX_BLOCK_0_1 {
    pub _bitfield: usize,
}
impl ::core::marker::Copy for PSAPI_WORKING_SET_EX_BLOCK_0_1 {}
impl ::core::clone::Clone for PSAPI_WORKING_SET_EX_BLOCK_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PSAPI_WORKING_SET_EX_BLOCK_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PSAPI_WORKING_SET_EX_BLOCK_0_1").field("_bitfield", &self._bitfield).finish()
    }
}
unsafe impl ::windows::core::Abi for PSAPI_WORKING_SET_EX_BLOCK_0_1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PSAPI_WORKING_SET_EX_BLOCK_0_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PSAPI_WORKING_SET_EX_BLOCK_0_1>()) == 0 }
    }
}
impl ::core::cmp::Eq for PSAPI_WORKING_SET_EX_BLOCK_0_1 {}
impl ::core::default::Default for PSAPI_WORKING_SET_EX_BLOCK_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ProcessStatus\"`*"]
pub struct PSAPI_WORKING_SET_EX_INFORMATION {
    pub VirtualAddress: *mut ::core::ffi::c_void,
    pub VirtualAttributes: PSAPI_WORKING_SET_EX_BLOCK,
}
impl ::core::marker::Copy for PSAPI_WORKING_SET_EX_INFORMATION {}
impl ::core::clone::Clone for PSAPI_WORKING_SET_EX_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PSAPI_WORKING_SET_EX_INFORMATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PSAPI_WORKING_SET_EX_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PSAPI_WORKING_SET_EX_INFORMATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for PSAPI_WORKING_SET_EX_INFORMATION {}
impl ::core::default::Default for PSAPI_WORKING_SET_EX_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ProcessStatus\"`*"]
pub struct PSAPI_WORKING_SET_INFORMATION {
    pub NumberOfEntries: usize,
    pub WorkingSetInfo: [PSAPI_WORKING_SET_BLOCK; 1],
}
impl ::core::marker::Copy for PSAPI_WORKING_SET_INFORMATION {}
impl ::core::clone::Clone for PSAPI_WORKING_SET_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PSAPI_WORKING_SET_INFORMATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PSAPI_WORKING_SET_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PSAPI_WORKING_SET_INFORMATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for PSAPI_WORKING_SET_INFORMATION {}
impl ::core::default::Default for PSAPI_WORKING_SET_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ProcessStatus\"`*"]
pub struct PSAPI_WS_WATCH_INFORMATION {
    pub FaultingPc: *mut ::core::ffi::c_void,
    pub FaultingVa: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for PSAPI_WS_WATCH_INFORMATION {}
impl ::core::clone::Clone for PSAPI_WS_WATCH_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PSAPI_WS_WATCH_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PSAPI_WS_WATCH_INFORMATION").field("FaultingPc", &self.FaultingPc).field("FaultingVa", &self.FaultingVa).finish()
    }
}
unsafe impl ::windows::core::Abi for PSAPI_WS_WATCH_INFORMATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PSAPI_WS_WATCH_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PSAPI_WS_WATCH_INFORMATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for PSAPI_WS_WATCH_INFORMATION {}
impl ::core::default::Default for PSAPI_WS_WATCH_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ProcessStatus\"`*"]
pub struct PSAPI_WS_WATCH_INFORMATION_EX {
    pub BasicInfo: PSAPI_WS_WATCH_INFORMATION,
    pub FaultingThreadId: usize,
    pub Flags: usize,
}
impl ::core::marker::Copy for PSAPI_WS_WATCH_INFORMATION_EX {}
impl ::core::clone::Clone for PSAPI_WS_WATCH_INFORMATION_EX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PSAPI_WS_WATCH_INFORMATION_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PSAPI_WS_WATCH_INFORMATION_EX").field("BasicInfo", &self.BasicInfo).field("FaultingThreadId", &self.FaultingThreadId).field("Flags", &self.Flags).finish()
    }
}
unsafe impl ::windows::core::Abi for PSAPI_WS_WATCH_INFORMATION_EX {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PSAPI_WS_WATCH_INFORMATION_EX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PSAPI_WS_WATCH_INFORMATION_EX>()) == 0 }
    }
}
impl ::core::cmp::Eq for PSAPI_WS_WATCH_INFORMATION_EX {}
impl ::core::default::Default for PSAPI_WS_WATCH_INFORMATION_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
