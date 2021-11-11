#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub fn WMCreateBackupRestorer(pcallback: ::windows::runtime::RawPtr, ppbackup: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub fn WMCreateEditor(ppeditor: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub fn WMCreateIndexer(ppindexer: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub fn WMCreateProfileManager(ppprofilemanager: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub fn WMCreateReader(punkcert: ::windows::runtime::RawPtr, dwrights: u32, ppreader: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub fn WMCreateSyncReader(punkcert: ::windows::runtime::RawPtr, dwrights: u32, ppsyncreader: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub fn WMCreateWriter(punkcert: ::windows::runtime::RawPtr, ppwriter: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub fn WMCreateWriterFileSink(ppsink: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub fn WMCreateWriterNetworkSink(ppsink: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub fn WMCreateWriterPushSink(ppsink: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WMIsContentProtected(pwszfilename: super::super::Foundation::PWSTR, pfisprotected: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
}
