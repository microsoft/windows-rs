#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: 'Win32_System_LibraryLoader', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AddDllDirectory<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(newdirectory: Param0) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AddDllDirectory(newdirectory: super::super::Foundation::PWSTR) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(AddDllDirectory(newdirectory.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_LibraryLoader', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BeginUpdateResourceA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(pfilename: Param0, bdeleteexistingresources: Param1) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BeginUpdateResourceA(pfilename: super::super::Foundation::PSTR, bdeleteexistingresources: super::super::Foundation::BOOL) -> super::super::Foundation::HANDLE;
        }
        ::core::mem::transmute(BeginUpdateResourceA(pfilename.into_param().abi(), bdeleteexistingresources.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_LibraryLoader', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BeginUpdateResourceW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(pfilename: Param0, bdeleteexistingresources: Param1) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BeginUpdateResourceW(pfilename: super::super::Foundation::PWSTR, bdeleteexistingresources: super::super::Foundation::BOOL) -> super::super::Foundation::HANDLE;
        }
        ::core::mem::transmute(BeginUpdateResourceW(pfilename.into_param().abi(), bdeleteexistingresources.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_LibraryLoader'*"]
pub const CURRENT_IMPORT_REDIRECTION_VERSION: u32 = 1u32;
#[doc = "*Required features: 'Win32_System_LibraryLoader', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DisableThreadLibraryCalls<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HINSTANCE>>(hlibmodule: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DisableThreadLibraryCalls(hlibmodule: super::super::Foundation::HINSTANCE) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(DisableThreadLibraryCalls(hlibmodule.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_LibraryLoader', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type ENUMRESLANGPROCA = ::core::option::Option<unsafe extern "system" fn(hmodule: super::super::Foundation::HINSTANCE, lptype: super::super::Foundation::PSTR, lpname: super::super::Foundation::PSTR, wlanguage: u16, lparam: isize) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: 'Win32_System_LibraryLoader', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type ENUMRESLANGPROCW = ::core::option::Option<unsafe extern "system" fn(hmodule: super::super::Foundation::HINSTANCE, lptype: super::super::Foundation::PWSTR, lpname: super::super::Foundation::PWSTR, wlanguage: u16, lparam: isize) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: 'Win32_System_LibraryLoader', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type ENUMRESNAMEPROCA = ::core::option::Option<unsafe extern "system" fn(hmodule: super::super::Foundation::HINSTANCE, lptype: super::super::Foundation::PSTR, lpname: super::super::Foundation::PSTR, lparam: isize) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: 'Win32_System_LibraryLoader', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type ENUMRESNAMEPROCW = ::core::option::Option<unsafe extern "system" fn(hmodule: super::super::Foundation::HINSTANCE, lptype: super::super::Foundation::PWSTR, lpname: super::super::Foundation::PWSTR, lparam: isize) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: 'Win32_System_LibraryLoader', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type ENUMRESTYPEPROCA = ::core::option::Option<unsafe extern "system" fn(hmodule: super::super::Foundation::HINSTANCE, lptype: super::super::Foundation::PSTR, lparam: isize) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: 'Win32_System_LibraryLoader', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type ENUMRESTYPEPROCW = ::core::option::Option<unsafe extern "system" fn(hmodule: super::super::Foundation::HINSTANCE, lptype: super::super::Foundation::PWSTR, lparam: isize) -> super::super::Foundation::BOOL>;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_LibraryLoader'*"]
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
#[doc = "*Required features: 'Win32_System_LibraryLoader', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EndUpdateResourceA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(hupdate: Param0, fdiscard: Param1) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EndUpdateResourceA(hupdate: super::super::Foundation::HANDLE, fdiscard: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(EndUpdateResourceA(hupdate.into_param().abi(), fdiscard.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_LibraryLoader', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EndUpdateResourceW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(hupdate: Param0, fdiscard: Param1) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EndUpdateResourceW(hupdate: super::super::Foundation::HANDLE, fdiscard: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(EndUpdateResourceW(hupdate.into_param().abi(), fdiscard.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_LibraryLoader', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumResourceLanguagesA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HINSTANCE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(hmodule: Param0, lptype: Param1, lpname: Param2, lpenumfunc: ENUMRESLANGPROCA, lparam: isize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnumResourceLanguagesA(hmodule: super::super::Foundation::HINSTANCE, lptype: super::super::Foundation::PSTR, lpname: super::super::Foundation::PSTR, lpenumfunc: ::windows::core::RawPtr, lparam: isize) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(EnumResourceLanguagesA(hmodule.into_param().abi(), lptype.into_param().abi(), lpname.into_param().abi(), ::core::mem::transmute(lpenumfunc), ::core::mem::transmute(lparam)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_LibraryLoader', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumResourceLanguagesExA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HINSTANCE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(hmodule: Param0, lptype: Param1, lpname: Param2, lpenumfunc: ENUMRESLANGPROCA, lparam: isize, dwflags: u32, langid: u16) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnumResourceLanguagesExA(hmodule: super::super::Foundation::HINSTANCE, lptype: super::super::Foundation::PSTR, lpname: super::super::Foundation::PSTR, lpenumfunc: ::windows::core::RawPtr, lparam: isize, dwflags: u32, langid: u16) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(EnumResourceLanguagesExA(hmodule.into_param().abi(), lptype.into_param().abi(), lpname.into_param().abi(), ::core::mem::transmute(lpenumfunc), ::core::mem::transmute(lparam), ::core::mem::transmute(dwflags), ::core::mem::transmute(langid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_LibraryLoader', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumResourceLanguagesExW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HINSTANCE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hmodule: Param0, lptype: Param1, lpname: Param2, lpenumfunc: ENUMRESLANGPROCW, lparam: isize, dwflags: u32, langid: u16) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnumResourceLanguagesExW(hmodule: super::super::Foundation::HINSTANCE, lptype: super::super::Foundation::PWSTR, lpname: super::super::Foundation::PWSTR, lpenumfunc: ::windows::core::RawPtr, lparam: isize, dwflags: u32, langid: u16) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(EnumResourceLanguagesExW(hmodule.into_param().abi(), lptype.into_param().abi(), lpname.into_param().abi(), ::core::mem::transmute(lpenumfunc), ::core::mem::transmute(lparam), ::core::mem::transmute(dwflags), ::core::mem::transmute(langid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_LibraryLoader', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumResourceLanguagesW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HINSTANCE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hmodule: Param0, lptype: Param1, lpname: Param2, lpenumfunc: ENUMRESLANGPROCW, lparam: isize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnumResourceLanguagesW(hmodule: super::super::Foundation::HINSTANCE, lptype: super::super::Foundation::PWSTR, lpname: super::super::Foundation::PWSTR, lpenumfunc: ::windows::core::RawPtr, lparam: isize) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(EnumResourceLanguagesW(hmodule.into_param().abi(), lptype.into_param().abi(), lpname.into_param().abi(), ::core::mem::transmute(lpenumfunc), ::core::mem::transmute(lparam)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_LibraryLoader', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumResourceNamesA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HINSTANCE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(hmodule: Param0, lptype: Param1, lpenumfunc: ENUMRESNAMEPROCA, lparam: isize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnumResourceNamesA(hmodule: super::super::Foundation::HINSTANCE, lptype: super::super::Foundation::PSTR, lpenumfunc: ::windows::core::RawPtr, lparam: isize) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(EnumResourceNamesA(hmodule.into_param().abi(), lptype.into_param().abi(), ::core::mem::transmute(lpenumfunc), ::core::mem::transmute(lparam)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_LibraryLoader', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumResourceNamesExA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HINSTANCE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(hmodule: Param0, lptype: Param1, lpenumfunc: ENUMRESNAMEPROCA, lparam: isize, dwflags: u32, langid: u16) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnumResourceNamesExA(hmodule: super::super::Foundation::HINSTANCE, lptype: super::super::Foundation::PSTR, lpenumfunc: ::windows::core::RawPtr, lparam: isize, dwflags: u32, langid: u16) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(EnumResourceNamesExA(hmodule.into_param().abi(), lptype.into_param().abi(), ::core::mem::transmute(lpenumfunc), ::core::mem::transmute(lparam), ::core::mem::transmute(dwflags), ::core::mem::transmute(langid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_LibraryLoader', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumResourceNamesExW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HINSTANCE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hmodule: Param0, lptype: Param1, lpenumfunc: ENUMRESNAMEPROCW, lparam: isize, dwflags: u32, langid: u16) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnumResourceNamesExW(hmodule: super::super::Foundation::HINSTANCE, lptype: super::super::Foundation::PWSTR, lpenumfunc: ::windows::core::RawPtr, lparam: isize, dwflags: u32, langid: u16) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(EnumResourceNamesExW(hmodule.into_param().abi(), lptype.into_param().abi(), ::core::mem::transmute(lpenumfunc), ::core::mem::transmute(lparam), ::core::mem::transmute(dwflags), ::core::mem::transmute(langid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_LibraryLoader', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumResourceNamesW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HINSTANCE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hmodule: Param0, lptype: Param1, lpenumfunc: ENUMRESNAMEPROCW, lparam: isize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnumResourceNamesW(hmodule: super::super::Foundation::HINSTANCE, lptype: super::super::Foundation::PWSTR, lpenumfunc: ::windows::core::RawPtr, lparam: isize) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(EnumResourceNamesW(hmodule.into_param().abi(), lptype.into_param().abi(), ::core::mem::transmute(lpenumfunc), ::core::mem::transmute(lparam)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_LibraryLoader', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumResourceTypesA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HINSTANCE>>(hmodule: Param0, lpenumfunc: ENUMRESTYPEPROCA, lparam: isize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnumResourceTypesA(hmodule: super::super::Foundation::HINSTANCE, lpenumfunc: ::windows::core::RawPtr, lparam: isize) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(EnumResourceTypesA(hmodule.into_param().abi(), ::core::mem::transmute(lpenumfunc), ::core::mem::transmute(lparam)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_LibraryLoader', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumResourceTypesExA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HINSTANCE>>(hmodule: Param0, lpenumfunc: ENUMRESTYPEPROCA, lparam: isize, dwflags: u32, langid: u16) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnumResourceTypesExA(hmodule: super::super::Foundation::HINSTANCE, lpenumfunc: ::windows::core::RawPtr, lparam: isize, dwflags: u32, langid: u16) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(EnumResourceTypesExA(hmodule.into_param().abi(), ::core::mem::transmute(lpenumfunc), ::core::mem::transmute(lparam), ::core::mem::transmute(dwflags), ::core::mem::transmute(langid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_LibraryLoader', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumResourceTypesExW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HINSTANCE>>(hmodule: Param0, lpenumfunc: ENUMRESTYPEPROCW, lparam: isize, dwflags: u32, langid: u16) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnumResourceTypesExW(hmodule: super::super::Foundation::HINSTANCE, lpenumfunc: ::windows::core::RawPtr, lparam: isize, dwflags: u32, langid: u16) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(EnumResourceTypesExW(hmodule.into_param().abi(), ::core::mem::transmute(lpenumfunc), ::core::mem::transmute(lparam), ::core::mem::transmute(dwflags), ::core::mem::transmute(langid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_LibraryLoader', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumResourceTypesW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HINSTANCE>>(hmodule: Param0, lpenumfunc: ENUMRESTYPEPROCW, lparam: isize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnumResourceTypesW(hmodule: super::super::Foundation::HINSTANCE, lpenumfunc: ::windows::core::RawPtr, lparam: isize) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(EnumResourceTypesW(hmodule.into_param().abi(), ::core::mem::transmute(lpenumfunc), ::core::mem::transmute(lparam)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_LibraryLoader'*"]
pub const FIND_RESOURCE_DIRECTORY_LANGUAGES: u32 = 1024u32;
#[doc = "*Required features: 'Win32_System_LibraryLoader'*"]
pub const FIND_RESOURCE_DIRECTORY_NAMES: u32 = 512u32;
#[doc = "*Required features: 'Win32_System_LibraryLoader'*"]
pub const FIND_RESOURCE_DIRECTORY_TYPES: u32 = 256u32;
#[doc = "*Required features: 'Win32_System_LibraryLoader', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindResourceA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HINSTANCE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(hmodule: Param0, lpname: Param1, lptype: Param2) -> super::super::Foundation::HRSRC {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FindResourceA(hmodule: super::super::Foundation::HINSTANCE, lpname: super::super::Foundation::PSTR, lptype: super::super::Foundation::PSTR) -> super::super::Foundation::HRSRC;
        }
        ::core::mem::transmute(FindResourceA(hmodule.into_param().abi(), lpname.into_param().abi(), lptype.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_LibraryLoader', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindResourceExA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HINSTANCE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(hmodule: Param0, lptype: Param1, lpname: Param2, wlanguage: u16) -> super::super::Foundation::HRSRC {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FindResourceExA(hmodule: super::super::Foundation::HINSTANCE, lptype: super::super::Foundation::PSTR, lpname: super::super::Foundation::PSTR, wlanguage: u16) -> super::super::Foundation::HRSRC;
        }
        ::core::mem::transmute(FindResourceExA(hmodule.into_param().abi(), lptype.into_param().abi(), lpname.into_param().abi(), ::core::mem::transmute(wlanguage)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_LibraryLoader', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindResourceExW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HINSTANCE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hmodule: Param0, lptype: Param1, lpname: Param2, wlanguage: u16) -> super::super::Foundation::HRSRC {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FindResourceExW(hmodule: super::super::Foundation::HINSTANCE, lptype: super::super::Foundation::PWSTR, lpname: super::super::Foundation::PWSTR, wlanguage: u16) -> super::super::Foundation::HRSRC;
        }
        ::core::mem::transmute(FindResourceExW(hmodule.into_param().abi(), lptype.into_param().abi(), lpname.into_param().abi(), ::core::mem::transmute(wlanguage)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_LibraryLoader', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindResourceW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HINSTANCE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hmodule: Param0, lpname: Param1, lptype: Param2) -> super::super::Foundation::HRSRC {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FindResourceW(hmodule: super::super::Foundation::HINSTANCE, lpname: super::super::Foundation::PWSTR, lptype: super::super::Foundation::PWSTR) -> super::super::Foundation::HRSRC;
        }
        ::core::mem::transmute(FindResourceW(hmodule.into_param().abi(), lpname.into_param().abi(), lptype.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_LibraryLoader', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FreeLibrary<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HINSTANCE>>(hlibmodule: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FreeLibrary(hlibmodule: super::super::Foundation::HINSTANCE) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FreeLibrary(hlibmodule.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_LibraryLoader', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FreeLibraryAndExitThread<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HINSTANCE>>(hlibmodule: Param0, dwexitcode: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FreeLibraryAndExitThread(hlibmodule: super::super::Foundation::HINSTANCE, dwexitcode: u32);
        }
        FreeLibraryAndExitThread(hlibmodule.into_param().abi(), ::core::mem::transmute(dwexitcode))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_LibraryLoader', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FreeResource(hresdata: isize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FreeResource(hresdata: isize) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FreeResource(::core::mem::transmute(hresdata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_LibraryLoader'*"]
pub const GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS: u32 = 4u32;
#[doc = "*Required features: 'Win32_System_LibraryLoader'*"]
pub const GET_MODULE_HANDLE_EX_FLAG_PIN: u32 = 1u32;
#[doc = "*Required features: 'Win32_System_LibraryLoader'*"]
pub const GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT: u32 = 2u32;
#[doc = "*Required features: 'Win32_System_LibraryLoader', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetDllDirectoryA(nbufferlength: u32, lpbuffer: super::super::Foundation::PSTR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetDllDirectoryA(nbufferlength: u32, lpbuffer: super::super::Foundation::PSTR) -> u32;
        }
        ::core::mem::transmute(GetDllDirectoryA(::core::mem::transmute(nbufferlength), ::core::mem::transmute(lpbuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_LibraryLoader', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetDllDirectoryW(nbufferlength: u32, lpbuffer: super::super::Foundation::PWSTR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetDllDirectoryW(nbufferlength: u32, lpbuffer: super::super::Foundation::PWSTR) -> u32;
        }
        ::core::mem::transmute(GetDllDirectoryW(::core::mem::transmute(nbufferlength), ::core::mem::transmute(lpbuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_LibraryLoader', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetModuleFileNameA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HINSTANCE>>(hmodule: Param0, lpfilename: super::super::Foundation::PSTR, nsize: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetModuleFileNameA(hmodule: super::super::Foundation::HINSTANCE, lpfilename: super::super::Foundation::PSTR, nsize: u32) -> u32;
        }
        ::core::mem::transmute(GetModuleFileNameA(hmodule.into_param().abi(), ::core::mem::transmute(lpfilename), ::core::mem::transmute(nsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_LibraryLoader', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetModuleFileNameW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HINSTANCE>>(hmodule: Param0, lpfilename: super::super::Foundation::PWSTR, nsize: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetModuleFileNameW(hmodule: super::super::Foundation::HINSTANCE, lpfilename: super::super::Foundation::PWSTR, nsize: u32) -> u32;
        }
        ::core::mem::transmute(GetModuleFileNameW(hmodule.into_param().abi(), ::core::mem::transmute(lpfilename), ::core::mem::transmute(nsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_LibraryLoader', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetModuleHandleA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(lpmodulename: Param0) -> super::super::Foundation::HINSTANCE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetModuleHandleA(lpmodulename: super::super::Foundation::PSTR) -> super::super::Foundation::HINSTANCE;
        }
        ::core::mem::transmute(GetModuleHandleA(lpmodulename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_LibraryLoader', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetModuleHandleExA<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(dwflags: u32, lpmodulename: Param1, phmodule: *mut super::super::Foundation::HINSTANCE) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetModuleHandleExA(dwflags: u32, lpmodulename: super::super::Foundation::PSTR, phmodule: *mut super::super::Foundation::HINSTANCE) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetModuleHandleExA(::core::mem::transmute(dwflags), lpmodulename.into_param().abi(), ::core::mem::transmute(phmodule)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_LibraryLoader', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetModuleHandleExW<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(dwflags: u32, lpmodulename: Param1, phmodule: *mut super::super::Foundation::HINSTANCE) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetModuleHandleExW(dwflags: u32, lpmodulename: super::super::Foundation::PWSTR, phmodule: *mut super::super::Foundation::HINSTANCE) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetModuleHandleExW(::core::mem::transmute(dwflags), lpmodulename.into_param().abi(), ::core::mem::transmute(phmodule)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_LibraryLoader', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetModuleHandleW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(lpmodulename: Param0) -> super::super::Foundation::HINSTANCE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetModuleHandleW(lpmodulename: super::super::Foundation::PWSTR) -> super::super::Foundation::HINSTANCE;
        }
        ::core::mem::transmute(GetModuleHandleW(lpmodulename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_LibraryLoader', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetProcAddress<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HINSTANCE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(hmodule: Param0, lpprocname: Param1) -> super::super::Foundation::FARPROC {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetProcAddress(hmodule: super::super::Foundation::HINSTANCE, lpprocname: super::super::Foundation::PSTR) -> super::super::Foundation::FARPROC;
        }
        ::core::mem::transmute(GetProcAddress(hmodule.into_param().abi(), lpprocname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_LibraryLoader'*"]
pub type LOAD_LIBRARY_FLAGS = u32;
#[doc = "*Required features: 'Win32_System_LibraryLoader'*"]
pub const DONT_RESOLVE_DLL_REFERENCES: LOAD_LIBRARY_FLAGS = 1u32;
#[doc = "*Required features: 'Win32_System_LibraryLoader'*"]
pub const LOAD_LIBRARY_AS_DATAFILE: LOAD_LIBRARY_FLAGS = 2u32;
#[doc = "*Required features: 'Win32_System_LibraryLoader'*"]
pub const LOAD_WITH_ALTERED_SEARCH_PATH: LOAD_LIBRARY_FLAGS = 8u32;
#[doc = "*Required features: 'Win32_System_LibraryLoader'*"]
pub const LOAD_IGNORE_CODE_AUTHZ_LEVEL: LOAD_LIBRARY_FLAGS = 16u32;
#[doc = "*Required features: 'Win32_System_LibraryLoader'*"]
pub const LOAD_LIBRARY_AS_IMAGE_RESOURCE: LOAD_LIBRARY_FLAGS = 32u32;
#[doc = "*Required features: 'Win32_System_LibraryLoader'*"]
pub const LOAD_LIBRARY_AS_DATAFILE_EXCLUSIVE: LOAD_LIBRARY_FLAGS = 64u32;
#[doc = "*Required features: 'Win32_System_LibraryLoader'*"]
pub const LOAD_LIBRARY_REQUIRE_SIGNED_TARGET: LOAD_LIBRARY_FLAGS = 128u32;
#[doc = "*Required features: 'Win32_System_LibraryLoader'*"]
pub const LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR: LOAD_LIBRARY_FLAGS = 256u32;
#[doc = "*Required features: 'Win32_System_LibraryLoader'*"]
pub const LOAD_LIBRARY_SEARCH_APPLICATION_DIR: LOAD_LIBRARY_FLAGS = 512u32;
#[doc = "*Required features: 'Win32_System_LibraryLoader'*"]
pub const LOAD_LIBRARY_SEARCH_USER_DIRS: LOAD_LIBRARY_FLAGS = 1024u32;
#[doc = "*Required features: 'Win32_System_LibraryLoader'*"]
pub const LOAD_LIBRARY_SEARCH_SYSTEM32: LOAD_LIBRARY_FLAGS = 2048u32;
#[doc = "*Required features: 'Win32_System_LibraryLoader'*"]
pub const LOAD_LIBRARY_SEARCH_DEFAULT_DIRS: LOAD_LIBRARY_FLAGS = 4096u32;
#[doc = "*Required features: 'Win32_System_LibraryLoader'*"]
pub const LOAD_LIBRARY_SAFE_CURRENT_DIRS: LOAD_LIBRARY_FLAGS = 8192u32;
#[doc = "*Required features: 'Win32_System_LibraryLoader'*"]
pub const LOAD_LIBRARY_SEARCH_SYSTEM32_NO_FORWARDER: LOAD_LIBRARY_FLAGS = 16384u32;
#[doc = "*Required features: 'Win32_System_LibraryLoader'*"]
pub const LOAD_LIBRARY_OS_INTEGRITY_CONTINUITY: u32 = 32768u32;
#[doc = "*Required features: 'Win32_System_LibraryLoader', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LoadLibraryA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(lplibfilename: Param0) -> super::super::Foundation::HINSTANCE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LoadLibraryA(lplibfilename: super::super::Foundation::PSTR) -> super::super::Foundation::HINSTANCE;
        }
        ::core::mem::transmute(LoadLibraryA(lplibfilename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_LibraryLoader', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LoadLibraryExA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(lplibfilename: Param0, hfile: Param1, dwflags: LOAD_LIBRARY_FLAGS) -> super::super::Foundation::HINSTANCE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LoadLibraryExA(lplibfilename: super::super::Foundation::PSTR, hfile: super::super::Foundation::HANDLE, dwflags: LOAD_LIBRARY_FLAGS) -> super::super::Foundation::HINSTANCE;
        }
        ::core::mem::transmute(LoadLibraryExA(lplibfilename.into_param().abi(), hfile.into_param().abi(), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_LibraryLoader', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LoadLibraryExW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(lplibfilename: Param0, hfile: Param1, dwflags: LOAD_LIBRARY_FLAGS) -> super::super::Foundation::HINSTANCE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LoadLibraryExW(lplibfilename: super::super::Foundation::PWSTR, hfile: super::super::Foundation::HANDLE, dwflags: LOAD_LIBRARY_FLAGS) -> super::super::Foundation::HINSTANCE;
        }
        ::core::mem::transmute(LoadLibraryExW(lplibfilename.into_param().abi(), hfile.into_param().abi(), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_LibraryLoader', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LoadLibraryW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(lplibfilename: Param0) -> super::super::Foundation::HINSTANCE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LoadLibraryW(lplibfilename: super::super::Foundation::PWSTR) -> super::super::Foundation::HINSTANCE;
        }
        ::core::mem::transmute(LoadLibraryW(lplibfilename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_LibraryLoader', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LoadModule<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(lpmodulename: Param0, lpparameterblock: *const ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LoadModule(lpmodulename: super::super::Foundation::PSTR, lpparameterblock: *const ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(LoadModule(lpmodulename.into_param().abi(), ::core::mem::transmute(lpparameterblock)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_LibraryLoader', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LoadPackagedLibrary<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(lpwlibfilename: Param0, reserved: u32) -> super::super::Foundation::HINSTANCE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LoadPackagedLibrary(lpwlibfilename: super::super::Foundation::PWSTR, reserved: u32) -> super::super::Foundation::HINSTANCE;
        }
        ::core::mem::transmute(LoadPackagedLibrary(lpwlibfilename.into_param().abi(), ::core::mem::transmute(reserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_LibraryLoader', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LoadResource<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HINSTANCE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HRSRC>>(hmodule: Param0, hresinfo: Param1) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LoadResource(hmodule: super::super::Foundation::HINSTANCE, hresinfo: super::super::Foundation::HRSRC) -> isize;
        }
        ::core::mem::transmute(LoadResource(hmodule.into_param().abi(), hresinfo.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_LibraryLoader'*"]
#[inline]
pub unsafe fn LockResource(hresdata: isize) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LockResource(hresdata: isize) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(LockResource(::core::mem::transmute(hresdata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_LibraryLoader', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type PGET_MODULE_HANDLE_EXA = ::core::option::Option<unsafe extern "system" fn(dwflags: u32, lpmodulename: super::super::Foundation::PSTR, phmodule: *mut super::super::Foundation::HINSTANCE) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: 'Win32_System_LibraryLoader', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type PGET_MODULE_HANDLE_EXW = ::core::option::Option<unsafe extern "system" fn(dwflags: u32, lpmodulename: super::super::Foundation::PWSTR, phmodule: *mut super::super::Foundation::HINSTANCE) -> super::super::Foundation::BOOL>;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_LibraryLoader', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct REDIRECTION_DESCRIPTOR {
    pub Version: u32,
    pub FunctionCount: u32,
    pub Redirections: *mut REDIRECTION_FUNCTION_DESCRIPTOR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for REDIRECTION_DESCRIPTOR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for REDIRECTION_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for REDIRECTION_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REDIRECTION_DESCRIPTOR").field("Version", &self.Version).field("FunctionCount", &self.FunctionCount).field("Redirections", &self.Redirections).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for REDIRECTION_DESCRIPTOR {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for REDIRECTION_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<REDIRECTION_DESCRIPTOR>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for REDIRECTION_DESCRIPTOR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for REDIRECTION_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_LibraryLoader', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct REDIRECTION_FUNCTION_DESCRIPTOR {
    pub DllName: super::super::Foundation::PSTR,
    pub FunctionName: super::super::Foundation::PSTR,
    pub RedirectionTarget: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for REDIRECTION_FUNCTION_DESCRIPTOR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for REDIRECTION_FUNCTION_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for REDIRECTION_FUNCTION_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REDIRECTION_FUNCTION_DESCRIPTOR").field("DllName", &self.DllName).field("FunctionName", &self.FunctionName).field("RedirectionTarget", &self.RedirectionTarget).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for REDIRECTION_FUNCTION_DESCRIPTOR {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for REDIRECTION_FUNCTION_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<REDIRECTION_FUNCTION_DESCRIPTOR>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for REDIRECTION_FUNCTION_DESCRIPTOR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for REDIRECTION_FUNCTION_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_LibraryLoader'*"]
pub const RESOURCE_ENUM_LN: u32 = 1u32;
#[doc = "*Required features: 'Win32_System_LibraryLoader'*"]
pub const RESOURCE_ENUM_MODULE_EXACT: u32 = 16u32;
#[doc = "*Required features: 'Win32_System_LibraryLoader'*"]
pub const RESOURCE_ENUM_MUI: u32 = 2u32;
#[doc = "*Required features: 'Win32_System_LibraryLoader'*"]
pub const RESOURCE_ENUM_MUI_SYSTEM: u32 = 4u32;
#[doc = "*Required features: 'Win32_System_LibraryLoader'*"]
pub const RESOURCE_ENUM_VALIDATE: u32 = 8u32;
#[doc = "*Required features: 'Win32_System_LibraryLoader', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RemoveDllDirectory(cookie: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RemoveDllDirectory(cookie: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(RemoveDllDirectory(::core::mem::transmute(cookie)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_LibraryLoader'*"]
pub const SUPPORT_LANG_NUMBER: u32 = 32u32;
#[doc = "*Required features: 'Win32_System_LibraryLoader', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetDefaultDllDirectories(directoryflags: LOAD_LIBRARY_FLAGS) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetDefaultDllDirectories(directoryflags: LOAD_LIBRARY_FLAGS) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetDefaultDllDirectories(::core::mem::transmute(directoryflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_LibraryLoader', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetDllDirectoryA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(lppathname: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetDllDirectoryA(lppathname: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetDllDirectoryA(lppathname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_LibraryLoader', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetDllDirectoryW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(lppathname: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetDllDirectoryW(lppathname: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetDllDirectoryW(lppathname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_LibraryLoader', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SizeofResource<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HINSTANCE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HRSRC>>(hmodule: Param0, hresinfo: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SizeofResource(hmodule: super::super::Foundation::HINSTANCE, hresinfo: super::super::Foundation::HRSRC) -> u32;
        }
        ::core::mem::transmute(SizeofResource(hmodule.into_param().abi(), hresinfo.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_LibraryLoader', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UpdateResourceA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(hupdate: Param0, lptype: Param1, lpname: Param2, wlanguage: u16, lpdata: *const ::core::ffi::c_void, cb: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UpdateResourceA(hupdate: super::super::Foundation::HANDLE, lptype: super::super::Foundation::PSTR, lpname: super::super::Foundation::PSTR, wlanguage: u16, lpdata: *const ::core::ffi::c_void, cb: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(UpdateResourceA(hupdate.into_param().abi(), lptype.into_param().abi(), lpname.into_param().abi(), ::core::mem::transmute(wlanguage), ::core::mem::transmute(lpdata), ::core::mem::transmute(cb)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_LibraryLoader', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UpdateResourceW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hupdate: Param0, lptype: Param1, lpname: Param2, wlanguage: u16, lpdata: *const ::core::ffi::c_void, cb: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UpdateResourceW(hupdate: super::super::Foundation::HANDLE, lptype: super::super::Foundation::PWSTR, lpname: super::super::Foundation::PWSTR, wlanguage: u16, lpdata: *const ::core::ffi::c_void, cb: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(UpdateResourceW(hupdate.into_param().abi(), lptype.into_param().abi(), lpname.into_param().abi(), ::core::mem::transmute(wlanguage), ::core::mem::transmute(lpdata), ::core::mem::transmute(cb)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
