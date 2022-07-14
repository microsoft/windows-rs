#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`*"]
#[inline]
pub unsafe fn AddDllDirectory<'a, P0>(newdirectory: P0) -> *mut ::core::ffi::c_void
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn AddDllDirectory(newdirectory: ::windows::core::PCWSTR) -> *mut ::core::ffi::c_void;
    }
    AddDllDirectory(newdirectory.into())
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BeginUpdateResourceA<'a, P0, P1>(pfilename: P0, bdeleteexistingresources: P1) -> ::windows::core::Result<super::super::Foundation::HANDLE>
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn BeginUpdateResourceA(pfilename: ::windows::core::PCSTR, bdeleteexistingresources: super::super::Foundation::BOOL) -> super::super::Foundation::HANDLE;
    }
    let result__ = BeginUpdateResourceA(pfilename.into(), bdeleteexistingresources.into());
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BeginUpdateResourceW<'a, P0, P1>(pfilename: P0, bdeleteexistingresources: P1) -> ::windows::core::Result<super::super::Foundation::HANDLE>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn BeginUpdateResourceW(pfilename: ::windows::core::PCWSTR, bdeleteexistingresources: super::super::Foundation::BOOL) -> super::super::Foundation::HANDLE;
    }
    let result__ = BeginUpdateResourceW(pfilename.into(), bdeleteexistingresources.into());
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`*"]
pub const CURRENT_IMPORT_REDIRECTION_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DisableThreadLibraryCalls<'a, P0>(hlibmodule: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DisableThreadLibraryCalls(hlibmodule: super::super::Foundation::HINSTANCE) -> super::super::Foundation::BOOL;
    }
    DisableThreadLibraryCalls(hlibmodule.into())
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
unsafe impl ::windows::core::Abi for ENUMUILANG {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ENUMUILANG {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ENUMUILANG>()) == 0 }
    }
}
impl ::core::cmp::Eq for ENUMUILANG {}
impl ::core::default::Default for ENUMUILANG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EndUpdateResourceA<'a, P0, P1>(hupdate: P0, fdiscard: P1) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EndUpdateResourceA(hupdate: super::super::Foundation::HANDLE, fdiscard: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    }
    EndUpdateResourceA(hupdate.into(), fdiscard.into())
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EndUpdateResourceW<'a, P0, P1>(hupdate: P0, fdiscard: P1) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EndUpdateResourceW(hupdate: super::super::Foundation::HANDLE, fdiscard: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    }
    EndUpdateResourceW(hupdate.into(), fdiscard.into())
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumResourceLanguagesA<'a, P0, P1, P2>(hmodule: P0, lptype: P1, lpname: P2, lpenumfunc: ENUMRESLANGPROCA, lparam: isize) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
    P2: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EnumResourceLanguagesA(hmodule: super::super::Foundation::HINSTANCE, lptype: ::windows::core::PCSTR, lpname: ::windows::core::PCSTR, lpenumfunc: *mut ::core::ffi::c_void, lparam: isize) -> super::super::Foundation::BOOL;
    }
    EnumResourceLanguagesA(hmodule.into(), lptype.into(), lpname.into(), ::core::mem::transmute(lpenumfunc), lparam)
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumResourceLanguagesExA<'a, P0, P1, P2>(hmodule: P0, lptype: P1, lpname: P2, lpenumfunc: ENUMRESLANGPROCA, lparam: isize, dwflags: u32, langid: u16) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
    P2: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EnumResourceLanguagesExA(hmodule: super::super::Foundation::HINSTANCE, lptype: ::windows::core::PCSTR, lpname: ::windows::core::PCSTR, lpenumfunc: *mut ::core::ffi::c_void, lparam: isize, dwflags: u32, langid: u16) -> super::super::Foundation::BOOL;
    }
    EnumResourceLanguagesExA(hmodule.into(), lptype.into(), lpname.into(), ::core::mem::transmute(lpenumfunc), lparam, dwflags, langid)
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumResourceLanguagesExW<'a, P0, P1, P2>(hmodule: P0, lptype: P1, lpname: P2, lpenumfunc: ENUMRESLANGPROCW, lparam: isize, dwflags: u32, langid: u16) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EnumResourceLanguagesExW(hmodule: super::super::Foundation::HINSTANCE, lptype: ::windows::core::PCWSTR, lpname: ::windows::core::PCWSTR, lpenumfunc: *mut ::core::ffi::c_void, lparam: isize, dwflags: u32, langid: u16) -> super::super::Foundation::BOOL;
    }
    EnumResourceLanguagesExW(hmodule.into(), lptype.into(), lpname.into(), ::core::mem::transmute(lpenumfunc), lparam, dwflags, langid)
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumResourceLanguagesW<'a, P0, P1, P2>(hmodule: P0, lptype: P1, lpname: P2, lpenumfunc: ENUMRESLANGPROCW, lparam: isize) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EnumResourceLanguagesW(hmodule: super::super::Foundation::HINSTANCE, lptype: ::windows::core::PCWSTR, lpname: ::windows::core::PCWSTR, lpenumfunc: *mut ::core::ffi::c_void, lparam: isize) -> super::super::Foundation::BOOL;
    }
    EnumResourceLanguagesW(hmodule.into(), lptype.into(), lpname.into(), ::core::mem::transmute(lpenumfunc), lparam)
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumResourceNamesA<'a, P0, P1>(hmodule: P0, lptype: P1, lpenumfunc: ENUMRESNAMEPROCA, lparam: isize) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EnumResourceNamesA(hmodule: super::super::Foundation::HINSTANCE, lptype: ::windows::core::PCSTR, lpenumfunc: *mut ::core::ffi::c_void, lparam: isize) -> super::super::Foundation::BOOL;
    }
    EnumResourceNamesA(hmodule.into(), lptype.into(), ::core::mem::transmute(lpenumfunc), lparam)
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumResourceNamesExA<'a, P0, P1>(hmodule: P0, lptype: P1, lpenumfunc: ENUMRESNAMEPROCA, lparam: isize, dwflags: u32, langid: u16) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EnumResourceNamesExA(hmodule: super::super::Foundation::HINSTANCE, lptype: ::windows::core::PCSTR, lpenumfunc: *mut ::core::ffi::c_void, lparam: isize, dwflags: u32, langid: u16) -> super::super::Foundation::BOOL;
    }
    EnumResourceNamesExA(hmodule.into(), lptype.into(), ::core::mem::transmute(lpenumfunc), lparam, dwflags, langid)
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumResourceNamesExW<'a, P0, P1>(hmodule: P0, lptype: P1, lpenumfunc: ENUMRESNAMEPROCW, lparam: isize, dwflags: u32, langid: u16) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EnumResourceNamesExW(hmodule: super::super::Foundation::HINSTANCE, lptype: ::windows::core::PCWSTR, lpenumfunc: *mut ::core::ffi::c_void, lparam: isize, dwflags: u32, langid: u16) -> super::super::Foundation::BOOL;
    }
    EnumResourceNamesExW(hmodule.into(), lptype.into(), ::core::mem::transmute(lpenumfunc), lparam, dwflags, langid)
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumResourceNamesW<'a, P0, P1>(hmodule: P0, lptype: P1, lpenumfunc: ENUMRESNAMEPROCW, lparam: isize) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EnumResourceNamesW(hmodule: super::super::Foundation::HINSTANCE, lptype: ::windows::core::PCWSTR, lpenumfunc: *mut ::core::ffi::c_void, lparam: isize) -> super::super::Foundation::BOOL;
    }
    EnumResourceNamesW(hmodule.into(), lptype.into(), ::core::mem::transmute(lpenumfunc), lparam)
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumResourceTypesA<'a, P0>(hmodule: P0, lpenumfunc: ENUMRESTYPEPROCA, lparam: isize) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EnumResourceTypesA(hmodule: super::super::Foundation::HINSTANCE, lpenumfunc: *mut ::core::ffi::c_void, lparam: isize) -> super::super::Foundation::BOOL;
    }
    EnumResourceTypesA(hmodule.into(), ::core::mem::transmute(lpenumfunc), lparam)
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumResourceTypesExA<'a, P0>(hmodule: P0, lpenumfunc: ENUMRESTYPEPROCA, lparam: isize, dwflags: u32, langid: u16) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EnumResourceTypesExA(hmodule: super::super::Foundation::HINSTANCE, lpenumfunc: *mut ::core::ffi::c_void, lparam: isize, dwflags: u32, langid: u16) -> super::super::Foundation::BOOL;
    }
    EnumResourceTypesExA(hmodule.into(), ::core::mem::transmute(lpenumfunc), lparam, dwflags, langid)
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumResourceTypesExW<'a, P0>(hmodule: P0, lpenumfunc: ENUMRESTYPEPROCW, lparam: isize, dwflags: u32, langid: u16) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EnumResourceTypesExW(hmodule: super::super::Foundation::HINSTANCE, lpenumfunc: *mut ::core::ffi::c_void, lparam: isize, dwflags: u32, langid: u16) -> super::super::Foundation::BOOL;
    }
    EnumResourceTypesExW(hmodule.into(), ::core::mem::transmute(lpenumfunc), lparam, dwflags, langid)
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumResourceTypesW<'a, P0>(hmodule: P0, lpenumfunc: ENUMRESTYPEPROCW, lparam: isize) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EnumResourceTypesW(hmodule: super::super::Foundation::HINSTANCE, lpenumfunc: *mut ::core::ffi::c_void, lparam: isize) -> super::super::Foundation::BOOL;
    }
    EnumResourceTypesW(hmodule.into(), ::core::mem::transmute(lpenumfunc), lparam)
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`*"]
pub const FIND_RESOURCE_DIRECTORY_LANGUAGES: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`*"]
pub const FIND_RESOURCE_DIRECTORY_NAMES: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`*"]
pub const FIND_RESOURCE_DIRECTORY_TYPES: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindResourceA<'a, P0, P1, P2>(hmodule: P0, lpname: P1, lptype: P2) -> ::windows::core::Result<super::super::Foundation::HRSRC>
where
    P0: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
    P2: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn FindResourceA(hmodule: super::super::Foundation::HINSTANCE, lpname: ::windows::core::PCSTR, lptype: ::windows::core::PCSTR) -> super::super::Foundation::HRSRC;
    }
    let result__ = FindResourceA(hmodule.into(), lpname.into(), lptype.into());
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindResourceExA<'a, P0, P1, P2>(hmodule: P0, lptype: P1, lpname: P2, wlanguage: u16) -> ::windows::core::Result<super::super::Foundation::HRSRC>
where
    P0: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
    P2: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn FindResourceExA(hmodule: super::super::Foundation::HINSTANCE, lptype: ::windows::core::PCSTR, lpname: ::windows::core::PCSTR, wlanguage: u16) -> super::super::Foundation::HRSRC;
    }
    let result__ = FindResourceExA(hmodule.into(), lptype.into(), lpname.into(), wlanguage);
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindResourceExW<'a, P0, P1, P2>(hmodule: P0, lptype: P1, lpname: P2, wlanguage: u16) -> super::super::Foundation::HRSRC
where
    P0: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn FindResourceExW(hmodule: super::super::Foundation::HINSTANCE, lptype: ::windows::core::PCWSTR, lpname: ::windows::core::PCWSTR, wlanguage: u16) -> super::super::Foundation::HRSRC;
    }
    FindResourceExW(hmodule.into(), lptype.into(), lpname.into(), wlanguage)
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindResourceW<'a, P0, P1, P2>(hmodule: P0, lpname: P1, lptype: P2) -> super::super::Foundation::HRSRC
where
    P0: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn FindResourceW(hmodule: super::super::Foundation::HINSTANCE, lpname: ::windows::core::PCWSTR, lptype: ::windows::core::PCWSTR) -> super::super::Foundation::HRSRC;
    }
    FindResourceW(hmodule.into(), lpname.into(), lptype.into())
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FreeLibrary<'a, P0>(hlibmodule: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn FreeLibrary(hlibmodule: super::super::Foundation::HINSTANCE) -> super::super::Foundation::BOOL;
    }
    FreeLibrary(hlibmodule.into())
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FreeLibraryAndExitThread<'a, P0>(hlibmodule: P0, dwexitcode: u32) -> !
where
    P0: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn FreeLibraryAndExitThread(hlibmodule: super::super::Foundation::HINSTANCE, dwexitcode: u32) -> !;
    }
    FreeLibraryAndExitThread(hlibmodule.into(), dwexitcode)
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FreeResource(hresdata: isize) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn FreeResource(hresdata: isize) -> super::super::Foundation::BOOL;
    }
    FreeResource(hresdata)
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`*"]
pub const GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`*"]
pub const GET_MODULE_HANDLE_EX_FLAG_PIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`*"]
pub const GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`*"]
#[inline]
pub unsafe fn GetDllDirectoryA(lpbuffer: &mut [u8]) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetDllDirectoryA(nbufferlength: u32, lpbuffer: ::windows::core::PSTR) -> u32;
    }
    GetDllDirectoryA(lpbuffer.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(lpbuffer)))
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`*"]
#[inline]
pub unsafe fn GetDllDirectoryW(lpbuffer: &mut [u16]) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetDllDirectoryW(nbufferlength: u32, lpbuffer: ::windows::core::PWSTR) -> u32;
    }
    GetDllDirectoryW(lpbuffer.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(lpbuffer)))
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetModuleFileNameA<'a, P0>(hmodule: P0, lpfilename: &mut [u8]) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetModuleFileNameA(hmodule: super::super::Foundation::HINSTANCE, lpfilename: ::windows::core::PSTR, nsize: u32) -> u32;
    }
    GetModuleFileNameA(hmodule.into(), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(lpfilename)), lpfilename.len() as _)
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetModuleFileNameW<'a, P0>(hmodule: P0, lpfilename: &mut [u16]) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetModuleFileNameW(hmodule: super::super::Foundation::HINSTANCE, lpfilename: ::windows::core::PWSTR, nsize: u32) -> u32;
    }
    GetModuleFileNameW(hmodule.into(), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(lpfilename)), lpfilename.len() as _)
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetModuleHandleA<'a, P0>(lpmodulename: P0) -> ::windows::core::Result<super::super::Foundation::HINSTANCE>
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetModuleHandleA(lpmodulename: ::windows::core::PCSTR) -> super::super::Foundation::HINSTANCE;
    }
    let result__ = GetModuleHandleA(lpmodulename.into());
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetModuleHandleExA<'a, P0>(dwflags: u32, lpmodulename: P0, phmodule: *mut super::super::Foundation::HINSTANCE) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetModuleHandleExA(dwflags: u32, lpmodulename: ::windows::core::PCSTR, phmodule: *mut super::super::Foundation::HINSTANCE) -> super::super::Foundation::BOOL;
    }
    GetModuleHandleExA(dwflags, lpmodulename.into(), ::core::mem::transmute(phmodule))
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetModuleHandleExW<'a, P0>(dwflags: u32, lpmodulename: P0, phmodule: *mut super::super::Foundation::HINSTANCE) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetModuleHandleExW(dwflags: u32, lpmodulename: ::windows::core::PCWSTR, phmodule: *mut super::super::Foundation::HINSTANCE) -> super::super::Foundation::BOOL;
    }
    GetModuleHandleExW(dwflags, lpmodulename.into(), ::core::mem::transmute(phmodule))
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetModuleHandleW<'a, P0>(lpmodulename: P0) -> ::windows::core::Result<super::super::Foundation::HINSTANCE>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetModuleHandleW(lpmodulename: ::windows::core::PCWSTR) -> super::super::Foundation::HINSTANCE;
    }
    let result__ = GetModuleHandleW(lpmodulename.into());
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetProcAddress<'a, P0, P1>(hmodule: P0, lpprocname: P1) -> super::super::Foundation::FARPROC
where
    P0: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetProcAddress(hmodule: super::super::Foundation::HINSTANCE, lpprocname: ::windows::core::PCSTR) -> super::super::Foundation::FARPROC;
    }
    GetProcAddress(hmodule.into(), lpprocname.into())
}
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
unsafe impl ::windows::core::Abi for LOAD_LIBRARY_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for LOAD_LIBRARY_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LOAD_LIBRARY_FLAGS").field(&self.0).finish()
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
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`*"]
pub const LOAD_LIBRARY_OS_INTEGRITY_CONTINUITY: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LoadLibraryA<'a, P0>(lplibfilename: P0) -> ::windows::core::Result<super::super::Foundation::HINSTANCE>
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn LoadLibraryA(lplibfilename: ::windows::core::PCSTR) -> super::super::Foundation::HINSTANCE;
    }
    let result__ = LoadLibraryA(lplibfilename.into());
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LoadLibraryExA<'a, P0, P1>(lplibfilename: P0, hfile: P1, dwflags: LOAD_LIBRARY_FLAGS) -> ::windows::core::Result<super::super::Foundation::HINSTANCE>
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn LoadLibraryExA(lplibfilename: ::windows::core::PCSTR, hfile: super::super::Foundation::HANDLE, dwflags: LOAD_LIBRARY_FLAGS) -> super::super::Foundation::HINSTANCE;
    }
    let result__ = LoadLibraryExA(lplibfilename.into(), hfile.into(), dwflags);
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LoadLibraryExW<'a, P0, P1>(lplibfilename: P0, hfile: P1, dwflags: LOAD_LIBRARY_FLAGS) -> ::windows::core::Result<super::super::Foundation::HINSTANCE>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn LoadLibraryExW(lplibfilename: ::windows::core::PCWSTR, hfile: super::super::Foundation::HANDLE, dwflags: LOAD_LIBRARY_FLAGS) -> super::super::Foundation::HINSTANCE;
    }
    let result__ = LoadLibraryExW(lplibfilename.into(), hfile.into(), dwflags);
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LoadLibraryW<'a, P0>(lplibfilename: P0) -> ::windows::core::Result<super::super::Foundation::HINSTANCE>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn LoadLibraryW(lplibfilename: ::windows::core::PCWSTR) -> super::super::Foundation::HINSTANCE;
    }
    let result__ = LoadLibraryW(lplibfilename.into());
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`*"]
#[inline]
pub unsafe fn LoadModule<'a, P0>(lpmodulename: P0, lpparameterblock: *const ::core::ffi::c_void) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn LoadModule(lpmodulename: ::windows::core::PCSTR, lpparameterblock: *const ::core::ffi::c_void) -> u32;
    }
    LoadModule(lpmodulename.into(), ::core::mem::transmute(lpparameterblock))
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LoadPackagedLibrary<'a, P0>(lpwlibfilename: P0, reserved: u32) -> ::windows::core::Result<super::super::Foundation::HINSTANCE>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn LoadPackagedLibrary(lpwlibfilename: ::windows::core::PCWSTR, reserved: u32) -> super::super::Foundation::HINSTANCE;
    }
    let result__ = LoadPackagedLibrary(lpwlibfilename.into(), reserved);
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LoadResource<'a, P0, P1>(hmodule: P0, hresinfo: P1) -> isize
where
    P0: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
    P1: ::std::convert::Into<super::super::Foundation::HRSRC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn LoadResource(hmodule: super::super::Foundation::HINSTANCE, hresinfo: super::super::Foundation::HRSRC) -> isize;
    }
    LoadResource(hmodule.into(), hresinfo.into())
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`*"]
#[inline]
pub unsafe fn LockResource(hresdata: isize) -> *mut ::core::ffi::c_void {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn LockResource(hresdata: isize) -> *mut ::core::ffi::c_void;
    }
    LockResource(hresdata)
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PGET_MODULE_HANDLE_EXA = ::core::option::Option<unsafe extern "system" fn(dwflags: u32, lpmodulename: ::windows::core::PCSTR, phmodule: *mut super::super::Foundation::HINSTANCE) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PGET_MODULE_HANDLE_EXW = ::core::option::Option<unsafe extern "system" fn(dwflags: u32, lpmodulename: ::windows::core::PCWSTR, phmodule: *mut super::super::Foundation::HINSTANCE) -> super::super::Foundation::BOOL>;
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
unsafe impl ::windows::core::Abi for REDIRECTION_DESCRIPTOR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for REDIRECTION_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<REDIRECTION_DESCRIPTOR>()) == 0 }
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
unsafe impl ::windows::core::Abi for REDIRECTION_FUNCTION_DESCRIPTOR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for REDIRECTION_FUNCTION_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<REDIRECTION_FUNCTION_DESCRIPTOR>()) == 0 }
    }
}
impl ::core::cmp::Eq for REDIRECTION_FUNCTION_DESCRIPTOR {}
impl ::core::default::Default for REDIRECTION_FUNCTION_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
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
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RemoveDllDirectory(cookie: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RemoveDllDirectory(cookie: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    }
    RemoveDllDirectory(::core::mem::transmute(cookie))
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`*"]
pub const SUPPORT_LANG_NUMBER: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetDefaultDllDirectories(directoryflags: LOAD_LIBRARY_FLAGS) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetDefaultDllDirectories(directoryflags: LOAD_LIBRARY_FLAGS) -> super::super::Foundation::BOOL;
    }
    SetDefaultDllDirectories(directoryflags)
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetDllDirectoryA<'a, P0>(lppathname: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetDllDirectoryA(lppathname: ::windows::core::PCSTR) -> super::super::Foundation::BOOL;
    }
    SetDllDirectoryA(lppathname.into())
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetDllDirectoryW<'a, P0>(lppathname: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetDllDirectoryW(lppathname: ::windows::core::PCWSTR) -> super::super::Foundation::BOOL;
    }
    SetDllDirectoryW(lppathname.into())
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SizeofResource<'a, P0, P1>(hmodule: P0, hresinfo: P1) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
    P1: ::std::convert::Into<super::super::Foundation::HRSRC>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SizeofResource(hmodule: super::super::Foundation::HINSTANCE, hresinfo: super::super::Foundation::HRSRC) -> u32;
    }
    SizeofResource(hmodule.into(), hresinfo.into())
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UpdateResourceA<'a, P0, P1, P2>(hupdate: P0, lptype: P1, lpname: P2, wlanguage: u16, lpdata: *const ::core::ffi::c_void, cb: u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
    P2: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn UpdateResourceA(hupdate: super::super::Foundation::HANDLE, lptype: ::windows::core::PCSTR, lpname: ::windows::core::PCSTR, wlanguage: u16, lpdata: *const ::core::ffi::c_void, cb: u32) -> super::super::Foundation::BOOL;
    }
    UpdateResourceA(hupdate.into(), lptype.into(), lpname.into(), wlanguage, ::core::mem::transmute(lpdata), cb)
}
#[doc = "*Required features: `\"Win32_System_LibraryLoader\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UpdateResourceW<'a, P0, P1, P2>(hupdate: P0, lptype: P1, lpname: P2, wlanguage: u16, lpdata: *const ::core::ffi::c_void, cb: u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn UpdateResourceW(hupdate: super::super::Foundation::HANDLE, lptype: ::windows::core::PCWSTR, lpname: ::windows::core::PCWSTR, wlanguage: u16, lpdata: *const ::core::ffi::c_void, cb: u32) -> super::super::Foundation::BOOL;
    }
    UpdateResourceW(hupdate.into(), lptype.into(), lpname.into(), wlanguage, ::core::mem::transmute(lpdata), cb)
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
