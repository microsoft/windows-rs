#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Storage_InstallableFileSystems`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FilterAttach(lpfiltername: super::super::Foundation::PWSTR, lpvolumename: super::super::Foundation::PWSTR, lpinstancename: super::super::Foundation::PWSTR, dwcreatedinstancenamelength: u32, lpcreatedinstancename: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Storage_InstallableFileSystems`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FilterAttachAtAltitude(lpfiltername: super::super::Foundation::PWSTR, lpvolumename: super::super::Foundation::PWSTR, lpaltitude: super::super::Foundation::PWSTR, lpinstancename: super::super::Foundation::PWSTR, dwcreatedinstancenamelength: u32, lpcreatedinstancename: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Storage_InstallableFileSystems`*"]
    pub fn FilterClose(hfilter: HFILTER) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Storage_InstallableFileSystems`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FilterConnectCommunicationPort(lpportname: super::super::Foundation::PWSTR, dwoptions: u32, lpcontext: *const ::core::ffi::c_void, wsizeofcontext: u16, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, hport: *mut super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Storage_InstallableFileSystems`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FilterCreate(lpfiltername: super::super::Foundation::PWSTR, hfilter: *mut HFILTER) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Storage_InstallableFileSystems`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FilterDetach(lpfiltername: super::super::Foundation::PWSTR, lpvolumename: super::super::Foundation::PWSTR, lpinstancename: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Storage_InstallableFileSystems`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FilterFindClose(hfilterfind: super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Storage_InstallableFileSystems`*"]
    pub fn FilterFindFirst(dwinformationclass: FILTER_INFORMATION_CLASS, lpbuffer: *mut ::core::ffi::c_void, dwbuffersize: u32, lpbytesreturned: *mut u32, lpfilterfind: *mut FilterFindHandle) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Storage_InstallableFileSystems`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FilterFindNext(hfilterfind: super::super::Foundation::HANDLE, dwinformationclass: FILTER_INFORMATION_CLASS, lpbuffer: *mut ::core::ffi::c_void, dwbuffersize: u32, lpbytesreturned: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Storage_InstallableFileSystems`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FilterGetDosName(lpvolumename: super::super::Foundation::PWSTR, lpdosname: super::super::Foundation::PWSTR, dwdosnamebuffersize: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Storage_InstallableFileSystems`*"]
    pub fn FilterGetInformation(hfilter: HFILTER, dwinformationclass: FILTER_INFORMATION_CLASS, lpbuffer: *mut ::core::ffi::c_void, dwbuffersize: u32, lpbytesreturned: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Storage_InstallableFileSystems`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn FilterGetMessage(hport: super::super::Foundation::HANDLE, lpmessagebuffer: *mut FILTER_MESSAGE_HEADER, dwmessagebuffersize: u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Storage_InstallableFileSystems`*"]
    pub fn FilterInstanceClose(hinstance: HFILTER_INSTANCE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Storage_InstallableFileSystems`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FilterInstanceCreate(lpfiltername: super::super::Foundation::PWSTR, lpvolumename: super::super::Foundation::PWSTR, lpinstancename: super::super::Foundation::PWSTR, hinstance: *mut HFILTER_INSTANCE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Storage_InstallableFileSystems`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FilterInstanceFindClose(hfilterinstancefind: super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Storage_InstallableFileSystems`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FilterInstanceFindFirst(lpfiltername: super::super::Foundation::PWSTR, dwinformationclass: INSTANCE_INFORMATION_CLASS, lpbuffer: *mut ::core::ffi::c_void, dwbuffersize: u32, lpbytesreturned: *mut u32, lpfilterinstancefind: *mut FilterInstanceFindHandle) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Storage_InstallableFileSystems`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FilterInstanceFindNext(hfilterinstancefind: super::super::Foundation::HANDLE, dwinformationclass: INSTANCE_INFORMATION_CLASS, lpbuffer: *mut ::core::ffi::c_void, dwbuffersize: u32, lpbytesreturned: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Storage_InstallableFileSystems`*"]
    pub fn FilterInstanceGetInformation(hinstance: HFILTER_INSTANCE, dwinformationclass: INSTANCE_INFORMATION_CLASS, lpbuffer: *mut ::core::ffi::c_void, dwbuffersize: u32, lpbytesreturned: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Storage_InstallableFileSystems`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FilterLoad(lpfiltername: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Storage_InstallableFileSystems`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FilterReplyMessage(hport: super::super::Foundation::HANDLE, lpreplybuffer: *const FILTER_REPLY_HEADER, dwreplybuffersize: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Storage_InstallableFileSystems`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FilterSendMessage(hport: super::super::Foundation::HANDLE, lpinbuffer: *const ::core::ffi::c_void, dwinbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, dwoutbuffersize: u32, lpbytesreturned: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Storage_InstallableFileSystems`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FilterUnload(lpfiltername: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Storage_InstallableFileSystems`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FilterVolumeFindClose(hvolumefind: super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Storage_InstallableFileSystems`*"]
    pub fn FilterVolumeFindFirst(dwinformationclass: FILTER_VOLUME_INFORMATION_CLASS, lpbuffer: *mut ::core::ffi::c_void, dwbuffersize: u32, lpbytesreturned: *mut u32, lpvolumefind: *mut FilterVolumeFindHandle) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Storage_InstallableFileSystems`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FilterVolumeFindNext(hvolumefind: super::super::Foundation::HANDLE, dwinformationclass: FILTER_VOLUME_INFORMATION_CLASS, lpbuffer: *mut ::core::ffi::c_void, dwbuffersize: u32, lpbytesreturned: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Storage_InstallableFileSystems`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FilterVolumeInstanceFindClose(hvolumeinstancefind: super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Storage_InstallableFileSystems`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FilterVolumeInstanceFindFirst(lpvolumename: super::super::Foundation::PWSTR, dwinformationclass: INSTANCE_INFORMATION_CLASS, lpbuffer: *mut ::core::ffi::c_void, dwbuffersize: u32, lpbytesreturned: *mut u32, lpvolumeinstancefind: *mut FilterVolumeInstanceFindHandle) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Storage_InstallableFileSystems`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FilterVolumeInstanceFindNext(hvolumeinstancefind: super::super::Foundation::HANDLE, dwinformationclass: INSTANCE_INFORMATION_CLASS, lpbuffer: *mut ::core::ffi::c_void, dwbuffersize: u32, lpbytesreturned: *mut u32) -> ::windows::runtime::HRESULT;
}
