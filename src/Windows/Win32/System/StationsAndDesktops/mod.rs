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
pub unsafe fn CloseDesktop<'a, Param0: ::windows::runtime::IntoParam<'a, HDESK>>(
    hdesktop: Param0,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CloseDesktop(hdesktop: HDESK) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CloseDesktop(hdesktop.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CloseWindowStation<'a, Param0: ::windows::runtime::IntoParam<'a, HWINSTA>>(
    hwinsta: Param0,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CloseWindowStation(hwinsta: HWINSTA) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CloseWindowStation(hwinsta.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security",
    feature = "Win32_UI_DisplayDevices"
))]
#[inline]
pub unsafe fn CreateDesktopA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    lpszdesktop: Param0,
    lpszdevice: Param1,
    pdevmode: *mut super::super::UI::DisplayDevices::DEVMODEA,
    dwflags: u32,
    dwdesiredaccess: u32,
    lpsa: *const super::super::Security::SECURITY_ATTRIBUTES,
) -> HDESK {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateDesktopA(
                lpszdesktop: super::super::Foundation::PSTR,
                lpszdevice: super::super::Foundation::PSTR,
                pdevmode: *mut super::super::UI::DisplayDevices::DEVMODEA,
                dwflags: u32,
                dwdesiredaccess: u32,
                lpsa: *const super::super::Security::SECURITY_ATTRIBUTES,
            ) -> HDESK;
        }
        ::std::mem::transmute(CreateDesktopA(
            lpszdesktop.into_param().abi(),
            lpszdevice.into_param().abi(),
            ::std::mem::transmute(pdevmode),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(dwdesiredaccess),
            ::std::mem::transmute(lpsa),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security",
    feature = "Win32_UI_DisplayDevices"
))]
#[inline]
pub unsafe fn CreateDesktopExA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    lpszdesktop: Param0,
    lpszdevice: Param1,
    pdevmode: *mut super::super::UI::DisplayDevices::DEVMODEA,
    dwflags: u32,
    dwdesiredaccess: u32,
    lpsa: *const super::super::Security::SECURITY_ATTRIBUTES,
    ulheapsize: u32,
    pvoid: *mut ::std::ffi::c_void,
) -> HDESK {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateDesktopExA(
                lpszdesktop: super::super::Foundation::PSTR,
                lpszdevice: super::super::Foundation::PSTR,
                pdevmode: *mut super::super::UI::DisplayDevices::DEVMODEA,
                dwflags: u32,
                dwdesiredaccess: u32,
                lpsa: *const super::super::Security::SECURITY_ATTRIBUTES,
                ulheapsize: u32,
                pvoid: *mut ::std::ffi::c_void,
            ) -> HDESK;
        }
        ::std::mem::transmute(CreateDesktopExA(
            lpszdesktop.into_param().abi(),
            lpszdevice.into_param().abi(),
            ::std::mem::transmute(pdevmode),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(dwdesiredaccess),
            ::std::mem::transmute(lpsa),
            ::std::mem::transmute(ulheapsize),
            ::std::mem::transmute(pvoid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security",
    feature = "Win32_UI_DisplayDevices"
))]
#[inline]
pub unsafe fn CreateDesktopExW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpszdesktop: Param0,
    lpszdevice: Param1,
    pdevmode: *mut super::super::UI::DisplayDevices::DEVMODEW,
    dwflags: u32,
    dwdesiredaccess: u32,
    lpsa: *const super::super::Security::SECURITY_ATTRIBUTES,
    ulheapsize: u32,
    pvoid: *mut ::std::ffi::c_void,
) -> HDESK {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateDesktopExW(
                lpszdesktop: super::super::Foundation::PWSTR,
                lpszdevice: super::super::Foundation::PWSTR,
                pdevmode: *mut super::super::UI::DisplayDevices::DEVMODEW,
                dwflags: u32,
                dwdesiredaccess: u32,
                lpsa: *const super::super::Security::SECURITY_ATTRIBUTES,
                ulheapsize: u32,
                pvoid: *mut ::std::ffi::c_void,
            ) -> HDESK;
        }
        ::std::mem::transmute(CreateDesktopExW(
            lpszdesktop.into_param().abi(),
            lpszdevice.into_param().abi(),
            ::std::mem::transmute(pdevmode),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(dwdesiredaccess),
            ::std::mem::transmute(lpsa),
            ::std::mem::transmute(ulheapsize),
            ::std::mem::transmute(pvoid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security",
    feature = "Win32_UI_DisplayDevices"
))]
#[inline]
pub unsafe fn CreateDesktopW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpszdesktop: Param0,
    lpszdevice: Param1,
    pdevmode: *mut super::super::UI::DisplayDevices::DEVMODEW,
    dwflags: u32,
    dwdesiredaccess: u32,
    lpsa: *const super::super::Security::SECURITY_ATTRIBUTES,
) -> HDESK {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateDesktopW(
                lpszdesktop: super::super::Foundation::PWSTR,
                lpszdevice: super::super::Foundation::PWSTR,
                pdevmode: *mut super::super::UI::DisplayDevices::DEVMODEW,
                dwflags: u32,
                dwdesiredaccess: u32,
                lpsa: *const super::super::Security::SECURITY_ATTRIBUTES,
            ) -> HDESK;
        }
        ::std::mem::transmute(CreateDesktopW(
            lpszdesktop.into_param().abi(),
            lpszdevice.into_param().abi(),
            ::std::mem::transmute(pdevmode),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(dwdesiredaccess),
            ::std::mem::transmute(lpsa),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateWindowStationA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    lpwinsta: Param0,
    dwflags: u32,
    dwdesiredaccess: u32,
    lpsa: *const super::super::Security::SECURITY_ATTRIBUTES,
) -> HWINSTA {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateWindowStationA(
                lpwinsta: super::super::Foundation::PSTR,
                dwflags: u32,
                dwdesiredaccess: u32,
                lpsa: *const super::super::Security::SECURITY_ATTRIBUTES,
            ) -> HWINSTA;
        }
        ::std::mem::transmute(CreateWindowStationA(
            lpwinsta.into_param().abi(),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(dwdesiredaccess),
            ::std::mem::transmute(lpsa),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateWindowStationW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpwinsta: Param0,
    dwflags: u32,
    dwdesiredaccess: u32,
    lpsa: *const super::super::Security::SECURITY_ATTRIBUTES,
) -> HWINSTA {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateWindowStationW(
                lpwinsta: super::super::Foundation::PWSTR,
                dwflags: u32,
                dwdesiredaccess: u32,
                lpsa: *const super::super::Security::SECURITY_ATTRIBUTES,
            ) -> HWINSTA;
        }
        ::std::mem::transmute(CreateWindowStationW(
            lpwinsta.into_param().abi(),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(dwdesiredaccess),
            ::std::mem::transmute(lpsa),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn EnumDesktopWindows<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HDESK>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::LPARAM>,
>(
    hdesktop: Param0,
    lpfn: ::std::option::Option<super::super::UI::WindowsAndMessaging::WNDENUMPROC>,
    lparam: Param2,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnumDesktopWindows(
                hdesktop: HDESK,
                lpfn: ::windows::runtime::RawPtr,
                lparam: super::super::Foundation::LPARAM,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(EnumDesktopWindows(
            hdesktop.into_param().abi(),
            ::std::mem::transmute(lpfn),
            lparam.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn EnumDesktopsA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HWINSTA>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::LPARAM>,
>(
    hwinsta: Param0,
    lpenumfunc: ::std::option::Option<super::super::UI::WindowsAndMessaging::DESKTOPENUMPROCA>,
    lparam: Param2,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnumDesktopsA(
                hwinsta: HWINSTA,
                lpenumfunc: ::windows::runtime::RawPtr,
                lparam: super::super::Foundation::LPARAM,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(EnumDesktopsA(
            hwinsta.into_param().abi(),
            ::std::mem::transmute(lpenumfunc),
            lparam.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn EnumDesktopsW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HWINSTA>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::LPARAM>,
>(
    hwinsta: Param0,
    lpenumfunc: ::std::option::Option<super::super::UI::WindowsAndMessaging::DESKTOPENUMPROCW>,
    lparam: Param2,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnumDesktopsW(
                hwinsta: HWINSTA,
                lpenumfunc: ::windows::runtime::RawPtr,
                lparam: super::super::Foundation::LPARAM,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(EnumDesktopsW(
            hwinsta.into_param().abi(),
            ::std::mem::transmute(lpenumfunc),
            lparam.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn EnumWindowStationsA<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::LPARAM>,
>(
    lpenumfunc: ::std::option::Option<super::super::UI::WindowsAndMessaging::WINSTAENUMPROCA>,
    lparam: Param1,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnumWindowStationsA(
                lpenumfunc: ::windows::runtime::RawPtr,
                lparam: super::super::Foundation::LPARAM,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(EnumWindowStationsA(
            ::std::mem::transmute(lpenumfunc),
            lparam.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn EnumWindowStationsW<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::LPARAM>,
>(
    lpenumfunc: ::std::option::Option<super::super::UI::WindowsAndMessaging::WINSTAENUMPROCW>,
    lparam: Param1,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnumWindowStationsW(
                lpenumfunc: ::windows::runtime::RawPtr,
                lparam: super::super::Foundation::LPARAM,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(EnumWindowStationsW(
            ::std::mem::transmute(lpenumfunc),
            lparam.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetProcessWindowStation() -> HWINSTA {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetProcessWindowStation() -> HWINSTA;
        }
        ::std::mem::transmute(GetProcessWindowStation())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetThreadDesktop(dwthreadid: u32) -> HDESK {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThreadDesktop(dwthreadid: u32) -> HDESK;
        }
        ::std::mem::transmute(GetThreadDesktop(::std::mem::transmute(dwthreadid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetUserObjectInformationA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hobj: Param0,
    nindex: USER_OBJECT_INFORMATION_INDEX,
    pvinfo: *mut ::std::ffi::c_void,
    nlength: u32,
    lpnlengthneeded: *mut u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetUserObjectInformationA(
                hobj: super::super::Foundation::HANDLE,
                nindex: USER_OBJECT_INFORMATION_INDEX,
                pvinfo: *mut ::std::ffi::c_void,
                nlength: u32,
                lpnlengthneeded: *mut u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetUserObjectInformationA(
            hobj.into_param().abi(),
            ::std::mem::transmute(nindex),
            ::std::mem::transmute(pvinfo),
            ::std::mem::transmute(nlength),
            ::std::mem::transmute(lpnlengthneeded),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetUserObjectInformationW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hobj: Param0,
    nindex: USER_OBJECT_INFORMATION_INDEX,
    pvinfo: *mut ::std::ffi::c_void,
    nlength: u32,
    lpnlengthneeded: *mut u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetUserObjectInformationW(
                hobj: super::super::Foundation::HANDLE,
                nindex: USER_OBJECT_INFORMATION_INDEX,
                pvinfo: *mut ::std::ffi::c_void,
                nlength: u32,
                lpnlengthneeded: *mut u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetUserObjectInformationW(
            hobj.into_param().abi(),
            ::std::mem::transmute(nindex),
            ::std::mem::transmute(pvinfo),
            ::std::mem::transmute(nlength),
            ::std::mem::transmute(lpnlengthneeded),
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
pub struct HDESK(pub isize);
impl ::std::default::Default for HDESK {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for HDESK {}
unsafe impl ::windows::runtime::Abi for HDESK {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: clone :: Clone,
    :: std :: marker :: Copy,
    :: std :: fmt :: Debug,
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
)]
#[repr(transparent)]
pub struct HWINSTA(pub isize);
impl ::std::default::Default for HWINSTA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for HWINSTA {}
unsafe impl ::windows::runtime::Abi for HWINSTA {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenDesktopA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    lpszdesktop: Param0,
    dwflags: u32,
    finherit: Param2,
    dwdesiredaccess: u32,
) -> HDESK {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenDesktopA(
                lpszdesktop: super::super::Foundation::PSTR,
                dwflags: u32,
                finherit: super::super::Foundation::BOOL,
                dwdesiredaccess: u32,
            ) -> HDESK;
        }
        ::std::mem::transmute(OpenDesktopA(
            lpszdesktop.into_param().abi(),
            ::std::mem::transmute(dwflags),
            finherit.into_param().abi(),
            ::std::mem::transmute(dwdesiredaccess),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenDesktopW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    lpszdesktop: Param0,
    dwflags: u32,
    finherit: Param2,
    dwdesiredaccess: u32,
) -> HDESK {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenDesktopW(
                lpszdesktop: super::super::Foundation::PWSTR,
                dwflags: u32,
                finherit: super::super::Foundation::BOOL,
                dwdesiredaccess: u32,
            ) -> HDESK;
        }
        ::std::mem::transmute(OpenDesktopW(
            lpszdesktop.into_param().abi(),
            ::std::mem::transmute(dwflags),
            finherit.into_param().abi(),
            ::std::mem::transmute(dwdesiredaccess),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenInputDesktop<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    dwflags: u32,
    finherit: Param1,
    dwdesiredaccess: u32,
) -> HDESK {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenInputDesktop(
                dwflags: u32,
                finherit: super::super::Foundation::BOOL,
                dwdesiredaccess: u32,
            ) -> HDESK;
        }
        ::std::mem::transmute(OpenInputDesktop(
            ::std::mem::transmute(dwflags),
            finherit.into_param().abi(),
            ::std::mem::transmute(dwdesiredaccess),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenWindowStationA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    lpszwinsta: Param0,
    finherit: Param1,
    dwdesiredaccess: u32,
) -> HWINSTA {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenWindowStationA(
                lpszwinsta: super::super::Foundation::PSTR,
                finherit: super::super::Foundation::BOOL,
                dwdesiredaccess: u32,
            ) -> HWINSTA;
        }
        ::std::mem::transmute(OpenWindowStationA(
            lpszwinsta.into_param().abi(),
            finherit.into_param().abi(),
            ::std::mem::transmute(dwdesiredaccess),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenWindowStationW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    lpszwinsta: Param0,
    finherit: Param1,
    dwdesiredaccess: u32,
) -> HWINSTA {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenWindowStationW(
                lpszwinsta: super::super::Foundation::PWSTR,
                finherit: super::super::Foundation::BOOL,
                dwdesiredaccess: u32,
            ) -> HWINSTA;
        }
        ::std::mem::transmute(OpenWindowStationW(
            lpszwinsta.into_param().abi(),
            finherit.into_param().abi(),
            ::std::mem::transmute(dwdesiredaccess),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetProcessWindowStation<'a, Param0: ::windows::runtime::IntoParam<'a, HWINSTA>>(
    hwinsta: Param0,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetProcessWindowStation(hwinsta: HWINSTA) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetProcessWindowStation(hwinsta.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetThreadDesktop<'a, Param0: ::windows::runtime::IntoParam<'a, HDESK>>(
    hdesktop: Param0,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetThreadDesktop(hdesktop: HDESK) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetThreadDesktop(hdesktop.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetUserObjectInformationA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hobj: Param0,
    nindex: i32,
    pvinfo: *const ::std::ffi::c_void,
    nlength: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetUserObjectInformationA(
                hobj: super::super::Foundation::HANDLE,
                nindex: i32,
                pvinfo: *const ::std::ffi::c_void,
                nlength: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetUserObjectInformationA(
            hobj.into_param().abi(),
            ::std::mem::transmute(nindex),
            ::std::mem::transmute(pvinfo),
            ::std::mem::transmute(nlength),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetUserObjectInformationW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hobj: Param0,
    nindex: i32,
    pvinfo: *const ::std::ffi::c_void,
    nlength: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetUserObjectInformationW(
                hobj: super::super::Foundation::HANDLE,
                nindex: i32,
                pvinfo: *const ::std::ffi::c_void,
                nlength: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetUserObjectInformationW(
            hobj.into_param().abi(),
            ::std::mem::transmute(nindex),
            ::std::mem::transmute(pvinfo),
            ::std::mem::transmute(nlength),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SwitchDesktop<'a, Param0: ::windows::runtime::IntoParam<'a, HDESK>>(
    hdesktop: Param0,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SwitchDesktop(hdesktop: HDESK) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SwitchDesktop(hdesktop.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct USEROBJECTFLAGS {
    pub fInherit: super::super::Foundation::BOOL,
    pub fReserved: super::super::Foundation::BOOL,
    pub dwFlags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl USEROBJECTFLAGS {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for USEROBJECTFLAGS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for USEROBJECTFLAGS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("USEROBJECTFLAGS")
            .field("fInherit", &self.fInherit)
            .field("fReserved", &self.fReserved)
            .field("dwFlags", &self.dwFlags)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for USEROBJECTFLAGS {
    fn eq(&self, other: &Self) -> bool {
        self.fInherit == other.fInherit
            && self.fReserved == other.fReserved
            && self.dwFlags == other.dwFlags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for USEROBJECTFLAGS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for USEROBJECTFLAGS {
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
pub struct USER_OBJECT_INFORMATION_INDEX(pub u32);
pub const UOI_FLAGS: USER_OBJECT_INFORMATION_INDEX = USER_OBJECT_INFORMATION_INDEX(1u32);
pub const UOI_HEAPSIZE: USER_OBJECT_INFORMATION_INDEX = USER_OBJECT_INFORMATION_INDEX(5u32);
pub const UOI_IO: USER_OBJECT_INFORMATION_INDEX = USER_OBJECT_INFORMATION_INDEX(6u32);
pub const UOI_NAME: USER_OBJECT_INFORMATION_INDEX = USER_OBJECT_INFORMATION_INDEX(2u32);
pub const UOI_TYPE: USER_OBJECT_INFORMATION_INDEX = USER_OBJECT_INFORMATION_INDEX(3u32);
pub const UOI_USER_SID: USER_OBJECT_INFORMATION_INDEX = USER_OBJECT_INFORMATION_INDEX(4u32);
impl ::std::convert::From<u32> for USER_OBJECT_INFORMATION_INDEX {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for USER_OBJECT_INFORMATION_INDEX {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for USER_OBJECT_INFORMATION_INDEX {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for USER_OBJECT_INFORMATION_INDEX {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for USER_OBJECT_INFORMATION_INDEX {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for USER_OBJECT_INFORMATION_INDEX {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for USER_OBJECT_INFORMATION_INDEX {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
