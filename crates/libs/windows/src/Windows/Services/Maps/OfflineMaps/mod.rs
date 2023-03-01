#[doc(hidden)]
#[repr(transparent)]
pub struct IOfflineMapPackage(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IOfflineMapPackage {
    type Vtable = IOfflineMapPackage_Vtbl;
}
impl ::core::clone::Clone for IOfflineMapPackage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IOfflineMapPackage {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa797673b_a5b5_4144_b525_e68c8862664b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineMapPackage_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut OfflineMapPackageStatus) -> ::windows::core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub EnclosingRegionName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub EstimatedSizeInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RemoveStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub StatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RequestStartDownloadAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestStartDownloadAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IOfflineMapPackageQueryResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IOfflineMapPackageQueryResult {
    type Vtable = IOfflineMapPackageQueryResult_Vtbl;
}
impl ::core::clone::Clone for IOfflineMapPackageQueryResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IOfflineMapPackageQueryResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x55585411_39e1_4e41_a4e1_5f4872bee199);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineMapPackageQueryResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut OfflineMapPackageQueryStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Packages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Packages: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IOfflineMapPackageStartDownloadResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IOfflineMapPackageStartDownloadResult {
    type Vtable = IOfflineMapPackageStartDownloadResult_Vtbl;
}
impl ::core::clone::Clone for IOfflineMapPackageStartDownloadResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IOfflineMapPackageStartDownloadResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd965b918_d4d6_4afe_9378_3ec71ef11c3d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineMapPackageStartDownloadResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut OfflineMapPackageStartDownloadStatus) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IOfflineMapPackageStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IOfflineMapPackageStatics {
    type Vtable = IOfflineMapPackageStatics_Vtbl;
}
impl ::core::clone::Clone for IOfflineMapPackageStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IOfflineMapPackageStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x185e7922_a831_4ab0_941f_6998fa929285);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOfflineMapPackageStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub FindPackagesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, querypoint: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))]
    FindPackagesAsync: usize,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub FindPackagesInBoundingBoxAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, queryboundingbox: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))]
    FindPackagesInBoundingBoxAsync: usize,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub FindPackagesInGeocircleAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, querycircle: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))]
    FindPackagesInGeocircleAsync: usize,
}
#[doc = "*Required features: `\"Services_Maps_OfflineMaps\"`*"]
#[repr(transparent)]
pub struct OfflineMapPackage(::windows::core::IUnknown);
impl OfflineMapPackage {
    pub fn Status(&self) -> ::windows::core::Result<OfflineMapPackageStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<OfflineMapPackageStatus>();
            (::windows::core::Interface::vtable(this).Status)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).DisplayName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn EnclosingRegionName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).EnclosingRegionName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn EstimatedSizeInBytes(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u64>();
            (::windows::core::Interface::vtable(this).EstimatedSizeInBytes)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStatusChanged(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveStatusChanged)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StatusChanged(&self, value: &super::super::super::Foundation::TypedEventHandler<OfflineMapPackage, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).StatusChanged)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestStartDownloadAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<OfflineMapPackageStartDownloadResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::IAsyncOperation<OfflineMapPackageStartDownloadResult>>();
            (::windows::core::Interface::vtable(this).RequestStartDownloadAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Geolocation\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub fn FindPackagesAsync(querypoint: &super::super::super::Devices::Geolocation::Geopoint) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<OfflineMapPackageQueryResult>> {
        Self::IOfflineMapPackageStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::IAsyncOperation<OfflineMapPackageQueryResult>>();
            (::windows::core::Interface::vtable(this).FindPackagesAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(querypoint), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Geolocation\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub fn FindPackagesInBoundingBoxAsync(queryboundingbox: &super::super::super::Devices::Geolocation::GeoboundingBox) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<OfflineMapPackageQueryResult>> {
        Self::IOfflineMapPackageStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::IAsyncOperation<OfflineMapPackageQueryResult>>();
            (::windows::core::Interface::vtable(this).FindPackagesInBoundingBoxAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(queryboundingbox), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Geolocation\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub fn FindPackagesInGeocircleAsync(querycircle: &super::super::super::Devices::Geolocation::Geocircle) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<OfflineMapPackageQueryResult>> {
        Self::IOfflineMapPackageStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::IAsyncOperation<OfflineMapPackageQueryResult>>();
            (::windows::core::Interface::vtable(this).FindPackagesInGeocircleAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(querycircle), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IOfflineMapPackageStatics<R, F: FnOnce(&IOfflineMapPackageStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<OfflineMapPackage, IOfflineMapPackageStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows::core::RuntimeType for OfflineMapPackage {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.OfflineMaps.OfflineMapPackage;{a797673b-a5b5-4144-b525-e68c8862664b})");
}
impl ::core::clone::Clone for OfflineMapPackage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for OfflineMapPackage {
    type Vtable = IOfflineMapPackage_Vtbl;
}
unsafe impl ::windows::core::ComInterface for OfflineMapPackage {
    const IID: ::windows::core::GUID = <IOfflineMapPackage as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for OfflineMapPackage {
    const NAME: &'static str = "Windows.Services.Maps.OfflineMaps.OfflineMapPackage";
}
::windows::imp::interface_hierarchy!(OfflineMapPackage, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for OfflineMapPackage {}
unsafe impl ::core::marker::Sync for OfflineMapPackage {}
#[doc = "*Required features: `\"Services_Maps_OfflineMaps\"`*"]
#[repr(transparent)]
pub struct OfflineMapPackageQueryResult(::windows::core::IUnknown);
impl OfflineMapPackageQueryResult {
    pub fn Status(&self) -> ::windows::core::Result<OfflineMapPackageQueryStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<OfflineMapPackageQueryStatus>();
            (::windows::core::Interface::vtable(this).Status)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Packages(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<OfflineMapPackage>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::Collections::IVectorView<OfflineMapPackage>>();
            (::windows::core::Interface::vtable(this).Packages)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows::core::RuntimeType for OfflineMapPackageQueryResult {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.OfflineMaps.OfflineMapPackageQueryResult;{55585411-39e1-4e41-a4e1-5f4872bee199})");
}
impl ::core::clone::Clone for OfflineMapPackageQueryResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for OfflineMapPackageQueryResult {
    type Vtable = IOfflineMapPackageQueryResult_Vtbl;
}
unsafe impl ::windows::core::ComInterface for OfflineMapPackageQueryResult {
    const IID: ::windows::core::GUID = <IOfflineMapPackageQueryResult as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for OfflineMapPackageQueryResult {
    const NAME: &'static str = "Windows.Services.Maps.OfflineMaps.OfflineMapPackageQueryResult";
}
::windows::imp::interface_hierarchy!(OfflineMapPackageQueryResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for OfflineMapPackageQueryResult {}
unsafe impl ::core::marker::Sync for OfflineMapPackageQueryResult {}
#[doc = "*Required features: `\"Services_Maps_OfflineMaps\"`*"]
#[repr(transparent)]
pub struct OfflineMapPackageStartDownloadResult(::windows::core::IUnknown);
impl OfflineMapPackageStartDownloadResult {
    pub fn Status(&self) -> ::windows::core::Result<OfflineMapPackageStartDownloadStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<OfflineMapPackageStartDownloadStatus>();
            (::windows::core::Interface::vtable(this).Status)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows::core::RuntimeType for OfflineMapPackageStartDownloadResult {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.OfflineMaps.OfflineMapPackageStartDownloadResult;{d965b918-d4d6-4afe-9378-3ec71ef11c3d})");
}
impl ::core::clone::Clone for OfflineMapPackageStartDownloadResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for OfflineMapPackageStartDownloadResult {
    type Vtable = IOfflineMapPackageStartDownloadResult_Vtbl;
}
unsafe impl ::windows::core::ComInterface for OfflineMapPackageStartDownloadResult {
    const IID: ::windows::core::GUID = <IOfflineMapPackageStartDownloadResult as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for OfflineMapPackageStartDownloadResult {
    const NAME: &'static str = "Windows.Services.Maps.OfflineMaps.OfflineMapPackageStartDownloadResult";
}
::windows::imp::interface_hierarchy!(OfflineMapPackageStartDownloadResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for OfflineMapPackageStartDownloadResult {}
unsafe impl ::core::marker::Sync for OfflineMapPackageStartDownloadResult {}
#[doc = "*Required features: `\"Services_Maps_OfflineMaps\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::windows::core::TypeKind for OfflineMapPackageQueryStatus {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for OfflineMapPackageQueryStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OfflineMapPackageQueryStatus").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for OfflineMapPackageQueryStatus {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Services.Maps.OfflineMaps.OfflineMapPackageQueryStatus;i4)");
}
#[doc = "*Required features: `\"Services_Maps_OfflineMaps\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::windows::core::TypeKind for OfflineMapPackageStartDownloadStatus {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for OfflineMapPackageStartDownloadStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OfflineMapPackageStartDownloadStatus").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for OfflineMapPackageStartDownloadStatus {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Services.Maps.OfflineMaps.OfflineMapPackageStartDownloadStatus;i4)");
}
#[doc = "*Required features: `\"Services_Maps_OfflineMaps\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::windows::core::TypeKind for OfflineMapPackageStatus {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for OfflineMapPackageStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OfflineMapPackageStatus").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for OfflineMapPackageStatus {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Services.Maps.OfflineMaps.OfflineMapPackageStatus;i4)");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
