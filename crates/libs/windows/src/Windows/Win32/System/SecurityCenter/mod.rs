#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
#[inline]
pub unsafe fn WscGetAntiMalwareUri() -> ::windows::core::Result<::windows::core::PWSTR> {
    ::windows::imp::link ! ( "wscapi.dll""system" fn WscGetAntiMalwareUri ( ppszuri : *mut :: windows::core::PWSTR ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
    WscGetAntiMalwareUri(&mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
#[inline]
pub unsafe fn WscGetSecurityProviderHealth(providers: u32, phealth: *mut WSC_SECURITY_PROVIDER_HEALTH) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "wscapi.dll""system" fn WscGetSecurityProviderHealth ( providers : u32 , phealth : *mut WSC_SECURITY_PROVIDER_HEALTH ) -> :: windows::core::HRESULT );
    WscGetSecurityProviderHealth(providers, phealth).ok()
}
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
#[inline]
pub unsafe fn WscQueryAntiMalwareUri() -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "wscapi.dll""system" fn WscQueryAntiMalwareUri ( ) -> :: windows::core::HRESULT );
    WscQueryAntiMalwareUri().ok()
}
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`, `\"Win32_Foundation\"`, `\"Win32_System_Threading\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Threading"))]
#[inline]
pub unsafe fn WscRegisterForChanges(reserved: *mut ::core::ffi::c_void, phcallbackregistration: *mut super::super::Foundation::HANDLE, lpcallbackaddress: super::Threading::LPTHREAD_START_ROUTINE, pcontext: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "wscapi.dll""system" fn WscRegisterForChanges ( reserved : *mut ::core::ffi::c_void , phcallbackregistration : *mut super::super::Foundation:: HANDLE , lpcallbackaddress : super::Threading:: LPTHREAD_START_ROUTINE , pcontext : *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    WscRegisterForChanges(reserved, phcallbackregistration, lpcallbackaddress, pcontext).ok()
}
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
#[inline]
pub unsafe fn WscRegisterForUserNotifications() -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "wscapi.dll""system" fn WscRegisterForUserNotifications ( ) -> :: windows::core::HRESULT );
    WscRegisterForUserNotifications().ok()
}
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WscUnRegisterChanges<P0>(hregistrationhandle: P0) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "wscapi.dll""system" fn WscUnRegisterChanges ( hregistrationhandle : super::super::Foundation:: HANDLE ) -> :: windows::core::HRESULT );
    WscUnRegisterChanges(hregistrationhandle.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IWSCDefaultProduct(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWSCDefaultProduct {
    pub unsafe fn SetDefaultProduct<P0>(&self, etype: SECURITY_PRODUCT_TYPE, pguid: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).SetDefaultProduct)(::windows::core::Interface::as_raw(self), etype, pguid.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IWSCDefaultProduct, ::windows::core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWSCDefaultProduct {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWSCDefaultProduct {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWSCDefaultProduct {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSCDefaultProduct").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IWSCDefaultProduct {
    type Vtable = IWSCDefaultProduct_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IWSCDefaultProduct {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IWSCDefaultProduct {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0476d69c_f21a_11e5_9ce9_5e5517507c66);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWSCDefaultProduct_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub SetDefaultProduct: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, etype: SECURITY_PRODUCT_TYPE, pguid: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IWSCProductList(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWSCProductList {
    pub unsafe fn Initialize(&self, provider: WSC_SECURITY_PROVIDER) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Initialize)(::windows::core::Interface::as_raw(self), provider).ok()
    }
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).Count)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item(&self, index: u32) -> ::windows::core::Result<IWscProduct> {
        let mut result__ = ::windows::core::zeroed::<IWscProduct>();
        (::windows::core::Interface::vtable(self).get_Item)(::windows::core::Interface::as_raw(self), index, &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IWSCProductList, ::windows::core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWSCProductList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWSCProductList {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWSCProductList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSCProductList").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IWSCProductList {
    type Vtable = IWSCProductList_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IWSCProductList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IWSCProductList {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x722a338c_6e8e_4e72_ac27_1417fb0c81c2);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWSCProductList_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, provider: WSC_SECURITY_PROVIDER) -> ::windows::core::HRESULT,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
}
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IWscProduct(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWscProduct {
    pub unsafe fn ProductName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).ProductName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ProductState(&self) -> ::windows::core::Result<WSC_SECURITY_PRODUCT_STATE> {
        let mut result__ = ::windows::core::zeroed::<WSC_SECURITY_PRODUCT_STATE>();
        (::windows::core::Interface::vtable(self).ProductState)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SignatureStatus(&self) -> ::windows::core::Result<WSC_SECURITY_SIGNATURE_STATUS> {
        let mut result__ = ::windows::core::zeroed::<WSC_SECURITY_SIGNATURE_STATUS>();
        (::windows::core::Interface::vtable(self).SignatureStatus)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn RemediationPath(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).RemediationPath)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ProductStateTimestamp(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).ProductStateTimestamp)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ProductGuid(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).ProductGuid)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ProductIsDefault(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).ProductIsDefault)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IWscProduct, ::windows::core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWscProduct {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWscProduct {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWscProduct {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWscProduct").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IWscProduct {
    type Vtable = IWscProduct_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IWscProduct {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IWscProduct {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8c38232e_3a45_4a27_92b0_1a16a975f669);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWscProduct_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub ProductName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub ProductState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut WSC_SECURITY_PRODUCT_STATE) -> ::windows::core::HRESULT,
    pub SignatureStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut WSC_SECURITY_SIGNATURE_STATUS) -> ::windows::core::HRESULT,
    pub RemediationPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub ProductStateTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub ProductGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ProductIsDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ProductIsDefault: usize,
}
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IWscProduct2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWscProduct2 {
    pub unsafe fn ProductName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.ProductName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ProductState(&self) -> ::windows::core::Result<WSC_SECURITY_PRODUCT_STATE> {
        let mut result__ = ::windows::core::zeroed::<WSC_SECURITY_PRODUCT_STATE>();
        (::windows::core::Interface::vtable(self).base__.ProductState)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SignatureStatus(&self) -> ::windows::core::Result<WSC_SECURITY_SIGNATURE_STATUS> {
        let mut result__ = ::windows::core::zeroed::<WSC_SECURITY_SIGNATURE_STATUS>();
        (::windows::core::Interface::vtable(self).base__.SignatureStatus)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn RemediationPath(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.RemediationPath)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ProductStateTimestamp(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.ProductStateTimestamp)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ProductGuid(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.ProductGuid)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ProductIsDefault(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.ProductIsDefault)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn AntivirusScanSubstatus(&self) -> ::windows::core::Result<WSC_SECURITY_PRODUCT_SUBSTATUS> {
        let mut result__ = ::windows::core::zeroed::<WSC_SECURITY_PRODUCT_SUBSTATUS>();
        (::windows::core::Interface::vtable(self).AntivirusScanSubstatus)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn AntivirusSettingsSubstatus(&self) -> ::windows::core::Result<WSC_SECURITY_PRODUCT_SUBSTATUS> {
        let mut result__ = ::windows::core::zeroed::<WSC_SECURITY_PRODUCT_SUBSTATUS>();
        (::windows::core::Interface::vtable(self).AntivirusSettingsSubstatus)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn AntivirusProtectionUpdateSubstatus(&self) -> ::windows::core::Result<WSC_SECURITY_PRODUCT_SUBSTATUS> {
        let mut result__ = ::windows::core::zeroed::<WSC_SECURITY_PRODUCT_SUBSTATUS>();
        (::windows::core::Interface::vtable(self).AntivirusProtectionUpdateSubstatus)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn FirewallDomainProfileSubstatus(&self) -> ::windows::core::Result<WSC_SECURITY_PRODUCT_SUBSTATUS> {
        let mut result__ = ::windows::core::zeroed::<WSC_SECURITY_PRODUCT_SUBSTATUS>();
        (::windows::core::Interface::vtable(self).FirewallDomainProfileSubstatus)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn FirewallPrivateProfileSubstatus(&self) -> ::windows::core::Result<WSC_SECURITY_PRODUCT_SUBSTATUS> {
        let mut result__ = ::windows::core::zeroed::<WSC_SECURITY_PRODUCT_SUBSTATUS>();
        (::windows::core::Interface::vtable(self).FirewallPrivateProfileSubstatus)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn FirewallPublicProfileSubstatus(&self) -> ::windows::core::Result<WSC_SECURITY_PRODUCT_SUBSTATUS> {
        let mut result__ = ::windows::core::zeroed::<WSC_SECURITY_PRODUCT_SUBSTATUS>();
        (::windows::core::Interface::vtable(self).FirewallPublicProfileSubstatus)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IWscProduct2, ::windows::core::IUnknown, super::Com::IDispatch, IWscProduct);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWscProduct2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWscProduct2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWscProduct2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWscProduct2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IWscProduct2 {
    type Vtable = IWscProduct2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IWscProduct2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IWscProduct2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf896ca54_fe09_4403_86d4_23cb488d81d8);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWscProduct2_Vtbl {
    pub base__: IWscProduct_Vtbl,
    pub AntivirusScanSubstatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pestatus: *mut WSC_SECURITY_PRODUCT_SUBSTATUS) -> ::windows::core::HRESULT,
    pub AntivirusSettingsSubstatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pestatus: *mut WSC_SECURITY_PRODUCT_SUBSTATUS) -> ::windows::core::HRESULT,
    pub AntivirusProtectionUpdateSubstatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pestatus: *mut WSC_SECURITY_PRODUCT_SUBSTATUS) -> ::windows::core::HRESULT,
    pub FirewallDomainProfileSubstatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pestatus: *mut WSC_SECURITY_PRODUCT_SUBSTATUS) -> ::windows::core::HRESULT,
    pub FirewallPrivateProfileSubstatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pestatus: *mut WSC_SECURITY_PRODUCT_SUBSTATUS) -> ::windows::core::HRESULT,
    pub FirewallPublicProfileSubstatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pestatus: *mut WSC_SECURITY_PRODUCT_SUBSTATUS) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IWscProduct3(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWscProduct3 {
    pub unsafe fn ProductName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.base__.ProductName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ProductState(&self) -> ::windows::core::Result<WSC_SECURITY_PRODUCT_STATE> {
        let mut result__ = ::windows::core::zeroed::<WSC_SECURITY_PRODUCT_STATE>();
        (::windows::core::Interface::vtable(self).base__.base__.ProductState)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SignatureStatus(&self) -> ::windows::core::Result<WSC_SECURITY_SIGNATURE_STATUS> {
        let mut result__ = ::windows::core::zeroed::<WSC_SECURITY_SIGNATURE_STATUS>();
        (::windows::core::Interface::vtable(self).base__.base__.SignatureStatus)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn RemediationPath(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.base__.RemediationPath)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ProductStateTimestamp(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.base__.ProductStateTimestamp)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ProductGuid(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.base__.ProductGuid)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ProductIsDefault(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.base__.ProductIsDefault)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn AntivirusScanSubstatus(&self) -> ::windows::core::Result<WSC_SECURITY_PRODUCT_SUBSTATUS> {
        let mut result__ = ::windows::core::zeroed::<WSC_SECURITY_PRODUCT_SUBSTATUS>();
        (::windows::core::Interface::vtable(self).base__.AntivirusScanSubstatus)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn AntivirusSettingsSubstatus(&self) -> ::windows::core::Result<WSC_SECURITY_PRODUCT_SUBSTATUS> {
        let mut result__ = ::windows::core::zeroed::<WSC_SECURITY_PRODUCT_SUBSTATUS>();
        (::windows::core::Interface::vtable(self).base__.AntivirusSettingsSubstatus)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn AntivirusProtectionUpdateSubstatus(&self) -> ::windows::core::Result<WSC_SECURITY_PRODUCT_SUBSTATUS> {
        let mut result__ = ::windows::core::zeroed::<WSC_SECURITY_PRODUCT_SUBSTATUS>();
        (::windows::core::Interface::vtable(self).base__.AntivirusProtectionUpdateSubstatus)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn FirewallDomainProfileSubstatus(&self) -> ::windows::core::Result<WSC_SECURITY_PRODUCT_SUBSTATUS> {
        let mut result__ = ::windows::core::zeroed::<WSC_SECURITY_PRODUCT_SUBSTATUS>();
        (::windows::core::Interface::vtable(self).base__.FirewallDomainProfileSubstatus)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn FirewallPrivateProfileSubstatus(&self) -> ::windows::core::Result<WSC_SECURITY_PRODUCT_SUBSTATUS> {
        let mut result__ = ::windows::core::zeroed::<WSC_SECURITY_PRODUCT_SUBSTATUS>();
        (::windows::core::Interface::vtable(self).base__.FirewallPrivateProfileSubstatus)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn FirewallPublicProfileSubstatus(&self) -> ::windows::core::Result<WSC_SECURITY_PRODUCT_SUBSTATUS> {
        let mut result__ = ::windows::core::zeroed::<WSC_SECURITY_PRODUCT_SUBSTATUS>();
        (::windows::core::Interface::vtable(self).base__.FirewallPublicProfileSubstatus)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn AntivirusDaysUntilExpired(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).AntivirusDaysUntilExpired)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IWscProduct3, ::windows::core::IUnknown, super::Com::IDispatch, IWscProduct, IWscProduct2);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWscProduct3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWscProduct3 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWscProduct3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWscProduct3").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IWscProduct3 {
    type Vtable = IWscProduct3_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IWscProduct3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IWscProduct3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x55536524_d1d1_4726_8c7c_04996a1904e7);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWscProduct3_Vtbl {
    pub base__: IWscProduct2_Vtbl,
    pub AntivirusDaysUntilExpired: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwdays: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSCDefaultProduct: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2981a36e_f22d_11e5_9ce9_5e5517507c66);
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSCProductList: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x17072f7b_9abe_4a74_a261_1eb76b55107a);
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SECURITY_PRODUCT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const SECURITY_PRODUCT_TYPE_ANTIVIRUS: SECURITY_PRODUCT_TYPE = SECURITY_PRODUCT_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const SECURITY_PRODUCT_TYPE_FIREWALL: SECURITY_PRODUCT_TYPE = SECURITY_PRODUCT_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const SECURITY_PRODUCT_TYPE_ANTISPYWARE: SECURITY_PRODUCT_TYPE = SECURITY_PRODUCT_TYPE(2i32);
impl ::core::marker::Copy for SECURITY_PRODUCT_TYPE {}
impl ::core::clone::Clone for SECURITY_PRODUCT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SECURITY_PRODUCT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SECURITY_PRODUCT_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SECURITY_PRODUCT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SECURITY_PRODUCT_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WSC_SECURITY_PRODUCT_STATE(pub i32);
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSC_SECURITY_PRODUCT_STATE_ON: WSC_SECURITY_PRODUCT_STATE = WSC_SECURITY_PRODUCT_STATE(0i32);
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSC_SECURITY_PRODUCT_STATE_OFF: WSC_SECURITY_PRODUCT_STATE = WSC_SECURITY_PRODUCT_STATE(1i32);
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSC_SECURITY_PRODUCT_STATE_SNOOZED: WSC_SECURITY_PRODUCT_STATE = WSC_SECURITY_PRODUCT_STATE(2i32);
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSC_SECURITY_PRODUCT_STATE_EXPIRED: WSC_SECURITY_PRODUCT_STATE = WSC_SECURITY_PRODUCT_STATE(3i32);
impl ::core::marker::Copy for WSC_SECURITY_PRODUCT_STATE {}
impl ::core::clone::Clone for WSC_SECURITY_PRODUCT_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WSC_SECURITY_PRODUCT_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WSC_SECURITY_PRODUCT_STATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WSC_SECURITY_PRODUCT_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WSC_SECURITY_PRODUCT_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WSC_SECURITY_PRODUCT_SUBSTATUS(pub i32);
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSC_SECURITY_PRODUCT_SUBSTATUS_NOT_SET: WSC_SECURITY_PRODUCT_SUBSTATUS = WSC_SECURITY_PRODUCT_SUBSTATUS(0i32);
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSC_SECURITY_PRODUCT_SUBSTATUS_NO_ACTION: WSC_SECURITY_PRODUCT_SUBSTATUS = WSC_SECURITY_PRODUCT_SUBSTATUS(1i32);
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSC_SECURITY_PRODUCT_SUBSTATUS_ACTION_RECOMMENDED: WSC_SECURITY_PRODUCT_SUBSTATUS = WSC_SECURITY_PRODUCT_SUBSTATUS(2i32);
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSC_SECURITY_PRODUCT_SUBSTATUS_ACTION_NEEDED: WSC_SECURITY_PRODUCT_SUBSTATUS = WSC_SECURITY_PRODUCT_SUBSTATUS(3i32);
impl ::core::marker::Copy for WSC_SECURITY_PRODUCT_SUBSTATUS {}
impl ::core::clone::Clone for WSC_SECURITY_PRODUCT_SUBSTATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WSC_SECURITY_PRODUCT_SUBSTATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WSC_SECURITY_PRODUCT_SUBSTATUS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WSC_SECURITY_PRODUCT_SUBSTATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WSC_SECURITY_PRODUCT_SUBSTATUS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WSC_SECURITY_PROVIDER(pub i32);
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSC_SECURITY_PROVIDER_FIREWALL: WSC_SECURITY_PROVIDER = WSC_SECURITY_PROVIDER(1i32);
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSC_SECURITY_PROVIDER_AUTOUPDATE_SETTINGS: WSC_SECURITY_PROVIDER = WSC_SECURITY_PROVIDER(2i32);
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSC_SECURITY_PROVIDER_ANTIVIRUS: WSC_SECURITY_PROVIDER = WSC_SECURITY_PROVIDER(4i32);
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSC_SECURITY_PROVIDER_ANTISPYWARE: WSC_SECURITY_PROVIDER = WSC_SECURITY_PROVIDER(8i32);
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSC_SECURITY_PROVIDER_INTERNET_SETTINGS: WSC_SECURITY_PROVIDER = WSC_SECURITY_PROVIDER(16i32);
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSC_SECURITY_PROVIDER_USER_ACCOUNT_CONTROL: WSC_SECURITY_PROVIDER = WSC_SECURITY_PROVIDER(32i32);
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSC_SECURITY_PROVIDER_SERVICE: WSC_SECURITY_PROVIDER = WSC_SECURITY_PROVIDER(64i32);
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSC_SECURITY_PROVIDER_NONE: WSC_SECURITY_PROVIDER = WSC_SECURITY_PROVIDER(0i32);
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSC_SECURITY_PROVIDER_ALL: WSC_SECURITY_PROVIDER = WSC_SECURITY_PROVIDER(127i32);
impl ::core::marker::Copy for WSC_SECURITY_PROVIDER {}
impl ::core::clone::Clone for WSC_SECURITY_PROVIDER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WSC_SECURITY_PROVIDER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WSC_SECURITY_PROVIDER {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WSC_SECURITY_PROVIDER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WSC_SECURITY_PROVIDER").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WSC_SECURITY_PROVIDER_HEALTH(pub i32);
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSC_SECURITY_PROVIDER_HEALTH_GOOD: WSC_SECURITY_PROVIDER_HEALTH = WSC_SECURITY_PROVIDER_HEALTH(0i32);
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSC_SECURITY_PROVIDER_HEALTH_NOTMONITORED: WSC_SECURITY_PROVIDER_HEALTH = WSC_SECURITY_PROVIDER_HEALTH(1i32);
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSC_SECURITY_PROVIDER_HEALTH_POOR: WSC_SECURITY_PROVIDER_HEALTH = WSC_SECURITY_PROVIDER_HEALTH(2i32);
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSC_SECURITY_PROVIDER_HEALTH_SNOOZE: WSC_SECURITY_PROVIDER_HEALTH = WSC_SECURITY_PROVIDER_HEALTH(3i32);
impl ::core::marker::Copy for WSC_SECURITY_PROVIDER_HEALTH {}
impl ::core::clone::Clone for WSC_SECURITY_PROVIDER_HEALTH {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WSC_SECURITY_PROVIDER_HEALTH {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WSC_SECURITY_PROVIDER_HEALTH {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WSC_SECURITY_PROVIDER_HEALTH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WSC_SECURITY_PROVIDER_HEALTH").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WSC_SECURITY_SIGNATURE_STATUS(pub i32);
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSC_SECURITY_PRODUCT_OUT_OF_DATE: WSC_SECURITY_SIGNATURE_STATUS = WSC_SECURITY_SIGNATURE_STATUS(0i32);
#[doc = "*Required features: `\"Win32_System_SecurityCenter\"`*"]
pub const WSC_SECURITY_PRODUCT_UP_TO_DATE: WSC_SECURITY_SIGNATURE_STATUS = WSC_SECURITY_SIGNATURE_STATUS(1i32);
impl ::core::marker::Copy for WSC_SECURITY_SIGNATURE_STATUS {}
impl ::core::clone::Clone for WSC_SECURITY_SIGNATURE_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WSC_SECURITY_SIGNATURE_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WSC_SECURITY_SIGNATURE_STATUS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WSC_SECURITY_SIGNATURE_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WSC_SECURITY_SIGNATURE_STATUS").field(&self.0).finish()
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
