#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`*"]
#[inline]
pub unsafe fn AddDllDirectory<P0>(newdirectory: P0) -> *mut ::core::ffi::c_void
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn AddDllDirectory ( newdirectory : :: windows::core::PCWSTR ) -> *mut ::core::ffi::c_void );
    AddDllDirectory(newdirectory.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BeginUpdateResourceA<P0, P1>(pfilename: P0, bdeleteexistingresources: P1) -> ::windows::core::Result<UPDATERESOURCE_HANDLE>
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn BeginUpdateResourceA ( pfilename : :: windows::core::PCSTR , bdeleteexistingresources : super::super::Foundation:: BOOL ) -> UPDATERESOURCE_HANDLE );
    let result__ = BeginUpdateResourceA(pfilename.into_param().abi(), bdeleteexistingresources.into_param().abi());
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BeginUpdateResourceW<P0, P1>(pfilename: P0, bdeleteexistingresources: P1) -> ::windows::core::Result<UPDATERESOURCE_HANDLE>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn BeginUpdateResourceW ( pfilename : :: windows::core::PCWSTR , bdeleteexistingresources : super::super::Foundation:: BOOL ) -> UPDATERESOURCE_HANDLE );
    let result__ = BeginUpdateResourceW(pfilename.into_param().abi(), bdeleteexistingresources.into_param().abi());
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DisableThreadLibraryCalls<P0>(hlibmodule: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HINSTANCE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn DisableThreadLibraryCalls ( hlibmodule : super::super::Foundation:: HINSTANCE ) -> super::super::Foundation:: BOOL );
    DisableThreadLibraryCalls(hlibmodule.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EndUpdateResourceA<P0, P1>(hupdate: P0, fdiscard: P1) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<UPDATERESOURCE_HANDLE>,
    P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn EndUpdateResourceA ( hupdate : UPDATERESOURCE_HANDLE , fdiscard : super::super::Foundation:: BOOL ) -> super::super::Foundation:: BOOL );
    EndUpdateResourceA(hupdate.into_param().abi(), fdiscard.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EndUpdateResourceW<P0, P1>(hupdate: P0, fdiscard: P1) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<UPDATERESOURCE_HANDLE>,
    P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn EndUpdateResourceW ( hupdate : UPDATERESOURCE_HANDLE , fdiscard : super::super::Foundation:: BOOL ) -> super::super::Foundation:: BOOL );
    EndUpdateResourceW(hupdate.into_param().abi(), fdiscard.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumResourceLanguagesA<P0, P1, P2>(hmodule: P0, lptype: P1, lpname: P2, lpenumfunc: ENUMRESLANGPROCA, lparam: isize) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HINSTANCE>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn EnumResourceLanguagesA ( hmodule : super::super::Foundation:: HINSTANCE , lptype : :: windows::core::PCSTR , lpname : :: windows::core::PCSTR , lpenumfunc : ENUMRESLANGPROCA , lparam : isize ) -> super::super::Foundation:: BOOL );
    EnumResourceLanguagesA(hmodule.into_param().abi(), lptype.into_param().abi(), lpname.into_param().abi(), lpenumfunc, lparam)
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumResourceLanguagesExA<P0, P1, P2>(hmodule: P0, lptype: P1, lpname: P2, lpenumfunc: ENUMRESLANGPROCA, lparam: isize, dwflags: u32, langid: u16) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HINSTANCE>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn EnumResourceLanguagesExA ( hmodule : super::super::Foundation:: HINSTANCE , lptype : :: windows::core::PCSTR , lpname : :: windows::core::PCSTR , lpenumfunc : ENUMRESLANGPROCA , lparam : isize , dwflags : u32 , langid : u16 ) -> super::super::Foundation:: BOOL );
    EnumResourceLanguagesExA(hmodule.into_param().abi(), lptype.into_param().abi(), lpname.into_param().abi(), lpenumfunc, lparam, dwflags, langid)
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumResourceLanguagesExW<P0, P1, P2>(hmodule: P0, lptype: P1, lpname: P2, lpenumfunc: ENUMRESLANGPROCW, lparam: isize, dwflags: u32, langid: u16) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HINSTANCE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn EnumResourceLanguagesExW ( hmodule : super::super::Foundation:: HINSTANCE , lptype : :: windows::core::PCWSTR , lpname : :: windows::core::PCWSTR , lpenumfunc : ENUMRESLANGPROCW , lparam : isize , dwflags : u32 , langid : u16 ) -> super::super::Foundation:: BOOL );
    EnumResourceLanguagesExW(hmodule.into_param().abi(), lptype.into_param().abi(), lpname.into_param().abi(), lpenumfunc, lparam, dwflags, langid)
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumResourceLanguagesW<P0, P1, P2>(hmodule: P0, lptype: P1, lpname: P2, lpenumfunc: ENUMRESLANGPROCW, lparam: isize) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HINSTANCE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn EnumResourceLanguagesW ( hmodule : super::super::Foundation:: HINSTANCE , lptype : :: windows::core::PCWSTR , lpname : :: windows::core::PCWSTR , lpenumfunc : ENUMRESLANGPROCW , lparam : isize ) -> super::super::Foundation:: BOOL );
    EnumResourceLanguagesW(hmodule.into_param().abi(), lptype.into_param().abi(), lpname.into_param().abi(), lpenumfunc, lparam)
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumResourceNamesA<P0, P1>(hmodule: P0, lptype: P1, lpenumfunc: ENUMRESNAMEPROCA, lparam: isize) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HINSTANCE>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn EnumResourceNamesA ( hmodule : super::super::Foundation:: HINSTANCE , lptype : :: windows::core::PCSTR , lpenumfunc : ENUMRESNAMEPROCA , lparam : isize ) -> super::super::Foundation:: BOOL );
    EnumResourceNamesA(hmodule.into_param().abi(), lptype.into_param().abi(), lpenumfunc, lparam)
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumResourceNamesExA<P0, P1>(hmodule: P0, lptype: P1, lpenumfunc: ENUMRESNAMEPROCA, lparam: isize, dwflags: u32, langid: u16) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HINSTANCE>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn EnumResourceNamesExA ( hmodule : super::super::Foundation:: HINSTANCE , lptype : :: windows::core::PCSTR , lpenumfunc : ENUMRESNAMEPROCA , lparam : isize , dwflags : u32 , langid : u16 ) -> super::super::Foundation:: BOOL );
    EnumResourceNamesExA(hmodule.into_param().abi(), lptype.into_param().abi(), lpenumfunc, lparam, dwflags, langid)
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumResourceNamesExW<P0, P1>(hmodule: P0, lptype: P1, lpenumfunc: ENUMRESNAMEPROCW, lparam: isize, dwflags: u32, langid: u16) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HINSTANCE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn EnumResourceNamesExW ( hmodule : super::super::Foundation:: HINSTANCE , lptype : :: windows::core::PCWSTR , lpenumfunc : ENUMRESNAMEPROCW , lparam : isize , dwflags : u32 , langid : u16 ) -> super::super::Foundation:: BOOL );
    EnumResourceNamesExW(hmodule.into_param().abi(), lptype.into_param().abi(), lpenumfunc, lparam, dwflags, langid)
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumResourceNamesW<P0, P1>(hmodule: P0, lptype: P1, lpenumfunc: ENUMRESNAMEPROCW, lparam: isize) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HINSTANCE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn EnumResourceNamesW ( hmodule : super::super::Foundation:: HINSTANCE , lptype : :: windows::core::PCWSTR , lpenumfunc : ENUMRESNAMEPROCW , lparam : isize ) -> super::super::Foundation:: BOOL );
    EnumResourceNamesW(hmodule.into_param().abi(), lptype.into_param().abi(), lpenumfunc, lparam)
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumResourceTypesA<P0>(hmodule: P0, lpenumfunc: ENUMRESTYPEPROCA, lparam: isize) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HINSTANCE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn EnumResourceTypesA ( hmodule : super::super::Foundation:: HINSTANCE , lpenumfunc : ENUMRESTYPEPROCA , lparam : isize ) -> super::super::Foundation:: BOOL );
    EnumResourceTypesA(hmodule.into_param().abi(), lpenumfunc, lparam)
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumResourceTypesExA<P0>(hmodule: P0, lpenumfunc: ENUMRESTYPEPROCA, lparam: isize, dwflags: u32, langid: u16) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HINSTANCE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn EnumResourceTypesExA ( hmodule : super::super::Foundation:: HINSTANCE , lpenumfunc : ENUMRESTYPEPROCA , lparam : isize , dwflags : u32 , langid : u16 ) -> super::super::Foundation:: BOOL );
    EnumResourceTypesExA(hmodule.into_param().abi(), lpenumfunc, lparam, dwflags, langid)
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumResourceTypesExW<P0>(hmodule: P0, lpenumfunc: ENUMRESTYPEPROCW, lparam: isize, dwflags: u32, langid: u16) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HINSTANCE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn EnumResourceTypesExW ( hmodule : super::super::Foundation:: HINSTANCE , lpenumfunc : ENUMRESTYPEPROCW , lparam : isize , dwflags : u32 , langid : u16 ) -> super::super::Foundation:: BOOL );
    EnumResourceTypesExW(hmodule.into_param().abi(), lpenumfunc, lparam, dwflags, langid)
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumResourceTypesW<P0>(hmodule: P0, lpenumfunc: ENUMRESTYPEPROCW, lparam: isize) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HINSTANCE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn EnumResourceTypesW ( hmodule : super::super::Foundation:: HINSTANCE , lpenumfunc : ENUMRESTYPEPROCW , lparam : isize ) -> super::super::Foundation:: BOOL );
    EnumResourceTypesW(hmodule.into_param().abi(), lpenumfunc, lparam)
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindResourceA<P0, P1, P2>(hmodule: P0, lpname: P1, lptype: P2) -> ::windows::core::Result<super::super::Foundation::HRSRC>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HINSTANCE>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn FindResourceA ( hmodule : super::super::Foundation:: HINSTANCE , lpname : :: windows::core::PCSTR , lptype : :: windows::core::PCSTR ) -> super::super::Foundation:: HRSRC );
    let result__ = FindResourceA(hmodule.into_param().abi(), lpname.into_param().abi(), lptype.into_param().abi());
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindResourceExA<P0, P1, P2>(hmodule: P0, lptype: P1, lpname: P2, wlanguage: u16) -> ::windows::core::Result<super::super::Foundation::HRSRC>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HINSTANCE>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn FindResourceExA ( hmodule : super::super::Foundation:: HINSTANCE , lptype : :: windows::core::PCSTR , lpname : :: windows::core::PCSTR , wlanguage : u16 ) -> super::super::Foundation:: HRSRC );
    let result__ = FindResourceExA(hmodule.into_param().abi(), lptype.into_param().abi(), lpname.into_param().abi(), wlanguage);
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindResourceExW<P0, P1, P2>(hmodule: P0, lptype: P1, lpname: P2, wlanguage: u16) -> super::super::Foundation::HRSRC
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HINSTANCE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn FindResourceExW ( hmodule : super::super::Foundation:: HINSTANCE , lptype : :: windows::core::PCWSTR , lpname : :: windows::core::PCWSTR , wlanguage : u16 ) -> super::super::Foundation:: HRSRC );
    FindResourceExW(hmodule.into_param().abi(), lptype.into_param().abi(), lpname.into_param().abi(), wlanguage)
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindResourceW<P0, P1, P2>(hmodule: P0, lpname: P1, lptype: P2) -> super::super::Foundation::HRSRC
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HINSTANCE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn FindResourceW ( hmodule : super::super::Foundation:: HINSTANCE , lpname : :: windows::core::PCWSTR , lptype : :: windows::core::PCWSTR ) -> super::super::Foundation:: HRSRC );
    FindResourceW(hmodule.into_param().abi(), lpname.into_param().abi(), lptype.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FreeLibrary<P0>(hlibmodule: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HINSTANCE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn FreeLibrary ( hlibmodule : super::super::Foundation:: HINSTANCE ) -> super::super::Foundation:: BOOL );
    FreeLibrary(hlibmodule.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FreeLibraryAndExitThread<P0>(hlibmodule: P0, dwexitcode: u32) -> !
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HINSTANCE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn FreeLibraryAndExitThread ( hlibmodule : super::super::Foundation:: HINSTANCE , dwexitcode : u32 ) -> ! );
    FreeLibraryAndExitThread(hlibmodule.into_param().abi(), dwexitcode)
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FreeResource<P0>(hresdata: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HGLOBAL>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn FreeResource ( hresdata : super::super::Foundation:: HGLOBAL ) -> super::super::Foundation:: BOOL );
    FreeResource(hresdata.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`*"]
#[inline]
pub unsafe fn GetDllDirectoryA(lpbuffer: ::core::option::Option<&mut [u8]>) -> u32 {
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetDllDirectoryA ( nbufferlength : u32 , lpbuffer : :: windows::core::PSTR ) -> u32 );
    GetDllDirectoryA(lpbuffer.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(lpbuffer.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`*"]
#[inline]
pub unsafe fn GetDllDirectoryW(lpbuffer: ::core::option::Option<&mut [u16]>) -> u32 {
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetDllDirectoryW ( nbufferlength : u32 , lpbuffer : :: windows::core::PWSTR ) -> u32 );
    GetDllDirectoryW(lpbuffer.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(lpbuffer.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetModuleFileNameA<P0>(hmodule: P0, lpfilename: &mut [u8]) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HINSTANCE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetModuleFileNameA ( hmodule : super::super::Foundation:: HINSTANCE , lpfilename : :: windows::core::PSTR , nsize : u32 ) -> u32 );
    GetModuleFileNameA(hmodule.into_param().abi(), ::core::mem::transmute(lpfilename.as_ptr()), lpfilename.len() as _)
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetModuleFileNameW<P0>(hmodule: P0, lpfilename: &mut [u16]) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HINSTANCE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetModuleFileNameW ( hmodule : super::super::Foundation:: HINSTANCE , lpfilename : :: windows::core::PWSTR , nsize : u32 ) -> u32 );
    GetModuleFileNameW(hmodule.into_param().abi(), ::core::mem::transmute(lpfilename.as_ptr()), lpfilename.len() as _)
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetModuleHandleA<P0>(lpmodulename: P0) -> ::windows::core::Result<super::super::Foundation::HINSTANCE>
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetModuleHandleA ( lpmodulename : :: windows::core::PCSTR ) -> super::super::Foundation:: HINSTANCE );
    let result__ = GetModuleHandleA(lpmodulename.into_param().abi());
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetModuleHandleExA<P0>(dwflags: u32, lpmodulename: P0, phmodule: *mut super::super::Foundation::HINSTANCE) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetModuleHandleExA ( dwflags : u32 , lpmodulename : :: windows::core::PCSTR , phmodule : *mut super::super::Foundation:: HINSTANCE ) -> super::super::Foundation:: BOOL );
    GetModuleHandleExA(dwflags, lpmodulename.into_param().abi(), phmodule)
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetModuleHandleExW<P0>(dwflags: u32, lpmodulename: P0, phmodule: *mut super::super::Foundation::HINSTANCE) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetModuleHandleExW ( dwflags : u32 , lpmodulename : :: windows::core::PCWSTR , phmodule : *mut super::super::Foundation:: HINSTANCE ) -> super::super::Foundation:: BOOL );
    GetModuleHandleExW(dwflags, lpmodulename.into_param().abi(), phmodule)
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetModuleHandleW<P0>(lpmodulename: P0) -> ::windows::core::Result<super::super::Foundation::HINSTANCE>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetModuleHandleW ( lpmodulename : :: windows::core::PCWSTR ) -> super::super::Foundation:: HINSTANCE );
    let result__ = GetModuleHandleW(lpmodulename.into_param().abi());
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetProcAddress<P0, P1>(hmodule: P0, lpprocname: P1) -> super::super::Foundation::FARPROC
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HINSTANCE>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetProcAddress ( hmodule : super::super::Foundation:: HINSTANCE , lpprocname : :: windows::core::PCSTR ) -> super::super::Foundation:: FARPROC );
    GetProcAddress(hmodule.into_param().abi(), lpprocname.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LoadLibraryA<P0>(lplibfilename: P0) -> ::windows::core::Result<super::super::Foundation::HINSTANCE>
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn LoadLibraryA ( lplibfilename : :: windows::core::PCSTR ) -> super::super::Foundation:: HINSTANCE );
    let result__ = LoadLibraryA(lplibfilename.into_param().abi());
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LoadLibraryExA<P0, P1>(lplibfilename: P0, hfile: P1, dwflags: LOAD_LIBRARY_FLAGS) -> ::windows::core::Result<super::super::Foundation::HINSTANCE>
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn LoadLibraryExA ( lplibfilename : :: windows::core::PCSTR , hfile : super::super::Foundation:: HANDLE , dwflags : LOAD_LIBRARY_FLAGS ) -> super::super::Foundation:: HINSTANCE );
    let result__ = LoadLibraryExA(lplibfilename.into_param().abi(), hfile.into_param().abi(), dwflags);
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LoadLibraryExW<P0, P1>(lplibfilename: P0, hfile: P1, dwflags: LOAD_LIBRARY_FLAGS) -> ::windows::core::Result<super::super::Foundation::HINSTANCE>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn LoadLibraryExW ( lplibfilename : :: windows::core::PCWSTR , hfile : super::super::Foundation:: HANDLE , dwflags : LOAD_LIBRARY_FLAGS ) -> super::super::Foundation:: HINSTANCE );
    let result__ = LoadLibraryExW(lplibfilename.into_param().abi(), hfile.into_param().abi(), dwflags);
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LoadLibraryW<P0>(lplibfilename: P0) -> ::windows::core::Result<super::super::Foundation::HINSTANCE>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn LoadLibraryW ( lplibfilename : :: windows::core::PCWSTR ) -> super::super::Foundation:: HINSTANCE );
    let result__ = LoadLibraryW(lplibfilename.into_param().abi());
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`*"]
#[inline]
pub unsafe fn LoadModule<P0>(lpmodulename: P0, lpparameterblock: *const ::core::ffi::c_void) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn LoadModule ( lpmodulename : :: windows::core::PCSTR , lpparameterblock : *const ::core::ffi::c_void ) -> u32 );
    LoadModule(lpmodulename.into_param().abi(), lpparameterblock)
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LoadPackagedLibrary<P0>(lpwlibfilename: P0, reserved: u32) -> ::windows::core::Result<super::super::Foundation::HINSTANCE>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn LoadPackagedLibrary ( lpwlibfilename : :: windows::core::PCWSTR , reserved : u32 ) -> super::super::Foundation:: HINSTANCE );
    let result__ = LoadPackagedLibrary(lpwlibfilename.into_param().abi(), reserved);
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LoadResource<P0, P1>(hmodule: P0, hresinfo: P1) -> ::windows::core::Result<super::super::Foundation::HGLOBAL>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HINSTANCE>,
    P1: ::windows::core::IntoParam<super::super::Foundation::HRSRC>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn LoadResource ( hmodule : super::super::Foundation:: HINSTANCE , hresinfo : super::super::Foundation:: HRSRC ) -> super::super::Foundation:: HGLOBAL );
    let result__ = LoadResource(hmodule.into_param().abi(), hresinfo.into_param().abi());
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LockResource<P0>(hresdata: P0) -> *mut ::core::ffi::c_void
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HGLOBAL>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn LockResource ( hresdata : super::super::Foundation:: HGLOBAL ) -> *mut ::core::ffi::c_void );
    LockResource(hresdata.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RemoveDllDirectory(cookie: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "kernel32.dll""system" fn RemoveDllDirectory ( cookie : *const ::core::ffi::c_void ) -> super::super::Foundation:: BOOL );
    RemoveDllDirectory(cookie)
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetDefaultDllDirectories(directoryflags: LOAD_LIBRARY_FLAGS) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "kernel32.dll""system" fn SetDefaultDllDirectories ( directoryflags : LOAD_LIBRARY_FLAGS ) -> super::super::Foundation:: BOOL );
    SetDefaultDllDirectories(directoryflags)
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetDllDirectoryA<P0>(lppathname: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn SetDllDirectoryA ( lppathname : :: windows::core::PCSTR ) -> super::super::Foundation:: BOOL );
    SetDllDirectoryA(lppathname.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetDllDirectoryW<P0>(lppathname: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn SetDllDirectoryW ( lppathname : :: windows::core::PCWSTR ) -> super::super::Foundation:: BOOL );
    SetDllDirectoryW(lppathname.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SizeofResource<P0, P1>(hmodule: P0, hresinfo: P1) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HINSTANCE>,
    P1: ::windows::core::IntoParam<super::super::Foundation::HRSRC>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn SizeofResource ( hmodule : super::super::Foundation:: HINSTANCE , hresinfo : super::super::Foundation:: HRSRC ) -> u32 );
    SizeofResource(hmodule.into_param().abi(), hresinfo.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UpdateResourceA<P0, P1, P2>(hupdate: P0, lptype: P1, lpname: P2, wlanguage: u16, lpdata: ::core::option::Option<*const ::core::ffi::c_void>, cb: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<UPDATERESOURCE_HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn UpdateResourceA ( hupdate : UPDATERESOURCE_HANDLE , lptype : :: windows::core::PCSTR , lpname : :: windows::core::PCSTR , wlanguage : u16 , lpdata : *const ::core::ffi::c_void , cb : u32 ) -> super::super::Foundation:: BOOL );
    UpdateResourceA(hupdate.into_param().abi(), lptype.into_param().abi(), lpname.into_param().abi(), wlanguage, ::core::mem::transmute(lpdata.unwrap_or(::std::ptr::null())), cb)
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UpdateResourceW<P0, P1, P2>(hupdate: P0, lptype: P1, lpname: P2, wlanguage: u16, lpdata: ::core::option::Option<*const ::core::ffi::c_void>, cb: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<UPDATERESOURCE_HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn UpdateResourceW ( hupdate : UPDATERESOURCE_HANDLE , lptype : :: windows::core::PCWSTR , lpname : :: windows::core::PCWSTR , wlanguage : u16 , lpdata : *const ::core::ffi::c_void , cb : u32 ) -> super::super::Foundation:: BOOL );
    UpdateResourceW(hupdate.into_param().abi(), lptype.into_param().abi(), lpname.into_param().abi(), wlanguage, ::core::mem::transmute(lpdata.unwrap_or(::std::ptr::null())), cb)
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`*"]
pub const CURRENT_IMPORT_REDIRECTION_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`*"]
pub const FIND_RESOURCE_DIRECTORY_LANGUAGES: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`*"]
pub const FIND_RESOURCE_DIRECTORY_NAMES: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`*"]
pub const FIND_RESOURCE_DIRECTORY_TYPES: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`*"]
pub const GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`*"]
pub const GET_MODULE_HANDLE_EX_FLAG_PIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`*"]
pub const GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`*"]
pub const LOAD_LIBRARY_OS_INTEGRITY_CONTINUITY: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`*"]
pub const RESOURCE_ENUM_LN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`*"]
pub const RESOURCE_ENUM_MODULE_EXACT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`*"]
pub const RESOURCE_ENUM_MUI: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`*"]
pub const RESOURCE_ENUM_MUI_SYSTEM: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`*"]
pub const RESOURCE_ENUM_VALIDATE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`*"]
pub const SUPPORT_LANG_NUMBER: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct LOAD_LIBRARY_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`*"]
pub const DONT_RESOLVE_DLL_REFERENCES: LOAD_LIBRARY_FLAGS = LOAD_LIBRARY_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`*"]
pub const LOAD_LIBRARY_AS_DATAFILE: LOAD_LIBRARY_FLAGS = LOAD_LIBRARY_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`*"]
pub const LOAD_WITH_ALTERED_SEARCH_PATH: LOAD_LIBRARY_FLAGS = LOAD_LIBRARY_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`*"]
pub const LOAD_IGNORE_CODE_AUTHZ_LEVEL: LOAD_LIBRARY_FLAGS = LOAD_LIBRARY_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`*"]
pub const LOAD_LIBRARY_AS_IMAGE_RESOURCE: LOAD_LIBRARY_FLAGS = LOAD_LIBRARY_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`*"]
pub const LOAD_LIBRARY_AS_DATAFILE_EXCLUSIVE: LOAD_LIBRARY_FLAGS = LOAD_LIBRARY_FLAGS(64u32);
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`*"]
pub const LOAD_LIBRARY_REQUIRE_SIGNED_TARGET: LOAD_LIBRARY_FLAGS = LOAD_LIBRARY_FLAGS(128u32);
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`*"]
pub const LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR: LOAD_LIBRARY_FLAGS = LOAD_LIBRARY_FLAGS(256u32);
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`*"]
pub const LOAD_LIBRARY_SEARCH_APPLICATION_DIR: LOAD_LIBRARY_FLAGS = LOAD_LIBRARY_FLAGS(512u32);
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`*"]
pub const LOAD_LIBRARY_SEARCH_USER_DIRS: LOAD_LIBRARY_FLAGS = LOAD_LIBRARY_FLAGS(1024u32);
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`*"]
pub const LOAD_LIBRARY_SEARCH_SYSTEM32: LOAD_LIBRARY_FLAGS = LOAD_LIBRARY_FLAGS(2048u32);
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`*"]
pub const LOAD_LIBRARY_SEARCH_DEFAULT_DIRS: LOAD_LIBRARY_FLAGS = LOAD_LIBRARY_FLAGS(4096u32);
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`*"]
pub const LOAD_LIBRARY_SAFE_CURRENT_DIRS: LOAD_LIBRARY_FLAGS = LOAD_LIBRARY_FLAGS(8192u32);
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`*"]
pub const LOAD_LIBRARY_SEARCH_SYSTEM32_NO_FORWARDER: LOAD_LIBRARY_FLAGS = LOAD_LIBRARY_FLAGS(16384u32);
impl ::core::marker::Copy for LOAD_LIBRARY_FLAGS {}
impl ::core::clone::Clone for LOAD_LIBRARY_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LOAD_LIBRARY_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for LOAD_LIBRARY_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for LOAD_LIBRARY_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LOAD_LIBRARY_FLAGS").field(&self.0).finish()
    }
}
impl LOAD_LIBRARY_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for LOAD_LIBRARY_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for LOAD_LIBRARY_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for LOAD_LIBRARY_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for LOAD_LIBRARY_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for LOAD_LIBRARY_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`*"]
pub struct ENUMUILANG {
    pub NumOfEnumUILang: u32,
    pub SizeOfEnumUIBuffer: u32,
    pub pEnumUIBuffer: *mut u16,
}
impl ::core::marker::Copy for ENUMUILANG {}
impl ::core::clone::Clone for ENUMUILANG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ENUMUILANG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENUMUILANG").field("NumOfEnumUILang", &self.NumOfEnumUILang).field("SizeOfEnumUIBuffer", &self.SizeOfEnumUIBuffer).field("pEnumUIBuffer", &self.pEnumUIBuffer).finish()
    }
}
impl ::windows::core::TypeKind for ENUMUILANG {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for ENUMUILANG {
    fn eq(&self, other: &Self) -> bool {
        self.NumOfEnumUILang == other.NumOfEnumUILang && self.SizeOfEnumUIBuffer == other.SizeOfEnumUIBuffer && self.pEnumUIBuffer == other.pEnumUIBuffer
    }
}
impl ::core::cmp::Eq for ENUMUILANG {}
impl ::core::default::Default for ENUMUILANG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`*"]
pub struct REDIRECTION_DESCRIPTOR {
    pub Version: u32,
    pub FunctionCount: u32,
    pub Redirections: *mut REDIRECTION_FUNCTION_DESCRIPTOR,
}
impl ::core::marker::Copy for REDIRECTION_DESCRIPTOR {}
impl ::core::clone::Clone for REDIRECTION_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for REDIRECTION_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REDIRECTION_DESCRIPTOR").field("Version", &self.Version).field("FunctionCount", &self.FunctionCount).field("Redirections", &self.Redirections).finish()
    }
}
impl ::windows::core::TypeKind for REDIRECTION_DESCRIPTOR {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for REDIRECTION_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.FunctionCount == other.FunctionCount && self.Redirections == other.Redirections
    }
}
impl ::core::cmp::Eq for REDIRECTION_DESCRIPTOR {}
impl ::core::default::Default for REDIRECTION_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`*"]
pub struct REDIRECTION_FUNCTION_DESCRIPTOR {
    pub DllName: ::windows::core::PCSTR,
    pub FunctionName: ::windows::core::PCSTR,
    pub RedirectionTarget: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for REDIRECTION_FUNCTION_DESCRIPTOR {}
impl ::core::clone::Clone for REDIRECTION_FUNCTION_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for REDIRECTION_FUNCTION_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REDIRECTION_FUNCTION_DESCRIPTOR").field("DllName", &self.DllName).field("FunctionName", &self.FunctionName).field("RedirectionTarget", &self.RedirectionTarget).finish()
    }
}
impl ::windows::core::TypeKind for REDIRECTION_FUNCTION_DESCRIPTOR {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for REDIRECTION_FUNCTION_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.DllName == other.DllName && self.FunctionName == other.FunctionName && self.RedirectionTarget == other.RedirectionTarget
    }
}
impl ::core::cmp::Eq for REDIRECTION_FUNCTION_DESCRIPTOR {}
impl ::core::default::Default for REDIRECTION_FUNCTION_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct UPDATERESOURCE_HANDLE(pub isize);
impl UPDATERESOURCE_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0 == 0
    }
}
impl ::core::default::Default for UPDATERESOURCE_HANDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for UPDATERESOURCE_HANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for UPDATERESOURCE_HANDLE {}
impl ::core::fmt::Debug for UPDATERESOURCE_HANDLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UPDATERESOURCE_HANDLE").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for UPDATERESOURCE_HANDLE {
    type TypeKind = ::windows::core::CopyType;
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type ENUMRESLANGPROCA = ::core::option::Option<unsafe extern "system" fn(hmodule: super::super::Foundation::HINSTANCE, lptype: ::windows::core::PCSTR, lpname: ::windows::core::PCSTR, wlanguage: u16, lparam: isize) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type ENUMRESLANGPROCW = ::core::option::Option<unsafe extern "system" fn(hmodule: super::super::Foundation::HINSTANCE, lptype: ::windows::core::PCWSTR, lpname: ::windows::core::PCWSTR, wlanguage: u16, lparam: isize) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type ENUMRESNAMEPROCA = ::core::option::Option<unsafe extern "system" fn(hmodule: super::super::Foundation::HINSTANCE, lptype: ::windows::core::PCSTR, lpname: ::windows::core::PCSTR, lparam: isize) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type ENUMRESNAMEPROCW = ::core::option::Option<unsafe extern "system" fn(hmodule: super::super::Foundation::HINSTANCE, lptype: ::windows::core::PCWSTR, lpname: ::windows::core::PCWSTR, lparam: isize) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type ENUMRESTYPEPROCA = ::core::option::Option<unsafe extern "system" fn(hmodule: super::super::Foundation::HINSTANCE, lptype: ::windows::core::PCSTR, lparam: isize) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type ENUMRESTYPEPROCW = ::core::option::Option<unsafe extern "system" fn(hmodule: super::super::Foundation::HINSTANCE, lptype: ::windows::core::PCWSTR, lparam: isize) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PGET_MODULE_HANDLE_EXA = ::core::option::Option<unsafe extern "system" fn(dwflags: u32, lpmodulename: ::windows::core::PCSTR, phmodule: *mut super::super::Foundation::HINSTANCE) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PGET_MODULE_HANDLE_EXW = ::core::option::Option<unsafe extern "system" fn(dwflags: u32, lpmodulename: ::windows::core::PCWSTR, phmodule: *mut super::super::Foundation::HINSTANCE) -> super::super::Foundation::BOOL>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
