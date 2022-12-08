#[doc = "*Required features: `\"Win32_Security_EnterpriseData\"`*"]
#[inline]
pub unsafe fn ProtectFileToEnterpriseIdentity<P0, P1>(fileorfolderpath: P0, identity: P1) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "efswrt.dll""system" fn ProtectFileToEnterpriseIdentity ( fileorfolderpath : :: windows::core::PCWSTR , identity : :: windows::core::PCWSTR ) -> :: windows::core::HRESULT );
    ProtectFileToEnterpriseIdentity(fileorfolderpath.into().abi(), identity.into().abi()).ok()
}
#[doc = "*Required features: `\"Win32_Security_EnterpriseData\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SrpCloseThreadNetworkContext(threadnetworkcontext: *mut HTHREAD_NETWORK_CONTEXT) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "srpapi.dll""system" fn SrpCloseThreadNetworkContext ( threadnetworkcontext : *mut HTHREAD_NETWORK_CONTEXT ) -> :: windows::core::HRESULT );
    SrpCloseThreadNetworkContext(threadnetworkcontext).ok()
}
#[doc = "*Required features: `\"Win32_Security_EnterpriseData\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SrpCreateThreadNetworkContext<P0>(enterpriseid: P0) -> ::windows::core::Result<HTHREAD_NETWORK_CONTEXT>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "srpapi.dll""system" fn SrpCreateThreadNetworkContext ( enterpriseid : :: windows::core::PCWSTR , threadnetworkcontext : *mut HTHREAD_NETWORK_CONTEXT ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    SrpCreateThreadNetworkContext(enterpriseid.into().abi(), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Security_EnterpriseData\"`*"]
#[inline]
pub unsafe fn SrpDisablePermissiveModeFileEncryption() -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "srpapi.dll""system" fn SrpDisablePermissiveModeFileEncryption ( ) -> :: windows::core::HRESULT );
    SrpDisablePermissiveModeFileEncryption().ok()
}
#[doc = "*Required features: `\"Win32_Security_EnterpriseData\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_Packaging_Appx\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Appx"))]
#[inline]
pub unsafe fn SrpDoesPolicyAllowAppExecution(packageid: *const super::super::Storage::Packaging::Appx::PACKAGE_ID) -> ::windows::core::Result<super::super::Foundation::BOOL> {
    ::windows::core::link ! ( "srpapi.dll""system" fn SrpDoesPolicyAllowAppExecution ( packageid : *const super::super::Storage::Packaging::Appx:: PACKAGE_ID , isallowed : *mut super::super::Foundation:: BOOL ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    SrpDoesPolicyAllowAppExecution(packageid, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Security_EnterpriseData\"`*"]
#[inline]
pub unsafe fn SrpEnablePermissiveModeFileEncryption<P0>(enterpriseid: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "srpapi.dll""system" fn SrpEnablePermissiveModeFileEncryption ( enterpriseid : :: windows::core::PCWSTR ) -> :: windows::core::HRESULT );
    SrpEnablePermissiveModeFileEncryption(enterpriseid.into().abi()).ok()
}
#[doc = "*Required features: `\"Win32_Security_EnterpriseData\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SrpGetEnterpriseIds<P0>(tokenhandle: P0, numberofbytes: ::core::option::Option<*mut u32>, enterpriseids: ::core::option::Option<*mut ::windows::core::PWSTR>, enterpriseidcount: *mut u32) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "srpapi.dll""system" fn SrpGetEnterpriseIds ( tokenhandle : super::super::Foundation:: HANDLE , numberofbytes : *mut u32 , enterpriseids : *mut :: windows::core::PWSTR , enterpriseidcount : *mut u32 ) -> :: windows::core::HRESULT );
    SrpGetEnterpriseIds(tokenhandle.into(), ::core::mem::transmute(numberofbytes.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(enterpriseids.unwrap_or(::std::ptr::null_mut())), enterpriseidcount).ok()
}
#[doc = "*Required features: `\"Win32_Security_EnterpriseData\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SrpGetEnterprisePolicy<P0>(tokenhandle: P0) -> ::windows::core::Result<ENTERPRISE_DATA_POLICIES>
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "srpapi.dll""system" fn SrpGetEnterprisePolicy ( tokenhandle : super::super::Foundation:: HANDLE , policyflags : *mut ENTERPRISE_DATA_POLICIES ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    SrpGetEnterprisePolicy(tokenhandle.into(), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Security_EnterpriseData\"`*"]
#[inline]
pub unsafe fn SrpHostingInitialize(version: SRPHOSTING_VERSION, r#type: SRPHOSTING_TYPE, pvdata: *const ::core::ffi::c_void, cbdata: u32) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "srpapi.dll""system" fn SrpHostingInitialize ( version : SRPHOSTING_VERSION , r#type : SRPHOSTING_TYPE , pvdata : *const ::core::ffi::c_void , cbdata : u32 ) -> :: windows::core::HRESULT );
    SrpHostingInitialize(version, r#type, pvdata, cbdata).ok()
}
#[doc = "*Required features: `\"Win32_Security_EnterpriseData\"`*"]
#[inline]
pub unsafe fn SrpHostingTerminate(r#type: SRPHOSTING_TYPE) {
    ::windows::core::link ! ( "srpapi.dll""system" fn SrpHostingTerminate ( r#type : SRPHOSTING_TYPE ) -> ( ) );
    SrpHostingTerminate(r#type)
}
#[doc = "*Required features: `\"Win32_Security_EnterpriseData\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SrpIsTokenService<P0>(tokenhandle: P0, istokenservice: *mut u8) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "srpapi.dll""system" fn SrpIsTokenService ( tokenhandle : super::super::Foundation:: HANDLE , istokenservice : *mut u8 ) -> super::super::Foundation:: NTSTATUS );
    SrpIsTokenService(tokenhandle.into(), istokenservice).ok()
}
#[doc = "*Required features: `\"Win32_Security_EnterpriseData\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SrpSetTokenEnterpriseId<P0, P1>(tokenhandle: P0, enterpriseid: P1) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "srpapi.dll""system" fn SrpSetTokenEnterpriseId ( tokenhandle : super::super::Foundation:: HANDLE , enterpriseid : :: windows::core::PCWSTR ) -> :: windows::core::HRESULT );
    SrpSetTokenEnterpriseId(tokenhandle.into(), enterpriseid.into().abi()).ok()
}
#[doc = "*Required features: `\"Win32_Security_EnterpriseData\"`*"]
#[inline]
pub unsafe fn UnprotectFile<P0>(fileorfolderpath: P0, options: ::core::option::Option<*const FILE_UNPROTECT_OPTIONS>) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "efswrt.dll""system" fn UnprotectFile ( fileorfolderpath : :: windows::core::PCWSTR , options : *const FILE_UNPROTECT_OPTIONS ) -> :: windows::core::HRESULT );
    UnprotectFile(fileorfolderpath.into().abi(), ::core::mem::transmute(options.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_Security_EnterpriseData\"`*"]
#[repr(transparent)]
pub struct IProtectionPolicyManagerInterop(::windows::core::IUnknown);
impl IProtectionPolicyManagerInterop {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RequestAccessForWindowAsync<P0, T>(&self, appwindow: P0, sourceidentity: &::windows::core::HSTRING, targetidentity: &::windows::core::HSTRING) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).RequestAccessForWindowAsync)(::windows::core::Vtable::as_raw(self), appwindow.into(), ::core::mem::transmute_copy(sourceidentity), ::core::mem::transmute_copy(targetidentity), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetForWindow<P0, T>(&self, appwindow: P0) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetForWindow)(::windows::core::Vtable::as_raw(self), appwindow.into(), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IProtectionPolicyManagerInterop, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for IProtectionPolicyManagerInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IProtectionPolicyManagerInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProtectionPolicyManagerInterop {}
impl ::core::fmt::Debug for IProtectionPolicyManagerInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProtectionPolicyManagerInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IProtectionPolicyManagerInterop {
    type Vtable = IProtectionPolicyManagerInterop_Vtbl;
}
unsafe impl ::windows::core::Interface for IProtectionPolicyManagerInterop {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4652651d_c1fe_4ba1_9f0a_c0f56596f721);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectionPolicyManagerInterop_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub RequestAccessForWindowAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, sourceidentity: *mut ::core::ffi::c_void, targetidentity: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RequestAccessForWindowAsync: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetForWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, result: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetForWindow: usize,
}
#[doc = "*Required features: `\"Win32_Security_EnterpriseData\"`*"]
#[repr(transparent)]
pub struct IProtectionPolicyManagerInterop2(::windows::core::IUnknown);
impl IProtectionPolicyManagerInterop2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RequestAccessForAppWithWindowAsync<P0, T>(&self, appwindow: P0, sourceidentity: &::windows::core::HSTRING, apppackagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).RequestAccessForAppWithWindowAsync)(::windows::core::Vtable::as_raw(self), appwindow.into(), ::core::mem::transmute_copy(sourceidentity), ::core::mem::transmute_copy(apppackagefamilyname), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RequestAccessWithAuditingInfoForWindowAsync<P0, P1, T>(&self, appwindow: P0, sourceidentity: &::windows::core::HSTRING, targetidentity: &::windows::core::HSTRING, auditinfounk: P1) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).RequestAccessWithAuditingInfoForWindowAsync)(::windows::core::Vtable::as_raw(self), appwindow.into(), ::core::mem::transmute_copy(sourceidentity), ::core::mem::transmute_copy(targetidentity), auditinfounk.into().abi(), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RequestAccessWithMessageForWindowAsync<P0, P1, T>(&self, appwindow: P0, sourceidentity: &::windows::core::HSTRING, targetidentity: &::windows::core::HSTRING, auditinfounk: P1, messagefromapp: &::windows::core::HSTRING) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).RequestAccessWithMessageForWindowAsync)(::windows::core::Vtable::as_raw(self), appwindow.into(), ::core::mem::transmute_copy(sourceidentity), ::core::mem::transmute_copy(targetidentity), auditinfounk.into().abi(), ::core::mem::transmute_copy(messagefromapp), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RequestAccessForAppWithAuditingInfoForWindowAsync<P0, P1, T>(&self, appwindow: P0, sourceidentity: &::windows::core::HSTRING, apppackagefamilyname: &::windows::core::HSTRING, auditinfounk: P1) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).RequestAccessForAppWithAuditingInfoForWindowAsync)(::windows::core::Vtable::as_raw(self), appwindow.into(), ::core::mem::transmute_copy(sourceidentity), ::core::mem::transmute_copy(apppackagefamilyname), auditinfounk.into().abi(), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RequestAccessForAppWithMessageForWindowAsync<P0, P1, T>(&self, appwindow: P0, sourceidentity: &::windows::core::HSTRING, apppackagefamilyname: &::windows::core::HSTRING, auditinfounk: P1, messagefromapp: &::windows::core::HSTRING) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).RequestAccessForAppWithMessageForWindowAsync)(::windows::core::Vtable::as_raw(self), appwindow.into(), ::core::mem::transmute_copy(sourceidentity), ::core::mem::transmute_copy(apppackagefamilyname), auditinfounk.into().abi(), ::core::mem::transmute_copy(messagefromapp), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IProtectionPolicyManagerInterop2, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for IProtectionPolicyManagerInterop2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IProtectionPolicyManagerInterop2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProtectionPolicyManagerInterop2 {}
impl ::core::fmt::Debug for IProtectionPolicyManagerInterop2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProtectionPolicyManagerInterop2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IProtectionPolicyManagerInterop2 {
    type Vtable = IProtectionPolicyManagerInterop2_Vtbl;
}
unsafe impl ::windows::core::Interface for IProtectionPolicyManagerInterop2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x157cfbe4_a78d_4156_b384_61fdac41e686);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectionPolicyManagerInterop2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub RequestAccessForAppWithWindowAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, sourceidentity: *mut ::core::ffi::c_void, apppackagefamilyname: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RequestAccessForAppWithWindowAsync: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RequestAccessWithAuditingInfoForWindowAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, sourceidentity: *mut ::core::ffi::c_void, targetidentity: *mut ::core::ffi::c_void, auditinfounk: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RequestAccessWithAuditingInfoForWindowAsync: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RequestAccessWithMessageForWindowAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, sourceidentity: *mut ::core::ffi::c_void, targetidentity: *mut ::core::ffi::c_void, auditinfounk: *mut ::core::ffi::c_void, messagefromapp: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RequestAccessWithMessageForWindowAsync: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RequestAccessForAppWithAuditingInfoForWindowAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, sourceidentity: *mut ::core::ffi::c_void, apppackagefamilyname: *mut ::core::ffi::c_void, auditinfounk: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RequestAccessForAppWithAuditingInfoForWindowAsync: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RequestAccessForAppWithMessageForWindowAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, sourceidentity: *mut ::core::ffi::c_void, apppackagefamilyname: *mut ::core::ffi::c_void, auditinfounk: *mut ::core::ffi::c_void, messagefromapp: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RequestAccessForAppWithMessageForWindowAsync: usize,
}
#[doc = "*Required features: `\"Win32_Security_EnterpriseData\"`*"]
#[repr(transparent)]
pub struct IProtectionPolicyManagerInterop3(::windows::core::IUnknown);
impl IProtectionPolicyManagerInterop3 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RequestAccessWithBehaviorForWindowAsync<P0, P1, T>(&self, appwindow: P0, sourceidentity: &::windows::core::HSTRING, targetidentity: &::windows::core::HSTRING, auditinfounk: P1, messagefromapp: &::windows::core::HSTRING, behavior: u32) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).RequestAccessWithBehaviorForWindowAsync)(::windows::core::Vtable::as_raw(self), appwindow.into(), ::core::mem::transmute_copy(sourceidentity), ::core::mem::transmute_copy(targetidentity), auditinfounk.into().abi(), ::core::mem::transmute_copy(messagefromapp), behavior, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RequestAccessForAppWithBehaviorForWindowAsync<P0, P1, T>(&self, appwindow: P0, sourceidentity: &::windows::core::HSTRING, apppackagefamilyname: &::windows::core::HSTRING, auditinfounk: P1, messagefromapp: &::windows::core::HSTRING, behavior: u32) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).RequestAccessForAppWithBehaviorForWindowAsync)(::windows::core::Vtable::as_raw(self), appwindow.into(), ::core::mem::transmute_copy(sourceidentity), ::core::mem::transmute_copy(apppackagefamilyname), auditinfounk.into().abi(), ::core::mem::transmute_copy(messagefromapp), behavior, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RequestAccessToFilesForAppForWindowAsync<P0, P1, P2, T>(&self, appwindow: P0, sourceitemlistunk: P1, apppackagefamilyname: &::windows::core::HSTRING, auditinfounk: P2) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        P2: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).RequestAccessToFilesForAppForWindowAsync)(::windows::core::Vtable::as_raw(self), appwindow.into(), sourceitemlistunk.into().abi(), ::core::mem::transmute_copy(apppackagefamilyname), auditinfounk.into().abi(), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RequestAccessToFilesForAppWithMessageAndBehaviorForWindowAsync<P0, P1, P2, T>(&self, appwindow: P0, sourceitemlistunk: P1, apppackagefamilyname: &::windows::core::HSTRING, auditinfounk: P2, messagefromapp: &::windows::core::HSTRING, behavior: u32) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        P2: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).RequestAccessToFilesForAppWithMessageAndBehaviorForWindowAsync)(::windows::core::Vtable::as_raw(self), appwindow.into(), sourceitemlistunk.into().abi(), ::core::mem::transmute_copy(apppackagefamilyname), auditinfounk.into().abi(), ::core::mem::transmute_copy(messagefromapp), behavior, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RequestAccessToFilesForProcessForWindowAsync<P0, P1, P2, T>(&self, appwindow: P0, sourceitemlistunk: P1, processid: u32, auditinfounk: P2) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        P2: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).RequestAccessToFilesForProcessForWindowAsync)(::windows::core::Vtable::as_raw(self), appwindow.into(), sourceitemlistunk.into().abi(), processid, auditinfounk.into().abi(), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RequestAccessToFilesForProcessWithMessageAndBehaviorForWindowAsync<P0, P1, P2, T>(&self, appwindow: P0, sourceitemlistunk: P1, processid: u32, auditinfounk: P2, messagefromapp: &::windows::core::HSTRING, behavior: u32) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        P2: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).RequestAccessToFilesForProcessWithMessageAndBehaviorForWindowAsync)(::windows::core::Vtable::as_raw(self), appwindow.into(), sourceitemlistunk.into().abi(), processid, auditinfounk.into().abi(), ::core::mem::transmute_copy(messagefromapp), behavior, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IProtectionPolicyManagerInterop3, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for IProtectionPolicyManagerInterop3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IProtectionPolicyManagerInterop3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProtectionPolicyManagerInterop3 {}
impl ::core::fmt::Debug for IProtectionPolicyManagerInterop3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProtectionPolicyManagerInterop3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IProtectionPolicyManagerInterop3 {
    type Vtable = IProtectionPolicyManagerInterop3_Vtbl;
}
unsafe impl ::windows::core::Interface for IProtectionPolicyManagerInterop3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc1c03933_b398_4d93_b0fd_2972adf802c2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectionPolicyManagerInterop3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub RequestAccessWithBehaviorForWindowAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, sourceidentity: *mut ::core::ffi::c_void, targetidentity: *mut ::core::ffi::c_void, auditinfounk: *mut ::core::ffi::c_void, messagefromapp: *mut ::core::ffi::c_void, behavior: u32, riid: *const ::windows::core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RequestAccessWithBehaviorForWindowAsync: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RequestAccessForAppWithBehaviorForWindowAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, sourceidentity: *mut ::core::ffi::c_void, apppackagefamilyname: *mut ::core::ffi::c_void, auditinfounk: *mut ::core::ffi::c_void, messagefromapp: *mut ::core::ffi::c_void, behavior: u32, riid: *const ::windows::core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RequestAccessForAppWithBehaviorForWindowAsync: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RequestAccessToFilesForAppForWindowAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, sourceitemlistunk: *mut ::core::ffi::c_void, apppackagefamilyname: *mut ::core::ffi::c_void, auditinfounk: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RequestAccessToFilesForAppForWindowAsync: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RequestAccessToFilesForAppWithMessageAndBehaviorForWindowAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, sourceitemlistunk: *mut ::core::ffi::c_void, apppackagefamilyname: *mut ::core::ffi::c_void, auditinfounk: *mut ::core::ffi::c_void, messagefromapp: *mut ::core::ffi::c_void, behavior: u32, riid: *const ::windows::core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RequestAccessToFilesForAppWithMessageAndBehaviorForWindowAsync: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RequestAccessToFilesForProcessForWindowAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, sourceitemlistunk: *mut ::core::ffi::c_void, processid: u32, auditinfounk: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RequestAccessToFilesForProcessForWindowAsync: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RequestAccessToFilesForProcessWithMessageAndBehaviorForWindowAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, sourceitemlistunk: *mut ::core::ffi::c_void, processid: u32, auditinfounk: *mut ::core::ffi::c_void, messagefromapp: *mut ::core::ffi::c_void, behavior: u32, riid: *const ::windows::core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RequestAccessToFilesForProcessWithMessageAndBehaviorForWindowAsync: usize,
}
#[doc = "*Required features: `\"Win32_Security_EnterpriseData\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ENTERPRISE_DATA_POLICIES(pub u32);
#[doc = "*Required features: `\"Win32_Security_EnterpriseData\"`*"]
pub const ENTERPRISE_POLICY_NONE: ENTERPRISE_DATA_POLICIES = ENTERPRISE_DATA_POLICIES(0u32);
#[doc = "*Required features: `\"Win32_Security_EnterpriseData\"`*"]
pub const ENTERPRISE_POLICY_ALLOWED: ENTERPRISE_DATA_POLICIES = ENTERPRISE_DATA_POLICIES(1u32);
#[doc = "*Required features: `\"Win32_Security_EnterpriseData\"`*"]
pub const ENTERPRISE_POLICY_ENLIGHTENED: ENTERPRISE_DATA_POLICIES = ENTERPRISE_DATA_POLICIES(2u32);
#[doc = "*Required features: `\"Win32_Security_EnterpriseData\"`*"]
pub const ENTERPRISE_POLICY_EXEMPT: ENTERPRISE_DATA_POLICIES = ENTERPRISE_DATA_POLICIES(4u32);
impl ::core::marker::Copy for ENTERPRISE_DATA_POLICIES {}
impl ::core::clone::Clone for ENTERPRISE_DATA_POLICIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ENTERPRISE_DATA_POLICIES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ENTERPRISE_DATA_POLICIES {
    type Abi = Self;
}
impl ::core::fmt::Debug for ENTERPRISE_DATA_POLICIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ENTERPRISE_DATA_POLICIES").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for ENTERPRISE_DATA_POLICIES {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for ENTERPRISE_DATA_POLICIES {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for ENTERPRISE_DATA_POLICIES {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for ENTERPRISE_DATA_POLICIES {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for ENTERPRISE_DATA_POLICIES {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Security_EnterpriseData\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SRPHOSTING_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Security_EnterpriseData\"`*"]
pub const SRPHOSTING_TYPE_NONE: SRPHOSTING_TYPE = SRPHOSTING_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Security_EnterpriseData\"`*"]
pub const SRPHOSTING_TYPE_WINHTTP: SRPHOSTING_TYPE = SRPHOSTING_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Security_EnterpriseData\"`*"]
pub const SRPHOSTING_TYPE_WININET: SRPHOSTING_TYPE = SRPHOSTING_TYPE(2i32);
impl ::core::marker::Copy for SRPHOSTING_TYPE {}
impl ::core::clone::Clone for SRPHOSTING_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SRPHOSTING_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SRPHOSTING_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for SRPHOSTING_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SRPHOSTING_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Security_EnterpriseData\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SRPHOSTING_VERSION(pub i32);
#[doc = "*Required features: `\"Win32_Security_EnterpriseData\"`*"]
pub const SRPHOSTING_VERSION1: SRPHOSTING_VERSION = SRPHOSTING_VERSION(1i32);
impl ::core::marker::Copy for SRPHOSTING_VERSION {}
impl ::core::clone::Clone for SRPHOSTING_VERSION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SRPHOSTING_VERSION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SRPHOSTING_VERSION {
    type Abi = Self;
}
impl ::core::fmt::Debug for SRPHOSTING_VERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SRPHOSTING_VERSION").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_EnterpriseData\"`*"]
pub struct FILE_UNPROTECT_OPTIONS {
    pub audit: u8,
}
impl ::core::marker::Copy for FILE_UNPROTECT_OPTIONS {}
impl ::core::clone::Clone for FILE_UNPROTECT_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FILE_UNPROTECT_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_UNPROTECT_OPTIONS").field("audit", &self.audit).finish()
    }
}
unsafe impl ::windows::core::Abi for FILE_UNPROTECT_OPTIONS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FILE_UNPROTECT_OPTIONS {
    fn eq(&self, other: &Self) -> bool {
        self.audit == other.audit
    }
}
impl ::core::cmp::Eq for FILE_UNPROTECT_OPTIONS {}
impl ::core::default::Default for FILE_UNPROTECT_OPTIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_EnterpriseData\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct HTHREAD_NETWORK_CONTEXT {
    pub ThreadId: u32,
    pub ThreadContext: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HTHREAD_NETWORK_CONTEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HTHREAD_NETWORK_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HTHREAD_NETWORK_CONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTHREAD_NETWORK_CONTEXT").field("ThreadId", &self.ThreadId).field("ThreadContext", &self.ThreadContext).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for HTHREAD_NETWORK_CONTEXT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HTHREAD_NETWORK_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.ThreadId == other.ThreadId && self.ThreadContext == other.ThreadContext
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HTHREAD_NETWORK_CONTEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HTHREAD_NETWORK_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
