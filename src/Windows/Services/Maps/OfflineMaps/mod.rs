#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IOfflineMapPackage(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IOfflineMapPackage {
    type Vtable = IOfflineMapPackage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2811717435, 42421, 16708, [181, 37, 230, 140, 136, 98, 102, 75]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineMapPackage_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut OfflineMapPackageStatus) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u64) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IOfflineMapPackageQueryResult(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IOfflineMapPackageQueryResult {
    type Vtable = IOfflineMapPackageQueryResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1431852049, 14817, 20033, [164, 225, 95, 72, 114, 190, 225, 153]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineMapPackageQueryResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut OfflineMapPackageQueryStatus) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IOfflineMapPackageStartDownloadResult(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IOfflineMapPackageStartDownloadResult {
    type Vtable = IOfflineMapPackageStartDownloadResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3647322392, 54486, 19198, [147, 120, 62, 199, 30, 241, 28, 61]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineMapPackageStartDownloadResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut OfflineMapPackageStartDownloadStatus) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IOfflineMapPackageStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IOfflineMapPackageStatics {
    type Vtable = IOfflineMapPackageStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(408844578, 43057, 19120, [148, 31, 105, 152, 250, 146, 146, 133]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineMapPackageStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, querypoint: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))] usize,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, queryboundingbox: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))] usize,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, querycircle: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))] usize,
);
#[doc = "*Required features: `Services_Maps_OfflineMaps`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct OfflineMapPackage(::windows::runtime::IInspectable);
impl OfflineMapPackage {
    #[doc = "*Required features: `Services_Maps_OfflineMaps`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<OfflineMapPackageStatus> {
        let this = self;
        unsafe {
            let mut result__: OfflineMapPackageStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<OfflineMapPackageStatus>(result__)
        }
    }
    #[doc = "*Required features: `Services_Maps_OfflineMaps`*"]
    pub fn DisplayName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Services_Maps_OfflineMaps`*"]
    pub fn EnclosingRegionName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Services_Maps_OfflineMaps`*"]
    pub fn EstimatedSizeInBytes(&self) -> ::windows::runtime::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Services_Maps_OfflineMaps`, `Foundation`*"]
    pub fn RemoveStatusChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Services_Maps_OfflineMaps`, `Foundation`*"]
    pub fn StatusChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<OfflineMapPackage, ::windows::runtime::IInspectable>>>(&self, value: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Services_Maps_OfflineMaps`, `Foundation`*"]
    pub fn RequestStartDownloadAsync(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<OfflineMapPackageStartDownloadResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<OfflineMapPackageStartDownloadResult>>(result__)
        }
    }
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    #[doc = "*Required features: `Services_Maps_OfflineMaps`, `Devices_Geolocation`, `Foundation`*"]
    pub fn FindPackagesAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Devices::Geolocation::Geopoint>>(querypoint: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<OfflineMapPackageQueryResult>> {
        Self::IOfflineMapPackageStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), querypoint.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<OfflineMapPackageQueryResult>>(result__)
        })
    }
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    #[doc = "*Required features: `Services_Maps_OfflineMaps`, `Devices_Geolocation`, `Foundation`*"]
    pub fn FindPackagesInBoundingBoxAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Devices::Geolocation::GeoboundingBox>>(queryboundingbox: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<OfflineMapPackageQueryResult>> {
        Self::IOfflineMapPackageStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), queryboundingbox.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<OfflineMapPackageQueryResult>>(result__)
        })
    }
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    #[doc = "*Required features: `Services_Maps_OfflineMaps`, `Devices_Geolocation`, `Foundation`*"]
    pub fn FindPackagesInGeocircleAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Devices::Geolocation::Geocircle>>(querycircle: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<OfflineMapPackageQueryResult>> {
        Self::IOfflineMapPackageStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), querycircle.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<OfflineMapPackageQueryResult>>(result__)
        })
    }
    pub fn IOfflineMapPackageStatics<R, F: FnOnce(&IOfflineMapPackageStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<OfflineMapPackage, IOfflineMapPackageStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for OfflineMapPackage {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.OfflineMaps.OfflineMapPackage;{a797673b-a5b5-4144-b525-e68c8862664b})");
}
unsafe impl ::windows::runtime::Interface for OfflineMapPackage {
    type Vtable = IOfflineMapPackage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2811717435, 42421, 16708, [181, 37, 230, 140, 136, 98, 102, 75]);
}
impl ::windows::runtime::RuntimeName for OfflineMapPackage {
    const NAME: &'static str = "Windows.Services.Maps.OfflineMaps.OfflineMapPackage";
}
impl ::std::convert::From<OfflineMapPackage> for ::windows::runtime::IUnknown {
    fn from(value: OfflineMapPackage) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&OfflineMapPackage> for ::windows::runtime::IUnknown {
    fn from(value: &OfflineMapPackage) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for OfflineMapPackage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &OfflineMapPackage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<OfflineMapPackage> for ::windows::runtime::IInspectable {
    fn from(value: OfflineMapPackage) -> Self {
        value.0
    }
}
impl ::std::convert::From<&OfflineMapPackage> for ::windows::runtime::IInspectable {
    fn from(value: &OfflineMapPackage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for OfflineMapPackage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a OfflineMapPackage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for OfflineMapPackage {}
unsafe impl ::std::marker::Sync for OfflineMapPackage {}
#[doc = "*Required features: `Services_Maps_OfflineMaps`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct OfflineMapPackageQueryResult(::windows::runtime::IInspectable);
impl OfflineMapPackageQueryResult {
    #[doc = "*Required features: `Services_Maps_OfflineMaps`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<OfflineMapPackageQueryStatus> {
        let this = self;
        unsafe {
            let mut result__: OfflineMapPackageQueryStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<OfflineMapPackageQueryStatus>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Services_Maps_OfflineMaps`, `Foundation_Collections`*"]
    pub fn Packages(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<OfflineMapPackage>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<OfflineMapPackage>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for OfflineMapPackageQueryResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.OfflineMaps.OfflineMapPackageQueryResult;{55585411-39e1-4e41-a4e1-5f4872bee199})");
}
unsafe impl ::windows::runtime::Interface for OfflineMapPackageQueryResult {
    type Vtable = IOfflineMapPackageQueryResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1431852049, 14817, 20033, [164, 225, 95, 72, 114, 190, 225, 153]);
}
impl ::windows::runtime::RuntimeName for OfflineMapPackageQueryResult {
    const NAME: &'static str = "Windows.Services.Maps.OfflineMaps.OfflineMapPackageQueryResult";
}
impl ::std::convert::From<OfflineMapPackageQueryResult> for ::windows::runtime::IUnknown {
    fn from(value: OfflineMapPackageQueryResult) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&OfflineMapPackageQueryResult> for ::windows::runtime::IUnknown {
    fn from(value: &OfflineMapPackageQueryResult) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for OfflineMapPackageQueryResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &OfflineMapPackageQueryResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<OfflineMapPackageQueryResult> for ::windows::runtime::IInspectable {
    fn from(value: OfflineMapPackageQueryResult) -> Self {
        value.0
    }
}
impl ::std::convert::From<&OfflineMapPackageQueryResult> for ::windows::runtime::IInspectable {
    fn from(value: &OfflineMapPackageQueryResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for OfflineMapPackageQueryResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a OfflineMapPackageQueryResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for OfflineMapPackageQueryResult {}
unsafe impl ::std::marker::Sync for OfflineMapPackageQueryResult {}
#[doc = "*Required features: `Services_Maps_OfflineMaps`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct OfflineMapPackageQueryStatus(pub i32);
impl OfflineMapPackageQueryStatus {
    pub const Success: OfflineMapPackageQueryStatus = OfflineMapPackageQueryStatus(0i32);
    pub const UnknownError: OfflineMapPackageQueryStatus = OfflineMapPackageQueryStatus(1i32);
    pub const InvalidCredentials: OfflineMapPackageQueryStatus = OfflineMapPackageQueryStatus(2i32);
    pub const NetworkFailure: OfflineMapPackageQueryStatus = OfflineMapPackageQueryStatus(3i32);
}
impl ::std::convert::From<i32> for OfflineMapPackageQueryStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for OfflineMapPackageQueryStatus {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for OfflineMapPackageQueryStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Services.Maps.OfflineMaps.OfflineMapPackageQueryStatus;i4)");
}
#[doc = "*Required features: `Services_Maps_OfflineMaps`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct OfflineMapPackageStartDownloadResult(::windows::runtime::IInspectable);
impl OfflineMapPackageStartDownloadResult {
    #[doc = "*Required features: `Services_Maps_OfflineMaps`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<OfflineMapPackageStartDownloadStatus> {
        let this = self;
        unsafe {
            let mut result__: OfflineMapPackageStartDownloadStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<OfflineMapPackageStartDownloadStatus>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for OfflineMapPackageStartDownloadResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.OfflineMaps.OfflineMapPackageStartDownloadResult;{d965b918-d4d6-4afe-9378-3ec71ef11c3d})");
}
unsafe impl ::windows::runtime::Interface for OfflineMapPackageStartDownloadResult {
    type Vtable = IOfflineMapPackageStartDownloadResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3647322392, 54486, 19198, [147, 120, 62, 199, 30, 241, 28, 61]);
}
impl ::windows::runtime::RuntimeName for OfflineMapPackageStartDownloadResult {
    const NAME: &'static str = "Windows.Services.Maps.OfflineMaps.OfflineMapPackageStartDownloadResult";
}
impl ::std::convert::From<OfflineMapPackageStartDownloadResult> for ::windows::runtime::IUnknown {
    fn from(value: OfflineMapPackageStartDownloadResult) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&OfflineMapPackageStartDownloadResult> for ::windows::runtime::IUnknown {
    fn from(value: &OfflineMapPackageStartDownloadResult) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for OfflineMapPackageStartDownloadResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &OfflineMapPackageStartDownloadResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<OfflineMapPackageStartDownloadResult> for ::windows::runtime::IInspectable {
    fn from(value: OfflineMapPackageStartDownloadResult) -> Self {
        value.0
    }
}
impl ::std::convert::From<&OfflineMapPackageStartDownloadResult> for ::windows::runtime::IInspectable {
    fn from(value: &OfflineMapPackageStartDownloadResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for OfflineMapPackageStartDownloadResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a OfflineMapPackageStartDownloadResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for OfflineMapPackageStartDownloadResult {}
unsafe impl ::std::marker::Sync for OfflineMapPackageStartDownloadResult {}
#[doc = "*Required features: `Services_Maps_OfflineMaps`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct OfflineMapPackageStartDownloadStatus(pub i32);
impl OfflineMapPackageStartDownloadStatus {
    pub const Success: OfflineMapPackageStartDownloadStatus = OfflineMapPackageStartDownloadStatus(0i32);
    pub const UnknownError: OfflineMapPackageStartDownloadStatus = OfflineMapPackageStartDownloadStatus(1i32);
    pub const InvalidCredentials: OfflineMapPackageStartDownloadStatus = OfflineMapPackageStartDownloadStatus(2i32);
    pub const DeniedWithoutCapability: OfflineMapPackageStartDownloadStatus = OfflineMapPackageStartDownloadStatus(3i32);
}
impl ::std::convert::From<i32> for OfflineMapPackageStartDownloadStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for OfflineMapPackageStartDownloadStatus {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for OfflineMapPackageStartDownloadStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Services.Maps.OfflineMaps.OfflineMapPackageStartDownloadStatus;i4)");
}
#[doc = "*Required features: `Services_Maps_OfflineMaps`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct OfflineMapPackageStatus(pub i32);
impl OfflineMapPackageStatus {
    pub const NotDownloaded: OfflineMapPackageStatus = OfflineMapPackageStatus(0i32);
    pub const Downloading: OfflineMapPackageStatus = OfflineMapPackageStatus(1i32);
    pub const Downloaded: OfflineMapPackageStatus = OfflineMapPackageStatus(2i32);
    pub const Deleting: OfflineMapPackageStatus = OfflineMapPackageStatus(3i32);
}
impl ::std::convert::From<i32> for OfflineMapPackageStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for OfflineMapPackageStatus {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for OfflineMapPackageStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Services.Maps.OfflineMaps.OfflineMapPackageStatus;i4)");
}
