#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc(hidden)]
#[repr(transparent)]
pub struct IOfflineMapPackage(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IOfflineMapPackage {
    type Vtable = IOfflineMapPackage_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa797673b_a5b5_4144_b525_e68c8862664b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineMapPackage_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut OfflineMapPackageStatus) -> ::windows::core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub EnclosingRegionName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub EstimatedSizeInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RemoveStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub StatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RequestStartDownloadAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestStartDownloadAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IOfflineMapPackageQueryResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IOfflineMapPackageQueryResult {
    type Vtable = IOfflineMapPackageQueryResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x55585411_39e1_4e41_a4e1_5f4872bee199);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineMapPackageQueryResult_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut OfflineMapPackageQueryStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Packages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Packages: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IOfflineMapPackageStartDownloadResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IOfflineMapPackageStartDownloadResult {
    type Vtable = IOfflineMapPackageStartDownloadResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd965b918_d4d6_4afe_9378_3ec71ef11c3d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineMapPackageStartDownloadResult_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut OfflineMapPackageStartDownloadStatus) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IOfflineMapPackageStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IOfflineMapPackageStatics {
    type Vtable = IOfflineMapPackageStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x185e7922_a831_4ab0_941f_6998fa929285);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineMapPackageStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub FindPackagesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, querypoint: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))]
    FindPackagesAsync: usize,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub FindPackagesInBoundingBoxAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, queryboundingbox: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))]
    FindPackagesInBoundingBoxAsync: usize,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub FindPackagesInGeocircleAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, querycircle: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))]
    FindPackagesInGeocircleAsync: usize,
}
#[doc = "*Required features: `\"Services_Maps_OfflineMaps\"`*"]
#[repr(transparent)]
pub struct OfflineMapPackage(::windows::core::IUnknown);
impl OfflineMapPackage {
    #[doc = "*Required features: `\"Services_Maps_OfflineMaps\"`*"]
    pub fn Status(&self) -> ::windows::core::Result<OfflineMapPackageStatus> {
        let this = self;
        unsafe {
            let mut result__: OfflineMapPackageStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::core::mem::transmute_copy(this), &mut result__).from_abi::<OfflineMapPackageStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Maps_OfflineMaps\"`*"]
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DisplayName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Maps_OfflineMaps\"`*"]
    pub fn EnclosingRegionName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EnclosingRegionName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Maps_OfflineMaps\"`*"]
    pub fn EstimatedSizeInBytes(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EstimatedSizeInBytes)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Maps_OfflineMaps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveStatusChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Services_Maps_OfflineMaps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<OfflineMapPackage, ::windows::core::IInspectable>>>(&self, value: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StatusChanged)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Maps_OfflineMaps\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestStartDownloadAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<OfflineMapPackageStartDownloadResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestStartDownloadAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<OfflineMapPackageStartDownloadResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Maps_OfflineMaps\"`, `\"Devices_Geolocation\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub fn FindPackagesAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Devices::Geolocation::Geopoint>>(querypoint: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<OfflineMapPackageQueryResult>> {
        Self::IOfflineMapPackageStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FindPackagesAsync)(::core::mem::transmute_copy(this), querypoint.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<OfflineMapPackageQueryResult>>(result__)
        })
    }
    #[doc = "*Required features: `\"Services_Maps_OfflineMaps\"`, `\"Devices_Geolocation\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub fn FindPackagesInBoundingBoxAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Devices::Geolocation::GeoboundingBox>>(queryboundingbox: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<OfflineMapPackageQueryResult>> {
        Self::IOfflineMapPackageStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FindPackagesInBoundingBoxAsync)(::core::mem::transmute_copy(this), queryboundingbox.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<OfflineMapPackageQueryResult>>(result__)
        })
    }
    #[doc = "*Required features: `\"Services_Maps_OfflineMaps\"`, `\"Devices_Geolocation\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub fn FindPackagesInGeocircleAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Devices::Geolocation::Geocircle>>(querycircle: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<OfflineMapPackageQueryResult>> {
        Self::IOfflineMapPackageStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FindPackagesInGeocircleAsync)(::core::mem::transmute_copy(this), querycircle.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<OfflineMapPackageQueryResult>>(result__)
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
impl ::core::fmt::Debug for OfflineMapPackage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OfflineMapPackage").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for OfflineMapPackage {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.OfflineMaps.OfflineMapPackage;{a797673b-a5b5-4144-b525-e68c8862664b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for OfflineMapPackage {
    type Vtable = IOfflineMapPackage_Vtbl;
    const IID: ::windows::core::GUID = <IOfflineMapPackage as ::windows::core::Interface>::IID;
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a OfflineMapPackage {
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a OfflineMapPackage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for OfflineMapPackage {}
unsafe impl ::core::marker::Sync for OfflineMapPackage {}
#[doc = "*Required features: `\"Services_Maps_OfflineMaps\"`*"]
#[repr(transparent)]
pub struct OfflineMapPackageQueryResult(::windows::core::IUnknown);
impl OfflineMapPackageQueryResult {
    #[doc = "*Required features: `\"Services_Maps_OfflineMaps\"`*"]
    pub fn Status(&self) -> ::windows::core::Result<OfflineMapPackageQueryStatus> {
        let this = self;
        unsafe {
            let mut result__: OfflineMapPackageQueryStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::core::mem::transmute_copy(this), &mut result__).from_abi::<OfflineMapPackageQueryStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Maps_OfflineMaps\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Packages(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<OfflineMapPackage>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Packages)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<OfflineMapPackage>>(result__)
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
impl ::core::fmt::Debug for OfflineMapPackageQueryResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OfflineMapPackageQueryResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for OfflineMapPackageQueryResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.OfflineMaps.OfflineMapPackageQueryResult;{55585411-39e1-4e41-a4e1-5f4872bee199})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for OfflineMapPackageQueryResult {
    type Vtable = IOfflineMapPackageQueryResult_Vtbl;
    const IID: ::windows::core::GUID = <IOfflineMapPackageQueryResult as ::windows::core::Interface>::IID;
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a OfflineMapPackageQueryResult {
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a OfflineMapPackageQueryResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for OfflineMapPackageQueryResult {}
unsafe impl ::core::marker::Sync for OfflineMapPackageQueryResult {}
#[doc = "*Required features: `\"Services_Maps_OfflineMaps\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for OfflineMapPackageQueryStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for OfflineMapPackageQueryStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for OfflineMapPackageQueryStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OfflineMapPackageQueryStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for OfflineMapPackageQueryStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Services.Maps.OfflineMaps.OfflineMapPackageQueryStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Services_Maps_OfflineMaps\"`*"]
#[repr(transparent)]
pub struct OfflineMapPackageStartDownloadResult(::windows::core::IUnknown);
impl OfflineMapPackageStartDownloadResult {
    #[doc = "*Required features: `\"Services_Maps_OfflineMaps\"`*"]
    pub fn Status(&self) -> ::windows::core::Result<OfflineMapPackageStartDownloadStatus> {
        let this = self;
        unsafe {
            let mut result__: OfflineMapPackageStartDownloadStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::core::mem::transmute_copy(this), &mut result__).from_abi::<OfflineMapPackageStartDownloadStatus>(result__)
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
impl ::core::fmt::Debug for OfflineMapPackageStartDownloadResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OfflineMapPackageStartDownloadResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for OfflineMapPackageStartDownloadResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.OfflineMaps.OfflineMapPackageStartDownloadResult;{d965b918-d4d6-4afe-9378-3ec71ef11c3d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for OfflineMapPackageStartDownloadResult {
    type Vtable = IOfflineMapPackageStartDownloadResult_Vtbl;
    const IID: ::windows::core::GUID = <IOfflineMapPackageStartDownloadResult as ::windows::core::Interface>::IID;
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a OfflineMapPackageStartDownloadResult {
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a OfflineMapPackageStartDownloadResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for OfflineMapPackageStartDownloadResult {}
unsafe impl ::core::marker::Sync for OfflineMapPackageStartDownloadResult {}
#[doc = "*Required features: `\"Services_Maps_OfflineMaps\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for OfflineMapPackageStartDownloadStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for OfflineMapPackageStartDownloadStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for OfflineMapPackageStartDownloadStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OfflineMapPackageStartDownloadStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for OfflineMapPackageStartDownloadStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Services.Maps.OfflineMaps.OfflineMapPackageStartDownloadStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Services_Maps_OfflineMaps\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for OfflineMapPackageStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for OfflineMapPackageStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for OfflineMapPackageStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OfflineMapPackageStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for OfflineMapPackageStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Services.Maps.OfflineMaps.OfflineMapPackageStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
