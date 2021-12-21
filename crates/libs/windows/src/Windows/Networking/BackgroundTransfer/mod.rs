#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[repr(C)]
#[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
pub struct BackgroundDownloadProgress {
    pub BytesReceived: u64,
    pub TotalBytesToReceive: u64,
    pub Status: BackgroundTransferStatus,
    pub HasResponseChanged: bool,
    pub HasRestarted: bool,
}
impl ::core::marker::Copy for BackgroundDownloadProgress {}
impl ::core::clone::Clone for BackgroundDownloadProgress {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for BackgroundDownloadProgress {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BackgroundDownloadProgress").field("BytesReceived", &self.BytesReceived).field("TotalBytesToReceive", &self.TotalBytesToReceive).field("Status", &self.Status).field("HasResponseChanged", &self.HasResponseChanged).field("HasRestarted", &self.HasRestarted).finish()
    }
}
unsafe impl ::windows::core::Abi for BackgroundDownloadProgress {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for BackgroundDownloadProgress {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Networking.BackgroundTransfer.BackgroundDownloadProgress;u8;u8;enum(Windows.Networking.BackgroundTransfer.BackgroundTransferStatus;i4);b1;b1)");
}
impl ::windows::core::DefaultType for BackgroundDownloadProgress {
    type DefaultType = Self;
}
impl ::core::cmp::PartialEq for BackgroundDownloadProgress {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<BackgroundDownloadProgress>()) == 0 }
    }
}
impl ::core::cmp::Eq for BackgroundDownloadProgress {}
impl ::core::default::Default for BackgroundDownloadProgress {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
#[repr(transparent)]
pub struct BackgroundDownloader(::windows::core::IUnknown);
impl BackgroundDownloader {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BackgroundDownloader, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer', 'Foundation', 'Storage'*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn CreateDownload<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>, Param1: ::windows::core::IntoParam<'a, super::super::Storage::IStorageFile>>(&self, uri: Param0, resultfile: Param1) -> ::windows::core::Result<DownloadOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), uri.into_param().abi(), resultfile.into_param().abi(), &mut result__).from_abi::<DownloadOperation>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer', 'Foundation', 'Storage'*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn CreateDownloadFromFile<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>, Param1: ::windows::core::IntoParam<'a, super::super::Storage::IStorageFile>, Param2: ::windows::core::IntoParam<'a, super::super::Storage::IStorageFile>>(&self, uri: Param0, resultfile: Param1, requestbodyfile: Param2) -> ::windows::core::Result<DownloadOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), uri.into_param().abi(), resultfile.into_param().abi(), requestbodyfile.into_param().abi(), &mut result__).from_abi::<DownloadOperation>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer', 'Foundation', 'Storage', 'Storage_Streams'*"]
    #[cfg(all(feature = "Foundation", feature = "Storage", feature = "Storage_Streams"))]
    pub fn CreateDownloadAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>, Param1: ::windows::core::IntoParam<'a, super::super::Storage::IStorageFile>, Param2: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IInputStream>>(&self, uri: Param0, resultfile: Param1, requestbodystream: Param2) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DownloadOperation>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), uri.into_param().abi(), resultfile.into_param().abi(), requestbodystream.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<DownloadOperation>>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
    pub fn TransferGroup(&self) -> ::windows::core::Result<BackgroundTransferGroup> {
        let this = &::windows::core::Interface::cast::<IBackgroundDownloader2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BackgroundTransferGroup>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
    pub fn SetTransferGroup<'a, Param0: ::windows::core::IntoParam<'a, BackgroundTransferGroup>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBackgroundDownloader2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer', 'UI_Notifications'*"]
    #[cfg(feature = "UI_Notifications")]
    pub fn SuccessToastNotification(&self) -> ::windows::core::Result<super::super::UI::Notifications::ToastNotification> {
        let this = &::windows::core::Interface::cast::<IBackgroundDownloader2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::Notifications::ToastNotification>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer', 'UI_Notifications'*"]
    #[cfg(feature = "UI_Notifications")]
    pub fn SetSuccessToastNotification<'a, Param0: ::windows::core::IntoParam<'a, super::super::UI::Notifications::ToastNotification>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBackgroundDownloader2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer', 'UI_Notifications'*"]
    #[cfg(feature = "UI_Notifications")]
    pub fn FailureToastNotification(&self) -> ::windows::core::Result<super::super::UI::Notifications::ToastNotification> {
        let this = &::windows::core::Interface::cast::<IBackgroundDownloader2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::Notifications::ToastNotification>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer', 'UI_Notifications'*"]
    #[cfg(feature = "UI_Notifications")]
    pub fn SetFailureToastNotification<'a, Param0: ::windows::core::IntoParam<'a, super::super::UI::Notifications::ToastNotification>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBackgroundDownloader2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer', 'UI_Notifications'*"]
    #[cfg(feature = "UI_Notifications")]
    pub fn SuccessTileNotification(&self) -> ::windows::core::Result<super::super::UI::Notifications::TileNotification> {
        let this = &::windows::core::Interface::cast::<IBackgroundDownloader2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::Notifications::TileNotification>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer', 'UI_Notifications'*"]
    #[cfg(feature = "UI_Notifications")]
    pub fn SetSuccessTileNotification<'a, Param0: ::windows::core::IntoParam<'a, super::super::UI::Notifications::TileNotification>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBackgroundDownloader2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer', 'UI_Notifications'*"]
    #[cfg(feature = "UI_Notifications")]
    pub fn FailureTileNotification(&self) -> ::windows::core::Result<super::super::UI::Notifications::TileNotification> {
        let this = &::windows::core::Interface::cast::<IBackgroundDownloader2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::Notifications::TileNotification>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer', 'UI_Notifications'*"]
    #[cfg(feature = "UI_Notifications")]
    pub fn SetFailureTileNotification<'a, Param0: ::windows::core::IntoParam<'a, super::super::UI::Notifications::TileNotification>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBackgroundDownloader2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
    pub fn CompletionGroup(&self) -> ::windows::core::Result<BackgroundTransferCompletionGroup> {
        let this = &::windows::core::Interface::cast::<IBackgroundDownloader3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BackgroundTransferCompletionGroup>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
    pub fn CreateWithCompletionGroup<'a, Param0: ::windows::core::IntoParam<'a, BackgroundTransferCompletionGroup>>(completiongroup: Param0) -> ::windows::core::Result<BackgroundDownloader> {
        Self::IBackgroundDownloaderFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), completiongroup.into_param().abi(), &mut result__).from_abi::<BackgroundDownloader>(result__)
        })
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer', 'Foundation', 'Foundation_Collections'*"]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn GetCurrentDownloadsAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<DownloadOperation>>> {
        Self::IBackgroundDownloaderStaticMethods(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<DownloadOperation>>>(result__)
        })
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer', 'Foundation', 'Foundation_Collections', 'deprecated'*"]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn GetCurrentDownloadsForGroupAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(group: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<DownloadOperation>>> {
        Self::IBackgroundDownloaderStaticMethods(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), group.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<DownloadOperation>>>(result__)
        })
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer', 'Foundation', 'Foundation_Collections'*"]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn GetCurrentDownloadsForTransferGroupAsync<'a, Param0: ::windows::core::IntoParam<'a, BackgroundTransferGroup>>(group: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<DownloadOperation>>> {
        Self::IBackgroundDownloaderStaticMethods2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), group.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<DownloadOperation>>>(result__)
        })
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer', 'Foundation', 'Foundation_Collections', 'deprecated'*"]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn RequestUnconstrainedDownloadsAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<DownloadOperation>>>(operations: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<UnconstrainedTransferRequestResult>> {
        Self::IBackgroundDownloaderUserConsent(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), operations.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<UnconstrainedTransferRequestResult>>(result__)
        })
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
    pub fn SetRequestHeader<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, headername: Param0, headervalue: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBackgroundTransferBase>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), headername.into_param().abi(), headervalue.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer', 'Security_Credentials'*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn ServerCredential(&self) -> ::windows::core::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = &::windows::core::Interface::cast::<IBackgroundTransferBase>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Security::Credentials::PasswordCredential>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer', 'Security_Credentials'*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn SetServerCredential<'a, Param0: ::windows::core::IntoParam<'a, super::super::Security::Credentials::PasswordCredential>>(&self, credential: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBackgroundTransferBase>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), credential.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer', 'Security_Credentials'*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn ProxyCredential(&self) -> ::windows::core::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = &::windows::core::Interface::cast::<IBackgroundTransferBase>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Security::Credentials::PasswordCredential>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer', 'Security_Credentials'*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn SetProxyCredential<'a, Param0: ::windows::core::IntoParam<'a, super::super::Security::Credentials::PasswordCredential>>(&self, credential: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBackgroundTransferBase>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), credential.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
    pub fn Method(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IBackgroundTransferBase>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
    pub fn SetMethod<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBackgroundTransferBase>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer', 'deprecated'*"]
    #[cfg(feature = "deprecated")]
    pub fn Group(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IBackgroundTransferBase>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer', 'deprecated'*"]
    #[cfg(feature = "deprecated")]
    pub fn SetGroup<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBackgroundTransferBase>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
    pub fn CostPolicy(&self) -> ::windows::core::Result<BackgroundTransferCostPolicy> {
        let this = &::windows::core::Interface::cast::<IBackgroundTransferBase>(self)?;
        unsafe {
            let mut result__: BackgroundTransferCostPolicy = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BackgroundTransferCostPolicy>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
    pub fn SetCostPolicy(&self, value: BackgroundTransferCostPolicy) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBackgroundTransferBase>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc(hidden)]
    pub fn IBackgroundDownloaderFactory<R, F: FnOnce(&IBackgroundDownloaderFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BackgroundDownloader, IBackgroundDownloaderFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IBackgroundDownloaderStaticMethods<R, F: FnOnce(&IBackgroundDownloaderStaticMethods) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BackgroundDownloader, IBackgroundDownloaderStaticMethods> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IBackgroundDownloaderStaticMethods2<R, F: FnOnce(&IBackgroundDownloaderStaticMethods2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BackgroundDownloader, IBackgroundDownloaderStaticMethods2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn IBackgroundDownloaderUserConsent<R, F: FnOnce(&IBackgroundDownloaderUserConsent) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BackgroundDownloader, IBackgroundDownloaderUserConsent> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for BackgroundDownloader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BackgroundDownloader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BackgroundDownloader {}
impl ::core::fmt::Debug for BackgroundDownloader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BackgroundDownloader").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BackgroundDownloader {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.BackgroundTransfer.BackgroundDownloader;{c1c79333-6649-4b1d-a826-a4b3dd234d0b})");
}
unsafe impl ::windows::core::Interface for BackgroundDownloader {
    type Vtable = IBackgroundDownloaderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc1c79333_6649_4b1d_a826_a4b3dd234d0b);
}
impl ::windows::core::RuntimeName for BackgroundDownloader {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.BackgroundDownloader";
}
impl ::core::convert::From<BackgroundDownloader> for ::windows::core::IUnknown {
    fn from(value: BackgroundDownloader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BackgroundDownloader> for ::windows::core::IUnknown {
    fn from(value: &BackgroundDownloader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BackgroundDownloader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &BackgroundDownloader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BackgroundDownloader> for ::windows::core::IInspectable {
    fn from(value: BackgroundDownloader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BackgroundDownloader> for ::windows::core::IInspectable {
    fn from(value: &BackgroundDownloader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BackgroundDownloader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &BackgroundDownloader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<BackgroundDownloader> for IBackgroundTransferBase {
    type Error = ::windows::core::Error;
    fn try_from(value: BackgroundDownloader) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&BackgroundDownloader> for IBackgroundTransferBase {
    type Error = ::windows::core::Error;
    fn try_from(value: &BackgroundDownloader) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTransferBase> for BackgroundDownloader {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTransferBase> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTransferBase> for &BackgroundDownloader {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTransferBase> {
        ::core::convert::TryInto::<IBackgroundTransferBase>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for BackgroundDownloader {}
unsafe impl ::core::marker::Sync for BackgroundDownloader {}
#[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
#[repr(transparent)]
pub struct BackgroundTransferBehavior(pub i32);
impl BackgroundTransferBehavior {
    pub const Parallel: Self = Self(0i32);
    pub const Serialized: Self = Self(1i32);
}
impl ::core::marker::Copy for BackgroundTransferBehavior {}
impl ::core::clone::Clone for BackgroundTransferBehavior {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for BackgroundTransferBehavior {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for BackgroundTransferBehavior {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BackgroundTransferBehavior {}
impl ::core::fmt::Debug for BackgroundTransferBehavior {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BackgroundTransferBehavior").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BackgroundTransferBehavior {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.BackgroundTransfer.BackgroundTransferBehavior;i4)");
}
impl ::windows::core::DefaultType for BackgroundTransferBehavior {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
#[repr(transparent)]
pub struct BackgroundTransferCompletionGroup(::windows::core::IUnknown);
impl BackgroundTransferCompletionGroup {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BackgroundTransferCompletionGroup, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer', 'ApplicationModel_Background'*"]
    #[cfg(feature = "ApplicationModel_Background")]
    pub fn Trigger(&self) -> ::windows::core::Result<super::super::ApplicationModel::Background::IBackgroundTrigger> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::ApplicationModel::Background::IBackgroundTrigger>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
    pub fn IsEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
    pub fn Enable(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::clone::Clone for BackgroundTransferCompletionGroup {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BackgroundTransferCompletionGroup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BackgroundTransferCompletionGroup {}
impl ::core::fmt::Debug for BackgroundTransferCompletionGroup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BackgroundTransferCompletionGroup").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BackgroundTransferCompletionGroup {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.BackgroundTransfer.BackgroundTransferCompletionGroup;{2d930225-986b-574d-7950-0add47f5d706})");
}
unsafe impl ::windows::core::Interface for BackgroundTransferCompletionGroup {
    type Vtable = IBackgroundTransferCompletionGroupVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2d930225_986b_574d_7950_0add47f5d706);
}
impl ::windows::core::RuntimeName for BackgroundTransferCompletionGroup {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.BackgroundTransferCompletionGroup";
}
impl ::core::convert::From<BackgroundTransferCompletionGroup> for ::windows::core::IUnknown {
    fn from(value: BackgroundTransferCompletionGroup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BackgroundTransferCompletionGroup> for ::windows::core::IUnknown {
    fn from(value: &BackgroundTransferCompletionGroup) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BackgroundTransferCompletionGroup {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &BackgroundTransferCompletionGroup {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BackgroundTransferCompletionGroup> for ::windows::core::IInspectable {
    fn from(value: BackgroundTransferCompletionGroup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BackgroundTransferCompletionGroup> for ::windows::core::IInspectable {
    fn from(value: &BackgroundTransferCompletionGroup) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BackgroundTransferCompletionGroup {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &BackgroundTransferCompletionGroup {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BackgroundTransferCompletionGroup {}
unsafe impl ::core::marker::Sync for BackgroundTransferCompletionGroup {}
#[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
#[repr(transparent)]
pub struct BackgroundTransferCompletionGroupTriggerDetails(::windows::core::IUnknown);
impl BackgroundTransferCompletionGroupTriggerDetails {
    #[doc = "*Required features: 'Networking_BackgroundTransfer', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Downloads(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<DownloadOperation>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<DownloadOperation>>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Uploads(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<UploadOperation>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<UploadOperation>>(result__)
        }
    }
}
impl ::core::clone::Clone for BackgroundTransferCompletionGroupTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BackgroundTransferCompletionGroupTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BackgroundTransferCompletionGroupTriggerDetails {}
impl ::core::fmt::Debug for BackgroundTransferCompletionGroupTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BackgroundTransferCompletionGroupTriggerDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BackgroundTransferCompletionGroupTriggerDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.BackgroundTransfer.BackgroundTransferCompletionGroupTriggerDetails;{7b6be286-6e47-5136-7fcb-fa4389f46f5b})");
}
unsafe impl ::windows::core::Interface for BackgroundTransferCompletionGroupTriggerDetails {
    type Vtable = IBackgroundTransferCompletionGroupTriggerDetailsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7b6be286_6e47_5136_7fcb_fa4389f46f5b);
}
impl ::windows::core::RuntimeName for BackgroundTransferCompletionGroupTriggerDetails {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.BackgroundTransferCompletionGroupTriggerDetails";
}
impl ::core::convert::From<BackgroundTransferCompletionGroupTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: BackgroundTransferCompletionGroupTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BackgroundTransferCompletionGroupTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: &BackgroundTransferCompletionGroupTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BackgroundTransferCompletionGroupTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &BackgroundTransferCompletionGroupTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BackgroundTransferCompletionGroupTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: BackgroundTransferCompletionGroupTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BackgroundTransferCompletionGroupTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: &BackgroundTransferCompletionGroupTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BackgroundTransferCompletionGroupTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &BackgroundTransferCompletionGroupTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BackgroundTransferCompletionGroupTriggerDetails {}
unsafe impl ::core::marker::Sync for BackgroundTransferCompletionGroupTriggerDetails {}
#[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
#[repr(transparent)]
pub struct BackgroundTransferContentPart(::windows::core::IUnknown);
impl BackgroundTransferContentPart {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BackgroundTransferContentPart, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
    pub fn SetHeader<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, headername: Param0, headervalue: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), headername.into_param().abi(), headervalue.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
    pub fn SetText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer', 'Storage'*"]
    #[cfg(feature = "Storage")]
    pub fn SetFile<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::IStorageFile>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
    pub fn CreateWithName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(name: Param0) -> ::windows::core::Result<BackgroundTransferContentPart> {
        Self::IBackgroundTransferContentPartFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<BackgroundTransferContentPart>(result__)
        })
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
    pub fn CreateWithNameAndFileName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(name: Param0, filename: Param1) -> ::windows::core::Result<BackgroundTransferContentPart> {
        Self::IBackgroundTransferContentPartFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), name.into_param().abi(), filename.into_param().abi(), &mut result__).from_abi::<BackgroundTransferContentPart>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IBackgroundTransferContentPartFactory<R, F: FnOnce(&IBackgroundTransferContentPartFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BackgroundTransferContentPart, IBackgroundTransferContentPartFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for BackgroundTransferContentPart {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BackgroundTransferContentPart {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BackgroundTransferContentPart {}
impl ::core::fmt::Debug for BackgroundTransferContentPart {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BackgroundTransferContentPart").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BackgroundTransferContentPart {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.BackgroundTransfer.BackgroundTransferContentPart;{e8e15657-d7d1-4ed8-838e-674ac217ace6})");
}
unsafe impl ::windows::core::Interface for BackgroundTransferContentPart {
    type Vtable = IBackgroundTransferContentPartVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe8e15657_d7d1_4ed8_838e_674ac217ace6);
}
impl ::windows::core::RuntimeName for BackgroundTransferContentPart {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.BackgroundTransferContentPart";
}
impl ::core::convert::From<BackgroundTransferContentPart> for ::windows::core::IUnknown {
    fn from(value: BackgroundTransferContentPart) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BackgroundTransferContentPart> for ::windows::core::IUnknown {
    fn from(value: &BackgroundTransferContentPart) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BackgroundTransferContentPart {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &BackgroundTransferContentPart {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BackgroundTransferContentPart> for ::windows::core::IInspectable {
    fn from(value: BackgroundTransferContentPart) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BackgroundTransferContentPart> for ::windows::core::IInspectable {
    fn from(value: &BackgroundTransferContentPart) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BackgroundTransferContentPart {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &BackgroundTransferContentPart {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BackgroundTransferContentPart {}
unsafe impl ::core::marker::Sync for BackgroundTransferContentPart {}
#[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
#[repr(transparent)]
pub struct BackgroundTransferCostPolicy(pub i32);
impl BackgroundTransferCostPolicy {
    pub const Default: Self = Self(0i32);
    pub const UnrestrictedOnly: Self = Self(1i32);
    pub const Always: Self = Self(2i32);
}
impl ::core::marker::Copy for BackgroundTransferCostPolicy {}
impl ::core::clone::Clone for BackgroundTransferCostPolicy {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for BackgroundTransferCostPolicy {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for BackgroundTransferCostPolicy {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BackgroundTransferCostPolicy {}
impl ::core::fmt::Debug for BackgroundTransferCostPolicy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BackgroundTransferCostPolicy").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BackgroundTransferCostPolicy {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.BackgroundTransfer.BackgroundTransferCostPolicy;i4)");
}
impl ::windows::core::DefaultType for BackgroundTransferCostPolicy {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
pub struct BackgroundTransferError {}
impl BackgroundTransferError {
    #[doc = "*Required features: 'Networking_BackgroundTransfer', 'Web'*"]
    #[cfg(feature = "Web")]
    pub fn GetStatus(hresult: i32) -> ::windows::core::Result<super::super::Web::WebErrorStatus> {
        Self::IBackgroundTransferErrorStaticMethods(|this| unsafe {
            let mut result__: super::super::Web::WebErrorStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), hresult, &mut result__).from_abi::<super::super::Web::WebErrorStatus>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IBackgroundTransferErrorStaticMethods<R, F: FnOnce(&IBackgroundTransferErrorStaticMethods) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BackgroundTransferError, IBackgroundTransferErrorStaticMethods> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for BackgroundTransferError {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.BackgroundTransferError";
}
#[repr(C)]
#[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
pub struct BackgroundTransferFileRange {
    pub Offset: u64,
    pub Length: u64,
}
impl ::core::marker::Copy for BackgroundTransferFileRange {}
impl ::core::clone::Clone for BackgroundTransferFileRange {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for BackgroundTransferFileRange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BackgroundTransferFileRange").field("Offset", &self.Offset).field("Length", &self.Length).finish()
    }
}
unsafe impl ::windows::core::Abi for BackgroundTransferFileRange {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for BackgroundTransferFileRange {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Networking.BackgroundTransfer.BackgroundTransferFileRange;u8;u8)");
}
impl ::windows::core::DefaultType for BackgroundTransferFileRange {
    type DefaultType = Self;
}
impl ::core::cmp::PartialEq for BackgroundTransferFileRange {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<BackgroundTransferFileRange>()) == 0 }
    }
}
impl ::core::cmp::Eq for BackgroundTransferFileRange {}
impl ::core::default::Default for BackgroundTransferFileRange {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
#[repr(transparent)]
pub struct BackgroundTransferGroup(::windows::core::IUnknown);
impl BackgroundTransferGroup {
    #[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
    pub fn TransferBehavior(&self) -> ::windows::core::Result<BackgroundTransferBehavior> {
        let this = self;
        unsafe {
            let mut result__: BackgroundTransferBehavior = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BackgroundTransferBehavior>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
    pub fn SetTransferBehavior(&self, value: BackgroundTransferBehavior) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
    pub fn CreateGroup<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(name: Param0) -> ::windows::core::Result<BackgroundTransferGroup> {
        Self::IBackgroundTransferGroupStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<BackgroundTransferGroup>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IBackgroundTransferGroupStatics<R, F: FnOnce(&IBackgroundTransferGroupStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BackgroundTransferGroup, IBackgroundTransferGroupStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for BackgroundTransferGroup {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BackgroundTransferGroup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BackgroundTransferGroup {}
impl ::core::fmt::Debug for BackgroundTransferGroup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BackgroundTransferGroup").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BackgroundTransferGroup {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.BackgroundTransfer.BackgroundTransferGroup;{d8c3e3e4-6459-4540-85eb-aaa1c8903677})");
}
unsafe impl ::windows::core::Interface for BackgroundTransferGroup {
    type Vtable = IBackgroundTransferGroupVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd8c3e3e4_6459_4540_85eb_aaa1c8903677);
}
impl ::windows::core::RuntimeName for BackgroundTransferGroup {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.BackgroundTransferGroup";
}
impl ::core::convert::From<BackgroundTransferGroup> for ::windows::core::IUnknown {
    fn from(value: BackgroundTransferGroup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BackgroundTransferGroup> for ::windows::core::IUnknown {
    fn from(value: &BackgroundTransferGroup) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BackgroundTransferGroup {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &BackgroundTransferGroup {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BackgroundTransferGroup> for ::windows::core::IInspectable {
    fn from(value: BackgroundTransferGroup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BackgroundTransferGroup> for ::windows::core::IInspectable {
    fn from(value: &BackgroundTransferGroup) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BackgroundTransferGroup {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &BackgroundTransferGroup {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BackgroundTransferGroup {}
unsafe impl ::core::marker::Sync for BackgroundTransferGroup {}
#[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
#[repr(transparent)]
pub struct BackgroundTransferPriority(pub i32);
impl BackgroundTransferPriority {
    pub const Default: Self = Self(0i32);
    pub const High: Self = Self(1i32);
    pub const Low: Self = Self(2i32);
}
impl ::core::marker::Copy for BackgroundTransferPriority {}
impl ::core::clone::Clone for BackgroundTransferPriority {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for BackgroundTransferPriority {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for BackgroundTransferPriority {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BackgroundTransferPriority {}
impl ::core::fmt::Debug for BackgroundTransferPriority {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BackgroundTransferPriority").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BackgroundTransferPriority {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.BackgroundTransfer.BackgroundTransferPriority;i4)");
}
impl ::windows::core::DefaultType for BackgroundTransferPriority {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
#[repr(transparent)]
pub struct BackgroundTransferRangesDownloadedEventArgs(::windows::core::IUnknown);
impl BackgroundTransferRangesDownloadedEventArgs {
    #[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
    pub fn WasDownloadRestarted(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AddedRanges(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<BackgroundTransferFileRange>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<BackgroundTransferFileRange>>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for BackgroundTransferRangesDownloadedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BackgroundTransferRangesDownloadedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BackgroundTransferRangesDownloadedEventArgs {}
impl ::core::fmt::Debug for BackgroundTransferRangesDownloadedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BackgroundTransferRangesDownloadedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BackgroundTransferRangesDownloadedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.BackgroundTransfer.BackgroundTransferRangesDownloadedEventArgs;{3ebc7453-bf48-4a88-9248-b0c165184f5c})");
}
unsafe impl ::windows::core::Interface for BackgroundTransferRangesDownloadedEventArgs {
    type Vtable = IBackgroundTransferRangesDownloadedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3ebc7453_bf48_4a88_9248_b0c165184f5c);
}
impl ::windows::core::RuntimeName for BackgroundTransferRangesDownloadedEventArgs {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.BackgroundTransferRangesDownloadedEventArgs";
}
impl ::core::convert::From<BackgroundTransferRangesDownloadedEventArgs> for ::windows::core::IUnknown {
    fn from(value: BackgroundTransferRangesDownloadedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BackgroundTransferRangesDownloadedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &BackgroundTransferRangesDownloadedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BackgroundTransferRangesDownloadedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &BackgroundTransferRangesDownloadedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BackgroundTransferRangesDownloadedEventArgs> for ::windows::core::IInspectable {
    fn from(value: BackgroundTransferRangesDownloadedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BackgroundTransferRangesDownloadedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &BackgroundTransferRangesDownloadedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BackgroundTransferRangesDownloadedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &BackgroundTransferRangesDownloadedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BackgroundTransferRangesDownloadedEventArgs {}
unsafe impl ::core::marker::Sync for BackgroundTransferRangesDownloadedEventArgs {}
#[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
#[repr(transparent)]
pub struct BackgroundTransferStatus(pub i32);
impl BackgroundTransferStatus {
    pub const Idle: Self = Self(0i32);
    pub const Running: Self = Self(1i32);
    pub const PausedByApplication: Self = Self(2i32);
    pub const PausedCostedNetwork: Self = Self(3i32);
    pub const PausedNoNetwork: Self = Self(4i32);
    pub const Completed: Self = Self(5i32);
    pub const Canceled: Self = Self(6i32);
    pub const Error: Self = Self(7i32);
    pub const PausedRecoverableWebErrorStatus: Self = Self(8i32);
    pub const PausedSystemPolicy: Self = Self(32i32);
}
impl ::core::marker::Copy for BackgroundTransferStatus {}
impl ::core::clone::Clone for BackgroundTransferStatus {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for BackgroundTransferStatus {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for BackgroundTransferStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BackgroundTransferStatus {}
impl ::core::fmt::Debug for BackgroundTransferStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BackgroundTransferStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BackgroundTransferStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.BackgroundTransfer.BackgroundTransferStatus;i4)");
}
impl ::windows::core::DefaultType for BackgroundTransferStatus {
    type DefaultType = Self;
}
#[repr(C)]
#[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
pub struct BackgroundUploadProgress {
    pub BytesReceived: u64,
    pub BytesSent: u64,
    pub TotalBytesToReceive: u64,
    pub TotalBytesToSend: u64,
    pub Status: BackgroundTransferStatus,
    pub HasResponseChanged: bool,
    pub HasRestarted: bool,
}
impl ::core::marker::Copy for BackgroundUploadProgress {}
impl ::core::clone::Clone for BackgroundUploadProgress {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for BackgroundUploadProgress {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BackgroundUploadProgress").field("BytesReceived", &self.BytesReceived).field("BytesSent", &self.BytesSent).field("TotalBytesToReceive", &self.TotalBytesToReceive).field("TotalBytesToSend", &self.TotalBytesToSend).field("Status", &self.Status).field("HasResponseChanged", &self.HasResponseChanged).field("HasRestarted", &self.HasRestarted).finish()
    }
}
unsafe impl ::windows::core::Abi for BackgroundUploadProgress {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for BackgroundUploadProgress {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Networking.BackgroundTransfer.BackgroundUploadProgress;u8;u8;u8;u8;enum(Windows.Networking.BackgroundTransfer.BackgroundTransferStatus;i4);b1;b1)");
}
impl ::windows::core::DefaultType for BackgroundUploadProgress {
    type DefaultType = Self;
}
impl ::core::cmp::PartialEq for BackgroundUploadProgress {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<BackgroundUploadProgress>()) == 0 }
    }
}
impl ::core::cmp::Eq for BackgroundUploadProgress {}
impl ::core::default::Default for BackgroundUploadProgress {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
#[repr(transparent)]
pub struct BackgroundUploader(::windows::core::IUnknown);
impl BackgroundUploader {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BackgroundUploader, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
    pub fn SetRequestHeader<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, headername: Param0, headervalue: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBackgroundTransferBase>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), headername.into_param().abi(), headervalue.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer', 'Security_Credentials'*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn ServerCredential(&self) -> ::windows::core::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = &::windows::core::Interface::cast::<IBackgroundTransferBase>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Security::Credentials::PasswordCredential>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer', 'Security_Credentials'*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn SetServerCredential<'a, Param0: ::windows::core::IntoParam<'a, super::super::Security::Credentials::PasswordCredential>>(&self, credential: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBackgroundTransferBase>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), credential.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer', 'Security_Credentials'*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn ProxyCredential(&self) -> ::windows::core::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = &::windows::core::Interface::cast::<IBackgroundTransferBase>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Security::Credentials::PasswordCredential>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer', 'Security_Credentials'*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn SetProxyCredential<'a, Param0: ::windows::core::IntoParam<'a, super::super::Security::Credentials::PasswordCredential>>(&self, credential: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBackgroundTransferBase>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), credential.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
    pub fn Method(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IBackgroundTransferBase>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
    pub fn SetMethod<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBackgroundTransferBase>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer', 'deprecated'*"]
    #[cfg(feature = "deprecated")]
    pub fn Group(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IBackgroundTransferBase>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer', 'deprecated'*"]
    #[cfg(feature = "deprecated")]
    pub fn SetGroup<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBackgroundTransferBase>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
    pub fn CostPolicy(&self) -> ::windows::core::Result<BackgroundTransferCostPolicy> {
        let this = &::windows::core::Interface::cast::<IBackgroundTransferBase>(self)?;
        unsafe {
            let mut result__: BackgroundTransferCostPolicy = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BackgroundTransferCostPolicy>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
    pub fn SetCostPolicy(&self, value: BackgroundTransferCostPolicy) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBackgroundTransferBase>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer', 'Foundation', 'Storage'*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn CreateUpload<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>, Param1: ::windows::core::IntoParam<'a, super::super::Storage::IStorageFile>>(&self, uri: Param0, sourcefile: Param1) -> ::windows::core::Result<UploadOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), uri.into_param().abi(), sourcefile.into_param().abi(), &mut result__).from_abi::<UploadOperation>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer', 'Foundation', 'Storage_Streams'*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn CreateUploadFromStreamAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>, Param1: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IInputStream>>(&self, uri: Param0, sourcestream: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<UploadOperation>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), uri.into_param().abi(), sourcestream.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<UploadOperation>>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer', 'Foundation', 'Foundation_Collections'*"]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn CreateUploadWithFormDataAndAutoBoundaryAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<BackgroundTransferContentPart>>>(&self, uri: Param0, parts: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<UploadOperation>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), uri.into_param().abi(), parts.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<UploadOperation>>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer', 'Foundation', 'Foundation_Collections'*"]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn CreateUploadWithSubTypeAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<BackgroundTransferContentPart>>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, uri: Param0, parts: Param1, subtype: Param2) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<UploadOperation>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), uri.into_param().abi(), parts.into_param().abi(), subtype.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<UploadOperation>>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer', 'Foundation', 'Foundation_Collections'*"]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn CreateUploadWithSubTypeAndBoundaryAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<BackgroundTransferContentPart>>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, uri: Param0, parts: Param1, subtype: Param2, boundary: Param3) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<UploadOperation>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), uri.into_param().abi(), parts.into_param().abi(), subtype.into_param().abi(), boundary.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<UploadOperation>>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
    pub fn TransferGroup(&self) -> ::windows::core::Result<BackgroundTransferGroup> {
        let this = &::windows::core::Interface::cast::<IBackgroundUploader2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BackgroundTransferGroup>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
    pub fn SetTransferGroup<'a, Param0: ::windows::core::IntoParam<'a, BackgroundTransferGroup>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBackgroundUploader2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer', 'UI_Notifications'*"]
    #[cfg(feature = "UI_Notifications")]
    pub fn SuccessToastNotification(&self) -> ::windows::core::Result<super::super::UI::Notifications::ToastNotification> {
        let this = &::windows::core::Interface::cast::<IBackgroundUploader2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::Notifications::ToastNotification>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer', 'UI_Notifications'*"]
    #[cfg(feature = "UI_Notifications")]
    pub fn SetSuccessToastNotification<'a, Param0: ::windows::core::IntoParam<'a, super::super::UI::Notifications::ToastNotification>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBackgroundUploader2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer', 'UI_Notifications'*"]
    #[cfg(feature = "UI_Notifications")]
    pub fn FailureToastNotification(&self) -> ::windows::core::Result<super::super::UI::Notifications::ToastNotification> {
        let this = &::windows::core::Interface::cast::<IBackgroundUploader2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::Notifications::ToastNotification>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer', 'UI_Notifications'*"]
    #[cfg(feature = "UI_Notifications")]
    pub fn SetFailureToastNotification<'a, Param0: ::windows::core::IntoParam<'a, super::super::UI::Notifications::ToastNotification>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBackgroundUploader2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer', 'UI_Notifications'*"]
    #[cfg(feature = "UI_Notifications")]
    pub fn SuccessTileNotification(&self) -> ::windows::core::Result<super::super::UI::Notifications::TileNotification> {
        let this = &::windows::core::Interface::cast::<IBackgroundUploader2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::Notifications::TileNotification>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer', 'UI_Notifications'*"]
    #[cfg(feature = "UI_Notifications")]
    pub fn SetSuccessTileNotification<'a, Param0: ::windows::core::IntoParam<'a, super::super::UI::Notifications::TileNotification>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBackgroundUploader2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer', 'UI_Notifications'*"]
    #[cfg(feature = "UI_Notifications")]
    pub fn FailureTileNotification(&self) -> ::windows::core::Result<super::super::UI::Notifications::TileNotification> {
        let this = &::windows::core::Interface::cast::<IBackgroundUploader2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::Notifications::TileNotification>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer', 'UI_Notifications'*"]
    #[cfg(feature = "UI_Notifications")]
    pub fn SetFailureTileNotification<'a, Param0: ::windows::core::IntoParam<'a, super::super::UI::Notifications::TileNotification>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBackgroundUploader2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
    pub fn CompletionGroup(&self) -> ::windows::core::Result<BackgroundTransferCompletionGroup> {
        let this = &::windows::core::Interface::cast::<IBackgroundUploader3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BackgroundTransferCompletionGroup>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
    pub fn CreateWithCompletionGroup<'a, Param0: ::windows::core::IntoParam<'a, BackgroundTransferCompletionGroup>>(completiongroup: Param0) -> ::windows::core::Result<BackgroundUploader> {
        Self::IBackgroundUploaderFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), completiongroup.into_param().abi(), &mut result__).from_abi::<BackgroundUploader>(result__)
        })
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer', 'Foundation', 'Foundation_Collections'*"]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn GetCurrentUploadsAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<UploadOperation>>> {
        Self::IBackgroundUploaderStaticMethods(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<UploadOperation>>>(result__)
        })
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer', 'Foundation', 'Foundation_Collections', 'deprecated'*"]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn GetCurrentUploadsForGroupAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(group: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<UploadOperation>>> {
        Self::IBackgroundUploaderStaticMethods(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), group.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<UploadOperation>>>(result__)
        })
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer', 'Foundation', 'Foundation_Collections'*"]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn GetCurrentUploadsForTransferGroupAsync<'a, Param0: ::windows::core::IntoParam<'a, BackgroundTransferGroup>>(group: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<UploadOperation>>> {
        Self::IBackgroundUploaderStaticMethods2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), group.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<UploadOperation>>>(result__)
        })
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer', 'Foundation', 'Foundation_Collections', 'deprecated'*"]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn RequestUnconstrainedUploadsAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<UploadOperation>>>(operations: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<UnconstrainedTransferRequestResult>> {
        Self::IBackgroundUploaderUserConsent(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), operations.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<UnconstrainedTransferRequestResult>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IBackgroundUploaderFactory<R, F: FnOnce(&IBackgroundUploaderFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BackgroundUploader, IBackgroundUploaderFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IBackgroundUploaderStaticMethods<R, F: FnOnce(&IBackgroundUploaderStaticMethods) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BackgroundUploader, IBackgroundUploaderStaticMethods> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IBackgroundUploaderStaticMethods2<R, F: FnOnce(&IBackgroundUploaderStaticMethods2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BackgroundUploader, IBackgroundUploaderStaticMethods2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn IBackgroundUploaderUserConsent<R, F: FnOnce(&IBackgroundUploaderUserConsent) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BackgroundUploader, IBackgroundUploaderUserConsent> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for BackgroundUploader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BackgroundUploader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BackgroundUploader {}
impl ::core::fmt::Debug for BackgroundUploader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BackgroundUploader").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BackgroundUploader {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.BackgroundTransfer.BackgroundUploader;{c595c9ae-cead-465b-8801-c55ac90a01ce})");
}
unsafe impl ::windows::core::Interface for BackgroundUploader {
    type Vtable = IBackgroundUploaderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc595c9ae_cead_465b_8801_c55ac90a01ce);
}
impl ::windows::core::RuntimeName for BackgroundUploader {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.BackgroundUploader";
}
impl ::core::convert::From<BackgroundUploader> for ::windows::core::IUnknown {
    fn from(value: BackgroundUploader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BackgroundUploader> for ::windows::core::IUnknown {
    fn from(value: &BackgroundUploader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BackgroundUploader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &BackgroundUploader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BackgroundUploader> for ::windows::core::IInspectable {
    fn from(value: BackgroundUploader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BackgroundUploader> for ::windows::core::IInspectable {
    fn from(value: &BackgroundUploader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BackgroundUploader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &BackgroundUploader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<BackgroundUploader> for IBackgroundTransferBase {
    type Error = ::windows::core::Error;
    fn try_from(value: BackgroundUploader) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&BackgroundUploader> for IBackgroundTransferBase {
    type Error = ::windows::core::Error;
    fn try_from(value: &BackgroundUploader) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTransferBase> for BackgroundUploader {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTransferBase> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTransferBase> for &BackgroundUploader {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTransferBase> {
        ::core::convert::TryInto::<IBackgroundTransferBase>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for BackgroundUploader {}
unsafe impl ::core::marker::Sync for BackgroundUploader {}
#[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
pub struct ContentPrefetcher {}
impl ContentPrefetcher {
    #[doc = "*Required features: 'Networking_BackgroundTransfer', 'Foundation', 'Foundation_Collections'*"]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn ContentUris() -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>> {
        Self::IContentPrefetcher(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>>(result__)
        })
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetIndirectContentUri<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(value: Param0) -> ::windows::core::Result<()> {
        Self::IContentPrefetcher(|this| unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() })
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn IndirectContentUri() -> ::windows::core::Result<super::super::Foundation::Uri> {
        Self::IContentPrefetcher(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        })
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn LastSuccessfulPrefetchTime() -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        Self::IContentPrefetcherTime(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IContentPrefetcher<R, F: FnOnce(&IContentPrefetcher) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ContentPrefetcher, IContentPrefetcher> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IContentPrefetcherTime<R, F: FnOnce(&IContentPrefetcherTime) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ContentPrefetcher, IContentPrefetcherTime> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for ContentPrefetcher {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.ContentPrefetcher";
}
#[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
#[repr(transparent)]
pub struct DownloadOperation(::windows::core::IUnknown);
impl DownloadOperation {
    #[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
    pub fn Guid(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = &::windows::core::Interface::cast::<IBackgroundTransferOperation>(self)?;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestedUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = &::windows::core::Interface::cast::<IBackgroundTransferOperation>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
    pub fn Method(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IBackgroundTransferOperation>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer', 'deprecated'*"]
    #[cfg(feature = "deprecated")]
    pub fn Group(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IBackgroundTransferOperation>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
    pub fn CostPolicy(&self) -> ::windows::core::Result<BackgroundTransferCostPolicy> {
        let this = &::windows::core::Interface::cast::<IBackgroundTransferOperation>(self)?;
        unsafe {
            let mut result__: BackgroundTransferCostPolicy = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BackgroundTransferCostPolicy>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
    pub fn SetCostPolicy(&self, value: BackgroundTransferCostPolicy) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBackgroundTransferOperation>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn GetResultStreamAt(&self, position: u64) -> ::windows::core::Result<super::super::Storage::Streams::IInputStream> {
        let this = &::windows::core::Interface::cast::<IBackgroundTransferOperation>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), position, &mut result__).from_abi::<super::super::Storage::Streams::IInputStream>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
    pub fn GetResponseInformation(&self) -> ::windows::core::Result<ResponseInformation> {
        let this = &::windows::core::Interface::cast::<IBackgroundTransferOperation>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ResponseInformation>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
    pub fn Priority(&self) -> ::windows::core::Result<BackgroundTransferPriority> {
        let this = &::windows::core::Interface::cast::<IBackgroundTransferOperationPriority>(self)?;
        unsafe {
            let mut result__: BackgroundTransferPriority = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BackgroundTransferPriority>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
    pub fn SetPriority(&self, value: BackgroundTransferPriority) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBackgroundTransferOperationPriority>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer', 'Storage'*"]
    #[cfg(feature = "Storage")]
    pub fn ResultFile(&self) -> ::windows::core::Result<super::super::Storage::IStorageFile> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::IStorageFile>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
    pub fn Progress(&self) -> ::windows::core::Result<BackgroundDownloadProgress> {
        let this = self;
        unsafe {
            let mut result__: BackgroundDownloadProgress = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BackgroundDownloadProgress>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn StartAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DownloadOperation, DownloadOperation>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<DownloadOperation, DownloadOperation>>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn AttachAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DownloadOperation, DownloadOperation>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<DownloadOperation, DownloadOperation>>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
    pub fn Pause(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
    pub fn Resume(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
    pub fn TransferGroup(&self) -> ::windows::core::Result<BackgroundTransferGroup> {
        let this = &::windows::core::Interface::cast::<IDownloadOperation2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BackgroundTransferGroup>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
    pub fn IsRandomAccessRequired(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IDownloadOperation3>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
    pub fn SetIsRandomAccessRequired(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IDownloadOperation3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn GetResultRandomAccessStreamReference(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = &::windows::core::Interface::cast::<IDownloadOperation3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IRandomAccessStreamReference>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetDownloadedRanges(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<BackgroundTransferFileRange>> {
        let this = &::windows::core::Interface::cast::<IDownloadOperation3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<BackgroundTransferFileRange>>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RangesDownloaded<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<DownloadOperation, BackgroundTransferRangesDownloadedEventArgs>>>(&self, eventhandler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IDownloadOperation3>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), eventhandler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveRangesDownloaded<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IDownloadOperation3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), eventcookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetRequestedUri<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IDownloadOperation3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer', 'Foundation_Collections', 'Web'*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Web"))]
    pub fn RecoverableWebErrorStatuses(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Web::WebErrorStatus>> {
        let this = &::windows::core::Interface::cast::<IDownloadOperation3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::super::Web::WebErrorStatus>>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer', 'Foundation', 'Web'*"]
    #[cfg(all(feature = "Foundation", feature = "Web"))]
    pub fn CurrentWebErrorStatus(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Web::WebErrorStatus>> {
        let this = &::windows::core::Interface::cast::<IDownloadOperation3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Web::WebErrorStatus>>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
    pub fn MakeCurrentInTransferGroup(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IDownloadOperation4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
    pub fn SetRequestHeader<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, headername: Param0, headervalue: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IDownloadOperation5>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), headername.into_param().abi(), headervalue.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
    pub fn RemoveRequestHeader<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, headername: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IDownloadOperation5>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), headername.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for DownloadOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DownloadOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DownloadOperation {}
impl ::core::fmt::Debug for DownloadOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DownloadOperation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DownloadOperation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.BackgroundTransfer.DownloadOperation;{bd87ebb0-5714-4e09-ba68-bef73903b0d7})");
}
unsafe impl ::windows::core::Interface for DownloadOperation {
    type Vtable = IDownloadOperationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbd87ebb0_5714_4e09_ba68_bef73903b0d7);
}
impl ::windows::core::RuntimeName for DownloadOperation {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.DownloadOperation";
}
impl ::core::convert::From<DownloadOperation> for ::windows::core::IUnknown {
    fn from(value: DownloadOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DownloadOperation> for ::windows::core::IUnknown {
    fn from(value: &DownloadOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DownloadOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &DownloadOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DownloadOperation> for ::windows::core::IInspectable {
    fn from(value: DownloadOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DownloadOperation> for ::windows::core::IInspectable {
    fn from(value: &DownloadOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DownloadOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &DownloadOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<DownloadOperation> for IBackgroundTransferOperation {
    type Error = ::windows::core::Error;
    fn try_from(value: DownloadOperation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DownloadOperation> for IBackgroundTransferOperation {
    type Error = ::windows::core::Error;
    fn try_from(value: &DownloadOperation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTransferOperation> for DownloadOperation {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTransferOperation> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTransferOperation> for &DownloadOperation {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTransferOperation> {
        ::core::convert::TryInto::<IBackgroundTransferOperation>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<DownloadOperation> for IBackgroundTransferOperationPriority {
    type Error = ::windows::core::Error;
    fn try_from(value: DownloadOperation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DownloadOperation> for IBackgroundTransferOperationPriority {
    type Error = ::windows::core::Error;
    fn try_from(value: &DownloadOperation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTransferOperationPriority> for DownloadOperation {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTransferOperationPriority> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTransferOperationPriority> for &DownloadOperation {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTransferOperationPriority> {
        ::core::convert::TryInto::<IBackgroundTransferOperationPriority>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for DownloadOperation {}
unsafe impl ::core::marker::Sync for DownloadOperation {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBackgroundDownloader(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBackgroundDownloader {
    type Vtable = IBackgroundDownloaderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc1c79333_6649_4b1d_a826_a4b3dd234d0b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundDownloaderVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, resultfile: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, resultfile: ::windows::core::RawPtr, requestbodyfile: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, resultfile: ::windows::core::RawPtr, requestbodystream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage", feature = "Storage_Streams")))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBackgroundDownloader2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBackgroundDownloader2 {
    type Vtable = IBackgroundDownloader2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa94a5847_348d_4a35_890e_8a1ef3798479);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundDownloader2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Notifications")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))] usize,
    #[cfg(feature = "UI_Notifications")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))] usize,
    #[cfg(feature = "UI_Notifications")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))] usize,
    #[cfg(feature = "UI_Notifications")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))] usize,
    #[cfg(feature = "UI_Notifications")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))] usize,
    #[cfg(feature = "UI_Notifications")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))] usize,
    #[cfg(feature = "UI_Notifications")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))] usize,
    #[cfg(feature = "UI_Notifications")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBackgroundDownloader3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBackgroundDownloader3 {
    type Vtable = IBackgroundDownloader3Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd11a8c48_86e8_48e2_b615_6976aabf861d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundDownloader3Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBackgroundDownloaderFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBackgroundDownloaderFactory {
    type Vtable = IBackgroundDownloaderFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x26836c24_d89e_46f4_a29a_4f4d4f144155);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundDownloaderFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, completiongroup: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBackgroundDownloaderStaticMethods(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBackgroundDownloaderStaticMethods {
    type Vtable = IBackgroundDownloaderStaticMethodsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x52a65a35_c64e_426c_9919_540d0d21a650);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundDownloaderStaticMethodsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "deprecated"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, group: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections", feature = "deprecated")))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBackgroundDownloaderStaticMethods2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBackgroundDownloaderStaticMethods2 {
    type Vtable = IBackgroundDownloaderStaticMethods2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2faa1327_1ad4_4ca5_b2cd_08dbf0746afe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundDownloaderStaticMethods2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, group: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IBackgroundDownloaderUserConsent(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IBackgroundDownloaderUserConsent {
    type Vtable = IBackgroundDownloaderUserConsentVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5d14e906_9266_4808_bd71_5925f2a3130a);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundDownloaderUserConsentVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "deprecated"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, operations: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections", feature = "deprecated")))] usize,
);
#[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
#[repr(transparent)]
pub struct IBackgroundTransferBase(::windows::core::IUnknown);
impl IBackgroundTransferBase {
    #[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
    pub fn SetRequestHeader<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, headername: Param0, headervalue: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), headername.into_param().abi(), headervalue.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer', 'Security_Credentials'*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn ServerCredential(&self) -> ::windows::core::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Security::Credentials::PasswordCredential>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer', 'Security_Credentials'*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn SetServerCredential<'a, Param0: ::windows::core::IntoParam<'a, super::super::Security::Credentials::PasswordCredential>>(&self, credential: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), credential.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer', 'Security_Credentials'*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn ProxyCredential(&self) -> ::windows::core::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Security::Credentials::PasswordCredential>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer', 'Security_Credentials'*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn SetProxyCredential<'a, Param0: ::windows::core::IntoParam<'a, super::super::Security::Credentials::PasswordCredential>>(&self, credential: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), credential.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
    pub fn Method(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
    pub fn SetMethod<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer', 'deprecated'*"]
    #[cfg(feature = "deprecated")]
    pub fn Group(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer', 'deprecated'*"]
    #[cfg(feature = "deprecated")]
    pub fn SetGroup<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
    pub fn CostPolicy(&self) -> ::windows::core::Result<BackgroundTransferCostPolicy> {
        let this = self;
        unsafe {
            let mut result__: BackgroundTransferCostPolicy = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BackgroundTransferCostPolicy>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
    pub fn SetCostPolicy(&self, value: BackgroundTransferCostPolicy) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), value).ok() }
    }
}
impl ::core::convert::From<IBackgroundTransferBase> for ::windows::core::IInspectable {
    fn from(value: IBackgroundTransferBase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundTransferBase> for ::windows::core::IInspectable {
    fn from(value: &IBackgroundTransferBase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IBackgroundTransferBase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IBackgroundTransferBase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IBackgroundTransferBase> for ::windows::core::IUnknown {
    fn from(value: IBackgroundTransferBase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundTransferBase> for ::windows::core::IUnknown {
    fn from(value: &IBackgroundTransferBase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IBackgroundTransferBase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IBackgroundTransferBase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IBackgroundTransferBase {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBackgroundTransferBase {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBackgroundTransferBase {}
impl ::core::fmt::Debug for IBackgroundTransferBase {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBackgroundTransferBase").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IBackgroundTransferBase {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{2a9da250-c769-458c-afe8-feb8d4d3b2ef}");
}
unsafe impl ::windows::core::Interface for IBackgroundTransferBase {
    type Vtable = IBackgroundTransferBaseVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2a9da250_c769_458c_afe8_feb8d4d3b2ef);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTransferBaseVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, headername: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, headervalue: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Security_Credentials")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))] usize,
    #[cfg(feature = "Security_Credentials")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, credential: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))] usize,
    #[cfg(feature = "Security_Credentials")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))] usize,
    #[cfg(feature = "Security_Credentials")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, credential: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "deprecated")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))] usize,
    #[cfg(feature = "deprecated")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BackgroundTransferCostPolicy) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: BackgroundTransferCostPolicy) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBackgroundTransferCompletionGroup(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBackgroundTransferCompletionGroup {
    type Vtable = IBackgroundTransferCompletionGroupVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2d930225_986b_574d_7950_0add47f5d706);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTransferCompletionGroupVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "ApplicationModel_Background")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Background"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBackgroundTransferCompletionGroupTriggerDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBackgroundTransferCompletionGroupTriggerDetails {
    type Vtable = IBackgroundTransferCompletionGroupTriggerDetailsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7b6be286_6e47_5136_7fcb_fa4389f46f5b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTransferCompletionGroupTriggerDetailsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBackgroundTransferContentPart(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBackgroundTransferContentPart {
    type Vtable = IBackgroundTransferContentPartVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe8e15657_d7d1_4ed8_838e_674ac217ace6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTransferContentPartVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, headername: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, headervalue: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage"))] usize,
);
#[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
#[repr(transparent)]
pub struct IBackgroundTransferContentPartFactory(::windows::core::IUnknown);
impl IBackgroundTransferContentPartFactory {
    #[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
    pub fn CreateWithName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0) -> ::windows::core::Result<BackgroundTransferContentPart> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<BackgroundTransferContentPart>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
    pub fn CreateWithNameAndFileName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, filename: Param1) -> ::windows::core::Result<BackgroundTransferContentPart> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), name.into_param().abi(), filename.into_param().abi(), &mut result__).from_abi::<BackgroundTransferContentPart>(result__)
        }
    }
}
impl ::core::convert::From<IBackgroundTransferContentPartFactory> for ::windows::core::IInspectable {
    fn from(value: IBackgroundTransferContentPartFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundTransferContentPartFactory> for ::windows::core::IInspectable {
    fn from(value: &IBackgroundTransferContentPartFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IBackgroundTransferContentPartFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IBackgroundTransferContentPartFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IBackgroundTransferContentPartFactory> for ::windows::core::IUnknown {
    fn from(value: IBackgroundTransferContentPartFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundTransferContentPartFactory> for ::windows::core::IUnknown {
    fn from(value: &IBackgroundTransferContentPartFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IBackgroundTransferContentPartFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IBackgroundTransferContentPartFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IBackgroundTransferContentPartFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBackgroundTransferContentPartFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBackgroundTransferContentPartFactory {}
impl ::core::fmt::Debug for IBackgroundTransferContentPartFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBackgroundTransferContentPartFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IBackgroundTransferContentPartFactory {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{90ef98a9-7a01-4a0b-9f80-a0b0bb370f8d}");
}
unsafe impl ::windows::core::Interface for IBackgroundTransferContentPartFactory {
    type Vtable = IBackgroundTransferContentPartFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x90ef98a9_7a01_4a0b_9f80_a0b0bb370f8d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTransferContentPartFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, filename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBackgroundTransferErrorStaticMethods(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBackgroundTransferErrorStaticMethods {
    type Vtable = IBackgroundTransferErrorStaticMethodsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaad33b04_1192_4bf4_8b68_39c5add244e2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTransferErrorStaticMethodsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Web")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hresult: i32, result__: *mut super::super::Web::WebErrorStatus) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Web"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBackgroundTransferGroup(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBackgroundTransferGroup {
    type Vtable = IBackgroundTransferGroupVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd8c3e3e4_6459_4540_85eb_aaa1c8903677);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTransferGroupVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BackgroundTransferBehavior) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: BackgroundTransferBehavior) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBackgroundTransferGroupStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBackgroundTransferGroupStatics {
    type Vtable = IBackgroundTransferGroupStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x02ec50b2_7d18_495b_aa22_32a97d45d3e2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTransferGroupStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
#[repr(transparent)]
pub struct IBackgroundTransferOperation(::windows::core::IUnknown);
impl IBackgroundTransferOperation {
    #[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
    pub fn Guid(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestedUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
    pub fn Method(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer', 'deprecated'*"]
    #[cfg(feature = "deprecated")]
    pub fn Group(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
    pub fn CostPolicy(&self) -> ::windows::core::Result<BackgroundTransferCostPolicy> {
        let this = self;
        unsafe {
            let mut result__: BackgroundTransferCostPolicy = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BackgroundTransferCostPolicy>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
    pub fn SetCostPolicy(&self, value: BackgroundTransferCostPolicy) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn GetResultStreamAt(&self, position: u64) -> ::windows::core::Result<super::super::Storage::Streams::IInputStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), position, &mut result__).from_abi::<super::super::Storage::Streams::IInputStream>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
    pub fn GetResponseInformation(&self) -> ::windows::core::Result<ResponseInformation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ResponseInformation>(result__)
        }
    }
}
impl ::core::convert::From<IBackgroundTransferOperation> for ::windows::core::IInspectable {
    fn from(value: IBackgroundTransferOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundTransferOperation> for ::windows::core::IInspectable {
    fn from(value: &IBackgroundTransferOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IBackgroundTransferOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IBackgroundTransferOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IBackgroundTransferOperation> for ::windows::core::IUnknown {
    fn from(value: IBackgroundTransferOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundTransferOperation> for ::windows::core::IUnknown {
    fn from(value: &IBackgroundTransferOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IBackgroundTransferOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IBackgroundTransferOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IBackgroundTransferOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBackgroundTransferOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBackgroundTransferOperation {}
impl ::core::fmt::Debug for IBackgroundTransferOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBackgroundTransferOperation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IBackgroundTransferOperation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{ded06846-90ca-44fb-8fb1-124154c0d539}");
}
unsafe impl ::windows::core::Interface for IBackgroundTransferOperation {
    type Vtable = IBackgroundTransferOperationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xded06846_90ca_44fb_8fb1_124154c0d539);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTransferOperationVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "deprecated")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BackgroundTransferCostPolicy) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: BackgroundTransferCostPolicy) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, position: u64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
#[repr(transparent)]
pub struct IBackgroundTransferOperationPriority(::windows::core::IUnknown);
impl IBackgroundTransferOperationPriority {
    #[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
    pub fn Priority(&self) -> ::windows::core::Result<BackgroundTransferPriority> {
        let this = self;
        unsafe {
            let mut result__: BackgroundTransferPriority = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BackgroundTransferPriority>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
    pub fn SetPriority(&self, value: BackgroundTransferPriority) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
}
impl ::core::convert::From<IBackgroundTransferOperationPriority> for ::windows::core::IInspectable {
    fn from(value: IBackgroundTransferOperationPriority) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundTransferOperationPriority> for ::windows::core::IInspectable {
    fn from(value: &IBackgroundTransferOperationPriority) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IBackgroundTransferOperationPriority {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IBackgroundTransferOperationPriority {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IBackgroundTransferOperationPriority> for ::windows::core::IUnknown {
    fn from(value: IBackgroundTransferOperationPriority) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBackgroundTransferOperationPriority> for ::windows::core::IUnknown {
    fn from(value: &IBackgroundTransferOperationPriority) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IBackgroundTransferOperationPriority {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IBackgroundTransferOperationPriority {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IBackgroundTransferOperationPriority {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBackgroundTransferOperationPriority {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBackgroundTransferOperationPriority {}
impl ::core::fmt::Debug for IBackgroundTransferOperationPriority {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBackgroundTransferOperationPriority").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IBackgroundTransferOperationPriority {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{04854327-5254-4b3a-915e-0aa49275c0f9}");
}
unsafe impl ::windows::core::Interface for IBackgroundTransferOperationPriority {
    type Vtable = IBackgroundTransferOperationPriorityVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x04854327_5254_4b3a_915e_0aa49275c0f9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTransferOperationPriorityVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BackgroundTransferPriority) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: BackgroundTransferPriority) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBackgroundTransferRangesDownloadedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBackgroundTransferRangesDownloadedEventArgs {
    type Vtable = IBackgroundTransferRangesDownloadedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3ebc7453_bf48_4a88_9248_b0c165184f5c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTransferRangesDownloadedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBackgroundUploader(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBackgroundUploader {
    type Vtable = IBackgroundUploaderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc595c9ae_cead_465b_8801_c55ac90a01ce);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundUploaderVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, sourcefile: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, sourcestream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, parts: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, parts: ::windows::core::RawPtr, subtype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, parts: ::windows::core::RawPtr, subtype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, boundary: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBackgroundUploader2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBackgroundUploader2 {
    type Vtable = IBackgroundUploader2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8e0612ce_0c34_4463_807f_198a1b8bd4ad);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundUploader2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Notifications")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))] usize,
    #[cfg(feature = "UI_Notifications")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))] usize,
    #[cfg(feature = "UI_Notifications")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))] usize,
    #[cfg(feature = "UI_Notifications")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))] usize,
    #[cfg(feature = "UI_Notifications")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))] usize,
    #[cfg(feature = "UI_Notifications")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))] usize,
    #[cfg(feature = "UI_Notifications")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))] usize,
    #[cfg(feature = "UI_Notifications")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBackgroundUploader3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBackgroundUploader3 {
    type Vtable = IBackgroundUploader3Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb95e9439_5bf0_4b3a_8c47_2c6199a854b9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundUploader3Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBackgroundUploaderFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBackgroundUploaderFactory {
    type Vtable = IBackgroundUploaderFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x736203c7_10e7_48a0_ac3c_1ac71095ec57);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundUploaderFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, completiongroup: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBackgroundUploaderStaticMethods(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBackgroundUploaderStaticMethods {
    type Vtable = IBackgroundUploaderStaticMethodsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2875cfb_9b05_4741_9121_740a83e247df);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundUploaderStaticMethodsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "deprecated"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, group: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections", feature = "deprecated")))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IBackgroundUploaderStaticMethods2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBackgroundUploaderStaticMethods2 {
    type Vtable = IBackgroundUploaderStaticMethods2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe919ac62_ea08_42f0_a2ac_07e467549080);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundUploaderStaticMethods2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, group: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IBackgroundUploaderUserConsent(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IBackgroundUploaderUserConsent {
    type Vtable = IBackgroundUploaderUserConsentVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3bb384cb_0760_461d_907f_5138f84d44c1);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundUploaderUserConsentVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "deprecated"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, operations: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections", feature = "deprecated")))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IContentPrefetcher(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IContentPrefetcher {
    type Vtable = IContentPrefetcherVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa8d6f754_7dc1_4cd9_8810_2a6aa9417e11);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentPrefetcherVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IContentPrefetcherTime(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IContentPrefetcherTime {
    type Vtable = IContentPrefetcherTimeVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe361fd08_132a_4fde_a7cc_fcb0e66523af);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentPrefetcherTimeVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IDownloadOperation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDownloadOperation {
    type Vtable = IDownloadOperationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbd87ebb0_5714_4e09_ba68_bef73903b0d7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDownloadOperationVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BackgroundDownloadProgress) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IDownloadOperation2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDownloadOperation2 {
    type Vtable = IDownloadOperation2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa3cced40_8f9c_4353_9cd4_290dee387c38);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDownloadOperation2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IDownloadOperation3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDownloadOperation3 {
    type Vtable = IDownloadOperation3Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5027351c_7d5e_4adc_b8d3_df5c6031b9cc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDownloadOperation3Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventhandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Web"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Web")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Web"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Web")))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IDownloadOperation4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDownloadOperation4 {
    type Vtable = IDownloadOperation4Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0cdaaef4_8cef_404a_966d_f058400bed80);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDownloadOperation4Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IDownloadOperation5(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDownloadOperation5 {
    type Vtable = IDownloadOperation5Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa699a86f_5590_463a_b8d6_1e491a2760a5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDownloadOperation5Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, headername: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, headervalue: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, headername: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IResponseInformation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IResponseInformation {
    type Vtable = IResponseInformationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf8bb9a12_f713_4792_8b68_d9d297f91d2e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResponseInformationVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IUnconstrainedTransferRequestResult(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IUnconstrainedTransferRequestResult {
    type Vtable = IUnconstrainedTransferRequestResultVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4c24b81f_d944_4112_a98e_6a69522b7ebb);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IUnconstrainedTransferRequestResultVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "deprecated")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IUploadOperation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUploadOperation {
    type Vtable = IUploadOperationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3e5624e0_7389_434c_8b35_427fd36bbdae);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUploadOperationVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BackgroundUploadProgress) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IUploadOperation2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUploadOperation2 {
    type Vtable = IUploadOperation2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x556189f2_2774_4df6_9fa5_209f2bfb12f7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUploadOperation2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IUploadOperation3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUploadOperation3 {
    type Vtable = IUploadOperation3Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x42c92ca3_de39_4546_bc62_3774b4294de3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUploadOperation3Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IUploadOperation4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUploadOperation4 {
    type Vtable = IUploadOperation4Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x50edef31_fac5_41ee_b030_dc77caee9faa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUploadOperation4Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, headername: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, headervalue: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, headername: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
#[repr(transparent)]
pub struct ResponseInformation(::windows::core::IUnknown);
impl ResponseInformation {
    #[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
    pub fn IsResumable(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ActualUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
    pub fn StatusCode(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Headers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>>(result__)
        }
    }
}
impl ::core::clone::Clone for ResponseInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ResponseInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ResponseInformation {}
impl ::core::fmt::Debug for ResponseInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ResponseInformation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ResponseInformation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.BackgroundTransfer.ResponseInformation;{f8bb9a12-f713-4792-8b68-d9d297f91d2e})");
}
unsafe impl ::windows::core::Interface for ResponseInformation {
    type Vtable = IResponseInformationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf8bb9a12_f713_4792_8b68_d9d297f91d2e);
}
impl ::windows::core::RuntimeName for ResponseInformation {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.ResponseInformation";
}
impl ::core::convert::From<ResponseInformation> for ::windows::core::IUnknown {
    fn from(value: ResponseInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ResponseInformation> for ::windows::core::IUnknown {
    fn from(value: &ResponseInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ResponseInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ResponseInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ResponseInformation> for ::windows::core::IInspectable {
    fn from(value: ResponseInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ResponseInformation> for ::windows::core::IInspectable {
    fn from(value: &ResponseInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ResponseInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ResponseInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ResponseInformation {}
unsafe impl ::core::marker::Sync for ResponseInformation {}
#[doc = "*Required features: 'Networking_BackgroundTransfer', 'deprecated'*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct UnconstrainedTransferRequestResult(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl UnconstrainedTransferRequestResult {
    #[doc = "*Required features: 'Networking_BackgroundTransfer', 'deprecated'*"]
    #[cfg(feature = "deprecated")]
    pub fn IsUnconstrained(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for UnconstrainedTransferRequestResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for UnconstrainedTransferRequestResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for UnconstrainedTransferRequestResult {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for UnconstrainedTransferRequestResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UnconstrainedTransferRequestResult").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for UnconstrainedTransferRequestResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.BackgroundTransfer.UnconstrainedTransferRequestResult;{4c24b81f-d944-4112-a98e-6a69522b7ebb})");
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for UnconstrainedTransferRequestResult {
    type Vtable = IUnconstrainedTransferRequestResultVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4c24b81f_d944_4112_a98e_6a69522b7ebb);
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for UnconstrainedTransferRequestResult {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.UnconstrainedTransferRequestResult";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<UnconstrainedTransferRequestResult> for ::windows::core::IUnknown {
    fn from(value: UnconstrainedTransferRequestResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&UnconstrainedTransferRequestResult> for ::windows::core::IUnknown {
    fn from(value: &UnconstrainedTransferRequestResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UnconstrainedTransferRequestResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &UnconstrainedTransferRequestResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<UnconstrainedTransferRequestResult> for ::windows::core::IInspectable {
    fn from(value: UnconstrainedTransferRequestResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&UnconstrainedTransferRequestResult> for ::windows::core::IInspectable {
    fn from(value: &UnconstrainedTransferRequestResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UnconstrainedTransferRequestResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &UnconstrainedTransferRequestResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for UnconstrainedTransferRequestResult {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for UnconstrainedTransferRequestResult {}
#[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
#[repr(transparent)]
pub struct UploadOperation(::windows::core::IUnknown);
impl UploadOperation {
    #[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
    pub fn Guid(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = &::windows::core::Interface::cast::<IBackgroundTransferOperation>(self)?;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestedUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = &::windows::core::Interface::cast::<IBackgroundTransferOperation>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
    pub fn Method(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IBackgroundTransferOperation>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer', 'deprecated'*"]
    #[cfg(feature = "deprecated")]
    pub fn Group(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IBackgroundTransferOperation>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
    pub fn CostPolicy(&self) -> ::windows::core::Result<BackgroundTransferCostPolicy> {
        let this = &::windows::core::Interface::cast::<IBackgroundTransferOperation>(self)?;
        unsafe {
            let mut result__: BackgroundTransferCostPolicy = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BackgroundTransferCostPolicy>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
    pub fn SetCostPolicy(&self, value: BackgroundTransferCostPolicy) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBackgroundTransferOperation>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn GetResultStreamAt(&self, position: u64) -> ::windows::core::Result<super::super::Storage::Streams::IInputStream> {
        let this = &::windows::core::Interface::cast::<IBackgroundTransferOperation>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), position, &mut result__).from_abi::<super::super::Storage::Streams::IInputStream>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
    pub fn GetResponseInformation(&self) -> ::windows::core::Result<ResponseInformation> {
        let this = &::windows::core::Interface::cast::<IBackgroundTransferOperation>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ResponseInformation>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
    pub fn Priority(&self) -> ::windows::core::Result<BackgroundTransferPriority> {
        let this = &::windows::core::Interface::cast::<IBackgroundTransferOperationPriority>(self)?;
        unsafe {
            let mut result__: BackgroundTransferPriority = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BackgroundTransferPriority>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
    pub fn SetPriority(&self, value: BackgroundTransferPriority) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBackgroundTransferOperationPriority>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer', 'Storage'*"]
    #[cfg(feature = "Storage")]
    pub fn SourceFile(&self) -> ::windows::core::Result<super::super::Storage::IStorageFile> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::IStorageFile>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
    pub fn Progress(&self) -> ::windows::core::Result<BackgroundUploadProgress> {
        let this = self;
        unsafe {
            let mut result__: BackgroundUploadProgress = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BackgroundUploadProgress>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn StartAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<UploadOperation, UploadOperation>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<UploadOperation, UploadOperation>>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn AttachAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<UploadOperation, UploadOperation>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<UploadOperation, UploadOperation>>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
    pub fn TransferGroup(&self) -> ::windows::core::Result<BackgroundTransferGroup> {
        let this = &::windows::core::Interface::cast::<IUploadOperation2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BackgroundTransferGroup>(result__)
        }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
    pub fn MakeCurrentInTransferGroup(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IUploadOperation3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
    pub fn SetRequestHeader<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, headername: Param0, headervalue: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IUploadOperation4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), headername.into_param().abi(), headervalue.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Networking_BackgroundTransfer'*"]
    pub fn RemoveRequestHeader<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, headername: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IUploadOperation4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), headername.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for UploadOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UploadOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UploadOperation {}
impl ::core::fmt::Debug for UploadOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UploadOperation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UploadOperation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.BackgroundTransfer.UploadOperation;{3e5624e0-7389-434c-8b35-427fd36bbdae})");
}
unsafe impl ::windows::core::Interface for UploadOperation {
    type Vtable = IUploadOperationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3e5624e0_7389_434c_8b35_427fd36bbdae);
}
impl ::windows::core::RuntimeName for UploadOperation {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.UploadOperation";
}
impl ::core::convert::From<UploadOperation> for ::windows::core::IUnknown {
    fn from(value: UploadOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UploadOperation> for ::windows::core::IUnknown {
    fn from(value: &UploadOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UploadOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &UploadOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UploadOperation> for ::windows::core::IInspectable {
    fn from(value: UploadOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UploadOperation> for ::windows::core::IInspectable {
    fn from(value: &UploadOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UploadOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &UploadOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<UploadOperation> for IBackgroundTransferOperation {
    type Error = ::windows::core::Error;
    fn try_from(value: UploadOperation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&UploadOperation> for IBackgroundTransferOperation {
    type Error = ::windows::core::Error;
    fn try_from(value: &UploadOperation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTransferOperation> for UploadOperation {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTransferOperation> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTransferOperation> for &UploadOperation {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTransferOperation> {
        ::core::convert::TryInto::<IBackgroundTransferOperation>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<UploadOperation> for IBackgroundTransferOperationPriority {
    type Error = ::windows::core::Error;
    fn try_from(value: UploadOperation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&UploadOperation> for IBackgroundTransferOperationPriority {
    type Error = ::windows::core::Error;
    fn try_from(value: &UploadOperation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTransferOperationPriority> for UploadOperation {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTransferOperationPriority> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundTransferOperationPriority> for &UploadOperation {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundTransferOperationPriority> {
        ::core::convert::TryInto::<IBackgroundTransferOperationPriority>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for UploadOperation {}
unsafe impl ::core::marker::Sync for UploadOperation {}
