#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_NetworkManagement_WebDav`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DavAddConnection();
    #[doc = "*Required features: `Win32_NetworkManagement_WebDav`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DavCancelConnectionsToServer();
    #[doc = "*Required features: `Win32_NetworkManagement_WebDav`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DavDeleteConnection();
    #[doc = "*Required features: `Win32_NetworkManagement_WebDav`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DavFlushFile();
    #[doc = "*Required features: `Win32_NetworkManagement_WebDav`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DavGetExtendedError();
    #[doc = "*Required features: `Win32_NetworkManagement_WebDav`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DavGetHTTPFromUNCPath();
    #[doc = "*Required features: `Win32_NetworkManagement_WebDav`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DavGetTheLockOwnerOfTheFile();
    #[doc = "*Required features: `Win32_NetworkManagement_WebDav`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DavGetUNCFromHTTPPath();
    #[doc = "*Required features: `Win32_NetworkManagement_WebDav`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DavInvalidateCache();
    #[doc = "*Required features: `Win32_NetworkManagement_WebDav`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DavRegisterAuthCallback();
    #[doc = "*Required features: `Win32_NetworkManagement_WebDav`*"]
    pub fn DavUnregisterAuthCallback();
}
