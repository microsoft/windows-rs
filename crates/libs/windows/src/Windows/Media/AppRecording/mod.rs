#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Media_AppRecording\"`*"]
#[repr(transparent)]
pub struct AppRecordingManager(::windows::core::IUnknown);
impl AppRecordingManager {
    #[doc = "*Required features: `\"Media_AppRecording\"`*"]
    pub fn GetStatus(&self) -> ::windows::core::Result<AppRecordingStatus> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetStatus)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppRecordingStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_AppRecording\"`, `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn StartRecordingToFileAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::StorageFile>>(&self, file: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AppRecordingResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StartRecordingToFileAsync)(::core::mem::transmute_copy(this), file.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<AppRecordingResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_AppRecording\"`, `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn RecordTimeSpanToFileAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::DateTime>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>, Param2: ::windows::core::IntoParam<'a, super::super::Storage::StorageFile>>(&self, starttime: Param0, duration: Param1, file: Param2) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AppRecordingResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RecordTimeSpanToFileAsync)(::core::mem::transmute_copy(this), starttime.into_param().abi(), duration.into_param().abi(), file.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<AppRecordingResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_AppRecording\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedScreenshotMediaEncodingSubtypes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SupportedScreenshotMediaEncodingSubtypes)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_AppRecording\"`, `\"Foundation_Collections\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub fn SaveScreenshotToFilesAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::StorageFolder>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(&self, folder: Param0, filenameprefix: Param1, option: AppRecordingSaveScreenshotOption, requestedformats: Param3) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AppRecordingSaveScreenshotResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SaveScreenshotToFilesAsync)(::core::mem::transmute_copy(this), folder.into_param().abi(), filenameprefix.into_param().abi(), option, requestedformats.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<AppRecordingSaveScreenshotResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_AppRecording\"`*"]
    pub fn GetDefault() -> ::windows::core::Result<AppRecordingManager> {
        Self::IAppRecordingManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDefault)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppRecordingManager>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAppRecordingManagerStatics<R, F: FnOnce(&IAppRecordingManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AppRecordingManager, IAppRecordingManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
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
unsafe impl ::windows::core::Interface for AppRecordingManager {
    type Vtable = IAppRecordingManager_Vtbl;
    const IID: ::windows::core::GUID = <IAppRecordingManager as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppRecordingManager {
    const NAME: &'static str = "Windows.Media.AppRecording.AppRecordingManager";
}
impl ::core::convert::From<AppRecordingManager> for ::windows::core::IUnknown {
    fn from(value: AppRecordingManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppRecordingManager> for ::windows::core::IUnknown {
    fn from(value: &AppRecordingManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppRecordingManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppRecordingManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppRecordingManager> for ::windows::core::IInspectable {
    fn from(value: AppRecordingManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppRecordingManager> for ::windows::core::IInspectable {
    fn from(value: &AppRecordingManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppRecordingManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppRecordingManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppRecordingManager {}
unsafe impl ::core::marker::Sync for AppRecordingManager {}
#[doc = "*Required features: `\"Media_AppRecording\"`*"]
#[repr(transparent)]
pub struct AppRecordingResult(::windows::core::IUnknown);
impl AppRecordingResult {
    #[doc = "*Required features: `\"Media_AppRecording\"`*"]
    pub fn Succeeded(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Succeeded)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_AppRecording\"`*"]
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExtendedError)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_AppRecording\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Duration)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_AppRecording\"`*"]
    pub fn IsFileTruncated(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsFileTruncated)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
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
unsafe impl ::windows::core::Interface for AppRecordingResult {
    type Vtable = IAppRecordingResult_Vtbl;
    const IID: ::windows::core::GUID = <IAppRecordingResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppRecordingResult {
    const NAME: &'static str = "Windows.Media.AppRecording.AppRecordingResult";
}
impl ::core::convert::From<AppRecordingResult> for ::windows::core::IUnknown {
    fn from(value: AppRecordingResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppRecordingResult> for ::windows::core::IUnknown {
    fn from(value: &AppRecordingResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppRecordingResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppRecordingResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppRecordingResult> for ::windows::core::IInspectable {
    fn from(value: AppRecordingResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppRecordingResult> for ::windows::core::IInspectable {
    fn from(value: &AppRecordingResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppRecordingResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppRecordingResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppRecordingResult {}
unsafe impl ::core::marker::Sync for AppRecordingResult {}
#[doc = "*Required features: `\"Media_AppRecording\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
#[doc = "*Required features: `\"Media_AppRecording\"`*"]
#[repr(transparent)]
pub struct AppRecordingSaveScreenshotResult(::windows::core::IUnknown);
impl AppRecordingSaveScreenshotResult {
    #[doc = "*Required features: `\"Media_AppRecording\"`*"]
    pub fn Succeeded(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Succeeded)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_AppRecording\"`*"]
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExtendedError)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_AppRecording\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SavedScreenshotInfos(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<AppRecordingSavedScreenshotInfo>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SavedScreenshotInfos)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<AppRecordingSavedScreenshotInfo>>(result__)
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
unsafe impl ::windows::core::Interface for AppRecordingSaveScreenshotResult {
    type Vtable = IAppRecordingSaveScreenshotResult_Vtbl;
    const IID: ::windows::core::GUID = <IAppRecordingSaveScreenshotResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppRecordingSaveScreenshotResult {
    const NAME: &'static str = "Windows.Media.AppRecording.AppRecordingSaveScreenshotResult";
}
impl ::core::convert::From<AppRecordingSaveScreenshotResult> for ::windows::core::IUnknown {
    fn from(value: AppRecordingSaveScreenshotResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppRecordingSaveScreenshotResult> for ::windows::core::IUnknown {
    fn from(value: &AppRecordingSaveScreenshotResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppRecordingSaveScreenshotResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppRecordingSaveScreenshotResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppRecordingSaveScreenshotResult> for ::windows::core::IInspectable {
    fn from(value: AppRecordingSaveScreenshotResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppRecordingSaveScreenshotResult> for ::windows::core::IInspectable {
    fn from(value: &AppRecordingSaveScreenshotResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppRecordingSaveScreenshotResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppRecordingSaveScreenshotResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppRecordingSaveScreenshotResult {}
unsafe impl ::core::marker::Sync for AppRecordingSaveScreenshotResult {}
#[doc = "*Required features: `\"Media_AppRecording\"`*"]
#[repr(transparent)]
pub struct AppRecordingSavedScreenshotInfo(::windows::core::IUnknown);
impl AppRecordingSavedScreenshotInfo {
    #[doc = "*Required features: `\"Media_AppRecording\"`, `\"Storage\"`*"]
    #[cfg(feature = "Storage")]
    pub fn File(&self) -> ::windows::core::Result<super::super::Storage::StorageFile> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).File)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::StorageFile>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_AppRecording\"`*"]
    pub fn MediaEncodingSubtype(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MediaEncodingSubtype)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
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
unsafe impl ::windows::core::Interface for AppRecordingSavedScreenshotInfo {
    type Vtable = IAppRecordingSavedScreenshotInfo_Vtbl;
    const IID: ::windows::core::GUID = <IAppRecordingSavedScreenshotInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppRecordingSavedScreenshotInfo {
    const NAME: &'static str = "Windows.Media.AppRecording.AppRecordingSavedScreenshotInfo";
}
impl ::core::convert::From<AppRecordingSavedScreenshotInfo> for ::windows::core::IUnknown {
    fn from(value: AppRecordingSavedScreenshotInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppRecordingSavedScreenshotInfo> for ::windows::core::IUnknown {
    fn from(value: &AppRecordingSavedScreenshotInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppRecordingSavedScreenshotInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppRecordingSavedScreenshotInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppRecordingSavedScreenshotInfo> for ::windows::core::IInspectable {
    fn from(value: AppRecordingSavedScreenshotInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppRecordingSavedScreenshotInfo> for ::windows::core::IInspectable {
    fn from(value: &AppRecordingSavedScreenshotInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppRecordingSavedScreenshotInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppRecordingSavedScreenshotInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppRecordingSavedScreenshotInfo {}
unsafe impl ::core::marker::Sync for AppRecordingSavedScreenshotInfo {}
#[doc = "*Required features: `\"Media_AppRecording\"`*"]
#[repr(transparent)]
pub struct AppRecordingStatus(::windows::core::IUnknown);
impl AppRecordingStatus {
    #[doc = "*Required features: `\"Media_AppRecording\"`*"]
    pub fn CanRecord(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CanRecord)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_AppRecording\"`*"]
    pub fn CanRecordTimeSpan(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CanRecordTimeSpan)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_AppRecording\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn HistoricalBufferDuration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HistoricalBufferDuration)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_AppRecording\"`*"]
    pub fn Details(&self) -> ::windows::core::Result<AppRecordingStatusDetails> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Details)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppRecordingStatusDetails>(result__)
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
unsafe impl ::windows::core::Interface for AppRecordingStatus {
    type Vtable = IAppRecordingStatus_Vtbl;
    const IID: ::windows::core::GUID = <IAppRecordingStatus as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppRecordingStatus {
    const NAME: &'static str = "Windows.Media.AppRecording.AppRecordingStatus";
}
impl ::core::convert::From<AppRecordingStatus> for ::windows::core::IUnknown {
    fn from(value: AppRecordingStatus) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppRecordingStatus> for ::windows::core::IUnknown {
    fn from(value: &AppRecordingStatus) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppRecordingStatus {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppRecordingStatus {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppRecordingStatus> for ::windows::core::IInspectable {
    fn from(value: AppRecordingStatus) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppRecordingStatus> for ::windows::core::IInspectable {
    fn from(value: &AppRecordingStatus) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppRecordingStatus {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppRecordingStatus {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppRecordingStatus {}
unsafe impl ::core::marker::Sync for AppRecordingStatus {}
#[doc = "*Required features: `\"Media_AppRecording\"`*"]
#[repr(transparent)]
pub struct AppRecordingStatusDetails(::windows::core::IUnknown);
impl AppRecordingStatusDetails {
    #[doc = "*Required features: `\"Media_AppRecording\"`*"]
    pub fn IsAnyAppBroadcasting(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsAnyAppBroadcasting)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_AppRecording\"`*"]
    pub fn IsCaptureResourceUnavailable(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsCaptureResourceUnavailable)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_AppRecording\"`*"]
    pub fn IsGameStreamInProgress(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsGameStreamInProgress)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_AppRecording\"`*"]
    pub fn IsTimeSpanRecordingDisabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsTimeSpanRecordingDisabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_AppRecording\"`*"]
    pub fn IsGpuConstrained(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsGpuConstrained)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_AppRecording\"`*"]
    pub fn IsAppInactive(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsAppInactive)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_AppRecording\"`*"]
    pub fn IsBlockedForApp(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsBlockedForApp)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_AppRecording\"`*"]
    pub fn IsDisabledByUser(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsDisabledByUser)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_AppRecording\"`*"]
    pub fn IsDisabledBySystem(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsDisabledBySystem)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
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
unsafe impl ::windows::core::Interface for AppRecordingStatusDetails {
    type Vtable = IAppRecordingStatusDetails_Vtbl;
    const IID: ::windows::core::GUID = <IAppRecordingStatusDetails as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppRecordingStatusDetails {
    const NAME: &'static str = "Windows.Media.AppRecording.AppRecordingStatusDetails";
}
impl ::core::convert::From<AppRecordingStatusDetails> for ::windows::core::IUnknown {
    fn from(value: AppRecordingStatusDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppRecordingStatusDetails> for ::windows::core::IUnknown {
    fn from(value: &AppRecordingStatusDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppRecordingStatusDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppRecordingStatusDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppRecordingStatusDetails> for ::windows::core::IInspectable {
    fn from(value: AppRecordingStatusDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppRecordingStatusDetails> for ::windows::core::IInspectable {
    fn from(value: &AppRecordingStatusDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppRecordingStatusDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppRecordingStatusDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppRecordingStatusDetails {}
unsafe impl ::core::marker::Sync for AppRecordingStatusDetails {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppRecordingManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppRecordingManager {
    type Vtable = IAppRecordingManager_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe7e26076_a044_48e2_a512_3094d574c7cc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppRecordingManager_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub StartRecordingToFileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    StartRecordingToFileAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub RecordTimeSpanToFileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, starttime: super::super::Foundation::DateTime, duration: super::super::Foundation::TimeSpan, file: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    RecordTimeSpanToFileAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedScreenshotMediaEncodingSubtypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedScreenshotMediaEncodingSubtypes: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub SaveScreenshotToFilesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, folder: ::windows::core::RawPtr, filenameprefix: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, option: AppRecordingSaveScreenshotOption, requestedformats: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage")))]
    SaveScreenshotToFilesAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppRecordingManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppRecordingManagerStatics {
    type Vtable = IAppRecordingManagerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x50e709f7_38ce_4bd3_9db2_e72bbe9de11d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppRecordingManagerStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppRecordingResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppRecordingResult {
    type Vtable = IAppRecordingResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3a900864_c66d_46f9_b2d9_5bc2dad070d7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppRecordingResult_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
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
unsafe impl ::windows::core::Interface for IAppRecordingSaveScreenshotResult {
    type Vtable = IAppRecordingSaveScreenshotResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9c5b8d0a_0abb_4457_aaee_24f9c12ec778);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppRecordingSaveScreenshotResult_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Succeeded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SavedScreenshotInfos: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SavedScreenshotInfos: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppRecordingSavedScreenshotInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppRecordingSavedScreenshotInfo {
    type Vtable = IAppRecordingSavedScreenshotInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b642d0a_189a_4d00_bf25_e1bb1249d594);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppRecordingSavedScreenshotInfo_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Storage")]
    pub File: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    File: usize,
    pub MediaEncodingSubtype: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppRecordingStatus(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppRecordingStatus {
    type Vtable = IAppRecordingStatus_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1d0cc82c_bc18_4b8a_a6ef_127efab3b5d9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppRecordingStatus_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CanRecord: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub CanRecordTimeSpan: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub HistoricalBufferDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    HistoricalBufferDuration: usize,
    pub Details: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppRecordingStatusDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppRecordingStatusDetails {
    type Vtable = IAppRecordingStatusDetails_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb538a9b0_14ed_4412_ac45_6d672c9c9949);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppRecordingStatusDetails_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
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
#[cfg(feature = "implement")]
::core::include!("impl.rs");
