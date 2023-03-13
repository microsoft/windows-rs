#[doc = "*Required features: `\"Win32_System_Iis\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetExtensionVersion(pver: *mut HSE_VERSION_INFO) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "rpcproxy.dll""system" fn GetExtensionVersion ( pver : *mut HSE_VERSION_INFO ) -> super::super::Foundation:: BOOL );
    GetExtensionVersion(pver)
}
#[doc = "*Required features: `\"Win32_System_Iis\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetFilterVersion(pver: *mut HTTP_FILTER_VERSION) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "rpcproxy.dll""system" fn GetFilterVersion ( pver : *mut HTTP_FILTER_VERSION ) -> super::super::Foundation:: BOOL );
    GetFilterVersion(pver)
}
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
#[inline]
pub unsafe fn HttpExtensionProc(pecb: *const EXTENSION_CONTROL_BLOCK) -> u32 {
    ::windows::imp::link ! ( "rpcproxy.dll""system" fn HttpExtensionProc ( pecb : *const EXTENSION_CONTROL_BLOCK ) -> u32 );
    HttpExtensionProc(pecb)
}
#[doc = "*Required features: `\"Win32_System_Iis\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HttpFilterProc(pfc: *mut HTTP_FILTER_CONTEXT, notificationtype: u32, pvnotification: *mut ::core::ffi::c_void) -> u32 {
    ::windows::imp::link ! ( "rpcproxy.dll""system" fn HttpFilterProc ( pfc : *mut HTTP_FILTER_CONTEXT , notificationtype : u32 , pvnotification : *mut ::core::ffi::c_void ) -> u32 );
    HttpFilterProc(pfc, notificationtype, pvnotification)
}
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
#[repr(transparent)]
pub struct AsyncIFtpAuthenticationProvider(::windows::core::IUnknown);
impl AsyncIFtpAuthenticationProvider {
    pub unsafe fn Begin_AuthenticateUser<P0, P1, P2, P3>(&self, pszsessionid: P0, pszsitename: P1, pszusername: P2, pszpassword: P3) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P3: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).Begin_AuthenticateUser)(::windows::core::Interface::as_raw(self), pszsessionid.into_param().abi(), pszsitename.into_param().abi(), pszusername.into_param().abi(), pszpassword.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Finish_AuthenticateUser(&self, ppszcanonicalusername: *mut ::windows::core::PWSTR, pfauthenticated: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Finish_AuthenticateUser)(::windows::core::Interface::as_raw(self), ppszcanonicalusername, pfauthenticated).ok()
    }
}
::windows::imp::interface_hierarchy!(AsyncIFtpAuthenticationProvider, ::windows::core::IUnknown);
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
}
impl ::core::clone::Clone for AsyncIFtpAuthenticationProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for AsyncIFtpAuthenticationProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc24efb65_9f3e_4996_8fb1_ce166916bab5);
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIFtpAuthenticationProvider_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
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
    pub unsafe fn Begin_GetUserAccessPermission<P0, P1, P2, P3>(&self, pszsessionid: P0, pszsitename: P1, pszvirtualpath: P2, pszusername: P3) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P3: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).Begin_GetUserAccessPermission)(::windows::core::Interface::as_raw(self), pszsessionid.into_param().abi(), pszsitename.into_param().abi(), pszvirtualpath.into_param().abi(), pszusername.into_param().abi()).ok()
    }
    pub unsafe fn Finish_GetUserAccessPermission(&self) -> ::windows::core::Result<FTP_ACCESS> {
        let mut result__ = ::windows::core::zeroed::<FTP_ACCESS>();
        (::windows::core::Interface::vtable(self).Finish_GetUserAccessPermission)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(AsyncIFtpAuthorizationProvider, ::windows::core::IUnknown);
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
}
impl ::core::clone::Clone for AsyncIFtpAuthorizationProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for AsyncIFtpAuthorizationProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x860dc339_07e5_4a5c_9c61_8820cea012bc);
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIFtpAuthorizationProvider_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Begin_GetUserAccessPermission: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszsessionid: ::windows::core::PCWSTR, pszsitename: ::windows::core::PCWSTR, pszvirtualpath: ::windows::core::PCWSTR, pszusername: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub Finish_GetUserAccessPermission: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pftpaccess: *mut FTP_ACCESS) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
#[repr(transparent)]
pub struct AsyncIFtpHomeDirectoryProvider(::windows::core::IUnknown);
impl AsyncIFtpHomeDirectoryProvider {
    pub unsafe fn Begin_GetUserHomeDirectoryData<P0, P1, P2>(&self, pszsessionid: P0, pszsitename: P1, pszusername: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).Begin_GetUserHomeDirectoryData)(::windows::core::Interface::as_raw(self), pszsessionid.into_param().abi(), pszsitename.into_param().abi(), pszusername.into_param().abi()).ok()
    }
    pub unsafe fn Finish_GetUserHomeDirectoryData(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).Finish_GetUserHomeDirectoryData)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(AsyncIFtpHomeDirectoryProvider, ::windows::core::IUnknown);
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
}
impl ::core::clone::Clone for AsyncIFtpHomeDirectoryProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for AsyncIFtpHomeDirectoryProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x73f81638_6295_42bd_a2be_4a657f7c479c);
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIFtpHomeDirectoryProvider_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Begin_GetUserHomeDirectoryData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszsessionid: ::windows::core::PCWSTR, pszsitename: ::windows::core::PCWSTR, pszusername: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub Finish_GetUserHomeDirectoryData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszhomedirectorydata: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
#[repr(transparent)]
pub struct AsyncIFtpLogProvider(::windows::core::IUnknown);
impl AsyncIFtpLogProvider {
    pub unsafe fn Begin_Log(&self, ploggingparameters: *const LOGGING_PARAMETERS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Begin_Log)(::windows::core::Interface::as_raw(self), ploggingparameters).ok()
    }
    pub unsafe fn Finish_Log(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Finish_Log)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(AsyncIFtpLogProvider, ::windows::core::IUnknown);
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
}
impl ::core::clone::Clone for AsyncIFtpLogProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for AsyncIFtpLogProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00a0ae46_2498_48b2_95e6_df678ed7d49f);
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIFtpLogProvider_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Begin_Log: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ploggingparameters: *const LOGGING_PARAMETERS) -> ::windows::core::HRESULT,
    pub Finish_Log: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
#[repr(transparent)]
pub struct AsyncIFtpPostprocessProvider(::windows::core::IUnknown);
impl AsyncIFtpPostprocessProvider {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Begin_HandlePostprocess(&self, ppostprocessparameters: *const POST_PROCESS_PARAMETERS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Begin_HandlePostprocess)(::windows::core::Interface::as_raw(self), ppostprocessparameters).ok()
    }
    pub unsafe fn Finish_HandlePostprocess(&self) -> ::windows::core::Result<FTP_PROCESS_STATUS> {
        let mut result__ = ::windows::core::zeroed::<FTP_PROCESS_STATUS>();
        (::windows::core::Interface::vtable(self).Finish_HandlePostprocess)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(AsyncIFtpPostprocessProvider, ::windows::core::IUnknown);
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
}
impl ::core::clone::Clone for AsyncIFtpPostprocessProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for AsyncIFtpPostprocessProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa16b2542_9694_4eb1_a564_6c2e91fdc133);
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIFtpPostprocessProvider_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
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
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Begin_HandlePreprocess(&self, ppreprocessparameters: *const PRE_PROCESS_PARAMETERS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Begin_HandlePreprocess)(::windows::core::Interface::as_raw(self), ppreprocessparameters).ok()
    }
    pub unsafe fn Finish_HandlePreprocess(&self) -> ::windows::core::Result<FTP_PROCESS_STATUS> {
        let mut result__ = ::windows::core::zeroed::<FTP_PROCESS_STATUS>();
        (::windows::core::Interface::vtable(self).Finish_HandlePreprocess)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(AsyncIFtpPreprocessProvider, ::windows::core::IUnknown);
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
}
impl ::core::clone::Clone for AsyncIFtpPreprocessProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for AsyncIFtpPreprocessProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6ff5fd8f_fd8e_48b1_a3e0_bf7073db4db5);
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIFtpPreprocessProvider_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
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
    pub unsafe fn Begin_IsUserInRole<P0, P1, P2, P3>(&self, pszsessionid: P0, pszsitename: P1, pszusername: P2, pszrole: P3) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P3: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).Begin_IsUserInRole)(::windows::core::Interface::as_raw(self), pszsessionid.into_param().abi(), pszsitename.into_param().abi(), pszusername.into_param().abi(), pszrole.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Finish_IsUserInRole(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).Finish_IsUserInRole)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(AsyncIFtpRoleProvider, ::windows::core::IUnknown);
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
}
impl ::core::clone::Clone for AsyncIFtpRoleProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for AsyncIFtpRoleProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3e83bf99_70ec_41ca_84b6_aca7c7a62caf);
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIFtpRoleProvider_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
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
    pub unsafe fn Begin_SinkNotify(&self, pcochangelist: &[MD_CHANGE_OBJECT_W]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Begin_SinkNotify)(::windows::core::Interface::as_raw(self), pcochangelist.len() as _, ::core::mem::transmute(pcochangelist.as_ptr())).ok()
    }
    pub unsafe fn Finish_SinkNotify(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Finish_SinkNotify)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Begin_ShutdownNotify(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Begin_ShutdownNotify)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Finish_ShutdownNotify(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Finish_ShutdownNotify)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(AsyncIMSAdminBaseSinkW, ::windows::core::IUnknown);
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
}
impl ::core::clone::Clone for AsyncIMSAdminBaseSinkW {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for AsyncIMSAdminBaseSinkW {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa9e69613_b80d_11d0_b9b9_00a0c922e750);
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIMSAdminBaseSinkW_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Begin_SinkNotify: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwmdnumelements: u32, pcochangelist: *const MD_CHANGE_OBJECT_W) -> ::windows::core::HRESULT,
    pub Finish_SinkNotify: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Begin_ShutdownNotify: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Finish_ShutdownNotify: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
#[repr(transparent)]
pub struct IADMEXT(::windows::core::IUnknown);
impl IADMEXT {
    pub unsafe fn Initialize(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Initialize)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn EnumDcomCLSIDs(&self, pclsiddcom: *mut ::windows::core::GUID, dwenumindex: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnumDcomCLSIDs)(::windows::core::Interface::as_raw(self), pclsiddcom, dwenumindex).ok()
    }
    pub unsafe fn Terminate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Terminate)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IADMEXT, ::windows::core::IUnknown);
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
}
impl ::core::clone::Clone for IADMEXT {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IADMEXT {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51dfe970_f6f2_11d0_b9bd_00a0c922e750);
}
#[repr(C)]
#[doc(hidden)]
pub struct IADMEXT_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EnumDcomCLSIDs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclsiddcom: *mut ::windows::core::GUID, dwenumindex: u32) -> ::windows::core::HRESULT,
    pub Terminate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
#[repr(transparent)]
pub struct IFtpAuthenticationProvider(::windows::core::IUnknown);
impl IFtpAuthenticationProvider {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AuthenticateUser<P0, P1, P2, P3>(&self, pszsessionid: P0, pszsitename: P1, pszusername: P2, pszpassword: P3, ppszcanonicalusername: *mut ::windows::core::PWSTR, pfauthenticated: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P3: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).AuthenticateUser)(::windows::core::Interface::as_raw(self), pszsessionid.into_param().abi(), pszsitename.into_param().abi(), pszusername.into_param().abi(), pszpassword.into_param().abi(), ppszcanonicalusername, pfauthenticated).ok()
    }
}
::windows::imp::interface_hierarchy!(IFtpAuthenticationProvider, ::windows::core::IUnknown);
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
}
impl ::core::clone::Clone for IFtpAuthenticationProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IFtpAuthenticationProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4659f95c_d5a8_4707_b2fc_6fd5794246cf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFtpAuthenticationProvider_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub AuthenticateUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszsessionid: ::windows::core::PCWSTR, pszsitename: ::windows::core::PCWSTR, pszusername: ::windows::core::PCWSTR, pszpassword: ::windows::core::PCWSTR, ppszcanonicalusername: *mut ::windows::core::PWSTR, pfauthenticated: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AuthenticateUser: usize,
}
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
#[repr(transparent)]
pub struct IFtpAuthorizationProvider(::windows::core::IUnknown);
impl IFtpAuthorizationProvider {
    pub unsafe fn GetUserAccessPermission<P0, P1, P2, P3>(&self, pszsessionid: P0, pszsitename: P1, pszvirtualpath: P2, pszusername: P3) -> ::windows::core::Result<FTP_ACCESS>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P3: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<FTP_ACCESS>();
        (::windows::core::Interface::vtable(self).GetUserAccessPermission)(::windows::core::Interface::as_raw(self), pszsessionid.into_param().abi(), pszsitename.into_param().abi(), pszvirtualpath.into_param().abi(), pszusername.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IFtpAuthorizationProvider, ::windows::core::IUnknown);
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
}
impl ::core::clone::Clone for IFtpAuthorizationProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IFtpAuthorizationProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa50ae7a1_a35a_42b4_a4f3_f4f7057a05d1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFtpAuthorizationProvider_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetUserAccessPermission: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszsessionid: ::windows::core::PCWSTR, pszsitename: ::windows::core::PCWSTR, pszvirtualpath: ::windows::core::PCWSTR, pszusername: ::windows::core::PCWSTR, pftpaccess: *mut FTP_ACCESS) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
#[repr(transparent)]
pub struct IFtpHomeDirectoryProvider(::windows::core::IUnknown);
impl IFtpHomeDirectoryProvider {
    pub unsafe fn GetUserHomeDirectoryData<P0, P1, P2>(&self, pszsessionid: P0, pszsitename: P1, pszusername: P2) -> ::windows::core::Result<::windows::core::PWSTR>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).GetUserHomeDirectoryData)(::windows::core::Interface::as_raw(self), pszsessionid.into_param().abi(), pszsitename.into_param().abi(), pszusername.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IFtpHomeDirectoryProvider, ::windows::core::IUnknown);
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
}
impl ::core::clone::Clone for IFtpHomeDirectoryProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IFtpHomeDirectoryProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0933b392_18dd_4097_8b9c_83325c35d9a6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFtpHomeDirectoryProvider_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetUserHomeDirectoryData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszsessionid: ::windows::core::PCWSTR, pszsitename: ::windows::core::PCWSTR, pszusername: ::windows::core::PCWSTR, ppszhomedirectorydata: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
#[repr(transparent)]
pub struct IFtpLogProvider(::windows::core::IUnknown);
impl IFtpLogProvider {
    pub unsafe fn Log(&self, ploggingparameters: *const LOGGING_PARAMETERS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Log)(::windows::core::Interface::as_raw(self), ploggingparameters).ok()
    }
}
::windows::imp::interface_hierarchy!(IFtpLogProvider, ::windows::core::IUnknown);
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
}
impl ::core::clone::Clone for IFtpLogProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IFtpLogProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa18a94cc_8299_4408_816c_7c3baca1a40e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFtpLogProvider_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Log: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ploggingparameters: *const LOGGING_PARAMETERS) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
#[repr(transparent)]
pub struct IFtpPostprocessProvider(::windows::core::IUnknown);
impl IFtpPostprocessProvider {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HandlePostprocess(&self, ppostprocessparameters: *const POST_PROCESS_PARAMETERS) -> ::windows::core::Result<FTP_PROCESS_STATUS> {
        let mut result__ = ::windows::core::zeroed::<FTP_PROCESS_STATUS>();
        (::windows::core::Interface::vtable(self).HandlePostprocess)(::windows::core::Interface::as_raw(self), ppostprocessparameters, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IFtpPostprocessProvider, ::windows::core::IUnknown);
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
}
impl ::core::clone::Clone for IFtpPostprocessProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IFtpPostprocessProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4522cbc6_16cd_49ad_8653_9a2c579e4280);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFtpPostprocessProvider_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub HandlePostprocess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppostprocessparameters: *const POST_PROCESS_PARAMETERS, pftpprocessstatus: *mut FTP_PROCESS_STATUS) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    HandlePostprocess: usize,
}
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
#[repr(transparent)]
pub struct IFtpPreprocessProvider(::windows::core::IUnknown);
impl IFtpPreprocessProvider {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HandlePreprocess(&self, ppreprocessparameters: *const PRE_PROCESS_PARAMETERS) -> ::windows::core::Result<FTP_PROCESS_STATUS> {
        let mut result__ = ::windows::core::zeroed::<FTP_PROCESS_STATUS>();
        (::windows::core::Interface::vtable(self).HandlePreprocess)(::windows::core::Interface::as_raw(self), ppreprocessparameters, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IFtpPreprocessProvider, ::windows::core::IUnknown);
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
}
impl ::core::clone::Clone for IFtpPreprocessProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IFtpPreprocessProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa3c19b60_5a28_471a_8f93_ab30411cee82);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFtpPreprocessProvider_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub HandlePreprocess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppreprocessparameters: *const PRE_PROCESS_PARAMETERS, pftpprocessstatus: *mut FTP_PROCESS_STATUS) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    HandlePreprocess: usize,
}
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
#[repr(transparent)]
pub struct IFtpProviderConstruct(::windows::core::IUnknown);
impl IFtpProviderConstruct {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Construct(&self, configurationentries: *const super::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Construct)(::windows::core::Interface::as_raw(self), configurationentries).ok()
    }
}
::windows::imp::interface_hierarchy!(IFtpProviderConstruct, ::windows::core::IUnknown);
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
}
impl ::core::clone::Clone for IFtpProviderConstruct {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IFtpProviderConstruct {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4d1a3f7b_412d_447c_b199_64f967e9a2da);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFtpProviderConstruct_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Construct: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, configurationentries: *const super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Construct: usize,
}
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
#[repr(transparent)]
pub struct IFtpRoleProvider(::windows::core::IUnknown);
impl IFtpRoleProvider {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsUserInRole<P0, P1, P2, P3>(&self, pszsessionid: P0, pszsitename: P1, pszusername: P2, pszrole: P3) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P3: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).IsUserInRole)(::windows::core::Interface::as_raw(self), pszsessionid.into_param().abi(), pszsitename.into_param().abi(), pszusername.into_param().abi(), pszrole.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IFtpRoleProvider, ::windows::core::IUnknown);
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
}
impl ::core::clone::Clone for IFtpRoleProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IFtpRoleProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x909c850d_8ca0_4674_96b8_cc2941535725);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFtpRoleProvider_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub IsUserInRole: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszsessionid: ::windows::core::PCWSTR, pszsitename: ::windows::core::PCWSTR, pszusername: ::windows::core::PCWSTR, pszrole: ::windows::core::PCWSTR, pfisinrole: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsUserInRole: usize,
}
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
#[repr(transparent)]
pub struct IMSAdminBase2W(::windows::core::IUnknown);
impl IMSAdminBase2W {
    pub unsafe fn AddKey<P0>(&self, hmdhandle: u32, pszmdpath: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.AddKey)(::windows::core::Interface::as_raw(self), hmdhandle, pszmdpath.into_param().abi()).ok()
    }
    pub unsafe fn DeleteKey<P0>(&self, hmdhandle: u32, pszmdpath: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.DeleteKey)(::windows::core::Interface::as_raw(self), hmdhandle, pszmdpath.into_param().abi()).ok()
    }
    pub unsafe fn DeleteChildKeys<P0>(&self, hmdhandle: u32, pszmdpath: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.DeleteChildKeys)(::windows::core::Interface::as_raw(self), hmdhandle, pszmdpath.into_param().abi()).ok()
    }
    pub unsafe fn EnumKeys<P0>(&self, hmdhandle: u32, pszmdpath: P0, pszmdname: &mut [u16; 256], dwmdenumobjectindex: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.EnumKeys)(::windows::core::Interface::as_raw(self), hmdhandle, pszmdpath.into_param().abi(), ::core::mem::transmute(pszmdname.as_ptr()), dwmdenumobjectindex).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CopyKey<P0, P1, P2, P3>(&self, hmdsourcehandle: u32, pszmdsourcepath: P0, hmddesthandle: u32, pszmddestpath: P1, bmdoverwriteflag: P2, bmdcopyflag: P3) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
        P3: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.CopyKey)(::windows::core::Interface::as_raw(self), hmdsourcehandle, pszmdsourcepath.into_param().abi(), hmddesthandle, pszmddestpath.into_param().abi(), bmdoverwriteflag.into_param().abi(), bmdcopyflag.into_param().abi()).ok()
    }
    pub unsafe fn RenameKey<P0, P1>(&self, hmdhandle: u32, pszmdpath: P0, pszmdnewname: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.RenameKey)(::windows::core::Interface::as_raw(self), hmdhandle, pszmdpath.into_param().abi(), pszmdnewname.into_param().abi()).ok()
    }
    pub unsafe fn SetData<P0>(&self, hmdhandle: u32, pszmdpath: P0, pmdrmddata: *mut METADATA_RECORD) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetData)(::windows::core::Interface::as_raw(self), hmdhandle, pszmdpath.into_param().abi(), pmdrmddata).ok()
    }
    pub unsafe fn GetData<P0>(&self, hmdhandle: u32, pszmdpath: P0, pmdrmddata: *mut METADATA_RECORD, pdwmdrequireddatalen: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.GetData)(::windows::core::Interface::as_raw(self), hmdhandle, pszmdpath.into_param().abi(), pmdrmddata, pdwmdrequireddatalen).ok()
    }
    pub unsafe fn DeleteData<P0>(&self, hmdhandle: u32, pszmdpath: P0, dwmdidentifier: u32, dwmddatatype: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.DeleteData)(::windows::core::Interface::as_raw(self), hmdhandle, pszmdpath.into_param().abi(), dwmdidentifier, dwmddatatype).ok()
    }
    pub unsafe fn EnumData<P0>(&self, hmdhandle: u32, pszmdpath: P0, pmdrmddata: *mut METADATA_RECORD, dwmdenumdataindex: u32, pdwmdrequireddatalen: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.EnumData)(::windows::core::Interface::as_raw(self), hmdhandle, pszmdpath.into_param().abi(), pmdrmddata, dwmdenumdataindex, pdwmdrequireddatalen).ok()
    }
    pub unsafe fn GetAllData<P0>(&self, hmdhandle: u32, pszmdpath: P0, dwmdattributes: u32, dwmdusertype: u32, dwmddatatype: u32, pdwmdnumdataentries: *mut u32, pdwmddatasetnumber: *mut u32, dwmdbuffersize: u32, pbmdbuffer: *mut u8, pdwmdrequiredbuffersize: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.GetAllData)(::windows::core::Interface::as_raw(self), hmdhandle, pszmdpath.into_param().abi(), dwmdattributes, dwmdusertype, dwmddatatype, pdwmdnumdataentries, pdwmddatasetnumber, dwmdbuffersize, pbmdbuffer, pdwmdrequiredbuffersize).ok()
    }
    pub unsafe fn DeleteAllData<P0>(&self, hmdhandle: u32, pszmdpath: P0, dwmdusertype: u32, dwmddatatype: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.DeleteAllData)(::windows::core::Interface::as_raw(self), hmdhandle, pszmdpath.into_param().abi(), dwmdusertype, dwmddatatype).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CopyData<P0, P1, P2>(&self, hmdsourcehandle: u32, pszmdsourcepath: P0, hmddesthandle: u32, pszmddestpath: P1, dwmdattributes: u32, dwmdusertype: u32, dwmddatatype: u32, bmdcopyflag: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.CopyData)(::windows::core::Interface::as_raw(self), hmdsourcehandle, pszmdsourcepath.into_param().abi(), hmddesthandle, pszmddestpath.into_param().abi(), dwmdattributes, dwmdusertype, dwmddatatype, bmdcopyflag.into_param().abi()).ok()
    }
    pub unsafe fn GetDataPaths<P0>(&self, hmdhandle: u32, pszmdpath: P0, dwmdidentifier: u32, dwmddatatype: u32, pszbuffer: &mut [u16], pdwmdrequiredbuffersize: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.GetDataPaths)(::windows::core::Interface::as_raw(self), hmdhandle, pszmdpath.into_param().abi(), dwmdidentifier, dwmddatatype, pszbuffer.len() as _, ::core::mem::transmute(pszbuffer.as_ptr()), pdwmdrequiredbuffersize).ok()
    }
    pub unsafe fn OpenKey<P0>(&self, hmdhandle: u32, pszmdpath: P0, dwmdaccessrequested: u32, dwmdtimeout: u32) -> ::windows::core::Result<u32>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.OpenKey)(::windows::core::Interface::as_raw(self), hmdhandle, pszmdpath.into_param().abi(), dwmdaccessrequested, dwmdtimeout, &mut result__).from_abi(result__)
    }
    pub unsafe fn CloseKey(&self, hmdhandle: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.CloseKey)(::windows::core::Interface::as_raw(self), hmdhandle).ok()
    }
    pub unsafe fn ChangePermissions(&self, hmdhandle: u32, dwmdtimeout: u32, dwmdaccessrequested: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ChangePermissions)(::windows::core::Interface::as_raw(self), hmdhandle, dwmdtimeout, dwmdaccessrequested).ok()
    }
    pub unsafe fn SaveData(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SaveData)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetHandleInfo(&self, hmdhandle: u32) -> ::windows::core::Result<METADATA_HANDLE_INFO> {
        let mut result__ = ::windows::core::zeroed::<METADATA_HANDLE_INFO>();
        (::windows::core::Interface::vtable(self).base__.GetHandleInfo)(::windows::core::Interface::as_raw(self), hmdhandle, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSystemChangeNumber(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetSystemChangeNumber)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDataSetNumber<P0>(&self, hmdhandle: u32, pszmdpath: P0) -> ::windows::core::Result<u32>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetDataSetNumber)(::windows::core::Interface::as_raw(self), hmdhandle, pszmdpath.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLastChangeTime<P0, P1>(&self, hmdhandle: u32, pszmdpath: P0, pftmdlastchangetime: *const super::super::Foundation::FILETIME, blocaltime: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.SetLastChangeTime)(::windows::core::Interface::as_raw(self), hmdhandle, pszmdpath.into_param().abi(), pftmdlastchangetime, blocaltime.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLastChangeTime<P0, P1>(&self, hmdhandle: u32, pszmdpath: P0, pftmdlastchangetime: *mut super::super::Foundation::FILETIME, blocaltime: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.GetLastChangeTime)(::windows::core::Interface::as_raw(self), hmdhandle, pszmdpath.into_param().abi(), pftmdlastchangetime, blocaltime.into_param().abi()).ok()
    }
    pub unsafe fn KeyExchangePhase1(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.KeyExchangePhase1)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn KeyExchangePhase2(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.KeyExchangePhase2)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Backup<P0>(&self, pszmdbackuplocation: P0, dwmdversion: u32, dwmdflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.Backup)(::windows::core::Interface::as_raw(self), pszmdbackuplocation.into_param().abi(), dwmdversion, dwmdflags).ok()
    }
    pub unsafe fn Restore<P0>(&self, pszmdbackuplocation: P0, dwmdversion: u32, dwmdflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.Restore)(::windows::core::Interface::as_raw(self), pszmdbackuplocation.into_param().abi(), dwmdversion, dwmdflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumBackups(&self, pszmdbackuplocation: &mut [u16; 256], pdwmdversion: *mut u32, pftmdbackuptime: *mut super::super::Foundation::FILETIME, dwmdenumindex: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.EnumBackups)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pszmdbackuplocation.as_ptr()), pdwmdversion, pftmdbackuptime, dwmdenumindex).ok()
    }
    pub unsafe fn DeleteBackup<P0>(&self, pszmdbackuplocation: P0, dwmdversion: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.DeleteBackup)(::windows::core::Interface::as_raw(self), pszmdbackuplocation.into_param().abi(), dwmdversion).ok()
    }
    pub unsafe fn UnmarshalInterface(&self) -> ::windows::core::Result<IMSAdminBaseW> {
        let mut result__ = ::windows::core::zeroed::<IMSAdminBaseW>();
        (::windows::core::Interface::vtable(self).base__.UnmarshalInterface)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetServerGuid(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetServerGuid)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn BackupWithPasswd<P0, P1>(&self, pszmdbackuplocation: P0, dwmdversion: u32, dwmdflags: u32, pszpasswd: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).BackupWithPasswd)(::windows::core::Interface::as_raw(self), pszmdbackuplocation.into_param().abi(), dwmdversion, dwmdflags, pszpasswd.into_param().abi()).ok()
    }
    pub unsafe fn RestoreWithPasswd<P0, P1>(&self, pszmdbackuplocation: P0, dwmdversion: u32, dwmdflags: u32, pszpasswd: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).RestoreWithPasswd)(::windows::core::Interface::as_raw(self), pszmdbackuplocation.into_param().abi(), dwmdversion, dwmdflags, pszpasswd.into_param().abi()).ok()
    }
    pub unsafe fn Export<P0, P1, P2>(&self, pszpasswd: P0, pszfilename: P1, pszsourcepath: P2, dwmdflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).Export)(::windows::core::Interface::as_raw(self), pszpasswd.into_param().abi(), pszfilename.into_param().abi(), pszsourcepath.into_param().abi(), dwmdflags).ok()
    }
    pub unsafe fn Import<P0, P1, P2, P3>(&self, pszpasswd: P0, pszfilename: P1, pszsourcepath: P2, pszdestpath: P3, dwmdflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P3: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).Import)(::windows::core::Interface::as_raw(self), pszpasswd.into_param().abi(), pszfilename.into_param().abi(), pszsourcepath.into_param().abi(), pszdestpath.into_param().abi(), dwmdflags).ok()
    }
    pub unsafe fn RestoreHistory<P0>(&self, pszmdhistorylocation: P0, dwmdmajorversion: u32, dwmdminorversion: u32, dwmdflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).RestoreHistory)(::windows::core::Interface::as_raw(self), pszmdhistorylocation.into_param().abi(), dwmdmajorversion, dwmdminorversion, dwmdflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumHistory(&self, pszmdhistorylocation: &mut [u16; 256], pdwmdmajorversion: *mut u32, pdwmdminorversion: *mut u32, pftmdhistorytime: *mut super::super::Foundation::FILETIME, dwmdenumindex: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnumHistory)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pszmdhistorylocation.as_ptr()), pdwmdmajorversion, pdwmdminorversion, pftmdhistorytime, dwmdenumindex).ok()
    }
}
::windows::imp::interface_hierarchy!(IMSAdminBase2W, ::windows::core::IUnknown, IMSAdminBaseW);
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
}
impl ::core::clone::Clone for IMSAdminBase2W {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMSAdminBase2W {
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
    pub unsafe fn AddKey<P0>(&self, hmdhandle: u32, pszmdpath: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.AddKey)(::windows::core::Interface::as_raw(self), hmdhandle, pszmdpath.into_param().abi()).ok()
    }
    pub unsafe fn DeleteKey<P0>(&self, hmdhandle: u32, pszmdpath: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.DeleteKey)(::windows::core::Interface::as_raw(self), hmdhandle, pszmdpath.into_param().abi()).ok()
    }
    pub unsafe fn DeleteChildKeys<P0>(&self, hmdhandle: u32, pszmdpath: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.DeleteChildKeys)(::windows::core::Interface::as_raw(self), hmdhandle, pszmdpath.into_param().abi()).ok()
    }
    pub unsafe fn EnumKeys<P0>(&self, hmdhandle: u32, pszmdpath: P0, pszmdname: &mut [u16; 256], dwmdenumobjectindex: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.EnumKeys)(::windows::core::Interface::as_raw(self), hmdhandle, pszmdpath.into_param().abi(), ::core::mem::transmute(pszmdname.as_ptr()), dwmdenumobjectindex).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CopyKey<P0, P1, P2, P3>(&self, hmdsourcehandle: u32, pszmdsourcepath: P0, hmddesthandle: u32, pszmddestpath: P1, bmdoverwriteflag: P2, bmdcopyflag: P3) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
        P3: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.CopyKey)(::windows::core::Interface::as_raw(self), hmdsourcehandle, pszmdsourcepath.into_param().abi(), hmddesthandle, pszmddestpath.into_param().abi(), bmdoverwriteflag.into_param().abi(), bmdcopyflag.into_param().abi()).ok()
    }
    pub unsafe fn RenameKey<P0, P1>(&self, hmdhandle: u32, pszmdpath: P0, pszmdnewname: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.RenameKey)(::windows::core::Interface::as_raw(self), hmdhandle, pszmdpath.into_param().abi(), pszmdnewname.into_param().abi()).ok()
    }
    pub unsafe fn SetData<P0>(&self, hmdhandle: u32, pszmdpath: P0, pmdrmddata: *mut METADATA_RECORD) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetData)(::windows::core::Interface::as_raw(self), hmdhandle, pszmdpath.into_param().abi(), pmdrmddata).ok()
    }
    pub unsafe fn GetData<P0>(&self, hmdhandle: u32, pszmdpath: P0, pmdrmddata: *mut METADATA_RECORD, pdwmdrequireddatalen: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.GetData)(::windows::core::Interface::as_raw(self), hmdhandle, pszmdpath.into_param().abi(), pmdrmddata, pdwmdrequireddatalen).ok()
    }
    pub unsafe fn DeleteData<P0>(&self, hmdhandle: u32, pszmdpath: P0, dwmdidentifier: u32, dwmddatatype: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.DeleteData)(::windows::core::Interface::as_raw(self), hmdhandle, pszmdpath.into_param().abi(), dwmdidentifier, dwmddatatype).ok()
    }
    pub unsafe fn EnumData<P0>(&self, hmdhandle: u32, pszmdpath: P0, pmdrmddata: *mut METADATA_RECORD, dwmdenumdataindex: u32, pdwmdrequireddatalen: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.EnumData)(::windows::core::Interface::as_raw(self), hmdhandle, pszmdpath.into_param().abi(), pmdrmddata, dwmdenumdataindex, pdwmdrequireddatalen).ok()
    }
    pub unsafe fn GetAllData<P0>(&self, hmdhandle: u32, pszmdpath: P0, dwmdattributes: u32, dwmdusertype: u32, dwmddatatype: u32, pdwmdnumdataentries: *mut u32, pdwmddatasetnumber: *mut u32, dwmdbuffersize: u32, pbmdbuffer: *mut u8, pdwmdrequiredbuffersize: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.GetAllData)(::windows::core::Interface::as_raw(self), hmdhandle, pszmdpath.into_param().abi(), dwmdattributes, dwmdusertype, dwmddatatype, pdwmdnumdataentries, pdwmddatasetnumber, dwmdbuffersize, pbmdbuffer, pdwmdrequiredbuffersize).ok()
    }
    pub unsafe fn DeleteAllData<P0>(&self, hmdhandle: u32, pszmdpath: P0, dwmdusertype: u32, dwmddatatype: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.DeleteAllData)(::windows::core::Interface::as_raw(self), hmdhandle, pszmdpath.into_param().abi(), dwmdusertype, dwmddatatype).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CopyData<P0, P1, P2>(&self, hmdsourcehandle: u32, pszmdsourcepath: P0, hmddesthandle: u32, pszmddestpath: P1, dwmdattributes: u32, dwmdusertype: u32, dwmddatatype: u32, bmdcopyflag: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.CopyData)(::windows::core::Interface::as_raw(self), hmdsourcehandle, pszmdsourcepath.into_param().abi(), hmddesthandle, pszmddestpath.into_param().abi(), dwmdattributes, dwmdusertype, dwmddatatype, bmdcopyflag.into_param().abi()).ok()
    }
    pub unsafe fn GetDataPaths<P0>(&self, hmdhandle: u32, pszmdpath: P0, dwmdidentifier: u32, dwmddatatype: u32, pszbuffer: &mut [u16], pdwmdrequiredbuffersize: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.GetDataPaths)(::windows::core::Interface::as_raw(self), hmdhandle, pszmdpath.into_param().abi(), dwmdidentifier, dwmddatatype, pszbuffer.len() as _, ::core::mem::transmute(pszbuffer.as_ptr()), pdwmdrequiredbuffersize).ok()
    }
    pub unsafe fn OpenKey<P0>(&self, hmdhandle: u32, pszmdpath: P0, dwmdaccessrequested: u32, dwmdtimeout: u32) -> ::windows::core::Result<u32>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.base__.OpenKey)(::windows::core::Interface::as_raw(self), hmdhandle, pszmdpath.into_param().abi(), dwmdaccessrequested, dwmdtimeout, &mut result__).from_abi(result__)
    }
    pub unsafe fn CloseKey(&self, hmdhandle: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.CloseKey)(::windows::core::Interface::as_raw(self), hmdhandle).ok()
    }
    pub unsafe fn ChangePermissions(&self, hmdhandle: u32, dwmdtimeout: u32, dwmdaccessrequested: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.ChangePermissions)(::windows::core::Interface::as_raw(self), hmdhandle, dwmdtimeout, dwmdaccessrequested).ok()
    }
    pub unsafe fn SaveData(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SaveData)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetHandleInfo(&self, hmdhandle: u32) -> ::windows::core::Result<METADATA_HANDLE_INFO> {
        let mut result__ = ::windows::core::zeroed::<METADATA_HANDLE_INFO>();
        (::windows::core::Interface::vtable(self).base__.base__.GetHandleInfo)(::windows::core::Interface::as_raw(self), hmdhandle, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSystemChangeNumber(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.base__.GetSystemChangeNumber)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDataSetNumber<P0>(&self, hmdhandle: u32, pszmdpath: P0) -> ::windows::core::Result<u32>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.base__.GetDataSetNumber)(::windows::core::Interface::as_raw(self), hmdhandle, pszmdpath.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLastChangeTime<P0, P1>(&self, hmdhandle: u32, pszmdpath: P0, pftmdlastchangetime: *const super::super::Foundation::FILETIME, blocaltime: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetLastChangeTime)(::windows::core::Interface::as_raw(self), hmdhandle, pszmdpath.into_param().abi(), pftmdlastchangetime, blocaltime.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLastChangeTime<P0, P1>(&self, hmdhandle: u32, pszmdpath: P0, pftmdlastchangetime: *mut super::super::Foundation::FILETIME, blocaltime: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.GetLastChangeTime)(::windows::core::Interface::as_raw(self), hmdhandle, pszmdpath.into_param().abi(), pftmdlastchangetime, blocaltime.into_param().abi()).ok()
    }
    pub unsafe fn KeyExchangePhase1(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.KeyExchangePhase1)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn KeyExchangePhase2(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.KeyExchangePhase2)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Backup<P0>(&self, pszmdbackuplocation: P0, dwmdversion: u32, dwmdflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.Backup)(::windows::core::Interface::as_raw(self), pszmdbackuplocation.into_param().abi(), dwmdversion, dwmdflags).ok()
    }
    pub unsafe fn Restore<P0>(&self, pszmdbackuplocation: P0, dwmdversion: u32, dwmdflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.Restore)(::windows::core::Interface::as_raw(self), pszmdbackuplocation.into_param().abi(), dwmdversion, dwmdflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumBackups(&self, pszmdbackuplocation: &mut [u16; 256], pdwmdversion: *mut u32, pftmdbackuptime: *mut super::super::Foundation::FILETIME, dwmdenumindex: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.EnumBackups)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pszmdbackuplocation.as_ptr()), pdwmdversion, pftmdbackuptime, dwmdenumindex).ok()
    }
    pub unsafe fn DeleteBackup<P0>(&self, pszmdbackuplocation: P0, dwmdversion: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.DeleteBackup)(::windows::core::Interface::as_raw(self), pszmdbackuplocation.into_param().abi(), dwmdversion).ok()
    }
    pub unsafe fn UnmarshalInterface(&self) -> ::windows::core::Result<IMSAdminBaseW> {
        let mut result__ = ::windows::core::zeroed::<IMSAdminBaseW>();
        (::windows::core::Interface::vtable(self).base__.base__.UnmarshalInterface)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetServerGuid(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetServerGuid)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn BackupWithPasswd<P0, P1>(&self, pszmdbackuplocation: P0, dwmdversion: u32, dwmdflags: u32, pszpasswd: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.BackupWithPasswd)(::windows::core::Interface::as_raw(self), pszmdbackuplocation.into_param().abi(), dwmdversion, dwmdflags, pszpasswd.into_param().abi()).ok()
    }
    pub unsafe fn RestoreWithPasswd<P0, P1>(&self, pszmdbackuplocation: P0, dwmdversion: u32, dwmdflags: u32, pszpasswd: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.RestoreWithPasswd)(::windows::core::Interface::as_raw(self), pszmdbackuplocation.into_param().abi(), dwmdversion, dwmdflags, pszpasswd.into_param().abi()).ok()
    }
    pub unsafe fn Export<P0, P1, P2>(&self, pszpasswd: P0, pszfilename: P1, pszsourcepath: P2, dwmdflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.Export)(::windows::core::Interface::as_raw(self), pszpasswd.into_param().abi(), pszfilename.into_param().abi(), pszsourcepath.into_param().abi(), dwmdflags).ok()
    }
    pub unsafe fn Import<P0, P1, P2, P3>(&self, pszpasswd: P0, pszfilename: P1, pszsourcepath: P2, pszdestpath: P3, dwmdflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P3: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.Import)(::windows::core::Interface::as_raw(self), pszpasswd.into_param().abi(), pszfilename.into_param().abi(), pszsourcepath.into_param().abi(), pszdestpath.into_param().abi(), dwmdflags).ok()
    }
    pub unsafe fn RestoreHistory<P0>(&self, pszmdhistorylocation: P0, dwmdmajorversion: u32, dwmdminorversion: u32, dwmdflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.RestoreHistory)(::windows::core::Interface::as_raw(self), pszmdhistorylocation.into_param().abi(), dwmdmajorversion, dwmdminorversion, dwmdflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumHistory(&self, pszmdhistorylocation: &mut [u16; 256], pdwmdmajorversion: *mut u32, pdwmdminorversion: *mut u32, pftmdhistorytime: *mut super::super::Foundation::FILETIME, dwmdenumindex: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.EnumHistory)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pszmdhistorylocation.as_ptr()), pdwmdmajorversion, pdwmdminorversion, pftmdhistorytime, dwmdenumindex).ok()
    }
    pub unsafe fn GetChildPaths<P0>(&self, hmdhandle: u32, pszmdpath: P0, pszbuffer: ::core::option::Option<&mut [u16]>, pcchmdrequiredbuffersize: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).GetChildPaths)(::windows::core::Interface::as_raw(self), hmdhandle, pszmdpath.into_param().abi(), pszbuffer.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(pszbuffer.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), ::core::mem::transmute(pcchmdrequiredbuffersize.unwrap_or(::std::ptr::null_mut()))).ok()
    }
}
::windows::imp::interface_hierarchy!(IMSAdminBase3W, ::windows::core::IUnknown, IMSAdminBaseW, IMSAdminBase2W);
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
}
impl ::core::clone::Clone for IMSAdminBase3W {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMSAdminBase3W {
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
    pub unsafe fn SinkNotify(&self, pcochangelist: &[MD_CHANGE_OBJECT_W]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SinkNotify)(::windows::core::Interface::as_raw(self), pcochangelist.len() as _, ::core::mem::transmute(pcochangelist.as_ptr())).ok()
    }
    pub unsafe fn ShutdownNotify(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ShutdownNotify)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IMSAdminBaseSinkW, ::windows::core::IUnknown);
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
}
impl ::core::clone::Clone for IMSAdminBaseSinkW {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMSAdminBaseSinkW {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa9e69612_b80d_11d0_b9b9_00a0c922e750);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSAdminBaseSinkW_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SinkNotify: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwmdnumelements: u32, pcochangelist: *const MD_CHANGE_OBJECT_W) -> ::windows::core::HRESULT,
    pub ShutdownNotify: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
#[repr(transparent)]
pub struct IMSAdminBaseW(::windows::core::IUnknown);
impl IMSAdminBaseW {
    pub unsafe fn AddKey<P0>(&self, hmdhandle: u32, pszmdpath: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).AddKey)(::windows::core::Interface::as_raw(self), hmdhandle, pszmdpath.into_param().abi()).ok()
    }
    pub unsafe fn DeleteKey<P0>(&self, hmdhandle: u32, pszmdpath: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).DeleteKey)(::windows::core::Interface::as_raw(self), hmdhandle, pszmdpath.into_param().abi()).ok()
    }
    pub unsafe fn DeleteChildKeys<P0>(&self, hmdhandle: u32, pszmdpath: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).DeleteChildKeys)(::windows::core::Interface::as_raw(self), hmdhandle, pszmdpath.into_param().abi()).ok()
    }
    pub unsafe fn EnumKeys<P0>(&self, hmdhandle: u32, pszmdpath: P0, pszmdname: &mut [u16; 256], dwmdenumobjectindex: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).EnumKeys)(::windows::core::Interface::as_raw(self), hmdhandle, pszmdpath.into_param().abi(), ::core::mem::transmute(pszmdname.as_ptr()), dwmdenumobjectindex).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CopyKey<P0, P1, P2, P3>(&self, hmdsourcehandle: u32, pszmdsourcepath: P0, hmddesthandle: u32, pszmddestpath: P1, bmdoverwriteflag: P2, bmdcopyflag: P3) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
        P3: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).CopyKey)(::windows::core::Interface::as_raw(self), hmdsourcehandle, pszmdsourcepath.into_param().abi(), hmddesthandle, pszmddestpath.into_param().abi(), bmdoverwriteflag.into_param().abi(), bmdcopyflag.into_param().abi()).ok()
    }
    pub unsafe fn RenameKey<P0, P1>(&self, hmdhandle: u32, pszmdpath: P0, pszmdnewname: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).RenameKey)(::windows::core::Interface::as_raw(self), hmdhandle, pszmdpath.into_param().abi(), pszmdnewname.into_param().abi()).ok()
    }
    pub unsafe fn SetData<P0>(&self, hmdhandle: u32, pszmdpath: P0, pmdrmddata: *mut METADATA_RECORD) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetData)(::windows::core::Interface::as_raw(self), hmdhandle, pszmdpath.into_param().abi(), pmdrmddata).ok()
    }
    pub unsafe fn GetData<P0>(&self, hmdhandle: u32, pszmdpath: P0, pmdrmddata: *mut METADATA_RECORD, pdwmdrequireddatalen: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).GetData)(::windows::core::Interface::as_raw(self), hmdhandle, pszmdpath.into_param().abi(), pmdrmddata, pdwmdrequireddatalen).ok()
    }
    pub unsafe fn DeleteData<P0>(&self, hmdhandle: u32, pszmdpath: P0, dwmdidentifier: u32, dwmddatatype: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).DeleteData)(::windows::core::Interface::as_raw(self), hmdhandle, pszmdpath.into_param().abi(), dwmdidentifier, dwmddatatype).ok()
    }
    pub unsafe fn EnumData<P0>(&self, hmdhandle: u32, pszmdpath: P0, pmdrmddata: *mut METADATA_RECORD, dwmdenumdataindex: u32, pdwmdrequireddatalen: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).EnumData)(::windows::core::Interface::as_raw(self), hmdhandle, pszmdpath.into_param().abi(), pmdrmddata, dwmdenumdataindex, pdwmdrequireddatalen).ok()
    }
    pub unsafe fn GetAllData<P0>(&self, hmdhandle: u32, pszmdpath: P0, dwmdattributes: u32, dwmdusertype: u32, dwmddatatype: u32, pdwmdnumdataentries: *mut u32, pdwmddatasetnumber: *mut u32, dwmdbuffersize: u32, pbmdbuffer: *mut u8, pdwmdrequiredbuffersize: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).GetAllData)(::windows::core::Interface::as_raw(self), hmdhandle, pszmdpath.into_param().abi(), dwmdattributes, dwmdusertype, dwmddatatype, pdwmdnumdataentries, pdwmddatasetnumber, dwmdbuffersize, pbmdbuffer, pdwmdrequiredbuffersize).ok()
    }
    pub unsafe fn DeleteAllData<P0>(&self, hmdhandle: u32, pszmdpath: P0, dwmdusertype: u32, dwmddatatype: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).DeleteAllData)(::windows::core::Interface::as_raw(self), hmdhandle, pszmdpath.into_param().abi(), dwmdusertype, dwmddatatype).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CopyData<P0, P1, P2>(&self, hmdsourcehandle: u32, pszmdsourcepath: P0, hmddesthandle: u32, pszmddestpath: P1, dwmdattributes: u32, dwmdusertype: u32, dwmddatatype: u32, bmdcopyflag: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).CopyData)(::windows::core::Interface::as_raw(self), hmdsourcehandle, pszmdsourcepath.into_param().abi(), hmddesthandle, pszmddestpath.into_param().abi(), dwmdattributes, dwmdusertype, dwmddatatype, bmdcopyflag.into_param().abi()).ok()
    }
    pub unsafe fn GetDataPaths<P0>(&self, hmdhandle: u32, pszmdpath: P0, dwmdidentifier: u32, dwmddatatype: u32, pszbuffer: &mut [u16], pdwmdrequiredbuffersize: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).GetDataPaths)(::windows::core::Interface::as_raw(self), hmdhandle, pszmdpath.into_param().abi(), dwmdidentifier, dwmddatatype, pszbuffer.len() as _, ::core::mem::transmute(pszbuffer.as_ptr()), pdwmdrequiredbuffersize).ok()
    }
    pub unsafe fn OpenKey<P0>(&self, hmdhandle: u32, pszmdpath: P0, dwmdaccessrequested: u32, dwmdtimeout: u32) -> ::windows::core::Result<u32>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).OpenKey)(::windows::core::Interface::as_raw(self), hmdhandle, pszmdpath.into_param().abi(), dwmdaccessrequested, dwmdtimeout, &mut result__).from_abi(result__)
    }
    pub unsafe fn CloseKey(&self, hmdhandle: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CloseKey)(::windows::core::Interface::as_raw(self), hmdhandle).ok()
    }
    pub unsafe fn ChangePermissions(&self, hmdhandle: u32, dwmdtimeout: u32, dwmdaccessrequested: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ChangePermissions)(::windows::core::Interface::as_raw(self), hmdhandle, dwmdtimeout, dwmdaccessrequested).ok()
    }
    pub unsafe fn SaveData(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SaveData)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetHandleInfo(&self, hmdhandle: u32) -> ::windows::core::Result<METADATA_HANDLE_INFO> {
        let mut result__ = ::windows::core::zeroed::<METADATA_HANDLE_INFO>();
        (::windows::core::Interface::vtable(self).GetHandleInfo)(::windows::core::Interface::as_raw(self), hmdhandle, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSystemChangeNumber(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetSystemChangeNumber)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDataSetNumber<P0>(&self, hmdhandle: u32, pszmdpath: P0) -> ::windows::core::Result<u32>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetDataSetNumber)(::windows::core::Interface::as_raw(self), hmdhandle, pszmdpath.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLastChangeTime<P0, P1>(&self, hmdhandle: u32, pszmdpath: P0, pftmdlastchangetime: *const super::super::Foundation::FILETIME, blocaltime: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetLastChangeTime)(::windows::core::Interface::as_raw(self), hmdhandle, pszmdpath.into_param().abi(), pftmdlastchangetime, blocaltime.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLastChangeTime<P0, P1>(&self, hmdhandle: u32, pszmdpath: P0, pftmdlastchangetime: *mut super::super::Foundation::FILETIME, blocaltime: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).GetLastChangeTime)(::windows::core::Interface::as_raw(self), hmdhandle, pszmdpath.into_param().abi(), pftmdlastchangetime, blocaltime.into_param().abi()).ok()
    }
    pub unsafe fn KeyExchangePhase1(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).KeyExchangePhase1)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn KeyExchangePhase2(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).KeyExchangePhase2)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Backup<P0>(&self, pszmdbackuplocation: P0, dwmdversion: u32, dwmdflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).Backup)(::windows::core::Interface::as_raw(self), pszmdbackuplocation.into_param().abi(), dwmdversion, dwmdflags).ok()
    }
    pub unsafe fn Restore<P0>(&self, pszmdbackuplocation: P0, dwmdversion: u32, dwmdflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).Restore)(::windows::core::Interface::as_raw(self), pszmdbackuplocation.into_param().abi(), dwmdversion, dwmdflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumBackups(&self, pszmdbackuplocation: &mut [u16; 256], pdwmdversion: *mut u32, pftmdbackuptime: *mut super::super::Foundation::FILETIME, dwmdenumindex: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnumBackups)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pszmdbackuplocation.as_ptr()), pdwmdversion, pftmdbackuptime, dwmdenumindex).ok()
    }
    pub unsafe fn DeleteBackup<P0>(&self, pszmdbackuplocation: P0, dwmdversion: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).DeleteBackup)(::windows::core::Interface::as_raw(self), pszmdbackuplocation.into_param().abi(), dwmdversion).ok()
    }
    pub unsafe fn UnmarshalInterface(&self) -> ::windows::core::Result<IMSAdminBaseW> {
        let mut result__ = ::windows::core::zeroed::<IMSAdminBaseW>();
        (::windows::core::Interface::vtable(self).UnmarshalInterface)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetServerGuid(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetServerGuid)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IMSAdminBaseW, ::windows::core::IUnknown);
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
}
impl ::core::clone::Clone for IMSAdminBaseW {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMSAdminBaseW {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x70b51430_b6ca_11d0_b9b9_00a0c922e750);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSAdminBaseW_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
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
    pub unsafe fn EnumeratePathsInFile<P0, P1>(&self, pszfilename: P0, pszkeytype: P1, pszbuffer: ::core::option::Option<&mut [u16]>, pdwmdrequiredbuffersize: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).EnumeratePathsInFile)(::windows::core::Interface::as_raw(self), pszfilename.into_param().abi(), pszkeytype.into_param().abi(), pszbuffer.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(pszbuffer.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pdwmdrequiredbuffersize).ok()
    }
}
::windows::imp::interface_hierarchy!(IMSImpExpHelpW, ::windows::core::IUnknown);
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
}
impl ::core::clone::Clone for IMSImpExpHelpW {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMSImpExpHelpW {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x29ff67ff_8050_480f_9f30_cc41635f2f9d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMSImpExpHelpW_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub EnumeratePathsInFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszfilename: ::windows::core::PCWSTR, pszkeytype: ::windows::core::PCWSTR, dwmdbuffersize: u32, pszbuffer: ::windows::core::PWSTR, pdwmdrequiredbuffersize: *mut u32) -> ::windows::core::HRESULT,
}
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
pub const CLSID_IImgCtx: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3050f3d6_98b5_11cf_bb82_00aa00bdce0b);
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const CLSID_IisServiceControl: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe8fb8621_588f_11d2_9d61_00c04f79c5fe);
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const CLSID_MSAdminBase_W: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa9e69610_b80d_11d0_b9b9_00a0c922e750);
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const CLSID_Request: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x920c25d0_25d9_11d0_a55f_00a0c90c2091);
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const CLSID_Response: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x46e19ba0_25dd_11d0_a55f_00a0c90c2091);
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const CLSID_ScriptingContext: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd97a6da0_a868_11cf_83ae_11b0c90c2bd8);
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const CLSID_Server: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa506d160_25e0_11d0_a55f_00a0c90c2091);
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const CLSID_Session: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x509f8f20_25de_11d0_a55f_00a0c90c2091);
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const CLSID_WamAdmin: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x61738644_f196_11d0_9953_00c04fd919c1);
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
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const FP_MD_ID_BEGIN_RESERVED: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const FP_MD_ID_END_RESERVED: u32 = 36863u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const FtpProvider: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x70bdc667_33b2_45f0_ac52_c3ca46f7a656);
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const GUID_IIS_ALL_TRACE_PROVIDERS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000000_0000_0000_0000_000000000000);
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const GUID_IIS_ASPNET_TRACE_PROVIDER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaff081fe_0247_4275_9c4e_021f3dc1da35);
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const GUID_IIS_ASP_TRACE_TRACE_PROVIDER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x06b94d9a_b15e_456e_a4ef_37c984a2cb4b);
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const GUID_IIS_ISAPI_TRACE_PROVIDER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa1c2040e_8840_4c31_ba11_9871031a19ea);
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const GUID_IIS_WWW_GLOBAL_TRACE_PROVIDER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd55d3bc9_cba9_44df_827e_132d3a4596c2);
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const GUID_IIS_WWW_SERVER_TRACE_PROVIDER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3a2a4e84_4c21_4981_ae10_3fda0d9b0f83);
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const GUID_IIS_WWW_SERVER_V2_TRACE_PROVIDER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xde4649c9_15e8_4fea_9d85_1cdda520c334);
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_APPEND_LOG_PARAMETER: u32 = 1003u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_APP_FLAG_IN_PROCESS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_APP_FLAG_ISOLATED_OOP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_APP_FLAG_POOLED_OOP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_EXEC_URL_DISABLE_CUSTOM_ERROR: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_EXEC_URL_HTTP_CACHE_ELIGIBLE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_EXEC_URL_IGNORE_CURRENT_INTERCEPTOR: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_EXEC_URL_IGNORE_VALIDATION_AND_RANGE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_EXEC_URL_NO_HEADERS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_EXEC_URL_SSI_CMD: u32 = 64u32;
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
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_VECTOR_ELEMENT_TYPE_FILE_HANDLE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_VECTOR_ELEMENT_TYPE_MEMORY_BUFFER: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_VERSION_MAJOR: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HSE_VERSION_MINOR: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HTTP_TRACE_EVENT_FLAG_STATIC_DESCRIPTIVE_FIELDS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HTTP_TRACE_LEVEL_END: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const HTTP_TRACE_LEVEL_START: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IISADMIN_EXTENSIONS_CLSID_MD_KEY: ::windows::core::PCWSTR = ::windows::w!("LM/IISADMIN/EXTENSIONS/DCOMCLSIDS");
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IISADMIN_EXTENSIONS_CLSID_MD_KEYA: ::windows::core::PCSTR = ::windows::s!("LM/IISADMIN/EXTENSIONS/DCOMCLSIDS");
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IISADMIN_EXTENSIONS_CLSID_MD_KEYW: ::windows::core::PCWSTR = ::windows::w!("LM/IISADMIN/EXTENSIONS/DCOMCLSIDS");
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IISADMIN_EXTENSIONS_REG_KEY: ::windows::core::PCWSTR = ::windows::w!("SOFTWARE\\Microsoft\\InetStp\\Extensions");
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IISADMIN_EXTENSIONS_REG_KEYA: ::windows::core::PCSTR = ::windows::s!("SOFTWARE\\Microsoft\\InetStp\\Extensions");
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IISADMIN_EXTENSIONS_REG_KEYW: ::windows::core::PCWSTR = ::windows::w!("SOFTWARE\\Microsoft\\InetStp\\Extensions");
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_CLASS_CERTMAPPER: ::windows::core::PCSTR = ::windows::s!("IIsCertMapper");
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_CLASS_CERTMAPPER_W: ::windows::core::PCWSTR = ::windows::w!("IIsCertMapper");
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_CLASS_COMPRESS_SCHEME: ::windows::core::PCSTR = ::windows::s!("IIsCompressionScheme");
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_CLASS_COMPRESS_SCHEMES: ::windows::core::PCSTR = ::windows::s!("IIsCompressionSchemes");
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_CLASS_COMPRESS_SCHEMES_W: ::windows::core::PCWSTR = ::windows::w!("IIsCompressionSchemes");
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_CLASS_COMPRESS_SCHEME_W: ::windows::core::PCWSTR = ::windows::w!("IIsCompressionScheme");
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_CLASS_COMPUTER: ::windows::core::PCSTR = ::windows::s!("IIsComputer");
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_CLASS_COMPUTER_W: ::windows::core::PCWSTR = ::windows::w!("IIsComputer");
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_CLASS_FILTER: ::windows::core::PCSTR = ::windows::s!("IIsFilter");
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_CLASS_FILTERS: ::windows::core::PCSTR = ::windows::s!("IIsFilters");
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_CLASS_FILTERS_W: ::windows::core::PCWSTR = ::windows::w!("IIsFilters");
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_CLASS_FILTER_W: ::windows::core::PCWSTR = ::windows::w!("IIsFilter");
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_CLASS_FTP_INFO: ::windows::core::PCSTR = ::windows::s!("IIsFtpInfo");
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_CLASS_FTP_INFO_W: ::windows::core::PCWSTR = ::windows::w!("IIsFtpInfo");
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_CLASS_FTP_SERVER: ::windows::core::PCSTR = ::windows::s!("IIsFtpServer");
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_CLASS_FTP_SERVER_W: ::windows::core::PCWSTR = ::windows::w!("IIsFtpServer");
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_CLASS_FTP_SERVICE: ::windows::core::PCSTR = ::windows::s!("IIsFtpService");
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_CLASS_FTP_SERVICE_W: ::windows::core::PCWSTR = ::windows::w!("IIsFtpService");
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_CLASS_FTP_VDIR: ::windows::core::PCSTR = ::windows::s!("IIsFtpVirtualDir");
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_CLASS_FTP_VDIR_W: ::windows::core::PCWSTR = ::windows::w!("IIsFtpVirtualDir");
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_CLASS_LOG_MODULE: ::windows::core::PCSTR = ::windows::s!("IIsLogModule");
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_CLASS_LOG_MODULES: ::windows::core::PCSTR = ::windows::s!("IIsLogModules");
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_CLASS_LOG_MODULES_W: ::windows::core::PCWSTR = ::windows::w!("IIsLogModules");
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_CLASS_LOG_MODULE_W: ::windows::core::PCWSTR = ::windows::w!("IIsLogModule");
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_CLASS_MIMEMAP: ::windows::core::PCSTR = ::windows::s!("IIsMimeMap");
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_CLASS_MIMEMAP_W: ::windows::core::PCWSTR = ::windows::w!("IIsMimeMap");
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_CLASS_WEB_DIR: ::windows::core::PCSTR = ::windows::s!("IIsWebDirectory");
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_CLASS_WEB_DIR_W: ::windows::core::PCWSTR = ::windows::w!("IIsWebDirectory");
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_CLASS_WEB_FILE: ::windows::core::PCSTR = ::windows::s!("IIsWebFile");
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_CLASS_WEB_FILE_W: ::windows::core::PCWSTR = ::windows::w!("IIsWebFile");
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_CLASS_WEB_INFO: ::windows::core::PCSTR = ::windows::s!("IIsWebInfo");
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_CLASS_WEB_INFO_W: ::windows::core::PCWSTR = ::windows::w!("IIsWebInfo");
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_CLASS_WEB_SERVER: ::windows::core::PCSTR = ::windows::s!("IIsWebServer");
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_CLASS_WEB_SERVER_W: ::windows::core::PCWSTR = ::windows::w!("IIsWebServer");
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_CLASS_WEB_SERVICE: ::windows::core::PCSTR = ::windows::s!("IIsWebService");
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_CLASS_WEB_SERVICE_W: ::windows::core::PCWSTR = ::windows::w!("IIsWebService");
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_CLASS_WEB_VDIR: ::windows::core::PCSTR = ::windows::s!("IIsWebVirtualDir");
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_CLASS_WEB_VDIR_W: ::windows::core::PCWSTR = ::windows::w!("IIsWebVirtualDir");
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_MD_ADSI_METAID_BEGIN: u32 = 130000u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_MD_ADSI_SCHEMA_PATH_A: ::windows::core::PCSTR = ::windows::s!("/Schema");
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_MD_ADSI_SCHEMA_PATH_W: ::windows::core::PCWSTR = ::windows::w!("/Schema");
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
pub const IIS_MD_INSTANCE_ROOT: ::windows::core::PCSTR = ::windows::s!("Root");
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_MD_ISAPI_FILTERS: ::windows::core::PCSTR = ::windows::s!("/Filters");
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_MD_LOCAL_MACHINE_PATH: ::windows::core::PCSTR = ::windows::s!("LM");
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
pub const IIS_MD_SVC_INFO_PATH: ::windows::core::PCSTR = ::windows::s!("Info");
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
pub const IIS_WEBSOCKET: ::windows::core::PCWSTR = ::windows::w!("websockets");
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const IIS_WEBSOCKET_SERVER_VARIABLE: ::windows::core::PCSTR = ::windows::s!("IIS_WEBSOCK");
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
pub const LIBID_ASPTypeLibrary: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd97a6da0_a85c_11cf_83ae_00a0c90c2bd8);
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const LIBID_IISRSTALib: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe8fb8614_588f_11d2_9d61_00c04f79c5fe);
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const LIBID_WAMREGLib: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x29822aa8_f302_11d0_9953_00c04fd919c1);
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
pub const MD_DEFAULT_BACKUP_LOCATION: ::windows::core::PCWSTR = ::windows::w!("MDBackUp");
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
pub const MD_INSERT_PATH_STRING: ::windows::core::PCWSTR = ::windows::w!("<%INSERT_PATH%>");
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const MD_INSERT_PATH_STRINGA: ::windows::core::PCSTR = ::windows::s!("<%INSERT_PATH%>");
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
pub const METADATA_DONT_EXPAND: u32 = 512u32;
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
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const POP3_MD_ID_BEGIN_RESERVED: u32 = 40960u32;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const POP3_MD_ID_END_RESERVED: u32 = 45055u32;
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
pub const WEB_CORE_ACTIVATE_DLL_ENTRY: ::windows::core::PCSTR = ::windows::s!("WebCoreActivate");
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const WEB_CORE_DLL_NAME: ::windows::core::PCWSTR = ::windows::w!("hwebcore.dll");
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const WEB_CORE_SET_METADATA_DLL_ENTRY: ::windows::core::PCSTR = ::windows::s!("WebCoreSetMetadata");
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub const WEB_CORE_SHUTDOWN_DLL_ENTRY: ::windows::core::PCSTR = ::windows::s!("WebCoreShutdown");
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::windows::core::TypeKind for FTP_ACCESS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for FTP_ACCESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FTP_ACCESS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::windows::core::TypeKind for FTP_PROCESS_STATUS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for FTP_PROCESS_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FTP_PROCESS_STATUS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::windows::core::TypeKind for HTTP_TRACE_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for HTTP_TRACE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTTP_TRACE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::windows::core::TypeKind for METADATATYPES {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for METADATATYPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("METADATATYPES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::windows::core::TypeKind for SF_PROPERTY_IIS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SF_PROPERTY_IIS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SF_PROPERTY_IIS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::windows::core::TypeKind for SF_REQ_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SF_REQ_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SF_REQ_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::windows::core::TypeKind for SF_STATUS_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SF_STATUS_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SF_STATUS_TYPE").field(&self.0).finish()
    }
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
impl ::windows::core::TypeKind for CERT_CONTEXT_EX {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::PartialEq for CERT_CONTEXT_EX {
    fn eq(&self, other: &Self) -> bool {
        self.CertContext == other.CertContext && self.cbAllocated == other.cbAllocated && self.dwCertificateFlags == other.dwCertificateFlags
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
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub struct CONFIGURATION_ENTRY {
    pub bstrKey: ::std::mem::ManuallyDrop<::windows::core::BSTR>,
    pub bstrValue: ::std::mem::ManuallyDrop<::windows::core::BSTR>,
}
impl ::core::clone::Clone for CONFIGURATION_ENTRY {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
impl ::core::fmt::Debug for CONFIGURATION_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CONFIGURATION_ENTRY").field("bstrKey", &self.bstrKey).field("bstrValue", &self.bstrValue).finish()
    }
}
impl ::windows::core::TypeKind for CONFIGURATION_ENTRY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for CONFIGURATION_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.bstrKey == other.bstrKey && self.bstrValue == other.bstrValue
    }
}
impl ::core::cmp::Eq for CONFIGURATION_ENTRY {}
impl ::core::default::Default for CONFIGURATION_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub struct EXTENSION_CONTROL_BLOCK {
    pub cbSize: u32,
    pub dwVersion: u32,
    pub ConnID: *mut ::core::ffi::c_void,
    pub dwHttpStatusCode: u32,
    pub lpszLogData: [u8; 80],
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
impl ::core::marker::Copy for EXTENSION_CONTROL_BLOCK {}
impl ::core::clone::Clone for EXTENSION_CONTROL_BLOCK {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::windows::core::TypeKind for EXTENSION_CONTROL_BLOCK {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for EXTENSION_CONTROL_BLOCK {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwVersion == other.dwVersion && self.ConnID == other.ConnID && self.dwHttpStatusCode == other.dwHttpStatusCode && self.lpszLogData == other.lpszLogData && self.lpszMethod == other.lpszMethod && self.lpszQueryString == other.lpszQueryString && self.lpszPathInfo == other.lpszPathInfo && self.lpszPathTranslated == other.lpszPathTranslated && self.cbTotalBytes == other.cbTotalBytes && self.cbAvailable == other.cbAvailable && self.lpbData == other.lpbData && self.lpszContentType == other.lpszContentType && self.GetServerVariable == other.GetServerVariable && self.WriteClient == other.WriteClient && self.ReadClient == other.ReadClient && self.ServerSupportFunction == other.ServerSupportFunction
    }
}
impl ::core::cmp::Eq for EXTENSION_CONTROL_BLOCK {}
impl ::core::default::Default for EXTENSION_CONTROL_BLOCK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
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
impl ::windows::core::TypeKind for HSE_CUSTOM_ERROR_INFO {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HSE_CUSTOM_ERROR_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.pszStatus == other.pszStatus && self.uHttpSubError == other.uHttpSubError && self.fAsync == other.fAsync
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
impl ::windows::core::TypeKind for HSE_EXEC_UNICODE_URL_INFO {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HSE_EXEC_UNICODE_URL_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.pszUrl == other.pszUrl && self.pszMethod == other.pszMethod && self.pszChildHeaders == other.pszChildHeaders && self.pUserInfo == other.pUserInfo && self.pEntity == other.pEntity && self.dwExecUrlFlags == other.dwExecUrlFlags
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
impl ::windows::core::TypeKind for HSE_EXEC_UNICODE_URL_USER_INFO {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HSE_EXEC_UNICODE_URL_USER_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.hImpersonationToken == other.hImpersonationToken && self.pszCustomUserName == other.pszCustomUserName && self.pszCustomAuthType == other.pszCustomAuthType
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
impl ::windows::core::TypeKind for HSE_EXEC_URL_ENTITY_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for HSE_EXEC_URL_ENTITY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbAvailable == other.cbAvailable && self.lpbData == other.lpbData
    }
}
impl ::core::cmp::Eq for HSE_EXEC_URL_ENTITY_INFO {}
impl ::core::default::Default for HSE_EXEC_URL_ENTITY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
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
impl ::windows::core::TypeKind for HSE_EXEC_URL_INFO {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HSE_EXEC_URL_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.pszUrl == other.pszUrl && self.pszMethod == other.pszMethod && self.pszChildHeaders == other.pszChildHeaders && self.pUserInfo == other.pUserInfo && self.pEntity == other.pEntity && self.dwExecUrlFlags == other.dwExecUrlFlags
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
impl ::windows::core::TypeKind for HSE_EXEC_URL_STATUS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for HSE_EXEC_URL_STATUS {
    fn eq(&self, other: &Self) -> bool {
        self.uHttpStatusCode == other.uHttpStatusCode && self.uHttpSubStatus == other.uHttpSubStatus && self.dwWin32Error == other.dwWin32Error
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
impl ::windows::core::TypeKind for HSE_EXEC_URL_USER_INFO {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HSE_EXEC_URL_USER_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.hImpersonationToken == other.hImpersonationToken && self.pszCustomUserName == other.pszCustomUserName && self.pszCustomAuthType == other.pszCustomAuthType
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
impl ::windows::core::TypeKind for HSE_RESPONSE_VECTOR {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for HSE_RESPONSE_VECTOR {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags && self.pszStatus == other.pszStatus && self.pszHeaders == other.pszHeaders && self.nElementCount == other.nElementCount && self.lpElementArray == other.lpElementArray
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
impl ::windows::core::TypeKind for HSE_SEND_HEADER_EX_INFO {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HSE_SEND_HEADER_EX_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.pszStatus == other.pszStatus && self.pszHeader == other.pszHeader && self.cchStatus == other.cchStatus && self.cchHeader == other.cchHeader && self.fKeepConn == other.fKeepConn
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
        f.debug_struct("HSE_TF_INFO").field("pContext", &self.pContext).field("hFile", &self.hFile).field("pszStatusCode", &self.pszStatusCode).field("BytesToWrite", &self.BytesToWrite).field("Offset", &self.Offset).field("pHead", &self.pHead).field("HeadLength", &self.HeadLength).field("pTail", &self.pTail).field("TailLength", &self.TailLength).field("dwFlags", &self.dwFlags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for HSE_TF_INFO {
    type TypeKind = ::windows::core::CopyType;
}
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
impl ::windows::core::TypeKind for HSE_TRACE_INFO {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HSE_TRACE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.fTraceRequest == other.fTraceRequest && self.TraceContextId == other.TraceContextId && self.dwReserved1 == other.dwReserved1 && self.dwReserved2 == other.dwReserved2
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
impl ::windows::core::TypeKind for HSE_UNICODE_URL_MAPEX_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for HSE_UNICODE_URL_MAPEX_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.lpszPath == other.lpszPath && self.dwFlags == other.dwFlags && self.cchMatchingPath == other.cchMatchingPath && self.cchMatchingURL == other.cchMatchingURL
    }
}
impl ::core::cmp::Eq for HSE_UNICODE_URL_MAPEX_INFO {}
impl ::core::default::Default for HSE_UNICODE_URL_MAPEX_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub struct HSE_URL_MAPEX_INFO {
    pub lpszPath: [u8; 260],
    pub dwFlags: u32,
    pub cchMatchingPath: u32,
    pub cchMatchingURL: u32,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
}
impl ::core::marker::Copy for HSE_URL_MAPEX_INFO {}
impl ::core::clone::Clone for HSE_URL_MAPEX_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HSE_URL_MAPEX_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HSE_URL_MAPEX_INFO").field("lpszPath", &self.lpszPath).field("dwFlags", &self.dwFlags).field("cchMatchingPath", &self.cchMatchingPath).field("cchMatchingURL", &self.cchMatchingURL).field("dwReserved1", &self.dwReserved1).field("dwReserved2", &self.dwReserved2).finish()
    }
}
impl ::windows::core::TypeKind for HSE_URL_MAPEX_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for HSE_URL_MAPEX_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.lpszPath == other.lpszPath && self.dwFlags == other.dwFlags && self.cchMatchingPath == other.cchMatchingPath && self.cchMatchingURL == other.cchMatchingURL && self.dwReserved1 == other.dwReserved1 && self.dwReserved2 == other.dwReserved2
    }
}
impl ::core::cmp::Eq for HSE_URL_MAPEX_INFO {}
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
impl ::windows::core::TypeKind for HSE_VECTOR_ELEMENT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for HSE_VECTOR_ELEMENT {
    fn eq(&self, other: &Self) -> bool {
        self.ElementType == other.ElementType && self.pvContext == other.pvContext && self.cbOffset == other.cbOffset && self.cbSize == other.cbSize
    }
}
impl ::core::cmp::Eq for HSE_VECTOR_ELEMENT {}
impl ::core::default::Default for HSE_VECTOR_ELEMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub struct HSE_VERSION_INFO {
    pub dwExtensionVersion: u32,
    pub lpszExtensionDesc: [u8; 256],
}
impl ::core::marker::Copy for HSE_VERSION_INFO {}
impl ::core::clone::Clone for HSE_VERSION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HSE_VERSION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HSE_VERSION_INFO").field("dwExtensionVersion", &self.dwExtensionVersion).field("lpszExtensionDesc", &self.lpszExtensionDesc).finish()
    }
}
impl ::windows::core::TypeKind for HSE_VERSION_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for HSE_VERSION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwExtensionVersion == other.dwExtensionVersion && self.lpszExtensionDesc == other.lpszExtensionDesc
    }
}
impl ::core::cmp::Eq for HSE_VERSION_INFO {}
impl ::core::default::Default for HSE_VERSION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
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
impl ::windows::core::TypeKind for HTTP_FILTER_ACCESS_DENIED {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for HTTP_FILTER_ACCESS_DENIED {
    fn eq(&self, other: &Self) -> bool {
        self.pszURL == other.pszURL && self.pszPhysicalPath == other.pszPhysicalPath && self.dwReason == other.dwReason
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
impl ::windows::core::TypeKind for HTTP_FILTER_AUTHENT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for HTTP_FILTER_AUTHENT {
    fn eq(&self, other: &Self) -> bool {
        self.pszUser == other.pszUser && self.cbUserBuff == other.cbUserBuff && self.pszPassword == other.pszPassword && self.cbPasswordBuff == other.cbPasswordBuff
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
impl ::windows::core::TypeKind for HTTP_FILTER_AUTH_COMPLETE_INFO {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HTTP_FILTER_AUTH_COMPLETE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.GetHeader == other.GetHeader && self.SetHeader == other.SetHeader && self.AddHeader == other.AddHeader && self.GetUserToken == other.GetUserToken && self.HttpStatus == other.HttpStatus && self.fResetAuth == other.fResetAuth && self.dwReserved == other.dwReserved
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
impl ::windows::core::TypeKind for HTTP_FILTER_CONTEXT {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HTTP_FILTER_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.Revision == other.Revision && self.ServerContext == other.ServerContext && self.ulReserved == other.ulReserved && self.fIsSecurePort == other.fIsSecurePort && self.pFilterContext == other.pFilterContext && self.GetServerVariable == other.GetServerVariable && self.AddResponseHeaders == other.AddResponseHeaders && self.WriteClient == other.WriteClient && self.AllocMem == other.AllocMem && self.ServerSupportFunction == other.ServerSupportFunction
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
impl ::windows::core::TypeKind for HTTP_FILTER_LOG {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for HTTP_FILTER_LOG {
    fn eq(&self, other: &Self) -> bool {
        self.pszClientHostName == other.pszClientHostName && self.pszClientUserName == other.pszClientUserName && self.pszServerName == other.pszServerName && self.pszOperation == other.pszOperation && self.pszTarget == other.pszTarget && self.pszParameters == other.pszParameters && self.dwHttpStatus == other.dwHttpStatus && self.dwWin32Status == other.dwWin32Status && self.dwBytesSent == other.dwBytesSent && self.dwBytesRecvd == other.dwBytesRecvd && self.msTimeForProcessing == other.msTimeForProcessing
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
impl ::windows::core::TypeKind for HTTP_FILTER_PREPROC_HEADERS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for HTTP_FILTER_PREPROC_HEADERS {
    fn eq(&self, other: &Self) -> bool {
        self.GetHeader == other.GetHeader && self.SetHeader == other.SetHeader && self.AddHeader == other.AddHeader && self.HttpStatus == other.HttpStatus && self.dwReserved == other.dwReserved
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
impl ::windows::core::TypeKind for HTTP_FILTER_RAW_DATA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for HTTP_FILTER_RAW_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.pvInData == other.pvInData && self.cbInData == other.cbInData && self.cbInBuffer == other.cbInBuffer && self.dwReserved == other.dwReserved
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
impl ::windows::core::TypeKind for HTTP_FILTER_URL_MAP {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for HTTP_FILTER_URL_MAP {
    fn eq(&self, other: &Self) -> bool {
        self.pszURL == other.pszURL && self.pszPhysicalPath == other.pszPhysicalPath && self.cbPathBuff == other.cbPathBuff
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
impl ::windows::core::TypeKind for HTTP_FILTER_URL_MAP_EX {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for HTTP_FILTER_URL_MAP_EX {
    fn eq(&self, other: &Self) -> bool {
        self.pszURL == other.pszURL && self.pszPhysicalPath == other.pszPhysicalPath && self.cbPathBuff == other.cbPathBuff && self.dwFlags == other.dwFlags && self.cchMatchingPath == other.cchMatchingPath && self.cchMatchingURL == other.cchMatchingURL && self.pszScriptMapEntry == other.pszScriptMapEntry
    }
}
impl ::core::cmp::Eq for HTTP_FILTER_URL_MAP_EX {}
impl ::core::default::Default for HTTP_FILTER_URL_MAP_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub struct HTTP_FILTER_VERSION {
    pub dwServerFilterVersion: u32,
    pub dwFilterVersion: u32,
    pub lpszFilterDesc: [u8; 257],
    pub dwFlags: u32,
}
impl ::core::marker::Copy for HTTP_FILTER_VERSION {}
impl ::core::clone::Clone for HTTP_FILTER_VERSION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HTTP_FILTER_VERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTTP_FILTER_VERSION").field("dwServerFilterVersion", &self.dwServerFilterVersion).field("dwFilterVersion", &self.dwFilterVersion).field("lpszFilterDesc", &self.lpszFilterDesc).field("dwFlags", &self.dwFlags).finish()
    }
}
impl ::windows::core::TypeKind for HTTP_FILTER_VERSION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for HTTP_FILTER_VERSION {
    fn eq(&self, other: &Self) -> bool {
        self.dwServerFilterVersion == other.dwServerFilterVersion && self.dwFilterVersion == other.dwFilterVersion && self.lpszFilterDesc == other.lpszFilterDesc && self.dwFlags == other.dwFlags
    }
}
impl ::core::cmp::Eq for HTTP_FILTER_VERSION {}
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
impl ::windows::core::TypeKind for HTTP_TRACE_CONFIGURATION {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HTTP_TRACE_CONFIGURATION {
    fn eq(&self, other: &Self) -> bool {
        self.pProviderGuid == other.pProviderGuid && self.dwAreas == other.dwAreas && self.dwVerbosity == other.dwVerbosity && self.fProviderEnabled == other.fProviderEnabled
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
impl ::windows::core::TypeKind for HTTP_TRACE_EVENT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for HTTP_TRACE_EVENT {
    fn eq(&self, other: &Self) -> bool {
        self.pProviderGuid == other.pProviderGuid && self.dwArea == other.dwArea && self.pAreaGuid == other.pAreaGuid && self.dwEvent == other.dwEvent && self.pszEventName == other.pszEventName && self.dwEventVersion == other.dwEventVersion && self.dwVerbosity == other.dwVerbosity && self.pActivityGuid == other.pActivityGuid && self.pRelatedActivityGuid == other.pRelatedActivityGuid && self.dwTimeStamp == other.dwTimeStamp && self.dwFlags == other.dwFlags && self.cEventItems == other.cEventItems && self.pEventItems == other.pEventItems
    }
}
impl ::core::cmp::Eq for HTTP_TRACE_EVENT {}
impl ::core::default::Default for HTTP_TRACE_EVENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
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
impl ::windows::core::TypeKind for HTTP_TRACE_EVENT_ITEM {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for HTTP_TRACE_EVENT_ITEM {
    fn eq(&self, other: &Self) -> bool {
        self.pszName == other.pszName && self.dwDataType == other.dwDataType && self.pbData == other.pbData && self.cbData == other.cbData && self.pszDataDescription == other.pszDataDescription
    }
}
impl ::core::cmp::Eq for HTTP_TRACE_EVENT_ITEM {}
impl ::core::default::Default for HTTP_TRACE_EVENT_ITEM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
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
impl ::windows::core::TypeKind for LOGGING_PARAMETERS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for LOGGING_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.pszSessionId == other.pszSessionId
            && self.pszSiteName == other.pszSiteName
            && self.pszUserName == other.pszUserName
            && self.pszHostName == other.pszHostName
            && self.pszRemoteIpAddress == other.pszRemoteIpAddress
            && self.dwRemoteIpPort == other.dwRemoteIpPort
            && self.pszLocalIpAddress == other.pszLocalIpAddress
            && self.dwLocalIpPort == other.dwLocalIpPort
            && self.BytesSent == other.BytesSent
            && self.BytesReceived == other.BytesReceived
            && self.pszCommand == other.pszCommand
            && self.pszCommandParameters == other.pszCommandParameters
            && self.pszFullPath == other.pszFullPath
            && self.dwElapsedMilliseconds == other.dwElapsedMilliseconds
            && self.FtpStatus == other.FtpStatus
            && self.FtpSubStatus == other.FtpSubStatus
            && self.hrStatus == other.hrStatus
            && self.pszInformation == other.pszInformation
    }
}
impl ::core::cmp::Eq for LOGGING_PARAMETERS {}
impl ::core::default::Default for LOGGING_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
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
impl ::windows::core::TypeKind for MD_CHANGE_OBJECT_W {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MD_CHANGE_OBJECT_W {
    fn eq(&self, other: &Self) -> bool {
        self.pszMDPath == other.pszMDPath && self.dwMDChangeType == other.dwMDChangeType && self.dwMDNumDataIDs == other.dwMDNumDataIDs && self.pdwMDDataIDs == other.pdwMDDataIDs
    }
}
impl ::core::cmp::Eq for MD_CHANGE_OBJECT_W {}
impl ::core::default::Default for MD_CHANGE_OBJECT_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
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
impl ::windows::core::TypeKind for METADATA_GETALL_INTERNAL_RECORD {
    type TypeKind = ::windows::core::CopyType;
}
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
impl ::windows::core::TypeKind for METADATA_GETALL_INTERNAL_RECORD_0 {
    type TypeKind = ::windows::core::CopyType;
}
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
impl ::windows::core::TypeKind for METADATA_GETALL_RECORD {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for METADATA_GETALL_RECORD {
    fn eq(&self, other: &Self) -> bool {
        self.dwMDIdentifier == other.dwMDIdentifier && self.dwMDAttributes == other.dwMDAttributes && self.dwMDUserType == other.dwMDUserType && self.dwMDDataType == other.dwMDDataType && self.dwMDDataLen == other.dwMDDataLen && self.dwMDDataOffset == other.dwMDDataOffset && self.dwMDDataTag == other.dwMDDataTag
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
impl ::windows::core::TypeKind for METADATA_HANDLE_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for METADATA_HANDLE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwMDPermissions == other.dwMDPermissions && self.dwMDSystemChangeNumber == other.dwMDSystemChangeNumber
    }
}
impl ::core::cmp::Eq for METADATA_HANDLE_INFO {}
impl ::core::default::Default for METADATA_HANDLE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
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
impl ::windows::core::TypeKind for METADATA_RECORD {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for METADATA_RECORD {
    fn eq(&self, other: &Self) -> bool {
        self.dwMDIdentifier == other.dwMDIdentifier && self.dwMDAttributes == other.dwMDAttributes && self.dwMDUserType == other.dwMDUserType && self.dwMDDataType == other.dwMDDataType && self.dwMDDataLen == other.dwMDDataLen && self.pbMDData == other.pbMDData && self.dwMDDataTag == other.dwMDDataTag
    }
}
impl ::core::cmp::Eq for METADATA_RECORD {}
impl ::core::default::Default for METADATA_RECORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
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
impl ::windows::core::TypeKind for POST_PROCESS_PARAMETERS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for POST_PROCESS_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.pszSessionId == other.pszSessionId
            && self.pszSiteName == other.pszSiteName
            && self.pszUserName == other.pszUserName
            && self.pszHostName == other.pszHostName
            && self.pszRemoteIpAddress == other.pszRemoteIpAddress
            && self.dwRemoteIpPort == other.dwRemoteIpPort
            && self.pszLocalIpAddress == other.pszLocalIpAddress
            && self.dwLocalIpPort == other.dwLocalIpPort
            && self.BytesSent == other.BytesSent
            && self.BytesReceived == other.BytesReceived
            && self.pszCommand == other.pszCommand
            && self.pszCommandParameters == other.pszCommandParameters
            && self.pszFullPath == other.pszFullPath
            && self.pszPhysicalPath == other.pszPhysicalPath
            && self.FtpStatus == other.FtpStatus
            && self.FtpSubStatus == other.FtpSubStatus
            && self.hrStatus == other.hrStatus
            && self.SessionStartTime == other.SessionStartTime
            && self.BytesSentPerSession == other.BytesSentPerSession
            && self.BytesReceivedPerSession == other.BytesReceivedPerSession
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
impl ::windows::core::TypeKind for PRE_PROCESS_PARAMETERS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PRE_PROCESS_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.pszSessionId == other.pszSessionId && self.pszSiteName == other.pszSiteName && self.pszUserName == other.pszUserName && self.pszHostName == other.pszHostName && self.pszRemoteIpAddress == other.pszRemoteIpAddress && self.dwRemoteIpPort == other.dwRemoteIpPort && self.pszLocalIpAddress == other.pszLocalIpAddress && self.dwLocalIpPort == other.dwLocalIpPort && self.pszCommand == other.pszCommand && self.pszCommandParameters == other.pszCommandParameters && self.SessionStartTime == other.SessionStartTime && self.BytesSentPerSession == other.BytesSentPerSession && self.BytesReceivedPerSession == other.BytesReceivedPerSession
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
#[repr(C)]
pub struct _IIS_CRYPTO_BLOB(pub u8);
#[doc = "*Required features: `\"Win32_System_Iis\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFN_GETEXTENSIONVERSION = ::core::option::Option<unsafe extern "system" fn(pver: *mut HSE_VERSION_INFO) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub type PFN_HSE_CACHE_INVALIDATION_CALLBACK = ::core::option::Option<unsafe extern "system" fn(pszurl: ::windows::core::PCWSTR) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub type PFN_HSE_GET_PROTOCOL_MANAGER_CUSTOM_INTERFACE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(pszprotocolmanagerdll: ::windows::core::PCWSTR, pszprotocolmanagerdllinitfunction: ::windows::core::PCWSTR, dwcustominterfaceid: u32, ppcustominterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
pub type PFN_HSE_IO_COMPLETION = ::core::option::Option<unsafe extern "system" fn(pecb: *mut EXTENSION_CONTROL_BLOCK, pcontext: *mut ::core::ffi::c_void, cbio: u32, dwerror: u32) -> ()>;
#[doc = "*Required features: `\"Win32_System_Iis\"`*"]
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
#[cfg(feature = "implement")]
::core::include!("impl.rs");
