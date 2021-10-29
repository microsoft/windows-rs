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
pub unsafe fn AddDllDirectory<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    newdirectory: Param0,
) -> *mut ::std::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AddDllDirectory(
                newdirectory: super::super::Foundation::PWSTR,
            ) -> *mut ::std::ffi::c_void;
        }
        ::std::mem::transmute(AddDllDirectory(newdirectory.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BeginUpdateResourceA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    pfilename: Param0,
    bdeleteexistingresources: Param1,
) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BeginUpdateResourceA(
                pfilename: super::super::Foundation::PSTR,
                bdeleteexistingresources: super::super::Foundation::BOOL,
            ) -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(BeginUpdateResourceA(
            pfilename.into_param().abi(),
            bdeleteexistingresources.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BeginUpdateResourceW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    pfilename: Param0,
    bdeleteexistingresources: Param1,
) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BeginUpdateResourceW(
                pfilename: super::super::Foundation::PWSTR,
                bdeleteexistingresources: super::super::Foundation::BOOL,
            ) -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(BeginUpdateResourceW(
            pfilename.into_param().abi(),
            bdeleteexistingresources.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const CURRENT_IMPORT_REDIRECTION_VERSION: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DisableThreadLibraryCalls<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HINSTANCE>,
>(
    hlibmodule: Param0,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DisableThreadLibraryCalls(
                hlibmodule: super::super::Foundation::HINSTANCE,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(DisableThreadLibraryCalls(hlibmodule.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub type ENUMRESLANGPROCA = unsafe extern "system" fn(
    hmodule: super::super::Foundation::HINSTANCE,
    lptype: super::super::Foundation::PSTR,
    lpname: super::super::Foundation::PSTR,
    wlanguage: u16,
    lparam: isize,
) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type ENUMRESLANGPROCW = unsafe extern "system" fn(
    hmodule: super::super::Foundation::HINSTANCE,
    lptype: super::super::Foundation::PWSTR,
    lpname: super::super::Foundation::PWSTR,
    wlanguage: u16,
    lparam: isize,
) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type ENUMRESNAMEPROCA = unsafe extern "system" fn(
    hmodule: super::super::Foundation::HINSTANCE,
    lptype: super::super::Foundation::PSTR,
    lpname: super::super::Foundation::PSTR,
    lparam: isize,
) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type ENUMRESNAMEPROCW = unsafe extern "system" fn(
    hmodule: super::super::Foundation::HINSTANCE,
    lptype: super::super::Foundation::PWSTR,
    lpname: super::super::Foundation::PWSTR,
    lparam: isize,
) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type ENUMRESTYPEPROCA = unsafe extern "system" fn(
    hmodule: super::super::Foundation::HINSTANCE,
    lptype: super::super::Foundation::PSTR,
    lparam: isize,
) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type ENUMRESTYPEPROCW = unsafe extern "system" fn(
    hmodule: super::super::Foundation::HINSTANCE,
    lptype: super::super::Foundation::PWSTR,
    lparam: isize,
) -> super::super::Foundation::BOOL;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct ENUMUILANG {
    pub NumOfEnumUILang: u32,
    pub SizeOfEnumUIBuffer: u32,
    pub pEnumUIBuffer: *mut u16,
}
impl ENUMUILANG {}
impl ::std::default::Default for ENUMUILANG {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for ENUMUILANG {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ENUMUILANG")
            .field("NumOfEnumUILang", &self.NumOfEnumUILang)
            .field("SizeOfEnumUIBuffer", &self.SizeOfEnumUIBuffer)
            .field("pEnumUIBuffer", &self.pEnumUIBuffer)
            .finish()
    }
}
impl ::std::cmp::PartialEq for ENUMUILANG {
    fn eq(&self, other: &Self) -> bool {
        self.NumOfEnumUILang == other.NumOfEnumUILang
            && self.SizeOfEnumUIBuffer == other.SizeOfEnumUIBuffer
            && self.pEnumUIBuffer == other.pEnumUIBuffer
    }
}
impl ::std::cmp::Eq for ENUMUILANG {}
unsafe impl ::windows::runtime::Abi for ENUMUILANG {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EndUpdateResourceA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    hupdate: Param0,
    fdiscard: Param1,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EndUpdateResourceA(
                hupdate: super::super::Foundation::HANDLE,
                fdiscard: super::super::Foundation::BOOL,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(EndUpdateResourceA(
            hupdate.into_param().abi(),
            fdiscard.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EndUpdateResourceW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    hupdate: Param0,
    fdiscard: Param1,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EndUpdateResourceW(
                hupdate: super::super::Foundation::HANDLE,
                fdiscard: super::super::Foundation::BOOL,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(EndUpdateResourceW(
            hupdate.into_param().abi(),
            fdiscard.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumResourceLanguagesA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HINSTANCE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hmodule: Param0,
    lptype: Param1,
    lpname: Param2,
    lpenumfunc: ::std::option::Option<ENUMRESLANGPROCA>,
    lparam: isize,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnumResourceLanguagesA(
                hmodule: super::super::Foundation::HINSTANCE,
                lptype: super::super::Foundation::PSTR,
                lpname: super::super::Foundation::PSTR,
                lpenumfunc: ::windows::runtime::RawPtr,
                lparam: isize,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(EnumResourceLanguagesA(
            hmodule.into_param().abi(),
            lptype.into_param().abi(),
            lpname.into_param().abi(),
            ::std::mem::transmute(lpenumfunc),
            ::std::mem::transmute(lparam),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumResourceLanguagesExA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HINSTANCE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hmodule: Param0,
    lptype: Param1,
    lpname: Param2,
    lpenumfunc: ::std::option::Option<ENUMRESLANGPROCA>,
    lparam: isize,
    dwflags: u32,
    langid: u16,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnumResourceLanguagesExA(
                hmodule: super::super::Foundation::HINSTANCE,
                lptype: super::super::Foundation::PSTR,
                lpname: super::super::Foundation::PSTR,
                lpenumfunc: ::windows::runtime::RawPtr,
                lparam: isize,
                dwflags: u32,
                langid: u16,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(EnumResourceLanguagesExA(
            hmodule.into_param().abi(),
            lptype.into_param().abi(),
            lpname.into_param().abi(),
            ::std::mem::transmute(lpenumfunc),
            ::std::mem::transmute(lparam),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(langid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumResourceLanguagesExW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HINSTANCE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hmodule: Param0,
    lptype: Param1,
    lpname: Param2,
    lpenumfunc: ::std::option::Option<ENUMRESLANGPROCW>,
    lparam: isize,
    dwflags: u32,
    langid: u16,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnumResourceLanguagesExW(
                hmodule: super::super::Foundation::HINSTANCE,
                lptype: super::super::Foundation::PWSTR,
                lpname: super::super::Foundation::PWSTR,
                lpenumfunc: ::windows::runtime::RawPtr,
                lparam: isize,
                dwflags: u32,
                langid: u16,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(EnumResourceLanguagesExW(
            hmodule.into_param().abi(),
            lptype.into_param().abi(),
            lpname.into_param().abi(),
            ::std::mem::transmute(lpenumfunc),
            ::std::mem::transmute(lparam),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(langid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumResourceLanguagesW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HINSTANCE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hmodule: Param0,
    lptype: Param1,
    lpname: Param2,
    lpenumfunc: ::std::option::Option<ENUMRESLANGPROCW>,
    lparam: isize,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnumResourceLanguagesW(
                hmodule: super::super::Foundation::HINSTANCE,
                lptype: super::super::Foundation::PWSTR,
                lpname: super::super::Foundation::PWSTR,
                lpenumfunc: ::windows::runtime::RawPtr,
                lparam: isize,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(EnumResourceLanguagesW(
            hmodule.into_param().abi(),
            lptype.into_param().abi(),
            lpname.into_param().abi(),
            ::std::mem::transmute(lpenumfunc),
            ::std::mem::transmute(lparam),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumResourceNamesA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HINSTANCE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hmodule: Param0,
    lptype: Param1,
    lpenumfunc: ::std::option::Option<ENUMRESNAMEPROCA>,
    lparam: isize,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnumResourceNamesA(
                hmodule: super::super::Foundation::HINSTANCE,
                lptype: super::super::Foundation::PSTR,
                lpenumfunc: ::windows::runtime::RawPtr,
                lparam: isize,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(EnumResourceNamesA(
            hmodule.into_param().abi(),
            lptype.into_param().abi(),
            ::std::mem::transmute(lpenumfunc),
            ::std::mem::transmute(lparam),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumResourceNamesExA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HINSTANCE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hmodule: Param0,
    lptype: Param1,
    lpenumfunc: ::std::option::Option<ENUMRESNAMEPROCA>,
    lparam: isize,
    dwflags: u32,
    langid: u16,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnumResourceNamesExA(
                hmodule: super::super::Foundation::HINSTANCE,
                lptype: super::super::Foundation::PSTR,
                lpenumfunc: ::windows::runtime::RawPtr,
                lparam: isize,
                dwflags: u32,
                langid: u16,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(EnumResourceNamesExA(
            hmodule.into_param().abi(),
            lptype.into_param().abi(),
            ::std::mem::transmute(lpenumfunc),
            ::std::mem::transmute(lparam),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(langid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumResourceNamesExW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HINSTANCE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hmodule: Param0,
    lptype: Param1,
    lpenumfunc: ::std::option::Option<ENUMRESNAMEPROCW>,
    lparam: isize,
    dwflags: u32,
    langid: u16,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnumResourceNamesExW(
                hmodule: super::super::Foundation::HINSTANCE,
                lptype: super::super::Foundation::PWSTR,
                lpenumfunc: ::windows::runtime::RawPtr,
                lparam: isize,
                dwflags: u32,
                langid: u16,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(EnumResourceNamesExW(
            hmodule.into_param().abi(),
            lptype.into_param().abi(),
            ::std::mem::transmute(lpenumfunc),
            ::std::mem::transmute(lparam),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(langid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumResourceNamesW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HINSTANCE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hmodule: Param0,
    lptype: Param1,
    lpenumfunc: ::std::option::Option<ENUMRESNAMEPROCW>,
    lparam: isize,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnumResourceNamesW(
                hmodule: super::super::Foundation::HINSTANCE,
                lptype: super::super::Foundation::PWSTR,
                lpenumfunc: ::windows::runtime::RawPtr,
                lparam: isize,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(EnumResourceNamesW(
            hmodule.into_param().abi(),
            lptype.into_param().abi(),
            ::std::mem::transmute(lpenumfunc),
            ::std::mem::transmute(lparam),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumResourceTypesA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HINSTANCE>,
>(
    hmodule: Param0,
    lpenumfunc: ::std::option::Option<ENUMRESTYPEPROCA>,
    lparam: isize,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnumResourceTypesA(
                hmodule: super::super::Foundation::HINSTANCE,
                lpenumfunc: ::windows::runtime::RawPtr,
                lparam: isize,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(EnumResourceTypesA(
            hmodule.into_param().abi(),
            ::std::mem::transmute(lpenumfunc),
            ::std::mem::transmute(lparam),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumResourceTypesExA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HINSTANCE>,
>(
    hmodule: Param0,
    lpenumfunc: ::std::option::Option<ENUMRESTYPEPROCA>,
    lparam: isize,
    dwflags: u32,
    langid: u16,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnumResourceTypesExA(
                hmodule: super::super::Foundation::HINSTANCE,
                lpenumfunc: ::windows::runtime::RawPtr,
                lparam: isize,
                dwflags: u32,
                langid: u16,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(EnumResourceTypesExA(
            hmodule.into_param().abi(),
            ::std::mem::transmute(lpenumfunc),
            ::std::mem::transmute(lparam),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(langid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumResourceTypesExW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HINSTANCE>,
>(
    hmodule: Param0,
    lpenumfunc: ::std::option::Option<ENUMRESTYPEPROCW>,
    lparam: isize,
    dwflags: u32,
    langid: u16,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnumResourceTypesExW(
                hmodule: super::super::Foundation::HINSTANCE,
                lpenumfunc: ::windows::runtime::RawPtr,
                lparam: isize,
                dwflags: u32,
                langid: u16,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(EnumResourceTypesExW(
            hmodule.into_param().abi(),
            ::std::mem::transmute(lpenumfunc),
            ::std::mem::transmute(lparam),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(langid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumResourceTypesW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HINSTANCE>,
>(
    hmodule: Param0,
    lpenumfunc: ::std::option::Option<ENUMRESTYPEPROCW>,
    lparam: isize,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnumResourceTypesW(
                hmodule: super::super::Foundation::HINSTANCE,
                lpenumfunc: ::windows::runtime::RawPtr,
                lparam: isize,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(EnumResourceTypesW(
            hmodule.into_param().abi(),
            ::std::mem::transmute(lpenumfunc),
            ::std::mem::transmute(lparam),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const FIND_RESOURCE_DIRECTORY_LANGUAGES: u32 = 1024u32;
pub const FIND_RESOURCE_DIRECTORY_NAMES: u32 = 512u32;
pub const FIND_RESOURCE_DIRECTORY_TYPES: u32 = 256u32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindResourceA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HINSTANCE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hmodule: Param0,
    lpname: Param1,
    lptype: Param2,
) -> HRSRC {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FindResourceA(
                hmodule: super::super::Foundation::HINSTANCE,
                lpname: super::super::Foundation::PSTR,
                lptype: super::super::Foundation::PSTR,
            ) -> HRSRC;
        }
        ::std::mem::transmute(FindResourceA(
            hmodule.into_param().abi(),
            lpname.into_param().abi(),
            lptype.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindResourceExA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HINSTANCE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hmodule: Param0,
    lptype: Param1,
    lpname: Param2,
    wlanguage: u16,
) -> HRSRC {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FindResourceExA(
                hmodule: super::super::Foundation::HINSTANCE,
                lptype: super::super::Foundation::PSTR,
                lpname: super::super::Foundation::PSTR,
                wlanguage: u16,
            ) -> HRSRC;
        }
        ::std::mem::transmute(FindResourceExA(
            hmodule.into_param().abi(),
            lptype.into_param().abi(),
            lpname.into_param().abi(),
            ::std::mem::transmute(wlanguage),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindResourceExW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HINSTANCE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hmodule: Param0,
    lptype: Param1,
    lpname: Param2,
    wlanguage: u16,
) -> HRSRC {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FindResourceExW(
                hmodule: super::super::Foundation::HINSTANCE,
                lptype: super::super::Foundation::PWSTR,
                lpname: super::super::Foundation::PWSTR,
                wlanguage: u16,
            ) -> HRSRC;
        }
        ::std::mem::transmute(FindResourceExW(
            hmodule.into_param().abi(),
            lptype.into_param().abi(),
            lpname.into_param().abi(),
            ::std::mem::transmute(wlanguage),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindResourceW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HINSTANCE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hmodule: Param0,
    lpname: Param1,
    lptype: Param2,
) -> HRSRC {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FindResourceW(
                hmodule: super::super::Foundation::HINSTANCE,
                lpname: super::super::Foundation::PWSTR,
                lptype: super::super::Foundation::PWSTR,
            ) -> HRSRC;
        }
        ::std::mem::transmute(FindResourceW(
            hmodule.into_param().abi(),
            lpname.into_param().abi(),
            lptype.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FreeLibrary<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HINSTANCE>,
>(
    hlibmodule: Param0,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FreeLibrary(
                hlibmodule: super::super::Foundation::HINSTANCE,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FreeLibrary(hlibmodule.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FreeLibraryAndExitThread<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HINSTANCE>,
>(
    hlibmodule: Param0,
    dwexitcode: u32,
) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FreeLibraryAndExitThread(
                hlibmodule: super::super::Foundation::HINSTANCE,
                dwexitcode: u32,
            );
        }
        ::std::mem::transmute(FreeLibraryAndExitThread(
            hlibmodule.into_param().abi(),
            ::std::mem::transmute(dwexitcode),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FreeResource(hresdata: isize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FreeResource(hresdata: isize) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FreeResource(::std::mem::transmute(hresdata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS: u32 = 4u32;
pub const GET_MODULE_HANDLE_EX_FLAG_PIN: u32 = 1u32;
pub const GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT: u32 = 2u32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetDllDirectoryA(
    nbufferlength: u32,
    lpbuffer: super::super::Foundation::PSTR,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetDllDirectoryA(
                nbufferlength: u32,
                lpbuffer: super::super::Foundation::PSTR,
            ) -> u32;
        }
        ::std::mem::transmute(GetDllDirectoryA(
            ::std::mem::transmute(nbufferlength),
            ::std::mem::transmute(lpbuffer),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetDllDirectoryW(
    nbufferlength: u32,
    lpbuffer: super::super::Foundation::PWSTR,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetDllDirectoryW(
                nbufferlength: u32,
                lpbuffer: super::super::Foundation::PWSTR,
            ) -> u32;
        }
        ::std::mem::transmute(GetDllDirectoryW(
            ::std::mem::transmute(nbufferlength),
            ::std::mem::transmute(lpbuffer),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetModuleFileNameA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HINSTANCE>,
>(
    hmodule: Param0,
    lpfilename: super::super::Foundation::PSTR,
    nsize: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetModuleFileNameA(
                hmodule: super::super::Foundation::HINSTANCE,
                lpfilename: super::super::Foundation::PSTR,
                nsize: u32,
            ) -> u32;
        }
        ::std::mem::transmute(GetModuleFileNameA(
            hmodule.into_param().abi(),
            ::std::mem::transmute(lpfilename),
            ::std::mem::transmute(nsize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetModuleFileNameW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HINSTANCE>,
>(
    hmodule: Param0,
    lpfilename: super::super::Foundation::PWSTR,
    nsize: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetModuleFileNameW(
                hmodule: super::super::Foundation::HINSTANCE,
                lpfilename: super::super::Foundation::PWSTR,
                nsize: u32,
            ) -> u32;
        }
        ::std::mem::transmute(GetModuleFileNameW(
            hmodule.into_param().abi(),
            ::std::mem::transmute(lpfilename),
            ::std::mem::transmute(nsize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetModuleHandleA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    lpmodulename: Param0,
) -> super::super::Foundation::HINSTANCE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetModuleHandleA(
                lpmodulename: super::super::Foundation::PSTR,
            ) -> super::super::Foundation::HINSTANCE;
        }
        ::std::mem::transmute(GetModuleHandleA(lpmodulename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetModuleHandleExA<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    dwflags: u32,
    lpmodulename: Param1,
    phmodule: *mut super::super::Foundation::HINSTANCE,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetModuleHandleExA(
                dwflags: u32,
                lpmodulename: super::super::Foundation::PSTR,
                phmodule: *mut super::super::Foundation::HINSTANCE,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetModuleHandleExA(
            ::std::mem::transmute(dwflags),
            lpmodulename.into_param().abi(),
            ::std::mem::transmute(phmodule),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetModuleHandleExW<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    dwflags: u32,
    lpmodulename: Param1,
    phmodule: *mut super::super::Foundation::HINSTANCE,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetModuleHandleExW(
                dwflags: u32,
                lpmodulename: super::super::Foundation::PWSTR,
                phmodule: *mut super::super::Foundation::HINSTANCE,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetModuleHandleExW(
            ::std::mem::transmute(dwflags),
            lpmodulename.into_param().abi(),
            ::std::mem::transmute(phmodule),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetModuleHandleW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpmodulename: Param0,
) -> super::super::Foundation::HINSTANCE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetModuleHandleW(
                lpmodulename: super::super::Foundation::PWSTR,
            ) -> super::super::Foundation::HINSTANCE;
        }
        ::std::mem::transmute(GetModuleHandleW(lpmodulename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetProcAddress<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HINSTANCE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hmodule: Param0,
    lpprocname: Param1,
) -> ::std::option::Option<super::super::Foundation::FARPROC> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetProcAddress(
                hmodule: super::super::Foundation::HINSTANCE,
                lpprocname: super::super::Foundation::PSTR,
            ) -> ::std::option::Option<super::super::Foundation::FARPROC>;
        }
        ::std::mem::transmute(GetProcAddress(
            hmodule.into_param().abi(),
            lpprocname.into_param().abi(),
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
pub struct HRSRC(pub isize);
impl ::std::default::Default for HRSRC {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for HRSRC {}
unsafe impl ::windows::runtime::Abi for HRSRC {
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
pub struct LOAD_LIBRARY_FLAGS(pub u32);
pub const DONT_RESOLVE_DLL_REFERENCES: LOAD_LIBRARY_FLAGS = LOAD_LIBRARY_FLAGS(1u32);
pub const LOAD_LIBRARY_AS_DATAFILE: LOAD_LIBRARY_FLAGS = LOAD_LIBRARY_FLAGS(2u32);
pub const LOAD_WITH_ALTERED_SEARCH_PATH: LOAD_LIBRARY_FLAGS = LOAD_LIBRARY_FLAGS(8u32);
pub const LOAD_IGNORE_CODE_AUTHZ_LEVEL: LOAD_LIBRARY_FLAGS = LOAD_LIBRARY_FLAGS(16u32);
pub const LOAD_LIBRARY_AS_IMAGE_RESOURCE: LOAD_LIBRARY_FLAGS = LOAD_LIBRARY_FLAGS(32u32);
pub const LOAD_LIBRARY_AS_DATAFILE_EXCLUSIVE: LOAD_LIBRARY_FLAGS = LOAD_LIBRARY_FLAGS(64u32);
pub const LOAD_LIBRARY_REQUIRE_SIGNED_TARGET: LOAD_LIBRARY_FLAGS = LOAD_LIBRARY_FLAGS(128u32);
pub const LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR: LOAD_LIBRARY_FLAGS = LOAD_LIBRARY_FLAGS(256u32);
pub const LOAD_LIBRARY_SEARCH_APPLICATION_DIR: LOAD_LIBRARY_FLAGS = LOAD_LIBRARY_FLAGS(512u32);
pub const LOAD_LIBRARY_SEARCH_USER_DIRS: LOAD_LIBRARY_FLAGS = LOAD_LIBRARY_FLAGS(1024u32);
pub const LOAD_LIBRARY_SEARCH_SYSTEM32: LOAD_LIBRARY_FLAGS = LOAD_LIBRARY_FLAGS(2048u32);
pub const LOAD_LIBRARY_SEARCH_DEFAULT_DIRS: LOAD_LIBRARY_FLAGS = LOAD_LIBRARY_FLAGS(4096u32);
pub const LOAD_LIBRARY_SAFE_CURRENT_DIRS: LOAD_LIBRARY_FLAGS = LOAD_LIBRARY_FLAGS(8192u32);
pub const LOAD_LIBRARY_SEARCH_SYSTEM32_NO_FORWARDER: LOAD_LIBRARY_FLAGS =
    LOAD_LIBRARY_FLAGS(16384u32);
impl ::std::convert::From<u32> for LOAD_LIBRARY_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for LOAD_LIBRARY_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for LOAD_LIBRARY_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for LOAD_LIBRARY_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for LOAD_LIBRARY_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for LOAD_LIBRARY_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for LOAD_LIBRARY_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const LOAD_LIBRARY_OS_INTEGRITY_CONTINUITY: u32 = 32768u32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LoadLibraryA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    lplibfilename: Param0,
) -> super::super::Foundation::HINSTANCE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LoadLibraryA(
                lplibfilename: super::super::Foundation::PSTR,
            ) -> super::super::Foundation::HINSTANCE;
        }
        ::std::mem::transmute(LoadLibraryA(lplibfilename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LoadLibraryExA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    lplibfilename: Param0,
    hfile: Param1,
    dwflags: LOAD_LIBRARY_FLAGS,
) -> super::super::Foundation::HINSTANCE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LoadLibraryExA(
                lplibfilename: super::super::Foundation::PSTR,
                hfile: super::super::Foundation::HANDLE,
                dwflags: LOAD_LIBRARY_FLAGS,
            ) -> super::super::Foundation::HINSTANCE;
        }
        ::std::mem::transmute(LoadLibraryExA(
            lplibfilename.into_param().abi(),
            hfile.into_param().abi(),
            ::std::mem::transmute(dwflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LoadLibraryExW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    lplibfilename: Param0,
    hfile: Param1,
    dwflags: LOAD_LIBRARY_FLAGS,
) -> super::super::Foundation::HINSTANCE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LoadLibraryExW(
                lplibfilename: super::super::Foundation::PWSTR,
                hfile: super::super::Foundation::HANDLE,
                dwflags: LOAD_LIBRARY_FLAGS,
            ) -> super::super::Foundation::HINSTANCE;
        }
        ::std::mem::transmute(LoadLibraryExW(
            lplibfilename.into_param().abi(),
            hfile.into_param().abi(),
            ::std::mem::transmute(dwflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LoadLibraryW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lplibfilename: Param0,
) -> super::super::Foundation::HINSTANCE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LoadLibraryW(
                lplibfilename: super::super::Foundation::PWSTR,
            ) -> super::super::Foundation::HINSTANCE;
        }
        ::std::mem::transmute(LoadLibraryW(lplibfilename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LoadModule<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    lpmodulename: Param0,
    lpparameterblock: *const ::std::ffi::c_void,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LoadModule(
                lpmodulename: super::super::Foundation::PSTR,
                lpparameterblock: *const ::std::ffi::c_void,
            ) -> u32;
        }
        ::std::mem::transmute(LoadModule(
            lpmodulename.into_param().abi(),
            ::std::mem::transmute(lpparameterblock),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LoadPackagedLibrary<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpwlibfilename: Param0,
    reserved: u32,
) -> super::super::Foundation::HINSTANCE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LoadPackagedLibrary(
                lpwlibfilename: super::super::Foundation::PWSTR,
                reserved: u32,
            ) -> super::super::Foundation::HINSTANCE;
        }
        ::std::mem::transmute(LoadPackagedLibrary(
            lpwlibfilename.into_param().abi(),
            ::std::mem::transmute(reserved),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LoadResource<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HINSTANCE>,
    Param1: ::windows::runtime::IntoParam<'a, HRSRC>,
>(
    hmodule: Param0,
    hresinfo: Param1,
) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LoadResource(hmodule: super::super::Foundation::HINSTANCE, hresinfo: HRSRC)
                -> isize;
        }
        ::std::mem::transmute(LoadResource(
            hmodule.into_param().abi(),
            hresinfo.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn LockResource(hresdata: isize) -> *mut ::std::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LockResource(hresdata: isize) -> *mut ::std::ffi::c_void;
        }
        ::std::mem::transmute(LockResource(::std::mem::transmute(hresdata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub type PGET_MODULE_HANDLE_EXA = unsafe extern "system" fn(
    dwflags: u32,
    lpmodulename: super::super::Foundation::PSTR,
    phmodule: *mut super::super::Foundation::HINSTANCE,
) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PGET_MODULE_HANDLE_EXW = unsafe extern "system" fn(
    dwflags: u32,
    lpmodulename: super::super::Foundation::PWSTR,
    phmodule: *mut super::super::Foundation::HINSTANCE,
) -> super::super::Foundation::BOOL;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct REDIRECTION_DESCRIPTOR {
    pub Version: u32,
    pub FunctionCount: u32,
    pub Redirections: *mut REDIRECTION_FUNCTION_DESCRIPTOR,
}
#[cfg(feature = "Win32_Foundation")]
impl REDIRECTION_DESCRIPTOR {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for REDIRECTION_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for REDIRECTION_DESCRIPTOR {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("REDIRECTION_DESCRIPTOR")
            .field("Version", &self.Version)
            .field("FunctionCount", &self.FunctionCount)
            .field("Redirections", &self.Redirections)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for REDIRECTION_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version
            && self.FunctionCount == other.FunctionCount
            && self.Redirections == other.Redirections
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for REDIRECTION_DESCRIPTOR {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for REDIRECTION_DESCRIPTOR {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct REDIRECTION_FUNCTION_DESCRIPTOR {
    pub DllName: super::super::Foundation::PSTR,
    pub FunctionName: super::super::Foundation::PSTR,
    pub RedirectionTarget: *mut ::std::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl REDIRECTION_FUNCTION_DESCRIPTOR {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for REDIRECTION_FUNCTION_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for REDIRECTION_FUNCTION_DESCRIPTOR {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("REDIRECTION_FUNCTION_DESCRIPTOR")
            .field("DllName", &self.DllName)
            .field("FunctionName", &self.FunctionName)
            .field("RedirectionTarget", &self.RedirectionTarget)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for REDIRECTION_FUNCTION_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.DllName == other.DllName
            && self.FunctionName == other.FunctionName
            && self.RedirectionTarget == other.RedirectionTarget
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for REDIRECTION_FUNCTION_DESCRIPTOR {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for REDIRECTION_FUNCTION_DESCRIPTOR {
    type Abi = Self;
    type DefaultType = Self;
}
pub const RESOURCE_ENUM_LN: u32 = 1u32;
pub const RESOURCE_ENUM_MODULE_EXACT: u32 = 16u32;
pub const RESOURCE_ENUM_MUI: u32 = 2u32;
pub const RESOURCE_ENUM_MUI_SYSTEM: u32 = 4u32;
pub const RESOURCE_ENUM_VALIDATE: u32 = 8u32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RemoveDllDirectory(
    cookie: *const ::std::ffi::c_void,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RemoveDllDirectory(
                cookie: *const ::std::ffi::c_void,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(RemoveDllDirectory(::std::mem::transmute(cookie)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const SUPPORT_LANG_NUMBER: u32 = 32u32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetDefaultDllDirectories(
    directoryflags: LOAD_LIBRARY_FLAGS,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetDefaultDllDirectories(
                directoryflags: LOAD_LIBRARY_FLAGS,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetDefaultDllDirectories(::std::mem::transmute(
            directoryflags,
        )))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetDllDirectoryA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    lppathname: Param0,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetDllDirectoryA(
                lppathname: super::super::Foundation::PSTR,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetDllDirectoryA(lppathname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetDllDirectoryW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lppathname: Param0,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetDllDirectoryW(
                lppathname: super::super::Foundation::PWSTR,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetDllDirectoryW(lppathname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SizeofResource<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HINSTANCE>,
    Param1: ::windows::runtime::IntoParam<'a, HRSRC>,
>(
    hmodule: Param0,
    hresinfo: Param1,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SizeofResource(hmodule: super::super::Foundation::HINSTANCE, hresinfo: HRSRC)
                -> u32;
        }
        ::std::mem::transmute(SizeofResource(
            hmodule.into_param().abi(),
            hresinfo.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UpdateResourceA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hupdate: Param0,
    lptype: Param1,
    lpname: Param2,
    wlanguage: u16,
    lpdata: *const ::std::ffi::c_void,
    cb: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UpdateResourceA(
                hupdate: super::super::Foundation::HANDLE,
                lptype: super::super::Foundation::PSTR,
                lpname: super::super::Foundation::PSTR,
                wlanguage: u16,
                lpdata: *const ::std::ffi::c_void,
                cb: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(UpdateResourceA(
            hupdate.into_param().abi(),
            lptype.into_param().abi(),
            lpname.into_param().abi(),
            ::std::mem::transmute(wlanguage),
            ::std::mem::transmute(lpdata),
            ::std::mem::transmute(cb),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UpdateResourceW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hupdate: Param0,
    lptype: Param1,
    lpname: Param2,
    wlanguage: u16,
    lpdata: *const ::std::ffi::c_void,
    cb: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UpdateResourceW(
                hupdate: super::super::Foundation::HANDLE,
                lptype: super::super::Foundation::PWSTR,
                lpname: super::super::Foundation::PWSTR,
                wlanguage: u16,
                lpdata: *const ::std::ffi::c_void,
                cb: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(UpdateResourceW(
            hupdate.into_param().abi(),
            lptype.into_param().abi(),
            lpname.into_param().abi(),
            ::std::mem::transmute(wlanguage),
            ::std::mem::transmute(lpdata),
            ::std::mem::transmute(cb),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
