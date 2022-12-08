#[doc(hidden)]
#[repr(transparent)]
pub struct IAppRecordingManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppRecordingManager {
    type Vtable = IAppRecordingManager_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppRecordingManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe7e26076_a044_48e2_a512_3094d574c7cc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppRecordingManager_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub StartRecordingToFileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    StartRecordingToFileAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub RecordTimeSpanToFileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, starttime: super::super::Foundation::DateTime, duration: super::super::Foundation::TimeSpan, file: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    RecordTimeSpanToFileAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedScreenshotMediaEncodingSubtypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedScreenshotMediaEncodingSubtypes: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub SaveScreenshotToFilesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, folder: *mut ::core::ffi::c_void, filenameprefix: *mut ::core::ffi::c_void, option: AppRecordingSaveScreenshotOption, requestedformats: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage")))]
    SaveScreenshotToFilesAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppRecordingManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppRecordingManagerStatics {
    type Vtable = IAppRecordingManagerStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppRecordingManagerStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x50e709f7_38ce_4bd3_9db2_e72bbe9de11d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppRecordingManagerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppRecordingResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppRecordingResult {
    type Vtable = IAppRecordingResult_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppRecordingResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3a900864_c66d_46f9_b2d9_5bc2dad070d7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppRecordingResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Succeeded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Duration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Duration: usize,
    pub IsFileTruncated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppRecordingSaveScreenshotResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppRecordingSaveScreenshotResult {
    type Vtable = IAppRecordingSaveScreenshotResult_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppRecordingSaveScreenshotResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9c5b8d0a_0abb_4457_aaee_24f9c12ec778);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppRecordingSaveScreenshotResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Succeeded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SavedScreenshotInfos: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SavedScreenshotInfos: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppRecordingSavedScreenshotInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppRecordingSavedScreenshotInfo {
    type Vtable = IAppRecordingSavedScreenshotInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppRecordingSavedScreenshotInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b642d0a_189a_4d00_bf25_e1bb1249d594);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppRecordingSavedScreenshotInfo_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Storage")]
    pub File: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    File: usize,
    pub MediaEncodingSubtype: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppRecordingStatus(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppRecordingStatus {
    type Vtable = IAppRecordingStatus_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppRecordingStatus {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1d0cc82c_bc18_4b8a_a6ef_127efab3b5d9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppRecordingStatus_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CanRecord: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub CanRecordTimeSpan: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub HistoricalBufferDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    HistoricalBufferDuration: usize,
    pub Details: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppRecordingStatusDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppRecordingStatusDetails {
    type Vtable = IAppRecordingStatusDetails_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppRecordingStatusDetails {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb538a9b0_14ed_4412_ac45_6d672c9c9949);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppRecordingStatusDetails_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsAnyAppBroadcasting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsCaptureResourceUnavailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsGameStreamInProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsTimeSpanRecordingDisabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsGpuConstrained: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsAppInactive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsBlockedForApp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsDisabledByUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsDisabledBySystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Media_AppRecording\"`*"]
#[repr(transparent)]
pub struct AppRecordingManager(::windows::core::IUnknown);
impl AppRecordingManager {
    pub fn GetStatus(&self) -> ::windows::core::Result<AppRecordingStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetStatus)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn StartRecordingToFileAsync(&self, file: &super::super::Storage::StorageFile) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AppRecordingResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StartRecordingToFileAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(file), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn RecordTimeSpanToFileAsync(&self, starttime: super::super::Foundation::DateTime, duration: super::super::Foundation::TimeSpan, file: &super::super::Storage::StorageFile) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AppRecordingResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RecordTimeSpanToFileAsync)(::windows::core::Vtable::as_raw(this), starttime, duration, ::core::mem::transmute_copy(file), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedScreenshotMediaEncodingSubtypes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SupportedScreenshotMediaEncodingSubtypes)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub fn SaveScreenshotToFilesAsync<P0, E0>(&self, folder: &super::super::Storage::StorageFolder, filenameprefix: &::windows::core::HSTRING, option: AppRecordingSaveScreenshotOption, requestedformats: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AppRecordingSaveScreenshotResult>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SaveScreenshotToFilesAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(folder), ::core::mem::transmute_copy(filenameprefix), option, requestedformats.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetDefault() -> ::windows::core::Result<AppRecordingManager> {
        Self::IAppRecordingManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDefault)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAppRecordingManagerStatics<R, F: FnOnce(&IAppRecordingManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<AppRecordingManager, IAppRecordingManagerStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for AppRecordingManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppRecordingManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppRecordingManager {}
impl ::core::fmt::Debug for AppRecordingManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppRecordingManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppRecordingManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.AppRecording.AppRecordingManager;{e7e26076-a044-48e2-a512-3094d574c7cc})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for AppRecordingManager {
    type Vtable = IAppRecordingManager_Vtbl;
}
unsafe impl ::windows::core::Interface for AppRecordingManager {
    const IID: ::windows::core::GUID = <IAppRecordingManager as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppRecordingManager {
    const NAME: &'static str = "Windows.Media.AppRecording.AppRecordingManager";
}
::windows::core::interface_hierarchy!(AppRecordingManager, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AppRecordingManager {}
unsafe impl ::core::marker::Sync for AppRecordingManager {}
#[doc = "*Required features: `\"Media_AppRecording\"`*"]
#[repr(transparent)]
pub struct AppRecordingResult(::windows::core::IUnknown);
impl AppRecordingResult {
    pub fn Succeeded(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Succeeded)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExtendedError)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Duration)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsFileTruncated(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsFileTruncated)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for AppRecordingResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppRecordingResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppRecordingResult {}
impl ::core::fmt::Debug for AppRecordingResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppRecordingResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppRecordingResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.AppRecording.AppRecordingResult;{3a900864-c66d-46f9-b2d9-5bc2dad070d7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for AppRecordingResult {
    type Vtable = IAppRecordingResult_Vtbl;
}
unsafe impl ::windows::core::Interface for AppRecordingResult {
    const IID: ::windows::core::GUID = <IAppRecordingResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppRecordingResult {
    const NAME: &'static str = "Windows.Media.AppRecording.AppRecordingResult";
}
::windows::core::interface_hierarchy!(AppRecordingResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AppRecordingResult {}
unsafe impl ::core::marker::Sync for AppRecordingResult {}
#[doc = "*Required features: `\"Media_AppRecording\"`*"]
#[repr(transparent)]
pub struct AppRecordingSaveScreenshotResult(::windows::core::IUnknown);
impl AppRecordingSaveScreenshotResult {
    pub fn Succeeded(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Succeeded)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExtendedError)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SavedScreenshotInfos(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<AppRecordingSavedScreenshotInfo>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SavedScreenshotInfos)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for AppRecordingSaveScreenshotResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppRecordingSaveScreenshotResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppRecordingSaveScreenshotResult {}
impl ::core::fmt::Debug for AppRecordingSaveScreenshotResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppRecordingSaveScreenshotResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppRecordingSaveScreenshotResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.AppRecording.AppRecordingSaveScreenshotResult;{9c5b8d0a-0abb-4457-aaee-24f9c12ec778})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for AppRecordingSaveScreenshotResult {
    type Vtable = IAppRecordingSaveScreenshotResult_Vtbl;
}
unsafe impl ::windows::core::Interface for AppRecordingSaveScreenshotResult {
    const IID: ::windows::core::GUID = <IAppRecordingSaveScreenshotResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppRecordingSaveScreenshotResult {
    const NAME: &'static str = "Windows.Media.AppRecording.AppRecordingSaveScreenshotResult";
}
::windows::core::interface_hierarchy!(AppRecordingSaveScreenshotResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AppRecordingSaveScreenshotResult {}
unsafe impl ::core::marker::Sync for AppRecordingSaveScreenshotResult {}
#[doc = "*Required features: `\"Media_AppRecording\"`*"]
#[repr(transparent)]
pub struct AppRecordingSavedScreenshotInfo(::windows::core::IUnknown);
impl AppRecordingSavedScreenshotInfo {
    #[doc = "*Required features: `\"Storage\"`*"]
    #[cfg(feature = "Storage")]
    pub fn File(&self) -> ::windows::core::Result<super::super::Storage::StorageFile> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).File)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn MediaEncodingSubtype(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MediaEncodingSubtype)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for AppRecordingSavedScreenshotInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppRecordingSavedScreenshotInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppRecordingSavedScreenshotInfo {}
impl ::core::fmt::Debug for AppRecordingSavedScreenshotInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppRecordingSavedScreenshotInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppRecordingSavedScreenshotInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.AppRecording.AppRecordingSavedScreenshotInfo;{9b642d0a-189a-4d00-bf25-e1bb1249d594})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for AppRecordingSavedScreenshotInfo {
    type Vtable = IAppRecordingSavedScreenshotInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for AppRecordingSavedScreenshotInfo {
    const IID: ::windows::core::GUID = <IAppRecordingSavedScreenshotInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppRecordingSavedScreenshotInfo {
    const NAME: &'static str = "Windows.Media.AppRecording.AppRecordingSavedScreenshotInfo";
}
::windows::core::interface_hierarchy!(AppRecordingSavedScreenshotInfo, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AppRecordingSavedScreenshotInfo {}
unsafe impl ::core::marker::Sync for AppRecordingSavedScreenshotInfo {}
#[doc = "*Required features: `\"Media_AppRecording\"`*"]
#[repr(transparent)]
pub struct AppRecordingStatus(::windows::core::IUnknown);
impl AppRecordingStatus {
    pub fn CanRecord(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CanRecord)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CanRecordTimeSpan(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CanRecordTimeSpan)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn HistoricalBufferDuration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HistoricalBufferDuration)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Details(&self) -> ::windows::core::Result<AppRecordingStatusDetails> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Details)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for AppRecordingStatus {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppRecordingStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppRecordingStatus {}
impl ::core::fmt::Debug for AppRecordingStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppRecordingStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppRecordingStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.AppRecording.AppRecordingStatus;{1d0cc82c-bc18-4b8a-a6ef-127efab3b5d9})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for AppRecordingStatus {
    type Vtable = IAppRecordingStatus_Vtbl;
}
unsafe impl ::windows::core::Interface for AppRecordingStatus {
    const IID: ::windows::core::GUID = <IAppRecordingStatus as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppRecordingStatus {
    const NAME: &'static str = "Windows.Media.AppRecording.AppRecordingStatus";
}
::windows::core::interface_hierarchy!(AppRecordingStatus, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AppRecordingStatus {}
unsafe impl ::core::marker::Sync for AppRecordingStatus {}
#[doc = "*Required features: `\"Media_AppRecording\"`*"]
#[repr(transparent)]
pub struct AppRecordingStatusDetails(::windows::core::IUnknown);
impl AppRecordingStatusDetails {
    pub fn IsAnyAppBroadcasting(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsAnyAppBroadcasting)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsCaptureResourceUnavailable(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsCaptureResourceUnavailable)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsGameStreamInProgress(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsGameStreamInProgress)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsTimeSpanRecordingDisabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsTimeSpanRecordingDisabled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsGpuConstrained(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsGpuConstrained)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsAppInactive(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsAppInactive)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsBlockedForApp(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsBlockedForApp)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsDisabledByUser(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsDisabledByUser)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsDisabledBySystem(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsDisabledBySystem)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for AppRecordingStatusDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppRecordingStatusDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppRecordingStatusDetails {}
impl ::core::fmt::Debug for AppRecordingStatusDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppRecordingStatusDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppRecordingStatusDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.AppRecording.AppRecordingStatusDetails;{b538a9b0-14ed-4412-ac45-6d672c9c9949})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for AppRecordingStatusDetails {
    type Vtable = IAppRecordingStatusDetails_Vtbl;
}
unsafe impl ::windows::core::Interface for AppRecordingStatusDetails {
    const IID: ::windows::core::GUID = <IAppRecordingStatusDetails as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppRecordingStatusDetails {
    const NAME: &'static str = "Windows.Media.AppRecording.AppRecordingStatusDetails";
}
::windows::core::interface_hierarchy!(AppRecordingStatusDetails, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AppRecordingStatusDetails {}
unsafe impl ::core::marker::Sync for AppRecordingStatusDetails {}
#[doc = "*Required features: `\"Media_AppRecording\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AppRecordingSaveScreenshotOption(pub i32);
impl AppRecordingSaveScreenshotOption {
    pub const None: Self = Self(0i32);
    pub const HdrContentVisible: Self = Self(1i32);
}
impl ::core::marker::Copy for AppRecordingSaveScreenshotOption {}
impl ::core::clone::Clone for AppRecordingSaveScreenshotOption {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppRecordingSaveScreenshotOption {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AppRecordingSaveScreenshotOption {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppRecordingSaveScreenshotOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppRecordingSaveScreenshotOption").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppRecordingSaveScreenshotOption {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.AppRecording.AppRecordingSaveScreenshotOption;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
