#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc(hidden)]
#[repr(transparent)]
pub struct ILicenseManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILicenseManagerStatics {
    type Vtable = ILicenseManagerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb5ac3ae0_da47_4f20_9a23_09182c9476ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILicenseManagerStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub AddLicenseAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, license: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    AddLicenseAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSatisfactionInfosAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contentids: ::windows::core::RawPtr, keyids: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSatisfactionInfosAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILicenseManagerStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILicenseManagerStatics2 {
    type Vtable = ILicenseManagerStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xab2ec47b_1f79_4480_b87e_2c499e601ba3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILicenseManagerStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub RefreshLicensesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, refreshoption: LicenseRefreshOption, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RefreshLicensesAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILicenseSatisfactionInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILicenseSatisfactionInfo {
    type Vtable = ILicenseSatisfactionInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3ccbb08f_db31_48d5_8384_fa17c81474e2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILicenseSatisfactionInfo_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub SatisfiedByDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SatisfiedByOpenLicense: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SatisfiedByTrial: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SatisfiedByPass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SatisfiedByInstallMedia: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SatisfiedBySignedInUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsSatisfied: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILicenseSatisfactionResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILicenseSatisfactionResult {
    type Vtable = ILicenseSatisfactionResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3c674f73_3c87_4ee1_8201_f428359bd3af);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILicenseSatisfactionResult_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub LicenseSatisfactionInfos: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    LicenseSatisfactionInfos: usize,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Store_LicenseManagement\"`*"]
pub struct LicenseManager {}
impl LicenseManager {
    #[doc = "*Required features: `\"ApplicationModel_Store_LicenseManagement\"`, `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn AddLicenseAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(license: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        Self::ILicenseManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AddLicenseAsync)(::core::mem::transmute_copy(this), license.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_LicenseManagement\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetSatisfactionInfosAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(contentids: Param0, keyids: Param1) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<LicenseSatisfactionResult>> {
        Self::ILicenseManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetSatisfactionInfosAsync)(::core::mem::transmute_copy(this), contentids.into_param().abi(), keyids.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<LicenseSatisfactionResult>>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_LicenseManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RefreshLicensesAsync(refreshoption: LicenseRefreshOption) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        Self::ILicenseManagerStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RefreshLicensesAsync)(::core::mem::transmute_copy(this), refreshoption, &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
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
#[doc = "*Required features: `\"ApplicationModel_Store_LicenseManagement\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for LicenseRefreshOption {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for LicenseRefreshOption {
    type Abi = Self;
}
impl ::core::fmt::Debug for LicenseRefreshOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LicenseRefreshOption").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LicenseRefreshOption {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Store.LicenseManagement.LicenseRefreshOption;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Store_LicenseManagement\"`*"]
#[repr(transparent)]
pub struct LicenseSatisfactionInfo(::windows::core::IUnknown);
impl LicenseSatisfactionInfo {
    #[doc = "*Required features: `\"ApplicationModel_Store_LicenseManagement\"`*"]
    pub fn SatisfiedByDevice(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SatisfiedByDevice)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_LicenseManagement\"`*"]
    pub fn SatisfiedByOpenLicense(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SatisfiedByOpenLicense)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_LicenseManagement\"`*"]
    pub fn SatisfiedByTrial(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SatisfiedByTrial)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_LicenseManagement\"`*"]
    pub fn SatisfiedByPass(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SatisfiedByPass)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_LicenseManagement\"`*"]
    pub fn SatisfiedByInstallMedia(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SatisfiedByInstallMedia)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_LicenseManagement\"`*"]
    pub fn SatisfiedBySignedInUser(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SatisfiedBySignedInUser)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_LicenseManagement\"`*"]
    pub fn IsSatisfied(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsSatisfied)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
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
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for LicenseSatisfactionInfo {
    type Vtable = ILicenseSatisfactionInfo_Vtbl;
    const IID: ::windows::core::GUID = <ILicenseSatisfactionInfo as ::windows::core::Interface>::IID;
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LicenseSatisfactionInfo {
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LicenseSatisfactionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for LicenseSatisfactionInfo {}
unsafe impl ::core::marker::Sync for LicenseSatisfactionInfo {}
#[doc = "*Required features: `\"ApplicationModel_Store_LicenseManagement\"`*"]
#[repr(transparent)]
pub struct LicenseSatisfactionResult(::windows::core::IUnknown);
impl LicenseSatisfactionResult {
    #[doc = "*Required features: `\"ApplicationModel_Store_LicenseManagement\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn LicenseSatisfactionInfos(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, LicenseSatisfactionInfo>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LicenseSatisfactionInfos)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, LicenseSatisfactionInfo>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_LicenseManagement\"`*"]
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExtendedError)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
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
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for LicenseSatisfactionResult {
    type Vtable = ILicenseSatisfactionResult_Vtbl;
    const IID: ::windows::core::GUID = <ILicenseSatisfactionResult as ::windows::core::Interface>::IID;
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LicenseSatisfactionResult {
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LicenseSatisfactionResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for LicenseSatisfactionResult {}
unsafe impl ::core::marker::Sync for LicenseSatisfactionResult {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
