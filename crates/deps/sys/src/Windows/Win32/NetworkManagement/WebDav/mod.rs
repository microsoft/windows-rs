#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn DavAddConnection(connectionhandle: *mut super::super::Foundation::HANDLE, remotename: super::super::Foundation::PWSTR, username: super::super::Foundation::PWSTR, password: super::super::Foundation::PWSTR, clientcert: *const u8, certsize: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DavCancelConnectionsToServer(lpname: super::super::Foundation::PWSTR, fforce: super::super::Foundation::BOOL) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DavDeleteConnection(connectionhandle: super::super::Foundation::HANDLE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DavFlushFile(hfile: super::super::Foundation::HANDLE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DavGetExtendedError(hfile: super::super::Foundation::HANDLE, exterror: *mut u32, exterrorstring: super::super::Foundation::PWSTR, cchsize: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DavGetHTTPFromUNCPath(uncpath: super::super::Foundation::PWSTR, url: super::super::Foundation::PWSTR, lpsize: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DavGetTheLockOwnerOfTheFile(filename: super::super::Foundation::PWSTR, lockownername: super::super::Foundation::PWSTR, lockownernamelengthinbytes: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DavGetUNCFromHTTPPath(url: super::super::Foundation::PWSTR, uncpath: super::super::Foundation::PWSTR, lpsize: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DavInvalidateCache(urlname: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DavRegisterAuthCallback(callback: ::core::option::Option<PFNDAVAUTHCALLBACK>, version: u32) -> u32;
    pub fn DavUnregisterAuthCallback(hcallback: u32);
}
pub type AUTHNEXTSTEP = i32;
pub const DefaultBehavior: AUTHNEXTSTEP = 0i32;
pub const RetryRequest: AUTHNEXTSTEP = 1i32;
pub const CancelRequest: AUTHNEXTSTEP = 2i32;
pub const DAV_AUTHN_SCHEME_BASIC: u32 = 1u32;
pub const DAV_AUTHN_SCHEME_CERT: u32 = 65536u32;
pub const DAV_AUTHN_SCHEME_DIGEST: u32 = 8u32;
pub const DAV_AUTHN_SCHEME_FBA: u32 = 1048576u32;
pub const DAV_AUTHN_SCHEME_NEGOTIATE: u32 = 16u32;
pub const DAV_AUTHN_SCHEME_NTLM: u32 = 2u32;
pub const DAV_AUTHN_SCHEME_PASSPORT: u32 = 4u32;
#[repr(C)]
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
#[cfg(feature = "Win32_Foundation")]
pub struct DAV_CALLBACK_AUTH_UNP {
    pub pszUserName: super::super::Foundation::PWSTR,
    pub ulUserNameLength: u32,
    pub pszPassword: super::super::Foundation::PWSTR,
    pub ulPasswordLength: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DAV_CALLBACK_AUTH_UNP {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DAV_CALLBACK_AUTH_UNP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
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
#[cfg(feature = "Win32_Foundation")]
pub type PFNDAVAUTHCALLBACK = unsafe extern "system" fn(lpwzservername: super::super::Foundation::PWSTR, lpwzremotename: super::super::Foundation::PWSTR, dwauthscheme: u32, dwflags: u32, pcallbackcred: *mut DAV_CALLBACK_CRED, nextstep: *mut AUTHNEXTSTEP, pfreecred: *mut ::core::option::Option<PFNDAVAUTHCALLBACK_FREECRED>) -> u32;
pub type PFNDAVAUTHCALLBACK_FREECRED = unsafe extern "system" fn(pbuffer: *const ::core::ffi::c_void) -> u32;
