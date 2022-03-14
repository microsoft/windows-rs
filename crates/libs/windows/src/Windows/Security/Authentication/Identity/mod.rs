#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "Security_Authentication_Identity_Core")]
pub mod Core;
#[cfg(feature = "Security_Authentication_Identity_Provider")]
pub mod Provider;
#[doc = "*Required features: `\"Security_Authentication_Identity\"`*"]
#[repr(transparent)]
pub struct EnterpriseKeyCredentialRegistrationInfo(::windows::core::IUnknown);
impl EnterpriseKeyCredentialRegistrationInfo {
    #[doc = "*Required features: `\"Security_Authentication_Identity\"`*"]
    pub fn TenantId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TenantId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Authentication_Identity\"`*"]
    pub fn TenantName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TenantName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Authentication_Identity\"`*"]
    pub fn Subject(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Subject)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Authentication_Identity\"`*"]
    pub fn KeyId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).KeyId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Authentication_Identity\"`*"]
    pub fn KeyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).KeyName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for EnterpriseKeyCredentialRegistrationInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for EnterpriseKeyCredentialRegistrationInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Identity.EnterpriseKeyCredentialRegistrationInfo;{38321acc-672b-4823-b603-6b3c753daf97})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for EnterpriseKeyCredentialRegistrationInfo {
    type Vtable = IEnterpriseKeyCredentialRegistrationInfo_Vtbl;
    const IID: ::windows::core::GUID = <IEnterpriseKeyCredentialRegistrationInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for EnterpriseKeyCredentialRegistrationInfo {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.EnterpriseKeyCredentialRegistrationInfo";
}
impl ::core::convert::From<EnterpriseKeyCredentialRegistrationInfo> for ::windows::core::IUnknown {
    fn from(value: EnterpriseKeyCredentialRegistrationInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EnterpriseKeyCredentialRegistrationInfo> for ::windows::core::IUnknown {
    fn from(value: &EnterpriseKeyCredentialRegistrationInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for EnterpriseKeyCredentialRegistrationInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a EnterpriseKeyCredentialRegistrationInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EnterpriseKeyCredentialRegistrationInfo> for ::windows::core::IInspectable {
    fn from(value: EnterpriseKeyCredentialRegistrationInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EnterpriseKeyCredentialRegistrationInfo> for ::windows::core::IInspectable {
    fn from(value: &EnterpriseKeyCredentialRegistrationInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for EnterpriseKeyCredentialRegistrationInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a EnterpriseKeyCredentialRegistrationInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for EnterpriseKeyCredentialRegistrationInfo {}
unsafe impl ::core::marker::Sync for EnterpriseKeyCredentialRegistrationInfo {}
#[doc = "*Required features: `\"Security_Authentication_Identity\"`*"]
#[repr(transparent)]
pub struct EnterpriseKeyCredentialRegistrationManager(::windows::core::IUnknown);
impl EnterpriseKeyCredentialRegistrationManager {
    #[doc = "*Required features: `\"Security_Authentication_Identity\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetRegistrationsAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<EnterpriseKeyCredentialRegistrationInfo>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetRegistrationsAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<EnterpriseKeyCredentialRegistrationInfo>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Authentication_Identity\"`*"]
    pub fn Current() -> ::windows::core::Result<EnterpriseKeyCredentialRegistrationManager> {
        Self::IEnterpriseKeyCredentialRegistrationManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Current)(::core::mem::transmute_copy(this), &mut result__).from_abi::<EnterpriseKeyCredentialRegistrationManager>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IEnterpriseKeyCredentialRegistrationManagerStatics<R, F: FnOnce(&IEnterpriseKeyCredentialRegistrationManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<EnterpriseKeyCredentialRegistrationManager, IEnterpriseKeyCredentialRegistrationManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for EnterpriseKeyCredentialRegistrationManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for EnterpriseKeyCredentialRegistrationManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Identity.EnterpriseKeyCredentialRegistrationManager;{83f3be3f-a25f-4cba-bb8e-bdc32d03c297})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for EnterpriseKeyCredentialRegistrationManager {
    type Vtable = IEnterpriseKeyCredentialRegistrationManager_Vtbl;
    const IID: ::windows::core::GUID = <IEnterpriseKeyCredentialRegistrationManager as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for EnterpriseKeyCredentialRegistrationManager {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.EnterpriseKeyCredentialRegistrationManager";
}
impl ::core::convert::From<EnterpriseKeyCredentialRegistrationManager> for ::windows::core::IUnknown {
    fn from(value: EnterpriseKeyCredentialRegistrationManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EnterpriseKeyCredentialRegistrationManager> for ::windows::core::IUnknown {
    fn from(value: &EnterpriseKeyCredentialRegistrationManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for EnterpriseKeyCredentialRegistrationManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a EnterpriseKeyCredentialRegistrationManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EnterpriseKeyCredentialRegistrationManager> for ::windows::core::IInspectable {
    fn from(value: EnterpriseKeyCredentialRegistrationManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EnterpriseKeyCredentialRegistrationManager> for ::windows::core::IInspectable {
    fn from(value: &EnterpriseKeyCredentialRegistrationManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for EnterpriseKeyCredentialRegistrationManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a EnterpriseKeyCredentialRegistrationManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for EnterpriseKeyCredentialRegistrationManager {}
unsafe impl ::core::marker::Sync for EnterpriseKeyCredentialRegistrationManager {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEnterpriseKeyCredentialRegistrationInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEnterpriseKeyCredentialRegistrationInfo {
    type Vtable = IEnterpriseKeyCredentialRegistrationInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x38321acc_672b_4823_b603_6b3c753daf97);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnterpriseKeyCredentialRegistrationInfo_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub TenantId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub TenantName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Subject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub KeyId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub KeyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEnterpriseKeyCredentialRegistrationManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEnterpriseKeyCredentialRegistrationManager {
    type Vtable = IEnterpriseKeyCredentialRegistrationManager_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x83f3be3f_a25f_4cba_bb8e_bdc32d03c297);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnterpriseKeyCredentialRegistrationManager_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetRegistrationsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetRegistrationsAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEnterpriseKeyCredentialRegistrationManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEnterpriseKeyCredentialRegistrationManagerStatics {
    type Vtable = IEnterpriseKeyCredentialRegistrationManagerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x77b85e9e_acf4_4bc0_bac2_40bb46efbb3f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnterpriseKeyCredentialRegistrationManagerStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Current: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
