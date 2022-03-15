#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "ApplicationModel_Store_Preview_InstallControl")]
pub mod InstallControl;
#[doc = "*Required features: `\"ApplicationModel_Store_Preview\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DeliveryOptimizationDownloadMode(pub i32);
impl DeliveryOptimizationDownloadMode {
    pub const Simple: Self = Self(0i32);
    pub const HttpOnly: Self = Self(1i32);
    pub const Lan: Self = Self(2i32);
    pub const Group: Self = Self(3i32);
    pub const Internet: Self = Self(4i32);
    pub const Bypass: Self = Self(5i32);
}
impl ::core::marker::Copy for DeliveryOptimizationDownloadMode {}
impl ::core::clone::Clone for DeliveryOptimizationDownloadMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DeliveryOptimizationDownloadMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DeliveryOptimizationDownloadMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for DeliveryOptimizationDownloadMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeliveryOptimizationDownloadMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DeliveryOptimizationDownloadMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Store.Preview.DeliveryOptimizationDownloadMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Store_Preview\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DeliveryOptimizationDownloadModeSource(pub i32);
impl DeliveryOptimizationDownloadModeSource {
    pub const Default: Self = Self(0i32);
    pub const Policy: Self = Self(1i32);
}
impl ::core::marker::Copy for DeliveryOptimizationDownloadModeSource {}
impl ::core::clone::Clone for DeliveryOptimizationDownloadModeSource {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DeliveryOptimizationDownloadModeSource {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DeliveryOptimizationDownloadModeSource {
    type Abi = Self;
}
impl ::core::fmt::Debug for DeliveryOptimizationDownloadModeSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeliveryOptimizationDownloadModeSource").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DeliveryOptimizationDownloadModeSource {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Store.Preview.DeliveryOptimizationDownloadModeSource;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Store_Preview\"`*"]
#[repr(transparent)]
pub struct DeliveryOptimizationSettings(::windows::core::IUnknown);
impl DeliveryOptimizationSettings {
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview\"`*"]
    pub fn DownloadMode(&self) -> ::windows::core::Result<DeliveryOptimizationDownloadMode> {
        let this = self;
        unsafe {
            let mut result__: DeliveryOptimizationDownloadMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DownloadMode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DeliveryOptimizationDownloadMode>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview\"`*"]
    pub fn DownloadModeSource(&self) -> ::windows::core::Result<DeliveryOptimizationDownloadModeSource> {
        let this = self;
        unsafe {
            let mut result__: DeliveryOptimizationDownloadModeSource = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DownloadModeSource)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DeliveryOptimizationDownloadModeSource>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview\"`*"]
    pub fn GetCurrentSettings() -> ::windows::core::Result<DeliveryOptimizationSettings> {
        Self::IDeliveryOptimizationSettingsStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetCurrentSettings)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DeliveryOptimizationSettings>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IDeliveryOptimizationSettingsStatics<R, F: FnOnce(&IDeliveryOptimizationSettingsStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DeliveryOptimizationSettings, IDeliveryOptimizationSettingsStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for DeliveryOptimizationSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DeliveryOptimizationSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DeliveryOptimizationSettings {}
impl ::core::fmt::Debug for DeliveryOptimizationSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeliveryOptimizationSettings").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DeliveryOptimizationSettings {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Store.Preview.DeliveryOptimizationSettings;{1810fda0-e853-565e-b874-7a8a7b9a0e0f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for DeliveryOptimizationSettings {
    type Vtable = IDeliveryOptimizationSettings_Vtbl;
    const IID: ::windows::core::GUID = <IDeliveryOptimizationSettings as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DeliveryOptimizationSettings {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.DeliveryOptimizationSettings";
}
impl ::core::convert::From<DeliveryOptimizationSettings> for ::windows::core::IUnknown {
    fn from(value: DeliveryOptimizationSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DeliveryOptimizationSettings> for ::windows::core::IUnknown {
    fn from(value: &DeliveryOptimizationSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DeliveryOptimizationSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DeliveryOptimizationSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DeliveryOptimizationSettings> for ::windows::core::IInspectable {
    fn from(value: DeliveryOptimizationSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DeliveryOptimizationSettings> for ::windows::core::IInspectable {
    fn from(value: &DeliveryOptimizationSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DeliveryOptimizationSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DeliveryOptimizationSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DeliveryOptimizationSettings {}
unsafe impl ::core::marker::Sync for DeliveryOptimizationSettings {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDeliveryOptimizationSettings(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDeliveryOptimizationSettings {
    type Vtable = IDeliveryOptimizationSettings_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1810fda0_e853_565e_b874_7a8a7b9a0e0f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeliveryOptimizationSettings_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub DownloadMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DeliveryOptimizationDownloadMode) -> ::windows::core::HRESULT,
    pub DownloadModeSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DeliveryOptimizationDownloadModeSource) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDeliveryOptimizationSettingsStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDeliveryOptimizationSettingsStatics {
    type Vtable = IDeliveryOptimizationSettingsStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5c817caf_aed5_5999_b4c9_8c60898bc4f3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeliveryOptimizationSettingsStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetCurrentSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreConfigurationStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStoreConfigurationStatics {
    type Vtable = IStoreConfigurationStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x728f7fc0_8628_42ec_84a2_07780eb44d8b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreConfigurationStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub SetSystemConfiguration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cataloghardwaremanufacturerid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, catalogstorecontentmodifierid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, systemconfigurationexpiration: super::super::super::Foundation::DateTime, cataloghardwaredescriptor: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetSystemConfiguration: usize,
    pub SetMobileOperatorConfiguration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mobileoperatorid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, appdownloadlimitinmegabytes: u32, updatedownloadlimitinmegabytes: u32) -> ::windows::core::HRESULT,
    pub SetStoreWebAccountId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, webaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub IsStoreWebAccountId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, webaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT,
    pub HardwareManufacturerInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub FilterUnsupportedSystemFeaturesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, systemfeatures: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FilterUnsupportedSystemFeaturesAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreConfigurationStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStoreConfigurationStatics2 {
    type Vtable = IStoreConfigurationStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x657c4595_c8b7_4fe9_9f4c_4d71027d347e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreConfigurationStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub PurchasePromptingPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PurchasePromptingPolicy: usize,
    #[cfg(feature = "Foundation")]
    pub SetPurchasePromptingPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPurchasePromptingPolicy: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreConfigurationStatics3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStoreConfigurationStatics3 {
    type Vtable = IStoreConfigurationStatics3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6d45f57c_f144_4cb5_9d3f_4eb05e30b6d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreConfigurationStatics3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub HasStoreWebAccount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")]
    pub HasStoreWebAccountForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    HasStoreWebAccountForUser: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub GetStoreLogDataAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: StoreLogOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    GetStoreLogDataAsync: usize,
    #[cfg(feature = "System")]
    pub SetStoreWebAccountIdForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, webaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetStoreWebAccountIdForUser: usize,
    #[cfg(feature = "System")]
    pub IsStoreWebAccountIdForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, webaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    IsStoreWebAccountIdForUser: usize,
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub GetPurchasePromptingPolicyForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))]
    GetPurchasePromptingPolicyForUser: usize,
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub SetPurchasePromptingPolicyForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))]
    SetPurchasePromptingPolicyForUser: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreConfigurationStatics4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStoreConfigurationStatics4 {
    type Vtable = IStoreConfigurationStatics4_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x20ff56d2_4ee3_4cf0_9b12_552c03310f75);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreConfigurationStatics4_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetStoreWebAccountId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")]
    pub GetStoreWebAccountIdForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetStoreWebAccountIdForUser: usize,
    pub SetEnterpriseStoreWebAccountId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, webaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")]
    pub SetEnterpriseStoreWebAccountIdForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, webaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetEnterpriseStoreWebAccountIdForUser: usize,
    pub GetEnterpriseStoreWebAccountId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")]
    pub GetEnterpriseStoreWebAccountIdForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetEnterpriseStoreWebAccountIdForUser: usize,
    pub ShouldRestrictToEnterpriseStoreOnly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")]
    pub ShouldRestrictToEnterpriseStoreOnlyForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    ShouldRestrictToEnterpriseStoreOnlyForUser: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreConfigurationStatics5(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStoreConfigurationStatics5 {
    type Vtable = IStoreConfigurationStatics5_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf7613191_8fa9_49db_822b_0160e7e4e5c5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreConfigurationStatics5_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsPinToDesktopSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsPinToTaskbarSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsPinToStartSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub PinToDesktop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, apppackagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")]
    pub PinToDesktopForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, apppackagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    PinToDesktopForUser: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreHardwareManufacturerInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStoreHardwareManufacturerInfo {
    type Vtable = IStoreHardwareManufacturerInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf292dc08_c654_43ac_a21f_34801c9d3388);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreHardwareManufacturerInfo_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub HardwareManufacturerId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub StoreContentModifierId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ModelName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ManufacturerName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorePreview(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStorePreview {
    type Vtable = IStorePreview_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8a157241_840e_49a9_bc01_5d5b01fbc8e9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorePreview_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub RequestProductPurchaseByProductIdAndSkuIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, skuid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestProductPurchaseByProductIdAndSkuIdAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub LoadAddOnProductInfosAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    LoadAddOnProductInfosAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorePreviewProductInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStorePreviewProductInfo {
    type Vtable = IStorePreviewProductInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1937dbb3_6c01_4c9d_85cd_5babaac2b351);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorePreviewProductInfo_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ProductId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ProductType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SkuInfoList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SkuInfoList: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorePreviewPurchaseResults(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStorePreviewPurchaseResults {
    type Vtable = IStorePreviewPurchaseResults_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb0daaed1_d6c5_4e53_a043_fba0d8e61231);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorePreviewPurchaseResults_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ProductPurchaseStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut StorePreviewProductPurchaseStatus) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorePreviewSkuInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStorePreviewSkuInfo {
    type Vtable = IStorePreviewSkuInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x81fd76e2_0b26_48d9_98ce_27461c669d6c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorePreviewSkuInfo_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ProductId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SkuId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SkuType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub CustomDeveloperData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub CurrencyCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub FormattedListPrice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ExtendedData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebAuthenticationCoreManagerHelper(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebAuthenticationCoreManagerHelper {
    type Vtable = IWebAuthenticationCoreManagerHelper_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x06a50525_e715_4123_9276_9d6f865ba55f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAuthenticationCoreManagerHelper_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "Security_Authentication_Web_Core", feature = "UI_Xaml"))]
    pub RequestTokenWithUIElementHostingAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, request: ::windows::core::RawPtr, uielement: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Authentication_Web_Core", feature = "UI_Xaml")))]
    RequestTokenWithUIElementHostingAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Authentication_Web_Core", feature = "Security_Credentials", feature = "UI_Xaml"))]
    pub RequestTokenWithUIElementHostingAndWebAccountAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, request: ::windows::core::RawPtr, webaccount: ::windows::core::RawPtr, uielement: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Authentication_Web_Core", feature = "Security_Credentials", feature = "UI_Xaml")))]
    RequestTokenWithUIElementHostingAndWebAccountAsync: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Store_Preview\"`*"]
pub struct StoreConfiguration {}
impl StoreConfiguration {
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetSystemConfiguration<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::DateTime>, Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(cataloghardwaremanufacturerid: Param0, catalogstorecontentmodifierid: Param1, systemconfigurationexpiration: Param2, cataloghardwaredescriptor: Param3) -> ::windows::core::Result<()> {
        Self::IStoreConfigurationStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetSystemConfiguration)(::core::mem::transmute_copy(this), cataloghardwaremanufacturerid.into_param().abi(), catalogstorecontentmodifierid.into_param().abi(), systemconfigurationexpiration.into_param().abi(), cataloghardwaredescriptor.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview\"`*"]
    pub fn SetMobileOperatorConfiguration<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(mobileoperatorid: Param0, appdownloadlimitinmegabytes: u32, updatedownloadlimitinmegabytes: u32) -> ::windows::core::Result<()> {
        Self::IStoreConfigurationStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetMobileOperatorConfiguration)(::core::mem::transmute_copy(this), mobileoperatorid.into_param().abi(), appdownloadlimitinmegabytes, updatedownloadlimitinmegabytes).ok() })
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview\"`*"]
    pub fn SetStoreWebAccountId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(webaccountid: Param0) -> ::windows::core::Result<()> {
        Self::IStoreConfigurationStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetStoreWebAccountId)(::core::mem::transmute_copy(this), webaccountid.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview\"`*"]
    pub fn IsStoreWebAccountId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(webaccountid: Param0) -> ::windows::core::Result<bool> {
        Self::IStoreConfigurationStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsStoreWebAccountId)(::core::mem::transmute_copy(this), webaccountid.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview\"`*"]
    pub fn HardwareManufacturerInfo() -> ::windows::core::Result<StoreHardwareManufacturerInfo> {
        Self::IStoreConfigurationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HardwareManufacturerInfo)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StoreHardwareManufacturerInfo>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FilterUnsupportedSystemFeaturesAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<StoreSystemFeature>>>(systemfeatures: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<StoreSystemFeature>>> {
        Self::IStoreConfigurationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FilterUnsupportedSystemFeaturesAsync)(::core::mem::transmute_copy(this), systemfeatures.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<StoreSystemFeature>>>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PurchasePromptingPolicy() -> ::windows::core::Result<super::super::super::Foundation::IReference<u32>> {
        Self::IStoreConfigurationStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PurchasePromptingPolicy)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<u32>>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetPurchasePromptingPolicy<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::IReference<u32>>>(value: Param0) -> ::windows::core::Result<()> {
        Self::IStoreConfigurationStatics2(|this| unsafe { (::windows::core::Interface::vtable(this).SetPurchasePromptingPolicy)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview\"`*"]
    pub fn HasStoreWebAccount() -> ::windows::core::Result<bool> {
        Self::IStoreConfigurationStatics3(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HasStoreWebAccount)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn HasStoreWebAccountForUser<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::User>>(user: Param0) -> ::windows::core::Result<bool> {
        Self::IStoreConfigurationStatics3(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HasStoreWebAccountForUser)(::core::mem::transmute_copy(this), user.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview\"`, `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn GetStoreLogDataAsync(options: StoreLogOptions) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::Streams::IRandomAccessStreamReference>> {
        Self::IStoreConfigurationStatics3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetStoreLogDataAsync)(::core::mem::transmute_copy(this), options, &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::Streams::IRandomAccessStreamReference>>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn SetStoreWebAccountIdForUser<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::User>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(user: Param0, webaccountid: Param1) -> ::windows::core::Result<()> {
        Self::IStoreConfigurationStatics3(|this| unsafe { (::windows::core::Interface::vtable(this).SetStoreWebAccountIdForUser)(::core::mem::transmute_copy(this), user.into_param().abi(), webaccountid.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn IsStoreWebAccountIdForUser<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::User>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(user: Param0, webaccountid: Param1) -> ::windows::core::Result<bool> {
        Self::IStoreConfigurationStatics3(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsStoreWebAccountIdForUser)(::core::mem::transmute_copy(this), user.into_param().abi(), webaccountid.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview\"`, `\"Foundation\"`, `\"System\"`*"]
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn GetPurchasePromptingPolicyForUser<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::User>>(user: Param0) -> ::windows::core::Result<super::super::super::Foundation::IReference<u32>> {
        Self::IStoreConfigurationStatics3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetPurchasePromptingPolicyForUser)(::core::mem::transmute_copy(this), user.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IReference<u32>>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview\"`, `\"Foundation\"`, `\"System\"`*"]
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn SetPurchasePromptingPolicyForUser<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::User>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::IReference<u32>>>(user: Param0, value: Param1) -> ::windows::core::Result<()> {
        Self::IStoreConfigurationStatics3(|this| unsafe { (::windows::core::Interface::vtable(this).SetPurchasePromptingPolicyForUser)(::core::mem::transmute_copy(this), user.into_param().abi(), value.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview\"`*"]
    pub fn GetStoreWebAccountId() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IStoreConfigurationStatics4(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetStoreWebAccountId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn GetStoreWebAccountIdForUser<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::User>>(user: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IStoreConfigurationStatics4(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetStoreWebAccountIdForUser)(::core::mem::transmute_copy(this), user.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview\"`*"]
    pub fn SetEnterpriseStoreWebAccountId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(webaccountid: Param0) -> ::windows::core::Result<()> {
        Self::IStoreConfigurationStatics4(|this| unsafe { (::windows::core::Interface::vtable(this).SetEnterpriseStoreWebAccountId)(::core::mem::transmute_copy(this), webaccountid.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn SetEnterpriseStoreWebAccountIdForUser<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::User>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(user: Param0, webaccountid: Param1) -> ::windows::core::Result<()> {
        Self::IStoreConfigurationStatics4(|this| unsafe { (::windows::core::Interface::vtable(this).SetEnterpriseStoreWebAccountIdForUser)(::core::mem::transmute_copy(this), user.into_param().abi(), webaccountid.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview\"`*"]
    pub fn GetEnterpriseStoreWebAccountId() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IStoreConfigurationStatics4(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetEnterpriseStoreWebAccountId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn GetEnterpriseStoreWebAccountIdForUser<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::User>>(user: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IStoreConfigurationStatics4(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetEnterpriseStoreWebAccountIdForUser)(::core::mem::transmute_copy(this), user.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview\"`*"]
    pub fn ShouldRestrictToEnterpriseStoreOnly() -> ::windows::core::Result<bool> {
        Self::IStoreConfigurationStatics4(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ShouldRestrictToEnterpriseStoreOnly)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn ShouldRestrictToEnterpriseStoreOnlyForUser<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::User>>(user: Param0) -> ::windows::core::Result<bool> {
        Self::IStoreConfigurationStatics4(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ShouldRestrictToEnterpriseStoreOnlyForUser)(::core::mem::transmute_copy(this), user.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview\"`*"]
    pub fn IsPinToDesktopSupported() -> ::windows::core::Result<bool> {
        Self::IStoreConfigurationStatics5(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsPinToDesktopSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview\"`*"]
    pub fn IsPinToTaskbarSupported() -> ::windows::core::Result<bool> {
        Self::IStoreConfigurationStatics5(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsPinToTaskbarSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview\"`*"]
    pub fn IsPinToStartSupported() -> ::windows::core::Result<bool> {
        Self::IStoreConfigurationStatics5(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsPinToStartSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview\"`*"]
    pub fn PinToDesktop<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(apppackagefamilyname: Param0) -> ::windows::core::Result<()> {
        Self::IStoreConfigurationStatics5(|this| unsafe { (::windows::core::Interface::vtable(this).PinToDesktop)(::core::mem::transmute_copy(this), apppackagefamilyname.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn PinToDesktopForUser<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::User>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(user: Param0, apppackagefamilyname: Param1) -> ::windows::core::Result<()> {
        Self::IStoreConfigurationStatics5(|this| unsafe { (::windows::core::Interface::vtable(this).PinToDesktopForUser)(::core::mem::transmute_copy(this), user.into_param().abi(), apppackagefamilyname.into_param().abi()).ok() })
    }
    #[doc(hidden)]
    pub fn IStoreConfigurationStatics<R, F: FnOnce(&IStoreConfigurationStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<StoreConfiguration, IStoreConfigurationStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IStoreConfigurationStatics2<R, F: FnOnce(&IStoreConfigurationStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<StoreConfiguration, IStoreConfigurationStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IStoreConfigurationStatics3<R, F: FnOnce(&IStoreConfigurationStatics3) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<StoreConfiguration, IStoreConfigurationStatics3> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IStoreConfigurationStatics4<R, F: FnOnce(&IStoreConfigurationStatics4) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<StoreConfiguration, IStoreConfigurationStatics4> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IStoreConfigurationStatics5<R, F: FnOnce(&IStoreConfigurationStatics5) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<StoreConfiguration, IStoreConfigurationStatics5> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for StoreConfiguration {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.StoreConfiguration";
}
#[doc = "*Required features: `\"ApplicationModel_Store_Preview\"`*"]
#[repr(transparent)]
pub struct StoreHardwareManufacturerInfo(::windows::core::IUnknown);
impl StoreHardwareManufacturerInfo {
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview\"`*"]
    pub fn HardwareManufacturerId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HardwareManufacturerId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview\"`*"]
    pub fn StoreContentModifierId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StoreContentModifierId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview\"`*"]
    pub fn ModelName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ModelName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview\"`*"]
    pub fn ManufacturerName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ManufacturerName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for StoreHardwareManufacturerInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StoreHardwareManufacturerInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StoreHardwareManufacturerInfo {}
impl ::core::fmt::Debug for StoreHardwareManufacturerInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StoreHardwareManufacturerInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StoreHardwareManufacturerInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Store.Preview.StoreHardwareManufacturerInfo;{f292dc08-c654-43ac-a21f-34801c9d3388})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for StoreHardwareManufacturerInfo {
    type Vtable = IStoreHardwareManufacturerInfo_Vtbl;
    const IID: ::windows::core::GUID = <IStoreHardwareManufacturerInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StoreHardwareManufacturerInfo {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.StoreHardwareManufacturerInfo";
}
impl ::core::convert::From<StoreHardwareManufacturerInfo> for ::windows::core::IUnknown {
    fn from(value: StoreHardwareManufacturerInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreHardwareManufacturerInfo> for ::windows::core::IUnknown {
    fn from(value: &StoreHardwareManufacturerInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StoreHardwareManufacturerInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StoreHardwareManufacturerInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StoreHardwareManufacturerInfo> for ::windows::core::IInspectable {
    fn from(value: StoreHardwareManufacturerInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StoreHardwareManufacturerInfo> for ::windows::core::IInspectable {
    fn from(value: &StoreHardwareManufacturerInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StoreHardwareManufacturerInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StoreHardwareManufacturerInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StoreHardwareManufacturerInfo {}
unsafe impl ::core::marker::Sync for StoreHardwareManufacturerInfo {}
#[doc = "*Required features: `\"ApplicationModel_Store_Preview\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct StoreLogOptions(pub u32);
impl StoreLogOptions {
    pub const None: Self = Self(0u32);
    pub const TryElevate: Self = Self(1u32);
}
impl ::core::marker::Copy for StoreLogOptions {}
impl ::core::clone::Clone for StoreLogOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for StoreLogOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for StoreLogOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for StoreLogOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StoreLogOptions").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for StoreLogOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for StoreLogOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for StoreLogOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for StoreLogOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for StoreLogOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for StoreLogOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Store.Preview.StoreLogOptions;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Store_Preview\"`*"]
pub struct StorePreview {}
impl StorePreview {
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestProductPurchaseByProductIdAndSkuIdAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(productid: Param0, skuid: Param1) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<StorePreviewPurchaseResults>> {
        Self::IStorePreview(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestProductPurchaseByProductIdAndSkuIdAsync)(::core::mem::transmute_copy(this), productid.into_param().abi(), skuid.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<StorePreviewPurchaseResults>>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn LoadAddOnProductInfosAsync() -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<StorePreviewProductInfo>>> {
        Self::IStorePreview(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LoadAddOnProductInfosAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<StorePreviewProductInfo>>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IStorePreview<R, F: FnOnce(&IStorePreview) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<StorePreview, IStorePreview> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for StorePreview {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.StorePreview";
}
#[doc = "*Required features: `\"ApplicationModel_Store_Preview\"`*"]
#[repr(transparent)]
pub struct StorePreviewProductInfo(::windows::core::IUnknown);
impl StorePreviewProductInfo {
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview\"`*"]
    pub fn ProductId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ProductId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview\"`*"]
    pub fn ProductType(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ProductType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview\"`*"]
    pub fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Title)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview\"`*"]
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Description)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SkuInfoList(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<StorePreviewSkuInfo>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SkuInfoList)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<StorePreviewSkuInfo>>(result__)
        }
    }
}
impl ::core::clone::Clone for StorePreviewProductInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StorePreviewProductInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StorePreviewProductInfo {}
impl ::core::fmt::Debug for StorePreviewProductInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorePreviewProductInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StorePreviewProductInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Store.Preview.StorePreviewProductInfo;{1937dbb3-6c01-4c9d-85cd-5babaac2b351})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for StorePreviewProductInfo {
    type Vtable = IStorePreviewProductInfo_Vtbl;
    const IID: ::windows::core::GUID = <IStorePreviewProductInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StorePreviewProductInfo {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.StorePreviewProductInfo";
}
impl ::core::convert::From<StorePreviewProductInfo> for ::windows::core::IUnknown {
    fn from(value: StorePreviewProductInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorePreviewProductInfo> for ::windows::core::IUnknown {
    fn from(value: &StorePreviewProductInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StorePreviewProductInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StorePreviewProductInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StorePreviewProductInfo> for ::windows::core::IInspectable {
    fn from(value: StorePreviewProductInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorePreviewProductInfo> for ::windows::core::IInspectable {
    fn from(value: &StorePreviewProductInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StorePreviewProductInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StorePreviewProductInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StorePreviewProductInfo {}
unsafe impl ::core::marker::Sync for StorePreviewProductInfo {}
#[doc = "*Required features: `\"ApplicationModel_Store_Preview\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct StorePreviewProductPurchaseStatus(pub i32);
impl StorePreviewProductPurchaseStatus {
    pub const Succeeded: Self = Self(0i32);
    pub const AlreadyPurchased: Self = Self(1i32);
    pub const NotFulfilled: Self = Self(2i32);
    pub const NotPurchased: Self = Self(3i32);
}
impl ::core::marker::Copy for StorePreviewProductPurchaseStatus {}
impl ::core::clone::Clone for StorePreviewProductPurchaseStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for StorePreviewProductPurchaseStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for StorePreviewProductPurchaseStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for StorePreviewProductPurchaseStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorePreviewProductPurchaseStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StorePreviewProductPurchaseStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Store.Preview.StorePreviewProductPurchaseStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Store_Preview\"`*"]
#[repr(transparent)]
pub struct StorePreviewPurchaseResults(::windows::core::IUnknown);
impl StorePreviewPurchaseResults {
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview\"`*"]
    pub fn ProductPurchaseStatus(&self) -> ::windows::core::Result<StorePreviewProductPurchaseStatus> {
        let this = self;
        unsafe {
            let mut result__: StorePreviewProductPurchaseStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ProductPurchaseStatus)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StorePreviewProductPurchaseStatus>(result__)
        }
    }
}
impl ::core::clone::Clone for StorePreviewPurchaseResults {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StorePreviewPurchaseResults {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StorePreviewPurchaseResults {}
impl ::core::fmt::Debug for StorePreviewPurchaseResults {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorePreviewPurchaseResults").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StorePreviewPurchaseResults {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Store.Preview.StorePreviewPurchaseResults;{b0daaed1-d6c5-4e53-a043-fba0d8e61231})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for StorePreviewPurchaseResults {
    type Vtable = IStorePreviewPurchaseResults_Vtbl;
    const IID: ::windows::core::GUID = <IStorePreviewPurchaseResults as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StorePreviewPurchaseResults {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.StorePreviewPurchaseResults";
}
impl ::core::convert::From<StorePreviewPurchaseResults> for ::windows::core::IUnknown {
    fn from(value: StorePreviewPurchaseResults) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorePreviewPurchaseResults> for ::windows::core::IUnknown {
    fn from(value: &StorePreviewPurchaseResults) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StorePreviewPurchaseResults {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StorePreviewPurchaseResults {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StorePreviewPurchaseResults> for ::windows::core::IInspectable {
    fn from(value: StorePreviewPurchaseResults) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorePreviewPurchaseResults> for ::windows::core::IInspectable {
    fn from(value: &StorePreviewPurchaseResults) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StorePreviewPurchaseResults {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StorePreviewPurchaseResults {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StorePreviewPurchaseResults {}
unsafe impl ::core::marker::Sync for StorePreviewPurchaseResults {}
#[doc = "*Required features: `\"ApplicationModel_Store_Preview\"`*"]
#[repr(transparent)]
pub struct StorePreviewSkuInfo(::windows::core::IUnknown);
impl StorePreviewSkuInfo {
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview\"`*"]
    pub fn ProductId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ProductId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview\"`*"]
    pub fn SkuId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SkuId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview\"`*"]
    pub fn SkuType(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SkuType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview\"`*"]
    pub fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Title)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview\"`*"]
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Description)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview\"`*"]
    pub fn CustomDeveloperData(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CustomDeveloperData)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview\"`*"]
    pub fn CurrencyCode(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CurrencyCode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview\"`*"]
    pub fn FormattedListPrice(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FormattedListPrice)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview\"`*"]
    pub fn ExtendedData(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExtendedData)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for StorePreviewSkuInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StorePreviewSkuInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StorePreviewSkuInfo {}
impl ::core::fmt::Debug for StorePreviewSkuInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorePreviewSkuInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StorePreviewSkuInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Store.Preview.StorePreviewSkuInfo;{81fd76e2-0b26-48d9-98ce-27461c669d6c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for StorePreviewSkuInfo {
    type Vtable = IStorePreviewSkuInfo_Vtbl;
    const IID: ::windows::core::GUID = <IStorePreviewSkuInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StorePreviewSkuInfo {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.StorePreviewSkuInfo";
}
impl ::core::convert::From<StorePreviewSkuInfo> for ::windows::core::IUnknown {
    fn from(value: StorePreviewSkuInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorePreviewSkuInfo> for ::windows::core::IUnknown {
    fn from(value: &StorePreviewSkuInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StorePreviewSkuInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StorePreviewSkuInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StorePreviewSkuInfo> for ::windows::core::IInspectable {
    fn from(value: StorePreviewSkuInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorePreviewSkuInfo> for ::windows::core::IInspectable {
    fn from(value: &StorePreviewSkuInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StorePreviewSkuInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StorePreviewSkuInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StorePreviewSkuInfo {}
unsafe impl ::core::marker::Sync for StorePreviewSkuInfo {}
#[doc = "*Required features: `\"ApplicationModel_Store_Preview\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct StoreSystemFeature(pub i32);
impl StoreSystemFeature {
    pub const ArchitectureX86: Self = Self(0i32);
    pub const ArchitectureX64: Self = Self(1i32);
    pub const ArchitectureArm: Self = Self(2i32);
    pub const DirectX9: Self = Self(3i32);
    pub const DirectX10: Self = Self(4i32);
    pub const DirectX11: Self = Self(5i32);
    pub const D3D12HardwareFL11: Self = Self(6i32);
    pub const D3D12HardwareFL12: Self = Self(7i32);
    pub const Memory300MB: Self = Self(8i32);
    pub const Memory750MB: Self = Self(9i32);
    pub const Memory1GB: Self = Self(10i32);
    pub const Memory2GB: Self = Self(11i32);
    pub const CameraFront: Self = Self(12i32);
    pub const CameraRear: Self = Self(13i32);
    pub const Gyroscope: Self = Self(14i32);
    pub const Hover: Self = Self(15i32);
    pub const Magnetometer: Self = Self(16i32);
    pub const Nfc: Self = Self(17i32);
    pub const Resolution720P: Self = Self(18i32);
    pub const ResolutionWvga: Self = Self(19i32);
    pub const ResolutionWvgaOr720P: Self = Self(20i32);
    pub const ResolutionWxga: Self = Self(21i32);
    pub const ResolutionWvgaOrWxga: Self = Self(22i32);
    pub const ResolutionWxgaOr720P: Self = Self(23i32);
    pub const Memory4GB: Self = Self(24i32);
    pub const Memory6GB: Self = Self(25i32);
    pub const Memory8GB: Self = Self(26i32);
    pub const Memory12GB: Self = Self(27i32);
    pub const Memory16GB: Self = Self(28i32);
    pub const Memory20GB: Self = Self(29i32);
    pub const VideoMemory2GB: Self = Self(30i32);
    pub const VideoMemory4GB: Self = Self(31i32);
    pub const VideoMemory6GB: Self = Self(32i32);
    pub const VideoMemory1GB: Self = Self(33i32);
    pub const ArchitectureArm64: Self = Self(34i32);
}
impl ::core::marker::Copy for StoreSystemFeature {}
impl ::core::clone::Clone for StoreSystemFeature {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for StoreSystemFeature {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for StoreSystemFeature {
    type Abi = Self;
}
impl ::core::fmt::Debug for StoreSystemFeature {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StoreSystemFeature").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StoreSystemFeature {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Store.Preview.StoreSystemFeature;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Store_Preview\"`*"]
pub struct WebAuthenticationCoreManagerHelper {}
impl WebAuthenticationCoreManagerHelper {
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview\"`, `\"Foundation\"`, `\"Security_Authentication_Web_Core\"`, `\"UI_Xaml\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Security_Authentication_Web_Core", feature = "UI_Xaml"))]
    pub fn RequestTokenWithUIElementHostingAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Security::Authentication::Web::Core::WebTokenRequest>, Param1: ::windows::core::IntoParam<'a, super::super::super::UI::Xaml::UIElement>>(request: Param0, uielement: Param1) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Security::Authentication::Web::Core::WebTokenRequestResult>> {
        Self::IWebAuthenticationCoreManagerHelper(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestTokenWithUIElementHostingAsync)(::core::mem::transmute_copy(this), request.into_param().abi(), uielement.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<super::super::super::Security::Authentication::Web::Core::WebTokenRequestResult>>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Store_Preview\"`, `\"Foundation\"`, `\"Security_Authentication_Web_Core\"`, `\"Security_Credentials\"`, `\"UI_Xaml\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Security_Authentication_Web_Core", feature = "Security_Credentials", feature = "UI_Xaml"))]
    pub fn RequestTokenWithUIElementHostingAndWebAccountAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Security::Authentication::Web::Core::WebTokenRequest>, Param1: ::windows::core::IntoParam<'a, super::super::super::Security::Credentials::WebAccount>, Param2: ::windows::core::IntoParam<'a, super::super::super::UI::Xaml::UIElement>>(request: Param0, webaccount: Param1, uielement: Param2) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Security::Authentication::Web::Core::WebTokenRequestResult>> {
        Self::IWebAuthenticationCoreManagerHelper(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestTokenWithUIElementHostingAndWebAccountAsync)(::core::mem::transmute_copy(this), request.into_param().abi(), webaccount.into_param().abi(), uielement.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<super::super::super::Security::Authentication::Web::Core::WebTokenRequestResult>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IWebAuthenticationCoreManagerHelper<R, F: FnOnce(&IWebAuthenticationCoreManagerHelper) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<WebAuthenticationCoreManagerHelper, IWebAuthenticationCoreManagerHelper> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for WebAuthenticationCoreManagerHelper {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.WebAuthenticationCoreManagerHelper";
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
