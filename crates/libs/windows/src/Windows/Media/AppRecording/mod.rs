#[doc(hidden)]
#[repr(transparent)]
pub struct IAppRecordingManager(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppRecordingManager {
    type Vtable = IAppRecordingManager_Vtbl;
}
impl ::core::clone::Clone for IAppRecordingManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAppRecordingManager {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe7e26076_a044_48e2_a512_3094d574c7cc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppRecordingManager_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub StartRecordingToFileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    StartRecordingToFileAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub RecordTimeSpanToFileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, starttime: super::super::Foundation::DateTime, duration: super::super::Foundation::TimeSpan, file: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    RecordTimeSpanToFileAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedScreenshotMediaEncodingSubtypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedScreenshotMediaEncodingSubtypes: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub SaveScreenshotToFilesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, folder: *mut ::core::ffi::c_void, filenameprefix: ::std::mem::MaybeUninit<::windows_core::HSTRING>, option: AppRecordingSaveScreenshotOption, requestedformats: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage")))]
    SaveScreenshotToFilesAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppRecordingManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppRecordingManagerStatics {
    type Vtable = IAppRecordingManagerStatics_Vtbl;
}
impl ::core::clone::Clone for IAppRecordingManagerStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAppRecordingManagerStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x50e709f7_38ce_4bd3_9db2_e72bbe9de11d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppRecordingManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppRecordingResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppRecordingResult {
    type Vtable = IAppRecordingResult_Vtbl;
}
impl ::core::clone::Clone for IAppRecordingResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAppRecordingResult {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3a900864_c66d_46f9_b2d9_5bc2dad070d7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppRecordingResult_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Succeeded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Duration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Duration: usize,
    pub IsFileTruncated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppRecordingSaveScreenshotResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppRecordingSaveScreenshotResult {
    type Vtable = IAppRecordingSaveScreenshotResult_Vtbl;
}
impl ::core::clone::Clone for IAppRecordingSaveScreenshotResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAppRecordingSaveScreenshotResult {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9c5b8d0a_0abb_4457_aaee_24f9c12ec778);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppRecordingSaveScreenshotResult_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Succeeded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SavedScreenshotInfos: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SavedScreenshotInfos: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppRecordingSavedScreenshotInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppRecordingSavedScreenshotInfo {
    type Vtable = IAppRecordingSavedScreenshotInfo_Vtbl;
}
impl ::core::clone::Clone for IAppRecordingSavedScreenshotInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAppRecordingSavedScreenshotInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9b642d0a_189a_4d00_bf25_e1bb1249d594);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppRecordingSavedScreenshotInfo_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage")]
    pub File: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    File: usize,
    pub MediaEncodingSubtype: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppRecordingStatus(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppRecordingStatus {
    type Vtable = IAppRecordingStatus_Vtbl;
}
impl ::core::clone::Clone for IAppRecordingStatus {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAppRecordingStatus {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1d0cc82c_bc18_4b8a_a6ef_127efab3b5d9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppRecordingStatus_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CanRecord: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub CanRecordTimeSpan: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub HistoricalBufferDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    HistoricalBufferDuration: usize,
    pub Details: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppRecordingStatusDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppRecordingStatusDetails {
    type Vtable = IAppRecordingStatusDetails_Vtbl;
}
impl ::core::clone::Clone for IAppRecordingStatusDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAppRecordingStatusDetails {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb538a9b0_14ed_4412_ac45_6d672c9c9949);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppRecordingStatusDetails_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsAnyAppBroadcasting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsCaptureResourceUnavailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsGameStreamInProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsTimeSpanRecordingDisabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsGpuConstrained: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsAppInactive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsBlockedForApp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsDisabledByUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsDisabledBySystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Media_AppRecording\"`*"]
#[repr(transparent)]
pub struct AppRecordingManager(::windows_core::IUnknown);
impl AppRecordingManager {
    pub fn GetStatus(&self) -> ::windows_core::Result<AppRecordingStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetStatus)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn StartRecordingToFileAsync<P0>(&self, file: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<AppRecordingResult>>
    where
        P0: ::windows_core::IntoParam<super::super::Storage::StorageFile>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartRecordingToFileAsync)(::windows_core::Interface::as_raw(this), file.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn RecordTimeSpanToFileAsync<P0>(&self, starttime: super::super::Foundation::DateTime, duration: super::super::Foundation::TimeSpan, file: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<AppRecordingResult>>
    where
        P0: ::windows_core::IntoParam<super::super::Storage::StorageFile>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RecordTimeSpanToFileAsync)(::windows_core::Interface::as_raw(this), starttime, duration, file.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedScreenshotMediaEncodingSubtypes(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SupportedScreenshotMediaEncodingSubtypes)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub fn SaveScreenshotToFilesAsync<P0, P1>(&self, folder: P0, filenameprefix: &::windows_core::HSTRING, option: AppRecordingSaveScreenshotOption, requestedformats: P1) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<AppRecordingSaveScreenshotResult>>
    where
        P0: ::windows_core::IntoParam<super::super::Storage::StorageFolder>,
        P1: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SaveScreenshotToFilesAsync)(::windows_core::Interface::as_raw(this), folder.into_param().abi(), ::core::mem::transmute_copy(filenameprefix), option, requestedformats.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn GetDefault() -> ::windows_core::Result<AppRecordingManager> {
        Self::IAppRecordingManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDefault)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAppRecordingManagerStatics<R, F: FnOnce(&IAppRecordingManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<AppRecordingManager, IAppRecordingManagerStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows_core::RuntimeType for AppRecordingManager {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.AppRecording.AppRecordingManager;{e7e26076-a044-48e2-a512-3094d574c7cc})");
}
impl ::core::clone::Clone for AppRecordingManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for AppRecordingManager {
    type Vtable = IAppRecordingManager_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppRecordingManager {
    const IID: ::windows_core::GUID = <IAppRecordingManager as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppRecordingManager {
    const NAME: &'static str = "Windows.Media.AppRecording.AppRecordingManager";
}
::windows_core::imp::interface_hierarchy!(AppRecordingManager, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for AppRecordingManager {}
unsafe impl ::core::marker::Sync for AppRecordingManager {}
#[doc = "*Required features: `\"Media_AppRecording\"`*"]
#[repr(transparent)]
pub struct AppRecordingResult(::windows_core::IUnknown);
impl AppRecordingResult {
    pub fn Succeeded(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Succeeded)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedError)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Duration)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsFileTruncated(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsFileTruncated)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for AppRecordingResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.AppRecording.AppRecordingResult;{3a900864-c66d-46f9-b2d9-5bc2dad070d7})");
}
impl ::core::clone::Clone for AppRecordingResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for AppRecordingResult {
    type Vtable = IAppRecordingResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppRecordingResult {
    const IID: ::windows_core::GUID = <IAppRecordingResult as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppRecordingResult {
    const NAME: &'static str = "Windows.Media.AppRecording.AppRecordingResult";
}
::windows_core::imp::interface_hierarchy!(AppRecordingResult, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for AppRecordingResult {}
unsafe impl ::core::marker::Sync for AppRecordingResult {}
#[doc = "*Required features: `\"Media_AppRecording\"`*"]
#[repr(transparent)]
pub struct AppRecordingSaveScreenshotResult(::windows_core::IUnknown);
impl AppRecordingSaveScreenshotResult {
    pub fn Succeeded(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Succeeded)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedError)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SavedScreenshotInfos(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<AppRecordingSavedScreenshotInfo>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SavedScreenshotInfos)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for AppRecordingSaveScreenshotResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.AppRecording.AppRecordingSaveScreenshotResult;{9c5b8d0a-0abb-4457-aaee-24f9c12ec778})");
}
impl ::core::clone::Clone for AppRecordingSaveScreenshotResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for AppRecordingSaveScreenshotResult {
    type Vtable = IAppRecordingSaveScreenshotResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppRecordingSaveScreenshotResult {
    const IID: ::windows_core::GUID = <IAppRecordingSaveScreenshotResult as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppRecordingSaveScreenshotResult {
    const NAME: &'static str = "Windows.Media.AppRecording.AppRecordingSaveScreenshotResult";
}
::windows_core::imp::interface_hierarchy!(AppRecordingSaveScreenshotResult, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for AppRecordingSaveScreenshotResult {}
unsafe impl ::core::marker::Sync for AppRecordingSaveScreenshotResult {}
#[doc = "*Required features: `\"Media_AppRecording\"`*"]
#[repr(transparent)]
pub struct AppRecordingSavedScreenshotInfo(::windows_core::IUnknown);
impl AppRecordingSavedScreenshotInfo {
    #[doc = "*Required features: `\"Storage\"`*"]
    #[cfg(feature = "Storage")]
    pub fn File(&self) -> ::windows_core::Result<super::super::Storage::StorageFile> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).File)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MediaEncodingSubtype(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MediaEncodingSubtype)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for AppRecordingSavedScreenshotInfo {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.AppRecording.AppRecordingSavedScreenshotInfo;{9b642d0a-189a-4d00-bf25-e1bb1249d594})");
}
impl ::core::clone::Clone for AppRecordingSavedScreenshotInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for AppRecordingSavedScreenshotInfo {
    type Vtable = IAppRecordingSavedScreenshotInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppRecordingSavedScreenshotInfo {
    const IID: ::windows_core::GUID = <IAppRecordingSavedScreenshotInfo as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppRecordingSavedScreenshotInfo {
    const NAME: &'static str = "Windows.Media.AppRecording.AppRecordingSavedScreenshotInfo";
}
::windows_core::imp::interface_hierarchy!(AppRecordingSavedScreenshotInfo, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for AppRecordingSavedScreenshotInfo {}
unsafe impl ::core::marker::Sync for AppRecordingSavedScreenshotInfo {}
#[doc = "*Required features: `\"Media_AppRecording\"`*"]
#[repr(transparent)]
pub struct AppRecordingStatus(::windows_core::IUnknown);
impl AppRecordingStatus {
    pub fn CanRecord(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CanRecord)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CanRecordTimeSpan(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CanRecordTimeSpan)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn HistoricalBufferDuration(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HistoricalBufferDuration)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Details(&self) -> ::windows_core::Result<AppRecordingStatusDetails> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Details)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for AppRecordingStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.AppRecording.AppRecordingStatus;{1d0cc82c-bc18-4b8a-a6ef-127efab3b5d9})");
}
impl ::core::clone::Clone for AppRecordingStatus {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for AppRecordingStatus {
    type Vtable = IAppRecordingStatus_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppRecordingStatus {
    const IID: ::windows_core::GUID = <IAppRecordingStatus as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppRecordingStatus {
    const NAME: &'static str = "Windows.Media.AppRecording.AppRecordingStatus";
}
::windows_core::imp::interface_hierarchy!(AppRecordingStatus, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for AppRecordingStatus {}
unsafe impl ::core::marker::Sync for AppRecordingStatus {}
#[doc = "*Required features: `\"Media_AppRecording\"`*"]
#[repr(transparent)]
pub struct AppRecordingStatusDetails(::windows_core::IUnknown);
impl AppRecordingStatusDetails {
    pub fn IsAnyAppBroadcasting(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsAnyAppBroadcasting)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsCaptureResourceUnavailable(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsCaptureResourceUnavailable)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsGameStreamInProgress(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsGameStreamInProgress)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsTimeSpanRecordingDisabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsTimeSpanRecordingDisabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsGpuConstrained(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsGpuConstrained)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsAppInactive(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsAppInactive)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsBlockedForApp(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsBlockedForApp)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsDisabledByUser(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsDisabledByUser)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsDisabledBySystem(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsDisabledBySystem)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for AppRecordingStatusDetails {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.AppRecording.AppRecordingStatusDetails;{b538a9b0-14ed-4412-ac45-6d672c9c9949})");
}
impl ::core::clone::Clone for AppRecordingStatusDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for AppRecordingStatusDetails {
    type Vtable = IAppRecordingStatusDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppRecordingStatusDetails {
    const IID: ::windows_core::GUID = <IAppRecordingStatusDetails as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppRecordingStatusDetails {
    const NAME: &'static str = "Windows.Media.AppRecording.AppRecordingStatusDetails";
}
::windows_core::imp::interface_hierarchy!(AppRecordingStatusDetails, ::windows_core::IUnknown, ::windows_core::IInspectable);
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
impl ::windows_core::TypeKind for AppRecordingSaveScreenshotOption {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AppRecordingSaveScreenshotOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppRecordingSaveScreenshotOption").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppRecordingSaveScreenshotOption {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.AppRecording.AppRecordingSaveScreenshotOption;i4)");
}
