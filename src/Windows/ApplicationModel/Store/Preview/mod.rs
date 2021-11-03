#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "ApplicationModel_Store_Preview_InstallControl")]
pub mod InstallControl;
#[doc = "*Required features: `ApplicationModel_Store_Preview`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DeliveryOptimizationDownloadMode(pub i32);
impl DeliveryOptimizationDownloadMode {
    pub const Simple: DeliveryOptimizationDownloadMode = DeliveryOptimizationDownloadMode(0i32);
    pub const HttpOnly: DeliveryOptimizationDownloadMode = DeliveryOptimizationDownloadMode(1i32);
    pub const Lan: DeliveryOptimizationDownloadMode = DeliveryOptimizationDownloadMode(2i32);
    pub const Group: DeliveryOptimizationDownloadMode = DeliveryOptimizationDownloadMode(3i32);
    pub const Internet: DeliveryOptimizationDownloadMode = DeliveryOptimizationDownloadMode(4i32);
    pub const Bypass: DeliveryOptimizationDownloadMode = DeliveryOptimizationDownloadMode(5i32);
}
impl ::std::convert::From<i32> for DeliveryOptimizationDownloadMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DeliveryOptimizationDownloadMode {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for DeliveryOptimizationDownloadMode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Store.Preview.DeliveryOptimizationDownloadMode;i4)");
}
#[doc = "*Required features: `ApplicationModel_Store_Preview`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DeliveryOptimizationDownloadModeSource(pub i32);
impl DeliveryOptimizationDownloadModeSource {
    pub const Default: DeliveryOptimizationDownloadModeSource = DeliveryOptimizationDownloadModeSource(0i32);
    pub const Policy: DeliveryOptimizationDownloadModeSource = DeliveryOptimizationDownloadModeSource(1i32);
}
impl ::std::convert::From<i32> for DeliveryOptimizationDownloadModeSource {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DeliveryOptimizationDownloadModeSource {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for DeliveryOptimizationDownloadModeSource {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Store.Preview.DeliveryOptimizationDownloadModeSource;i4)");
}
#[doc = "*Required features: `ApplicationModel_Store_Preview`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct DeliveryOptimizationSettings(::windows::runtime::IInspectable);
impl DeliveryOptimizationSettings {
    #[doc = "*Required features: `ApplicationModel_Store_Preview`*"]
    pub fn DownloadMode(&self) -> ::windows::runtime::Result<DeliveryOptimizationDownloadMode> {
        let this = self;
        unsafe {
            let mut result__: DeliveryOptimizationDownloadMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<DeliveryOptimizationDownloadMode>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Store_Preview`*"]
    pub fn DownloadModeSource(&self) -> ::windows::runtime::Result<DeliveryOptimizationDownloadModeSource> {
        let this = self;
        unsafe {
            let mut result__: DeliveryOptimizationDownloadModeSource = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<DeliveryOptimizationDownloadModeSource>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Store_Preview`*"]
    pub fn GetCurrentSettings() -> ::windows::runtime::Result<DeliveryOptimizationSettings> {
        Self::IDeliveryOptimizationSettingsStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<DeliveryOptimizationSettings>(result__)
        })
    }
    pub fn IDeliveryOptimizationSettingsStatics<R, F: FnOnce(&IDeliveryOptimizationSettingsStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<DeliveryOptimizationSettings, IDeliveryOptimizationSettingsStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DeliveryOptimizationSettings {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Store.Preview.DeliveryOptimizationSettings;{1810fda0-e853-565e-b874-7a8a7b9a0e0f})");
}
unsafe impl ::windows::runtime::Interface for DeliveryOptimizationSettings {
    type Vtable = IDeliveryOptimizationSettings_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(403766688, 59475, 22110, [184, 116, 122, 138, 123, 154, 14, 15]);
}
impl ::windows::runtime::RuntimeName for DeliveryOptimizationSettings {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.DeliveryOptimizationSettings";
}
unsafe impl ::std::marker::Send for DeliveryOptimizationSettings {}
unsafe impl ::std::marker::Sync for DeliveryOptimizationSettings {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IDeliveryOptimizationSettings(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDeliveryOptimizationSettings {
    type Vtable = IDeliveryOptimizationSettings_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(403766688, 59475, 22110, [184, 116, 122, 138, 123, 154, 14, 15]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeliveryOptimizationSettings_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut DeliveryOptimizationDownloadMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut DeliveryOptimizationDownloadModeSource) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IDeliveryOptimizationSettingsStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDeliveryOptimizationSettingsStatics {
    type Vtable = IDeliveryOptimizationSettingsStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1551989935, 44757, 22937, [180, 201, 140, 96, 137, 139, 196, 243]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeliveryOptimizationSettingsStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IStoreConfigurationStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IStoreConfigurationStatics {
    type Vtable = IStoreConfigurationStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1922006976, 34344, 17132, [132, 162, 7, 120, 14, 180, 77, 139]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreConfigurationStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cataloghardwaremanufacturerid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, catalogstorecontentmodifierid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, systemconfigurationexpiration: super::super::super::Foundation::DateTime, cataloghardwaredescriptor: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mobileoperatorid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, appdownloadlimitinmegabytes: u32, updatedownloadlimitinmegabytes: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, webaccountid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, webaccountid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, systemfeatures: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IStoreConfigurationStatics2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IStoreConfigurationStatics2 {
    type Vtable = IStoreConfigurationStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1702643093, 51383, 20457, [159, 76, 77, 113, 2, 125, 52, 126]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreConfigurationStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IStoreConfigurationStatics3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IStoreConfigurationStatics3 {
    type Vtable = IStoreConfigurationStatics3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1833301372, 61764, 19637, [157, 63, 78, 176, 94, 48, 182, 211]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreConfigurationStatics3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, user: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, options: StoreLogOptions, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, user: ::windows::runtime::RawPtr, webaccountid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, user: ::windows::runtime::RawPtr, webaccountid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    #[cfg(all(feature = "Foundation", feature = "System"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, user: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))] usize,
    #[cfg(all(feature = "Foundation", feature = "System"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, user: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IStoreConfigurationStatics4(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IStoreConfigurationStatics4 {
    type Vtable = IStoreConfigurationStatics4_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(553604818, 20195, 19696, [155, 18, 85, 44, 3, 49, 15, 117]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreConfigurationStatics4_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, user: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, webaccountid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, user: ::windows::runtime::RawPtr, webaccountid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, user: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, user: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "System"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IStoreConfigurationStatics5(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IStoreConfigurationStatics5 {
    type Vtable = IStoreConfigurationStatics5_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4150342033, 36777, 18907, [130, 43, 1, 96, 231, 228, 229, 197]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreConfigurationStatics5_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, apppackagefamilyname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, user: ::windows::runtime::RawPtr, apppackagefamilyname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "System"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IStoreHardwareManufacturerInfo(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IStoreHardwareManufacturerInfo {
    type Vtable = IStoreHardwareManufacturerInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4069710856, 50772, 17324, [162, 31, 52, 128, 28, 157, 51, 136]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreHardwareManufacturerInfo_abi(
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
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IStorePreview(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IStorePreview {
    type Vtable = IStorePreview_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2316661313, 33806, 18857, [188, 1, 93, 91, 1, 251, 200, 233]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorePreview_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, productid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, skuid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IStorePreviewProductInfo(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IStorePreviewProductInfo {
    type Vtable = IStorePreviewProductInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(423091123, 27649, 19613, [133, 205, 91, 171, 170, 194, 179, 81]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorePreviewProductInfo_abi(
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
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IStorePreviewPurchaseResults(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IStorePreviewPurchaseResults {
    type Vtable = IStorePreviewPurchaseResults_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2967121617, 54981, 20051, [160, 67, 251, 160, 216, 230, 18, 49]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorePreviewPurchaseResults_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut StorePreviewProductPurchaseStatus) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IStorePreviewSkuInfo(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IStorePreviewSkuInfo {
    type Vtable = IStorePreviewSkuInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2180871906, 2854, 18649, [152, 206, 39, 70, 28, 102, 157, 108]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorePreviewSkuInfo_abi(
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IWebAuthenticationCoreManagerHelper(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWebAuthenticationCoreManagerHelper {
    type Vtable = IWebAuthenticationCoreManagerHelper_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(111478053, 59157, 16675, [146, 118, 157, 111, 134, 91, 165, 95]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAuthenticationCoreManagerHelper_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Security_Authentication_Web_Core", feature = "UI_Xaml"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, request: ::windows::runtime::RawPtr, uielement: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Authentication_Web_Core", feature = "UI_Xaml")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Authentication_Web_Core", feature = "Security_Credentials", feature = "UI_Xaml"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, request: ::windows::runtime::RawPtr, webaccount: ::windows::runtime::RawPtr, uielement: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Authentication_Web_Core", feature = "Security_Credentials", feature = "UI_Xaml")))] usize,
);
#[doc = "*Required features: `ApplicationModel_Store_Preview`*"]
pub struct StoreConfiguration {}
impl StoreConfiguration {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Store_Preview`, `Foundation`*"]
    pub fn SetSystemConfiguration<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::DateTime>, Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        cataloghardwaremanufacturerid: Param0,
        catalogstorecontentmodifierid: Param1,
        systemconfigurationexpiration: Param2,
        cataloghardwaredescriptor: Param3,
    ) -> ::windows::runtime::Result<()> {
        Self::IStoreConfigurationStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), cataloghardwaremanufacturerid.into_param().abi(), catalogstorecontentmodifierid.into_param().abi(), systemconfigurationexpiration.into_param().abi(), cataloghardwaredescriptor.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `ApplicationModel_Store_Preview`*"]
    pub fn SetMobileOperatorConfiguration<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(mobileoperatorid: Param0, appdownloadlimitinmegabytes: u32, updatedownloadlimitinmegabytes: u32) -> ::windows::runtime::Result<()> {
        Self::IStoreConfigurationStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), mobileoperatorid.into_param().abi(), appdownloadlimitinmegabytes, updatedownloadlimitinmegabytes).ok() })
    }
    #[doc = "*Required features: `ApplicationModel_Store_Preview`*"]
    pub fn SetStoreWebAccountId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(webaccountid: Param0) -> ::windows::runtime::Result<()> {
        Self::IStoreConfigurationStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), webaccountid.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `ApplicationModel_Store_Preview`*"]
    pub fn IsStoreWebAccountId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(webaccountid: Param0) -> ::windows::runtime::Result<bool> {
        Self::IStoreConfigurationStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), webaccountid.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_Store_Preview`*"]
    pub fn HardwareManufacturerInfo() -> ::windows::runtime::Result<StoreHardwareManufacturerInfo> {
        Self::IStoreConfigurationStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<StoreHardwareManufacturerInfo>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `ApplicationModel_Store_Preview`, `Foundation`, `Foundation_Collections`*"]
    pub fn FilterUnsupportedSystemFeaturesAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<StoreSystemFeature>>>(systemfeatures: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<StoreSystemFeature>>> {
        Self::IStoreConfigurationStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), systemfeatures.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<StoreSystemFeature>>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Store_Preview`, `Foundation`*"]
    pub fn PurchasePromptingPolicy() -> ::windows::runtime::Result<super::super::super::Foundation::IReference<u32>> {
        Self::IStoreConfigurationStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<u32>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Store_Preview`, `Foundation`*"]
    pub fn SetPurchasePromptingPolicy<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IReference<u32>>>(value: Param0) -> ::windows::runtime::Result<()> {
        Self::IStoreConfigurationStatics2(|this| unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `ApplicationModel_Store_Preview`*"]
    pub fn HasStoreWebAccount() -> ::windows::runtime::Result<bool> {
        Self::IStoreConfigurationStatics3(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `ApplicationModel_Store_Preview`, `System`*"]
    pub fn HasStoreWebAccountForUser<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::System::User>>(user: Param0) -> ::windows::runtime::Result<bool> {
        Self::IStoreConfigurationStatics3(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), user.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `ApplicationModel_Store_Preview`, `Foundation`, `Storage_Streams`*"]
    pub fn GetStoreLogDataAsync(options: StoreLogOptions) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::Streams::IRandomAccessStreamReference>> {
        Self::IStoreConfigurationStatics3(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), options, &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::Streams::IRandomAccessStreamReference>>(result__)
        })
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `ApplicationModel_Store_Preview`, `System`*"]
    pub fn SetStoreWebAccountIdForUser<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::System::User>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(user: Param0, webaccountid: Param1) -> ::windows::runtime::Result<()> {
        Self::IStoreConfigurationStatics3(|this| unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), user.into_param().abi(), webaccountid.into_param().abi()).ok() })
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `ApplicationModel_Store_Preview`, `System`*"]
    pub fn IsStoreWebAccountIdForUser<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::System::User>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(user: Param0, webaccountid: Param1) -> ::windows::runtime::Result<bool> {
        Self::IStoreConfigurationStatics3(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), user.into_param().abi(), webaccountid.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "System"))]
    #[doc = "*Required features: `ApplicationModel_Store_Preview`, `Foundation`, `System`*"]
    pub fn GetPurchasePromptingPolicyForUser<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::System::User>>(user: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<u32>> {
        Self::IStoreConfigurationStatics3(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), user.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IReference<u32>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "System"))]
    #[doc = "*Required features: `ApplicationModel_Store_Preview`, `Foundation`, `System`*"]
    pub fn SetPurchasePromptingPolicyForUser<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::System::User>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IReference<u32>>>(user: Param0, value: Param1) -> ::windows::runtime::Result<()> {
        Self::IStoreConfigurationStatics3(|this| unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), user.into_param().abi(), value.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `ApplicationModel_Store_Preview`*"]
    pub fn GetStoreWebAccountId() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IStoreConfigurationStatics4(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `ApplicationModel_Store_Preview`, `System`*"]
    pub fn GetStoreWebAccountIdForUser<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::System::User>>(user: Param0) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IStoreConfigurationStatics4(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), user.into_param().abi(), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_Store_Preview`*"]
    pub fn SetEnterpriseStoreWebAccountId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(webaccountid: Param0) -> ::windows::runtime::Result<()> {
        Self::IStoreConfigurationStatics4(|this| unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), webaccountid.into_param().abi()).ok() })
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `ApplicationModel_Store_Preview`, `System`*"]
    pub fn SetEnterpriseStoreWebAccountIdForUser<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::System::User>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(user: Param0, webaccountid: Param1) -> ::windows::runtime::Result<()> {
        Self::IStoreConfigurationStatics4(|this| unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), user.into_param().abi(), webaccountid.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `ApplicationModel_Store_Preview`*"]
    pub fn GetEnterpriseStoreWebAccountId() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IStoreConfigurationStatics4(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `ApplicationModel_Store_Preview`, `System`*"]
    pub fn GetEnterpriseStoreWebAccountIdForUser<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::System::User>>(user: Param0) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IStoreConfigurationStatics4(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), user.into_param().abi(), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_Store_Preview`*"]
    pub fn ShouldRestrictToEnterpriseStoreOnly() -> ::windows::runtime::Result<bool> {
        Self::IStoreConfigurationStatics4(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `ApplicationModel_Store_Preview`, `System`*"]
    pub fn ShouldRestrictToEnterpriseStoreOnlyForUser<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::System::User>>(user: Param0) -> ::windows::runtime::Result<bool> {
        Self::IStoreConfigurationStatics4(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), user.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_Store_Preview`*"]
    pub fn IsPinToDesktopSupported() -> ::windows::runtime::Result<bool> {
        Self::IStoreConfigurationStatics5(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_Store_Preview`*"]
    pub fn IsPinToTaskbarSupported() -> ::windows::runtime::Result<bool> {
        Self::IStoreConfigurationStatics5(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_Store_Preview`*"]
    pub fn IsPinToStartSupported() -> ::windows::runtime::Result<bool> {
        Self::IStoreConfigurationStatics5(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_Store_Preview`*"]
    pub fn PinToDesktop<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(apppackagefamilyname: Param0) -> ::windows::runtime::Result<()> {
        Self::IStoreConfigurationStatics5(|this| unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), apppackagefamilyname.into_param().abi()).ok() })
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `ApplicationModel_Store_Preview`, `System`*"]
    pub fn PinToDesktopForUser<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::System::User>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(user: Param0, apppackagefamilyname: Param1) -> ::windows::runtime::Result<()> {
        Self::IStoreConfigurationStatics5(|this| unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), user.into_param().abi(), apppackagefamilyname.into_param().abi()).ok() })
    }
    pub fn IStoreConfigurationStatics<R, F: FnOnce(&IStoreConfigurationStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<StoreConfiguration, IStoreConfigurationStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IStoreConfigurationStatics2<R, F: FnOnce(&IStoreConfigurationStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<StoreConfiguration, IStoreConfigurationStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IStoreConfigurationStatics3<R, F: FnOnce(&IStoreConfigurationStatics3) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<StoreConfiguration, IStoreConfigurationStatics3> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IStoreConfigurationStatics4<R, F: FnOnce(&IStoreConfigurationStatics4) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<StoreConfiguration, IStoreConfigurationStatics4> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IStoreConfigurationStatics5<R, F: FnOnce(&IStoreConfigurationStatics5) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<StoreConfiguration, IStoreConfigurationStatics5> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for StoreConfiguration {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.StoreConfiguration";
}
#[doc = "*Required features: `ApplicationModel_Store_Preview`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct StoreHardwareManufacturerInfo(::windows::runtime::IInspectable);
impl StoreHardwareManufacturerInfo {
    #[doc = "*Required features: `ApplicationModel_Store_Preview`*"]
    pub fn HardwareManufacturerId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Store_Preview`*"]
    pub fn StoreContentModifierId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Store_Preview`*"]
    pub fn ModelName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Store_Preview`*"]
    pub fn ManufacturerName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for StoreHardwareManufacturerInfo {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Store.Preview.StoreHardwareManufacturerInfo;{f292dc08-c654-43ac-a21f-34801c9d3388})");
}
unsafe impl ::windows::runtime::Interface for StoreHardwareManufacturerInfo {
    type Vtable = IStoreHardwareManufacturerInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4069710856, 50772, 17324, [162, 31, 52, 128, 28, 157, 51, 136]);
}
impl ::windows::runtime::RuntimeName for StoreHardwareManufacturerInfo {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.StoreHardwareManufacturerInfo";
}
unsafe impl ::std::marker::Send for StoreHardwareManufacturerInfo {}
unsafe impl ::std::marker::Sync for StoreHardwareManufacturerInfo {}
#[doc = "*Required features: `ApplicationModel_Store_Preview`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct StoreLogOptions(pub u32);
impl StoreLogOptions {
    pub const None: StoreLogOptions = StoreLogOptions(0u32);
    pub const TryElevate: StoreLogOptions = StoreLogOptions(1u32);
}
impl ::std::convert::From<u32> for StoreLogOptions {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for StoreLogOptions {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for StoreLogOptions {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Store.Preview.StoreLogOptions;u4)");
}
impl ::std::ops::BitOr for StoreLogOptions {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for StoreLogOptions {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for StoreLogOptions {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for StoreLogOptions {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for StoreLogOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `ApplicationModel_Store_Preview`*"]
pub struct StorePreview {}
impl StorePreview {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Store_Preview`, `Foundation`*"]
    pub fn RequestProductPurchaseByProductIdAndSkuIdAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(productid: Param0, skuid: Param1) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<StorePreviewPurchaseResults>> {
        Self::IStorePreview(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), productid.into_param().abi(), skuid.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<StorePreviewPurchaseResults>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `ApplicationModel_Store_Preview`, `Foundation`, `Foundation_Collections`*"]
    pub fn LoadAddOnProductInfosAsync() -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<StorePreviewProductInfo>>> {
        Self::IStorePreview(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<StorePreviewProductInfo>>>(result__)
        })
    }
    pub fn IStorePreview<R, F: FnOnce(&IStorePreview) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<StorePreview, IStorePreview> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for StorePreview {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.StorePreview";
}
#[doc = "*Required features: `ApplicationModel_Store_Preview`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct StorePreviewProductInfo(::windows::runtime::IInspectable);
impl StorePreviewProductInfo {
    #[doc = "*Required features: `ApplicationModel_Store_Preview`*"]
    pub fn ProductId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Store_Preview`*"]
    pub fn ProductType(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Store_Preview`*"]
    pub fn Title(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Store_Preview`*"]
    pub fn Description(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Store_Preview`, `Foundation_Collections`*"]
    pub fn SkuInfoList(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<StorePreviewSkuInfo>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<StorePreviewSkuInfo>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for StorePreviewProductInfo {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Store.Preview.StorePreviewProductInfo;{1937dbb3-6c01-4c9d-85cd-5babaac2b351})");
}
unsafe impl ::windows::runtime::Interface for StorePreviewProductInfo {
    type Vtable = IStorePreviewProductInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(423091123, 27649, 19613, [133, 205, 91, 171, 170, 194, 179, 81]);
}
impl ::windows::runtime::RuntimeName for StorePreviewProductInfo {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.StorePreviewProductInfo";
}
unsafe impl ::std::marker::Send for StorePreviewProductInfo {}
unsafe impl ::std::marker::Sync for StorePreviewProductInfo {}
#[doc = "*Required features: `ApplicationModel_Store_Preview`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct StorePreviewProductPurchaseStatus(pub i32);
impl StorePreviewProductPurchaseStatus {
    pub const Succeeded: StorePreviewProductPurchaseStatus = StorePreviewProductPurchaseStatus(0i32);
    pub const AlreadyPurchased: StorePreviewProductPurchaseStatus = StorePreviewProductPurchaseStatus(1i32);
    pub const NotFulfilled: StorePreviewProductPurchaseStatus = StorePreviewProductPurchaseStatus(2i32);
    pub const NotPurchased: StorePreviewProductPurchaseStatus = StorePreviewProductPurchaseStatus(3i32);
}
impl ::std::convert::From<i32> for StorePreviewProductPurchaseStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for StorePreviewProductPurchaseStatus {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for StorePreviewProductPurchaseStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Store.Preview.StorePreviewProductPurchaseStatus;i4)");
}
#[doc = "*Required features: `ApplicationModel_Store_Preview`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct StorePreviewPurchaseResults(::windows::runtime::IInspectable);
impl StorePreviewPurchaseResults {
    #[doc = "*Required features: `ApplicationModel_Store_Preview`*"]
    pub fn ProductPurchaseStatus(&self) -> ::windows::runtime::Result<StorePreviewProductPurchaseStatus> {
        let this = self;
        unsafe {
            let mut result__: StorePreviewProductPurchaseStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<StorePreviewProductPurchaseStatus>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for StorePreviewPurchaseResults {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Store.Preview.StorePreviewPurchaseResults;{b0daaed1-d6c5-4e53-a043-fba0d8e61231})");
}
unsafe impl ::windows::runtime::Interface for StorePreviewPurchaseResults {
    type Vtable = IStorePreviewPurchaseResults_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2967121617, 54981, 20051, [160, 67, 251, 160, 216, 230, 18, 49]);
}
impl ::windows::runtime::RuntimeName for StorePreviewPurchaseResults {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.StorePreviewPurchaseResults";
}
unsafe impl ::std::marker::Send for StorePreviewPurchaseResults {}
unsafe impl ::std::marker::Sync for StorePreviewPurchaseResults {}
#[doc = "*Required features: `ApplicationModel_Store_Preview`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct StorePreviewSkuInfo(::windows::runtime::IInspectable);
impl StorePreviewSkuInfo {
    #[doc = "*Required features: `ApplicationModel_Store_Preview`*"]
    pub fn ProductId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Store_Preview`*"]
    pub fn SkuId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Store_Preview`*"]
    pub fn SkuType(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Store_Preview`*"]
    pub fn Title(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Store_Preview`*"]
    pub fn Description(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Store_Preview`*"]
    pub fn CustomDeveloperData(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Store_Preview`*"]
    pub fn CurrencyCode(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Store_Preview`*"]
    pub fn FormattedListPrice(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Store_Preview`*"]
    pub fn ExtendedData(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for StorePreviewSkuInfo {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Store.Preview.StorePreviewSkuInfo;{81fd76e2-0b26-48d9-98ce-27461c669d6c})");
}
unsafe impl ::windows::runtime::Interface for StorePreviewSkuInfo {
    type Vtable = IStorePreviewSkuInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2180871906, 2854, 18649, [152, 206, 39, 70, 28, 102, 157, 108]);
}
impl ::windows::runtime::RuntimeName for StorePreviewSkuInfo {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.StorePreviewSkuInfo";
}
unsafe impl ::std::marker::Send for StorePreviewSkuInfo {}
unsafe impl ::std::marker::Sync for StorePreviewSkuInfo {}
#[doc = "*Required features: `ApplicationModel_Store_Preview`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct StoreSystemFeature(pub i32);
impl StoreSystemFeature {
    pub const ArchitectureX86: StoreSystemFeature = StoreSystemFeature(0i32);
    pub const ArchitectureX64: StoreSystemFeature = StoreSystemFeature(1i32);
    pub const ArchitectureArm: StoreSystemFeature = StoreSystemFeature(2i32);
    pub const DirectX9: StoreSystemFeature = StoreSystemFeature(3i32);
    pub const DirectX10: StoreSystemFeature = StoreSystemFeature(4i32);
    pub const DirectX11: StoreSystemFeature = StoreSystemFeature(5i32);
    pub const D3D12HardwareFL11: StoreSystemFeature = StoreSystemFeature(6i32);
    pub const D3D12HardwareFL12: StoreSystemFeature = StoreSystemFeature(7i32);
    pub const Memory300MB: StoreSystemFeature = StoreSystemFeature(8i32);
    pub const Memory750MB: StoreSystemFeature = StoreSystemFeature(9i32);
    pub const Memory1GB: StoreSystemFeature = StoreSystemFeature(10i32);
    pub const Memory2GB: StoreSystemFeature = StoreSystemFeature(11i32);
    pub const CameraFront: StoreSystemFeature = StoreSystemFeature(12i32);
    pub const CameraRear: StoreSystemFeature = StoreSystemFeature(13i32);
    pub const Gyroscope: StoreSystemFeature = StoreSystemFeature(14i32);
    pub const Hover: StoreSystemFeature = StoreSystemFeature(15i32);
    pub const Magnetometer: StoreSystemFeature = StoreSystemFeature(16i32);
    pub const Nfc: StoreSystemFeature = StoreSystemFeature(17i32);
    pub const Resolution720P: StoreSystemFeature = StoreSystemFeature(18i32);
    pub const ResolutionWvga: StoreSystemFeature = StoreSystemFeature(19i32);
    pub const ResolutionWvgaOr720P: StoreSystemFeature = StoreSystemFeature(20i32);
    pub const ResolutionWxga: StoreSystemFeature = StoreSystemFeature(21i32);
    pub const ResolutionWvgaOrWxga: StoreSystemFeature = StoreSystemFeature(22i32);
    pub const ResolutionWxgaOr720P: StoreSystemFeature = StoreSystemFeature(23i32);
    pub const Memory4GB: StoreSystemFeature = StoreSystemFeature(24i32);
    pub const Memory6GB: StoreSystemFeature = StoreSystemFeature(25i32);
    pub const Memory8GB: StoreSystemFeature = StoreSystemFeature(26i32);
    pub const Memory12GB: StoreSystemFeature = StoreSystemFeature(27i32);
    pub const Memory16GB: StoreSystemFeature = StoreSystemFeature(28i32);
    pub const Memory20GB: StoreSystemFeature = StoreSystemFeature(29i32);
    pub const VideoMemory2GB: StoreSystemFeature = StoreSystemFeature(30i32);
    pub const VideoMemory4GB: StoreSystemFeature = StoreSystemFeature(31i32);
    pub const VideoMemory6GB: StoreSystemFeature = StoreSystemFeature(32i32);
    pub const VideoMemory1GB: StoreSystemFeature = StoreSystemFeature(33i32);
    pub const ArchitectureArm64: StoreSystemFeature = StoreSystemFeature(34i32);
}
impl ::std::convert::From<i32> for StoreSystemFeature {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for StoreSystemFeature {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for StoreSystemFeature {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Store.Preview.StoreSystemFeature;i4)");
}
#[doc = "*Required features: `ApplicationModel_Store_Preview`*"]
pub struct WebAuthenticationCoreManagerHelper {}
impl WebAuthenticationCoreManagerHelper {
    #[cfg(all(feature = "Foundation", feature = "Security_Authentication_Web_Core", feature = "UI_Xaml"))]
    #[doc = "*Required features: `ApplicationModel_Store_Preview`, `Foundation`, `Security_Authentication_Web_Core`, `UI_Xaml`*"]
    pub fn RequestTokenWithUIElementHostingAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Security::Authentication::Web::Core::WebTokenRequest>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::UI::Xaml::UIElement>>(request: Param0, uielement: Param1) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Security::Authentication::Web::Core::WebTokenRequestResult>> {
        Self::IWebAuthenticationCoreManagerHelper(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), request.into_param().abi(), uielement.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<super::super::super::Security::Authentication::Web::Core::WebTokenRequestResult>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Security_Authentication_Web_Core", feature = "Security_Credentials", feature = "UI_Xaml"))]
    #[doc = "*Required features: `ApplicationModel_Store_Preview`, `Foundation`, `Security_Authentication_Web_Core`, `Security_Credentials`, `UI_Xaml`*"]
    pub fn RequestTokenWithUIElementHostingAndWebAccountAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Security::Authentication::Web::Core::WebTokenRequest>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Security::Credentials::WebAccount>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::UI::Xaml::UIElement>>(
        request: Param0,
        webaccount: Param1,
        uielement: Param2,
    ) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Security::Authentication::Web::Core::WebTokenRequestResult>> {
        Self::IWebAuthenticationCoreManagerHelper(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), request.into_param().abi(), webaccount.into_param().abi(), uielement.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<super::super::super::Security::Authentication::Web::Core::WebTokenRequestResult>>(result__)
        })
    }
    pub fn IWebAuthenticationCoreManagerHelper<R, F: FnOnce(&IWebAuthenticationCoreManagerHelper) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<WebAuthenticationCoreManagerHelper, IWebAuthenticationCoreManagerHelper> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for WebAuthenticationCoreManagerHelper {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.WebAuthenticationCoreManagerHelper";
}
