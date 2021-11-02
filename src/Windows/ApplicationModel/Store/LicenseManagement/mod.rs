#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ILicenseManagerStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILicenseManagerStatics {
    type Vtable = ILicenseManagerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3047963360, 55879, 20256, [154, 35, 9, 24, 44, 148, 118, 255]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILicenseManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, license: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, contentids: ::windows::runtime::RawPtr, keyids: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ILicenseManagerStatics2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILicenseManagerStatics2 {
    type Vtable = ILicenseManagerStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2871968891, 8057, 17536, [184, 126, 44, 73, 158, 96, 27, 163]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILicenseManagerStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, refreshoption: LicenseRefreshOption, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ILicenseSatisfactionInfo(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILicenseSatisfactionInfo {
    type Vtable = ILicenseSatisfactionInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1019981967, 56113, 18645, [131, 132, 250, 23, 200, 20, 116, 226]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILicenseSatisfactionInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ILicenseSatisfactionResult(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILicenseSatisfactionResult {
    type Vtable = ILicenseSatisfactionResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1013403507, 15495, 20193, [130, 1, 244, 40, 53, 155, 211, 175]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILicenseSatisfactionResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `ApplicationModel_Store_LicenseManagement`*"]
pub struct LicenseManager {}
impl LicenseManager {
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `ApplicationModel_Store_LicenseManagement`, `Foundation`, `Storage_Streams`*"]
    pub fn AddLicenseAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(license: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        Self::ILicenseManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), license.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `ApplicationModel_Store_LicenseManagement`, `Foundation`, `Foundation_Collections`*"]
    pub fn GetSatisfactionInfosAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>>>(contentids: Param0, keyids: Param1) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<LicenseSatisfactionResult>> {
        Self::ILicenseManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), contentids.into_param().abi(), keyids.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<LicenseSatisfactionResult>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Store_LicenseManagement`, `Foundation`*"]
    pub fn RefreshLicensesAsync(refreshoption: LicenseRefreshOption) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        Self::ILicenseManagerStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), refreshoption, &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        })
    }
    pub fn ILicenseManagerStatics<R, F: FnOnce(&ILicenseManagerStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<LicenseManager, ILicenseManagerStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ILicenseManagerStatics2<R, F: FnOnce(&ILicenseManagerStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<LicenseManager, ILicenseManagerStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for LicenseManager {
    const NAME: &'static str = "Windows.ApplicationModel.Store.LicenseManagement.LicenseManager";
}
#[doc = "*Required features: `ApplicationModel_Store_LicenseManagement`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct LicenseRefreshOption(pub i32);
impl LicenseRefreshOption {
    pub const RunningLicenses: LicenseRefreshOption = LicenseRefreshOption(0i32);
    pub const AllLicenses: LicenseRefreshOption = LicenseRefreshOption(1i32);
}
impl ::std::convert::From<i32> for LicenseRefreshOption {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for LicenseRefreshOption {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for LicenseRefreshOption {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Store.LicenseManagement.LicenseRefreshOption;i4)");
}
#[doc = "*Required features: `ApplicationModel_Store_LicenseManagement`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct LicenseSatisfactionInfo(::windows::runtime::IInspectable);
impl LicenseSatisfactionInfo {
    #[doc = "*Required features: `ApplicationModel_Store_LicenseManagement`*"]
    pub fn SatisfiedByDevice(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Store_LicenseManagement`*"]
    pub fn SatisfiedByOpenLicense(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Store_LicenseManagement`*"]
    pub fn SatisfiedByTrial(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Store_LicenseManagement`*"]
    pub fn SatisfiedByPass(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Store_LicenseManagement`*"]
    pub fn SatisfiedByInstallMedia(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Store_LicenseManagement`*"]
    pub fn SatisfiedBySignedInUser(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Store_LicenseManagement`*"]
    pub fn IsSatisfied(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for LicenseSatisfactionInfo {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Store.LicenseManagement.LicenseSatisfactionInfo;{3ccbb08f-db31-48d5-8384-fa17c81474e2})");
}
unsafe impl ::windows::runtime::Interface for LicenseSatisfactionInfo {
    type Vtable = ILicenseSatisfactionInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1019981967, 56113, 18645, [131, 132, 250, 23, 200, 20, 116, 226]);
}
impl ::windows::runtime::RuntimeName for LicenseSatisfactionInfo {
    const NAME: &'static str = "Windows.ApplicationModel.Store.LicenseManagement.LicenseSatisfactionInfo";
}
impl ::std::convert::From<LicenseSatisfactionInfo> for ::windows::runtime::IUnknown {
    fn from(value: LicenseSatisfactionInfo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&LicenseSatisfactionInfo> for ::windows::runtime::IUnknown {
    fn from(value: &LicenseSatisfactionInfo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for LicenseSatisfactionInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &LicenseSatisfactionInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<LicenseSatisfactionInfo> for ::windows::runtime::IInspectable {
    fn from(value: LicenseSatisfactionInfo) -> Self {
        value.0
    }
}
impl ::std::convert::From<&LicenseSatisfactionInfo> for ::windows::runtime::IInspectable {
    fn from(value: &LicenseSatisfactionInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for LicenseSatisfactionInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a LicenseSatisfactionInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for LicenseSatisfactionInfo {}
unsafe impl ::std::marker::Sync for LicenseSatisfactionInfo {}
#[doc = "*Required features: `ApplicationModel_Store_LicenseManagement`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct LicenseSatisfactionResult(::windows::runtime::IInspectable);
impl LicenseSatisfactionResult {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Store_LicenseManagement`, `Foundation_Collections`*"]
    pub fn LicenseSatisfactionInfos(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, LicenseSatisfactionInfo>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, LicenseSatisfactionInfo>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Store_LicenseManagement`*"]
    pub fn ExtendedError(&self) -> ::windows::runtime::Result<::windows::runtime::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::HRESULT = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HRESULT>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for LicenseSatisfactionResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Store.LicenseManagement.LicenseSatisfactionResult;{3c674f73-3c87-4ee1-8201-f428359bd3af})");
}
unsafe impl ::windows::runtime::Interface for LicenseSatisfactionResult {
    type Vtable = ILicenseSatisfactionResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1013403507, 15495, 20193, [130, 1, 244, 40, 53, 155, 211, 175]);
}
impl ::windows::runtime::RuntimeName for LicenseSatisfactionResult {
    const NAME: &'static str = "Windows.ApplicationModel.Store.LicenseManagement.LicenseSatisfactionResult";
}
impl ::std::convert::From<LicenseSatisfactionResult> for ::windows::runtime::IUnknown {
    fn from(value: LicenseSatisfactionResult) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&LicenseSatisfactionResult> for ::windows::runtime::IUnknown {
    fn from(value: &LicenseSatisfactionResult) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for LicenseSatisfactionResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &LicenseSatisfactionResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<LicenseSatisfactionResult> for ::windows::runtime::IInspectable {
    fn from(value: LicenseSatisfactionResult) -> Self {
        value.0
    }
}
impl ::std::convert::From<&LicenseSatisfactionResult> for ::windows::runtime::IInspectable {
    fn from(value: &LicenseSatisfactionResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for LicenseSatisfactionResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a LicenseSatisfactionResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for LicenseSatisfactionResult {}
unsafe impl ::std::marker::Sync for LicenseSatisfactionResult {}
