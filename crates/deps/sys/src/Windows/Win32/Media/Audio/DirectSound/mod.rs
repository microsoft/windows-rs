#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub fn DirectSoundCaptureCreate(pcguiddevice: *const ::windows::runtime::GUID, ppdsc: *mut ::windows::runtime::RawPtr, punkouter: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub fn DirectSoundCaptureCreate8(pcguiddevice: *const ::windows::runtime::GUID, ppdsc8: *mut ::windows::runtime::RawPtr, punkouter: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DirectSoundCaptureEnumerateA(pdsenumcallback: ::windows::runtime::RawPtr, pcontext: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DirectSoundCaptureEnumerateW(pdsenumcallback: ::windows::runtime::RawPtr, pcontext: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub fn DirectSoundCreate(pcguiddevice: *const ::windows::runtime::GUID, ppds: *mut ::windows::runtime::RawPtr, punkouter: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub fn DirectSoundCreate8(pcguiddevice: *const ::windows::runtime::GUID, ppds8: *mut ::windows::runtime::RawPtr, punkouter: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DirectSoundEnumerateA(pdsenumcallback: ::windows::runtime::RawPtr, pcontext: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DirectSoundEnumerateW(pdsenumcallback: ::windows::runtime::RawPtr, pcontext: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DirectSoundFullDuplexCreate(pcguidcapturedevice: *const ::windows::runtime::GUID, pcguidrenderdevice: *const ::windows::runtime::GUID, pcdscbufferdesc: *const DSCBUFFERDESC, pcdsbufferdesc: *const DSBUFFERDESC, hwnd: super::super::super::Foundation::HWND, dwlevel: u32, ppdsfd: *mut ::windows::runtime::RawPtr, ppdscbuffer8: *mut ::windows::runtime::RawPtr, ppdsbuffer8: *mut ::windows::runtime::RawPtr, punkouter: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_Audio_DirectSound`*"]
    pub fn GetDeviceID(pguidsrc: *const ::windows::runtime::GUID, pguiddest: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
}
