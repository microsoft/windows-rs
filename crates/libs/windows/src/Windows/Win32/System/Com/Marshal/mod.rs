#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BSTR_UserFree(param0: *const u32, param1: *const super::super::super::Foundation::BSTR) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BSTR_UserFree(param0: *const u32, param1: *const super::super::super::Foundation::BSTR);
        }
        BSTR_UserFree(::core::mem::transmute(param0), ::core::mem::transmute(param1))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BSTR_UserFree64(param0: *const u32, param1: *const super::super::super::Foundation::BSTR) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BSTR_UserFree64(param0: *const u32, param1: *const super::super::super::Foundation::BSTR);
        }
        BSTR_UserFree64(::core::mem::transmute(param0), ::core::mem::transmute(param1))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BSTR_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::super::super::Foundation::BSTR) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BSTR_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::super::super::Foundation::BSTR) -> *mut u8;
        }
        ::core::mem::transmute(BSTR_UserMarshal(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BSTR_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::super::super::Foundation::BSTR) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BSTR_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::super::super::Foundation::BSTR) -> *mut u8;
        }
        ::core::mem::transmute(BSTR_UserMarshal64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BSTR_UserSize(param0: *const u32, param1: u32, param2: *const super::super::super::Foundation::BSTR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BSTR_UserSize(param0: *const u32, param1: u32, param2: *const super::super::super::Foundation::BSTR) -> u32;
        }
        ::core::mem::transmute(BSTR_UserSize(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BSTR_UserSize64(param0: *const u32, param1: u32, param2: *const super::super::super::Foundation::BSTR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BSTR_UserSize64(param0: *const u32, param1: u32, param2: *const super::super::super::Foundation::BSTR) -> u32;
        }
        ::core::mem::transmute(BSTR_UserSize64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BSTR_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::super::super::Foundation::BSTR) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BSTR_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::super::super::Foundation::BSTR) -> *mut u8;
        }
        ::core::mem::transmute(BSTR_UserUnmarshal(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BSTR_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::super::super::Foundation::BSTR) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BSTR_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::super::super::Foundation::BSTR) -> *mut u8;
        }
        ::core::mem::transmute(BSTR_UserUnmarshal64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
#[inline]
pub unsafe fn CLIPFORMAT_UserFree(param0: *const u32, param1: *const u16) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CLIPFORMAT_UserFree(param0: *const u32, param1: *const u16);
        }
        CLIPFORMAT_UserFree(::core::mem::transmute(param0), ::core::mem::transmute(param1))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
#[inline]
pub unsafe fn CLIPFORMAT_UserFree64(param0: *const u32, param1: *const u16) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CLIPFORMAT_UserFree64(param0: *const u32, param1: *const u16);
        }
        CLIPFORMAT_UserFree64(::core::mem::transmute(param0), ::core::mem::transmute(param1))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
#[inline]
pub unsafe fn CLIPFORMAT_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const u16) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CLIPFORMAT_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const u16) -> *mut u8;
        }
        ::core::mem::transmute(CLIPFORMAT_UserMarshal(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
#[inline]
pub unsafe fn CLIPFORMAT_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const u16) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CLIPFORMAT_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const u16) -> *mut u8;
        }
        ::core::mem::transmute(CLIPFORMAT_UserMarshal64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
#[inline]
pub unsafe fn CLIPFORMAT_UserSize(param0: *const u32, param1: u32, param2: *const u16) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CLIPFORMAT_UserSize(param0: *const u32, param1: u32, param2: *const u16) -> u32;
        }
        ::core::mem::transmute(CLIPFORMAT_UserSize(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
#[inline]
pub unsafe fn CLIPFORMAT_UserSize64(param0: *const u32, param1: u32, param2: *const u16) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CLIPFORMAT_UserSize64(param0: *const u32, param1: u32, param2: *const u16) -> u32;
        }
        ::core::mem::transmute(CLIPFORMAT_UserSize64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
#[inline]
pub unsafe fn CLIPFORMAT_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut u16) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CLIPFORMAT_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut u16) -> *mut u8;
        }
        ::core::mem::transmute(CLIPFORMAT_UserUnmarshal(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
#[inline]
pub unsafe fn CLIPFORMAT_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut u16) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CLIPFORMAT_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut u16) -> *mut u8;
        }
        ::core::mem::transmute(CLIPFORMAT_UserUnmarshal64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
#[inline]
pub unsafe fn CoGetMarshalSizeMax<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(pulsize: *mut u32, riid: *const ::windows::core::GUID, punk: Param2, dwdestcontext: u32, pvdestcontext: *const ::core::ffi::c_void, mshlflags: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetMarshalSizeMax(pulsize: *mut u32, riid: *const ::windows::core::GUID, punk: *mut ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *const ::core::ffi::c_void, mshlflags: u32) -> ::windows::core::HRESULT;
        }
        CoGetMarshalSizeMax(::core::mem::transmute(pulsize), ::core::mem::transmute(riid), punk.into_param().abi(), ::core::mem::transmute(dwdestcontext), ::core::mem::transmute(pvdestcontext), ::core::mem::transmute(mshlflags)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
#[inline]
pub unsafe fn CoGetStandardMarshal<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(riid: *const ::windows::core::GUID, punk: Param1, dwdestcontext: u32, pvdestcontext: *const ::core::ffi::c_void, mshlflags: u32) -> ::windows::core::Result<IMarshal> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetStandardMarshal(riid: *const ::windows::core::GUID, punk: *mut ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *const ::core::ffi::c_void, mshlflags: u32, ppmarshal: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        CoGetStandardMarshal(::core::mem::transmute(riid), punk.into_param().abi(), ::core::mem::transmute(dwdestcontext), ::core::mem::transmute(pvdestcontext), ::core::mem::transmute(mshlflags), ::core::mem::transmute(&mut result__)).from_abi::<IMarshal>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
#[inline]
pub unsafe fn CoGetStdMarshalEx<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(punkouter: Param0, smexflags: u32) -> ::windows::core::Result<::windows::core::IUnknown> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetStdMarshalEx(punkouter: *mut ::core::ffi::c_void, smexflags: u32, ppunkinner: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        CoGetStdMarshalEx(punkouter.into_param().abi(), ::core::mem::transmute(smexflags), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
#[inline]
pub unsafe fn CoMarshalHresult<'a, Param0: ::windows::core::IntoParam<'a, super::IStream>>(pstm: Param0, hresult: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoMarshalHresult(pstm: ::windows::core::RawPtr, hresult: ::windows::core::HRESULT) -> ::windows::core::HRESULT;
        }
        CoMarshalHresult(pstm.into_param().abi(), ::core::mem::transmute(hresult)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
#[inline]
pub unsafe fn CoMarshalInterThreadInterfaceInStream<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(riid: *const ::windows::core::GUID, punk: Param1) -> ::windows::core::Result<super::IStream> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoMarshalInterThreadInterfaceInStream(riid: *const ::windows::core::GUID, punk: *mut ::core::ffi::c_void, ppstm: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        CoMarshalInterThreadInterfaceInStream(::core::mem::transmute(riid), punk.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::IStream>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
#[inline]
pub unsafe fn CoMarshalInterface<'a, Param0: ::windows::core::IntoParam<'a, super::IStream>, Param2: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(pstm: Param0, riid: *const ::windows::core::GUID, punk: Param2, dwdestcontext: u32, pvdestcontext: *const ::core::ffi::c_void, mshlflags: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoMarshalInterface(pstm: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, punk: *mut ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *const ::core::ffi::c_void, mshlflags: u32) -> ::windows::core::HRESULT;
        }
        CoMarshalInterface(pstm.into_param().abi(), ::core::mem::transmute(riid), punk.into_param().abi(), ::core::mem::transmute(dwdestcontext), ::core::mem::transmute(pvdestcontext), ::core::mem::transmute(mshlflags)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
#[inline]
pub unsafe fn CoReleaseMarshalData<'a, Param0: ::windows::core::IntoParam<'a, super::IStream>>(pstm: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoReleaseMarshalData(pstm: ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        CoReleaseMarshalData(pstm.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
#[inline]
pub unsafe fn CoUnmarshalHresult<'a, Param0: ::windows::core::IntoParam<'a, super::IStream>>(pstm: Param0) -> ::windows::core::Result<::windows::core::HRESULT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoUnmarshalHresult(pstm: ::windows::core::RawPtr, phresult: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
        CoUnmarshalHresult(pstm.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::HRESULT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
#[inline]
pub unsafe fn CoUnmarshalInterface<'a, Param0: ::windows::core::IntoParam<'a, super::IStream>, T: ::windows::core::Interface>(pstm: Param0) -> ::windows::core::Result<T> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoUnmarshalInterface(pstm: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        let mut result__ = ::core::option::Option::None;
        CoUnmarshalInterface(pstm.into_param().abi(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HACCEL_UserFree(param0: *const u32, param1: *const super::super::super::UI::WindowsAndMessaging::HACCEL) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HACCEL_UserFree(param0: *const u32, param1: *const super::super::super::UI::WindowsAndMessaging::HACCEL);
        }
        HACCEL_UserFree(::core::mem::transmute(param0), ::core::mem::transmute(param1))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HACCEL_UserFree64(param0: *const u32, param1: *const super::super::super::UI::WindowsAndMessaging::HACCEL) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HACCEL_UserFree64(param0: *const u32, param1: *const super::super::super::UI::WindowsAndMessaging::HACCEL);
        }
        HACCEL_UserFree64(::core::mem::transmute(param0), ::core::mem::transmute(param1))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HACCEL_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::super::super::UI::WindowsAndMessaging::HACCEL) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HACCEL_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::super::super::UI::WindowsAndMessaging::HACCEL) -> *mut u8;
        }
        ::core::mem::transmute(HACCEL_UserMarshal(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HACCEL_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::super::super::UI::WindowsAndMessaging::HACCEL) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HACCEL_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::super::super::UI::WindowsAndMessaging::HACCEL) -> *mut u8;
        }
        ::core::mem::transmute(HACCEL_UserMarshal64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HACCEL_UserSize(param0: *const u32, param1: u32, param2: *const super::super::super::UI::WindowsAndMessaging::HACCEL) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HACCEL_UserSize(param0: *const u32, param1: u32, param2: *const super::super::super::UI::WindowsAndMessaging::HACCEL) -> u32;
        }
        ::core::mem::transmute(HACCEL_UserSize(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HACCEL_UserSize64(param0: *const u32, param1: u32, param2: *const super::super::super::UI::WindowsAndMessaging::HACCEL) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HACCEL_UserSize64(param0: *const u32, param1: u32, param2: *const super::super::super::UI::WindowsAndMessaging::HACCEL) -> u32;
        }
        ::core::mem::transmute(HACCEL_UserSize64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HACCEL_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::super::super::UI::WindowsAndMessaging::HACCEL) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HACCEL_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::super::super::UI::WindowsAndMessaging::HACCEL) -> *mut u8;
        }
        ::core::mem::transmute(HACCEL_UserUnmarshal(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HACCEL_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::super::super::UI::WindowsAndMessaging::HACCEL) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HACCEL_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::super::super::UI::WindowsAndMessaging::HACCEL) -> *mut u8;
        }
        ::core::mem::transmute(HACCEL_UserUnmarshal64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HBITMAP_UserFree(param0: *const u32, param1: *const super::super::super::Graphics::Gdi::HBITMAP) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HBITMAP_UserFree(param0: *const u32, param1: *const super::super::super::Graphics::Gdi::HBITMAP);
        }
        HBITMAP_UserFree(::core::mem::transmute(param0), ::core::mem::transmute(param1))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HBITMAP_UserFree64(param0: *const u32, param1: *const super::super::super::Graphics::Gdi::HBITMAP) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HBITMAP_UserFree64(param0: *const u32, param1: *const super::super::super::Graphics::Gdi::HBITMAP);
        }
        HBITMAP_UserFree64(::core::mem::transmute(param0), ::core::mem::transmute(param1))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HBITMAP_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::super::super::Graphics::Gdi::HBITMAP) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HBITMAP_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::super::super::Graphics::Gdi::HBITMAP) -> *mut u8;
        }
        ::core::mem::transmute(HBITMAP_UserMarshal(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HBITMAP_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::super::super::Graphics::Gdi::HBITMAP) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HBITMAP_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::super::super::Graphics::Gdi::HBITMAP) -> *mut u8;
        }
        ::core::mem::transmute(HBITMAP_UserMarshal64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HBITMAP_UserSize(param0: *const u32, param1: u32, param2: *const super::super::super::Graphics::Gdi::HBITMAP) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HBITMAP_UserSize(param0: *const u32, param1: u32, param2: *const super::super::super::Graphics::Gdi::HBITMAP) -> u32;
        }
        ::core::mem::transmute(HBITMAP_UserSize(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HBITMAP_UserSize64(param0: *const u32, param1: u32, param2: *const super::super::super::Graphics::Gdi::HBITMAP) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HBITMAP_UserSize64(param0: *const u32, param1: u32, param2: *const super::super::super::Graphics::Gdi::HBITMAP) -> u32;
        }
        ::core::mem::transmute(HBITMAP_UserSize64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HBITMAP_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::super::super::Graphics::Gdi::HBITMAP) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HBITMAP_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::super::super::Graphics::Gdi::HBITMAP) -> *mut u8;
        }
        ::core::mem::transmute(HBITMAP_UserUnmarshal(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HBITMAP_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::super::super::Graphics::Gdi::HBITMAP) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HBITMAP_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::super::super::Graphics::Gdi::HBITMAP) -> *mut u8;
        }
        ::core::mem::transmute(HBITMAP_UserUnmarshal64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HDC_UserFree(param0: *const u32, param1: *const super::super::super::Graphics::Gdi::HDC) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HDC_UserFree(param0: *const u32, param1: *const super::super::super::Graphics::Gdi::HDC);
        }
        HDC_UserFree(::core::mem::transmute(param0), ::core::mem::transmute(param1))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HDC_UserFree64(param0: *const u32, param1: *const super::super::super::Graphics::Gdi::HDC) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HDC_UserFree64(param0: *const u32, param1: *const super::super::super::Graphics::Gdi::HDC);
        }
        HDC_UserFree64(::core::mem::transmute(param0), ::core::mem::transmute(param1))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HDC_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::super::super::Graphics::Gdi::HDC) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HDC_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::super::super::Graphics::Gdi::HDC) -> *mut u8;
        }
        ::core::mem::transmute(HDC_UserMarshal(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HDC_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::super::super::Graphics::Gdi::HDC) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HDC_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::super::super::Graphics::Gdi::HDC) -> *mut u8;
        }
        ::core::mem::transmute(HDC_UserMarshal64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HDC_UserSize(param0: *const u32, param1: u32, param2: *const super::super::super::Graphics::Gdi::HDC) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HDC_UserSize(param0: *const u32, param1: u32, param2: *const super::super::super::Graphics::Gdi::HDC) -> u32;
        }
        ::core::mem::transmute(HDC_UserSize(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HDC_UserSize64(param0: *const u32, param1: u32, param2: *const super::super::super::Graphics::Gdi::HDC) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HDC_UserSize64(param0: *const u32, param1: u32, param2: *const super::super::super::Graphics::Gdi::HDC) -> u32;
        }
        ::core::mem::transmute(HDC_UserSize64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HDC_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::super::super::Graphics::Gdi::HDC) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HDC_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::super::super::Graphics::Gdi::HDC) -> *mut u8;
        }
        ::core::mem::transmute(HDC_UserUnmarshal(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HDC_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::super::super::Graphics::Gdi::HDC) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HDC_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::super::super::Graphics::Gdi::HDC) -> *mut u8;
        }
        ::core::mem::transmute(HDC_UserUnmarshal64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
#[inline]
pub unsafe fn HGLOBAL_UserFree(param0: *const u32, param1: *const isize) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HGLOBAL_UserFree(param0: *const u32, param1: *const isize);
        }
        HGLOBAL_UserFree(::core::mem::transmute(param0), ::core::mem::transmute(param1))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
#[inline]
pub unsafe fn HGLOBAL_UserFree64(param0: *const u32, param1: *const isize) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HGLOBAL_UserFree64(param0: *const u32, param1: *const isize);
        }
        HGLOBAL_UserFree64(::core::mem::transmute(param0), ::core::mem::transmute(param1))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
#[inline]
pub unsafe fn HGLOBAL_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const isize) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HGLOBAL_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const isize) -> *mut u8;
        }
        ::core::mem::transmute(HGLOBAL_UserMarshal(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
#[inline]
pub unsafe fn HGLOBAL_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const isize) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HGLOBAL_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const isize) -> *mut u8;
        }
        ::core::mem::transmute(HGLOBAL_UserMarshal64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
#[inline]
pub unsafe fn HGLOBAL_UserSize(param0: *const u32, param1: u32, param2: *const isize) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HGLOBAL_UserSize(param0: *const u32, param1: u32, param2: *const isize) -> u32;
        }
        ::core::mem::transmute(HGLOBAL_UserSize(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
#[inline]
pub unsafe fn HGLOBAL_UserSize64(param0: *const u32, param1: u32, param2: *const isize) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HGLOBAL_UserSize64(param0: *const u32, param1: u32, param2: *const isize) -> u32;
        }
        ::core::mem::transmute(HGLOBAL_UserSize64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
#[inline]
pub unsafe fn HGLOBAL_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut isize) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HGLOBAL_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut isize) -> *mut u8;
        }
        ::core::mem::transmute(HGLOBAL_UserUnmarshal(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
#[inline]
pub unsafe fn HGLOBAL_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut isize) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HGLOBAL_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut isize) -> *mut u8;
        }
        ::core::mem::transmute(HGLOBAL_UserUnmarshal64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HICON_UserFree(param0: *const u32, param1: *const super::super::super::UI::WindowsAndMessaging::HICON) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HICON_UserFree(param0: *const u32, param1: *const super::super::super::UI::WindowsAndMessaging::HICON);
        }
        HICON_UserFree(::core::mem::transmute(param0), ::core::mem::transmute(param1))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HICON_UserFree64(param0: *const u32, param1: *const super::super::super::UI::WindowsAndMessaging::HICON) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HICON_UserFree64(param0: *const u32, param1: *const super::super::super::UI::WindowsAndMessaging::HICON);
        }
        HICON_UserFree64(::core::mem::transmute(param0), ::core::mem::transmute(param1))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HICON_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::super::super::UI::WindowsAndMessaging::HICON) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HICON_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::super::super::UI::WindowsAndMessaging::HICON) -> *mut u8;
        }
        ::core::mem::transmute(HICON_UserMarshal(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HICON_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::super::super::UI::WindowsAndMessaging::HICON) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HICON_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::super::super::UI::WindowsAndMessaging::HICON) -> *mut u8;
        }
        ::core::mem::transmute(HICON_UserMarshal64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HICON_UserSize(param0: *const u32, param1: u32, param2: *const super::super::super::UI::WindowsAndMessaging::HICON) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HICON_UserSize(param0: *const u32, param1: u32, param2: *const super::super::super::UI::WindowsAndMessaging::HICON) -> u32;
        }
        ::core::mem::transmute(HICON_UserSize(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HICON_UserSize64(param0: *const u32, param1: u32, param2: *const super::super::super::UI::WindowsAndMessaging::HICON) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HICON_UserSize64(param0: *const u32, param1: u32, param2: *const super::super::super::UI::WindowsAndMessaging::HICON) -> u32;
        }
        ::core::mem::transmute(HICON_UserSize64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HICON_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::super::super::UI::WindowsAndMessaging::HICON) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HICON_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::super::super::UI::WindowsAndMessaging::HICON) -> *mut u8;
        }
        ::core::mem::transmute(HICON_UserUnmarshal(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HICON_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::super::super::UI::WindowsAndMessaging::HICON) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HICON_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::super::super::UI::WindowsAndMessaging::HICON) -> *mut u8;
        }
        ::core::mem::transmute(HICON_UserUnmarshal64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HMENU_UserFree(param0: *const u32, param1: *const super::super::super::UI::WindowsAndMessaging::HMENU) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HMENU_UserFree(param0: *const u32, param1: *const super::super::super::UI::WindowsAndMessaging::HMENU);
        }
        HMENU_UserFree(::core::mem::transmute(param0), ::core::mem::transmute(param1))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HMENU_UserFree64(param0: *const u32, param1: *const super::super::super::UI::WindowsAndMessaging::HMENU) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HMENU_UserFree64(param0: *const u32, param1: *const super::super::super::UI::WindowsAndMessaging::HMENU);
        }
        HMENU_UserFree64(::core::mem::transmute(param0), ::core::mem::transmute(param1))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HMENU_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::super::super::UI::WindowsAndMessaging::HMENU) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HMENU_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::super::super::UI::WindowsAndMessaging::HMENU) -> *mut u8;
        }
        ::core::mem::transmute(HMENU_UserMarshal(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HMENU_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::super::super::UI::WindowsAndMessaging::HMENU) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HMENU_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::super::super::UI::WindowsAndMessaging::HMENU) -> *mut u8;
        }
        ::core::mem::transmute(HMENU_UserMarshal64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HMENU_UserSize(param0: *const u32, param1: u32, param2: *const super::super::super::UI::WindowsAndMessaging::HMENU) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HMENU_UserSize(param0: *const u32, param1: u32, param2: *const super::super::super::UI::WindowsAndMessaging::HMENU) -> u32;
        }
        ::core::mem::transmute(HMENU_UserSize(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HMENU_UserSize64(param0: *const u32, param1: u32, param2: *const super::super::super::UI::WindowsAndMessaging::HMENU) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HMENU_UserSize64(param0: *const u32, param1: u32, param2: *const super::super::super::UI::WindowsAndMessaging::HMENU) -> u32;
        }
        ::core::mem::transmute(HMENU_UserSize64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HMENU_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::super::super::UI::WindowsAndMessaging::HMENU) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HMENU_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::super::super::UI::WindowsAndMessaging::HMENU) -> *mut u8;
        }
        ::core::mem::transmute(HMENU_UserUnmarshal(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HMENU_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::super::super::UI::WindowsAndMessaging::HMENU) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HMENU_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::super::super::UI::WindowsAndMessaging::HMENU) -> *mut u8;
        }
        ::core::mem::transmute(HMENU_UserUnmarshal64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HPALETTE_UserFree(param0: *const u32, param1: *const super::super::super::Graphics::Gdi::HPALETTE) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HPALETTE_UserFree(param0: *const u32, param1: *const super::super::super::Graphics::Gdi::HPALETTE);
        }
        HPALETTE_UserFree(::core::mem::transmute(param0), ::core::mem::transmute(param1))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HPALETTE_UserFree64(param0: *const u32, param1: *const super::super::super::Graphics::Gdi::HPALETTE) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HPALETTE_UserFree64(param0: *const u32, param1: *const super::super::super::Graphics::Gdi::HPALETTE);
        }
        HPALETTE_UserFree64(::core::mem::transmute(param0), ::core::mem::transmute(param1))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HPALETTE_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::super::super::Graphics::Gdi::HPALETTE) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HPALETTE_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::super::super::Graphics::Gdi::HPALETTE) -> *mut u8;
        }
        ::core::mem::transmute(HPALETTE_UserMarshal(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HPALETTE_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::super::super::Graphics::Gdi::HPALETTE) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HPALETTE_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::super::super::Graphics::Gdi::HPALETTE) -> *mut u8;
        }
        ::core::mem::transmute(HPALETTE_UserMarshal64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HPALETTE_UserSize(param0: *const u32, param1: u32, param2: *const super::super::super::Graphics::Gdi::HPALETTE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HPALETTE_UserSize(param0: *const u32, param1: u32, param2: *const super::super::super::Graphics::Gdi::HPALETTE) -> u32;
        }
        ::core::mem::transmute(HPALETTE_UserSize(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HPALETTE_UserSize64(param0: *const u32, param1: u32, param2: *const super::super::super::Graphics::Gdi::HPALETTE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HPALETTE_UserSize64(param0: *const u32, param1: u32, param2: *const super::super::super::Graphics::Gdi::HPALETTE) -> u32;
        }
        ::core::mem::transmute(HPALETTE_UserSize64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HPALETTE_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::super::super::Graphics::Gdi::HPALETTE) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HPALETTE_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::super::super::Graphics::Gdi::HPALETTE) -> *mut u8;
        }
        ::core::mem::transmute(HPALETTE_UserUnmarshal(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HPALETTE_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::super::super::Graphics::Gdi::HPALETTE) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HPALETTE_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::super::super::Graphics::Gdi::HPALETTE) -> *mut u8;
        }
        ::core::mem::transmute(HPALETTE_UserUnmarshal64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HWND_UserFree(param0: *const u32, param1: *const super::super::super::Foundation::HWND) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HWND_UserFree(param0: *const u32, param1: *const super::super::super::Foundation::HWND);
        }
        HWND_UserFree(::core::mem::transmute(param0), ::core::mem::transmute(param1))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HWND_UserFree64(param0: *const u32, param1: *const super::super::super::Foundation::HWND) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HWND_UserFree64(param0: *const u32, param1: *const super::super::super::Foundation::HWND);
        }
        HWND_UserFree64(::core::mem::transmute(param0), ::core::mem::transmute(param1))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HWND_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::super::super::Foundation::HWND) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HWND_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::super::super::Foundation::HWND) -> *mut u8;
        }
        ::core::mem::transmute(HWND_UserMarshal(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HWND_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::super::super::Foundation::HWND) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HWND_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::super::super::Foundation::HWND) -> *mut u8;
        }
        ::core::mem::transmute(HWND_UserMarshal64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HWND_UserSize(param0: *const u32, param1: u32, param2: *const super::super::super::Foundation::HWND) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HWND_UserSize(param0: *const u32, param1: u32, param2: *const super::super::super::Foundation::HWND) -> u32;
        }
        ::core::mem::transmute(HWND_UserSize(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HWND_UserSize64(param0: *const u32, param1: u32, param2: *const super::super::super::Foundation::HWND) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HWND_UserSize64(param0: *const u32, param1: u32, param2: *const super::super::super::Foundation::HWND) -> u32;
        }
        ::core::mem::transmute(HWND_UserSize64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HWND_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::super::super::Foundation::HWND) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HWND_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::super::super::Foundation::HWND) -> *mut u8;
        }
        ::core::mem::transmute(HWND_UserUnmarshal(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HWND_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::super::super::Foundation::HWND) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HWND_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::super::super::Foundation::HWND) -> *mut u8;
        }
        ::core::mem::transmute(HWND_UserUnmarshal64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
#[repr(transparent)]
pub struct IMarshal(::windows::core::IUnknown);
impl IMarshal {
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
    pub unsafe fn GetUnmarshalClass(&self, riid: *const ::windows::core::GUID, pv: *const ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *mut ::core::ffi::c_void, mshlflags: u32, pcid: *mut ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetUnmarshalClass)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(pv), ::core::mem::transmute(dwdestcontext), ::core::mem::transmute(pvdestcontext), ::core::mem::transmute(mshlflags), ::core::mem::transmute(pcid)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
    pub unsafe fn GetMarshalSizeMax(&self, riid: *const ::windows::core::GUID, pv: *const ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *mut ::core::ffi::c_void, mshlflags: u32, psize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetMarshalSizeMax)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(pv), ::core::mem::transmute(dwdestcontext), ::core::mem::transmute(pvdestcontext), ::core::mem::transmute(mshlflags), ::core::mem::transmute(psize)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
    pub unsafe fn MarshalInterface<'a, Param0: ::windows::core::IntoParam<'a, super::IStream>>(&self, pstm: Param0, riid: *const ::windows::core::GUID, pv: *const ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *mut ::core::ffi::c_void, mshlflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).MarshalInterface)(::core::mem::transmute_copy(self), pstm.into_param().abi(), ::core::mem::transmute(riid), ::core::mem::transmute(pv), ::core::mem::transmute(dwdestcontext), ::core::mem::transmute(pvdestcontext), ::core::mem::transmute(mshlflags)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
    pub unsafe fn UnmarshalInterface<'a, Param0: ::windows::core::IntoParam<'a, super::IStream>>(&self, pstm: Param0, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UnmarshalInterface)(::core::mem::transmute_copy(self), pstm.into_param().abi(), ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
    pub unsafe fn ReleaseMarshalData<'a, Param0: ::windows::core::IntoParam<'a, super::IStream>>(&self, pstm: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReleaseMarshalData)(::core::mem::transmute_copy(self), pstm.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
    pub unsafe fn DisconnectObject(&self, dwreserved: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DisconnectObject)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwreserved)).ok()
    }
}
impl ::core::convert::From<IMarshal> for ::windows::core::IUnknown {
    fn from(value: IMarshal) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMarshal> for ::windows::core::IUnknown {
    fn from(value: &IMarshal) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMarshal {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMarshal {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMarshal {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMarshal {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMarshal {}
impl ::core::fmt::Debug for IMarshal {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMarshal").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMarshal {
    type Vtable = IMarshal_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000003_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMarshal_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetUnmarshalClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, pv: *const ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *mut ::core::ffi::c_void, mshlflags: u32, pcid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetMarshalSizeMax: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, pv: *const ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *mut ::core::ffi::c_void, mshlflags: u32, psize: *mut u32) -> ::windows::core::HRESULT,
    pub MarshalInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstm: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, pv: *const ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *mut ::core::ffi::c_void, mshlflags: u32) -> ::windows::core::HRESULT,
    pub UnmarshalInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstm: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ReleaseMarshalData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstm: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub DisconnectObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwreserved: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
#[repr(transparent)]
pub struct IMarshal2(::windows::core::IUnknown);
impl IMarshal2 {
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
    pub unsafe fn GetUnmarshalClass(&self, riid: *const ::windows::core::GUID, pv: *const ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *mut ::core::ffi::c_void, mshlflags: u32, pcid: *mut ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetUnmarshalClass)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(pv), ::core::mem::transmute(dwdestcontext), ::core::mem::transmute(pvdestcontext), ::core::mem::transmute(mshlflags), ::core::mem::transmute(pcid)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
    pub unsafe fn GetMarshalSizeMax(&self, riid: *const ::windows::core::GUID, pv: *const ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *mut ::core::ffi::c_void, mshlflags: u32, psize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetMarshalSizeMax)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(pv), ::core::mem::transmute(dwdestcontext), ::core::mem::transmute(pvdestcontext), ::core::mem::transmute(mshlflags), ::core::mem::transmute(psize)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
    pub unsafe fn MarshalInterface<'a, Param0: ::windows::core::IntoParam<'a, super::IStream>>(&self, pstm: Param0, riid: *const ::windows::core::GUID, pv: *const ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *mut ::core::ffi::c_void, mshlflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.MarshalInterface)(::core::mem::transmute_copy(self), pstm.into_param().abi(), ::core::mem::transmute(riid), ::core::mem::transmute(pv), ::core::mem::transmute(dwdestcontext), ::core::mem::transmute(pvdestcontext), ::core::mem::transmute(mshlflags)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
    pub unsafe fn UnmarshalInterface<'a, Param0: ::windows::core::IntoParam<'a, super::IStream>>(&self, pstm: Param0, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.UnmarshalInterface)(::core::mem::transmute_copy(self), pstm.into_param().abi(), ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
    pub unsafe fn ReleaseMarshalData<'a, Param0: ::windows::core::IntoParam<'a, super::IStream>>(&self, pstm: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.ReleaseMarshalData)(::core::mem::transmute_copy(self), pstm.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
    pub unsafe fn DisconnectObject(&self, dwreserved: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.DisconnectObject)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwreserved)).ok()
    }
}
impl ::core::convert::From<IMarshal2> for ::windows::core::IUnknown {
    fn from(value: IMarshal2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMarshal2> for ::windows::core::IUnknown {
    fn from(value: &IMarshal2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMarshal2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMarshal2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IMarshal2> for IMarshal {
    fn from(value: IMarshal2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMarshal2> for IMarshal {
    fn from(value: &IMarshal2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMarshal> for IMarshal2 {
    fn into_param(self) -> ::windows::core::Param<'a, IMarshal> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMarshal> for &'a IMarshal2 {
    fn into_param(self) -> ::windows::core::Param<'a, IMarshal> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMarshal2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMarshal2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMarshal2 {}
impl ::core::fmt::Debug for IMarshal2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMarshal2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMarshal2 {
    type Vtable = IMarshal2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x000001cf_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMarshal2_Vtbl {
    pub base: IMarshal_Vtbl,
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
#[repr(transparent)]
pub struct IMarshalingStream(::windows::core::IUnknown);
impl IMarshalingStream {
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
    pub unsafe fn Read(&self, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.Read)(::core::mem::transmute_copy(self), ::core::mem::transmute(pv), ::core::mem::transmute(cb), ::core::mem::transmute(pcbread)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
    pub unsafe fn Write(&self, pv: *const ::core::ffi::c_void, cb: u32) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.Write)(::core::mem::transmute_copy(self), ::core::mem::transmute(pv), ::core::mem::transmute(cb), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
    pub unsafe fn Seek(&self, dlibmove: i64, dworigin: super::STREAM_SEEK) -> ::windows::core::Result<u64> {
        let mut result__: u64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Seek)(::core::mem::transmute_copy(self), ::core::mem::transmute(dlibmove), ::core::mem::transmute(dworigin), ::core::mem::transmute(&mut result__)).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
    pub unsafe fn SetSize(&self, libnewsize: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetSize)(::core::mem::transmute_copy(self), ::core::mem::transmute(libnewsize)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
    pub unsafe fn CopyTo<'a, Param0: ::windows::core::IntoParam<'a, super::IStream>>(&self, pstm: Param0, cb: u64, pcbread: *mut u64, pcbwritten: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.CopyTo)(::core::mem::transmute_copy(self), pstm.into_param().abi(), ::core::mem::transmute(cb), ::core::mem::transmute(pcbread), ::core::mem::transmute(pcbwritten)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn Commit(&self, grfcommitflags: super::StructuredStorage::STGC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Commit)(::core::mem::transmute_copy(self), ::core::mem::transmute(grfcommitflags)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
    pub unsafe fn Revert(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Revert)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
    pub unsafe fn LockRegion(&self, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.LockRegion)(::core::mem::transmute_copy(self), ::core::mem::transmute(liboffset), ::core::mem::transmute(cb), ::core::mem::transmute(dwlocktype)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
    pub unsafe fn UnlockRegion(&self, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.UnlockRegion)(::core::mem::transmute_copy(self), ::core::mem::transmute(liboffset), ::core::mem::transmute(cb), ::core::mem::transmute(dwlocktype)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Stat(&self, pstatstg: *mut super::STATSTG, grfstatflag: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Stat)(::core::mem::transmute_copy(self), ::core::mem::transmute(pstatstg), ::core::mem::transmute(grfstatflag)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<super::IStream> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Clone)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::IStream>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
    pub unsafe fn GetMarshalingContextAttribute(&self, attribute: super::CO_MARSHALING_CONTEXT_ATTRIBUTES) -> ::windows::core::Result<usize> {
        let mut result__: usize = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetMarshalingContextAttribute)(::core::mem::transmute_copy(self), ::core::mem::transmute(attribute), ::core::mem::transmute(&mut result__)).from_abi::<usize>(result__)
    }
}
impl ::core::convert::From<IMarshalingStream> for ::windows::core::IUnknown {
    fn from(value: IMarshalingStream) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMarshalingStream> for ::windows::core::IUnknown {
    fn from(value: &IMarshalingStream) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMarshalingStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMarshalingStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IMarshalingStream> for super::ISequentialStream {
    fn from(value: IMarshalingStream) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMarshalingStream> for super::ISequentialStream {
    fn from(value: &IMarshalingStream) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::ISequentialStream> for IMarshalingStream {
    fn into_param(self) -> ::windows::core::Param<'a, super::ISequentialStream> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::ISequentialStream> for &'a IMarshalingStream {
    fn into_param(self) -> ::windows::core::Param<'a, super::ISequentialStream> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IMarshalingStream> for super::IStream {
    fn from(value: IMarshalingStream) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMarshalingStream> for super::IStream {
    fn from(value: &IMarshalingStream) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IStream> for IMarshalingStream {
    fn into_param(self) -> ::windows::core::Param<'a, super::IStream> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IStream> for &'a IMarshalingStream {
    fn into_param(self) -> ::windows::core::Param<'a, super::IStream> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMarshalingStream {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMarshalingStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMarshalingStream {}
impl ::core::fmt::Debug for IMarshalingStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMarshalingStream").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMarshalingStream {
    type Vtable = IMarshalingStream_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd8f2f5e6_6102_4863_9f26_389a4676efde);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMarshalingStream_Vtbl {
    pub base: super::IStream_Vtbl,
    pub GetMarshalingContextAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attribute: super::CO_MARSHALING_CONTEXT_ATTRIBUTES, pattributevalue: *mut usize) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
#[inline]
pub unsafe fn LPSAFEARRAY_UserFree(param0: *const u32, param1: *const *const super::SAFEARRAY) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LPSAFEARRAY_UserFree(param0: *const u32, param1: *const *const super::SAFEARRAY);
        }
        LPSAFEARRAY_UserFree(::core::mem::transmute(param0), ::core::mem::transmute(param1))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
#[inline]
pub unsafe fn LPSAFEARRAY_UserFree64(param0: *const u32, param1: *const *const super::SAFEARRAY) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LPSAFEARRAY_UserFree64(param0: *const u32, param1: *const *const super::SAFEARRAY);
        }
        LPSAFEARRAY_UserFree64(::core::mem::transmute(param0), ::core::mem::transmute(param1))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
#[inline]
pub unsafe fn LPSAFEARRAY_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const *const super::SAFEARRAY) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LPSAFEARRAY_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const *const super::SAFEARRAY) -> *mut u8;
        }
        ::core::mem::transmute(LPSAFEARRAY_UserMarshal(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
#[inline]
pub unsafe fn LPSAFEARRAY_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const *const super::SAFEARRAY) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LPSAFEARRAY_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const *const super::SAFEARRAY) -> *mut u8;
        }
        ::core::mem::transmute(LPSAFEARRAY_UserMarshal64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
#[inline]
pub unsafe fn LPSAFEARRAY_UserSize(param0: *const u32, param1: u32, param2: *const *const super::SAFEARRAY) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LPSAFEARRAY_UserSize(param0: *const u32, param1: u32, param2: *const *const super::SAFEARRAY) -> u32;
        }
        ::core::mem::transmute(LPSAFEARRAY_UserSize(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
#[inline]
pub unsafe fn LPSAFEARRAY_UserSize64(param0: *const u32, param1: u32, param2: *const *const super::SAFEARRAY) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LPSAFEARRAY_UserSize64(param0: *const u32, param1: u32, param2: *const *const super::SAFEARRAY) -> u32;
        }
        ::core::mem::transmute(LPSAFEARRAY_UserSize64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
#[inline]
pub unsafe fn LPSAFEARRAY_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut *mut super::SAFEARRAY) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LPSAFEARRAY_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut *mut super::SAFEARRAY) -> *mut u8;
        }
        ::core::mem::transmute(LPSAFEARRAY_UserUnmarshal(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
#[inline]
pub unsafe fn LPSAFEARRAY_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut *mut super::SAFEARRAY) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LPSAFEARRAY_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut *mut super::SAFEARRAY) -> *mut u8;
        }
        ::core::mem::transmute(LPSAFEARRAY_UserUnmarshal64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
#[inline]
pub unsafe fn SNB_UserFree(param0: *const u32, param1: *const *const *const u16) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SNB_UserFree(param0: *const u32, param1: *const *const *const u16);
        }
        SNB_UserFree(::core::mem::transmute(param0), ::core::mem::transmute(param1))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
#[inline]
pub unsafe fn SNB_UserFree64(param0: *const u32, param1: *const *const *const u16) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SNB_UserFree64(param0: *const u32, param1: *const *const *const u16);
        }
        SNB_UserFree64(::core::mem::transmute(param0), ::core::mem::transmute(param1))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
#[inline]
pub unsafe fn SNB_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const *const *const u16) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SNB_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const *const *const u16) -> *mut u8;
        }
        ::core::mem::transmute(SNB_UserMarshal(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
#[inline]
pub unsafe fn SNB_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const *const *const u16) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SNB_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const *const *const u16) -> *mut u8;
        }
        ::core::mem::transmute(SNB_UserMarshal64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
#[inline]
pub unsafe fn SNB_UserSize(param0: *const u32, param1: u32, param2: *const *const *const u16) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SNB_UserSize(param0: *const u32, param1: u32, param2: *const *const *const u16) -> u32;
        }
        ::core::mem::transmute(SNB_UserSize(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
#[inline]
pub unsafe fn SNB_UserSize64(param0: *const u32, param1: u32, param2: *const *const *const u16) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SNB_UserSize64(param0: *const u32, param1: u32, param2: *const *const *const u16) -> u32;
        }
        ::core::mem::transmute(SNB_UserSize64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
#[inline]
pub unsafe fn SNB_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut *mut *mut u16) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SNB_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut *mut *mut u16) -> *mut u8;
        }
        ::core::mem::transmute(SNB_UserUnmarshal(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
#[inline]
pub unsafe fn SNB_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut *mut *mut u16) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SNB_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut *mut *mut u16) -> *mut u8;
        }
        ::core::mem::transmute(SNB_UserUnmarshal64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct STDMSHLFLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
pub const SMEXF_SERVER: STDMSHLFLAGS = STDMSHLFLAGS(1i32);
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
pub const SMEXF_HANDLER: STDMSHLFLAGS = STDMSHLFLAGS(2i32);
impl ::core::marker::Copy for STDMSHLFLAGS {}
impl ::core::clone::Clone for STDMSHLFLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for STDMSHLFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for STDMSHLFLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for STDMSHLFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STDMSHLFLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn STGMEDIUM_UserFree(param0: *const u32, param1: *const super::STGMEDIUM) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn STGMEDIUM_UserFree(param0: *const u32, param1: *const super::STGMEDIUM);
        }
        STGMEDIUM_UserFree(::core::mem::transmute(param0), ::core::mem::transmute(param1))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn STGMEDIUM_UserFree64(param0: *const u32, param1: *const super::STGMEDIUM) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn STGMEDIUM_UserFree64(param0: *const u32, param1: *const super::STGMEDIUM);
        }
        STGMEDIUM_UserFree64(::core::mem::transmute(param0), ::core::mem::transmute(param1))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn STGMEDIUM_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::STGMEDIUM) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn STGMEDIUM_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::STGMEDIUM) -> *mut u8;
        }
        ::core::mem::transmute(STGMEDIUM_UserMarshal(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn STGMEDIUM_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::STGMEDIUM) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn STGMEDIUM_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::STGMEDIUM) -> *mut u8;
        }
        ::core::mem::transmute(STGMEDIUM_UserMarshal64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn STGMEDIUM_UserSize(param0: *const u32, param1: u32, param2: *const super::STGMEDIUM) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn STGMEDIUM_UserSize(param0: *const u32, param1: u32, param2: *const super::STGMEDIUM) -> u32;
        }
        ::core::mem::transmute(STGMEDIUM_UserSize(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn STGMEDIUM_UserSize64(param0: *const u32, param1: u32, param2: *const super::STGMEDIUM) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn STGMEDIUM_UserSize64(param0: *const u32, param1: u32, param2: *const super::STGMEDIUM) -> u32;
        }
        ::core::mem::transmute(STGMEDIUM_UserSize64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn STGMEDIUM_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::STGMEDIUM) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn STGMEDIUM_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::STGMEDIUM) -> *mut u8;
        }
        ::core::mem::transmute(STGMEDIUM_UserUnmarshal(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn STGMEDIUM_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::STGMEDIUM) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn STGMEDIUM_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::STGMEDIUM) -> *mut u8;
        }
        ::core::mem::transmute(STGMEDIUM_UserUnmarshal64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VARIANT_UserFree(param0: *const u32, param1: *const super::VARIANT) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VARIANT_UserFree(param0: *const u32, param1: *const super::VARIANT);
        }
        VARIANT_UserFree(::core::mem::transmute(param0), ::core::mem::transmute(param1))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VARIANT_UserFree64(param0: *const u32, param1: *const super::VARIANT) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VARIANT_UserFree64(param0: *const u32, param1: *const super::VARIANT);
        }
        VARIANT_UserFree64(::core::mem::transmute(param0), ::core::mem::transmute(param1))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VARIANT_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::VARIANT) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VARIANT_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::VARIANT) -> *mut u8;
        }
        ::core::mem::transmute(VARIANT_UserMarshal(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VARIANT_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::VARIANT) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VARIANT_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::VARIANT) -> *mut u8;
        }
        ::core::mem::transmute(VARIANT_UserMarshal64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VARIANT_UserSize(param0: *const u32, param1: u32, param2: *const super::VARIANT) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VARIANT_UserSize(param0: *const u32, param1: u32, param2: *const super::VARIANT) -> u32;
        }
        ::core::mem::transmute(VARIANT_UserSize(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VARIANT_UserSize64(param0: *const u32, param1: u32, param2: *const super::VARIANT) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VARIANT_UserSize64(param0: *const u32, param1: u32, param2: *const super::VARIANT) -> u32;
        }
        ::core::mem::transmute(VARIANT_UserSize64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VARIANT_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::VARIANT) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VARIANT_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::VARIANT) -> *mut u8;
        }
        ::core::mem::transmute(VARIANT_UserUnmarshal(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
#[inline]
pub unsafe fn VARIANT_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::VARIANT) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VARIANT_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::VARIANT) -> *mut u8;
        }
        ::core::mem::transmute(VARIANT_UserUnmarshal64(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
