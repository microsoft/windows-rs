#![allow(non_snake_case, non_camel_case_types)]
#[doc = "*Required features: `Win32_NetworkManagement_WebDav`*"]
pub const DAV_AUTHN_SCHEME_BASIC: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_WebDav`*"]
pub const DAV_AUTHN_SCHEME_CERT: u32 = 65536u32;
#[doc = "*Required features: `Win32_NetworkManagement_WebDav`*"]
pub const DAV_AUTHN_SCHEME_DIGEST: u32 = 8u32;
#[doc = "*Required features: `Win32_NetworkManagement_WebDav`*"]
pub const DAV_AUTHN_SCHEME_FBA: u32 = 1048576u32;
#[doc = "*Required features: `Win32_NetworkManagement_WebDav`*"]
pub const DAV_AUTHN_SCHEME_NEGOTIATE: u32 = 16u32;
#[doc = "*Required features: `Win32_NetworkManagement_WebDav`*"]
pub const DAV_AUTHN_SCHEME_NTLM: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_WebDav`*"]
pub const DAV_AUTHN_SCHEME_PASSPORT: u32 = 4u32;
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_NetworkManagement_WebDav`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DavAddConnection(connectionhandle: *mut super::super::Foundation::HANDLE, remotename: super::super::Foundation::PWSTR, username: super::super::Foundation::PWSTR, password: super::super::Foundation::PWSTR, clientcert: *const u8, certsize: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WebDav`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DavCancelConnectionsToServer(lpname: super::super::Foundation::PWSTR, fforce: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WebDav`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DavDeleteConnection(connectionhandle: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WebDav`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DavFlushFile(hfile: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WebDav`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DavGetExtendedError(hfile: super::super::Foundation::HANDLE, exterror: *mut u32, exterrorstring: super::super::Foundation::PWSTR, cchsize: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WebDav`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DavGetHTTPFromUNCPath(uncpath: super::super::Foundation::PWSTR, url: super::super::Foundation::PWSTR, lpsize: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WebDav`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DavGetTheLockOwnerOfTheFile(filename: super::super::Foundation::PWSTR, lockownername: super::super::Foundation::PWSTR, lockownernamelengthinbytes: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WebDav`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DavGetUNCFromHTTPPath(url: super::super::Foundation::PWSTR, uncpath: super::super::Foundation::PWSTR, lpsize: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WebDav`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DavInvalidateCache(urlname: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WebDav`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DavRegisterAuthCallback(callback: PFNDAVAUTHCALLBACK, version: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WebDav`*"]
    pub fn DavUnregisterAuthCallback(hcallback: u32);
}
