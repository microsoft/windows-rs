#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc(hidden)]
#[repr(transparent)]
pub struct ILicenseManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILicenseManagerStatics {
    type Vtable = ILicenseManagerStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb5ac3ae0_da47_4f20_9a23_09182c9476ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILicenseManagerStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, license: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contentids: ::windows::core::RawPtr, keyids: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ILicenseManagerStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILicenseManagerStatics2 {
    type Vtable = ILicenseManagerStatics2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xab2ec47b_1f79_4480_b87e_2c499e601ba3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILicenseManagerStatics2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, refreshoption: LicenseRefreshOption, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ILicenseSatisfactionInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILicenseSatisfactionInfo {
    type Vtable = ILicenseSatisfactionInfoVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3ccbb08f_db31_48d5_8384_fa17c81474e2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILicenseSatisfactionInfoVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ILicenseSatisfactionResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILicenseSatisfactionResult {
    type Vtable = ILicenseSatisfactionResultVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3c674f73_3c87_4ee1_8201_f428359bd3af);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILicenseSatisfactionResultVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'ApplicationModel_Store_LicenseManagement'*"]
pub struct LicenseManager {}
impl LicenseManager {
    #[doc = "*Required features: 'ApplicationModel_Store_LicenseManagement', 'Foundation', 'Storage_Streams'*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn AddLicenseAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(license: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        Self::ILicenseManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), license.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc = "*Required features: 'ApplicationModel_Store_LicenseManagement', 'Foundation', 'Foundation_Collections'*"]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn GetSatisfactionInfosAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(contentids: Param0, keyids: Param1) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<LicenseSatisfactionResult>> {
        Self::ILicenseManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), contentids.into_param().abi(), keyids.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<LicenseSatisfactionResult>>(result__)
        })
    }
    #[doc = "*Required features: 'ApplicationModel_Store_LicenseManagement', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RefreshLicensesAsync(refreshoption: LicenseRefreshOption) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        Self::ILicenseManagerStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), refreshoption, &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ILicenseManagerStatics<R, F: FnOnce(&ILicenseManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<LicenseManager, ILicenseManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn ILicenseManagerStatics2<R, F: FnOnce(&ILicenseManagerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<LicenseManager, ILicenseManagerStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for LicenseManager {
    const NAME: &'static str = "Windows.ApplicationModel.Store.LicenseManagement.LicenseManager";
}
#[doc = "*Required features: 'ApplicationModel_Store_LicenseManagement'*"]
#[repr(transparent)]
pub struct LicenseRefreshOption(pub i32);
impl LicenseRefreshOption {
    pub const RunningLicenses: Self = Self(0i32);
    pub const AllLicenses: Self = Self(1i32);
}
impl ::core::marker::Copy for LicenseRefreshOption {}
impl ::core::clone::Clone for LicenseRefreshOption {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LicenseRefreshOption {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for LicenseRefreshOption {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LicenseRefreshOption {}
impl ::core::fmt::Debug for LicenseRefreshOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LicenseRefreshOption").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LicenseRefreshOption {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Store.LicenseManagement.LicenseRefreshOption;i4)");
}
impl ::windows::core::DefaultType for LicenseRefreshOption {
    type DefaultType = Self;
}
#[doc = "*Required features: 'ApplicationModel_Store_LicenseManagement'*"]
#[repr(transparent)]
pub struct LicenseSatisfactionInfo(::windows::core::IUnknown);
impl LicenseSatisfactionInfo {
    #[doc = "*Required features: 'ApplicationModel_Store_LicenseManagement'*"]
    pub fn SatisfiedByDevice(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Store_LicenseManagement'*"]
    pub fn SatisfiedByOpenLicense(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Store_LicenseManagement'*"]
    pub fn SatisfiedByTrial(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Store_LicenseManagement'*"]
    pub fn SatisfiedByPass(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Store_LicenseManagement'*"]
    pub fn SatisfiedByInstallMedia(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Store_LicenseManagement'*"]
    pub fn SatisfiedBySignedInUser(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Store_LicenseManagement'*"]
    pub fn IsSatisfied(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for LicenseSatisfactionInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LicenseSatisfactionInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LicenseSatisfactionInfo {}
impl ::core::fmt::Debug for LicenseSatisfactionInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LicenseSatisfactionInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LicenseSatisfactionInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Store.LicenseManagement.LicenseSatisfactionInfo;{3ccbb08f-db31-48d5-8384-fa17c81474e2})");
}
unsafe impl ::windows::core::Interface for LicenseSatisfactionInfo {
    type Vtable = ILicenseSatisfactionInfoVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3ccbb08f_db31_48d5_8384_fa17c81474e2);
}
impl ::windows::core::RuntimeName for LicenseSatisfactionInfo {
    const NAME: &'static str = "Windows.ApplicationModel.Store.LicenseManagement.LicenseSatisfactionInfo";
}
impl ::core::convert::From<LicenseSatisfactionInfo> for ::windows::core::IUnknown {
    fn from(value: LicenseSatisfactionInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LicenseSatisfactionInfo> for ::windows::core::IUnknown {
    fn from(value: &LicenseSatisfactionInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LicenseSatisfactionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &LicenseSatisfactionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LicenseSatisfactionInfo> for ::windows::core::IInspectable {
    fn from(value: LicenseSatisfactionInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LicenseSatisfactionInfo> for ::windows::core::IInspectable {
    fn from(value: &LicenseSatisfactionInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LicenseSatisfactionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &LicenseSatisfactionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for LicenseSatisfactionInfo {}
unsafe impl ::core::marker::Sync for LicenseSatisfactionInfo {}
#[doc = "*Required features: 'ApplicationModel_Store_LicenseManagement'*"]
#[repr(transparent)]
pub struct LicenseSatisfactionResult(::windows::core::IUnknown);
impl LicenseSatisfactionResult {
    #[doc = "*Required features: 'ApplicationModel_Store_LicenseManagement', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn LicenseSatisfactionInfos(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, LicenseSatisfactionInfo>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, LicenseSatisfactionInfo>>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Store_LicenseManagement'*"]
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
}
impl ::core::clone::Clone for LicenseSatisfactionResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LicenseSatisfactionResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LicenseSatisfactionResult {}
impl ::core::fmt::Debug for LicenseSatisfactionResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LicenseSatisfactionResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LicenseSatisfactionResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Store.LicenseManagement.LicenseSatisfactionResult;{3c674f73-3c87-4ee1-8201-f428359bd3af})");
}
unsafe impl ::windows::core::Interface for LicenseSatisfactionResult {
    type Vtable = ILicenseSatisfactionResultVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3c674f73_3c87_4ee1_8201_f428359bd3af);
}
impl ::windows::core::RuntimeName for LicenseSatisfactionResult {
    const NAME: &'static str = "Windows.ApplicationModel.Store.LicenseManagement.LicenseSatisfactionResult";
}
impl ::core::convert::From<LicenseSatisfactionResult> for ::windows::core::IUnknown {
    fn from(value: LicenseSatisfactionResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LicenseSatisfactionResult> for ::windows::core::IUnknown {
    fn from(value: &LicenseSatisfactionResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LicenseSatisfactionResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &LicenseSatisfactionResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LicenseSatisfactionResult> for ::windows::core::IInspectable {
    fn from(value: LicenseSatisfactionResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LicenseSatisfactionResult> for ::windows::core::IInspectable {
    fn from(value: &LicenseSatisfactionResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LicenseSatisfactionResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &LicenseSatisfactionResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for LicenseSatisfactionResult {}
unsafe impl ::core::marker::Sync for LicenseSatisfactionResult {}
