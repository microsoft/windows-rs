#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[doc(hidden)]
pub struct IOfflineMapPackage(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IOfflineMapPackage {
    type Vtable = IOfflineMapPackage_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa797673b_a5b5_4144_b525_e68c8862664b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineMapPackage_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut OfflineMapPackageStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IOfflineMapPackageQueryResult(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IOfflineMapPackageQueryResult {
    type Vtable = IOfflineMapPackageQueryResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x55585411_39e1_4e41_a4e1_5f4872bee199);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineMapPackageQueryResult_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut OfflineMapPackageQueryStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IOfflineMapPackageStartDownloadResult(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IOfflineMapPackageStartDownloadResult {
    type Vtable = IOfflineMapPackageStartDownloadResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd965b918_d4d6_4afe_9378_3ec71ef11c3d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineMapPackageStartDownloadResult_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut OfflineMapPackageStartDownloadStatus) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IOfflineMapPackageStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IOfflineMapPackageStatics {
    type Vtable = IOfflineMapPackageStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x185e7922_a831_4ab0_941f_6998fa929285);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineMapPackageStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, querypoint: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))] usize,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, queryboundingbox: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))] usize,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, querycircle: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct OfflineMapPackage(pub ::windows::core::IInspectable);
impl OfflineMapPackage {
    pub fn Status(&self) -> ::windows::core::Result<OfflineMapPackageStatus> {
        let this = self;
        unsafe {
            let mut result__: OfflineMapPackageStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<OfflineMapPackageStatus>(result__)
        }
    }
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn EnclosingRegionName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn EstimatedSizeInBytes(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn StatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<OfflineMapPackage, ::windows::core::IInspectable>>>(&self, value: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RequestStartDownloadAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<OfflineMapPackageStartDownloadResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<OfflineMapPackageStartDownloadResult>>(result__)
        }
    }
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub fn FindPackagesAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Devices::Geolocation::Geopoint>>(querypoint: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<OfflineMapPackageQueryResult>> {
        Self::IOfflineMapPackageStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), querypoint.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<OfflineMapPackageQueryResult>>(result__)
        })
    }
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub fn FindPackagesInBoundingBoxAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Devices::Geolocation::GeoboundingBox>>(queryboundingbox: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<OfflineMapPackageQueryResult>> {
        Self::IOfflineMapPackageStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), queryboundingbox.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<OfflineMapPackageQueryResult>>(result__)
        })
    }
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub fn FindPackagesInGeocircleAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Devices::Geolocation::Geocircle>>(querycircle: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<OfflineMapPackageQueryResult>> {
        Self::IOfflineMapPackageStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), querycircle.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<OfflineMapPackageQueryResult>>(result__)
        })
    }
    pub fn IOfflineMapPackageStatics<R, F: FnOnce(&IOfflineMapPackageStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<OfflineMapPackage, IOfflineMapPackageStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for OfflineMapPackage {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.OfflineMaps.OfflineMapPackage;{a797673b-a5b5-4144-b525-e68c8862664b})");
}
unsafe impl ::windows::core::Interface for OfflineMapPackage {
    type Vtable = IOfflineMapPackage_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa797673b_a5b5_4144_b525_e68c8862664b);
}
impl ::windows::core::RuntimeName for OfflineMapPackage {
    const NAME: &'static str = "Windows.Services.Maps.OfflineMaps.OfflineMapPackage";
}
impl ::core::convert::From<OfflineMapPackage> for ::windows::core::IUnknown {
    fn from(value: OfflineMapPackage) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&OfflineMapPackage> for ::windows::core::IUnknown {
    fn from(value: &OfflineMapPackage) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for OfflineMapPackage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a OfflineMapPackage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<OfflineMapPackage> for ::windows::core::IInspectable {
    fn from(value: OfflineMapPackage) -> Self {
        value.0
    }
}
impl ::core::convert::From<&OfflineMapPackage> for ::windows::core::IInspectable {
    fn from(value: &OfflineMapPackage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for OfflineMapPackage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a OfflineMapPackage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for OfflineMapPackage {}
unsafe impl ::core::marker::Sync for OfflineMapPackage {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct OfflineMapPackageQueryResult(pub ::windows::core::IInspectable);
impl OfflineMapPackageQueryResult {
    pub fn Status(&self) -> ::windows::core::Result<OfflineMapPackageQueryStatus> {
        let this = self;
        unsafe {
            let mut result__: OfflineMapPackageQueryStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<OfflineMapPackageQueryStatus>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Packages(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<OfflineMapPackage>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<OfflineMapPackage>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for OfflineMapPackageQueryResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.OfflineMaps.OfflineMapPackageQueryResult;{55585411-39e1-4e41-a4e1-5f4872bee199})");
}
unsafe impl ::windows::core::Interface for OfflineMapPackageQueryResult {
    type Vtable = IOfflineMapPackageQueryResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x55585411_39e1_4e41_a4e1_5f4872bee199);
}
impl ::windows::core::RuntimeName for OfflineMapPackageQueryResult {
    const NAME: &'static str = "Windows.Services.Maps.OfflineMaps.OfflineMapPackageQueryResult";
}
impl ::core::convert::From<OfflineMapPackageQueryResult> for ::windows::core::IUnknown {
    fn from(value: OfflineMapPackageQueryResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&OfflineMapPackageQueryResult> for ::windows::core::IUnknown {
    fn from(value: &OfflineMapPackageQueryResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for OfflineMapPackageQueryResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a OfflineMapPackageQueryResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<OfflineMapPackageQueryResult> for ::windows::core::IInspectable {
    fn from(value: OfflineMapPackageQueryResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&OfflineMapPackageQueryResult> for ::windows::core::IInspectable {
    fn from(value: &OfflineMapPackageQueryResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for OfflineMapPackageQueryResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a OfflineMapPackageQueryResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for OfflineMapPackageQueryResult {}
unsafe impl ::core::marker::Sync for OfflineMapPackageQueryResult {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct OfflineMapPackageQueryStatus(pub i32);
impl OfflineMapPackageQueryStatus {
    pub const Success: OfflineMapPackageQueryStatus = OfflineMapPackageQueryStatus(0i32);
    pub const UnknownError: OfflineMapPackageQueryStatus = OfflineMapPackageQueryStatus(1i32);
    pub const InvalidCredentials: OfflineMapPackageQueryStatus = OfflineMapPackageQueryStatus(2i32);
    pub const NetworkFailure: OfflineMapPackageQueryStatus = OfflineMapPackageQueryStatus(3i32);
}
impl ::core::convert::From<i32> for OfflineMapPackageQueryStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for OfflineMapPackageQueryStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for OfflineMapPackageQueryStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Services.Maps.OfflineMaps.OfflineMapPackageQueryStatus;i4)");
}
impl ::windows::core::DefaultType for OfflineMapPackageQueryStatus {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct OfflineMapPackageStartDownloadResult(pub ::windows::core::IInspectable);
impl OfflineMapPackageStartDownloadResult {
    pub fn Status(&self) -> ::windows::core::Result<OfflineMapPackageStartDownloadStatus> {
        let this = self;
        unsafe {
            let mut result__: OfflineMapPackageStartDownloadStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<OfflineMapPackageStartDownloadStatus>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for OfflineMapPackageStartDownloadResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.OfflineMaps.OfflineMapPackageStartDownloadResult;{d965b918-d4d6-4afe-9378-3ec71ef11c3d})");
}
unsafe impl ::windows::core::Interface for OfflineMapPackageStartDownloadResult {
    type Vtable = IOfflineMapPackageStartDownloadResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd965b918_d4d6_4afe_9378_3ec71ef11c3d);
}
impl ::windows::core::RuntimeName for OfflineMapPackageStartDownloadResult {
    const NAME: &'static str = "Windows.Services.Maps.OfflineMaps.OfflineMapPackageStartDownloadResult";
}
impl ::core::convert::From<OfflineMapPackageStartDownloadResult> for ::windows::core::IUnknown {
    fn from(value: OfflineMapPackageStartDownloadResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&OfflineMapPackageStartDownloadResult> for ::windows::core::IUnknown {
    fn from(value: &OfflineMapPackageStartDownloadResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for OfflineMapPackageStartDownloadResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a OfflineMapPackageStartDownloadResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<OfflineMapPackageStartDownloadResult> for ::windows::core::IInspectable {
    fn from(value: OfflineMapPackageStartDownloadResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&OfflineMapPackageStartDownloadResult> for ::windows::core::IInspectable {
    fn from(value: &OfflineMapPackageStartDownloadResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for OfflineMapPackageStartDownloadResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a OfflineMapPackageStartDownloadResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for OfflineMapPackageStartDownloadResult {}
unsafe impl ::core::marker::Sync for OfflineMapPackageStartDownloadResult {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct OfflineMapPackageStartDownloadStatus(pub i32);
impl OfflineMapPackageStartDownloadStatus {
    pub const Success: OfflineMapPackageStartDownloadStatus = OfflineMapPackageStartDownloadStatus(0i32);
    pub const UnknownError: OfflineMapPackageStartDownloadStatus = OfflineMapPackageStartDownloadStatus(1i32);
    pub const InvalidCredentials: OfflineMapPackageStartDownloadStatus = OfflineMapPackageStartDownloadStatus(2i32);
    pub const DeniedWithoutCapability: OfflineMapPackageStartDownloadStatus = OfflineMapPackageStartDownloadStatus(3i32);
}
impl ::core::convert::From<i32> for OfflineMapPackageStartDownloadStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for OfflineMapPackageStartDownloadStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for OfflineMapPackageStartDownloadStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Services.Maps.OfflineMaps.OfflineMapPackageStartDownloadStatus;i4)");
}
impl ::windows::core::DefaultType for OfflineMapPackageStartDownloadStatus {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct OfflineMapPackageStatus(pub i32);
impl OfflineMapPackageStatus {
    pub const NotDownloaded: OfflineMapPackageStatus = OfflineMapPackageStatus(0i32);
    pub const Downloading: OfflineMapPackageStatus = OfflineMapPackageStatus(1i32);
    pub const Downloaded: OfflineMapPackageStatus = OfflineMapPackageStatus(2i32);
    pub const Deleting: OfflineMapPackageStatus = OfflineMapPackageStatus(3i32);
}
impl ::core::convert::From<i32> for OfflineMapPackageStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for OfflineMapPackageStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for OfflineMapPackageStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Services.Maps.OfflineMaps.OfflineMapPackageStatus;i4)");
}
impl ::windows::core::DefaultType for OfflineMapPackageStatus {
    type DefaultType = Self;
}
