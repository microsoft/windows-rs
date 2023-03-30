#[doc = "*Required features: `\"Win32_NetworkManagement_WebDav\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DavAddConnection<P0, P1, P2>(connectionhandle: *mut super::super::Foundation::HANDLE, remotename: P0, username: P1, password: P2, clientcert: &[u8]) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "netapi32.dll""system" fn DavAddConnection ( connectionhandle : *mut super::super::Foundation:: HANDLE , remotename : ::windows::core::PCWSTR , username : ::windows::core::PCWSTR , password : ::windows::core::PCWSTR , clientcert : *const u8 , certsize : u32 ) -> u32 );
    DavAddConnection(connectionhandle, remotename.into_param().abi(), username.into_param().abi(), password.into_param().abi(), ::core::mem::transmute(clientcert.as_ptr()), clientcert.len() as _)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WebDav\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DavCancelConnectionsToServer<P0, P1>(lpname: P0, fforce: P1) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "davclnt.dll""system" fn DavCancelConnectionsToServer ( lpname : ::windows::core::PCWSTR , fforce : super::super::Foundation:: BOOL ) -> u32 );
    DavCancelConnectionsToServer(lpname.into_param().abi(), fforce.into_param().abi())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WebDav\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DavDeleteConnection<P0>(connectionhandle: P0) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "netapi32.dll""system" fn DavDeleteConnection ( connectionhandle : super::super::Foundation:: HANDLE ) -> u32 );
    DavDeleteConnection(connectionhandle.into_param().abi())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WebDav\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DavFlushFile<P0>(hfile: P0) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "netapi32.dll""system" fn DavFlushFile ( hfile : super::super::Foundation:: HANDLE ) -> u32 );
    DavFlushFile(hfile.into_param().abi())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WebDav\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DavGetExtendedError<P0>(hfile: P0, exterror: *mut u32, exterrorstring: ::windows::core::PWSTR, cchsize: *mut u32) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "netapi32.dll""system" fn DavGetExtendedError ( hfile : super::super::Foundation:: HANDLE , exterror : *mut u32 , exterrorstring : ::windows::core::PWSTR , cchsize : *mut u32 ) -> u32 );
    DavGetExtendedError(hfile.into_param().abi(), exterror, ::core::mem::transmute(exterrorstring), cchsize)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WebDav\"`*"]
#[inline]
pub unsafe fn DavGetHTTPFromUNCPath<P0>(uncpath: P0, url: ::windows::core::PWSTR, lpsize: *mut u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "netapi32.dll""system" fn DavGetHTTPFromUNCPath ( uncpath : ::windows::core::PCWSTR , url : ::windows::core::PWSTR , lpsize : *mut u32 ) -> u32 );
    DavGetHTTPFromUNCPath(uncpath.into_param().abi(), ::core::mem::transmute(url), lpsize)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WebDav\"`*"]
#[inline]
pub unsafe fn DavGetTheLockOwnerOfTheFile<P0>(filename: P0, lockownername: ::windows::core::PWSTR, lockownernamelengthinbytes: *mut u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "davclnt.dll""system" fn DavGetTheLockOwnerOfTheFile ( filename : ::windows::core::PCWSTR , lockownername : ::windows::core::PWSTR , lockownernamelengthinbytes : *mut u32 ) -> u32 );
    DavGetTheLockOwnerOfTheFile(filename.into_param().abi(), ::core::mem::transmute(lockownername), lockownernamelengthinbytes)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WebDav\"`*"]
#[inline]
pub unsafe fn DavGetUNCFromHTTPPath<P0>(url: P0, uncpath: ::windows::core::PWSTR, lpsize: *mut u32) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "netapi32.dll""system" fn DavGetUNCFromHTTPPath ( url : ::windows::core::PCWSTR , uncpath : ::windows::core::PWSTR , lpsize : *mut u32 ) -> u32 );
    DavGetUNCFromHTTPPath(url.into_param().abi(), ::core::mem::transmute(uncpath), lpsize)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WebDav\"`*"]
#[inline]
pub unsafe fn DavInvalidateCache<P0>(urlname: P0) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "davclnt.dll""system" fn DavInvalidateCache ( urlname : ::windows::core::PCWSTR ) -> u32 );
    DavInvalidateCache(urlname.into_param().abi())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WebDav\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DavRegisterAuthCallback(callback: PFNDAVAUTHCALLBACK, version: u32) -> u32 {
    ::windows_targets::link ! ( "davclnt.dll""system" fn DavRegisterAuthCallback ( callback : PFNDAVAUTHCALLBACK , version : u32 ) -> u32 );
    DavRegisterAuthCallback(callback, version)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WebDav\"`*"]
#[inline]
pub unsafe fn DavUnregisterAuthCallback(hcallback: u32) {
    ::windows_targets::link ! ( "davclnt.dll""system" fn DavUnregisterAuthCallback ( hcallback : u32 ) -> ( ) );
    DavUnregisterAuthCallback(hcallback)
}
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
#[doc = "*Required features: `\"Win32_NetworkManagement_WebDav\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AUTHNEXTSTEP(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WebDav\"`*"]
pub const DefaultBehavior: AUTHNEXTSTEP = AUTHNEXTSTEP(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WebDav\"`*"]
pub const RetryRequest: AUTHNEXTSTEP = AUTHNEXTSTEP(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WebDav\"`*"]
pub const CancelRequest: AUTHNEXTSTEP = AUTHNEXTSTEP(2i32);
impl ::core::marker::Copy for AUTHNEXTSTEP {}
impl ::core::clone::Clone for AUTHNEXTSTEP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AUTHNEXTSTEP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for AUTHNEXTSTEP {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for AUTHNEXTSTEP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AUTHNEXTSTEP").field(&self.0).finish()
    }
}
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
impl ::core::fmt::Debug for DAV_CALLBACK_AUTH_BLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DAV_CALLBACK_AUTH_BLOB").field("pBuffer", &self.pBuffer).field("ulSize", &self.ulSize).field("ulType", &self.ulType).finish()
    }
}
impl ::windows::core::TypeKind for DAV_CALLBACK_AUTH_BLOB {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DAV_CALLBACK_AUTH_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.pBuffer == other.pBuffer && self.ulSize == other.ulSize && self.ulType == other.ulType
    }
}
impl ::core::cmp::Eq for DAV_CALLBACK_AUTH_BLOB {}
impl ::core::default::Default for DAV_CALLBACK_AUTH_BLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WebDav\"`*"]
pub struct DAV_CALLBACK_AUTH_UNP {
    pub pszUserName: ::windows::core::PWSTR,
    pub ulUserNameLength: u32,
    pub pszPassword: ::windows::core::PWSTR,
    pub ulPasswordLength: u32,
}
impl ::core::marker::Copy for DAV_CALLBACK_AUTH_UNP {}
impl ::core::clone::Clone for DAV_CALLBACK_AUTH_UNP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DAV_CALLBACK_AUTH_UNP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DAV_CALLBACK_AUTH_UNP").field("pszUserName", &self.pszUserName).field("ulUserNameLength", &self.ulUserNameLength).field("pszPassword", &self.pszPassword).field("ulPasswordLength", &self.ulPasswordLength).finish()
    }
}
impl ::windows::core::TypeKind for DAV_CALLBACK_AUTH_UNP {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DAV_CALLBACK_AUTH_UNP {
    fn eq(&self, other: &Self) -> bool {
        self.pszUserName == other.pszUserName && self.ulUserNameLength == other.ulUserNameLength && self.pszPassword == other.pszPassword && self.ulPasswordLength == other.ulPasswordLength
    }
}
impl ::core::cmp::Eq for DAV_CALLBACK_AUTH_UNP {}
impl ::core::default::Default for DAV_CALLBACK_AUTH_UNP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DAV_CALLBACK_CRED {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DAV_CALLBACK_CRED").field("AuthBlob", &self.AuthBlob).field("UNPBlob", &self.UNPBlob).field("bAuthBlobValid", &self.bAuthBlobValid).field("bSave", &self.bSave).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DAV_CALLBACK_CRED {
    type TypeKind = ::windows::core::CopyType;
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
impl ::core::default::Default for DAV_CALLBACK_CRED {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WebDav\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFNDAVAUTHCALLBACK = ::core::option::Option<unsafe extern "system" fn(lpwzservername: ::windows::core::PCWSTR, lpwzremotename: ::windows::core::PCWSTR, dwauthscheme: u32, dwflags: u32, pcallbackcred: *mut DAV_CALLBACK_CRED, nextstep: *mut AUTHNEXTSTEP, pfreecred: *mut PFNDAVAUTHCALLBACK_FREECRED) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WebDav\"`*"]
pub type PFNDAVAUTHCALLBACK_FREECRED = ::core::option::Option<unsafe extern "system" fn(pbuffer: *const ::core::ffi::c_void) -> u32>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
