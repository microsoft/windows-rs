#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub fn DirectDrawCreate(lpguid: *mut ::windows::runtime::GUID, lplpdd: *mut ::windows::runtime::RawPtr, punkouter: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub fn DirectDrawCreateClipper(dwflags: u32, lplpddclipper: *mut ::windows::runtime::RawPtr, punkouter: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`*"]
    pub fn DirectDrawCreateEx(lpguid: *mut ::windows::runtime::GUID, lplpdd: *mut *mut ::core::ffi::c_void, iid: *const ::windows::runtime::GUID, punkouter: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DirectDrawEnumerateA(lpcallback: ::windows::runtime::RawPtr, lpcontext: *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn DirectDrawEnumerateExA(lpcallback: ::windows::runtime::RawPtr, lpcontext: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn DirectDrawEnumerateExW(lpcallback: ::windows::runtime::RawPtr, lpcontext: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_DirectDraw`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DirectDrawEnumerateW(lpcallback: ::windows::runtime::RawPtr, lpcontext: *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
}
