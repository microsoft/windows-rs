#[cfg(feature = "Win32_System_Memory_NonVolatile")]
pub mod NonVolatile;
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AddSecureMemoryCacheCallback(pfncallback: PSECURE_MEMORY_CACHE_CALLBACK) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "kernel32.dll""system" fn AddSecureMemoryCacheCallback ( pfncallback : PSECURE_MEMORY_CACHE_CALLBACK ) -> super::super::Foundation:: BOOL );
    AddSecureMemoryCacheCallback(pfncallback)
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AllocateUserPhysicalPages<P0>(hprocess: P0, numberofpages: *mut usize, pagearray: *mut usize) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn AllocateUserPhysicalPages ( hprocess : super::super::Foundation:: HANDLE , numberofpages : *mut usize , pagearray : *mut usize ) -> super::super::Foundation:: BOOL );
    AllocateUserPhysicalPages(hprocess.into_param().abi(), numberofpages, pagearray)
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AllocateUserPhysicalPages2<P0>(objecthandle: P0, numberofpages: *mut usize, pagearray: *mut usize, extendedparameters: ::core::option::Option<&mut [MEM_EXTENDED_PARAMETER]>) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "api-ms-win-core-memory-l1-1-8.dll""system" fn AllocateUserPhysicalPages2 ( objecthandle : super::super::Foundation:: HANDLE , numberofpages : *mut usize , pagearray : *mut usize , extendedparameters : *mut MEM_EXTENDED_PARAMETER , extendedparametercount : u32 ) -> super::super::Foundation:: BOOL );
    AllocateUserPhysicalPages2(objecthandle.into_param().abi(), numberofpages, pagearray, ::core::mem::transmute(extendedparameters.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), extendedparameters.as_deref().map_or(0, |slice| slice.len() as _))
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AllocateUserPhysicalPagesNuma<P0>(hprocess: P0, numberofpages: *mut usize, pagearray: *mut usize, nndpreferred: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn AllocateUserPhysicalPagesNuma ( hprocess : super::super::Foundation:: HANDLE , numberofpages : *mut usize , pagearray : *mut usize , nndpreferred : u32 ) -> super::super::Foundation:: BOOL );
    AllocateUserPhysicalPagesNuma(hprocess.into_param().abi(), numberofpages, pagearray, nndpreferred)
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateFileMapping2<P0, P1>(file: P0, securityattributes: ::core::option::Option<*const super::super::Security::SECURITY_ATTRIBUTES>, desiredaccess: u32, pageprotection: PAGE_PROTECTION_FLAGS, allocationattributes: u32, maximumsize: u64, name: P1, extendedparameters: ::core::option::Option<&mut [MEM_EXTENDED_PARAMETER]>) -> ::windows::core::Result<super::super::Foundation::HANDLE>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "api-ms-win-core-memory-l1-1-7.dll""system" fn CreateFileMapping2 ( file : super::super::Foundation:: HANDLE , securityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES , desiredaccess : u32 , pageprotection : PAGE_PROTECTION_FLAGS , allocationattributes : u32 , maximumsize : u64 , name : ::windows::core::PCWSTR , extendedparameters : *mut MEM_EXTENDED_PARAMETER , parametercount : u32 ) -> super::super::Foundation:: HANDLE );
    let result__ = CreateFileMapping2(file.into_param().abi(), ::core::mem::transmute(securityattributes.unwrap_or(::std::ptr::null())), desiredaccess, pageprotection, allocationattributes, maximumsize, name.into_param().abi(), ::core::mem::transmute(extendedparameters.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), extendedparameters.as_deref().map_or(0, |slice| slice.len() as _));
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateFileMappingA<P0, P1>(hfile: P0, lpfilemappingattributes: ::core::option::Option<*const super::super::Security::SECURITY_ATTRIBUTES>, flprotect: PAGE_PROTECTION_FLAGS, dwmaximumsizehigh: u32, dwmaximumsizelow: u32, lpname: P1) -> ::windows::core::Result<super::super::Foundation::HANDLE>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn CreateFileMappingA ( hfile : super::super::Foundation:: HANDLE , lpfilemappingattributes : *const super::super::Security:: SECURITY_ATTRIBUTES , flprotect : PAGE_PROTECTION_FLAGS , dwmaximumsizehigh : u32 , dwmaximumsizelow : u32 , lpname : ::windows::core::PCSTR ) -> super::super::Foundation:: HANDLE );
    let result__ = CreateFileMappingA(hfile.into_param().abi(), ::core::mem::transmute(lpfilemappingattributes.unwrap_or(::std::ptr::null())), flprotect, dwmaximumsizehigh, dwmaximumsizelow, lpname.into_param().abi());
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateFileMappingFromApp<P0, P1>(hfile: P0, securityattributes: ::core::option::Option<*const super::super::Security::SECURITY_ATTRIBUTES>, pageprotection: PAGE_PROTECTION_FLAGS, maximumsize: u64, name: P1) -> ::windows::core::Result<super::super::Foundation::HANDLE>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn CreateFileMappingFromApp ( hfile : super::super::Foundation:: HANDLE , securityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES , pageprotection : PAGE_PROTECTION_FLAGS , maximumsize : u64 , name : ::windows::core::PCWSTR ) -> super::super::Foundation:: HANDLE );
    let result__ = CreateFileMappingFromApp(hfile.into_param().abi(), ::core::mem::transmute(securityattributes.unwrap_or(::std::ptr::null())), pageprotection, maximumsize, name.into_param().abi());
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateFileMappingNumaA<P0, P1>(hfile: P0, lpfilemappingattributes: ::core::option::Option<*const super::super::Security::SECURITY_ATTRIBUTES>, flprotect: PAGE_PROTECTION_FLAGS, dwmaximumsizehigh: u32, dwmaximumsizelow: u32, lpname: P1, nndpreferred: u32) -> ::windows::core::Result<super::super::Foundation::HANDLE>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn CreateFileMappingNumaA ( hfile : super::super::Foundation:: HANDLE , lpfilemappingattributes : *const super::super::Security:: SECURITY_ATTRIBUTES , flprotect : PAGE_PROTECTION_FLAGS , dwmaximumsizehigh : u32 , dwmaximumsizelow : u32 , lpname : ::windows::core::PCSTR , nndpreferred : u32 ) -> super::super::Foundation:: HANDLE );
    let result__ = CreateFileMappingNumaA(hfile.into_param().abi(), ::core::mem::transmute(lpfilemappingattributes.unwrap_or(::std::ptr::null())), flprotect, dwmaximumsizehigh, dwmaximumsizelow, lpname.into_param().abi(), nndpreferred);
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateFileMappingNumaW<P0, P1>(hfile: P0, lpfilemappingattributes: ::core::option::Option<*const super::super::Security::SECURITY_ATTRIBUTES>, flprotect: PAGE_PROTECTION_FLAGS, dwmaximumsizehigh: u32, dwmaximumsizelow: u32, lpname: P1, nndpreferred: u32) -> ::windows::core::Result<super::super::Foundation::HANDLE>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn CreateFileMappingNumaW ( hfile : super::super::Foundation:: HANDLE , lpfilemappingattributes : *const super::super::Security:: SECURITY_ATTRIBUTES , flprotect : PAGE_PROTECTION_FLAGS , dwmaximumsizehigh : u32 , dwmaximumsizelow : u32 , lpname : ::windows::core::PCWSTR , nndpreferred : u32 ) -> super::super::Foundation:: HANDLE );
    let result__ = CreateFileMappingNumaW(hfile.into_param().abi(), ::core::mem::transmute(lpfilemappingattributes.unwrap_or(::std::ptr::null())), flprotect, dwmaximumsizehigh, dwmaximumsizelow, lpname.into_param().abi(), nndpreferred);
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateFileMappingW<P0, P1>(hfile: P0, lpfilemappingattributes: ::core::option::Option<*const super::super::Security::SECURITY_ATTRIBUTES>, flprotect: PAGE_PROTECTION_FLAGS, dwmaximumsizehigh: u32, dwmaximumsizelow: u32, lpname: P1) -> ::windows::core::Result<super::super::Foundation::HANDLE>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn CreateFileMappingW ( hfile : super::super::Foundation:: HANDLE , lpfilemappingattributes : *const super::super::Security:: SECURITY_ATTRIBUTES , flprotect : PAGE_PROTECTION_FLAGS , dwmaximumsizehigh : u32 , dwmaximumsizelow : u32 , lpname : ::windows::core::PCWSTR ) -> super::super::Foundation:: HANDLE );
    let result__ = CreateFileMappingW(hfile.into_param().abi(), ::core::mem::transmute(lpfilemappingattributes.unwrap_or(::std::ptr::null())), flprotect, dwmaximumsizehigh, dwmaximumsizelow, lpname.into_param().abi());
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateMemoryResourceNotification(notificationtype: MEMORY_RESOURCE_NOTIFICATION_TYPE) -> ::windows::core::Result<super::super::Foundation::HANDLE> {
    ::windows_targets::link ! ( "kernel32.dll""system" fn CreateMemoryResourceNotification ( notificationtype : MEMORY_RESOURCE_NOTIFICATION_TYPE ) -> super::super::Foundation:: HANDLE );
    let result__ = CreateMemoryResourceNotification(notificationtype);
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
#[inline]
pub unsafe fn DiscardVirtualMemory(virtualaddress: &mut [u8]) -> u32 {
    ::windows_targets::link ! ( "kernel32.dll""system" fn DiscardVirtualMemory ( virtualaddress : *mut ::core::ffi::c_void , size : usize ) -> u32 );
    DiscardVirtualMemory(::core::mem::transmute(virtualaddress.as_ptr()), virtualaddress.len() as _)
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FlushViewOfFile(lpbaseaddress: *const ::core::ffi::c_void, dwnumberofbytestoflush: usize) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "kernel32.dll""system" fn FlushViewOfFile ( lpbaseaddress : *const ::core::ffi::c_void , dwnumberofbytestoflush : usize ) -> super::super::Foundation:: BOOL );
    FlushViewOfFile(lpbaseaddress, dwnumberofbytestoflush)
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FreeUserPhysicalPages<P0>(hprocess: P0, numberofpages: *mut usize, pagearray: *const usize) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn FreeUserPhysicalPages ( hprocess : super::super::Foundation:: HANDLE , numberofpages : *mut usize , pagearray : *const usize ) -> super::super::Foundation:: BOOL );
    FreeUserPhysicalPages(hprocess.into_param().abi(), numberofpages, pagearray)
}
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
#[inline]
pub unsafe fn GetLargePageMinimum() -> usize {
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetLargePageMinimum ( ) -> usize );
    GetLargePageMinimum()
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetMemoryErrorHandlingCapabilities(capabilities: *mut u32) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetMemoryErrorHandlingCapabilities ( capabilities : *mut u32 ) -> super::super::Foundation:: BOOL );
    GetMemoryErrorHandlingCapabilities(capabilities)
}
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
#[inline]
pub unsafe fn GetProcessHeap() -> ::windows::core::Result<HeapHandle> {
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetProcessHeap ( ) -> HeapHandle );
    let result__ = GetProcessHeap();
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
#[inline]
pub unsafe fn GetProcessHeaps(processheaps: &mut [HeapHandle]) -> u32 {
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetProcessHeaps ( numberofheaps : u32 , processheaps : *mut HeapHandle ) -> u32 );
    GetProcessHeaps(processheaps.len() as _, ::core::mem::transmute(processheaps.as_ptr()))
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetProcessWorkingSetSizeEx<P0>(hprocess: P0, lpminimumworkingsetsize: *mut usize, lpmaximumworkingsetsize: *mut usize, flags: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetProcessWorkingSetSizeEx ( hprocess : super::super::Foundation:: HANDLE , lpminimumworkingsetsize : *mut usize , lpmaximumworkingsetsize : *mut usize , flags : *mut u32 ) -> super::super::Foundation:: BOOL );
    GetProcessWorkingSetSizeEx(hprocess.into_param().abi(), lpminimumworkingsetsize, lpmaximumworkingsetsize, flags)
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetSystemFileCacheSize(lpminimumfilecachesize: *mut usize, lpmaximumfilecachesize: *mut usize, lpflags: *mut u32) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetSystemFileCacheSize ( lpminimumfilecachesize : *mut usize , lpmaximumfilecachesize : *mut usize , lpflags : *mut u32 ) -> super::super::Foundation:: BOOL );
    GetSystemFileCacheSize(lpminimumfilecachesize, lpmaximumfilecachesize, lpflags)
}
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
#[inline]
pub unsafe fn GetWriteWatch(dwflags: u32, lpbaseaddress: *const ::core::ffi::c_void, dwregionsize: usize, lpaddresses: ::core::option::Option<*mut *mut ::core::ffi::c_void>, lpdwcount: ::core::option::Option<*mut usize>, lpdwgranularity: ::core::option::Option<*mut u32>) -> u32 {
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetWriteWatch ( dwflags : u32 , lpbaseaddress : *const ::core::ffi::c_void , dwregionsize : usize , lpaddresses : *mut *mut ::core::ffi::c_void , lpdwcount : *mut usize , lpdwgranularity : *mut u32 ) -> u32 );
    GetWriteWatch(dwflags, lpbaseaddress, dwregionsize, ::core::mem::transmute(lpaddresses.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lpdwcount.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lpdwgranularity.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GlobalAlloc(uflags: GLOBAL_ALLOC_FLAGS, dwbytes: usize) -> ::windows::core::Result<super::super::Foundation::HGLOBAL> {
    ::windows_targets::link ! ( "kernel32.dll""system" fn GlobalAlloc ( uflags : GLOBAL_ALLOC_FLAGS , dwbytes : usize ) -> super::super::Foundation:: HGLOBAL );
    let result__ = GlobalAlloc(uflags, dwbytes);
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GlobalFlags<P0>(hmem: P0) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HGLOBAL>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GlobalFlags ( hmem : super::super::Foundation:: HGLOBAL ) -> u32 );
    GlobalFlags(hmem.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GlobalFree<P0>(hmem: P0) -> ::windows::core::Result<super::super::Foundation::HGLOBAL>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HGLOBAL>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GlobalFree ( hmem : super::super::Foundation:: HGLOBAL ) -> super::super::Foundation:: HGLOBAL );
    let result__ = GlobalFree(hmem.into_param().abi());
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GlobalHandle(pmem: *const ::core::ffi::c_void) -> ::windows::core::Result<super::super::Foundation::HGLOBAL> {
    ::windows_targets::link ! ( "kernel32.dll""system" fn GlobalHandle ( pmem : *const ::core::ffi::c_void ) -> super::super::Foundation:: HGLOBAL );
    let result__ = GlobalHandle(pmem);
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GlobalLock<P0>(hmem: P0) -> *mut ::core::ffi::c_void
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HGLOBAL>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GlobalLock ( hmem : super::super::Foundation:: HGLOBAL ) -> *mut ::core::ffi::c_void );
    GlobalLock(hmem.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GlobalReAlloc<P0>(hmem: P0, dwbytes: usize, uflags: u32) -> ::windows::core::Result<super::super::Foundation::HGLOBAL>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HGLOBAL>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GlobalReAlloc ( hmem : super::super::Foundation:: HGLOBAL , dwbytes : usize , uflags : u32 ) -> super::super::Foundation:: HGLOBAL );
    let result__ = GlobalReAlloc(hmem.into_param().abi(), dwbytes, uflags);
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GlobalSize<P0>(hmem: P0) -> usize
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HGLOBAL>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GlobalSize ( hmem : super::super::Foundation:: HGLOBAL ) -> usize );
    GlobalSize(hmem.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GlobalUnlock<P0>(hmem: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HGLOBAL>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GlobalUnlock ( hmem : super::super::Foundation:: HGLOBAL ) -> super::super::Foundation:: BOOL );
    GlobalUnlock(hmem.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
#[inline]
pub unsafe fn HeapAlloc<P0>(hheap: P0, dwflags: HEAP_FLAGS, dwbytes: usize) -> *mut ::core::ffi::c_void
where
    P0: ::windows::core::IntoParam<HeapHandle>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn HeapAlloc ( hheap : HeapHandle , dwflags : HEAP_FLAGS , dwbytes : usize ) -> *mut ::core::ffi::c_void );
    HeapAlloc(hheap.into_param().abi(), dwflags, dwbytes)
}
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
#[inline]
pub unsafe fn HeapCompact<P0>(hheap: P0, dwflags: HEAP_FLAGS) -> usize
where
    P0: ::windows::core::IntoParam<HeapHandle>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn HeapCompact ( hheap : HeapHandle , dwflags : HEAP_FLAGS ) -> usize );
    HeapCompact(hheap.into_param().abi(), dwflags)
}
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
#[inline]
pub unsafe fn HeapCreate(floptions: HEAP_FLAGS, dwinitialsize: usize, dwmaximumsize: usize) -> ::windows::core::Result<HeapHandle> {
    ::windows_targets::link ! ( "kernel32.dll""system" fn HeapCreate ( floptions : HEAP_FLAGS , dwinitialsize : usize , dwmaximumsize : usize ) -> HeapHandle );
    let result__ = HeapCreate(floptions, dwinitialsize, dwmaximumsize);
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HeapDestroy<P0>(hheap: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<HeapHandle>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn HeapDestroy ( hheap : HeapHandle ) -> super::super::Foundation:: BOOL );
    HeapDestroy(hheap.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HeapFree<P0>(hheap: P0, dwflags: HEAP_FLAGS, lpmem: ::core::option::Option<*const ::core::ffi::c_void>) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<HeapHandle>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn HeapFree ( hheap : HeapHandle , dwflags : HEAP_FLAGS , lpmem : *const ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    HeapFree(hheap.into_param().abi(), dwflags, ::core::mem::transmute(lpmem.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HeapLock<P0>(hheap: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<HeapHandle>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn HeapLock ( hheap : HeapHandle ) -> super::super::Foundation:: BOOL );
    HeapLock(hheap.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HeapQueryInformation<P0>(heaphandle: P0, heapinformationclass: HEAP_INFORMATION_CLASS, heapinformation: ::core::option::Option<*mut ::core::ffi::c_void>, heapinformationlength: usize, returnlength: ::core::option::Option<*mut usize>) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<HeapHandle>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn HeapQueryInformation ( heaphandle : HeapHandle , heapinformationclass : HEAP_INFORMATION_CLASS , heapinformation : *mut ::core::ffi::c_void , heapinformationlength : usize , returnlength : *mut usize ) -> super::super::Foundation:: BOOL );
    HeapQueryInformation(heaphandle.into_param().abi(), heapinformationclass, ::core::mem::transmute(heapinformation.unwrap_or(::std::ptr::null_mut())), heapinformationlength, ::core::mem::transmute(returnlength.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
#[inline]
pub unsafe fn HeapReAlloc<P0>(hheap: P0, dwflags: HEAP_FLAGS, lpmem: ::core::option::Option<*const ::core::ffi::c_void>, dwbytes: usize) -> *mut ::core::ffi::c_void
where
    P0: ::windows::core::IntoParam<HeapHandle>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn HeapReAlloc ( hheap : HeapHandle , dwflags : HEAP_FLAGS , lpmem : *const ::core::ffi::c_void , dwbytes : usize ) -> *mut ::core::ffi::c_void );
    HeapReAlloc(hheap.into_param().abi(), dwflags, ::core::mem::transmute(lpmem.unwrap_or(::std::ptr::null())), dwbytes)
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HeapSetInformation<P0>(heaphandle: P0, heapinformationclass: HEAP_INFORMATION_CLASS, heapinformation: ::core::option::Option<*const ::core::ffi::c_void>, heapinformationlength: usize) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<HeapHandle>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn HeapSetInformation ( heaphandle : HeapHandle , heapinformationclass : HEAP_INFORMATION_CLASS , heapinformation : *const ::core::ffi::c_void , heapinformationlength : usize ) -> super::super::Foundation:: BOOL );
    HeapSetInformation(heaphandle.into_param().abi(), heapinformationclass, ::core::mem::transmute(heapinformation.unwrap_or(::std::ptr::null())), heapinformationlength)
}
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
#[inline]
pub unsafe fn HeapSize<P0>(hheap: P0, dwflags: HEAP_FLAGS, lpmem: *const ::core::ffi::c_void) -> usize
where
    P0: ::windows::core::IntoParam<HeapHandle>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn HeapSize ( hheap : HeapHandle , dwflags : HEAP_FLAGS , lpmem : *const ::core::ffi::c_void ) -> usize );
    HeapSize(hheap.into_param().abi(), dwflags, lpmem)
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HeapSummary<P0>(hheap: P0, dwflags: u32, lpsummary: *mut HEAP_SUMMARY) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn HeapSummary ( hheap : super::super::Foundation:: HANDLE , dwflags : u32 , lpsummary : *mut HEAP_SUMMARY ) -> super::super::Foundation:: BOOL );
    HeapSummary(hheap.into_param().abi(), dwflags, lpsummary)
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HeapUnlock<P0>(hheap: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<HeapHandle>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn HeapUnlock ( hheap : HeapHandle ) -> super::super::Foundation:: BOOL );
    HeapUnlock(hheap.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HeapValidate<P0>(hheap: P0, dwflags: HEAP_FLAGS, lpmem: ::core::option::Option<*const ::core::ffi::c_void>) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<HeapHandle>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn HeapValidate ( hheap : HeapHandle , dwflags : HEAP_FLAGS , lpmem : *const ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    HeapValidate(hheap.into_param().abi(), dwflags, ::core::mem::transmute(lpmem.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HeapWalk<P0>(hheap: P0, lpentry: *mut PROCESS_HEAP_ENTRY) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<HeapHandle>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn HeapWalk ( hheap : HeapHandle , lpentry : *mut PROCESS_HEAP_ENTRY ) -> super::super::Foundation:: BOOL );
    HeapWalk(hheap.into_param().abi(), lpentry)
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsBadCodePtr(lpfn: super::super::Foundation::FARPROC) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "kernel32.dll""system" fn IsBadCodePtr ( lpfn : super::super::Foundation:: FARPROC ) -> super::super::Foundation:: BOOL );
    IsBadCodePtr(lpfn)
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsBadReadPtr(lp: ::core::option::Option<*const ::core::ffi::c_void>, ucb: usize) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "kernel32.dll""system" fn IsBadReadPtr ( lp : *const ::core::ffi::c_void , ucb : usize ) -> super::super::Foundation:: BOOL );
    IsBadReadPtr(::core::mem::transmute(lp.unwrap_or(::std::ptr::null())), ucb)
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsBadStringPtrA<P0>(lpsz: P0, ucchmax: usize) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn IsBadStringPtrA ( lpsz : ::windows::core::PCSTR , ucchmax : usize ) -> super::super::Foundation:: BOOL );
    IsBadStringPtrA(lpsz.into_param().abi(), ucchmax)
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsBadStringPtrW<P0>(lpsz: P0, ucchmax: usize) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn IsBadStringPtrW ( lpsz : ::windows::core::PCWSTR , ucchmax : usize ) -> super::super::Foundation:: BOOL );
    IsBadStringPtrW(lpsz.into_param().abi(), ucchmax)
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsBadWritePtr(lp: ::core::option::Option<*const ::core::ffi::c_void>, ucb: usize) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "kernel32.dll""system" fn IsBadWritePtr ( lp : *const ::core::ffi::c_void , ucb : usize ) -> super::super::Foundation:: BOOL );
    IsBadWritePtr(::core::mem::transmute(lp.unwrap_or(::std::ptr::null())), ucb)
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LocalAlloc(uflags: LOCAL_ALLOC_FLAGS, ubytes: usize) -> ::windows::core::Result<super::super::Foundation::HLOCAL> {
    ::windows_targets::link ! ( "kernel32.dll""system" fn LocalAlloc ( uflags : LOCAL_ALLOC_FLAGS , ubytes : usize ) -> super::super::Foundation:: HLOCAL );
    let result__ = LocalAlloc(uflags, ubytes);
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LocalFlags<P0>(hmem: P0) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HLOCAL>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn LocalFlags ( hmem : super::super::Foundation:: HLOCAL ) -> u32 );
    LocalFlags(hmem.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LocalFree<P0>(hmem: P0) -> ::windows::core::Result<super::super::Foundation::HLOCAL>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HLOCAL>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn LocalFree ( hmem : super::super::Foundation:: HLOCAL ) -> super::super::Foundation:: HLOCAL );
    let result__ = LocalFree(hmem.into_param().abi());
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LocalHandle(pmem: *const ::core::ffi::c_void) -> ::windows::core::Result<super::super::Foundation::HLOCAL> {
    ::windows_targets::link ! ( "kernel32.dll""system" fn LocalHandle ( pmem : *const ::core::ffi::c_void ) -> super::super::Foundation:: HLOCAL );
    let result__ = LocalHandle(pmem);
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LocalLock<P0>(hmem: P0) -> *mut ::core::ffi::c_void
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HLOCAL>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn LocalLock ( hmem : super::super::Foundation:: HLOCAL ) -> *mut ::core::ffi::c_void );
    LocalLock(hmem.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LocalReAlloc<P0>(hmem: P0, ubytes: usize, uflags: u32) -> ::windows::core::Result<super::super::Foundation::HLOCAL>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HLOCAL>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn LocalReAlloc ( hmem : super::super::Foundation:: HLOCAL , ubytes : usize , uflags : u32 ) -> super::super::Foundation:: HLOCAL );
    let result__ = LocalReAlloc(hmem.into_param().abi(), ubytes, uflags);
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LocalSize<P0>(hmem: P0) -> usize
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HLOCAL>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn LocalSize ( hmem : super::super::Foundation:: HLOCAL ) -> usize );
    LocalSize(hmem.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LocalUnlock<P0>(hmem: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HLOCAL>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn LocalUnlock ( hmem : super::super::Foundation:: HLOCAL ) -> super::super::Foundation:: BOOL );
    LocalUnlock(hmem.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MapUserPhysicalPages(virtualaddress: *const ::core::ffi::c_void, pagearray: ::core::option::Option<&[usize]>) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "kernel32.dll""system" fn MapUserPhysicalPages ( virtualaddress : *const ::core::ffi::c_void , numberofpages : usize , pagearray : *const usize ) -> super::super::Foundation:: BOOL );
    MapUserPhysicalPages(virtualaddress, pagearray.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(pagearray.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MapUserPhysicalPagesScatter(virtualaddresses: *const *const ::core::ffi::c_void, numberofpages: usize, pagearray: ::core::option::Option<*const usize>) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "kernel32.dll""system" fn MapUserPhysicalPagesScatter ( virtualaddresses : *const *const ::core::ffi::c_void , numberofpages : usize , pagearray : *const usize ) -> super::super::Foundation:: BOOL );
    MapUserPhysicalPagesScatter(virtualaddresses, numberofpages, ::core::mem::transmute(pagearray.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MapViewOfFile<P0>(hfilemappingobject: P0, dwdesiredaccess: FILE_MAP, dwfileoffsethigh: u32, dwfileoffsetlow: u32, dwnumberofbytestomap: usize) -> ::windows::core::Result<MEMORYMAPPEDVIEW_HANDLE>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn MapViewOfFile ( hfilemappingobject : super::super::Foundation:: HANDLE , dwdesiredaccess : FILE_MAP , dwfileoffsethigh : u32 , dwfileoffsetlow : u32 , dwnumberofbytestomap : usize ) -> MEMORYMAPPEDVIEW_HANDLE );
    let result__ = MapViewOfFile(hfilemappingobject.into_param().abi(), dwdesiredaccess, dwfileoffsethigh, dwfileoffsetlow, dwnumberofbytestomap);
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MapViewOfFile3<P0, P1>(filemapping: P0, process: P1, baseaddress: ::core::option::Option<*const ::core::ffi::c_void>, offset: u64, viewsize: usize, allocationtype: VIRTUAL_ALLOCATION_TYPE, pageprotection: u32, extendedparameters: ::core::option::Option<&mut [MEM_EXTENDED_PARAMETER]>) -> ::windows::core::Result<MEMORYMAPPEDVIEW_HANDLE>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "api-ms-win-core-memory-l1-1-6.dll""system" fn MapViewOfFile3 ( filemapping : super::super::Foundation:: HANDLE , process : super::super::Foundation:: HANDLE , baseaddress : *const ::core::ffi::c_void , offset : u64 , viewsize : usize , allocationtype : VIRTUAL_ALLOCATION_TYPE , pageprotection : u32 , extendedparameters : *mut MEM_EXTENDED_PARAMETER , parametercount : u32 ) -> MEMORYMAPPEDVIEW_HANDLE );
    let result__ = MapViewOfFile3(filemapping.into_param().abi(), process.into_param().abi(), ::core::mem::transmute(baseaddress.unwrap_or(::std::ptr::null())), offset, viewsize, allocationtype, pageprotection, ::core::mem::transmute(extendedparameters.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), extendedparameters.as_deref().map_or(0, |slice| slice.len() as _));
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MapViewOfFile3FromApp<P0, P1>(filemapping: P0, process: P1, baseaddress: ::core::option::Option<*const ::core::ffi::c_void>, offset: u64, viewsize: usize, allocationtype: VIRTUAL_ALLOCATION_TYPE, pageprotection: u32, extendedparameters: ::core::option::Option<&mut [MEM_EXTENDED_PARAMETER]>) -> ::windows::core::Result<MEMORYMAPPEDVIEW_HANDLE>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "api-ms-win-core-memory-l1-1-6.dll""system" fn MapViewOfFile3FromApp ( filemapping : super::super::Foundation:: HANDLE , process : super::super::Foundation:: HANDLE , baseaddress : *const ::core::ffi::c_void , offset : u64 , viewsize : usize , allocationtype : VIRTUAL_ALLOCATION_TYPE , pageprotection : u32 , extendedparameters : *mut MEM_EXTENDED_PARAMETER , parametercount : u32 ) -> MEMORYMAPPEDVIEW_HANDLE );
    let result__ = MapViewOfFile3FromApp(filemapping.into_param().abi(), process.into_param().abi(), ::core::mem::transmute(baseaddress.unwrap_or(::std::ptr::null())), offset, viewsize, allocationtype, pageprotection, ::core::mem::transmute(extendedparameters.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), extendedparameters.as_deref().map_or(0, |slice| slice.len() as _));
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MapViewOfFileEx<P0>(hfilemappingobject: P0, dwdesiredaccess: FILE_MAP, dwfileoffsethigh: u32, dwfileoffsetlow: u32, dwnumberofbytestomap: usize, lpbaseaddress: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<MEMORYMAPPEDVIEW_HANDLE>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn MapViewOfFileEx ( hfilemappingobject : super::super::Foundation:: HANDLE , dwdesiredaccess : FILE_MAP , dwfileoffsethigh : u32 , dwfileoffsetlow : u32 , dwnumberofbytestomap : usize , lpbaseaddress : *const ::core::ffi::c_void ) -> MEMORYMAPPEDVIEW_HANDLE );
    let result__ = MapViewOfFileEx(hfilemappingobject.into_param().abi(), dwdesiredaccess, dwfileoffsethigh, dwfileoffsetlow, dwnumberofbytestomap, ::core::mem::transmute(lpbaseaddress.unwrap_or(::std::ptr::null())));
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MapViewOfFileExNuma<P0>(hfilemappingobject: P0, dwdesiredaccess: FILE_MAP, dwfileoffsethigh: u32, dwfileoffsetlow: u32, dwnumberofbytestomap: usize, lpbaseaddress: ::core::option::Option<*const ::core::ffi::c_void>, nndpreferred: u32) -> ::windows::core::Result<MEMORYMAPPEDVIEW_HANDLE>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn MapViewOfFileExNuma ( hfilemappingobject : super::super::Foundation:: HANDLE , dwdesiredaccess : FILE_MAP , dwfileoffsethigh : u32 , dwfileoffsetlow : u32 , dwnumberofbytestomap : usize , lpbaseaddress : *const ::core::ffi::c_void , nndpreferred : u32 ) -> MEMORYMAPPEDVIEW_HANDLE );
    let result__ = MapViewOfFileExNuma(hfilemappingobject.into_param().abi(), dwdesiredaccess, dwfileoffsethigh, dwfileoffsetlow, dwnumberofbytestomap, ::core::mem::transmute(lpbaseaddress.unwrap_or(::std::ptr::null())), nndpreferred);
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MapViewOfFileFromApp<P0>(hfilemappingobject: P0, desiredaccess: FILE_MAP, fileoffset: u64, numberofbytestomap: usize) -> ::windows::core::Result<MEMORYMAPPEDVIEW_HANDLE>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn MapViewOfFileFromApp ( hfilemappingobject : super::super::Foundation:: HANDLE , desiredaccess : FILE_MAP , fileoffset : u64 , numberofbytestomap : usize ) -> MEMORYMAPPEDVIEW_HANDLE );
    let result__ = MapViewOfFileFromApp(hfilemappingobject.into_param().abi(), desiredaccess, fileoffset, numberofbytestomap);
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MapViewOfFileNuma2<P0, P1>(filemappinghandle: P0, processhandle: P1, offset: u64, baseaddress: ::core::option::Option<*const ::core::ffi::c_void>, viewsize: usize, allocationtype: u32, pageprotection: u32, preferrednode: u32) -> ::windows::core::Result<MEMORYMAPPEDVIEW_HANDLE>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "api-ms-win-core-memory-l1-1-5.dll""system" fn MapViewOfFileNuma2 ( filemappinghandle : super::super::Foundation:: HANDLE , processhandle : super::super::Foundation:: HANDLE , offset : u64 , baseaddress : *const ::core::ffi::c_void , viewsize : usize , allocationtype : u32 , pageprotection : u32 , preferrednode : u32 ) -> MEMORYMAPPEDVIEW_HANDLE );
    let result__ = MapViewOfFileNuma2(filemappinghandle.into_param().abi(), processhandle.into_param().abi(), offset, ::core::mem::transmute(baseaddress.unwrap_or(::std::ptr::null())), viewsize, allocationtype, pageprotection, preferrednode);
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
#[inline]
pub unsafe fn OfferVirtualMemory(virtualaddress: &mut [u8], priority: OFFER_PRIORITY) -> u32 {
    ::windows_targets::link ! ( "kernel32.dll""system" fn OfferVirtualMemory ( virtualaddress : *mut ::core::ffi::c_void , size : usize , priority : OFFER_PRIORITY ) -> u32 );
    OfferVirtualMemory(::core::mem::transmute(virtualaddress.as_ptr()), virtualaddress.len() as _, priority)
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenDedicatedMemoryPartition<P0, P1>(partition: P0, dedicatedmemorytypeid: u64, desiredaccess: u32, inherithandle: P1) -> super::super::Foundation::HANDLE
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "api-ms-win-core-memory-l1-1-8.dll""system" fn OpenDedicatedMemoryPartition ( partition : super::super::Foundation:: HANDLE , dedicatedmemorytypeid : u64 , desiredaccess : u32 , inherithandle : super::super::Foundation:: BOOL ) -> super::super::Foundation:: HANDLE );
    OpenDedicatedMemoryPartition(partition.into_param().abi(), dedicatedmemorytypeid, desiredaccess, inherithandle.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenFileMappingA<P0, P1>(dwdesiredaccess: u32, binherithandle: P0, lpname: P1) -> ::windows::core::Result<super::super::Foundation::HANDLE>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn OpenFileMappingA ( dwdesiredaccess : u32 , binherithandle : super::super::Foundation:: BOOL , lpname : ::windows::core::PCSTR ) -> super::super::Foundation:: HANDLE );
    let result__ = OpenFileMappingA(dwdesiredaccess, binherithandle.into_param().abi(), lpname.into_param().abi());
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenFileMappingFromApp<P0, P1>(desiredaccess: u32, inherithandle: P0, name: P1) -> ::windows::core::Result<super::super::Foundation::HANDLE>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "api-ms-win-core-memory-l1-1-3.dll""system" fn OpenFileMappingFromApp ( desiredaccess : u32 , inherithandle : super::super::Foundation:: BOOL , name : ::windows::core::PCWSTR ) -> super::super::Foundation:: HANDLE );
    let result__ = OpenFileMappingFromApp(desiredaccess, inherithandle.into_param().abi(), name.into_param().abi());
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenFileMappingW<P0, P1>(dwdesiredaccess: u32, binherithandle: P0, lpname: P1) -> ::windows::core::Result<super::super::Foundation::HANDLE>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn OpenFileMappingW ( dwdesiredaccess : u32 , binherithandle : super::super::Foundation:: BOOL , lpname : ::windows::core::PCWSTR ) -> super::super::Foundation:: HANDLE );
    let result__ = OpenFileMappingW(dwdesiredaccess, binherithandle.into_param().abi(), lpname.into_param().abi());
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PrefetchVirtualMemory<P0>(hprocess: P0, virtualaddresses: &[WIN32_MEMORY_RANGE_ENTRY], flags: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn PrefetchVirtualMemory ( hprocess : super::super::Foundation:: HANDLE , numberofentries : usize , virtualaddresses : *const WIN32_MEMORY_RANGE_ENTRY , flags : u32 ) -> super::super::Foundation:: BOOL );
    PrefetchVirtualMemory(hprocess.into_param().abi(), virtualaddresses.len() as _, ::core::mem::transmute(virtualaddresses.as_ptr()), flags)
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn QueryMemoryResourceNotification<P0>(resourcenotificationhandle: P0, resourcestate: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn QueryMemoryResourceNotification ( resourcenotificationhandle : super::super::Foundation:: HANDLE , resourcestate : *mut super::super::Foundation:: BOOL ) -> super::super::Foundation:: BOOL );
    QueryMemoryResourceNotification(resourcenotificationhandle.into_param().abi(), resourcestate)
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn QueryPartitionInformation<P0>(partition: P0, partitioninformationclass: WIN32_MEMORY_PARTITION_INFORMATION_CLASS, partitioninformation: *mut ::core::ffi::c_void, partitioninformationlength: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "api-ms-win-core-memory-l1-1-8.dll""system" fn QueryPartitionInformation ( partition : super::super::Foundation:: HANDLE , partitioninformationclass : WIN32_MEMORY_PARTITION_INFORMATION_CLASS , partitioninformation : *mut ::core::ffi::c_void , partitioninformationlength : u32 ) -> super::super::Foundation:: BOOL );
    QueryPartitionInformation(partition.into_param().abi(), partitioninformationclass, partitioninformation, partitioninformationlength)
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn QueryVirtualMemoryInformation<P0>(process: P0, virtualaddress: *const ::core::ffi::c_void, memoryinformationclass: WIN32_MEMORY_INFORMATION_CLASS, memoryinformation: *mut ::core::ffi::c_void, memoryinformationsize: usize, returnsize: ::core::option::Option<*mut usize>) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "api-ms-win-core-memory-l1-1-4.dll""system" fn QueryVirtualMemoryInformation ( process : super::super::Foundation:: HANDLE , virtualaddress : *const ::core::ffi::c_void , memoryinformationclass : WIN32_MEMORY_INFORMATION_CLASS , memoryinformation : *mut ::core::ffi::c_void , memoryinformationsize : usize , returnsize : *mut usize ) -> super::super::Foundation:: BOOL );
    QueryVirtualMemoryInformation(process.into_param().abi(), virtualaddress, memoryinformationclass, memoryinformation, memoryinformationsize, ::core::mem::transmute(returnsize.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
#[inline]
pub unsafe fn ReclaimVirtualMemory(virtualaddress: &[u8]) -> u32 {
    ::windows_targets::link ! ( "kernel32.dll""system" fn ReclaimVirtualMemory ( virtualaddress : *const ::core::ffi::c_void , size : usize ) -> u32 );
    ReclaimVirtualMemory(::core::mem::transmute(virtualaddress.as_ptr()), virtualaddress.len() as _)
}
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
#[inline]
pub unsafe fn RegisterBadMemoryNotification(callback: PBAD_MEMORY_CALLBACK_ROUTINE) -> *mut ::core::ffi::c_void {
    ::windows_targets::link ! ( "kernel32.dll""system" fn RegisterBadMemoryNotification ( callback : PBAD_MEMORY_CALLBACK_ROUTINE ) -> *mut ::core::ffi::c_void );
    RegisterBadMemoryNotification(callback)
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RemoveSecureMemoryCacheCallback(pfncallback: PSECURE_MEMORY_CACHE_CALLBACK) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "kernel32.dll""system" fn RemoveSecureMemoryCacheCallback ( pfncallback : PSECURE_MEMORY_CACHE_CALLBACK ) -> super::super::Foundation:: BOOL );
    RemoveSecureMemoryCacheCallback(pfncallback)
}
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
#[inline]
pub unsafe fn ResetWriteWatch(lpbaseaddress: *const ::core::ffi::c_void, dwregionsize: usize) -> u32 {
    ::windows_targets::link ! ( "kernel32.dll""system" fn ResetWriteWatch ( lpbaseaddress : *const ::core::ffi::c_void , dwregionsize : usize ) -> u32 );
    ResetWriteWatch(lpbaseaddress, dwregionsize)
}
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
#[inline]
pub unsafe fn RtlCompareMemory(source1: *const ::core::ffi::c_void, source2: *const ::core::ffi::c_void, length: usize) -> usize {
    ::windows_targets::link ! ( "kernel32.dll""system" fn RtlCompareMemory ( source1 : *const ::core::ffi::c_void , source2 : *const ::core::ffi::c_void , length : usize ) -> usize );
    RtlCompareMemory(source1, source2, length)
}
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
#[inline]
pub unsafe fn RtlCrc32(buffer: *const ::core::ffi::c_void, size: usize, initialcrc: u32) -> u32 {
    ::windows_targets::link ! ( "ntdll.dll""system" fn RtlCrc32 ( buffer : *const ::core::ffi::c_void , size : usize , initialcrc : u32 ) -> u32 );
    RtlCrc32(buffer, size, initialcrc)
}
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
#[inline]
pub unsafe fn RtlCrc64(buffer: *const ::core::ffi::c_void, size: usize, initialcrc: u64) -> u64 {
    ::windows_targets::link ! ( "ntdll.dll""system" fn RtlCrc64 ( buffer : *const ::core::ffi::c_void , size : usize , initialcrc : u64 ) -> u64 );
    RtlCrc64(buffer, size, initialcrc)
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RtlIsZeroMemory(buffer: *const ::core::ffi::c_void, length: usize) -> super::super::Foundation::BOOLEAN {
    ::windows_targets::link ! ( "ntdll.dll""system" fn RtlIsZeroMemory ( buffer : *const ::core::ffi::c_void , length : usize ) -> super::super::Foundation:: BOOLEAN );
    RtlIsZeroMemory(buffer, length)
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetProcessValidCallTargets<P0>(hprocess: P0, virtualaddress: *const ::core::ffi::c_void, regionsize: usize, offsetinformation: &mut [CFG_CALL_TARGET_INFO]) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "api-ms-win-core-memory-l1-1-3.dll""system" fn SetProcessValidCallTargets ( hprocess : super::super::Foundation:: HANDLE , virtualaddress : *const ::core::ffi::c_void , regionsize : usize , numberofoffsets : u32 , offsetinformation : *mut CFG_CALL_TARGET_INFO ) -> super::super::Foundation:: BOOL );
    SetProcessValidCallTargets(hprocess.into_param().abi(), virtualaddress, regionsize, offsetinformation.len() as _, ::core::mem::transmute(offsetinformation.as_ptr()))
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetProcessValidCallTargetsForMappedView<P0, P1>(process: P0, virtualaddress: *const ::core::ffi::c_void, regionsize: usize, offsetinformation: &mut [CFG_CALL_TARGET_INFO], section: P1, expectedfileoffset: u64) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "api-ms-win-core-memory-l1-1-7.dll""system" fn SetProcessValidCallTargetsForMappedView ( process : super::super::Foundation:: HANDLE , virtualaddress : *const ::core::ffi::c_void , regionsize : usize , numberofoffsets : u32 , offsetinformation : *mut CFG_CALL_TARGET_INFO , section : super::super::Foundation:: HANDLE , expectedfileoffset : u64 ) -> super::super::Foundation:: BOOL );
    SetProcessValidCallTargetsForMappedView(process.into_param().abi(), virtualaddress, regionsize, offsetinformation.len() as _, ::core::mem::transmute(offsetinformation.as_ptr()), section.into_param().abi(), expectedfileoffset)
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetProcessWorkingSetSizeEx<P0>(hprocess: P0, dwminimumworkingsetsize: usize, dwmaximumworkingsetsize: usize, flags: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn SetProcessWorkingSetSizeEx ( hprocess : super::super::Foundation:: HANDLE , dwminimumworkingsetsize : usize , dwmaximumworkingsetsize : usize , flags : u32 ) -> super::super::Foundation:: BOOL );
    SetProcessWorkingSetSizeEx(hprocess.into_param().abi(), dwminimumworkingsetsize, dwmaximumworkingsetsize, flags)
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetSystemFileCacheSize(minimumfilecachesize: usize, maximumfilecachesize: usize, flags: u32) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "kernel32.dll""system" fn SetSystemFileCacheSize ( minimumfilecachesize : usize , maximumfilecachesize : usize , flags : u32 ) -> super::super::Foundation:: BOOL );
    SetSystemFileCacheSize(minimumfilecachesize, maximumfilecachesize, flags)
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UnmapViewOfFile<P0>(lpbaseaddress: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<MEMORYMAPPEDVIEW_HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn UnmapViewOfFile ( lpbaseaddress : MEMORYMAPPEDVIEW_HANDLE ) -> super::super::Foundation:: BOOL );
    UnmapViewOfFile(lpbaseaddress.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UnmapViewOfFile2<P0, P1>(process: P0, baseaddress: P1, unmapflags: UNMAP_VIEW_OF_FILE_FLAGS) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<MEMORYMAPPEDVIEW_HANDLE>,
{
    ::windows_targets::link ! ( "api-ms-win-core-memory-l1-1-5.dll""system" fn UnmapViewOfFile2 ( process : super::super::Foundation:: HANDLE , baseaddress : MEMORYMAPPEDVIEW_HANDLE , unmapflags : UNMAP_VIEW_OF_FILE_FLAGS ) -> super::super::Foundation:: BOOL );
    UnmapViewOfFile2(process.into_param().abi(), baseaddress.into_param().abi(), unmapflags)
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UnmapViewOfFileEx<P0>(baseaddress: P0, unmapflags: UNMAP_VIEW_OF_FILE_FLAGS) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<MEMORYMAPPEDVIEW_HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn UnmapViewOfFileEx ( baseaddress : MEMORYMAPPEDVIEW_HANDLE , unmapflags : UNMAP_VIEW_OF_FILE_FLAGS ) -> super::super::Foundation:: BOOL );
    UnmapViewOfFileEx(baseaddress.into_param().abi(), unmapflags)
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UnregisterBadMemoryNotification(registrationhandle: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "kernel32.dll""system" fn UnregisterBadMemoryNotification ( registrationhandle : *const ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    UnregisterBadMemoryNotification(registrationhandle)
}
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
#[inline]
pub unsafe fn VirtualAlloc(lpaddress: ::core::option::Option<*const ::core::ffi::c_void>, dwsize: usize, flallocationtype: VIRTUAL_ALLOCATION_TYPE, flprotect: PAGE_PROTECTION_FLAGS) -> *mut ::core::ffi::c_void {
    ::windows_targets::link ! ( "kernel32.dll""system" fn VirtualAlloc ( lpaddress : *const ::core::ffi::c_void , dwsize : usize , flallocationtype : VIRTUAL_ALLOCATION_TYPE , flprotect : PAGE_PROTECTION_FLAGS ) -> *mut ::core::ffi::c_void );
    VirtualAlloc(::core::mem::transmute(lpaddress.unwrap_or(::std::ptr::null())), dwsize, flallocationtype, flprotect)
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VirtualAlloc2<P0>(process: P0, baseaddress: ::core::option::Option<*const ::core::ffi::c_void>, size: usize, allocationtype: VIRTUAL_ALLOCATION_TYPE, pageprotection: u32, extendedparameters: ::core::option::Option<&mut [MEM_EXTENDED_PARAMETER]>) -> *mut ::core::ffi::c_void
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "api-ms-win-core-memory-l1-1-6.dll""system" fn VirtualAlloc2 ( process : super::super::Foundation:: HANDLE , baseaddress : *const ::core::ffi::c_void , size : usize , allocationtype : VIRTUAL_ALLOCATION_TYPE , pageprotection : u32 , extendedparameters : *mut MEM_EXTENDED_PARAMETER , parametercount : u32 ) -> *mut ::core::ffi::c_void );
    VirtualAlloc2(process.into_param().abi(), ::core::mem::transmute(baseaddress.unwrap_or(::std::ptr::null())), size, allocationtype, pageprotection, ::core::mem::transmute(extendedparameters.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), extendedparameters.as_deref().map_or(0, |slice| slice.len() as _))
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VirtualAlloc2FromApp<P0>(process: P0, baseaddress: ::core::option::Option<*const ::core::ffi::c_void>, size: usize, allocationtype: VIRTUAL_ALLOCATION_TYPE, pageprotection: u32, extendedparameters: ::core::option::Option<&mut [MEM_EXTENDED_PARAMETER]>) -> *mut ::core::ffi::c_void
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "api-ms-win-core-memory-l1-1-6.dll""system" fn VirtualAlloc2FromApp ( process : super::super::Foundation:: HANDLE , baseaddress : *const ::core::ffi::c_void , size : usize , allocationtype : VIRTUAL_ALLOCATION_TYPE , pageprotection : u32 , extendedparameters : *mut MEM_EXTENDED_PARAMETER , parametercount : u32 ) -> *mut ::core::ffi::c_void );
    VirtualAlloc2FromApp(process.into_param().abi(), ::core::mem::transmute(baseaddress.unwrap_or(::std::ptr::null())), size, allocationtype, pageprotection, ::core::mem::transmute(extendedparameters.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), extendedparameters.as_deref().map_or(0, |slice| slice.len() as _))
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VirtualAllocEx<P0>(hprocess: P0, lpaddress: ::core::option::Option<*const ::core::ffi::c_void>, dwsize: usize, flallocationtype: VIRTUAL_ALLOCATION_TYPE, flprotect: PAGE_PROTECTION_FLAGS) -> *mut ::core::ffi::c_void
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn VirtualAllocEx ( hprocess : super::super::Foundation:: HANDLE , lpaddress : *const ::core::ffi::c_void , dwsize : usize , flallocationtype : VIRTUAL_ALLOCATION_TYPE , flprotect : PAGE_PROTECTION_FLAGS ) -> *mut ::core::ffi::c_void );
    VirtualAllocEx(hprocess.into_param().abi(), ::core::mem::transmute(lpaddress.unwrap_or(::std::ptr::null())), dwsize, flallocationtype, flprotect)
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VirtualAllocExNuma<P0>(hprocess: P0, lpaddress: ::core::option::Option<*const ::core::ffi::c_void>, dwsize: usize, flallocationtype: VIRTUAL_ALLOCATION_TYPE, flprotect: u32, nndpreferred: u32) -> *mut ::core::ffi::c_void
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn VirtualAllocExNuma ( hprocess : super::super::Foundation:: HANDLE , lpaddress : *const ::core::ffi::c_void , dwsize : usize , flallocationtype : VIRTUAL_ALLOCATION_TYPE , flprotect : u32 , nndpreferred : u32 ) -> *mut ::core::ffi::c_void );
    VirtualAllocExNuma(hprocess.into_param().abi(), ::core::mem::transmute(lpaddress.unwrap_or(::std::ptr::null())), dwsize, flallocationtype, flprotect, nndpreferred)
}
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
#[inline]
pub unsafe fn VirtualAllocFromApp(baseaddress: ::core::option::Option<*const ::core::ffi::c_void>, size: usize, allocationtype: VIRTUAL_ALLOCATION_TYPE, protection: u32) -> *mut ::core::ffi::c_void {
    ::windows_targets::link ! ( "api-ms-win-core-memory-l1-1-3.dll""system" fn VirtualAllocFromApp ( baseaddress : *const ::core::ffi::c_void , size : usize , allocationtype : VIRTUAL_ALLOCATION_TYPE , protection : u32 ) -> *mut ::core::ffi::c_void );
    VirtualAllocFromApp(::core::mem::transmute(baseaddress.unwrap_or(::std::ptr::null())), size, allocationtype, protection)
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VirtualFree(lpaddress: *mut ::core::ffi::c_void, dwsize: usize, dwfreetype: VIRTUAL_FREE_TYPE) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "kernel32.dll""system" fn VirtualFree ( lpaddress : *mut ::core::ffi::c_void , dwsize : usize , dwfreetype : VIRTUAL_FREE_TYPE ) -> super::super::Foundation:: BOOL );
    VirtualFree(lpaddress, dwsize, dwfreetype)
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VirtualFreeEx<P0>(hprocess: P0, lpaddress: *mut ::core::ffi::c_void, dwsize: usize, dwfreetype: VIRTUAL_FREE_TYPE) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn VirtualFreeEx ( hprocess : super::super::Foundation:: HANDLE , lpaddress : *mut ::core::ffi::c_void , dwsize : usize , dwfreetype : VIRTUAL_FREE_TYPE ) -> super::super::Foundation:: BOOL );
    VirtualFreeEx(hprocess.into_param().abi(), lpaddress, dwsize, dwfreetype)
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VirtualLock(lpaddress: *const ::core::ffi::c_void, dwsize: usize) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "kernel32.dll""system" fn VirtualLock ( lpaddress : *const ::core::ffi::c_void , dwsize : usize ) -> super::super::Foundation:: BOOL );
    VirtualLock(lpaddress, dwsize)
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VirtualProtect(lpaddress: *const ::core::ffi::c_void, dwsize: usize, flnewprotect: PAGE_PROTECTION_FLAGS, lpfloldprotect: *mut PAGE_PROTECTION_FLAGS) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "kernel32.dll""system" fn VirtualProtect ( lpaddress : *const ::core::ffi::c_void , dwsize : usize , flnewprotect : PAGE_PROTECTION_FLAGS , lpfloldprotect : *mut PAGE_PROTECTION_FLAGS ) -> super::super::Foundation:: BOOL );
    VirtualProtect(lpaddress, dwsize, flnewprotect, lpfloldprotect)
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VirtualProtectEx<P0>(hprocess: P0, lpaddress: *const ::core::ffi::c_void, dwsize: usize, flnewprotect: PAGE_PROTECTION_FLAGS, lpfloldprotect: *mut PAGE_PROTECTION_FLAGS) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn VirtualProtectEx ( hprocess : super::super::Foundation:: HANDLE , lpaddress : *const ::core::ffi::c_void , dwsize : usize , flnewprotect : PAGE_PROTECTION_FLAGS , lpfloldprotect : *mut PAGE_PROTECTION_FLAGS ) -> super::super::Foundation:: BOOL );
    VirtualProtectEx(hprocess.into_param().abi(), lpaddress, dwsize, flnewprotect, lpfloldprotect)
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VirtualProtectFromApp(address: *const ::core::ffi::c_void, size: usize, newprotection: u32, oldprotection: *mut u32) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "api-ms-win-core-memory-l1-1-3.dll""system" fn VirtualProtectFromApp ( address : *const ::core::ffi::c_void , size : usize , newprotection : u32 , oldprotection : *mut u32 ) -> super::super::Foundation:: BOOL );
    VirtualProtectFromApp(address, size, newprotection, oldprotection)
}
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
#[inline]
pub unsafe fn VirtualQuery(lpaddress: ::core::option::Option<*const ::core::ffi::c_void>, lpbuffer: *mut MEMORY_BASIC_INFORMATION, dwlength: usize) -> usize {
    ::windows_targets::link ! ( "kernel32.dll""system" fn VirtualQuery ( lpaddress : *const ::core::ffi::c_void , lpbuffer : *mut MEMORY_BASIC_INFORMATION , dwlength : usize ) -> usize );
    VirtualQuery(::core::mem::transmute(lpaddress.unwrap_or(::std::ptr::null())), lpbuffer, dwlength)
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VirtualQueryEx<P0>(hprocess: P0, lpaddress: ::core::option::Option<*const ::core::ffi::c_void>, lpbuffer: *mut MEMORY_BASIC_INFORMATION, dwlength: usize) -> usize
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn VirtualQueryEx ( hprocess : super::super::Foundation:: HANDLE , lpaddress : *const ::core::ffi::c_void , lpbuffer : *mut MEMORY_BASIC_INFORMATION , dwlength : usize ) -> usize );
    VirtualQueryEx(hprocess.into_param().abi(), ::core::mem::transmute(lpaddress.unwrap_or(::std::ptr::null())), lpbuffer, dwlength)
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VirtualUnlock(lpaddress: *const ::core::ffi::c_void, dwsize: usize) -> super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "kernel32.dll""system" fn VirtualUnlock ( lpaddress : *const ::core::ffi::c_void , dwsize : usize ) -> super::super::Foundation:: BOOL );
    VirtualUnlock(lpaddress, dwsize)
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VirtualUnlockEx<P0>(process: P0, address: *const ::core::ffi::c_void, size: usize) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "api-ms-win-core-memory-l1-1-5.dll""system" fn VirtualUnlockEx ( process : super::super::Foundation:: HANDLE , address : *const ::core::ffi::c_void , size : usize ) -> super::super::Foundation:: BOOL );
    VirtualUnlockEx(process.into_param().abi(), address, size)
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
pub const MEHC_PATROL_SCRUBBER_PRESENT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::windows::core::TypeKind for FILE_MAP {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for FILE_MAP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FILE_MAP").field(&self.0).finish()
    }
}
impl FILE_MAP {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
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
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::windows::core::TypeKind for GLOBAL_ALLOC_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for GLOBAL_ALLOC_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GLOBAL_ALLOC_FLAGS").field(&self.0).finish()
    }
}
impl GLOBAL_ALLOC_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
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
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::windows::core::TypeKind for HEAP_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for HEAP_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HEAP_FLAGS").field(&self.0).finish()
    }
}
impl HEAP_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
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
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::windows::core::TypeKind for HEAP_INFORMATION_CLASS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for HEAP_INFORMATION_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HEAP_INFORMATION_CLASS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::windows::core::TypeKind for LOCAL_ALLOC_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for LOCAL_ALLOC_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LOCAL_ALLOC_FLAGS").field(&self.0).finish()
    }
}
impl LOCAL_ALLOC_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
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
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::windows::core::TypeKind for MEMORY_RESOURCE_NOTIFICATION_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MEMORY_RESOURCE_NOTIFICATION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MEMORY_RESOURCE_NOTIFICATION_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MEM_DEDICATED_ATTRIBUTE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const MemDedicatedAttributeReadBandwidth: MEM_DEDICATED_ATTRIBUTE_TYPE = MEM_DEDICATED_ATTRIBUTE_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const MemDedicatedAttributeReadLatency: MEM_DEDICATED_ATTRIBUTE_TYPE = MEM_DEDICATED_ATTRIBUTE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const MemDedicatedAttributeWriteBandwidth: MEM_DEDICATED_ATTRIBUTE_TYPE = MEM_DEDICATED_ATTRIBUTE_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const MemDedicatedAttributeWriteLatency: MEM_DEDICATED_ATTRIBUTE_TYPE = MEM_DEDICATED_ATTRIBUTE_TYPE(3i32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const MemDedicatedAttributeMax: MEM_DEDICATED_ATTRIBUTE_TYPE = MEM_DEDICATED_ATTRIBUTE_TYPE(4i32);
impl ::core::marker::Copy for MEM_DEDICATED_ATTRIBUTE_TYPE {}
impl ::core::clone::Clone for MEM_DEDICATED_ATTRIBUTE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MEM_DEDICATED_ATTRIBUTE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MEM_DEDICATED_ATTRIBUTE_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MEM_DEDICATED_ATTRIBUTE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MEM_DEDICATED_ATTRIBUTE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::windows::core::TypeKind for MEM_EXTENDED_PARAMETER_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MEM_EXTENDED_PARAMETER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MEM_EXTENDED_PARAMETER_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MEM_SECTION_EXTENDED_PARAMETER_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const MemSectionExtendedParameterInvalidType: MEM_SECTION_EXTENDED_PARAMETER_TYPE = MEM_SECTION_EXTENDED_PARAMETER_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const MemSectionExtendedParameterUserPhysicalFlags: MEM_SECTION_EXTENDED_PARAMETER_TYPE = MEM_SECTION_EXTENDED_PARAMETER_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const MemSectionExtendedParameterNumaNode: MEM_SECTION_EXTENDED_PARAMETER_TYPE = MEM_SECTION_EXTENDED_PARAMETER_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const MemSectionExtendedParameterSigningLevel: MEM_SECTION_EXTENDED_PARAMETER_TYPE = MEM_SECTION_EXTENDED_PARAMETER_TYPE(3i32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const MemSectionExtendedParameterMax: MEM_SECTION_EXTENDED_PARAMETER_TYPE = MEM_SECTION_EXTENDED_PARAMETER_TYPE(4i32);
impl ::core::marker::Copy for MEM_SECTION_EXTENDED_PARAMETER_TYPE {}
impl ::core::clone::Clone for MEM_SECTION_EXTENDED_PARAMETER_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MEM_SECTION_EXTENDED_PARAMETER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MEM_SECTION_EXTENDED_PARAMETER_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MEM_SECTION_EXTENDED_PARAMETER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MEM_SECTION_EXTENDED_PARAMETER_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::windows::core::TypeKind for OFFER_PRIORITY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for OFFER_PRIORITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OFFER_PRIORITY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::windows::core::TypeKind for PAGE_PROTECTION_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PAGE_PROTECTION_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PAGE_PROTECTION_FLAGS").field(&self.0).finish()
    }
}
impl PAGE_PROTECTION_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
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
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::windows::core::TypeKind for PAGE_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PAGE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PAGE_TYPE").field(&self.0).finish()
    }
}
impl PAGE_TYPE {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
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
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SECTION_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const SECTION_ALL_ACCESS: SECTION_FLAGS = SECTION_FLAGS(983071u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const SECTION_QUERY: SECTION_FLAGS = SECTION_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const SECTION_MAP_WRITE: SECTION_FLAGS = SECTION_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const SECTION_MAP_READ: SECTION_FLAGS = SECTION_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const SECTION_MAP_EXECUTE: SECTION_FLAGS = SECTION_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const SECTION_EXTEND_SIZE: SECTION_FLAGS = SECTION_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const SECTION_MAP_EXECUTE_EXPLICIT: SECTION_FLAGS = SECTION_FLAGS(32u32);
impl ::core::marker::Copy for SECTION_FLAGS {}
impl ::core::clone::Clone for SECTION_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SECTION_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SECTION_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SECTION_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SECTION_FLAGS").field(&self.0).finish()
    }
}
impl SECTION_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for SECTION_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for SECTION_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for SECTION_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for SECTION_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for SECTION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::windows::core::TypeKind for UNMAP_VIEW_OF_FILE_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for UNMAP_VIEW_OF_FILE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UNMAP_VIEW_OF_FILE_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::windows::core::TypeKind for VIRTUAL_ALLOCATION_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for VIRTUAL_ALLOCATION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VIRTUAL_ALLOCATION_TYPE").field(&self.0).finish()
    }
}
impl VIRTUAL_ALLOCATION_TYPE {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
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
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::windows::core::TypeKind for VIRTUAL_FREE_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for VIRTUAL_FREE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VIRTUAL_FREE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::windows::core::TypeKind for WIN32_MEMORY_INFORMATION_CLASS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WIN32_MEMORY_INFORMATION_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WIN32_MEMORY_INFORMATION_CLASS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::windows::core::TypeKind for WIN32_MEMORY_PARTITION_INFORMATION_CLASS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WIN32_MEMORY_PARTITION_INFORMATION_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WIN32_MEMORY_PARTITION_INFORMATION_CLASS").field(&self.0).finish()
    }
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
impl ::windows::core::TypeKind for CFG_CALL_TARGET_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for CFG_CALL_TARGET_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Offset == other.Offset && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for CFG_CALL_TARGET_INFO {}
impl ::core::default::Default for CFG_CALL_TARGET_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::windows::core::TypeKind for HEAP_SUMMARY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for HEAP_SUMMARY {
    fn eq(&self, other: &Self) -> bool {
        self.cb == other.cb && self.cbAllocated == other.cbAllocated && self.cbCommitted == other.cbCommitted && self.cbReserved == other.cbReserved && self.cbMaxReserve == other.cbMaxReserve
    }
}
impl ::core::cmp::Eq for HEAP_SUMMARY {}
impl ::core::default::Default for HEAP_SUMMARY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::windows::core::TypeKind for HeapHandle {
    type TypeKind = ::windows::core::CopyType;
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MEMORYMAPPEDVIEW_HANDLE(pub isize);
impl MEMORYMAPPEDVIEW_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for MEMORYMAPPEDVIEW_HANDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for MEMORYMAPPEDVIEW_HANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for MEMORYMAPPEDVIEW_HANDLE {}
impl ::core::fmt::Debug for MEMORYMAPPEDVIEW_HANDLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MEMORYMAPPEDVIEW_HANDLE").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for MEMORYMAPPEDVIEW_HANDLE {
    type TypeKind = ::windows::core::CopyType;
}
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
impl ::windows::core::TypeKind for MEMORY_BASIC_INFORMATION {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for MEMORY_BASIC_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.BaseAddress == other.BaseAddress && self.AllocationBase == other.AllocationBase && self.AllocationProtect == other.AllocationProtect && self.PartitionId == other.PartitionId && self.RegionSize == other.RegionSize && self.State == other.State && self.Protect == other.Protect && self.Type == other.Type
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
impl ::windows::core::TypeKind for MEMORY_BASIC_INFORMATION {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::PartialEq for MEMORY_BASIC_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.BaseAddress == other.BaseAddress && self.AllocationBase == other.AllocationBase && self.AllocationProtect == other.AllocationProtect && self.RegionSize == other.RegionSize && self.State == other.State && self.Protect == other.Protect && self.Type == other.Type
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
impl ::windows::core::TypeKind for MEMORY_BASIC_INFORMATION32 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MEMORY_BASIC_INFORMATION32 {
    fn eq(&self, other: &Self) -> bool {
        self.BaseAddress == other.BaseAddress && self.AllocationBase == other.AllocationBase && self.AllocationProtect == other.AllocationProtect && self.RegionSize == other.RegionSize && self.State == other.State && self.Protect == other.Protect && self.Type == other.Type
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
impl ::windows::core::TypeKind for MEMORY_BASIC_INFORMATION64 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MEMORY_BASIC_INFORMATION64 {
    fn eq(&self, other: &Self) -> bool {
        self.BaseAddress == other.BaseAddress && self.AllocationBase == other.AllocationBase && self.AllocationProtect == other.AllocationProtect && self.__alignment1 == other.__alignment1 && self.RegionSize == other.RegionSize && self.State == other.State && self.Protect == other.Protect && self.Type == other.Type && self.__alignment2 == other.__alignment2
    }
}
impl ::core::cmp::Eq for MEMORY_BASIC_INFORMATION64 {}
impl ::core::default::Default for MEMORY_BASIC_INFORMATION64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub struct MEMORY_PARTITION_DEDICATED_MEMORY_ATTRIBUTE {
    pub Type: MEM_DEDICATED_ATTRIBUTE_TYPE,
    pub Reserved: u32,
    pub Value: u64,
}
impl ::core::marker::Copy for MEMORY_PARTITION_DEDICATED_MEMORY_ATTRIBUTE {}
impl ::core::clone::Clone for MEMORY_PARTITION_DEDICATED_MEMORY_ATTRIBUTE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MEMORY_PARTITION_DEDICATED_MEMORY_ATTRIBUTE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MEMORY_PARTITION_DEDICATED_MEMORY_ATTRIBUTE").field("Type", &self.Type).field("Reserved", &self.Reserved).field("Value", &self.Value).finish()
    }
}
impl ::windows::core::TypeKind for MEMORY_PARTITION_DEDICATED_MEMORY_ATTRIBUTE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MEMORY_PARTITION_DEDICATED_MEMORY_ATTRIBUTE {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Reserved == other.Reserved && self.Value == other.Value
    }
}
impl ::core::cmp::Eq for MEMORY_PARTITION_DEDICATED_MEMORY_ATTRIBUTE {}
impl ::core::default::Default for MEMORY_PARTITION_DEDICATED_MEMORY_ATTRIBUTE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub struct MEMORY_PARTITION_DEDICATED_MEMORY_INFORMATION {
    pub NextEntryOffset: u32,
    pub SizeOfInformation: u32,
    pub Flags: u32,
    pub AttributesOffset: u32,
    pub AttributeCount: u32,
    pub Reserved: u32,
    pub TypeId: u64,
}
impl ::core::marker::Copy for MEMORY_PARTITION_DEDICATED_MEMORY_INFORMATION {}
impl ::core::clone::Clone for MEMORY_PARTITION_DEDICATED_MEMORY_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MEMORY_PARTITION_DEDICATED_MEMORY_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MEMORY_PARTITION_DEDICATED_MEMORY_INFORMATION").field("NextEntryOffset", &self.NextEntryOffset).field("SizeOfInformation", &self.SizeOfInformation).field("Flags", &self.Flags).field("AttributesOffset", &self.AttributesOffset).field("AttributeCount", &self.AttributeCount).field("Reserved", &self.Reserved).field("TypeId", &self.TypeId).finish()
    }
}
impl ::windows::core::TypeKind for MEMORY_PARTITION_DEDICATED_MEMORY_INFORMATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MEMORY_PARTITION_DEDICATED_MEMORY_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.NextEntryOffset == other.NextEntryOffset && self.SizeOfInformation == other.SizeOfInformation && self.Flags == other.Flags && self.AttributesOffset == other.AttributesOffset && self.AttributeCount == other.AttributeCount && self.Reserved == other.Reserved && self.TypeId == other.TypeId
    }
}
impl ::core::cmp::Eq for MEMORY_PARTITION_DEDICATED_MEMORY_INFORMATION {}
impl ::core::default::Default for MEMORY_PARTITION_DEDICATED_MEMORY_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::windows::core::TypeKind for MEM_ADDRESS_REQUIREMENTS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MEM_ADDRESS_REQUIREMENTS {
    fn eq(&self, other: &Self) -> bool {
        self.LowestStartingAddress == other.LowestStartingAddress && self.HighestEndingAddress == other.HighestEndingAddress && self.Alignment == other.Alignment
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
impl ::windows::core::TypeKind for MEM_EXTENDED_PARAMETER {
    type TypeKind = ::windows::core::CopyType;
}
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
impl ::windows::core::TypeKind for MEM_EXTENDED_PARAMETER_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MEM_EXTENDED_PARAMETER_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
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
impl ::windows::core::TypeKind for MEM_EXTENDED_PARAMETER_1 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MEM_EXTENDED_PARAMETER_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
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
impl ::windows::core::TypeKind for PROCESS_HEAP_ENTRY {
    type TypeKind = ::windows::core::CopyType;
}
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
impl ::windows::core::TypeKind for PROCESS_HEAP_ENTRY_0 {
    type TypeKind = ::windows::core::CopyType;
}
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
impl ::windows::core::TypeKind for PROCESS_HEAP_ENTRY_0_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PROCESS_HEAP_ENTRY_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.hMem == other.hMem && self.dwReserved == other.dwReserved
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
impl ::windows::core::TypeKind for PROCESS_HEAP_ENTRY_0_1 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PROCESS_HEAP_ENTRY_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.dwCommittedSize == other.dwCommittedSize && self.dwUnCommittedSize == other.dwUnCommittedSize && self.lpFirstBlock == other.lpFirstBlock && self.lpLastBlock == other.lpLastBlock
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
impl ::windows::core::TypeKind for WIN32_MEMORY_PARTITION_INFORMATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WIN32_MEMORY_PARTITION_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.NumaNode == other.NumaNode && self.Channel == other.Channel && self.NumberOfNumaNodes == other.NumberOfNumaNodes && self.ResidentAvailablePages == other.ResidentAvailablePages && self.CommittedPages == other.CommittedPages && self.CommitLimit == other.CommitLimit && self.PeakCommitment == other.PeakCommitment && self.TotalNumberOfPages == other.TotalNumberOfPages && self.AvailablePages == other.AvailablePages && self.ZeroPages == other.ZeroPages && self.FreePages == other.FreePages && self.StandbyPages == other.StandbyPages && self.Reserved == other.Reserved && self.MaximumCommitLimit == other.MaximumCommitLimit && self.Reserved2 == other.Reserved2 && self.PartitionId == other.PartitionId
    }
}
impl ::core::cmp::Eq for WIN32_MEMORY_PARTITION_INFORMATION {}
impl ::core::default::Default for WIN32_MEMORY_PARTITION_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::windows::core::TypeKind for WIN32_MEMORY_RANGE_ENTRY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WIN32_MEMORY_RANGE_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.VirtualAddress == other.VirtualAddress && self.NumberOfBytes == other.NumberOfBytes
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
impl ::windows::core::TypeKind for WIN32_MEMORY_REGION_INFORMATION {
    type TypeKind = ::windows::core::CopyType;
}
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
impl ::windows::core::TypeKind for WIN32_MEMORY_REGION_INFORMATION_0 {
    type TypeKind = ::windows::core::CopyType;
}
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
impl ::windows::core::TypeKind for WIN32_MEMORY_REGION_INFORMATION_0_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WIN32_MEMORY_REGION_INFORMATION_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for WIN32_MEMORY_REGION_INFORMATION_0_0 {}
impl ::core::default::Default for WIN32_MEMORY_REGION_INFORMATION_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub type PBAD_MEMORY_CALLBACK_ROUTINE = ::core::option::Option<unsafe extern "system" fn() -> ()>;
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PSECURE_MEMORY_CACHE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(addr: *const ::core::ffi::c_void, range: usize) -> super::super::Foundation::BOOLEAN>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
