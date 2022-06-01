#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const ADMINDATA_MAX_NAME_LEN: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const APPCTR_MD_ID_BEGIN_RESERVED: u32 = 57344u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const APPCTR_MD_ID_END_RESERVED: u32 = 61439u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const APPSTATUS_NOTDEFINED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const APPSTATUS_RUNNING: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const APPSTATUS_STOPPED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const ASP_MD_ID_BEGIN_RESERVED: u32 = 28672u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const ASP_MD_ID_END_RESERVED: u32 = 29951u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const ASP_MD_SERVER_BASE: u32 = 7000u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const ASP_MD_UT_APP: u32 = 101u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
#[repr(transparent)]
pub struct AsyncIFtpAuthenticationProvider(::windows::core::IUnknown);
impl AsyncIFtpAuthenticationProvider {
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn Begin_AuthenticateUser<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pszsessionid: Param0, pszsitename: Param1, pszusername: Param2, pszpassword: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Begin_AuthenticateUser)(::windows::core::Interface::as_raw(self), pszsessionid.into_param().abi(), pszsitename.into_param().abi(), pszusername.into_param().abi(), pszpassword.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Finish_AuthenticateUser(&self, ppszcanonicalusername: *mut ::windows::core::PWSTR, pfauthenticated: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Finish_AuthenticateUser)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppszcanonicalusername), ::core::mem::transmute(pfauthenticated)).ok()
    }
}
impl ::core::convert::From<AsyncIFtpAuthenticationProvider> for ::windows::core::IUnknown {
    fn from(value: AsyncIFtpAuthenticationProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AsyncIFtpAuthenticationProvider> for ::windows::core::IUnknown {
    fn from(value: &AsyncIFtpAuthenticationProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AsyncIFtpAuthenticationProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AsyncIFtpAuthenticationProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for AsyncIFtpAuthenticationProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AsyncIFtpAuthenticationProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AsyncIFtpAuthenticationProvider {}
impl ::core::fmt::Debug for AsyncIFtpAuthenticationProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AsyncIFtpAuthenticationProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for AsyncIFtpAuthenticationProvider {
    type Vtable = AsyncIFtpAuthenticationProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc24efb65_9f3e_4996_8fb1_ce166916bab5);
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIFtpAuthenticationProvider_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Begin_AuthenticateUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszsessionid: ::windows::core::PCWSTR, pszsitename: ::windows::core::PCWSTR, pszusername: ::windows::core::PCWSTR, pszpassword: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Finish_AuthenticateUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszcanonicalusername: *mut ::windows::core::PWSTR, pfauthenticated: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Finish_AuthenticateUser: usize,
}
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
#[repr(transparent)]
pub struct AsyncIFtpAuthorizationProvider(::windows::core::IUnknown);
impl AsyncIFtpAuthorizationProvider {
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn Begin_GetUserAccessPermission<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pszsessionid: Param0, pszsitename: Param1, pszvirtualpath: Param2, pszusername: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Begin_GetUserAccessPermission)(::windows::core::Interface::as_raw(self), pszsessionid.into_param().abi(), pszsitename.into_param().abi(), pszvirtualpath.into_param().abi(), pszusername.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn Finish_GetUserAccessPermission(&self) -> ::windows::core::Result<FTP_ACCESS> {
        let mut result__ = ::core::mem::MaybeUninit::<FTP_ACCESS>::zeroed();
        (::windows::core::Interface::vtable(self).Finish_GetUserAccessPermission)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<FTP_ACCESS>(result__)
    }
}
impl ::core::convert::From<AsyncIFtpAuthorizationProvider> for ::windows::core::IUnknown {
    fn from(value: AsyncIFtpAuthorizationProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AsyncIFtpAuthorizationProvider> for ::windows::core::IUnknown {
    fn from(value: &AsyncIFtpAuthorizationProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AsyncIFtpAuthorizationProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AsyncIFtpAuthorizationProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for AsyncIFtpAuthorizationProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AsyncIFtpAuthorizationProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AsyncIFtpAuthorizationProvider {}
impl ::core::fmt::Debug for AsyncIFtpAuthorizationProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AsyncIFtpAuthorizationProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for AsyncIFtpAuthorizationProvider {
    type Vtable = AsyncIFtpAuthorizationProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x860dc339_07e5_4a5c_9c61_8820cea012bc);
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIFtpAuthorizationProvider_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Begin_GetUserAccessPermission: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszsessionid: ::windows::core::PCWSTR, pszsitename: ::windows::core::PCWSTR, pszvirtualpath: ::windows::core::PCWSTR, pszusername: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub Finish_GetUserAccessPermission: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pftpaccess: *mut FTP_ACCESS) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
#[repr(transparent)]
pub struct AsyncIFtpHomeDirectoryProvider(::windows::core::IUnknown);
impl AsyncIFtpHomeDirectoryProvider {
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn Begin_GetUserHomeDirectoryData<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pszsessionid: Param0, pszsitename: Param1, pszusername: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Begin_GetUserHomeDirectoryData)(::windows::core::Interface::as_raw(self), pszsessionid.into_param().abi(), pszsitename.into_param().abi(), pszusername.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn Finish_GetUserHomeDirectoryData(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows::core::PWSTR>::zeroed();
        (::windows::core::Interface::vtable(self).Finish_GetUserHomeDirectoryData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
}
impl ::core::convert::From<AsyncIFtpHomeDirectoryProvider> for ::windows::core::IUnknown {
    fn from(value: AsyncIFtpHomeDirectoryProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AsyncIFtpHomeDirectoryProvider> for ::windows::core::IUnknown {
    fn from(value: &AsyncIFtpHomeDirectoryProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AsyncIFtpHomeDirectoryProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AsyncIFtpHomeDirectoryProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for AsyncIFtpHomeDirectoryProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AsyncIFtpHomeDirectoryProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AsyncIFtpHomeDirectoryProvider {}
impl ::core::fmt::Debug for AsyncIFtpHomeDirectoryProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AsyncIFtpHomeDirectoryProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for AsyncIFtpHomeDirectoryProvider {
    type Vtable = AsyncIFtpHomeDirectoryProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x73f81638_6295_42bd_a2be_4a657f7c479c);
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIFtpHomeDirectoryProvider_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Begin_GetUserHomeDirectoryData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszsessionid: ::windows::core::PCWSTR, pszsitename: ::windows::core::PCWSTR, pszusername: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub Finish_GetUserHomeDirectoryData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszhomedirectorydata: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
#[repr(transparent)]
pub struct AsyncIFtpLogProvider(::windows::core::IUnknown);
impl AsyncIFtpLogProvider {
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn Begin_Log(&self, ploggingparameters: *const LOGGING_PARAMETERS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Begin_Log)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ploggingparameters)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn Finish_Log(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Finish_Log)(::windows::core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<AsyncIFtpLogProvider> for ::windows::core::IUnknown {
    fn from(value: AsyncIFtpLogProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AsyncIFtpLogProvider> for ::windows::core::IUnknown {
    fn from(value: &AsyncIFtpLogProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AsyncIFtpLogProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AsyncIFtpLogProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for AsyncIFtpLogProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AsyncIFtpLogProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AsyncIFtpLogProvider {}
impl ::core::fmt::Debug for AsyncIFtpLogProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AsyncIFtpLogProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for AsyncIFtpLogProvider {
    type Vtable = AsyncIFtpLogProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00a0ae46_2498_48b2_95e6_df678ed7d49f);
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIFtpLogProvider_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Begin_Log: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ploggingparameters: *const LOGGING_PARAMETERS) -> ::windows::core::HRESULT,
    pub Finish_Log: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
#[repr(transparent)]
pub struct AsyncIFtpPostprocessProvider(::windows::core::IUnknown);
impl AsyncIFtpPostprocessProvider {
    #[doc = "*Required features: `\"Win32_System_Iis\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Begin_HandlePostprocess(&self, ppostprocessparameters: *const POST_PROCESS_PARAMETERS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Begin_HandlePostprocess)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppostprocessparameters)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn Finish_HandlePostprocess(&self) -> ::windows::core::Result<FTP_PROCESS_STATUS> {
        let mut result__ = ::core::mem::MaybeUninit::<FTP_PROCESS_STATUS>::zeroed();
        (::windows::core::Interface::vtable(self).Finish_HandlePostprocess)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<FTP_PROCESS_STATUS>(result__)
    }
}
impl ::core::convert::From<AsyncIFtpPostprocessProvider> for ::windows::core::IUnknown {
    fn from(value: AsyncIFtpPostprocessProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AsyncIFtpPostprocessProvider> for ::windows::core::IUnknown {
    fn from(value: &AsyncIFtpPostprocessProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AsyncIFtpPostprocessProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AsyncIFtpPostprocessProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for AsyncIFtpPostprocessProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AsyncIFtpPostprocessProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AsyncIFtpPostprocessProvider {}
impl ::core::fmt::Debug for AsyncIFtpPostprocessProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AsyncIFtpPostprocessProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for AsyncIFtpPostprocessProvider {
    type Vtable = AsyncIFtpPostprocessProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa16b2542_9694_4eb1_a564_6c2e91fdc133);
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIFtpPostprocessProvider_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Begin_HandlePostprocess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppostprocessparameters: *const POST_PROCESS_PARAMETERS) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Begin_HandlePostprocess: usize,
    pub Finish_HandlePostprocess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pftpprocessstatus: *mut FTP_PROCESS_STATUS) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
#[repr(transparent)]
pub struct AsyncIFtpPreprocessProvider(::windows::core::IUnknown);
impl AsyncIFtpPreprocessProvider {
    #[doc = "*Required features: `\"Win32_System_Iis\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Begin_HandlePreprocess(&self, ppreprocessparameters: *const PRE_PROCESS_PARAMETERS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Begin_HandlePreprocess)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppreprocessparameters)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn Finish_HandlePreprocess(&self) -> ::windows::core::Result<FTP_PROCESS_STATUS> {
        let mut result__ = ::core::mem::MaybeUninit::<FTP_PROCESS_STATUS>::zeroed();
        (::windows::core::Interface::vtable(self).Finish_HandlePreprocess)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<FTP_PROCESS_STATUS>(result__)
    }
}
impl ::core::convert::From<AsyncIFtpPreprocessProvider> for ::windows::core::IUnknown {
    fn from(value: AsyncIFtpPreprocessProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AsyncIFtpPreprocessProvider> for ::windows::core::IUnknown {
    fn from(value: &AsyncIFtpPreprocessProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AsyncIFtpPreprocessProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AsyncIFtpPreprocessProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for AsyncIFtpPreprocessProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AsyncIFtpPreprocessProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AsyncIFtpPreprocessProvider {}
impl ::core::fmt::Debug for AsyncIFtpPreprocessProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AsyncIFtpPreprocessProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for AsyncIFtpPreprocessProvider {
    type Vtable = AsyncIFtpPreprocessProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6ff5fd8f_fd8e_48b1_a3e0_bf7073db4db5);
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIFtpPreprocessProvider_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Begin_HandlePreprocess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppreprocessparameters: *const PRE_PROCESS_PARAMETERS) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Begin_HandlePreprocess: usize,
    pub Finish_HandlePreprocess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pftpprocessstatus: *mut FTP_PROCESS_STATUS) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
#[repr(transparent)]
pub struct AsyncIFtpRoleProvider(::windows::core::IUnknown);
impl AsyncIFtpRoleProvider {
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn Begin_IsUserInRole<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pszsessionid: Param0, pszsitename: Param1, pszusername: Param2, pszrole: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Begin_IsUserInRole)(::windows::core::Interface::as_raw(self), pszsessionid.into_param().abi(), pszsitename.into_param().abi(), pszusername.into_param().abi(), pszrole.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Finish_IsUserInRole(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::BOOL>::zeroed();
        (::windows::core::Interface::vtable(self).Finish_IsUserInRole)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
impl ::core::convert::From<AsyncIFtpRoleProvider> for ::windows::core::IUnknown {
    fn from(value: AsyncIFtpRoleProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AsyncIFtpRoleProvider> for ::windows::core::IUnknown {
    fn from(value: &AsyncIFtpRoleProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AsyncIFtpRoleProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AsyncIFtpRoleProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for AsyncIFtpRoleProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AsyncIFtpRoleProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AsyncIFtpRoleProvider {}
impl ::core::fmt::Debug for AsyncIFtpRoleProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AsyncIFtpRoleProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for AsyncIFtpRoleProvider {
    type Vtable = AsyncIFtpRoleProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3e83bf99_70ec_41ca_84b6_aca7c7a62caf);
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIFtpRoleProvider_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Begin_IsUserInRole: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszsessionid: ::windows::core::PCWSTR, pszsitename: ::windows::core::PCWSTR, pszusername: ::windows::core::PCWSTR, pszrole: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Finish_IsUserInRole: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfisinrole: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Finish_IsUserInRole: usize,
}
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
#[repr(transparent)]
pub struct AsyncIMSAdminBaseSinkW(::windows::core::IUnknown);
impl AsyncIMSAdminBaseSinkW {
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn Begin_SinkNotify(&self, pcochangelist: &[MD_CHANGE_OBJECT_W]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Begin_SinkNotify)(::windows::core::Interface::as_raw(self), pcochangelist.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(pcochangelist))).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn Finish_SinkNotify(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Finish_SinkNotify)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn Begin_ShutdownNotify(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Begin_ShutdownNotify)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn Finish_ShutdownNotify(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Finish_ShutdownNotify)(::windows::core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<AsyncIMSAdminBaseSinkW> for ::windows::core::IUnknown {
    fn from(value: AsyncIMSAdminBaseSinkW) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AsyncIMSAdminBaseSinkW> for ::windows::core::IUnknown {
    fn from(value: &AsyncIMSAdminBaseSinkW) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AsyncIMSAdminBaseSinkW {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AsyncIMSAdminBaseSinkW {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for AsyncIMSAdminBaseSinkW {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AsyncIMSAdminBaseSinkW {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AsyncIMSAdminBaseSinkW {}
impl ::core::fmt::Debug for AsyncIMSAdminBaseSinkW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AsyncIMSAdminBaseSinkW").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for AsyncIMSAdminBaseSinkW {
    type Vtable = AsyncIMSAdminBaseSinkW_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa9e69613_b80d_11d0_b9b9_00a0c922e750);
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIMSAdminBaseSinkW_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Begin_SinkNotify: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwmdnumelements: u32, pcochangelist: *const MD_CHANGE_OBJECT_W) -> ::windows::core::HRESULT,
    pub Finish_SinkNotify: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Begin_ShutdownNotify: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Finish_ShutdownNotify: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Iis\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub struct CERT_CONTEXT_EX {
    pub CertContext: super::super::Security::Cryptography::CERT_CONTEXT,
    pub cbAllocated: u32,
    pub dwCertificateFlags: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::marker::Copy for CERT_CONTEXT_EX {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::clone::Clone for CERT_CONTEXT_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::fmt::Debug for CERT_CONTEXT_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERT_CONTEXT_EX").field("CertContext", &self.CertContext).field("cbAllocated", &self.cbAllocated).field("dwCertificateFlags", &self.dwCertificateFlags).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
unsafe impl ::windows::core::Abi for CERT_CONTEXT_EX {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::PartialEq for CERT_CONTEXT_EX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CERT_CONTEXT_EX>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::Eq for CERT_CONTEXT_EX {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::default::Default for CERT_CONTEXT_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const CLSID_IImgCtx: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3050f3d6_98b5_11cf_bb82_00aa00bdce0b);
pub const CLSID_IisServiceControl: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe8fb8621_588f_11d2_9d61_00c04f79c5fe);
pub const CLSID_MSAdminBase_W: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa9e69610_b80d_11d0_b9b9_00a0c922e750);
pub const CLSID_Request: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x920c25d0_25d9_11d0_a55f_00a0c90c2091);
pub const CLSID_Response: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x46e19ba0_25dd_11d0_a55f_00a0c90c2091);
pub const CLSID_ScriptingContext: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd97a6da0_a868_11cf_83ae_11b0c90c2bd8);
pub const CLSID_Server: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa506d160_25e0_11d0_a55f_00a0c90c2091);
pub const CLSID_Session: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x509f8f20_25de_11d0_a55f_00a0c90c2091);
pub const CLSID_WamAdmin: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x61738644_f196_11d0_9953_00c04fd919c1);
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Iis\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CONFIGURATION_ENTRY {
    pub bstrKey: super::super::Foundation::BSTR,
    pub bstrValue: super::super::Foundation::BSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CONFIGURATION_ENTRY {
    fn clone(&self) -> Self {
        Self { bstrKey: self.bstrKey.clone(), bstrValue: self.bstrValue.clone() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CONFIGURATION_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CONFIGURATION_ENTRY").field("bstrKey", &self.bstrKey).field("bstrValue", &self.bstrValue).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CONFIGURATION_ENTRY {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CONFIGURATION_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.bstrKey == other.bstrKey && self.bstrValue == other.bstrValue
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CONFIGURATION_ENTRY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CONFIGURATION_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const DISPID_HTTPREQUEST_ABORT: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const DISPID_HTTPREQUEST_BASE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const DISPID_HTTPREQUEST_GETALLRESPONSEHEADERS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const DISPID_HTTPREQUEST_GETRESPONSEHEADER: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const DISPID_HTTPREQUEST_OPEN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const DISPID_HTTPREQUEST_OPTION: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const DISPID_HTTPREQUEST_RESPONSEBODY: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const DISPID_HTTPREQUEST_RESPONSESTREAM: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const DISPID_HTTPREQUEST_RESPONSETEXT: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const DISPID_HTTPREQUEST_SEND: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const DISPID_HTTPREQUEST_SETAUTOLOGONPOLICY: u32 = 18u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const DISPID_HTTPREQUEST_SETCLIENTCERTIFICATE: u32 = 17u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const DISPID_HTTPREQUEST_SETCREDENTIALS: u32 = 14u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const DISPID_HTTPREQUEST_SETPROXY: u32 = 13u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const DISPID_HTTPREQUEST_SETREQUESTHEADER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const DISPID_HTTPREQUEST_SETTIMEOUTS: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const DISPID_HTTPREQUEST_STATUS: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const DISPID_HTTPREQUEST_STATUSTEXT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const DISPID_HTTPREQUEST_WAITFORRESPONSE: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const DWN_COLORMODE: u32 = 63u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const DWN_DOWNLOADONLY: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const DWN_FORCEDITHER: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const DWN_MIRRORIMAGE: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const DWN_RAWIMAGE: u32 = 256u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Iis\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EXTENSION_CONTROL_BLOCK {
    pub cbSize: u32,
    pub dwVersion: u32,
    pub ConnID: *mut ::core::ffi::c_void,
    pub dwHttpStatusCode: u32,
    pub lpszLogData: [super::super::Foundation::CHAR; 80],
    pub lpszMethod: ::windows::core::PSTR,
    pub lpszQueryString: ::windows::core::PSTR,
    pub lpszPathInfo: ::windows::core::PSTR,
    pub lpszPathTranslated: ::windows::core::PSTR,
    pub cbTotalBytes: u32,
    pub cbAvailable: u32,
    pub lpbData: *mut u8,
    pub lpszContentType: ::windows::core::PSTR,
    pub GetServerVariable: isize,
    pub WriteClient: isize,
    pub ReadClient: isize,
    pub ServerSupportFunction: isize,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EXTENSION_CONTROL_BLOCK {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EXTENSION_CONTROL_BLOCK {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EXTENSION_CONTROL_BLOCK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EXTENSION_CONTROL_BLOCK")
            .field("cbSize", &self.cbSize)
            .field("dwVersion", &self.dwVersion)
            .field("ConnID", &self.ConnID)
            .field("dwHttpStatusCode", &self.dwHttpStatusCode)
            .field("lpszLogData", &self.lpszLogData)
            .field("lpszMethod", &self.lpszMethod)
            .field("lpszQueryString", &self.lpszQueryString)
            .field("lpszPathInfo", &self.lpszPathInfo)
            .field("lpszPathTranslated", &self.lpszPathTranslated)
            .field("cbTotalBytes", &self.cbTotalBytes)
            .field("cbAvailable", &self.cbAvailable)
            .field("lpbData", &self.lpbData)
            .field("lpszContentType", &self.lpszContentType)
            .field("GetServerVariable", &self.GetServerVariable)
            .field("WriteClient", &self.WriteClient)
            .field("ReadClient", &self.ReadClient)
            .field("ServerSupportFunction", &self.ServerSupportFunction)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for EXTENSION_CONTROL_BLOCK {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EXTENSION_CONTROL_BLOCK {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EXTENSION_CONTROL_BLOCK>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EXTENSION_CONTROL_BLOCK {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EXTENSION_CONTROL_BLOCK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const FP_MD_ID_BEGIN_RESERVED: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const FP_MD_ID_END_RESERVED: u32 = 36863u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FTP_ACCESS(pub i32);
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const FTP_ACCESS_NONE: FTP_ACCESS = FTP_ACCESS(0i32);
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const FTP_ACCESS_READ: FTP_ACCESS = FTP_ACCESS(1i32);
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const FTP_ACCESS_WRITE: FTP_ACCESS = FTP_ACCESS(2i32);
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const FTP_ACCESS_READ_WRITE: FTP_ACCESS = FTP_ACCESS(3i32);
impl ::core::marker::Copy for FTP_ACCESS {}
impl ::core::clone::Clone for FTP_ACCESS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FTP_ACCESS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FTP_ACCESS {
    type Abi = Self;
}
impl ::core::fmt::Debug for FTP_ACCESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FTP_ACCESS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FTP_PROCESS_STATUS(pub i32);
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const FTP_PROCESS_CONTINUE: FTP_PROCESS_STATUS = FTP_PROCESS_STATUS(0i32);
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const FTP_PROCESS_CLOSE_SESSION: FTP_PROCESS_STATUS = FTP_PROCESS_STATUS(1i32);
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const FTP_PROCESS_TERMINATE_SESSION: FTP_PROCESS_STATUS = FTP_PROCESS_STATUS(2i32);
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const FTP_PROCESS_REJECT_COMMAND: FTP_PROCESS_STATUS = FTP_PROCESS_STATUS(3i32);
impl ::core::marker::Copy for FTP_PROCESS_STATUS {}
impl ::core::clone::Clone for FTP_PROCESS_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FTP_PROCESS_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FTP_PROCESS_STATUS {
    type Abi = Self;
}
impl ::core::fmt::Debug for FTP_PROCESS_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FTP_PROCESS_STATUS").field(&self.0).finish()
    }
}
pub const FtpProvider: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x70bdc667_33b2_45f0_ac52_c3ca46f7a656);
pub const GUID_IIS_ALL_TRACE_PROVIDERS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000000_0000_0000_0000_000000000000);
pub const GUID_IIS_ASPNET_TRACE_PROVIDER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaff081fe_0247_4275_9c4e_021f3dc1da35);
pub const GUID_IIS_ASP_TRACE_TRACE_PROVIDER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x06b94d9a_b15e_456e_a4ef_37c984a2cb4b);
pub const GUID_IIS_ISAPI_TRACE_PROVIDER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa1c2040e_8840_4c31_ba11_9871031a19ea);
pub const GUID_IIS_WWW_GLOBAL_TRACE_PROVIDER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd55d3bc9_cba9_44df_827e_132d3a4596c2);
pub const GUID_IIS_WWW_SERVER_TRACE_PROVIDER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3a2a4e84_4c21_4981_ae10_3fda0d9b0f83);
pub const GUID_IIS_WWW_SERVER_V2_TRACE_PROVIDER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xde4649c9_15e8_4fea_9d85_1cdda520c334);
#[doc = "*Required features: `\"Win32_System_Iis\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetExtensionVersion(pver: *mut HSE_VERSION_INFO) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetExtensionVersion(pver: *mut HSE_VERSION_INFO) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetExtensionVersion(::core::mem::transmute(pver)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Iis\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetFilterVersion(pver: *mut HTTP_FILTER_VERSION) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetFilterVersion(pver: *mut HTTP_FILTER_VERSION) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetFilterVersion(::core::mem::transmute(pver)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_APPEND_LOG_PARAMETER: u32 = 1003u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_APP_FLAG_IN_PROCESS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_APP_FLAG_ISOLATED_OOP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_APP_FLAG_POOLED_OOP: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Iis\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct HSE_CUSTOM_ERROR_INFO {
    pub pszStatus: ::windows::core::PSTR,
    pub uHttpSubError: u16,
    pub fAsync: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HSE_CUSTOM_ERROR_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HSE_CUSTOM_ERROR_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HSE_CUSTOM_ERROR_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HSE_CUSTOM_ERROR_INFO").field("pszStatus", &self.pszStatus).field("uHttpSubError", &self.uHttpSubError).field("fAsync", &self.fAsync).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for HSE_CUSTOM_ERROR_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HSE_CUSTOM_ERROR_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HSE_CUSTOM_ERROR_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HSE_CUSTOM_ERROR_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HSE_CUSTOM_ERROR_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Iis\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct HSE_EXEC_UNICODE_URL_INFO {
    pub pszUrl: ::windows::core::PWSTR,
    pub pszMethod: ::windows::core::PSTR,
    pub pszChildHeaders: ::windows::core::PSTR,
    pub pUserInfo: *mut HSE_EXEC_UNICODE_URL_USER_INFO,
    pub pEntity: *mut HSE_EXEC_URL_ENTITY_INFO,
    pub dwExecUrlFlags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HSE_EXEC_UNICODE_URL_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HSE_EXEC_UNICODE_URL_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HSE_EXEC_UNICODE_URL_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HSE_EXEC_UNICODE_URL_INFO").field("pszUrl", &self.pszUrl).field("pszMethod", &self.pszMethod).field("pszChildHeaders", &self.pszChildHeaders).field("pUserInfo", &self.pUserInfo).field("pEntity", &self.pEntity).field("dwExecUrlFlags", &self.dwExecUrlFlags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for HSE_EXEC_UNICODE_URL_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HSE_EXEC_UNICODE_URL_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HSE_EXEC_UNICODE_URL_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HSE_EXEC_UNICODE_URL_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HSE_EXEC_UNICODE_URL_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Iis\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct HSE_EXEC_UNICODE_URL_USER_INFO {
    pub hImpersonationToken: super::super::Foundation::HANDLE,
    pub pszCustomUserName: ::windows::core::PWSTR,
    pub pszCustomAuthType: ::windows::core::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HSE_EXEC_UNICODE_URL_USER_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HSE_EXEC_UNICODE_URL_USER_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HSE_EXEC_UNICODE_URL_USER_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HSE_EXEC_UNICODE_URL_USER_INFO").field("hImpersonationToken", &self.hImpersonationToken).field("pszCustomUserName", &self.pszCustomUserName).field("pszCustomAuthType", &self.pszCustomAuthType).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for HSE_EXEC_UNICODE_URL_USER_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HSE_EXEC_UNICODE_URL_USER_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HSE_EXEC_UNICODE_URL_USER_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HSE_EXEC_UNICODE_URL_USER_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HSE_EXEC_UNICODE_URL_USER_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_EXEC_URL_DISABLE_CUSTOM_ERROR: u32 = 32u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub struct HSE_EXEC_URL_ENTITY_INFO {
    pub cbAvailable: u32,
    pub lpbData: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for HSE_EXEC_URL_ENTITY_INFO {}
impl ::core::clone::Clone for HSE_EXEC_URL_ENTITY_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HSE_EXEC_URL_ENTITY_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HSE_EXEC_URL_ENTITY_INFO").field("cbAvailable", &self.cbAvailable).field("lpbData", &self.lpbData).finish()
    }
}
unsafe impl ::windows::core::Abi for HSE_EXEC_URL_ENTITY_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HSE_EXEC_URL_ENTITY_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HSE_EXEC_URL_ENTITY_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for HSE_EXEC_URL_ENTITY_INFO {}
impl ::core::default::Default for HSE_EXEC_URL_ENTITY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_EXEC_URL_HTTP_CACHE_ELIGIBLE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_EXEC_URL_IGNORE_CURRENT_INTERCEPTOR: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_EXEC_URL_IGNORE_VALIDATION_AND_RANGE: u32 = 16u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Iis\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct HSE_EXEC_URL_INFO {
    pub pszUrl: ::windows::core::PSTR,
    pub pszMethod: ::windows::core::PSTR,
    pub pszChildHeaders: ::windows::core::PSTR,
    pub pUserInfo: *mut HSE_EXEC_URL_USER_INFO,
    pub pEntity: *mut HSE_EXEC_URL_ENTITY_INFO,
    pub dwExecUrlFlags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HSE_EXEC_URL_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HSE_EXEC_URL_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HSE_EXEC_URL_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HSE_EXEC_URL_INFO").field("pszUrl", &self.pszUrl).field("pszMethod", &self.pszMethod).field("pszChildHeaders", &self.pszChildHeaders).field("pUserInfo", &self.pUserInfo).field("pEntity", &self.pEntity).field("dwExecUrlFlags", &self.dwExecUrlFlags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for HSE_EXEC_URL_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HSE_EXEC_URL_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HSE_EXEC_URL_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HSE_EXEC_URL_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HSE_EXEC_URL_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_EXEC_URL_NO_HEADERS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_EXEC_URL_SSI_CMD: u32 = 64u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub struct HSE_EXEC_URL_STATUS {
    pub uHttpStatusCode: u16,
    pub uHttpSubStatus: u16,
    pub dwWin32Error: u32,
}
impl ::core::marker::Copy for HSE_EXEC_URL_STATUS {}
impl ::core::clone::Clone for HSE_EXEC_URL_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HSE_EXEC_URL_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HSE_EXEC_URL_STATUS").field("uHttpStatusCode", &self.uHttpStatusCode).field("uHttpSubStatus", &self.uHttpSubStatus).field("dwWin32Error", &self.dwWin32Error).finish()
    }
}
unsafe impl ::windows::core::Abi for HSE_EXEC_URL_STATUS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HSE_EXEC_URL_STATUS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HSE_EXEC_URL_STATUS>()) == 0 }
    }
}
impl ::core::cmp::Eq for HSE_EXEC_URL_STATUS {}
impl ::core::default::Default for HSE_EXEC_URL_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Iis\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct HSE_EXEC_URL_USER_INFO {
    pub hImpersonationToken: super::super::Foundation::HANDLE,
    pub pszCustomUserName: ::windows::core::PSTR,
    pub pszCustomAuthType: ::windows::core::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HSE_EXEC_URL_USER_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HSE_EXEC_URL_USER_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HSE_EXEC_URL_USER_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HSE_EXEC_URL_USER_INFO").field("hImpersonationToken", &self.hImpersonationToken).field("pszCustomUserName", &self.pszCustomUserName).field("pszCustomAuthType", &self.pszCustomAuthType).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for HSE_EXEC_URL_USER_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HSE_EXEC_URL_USER_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HSE_EXEC_URL_USER_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HSE_EXEC_URL_USER_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HSE_EXEC_URL_USER_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_IO_ASYNC: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_IO_CACHE_RESPONSE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_IO_DISCONNECT_AFTER_SEND: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_IO_FINAL_SEND: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_IO_NODELAY: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_IO_SEND_HEADERS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_IO_SYNC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_IO_TRY_SKIP_CUSTOM_ERRORS: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_LOG_BUFFER_LEN: u32 = 80u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_MAX_EXT_DLL_NAME_LEN: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_REQ_ABORTIVE_CLOSE: u32 = 1014u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_REQ_ASYNC_READ_CLIENT: u32 = 1010u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_REQ_BASE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_REQ_CANCEL_IO: u32 = 1049u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_REQ_CLOSE_CONNECTION: u32 = 1017u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_REQ_DONE_WITH_SESSION: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_REQ_END_RESERVED: u32 = 1000u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_REQ_EXEC_UNICODE_URL: u32 = 1025u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_REQ_EXEC_URL: u32 = 1026u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_REQ_GET_ANONYMOUS_TOKEN: u32 = 1038u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_REQ_GET_CACHE_INVALIDATION_CALLBACK: u32 = 1040u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_REQ_GET_CERT_INFO_EX: u32 = 1015u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_REQ_GET_CHANNEL_BINDING_TOKEN: u32 = 1050u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_REQ_GET_CONFIG_OBJECT: u32 = 1046u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_REQ_GET_EXEC_URL_STATUS: u32 = 1027u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_REQ_GET_IMPERSONATION_TOKEN: u32 = 1011u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_REQ_GET_PROTOCOL_MANAGER_CUSTOM_INTERFACE_CALLBACK: u32 = 1048u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_REQ_GET_SSPI_INFO: u32 = 1002u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_REQ_GET_TRACE_INFO: u32 = 1042u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_REQ_GET_TRACE_INFO_EX: u32 = 1044u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_REQ_GET_UNICODE_ANONYMOUS_TOKEN: u32 = 1041u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_REQ_GET_WORKER_PROCESS_SETTINGS: u32 = 1047u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_REQ_IO_COMPLETION: u32 = 1005u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_REQ_IS_CONNECTED: u32 = 1018u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_REQ_IS_IN_PROCESS: u32 = 1030u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_REQ_IS_KEEP_CONN: u32 = 1008u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_REQ_MAP_UNICODE_URL_TO_PATH: u32 = 1023u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_REQ_MAP_UNICODE_URL_TO_PATH_EX: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_REQ_MAP_URL_TO_PATH: u32 = 1001u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_REQ_MAP_URL_TO_PATH_EX: u32 = 1012u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_REQ_NORMALIZE_URL: u32 = 1033u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_REQ_RAISE_TRACE_EVENT: u32 = 1045u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_REQ_REFRESH_ISAPI_ACL: u32 = 1007u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_REQ_REPORT_UNHEALTHY: u32 = 1032u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_REQ_SEND_CUSTOM_ERROR: u32 = 1028u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_REQ_SEND_RESPONSE_HEADER: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_REQ_SEND_RESPONSE_HEADER_EX: u32 = 1016u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_REQ_SEND_URL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_REQ_SEND_URL_REDIRECT_RESP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_REQ_SET_FLUSH_FLAG: u32 = 1043u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_REQ_TRANSMIT_FILE: u32 = 1006u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_REQ_VECTOR_SEND: u32 = 1037u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub struct HSE_RESPONSE_VECTOR {
    pub dwFlags: u32,
    pub pszStatus: ::windows::core::PSTR,
    pub pszHeaders: ::windows::core::PSTR,
    pub nElementCount: u32,
    pub lpElementArray: *mut HSE_VECTOR_ELEMENT,
}
impl ::core::marker::Copy for HSE_RESPONSE_VECTOR {}
impl ::core::clone::Clone for HSE_RESPONSE_VECTOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HSE_RESPONSE_VECTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HSE_RESPONSE_VECTOR").field("dwFlags", &self.dwFlags).field("pszStatus", &self.pszStatus).field("pszHeaders", &self.pszHeaders).field("nElementCount", &self.nElementCount).field("lpElementArray", &self.lpElementArray).finish()
    }
}
unsafe impl ::windows::core::Abi for HSE_RESPONSE_VECTOR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HSE_RESPONSE_VECTOR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HSE_RESPONSE_VECTOR>()) == 0 }
    }
}
impl ::core::cmp::Eq for HSE_RESPONSE_VECTOR {}
impl ::core::default::Default for HSE_RESPONSE_VECTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Iis\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct HSE_SEND_HEADER_EX_INFO {
    pub pszStatus: ::windows::core::PCSTR,
    pub pszHeader: ::windows::core::PCSTR,
    pub cchStatus: u32,
    pub cchHeader: u32,
    pub fKeepConn: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HSE_SEND_HEADER_EX_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HSE_SEND_HEADER_EX_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HSE_SEND_HEADER_EX_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HSE_SEND_HEADER_EX_INFO").field("pszStatus", &self.pszStatus).field("pszHeader", &self.pszHeader).field("cchStatus", &self.cchStatus).field("cchHeader", &self.cchHeader).field("fKeepConn", &self.fKeepConn).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for HSE_SEND_HEADER_EX_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HSE_SEND_HEADER_EX_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HSE_SEND_HEADER_EX_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HSE_SEND_HEADER_EX_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HSE_SEND_HEADER_EX_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_STATUS_ERROR: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_STATUS_PENDING: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_STATUS_SUCCESS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_STATUS_SUCCESS_AND_KEEP_CONN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_TERM_ADVISORY_UNLOAD: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_TERM_MUST_UNLOAD: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Iis\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct HSE_TF_INFO {
    pub pfnHseIO: PFN_HSE_IO_COMPLETION,
    pub pContext: *mut ::core::ffi::c_void,
    pub hFile: super::super::Foundation::HANDLE,
    pub pszStatusCode: ::windows::core::PCSTR,
    pub BytesToWrite: u32,
    pub Offset: u32,
    pub pHead: *mut ::core::ffi::c_void,
    pub HeadLength: u32,
    pub pTail: *mut ::core::ffi::c_void,
    pub TailLength: u32,
    pub dwFlags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HSE_TF_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HSE_TF_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HSE_TF_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HSE_TF_INFO").field("pfnHseIO", &self.pfnHseIO.map(|f| f as usize)).field("pContext", &self.pContext).field("hFile", &self.hFile).field("pszStatusCode", &self.pszStatusCode).field("BytesToWrite", &self.BytesToWrite).field("Offset", &self.Offset).field("pHead", &self.pHead).field("HeadLength", &self.HeadLength).field("pTail", &self.pTail).field("TailLength", &self.TailLength).field("dwFlags", &self.dwFlags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for HSE_TF_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HSE_TF_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HSE_TF_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HSE_TF_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HSE_TF_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Iis\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct HSE_TRACE_INFO {
    pub fTraceRequest: super::super::Foundation::BOOL,
    pub TraceContextId: [u8; 16],
    pub dwReserved1: u32,
    pub dwReserved2: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HSE_TRACE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HSE_TRACE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HSE_TRACE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HSE_TRACE_INFO").field("fTraceRequest", &self.fTraceRequest).field("TraceContextId", &self.TraceContextId).field("dwReserved1", &self.dwReserved1).field("dwReserved2", &self.dwReserved2).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for HSE_TRACE_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HSE_TRACE_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HSE_TRACE_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HSE_TRACE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HSE_TRACE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub struct HSE_UNICODE_URL_MAPEX_INFO {
    pub lpszPath: [u16; 260],
    pub dwFlags: u32,
    pub cchMatchingPath: u32,
    pub cchMatchingURL: u32,
}
impl ::core::marker::Copy for HSE_UNICODE_URL_MAPEX_INFO {}
impl ::core::clone::Clone for HSE_UNICODE_URL_MAPEX_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HSE_UNICODE_URL_MAPEX_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HSE_UNICODE_URL_MAPEX_INFO").field("lpszPath", &self.lpszPath).field("dwFlags", &self.dwFlags).field("cchMatchingPath", &self.cchMatchingPath).field("cchMatchingURL", &self.cchMatchingURL).finish()
    }
}
unsafe impl ::windows::core::Abi for HSE_UNICODE_URL_MAPEX_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HSE_UNICODE_URL_MAPEX_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HSE_UNICODE_URL_MAPEX_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for HSE_UNICODE_URL_MAPEX_INFO {}
impl ::core::default::Default for HSE_UNICODE_URL_MAPEX_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_URL_FLAGS_DONT_CACHE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_URL_FLAGS_EXECUTE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_URL_FLAGS_MAP_CERT: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_URL_FLAGS_MASK: u32 = 1023u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_URL_FLAGS_NEGO_CERT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_URL_FLAGS_READ: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_URL_FLAGS_REQUIRE_CERT: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_URL_FLAGS_SCRIPT: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_URL_FLAGS_SSL: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_URL_FLAGS_SSL128: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_URL_FLAGS_WRITE: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Iis\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct HSE_URL_MAPEX_INFO {
    pub lpszPath: [super::super::Foundation::CHAR; 260],
    pub dwFlags: u32,
    pub cchMatchingPath: u32,
    pub cchMatchingURL: u32,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HSE_URL_MAPEX_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HSE_URL_MAPEX_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HSE_URL_MAPEX_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HSE_URL_MAPEX_INFO").field("lpszPath", &self.lpszPath).field("dwFlags", &self.dwFlags).field("cchMatchingPath", &self.cchMatchingPath).field("cchMatchingURL", &self.cchMatchingURL).field("dwReserved1", &self.dwReserved1).field("dwReserved2", &self.dwReserved2).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for HSE_URL_MAPEX_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HSE_URL_MAPEX_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HSE_URL_MAPEX_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HSE_URL_MAPEX_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HSE_URL_MAPEX_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub struct HSE_VECTOR_ELEMENT {
    pub ElementType: u32,
    pub pvContext: *mut ::core::ffi::c_void,
    pub cbOffset: u64,
    pub cbSize: u64,
}
impl ::core::marker::Copy for HSE_VECTOR_ELEMENT {}
impl ::core::clone::Clone for HSE_VECTOR_ELEMENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HSE_VECTOR_ELEMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HSE_VECTOR_ELEMENT").field("ElementType", &self.ElementType).field("pvContext", &self.pvContext).field("cbOffset", &self.cbOffset).field("cbSize", &self.cbSize).finish()
    }
}
unsafe impl ::windows::core::Abi for HSE_VECTOR_ELEMENT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HSE_VECTOR_ELEMENT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HSE_VECTOR_ELEMENT>()) == 0 }
    }
}
impl ::core::cmp::Eq for HSE_VECTOR_ELEMENT {}
impl ::core::default::Default for HSE_VECTOR_ELEMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_VECTOR_ELEMENT_TYPE_FILE_HANDLE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_VECTOR_ELEMENT_TYPE_MEMORY_BUFFER: u32 = 0u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Iis\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct HSE_VERSION_INFO {
    pub dwExtensionVersion: u32,
    pub lpszExtensionDesc: [super::super::Foundation::CHAR; 256],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HSE_VERSION_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HSE_VERSION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HSE_VERSION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HSE_VERSION_INFO").field("dwExtensionVersion", &self.dwExtensionVersion).field("lpszExtensionDesc", &self.lpszExtensionDesc).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for HSE_VERSION_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HSE_VERSION_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HSE_VERSION_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HSE_VERSION_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HSE_VERSION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_VERSION_MAJOR: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_VERSION_MINOR: u32 = 0u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub struct HTTP_FILTER_ACCESS_DENIED {
    pub pszURL: ::windows::core::PCSTR,
    pub pszPhysicalPath: ::windows::core::PCSTR,
    pub dwReason: u32,
}
impl ::core::marker::Copy for HTTP_FILTER_ACCESS_DENIED {}
impl ::core::clone::Clone for HTTP_FILTER_ACCESS_DENIED {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HTTP_FILTER_ACCESS_DENIED {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_FILTER_ACCESS_DENIED").field("pszURL", &self.pszURL).field("pszPhysicalPath", &self.pszPhysicalPath).field("dwReason", &self.dwReason).finish()
    }
}
unsafe impl ::windows::core::Abi for HTTP_FILTER_ACCESS_DENIED {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HTTP_FILTER_ACCESS_DENIED {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_FILTER_ACCESS_DENIED>()) == 0 }
    }
}
impl ::core::cmp::Eq for HTTP_FILTER_ACCESS_DENIED {}
impl ::core::default::Default for HTTP_FILTER_ACCESS_DENIED {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub struct HTTP_FILTER_AUTHENT {
    pub pszUser: ::windows::core::PSTR,
    pub cbUserBuff: u32,
    pub pszPassword: ::windows::core::PSTR,
    pub cbPasswordBuff: u32,
}
impl ::core::marker::Copy for HTTP_FILTER_AUTHENT {}
impl ::core::clone::Clone for HTTP_FILTER_AUTHENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HTTP_FILTER_AUTHENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_FILTER_AUTHENT").field("pszUser", &self.pszUser).field("cbUserBuff", &self.cbUserBuff).field("pszPassword", &self.pszPassword).field("cbPasswordBuff", &self.cbPasswordBuff).finish()
    }
}
unsafe impl ::windows::core::Abi for HTTP_FILTER_AUTHENT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HTTP_FILTER_AUTHENT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_FILTER_AUTHENT>()) == 0 }
    }
}
impl ::core::cmp::Eq for HTTP_FILTER_AUTHENT {}
impl ::core::default::Default for HTTP_FILTER_AUTHENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Iis\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct HTTP_FILTER_AUTH_COMPLETE_INFO {
    pub GetHeader: isize,
    pub SetHeader: isize,
    pub AddHeader: isize,
    pub GetUserToken: isize,
    pub HttpStatus: u32,
    pub fResetAuth: super::super::Foundation::BOOL,
    pub dwReserved: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HTTP_FILTER_AUTH_COMPLETE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HTTP_FILTER_AUTH_COMPLETE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HTTP_FILTER_AUTH_COMPLETE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_FILTER_AUTH_COMPLETE_INFO").field("GetHeader", &self.GetHeader).field("SetHeader", &self.SetHeader).field("AddHeader", &self.AddHeader).field("GetUserToken", &self.GetUserToken).field("HttpStatus", &self.HttpStatus).field("fResetAuth", &self.fResetAuth).field("dwReserved", &self.dwReserved).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for HTTP_FILTER_AUTH_COMPLETE_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HTTP_FILTER_AUTH_COMPLETE_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_FILTER_AUTH_COMPLETE_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HTTP_FILTER_AUTH_COMPLETE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HTTP_FILTER_AUTH_COMPLETE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Iis\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct HTTP_FILTER_CONTEXT {
    pub cbSize: u32,
    pub Revision: u32,
    pub ServerContext: *mut ::core::ffi::c_void,
    pub ulReserved: u32,
    pub fIsSecurePort: super::super::Foundation::BOOL,
    pub pFilterContext: *mut ::core::ffi::c_void,
    pub GetServerVariable: isize,
    pub AddResponseHeaders: isize,
    pub WriteClient: isize,
    pub AllocMem: isize,
    pub ServerSupportFunction: isize,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HTTP_FILTER_CONTEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HTTP_FILTER_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HTTP_FILTER_CONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_FILTER_CONTEXT")
            .field("cbSize", &self.cbSize)
            .field("Revision", &self.Revision)
            .field("ServerContext", &self.ServerContext)
            .field("ulReserved", &self.ulReserved)
            .field("fIsSecurePort", &self.fIsSecurePort)
            .field("pFilterContext", &self.pFilterContext)
            .field("GetServerVariable", &self.GetServerVariable)
            .field("AddResponseHeaders", &self.AddResponseHeaders)
            .field("WriteClient", &self.WriteClient)
            .field("AllocMem", &self.AllocMem)
            .field("ServerSupportFunction", &self.ServerSupportFunction)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for HTTP_FILTER_CONTEXT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HTTP_FILTER_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_FILTER_CONTEXT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HTTP_FILTER_CONTEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HTTP_FILTER_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub struct HTTP_FILTER_LOG {
    pub pszClientHostName: ::windows::core::PCSTR,
    pub pszClientUserName: ::windows::core::PCSTR,
    pub pszServerName: ::windows::core::PCSTR,
    pub pszOperation: ::windows::core::PCSTR,
    pub pszTarget: ::windows::core::PCSTR,
    pub pszParameters: ::windows::core::PCSTR,
    pub dwHttpStatus: u32,
    pub dwWin32Status: u32,
    pub dwBytesSent: u32,
    pub dwBytesRecvd: u32,
    pub msTimeForProcessing: u32,
}
impl ::core::marker::Copy for HTTP_FILTER_LOG {}
impl ::core::clone::Clone for HTTP_FILTER_LOG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HTTP_FILTER_LOG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_FILTER_LOG")
            .field("pszClientHostName", &self.pszClientHostName)
            .field("pszClientUserName", &self.pszClientUserName)
            .field("pszServerName", &self.pszServerName)
            .field("pszOperation", &self.pszOperation)
            .field("pszTarget", &self.pszTarget)
            .field("pszParameters", &self.pszParameters)
            .field("dwHttpStatus", &self.dwHttpStatus)
            .field("dwWin32Status", &self.dwWin32Status)
            .field("dwBytesSent", &self.dwBytesSent)
            .field("dwBytesRecvd", &self.dwBytesRecvd)
            .field("msTimeForProcessing", &self.msTimeForProcessing)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for HTTP_FILTER_LOG {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HTTP_FILTER_LOG {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_FILTER_LOG>()) == 0 }
    }
}
impl ::core::cmp::Eq for HTTP_FILTER_LOG {}
impl ::core::default::Default for HTTP_FILTER_LOG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub struct HTTP_FILTER_PREPROC_HEADERS {
    pub GetHeader: isize,
    pub SetHeader: isize,
    pub AddHeader: isize,
    pub HttpStatus: u32,
    pub dwReserved: u32,
}
impl ::core::marker::Copy for HTTP_FILTER_PREPROC_HEADERS {}
impl ::core::clone::Clone for HTTP_FILTER_PREPROC_HEADERS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HTTP_FILTER_PREPROC_HEADERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_FILTER_PREPROC_HEADERS").field("GetHeader", &self.GetHeader).field("SetHeader", &self.SetHeader).field("AddHeader", &self.AddHeader).field("HttpStatus", &self.HttpStatus).field("dwReserved", &self.dwReserved).finish()
    }
}
unsafe impl ::windows::core::Abi for HTTP_FILTER_PREPROC_HEADERS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HTTP_FILTER_PREPROC_HEADERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_FILTER_PREPROC_HEADERS>()) == 0 }
    }
}
impl ::core::cmp::Eq for HTTP_FILTER_PREPROC_HEADERS {}
impl ::core::default::Default for HTTP_FILTER_PREPROC_HEADERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub struct HTTP_FILTER_RAW_DATA {
    pub pvInData: *mut ::core::ffi::c_void,
    pub cbInData: u32,
    pub cbInBuffer: u32,
    pub dwReserved: u32,
}
impl ::core::marker::Copy for HTTP_FILTER_RAW_DATA {}
impl ::core::clone::Clone for HTTP_FILTER_RAW_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HTTP_FILTER_RAW_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_FILTER_RAW_DATA").field("pvInData", &self.pvInData).field("cbInData", &self.cbInData).field("cbInBuffer", &self.cbInBuffer).field("dwReserved", &self.dwReserved).finish()
    }
}
unsafe impl ::windows::core::Abi for HTTP_FILTER_RAW_DATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HTTP_FILTER_RAW_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_FILTER_RAW_DATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for HTTP_FILTER_RAW_DATA {}
impl ::core::default::Default for HTTP_FILTER_RAW_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub struct HTTP_FILTER_URL_MAP {
    pub pszURL: ::windows::core::PCSTR,
    pub pszPhysicalPath: ::windows::core::PSTR,
    pub cbPathBuff: u32,
}
impl ::core::marker::Copy for HTTP_FILTER_URL_MAP {}
impl ::core::clone::Clone for HTTP_FILTER_URL_MAP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HTTP_FILTER_URL_MAP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_FILTER_URL_MAP").field("pszURL", &self.pszURL).field("pszPhysicalPath", &self.pszPhysicalPath).field("cbPathBuff", &self.cbPathBuff).finish()
    }
}
unsafe impl ::windows::core::Abi for HTTP_FILTER_URL_MAP {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HTTP_FILTER_URL_MAP {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_FILTER_URL_MAP>()) == 0 }
    }
}
impl ::core::cmp::Eq for HTTP_FILTER_URL_MAP {}
impl ::core::default::Default for HTTP_FILTER_URL_MAP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub struct HTTP_FILTER_URL_MAP_EX {
    pub pszURL: ::windows::core::PCSTR,
    pub pszPhysicalPath: ::windows::core::PSTR,
    pub cbPathBuff: u32,
    pub dwFlags: u32,
    pub cchMatchingPath: u32,
    pub cchMatchingURL: u32,
    pub pszScriptMapEntry: ::windows::core::PCSTR,
}
impl ::core::marker::Copy for HTTP_FILTER_URL_MAP_EX {}
impl ::core::clone::Clone for HTTP_FILTER_URL_MAP_EX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HTTP_FILTER_URL_MAP_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_FILTER_URL_MAP_EX").field("pszURL", &self.pszURL).field("pszPhysicalPath", &self.pszPhysicalPath).field("cbPathBuff", &self.cbPathBuff).field("dwFlags", &self.dwFlags).field("cchMatchingPath", &self.cchMatchingPath).field("cchMatchingURL", &self.cchMatchingURL).field("pszScriptMapEntry", &self.pszScriptMapEntry).finish()
    }
}
unsafe impl ::windows::core::Abi for HTTP_FILTER_URL_MAP_EX {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HTTP_FILTER_URL_MAP_EX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_FILTER_URL_MAP_EX>()) == 0 }
    }
}
impl ::core::cmp::Eq for HTTP_FILTER_URL_MAP_EX {}
impl ::core::default::Default for HTTP_FILTER_URL_MAP_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Iis\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct HTTP_FILTER_VERSION {
    pub dwServerFilterVersion: u32,
    pub dwFilterVersion: u32,
    pub lpszFilterDesc: [super::super::Foundation::CHAR; 257],
    pub dwFlags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HTTP_FILTER_VERSION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HTTP_FILTER_VERSION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HTTP_FILTER_VERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_FILTER_VERSION").field("dwServerFilterVersion", &self.dwServerFilterVersion).field("dwFilterVersion", &self.dwFilterVersion).field("lpszFilterDesc", &self.lpszFilterDesc).field("dwFlags", &self.dwFlags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for HTTP_FILTER_VERSION {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HTTP_FILTER_VERSION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_FILTER_VERSION>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HTTP_FILTER_VERSION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HTTP_FILTER_VERSION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Iis\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct HTTP_TRACE_CONFIGURATION {
    pub pProviderGuid: *const ::windows::core::GUID,
    pub dwAreas: u32,
    pub dwVerbosity: u32,
    pub fProviderEnabled: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HTTP_TRACE_CONFIGURATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HTTP_TRACE_CONFIGURATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HTTP_TRACE_CONFIGURATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_TRACE_CONFIGURATION").field("pProviderGuid", &self.pProviderGuid).field("dwAreas", &self.dwAreas).field("dwVerbosity", &self.dwVerbosity).field("fProviderEnabled", &self.fProviderEnabled).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for HTTP_TRACE_CONFIGURATION {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HTTP_TRACE_CONFIGURATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_TRACE_CONFIGURATION>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HTTP_TRACE_CONFIGURATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HTTP_TRACE_CONFIGURATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub struct HTTP_TRACE_EVENT {
    pub pProviderGuid: *const ::windows::core::GUID,
    pub dwArea: u32,
    pub pAreaGuid: *const ::windows::core::GUID,
    pub dwEvent: u32,
    pub pszEventName: ::windows::core::PCWSTR,
    pub dwEventVersion: u32,
    pub dwVerbosity: u32,
    pub pActivityGuid: *const ::windows::core::GUID,
    pub pRelatedActivityGuid: *const ::windows::core::GUID,
    pub dwTimeStamp: u32,
    pub dwFlags: u32,
    pub cEventItems: u32,
    pub pEventItems: *mut HTTP_TRACE_EVENT_ITEM,
}
impl ::core::marker::Copy for HTTP_TRACE_EVENT {}
impl ::core::clone::Clone for HTTP_TRACE_EVENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HTTP_TRACE_EVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_TRACE_EVENT")
            .field("pProviderGuid", &self.pProviderGuid)
            .field("dwArea", &self.dwArea)
            .field("pAreaGuid", &self.pAreaGuid)
            .field("dwEvent", &self.dwEvent)
            .field("pszEventName", &self.pszEventName)
            .field("dwEventVersion", &self.dwEventVersion)
            .field("dwVerbosity", &self.dwVerbosity)
            .field("pActivityGuid", &self.pActivityGuid)
            .field("pRelatedActivityGuid", &self.pRelatedActivityGuid)
            .field("dwTimeStamp", &self.dwTimeStamp)
            .field("dwFlags", &self.dwFlags)
            .field("cEventItems", &self.cEventItems)
            .field("pEventItems", &self.pEventItems)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for HTTP_TRACE_EVENT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HTTP_TRACE_EVENT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_TRACE_EVENT>()) == 0 }
    }
}
impl ::core::cmp::Eq for HTTP_TRACE_EVENT {}
impl ::core::default::Default for HTTP_TRACE_EVENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HTTP_TRACE_EVENT_FLAG_STATIC_DESCRIPTIVE_FIELDS: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub struct HTTP_TRACE_EVENT_ITEM {
    pub pszName: ::windows::core::PCWSTR,
    pub dwDataType: HTTP_TRACE_TYPE,
    pub pbData: *mut u8,
    pub cbData: u32,
    pub pszDataDescription: ::windows::core::PCWSTR,
}
impl ::core::marker::Copy for HTTP_TRACE_EVENT_ITEM {}
impl ::core::clone::Clone for HTTP_TRACE_EVENT_ITEM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HTTP_TRACE_EVENT_ITEM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_TRACE_EVENT_ITEM").field("pszName", &self.pszName).field("dwDataType", &self.dwDataType).field("pbData", &self.pbData).field("cbData", &self.cbData).field("pszDataDescription", &self.pszDataDescription).finish()
    }
}
unsafe impl ::windows::core::Abi for HTTP_TRACE_EVENT_ITEM {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HTTP_TRACE_EVENT_ITEM {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HTTP_TRACE_EVENT_ITEM>()) == 0 }
    }
}
impl ::core::cmp::Eq for HTTP_TRACE_EVENT_ITEM {}
impl ::core::default::Default for HTTP_TRACE_EVENT_ITEM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HTTP_TRACE_LEVEL_END: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HTTP_TRACE_LEVEL_START: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HTTP_TRACE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HTTP_TRACE_TYPE_BYTE: HTTP_TRACE_TYPE = HTTP_TRACE_TYPE(17i32);
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HTTP_TRACE_TYPE_USHORT: HTTP_TRACE_TYPE = HTTP_TRACE_TYPE(18i32);
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HTTP_TRACE_TYPE_ULONG: HTTP_TRACE_TYPE = HTTP_TRACE_TYPE(19i32);
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HTTP_TRACE_TYPE_ULONGLONG: HTTP_TRACE_TYPE = HTTP_TRACE_TYPE(21i32);
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HTTP_TRACE_TYPE_CHAR: HTTP_TRACE_TYPE = HTTP_TRACE_TYPE(16i32);
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HTTP_TRACE_TYPE_SHORT: HTTP_TRACE_TYPE = HTTP_TRACE_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HTTP_TRACE_TYPE_LONG: HTTP_TRACE_TYPE = HTTP_TRACE_TYPE(3i32);
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HTTP_TRACE_TYPE_LONGLONG: HTTP_TRACE_TYPE = HTTP_TRACE_TYPE(20i32);
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HTTP_TRACE_TYPE_LPCWSTR: HTTP_TRACE_TYPE = HTTP_TRACE_TYPE(31i32);
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HTTP_TRACE_TYPE_LPCSTR: HTTP_TRACE_TYPE = HTTP_TRACE_TYPE(30i32);
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HTTP_TRACE_TYPE_LPCGUID: HTTP_TRACE_TYPE = HTTP_TRACE_TYPE(72i32);
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HTTP_TRACE_TYPE_BOOL: HTTP_TRACE_TYPE = HTTP_TRACE_TYPE(11i32);
impl ::core::marker::Copy for HTTP_TRACE_TYPE {}
impl ::core::clone::Clone for HTTP_TRACE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HTTP_TRACE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for HTTP_TRACE_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for HTTP_TRACE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTTP_TRACE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Iis\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HttpExtensionProc(pecb: *const EXTENSION_CONTROL_BLOCK) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpExtensionProc(pecb: *const EXTENSION_CONTROL_BLOCK) -> u32;
        }
        ::core::mem::transmute(HttpExtensionProc(::core::mem::transmute(pecb)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Iis\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HttpFilterProc(pfc: *mut HTTP_FILTER_CONTEXT, notificationtype: u32, pvnotification: *mut ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HttpFilterProc(pfc: *mut HTTP_FILTER_CONTEXT, notificationtype: u32, pvnotification: *mut ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(HttpFilterProc(::core::mem::transmute(pfc), ::core::mem::transmute(notificationtype), ::core::mem::transmute(pvnotification)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
#[repr(transparent)]
pub struct IADMEXT(::windows::core::IUnknown);
impl IADMEXT {
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn Initialize(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Initialize)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn EnumDcomCLSIDs(&self, pclsiddcom: *mut ::windows::core::GUID, dwenumindex: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnumDcomCLSIDs)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pclsiddcom), ::core::mem::transmute(dwenumindex)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn Terminate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Terminate)(::windows::core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IADMEXT> for ::windows::core::IUnknown {
    fn from(value: IADMEXT) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IADMEXT> for ::windows::core::IUnknown {
    fn from(value: &IADMEXT) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IADMEXT {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IADMEXT {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IADMEXT {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IADMEXT {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IADMEXT {}
impl ::core::fmt::Debug for IADMEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IADMEXT").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IADMEXT {
    type Vtable = IADMEXT_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51dfe970_f6f2_11d0_b9bd_00a0c922e750);
}
#[repr(C)]
#[doc(hidden)]
pub struct IADMEXT_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EnumDcomCLSIDs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclsiddcom: *mut ::windows::core::GUID, dwenumindex: u32) -> ::windows::core::HRESULT,
    pub Terminate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
#[repr(transparent)]
pub struct IFtpAuthenticationProvider(::windows::core::IUnknown);
impl IFtpAuthenticationProvider {
    #[doc = "*Required features: `\"Win32_System_Iis\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AuthenticateUser<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pszsessionid: Param0, pszsitename: Param1, pszusername: Param2, pszpassword: Param3, ppszcanonicalusername: *mut ::windows::core::PWSTR, pfauthenticated: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AuthenticateUser)(::windows::core::Interface::as_raw(self), pszsessionid.into_param().abi(), pszsitename.into_param().abi(), pszusername.into_param().abi(), pszpassword.into_param().abi(), ::core::mem::transmute(ppszcanonicalusername), ::core::mem::transmute(pfauthenticated)).ok()
    }
}
impl ::core::convert::From<IFtpAuthenticationProvider> for ::windows::core::IUnknown {
    fn from(value: IFtpAuthenticationProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFtpAuthenticationProvider> for ::windows::core::IUnknown {
    fn from(value: &IFtpAuthenticationProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFtpAuthenticationProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFtpAuthenticationProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFtpAuthenticationProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFtpAuthenticationProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFtpAuthenticationProvider {}
impl ::core::fmt::Debug for IFtpAuthenticationProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFtpAuthenticationProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IFtpAuthenticationProvider {
    type Vtable = IFtpAuthenticationProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4659f95c_d5a8_4707_b2fc_6fd5794246cf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFtpAuthenticationProvider_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub AuthenticateUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszsessionid: ::windows::core::PCWSTR, pszsitename: ::windows::core::PCWSTR, pszusername: ::windows::core::PCWSTR, pszpassword: ::windows::core::PCWSTR, ppszcanonicalusername: *mut ::windows::core::PWSTR, pfauthenticated: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AuthenticateUser: usize,
}
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
#[repr(transparent)]
pub struct IFtpAuthorizationProvider(::windows::core::IUnknown);
impl IFtpAuthorizationProvider {
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn GetUserAccessPermission<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pszsessionid: Param0, pszsitename: Param1, pszvirtualpath: Param2, pszusername: Param3) -> ::windows::core::Result<FTP_ACCESS> {
        let mut result__ = ::core::mem::MaybeUninit::<FTP_ACCESS>::zeroed();
        (::windows::core::Interface::vtable(self).GetUserAccessPermission)(::windows::core::Interface::as_raw(self), pszsessionid.into_param().abi(), pszsitename.into_param().abi(), pszvirtualpath.into_param().abi(), pszusername.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<FTP_ACCESS>(result__)
    }
}
impl ::core::convert::From<IFtpAuthorizationProvider> for ::windows::core::IUnknown {
    fn from(value: IFtpAuthorizationProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFtpAuthorizationProvider> for ::windows::core::IUnknown {
    fn from(value: &IFtpAuthorizationProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFtpAuthorizationProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFtpAuthorizationProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFtpAuthorizationProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFtpAuthorizationProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFtpAuthorizationProvider {}
impl ::core::fmt::Debug for IFtpAuthorizationProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFtpAuthorizationProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IFtpAuthorizationProvider {
    type Vtable = IFtpAuthorizationProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa50ae7a1_a35a_42b4_a4f3_f4f7057a05d1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFtpAuthorizationProvider_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetUserAccessPermission: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszsessionid: ::windows::core::PCWSTR, pszsitename: ::windows::core::PCWSTR, pszvirtualpath: ::windows::core::PCWSTR, pszusername: ::windows::core::PCWSTR, pftpaccess: *mut FTP_ACCESS) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
#[repr(transparent)]
pub struct IFtpHomeDirectoryProvider(::windows::core::IUnknown);
impl IFtpHomeDirectoryProvider {
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn GetUserHomeDirectoryData<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pszsessionid: Param0, pszsitename: Param1, pszusername: Param2) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<::windows::core::PWSTR>::zeroed();
        (::windows::core::Interface::vtable(self).GetUserHomeDirectoryData)(::windows::core::Interface::as_raw(self), pszsessionid.into_param().abi(), pszsitename.into_param().abi(), pszusername.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
}
impl ::core::convert::From<IFtpHomeDirectoryProvider> for ::windows::core::IUnknown {
    fn from(value: IFtpHomeDirectoryProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFtpHomeDirectoryProvider> for ::windows::core::IUnknown {
    fn from(value: &IFtpHomeDirectoryProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFtpHomeDirectoryProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFtpHomeDirectoryProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFtpHomeDirectoryProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFtpHomeDirectoryProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFtpHomeDirectoryProvider {}
impl ::core::fmt::Debug for IFtpHomeDirectoryProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFtpHomeDirectoryProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IFtpHomeDirectoryProvider {
    type Vtable = IFtpHomeDirectoryProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0933b392_18dd_4097_8b9c_83325c35d9a6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFtpHomeDirectoryProvider_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetUserHomeDirectoryData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszsessionid: ::windows::core::PCWSTR, pszsitename: ::windows::core::PCWSTR, pszusername: ::windows::core::PCWSTR, ppszhomedirectorydata: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
#[repr(transparent)]
pub struct IFtpLogProvider(::windows::core::IUnknown);
impl IFtpLogProvider {
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn Log(&self, ploggingparameters: *const LOGGING_PARAMETERS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Log)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ploggingparameters)).ok()
    }
}
impl ::core::convert::From<IFtpLogProvider> for ::windows::core::IUnknown {
    fn from(value: IFtpLogProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFtpLogProvider> for ::windows::core::IUnknown {
    fn from(value: &IFtpLogProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFtpLogProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFtpLogProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFtpLogProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFtpLogProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFtpLogProvider {}
impl ::core::fmt::Debug for IFtpLogProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFtpLogProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IFtpLogProvider {
    type Vtable = IFtpLogProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa18a94cc_8299_4408_816c_7c3baca1a40e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFtpLogProvider_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Log: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ploggingparameters: *const LOGGING_PARAMETERS) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
#[repr(transparent)]
pub struct IFtpPostprocessProvider(::windows::core::IUnknown);
impl IFtpPostprocessProvider {
    #[doc = "*Required features: `\"Win32_System_Iis\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HandlePostprocess(&self, ppostprocessparameters: *const POST_PROCESS_PARAMETERS) -> ::windows::core::Result<FTP_PROCESS_STATUS> {
        let mut result__ = ::core::mem::MaybeUninit::<FTP_PROCESS_STATUS>::zeroed();
        (::windows::core::Interface::vtable(self).HandlePostprocess)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppostprocessparameters), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<FTP_PROCESS_STATUS>(result__)
    }
}
impl ::core::convert::From<IFtpPostprocessProvider> for ::windows::core::IUnknown {
    fn from(value: IFtpPostprocessProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFtpPostprocessProvider> for ::windows::core::IUnknown {
    fn from(value: &IFtpPostprocessProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFtpPostprocessProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFtpPostprocessProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFtpPostprocessProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFtpPostprocessProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFtpPostprocessProvider {}
impl ::core::fmt::Debug for IFtpPostprocessProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFtpPostprocessProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IFtpPostprocessProvider {
    type Vtable = IFtpPostprocessProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4522cbc6_16cd_49ad_8653_9a2c579e4280);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFtpPostprocessProvider_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub HandlePostprocess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppostprocessparameters: *const POST_PROCESS_PARAMETERS, pftpprocessstatus: *mut FTP_PROCESS_STATUS) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    HandlePostprocess: usize,
}
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
#[repr(transparent)]
pub struct IFtpPreprocessProvider(::windows::core::IUnknown);
impl IFtpPreprocessProvider {
    #[doc = "*Required features: `\"Win32_System_Iis\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HandlePreprocess(&self, ppreprocessparameters: *const PRE_PROCESS_PARAMETERS) -> ::windows::core::Result<FTP_PROCESS_STATUS> {
        let mut result__ = ::core::mem::MaybeUninit::<FTP_PROCESS_STATUS>::zeroed();
        (::windows::core::Interface::vtable(self).HandlePreprocess)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppreprocessparameters), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<FTP_PROCESS_STATUS>(result__)
    }
}
impl ::core::convert::From<IFtpPreprocessProvider> for ::windows::core::IUnknown {
    fn from(value: IFtpPreprocessProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFtpPreprocessProvider> for ::windows::core::IUnknown {
    fn from(value: &IFtpPreprocessProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFtpPreprocessProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFtpPreprocessProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFtpPreprocessProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFtpPreprocessProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFtpPreprocessProvider {}
impl ::core::fmt::Debug for IFtpPreprocessProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFtpPreprocessProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IFtpPreprocessProvider {
    type Vtable = IFtpPreprocessProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa3c19b60_5a28_471a_8f93_ab30411cee82);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFtpPreprocessProvider_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub HandlePreprocess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppreprocessparameters: *const PRE_PROCESS_PARAMETERS, pftpprocessstatus: *mut FTP_PROCESS_STATUS) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    HandlePreprocess: usize,
}
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
#[repr(transparent)]
pub struct IFtpProviderConstruct(::windows::core::IUnknown);
impl IFtpProviderConstruct {
    #[doc = "*Required features: `\"Win32_System_Iis\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Construct(&self, configurationentries: *const super::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Construct)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(configurationentries)).ok()
    }
}
impl ::core::convert::From<IFtpProviderConstruct> for ::windows::core::IUnknown {
    fn from(value: IFtpProviderConstruct) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFtpProviderConstruct> for ::windows::core::IUnknown {
    fn from(value: &IFtpProviderConstruct) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFtpProviderConstruct {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFtpProviderConstruct {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFtpProviderConstruct {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFtpProviderConstruct {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFtpProviderConstruct {}
impl ::core::fmt::Debug for IFtpProviderConstruct {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFtpProviderConstruct").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IFtpProviderConstruct {
    type Vtable = IFtpProviderConstruct_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4d1a3f7b_412d_447c_b199_64f967e9a2da);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFtpProviderConstruct_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Construct: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, configurationentries: *const super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Construct: usize,
}
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
#[repr(transparent)]
pub struct IFtpRoleProvider(::windows::core::IUnknown);
impl IFtpRoleProvider {
    #[doc = "*Required features: `\"Win32_System_Iis\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsUserInRole<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pszsessionid: Param0, pszsitename: Param1, pszusername: Param2, pszrole: Param3) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::BOOL>::zeroed();
        (::windows::core::Interface::vtable(self).IsUserInRole)(::windows::core::Interface::as_raw(self), pszsessionid.into_param().abi(), pszsitename.into_param().abi(), pszusername.into_param().abi(), pszrole.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
impl ::core::convert::From<IFtpRoleProvider> for ::windows::core::IUnknown {
    fn from(value: IFtpRoleProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFtpRoleProvider> for ::windows::core::IUnknown {
    fn from(value: &IFtpRoleProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFtpRoleProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFtpRoleProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFtpRoleProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFtpRoleProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFtpRoleProvider {}
impl ::core::fmt::Debug for IFtpRoleProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFtpRoleProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IFtpRoleProvider {
    type Vtable = IFtpRoleProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x909c850d_8ca0_4674_96b8_cc2941535725);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFtpRoleProvider_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub IsUserInRole: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszsessionid: ::windows::core::PCWSTR, pszsitename: ::windows::core::PCWSTR, pszusername: ::windows::core::PCWSTR, pszrole: ::windows::core::PCWSTR, pfisinrole: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsUserInRole: usize,
}
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IISADMIN_EXTENSIONS_CLSID_MD_KEY: &str = "LM/IISADMIN/EXTENSIONS/DCOMCLSIDS";
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IISADMIN_EXTENSIONS_CLSID_MD_KEYA: &str = "LM/IISADMIN/EXTENSIONS/DCOMCLSIDS";
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IISADMIN_EXTENSIONS_CLSID_MD_KEYW: &str = "LM/IISADMIN/EXTENSIONS/DCOMCLSIDS";
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IISADMIN_EXTENSIONS_REG_KEY: &str = "SOFTWARE\\Microsoft\\InetStp\\Extensions";
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IISADMIN_EXTENSIONS_REG_KEYA: &str = "SOFTWARE\\Microsoft\\InetStp\\Extensions";
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IISADMIN_EXTENSIONS_REG_KEYW: &str = "SOFTWARE\\Microsoft\\InetStp\\Extensions";
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_CLASS_CERTMAPPER: &str = "IIsCertMapper";
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_CLASS_CERTMAPPER_W: &str = "IIsCertMapper";
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_CLASS_COMPRESS_SCHEME: &str = "IIsCompressionScheme";
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_CLASS_COMPRESS_SCHEMES: &str = "IIsCompressionSchemes";
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_CLASS_COMPRESS_SCHEMES_W: &str = "IIsCompressionSchemes";
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_CLASS_COMPRESS_SCHEME_W: &str = "IIsCompressionScheme";
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_CLASS_COMPUTER: &str = "IIsComputer";
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_CLASS_COMPUTER_W: &str = "IIsComputer";
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_CLASS_FILTER: &str = "IIsFilter";
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_CLASS_FILTERS: &str = "IIsFilters";
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_CLASS_FILTERS_W: &str = "IIsFilters";
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_CLASS_FILTER_W: &str = "IIsFilter";
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_CLASS_FTP_INFO: &str = "IIsFtpInfo";
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_CLASS_FTP_INFO_W: &str = "IIsFtpInfo";
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_CLASS_FTP_SERVER: &str = "IIsFtpServer";
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_CLASS_FTP_SERVER_W: &str = "IIsFtpServer";
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_CLASS_FTP_SERVICE: &str = "IIsFtpService";
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_CLASS_FTP_SERVICE_W: &str = "IIsFtpService";
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_CLASS_FTP_VDIR: &str = "IIsFtpVirtualDir";
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_CLASS_FTP_VDIR_W: &str = "IIsFtpVirtualDir";
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_CLASS_LOG_MODULE: &str = "IIsLogModule";
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_CLASS_LOG_MODULES: &str = "IIsLogModules";
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_CLASS_LOG_MODULES_W: &str = "IIsLogModules";
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_CLASS_LOG_MODULE_W: &str = "IIsLogModule";
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_CLASS_MIMEMAP: &str = "IIsMimeMap";
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_CLASS_MIMEMAP_W: &str = "IIsMimeMap";
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_CLASS_WEB_DIR: &str = "IIsWebDirectory";
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_CLASS_WEB_DIR_W: &str = "IIsWebDirectory";
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_CLASS_WEB_FILE: &str = "IIsWebFile";
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_CLASS_WEB_FILE_W: &str = "IIsWebFile";
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_CLASS_WEB_INFO: &str = "IIsWebInfo";
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_CLASS_WEB_INFO_W: &str = "IIsWebInfo";
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_CLASS_WEB_SERVER: &str = "IIsWebServer";
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_CLASS_WEB_SERVER_W: &str = "IIsWebServer";
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_CLASS_WEB_SERVICE: &str = "IIsWebService";
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_CLASS_WEB_SERVICE_W: &str = "IIsWebService";
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_CLASS_WEB_VDIR: &str = "IIsWebVirtualDir";
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_CLASS_WEB_VDIR_W: &str = "IIsWebVirtualDir";
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_MD_ADSI_METAID_BEGIN: u32 = 130000u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_MD_ADSI_SCHEMA_PATH_A: &str = "/Schema";
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_MD_ADSI_SCHEMA_PATH_W: &str = "/Schema";
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_MD_APPPOOL_BASE: u32 = 9000u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_MD_APP_BASE: u32 = 9100u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_MD_FILE_PROP_BASE: u32 = 6000u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_MD_FTP_BASE: u32 = 5000u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_MD_GLOBAL_BASE: u32 = 9200u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_MD_HTTP_BASE: u32 = 2000u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_MD_ID_BEGIN_RESERVED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_MD_ID_END_RESERVED: u32 = 32767u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_MD_INSTANCE_ROOT: &str = "Root";
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_MD_ISAPI_FILTERS: &str = "/Filters";
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_MD_LOCAL_MACHINE_PATH: &str = "LM";
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_MD_LOGCUSTOM_BASE: u32 = 4500u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_MD_LOGCUSTOM_LAST: u32 = 4508u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_MD_LOG_BASE: u32 = 4000u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_MD_LOG_LAST: u32 = 4015u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_MD_SERVER_BASE: u32 = 1000u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_MD_SSL_BASE: u32 = 5500u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_MD_SVC_INFO_PATH: &str = "Info";
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_MD_UT_END_RESERVED: u32 = 2000u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_MD_UT_FILE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_MD_UT_SERVER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_MD_UT_WAM: u32 = 100u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_MD_VR_BASE: u32 = 3000u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_WEBSOCKET: &str = "websockets";
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_WEBSOCKET_SERVER_VARIABLE: &str = "IIS_WEBSOCK";
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IMAP_MD_ID_BEGIN_RESERVED: u32 = 49152u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IMAP_MD_ID_END_RESERVED: u32 = 53247u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IMGANIM_ANIMATED: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IMGANIM_MASK: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IMGBITS_MASK: u32 = 234881024u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IMGBITS_NONE: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IMGBITS_PARTIAL: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IMGBITS_TOTAL: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IMGCHG_ANIMATE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IMGCHG_COMPLETE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IMGCHG_MASK: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IMGCHG_SIZE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IMGCHG_VIEW: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IMGLOAD_COMPLETE: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IMGLOAD_ERROR: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IMGLOAD_LOADING: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IMGLOAD_MASK: u32 = 32505856u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IMGLOAD_NOTLOADED: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IMGLOAD_STOPPED: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IMGTRANS_MASK: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IMGTRANS_OPAQUE: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
#[repr(transparent)]
pub struct IMSAdminBase2W(::windows::core::IUnknown);
impl IMSAdminBase2W {
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn AddKey<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, hmdhandle: u32, pszmdpath: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.AddKey)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(hmdhandle), pszmdpath.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn DeleteKey<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, hmdhandle: u32, pszmdpath: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.DeleteKey)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(hmdhandle), pszmdpath.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn DeleteChildKeys<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, hmdhandle: u32, pszmdpath: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.DeleteChildKeys)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(hmdhandle), pszmdpath.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn EnumKeys<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, hmdhandle: u32, pszmdpath: Param1, pszmdname: &mut [u16; 256], dwmdenumobjectindex: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.EnumKeys)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(hmdhandle), pszmdpath.into_param().abi(), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pszmdname)), ::core::mem::transmute(dwmdenumobjectindex)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CopyKey<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, hmdsourcehandle: u32, pszmdsourcepath: Param1, hmddesthandle: u32, pszmddestpath: Param3, bmdoverwriteflag: Param4, bmdcopyflag: Param5) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.CopyKey)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(hmdsourcehandle), pszmdsourcepath.into_param().abi(), ::core::mem::transmute(hmddesthandle), pszmddestpath.into_param().abi(), bmdoverwriteflag.into_param().abi(), bmdcopyflag.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn RenameKey<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, hmdhandle: u32, pszmdpath: Param1, pszmdnewname: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.RenameKey)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(hmdhandle), pszmdpath.into_param().abi(), pszmdnewname.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn SetData<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, hmdhandle: u32, pszmdpath: Param1, pmdrmddata: *mut METADATA_RECORD) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(hmdhandle), pszmdpath.into_param().abi(), ::core::mem::transmute(pmdrmddata)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn GetData<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, hmdhandle: u32, pszmdpath: Param1, pmdrmddata: *mut METADATA_RECORD, pdwmdrequireddatalen: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(hmdhandle), pszmdpath.into_param().abi(), ::core::mem::transmute(pmdrmddata), ::core::mem::transmute(pdwmdrequireddatalen)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn DeleteData<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, hmdhandle: u32, pszmdpath: Param1, dwmdidentifier: u32, dwmddatatype: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.DeleteData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(hmdhandle), pszmdpath.into_param().abi(), ::core::mem::transmute(dwmdidentifier), ::core::mem::transmute(dwmddatatype)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn EnumData<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, hmdhandle: u32, pszmdpath: Param1, pmdrmddata: *mut METADATA_RECORD, dwmdenumdataindex: u32, pdwmdrequireddatalen: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.EnumData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(hmdhandle), pszmdpath.into_param().abi(), ::core::mem::transmute(pmdrmddata), ::core::mem::transmute(dwmdenumdataindex), ::core::mem::transmute(pdwmdrequireddatalen)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn GetAllData<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, hmdhandle: u32, pszmdpath: Param1, dwmdattributes: u32, dwmdusertype: u32, dwmddatatype: u32, pdwmdnumdataentries: *mut u32, pdwmddatasetnumber: *mut u32, dwmdbuffersize: u32, pbmdbuffer: *mut u8, pdwmdrequiredbuffersize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetAllData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(hmdhandle), pszmdpath.into_param().abi(), ::core::mem::transmute(dwmdattributes), ::core::mem::transmute(dwmdusertype), ::core::mem::transmute(dwmddatatype), ::core::mem::transmute(pdwmdnumdataentries), ::core::mem::transmute(pdwmddatasetnumber), ::core::mem::transmute(dwmdbuffersize), ::core::mem::transmute(pbmdbuffer), ::core::mem::transmute(pdwmdrequiredbuffersize)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn DeleteAllData<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, hmdhandle: u32, pszmdpath: Param1, dwmdusertype: u32, dwmddatatype: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.DeleteAllData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(hmdhandle), pszmdpath.into_param().abi(), ::core::mem::transmute(dwmdusertype), ::core::mem::transmute(dwmddatatype)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CopyData<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param7: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, hmdsourcehandle: u32, pszmdsourcepath: Param1, hmddesthandle: u32, pszmddestpath: Param3, dwmdattributes: u32, dwmdusertype: u32, dwmddatatype: u32, bmdcopyflag: Param7) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.CopyData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(hmdsourcehandle), pszmdsourcepath.into_param().abi(), ::core::mem::transmute(hmddesthandle), pszmddestpath.into_param().abi(), ::core::mem::transmute(dwmdattributes), ::core::mem::transmute(dwmdusertype), ::core::mem::transmute(dwmddatatype), bmdcopyflag.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn GetDataPaths<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, hmdhandle: u32, pszmdpath: Param1, dwmdidentifier: u32, dwmddatatype: u32, pszbuffer: &mut [u16], pdwmdrequiredbuffersize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetDataPaths)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(hmdhandle), pszmdpath.into_param().abi(), ::core::mem::transmute(dwmdidentifier), ::core::mem::transmute(dwmddatatype), pszbuffer.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pszbuffer)), ::core::mem::transmute(pdwmdrequiredbuffersize)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn OpenKey<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, hmdhandle: u32, pszmdpath: Param1, dwmdaccessrequested: u32, dwmdtimeout: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows::core::Interface::vtable(self).base__.OpenKey)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(hmdhandle), pszmdpath.into_param().abi(), ::core::mem::transmute(dwmdaccessrequested), ::core::mem::transmute(dwmdtimeout), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn CloseKey(&self, hmdhandle: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.CloseKey)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(hmdhandle)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn ChangePermissions(&self, hmdhandle: u32, dwmdtimeout: u32, dwmdaccessrequested: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ChangePermissions)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(hmdhandle), ::core::mem::transmute(dwmdtimeout), ::core::mem::transmute(dwmdaccessrequested)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn SaveData(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SaveData)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn GetHandleInfo(&self, hmdhandle: u32) -> ::windows::core::Result<METADATA_HANDLE_INFO> {
        let mut result__ = ::core::mem::MaybeUninit::<METADATA_HANDLE_INFO>::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetHandleInfo)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(hmdhandle), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<METADATA_HANDLE_INFO>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn GetSystemChangeNumber(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetSystemChangeNumber)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn GetDataSetNumber<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, hmdhandle: u32, pszmdpath: Param1) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetDataSetNumber)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(hmdhandle), pszmdpath.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLastChangeTime<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, hmdhandle: u32, pszmdpath: Param1, pftmdlastchangetime: *const super::super::Foundation::FILETIME, blocaltime: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetLastChangeTime)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(hmdhandle), pszmdpath.into_param().abi(), ::core::mem::transmute(pftmdlastchangetime), blocaltime.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLastChangeTime<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, hmdhandle: u32, pszmdpath: Param1, pftmdlastchangetime: *mut super::super::Foundation::FILETIME, blocaltime: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetLastChangeTime)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(hmdhandle), pszmdpath.into_param().abi(), ::core::mem::transmute(pftmdlastchangetime), blocaltime.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn KeyExchangePhase1(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.KeyExchangePhase1)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn KeyExchangePhase2(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.KeyExchangePhase2)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn Backup<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pszmdbackuplocation: Param0, dwmdversion: u32, dwmdflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Backup)(::windows::core::Interface::as_raw(self), pszmdbackuplocation.into_param().abi(), ::core::mem::transmute(dwmdversion), ::core::mem::transmute(dwmdflags)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn Restore<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pszmdbackuplocation: Param0, dwmdversion: u32, dwmdflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Restore)(::windows::core::Interface::as_raw(self), pszmdbackuplocation.into_param().abi(), ::core::mem::transmute(dwmdversion), ::core::mem::transmute(dwmdflags)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumBackups(&self, pszmdbackuplocation: &mut [u16; 256], pdwmdversion: *mut u32, pftmdbackuptime: *mut super::super::Foundation::FILETIME, dwmdenumindex: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.EnumBackups)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pszmdbackuplocation)), ::core::mem::transmute(pdwmdversion), ::core::mem::transmute(pftmdbackuptime), ::core::mem::transmute(dwmdenumindex)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn DeleteBackup<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pszmdbackuplocation: Param0, dwmdversion: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.DeleteBackup)(::windows::core::Interface::as_raw(self), pszmdbackuplocation.into_param().abi(), ::core::mem::transmute(dwmdversion)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn UnmarshalInterface(&self) -> ::windows::core::Result<IMSAdminBaseW> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows::core::Interface::vtable(self).base__.UnmarshalInterface)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSAdminBaseW>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn GetServerGuid(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetServerGuid)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn BackupWithPasswd<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pszmdbackuplocation: Param0, dwmdversion: u32, dwmdflags: u32, pszpasswd: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BackupWithPasswd)(::windows::core::Interface::as_raw(self), pszmdbackuplocation.into_param().abi(), ::core::mem::transmute(dwmdversion), ::core::mem::transmute(dwmdflags), pszpasswd.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn RestoreWithPasswd<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pszmdbackuplocation: Param0, dwmdversion: u32, dwmdflags: u32, pszpasswd: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RestoreWithPasswd)(::windows::core::Interface::as_raw(self), pszmdbackuplocation.into_param().abi(), ::core::mem::transmute(dwmdversion), ::core::mem::transmute(dwmdflags), pszpasswd.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn Export<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pszpasswd: Param0, pszfilename: Param1, pszsourcepath: Param2, dwmdflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Export)(::windows::core::Interface::as_raw(self), pszpasswd.into_param().abi(), pszfilename.into_param().abi(), pszsourcepath.into_param().abi(), ::core::mem::transmute(dwmdflags)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn Import<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pszpasswd: Param0, pszfilename: Param1, pszsourcepath: Param2, pszdestpath: Param3, dwmdflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Import)(::windows::core::Interface::as_raw(self), pszpasswd.into_param().abi(), pszfilename.into_param().abi(), pszsourcepath.into_param().abi(), pszdestpath.into_param().abi(), ::core::mem::transmute(dwmdflags)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn RestoreHistory<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pszmdhistorylocation: Param0, dwmdmajorversion: u32, dwmdminorversion: u32, dwmdflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RestoreHistory)(::windows::core::Interface::as_raw(self), pszmdhistorylocation.into_param().abi(), ::core::mem::transmute(dwmdmajorversion), ::core::mem::transmute(dwmdminorversion), ::core::mem::transmute(dwmdflags)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumHistory(&self, pszmdhistorylocation: &mut [u16; 256], pdwmdmajorversion: *mut u32, pdwmdminorversion: *mut u32, pftmdhistorytime: *mut super::super::Foundation::FILETIME, dwmdenumindex: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnumHistory)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pszmdhistorylocation)), ::core::mem::transmute(pdwmdmajorversion), ::core::mem::transmute(pdwmdminorversion), ::core::mem::transmute(pftmdhistorytime), ::core::mem::transmute(dwmdenumindex)).ok()
    }
}
impl ::core::convert::From<IMSAdminBase2W> for ::windows::core::IUnknown {
    fn from(value: IMSAdminBase2W) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMSAdminBase2W> for ::windows::core::IUnknown {
    fn from(value: &IMSAdminBase2W) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMSAdminBase2W {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMSAdminBase2W {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IMSAdminBase2W> for IMSAdminBaseW {
    fn from(value: IMSAdminBase2W) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMSAdminBase2W> for IMSAdminBaseW {
    fn from(value: &IMSAdminBase2W) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMSAdminBaseW> for IMSAdminBase2W {
    fn into_param(self) -> ::windows::core::Param<'a, IMSAdminBaseW> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMSAdminBaseW> for &'a IMSAdminBase2W {
    fn into_param(self) -> ::windows::core::Param<'a, IMSAdminBaseW> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMSAdminBase2W {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMSAdminBase2W {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMSAdminBase2W {}
impl ::core::fmt::Debug for IMSAdminBase2W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSAdminBase2W").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMSAdminBase2W {
    type Vtable = IMSAdminBase2W_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8298d101_f992_43b7_8eca_5052d885b995);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSAdminBase2W_Vtbl {
    pub base__: IMSAdminBaseW_Vtbl,
    pub BackupWithPasswd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszmdbackuplocation: ::windows::core::PCWSTR, dwmdversion: u32, dwmdflags: u32, pszpasswd: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub RestoreWithPasswd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszmdbackuplocation: ::windows::core::PCWSTR, dwmdversion: u32, dwmdflags: u32, pszpasswd: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub Export: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpasswd: ::windows::core::PCWSTR, pszfilename: ::windows::core::PCWSTR, pszsourcepath: ::windows::core::PCWSTR, dwmdflags: u32) -> ::windows::core::HRESULT,
    pub Import: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpasswd: ::windows::core::PCWSTR, pszfilename: ::windows::core::PCWSTR, pszsourcepath: ::windows::core::PCWSTR, pszdestpath: ::windows::core::PCWSTR, dwmdflags: u32) -> ::windows::core::HRESULT,
    pub RestoreHistory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszmdhistorylocation: ::windows::core::PCWSTR, dwmdmajorversion: u32, dwmdminorversion: u32, dwmdflags: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub EnumHistory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszmdhistorylocation: ::windows::core::PWSTR, pdwmdmajorversion: *mut u32, pdwmdminorversion: *mut u32, pftmdhistorytime: *mut super::super::Foundation::FILETIME, dwmdenumindex: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnumHistory: usize,
}
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
#[repr(transparent)]
pub struct IMSAdminBase3W(::windows::core::IUnknown);
impl IMSAdminBase3W {
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn AddKey<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, hmdhandle: u32, pszmdpath: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.AddKey)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(hmdhandle), pszmdpath.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn DeleteKey<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, hmdhandle: u32, pszmdpath: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.DeleteKey)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(hmdhandle), pszmdpath.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn DeleteChildKeys<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, hmdhandle: u32, pszmdpath: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.DeleteChildKeys)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(hmdhandle), pszmdpath.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn EnumKeys<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, hmdhandle: u32, pszmdpath: Param1, pszmdname: &mut [u16; 256], dwmdenumobjectindex: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.EnumKeys)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(hmdhandle), pszmdpath.into_param().abi(), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pszmdname)), ::core::mem::transmute(dwmdenumobjectindex)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CopyKey<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, hmdsourcehandle: u32, pszmdsourcepath: Param1, hmddesthandle: u32, pszmddestpath: Param3, bmdoverwriteflag: Param4, bmdcopyflag: Param5) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.CopyKey)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(hmdsourcehandle), pszmdsourcepath.into_param().abi(), ::core::mem::transmute(hmddesthandle), pszmddestpath.into_param().abi(), bmdoverwriteflag.into_param().abi(), bmdcopyflag.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn RenameKey<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, hmdhandle: u32, pszmdpath: Param1, pszmdnewname: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.RenameKey)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(hmdhandle), pszmdpath.into_param().abi(), pszmdnewname.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn SetData<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, hmdhandle: u32, pszmdpath: Param1, pmdrmddata: *mut METADATA_RECORD) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(hmdhandle), pszmdpath.into_param().abi(), ::core::mem::transmute(pmdrmddata)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn GetData<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, hmdhandle: u32, pszmdpath: Param1, pmdrmddata: *mut METADATA_RECORD, pdwmdrequireddatalen: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(hmdhandle), pszmdpath.into_param().abi(), ::core::mem::transmute(pmdrmddata), ::core::mem::transmute(pdwmdrequireddatalen)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn DeleteData<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, hmdhandle: u32, pszmdpath: Param1, dwmdidentifier: u32, dwmddatatype: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.DeleteData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(hmdhandle), pszmdpath.into_param().abi(), ::core::mem::transmute(dwmdidentifier), ::core::mem::transmute(dwmddatatype)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn EnumData<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, hmdhandle: u32, pszmdpath: Param1, pmdrmddata: *mut METADATA_RECORD, dwmdenumdataindex: u32, pdwmdrequireddatalen: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.EnumData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(hmdhandle), pszmdpath.into_param().abi(), ::core::mem::transmute(pmdrmddata), ::core::mem::transmute(dwmdenumdataindex), ::core::mem::transmute(pdwmdrequireddatalen)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn GetAllData<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, hmdhandle: u32, pszmdpath: Param1, dwmdattributes: u32, dwmdusertype: u32, dwmddatatype: u32, pdwmdnumdataentries: *mut u32, pdwmddatasetnumber: *mut u32, dwmdbuffersize: u32, pbmdbuffer: *mut u8, pdwmdrequiredbuffersize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetAllData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(hmdhandle), pszmdpath.into_param().abi(), ::core::mem::transmute(dwmdattributes), ::core::mem::transmute(dwmdusertype), ::core::mem::transmute(dwmddatatype), ::core::mem::transmute(pdwmdnumdataentries), ::core::mem::transmute(pdwmddatasetnumber), ::core::mem::transmute(dwmdbuffersize), ::core::mem::transmute(pbmdbuffer), ::core::mem::transmute(pdwmdrequiredbuffersize)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn DeleteAllData<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, hmdhandle: u32, pszmdpath: Param1, dwmdusertype: u32, dwmddatatype: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.DeleteAllData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(hmdhandle), pszmdpath.into_param().abi(), ::core::mem::transmute(dwmdusertype), ::core::mem::transmute(dwmddatatype)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CopyData<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param7: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, hmdsourcehandle: u32, pszmdsourcepath: Param1, hmddesthandle: u32, pszmddestpath: Param3, dwmdattributes: u32, dwmdusertype: u32, dwmddatatype: u32, bmdcopyflag: Param7) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.CopyData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(hmdsourcehandle), pszmdsourcepath.into_param().abi(), ::core::mem::transmute(hmddesthandle), pszmddestpath.into_param().abi(), ::core::mem::transmute(dwmdattributes), ::core::mem::transmute(dwmdusertype), ::core::mem::transmute(dwmddatatype), bmdcopyflag.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn GetDataPaths<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, hmdhandle: u32, pszmdpath: Param1, dwmdidentifier: u32, dwmddatatype: u32, pszbuffer: &mut [u16], pdwmdrequiredbuffersize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetDataPaths)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(hmdhandle), pszmdpath.into_param().abi(), ::core::mem::transmute(dwmdidentifier), ::core::mem::transmute(dwmddatatype), pszbuffer.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pszbuffer)), ::core::mem::transmute(pdwmdrequiredbuffersize)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn OpenKey<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, hmdhandle: u32, pszmdpath: Param1, dwmdaccessrequested: u32, dwmdtimeout: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.OpenKey)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(hmdhandle), pszmdpath.into_param().abi(), ::core::mem::transmute(dwmdaccessrequested), ::core::mem::transmute(dwmdtimeout), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn CloseKey(&self, hmdhandle: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.CloseKey)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(hmdhandle)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn ChangePermissions(&self, hmdhandle: u32, dwmdtimeout: u32, dwmdaccessrequested: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.ChangePermissions)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(hmdhandle), ::core::mem::transmute(dwmdtimeout), ::core::mem::transmute(dwmdaccessrequested)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn SaveData(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SaveData)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn GetHandleInfo(&self, hmdhandle: u32) -> ::windows::core::Result<METADATA_HANDLE_INFO> {
        let mut result__ = ::core::mem::MaybeUninit::<METADATA_HANDLE_INFO>::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.GetHandleInfo)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(hmdhandle), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<METADATA_HANDLE_INFO>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn GetSystemChangeNumber(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.GetSystemChangeNumber)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn GetDataSetNumber<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, hmdhandle: u32, pszmdpath: Param1) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.GetDataSetNumber)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(hmdhandle), pszmdpath.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLastChangeTime<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, hmdhandle: u32, pszmdpath: Param1, pftmdlastchangetime: *const super::super::Foundation::FILETIME, blocaltime: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetLastChangeTime)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(hmdhandle), pszmdpath.into_param().abi(), ::core::mem::transmute(pftmdlastchangetime), blocaltime.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLastChangeTime<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, hmdhandle: u32, pszmdpath: Param1, pftmdlastchangetime: *mut super::super::Foundation::FILETIME, blocaltime: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetLastChangeTime)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(hmdhandle), pszmdpath.into_param().abi(), ::core::mem::transmute(pftmdlastchangetime), blocaltime.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn KeyExchangePhase1(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.KeyExchangePhase1)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn KeyExchangePhase2(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.KeyExchangePhase2)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn Backup<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pszmdbackuplocation: Param0, dwmdversion: u32, dwmdflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.Backup)(::windows::core::Interface::as_raw(self), pszmdbackuplocation.into_param().abi(), ::core::mem::transmute(dwmdversion), ::core::mem::transmute(dwmdflags)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn Restore<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pszmdbackuplocation: Param0, dwmdversion: u32, dwmdflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.Restore)(::windows::core::Interface::as_raw(self), pszmdbackuplocation.into_param().abi(), ::core::mem::transmute(dwmdversion), ::core::mem::transmute(dwmdflags)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumBackups(&self, pszmdbackuplocation: &mut [u16; 256], pdwmdversion: *mut u32, pftmdbackuptime: *mut super::super::Foundation::FILETIME, dwmdenumindex: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.EnumBackups)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pszmdbackuplocation)), ::core::mem::transmute(pdwmdversion), ::core::mem::transmute(pftmdbackuptime), ::core::mem::transmute(dwmdenumindex)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn DeleteBackup<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pszmdbackuplocation: Param0, dwmdversion: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.DeleteBackup)(::windows::core::Interface::as_raw(self), pszmdbackuplocation.into_param().abi(), ::core::mem::transmute(dwmdversion)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn UnmarshalInterface(&self) -> ::windows::core::Result<IMSAdminBaseW> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.UnmarshalInterface)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSAdminBaseW>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn GetServerGuid(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetServerGuid)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn BackupWithPasswd<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pszmdbackuplocation: Param0, dwmdversion: u32, dwmdflags: u32, pszpasswd: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.BackupWithPasswd)(::windows::core::Interface::as_raw(self), pszmdbackuplocation.into_param().abi(), ::core::mem::transmute(dwmdversion), ::core::mem::transmute(dwmdflags), pszpasswd.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn RestoreWithPasswd<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pszmdbackuplocation: Param0, dwmdversion: u32, dwmdflags: u32, pszpasswd: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.RestoreWithPasswd)(::windows::core::Interface::as_raw(self), pszmdbackuplocation.into_param().abi(), ::core::mem::transmute(dwmdversion), ::core::mem::transmute(dwmdflags), pszpasswd.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn Export<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pszpasswd: Param0, pszfilename: Param1, pszsourcepath: Param2, dwmdflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Export)(::windows::core::Interface::as_raw(self), pszpasswd.into_param().abi(), pszfilename.into_param().abi(), pszsourcepath.into_param().abi(), ::core::mem::transmute(dwmdflags)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn Import<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pszpasswd: Param0, pszfilename: Param1, pszsourcepath: Param2, pszdestpath: Param3, dwmdflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Import)(::windows::core::Interface::as_raw(self), pszpasswd.into_param().abi(), pszfilename.into_param().abi(), pszsourcepath.into_param().abi(), pszdestpath.into_param().abi(), ::core::mem::transmute(dwmdflags)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn RestoreHistory<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pszmdhistorylocation: Param0, dwmdmajorversion: u32, dwmdminorversion: u32, dwmdflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.RestoreHistory)(::windows::core::Interface::as_raw(self), pszmdhistorylocation.into_param().abi(), ::core::mem::transmute(dwmdmajorversion), ::core::mem::transmute(dwmdminorversion), ::core::mem::transmute(dwmdflags)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumHistory(&self, pszmdhistorylocation: &mut [u16; 256], pdwmdmajorversion: *mut u32, pdwmdminorversion: *mut u32, pftmdhistorytime: *mut super::super::Foundation::FILETIME, dwmdenumindex: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.EnumHistory)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pszmdhistorylocation)), ::core::mem::transmute(pdwmdmajorversion), ::core::mem::transmute(pdwmdminorversion), ::core::mem::transmute(pftmdhistorytime), ::core::mem::transmute(dwmdenumindex)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn GetChildPaths<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, hmdhandle: u32, pszmdpath: Param1, pszbuffer: &mut [u16], pcchmdrequiredbuffersize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetChildPaths)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(hmdhandle), pszmdpath.into_param().abi(), pszbuffer.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pszbuffer)), ::core::mem::transmute(pcchmdrequiredbuffersize)).ok()
    }
}
impl ::core::convert::From<IMSAdminBase3W> for ::windows::core::IUnknown {
    fn from(value: IMSAdminBase3W) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMSAdminBase3W> for ::windows::core::IUnknown {
    fn from(value: &IMSAdminBase3W) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMSAdminBase3W {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMSAdminBase3W {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IMSAdminBase3W> for IMSAdminBaseW {
    fn from(value: IMSAdminBase3W) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMSAdminBase3W> for IMSAdminBaseW {
    fn from(value: &IMSAdminBase3W) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMSAdminBaseW> for IMSAdminBase3W {
    fn into_param(self) -> ::windows::core::Param<'a, IMSAdminBaseW> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMSAdminBaseW> for &'a IMSAdminBase3W {
    fn into_param(self) -> ::windows::core::Param<'a, IMSAdminBaseW> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IMSAdminBase3W> for IMSAdminBase2W {
    fn from(value: IMSAdminBase3W) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMSAdminBase3W> for IMSAdminBase2W {
    fn from(value: &IMSAdminBase3W) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMSAdminBase2W> for IMSAdminBase3W {
    fn into_param(self) -> ::windows::core::Param<'a, IMSAdminBase2W> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMSAdminBase2W> for &'a IMSAdminBase3W {
    fn into_param(self) -> ::windows::core::Param<'a, IMSAdminBase2W> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMSAdminBase3W {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMSAdminBase3W {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMSAdminBase3W {}
impl ::core::fmt::Debug for IMSAdminBase3W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSAdminBase3W").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMSAdminBase3W {
    type Vtable = IMSAdminBase3W_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf612954d_3b0b_4c56_9563_227b7be624b4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSAdminBase3W_Vtbl {
    pub base__: IMSAdminBase2W_Vtbl,
    pub GetChildPaths: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hmdhandle: u32, pszmdpath: ::windows::core::PCWSTR, cchmdbuffersize: u32, pszbuffer: ::windows::core::PWSTR, pcchmdrequiredbuffersize: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
#[repr(transparent)]
pub struct IMSAdminBaseSinkW(::windows::core::IUnknown);
impl IMSAdminBaseSinkW {
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn SinkNotify(&self, pcochangelist: &[MD_CHANGE_OBJECT_W]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SinkNotify)(::windows::core::Interface::as_raw(self), pcochangelist.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(pcochangelist))).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn ShutdownNotify(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ShutdownNotify)(::windows::core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IMSAdminBaseSinkW> for ::windows::core::IUnknown {
    fn from(value: IMSAdminBaseSinkW) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMSAdminBaseSinkW> for ::windows::core::IUnknown {
    fn from(value: &IMSAdminBaseSinkW) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMSAdminBaseSinkW {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMSAdminBaseSinkW {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMSAdminBaseSinkW {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMSAdminBaseSinkW {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMSAdminBaseSinkW {}
impl ::core::fmt::Debug for IMSAdminBaseSinkW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSAdminBaseSinkW").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMSAdminBaseSinkW {
    type Vtable = IMSAdminBaseSinkW_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa9e69612_b80d_11d0_b9b9_00a0c922e750);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSAdminBaseSinkW_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub SinkNotify: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwmdnumelements: u32, pcochangelist: *const MD_CHANGE_OBJECT_W) -> ::windows::core::HRESULT,
    pub ShutdownNotify: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
#[repr(transparent)]
pub struct IMSAdminBaseW(::windows::core::IUnknown);
impl IMSAdminBaseW {
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn AddKey<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, hmdhandle: u32, pszmdpath: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddKey)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(hmdhandle), pszmdpath.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn DeleteKey<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, hmdhandle: u32, pszmdpath: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DeleteKey)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(hmdhandle), pszmdpath.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn DeleteChildKeys<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, hmdhandle: u32, pszmdpath: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DeleteChildKeys)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(hmdhandle), pszmdpath.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn EnumKeys<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, hmdhandle: u32, pszmdpath: Param1, pszmdname: &mut [u16; 256], dwmdenumobjectindex: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnumKeys)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(hmdhandle), pszmdpath.into_param().abi(), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pszmdname)), ::core::mem::transmute(dwmdenumobjectindex)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CopyKey<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, hmdsourcehandle: u32, pszmdsourcepath: Param1, hmddesthandle: u32, pszmddestpath: Param3, bmdoverwriteflag: Param4, bmdcopyflag: Param5) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CopyKey)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(hmdsourcehandle), pszmdsourcepath.into_param().abi(), ::core::mem::transmute(hmddesthandle), pszmddestpath.into_param().abi(), bmdoverwriteflag.into_param().abi(), bmdcopyflag.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn RenameKey<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, hmdhandle: u32, pszmdpath: Param1, pszmdnewname: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RenameKey)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(hmdhandle), pszmdpath.into_param().abi(), pszmdnewname.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn SetData<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, hmdhandle: u32, pszmdpath: Param1, pmdrmddata: *mut METADATA_RECORD) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(hmdhandle), pszmdpath.into_param().abi(), ::core::mem::transmute(pmdrmddata)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn GetData<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, hmdhandle: u32, pszmdpath: Param1, pmdrmddata: *mut METADATA_RECORD, pdwmdrequireddatalen: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(hmdhandle), pszmdpath.into_param().abi(), ::core::mem::transmute(pmdrmddata), ::core::mem::transmute(pdwmdrequireddatalen)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn DeleteData<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, hmdhandle: u32, pszmdpath: Param1, dwmdidentifier: u32, dwmddatatype: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DeleteData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(hmdhandle), pszmdpath.into_param().abi(), ::core::mem::transmute(dwmdidentifier), ::core::mem::transmute(dwmddatatype)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn EnumData<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, hmdhandle: u32, pszmdpath: Param1, pmdrmddata: *mut METADATA_RECORD, dwmdenumdataindex: u32, pdwmdrequireddatalen: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnumData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(hmdhandle), pszmdpath.into_param().abi(), ::core::mem::transmute(pmdrmddata), ::core::mem::transmute(dwmdenumdataindex), ::core::mem::transmute(pdwmdrequireddatalen)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn GetAllData<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, hmdhandle: u32, pszmdpath: Param1, dwmdattributes: u32, dwmdusertype: u32, dwmddatatype: u32, pdwmdnumdataentries: *mut u32, pdwmddatasetnumber: *mut u32, dwmdbuffersize: u32, pbmdbuffer: *mut u8, pdwmdrequiredbuffersize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetAllData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(hmdhandle), pszmdpath.into_param().abi(), ::core::mem::transmute(dwmdattributes), ::core::mem::transmute(dwmdusertype), ::core::mem::transmute(dwmddatatype), ::core::mem::transmute(pdwmdnumdataentries), ::core::mem::transmute(pdwmddatasetnumber), ::core::mem::transmute(dwmdbuffersize), ::core::mem::transmute(pbmdbuffer), ::core::mem::transmute(pdwmdrequiredbuffersize)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn DeleteAllData<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, hmdhandle: u32, pszmdpath: Param1, dwmdusertype: u32, dwmddatatype: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DeleteAllData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(hmdhandle), pszmdpath.into_param().abi(), ::core::mem::transmute(dwmdusertype), ::core::mem::transmute(dwmddatatype)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CopyData<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param7: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, hmdsourcehandle: u32, pszmdsourcepath: Param1, hmddesthandle: u32, pszmddestpath: Param3, dwmdattributes: u32, dwmdusertype: u32, dwmddatatype: u32, bmdcopyflag: Param7) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CopyData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(hmdsourcehandle), pszmdsourcepath.into_param().abi(), ::core::mem::transmute(hmddesthandle), pszmddestpath.into_param().abi(), ::core::mem::transmute(dwmdattributes), ::core::mem::transmute(dwmdusertype), ::core::mem::transmute(dwmddatatype), bmdcopyflag.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn GetDataPaths<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, hmdhandle: u32, pszmdpath: Param1, dwmdidentifier: u32, dwmddatatype: u32, pszbuffer: &mut [u16], pdwmdrequiredbuffersize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDataPaths)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(hmdhandle), pszmdpath.into_param().abi(), ::core::mem::transmute(dwmdidentifier), ::core::mem::transmute(dwmddatatype), pszbuffer.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pszbuffer)), ::core::mem::transmute(pdwmdrequiredbuffersize)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn OpenKey<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, hmdhandle: u32, pszmdpath: Param1, dwmdaccessrequested: u32, dwmdtimeout: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows::core::Interface::vtable(self).OpenKey)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(hmdhandle), pszmdpath.into_param().abi(), ::core::mem::transmute(dwmdaccessrequested), ::core::mem::transmute(dwmdtimeout), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn CloseKey(&self, hmdhandle: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CloseKey)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(hmdhandle)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn ChangePermissions(&self, hmdhandle: u32, dwmdtimeout: u32, dwmdaccessrequested: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ChangePermissions)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(hmdhandle), ::core::mem::transmute(dwmdtimeout), ::core::mem::transmute(dwmdaccessrequested)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn SaveData(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SaveData)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn GetHandleInfo(&self, hmdhandle: u32) -> ::windows::core::Result<METADATA_HANDLE_INFO> {
        let mut result__ = ::core::mem::MaybeUninit::<METADATA_HANDLE_INFO>::zeroed();
        (::windows::core::Interface::vtable(self).GetHandleInfo)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(hmdhandle), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<METADATA_HANDLE_INFO>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn GetSystemChangeNumber(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows::core::Interface::vtable(self).GetSystemChangeNumber)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn GetDataSetNumber<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, hmdhandle: u32, pszmdpath: Param1) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        (::windows::core::Interface::vtable(self).GetDataSetNumber)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(hmdhandle), pszmdpath.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLastChangeTime<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, hmdhandle: u32, pszmdpath: Param1, pftmdlastchangetime: *const super::super::Foundation::FILETIME, blocaltime: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetLastChangeTime)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(hmdhandle), pszmdpath.into_param().abi(), ::core::mem::transmute(pftmdlastchangetime), blocaltime.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLastChangeTime<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, hmdhandle: u32, pszmdpath: Param1, pftmdlastchangetime: *mut super::super::Foundation::FILETIME, blocaltime: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetLastChangeTime)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(hmdhandle), pszmdpath.into_param().abi(), ::core::mem::transmute(pftmdlastchangetime), blocaltime.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn KeyExchangePhase1(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).KeyExchangePhase1)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn KeyExchangePhase2(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).KeyExchangePhase2)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn Backup<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pszmdbackuplocation: Param0, dwmdversion: u32, dwmdflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Backup)(::windows::core::Interface::as_raw(self), pszmdbackuplocation.into_param().abi(), ::core::mem::transmute(dwmdversion), ::core::mem::transmute(dwmdflags)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn Restore<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pszmdbackuplocation: Param0, dwmdversion: u32, dwmdflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Restore)(::windows::core::Interface::as_raw(self), pszmdbackuplocation.into_param().abi(), ::core::mem::transmute(dwmdversion), ::core::mem::transmute(dwmdflags)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumBackups(&self, pszmdbackuplocation: &mut [u16; 256], pdwmdversion: *mut u32, pftmdbackuptime: *mut super::super::Foundation::FILETIME, dwmdenumindex: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnumBackups)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pszmdbackuplocation)), ::core::mem::transmute(pdwmdversion), ::core::mem::transmute(pftmdbackuptime), ::core::mem::transmute(dwmdenumindex)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn DeleteBackup<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pszmdbackuplocation: Param0, dwmdversion: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DeleteBackup)(::windows::core::Interface::as_raw(self), pszmdbackuplocation.into_param().abi(), ::core::mem::transmute(dwmdversion)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn UnmarshalInterface(&self) -> ::windows::core::Result<IMSAdminBaseW> {
        let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
        (::windows::core::Interface::vtable(self).UnmarshalInterface)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMSAdminBaseW>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn GetServerGuid(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetServerGuid)(::windows::core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IMSAdminBaseW> for ::windows::core::IUnknown {
    fn from(value: IMSAdminBaseW) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMSAdminBaseW> for ::windows::core::IUnknown {
    fn from(value: &IMSAdminBaseW) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMSAdminBaseW {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMSAdminBaseW {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMSAdminBaseW {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMSAdminBaseW {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMSAdminBaseW {}
impl ::core::fmt::Debug for IMSAdminBaseW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSAdminBaseW").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMSAdminBaseW {
    type Vtable = IMSAdminBaseW_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x70b51430_b6ca_11d0_b9b9_00a0c922e750);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSAdminBaseW_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub AddKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hmdhandle: u32, pszmdpath: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub DeleteKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hmdhandle: u32, pszmdpath: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub DeleteChildKeys: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hmdhandle: u32, pszmdpath: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub EnumKeys: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hmdhandle: u32, pszmdpath: ::windows::core::PCWSTR, pszmdname: ::windows::core::PWSTR, dwmdenumobjectindex: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CopyKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hmdsourcehandle: u32, pszmdsourcepath: ::windows::core::PCWSTR, hmddesthandle: u32, pszmddestpath: ::windows::core::PCWSTR, bmdoverwriteflag: super::super::Foundation::BOOL, bmdcopyflag: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CopyKey: usize,
    pub RenameKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hmdhandle: u32, pszmdpath: ::windows::core::PCWSTR, pszmdnewname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub SetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hmdhandle: u32, pszmdpath: ::windows::core::PCWSTR, pmdrmddata: *mut METADATA_RECORD) -> ::windows::core::HRESULT,
    pub GetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hmdhandle: u32, pszmdpath: ::windows::core::PCWSTR, pmdrmddata: *mut METADATA_RECORD, pdwmdrequireddatalen: *mut u32) -> ::windows::core::HRESULT,
    pub DeleteData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hmdhandle: u32, pszmdpath: ::windows::core::PCWSTR, dwmdidentifier: u32, dwmddatatype: u32) -> ::windows::core::HRESULT,
    pub EnumData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hmdhandle: u32, pszmdpath: ::windows::core::PCWSTR, pmdrmddata: *mut METADATA_RECORD, dwmdenumdataindex: u32, pdwmdrequireddatalen: *mut u32) -> ::windows::core::HRESULT,
    pub GetAllData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hmdhandle: u32, pszmdpath: ::windows::core::PCWSTR, dwmdattributes: u32, dwmdusertype: u32, dwmddatatype: u32, pdwmdnumdataentries: *mut u32, pdwmddatasetnumber: *mut u32, dwmdbuffersize: u32, pbmdbuffer: *mut u8, pdwmdrequiredbuffersize: *mut u32) -> ::windows::core::HRESULT,
    pub DeleteAllData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hmdhandle: u32, pszmdpath: ::windows::core::PCWSTR, dwmdusertype: u32, dwmddatatype: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CopyData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hmdsourcehandle: u32, pszmdsourcepath: ::windows::core::PCWSTR, hmddesthandle: u32, pszmddestpath: ::windows::core::PCWSTR, dwmdattributes: u32, dwmdusertype: u32, dwmddatatype: u32, bmdcopyflag: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CopyData: usize,
    pub GetDataPaths: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hmdhandle: u32, pszmdpath: ::windows::core::PCWSTR, dwmdidentifier: u32, dwmddatatype: u32, dwmdbuffersize: u32, pszbuffer: ::windows::core::PWSTR, pdwmdrequiredbuffersize: *mut u32) -> ::windows::core::HRESULT,
    pub OpenKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hmdhandle: u32, pszmdpath: ::windows::core::PCWSTR, dwmdaccessrequested: u32, dwmdtimeout: u32, phmdnewhandle: *mut u32) -> ::windows::core::HRESULT,
    pub CloseKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hmdhandle: u32) -> ::windows::core::HRESULT,
    pub ChangePermissions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hmdhandle: u32, dwmdtimeout: u32, dwmdaccessrequested: u32) -> ::windows::core::HRESULT,
    pub SaveData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetHandleInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hmdhandle: u32, pmdhiinfo: *mut METADATA_HANDLE_INFO) -> ::windows::core::HRESULT,
    pub GetSystemChangeNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwsystemchangenumber: *mut u32) -> ::windows::core::HRESULT,
    pub GetDataSetNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hmdhandle: u32, pszmdpath: ::windows::core::PCWSTR, pdwmddatasetnumber: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetLastChangeTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hmdhandle: u32, pszmdpath: ::windows::core::PCWSTR, pftmdlastchangetime: *const super::super::Foundation::FILETIME, blocaltime: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetLastChangeTime: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetLastChangeTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hmdhandle: u32, pszmdpath: ::windows::core::PCWSTR, pftmdlastchangetime: *mut super::super::Foundation::FILETIME, blocaltime: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetLastChangeTime: usize,
    pub KeyExchangePhase1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub KeyExchangePhase2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Backup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszmdbackuplocation: ::windows::core::PCWSTR, dwmdversion: u32, dwmdflags: u32) -> ::windows::core::HRESULT,
    pub Restore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszmdbackuplocation: ::windows::core::PCWSTR, dwmdversion: u32, dwmdflags: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub EnumBackups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszmdbackuplocation: ::windows::core::PWSTR, pdwmdversion: *mut u32, pftmdbackuptime: *mut super::super::Foundation::FILETIME, dwmdenumindex: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnumBackups: usize,
    pub DeleteBackup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszmdbackuplocation: ::windows::core::PCWSTR, dwmdversion: u32) -> ::windows::core::HRESULT,
    pub UnmarshalInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piadmbwinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetServerGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
#[repr(transparent)]
pub struct IMSImpExpHelpW(::windows::core::IUnknown);
impl IMSImpExpHelpW {
    #[doc = "*Required features: `\"Win32_System_Iis\"`*"]
    pub unsafe fn EnumeratePathsInFile<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pszfilename: Param0, pszkeytype: Param1, pszbuffer: &mut [u16], pdwmdrequiredbuffersize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnumeratePathsInFile)(::windows::core::Interface::as_raw(self), pszfilename.into_param().abi(), pszkeytype.into_param().abi(), pszbuffer.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pszbuffer)), ::core::mem::transmute(pdwmdrequiredbuffersize)).ok()
    }
}
impl ::core::convert::From<IMSImpExpHelpW> for ::windows::core::IUnknown {
    fn from(value: IMSImpExpHelpW) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMSImpExpHelpW> for ::windows::core::IUnknown {
    fn from(value: &IMSImpExpHelpW) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMSImpExpHelpW {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMSImpExpHelpW {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMSImpExpHelpW {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMSImpExpHelpW {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMSImpExpHelpW {}
impl ::core::fmt::Debug for IMSImpExpHelpW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMSImpExpHelpW").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMSImpExpHelpW {
    type Vtable = IMSImpExpHelpW_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x29ff67ff_8050_480f_9f30_cc41635f2f9d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSImpExpHelpW_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub EnumeratePathsInFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszfilename: ::windows::core::PCWSTR, pszkeytype: ::windows::core::PCWSTR, dwmdbuffersize: u32, pszbuffer: ::windows::core::PWSTR, pdwmdrequiredbuffersize: *mut u32) -> ::windows::core::HRESULT,
}
pub const LIBID_ASPTypeLibrary: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd97a6da0_a85c_11cf_83ae_00a0c90c2bd8);
pub const LIBID_IISRSTALib: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe8fb8614_588f_11d2_9d61_00c04f79c5fe);
pub const LIBID_WAMREGLib: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x29822aa8_f302_11d0_9953_00c04fd919c1);
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub struct LOGGING_PARAMETERS {
    pub pszSessionId: ::windows::core::PCWSTR,
    pub pszSiteName: ::windows::core::PCWSTR,
    pub pszUserName: ::windows::core::PCWSTR,
    pub pszHostName: ::windows::core::PCWSTR,
    pub pszRemoteIpAddress: ::windows::core::PCWSTR,
    pub dwRemoteIpPort: u32,
    pub pszLocalIpAddress: ::windows::core::PCWSTR,
    pub dwLocalIpPort: u32,
    pub BytesSent: u64,
    pub BytesReceived: u64,
    pub pszCommand: ::windows::core::PCWSTR,
    pub pszCommandParameters: ::windows::core::PCWSTR,
    pub pszFullPath: ::windows::core::PCWSTR,
    pub dwElapsedMilliseconds: u32,
    pub FtpStatus: u32,
    pub FtpSubStatus: u32,
    pub hrStatus: ::windows::core::HRESULT,
    pub pszInformation: ::windows::core::PCWSTR,
}
impl ::core::marker::Copy for LOGGING_PARAMETERS {}
impl ::core::clone::Clone for LOGGING_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for LOGGING_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LOGGING_PARAMETERS")
            .field("pszSessionId", &self.pszSessionId)
            .field("pszSiteName", &self.pszSiteName)
            .field("pszUserName", &self.pszUserName)
            .field("pszHostName", &self.pszHostName)
            .field("pszRemoteIpAddress", &self.pszRemoteIpAddress)
            .field("dwRemoteIpPort", &self.dwRemoteIpPort)
            .field("pszLocalIpAddress", &self.pszLocalIpAddress)
            .field("dwLocalIpPort", &self.dwLocalIpPort)
            .field("BytesSent", &self.BytesSent)
            .field("BytesReceived", &self.BytesReceived)
            .field("pszCommand", &self.pszCommand)
            .field("pszCommandParameters", &self.pszCommandParameters)
            .field("pszFullPath", &self.pszFullPath)
            .field("dwElapsedMilliseconds", &self.dwElapsedMilliseconds)
            .field("FtpStatus", &self.FtpStatus)
            .field("FtpSubStatus", &self.FtpSubStatus)
            .field("hrStatus", &self.hrStatus)
            .field("pszInformation", &self.pszInformation)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for LOGGING_PARAMETERS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for LOGGING_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<LOGGING_PARAMETERS>()) == 0 }
    }
}
impl ::core::cmp::Eq for LOGGING_PARAMETERS {}
impl ::core::default::Default for LOGGING_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MB_DONT_IMPERSONATE: u32 = 9033u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ACCESS_EXECUTE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ACCESS_MAP_CERT: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ACCESS_MASK: u32 = 65535u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ACCESS_NEGO_CERT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ACCESS_NO_PHYSICAL_DIR: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ACCESS_NO_REMOTE_EXECUTE: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ACCESS_NO_REMOTE_READ: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ACCESS_NO_REMOTE_SCRIPT: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ACCESS_NO_REMOTE_WRITE: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ACCESS_PERM: u32 = 6016u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ACCESS_READ: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ACCESS_REQUIRE_CERT: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ACCESS_SCRIPT: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ACCESS_SOURCE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ACCESS_SSL: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ACCESS_SSL128: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ACCESS_WRITE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ACR_ENUM_KEYS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ACR_READ: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ACR_RESTRICTED_WRITE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ACR_UNSECURE_PROPS_READ: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ACR_WRITE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ACR_WRITE_DAC: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ADMIN_ACL: u32 = 6027u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ADMIN_INSTANCE: u32 = 2115u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ADV_CACHE_TTL: u32 = 2064u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ADV_NOTIFY_PWD_EXP_IN_DAYS: u32 = 2063u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_AD_CONNECTIONS_PASSWORD: u32 = 5015u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_AD_CONNECTIONS_USERNAME: u32 = 5014u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ALLOW_ANONYMOUS: u32 = 5005u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ALLOW_KEEPALIVES: u32 = 6038u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ALLOW_PATH_INFO_FOR_SCRIPT_MAPPINGS: u32 = 2095u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ALLOW_REPLACE_ON_RENAME: u32 = 5009u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ANONYMOUS_ONLY: u32 = 5006u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ANONYMOUS_PWD: u32 = 6021u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ANONYMOUS_USER_NAME: u32 = 6020u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ANONYMOUS_USE_SUBAUTH: u32 = 6022u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_APPPOOL_32_BIT_APP_ON_WIN64: u32 = 9040u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_APPPOOL_ALLOW_TRANSIENT_REGISTRATION: u32 = 9202u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_APPPOOL_APPPOOL_ID: u32 = 9201u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_APPPOOL_AUTO_SHUTDOWN_EXE: u32 = 9035u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_APPPOOL_AUTO_SHUTDOWN_PARAMS: u32 = 9036u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_APPPOOL_AUTO_START: u32 = 9028u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_APPPOOL_COMMAND: u32 = 9026u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_APPPOOL_COMMAND_START: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_APPPOOL_COMMAND_STOP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_APPPOOL_DISALLOW_OVERLAPPING_ROTATION: u32 = 9015u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_APPPOOL_DISALLOW_ROTATION_ON_CONFIG_CHANGE: u32 = 9018u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_APPPOOL_IDENTITY_TYPE: u32 = 9021u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_APPPOOL_IDENTITY_TYPE_LOCALSERVICE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_APPPOOL_IDENTITY_TYPE_LOCALSYSTEM: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_APPPOOL_IDENTITY_TYPE_NETWORKSERVICE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_APPPOOL_IDENTITY_TYPE_SPECIFICUSER: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_APPPOOL_IDLE_TIMEOUT: u32 = 9005u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_APPPOOL_MANAGED_PIPELINE_MODE: u32 = 9041u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_APPPOOL_MANAGED_RUNTIME_VERSION: u32 = 9039u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_APPPOOL_MAX_PROCESS_COUNT: u32 = 9003u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_APPPOOL_ORPHAN_ACTION_EXE: u32 = 9031u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_APPPOOL_ORPHAN_ACTION_PARAMS: u32 = 9032u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_APPPOOL_ORPHAN_PROCESSES_FOR_DEBUGGING: u32 = 9009u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_APPPOOL_PERIODIC_RESTART_CONNECTIONS: u32 = 9104u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_APPPOOL_PERIODIC_RESTART_MEMORY: u32 = 9024u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_APPPOOL_PERIODIC_RESTART_PRIVATE_MEMORY: u32 = 9038u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_APPPOOL_PERIODIC_RESTART_REQUEST_COUNT: u32 = 9002u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_APPPOOL_PERIODIC_RESTART_SCHEDULE: u32 = 9020u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_APPPOOL_PERIODIC_RESTART_TIME: u32 = 9001u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_APPPOOL_PINGING_ENABLED: u32 = 9004u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_APPPOOL_PING_INTERVAL: u32 = 9013u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_APPPOOL_PING_RESPONSE_TIMELIMIT: u32 = 9014u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_APPPOOL_RAPID_FAIL_PROTECTION_ENABLED: u32 = 9006u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_APPPOOL_SHUTDOWN_TIMELIMIT: u32 = 9012u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_APPPOOL_SMP_AFFINITIZED: u32 = 9007u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_APPPOOL_SMP_AFFINITIZED_PROCESSOR_MASK: u32 = 9008u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_APPPOOL_STARTUP_TIMELIMIT: u32 = 9011u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_APPPOOL_STATE: u32 = 9027u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_APPPOOL_STATE_STARTED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_APPPOOL_STATE_STARTING: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_APPPOOL_STATE_STOPPED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_APPPOOL_STATE_STOPPING: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_APPPOOL_UL_APPPOOL_QUEUE_LENGTH: u32 = 9017u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_APP_ALLOW_TRANSIENT_REGISTRATION: u32 = 9102u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_APP_APPPOOL_ID: u32 = 9101u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_APP_AUTO_START: u32 = 9103u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_APP_DEPENDENCIES: u32 = 2167u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_APP_FRIENDLY_NAME: u32 = 2102u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_APP_ISOLATED: u32 = 2104u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_APP_OOP_RECOVER_LIMIT: u32 = 2110u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_APP_PACKAGE_ID: u32 = 2106u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_APP_PACKAGE_NAME: u32 = 2107u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_APP_PERIODIC_RESTART_REQUESTS: u32 = 2112u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_APP_PERIODIC_RESTART_SCHEDULE: u32 = 2113u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_APP_PERIODIC_RESTART_TIME: u32 = 2111u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_APP_POOL_LOG_EVENT_ON_PROCESSMODEL: u32 = 9042u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_APP_POOL_LOG_EVENT_ON_RECYCLE: u32 = 9037u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_APP_POOL_PROCESSMODEL_IDLE_TIMEOUT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_APP_POOL_RECYCLE_CONFIG_CHANGE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_APP_POOL_RECYCLE_ISAPI_UNHEALTHY: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_APP_POOL_RECYCLE_MEMORY: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_APP_POOL_RECYCLE_ON_DEMAND: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_APP_POOL_RECYCLE_PRIVATE_MEMORY: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_APP_POOL_RECYCLE_REQUESTS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_APP_POOL_RECYCLE_SCHEDULE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_APP_POOL_RECYCLE_TIME: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_APP_ROOT: u32 = 2103u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_APP_SHUTDOWN_TIME_LIMIT: u32 = 2114u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_APP_TRACE_URL_LIST: u32 = 2118u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_APP_WAM_CLSID: u32 = 2105u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ASP_ALLOWOUTOFPROCCMPNTS: u32 = 7014u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ASP_ALLOWOUTOFPROCCOMPONENTS: u32 = 7014u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ASP_ALLOWSESSIONSTATE: u32 = 7011u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ASP_BUFFERINGON: u32 = 7000u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ASP_BUFFER_LIMIT: u32 = 7052u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ASP_CALCLINENUMBER: u32 = 7050u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ASP_CODEPAGE: u32 = 7016u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ASP_DISKTEMPLATECACHEDIRECTORY: u32 = 7036u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ASP_ENABLEAPPLICATIONRESTART: u32 = 7027u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ASP_ENABLEASPHTMLFALLBACK: u32 = 7021u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ASP_ENABLECHUNKEDENCODING: u32 = 7022u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ASP_ENABLECLIENTDEBUG: u32 = 7019u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ASP_ENABLEPARENTPATHS: u32 = 7008u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ASP_ENABLESERVERDEBUG: u32 = 7018u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ASP_ENABLETYPELIBCACHE: u32 = 7023u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ASP_ERRORSTONTLOG: u32 = 7024u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ASP_EXCEPTIONCATCHENABLE: u32 = 7015u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ASP_EXECUTEINMTA: u32 = 7041u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ASP_ID_LAST: u32 = 7053u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ASP_KEEPSESSIONIDSECURE: u32 = 7043u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ASP_LCID: u32 = 7042u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ASP_LOGERRORREQUESTS: u32 = 7001u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ASP_MAXDISKTEMPLATECACHEFILES: u32 = 7040u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ASP_MAXREQUESTENTITY: u32 = 7053u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ASP_MAX_REQUEST_ENTITY_ALLOWED: u32 = 7053u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ASP_MEMFREEFACTOR: u32 = 7009u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ASP_MINUSEDBLOCKS: u32 = 7010u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ASP_PROCESSORTHREADMAX: u32 = 7025u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ASP_QUEUECONNECTIONTESTTIME: u32 = 7028u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ASP_QUEUETIMEOUT: u32 = 7013u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ASP_REQEUSTQUEUEMAX: u32 = 7026u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ASP_RUN_ONEND_ANON: u32 = 7051u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ASP_SCRIPTENGINECACHEMAX: u32 = 7005u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ASP_SCRIPTERRORMESSAGE: u32 = 7003u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ASP_SCRIPTERRORSSENTTOBROWSER: u32 = 7002u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ASP_SCRIPTFILECACHESIZE: u32 = 7004u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ASP_SCRIPTLANGUAGE: u32 = 7012u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ASP_SCRIPTLANGUAGELIST: u32 = 7017u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ASP_SCRIPTTIMEOUT: u32 = 7006u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ASP_SERVICE_ENABLE_SXS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ASP_SERVICE_ENABLE_TRACKER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ASP_SERVICE_FLAGS: u32 = 7044u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ASP_SERVICE_FLAG_FUSION: u32 = 7046u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ASP_SERVICE_FLAG_PARTITIONS: u32 = 7047u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ASP_SERVICE_FLAG_TRACKER: u32 = 7045u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ASP_SERVICE_PARTITION_ID: u32 = 7048u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ASP_SERVICE_SXS_NAME: u32 = 7049u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ASP_SERVICE_USE_PARTITION: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ASP_SESSIONMAX: u32 = 7029u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ASP_SESSIONTIMEOUT: u32 = 7007u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ASP_THREADGATEENABLED: u32 = 7030u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ASP_THREADGATELOADHIGH: u32 = 7035u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ASP_THREADGATELOADLOW: u32 = 7034u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ASP_THREADGATESLEEPDELAY: u32 = 7032u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ASP_THREADGATESLEEPMAX: u32 = 7033u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ASP_THREADGATETIMESLICE: u32 = 7031u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ASP_TRACKTHREADINGMODEL: u32 = 7020u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_AUTHORIZATION: u32 = 6000u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_AUTHORIZATION_PERSISTENCE: u32 = 6031u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_AUTH_ADVNOTIFY_DISABLE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_AUTH_ANONYMOUS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_AUTH_BASIC: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_AUTH_CHANGE_DISABLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_AUTH_CHANGE_FLAGS: u32 = 2068u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_AUTH_CHANGE_UNSECURE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_AUTH_CHANGE_URL: u32 = 2060u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_AUTH_EXPIRED_UNSECUREURL: u32 = 2067u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_AUTH_EXPIRED_URL: u32 = 2061u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_AUTH_MD5: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_AUTH_NT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_AUTH_PASSPORT: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_AUTH_SINGLEREQUEST: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_AUTH_SINGLEREQUESTALWAYSIFPROXY: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_AUTH_SINGLEREQUESTIFPROXY: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_BACKUP_FORCE_BACKUP: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_BACKUP_HIGHEST_VERSION: u32 = 4294967294u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_BACKUP_MAX_LEN: u32 = 100u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_BACKUP_MAX_VERSION: u32 = 9999u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_BACKUP_NEXT_VERSION: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_BACKUP_OVERWRITE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_BACKUP_SAVE_FIRST: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_BANNER_MESSAGE: u32 = 5011u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_BINDINGS: u32 = 2022u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_CACHE_EXTENSIONS: u32 = 6034u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_CAL_AUTH_RESERVE_TIMEOUT: u32 = 2131u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_CAL_SSL_RESERVE_TIMEOUT: u32 = 2132u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_CAL_VC_PER_CONNECT: u32 = 2130u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_CAL_W3_ERROR: u32 = 2133u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_CC_MAX_AGE: u32 = 6042u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_CC_NO_CACHE: u32 = 6041u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_CC_OTHER: u32 = 6043u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_CENTRAL_W3C_LOGGING_ENABLED: u32 = 2119u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_CERT_CACHE_RETRIEVAL_ONLY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_CERT_CHECK_REVOCATION_FRESHNESS_TIME: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_CERT_NO_REVOC_CHECK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_CERT_NO_USAGE_CHECK: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_CGI_RESTRICTION_LIST: u32 = 2164u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub struct MD_CHANGE_OBJECT_W {
    pub pszMDPath: ::windows::core::PWSTR,
    pub dwMDChangeType: u32,
    pub dwMDNumDataIDs: u32,
    pub pdwMDDataIDs: *mut u32,
}
impl ::core::marker::Copy for MD_CHANGE_OBJECT_W {}
impl ::core::clone::Clone for MD_CHANGE_OBJECT_W {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MD_CHANGE_OBJECT_W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MD_CHANGE_OBJECT_W").field("pszMDPath", &self.pszMDPath).field("dwMDChangeType", &self.dwMDChangeType).field("dwMDNumDataIDs", &self.dwMDNumDataIDs).field("pdwMDDataIDs", &self.pdwMDDataIDs).finish()
    }
}
unsafe impl ::windows::core::Abi for MD_CHANGE_OBJECT_W {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MD_CHANGE_OBJECT_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MD_CHANGE_OBJECT_W>()) == 0 }
    }
}
impl ::core::cmp::Eq for MD_CHANGE_OBJECT_W {}
impl ::core::default::Default for MD_CHANGE_OBJECT_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_CHANGE_TYPE_ADD_OBJECT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_CHANGE_TYPE_DELETE_DATA: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_CHANGE_TYPE_DELETE_OBJECT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_CHANGE_TYPE_RENAME_OBJECT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_CHANGE_TYPE_RESTORE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_CHANGE_TYPE_SET_DATA: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_COMMENTS: u32 = 9990u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_CONNECTION_TIMEOUT: u32 = 1013u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_CPU_ACTION: u32 = 9022u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_CPU_APP_ENABLED: u32 = 2141u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_CPU_CGI_ENABLED: u32 = 2140u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_CPU_CGI_LIMIT: u32 = 2148u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_CPU_DISABLE_ALL_LOGGING: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_CPU_ENABLE_ACTIVE_PROCS: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_CPU_ENABLE_ALL_PROC_LOGGING: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_CPU_ENABLE_APP_LOGGING: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_CPU_ENABLE_CGI_LOGGING: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_CPU_ENABLE_EVENT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_CPU_ENABLE_KERNEL_TIME: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_CPU_ENABLE_LOGGING: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_CPU_ENABLE_PAGE_FAULTS: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_CPU_ENABLE_PROC_TYPE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_CPU_ENABLE_TERMINATED_PROCS: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_CPU_ENABLE_TOTAL_PROCS: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_CPU_ENABLE_USER_TIME: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_CPU_KILL_W3WP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_CPU_LIMIT: u32 = 9023u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_CPU_LIMITS_ENABLED: u32 = 2143u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_CPU_LIMIT_LOGEVENT: u32 = 2149u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_CPU_LIMIT_PAUSE: u32 = 2152u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_CPU_LIMIT_PRIORITY: u32 = 2150u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_CPU_LIMIT_PROCSTOP: u32 = 2151u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_CPU_LOGGING_INTERVAL: u32 = 2145u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_CPU_LOGGING_MASK: u32 = 4507u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_CPU_LOGGING_OPTIONS: u32 = 2146u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_CPU_NO_ACTION: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_CPU_RESET_INTERVAL: u32 = 2144u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_CPU_THROTTLE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_CPU_TRACE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_CREATE_PROCESS_AS_USER: u32 = 6035u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_CREATE_PROC_NEW_CONSOLE: u32 = 6036u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_CUSTOM_DEPLOYMENT_DATA: u32 = 6055u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_CUSTOM_ERROR: u32 = 6008u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_CUSTOM_ERROR_DESC: u32 = 2120u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_DEFAULT_BACKUP_LOCATION: &str = "MDBackUp";
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_DEFAULT_LOAD_FILE: u32 = 6006u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_DEFAULT_LOGON_DOMAIN: u32 = 6012u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_DEMAND_START_THRESHOLD: u32 = 9207u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_DIRBROW_ENABLED: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_DIRBROW_LOADDEFAULT: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_DIRBROW_LONG_DATE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_DIRBROW_SHOW_DATE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_DIRBROW_SHOW_EXTENSION: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_DIRBROW_SHOW_SIZE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_DIRBROW_SHOW_TIME: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_DIRECTORY_BROWSING: u32 = 6005u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_DISABLE_SOCKET_POOLING: u32 = 1029u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_DONT_LOG: u32 = 6023u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_DOWNLEVEL_ADMIN_INSTANCE: u32 = 1021u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_DO_REVERSE_DNS: u32 = 6029u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ENABLEDPROTOCOLS: u32 = 2023u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ENABLE_URL_AUTHORIZATION: u32 = 6048u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ERROR_CANNOT_REMOVE_SECURE_ATTRIBUTE: i32 = -2146646008i32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ERROR_DATA_NOT_FOUND: i32 = -2146646015i32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ERROR_IISAO_INVALID_SCHEMA: i32 = -2146646000i32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ERROR_INVALID_VERSION: i32 = -2146646014i32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ERROR_NOT_INITIALIZED: i32 = -2146646016i32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ERROR_NO_SESSION_KEY: i32 = -2146645987i32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ERROR_READ_METABASE_FILE: i32 = -2146645991i32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ERROR_SECURE_CHANNEL_FAILURE: i32 = -2146646010i32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ERROR_SUB400_INVALID_CONTENT_LENGTH: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ERROR_SUB400_INVALID_DEPTH: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ERROR_SUB400_INVALID_DESTINATION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ERROR_SUB400_INVALID_IF: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ERROR_SUB400_INVALID_LOCK_TOKEN: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ERROR_SUB400_INVALID_OVERWRITE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ERROR_SUB400_INVALID_REQUEST_BODY: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ERROR_SUB400_INVALID_TIMEOUT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ERROR_SUB400_INVALID_TRANSLATE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ERROR_SUB400_INVALID_WEBSOCKET_REQUEST: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ERROR_SUB400_INVALID_XFF_HEADER: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ERROR_SUB401_APPLICATION: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ERROR_SUB401_FILTER: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ERROR_SUB401_LOGON: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ERROR_SUB401_LOGON_ACL: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ERROR_SUB401_LOGON_CONFIG: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ERROR_SUB401_URLAUTH_POLICY: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ERROR_SUB403_ADDR_REJECT: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ERROR_SUB403_APPPOOL_DENIED: u32 = 18u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ERROR_SUB403_CAL_EXCEEDED: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ERROR_SUB403_CERT_BAD: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ERROR_SUB403_CERT_REQUIRED: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ERROR_SUB403_CERT_REVOKED: u32 = 13u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ERROR_SUB403_CERT_TIME_INVALID: u32 = 17u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ERROR_SUB403_DIR_LIST_DENIED: u32 = 14u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ERROR_SUB403_EXECUTE_ACCESS_DENIED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ERROR_SUB403_INFINITE_DEPTH_DENIED: u32 = 22u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ERROR_SUB403_INSUFFICIENT_PRIVILEGE_FOR_CGI: u32 = 19u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ERROR_SUB403_INVALID_CNFG: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ERROR_SUB403_LOCK_TOKEN_REQUIRED: u32 = 23u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ERROR_SUB403_MAPPER_DENY_ACCESS: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ERROR_SUB403_PASSPORT_LOGIN_FAILURE: u32 = 20u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ERROR_SUB403_PWD_CHANGE: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ERROR_SUB403_READ_ACCESS_DENIED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ERROR_SUB403_SITE_ACCESS_DENIED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ERROR_SUB403_SOURCE_ACCESS_DENIED: u32 = 21u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ERROR_SUB403_SSL128_REQUIRED: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ERROR_SUB403_SSL_REQUIRED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ERROR_SUB403_TOO_MANY_USERS: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ERROR_SUB403_VALIDATION_FAILURE: u32 = 24u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ERROR_SUB403_WRITE_ACCESS_DENIED: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ERROR_SUB404_DENIED_BY_FILTERING_RULE: u32 = 19u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ERROR_SUB404_DENIED_BY_MIMEMAP: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ERROR_SUB404_DENIED_BY_POLICY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ERROR_SUB404_FILE_ATTRIBUTE_HIDDEN: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ERROR_SUB404_FILE_EXTENSION_DENIED: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ERROR_SUB404_HIDDEN_SEGMENT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ERROR_SUB404_NO_HANDLER: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ERROR_SUB404_PRECONDITIONED_HANDLER: u32 = 17u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ERROR_SUB404_QUERY_STRING_SEQUENCE_DENIED: u32 = 18u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ERROR_SUB404_QUERY_STRING_TOO_LONG: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ERROR_SUB404_SITE_NOT_FOUND: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ERROR_SUB404_STATICFILE_DAV: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ERROR_SUB404_TOO_MANY_URL_SEGMENTS: u32 = 20u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ERROR_SUB404_URL_DOUBLE_ESCAPED: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ERROR_SUB404_URL_HAS_HIGH_BIT_CHARS: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ERROR_SUB404_URL_SEQUENCE_DENIED: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ERROR_SUB404_URL_TOO_LONG: u32 = 14u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ERROR_SUB404_VERB_DENIED: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ERROR_SUB413_CONTENT_LENGTH_TOO_LARGE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ERROR_SUB423_LOCK_TOKEN_SUBMITTED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ERROR_SUB423_NO_CONFLICTING_LOCK: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ERROR_SUB500_ASPNET_HANDLERS: u32 = 23u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ERROR_SUB500_ASPNET_IMPERSONATION: u32 = 24u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ERROR_SUB500_ASPNET_MODULES: u32 = 22u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ERROR_SUB500_BAD_METADATA: u32 = 19u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ERROR_SUB500_HANDLERS_MODULE: u32 = 21u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ERROR_SUB500_UNC_ACCESS: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ERROR_SUB500_URLAUTH_NO_SCOPE: u32 = 20u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ERROR_SUB500_URLAUTH_NO_STORE: u32 = 17u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ERROR_SUB500_URLAUTH_STORE_ERROR: u32 = 18u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ERROR_SUB502_ARR_CONNECTION_ERROR: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ERROR_SUB502_ARR_NO_SERVER: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ERROR_SUB502_PREMATURE_EXIT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ERROR_SUB502_TIMEOUT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ERROR_SUB503_APP_CONCURRENT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ERROR_SUB503_ASPNET_QUEUE_FULL: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ERROR_SUB503_CONNECTION_LIMIT: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ERROR_SUB503_CPU_LIMIT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ERROR_SUB503_FASTCGI_QUEUE_FULL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_EXIT_MESSAGE: u32 = 5001u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_EXPORT_INHERITED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_EXPORT_NODE_ONLY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_EXTLOG_BYTES_RECV: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_EXTLOG_BYTES_SENT: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_EXTLOG_CLIENT_IP: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_EXTLOG_COMPUTER_NAME: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_EXTLOG_COOKIE: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_EXTLOG_DATE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_EXTLOG_HOST: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_EXTLOG_HTTP_STATUS: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_EXTLOG_HTTP_SUB_STATUS: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_EXTLOG_METHOD: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_EXTLOG_PROTOCOL_VERSION: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_EXTLOG_REFERER: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_EXTLOG_SERVER_IP: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_EXTLOG_SERVER_PORT: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_EXTLOG_SITE_NAME: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_EXTLOG_TIME: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_EXTLOG_TIME_TAKEN: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_EXTLOG_URI_QUERY: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_EXTLOG_URI_STEM: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_EXTLOG_USERNAME: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_EXTLOG_USER_AGENT: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_EXTLOG_WIN32_STATUS: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_FILTER_DESCRIPTION: u32 = 2045u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_FILTER_ENABLED: u32 = 2043u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_FILTER_ENABLE_CACHE: u32 = 2046u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_FILTER_FLAGS: u32 = 2044u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_FILTER_IMAGE_PATH: u32 = 2041u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_FILTER_LOAD_ORDER: u32 = 2040u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_FILTER_STATE: u32 = 2042u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_FILTER_STATE_LOADED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_FILTER_STATE_UNLOADED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_FOOTER_DOCUMENT: u32 = 6009u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_FOOTER_ENABLED: u32 = 6010u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_FRONTPAGE_WEB: u32 = 2072u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_FTPS_128_BITS: u32 = 5053u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_FTPS_ALLOW_CCC: u32 = 5054u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_FTPS_SECURE_ANONYMOUS: u32 = 5052u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_FTPS_SECURE_CONTROL_CHANNEL: u32 = 5050u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_FTPS_SECURE_DATA_CHANNEL: u32 = 5051u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_FTP_KEEP_PARTIAL_UPLOADS: u32 = 5019u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_FTP_LOG_IN_UTF_8: u32 = 5013u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_FTP_PASV_RESPONSE_IP: u32 = 5018u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_FTP_UTF8_FILE_NAMES: u32 = 5020u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_GLOBAL_BINARY_LOGGING_ENABLED: u32 = 4016u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_GLOBAL_BINSCHEMATIMESTAMP: u32 = 9991u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_GLOBAL_CHANGE_NUMBER: u32 = 9997u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_GLOBAL_EDIT_WHILE_RUNNING_MAJOR_VERSION_NUMBER: u32 = 9994u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_GLOBAL_EDIT_WHILE_RUNNING_MINOR_VERSION_NUMBER: u32 = 9993u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_GLOBAL_LOG_IN_UTF_8: u32 = 9206u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_GLOBAL_SESSIONKEY: u32 = 9999u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_GLOBAL_STANDARD_APP_MODE_ENABLED: u32 = 9203u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_GLOBAL_XMLSCHEMATIMESTAMP: u32 = 9992u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_GREETING_MESSAGE: u32 = 5002u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_HC_CACHE_CONTROL_HEADER: u32 = 2211u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_HC_COMPRESSION_BUFFER_SIZE: u32 = 2223u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_HC_COMPRESSION_DIRECTORY: u32 = 2210u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_HC_COMPRESSION_DLL: u32 = 2237u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_HC_CREATE_FLAGS: u32 = 2243u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_HC_DO_DISK_SPACE_LIMITING: u32 = 2216u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_HC_DO_DYNAMIC_COMPRESSION: u32 = 2213u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_HC_DO_NAMESPACE_DYNAMIC_COMPRESSION: u32 = 2255u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_HC_DO_NAMESPACE_STATIC_COMPRESSION: u32 = 2256u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_HC_DO_ON_DEMAND_COMPRESSION: u32 = 2215u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_HC_DO_STATIC_COMPRESSION: u32 = 2214u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_HC_DYNAMIC_COMPRESSION_LEVEL: u32 = 2241u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_HC_EXPIRES_HEADER: u32 = 2212u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_HC_FILES_DELETED_PER_DISK_FREE: u32 = 2225u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_HC_FILE_EXTENSIONS: u32 = 2238u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_HC_IO_BUFFER_SIZE: u32 = 2222u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_HC_MAX_DISK_SPACE_USAGE: u32 = 2221u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_HC_MAX_QUEUE_LENGTH: u32 = 2224u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_HC_MIME_TYPE: u32 = 2239u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_HC_MIN_FILE_SIZE_FOR_COMP: u32 = 2226u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_HC_NO_COMPRESSION_FOR_HTTP_10: u32 = 2217u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_HC_NO_COMPRESSION_FOR_PROXIES: u32 = 2218u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_HC_NO_COMPRESSION_FOR_RANGE: u32 = 2219u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_HC_ON_DEMAND_COMP_LEVEL: u32 = 2242u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_HC_PRIORITY: u32 = 2240u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_HC_SCRIPT_FILE_EXTENSIONS: u32 = 2244u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_HC_SEND_CACHE_HEADERS: u32 = 2220u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_HEADER_WAIT_TIMEOUT: u32 = 9204u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_HISTORY_LATEST: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_HTTPERRORS_EXISTING_RESPONSE: u32 = 6056u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_HTTP_CUSTOM: u32 = 6004u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_HTTP_EXPIRES: u32 = 6002u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_HTTP_FORWARDER_CUSTOM: u32 = 6054u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_HTTP_PICS: u32 = 6003u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_HTTP_REDIRECT: u32 = 6011u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_IISADMIN_EXTENSIONS: u32 = 1028u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_IMPORT_INHERITED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_IMPORT_MERGE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_IMPORT_NODE_ONLY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_INSERT_PATH_STRING: &str = "<%INSERT_PATH%>";
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_INSERT_PATH_STRINGA: &str = "<%INSERT_PATH%>";
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_IN_PROCESS_ISAPI_APPS: u32 = 2073u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_IP_SEC: u32 = 6019u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ISAPI_RESTRICTION_LIST: u32 = 2163u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_IS_CONTENT_INDEXED: u32 = 6039u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_KEY_TYPE: u32 = 1002u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_LEVELS_TO_SCAN: u32 = 1022u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_LOAD_BALANCER_CAPABILITIES: u32 = 9034u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_LOAD_BALANCER_CAPABILITIES_BASIC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_LOAD_BALANCER_CAPABILITIES_SOPHISTICATED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_LOCATION: u32 = 9989u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_LOGCUSTOM_DATATYPE_DOUBLE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_LOGCUSTOM_DATATYPE_FLOAT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_LOGCUSTOM_DATATYPE_INT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_LOGCUSTOM_DATATYPE_LONG: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_LOGCUSTOM_DATATYPE_LPSTR: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_LOGCUSTOM_DATATYPE_LPWSTR: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_LOGCUSTOM_DATATYPE_UINT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_LOGCUSTOM_DATATYPE_ULONG: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_LOGCUSTOM_PROPERTY_DATATYPE: u32 = 4505u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_LOGCUSTOM_PROPERTY_HEADER: u32 = 4502u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_LOGCUSTOM_PROPERTY_ID: u32 = 4503u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_LOGCUSTOM_PROPERTY_MASK: u32 = 4504u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_LOGCUSTOM_PROPERTY_NAME: u32 = 4501u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_LOGCUSTOM_PROPERTY_NODE_ID: u32 = 4508u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_LOGCUSTOM_SERVICES_STRING: u32 = 4506u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_LOGEXT_FIELD_MASK: u32 = 4013u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_LOGEXT_FIELD_MASK2: u32 = 4014u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_LOGFILE_DIRECTORY: u32 = 4001u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_LOGFILE_LOCALTIME_ROLLOVER: u32 = 4015u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_LOGFILE_PERIOD: u32 = 4003u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_LOGFILE_PERIOD_DAILY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_LOGFILE_PERIOD_HOURLY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_LOGFILE_PERIOD_MAXSIZE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_LOGFILE_PERIOD_MONTHLY: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_LOGFILE_PERIOD_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_LOGFILE_PERIOD_WEEKLY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_LOGFILE_TRUNCATE_SIZE: u32 = 4004u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_LOGON_BATCH: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_LOGON_INTERACTIVE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_LOGON_METHOD: u32 = 6013u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_LOGON_NETWORK: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_LOGON_NETWORK_CLEARTEXT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_LOGSQL_DATA_SOURCES: u32 = 4007u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_LOGSQL_PASSWORD: u32 = 4010u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_LOGSQL_TABLE_NAME: u32 = 4008u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_LOGSQL_USER_NAME: u32 = 4009u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_LOG_ANONYMOUS: u32 = 5007u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_LOG_NONANONYMOUS: u32 = 5008u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_LOG_PLUGINS_AVAILABLE: u32 = 4012u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_LOG_PLUGIN_MOD_ID: u32 = 4005u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_LOG_PLUGIN_ORDER: u32 = 4011u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_LOG_PLUGIN_UI_ID: u32 = 4006u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_LOG_TYPE: u32 = 4000u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_LOG_TYPE_DISABLED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_LOG_TYPE_ENABLED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_LOG_UNUSED1: u32 = 4002u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_MAX_BANDWIDTH: u32 = 1000u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_MAX_BANDWIDTH_BLOCKED: u32 = 1003u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_MAX_CHANGE_ENTRIES: u32 = 100u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_MAX_CLIENTS_MESSAGE: u32 = 5003u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_MAX_CONNECTIONS: u32 = 1014u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_MAX_ENDPOINT_CONNECTIONS: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_MAX_ERROR_FILES: u32 = 9988u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_MAX_GLOBAL_BANDWIDTH: u32 = 9201u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_MAX_GLOBAL_CONNECTIONS: u32 = 9202u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_MAX_REQUEST_ENTITY_ALLOWED: u32 = 6051u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_MD_SERVER_SS_AUTH_MAPPING: u32 = 2200u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_METADATA_ID_REGISTRATION: u32 = 1030u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_MIME_MAP: u32 = 6015u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_MIN_FILE_BYTES_PER_SEC: u32 = 9205u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_MSDOS_DIR_OUTPUT: u32 = 5004u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_NETLOGON_WKS_DNS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_NETLOGON_WKS_IP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_NETLOGON_WKS_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_NET_LOGON_WKS: u32 = 2065u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_NOTIFEXAUTH_NTLMSSL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_NOTIFY_ACCESS_DENIED: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_NOTIFY_AUTHENTICATION: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_NOTIFY_AUTH_COMPLETE: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_NOTIFY_END_OF_NET_SESSION: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_NOTIFY_END_OF_REQUEST: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_NOTIFY_LOG: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_NOTIFY_NONSECURE_PORT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_NOTIFY_ORDER_DEFAULT: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_NOTIFY_ORDER_HIGH: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_NOTIFY_ORDER_LOW: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_NOTIFY_ORDER_MEDIUM: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_NOTIFY_PREPROC_HEADERS: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_NOTIFY_READ_RAW_DATA: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_NOTIFY_SECURE_PORT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_NOTIFY_SEND_RAW_DATA: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_NOTIFY_SEND_RESPONSE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_NOTIFY_URL_MAP: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_NOT_DELETABLE: u32 = 2116u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_NTAUTHENTICATION_PROVIDERS: u32 = 6032u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_PASSIVE_PORT_RANGE: u32 = 5016u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_PASSPORT_NEED_MAPPING: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_PASSPORT_NO_MAPPING: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_PASSPORT_REQUIRE_AD_MAPPING: u32 = 6052u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_PASSPORT_TRY_MAPPING: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_POOL_IDC_TIMEOUT: u32 = 6037u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_PROCESS_NTCR_IF_LOGGED_ON: u32 = 2070u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_PUT_READ_SIZE: u32 = 6046u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_RAPID_FAIL_PROTECTION_INTERVAL: u32 = 9029u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_RAPID_FAIL_PROTECTION_MAX_CRASHES: u32 = 9030u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_REALM: u32 = 6001u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_REDIRECT_HEADERS: u32 = 6044u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_RESTRICTION_LIST_CUSTOM_DESC: u32 = 2165u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ROOT_ENABLE_EDIT_WHILE_RUNNING: u32 = 9998u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ROOT_ENABLE_HISTORY: u32 = 9996u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_ROOT_MAX_HISTORY_FILES: u32 = 9995u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_SCHEMA_METAID: u32 = 1004u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_SCRIPTMAPFLAG_ALLOWED_ON_READ_DIR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_SCRIPTMAPFLAG_CHECK_PATH_INFO: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_SCRIPTMAPFLAG_SCRIPT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_SCRIPT_MAPS: u32 = 6014u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_SCRIPT_TIMEOUT: u32 = 6033u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_SECURE_BINDINGS: u32 = 2021u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_SECURITY_SETUP_REQUIRED: u32 = 2166u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_SERVER_AUTOSTART: u32 = 1017u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_SERVER_BINDINGS: u32 = 1023u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_SERVER_COMMAND: u32 = 1012u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_SERVER_COMMAND_CONTINUE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_SERVER_COMMAND_PAUSE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_SERVER_COMMAND_START: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_SERVER_COMMAND_STOP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_SERVER_COMMENT: u32 = 1015u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_SERVER_CONFIGURATION_INFO: u32 = 1027u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_SERVER_CONFIG_ALLOW_ENCRYPT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_SERVER_CONFIG_AUTO_PW_SYNC: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_SERVER_CONFIG_SSL_128: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_SERVER_CONFIG_SSL_40: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_SERVER_LISTEN_BACKLOG: u32 = 1019u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_SERVER_LISTEN_TIMEOUT: u32 = 1020u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_SERVER_SIZE: u32 = 1018u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_SERVER_SIZE_LARGE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_SERVER_SIZE_MEDIUM: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_SERVER_SIZE_SMALL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_SERVER_STATE: u32 = 1016u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_SERVER_STATE_CONTINUING: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_SERVER_STATE_PAUSED: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_SERVER_STATE_PAUSING: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_SERVER_STATE_STARTED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_SERVER_STATE_STARTING: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_SERVER_STATE_STOPPED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_SERVER_STATE_STOPPING: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_SET_HOST_NAME: u32 = 2154u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_SHOW_4_DIGIT_YEAR: u32 = 5010u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_SSI_EXEC_DISABLED: u32 = 6028u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_SSL_ACCESS_PERM: u32 = 6030u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_SSL_ALWAYS_NEGO_CLIENT_CERT: u32 = 5521u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_SSL_KEY_PASSWORD: u32 = 5502u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_SSL_KEY_REQUEST: u32 = 5503u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_SSL_PRIVATE_KEY: u32 = 5501u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_SSL_PUBLIC_KEY: u32 = 5500u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_SSL_USE_DS_MAPPER: u32 = 5519u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_STOP_LISTENING: u32 = 9987u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_SUPPRESS_DEFAULT_BANNER: u32 = 5017u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_UPLOAD_READAHEAD_SIZE: u32 = 6045u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_URL_AUTHORIZATION_IMPERSONATION_LEVEL: u32 = 6053u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_URL_AUTHORIZATION_SCOPE_NAME: u32 = 6050u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_URL_AUTHORIZATION_STORE_NAME: u32 = 6049u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_USER_ISOLATION: u32 = 5012u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_USER_ISOLATION_AD: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_USER_ISOLATION_BASIC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_USER_ISOLATION_LAST: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_USER_ISOLATION_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_USE_DIGEST_SSP: u32 = 6047u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_USE_HOST_NAME: u32 = 2066u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_VR_IGNORE_TRANSLATE: u32 = 3008u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_VR_NO_CACHE: u32 = 3007u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_VR_PASSTHROUGH: u32 = 3006u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_VR_PASSWORD: u32 = 3003u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_VR_PATH: u32 = 3001u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_VR_USERNAME: u32 = 3002u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_WAM_PWD: u32 = 7502u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_WAM_USER_NAME: u32 = 7501u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_WARNING_DUP_NAME: i32 = 837636i32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_WARNING_INVALID_DATA: i32 = 837637i32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_WARNING_PATH_NOT_FOUND: i32 = 837635i32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_WARNING_PATH_NOT_INSERTED: i32 = 837639i32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_WARNING_SAVE_FAILED: i32 = 837641i32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_WEBDAV_MAX_ATTRIBUTES_PER_ELEMENT: u32 = 8501u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_WEB_SVC_EXT_RESTRICTION_LIST: u32 = 2168u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_WIN32_ERROR: u32 = 1099u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct METADATATYPES(pub i32);
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const ALL_METADATA: METADATATYPES = METADATATYPES(0i32);
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const DWORD_METADATA: METADATATYPES = METADATATYPES(1i32);
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const STRING_METADATA: METADATATYPES = METADATATYPES(2i32);
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const BINARY_METADATA: METADATATYPES = METADATATYPES(3i32);
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const EXPANDSZ_METADATA: METADATATYPES = METADATATYPES(4i32);
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MULTISZ_METADATA: METADATATYPES = METADATATYPES(5i32);
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const INVALID_END_METADATA: METADATATYPES = METADATATYPES(6i32);
impl ::core::marker::Copy for METADATATYPES {}
impl ::core::clone::Clone for METADATATYPES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for METADATATYPES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for METADATATYPES {
    type Abi = Self;
}
impl ::core::fmt::Debug for METADATATYPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("METADATATYPES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const METADATA_DONT_EXPAND: u32 = 512u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub struct METADATA_GETALL_INTERNAL_RECORD {
    pub dwMDIdentifier: u32,
    pub dwMDAttributes: u32,
    pub dwMDUserType: u32,
    pub dwMDDataType: u32,
    pub dwMDDataLen: u32,
    pub Anonymous: METADATA_GETALL_INTERNAL_RECORD_0,
    pub dwMDDataTag: u32,
}
impl ::core::marker::Copy for METADATA_GETALL_INTERNAL_RECORD {}
impl ::core::clone::Clone for METADATA_GETALL_INTERNAL_RECORD {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for METADATA_GETALL_INTERNAL_RECORD {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for METADATA_GETALL_INTERNAL_RECORD {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<METADATA_GETALL_INTERNAL_RECORD>()) == 0 }
    }
}
impl ::core::cmp::Eq for METADATA_GETALL_INTERNAL_RECORD {}
impl ::core::default::Default for METADATA_GETALL_INTERNAL_RECORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub union METADATA_GETALL_INTERNAL_RECORD_0 {
    pub dwMDDataOffset: usize,
    pub pbMDData: *mut u8,
}
impl ::core::marker::Copy for METADATA_GETALL_INTERNAL_RECORD_0 {}
impl ::core::clone::Clone for METADATA_GETALL_INTERNAL_RECORD_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for METADATA_GETALL_INTERNAL_RECORD_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for METADATA_GETALL_INTERNAL_RECORD_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<METADATA_GETALL_INTERNAL_RECORD_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for METADATA_GETALL_INTERNAL_RECORD_0 {}
impl ::core::default::Default for METADATA_GETALL_INTERNAL_RECORD_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub struct METADATA_GETALL_RECORD {
    pub dwMDIdentifier: u32,
    pub dwMDAttributes: u32,
    pub dwMDUserType: u32,
    pub dwMDDataType: u32,
    pub dwMDDataLen: u32,
    pub dwMDDataOffset: u32,
    pub dwMDDataTag: u32,
}
impl ::core::marker::Copy for METADATA_GETALL_RECORD {}
impl ::core::clone::Clone for METADATA_GETALL_RECORD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for METADATA_GETALL_RECORD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("METADATA_GETALL_RECORD").field("dwMDIdentifier", &self.dwMDIdentifier).field("dwMDAttributes", &self.dwMDAttributes).field("dwMDUserType", &self.dwMDUserType).field("dwMDDataType", &self.dwMDDataType).field("dwMDDataLen", &self.dwMDDataLen).field("dwMDDataOffset", &self.dwMDDataOffset).field("dwMDDataTag", &self.dwMDDataTag).finish()
    }
}
unsafe impl ::windows::core::Abi for METADATA_GETALL_RECORD {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for METADATA_GETALL_RECORD {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<METADATA_GETALL_RECORD>()) == 0 }
    }
}
impl ::core::cmp::Eq for METADATA_GETALL_RECORD {}
impl ::core::default::Default for METADATA_GETALL_RECORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub struct METADATA_HANDLE_INFO {
    pub dwMDPermissions: u32,
    pub dwMDSystemChangeNumber: u32,
}
impl ::core::marker::Copy for METADATA_HANDLE_INFO {}
impl ::core::clone::Clone for METADATA_HANDLE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for METADATA_HANDLE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("METADATA_HANDLE_INFO").field("dwMDPermissions", &self.dwMDPermissions).field("dwMDSystemChangeNumber", &self.dwMDSystemChangeNumber).finish()
    }
}
unsafe impl ::windows::core::Abi for METADATA_HANDLE_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for METADATA_HANDLE_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<METADATA_HANDLE_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for METADATA_HANDLE_INFO {}
impl ::core::default::Default for METADATA_HANDLE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const METADATA_INHERIT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const METADATA_INSERT_PATH: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const METADATA_ISINHERITED: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const METADATA_LOCAL_MACHINE_ONLY: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const METADATA_MASTER_ROOT_HANDLE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const METADATA_MAX_NAME_LEN: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const METADATA_NON_SECURE_ONLY: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const METADATA_NO_ATTRIBUTES: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const METADATA_PARTIAL_PATH: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const METADATA_PERMISSION_READ: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const METADATA_PERMISSION_WRITE: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub struct METADATA_RECORD {
    pub dwMDIdentifier: u32,
    pub dwMDAttributes: u32,
    pub dwMDUserType: u32,
    pub dwMDDataType: u32,
    pub dwMDDataLen: u32,
    pub pbMDData: *mut u8,
    pub dwMDDataTag: u32,
}
impl ::core::marker::Copy for METADATA_RECORD {}
impl ::core::clone::Clone for METADATA_RECORD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for METADATA_RECORD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("METADATA_RECORD").field("dwMDIdentifier", &self.dwMDIdentifier).field("dwMDAttributes", &self.dwMDAttributes).field("dwMDUserType", &self.dwMDUserType).field("dwMDDataType", &self.dwMDDataType).field("dwMDDataLen", &self.dwMDDataLen).field("pbMDData", &self.pbMDData).field("dwMDDataTag", &self.dwMDDataTag).finish()
    }
}
unsafe impl ::windows::core::Abi for METADATA_RECORD {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for METADATA_RECORD {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<METADATA_RECORD>()) == 0 }
    }
}
impl ::core::cmp::Eq for METADATA_RECORD {}
impl ::core::default::Default for METADATA_RECORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const METADATA_REFERENCE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const METADATA_SECURE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const METADATA_VOLATILE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MSCS_MD_ID_BEGIN_RESERVED: u32 = 53248u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MSCS_MD_ID_END_RESERVED: u32 = 57343u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const NNTP_MD_ID_BEGIN_RESERVED: u32 = 45056u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const NNTP_MD_ID_END_RESERVED: u32 = 49151u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_GETEXTENSIONVERSION = ::core::option::Option<unsafe extern "system" fn(pver: *mut HSE_VERSION_INFO) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub type PFN_HSE_CACHE_INVALIDATION_CALLBACK = ::core::option::Option<unsafe extern "system" fn(pszurl: ::windows::core::PCWSTR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub type PFN_HSE_GET_PROTOCOL_MANAGER_CUSTOM_INTERFACE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(pszprotocolmanagerdll: ::windows::core::PCWSTR, pszprotocolmanagerdllinitfunction: ::windows::core::PCWSTR, dwcustominterfaceid: u32, ppcustominterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_Iis\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_HSE_IO_COMPLETION = ::core::option::Option<unsafe extern "system" fn(pecb: *mut EXTENSION_CONTROL_BLOCK, pcontext: *mut ::core::ffi::c_void, cbio: u32, dwerror: u32)>;
#[doc = "*Required features: `\"Win32_System_Iis\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_HTTPEXTENSIONPROC = ::core::option::Option<unsafe extern "system" fn(pecb: *mut EXTENSION_CONTROL_BLOCK) -> u32>;
#[doc = "*Required features: `\"Win32_System_Iis\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_TERMINATEEXTENSION = ::core::option::Option<unsafe extern "system" fn(dwflags: u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub type PFN_WEB_CORE_ACTIVATE = ::core::option::Option<unsafe extern "system" fn(pszapphostconfigfile: ::windows::core::PCWSTR, pszrootwebconfigfile: ::windows::core::PCWSTR, pszinstancename: ::windows::core::PCWSTR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub type PFN_WEB_CORE_SET_METADATA_DLL_ENTRY = ::core::option::Option<unsafe extern "system" fn(pszmetadatatype: ::windows::core::PCWSTR, pszvalue: ::windows::core::PCWSTR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub type PFN_WEB_CORE_SHUTDOWN = ::core::option::Option<unsafe extern "system" fn(fimmediate: u32) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const POP3_MD_ID_BEGIN_RESERVED: u32 = 40960u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const POP3_MD_ID_END_RESERVED: u32 = 45055u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Iis\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct POST_PROCESS_PARAMETERS {
    pub pszSessionId: ::windows::core::PCWSTR,
    pub pszSiteName: ::windows::core::PCWSTR,
    pub pszUserName: ::windows::core::PCWSTR,
    pub pszHostName: ::windows::core::PCWSTR,
    pub pszRemoteIpAddress: ::windows::core::PCWSTR,
    pub dwRemoteIpPort: u32,
    pub pszLocalIpAddress: ::windows::core::PCWSTR,
    pub dwLocalIpPort: u32,
    pub BytesSent: u64,
    pub BytesReceived: u64,
    pub pszCommand: ::windows::core::PCWSTR,
    pub pszCommandParameters: ::windows::core::PCWSTR,
    pub pszFullPath: ::windows::core::PCWSTR,
    pub pszPhysicalPath: ::windows::core::PCWSTR,
    pub FtpStatus: u32,
    pub FtpSubStatus: u32,
    pub hrStatus: ::windows::core::HRESULT,
    pub SessionStartTime: super::super::Foundation::FILETIME,
    pub BytesSentPerSession: u64,
    pub BytesReceivedPerSession: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for POST_PROCESS_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for POST_PROCESS_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for POST_PROCESS_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("POST_PROCESS_PARAMETERS")
            .field("pszSessionId", &self.pszSessionId)
            .field("pszSiteName", &self.pszSiteName)
            .field("pszUserName", &self.pszUserName)
            .field("pszHostName", &self.pszHostName)
            .field("pszRemoteIpAddress", &self.pszRemoteIpAddress)
            .field("dwRemoteIpPort", &self.dwRemoteIpPort)
            .field("pszLocalIpAddress", &self.pszLocalIpAddress)
            .field("dwLocalIpPort", &self.dwLocalIpPort)
            .field("BytesSent", &self.BytesSent)
            .field("BytesReceived", &self.BytesReceived)
            .field("pszCommand", &self.pszCommand)
            .field("pszCommandParameters", &self.pszCommandParameters)
            .field("pszFullPath", &self.pszFullPath)
            .field("pszPhysicalPath", &self.pszPhysicalPath)
            .field("FtpStatus", &self.FtpStatus)
            .field("FtpSubStatus", &self.FtpSubStatus)
            .field("hrStatus", &self.hrStatus)
            .field("SessionStartTime", &self.SessionStartTime)
            .field("BytesSentPerSession", &self.BytesSentPerSession)
            .field("BytesReceivedPerSession", &self.BytesReceivedPerSession)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for POST_PROCESS_PARAMETERS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for POST_PROCESS_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<POST_PROCESS_PARAMETERS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for POST_PROCESS_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for POST_PROCESS_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Iis\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PRE_PROCESS_PARAMETERS {
    pub pszSessionId: ::windows::core::PCWSTR,
    pub pszSiteName: ::windows::core::PCWSTR,
    pub pszUserName: ::windows::core::PCWSTR,
    pub pszHostName: ::windows::core::PCWSTR,
    pub pszRemoteIpAddress: ::windows::core::PCWSTR,
    pub dwRemoteIpPort: u32,
    pub pszLocalIpAddress: ::windows::core::PCWSTR,
    pub dwLocalIpPort: u32,
    pub pszCommand: ::windows::core::PCWSTR,
    pub pszCommandParameters: ::windows::core::PCWSTR,
    pub SessionStartTime: super::super::Foundation::FILETIME,
    pub BytesSentPerSession: u64,
    pub BytesReceivedPerSession: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PRE_PROCESS_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PRE_PROCESS_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PRE_PROCESS_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PRE_PROCESS_PARAMETERS")
            .field("pszSessionId", &self.pszSessionId)
            .field("pszSiteName", &self.pszSiteName)
            .field("pszUserName", &self.pszUserName)
            .field("pszHostName", &self.pszHostName)
            .field("pszRemoteIpAddress", &self.pszRemoteIpAddress)
            .field("dwRemoteIpPort", &self.dwRemoteIpPort)
            .field("pszLocalIpAddress", &self.pszLocalIpAddress)
            .field("dwLocalIpPort", &self.dwLocalIpPort)
            .field("pszCommand", &self.pszCommand)
            .field("pszCommandParameters", &self.pszCommandParameters)
            .field("SessionStartTime", &self.SessionStartTime)
            .field("BytesSentPerSession", &self.BytesSentPerSession)
            .field("BytesReceivedPerSession", &self.BytesReceivedPerSession)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PRE_PROCESS_PARAMETERS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PRE_PROCESS_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PRE_PROCESS_PARAMETERS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PRE_PROCESS_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PRE_PROCESS_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const SF_DENIED_APPLICATION: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const SF_DENIED_BY_CONFIG: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const SF_DENIED_FILTER: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const SF_DENIED_LOGON: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const SF_DENIED_RESOURCE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const SF_MAX_AUTH_TYPE: u32 = 33u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const SF_MAX_FILTER_DESC_LEN: u32 = 257u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const SF_MAX_PASSWORD: u32 = 257u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const SF_MAX_USERNAME: u32 = 257u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const SF_NOTIFY_ACCESS_DENIED: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const SF_NOTIFY_AUTHENTICATION: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const SF_NOTIFY_AUTH_COMPLETE: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const SF_NOTIFY_END_OF_NET_SESSION: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const SF_NOTIFY_END_OF_REQUEST: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const SF_NOTIFY_LOG: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const SF_NOTIFY_NONSECURE_PORT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const SF_NOTIFY_ORDER_DEFAULT: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const SF_NOTIFY_ORDER_HIGH: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const SF_NOTIFY_ORDER_LOW: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const SF_NOTIFY_ORDER_MEDIUM: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const SF_NOTIFY_PREPROC_HEADERS: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const SF_NOTIFY_READ_RAW_DATA: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const SF_NOTIFY_SECURE_PORT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const SF_NOTIFY_SEND_RAW_DATA: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const SF_NOTIFY_SEND_RESPONSE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const SF_NOTIFY_URL_MAP: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SF_PROPERTY_IIS(pub i32);
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const SF_PROPERTY_SSL_CTXT: SF_PROPERTY_IIS = SF_PROPERTY_IIS(0i32);
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const SF_PROPERTY_INSTANCE_NUM_ID: SF_PROPERTY_IIS = SF_PROPERTY_IIS(1i32);
impl ::core::marker::Copy for SF_PROPERTY_IIS {}
impl ::core::clone::Clone for SF_PROPERTY_IIS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SF_PROPERTY_IIS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SF_PROPERTY_IIS {
    type Abi = Self;
}
impl ::core::fmt::Debug for SF_PROPERTY_IIS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SF_PROPERTY_IIS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SF_REQ_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const SF_REQ_SEND_RESPONSE_HEADER: SF_REQ_TYPE = SF_REQ_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const SF_REQ_ADD_HEADERS_ON_DENIAL: SF_REQ_TYPE = SF_REQ_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const SF_REQ_SET_NEXT_READ_SIZE: SF_REQ_TYPE = SF_REQ_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const SF_REQ_SET_PROXY_INFO: SF_REQ_TYPE = SF_REQ_TYPE(3i32);
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const SF_REQ_GET_CONNID: SF_REQ_TYPE = SF_REQ_TYPE(4i32);
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const SF_REQ_SET_CERTIFICATE_INFO: SF_REQ_TYPE = SF_REQ_TYPE(5i32);
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const SF_REQ_GET_PROPERTY: SF_REQ_TYPE = SF_REQ_TYPE(6i32);
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const SF_REQ_NORMALIZE_URL: SF_REQ_TYPE = SF_REQ_TYPE(7i32);
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const SF_REQ_DISABLE_NOTIFICATIONS: SF_REQ_TYPE = SF_REQ_TYPE(8i32);
impl ::core::marker::Copy for SF_REQ_TYPE {}
impl ::core::clone::Clone for SF_REQ_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SF_REQ_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SF_REQ_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for SF_REQ_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SF_REQ_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SF_STATUS_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const SF_STATUS_REQ_FINISHED: SF_STATUS_TYPE = SF_STATUS_TYPE(134217728i32);
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const SF_STATUS_REQ_FINISHED_KEEP_CONN: SF_STATUS_TYPE = SF_STATUS_TYPE(134217729i32);
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const SF_STATUS_REQ_NEXT_NOTIFICATION: SF_STATUS_TYPE = SF_STATUS_TYPE(134217730i32);
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const SF_STATUS_REQ_HANDLED_NOTIFICATION: SF_STATUS_TYPE = SF_STATUS_TYPE(134217731i32);
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const SF_STATUS_REQ_ERROR: SF_STATUS_TYPE = SF_STATUS_TYPE(134217732i32);
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const SF_STATUS_REQ_READ_NEXT: SF_STATUS_TYPE = SF_STATUS_TYPE(134217733i32);
impl ::core::marker::Copy for SF_STATUS_TYPE {}
impl ::core::clone::Clone for SF_STATUS_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SF_STATUS_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SF_STATUS_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for SF_STATUS_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SF_STATUS_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const SMTP_MD_ID_BEGIN_RESERVED: u32 = 36864u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const SMTP_MD_ID_END_RESERVED: u32 = 40959u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const USER_MD_ID_BASE_RESERVED: u32 = 65535u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const WAM_MD_ID_BEGIN_RESERVED: u32 = 29952u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const WAM_MD_ID_END_RESERVED: u32 = 32767u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const WAM_MD_SERVER_BASE: u32 = 7500u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const WEBDAV_MD_SERVER_BASE: u32 = 8500u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const WEB_CORE_ACTIVATE_DLL_ENTRY: &str = "WebCoreActivate";
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const WEB_CORE_DLL_NAME: &str = "hwebcore.dll";
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const WEB_CORE_SET_METADATA_DLL_ENTRY: &str = "WebCoreSetMetadata";
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const WEB_CORE_SHUTDOWN_DLL_ENTRY: &str = "WebCoreShutdown";
#[repr(C)]
pub struct _IIS_CRYPTO_BLOB(pub u8);
#[cfg(feature = "implement")]
::core::include!("impl.rs");
