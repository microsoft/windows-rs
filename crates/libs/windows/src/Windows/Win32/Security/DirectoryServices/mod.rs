#[doc = "*Required features: `\"Win32_Security_DirectoryServices\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Authorization_UI\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authorization_UI"))]
#[inline]
pub unsafe fn DSCreateISecurityInfoObject<P0, P1, P2>(pwszobjectpath: P0, pwszobjectclass: P1, dwflags: u32, ppsi: *mut ::core::option::Option<super::Authorization::UI::ISecurityInformation>, pfnreadsd: PFNREADOBJECTSECURITY, pfnwritesd: PFNWRITEOBJECTSECURITY, lpcontext: P2) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<super::super::Foundation::LPARAM>,
{
    ::windows_targets::link!("dssec.dll" "system" fn DSCreateISecurityInfoObject(pwszobjectpath : ::windows_core::PCWSTR, pwszobjectclass : ::windows_core::PCWSTR, dwflags : u32, ppsi : *mut * mut::core::ffi::c_void, pfnreadsd : PFNREADOBJECTSECURITY, pfnwritesd : PFNWRITEOBJECTSECURITY, lpcontext : super::super::Foundation:: LPARAM) -> ::windows_core::HRESULT);
    DSCreateISecurityInfoObject(pwszobjectpath.into_param().abi(), pwszobjectclass.into_param().abi(), dwflags, ::core::mem::transmute(ppsi), pfnreadsd, pfnwritesd, lpcontext.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_Security_DirectoryServices\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Authorization_UI\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authorization_UI"))]
#[inline]
pub unsafe fn DSCreateISecurityInfoObjectEx<P0, P1, P2, P3, P4, P5>(pwszobjectpath: P0, pwszobjectclass: P1, pwszserver: P2, pwszusername: P3, pwszpassword: P4, dwflags: u32, ppsi: *mut ::core::option::Option<super::Authorization::UI::ISecurityInformation>, pfnreadsd: PFNREADOBJECTSECURITY, pfnwritesd: PFNWRITEOBJECTSECURITY, lpcontext: P5) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P3: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P4: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P5: ::windows_core::IntoParam<super::super::Foundation::LPARAM>,
{
    ::windows_targets::link!("dssec.dll" "system" fn DSCreateISecurityInfoObjectEx(pwszobjectpath : ::windows_core::PCWSTR, pwszobjectclass : ::windows_core::PCWSTR, pwszserver : ::windows_core::PCWSTR, pwszusername : ::windows_core::PCWSTR, pwszpassword : ::windows_core::PCWSTR, dwflags : u32, ppsi : *mut * mut::core::ffi::c_void, pfnreadsd : PFNREADOBJECTSECURITY, pfnwritesd : PFNWRITEOBJECTSECURITY, lpcontext : super::super::Foundation:: LPARAM) -> ::windows_core::HRESULT);
    DSCreateISecurityInfoObjectEx(pwszobjectpath.into_param().abi(), pwszobjectclass.into_param().abi(), pwszserver.into_param().abi(), pwszusername.into_param().abi(), pwszpassword.into_param().abi(), dwflags, ::core::mem::transmute(ppsi), pfnreadsd, pfnwritesd, lpcontext.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_Security_DirectoryServices\"`, `\"Win32_Foundation\"`, `\"Win32_UI_Controls\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
#[inline]
pub unsafe fn DSCreateSecurityPage<P0, P1, P2>(pwszobjectpath: P0, pwszobjectclass: P1, dwflags: u32, phpage: *mut super::super::UI::Controls::HPROPSHEETPAGE, pfnreadsd: PFNREADOBJECTSECURITY, pfnwritesd: PFNWRITEOBJECTSECURITY, lpcontext: P2) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<super::super::Foundation::LPARAM>,
{
    ::windows_targets::link!("dssec.dll" "system" fn DSCreateSecurityPage(pwszobjectpath : ::windows_core::PCWSTR, pwszobjectclass : ::windows_core::PCWSTR, dwflags : u32, phpage : *mut super::super::UI::Controls:: HPROPSHEETPAGE, pfnreadsd : PFNREADOBJECTSECURITY, pfnwritesd : PFNWRITEOBJECTSECURITY, lpcontext : super::super::Foundation:: LPARAM) -> ::windows_core::HRESULT);
    DSCreateSecurityPage(pwszobjectpath.into_param().abi(), pwszobjectclass.into_param().abi(), dwflags, phpage, pfnreadsd, pfnwritesd, lpcontext.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_Security_DirectoryServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DSEditSecurity<P0, P1, P2, P3, P4>(hwndowner: P0, pwszobjectpath: P1, pwszobjectclass: P2, dwflags: u32, pwszcaption: P3, pfnreadsd: PFNREADOBJECTSECURITY, pfnwritesd: PFNWRITEOBJECTSECURITY, lpcontext: P4) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P3: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P4: ::windows_core::IntoParam<super::super::Foundation::LPARAM>,
{
    ::windows_targets::link!("dssec.dll" "system" fn DSEditSecurity(hwndowner : super::super::Foundation:: HWND, pwszobjectpath : ::windows_core::PCWSTR, pwszobjectclass : ::windows_core::PCWSTR, dwflags : u32, pwszcaption : ::windows_core::PCWSTR, pfnreadsd : PFNREADOBJECTSECURITY, pfnwritesd : PFNWRITEOBJECTSECURITY, lpcontext : super::super::Foundation:: LPARAM) -> ::windows_core::HRESULT);
    DSEditSecurity(hwndowner.into_param().abi(), pwszobjectpath.into_param().abi(), pwszobjectclass.into_param().abi(), dwflags, pwszcaption.into_param().abi(), pfnreadsd, pfnwritesd, lpcontext.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_Security_DirectoryServices\"`*"]
pub const DSSI_IS_ROOT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_DirectoryServices\"`*"]
pub const DSSI_NO_ACCESS_CHECK: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_DirectoryServices\"`*"]
pub const DSSI_NO_EDIT_OWNER: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_DirectoryServices\"`*"]
pub const DSSI_NO_EDIT_SACL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_DirectoryServices\"`*"]
pub const DSSI_NO_FILTER: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Security_DirectoryServices\"`*"]
pub const DSSI_NO_READONLY_MESSAGE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Security_DirectoryServices\"`*"]
pub const DSSI_READ_ONLY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_DirectoryServices\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Authorization_UI\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authorization_UI"))]
pub type PFNDSCREATEISECINFO = ::core::option::Option<unsafe extern "system" fn(param0: ::windows_core::PCWSTR, param1: ::windows_core::PCWSTR, param2: u32, param3: *mut ::core::option::Option<super::Authorization::UI::ISecurityInformation>, param4: PFNREADOBJECTSECURITY, param5: PFNWRITEOBJECTSECURITY, param6: super::super::Foundation::LPARAM) -> ::windows_core::HRESULT>;
#[doc = "*Required features: `\"Win32_Security_DirectoryServices\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Authorization_UI\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authorization_UI"))]
pub type PFNDSCREATEISECINFOEX = ::core::option::Option<unsafe extern "system" fn(param0: ::windows_core::PCWSTR, param1: ::windows_core::PCWSTR, param2: ::windows_core::PCWSTR, param3: ::windows_core::PCWSTR, param4: ::windows_core::PCWSTR, param5: u32, param6: *mut ::core::option::Option<super::Authorization::UI::ISecurityInformation>, param7: PFNREADOBJECTSECURITY, param8: PFNWRITEOBJECTSECURITY, param9: super::super::Foundation::LPARAM) -> ::windows_core::HRESULT>;
#[doc = "*Required features: `\"Win32_Security_DirectoryServices\"`, `\"Win32_Foundation\"`, `\"Win32_UI_Controls\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
pub type PFNDSCREATESECPAGE = ::core::option::Option<unsafe extern "system" fn(param0: ::windows_core::PCWSTR, param1: ::windows_core::PCWSTR, param2: u32, param3: *mut super::super::UI::Controls::HPROPSHEETPAGE, param4: PFNREADOBJECTSECURITY, param5: PFNWRITEOBJECTSECURITY, param6: super::super::Foundation::LPARAM) -> ::windows_core::HRESULT>;
#[doc = "*Required features: `\"Win32_Security_DirectoryServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFNDSEDITSECURITY = ::core::option::Option<unsafe extern "system" fn(param0: super::super::Foundation::HWND, param1: ::windows_core::PCWSTR, param2: ::windows_core::PCWSTR, param3: u32, param4: ::windows_core::PCWSTR, param5: PFNREADOBJECTSECURITY, param6: PFNWRITEOBJECTSECURITY, param7: super::super::Foundation::LPARAM) -> ::windows_core::HRESULT>;
#[doc = "*Required features: `\"Win32_Security_DirectoryServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFNREADOBJECTSECURITY = ::core::option::Option<unsafe extern "system" fn(param0: ::windows_core::PCWSTR, param1: u32, param2: *mut super::PSECURITY_DESCRIPTOR, param3: super::super::Foundation::LPARAM) -> ::windows_core::HRESULT>;
#[doc = "*Required features: `\"Win32_Security_DirectoryServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFNWRITEOBJECTSECURITY = ::core::option::Option<unsafe extern "system" fn(param0: ::windows_core::PCWSTR, param1: u32, param2: super::PSECURITY_DESCRIPTOR, param3: super::super::Foundation::LPARAM) -> ::windows_core::HRESULT>;
