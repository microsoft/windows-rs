#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "Security_Authentication_Identity_Core")]
pub mod Core;
#[cfg(feature = "Security_Authentication_Identity_Provider")]
pub mod Provider;
#[doc = "*Required features: `Security_Authentication_Identity`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct EnterpriseKeyCredentialRegistrationInfo(pub ::windows::core::IInspectable);
impl EnterpriseKeyCredentialRegistrationInfo {
    #[doc = "*Required features: `Security_Authentication_Identity`*"]
    pub fn TenantId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Security_Authentication_Identity`*"]
    pub fn TenantName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Security_Authentication_Identity`*"]
    pub fn Subject(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Security_Authentication_Identity`*"]
    pub fn KeyId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Security_Authentication_Identity`*"]
    pub fn KeyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for EnterpriseKeyCredentialRegistrationInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Identity.EnterpriseKeyCredentialRegistrationInfo;{38321acc-672b-4823-b603-6b3c753daf97})");
}
unsafe impl ::windows::core::Interface for EnterpriseKeyCredentialRegistrationInfo {
    type Vtable = IEnterpriseKeyCredentialRegistrationInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x38321acc_672b_4823_b603_6b3c753daf97);
}
impl ::windows::core::RuntimeName for EnterpriseKeyCredentialRegistrationInfo {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.EnterpriseKeyCredentialRegistrationInfo";
}
impl ::core::convert::From<EnterpriseKeyCredentialRegistrationInfo> for ::windows::core::IUnknown {
    fn from(value: EnterpriseKeyCredentialRegistrationInfo) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&EnterpriseKeyCredentialRegistrationInfo> for ::windows::core::IUnknown {
    fn from(value: &EnterpriseKeyCredentialRegistrationInfo) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for EnterpriseKeyCredentialRegistrationInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a EnterpriseKeyCredentialRegistrationInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<EnterpriseKeyCredentialRegistrationInfo> for ::windows::core::IInspectable {
    fn from(value: EnterpriseKeyCredentialRegistrationInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&EnterpriseKeyCredentialRegistrationInfo> for ::windows::core::IInspectable {
    fn from(value: &EnterpriseKeyCredentialRegistrationInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for EnterpriseKeyCredentialRegistrationInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a EnterpriseKeyCredentialRegistrationInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for EnterpriseKeyCredentialRegistrationInfo {}
unsafe impl ::core::marker::Sync for EnterpriseKeyCredentialRegistrationInfo {}
#[doc = "*Required features: `Security_Authentication_Identity`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct EnterpriseKeyCredentialRegistrationManager(pub ::windows::core::IInspectable);
impl EnterpriseKeyCredentialRegistrationManager {
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Security_Authentication_Identity`, `Foundation`, `Foundation_Collections`*"]
    pub fn GetRegistrationsAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<EnterpriseKeyCredentialRegistrationInfo>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<EnterpriseKeyCredentialRegistrationInfo>>>(result__)
        }
    }
    #[doc = "*Required features: `Security_Authentication_Identity`*"]
    pub fn Current() -> ::windows::core::Result<EnterpriseKeyCredentialRegistrationManager> {
        Self::IEnterpriseKeyCredentialRegistrationManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<EnterpriseKeyCredentialRegistrationManager>(result__)
        })
    }
    pub fn IEnterpriseKeyCredentialRegistrationManagerStatics<R, F: FnOnce(&IEnterpriseKeyCredentialRegistrationManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<EnterpriseKeyCredentialRegistrationManager, IEnterpriseKeyCredentialRegistrationManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for EnterpriseKeyCredentialRegistrationManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Identity.EnterpriseKeyCredentialRegistrationManager;{83f3be3f-a25f-4cba-bb8e-bdc32d03c297})");
}
unsafe impl ::windows::core::Interface for EnterpriseKeyCredentialRegistrationManager {
    type Vtable = IEnterpriseKeyCredentialRegistrationManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x83f3be3f_a25f_4cba_bb8e_bdc32d03c297);
}
impl ::windows::core::RuntimeName for EnterpriseKeyCredentialRegistrationManager {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.EnterpriseKeyCredentialRegistrationManager";
}
impl ::core::convert::From<EnterpriseKeyCredentialRegistrationManager> for ::windows::core::IUnknown {
    fn from(value: EnterpriseKeyCredentialRegistrationManager) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&EnterpriseKeyCredentialRegistrationManager> for ::windows::core::IUnknown {
    fn from(value: &EnterpriseKeyCredentialRegistrationManager) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for EnterpriseKeyCredentialRegistrationManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a EnterpriseKeyCredentialRegistrationManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<EnterpriseKeyCredentialRegistrationManager> for ::windows::core::IInspectable {
    fn from(value: EnterpriseKeyCredentialRegistrationManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&EnterpriseKeyCredentialRegistrationManager> for ::windows::core::IInspectable {
    fn from(value: &EnterpriseKeyCredentialRegistrationManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for EnterpriseKeyCredentialRegistrationManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a EnterpriseKeyCredentialRegistrationManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for EnterpriseKeyCredentialRegistrationManager {}
unsafe impl ::core::marker::Sync for EnterpriseKeyCredentialRegistrationManager {}
#[repr(transparent)]
#[doc(hidden)]
pub struct IEnterpriseKeyCredentialRegistrationInfo(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IEnterpriseKeyCredentialRegistrationInfo {
    type Vtable = IEnterpriseKeyCredentialRegistrationInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x38321acc_672b_4823_b603_6b3c753daf97);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnterpriseKeyCredentialRegistrationInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IEnterpriseKeyCredentialRegistrationManager(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IEnterpriseKeyCredentialRegistrationManager {
    type Vtable = IEnterpriseKeyCredentialRegistrationManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x83f3be3f_a25f_4cba_bb8e_bdc32d03c297);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnterpriseKeyCredentialRegistrationManager_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IEnterpriseKeyCredentialRegistrationManagerStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IEnterpriseKeyCredentialRegistrationManagerStatics {
    type Vtable = IEnterpriseKeyCredentialRegistrationManagerStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x77b85e9e_acf4_4bc0_bac2_40bb46efbb3f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnterpriseKeyCredentialRegistrationManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
