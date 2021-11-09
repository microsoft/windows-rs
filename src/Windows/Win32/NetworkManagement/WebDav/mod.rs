#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Win32_NetworkManagement_WebDav`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AUTHNEXTSTEP(pub i32);
pub const DefaultBehavior: AUTHNEXTSTEP = AUTHNEXTSTEP(0i32);
pub const RetryRequest: AUTHNEXTSTEP = AUTHNEXTSTEP(1i32);
pub const CancelRequest: AUTHNEXTSTEP = AUTHNEXTSTEP(2i32);
impl ::core::convert::From<i32> for AUTHNEXTSTEP {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AUTHNEXTSTEP {
    type Abi = Self;
}
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
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_WebDav`*"]
pub struct DAV_CALLBACK_AUTH_BLOB {
    pub pBuffer: *mut ::core::ffi::c_void,
    pub ulSize: u32,
    pub ulType: u32,
}
impl DAV_CALLBACK_AUTH_BLOB {}
impl ::core::default::Default for DAV_CALLBACK_AUTH_BLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DAV_CALLBACK_AUTH_BLOB {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DAV_CALLBACK_AUTH_BLOB").field("pBuffer", &self.pBuffer).field("ulSize", &self.ulSize).field("ulType", &self.ulType).finish()
    }
}
impl ::core::cmp::PartialEq for DAV_CALLBACK_AUTH_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.pBuffer == other.pBuffer && self.ulSize == other.ulSize && self.ulType == other.ulType
    }
}
impl ::core::cmp::Eq for DAV_CALLBACK_AUTH_BLOB {}
unsafe impl ::windows::runtime::Abi for DAV_CALLBACK_AUTH_BLOB {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_NetworkManagement_WebDav`, `Win32_Foundation`*"]
pub struct DAV_CALLBACK_AUTH_UNP {
    pub pszUserName: super::super::Foundation::PWSTR,
    pub ulUserNameLength: u32,
    pub pszPassword: super::super::Foundation::PWSTR,
    pub ulPasswordLength: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl DAV_CALLBACK_AUTH_UNP {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DAV_CALLBACK_AUTH_UNP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DAV_CALLBACK_AUTH_UNP {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DAV_CALLBACK_AUTH_UNP").field("pszUserName", &self.pszUserName).field("ulUserNameLength", &self.ulUserNameLength).field("pszPassword", &self.pszPassword).field("ulPasswordLength", &self.ulPasswordLength).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DAV_CALLBACK_AUTH_UNP {
    fn eq(&self, other: &Self) -> bool {
        self.pszUserName == other.pszUserName && self.ulUserNameLength == other.ulUserNameLength && self.pszPassword == other.pszPassword && self.ulPasswordLength == other.ulPasswordLength
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DAV_CALLBACK_AUTH_UNP {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DAV_CALLBACK_AUTH_UNP {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_NetworkManagement_WebDav`, `Win32_Foundation`*"]
pub struct DAV_CALLBACK_CRED {
    pub AuthBlob: DAV_CALLBACK_AUTH_BLOB,
    pub UNPBlob: DAV_CALLBACK_AUTH_UNP,
    pub bAuthBlobValid: super::super::Foundation::BOOL,
    pub bSave: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl DAV_CALLBACK_CRED {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DAV_CALLBACK_CRED {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DAV_CALLBACK_CRED {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DAV_CALLBACK_CRED").field("AuthBlob", &self.AuthBlob).field("UNPBlob", &self.UNPBlob).field("bAuthBlobValid", &self.bAuthBlobValid).field("bSave", &self.bSave).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DAV_CALLBACK_CRED {
    fn eq(&self, other: &Self) -> bool {
        self.AuthBlob == other.AuthBlob && self.UNPBlob == other.UNPBlob && self.bAuthBlobValid == other.bAuthBlobValid && self.bSave == other.bSave
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DAV_CALLBACK_CRED {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DAV_CALLBACK_CRED {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_WebDav`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DavAddConnection<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(connectionhandle: *mut super::super::Foundation::HANDLE, remotename: Param1, username: Param2, password: Param3, clientcert: *const u8, certsize: u32) -> u32 {
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
#[doc = "*Required features: `Win32_NetworkManagement_WebDav`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DavCancelConnectionsToServer<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(lpname: Param0, fforce: Param1) -> u32 {
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
#[doc = "*Required features: `Win32_NetworkManagement_WebDav`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DavDeleteConnection<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(connectionhandle: Param0) -> u32 {
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
#[doc = "*Required features: `Win32_NetworkManagement_WebDav`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DavFlushFile<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hfile: Param0) -> u32 {
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
#[doc = "*Required features: `Win32_NetworkManagement_WebDav`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DavGetExtendedError<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hfile: Param0, exterror: *mut u32, exterrorstring: super::super::Foundation::PWSTR, cchsize: *mut u32) -> u32 {
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
#[doc = "*Required features: `Win32_NetworkManagement_WebDav`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DavGetHTTPFromUNCPath<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(uncpath: Param0, url: super::super::Foundation::PWSTR, lpsize: *mut u32) -> u32 {
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
#[doc = "*Required features: `Win32_NetworkManagement_WebDav`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DavGetTheLockOwnerOfTheFile<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(filename: Param0, lockownername: super::super::Foundation::PWSTR, lockownernamelengthinbytes: *mut u32) -> u32 {
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
#[doc = "*Required features: `Win32_NetworkManagement_WebDav`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DavGetUNCFromHTTPPath<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(url: Param0, uncpath: super::super::Foundation::PWSTR, lpsize: *mut u32) -> u32 {
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
#[doc = "*Required features: `Win32_NetworkManagement_WebDav`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DavInvalidateCache<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(urlname: Param0) -> u32 {
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
#[doc = "*Required features: `Win32_NetworkManagement_WebDav`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DavRegisterAuthCallback(callback: ::core::option::Option<PFNDAVAUTHCALLBACK>, version: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DavRegisterAuthCallback(callback: ::windows::runtime::RawPtr, version: u32) -> u32;
        }
        ::core::mem::transmute(DavRegisterAuthCallback(::core::mem::transmute(callback), ::core::mem::transmute(version)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_NetworkManagement_WebDav`*"]
#[inline]
pub unsafe fn DavUnregisterAuthCallback(hcallback: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DavUnregisterAuthCallback(hcallback: u32);
        }
        ::core::mem::transmute(DavUnregisterAuthCallback(::core::mem::transmute(hcallback)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_NetworkManagement_WebDav`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFNDAVAUTHCALLBACK = unsafe extern "system" fn(lpwzservername: super::super::Foundation::PWSTR, lpwzremotename: super::super::Foundation::PWSTR, dwauthscheme: u32, dwflags: u32, pcallbackcred: *mut DAV_CALLBACK_CRED, nextstep: *mut AUTHNEXTSTEP, pfreecred: *mut ::windows::runtime::RawPtr) -> u32;
#[doc = "*Required features: `Win32_NetworkManagement_WebDav`*"]
pub type PFNDAVAUTHCALLBACK_FREECRED = unsafe extern "system" fn(pbuffer: *const ::core::ffi::c_void) -> u32;
