#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `\"Win32_NetworkManagement_WebDav\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DavAddConnection(connectionhandle: *mut super::super::Foundation::HANDLE, remotename: ::windows_sys::core::PCWSTR, username: ::windows_sys::core::PCWSTR, password: ::windows_sys::core::PCWSTR, clientcert: *const u8, certsize: u32) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_WebDav\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DavCancelConnectionsToServer(lpname: ::windows_sys::core::PCWSTR, fforce: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_WebDav\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DavDeleteConnection(connectionhandle: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_WebDav\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DavFlushFile(hfile: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_WebDav\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DavGetExtendedError(hfile: super::super::Foundation::HANDLE, exterror: *mut u32, exterrorstring: ::windows_sys::core::PWSTR, cchsize: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_WebDav\"`*"]
    pub fn DavGetHTTPFromUNCPath(uncpath: ::windows_sys::core::PCWSTR, url: ::windows_sys::core::PWSTR, lpsize: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_WebDav\"`*"]
    pub fn DavGetTheLockOwnerOfTheFile(filename: ::windows_sys::core::PCWSTR, lockownername: ::windows_sys::core::PWSTR, lockownernamelengthinbytes: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_WebDav\"`*"]
    pub fn DavGetUNCFromHTTPPath(url: ::windows_sys::core::PCWSTR, uncpath: ::windows_sys::core::PWSTR, lpsize: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_WebDav\"`*"]
    pub fn DavInvalidateCache(urlname: ::windows_sys::core::PCWSTR) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_WebDav\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DavRegisterAuthCallback(callback: PFNDAVAUTHCALLBACK, version: u32) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_WebDav\"`*"]
    pub fn DavUnregisterAuthCallback(hcallback: u32);
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WebDav\"`*"]
pub type AUTHNEXTSTEP = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WebDav\"`*"]
pub const DefaultBehavior: AUTHNEXTSTEP = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WebDav\"`*"]
pub const RetryRequest: AUTHNEXTSTEP = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WebDav\"`*"]
pub const CancelRequest: AUTHNEXTSTEP = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WebDav\"`*"]
pub const DAV_AUTHN_SCHEME_BASIC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WebDav\"`*"]
pub const DAV_AUTHN_SCHEME_CERT: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WebDav\"`*"]
pub const DAV_AUTHN_SCHEME_DIGEST: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WebDav\"`*"]
pub const DAV_AUTHN_SCHEME_FBA: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WebDav\"`*"]
pub const DAV_AUTHN_SCHEME_NEGOTIATE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WebDav\"`*"]
pub const DAV_AUTHN_SCHEME_NTLM: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WebDav\"`*"]
pub const DAV_AUTHN_SCHEME_PASSPORT: u32 = 4u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WebDav\"`*"]
pub struct DAV_CALLBACK_AUTH_BLOB {
    pub pBuffer: *mut ::core::ffi::c_void,
    pub ulSize: u32,
    pub ulType: u32,
}
impl ::core::marker::Copy for DAV_CALLBACK_AUTH_BLOB {}
impl ::core::clone::Clone for DAV_CALLBACK_AUTH_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WebDav\"`*"]
pub struct DAV_CALLBACK_AUTH_UNP {
    pub pszUserName: ::windows_sys::core::PWSTR,
    pub ulUserNameLength: u32,
    pub pszPassword: ::windows_sys::core::PWSTR,
    pub ulPasswordLength: u32,
}
impl ::core::marker::Copy for DAV_CALLBACK_AUTH_UNP {}
impl ::core::clone::Clone for DAV_CALLBACK_AUTH_UNP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WebDav\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DAV_CALLBACK_CRED {
    pub AuthBlob: DAV_CALLBACK_AUTH_BLOB,
    pub UNPBlob: DAV_CALLBACK_AUTH_UNP,
    pub bAuthBlobValid: super::super::Foundation::BOOL,
    pub bSave: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DAV_CALLBACK_CRED {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DAV_CALLBACK_CRED {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WebDav\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFNDAVAUTHCALLBACK = ::core::option::Option<unsafe extern "system" fn(lpwzservername: ::windows_sys::core::PCWSTR, lpwzremotename: ::windows_sys::core::PCWSTR, dwauthscheme: u32, dwflags: u32, pcallbackcred: *mut DAV_CALLBACK_CRED, nextstep: *mut AUTHNEXTSTEP, pfreecred: *mut PFNDAVAUTHCALLBACK_FREECRED) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WebDav\"`*"]
pub type PFNDAVAUTHCALLBACK_FREECRED = ::core::option::Option<unsafe extern "system" fn(pbuffer: *const ::core::ffi::c_void) -> u32>;
