#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "Security_Authentication_Identity_Core")]
pub mod Core;
#[cfg(feature = "Security_Authentication_Identity_Provider")]
pub mod Provider;
#[doc = "*Required features: `Security_Authentication_Identity`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct EnterpriseKeyCredentialRegistrationInfo(::windows::runtime::IInspectable);
impl EnterpriseKeyCredentialRegistrationInfo {
    #[doc = "*Required features: `Security_Authentication_Identity`*"]
    pub fn TenantId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Security_Authentication_Identity`*"]
    pub fn TenantName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Security_Authentication_Identity`*"]
    pub fn Subject(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Security_Authentication_Identity`*"]
    pub fn KeyId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Security_Authentication_Identity`*"]
    pub fn KeyName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for EnterpriseKeyCredentialRegistrationInfo {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Identity.EnterpriseKeyCredentialRegistrationInfo;{38321acc-672b-4823-b603-6b3c753daf97})");
}
unsafe impl ::windows::runtime::Interface for EnterpriseKeyCredentialRegistrationInfo {
    type Vtable = IEnterpriseKeyCredentialRegistrationInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(942807756, 26411, 18467, [182, 3, 107, 60, 117, 61, 175, 151]);
}
impl ::windows::runtime::RuntimeName for EnterpriseKeyCredentialRegistrationInfo {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.EnterpriseKeyCredentialRegistrationInfo";
}
impl ::std::convert::From<EnterpriseKeyCredentialRegistrationInfo> for ::windows::runtime::IUnknown {
    fn from(value: EnterpriseKeyCredentialRegistrationInfo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&EnterpriseKeyCredentialRegistrationInfo> for ::windows::runtime::IUnknown {
    fn from(value: &EnterpriseKeyCredentialRegistrationInfo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for EnterpriseKeyCredentialRegistrationInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &EnterpriseKeyCredentialRegistrationInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<EnterpriseKeyCredentialRegistrationInfo> for ::windows::runtime::IInspectable {
    fn from(value: EnterpriseKeyCredentialRegistrationInfo) -> Self {
        value.0
    }
}
impl ::std::convert::From<&EnterpriseKeyCredentialRegistrationInfo> for ::windows::runtime::IInspectable {
    fn from(value: &EnterpriseKeyCredentialRegistrationInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for EnterpriseKeyCredentialRegistrationInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a EnterpriseKeyCredentialRegistrationInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for EnterpriseKeyCredentialRegistrationInfo {}
unsafe impl ::std::marker::Sync for EnterpriseKeyCredentialRegistrationInfo {}
#[doc = "*Required features: `Security_Authentication_Identity`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct EnterpriseKeyCredentialRegistrationManager(::windows::runtime::IInspectable);
impl EnterpriseKeyCredentialRegistrationManager {
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Security_Authentication_Identity`, `Foundation`, `Foundation_Collections`*"]
    pub fn GetRegistrationsAsync(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<EnterpriseKeyCredentialRegistrationInfo>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<EnterpriseKeyCredentialRegistrationInfo>>>(result__)
        }
    }
    #[doc = "*Required features: `Security_Authentication_Identity`*"]
    pub fn Current() -> ::windows::runtime::Result<EnterpriseKeyCredentialRegistrationManager> {
        Self::IEnterpriseKeyCredentialRegistrationManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<EnterpriseKeyCredentialRegistrationManager>(result__)
        })
    }
    pub fn IEnterpriseKeyCredentialRegistrationManagerStatics<R, F: FnOnce(&IEnterpriseKeyCredentialRegistrationManagerStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<EnterpriseKeyCredentialRegistrationManager, IEnterpriseKeyCredentialRegistrationManagerStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for EnterpriseKeyCredentialRegistrationManager {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Identity.EnterpriseKeyCredentialRegistrationManager;{83f3be3f-a25f-4cba-bb8e-bdc32d03c297})");
}
unsafe impl ::windows::runtime::Interface for EnterpriseKeyCredentialRegistrationManager {
    type Vtable = IEnterpriseKeyCredentialRegistrationManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2213789247, 41567, 19642, [187, 142, 189, 195, 45, 3, 194, 151]);
}
impl ::windows::runtime::RuntimeName for EnterpriseKeyCredentialRegistrationManager {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.EnterpriseKeyCredentialRegistrationManager";
}
impl ::std::convert::From<EnterpriseKeyCredentialRegistrationManager> for ::windows::runtime::IUnknown {
    fn from(value: EnterpriseKeyCredentialRegistrationManager) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&EnterpriseKeyCredentialRegistrationManager> for ::windows::runtime::IUnknown {
    fn from(value: &EnterpriseKeyCredentialRegistrationManager) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for EnterpriseKeyCredentialRegistrationManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &EnterpriseKeyCredentialRegistrationManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<EnterpriseKeyCredentialRegistrationManager> for ::windows::runtime::IInspectable {
    fn from(value: EnterpriseKeyCredentialRegistrationManager) -> Self {
        value.0
    }
}
impl ::std::convert::From<&EnterpriseKeyCredentialRegistrationManager> for ::windows::runtime::IInspectable {
    fn from(value: &EnterpriseKeyCredentialRegistrationManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for EnterpriseKeyCredentialRegistrationManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a EnterpriseKeyCredentialRegistrationManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for EnterpriseKeyCredentialRegistrationManager {}
unsafe impl ::std::marker::Sync for EnterpriseKeyCredentialRegistrationManager {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IEnterpriseKeyCredentialRegistrationInfo(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IEnterpriseKeyCredentialRegistrationInfo {
    type Vtable = IEnterpriseKeyCredentialRegistrationInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(942807756, 26411, 18467, [182, 3, 107, 60, 117, 61, 175, 151]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnterpriseKeyCredentialRegistrationInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IEnterpriseKeyCredentialRegistrationManager(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IEnterpriseKeyCredentialRegistrationManager {
    type Vtable = IEnterpriseKeyCredentialRegistrationManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2213789247, 41567, 19642, [187, 142, 189, 195, 45, 3, 194, 151]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnterpriseKeyCredentialRegistrationManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IEnterpriseKeyCredentialRegistrationManagerStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IEnterpriseKeyCredentialRegistrationManagerStatics {
    type Vtable = IEnterpriseKeyCredentialRegistrationManagerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2008571550, 44276, 19392, [186, 194, 64, 187, 70, 239, 187, 63]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnterpriseKeyCredentialRegistrationManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
