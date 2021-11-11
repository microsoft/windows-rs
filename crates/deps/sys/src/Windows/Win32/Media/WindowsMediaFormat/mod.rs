#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub fn WMCreateBackupRestorer();
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub fn WMCreateEditor();
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub fn WMCreateIndexer();
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub fn WMCreateProfileManager();
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub fn WMCreateReader();
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub fn WMCreateSyncReader();
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub fn WMCreateWriter();
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub fn WMCreateWriterFileSink();
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub fn WMCreateWriterNetworkSink();
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`*"]
    pub fn WMCreateWriterPushSink();
    #[doc = "*Required features: `Win32_Media_WindowsMediaFormat`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WMIsContentProtected();
}
