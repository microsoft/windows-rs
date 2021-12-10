#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc(hidden)]
#[repr(transparent)]
pub struct IOfflineMapPackage(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IOfflineMapPackage {
    type Vtable = IOfflineMapPackageVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa797673b_a5b5_4144_b525_e68c8862664b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineMapPackageVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut OfflineMapPackageStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IOfflineMapPackageQueryResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IOfflineMapPackageQueryResult {
    type Vtable = IOfflineMapPackageQueryResultVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x55585411_39e1_4e41_a4e1_5f4872bee199);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineMapPackageQueryResultVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut OfflineMapPackageQueryStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IOfflineMapPackageStartDownloadResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IOfflineMapPackageStartDownloadResult {
    type Vtable = IOfflineMapPackageStartDownloadResultVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd965b918_d4d6_4afe_9378_3ec71ef11c3d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineMapPackageStartDownloadResultVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut OfflineMapPackageStartDownloadStatus) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IOfflineMapPackageStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IOfflineMapPackageStatics {
    type Vtable = IOfflineMapPackageStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x185e7922_a831_4ab0_941f_6998fa929285);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineMapPackageStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, querypoint: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))] usize,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, queryboundingbox: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))] usize,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, querycircle: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))] usize,
);
#[doc = "*Required features: 'Services_Maps_OfflineMaps'*"]
#[repr(transparent)]
pub struct OfflineMapPackage(::windows::core::IUnknown);
impl OfflineMapPackage {
    #[doc = "*Required features: 'Services_Maps_OfflineMaps'*"]
    pub fn Status(&self) -> ::windows::core::Result<OfflineMapPackageStatus> {
        let this = self;
        unsafe {
            let mut result__: OfflineMapPackageStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<OfflineMapPackageStatus>(result__)
        }
    }
    #[doc = "*Required features: 'Services_Maps_OfflineMaps'*"]
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Services_Maps_OfflineMaps'*"]
    pub fn EnclosingRegionName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Services_Maps_OfflineMaps'*"]
    pub fn EstimatedSizeInBytes(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: 'Services_Maps_OfflineMaps', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Services_Maps_OfflineMaps', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn StatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<OfflineMapPackage, ::windows::core::IInspectable>>>(&self, value: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Services_Maps_OfflineMaps', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestStartDownloadAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<OfflineMapPackageStartDownloadResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<OfflineMapPackageStartDownloadResult>>(result__)
        }
    }
    #[doc = "*Required features: 'Services_Maps_OfflineMaps', 'Devices_Geolocation', 'Foundation'*"]
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub fn FindPackagesAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Devices::Geolocation::Geopoint>>(querypoint: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<OfflineMapPackageQueryResult>> {
        Self::IOfflineMapPackageStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), querypoint.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<OfflineMapPackageQueryResult>>(result__)
        })
    }
    #[doc = "*Required features: 'Services_Maps_OfflineMaps', 'Devices_Geolocation', 'Foundation'*"]
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub fn FindPackagesInBoundingBoxAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Devices::Geolocation::GeoboundingBox>>(queryboundingbox: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<OfflineMapPackageQueryResult>> {
        Self::IOfflineMapPackageStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), queryboundingbox.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<OfflineMapPackageQueryResult>>(result__)
        })
    }
    #[doc = "*Required features: 'Services_Maps_OfflineMaps', 'Devices_Geolocation', 'Foundation'*"]
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub fn FindPackagesInGeocircleAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Devices::Geolocation::Geocircle>>(querycircle: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<OfflineMapPackageQueryResult>> {
        Self::IOfflineMapPackageStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), querycircle.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<OfflineMapPackageQueryResult>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IOfflineMapPackageStatics<R, F: FnOnce(&IOfflineMapPackageStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<OfflineMapPackage, IOfflineMapPackageStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for OfflineMapPackage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for OfflineMapPackage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for OfflineMapPackage {}
unsafe impl ::windows::core::RuntimeType for OfflineMapPackage {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.OfflineMaps.OfflineMapPackage;{a797673b-a5b5-4144-b525-e68c8862664b})");
}
unsafe impl ::windows::core::Interface for OfflineMapPackage {
    type Vtable = IOfflineMapPackageVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa797673b_a5b5_4144_b525_e68c8862664b);
}
impl ::windows::core::RuntimeName for OfflineMapPackage {
    const NAME: &'static str = "Windows.Services.Maps.OfflineMaps.OfflineMapPackage";
}
impl ::core::convert::From<OfflineMapPackage> for ::windows::core::IUnknown {
    fn from(value: OfflineMapPackage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&OfflineMapPackage> for ::windows::core::IUnknown {
    fn from(value: &OfflineMapPackage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for OfflineMapPackage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &OfflineMapPackage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<OfflineMapPackage> for ::windows::core::IInspectable {
    fn from(value: OfflineMapPackage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&OfflineMapPackage> for ::windows::core::IInspectable {
    fn from(value: &OfflineMapPackage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for OfflineMapPackage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &OfflineMapPackage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for OfflineMapPackage {}
unsafe impl ::core::marker::Sync for OfflineMapPackage {}
#[doc = "*Required features: 'Services_Maps_OfflineMaps'*"]
#[repr(transparent)]
pub struct OfflineMapPackageQueryResult(::windows::core::IUnknown);
impl OfflineMapPackageQueryResult {
    #[doc = "*Required features: 'Services_Maps_OfflineMaps'*"]
    pub fn Status(&self) -> ::windows::core::Result<OfflineMapPackageQueryStatus> {
        let this = self;
        unsafe {
            let mut result__: OfflineMapPackageQueryStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<OfflineMapPackageQueryStatus>(result__)
        }
    }
    #[doc = "*Required features: 'Services_Maps_OfflineMaps', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Packages(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<OfflineMapPackage>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<OfflineMapPackage>>(result__)
        }
    }
}
impl ::core::clone::Clone for OfflineMapPackageQueryResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for OfflineMapPackageQueryResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for OfflineMapPackageQueryResult {}
unsafe impl ::windows::core::RuntimeType for OfflineMapPackageQueryResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.OfflineMaps.OfflineMapPackageQueryResult;{55585411-39e1-4e41-a4e1-5f4872bee199})");
}
unsafe impl ::windows::core::Interface for OfflineMapPackageQueryResult {
    type Vtable = IOfflineMapPackageQueryResultVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x55585411_39e1_4e41_a4e1_5f4872bee199);
}
impl ::windows::core::RuntimeName for OfflineMapPackageQueryResult {
    const NAME: &'static str = "Windows.Services.Maps.OfflineMaps.OfflineMapPackageQueryResult";
}
impl ::core::convert::From<OfflineMapPackageQueryResult> for ::windows::core::IUnknown {
    fn from(value: OfflineMapPackageQueryResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&OfflineMapPackageQueryResult> for ::windows::core::IUnknown {
    fn from(value: &OfflineMapPackageQueryResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for OfflineMapPackageQueryResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &OfflineMapPackageQueryResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<OfflineMapPackageQueryResult> for ::windows::core::IInspectable {
    fn from(value: OfflineMapPackageQueryResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&OfflineMapPackageQueryResult> for ::windows::core::IInspectable {
    fn from(value: &OfflineMapPackageQueryResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for OfflineMapPackageQueryResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &OfflineMapPackageQueryResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for OfflineMapPackageQueryResult {}
unsafe impl ::core::marker::Sync for OfflineMapPackageQueryResult {}
#[doc = "*Required features: 'Services_Maps_OfflineMaps'*"]
#[repr(transparent)]
pub struct OfflineMapPackageQueryStatus(pub i32);
impl OfflineMapPackageQueryStatus {
    pub const Success: Self = Self(0i32);
    pub const UnknownError: Self = Self(1i32);
    pub const InvalidCredentials: Self = Self(2i32);
    pub const NetworkFailure: Self = Self(3i32);
}
impl ::core::marker::Copy for OfflineMapPackageQueryStatus {}
impl ::core::clone::Clone for OfflineMapPackageQueryStatus {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for OfflineMapPackageQueryStatus {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for OfflineMapPackageQueryStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for OfflineMapPackageQueryStatus {}
unsafe impl ::windows::core::RuntimeType for OfflineMapPackageQueryStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Services.Maps.OfflineMaps.OfflineMapPackageQueryStatus;i4)");
}
impl ::windows::core::DefaultType for OfflineMapPackageQueryStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Services_Maps_OfflineMaps'*"]
#[repr(transparent)]
pub struct OfflineMapPackageStartDownloadResult(::windows::core::IUnknown);
impl OfflineMapPackageStartDownloadResult {
    #[doc = "*Required features: 'Services_Maps_OfflineMaps'*"]
    pub fn Status(&self) -> ::windows::core::Result<OfflineMapPackageStartDownloadStatus> {
        let this = self;
        unsafe {
            let mut result__: OfflineMapPackageStartDownloadStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<OfflineMapPackageStartDownloadStatus>(result__)
        }
    }
}
impl ::core::clone::Clone for OfflineMapPackageStartDownloadResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for OfflineMapPackageStartDownloadResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for OfflineMapPackageStartDownloadResult {}
unsafe impl ::windows::core::RuntimeType for OfflineMapPackageStartDownloadResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.OfflineMaps.OfflineMapPackageStartDownloadResult;{d965b918-d4d6-4afe-9378-3ec71ef11c3d})");
}
unsafe impl ::windows::core::Interface for OfflineMapPackageStartDownloadResult {
    type Vtable = IOfflineMapPackageStartDownloadResultVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd965b918_d4d6_4afe_9378_3ec71ef11c3d);
}
impl ::windows::core::RuntimeName for OfflineMapPackageStartDownloadResult {
    const NAME: &'static str = "Windows.Services.Maps.OfflineMaps.OfflineMapPackageStartDownloadResult";
}
impl ::core::convert::From<OfflineMapPackageStartDownloadResult> for ::windows::core::IUnknown {
    fn from(value: OfflineMapPackageStartDownloadResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&OfflineMapPackageStartDownloadResult> for ::windows::core::IUnknown {
    fn from(value: &OfflineMapPackageStartDownloadResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for OfflineMapPackageStartDownloadResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &OfflineMapPackageStartDownloadResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<OfflineMapPackageStartDownloadResult> for ::windows::core::IInspectable {
    fn from(value: OfflineMapPackageStartDownloadResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&OfflineMapPackageStartDownloadResult> for ::windows::core::IInspectable {
    fn from(value: &OfflineMapPackageStartDownloadResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for OfflineMapPackageStartDownloadResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &OfflineMapPackageStartDownloadResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for OfflineMapPackageStartDownloadResult {}
unsafe impl ::core::marker::Sync for OfflineMapPackageStartDownloadResult {}
#[doc = "*Required features: 'Services_Maps_OfflineMaps'*"]
#[repr(transparent)]
pub struct OfflineMapPackageStartDownloadStatus(pub i32);
impl OfflineMapPackageStartDownloadStatus {
    pub const Success: Self = Self(0i32);
    pub const UnknownError: Self = Self(1i32);
    pub const InvalidCredentials: Self = Self(2i32);
    pub const DeniedWithoutCapability: Self = Self(3i32);
}
impl ::core::marker::Copy for OfflineMapPackageStartDownloadStatus {}
impl ::core::clone::Clone for OfflineMapPackageStartDownloadStatus {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for OfflineMapPackageStartDownloadStatus {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for OfflineMapPackageStartDownloadStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for OfflineMapPackageStartDownloadStatus {}
unsafe impl ::windows::core::RuntimeType for OfflineMapPackageStartDownloadStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Services.Maps.OfflineMaps.OfflineMapPackageStartDownloadStatus;i4)");
}
impl ::windows::core::DefaultType for OfflineMapPackageStartDownloadStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Services_Maps_OfflineMaps'*"]
#[repr(transparent)]
pub struct OfflineMapPackageStatus(pub i32);
impl OfflineMapPackageStatus {
    pub const NotDownloaded: Self = Self(0i32);
    pub const Downloading: Self = Self(1i32);
    pub const Downloaded: Self = Self(2i32);
    pub const Deleting: Self = Self(3i32);
}
impl ::core::marker::Copy for OfflineMapPackageStatus {}
impl ::core::clone::Clone for OfflineMapPackageStatus {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for OfflineMapPackageStatus {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for OfflineMapPackageStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for OfflineMapPackageStatus {}
unsafe impl ::windows::core::RuntimeType for OfflineMapPackageStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Services.Maps.OfflineMaps.OfflineMapPackageStatus;i4)");
}
impl ::windows::core::DefaultType for OfflineMapPackageStatus {
    type DefaultType = Self;
}
