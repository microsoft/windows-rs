#[doc = "*Required features: `\"Win32_System_Mailslots\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateMailslotA<P0>(lpname: P0, nmaxmessagesize: u32, lreadtimeout: u32, lpsecurityattributes: ::core::option::Option<*const super::super::Security::SECURITY_ATTRIBUTES>) -> ::windows::core::Result<super::super::Foundation::HANDLE>
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn CreateMailslotA ( lpname : ::windows::core::PCSTR , nmaxmessagesize : u32 , lreadtimeout : u32 , lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES ) -> super::super::Foundation:: HANDLE );
    let result__ = CreateMailslotA(lpname.into_param().abi(), nmaxmessagesize, lreadtimeout, ::core::mem::transmute(lpsecurityattributes.unwrap_or(::std::ptr::null())));
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_Mailslots\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateMailslotW<P0>(lpname: P0, nmaxmessagesize: u32, lreadtimeout: u32, lpsecurityattributes: ::core::option::Option<*const super::super::Security::SECURITY_ATTRIBUTES>) -> ::windows::core::Result<super::super::Foundation::HANDLE>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn CreateMailslotW ( lpname : ::windows::core::PCWSTR , nmaxmessagesize : u32 , lreadtimeout : u32 , lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES ) -> super::super::Foundation:: HANDLE );
    let result__ = CreateMailslotW(lpname.into_param().abi(), nmaxmessagesize, lreadtimeout, ::core::mem::transmute(lpsecurityattributes.unwrap_or(::std::ptr::null())));
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_Mailslots\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetMailslotInfo<P0>(hmailslot: P0, lpmaxmessagesize: ::core::option::Option<*mut u32>, lpnextsize: ::core::option::Option<*mut u32>, lpmessagecount: ::core::option::Option<*mut u32>, lpreadtimeout: ::core::option::Option<*mut u32>) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetMailslotInfo ( hmailslot : super::super::Foundation:: HANDLE , lpmaxmessagesize : *mut u32 , lpnextsize : *mut u32 , lpmessagecount : *mut u32 , lpreadtimeout : *mut u32 ) -> super::super::Foundation:: BOOL );
    GetMailslotInfo(hmailslot.into_param().abi(), ::core::mem::transmute(lpmaxmessagesize.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lpnextsize.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lpmessagecount.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lpreadtimeout.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_Mailslots\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetMailslotInfo<P0>(hmailslot: P0, lreadtimeout: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn SetMailslotInfo ( hmailslot : super::super::Foundation:: HANDLE , lreadtimeout : u32 ) -> super::super::Foundation:: BOOL );
    SetMailslotInfo(hmailslot.into_param().abi(), lreadtimeout)
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
