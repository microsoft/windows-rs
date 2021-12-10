#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
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
unsafe impl ::windows::core::Abi for DAV_CALLBACK_AUTH_BLOB {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DAV_CALLBACK_AUTH_BLOB {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DAV_CALLBACK_AUTH_BLOB>()) == 0 }
    }
}
impl ::core::cmp::Eq for DAV_CALLBACK_AUTH_BLOB {}
impl ::core::default::Default for DAV_CALLBACK_AUTH_BLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DAV_CALLBACK_AUTH_UNP {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DAV_CALLBACK_AUTH_UNP {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DAV_CALLBACK_AUTH_UNP>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DAV_CALLBACK_AUTH_UNP {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DAV_CALLBACK_AUTH_UNP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
unsafe impl ::windows::core::Abi for DAV_CALLBACK_CRED {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DAV_CALLBACK_CRED {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DAV_CALLBACK_CRED>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DAV_CALLBACK_CRED {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DAV_CALLBACK_CRED {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DavAddConnection<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(connectionhandle: *mut super::super::Foundation::HANDLE, remotename: Param1, username: Param2, password: Param3, clientcert: *const u8, certsize: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DavAddConnection(connectionhandle: *mut super::super::Foundation::HANDLE, remotename: super::super::Foundation::PWSTR, username: super::super::Foundation::PWSTR, password: super::super::Foundation::PWSTR, clientcert: *const u8, certsize: u32) -> u32;
        }
        ::core::mem::transmute(DavAddConnection(::core::mem::transmute(connectionhandle), remotename.into_param().abi(), username.into_param().abi(), password.into_param().abi(), ::core::mem::transmute(clientcert), ::core::mem::transmute(certsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DavCancelConnectionsToServer<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(lpname: Param0, fforce: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DavCancelConnectionsToServer(lpname: super::super::Foundation::PWSTR, fforce: super::super::Foundation::BOOL) -> u32;
        }
        ::core::mem::transmute(DavCancelConnectionsToServer(lpname.into_param().abi(), fforce.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DavDeleteConnection<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(connectionhandle: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DavDeleteConnection(connectionhandle: super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(DavDeleteConnection(connectionhandle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DavFlushFile<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hfile: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DavFlushFile(hfile: super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(DavFlushFile(hfile.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DavGetExtendedError<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hfile: Param0, exterror: *mut u32, exterrorstring: super::super::Foundation::PWSTR, cchsize: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DavGetExtendedError(hfile: super::super::Foundation::HANDLE, exterror: *mut u32, exterrorstring: super::super::Foundation::PWSTR, cchsize: *mut u32) -> u32;
        }
        ::core::mem::transmute(DavGetExtendedError(hfile.into_param().abi(), ::core::mem::transmute(exterror), ::core::mem::transmute(exterrorstring), ::core::mem::transmute(cchsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DavGetHTTPFromUNCPath<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(uncpath: Param0, url: super::super::Foundation::PWSTR, lpsize: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DavGetHTTPFromUNCPath(uncpath: super::super::Foundation::PWSTR, url: super::super::Foundation::PWSTR, lpsize: *mut u32) -> u32;
        }
        ::core::mem::transmute(DavGetHTTPFromUNCPath(uncpath.into_param().abi(), ::core::mem::transmute(url), ::core::mem::transmute(lpsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DavGetTheLockOwnerOfTheFile<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(filename: Param0, lockownername: super::super::Foundation::PWSTR, lockownernamelengthinbytes: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DavGetTheLockOwnerOfTheFile(filename: super::super::Foundation::PWSTR, lockownername: super::super::Foundation::PWSTR, lockownernamelengthinbytes: *mut u32) -> u32;
        }
        ::core::mem::transmute(DavGetTheLockOwnerOfTheFile(filename.into_param().abi(), ::core::mem::transmute(lockownername), ::core::mem::transmute(lockownernamelengthinbytes)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DavGetUNCFromHTTPPath<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(url: Param0, uncpath: super::super::Foundation::PWSTR, lpsize: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DavGetUNCFromHTTPPath(url: super::super::Foundation::PWSTR, uncpath: super::super::Foundation::PWSTR, lpsize: *mut u32) -> u32;
        }
        ::core::mem::transmute(DavGetUNCFromHTTPPath(url.into_param().abi(), ::core::mem::transmute(uncpath), ::core::mem::transmute(lpsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DavInvalidateCache<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(urlname: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DavInvalidateCache(urlname: super::super::Foundation::PWSTR) -> u32;
        }
        ::core::mem::transmute(DavInvalidateCache(urlname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DavRegisterAuthCallback(callback: PFNDAVAUTHCALLBACK, version: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DavRegisterAuthCallback(callback: ::windows::core::RawPtr, version: u32) -> u32;
        }
        ::core::mem::transmute(DavRegisterAuthCallback(::core::mem::transmute(callback), ::core::mem::transmute(version)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DavUnregisterAuthCallback(hcallback: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DavUnregisterAuthCallback(hcallback: u32);
        }
        DavUnregisterAuthCallback(::core::mem::transmute(hcallback))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub type PFNDAVAUTHCALLBACK = ::core::option::Option<unsafe extern "system" fn(lpwzservername: super::super::Foundation::PWSTR, lpwzremotename: super::super::Foundation::PWSTR, dwauthscheme: u32, dwflags: u32, pcallbackcred: *mut DAV_CALLBACK_CRED, nextstep: *mut AUTHNEXTSTEP, pfreecred: *mut PFNDAVAUTHCALLBACK_FREECRED) -> u32>;
pub type PFNDAVAUTHCALLBACK_FREECRED = ::core::option::Option<unsafe extern "system" fn(pbuffer: *const ::core::ffi::c_void) -> u32>;
