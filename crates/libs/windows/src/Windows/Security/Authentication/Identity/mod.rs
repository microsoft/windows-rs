#[cfg(feature = "Security_Authentication_Identity_Core")]
pub mod Core;
#[doc(hidden)]
#[repr(transparent)]
pub struct IEnterpriseKeyCredentialRegistrationInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEnterpriseKeyCredentialRegistrationInfo {
    type Vtable = IEnterpriseKeyCredentialRegistrationInfo_Vtbl;
}
impl ::core::clone::Clone for IEnterpriseKeyCredentialRegistrationInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEnterpriseKeyCredentialRegistrationInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x38321acc_672b_4823_b603_6b3c753daf97);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnterpriseKeyCredentialRegistrationInfo_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub TenantId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub TenantName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Subject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub KeyId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub KeyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEnterpriseKeyCredentialRegistrationManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEnterpriseKeyCredentialRegistrationManager {
    type Vtable = IEnterpriseKeyCredentialRegistrationManager_Vtbl;
}
impl ::core::clone::Clone for IEnterpriseKeyCredentialRegistrationManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEnterpriseKeyCredentialRegistrationManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x83f3be3f_a25f_4cba_bb8e_bdc32d03c297);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnterpriseKeyCredentialRegistrationManager_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetRegistrationsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetRegistrationsAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEnterpriseKeyCredentialRegistrationManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEnterpriseKeyCredentialRegistrationManagerStatics {
    type Vtable = IEnterpriseKeyCredentialRegistrationManagerStatics_Vtbl;
}
impl ::core::clone::Clone for IEnterpriseKeyCredentialRegistrationManagerStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEnterpriseKeyCredentialRegistrationManagerStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x77b85e9e_acf4_4bc0_bac2_40bb46efbb3f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnterpriseKeyCredentialRegistrationManagerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Current: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Security_Authentication_Identity\"`*"]
#[repr(transparent)]
pub struct EnterpriseKeyCredentialRegistrationInfo(::windows::core::IUnknown);
impl EnterpriseKeyCredentialRegistrationInfo {
    pub fn TenantId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).TenantId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TenantName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).TenantName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Subject(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Subject)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn KeyId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).KeyId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn KeyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).KeyName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for EnterpriseKeyCredentialRegistrationInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EnterpriseKeyCredentialRegistrationInfo {}
impl ::core::fmt::Debug for EnterpriseKeyCredentialRegistrationInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EnterpriseKeyCredentialRegistrationInfo").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for EnterpriseKeyCredentialRegistrationInfo {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Identity.EnterpriseKeyCredentialRegistrationInfo;{38321acc-672b-4823-b603-6b3c753daf97})");
}
impl ::core::clone::Clone for EnterpriseKeyCredentialRegistrationInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for EnterpriseKeyCredentialRegistrationInfo {
    type Vtable = IEnterpriseKeyCredentialRegistrationInfo_Vtbl;
}
unsafe impl ::windows::core::ComInterface for EnterpriseKeyCredentialRegistrationInfo {
    const IID: ::windows::core::GUID = <IEnterpriseKeyCredentialRegistrationInfo as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for EnterpriseKeyCredentialRegistrationInfo {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.EnterpriseKeyCredentialRegistrationInfo";
}
::windows::imp::interface_hierarchy!(EnterpriseKeyCredentialRegistrationInfo, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for EnterpriseKeyCredentialRegistrationInfo {}
unsafe impl ::core::marker::Sync for EnterpriseKeyCredentialRegistrationInfo {}
#[doc = "*Required features: `\"Security_Authentication_Identity\"`*"]
#[repr(transparent)]
pub struct EnterpriseKeyCredentialRegistrationManager(::windows::core::IUnknown);
impl EnterpriseKeyCredentialRegistrationManager {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetRegistrationsAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<EnterpriseKeyCredentialRegistrationInfo>>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<EnterpriseKeyCredentialRegistrationInfo>>>();
            (::windows::core::Interface::vtable(this).GetRegistrationsAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Current() -> ::windows::core::Result<EnterpriseKeyCredentialRegistrationManager> {
        Self::IEnterpriseKeyCredentialRegistrationManagerStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<EnterpriseKeyCredentialRegistrationManager>();
            (::windows::core::Interface::vtable(this).Current)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IEnterpriseKeyCredentialRegistrationManagerStatics<R, F: FnOnce(&IEnterpriseKeyCredentialRegistrationManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<EnterpriseKeyCredentialRegistrationManager, IEnterpriseKeyCredentialRegistrationManagerStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for EnterpriseKeyCredentialRegistrationManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EnterpriseKeyCredentialRegistrationManager {}
impl ::core::fmt::Debug for EnterpriseKeyCredentialRegistrationManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EnterpriseKeyCredentialRegistrationManager").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for EnterpriseKeyCredentialRegistrationManager {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Identity.EnterpriseKeyCredentialRegistrationManager;{83f3be3f-a25f-4cba-bb8e-bdc32d03c297})");
}
impl ::core::clone::Clone for EnterpriseKeyCredentialRegistrationManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for EnterpriseKeyCredentialRegistrationManager {
    type Vtable = IEnterpriseKeyCredentialRegistrationManager_Vtbl;
}
unsafe impl ::windows::core::ComInterface for EnterpriseKeyCredentialRegistrationManager {
    const IID: ::windows::core::GUID = <IEnterpriseKeyCredentialRegistrationManager as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for EnterpriseKeyCredentialRegistrationManager {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.EnterpriseKeyCredentialRegistrationManager";
}
::windows::imp::interface_hierarchy!(EnterpriseKeyCredentialRegistrationManager, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for EnterpriseKeyCredentialRegistrationManager {}
unsafe impl ::core::marker::Sync for EnterpriseKeyCredentialRegistrationManager {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
