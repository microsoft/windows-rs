#[doc(hidden)]
#[repr(transparent)]
pub struct ILicenseManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILicenseManagerStatics {
    type Vtable = ILicenseManagerStatics_Vtbl;
}
impl ::core::clone::Clone for ILicenseManagerStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ILicenseManagerStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb5ac3ae0_da47_4f20_9a23_09182c9476ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILicenseManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub AddLicenseAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, license: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    AddLicenseAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSatisfactionInfosAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contentids: *mut ::core::ffi::c_void, keyids: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSatisfactionInfosAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILicenseManagerStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILicenseManagerStatics2 {
    type Vtable = ILicenseManagerStatics2_Vtbl;
}
impl ::core::clone::Clone for ILicenseManagerStatics2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ILicenseManagerStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xab2ec47b_1f79_4480_b87e_2c499e601ba3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILicenseManagerStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub RefreshLicensesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, refreshoption: LicenseRefreshOption, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RefreshLicensesAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILicenseSatisfactionInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILicenseSatisfactionInfo {
    type Vtable = ILicenseSatisfactionInfo_Vtbl;
}
impl ::core::clone::Clone for ILicenseSatisfactionInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ILicenseSatisfactionInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3ccbb08f_db31_48d5_8384_fa17c81474e2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILicenseSatisfactionInfo_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SatisfiedByDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SatisfiedByOpenLicense: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SatisfiedByTrial: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SatisfiedByPass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SatisfiedByInstallMedia: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SatisfiedBySignedInUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsSatisfied: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILicenseSatisfactionResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILicenseSatisfactionResult {
    type Vtable = ILicenseSatisfactionResult_Vtbl;
}
impl ::core::clone::Clone for ILicenseSatisfactionResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ILicenseSatisfactionResult {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3c674f73_3c87_4ee1_8201_f428359bd3af);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILicenseSatisfactionResult_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub LicenseSatisfactionInfos: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    LicenseSatisfactionInfos: usize,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Store_LicenseManagement\"`*"]
pub struct LicenseManager;
impl LicenseManager {
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn AddLicenseAsync<P0>(license: P0) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::TryIntoParam<super::super::super::Storage::Streams::IBuffer>,
    {
        Self::ILicenseManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AddLicenseAsync)(::windows_core::Interface::as_raw(this), license.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetSatisfactionInfosAsync<P0, P1>(contentids: P0, keyids: P1) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperation<LicenseSatisfactionResult>>
    where
        P0: ::windows_core::TryIntoParam<super::super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>,
        P1: ::windows_core::TryIntoParam<super::super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>,
    {
        Self::ILicenseManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetSatisfactionInfosAsync)(::windows_core::Interface::as_raw(this), contentids.try_into_param()?.abi(), keyids.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RefreshLicensesAsync(refreshoption: LicenseRefreshOption) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        Self::ILicenseManagerStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RefreshLicensesAsync)(::windows_core::Interface::as_raw(this), refreshoption, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ILicenseManagerStatics<R, F: FnOnce(&ILicenseManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<LicenseManager, ILicenseManagerStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ILicenseManagerStatics2<R, F: FnOnce(&ILicenseManagerStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<LicenseManager, ILicenseManagerStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for LicenseManager {
    const NAME: &'static str = "Windows.ApplicationModel.Store.LicenseManagement.LicenseManager";
}
#[doc = "*Required features: `\"ApplicationModel_Store_LicenseManagement\"`*"]
#[repr(transparent)]
pub struct LicenseSatisfactionInfo(::windows_core::IUnknown);
impl LicenseSatisfactionInfo {
    pub fn SatisfiedByDevice(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SatisfiedByDevice)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SatisfiedByOpenLicense(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SatisfiedByOpenLicense)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SatisfiedByTrial(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SatisfiedByTrial)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SatisfiedByPass(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SatisfiedByPass)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SatisfiedByInstallMedia(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SatisfiedByInstallMedia)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SatisfiedBySignedInUser(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SatisfiedBySignedInUser)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsSatisfied(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsSatisfied)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for LicenseSatisfactionInfo {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Store.LicenseManagement.LicenseSatisfactionInfo;{3ccbb08f-db31-48d5-8384-fa17c81474e2})");
}
impl ::core::clone::Clone for LicenseSatisfactionInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for LicenseSatisfactionInfo {
    type Vtable = ILicenseSatisfactionInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for LicenseSatisfactionInfo {
    const IID: ::windows_core::GUID = <ILicenseSatisfactionInfo as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for LicenseSatisfactionInfo {
    const NAME: &'static str = "Windows.ApplicationModel.Store.LicenseManagement.LicenseSatisfactionInfo";
}
::windows_core::imp::interface_hierarchy!(LicenseSatisfactionInfo, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for LicenseSatisfactionInfo {}
unsafe impl ::core::marker::Sync for LicenseSatisfactionInfo {}
#[doc = "*Required features: `\"ApplicationModel_Store_LicenseManagement\"`*"]
#[repr(transparent)]
pub struct LicenseSatisfactionResult(::windows_core::IUnknown);
impl LicenseSatisfactionResult {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn LicenseSatisfactionInfos(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, LicenseSatisfactionInfo>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LicenseSatisfactionInfos)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedError)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for LicenseSatisfactionResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Store.LicenseManagement.LicenseSatisfactionResult;{3c674f73-3c87-4ee1-8201-f428359bd3af})");
}
impl ::core::clone::Clone for LicenseSatisfactionResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for LicenseSatisfactionResult {
    type Vtable = ILicenseSatisfactionResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for LicenseSatisfactionResult {
    const IID: ::windows_core::GUID = <ILicenseSatisfactionResult as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for LicenseSatisfactionResult {
    const NAME: &'static str = "Windows.ApplicationModel.Store.LicenseManagement.LicenseSatisfactionResult";
}
::windows_core::imp::interface_hierarchy!(LicenseSatisfactionResult, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for LicenseSatisfactionResult {}
unsafe impl ::core::marker::Sync for LicenseSatisfactionResult {}
#[doc = "*Required features: `\"ApplicationModel_Store_LicenseManagement\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::windows_core::TypeKind for LicenseRefreshOption {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for LicenseRefreshOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LicenseRefreshOption").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for LicenseRefreshOption {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Store.LicenseManagement.LicenseRefreshOption;i4)");
}
