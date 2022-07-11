#[doc = "*Required features: `\"Win32_System_Mailslots\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateMailslotA<'a, Param0: ::std::convert::Into<::windows::core::PCSTR>>(lpname: Param0, nmaxmessagesize: u32, lreadtimeout: u32, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES) -> ::windows::core::Result<super::super::Foundation::HANDLE> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreateMailslotA(lpname: ::windows::core::PCSTR, nmaxmessagesize: u32, lreadtimeout: u32, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES) -> super::super::Foundation::HANDLE;
    }
    let result__ = CreateMailslotA(lpname.into(), ::core::mem::transmute(nmaxmessagesize), ::core::mem::transmute(lreadtimeout), ::core::mem::transmute(lpsecurityattributes));
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_Mailslots\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateMailslotW<'a, Param0: ::std::convert::Into<::windows::core::PCWSTR>>(lpname: Param0, nmaxmessagesize: u32, lreadtimeout: u32, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES) -> ::windows::core::Result<super::super::Foundation::HANDLE> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreateMailslotW(lpname: ::windows::core::PCWSTR, nmaxmessagesize: u32, lreadtimeout: u32, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES) -> super::super::Foundation::HANDLE;
    }
    let result__ = CreateMailslotW(lpname.into(), ::core::mem::transmute(nmaxmessagesize), ::core::mem::transmute(lreadtimeout), ::core::mem::transmute(lpsecurityattributes));
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_Mailslots\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetMailslotInfo<'a, Param0: ::std::convert::Into<super::super::Foundation::HANDLE>>(hmailslot: Param0, lpmaxmessagesize: *mut u32, lpnextsize: *mut u32, lpmessagecount: *mut u32, lpreadtimeout: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetMailslotInfo(hmailslot: super::super::Foundation::HANDLE, lpmaxmessagesize: *mut u32, lpnextsize: *mut u32, lpmessagecount: *mut u32, lpreadtimeout: *mut u32) -> super::super::Foundation::BOOL;
    }
    ::core::mem::transmute(GetMailslotInfo(hmailslot.into(), ::core::mem::transmute(lpmaxmessagesize), ::core::mem::transmute(lpnextsize), ::core::mem::transmute(lpmessagecount), ::core::mem::transmute(lpreadtimeout)))
}
#[doc = "*Required features: `\"Win32_System_Mailslots\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetMailslotInfo<'a, Param0: ::std::convert::Into<super::super::Foundation::HANDLE>>(hmailslot: Param0, lreadtimeout: u32) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetMailslotInfo(hmailslot: super::super::Foundation::HANDLE, lreadtimeout: u32) -> super::super::Foundation::BOOL;
    }
    ::core::mem::transmute(SetMailslotInfo(hmailslot.into(), ::core::mem::transmute(lreadtimeout)))
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
