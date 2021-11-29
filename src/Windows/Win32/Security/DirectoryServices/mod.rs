#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authorization_UI"))]
#[inline]
pub unsafe fn DSCreateISecurityInfoObject<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param6: ::windows::core::IntoParam<'a, super::super::Foundation::LPARAM>>(
    pwszobjectpath: Param0,
    pwszobjectclass: Param1,
    dwflags: u32,
    ppsi: *mut ::core::option::Option<super::Authorization::UI::ISecurityInformation>,
    pfnreadsd: PFNREADOBJECTSECURITY,
    pfnwritesd: PFNWRITEOBJECTSECURITY,
    lpcontext: Param6,
) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DSCreateISecurityInfoObject(pwszobjectpath: super::super::Foundation::PWSTR, pwszobjectclass: super::super::Foundation::PWSTR, dwflags: u32, ppsi: *mut ::windows::core::RawPtr, pfnreadsd: ::windows::core::RawPtr, pfnwritesd: ::windows::core::RawPtr, lpcontext: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT;
        }
        DSCreateISecurityInfoObject(pwszobjectpath.into_param().abi(), pwszobjectclass.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(ppsi), ::core::mem::transmute(pfnreadsd), ::core::mem::transmute(pfnwritesd), lpcontext.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authorization_UI"))]
#[inline]
pub unsafe fn DSCreateISecurityInfoObjectEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param9: ::windows::core::IntoParam<'a, super::super::Foundation::LPARAM>>(
    pwszobjectpath: Param0,
    pwszobjectclass: Param1,
    pwszserver: Param2,
    pwszusername: Param3,
    pwszpassword: Param4,
    dwflags: u32,
    ppsi: *mut ::core::option::Option<super::Authorization::UI::ISecurityInformation>,
    pfnreadsd: PFNREADOBJECTSECURITY,
    pfnwritesd: PFNWRITEOBJECTSECURITY,
    lpcontext: Param9,
) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DSCreateISecurityInfoObjectEx(pwszobjectpath: super::super::Foundation::PWSTR, pwszobjectclass: super::super::Foundation::PWSTR, pwszserver: super::super::Foundation::PWSTR, pwszusername: super::super::Foundation::PWSTR, pwszpassword: super::super::Foundation::PWSTR, dwflags: u32, ppsi: *mut ::windows::core::RawPtr, pfnreadsd: ::windows::core::RawPtr, pfnwritesd: ::windows::core::RawPtr, lpcontext: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT;
        }
        DSCreateISecurityInfoObjectEx(
            pwszobjectpath.into_param().abi(),
            pwszobjectclass.into_param().abi(),
            pwszserver.into_param().abi(),
            pwszusername.into_param().abi(),
            pwszpassword.into_param().abi(),
            ::core::mem::transmute(dwflags),
            ::core::mem::transmute(ppsi),
            ::core::mem::transmute(pfnreadsd),
            ::core::mem::transmute(pfnwritesd),
            lpcontext.into_param().abi(),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
#[inline]
pub unsafe fn DSCreateSecurityPage<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param6: ::windows::core::IntoParam<'a, super::super::Foundation::LPARAM>>(pwszobjectpath: Param0, pwszobjectclass: Param1, dwflags: u32, phpage: *mut super::super::UI::Controls::HPROPSHEETPAGE, pfnreadsd: PFNREADOBJECTSECURITY, pfnwritesd: PFNWRITEOBJECTSECURITY, lpcontext: Param6) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DSCreateSecurityPage(pwszobjectpath: super::super::Foundation::PWSTR, pwszobjectclass: super::super::Foundation::PWSTR, dwflags: u32, phpage: *mut super::super::UI::Controls::HPROPSHEETPAGE, pfnreadsd: ::windows::core::RawPtr, pfnwritesd: ::windows::core::RawPtr, lpcontext: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT;
        }
        DSCreateSecurityPage(pwszobjectpath.into_param().abi(), pwszobjectclass.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(phpage), ::core::mem::transmute(pfnreadsd), ::core::mem::transmute(pfnwritesd), lpcontext.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DSEditSecurity<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param7: ::windows::core::IntoParam<'a, super::super::Foundation::LPARAM>>(
    hwndowner: Param0,
    pwszobjectpath: Param1,
    pwszobjectclass: Param2,
    dwflags: u32,
    pwszcaption: Param4,
    pfnreadsd: PFNREADOBJECTSECURITY,
    pfnwritesd: PFNWRITEOBJECTSECURITY,
    lpcontext: Param7,
) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DSEditSecurity(hwndowner: super::super::Foundation::HWND, pwszobjectpath: super::super::Foundation::PWSTR, pwszobjectclass: super::super::Foundation::PWSTR, dwflags: u32, pwszcaption: super::super::Foundation::PWSTR, pfnreadsd: ::windows::core::RawPtr, pfnwritesd: ::windows::core::RawPtr, lpcontext: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT;
        }
        DSEditSecurity(hwndowner.into_param().abi(), pwszobjectpath.into_param().abi(), pwszobjectclass.into_param().abi(), ::core::mem::transmute(dwflags), pwszcaption.into_param().abi(), ::core::mem::transmute(pfnreadsd), ::core::mem::transmute(pfnwritesd), lpcontext.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const DSSI_IS_ROOT: u32 = 16u32;
pub const DSSI_NO_ACCESS_CHECK: u32 = 2u32;
pub const DSSI_NO_EDIT_OWNER: u32 = 8u32;
pub const DSSI_NO_EDIT_SACL: u32 = 4u32;
pub const DSSI_NO_FILTER: u32 = 32u32;
pub const DSSI_NO_READONLY_MESSAGE: u32 = 64u32;
pub const DSSI_READ_ONLY: u32 = 1u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authorization_UI"))]
pub type PFNDSCREATEISECINFO = ::core::option::Option<unsafe extern "system" fn(param0: super::super::Foundation::PWSTR, param1: super::super::Foundation::PWSTR, param2: u32, param3: *mut ::windows::core::RawPtr, param4: ::windows::core::RawPtr, param5: ::windows::core::RawPtr, param6: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT>;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authorization_UI"))]
pub type PFNDSCREATEISECINFOEX = ::core::option::Option<unsafe extern "system" fn(param0: super::super::Foundation::PWSTR, param1: super::super::Foundation::PWSTR, param2: super::super::Foundation::PWSTR, param3: super::super::Foundation::PWSTR, param4: super::super::Foundation::PWSTR, param5: u32, param6: *mut ::windows::core::RawPtr, param7: ::windows::core::RawPtr, param8: ::windows::core::RawPtr, param9: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT>;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
pub type PFNDSCREATESECPAGE = ::core::option::Option<unsafe extern "system" fn(param0: super::super::Foundation::PWSTR, param1: super::super::Foundation::PWSTR, param2: u32, param3: *mut super::super::UI::Controls::HPROPSHEETPAGE, param4: ::windows::core::RawPtr, param5: ::windows::core::RawPtr, param6: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT>;
#[cfg(feature = "Win32_Foundation")]
pub type PFNDSEDITSECURITY = ::core::option::Option<unsafe extern "system" fn(param0: super::super::Foundation::HWND, param1: super::super::Foundation::PWSTR, param2: super::super::Foundation::PWSTR, param3: u32, param4: super::super::Foundation::PWSTR, param5: ::windows::core::RawPtr, param6: ::windows::core::RawPtr, param7: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT>;
#[cfg(feature = "Win32_Foundation")]
pub type PFNREADOBJECTSECURITY = ::core::option::Option<unsafe extern "system" fn(param0: super::super::Foundation::PWSTR, param1: u32, param2: *mut *mut super::SECURITY_DESCRIPTOR, param3: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT>;
#[cfg(feature = "Win32_Foundation")]
pub type PFNWRITEOBJECTSECURITY = ::core::option::Option<unsafe extern "system" fn(param0: super::super::Foundation::PWSTR, param1: u32, param2: *mut super::SECURITY_DESCRIPTOR, param3: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT>;
