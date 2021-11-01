#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Networking_BackgroundTransfer`*"]
pub struct BackgroundDownloadProgress {
    pub BytesReceived: u64,
    pub TotalBytesToReceive: u64,
    pub Status: BackgroundTransferStatus,
    pub HasResponseChanged: bool,
    pub HasRestarted: bool,
}
impl BackgroundDownloadProgress {}
impl ::std::default::Default for BackgroundDownloadProgress {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for BackgroundDownloadProgress {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("BackgroundDownloadProgress").field("BytesReceived", &self.BytesReceived).field("TotalBytesToReceive", &self.TotalBytesToReceive).field("Status", &self.Status).field("HasResponseChanged", &self.HasResponseChanged).field("HasRestarted", &self.HasRestarted).finish()
    }
}
impl ::std::cmp::PartialEq for BackgroundDownloadProgress {
    fn eq(&self, other: &Self) -> bool {
        self.BytesReceived == other.BytesReceived && self.TotalBytesToReceive == other.TotalBytesToReceive && self.Status == other.Status && self.HasResponseChanged == other.HasResponseChanged && self.HasRestarted == other.HasRestarted
    }
}
impl ::std::cmp::Eq for BackgroundDownloadProgress {}
unsafe impl ::windows::runtime::Abi for BackgroundDownloadProgress {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for BackgroundDownloadProgress {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"struct(Windows.Networking.BackgroundTransfer.BackgroundDownloadProgress;u8;u8;enum(Windows.Networking.BackgroundTransfer.BackgroundTransferStatus;i4);b1;b1)");
}
#[doc = "*Required features: `Networking_BackgroundTransfer`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct BackgroundDownloader(::windows::runtime::IInspectable);
impl BackgroundDownloader {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<BackgroundDownloader, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    #[doc = "*Required features: `Networking_BackgroundTransfer`, `Foundation`, `Storage`*"]
    pub fn CreateDownload<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>, Param1: ::windows::runtime::IntoParam<'a, super::super::Storage::IStorageFile>>(&self, uri: Param0, resultfile: Param1) -> ::windows::runtime::Result<DownloadOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), uri.into_param().abi(), resultfile.into_param().abi(), &mut result__).from_abi::<DownloadOperation>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    #[doc = "*Required features: `Networking_BackgroundTransfer`, `Foundation`, `Storage`*"]
    pub fn CreateDownloadFromFile<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>, Param1: ::windows::runtime::IntoParam<'a, super::super::Storage::IStorageFile>, Param2: ::windows::runtime::IntoParam<'a, super::super::Storage::IStorageFile>>(&self, uri: Param0, resultfile: Param1, requestbodyfile: Param2) -> ::windows::runtime::Result<DownloadOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), uri.into_param().abi(), resultfile.into_param().abi(), requestbodyfile.into_param().abi(), &mut result__).from_abi::<DownloadOperation>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Networking_BackgroundTransfer`, `Foundation`, `Storage`, `Storage_Streams`*"]
    pub fn CreateDownloadAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>, Param1: ::windows::runtime::IntoParam<'a, super::super::Storage::IStorageFile>, Param2: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IInputStream>>(&self, uri: Param0, resultfile: Param1, requestbodystream: Param2) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<DownloadOperation>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), uri.into_param().abi(), resultfile.into_param().abi(), requestbodystream.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<DownloadOperation>>(result__)
        }
    }
    #[doc = "*Required features: `Networking_BackgroundTransfer`*"]
    pub fn TransferGroup(&self) -> ::windows::runtime::Result<BackgroundTransferGroup> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundDownloader2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<BackgroundTransferGroup>(result__)
        }
    }
    #[doc = "*Required features: `Networking_BackgroundTransfer`*"]
    pub fn SetTransferGroup<'a, Param0: ::windows::runtime::IntoParam<'a, BackgroundTransferGroup>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundDownloader2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Notifications")]
    #[doc = "*Required features: `Networking_BackgroundTransfer`, `UI_Notifications`*"]
    pub fn SuccessToastNotification(&self) -> ::windows::runtime::Result<super::super::UI::Notifications::ToastNotification> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundDownloader2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::Notifications::ToastNotification>(result__)
        }
    }
    #[cfg(feature = "UI_Notifications")]
    #[doc = "*Required features: `Networking_BackgroundTransfer`, `UI_Notifications`*"]
    pub fn SetSuccessToastNotification<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::UI::Notifications::ToastNotification>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundDownloader2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Notifications")]
    #[doc = "*Required features: `Networking_BackgroundTransfer`, `UI_Notifications`*"]
    pub fn FailureToastNotification(&self) -> ::windows::runtime::Result<super::super::UI::Notifications::ToastNotification> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundDownloader2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::Notifications::ToastNotification>(result__)
        }
    }
    #[cfg(feature = "UI_Notifications")]
    #[doc = "*Required features: `Networking_BackgroundTransfer`, `UI_Notifications`*"]
    pub fn SetFailureToastNotification<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::UI::Notifications::ToastNotification>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundDownloader2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Notifications")]
    #[doc = "*Required features: `Networking_BackgroundTransfer`, `UI_Notifications`*"]
    pub fn SuccessTileNotification(&self) -> ::windows::runtime::Result<super::super::UI::Notifications::TileNotification> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundDownloader2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::Notifications::TileNotification>(result__)
        }
    }
    #[cfg(feature = "UI_Notifications")]
    #[doc = "*Required features: `Networking_BackgroundTransfer`, `UI_Notifications`*"]
    pub fn SetSuccessTileNotification<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::UI::Notifications::TileNotification>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundDownloader2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Notifications")]
    #[doc = "*Required features: `Networking_BackgroundTransfer`, `UI_Notifications`*"]
    pub fn FailureTileNotification(&self) -> ::windows::runtime::Result<super::super::UI::Notifications::TileNotification> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundDownloader2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::Notifications::TileNotification>(result__)
        }
    }
    #[cfg(feature = "UI_Notifications")]
    #[doc = "*Required features: `Networking_BackgroundTransfer`, `UI_Notifications`*"]
    pub fn SetFailureTileNotification<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::UI::Notifications::TileNotification>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundDownloader2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Networking_BackgroundTransfer`*"]
    pub fn CompletionGroup(&self) -> ::windows::runtime::Result<BackgroundTransferCompletionGroup> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundDownloader3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<BackgroundTransferCompletionGroup>(result__)
        }
    }
    #[doc = "*Required features: `Networking_BackgroundTransfer`*"]
    pub fn SetRequestHeader<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, headername: Param0, headervalue: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundTransferBase>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), headername.into_param().abi(), headervalue.into_param().abi()).ok() }
    }
    #[cfg(feature = "Security_Credentials")]
    #[doc = "*Required features: `Networking_BackgroundTransfer`, `Security_Credentials`*"]
    pub fn ServerCredential(&self) -> ::windows::runtime::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundTransferBase>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Security::Credentials::PasswordCredential>(result__)
        }
    }
    #[cfg(feature = "Security_Credentials")]
    #[doc = "*Required features: `Networking_BackgroundTransfer`, `Security_Credentials`*"]
    pub fn SetServerCredential<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Security::Credentials::PasswordCredential>>(&self, credential: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundTransferBase>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), credential.into_param().abi()).ok() }
    }
    #[cfg(feature = "Security_Credentials")]
    #[doc = "*Required features: `Networking_BackgroundTransfer`, `Security_Credentials`*"]
    pub fn ProxyCredential(&self) -> ::windows::runtime::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundTransferBase>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Security::Credentials::PasswordCredential>(result__)
        }
    }
    #[cfg(feature = "Security_Credentials")]
    #[doc = "*Required features: `Networking_BackgroundTransfer`, `Security_Credentials`*"]
    pub fn SetProxyCredential<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Security::Credentials::PasswordCredential>>(&self, credential: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundTransferBase>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), credential.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Networking_BackgroundTransfer`*"]
    pub fn Method(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundTransferBase>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Networking_BackgroundTransfer`*"]
    pub fn SetMethod<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundTransferBase>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Networking_BackgroundTransfer`*"]
    pub fn Group(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundTransferBase>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Networking_BackgroundTransfer`*"]
    pub fn SetGroup<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundTransferBase>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Networking_BackgroundTransfer`*"]
    pub fn CostPolicy(&self) -> ::windows::runtime::Result<BackgroundTransferCostPolicy> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundTransferBase>(self)?;
        unsafe {
            let mut result__: BackgroundTransferCostPolicy = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<BackgroundTransferCostPolicy>(result__)
        }
    }
    #[doc = "*Required features: `Networking_BackgroundTransfer`*"]
    pub fn SetCostPolicy(&self, value: BackgroundTransferCostPolicy) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundTransferBase>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Networking_BackgroundTransfer`*"]
    pub fn CreateWithCompletionGroup<'a, Param0: ::windows::runtime::IntoParam<'a, BackgroundTransferCompletionGroup>>(completiongroup: Param0) -> ::windows::runtime::Result<BackgroundDownloader> {
        Self::IBackgroundDownloaderFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), completiongroup.into_param().abi(), &mut result__).from_abi::<BackgroundDownloader>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Networking_BackgroundTransfer`, `Foundation`, `Foundation_Collections`*"]
    pub fn GetCurrentDownloadsAsync() -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<DownloadOperation>>> {
        Self::IBackgroundDownloaderStaticMethods(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<DownloadOperation>>>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Networking_BackgroundTransfer`, `Foundation`, `Foundation_Collections`*"]
    pub fn GetCurrentDownloadsForGroupAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(group: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<DownloadOperation>>> {
        Self::IBackgroundDownloaderStaticMethods(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), group.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<DownloadOperation>>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Networking_BackgroundTransfer`, `Foundation`, `Foundation_Collections`*"]
    pub fn GetCurrentDownloadsForTransferGroupAsync<'a, Param0: ::windows::runtime::IntoParam<'a, BackgroundTransferGroup>>(group: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<DownloadOperation>>> {
        Self::IBackgroundDownloaderStaticMethods2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), group.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<DownloadOperation>>>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Networking_BackgroundTransfer`, `Foundation`, `Foundation_Collections`*"]
    pub fn RequestUnconstrainedDownloadsAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<DownloadOperation>>>(operations: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<UnconstrainedTransferRequestResult>> {
        Self::IBackgroundDownloaderUserConsent(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), operations.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<UnconstrainedTransferRequestResult>>(result__)
        })
    }
    pub fn IBackgroundDownloaderFactory<R, F: FnOnce(&IBackgroundDownloaderFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<BackgroundDownloader, IBackgroundDownloaderFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IBackgroundDownloaderStaticMethods<R, F: FnOnce(&IBackgroundDownloaderStaticMethods) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<BackgroundDownloader, IBackgroundDownloaderStaticMethods> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IBackgroundDownloaderStaticMethods2<R, F: FnOnce(&IBackgroundDownloaderStaticMethods2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<BackgroundDownloader, IBackgroundDownloaderStaticMethods2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IBackgroundDownloaderUserConsent<R, F: FnOnce(&IBackgroundDownloaderUserConsent) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<BackgroundDownloader, IBackgroundDownloaderUserConsent> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for BackgroundDownloader {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.BackgroundTransfer.BackgroundDownloader;{c1c79333-6649-4b1d-a826-a4b3dd234d0b})");
}
unsafe impl ::windows::runtime::Interface for BackgroundDownloader {
    type Vtable = IBackgroundDownloader_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3251082035, 26185, 19229, [168, 38, 164, 179, 221, 35, 77, 11]);
}
impl ::windows::runtime::RuntimeName for BackgroundDownloader {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.BackgroundDownloader";
}
impl ::std::convert::From<BackgroundDownloader> for ::windows::runtime::IUnknown {
    fn from(value: BackgroundDownloader) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&BackgroundDownloader> for ::windows::runtime::IUnknown {
    fn from(value: &BackgroundDownloader) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for BackgroundDownloader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &BackgroundDownloader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<BackgroundDownloader> for ::windows::runtime::IInspectable {
    fn from(value: BackgroundDownloader) -> Self {
        value.0
    }
}
impl ::std::convert::From<&BackgroundDownloader> for ::windows::runtime::IInspectable {
    fn from(value: &BackgroundDownloader) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for BackgroundDownloader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a BackgroundDownloader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<BackgroundDownloader> for IBackgroundTransferBase {
    type Error = ::windows::runtime::Error;
    fn try_from(value: BackgroundDownloader) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&BackgroundDownloader> for IBackgroundTransferBase {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &BackgroundDownloader) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTransferBase> for BackgroundDownloader {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTransferBase> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTransferBase> for &BackgroundDownloader {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTransferBase> {
        ::std::convert::TryInto::<IBackgroundTransferBase>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for BackgroundDownloader {}
unsafe impl ::std::marker::Sync for BackgroundDownloader {}
#[doc = "*Required features: `Networking_BackgroundTransfer`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct BackgroundTransferBehavior(pub i32);
impl BackgroundTransferBehavior {
    pub const Parallel: BackgroundTransferBehavior = BackgroundTransferBehavior(0i32);
    pub const Serialized: BackgroundTransferBehavior = BackgroundTransferBehavior(1i32);
}
impl ::std::convert::From<i32> for BackgroundTransferBehavior {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for BackgroundTransferBehavior {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for BackgroundTransferBehavior {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Networking.BackgroundTransfer.BackgroundTransferBehavior;i4)");
}
#[doc = "*Required features: `Networking_BackgroundTransfer`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct BackgroundTransferCompletionGroup(::windows::runtime::IInspectable);
impl BackgroundTransferCompletionGroup {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<BackgroundTransferCompletionGroup, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "ApplicationModel_Background")]
    #[doc = "*Required features: `Networking_BackgroundTransfer`, `ApplicationModel_Background`*"]
    pub fn Trigger(&self) -> ::windows::runtime::Result<super::super::ApplicationModel::Background::IBackgroundTrigger> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::ApplicationModel::Background::IBackgroundTrigger>(result__)
        }
    }
    #[doc = "*Required features: `Networking_BackgroundTransfer`*"]
    pub fn IsEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Networking_BackgroundTransfer`*"]
    pub fn Enable(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for BackgroundTransferCompletionGroup {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.BackgroundTransfer.BackgroundTransferCompletionGroup;{2d930225-986b-574d-7950-0add47f5d706})");
}
unsafe impl ::windows::runtime::Interface for BackgroundTransferCompletionGroup {
    type Vtable = IBackgroundTransferCompletionGroup_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(764609061, 39019, 22349, [121, 80, 10, 221, 71, 245, 215, 6]);
}
impl ::windows::runtime::RuntimeName for BackgroundTransferCompletionGroup {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.BackgroundTransferCompletionGroup";
}
impl ::std::convert::From<BackgroundTransferCompletionGroup> for ::windows::runtime::IUnknown {
    fn from(value: BackgroundTransferCompletionGroup) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&BackgroundTransferCompletionGroup> for ::windows::runtime::IUnknown {
    fn from(value: &BackgroundTransferCompletionGroup) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for BackgroundTransferCompletionGroup {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &BackgroundTransferCompletionGroup {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<BackgroundTransferCompletionGroup> for ::windows::runtime::IInspectable {
    fn from(value: BackgroundTransferCompletionGroup) -> Self {
        value.0
    }
}
impl ::std::convert::From<&BackgroundTransferCompletionGroup> for ::windows::runtime::IInspectable {
    fn from(value: &BackgroundTransferCompletionGroup) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for BackgroundTransferCompletionGroup {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a BackgroundTransferCompletionGroup {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for BackgroundTransferCompletionGroup {}
unsafe impl ::std::marker::Sync for BackgroundTransferCompletionGroup {}
#[doc = "*Required features: `Networking_BackgroundTransfer`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct BackgroundTransferCompletionGroupTriggerDetails(::windows::runtime::IInspectable);
impl BackgroundTransferCompletionGroupTriggerDetails {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Networking_BackgroundTransfer`, `Foundation_Collections`*"]
    pub fn Downloads(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<DownloadOperation>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<DownloadOperation>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Networking_BackgroundTransfer`, `Foundation_Collections`*"]
    pub fn Uploads(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<UploadOperation>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<UploadOperation>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for BackgroundTransferCompletionGroupTriggerDetails {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.BackgroundTransfer.BackgroundTransferCompletionGroupTriggerDetails;{7b6be286-6e47-5136-7fcb-fa4389f46f5b})");
}
unsafe impl ::windows::runtime::Interface for BackgroundTransferCompletionGroupTriggerDetails {
    type Vtable = IBackgroundTransferCompletionGroupTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2070667910, 28231, 20790, [127, 203, 250, 67, 137, 244, 111, 91]);
}
impl ::windows::runtime::RuntimeName for BackgroundTransferCompletionGroupTriggerDetails {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.BackgroundTransferCompletionGroupTriggerDetails";
}
impl ::std::convert::From<BackgroundTransferCompletionGroupTriggerDetails> for ::windows::runtime::IUnknown {
    fn from(value: BackgroundTransferCompletionGroupTriggerDetails) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&BackgroundTransferCompletionGroupTriggerDetails> for ::windows::runtime::IUnknown {
    fn from(value: &BackgroundTransferCompletionGroupTriggerDetails) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for BackgroundTransferCompletionGroupTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &BackgroundTransferCompletionGroupTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<BackgroundTransferCompletionGroupTriggerDetails> for ::windows::runtime::IInspectable {
    fn from(value: BackgroundTransferCompletionGroupTriggerDetails) -> Self {
        value.0
    }
}
impl ::std::convert::From<&BackgroundTransferCompletionGroupTriggerDetails> for ::windows::runtime::IInspectable {
    fn from(value: &BackgroundTransferCompletionGroupTriggerDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for BackgroundTransferCompletionGroupTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a BackgroundTransferCompletionGroupTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for BackgroundTransferCompletionGroupTriggerDetails {}
unsafe impl ::std::marker::Sync for BackgroundTransferCompletionGroupTriggerDetails {}
#[doc = "*Required features: `Networking_BackgroundTransfer`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct BackgroundTransferContentPart(::windows::runtime::IInspectable);
impl BackgroundTransferContentPart {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<BackgroundTransferContentPart, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Networking_BackgroundTransfer`*"]
    pub fn SetHeader<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, headername: Param0, headervalue: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), headername.into_param().abi(), headervalue.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Networking_BackgroundTransfer`*"]
    pub fn SetText<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage")]
    #[doc = "*Required features: `Networking_BackgroundTransfer`, `Storage`*"]
    pub fn SetFile<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::IStorageFile>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Networking_BackgroundTransfer`*"]
    pub fn CreateWithName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(name: Param0) -> ::windows::runtime::Result<BackgroundTransferContentPart> {
        Self::IBackgroundTransferContentPartFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<BackgroundTransferContentPart>(result__)
        })
    }
    #[doc = "*Required features: `Networking_BackgroundTransfer`*"]
    pub fn CreateWithNameAndFileName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(name: Param0, filename: Param1) -> ::windows::runtime::Result<BackgroundTransferContentPart> {
        Self::IBackgroundTransferContentPartFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), name.into_param().abi(), filename.into_param().abi(), &mut result__).from_abi::<BackgroundTransferContentPart>(result__)
        })
    }
    pub fn IBackgroundTransferContentPartFactory<R, F: FnOnce(&IBackgroundTransferContentPartFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<BackgroundTransferContentPart, IBackgroundTransferContentPartFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for BackgroundTransferContentPart {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.BackgroundTransfer.BackgroundTransferContentPart;{e8e15657-d7d1-4ed8-838e-674ac217ace6})");
}
unsafe impl ::windows::runtime::Interface for BackgroundTransferContentPart {
    type Vtable = IBackgroundTransferContentPart_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3907081815, 55249, 20184, [131, 142, 103, 74, 194, 23, 172, 230]);
}
impl ::windows::runtime::RuntimeName for BackgroundTransferContentPart {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.BackgroundTransferContentPart";
}
impl ::std::convert::From<BackgroundTransferContentPart> for ::windows::runtime::IUnknown {
    fn from(value: BackgroundTransferContentPart) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&BackgroundTransferContentPart> for ::windows::runtime::IUnknown {
    fn from(value: &BackgroundTransferContentPart) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for BackgroundTransferContentPart {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &BackgroundTransferContentPart {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<BackgroundTransferContentPart> for ::windows::runtime::IInspectable {
    fn from(value: BackgroundTransferContentPart) -> Self {
        value.0
    }
}
impl ::std::convert::From<&BackgroundTransferContentPart> for ::windows::runtime::IInspectable {
    fn from(value: &BackgroundTransferContentPart) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for BackgroundTransferContentPart {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a BackgroundTransferContentPart {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for BackgroundTransferContentPart {}
unsafe impl ::std::marker::Sync for BackgroundTransferContentPart {}
#[doc = "*Required features: `Networking_BackgroundTransfer`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct BackgroundTransferCostPolicy(pub i32);
impl BackgroundTransferCostPolicy {
    pub const Default: BackgroundTransferCostPolicy = BackgroundTransferCostPolicy(0i32);
    pub const UnrestrictedOnly: BackgroundTransferCostPolicy = BackgroundTransferCostPolicy(1i32);
    pub const Always: BackgroundTransferCostPolicy = BackgroundTransferCostPolicy(2i32);
}
impl ::std::convert::From<i32> for BackgroundTransferCostPolicy {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for BackgroundTransferCostPolicy {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for BackgroundTransferCostPolicy {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Networking.BackgroundTransfer.BackgroundTransferCostPolicy;i4)");
}
#[doc = "*Required features: `Networking_BackgroundTransfer`*"]
pub struct BackgroundTransferError {}
impl BackgroundTransferError {
    #[cfg(feature = "Web")]
    #[doc = "*Required features: `Networking_BackgroundTransfer`, `Web`*"]
    pub fn GetStatus(hresult: i32) -> ::windows::runtime::Result<super::super::Web::WebErrorStatus> {
        Self::IBackgroundTransferErrorStaticMethods(|this| unsafe {
            let mut result__: super::super::Web::WebErrorStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), hresult, &mut result__).from_abi::<super::super::Web::WebErrorStatus>(result__)
        })
    }
    pub fn IBackgroundTransferErrorStaticMethods<R, F: FnOnce(&IBackgroundTransferErrorStaticMethods) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<BackgroundTransferError, IBackgroundTransferErrorStaticMethods> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for BackgroundTransferError {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.BackgroundTransferError";
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Networking_BackgroundTransfer`*"]
pub struct BackgroundTransferFileRange {
    pub Offset: u64,
    pub Length: u64,
}
impl BackgroundTransferFileRange {}
impl ::std::default::Default for BackgroundTransferFileRange {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for BackgroundTransferFileRange {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("BackgroundTransferFileRange").field("Offset", &self.Offset).field("Length", &self.Length).finish()
    }
}
impl ::std::cmp::PartialEq for BackgroundTransferFileRange {
    fn eq(&self, other: &Self) -> bool {
        self.Offset == other.Offset && self.Length == other.Length
    }
}
impl ::std::cmp::Eq for BackgroundTransferFileRange {}
unsafe impl ::windows::runtime::Abi for BackgroundTransferFileRange {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for BackgroundTransferFileRange {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"struct(Windows.Networking.BackgroundTransfer.BackgroundTransferFileRange;u8;u8)");
}
#[doc = "*Required features: `Networking_BackgroundTransfer`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct BackgroundTransferGroup(::windows::runtime::IInspectable);
impl BackgroundTransferGroup {
    #[doc = "*Required features: `Networking_BackgroundTransfer`*"]
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Networking_BackgroundTransfer`*"]
    pub fn TransferBehavior(&self) -> ::windows::runtime::Result<BackgroundTransferBehavior> {
        let this = self;
        unsafe {
            let mut result__: BackgroundTransferBehavior = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<BackgroundTransferBehavior>(result__)
        }
    }
    #[doc = "*Required features: `Networking_BackgroundTransfer`*"]
    pub fn SetTransferBehavior(&self, value: BackgroundTransferBehavior) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Networking_BackgroundTransfer`*"]
    pub fn CreateGroup<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(name: Param0) -> ::windows::runtime::Result<BackgroundTransferGroup> {
        Self::IBackgroundTransferGroupStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<BackgroundTransferGroup>(result__)
        })
    }
    pub fn IBackgroundTransferGroupStatics<R, F: FnOnce(&IBackgroundTransferGroupStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<BackgroundTransferGroup, IBackgroundTransferGroupStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for BackgroundTransferGroup {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.BackgroundTransfer.BackgroundTransferGroup;{d8c3e3e4-6459-4540-85eb-aaa1c8903677})");
}
unsafe impl ::windows::runtime::Interface for BackgroundTransferGroup {
    type Vtable = IBackgroundTransferGroup_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3636716516, 25689, 17728, [133, 235, 170, 161, 200, 144, 54, 119]);
}
impl ::windows::runtime::RuntimeName for BackgroundTransferGroup {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.BackgroundTransferGroup";
}
impl ::std::convert::From<BackgroundTransferGroup> for ::windows::runtime::IUnknown {
    fn from(value: BackgroundTransferGroup) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&BackgroundTransferGroup> for ::windows::runtime::IUnknown {
    fn from(value: &BackgroundTransferGroup) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for BackgroundTransferGroup {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &BackgroundTransferGroup {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<BackgroundTransferGroup> for ::windows::runtime::IInspectable {
    fn from(value: BackgroundTransferGroup) -> Self {
        value.0
    }
}
impl ::std::convert::From<&BackgroundTransferGroup> for ::windows::runtime::IInspectable {
    fn from(value: &BackgroundTransferGroup) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for BackgroundTransferGroup {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a BackgroundTransferGroup {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for BackgroundTransferGroup {}
unsafe impl ::std::marker::Sync for BackgroundTransferGroup {}
#[doc = "*Required features: `Networking_BackgroundTransfer`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct BackgroundTransferPriority(pub i32);
impl BackgroundTransferPriority {
    pub const Default: BackgroundTransferPriority = BackgroundTransferPriority(0i32);
    pub const High: BackgroundTransferPriority = BackgroundTransferPriority(1i32);
    pub const Low: BackgroundTransferPriority = BackgroundTransferPriority(2i32);
}
impl ::std::convert::From<i32> for BackgroundTransferPriority {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for BackgroundTransferPriority {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for BackgroundTransferPriority {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Networking.BackgroundTransfer.BackgroundTransferPriority;i4)");
}
#[doc = "*Required features: `Networking_BackgroundTransfer`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct BackgroundTransferRangesDownloadedEventArgs(::windows::runtime::IInspectable);
impl BackgroundTransferRangesDownloadedEventArgs {
    #[doc = "*Required features: `Networking_BackgroundTransfer`*"]
    pub fn WasDownloadRestarted(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Networking_BackgroundTransfer`, `Foundation_Collections`*"]
    pub fn AddedRanges(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<BackgroundTransferFileRange>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<BackgroundTransferFileRange>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_BackgroundTransfer`, `Foundation`*"]
    pub fn GetDeferral(&self) -> ::windows::runtime::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for BackgroundTransferRangesDownloadedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.BackgroundTransfer.BackgroundTransferRangesDownloadedEventArgs;{3ebc7453-bf48-4a88-9248-b0c165184f5c})");
}
unsafe impl ::windows::runtime::Interface for BackgroundTransferRangesDownloadedEventArgs {
    type Vtable = IBackgroundTransferRangesDownloadedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1052537939, 48968, 19080, [146, 72, 176, 193, 101, 24, 79, 92]);
}
impl ::windows::runtime::RuntimeName for BackgroundTransferRangesDownloadedEventArgs {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.BackgroundTransferRangesDownloadedEventArgs";
}
impl ::std::convert::From<BackgroundTransferRangesDownloadedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: BackgroundTransferRangesDownloadedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&BackgroundTransferRangesDownloadedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &BackgroundTransferRangesDownloadedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for BackgroundTransferRangesDownloadedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &BackgroundTransferRangesDownloadedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<BackgroundTransferRangesDownloadedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: BackgroundTransferRangesDownloadedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&BackgroundTransferRangesDownloadedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &BackgroundTransferRangesDownloadedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for BackgroundTransferRangesDownloadedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a BackgroundTransferRangesDownloadedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for BackgroundTransferRangesDownloadedEventArgs {}
unsafe impl ::std::marker::Sync for BackgroundTransferRangesDownloadedEventArgs {}
#[doc = "*Required features: `Networking_BackgroundTransfer`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct BackgroundTransferStatus(pub i32);
impl BackgroundTransferStatus {
    pub const Idle: BackgroundTransferStatus = BackgroundTransferStatus(0i32);
    pub const Running: BackgroundTransferStatus = BackgroundTransferStatus(1i32);
    pub const PausedByApplication: BackgroundTransferStatus = BackgroundTransferStatus(2i32);
    pub const PausedCostedNetwork: BackgroundTransferStatus = BackgroundTransferStatus(3i32);
    pub const PausedNoNetwork: BackgroundTransferStatus = BackgroundTransferStatus(4i32);
    pub const Completed: BackgroundTransferStatus = BackgroundTransferStatus(5i32);
    pub const Canceled: BackgroundTransferStatus = BackgroundTransferStatus(6i32);
    pub const Error: BackgroundTransferStatus = BackgroundTransferStatus(7i32);
    pub const PausedRecoverableWebErrorStatus: BackgroundTransferStatus = BackgroundTransferStatus(8i32);
    pub const PausedSystemPolicy: BackgroundTransferStatus = BackgroundTransferStatus(32i32);
}
impl ::std::convert::From<i32> for BackgroundTransferStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for BackgroundTransferStatus {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for BackgroundTransferStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Networking.BackgroundTransfer.BackgroundTransferStatus;i4)");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Networking_BackgroundTransfer`*"]
pub struct BackgroundUploadProgress {
    pub BytesReceived: u64,
    pub BytesSent: u64,
    pub TotalBytesToReceive: u64,
    pub TotalBytesToSend: u64,
    pub Status: BackgroundTransferStatus,
    pub HasResponseChanged: bool,
    pub HasRestarted: bool,
}
impl BackgroundUploadProgress {}
impl ::std::default::Default for BackgroundUploadProgress {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for BackgroundUploadProgress {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("BackgroundUploadProgress")
            .field("BytesReceived", &self.BytesReceived)
            .field("BytesSent", &self.BytesSent)
            .field("TotalBytesToReceive", &self.TotalBytesToReceive)
            .field("TotalBytesToSend", &self.TotalBytesToSend)
            .field("Status", &self.Status)
            .field("HasResponseChanged", &self.HasResponseChanged)
            .field("HasRestarted", &self.HasRestarted)
            .finish()
    }
}
impl ::std::cmp::PartialEq for BackgroundUploadProgress {
    fn eq(&self, other: &Self) -> bool {
        self.BytesReceived == other.BytesReceived && self.BytesSent == other.BytesSent && self.TotalBytesToReceive == other.TotalBytesToReceive && self.TotalBytesToSend == other.TotalBytesToSend && self.Status == other.Status && self.HasResponseChanged == other.HasResponseChanged && self.HasRestarted == other.HasRestarted
    }
}
impl ::std::cmp::Eq for BackgroundUploadProgress {}
unsafe impl ::windows::runtime::Abi for BackgroundUploadProgress {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for BackgroundUploadProgress {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"struct(Windows.Networking.BackgroundTransfer.BackgroundUploadProgress;u8;u8;u8;u8;enum(Windows.Networking.BackgroundTransfer.BackgroundTransferStatus;i4);b1;b1)");
}
#[doc = "*Required features: `Networking_BackgroundTransfer`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct BackgroundUploader(::windows::runtime::IInspectable);
impl BackgroundUploader {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<BackgroundUploader, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    #[doc = "*Required features: `Networking_BackgroundTransfer`, `Foundation`, `Storage`*"]
    pub fn CreateUpload<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>, Param1: ::windows::runtime::IntoParam<'a, super::super::Storage::IStorageFile>>(&self, uri: Param0, sourcefile: Param1) -> ::windows::runtime::Result<UploadOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), uri.into_param().abi(), sourcefile.into_param().abi(), &mut result__).from_abi::<UploadOperation>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Networking_BackgroundTransfer`, `Foundation`, `Storage_Streams`*"]
    pub fn CreateUploadFromStreamAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>, Param1: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IInputStream>>(&self, uri: Param0, sourcestream: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<UploadOperation>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), uri.into_param().abi(), sourcestream.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<UploadOperation>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Networking_BackgroundTransfer`, `Foundation`, `Foundation_Collections`*"]
    pub fn CreateUploadWithFormDataAndAutoBoundaryAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<BackgroundTransferContentPart>>>(&self, uri: Param0, parts: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<UploadOperation>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), uri.into_param().abi(), parts.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<UploadOperation>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Networking_BackgroundTransfer`, `Foundation`, `Foundation_Collections`*"]
    pub fn CreateUploadWithSubTypeAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<BackgroundTransferContentPart>>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, uri: Param0, parts: Param1, subtype: Param2) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<UploadOperation>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), uri.into_param().abi(), parts.into_param().abi(), subtype.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<UploadOperation>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Networking_BackgroundTransfer`, `Foundation`, `Foundation_Collections`*"]
    pub fn CreateUploadWithSubTypeAndBoundaryAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<BackgroundTransferContentPart>>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        uri: Param0,
        parts: Param1,
        subtype: Param2,
        boundary: Param3,
    ) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<UploadOperation>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), uri.into_param().abi(), parts.into_param().abi(), subtype.into_param().abi(), boundary.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<UploadOperation>>(result__)
        }
    }
    #[doc = "*Required features: `Networking_BackgroundTransfer`*"]
    pub fn SetRequestHeader<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, headername: Param0, headervalue: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundTransferBase>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), headername.into_param().abi(), headervalue.into_param().abi()).ok() }
    }
    #[cfg(feature = "Security_Credentials")]
    #[doc = "*Required features: `Networking_BackgroundTransfer`, `Security_Credentials`*"]
    pub fn ServerCredential(&self) -> ::windows::runtime::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundTransferBase>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Security::Credentials::PasswordCredential>(result__)
        }
    }
    #[cfg(feature = "Security_Credentials")]
    #[doc = "*Required features: `Networking_BackgroundTransfer`, `Security_Credentials`*"]
    pub fn SetServerCredential<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Security::Credentials::PasswordCredential>>(&self, credential: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundTransferBase>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), credential.into_param().abi()).ok() }
    }
    #[cfg(feature = "Security_Credentials")]
    #[doc = "*Required features: `Networking_BackgroundTransfer`, `Security_Credentials`*"]
    pub fn ProxyCredential(&self) -> ::windows::runtime::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundTransferBase>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Security::Credentials::PasswordCredential>(result__)
        }
    }
    #[cfg(feature = "Security_Credentials")]
    #[doc = "*Required features: `Networking_BackgroundTransfer`, `Security_Credentials`*"]
    pub fn SetProxyCredential<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Security::Credentials::PasswordCredential>>(&self, credential: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundTransferBase>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), credential.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Networking_BackgroundTransfer`*"]
    pub fn Method(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundTransferBase>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Networking_BackgroundTransfer`*"]
    pub fn SetMethod<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundTransferBase>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Networking_BackgroundTransfer`*"]
    pub fn Group(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundTransferBase>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Networking_BackgroundTransfer`*"]
    pub fn SetGroup<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundTransferBase>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Networking_BackgroundTransfer`*"]
    pub fn CostPolicy(&self) -> ::windows::runtime::Result<BackgroundTransferCostPolicy> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundTransferBase>(self)?;
        unsafe {
            let mut result__: BackgroundTransferCostPolicy = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<BackgroundTransferCostPolicy>(result__)
        }
    }
    #[doc = "*Required features: `Networking_BackgroundTransfer`*"]
    pub fn SetCostPolicy(&self, value: BackgroundTransferCostPolicy) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundTransferBase>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Networking_BackgroundTransfer`*"]
    pub fn TransferGroup(&self) -> ::windows::runtime::Result<BackgroundTransferGroup> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundUploader2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<BackgroundTransferGroup>(result__)
        }
    }
    #[doc = "*Required features: `Networking_BackgroundTransfer`*"]
    pub fn SetTransferGroup<'a, Param0: ::windows::runtime::IntoParam<'a, BackgroundTransferGroup>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundUploader2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Notifications")]
    #[doc = "*Required features: `Networking_BackgroundTransfer`, `UI_Notifications`*"]
    pub fn SuccessToastNotification(&self) -> ::windows::runtime::Result<super::super::UI::Notifications::ToastNotification> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundUploader2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::Notifications::ToastNotification>(result__)
        }
    }
    #[cfg(feature = "UI_Notifications")]
    #[doc = "*Required features: `Networking_BackgroundTransfer`, `UI_Notifications`*"]
    pub fn SetSuccessToastNotification<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::UI::Notifications::ToastNotification>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundUploader2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Notifications")]
    #[doc = "*Required features: `Networking_BackgroundTransfer`, `UI_Notifications`*"]
    pub fn FailureToastNotification(&self) -> ::windows::runtime::Result<super::super::UI::Notifications::ToastNotification> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundUploader2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::Notifications::ToastNotification>(result__)
        }
    }
    #[cfg(feature = "UI_Notifications")]
    #[doc = "*Required features: `Networking_BackgroundTransfer`, `UI_Notifications`*"]
    pub fn SetFailureToastNotification<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::UI::Notifications::ToastNotification>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundUploader2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Notifications")]
    #[doc = "*Required features: `Networking_BackgroundTransfer`, `UI_Notifications`*"]
    pub fn SuccessTileNotification(&self) -> ::windows::runtime::Result<super::super::UI::Notifications::TileNotification> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundUploader2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::Notifications::TileNotification>(result__)
        }
    }
    #[cfg(feature = "UI_Notifications")]
    #[doc = "*Required features: `Networking_BackgroundTransfer`, `UI_Notifications`*"]
    pub fn SetSuccessTileNotification<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::UI::Notifications::TileNotification>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundUploader2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Notifications")]
    #[doc = "*Required features: `Networking_BackgroundTransfer`, `UI_Notifications`*"]
    pub fn FailureTileNotification(&self) -> ::windows::runtime::Result<super::super::UI::Notifications::TileNotification> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundUploader2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::Notifications::TileNotification>(result__)
        }
    }
    #[cfg(feature = "UI_Notifications")]
    #[doc = "*Required features: `Networking_BackgroundTransfer`, `UI_Notifications`*"]
    pub fn SetFailureTileNotification<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::UI::Notifications::TileNotification>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundUploader2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Networking_BackgroundTransfer`*"]
    pub fn CompletionGroup(&self) -> ::windows::runtime::Result<BackgroundTransferCompletionGroup> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundUploader3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<BackgroundTransferCompletionGroup>(result__)
        }
    }
    #[doc = "*Required features: `Networking_BackgroundTransfer`*"]
    pub fn CreateWithCompletionGroup<'a, Param0: ::windows::runtime::IntoParam<'a, BackgroundTransferCompletionGroup>>(completiongroup: Param0) -> ::windows::runtime::Result<BackgroundUploader> {
        Self::IBackgroundUploaderFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), completiongroup.into_param().abi(), &mut result__).from_abi::<BackgroundUploader>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Networking_BackgroundTransfer`, `Foundation`, `Foundation_Collections`*"]
    pub fn GetCurrentUploadsAsync() -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<UploadOperation>>> {
        Self::IBackgroundUploaderStaticMethods(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<UploadOperation>>>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Networking_BackgroundTransfer`, `Foundation`, `Foundation_Collections`*"]
    pub fn GetCurrentUploadsForGroupAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(group: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<UploadOperation>>> {
        Self::IBackgroundUploaderStaticMethods(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), group.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<UploadOperation>>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Networking_BackgroundTransfer`, `Foundation`, `Foundation_Collections`*"]
    pub fn GetCurrentUploadsForTransferGroupAsync<'a, Param0: ::windows::runtime::IntoParam<'a, BackgroundTransferGroup>>(group: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<UploadOperation>>> {
        Self::IBackgroundUploaderStaticMethods2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), group.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<UploadOperation>>>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Networking_BackgroundTransfer`, `Foundation`, `Foundation_Collections`*"]
    pub fn RequestUnconstrainedUploadsAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<UploadOperation>>>(operations: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<UnconstrainedTransferRequestResult>> {
        Self::IBackgroundUploaderUserConsent(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), operations.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<UnconstrainedTransferRequestResult>>(result__)
        })
    }
    pub fn IBackgroundUploaderFactory<R, F: FnOnce(&IBackgroundUploaderFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<BackgroundUploader, IBackgroundUploaderFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IBackgroundUploaderStaticMethods<R, F: FnOnce(&IBackgroundUploaderStaticMethods) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<BackgroundUploader, IBackgroundUploaderStaticMethods> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IBackgroundUploaderStaticMethods2<R, F: FnOnce(&IBackgroundUploaderStaticMethods2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<BackgroundUploader, IBackgroundUploaderStaticMethods2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IBackgroundUploaderUserConsent<R, F: FnOnce(&IBackgroundUploaderUserConsent) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<BackgroundUploader, IBackgroundUploaderUserConsent> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for BackgroundUploader {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.BackgroundTransfer.BackgroundUploader;{c595c9ae-cead-465b-8801-c55ac90a01ce})");
}
unsafe impl ::windows::runtime::Interface for BackgroundUploader {
    type Vtable = IBackgroundUploader_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3314928046, 52909, 18011, [136, 1, 197, 90, 201, 10, 1, 206]);
}
impl ::windows::runtime::RuntimeName for BackgroundUploader {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.BackgroundUploader";
}
impl ::std::convert::From<BackgroundUploader> for ::windows::runtime::IUnknown {
    fn from(value: BackgroundUploader) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&BackgroundUploader> for ::windows::runtime::IUnknown {
    fn from(value: &BackgroundUploader) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for BackgroundUploader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &BackgroundUploader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<BackgroundUploader> for ::windows::runtime::IInspectable {
    fn from(value: BackgroundUploader) -> Self {
        value.0
    }
}
impl ::std::convert::From<&BackgroundUploader> for ::windows::runtime::IInspectable {
    fn from(value: &BackgroundUploader) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for BackgroundUploader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a BackgroundUploader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<BackgroundUploader> for IBackgroundTransferBase {
    type Error = ::windows::runtime::Error;
    fn try_from(value: BackgroundUploader) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&BackgroundUploader> for IBackgroundTransferBase {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &BackgroundUploader) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTransferBase> for BackgroundUploader {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTransferBase> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTransferBase> for &BackgroundUploader {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTransferBase> {
        ::std::convert::TryInto::<IBackgroundTransferBase>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for BackgroundUploader {}
unsafe impl ::std::marker::Sync for BackgroundUploader {}
#[doc = "*Required features: `Networking_BackgroundTransfer`*"]
pub struct ContentPrefetcher {}
impl ContentPrefetcher {
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Networking_BackgroundTransfer`, `Foundation`, `Foundation_Collections`*"]
    pub fn ContentUris() -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>> {
        Self::IContentPrefetcher(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_BackgroundTransfer`, `Foundation`*"]
    pub fn SetIndirectContentUri<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>>(value: Param0) -> ::windows::runtime::Result<()> {
        Self::IContentPrefetcher(|this| unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_BackgroundTransfer`, `Foundation`*"]
    pub fn IndirectContentUri() -> ::windows::runtime::Result<super::super::Foundation::Uri> {
        Self::IContentPrefetcher(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_BackgroundTransfer`, `Foundation`*"]
    pub fn LastSuccessfulPrefetchTime() -> ::windows::runtime::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        Self::IContentPrefetcherTime(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>(result__)
        })
    }
    pub fn IContentPrefetcher<R, F: FnOnce(&IContentPrefetcher) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ContentPrefetcher, IContentPrefetcher> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IContentPrefetcherTime<R, F: FnOnce(&IContentPrefetcherTime) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ContentPrefetcher, IContentPrefetcherTime> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for ContentPrefetcher {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.ContentPrefetcher";
}
#[doc = "*Required features: `Networking_BackgroundTransfer`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct DownloadOperation(::windows::runtime::IInspectable);
impl DownloadOperation {
    #[cfg(feature = "Storage")]
    #[doc = "*Required features: `Networking_BackgroundTransfer`, `Storage`*"]
    pub fn ResultFile(&self) -> ::windows::runtime::Result<super::super::Storage::IStorageFile> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::IStorageFile>(result__)
        }
    }
    #[doc = "*Required features: `Networking_BackgroundTransfer`*"]
    pub fn Progress(&self) -> ::windows::runtime::Result<BackgroundDownloadProgress> {
        let this = self;
        unsafe {
            let mut result__: BackgroundDownloadProgress = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<BackgroundDownloadProgress>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_BackgroundTransfer`, `Foundation`*"]
    pub fn StartAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperationWithProgress<DownloadOperation, DownloadOperation>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<DownloadOperation, DownloadOperation>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_BackgroundTransfer`, `Foundation`*"]
    pub fn AttachAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperationWithProgress<DownloadOperation, DownloadOperation>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<DownloadOperation, DownloadOperation>>(result__)
        }
    }
    #[doc = "*Required features: `Networking_BackgroundTransfer`*"]
    pub fn Pause(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Networking_BackgroundTransfer`*"]
    pub fn Resume(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Networking_BackgroundTransfer`*"]
    pub fn Guid(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundTransferOperation>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_BackgroundTransfer`, `Foundation`*"]
    pub fn RequestedUri(&self) -> ::windows::runtime::Result<super::super::Foundation::Uri> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundTransferOperation>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `Networking_BackgroundTransfer`*"]
    pub fn Method(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundTransferOperation>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Networking_BackgroundTransfer`*"]
    pub fn Group(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundTransferOperation>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Networking_BackgroundTransfer`*"]
    pub fn CostPolicy(&self) -> ::windows::runtime::Result<BackgroundTransferCostPolicy> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundTransferOperation>(self)?;
        unsafe {
            let mut result__: BackgroundTransferCostPolicy = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<BackgroundTransferCostPolicy>(result__)
        }
    }
    #[doc = "*Required features: `Networking_BackgroundTransfer`*"]
    pub fn SetCostPolicy(&self, value: BackgroundTransferCostPolicy) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundTransferOperation>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Networking_BackgroundTransfer`, `Storage_Streams`*"]
    pub fn GetResultStreamAt(&self, position: u64) -> ::windows::runtime::Result<super::super::Storage::Streams::IInputStream> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundTransferOperation>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), position, &mut result__).from_abi::<super::super::Storage::Streams::IInputStream>(result__)
        }
    }
    #[doc = "*Required features: `Networking_BackgroundTransfer`*"]
    pub fn GetResponseInformation(&self) -> ::windows::runtime::Result<ResponseInformation> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundTransferOperation>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ResponseInformation>(result__)
        }
    }
    #[doc = "*Required features: `Networking_BackgroundTransfer`*"]
    pub fn Priority(&self) -> ::windows::runtime::Result<BackgroundTransferPriority> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundTransferOperationPriority>(self)?;
        unsafe {
            let mut result__: BackgroundTransferPriority = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<BackgroundTransferPriority>(result__)
        }
    }
    #[doc = "*Required features: `Networking_BackgroundTransfer`*"]
    pub fn SetPriority(&self, value: BackgroundTransferPriority) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundTransferOperationPriority>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Networking_BackgroundTransfer`*"]
    pub fn TransferGroup(&self) -> ::windows::runtime::Result<BackgroundTransferGroup> {
        let this = &::windows::runtime::Interface::cast::<IDownloadOperation2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<BackgroundTransferGroup>(result__)
        }
    }
    #[doc = "*Required features: `Networking_BackgroundTransfer`*"]
    pub fn IsRandomAccessRequired(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IDownloadOperation3>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Networking_BackgroundTransfer`*"]
    pub fn SetIsRandomAccessRequired(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IDownloadOperation3>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Networking_BackgroundTransfer`, `Storage_Streams`*"]
    pub fn GetResultRandomAccessStreamReference(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = &::windows::runtime::Interface::cast::<IDownloadOperation3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IRandomAccessStreamReference>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Networking_BackgroundTransfer`, `Foundation_Collections`*"]
    pub fn GetDownloadedRanges(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<BackgroundTransferFileRange>> {
        let this = &::windows::runtime::Interface::cast::<IDownloadOperation3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<BackgroundTransferFileRange>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_BackgroundTransfer`, `Foundation`*"]
    pub fn RangesDownloaded<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<DownloadOperation, BackgroundTransferRangesDownloadedEventArgs>>>(&self, eventhandler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<IDownloadOperation3>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), eventhandler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_BackgroundTransfer`, `Foundation`*"]
    pub fn RemoveRangesDownloaded<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IDownloadOperation3>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), eventcookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_BackgroundTransfer`, `Foundation`*"]
    pub fn SetRequestedUri<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IDownloadOperation3>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Web"))]
    #[doc = "*Required features: `Networking_BackgroundTransfer`, `Foundation_Collections`, `Web`*"]
    pub fn RecoverableWebErrorStatuses(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<super::super::Web::WebErrorStatus>> {
        let this = &::windows::runtime::Interface::cast::<IDownloadOperation3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::super::Web::WebErrorStatus>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Web"))]
    #[doc = "*Required features: `Networking_BackgroundTransfer`, `Foundation`, `Web`*"]
    pub fn CurrentWebErrorStatus(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<super::super::Web::WebErrorStatus>> {
        let this = &::windows::runtime::Interface::cast::<IDownloadOperation3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Web::WebErrorStatus>>(result__)
        }
    }
    #[doc = "*Required features: `Networking_BackgroundTransfer`*"]
    pub fn MakeCurrentInTransferGroup(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IDownloadOperation4>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Networking_BackgroundTransfer`*"]
    pub fn SetRequestHeader<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, headername: Param0, headervalue: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IDownloadOperation5>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), headername.into_param().abi(), headervalue.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Networking_BackgroundTransfer`*"]
    pub fn RemoveRequestHeader<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, headername: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IDownloadOperation5>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), headername.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DownloadOperation {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.BackgroundTransfer.DownloadOperation;{bd87ebb0-5714-4e09-ba68-bef73903b0d7})");
}
unsafe impl ::windows::runtime::Interface for DownloadOperation {
    type Vtable = IDownloadOperation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3179801520, 22292, 19977, [186, 104, 190, 247, 57, 3, 176, 215]);
}
impl ::windows::runtime::RuntimeName for DownloadOperation {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.DownloadOperation";
}
impl ::std::convert::From<DownloadOperation> for ::windows::runtime::IUnknown {
    fn from(value: DownloadOperation) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&DownloadOperation> for ::windows::runtime::IUnknown {
    fn from(value: &DownloadOperation) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DownloadOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &DownloadOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<DownloadOperation> for ::windows::runtime::IInspectable {
    fn from(value: DownloadOperation) -> Self {
        value.0
    }
}
impl ::std::convert::From<&DownloadOperation> for ::windows::runtime::IInspectable {
    fn from(value: &DownloadOperation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for DownloadOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a DownloadOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<DownloadOperation> for IBackgroundTransferOperation {
    type Error = ::windows::runtime::Error;
    fn try_from(value: DownloadOperation) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&DownloadOperation> for IBackgroundTransferOperation {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &DownloadOperation) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTransferOperation> for DownloadOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTransferOperation> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTransferOperation> for &DownloadOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTransferOperation> {
        ::std::convert::TryInto::<IBackgroundTransferOperation>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<DownloadOperation> for IBackgroundTransferOperationPriority {
    type Error = ::windows::runtime::Error;
    fn try_from(value: DownloadOperation) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&DownloadOperation> for IBackgroundTransferOperationPriority {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &DownloadOperation) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTransferOperationPriority> for DownloadOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTransferOperationPriority> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTransferOperationPriority> for &DownloadOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTransferOperationPriority> {
        ::std::convert::TryInto::<IBackgroundTransferOperationPriority>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for DownloadOperation {}
unsafe impl ::std::marker::Sync for DownloadOperation {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IBackgroundDownloader(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBackgroundDownloader {
    type Vtable = IBackgroundDownloader_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3251082035, 26185, 19229, [168, 38, 164, 179, 221, 35, 77, 11]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundDownloader_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uri: ::windows::runtime::RawPtr, resultfile: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uri: ::windows::runtime::RawPtr, resultfile: ::windows::runtime::RawPtr, requestbodyfile: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uri: ::windows::runtime::RawPtr, resultfile: ::windows::runtime::RawPtr, requestbodystream: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage", feature = "Storage_Streams")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IBackgroundDownloader2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBackgroundDownloader2 {
    type Vtable = IBackgroundDownloader2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2840221767, 13453, 18997, [137, 14, 138, 30, 243, 121, 132, 121]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundDownloader2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "UI_Notifications")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))] usize,
    #[cfg(feature = "UI_Notifications")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))] usize,
    #[cfg(feature = "UI_Notifications")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))] usize,
    #[cfg(feature = "UI_Notifications")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))] usize,
    #[cfg(feature = "UI_Notifications")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))] usize,
    #[cfg(feature = "UI_Notifications")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))] usize,
    #[cfg(feature = "UI_Notifications")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))] usize,
    #[cfg(feature = "UI_Notifications")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IBackgroundDownloader3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBackgroundDownloader3 {
    type Vtable = IBackgroundDownloader3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3508177992, 34536, 18658, [182, 21, 105, 118, 170, 191, 134, 29]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundDownloader3_abi(
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
pub struct IBackgroundDownloaderFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBackgroundDownloaderFactory {
    type Vtable = IBackgroundDownloaderFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(646147108, 55454, 18164, [162, 154, 79, 77, 79, 20, 65, 85]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundDownloaderFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, completiongroup: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IBackgroundDownloaderStaticMethods(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBackgroundDownloaderStaticMethods {
    type Vtable = IBackgroundDownloaderStaticMethods_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1386633781, 50766, 17004, [153, 25, 84, 13, 13, 33, 166, 80]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundDownloaderStaticMethods_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, group: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IBackgroundDownloaderStaticMethods2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBackgroundDownloaderStaticMethods2 {
    type Vtable = IBackgroundDownloaderStaticMethods2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(799675175, 6868, 19621, [178, 205, 8, 219, 240, 116, 106, 254]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundDownloaderStaticMethods2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, group: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IBackgroundDownloaderUserConsent(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBackgroundDownloaderUserConsent {
    type Vtable = IBackgroundDownloaderUserConsent_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1561651462, 37478, 18440, [189, 113, 89, 37, 242, 163, 19, 10]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundDownloaderUserConsent_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, operations: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc = "*Required features: `Networking_BackgroundTransfer`*"]
pub struct IBackgroundTransferBase(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBackgroundTransferBase {
    type Vtable = IBackgroundTransferBase_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(714973776, 51049, 17804, [175, 232, 254, 184, 212, 211, 178, 239]);
}
impl IBackgroundTransferBase {
    #[doc = "*Required features: `Networking_BackgroundTransfer`*"]
    pub fn SetRequestHeader<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, headername: Param0, headervalue: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), headername.into_param().abi(), headervalue.into_param().abi()).ok() }
    }
    #[cfg(feature = "Security_Credentials")]
    #[doc = "*Required features: `Networking_BackgroundTransfer`, `Security_Credentials`*"]
    pub fn ServerCredential(&self) -> ::windows::runtime::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Security::Credentials::PasswordCredential>(result__)
        }
    }
    #[cfg(feature = "Security_Credentials")]
    #[doc = "*Required features: `Networking_BackgroundTransfer`, `Security_Credentials`*"]
    pub fn SetServerCredential<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Security::Credentials::PasswordCredential>>(&self, credential: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), credential.into_param().abi()).ok() }
    }
    #[cfg(feature = "Security_Credentials")]
    #[doc = "*Required features: `Networking_BackgroundTransfer`, `Security_Credentials`*"]
    pub fn ProxyCredential(&self) -> ::windows::runtime::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Security::Credentials::PasswordCredential>(result__)
        }
    }
    #[cfg(feature = "Security_Credentials")]
    #[doc = "*Required features: `Networking_BackgroundTransfer`, `Security_Credentials`*"]
    pub fn SetProxyCredential<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Security::Credentials::PasswordCredential>>(&self, credential: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), credential.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Networking_BackgroundTransfer`*"]
    pub fn Method(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Networking_BackgroundTransfer`*"]
    pub fn SetMethod<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Networking_BackgroundTransfer`*"]
    pub fn Group(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Networking_BackgroundTransfer`*"]
    pub fn SetGroup<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Networking_BackgroundTransfer`*"]
    pub fn CostPolicy(&self) -> ::windows::runtime::Result<BackgroundTransferCostPolicy> {
        let this = self;
        unsafe {
            let mut result__: BackgroundTransferCostPolicy = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<BackgroundTransferCostPolicy>(result__)
        }
    }
    #[doc = "*Required features: `Networking_BackgroundTransfer`*"]
    pub fn SetCostPolicy(&self, value: BackgroundTransferCostPolicy) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IBackgroundTransferBase {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{2a9da250-c769-458c-afe8-feb8d4d3b2ef}");
}
impl ::std::convert::From<IBackgroundTransferBase> for ::windows::runtime::IUnknown {
    fn from(value: IBackgroundTransferBase) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IBackgroundTransferBase> for ::windows::runtime::IUnknown {
    fn from(value: &IBackgroundTransferBase) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IBackgroundTransferBase {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IBackgroundTransferBase {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IBackgroundTransferBase> for ::windows::runtime::IInspectable {
    fn from(value: IBackgroundTransferBase) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IBackgroundTransferBase> for ::windows::runtime::IInspectable {
    fn from(value: &IBackgroundTransferBase) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IBackgroundTransferBase {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IBackgroundTransferBase {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTransferBase_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, headername: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, headervalue: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Security_Credentials")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))] usize,
    #[cfg(feature = "Security_Credentials")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, credential: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))] usize,
    #[cfg(feature = "Security_Credentials")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))] usize,
    #[cfg(feature = "Security_Credentials")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, credential: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut BackgroundTransferCostPolicy) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: BackgroundTransferCostPolicy) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IBackgroundTransferCompletionGroup(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBackgroundTransferCompletionGroup {
    type Vtable = IBackgroundTransferCompletionGroup_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(764609061, 39019, 22349, [121, 80, 10, 221, 71, 245, 215, 6]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTransferCompletionGroup_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "ApplicationModel_Background")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Background"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IBackgroundTransferCompletionGroupTriggerDetails(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBackgroundTransferCompletionGroupTriggerDetails {
    type Vtable = IBackgroundTransferCompletionGroupTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2070667910, 28231, 20790, [127, 203, 250, 67, 137, 244, 111, 91]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTransferCompletionGroupTriggerDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IBackgroundTransferContentPart(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBackgroundTransferContentPart {
    type Vtable = IBackgroundTransferContentPart_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3907081815, 55249, 20184, [131, 142, 103, 74, 194, 23, 172, 230]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTransferContentPart_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, headername: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, headervalue: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc = "*Required features: `Networking_BackgroundTransfer`*"]
pub struct IBackgroundTransferContentPartFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBackgroundTransferContentPartFactory {
    type Vtable = IBackgroundTransferContentPartFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2431621289, 31233, 18955, [159, 128, 160, 176, 187, 55, 15, 141]);
}
impl IBackgroundTransferContentPartFactory {
    #[doc = "*Required features: `Networking_BackgroundTransfer`*"]
    pub fn CreateWithName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0) -> ::windows::runtime::Result<BackgroundTransferContentPart> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<BackgroundTransferContentPart>(result__)
        }
    }
    #[doc = "*Required features: `Networking_BackgroundTransfer`*"]
    pub fn CreateWithNameAndFileName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, filename: Param1) -> ::windows::runtime::Result<BackgroundTransferContentPart> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), name.into_param().abi(), filename.into_param().abi(), &mut result__).from_abi::<BackgroundTransferContentPart>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IBackgroundTransferContentPartFactory {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{90ef98a9-7a01-4a0b-9f80-a0b0bb370f8d}");
}
impl ::std::convert::From<IBackgroundTransferContentPartFactory> for ::windows::runtime::IUnknown {
    fn from(value: IBackgroundTransferContentPartFactory) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IBackgroundTransferContentPartFactory> for ::windows::runtime::IUnknown {
    fn from(value: &IBackgroundTransferContentPartFactory) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IBackgroundTransferContentPartFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IBackgroundTransferContentPartFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IBackgroundTransferContentPartFactory> for ::windows::runtime::IInspectable {
    fn from(value: IBackgroundTransferContentPartFactory) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IBackgroundTransferContentPartFactory> for ::windows::runtime::IInspectable {
    fn from(value: &IBackgroundTransferContentPartFactory) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IBackgroundTransferContentPartFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IBackgroundTransferContentPartFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTransferContentPartFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, filename: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IBackgroundTransferErrorStaticMethods(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBackgroundTransferErrorStaticMethods {
    type Vtable = IBackgroundTransferErrorStaticMethods_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2865969924, 4498, 19444, [139, 104, 57, 197, 173, 210, 68, 226]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTransferErrorStaticMethods_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Web")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hresult: i32, result__: *mut super::super::Web::WebErrorStatus) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Web"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IBackgroundTransferGroup(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBackgroundTransferGroup {
    type Vtable = IBackgroundTransferGroup_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3636716516, 25689, 17728, [133, 235, 170, 161, 200, 144, 54, 119]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTransferGroup_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut BackgroundTransferBehavior) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: BackgroundTransferBehavior) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IBackgroundTransferGroupStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBackgroundTransferGroupStatics {
    type Vtable = IBackgroundTransferGroupStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(49041586, 32024, 18779, [170, 34, 50, 169, 125, 69, 211, 226]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTransferGroupStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc = "*Required features: `Networking_BackgroundTransfer`*"]
pub struct IBackgroundTransferOperation(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBackgroundTransferOperation {
    type Vtable = IBackgroundTransferOperation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3738200134, 37066, 17659, [143, 177, 18, 65, 84, 192, 213, 57]);
}
impl IBackgroundTransferOperation {
    #[doc = "*Required features: `Networking_BackgroundTransfer`*"]
    pub fn Guid(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_BackgroundTransfer`, `Foundation`*"]
    pub fn RequestedUri(&self) -> ::windows::runtime::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `Networking_BackgroundTransfer`*"]
    pub fn Method(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Networking_BackgroundTransfer`*"]
    pub fn Group(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Networking_BackgroundTransfer`*"]
    pub fn CostPolicy(&self) -> ::windows::runtime::Result<BackgroundTransferCostPolicy> {
        let this = self;
        unsafe {
            let mut result__: BackgroundTransferCostPolicy = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<BackgroundTransferCostPolicy>(result__)
        }
    }
    #[doc = "*Required features: `Networking_BackgroundTransfer`*"]
    pub fn SetCostPolicy(&self, value: BackgroundTransferCostPolicy) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Networking_BackgroundTransfer`, `Storage_Streams`*"]
    pub fn GetResultStreamAt(&self, position: u64) -> ::windows::runtime::Result<super::super::Storage::Streams::IInputStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), position, &mut result__).from_abi::<super::super::Storage::Streams::IInputStream>(result__)
        }
    }
    #[doc = "*Required features: `Networking_BackgroundTransfer`*"]
    pub fn GetResponseInformation(&self) -> ::windows::runtime::Result<ResponseInformation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ResponseInformation>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IBackgroundTransferOperation {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{ded06846-90ca-44fb-8fb1-124154c0d539}");
}
impl ::std::convert::From<IBackgroundTransferOperation> for ::windows::runtime::IUnknown {
    fn from(value: IBackgroundTransferOperation) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IBackgroundTransferOperation> for ::windows::runtime::IUnknown {
    fn from(value: &IBackgroundTransferOperation) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IBackgroundTransferOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IBackgroundTransferOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IBackgroundTransferOperation> for ::windows::runtime::IInspectable {
    fn from(value: IBackgroundTransferOperation) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IBackgroundTransferOperation> for ::windows::runtime::IInspectable {
    fn from(value: &IBackgroundTransferOperation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IBackgroundTransferOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IBackgroundTransferOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTransferOperation_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut BackgroundTransferCostPolicy) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: BackgroundTransferCostPolicy) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, position: u64, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc = "*Required features: `Networking_BackgroundTransfer`*"]
pub struct IBackgroundTransferOperationPriority(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBackgroundTransferOperationPriority {
    type Vtable = IBackgroundTransferOperationPriority_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(75842343, 21076, 19258, [145, 94, 10, 164, 146, 117, 192, 249]);
}
impl IBackgroundTransferOperationPriority {
    #[doc = "*Required features: `Networking_BackgroundTransfer`*"]
    pub fn Priority(&self) -> ::windows::runtime::Result<BackgroundTransferPriority> {
        let this = self;
        unsafe {
            let mut result__: BackgroundTransferPriority = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<BackgroundTransferPriority>(result__)
        }
    }
    #[doc = "*Required features: `Networking_BackgroundTransfer`*"]
    pub fn SetPriority(&self, value: BackgroundTransferPriority) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IBackgroundTransferOperationPriority {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{04854327-5254-4b3a-915e-0aa49275c0f9}");
}
impl ::std::convert::From<IBackgroundTransferOperationPriority> for ::windows::runtime::IUnknown {
    fn from(value: IBackgroundTransferOperationPriority) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IBackgroundTransferOperationPriority> for ::windows::runtime::IUnknown {
    fn from(value: &IBackgroundTransferOperationPriority) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IBackgroundTransferOperationPriority {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IBackgroundTransferOperationPriority {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IBackgroundTransferOperationPriority> for ::windows::runtime::IInspectable {
    fn from(value: IBackgroundTransferOperationPriority) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IBackgroundTransferOperationPriority> for ::windows::runtime::IInspectable {
    fn from(value: &IBackgroundTransferOperationPriority) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IBackgroundTransferOperationPriority {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IBackgroundTransferOperationPriority {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTransferOperationPriority_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut BackgroundTransferPriority) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: BackgroundTransferPriority) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IBackgroundTransferRangesDownloadedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBackgroundTransferRangesDownloadedEventArgs {
    type Vtable = IBackgroundTransferRangesDownloadedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1052537939, 48968, 19080, [146, 72, 176, 193, 101, 24, 79, 92]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTransferRangesDownloadedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IBackgroundUploader(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBackgroundUploader {
    type Vtable = IBackgroundUploader_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3314928046, 52909, 18011, [136, 1, 197, 90, 201, 10, 1, 206]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundUploader_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uri: ::windows::runtime::RawPtr, sourcefile: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uri: ::windows::runtime::RawPtr, sourcestream: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uri: ::windows::runtime::RawPtr, parts: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uri: ::windows::runtime::RawPtr, parts: ::windows::runtime::RawPtr, subtype: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uri: ::windows::runtime::RawPtr, parts: ::windows::runtime::RawPtr, subtype: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, boundary: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IBackgroundUploader2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBackgroundUploader2 {
    type Vtable = IBackgroundUploader2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2382762702, 3124, 17507, [128, 127, 25, 138, 27, 139, 212, 173]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundUploader2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "UI_Notifications")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))] usize,
    #[cfg(feature = "UI_Notifications")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))] usize,
    #[cfg(feature = "UI_Notifications")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))] usize,
    #[cfg(feature = "UI_Notifications")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))] usize,
    #[cfg(feature = "UI_Notifications")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))] usize,
    #[cfg(feature = "UI_Notifications")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))] usize,
    #[cfg(feature = "UI_Notifications")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))] usize,
    #[cfg(feature = "UI_Notifications")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IBackgroundUploader3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBackgroundUploader3 {
    type Vtable = IBackgroundUploader3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3109983289, 23536, 19258, [140, 71, 44, 97, 153, 168, 84, 185]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundUploader3_abi(
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
pub struct IBackgroundUploaderFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBackgroundUploaderFactory {
    type Vtable = IBackgroundUploaderFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1935803335, 4327, 18592, [172, 60, 26, 199, 16, 149, 236, 87]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundUploaderFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, completiongroup: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IBackgroundUploaderStaticMethods(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBackgroundUploaderStaticMethods {
    type Vtable = IBackgroundUploaderStaticMethods_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4068957435, 39685, 18241, [145, 33, 116, 10, 131, 226, 71, 223]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundUploaderStaticMethods_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, group: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IBackgroundUploaderStaticMethods2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBackgroundUploaderStaticMethods2 {
    type Vtable = IBackgroundUploaderStaticMethods2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3910773858, 59912, 17136, [162, 172, 7, 228, 103, 84, 144, 128]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundUploaderStaticMethods2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, group: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IBackgroundUploaderUserConsent(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBackgroundUploaderUserConsent {
    type Vtable = IBackgroundUploaderUserConsent_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1001620683, 1888, 17949, [144, 127, 81, 56, 248, 77, 68, 193]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundUploaderUserConsent_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, operations: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IContentPrefetcher(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContentPrefetcher {
    type Vtable = IContentPrefetcher_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2832660308, 32193, 19673, [136, 16, 42, 106, 169, 65, 126, 17]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentPrefetcher_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IContentPrefetcherTime(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContentPrefetcherTime {
    type Vtable = IContentPrefetcherTime_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3814849800, 4906, 20446, [167, 204, 252, 176, 230, 101, 35, 175]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentPrefetcherTime_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IDownloadOperation(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDownloadOperation {
    type Vtable = IDownloadOperation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3179801520, 22292, 19977, [186, 104, 190, 247, 57, 3, 176, 215]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDownloadOperation_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut BackgroundDownloadProgress) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IDownloadOperation2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDownloadOperation2 {
    type Vtable = IDownloadOperation2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2748116288, 36764, 17235, [156, 212, 41, 13, 238, 56, 124, 56]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDownloadOperation2_abi(
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
pub struct IDownloadOperation3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDownloadOperation3 {
    type Vtable = IDownloadOperation3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1344746780, 32094, 19164, [184, 211, 223, 92, 96, 49, 185, 204]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDownloadOperation3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, eventhandler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Web"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Web")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Web"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Web")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IDownloadOperation4(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDownloadOperation4 {
    type Vtable = IDownloadOperation4_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(215658228, 36079, 16458, [150, 109, 240, 88, 64, 11, 237, 128]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDownloadOperation4_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IDownloadOperation5(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDownloadOperation5 {
    type Vtable = IDownloadOperation5_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2795087983, 21904, 17978, [184, 214, 30, 73, 26, 39, 96, 165]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDownloadOperation5_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, headername: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, headervalue: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, headername: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IResponseInformation(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IResponseInformation {
    type Vtable = IResponseInformation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4173044242, 63251, 18322, [139, 104, 217, 210, 151, 249, 29, 46]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResponseInformation_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IUnconstrainedTransferRequestResult(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUnconstrainedTransferRequestResult {
    type Vtable = IUnconstrainedTransferRequestResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1277474847, 55620, 16658, [169, 142, 106, 105, 82, 43, 126, 187]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUnconstrainedTransferRequestResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IUploadOperation(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUploadOperation {
    type Vtable = IUploadOperation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1045832928, 29577, 17228, [139, 53, 66, 127, 211, 107, 189, 174]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUploadOperation_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut BackgroundUploadProgress) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IUploadOperation2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUploadOperation2 {
    type Vtable = IUploadOperation2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1432455666, 10100, 19958, [159, 165, 32, 159, 43, 251, 18, 247]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUploadOperation2_abi(
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
pub struct IUploadOperation3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUploadOperation3 {
    type Vtable = IUploadOperation3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1120480419, 56889, 17734, [188, 98, 55, 116, 180, 41, 77, 227]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUploadOperation3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IUploadOperation4(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUploadOperation4 {
    type Vtable = IUploadOperation4_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1357770545, 64197, 16878, [176, 48, 220, 119, 202, 238, 159, 170]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUploadOperation4_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, headername: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, headervalue: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, headername: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Networking_BackgroundTransfer`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ResponseInformation(::windows::runtime::IInspectable);
impl ResponseInformation {
    #[doc = "*Required features: `Networking_BackgroundTransfer`*"]
    pub fn IsResumable(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_BackgroundTransfer`, `Foundation`*"]
    pub fn ActualUri(&self) -> ::windows::runtime::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `Networking_BackgroundTransfer`*"]
    pub fn StatusCode(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Networking_BackgroundTransfer`, `Foundation_Collections`*"]
    pub fn Headers(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ResponseInformation {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.BackgroundTransfer.ResponseInformation;{f8bb9a12-f713-4792-8b68-d9d297f91d2e})");
}
unsafe impl ::windows::runtime::Interface for ResponseInformation {
    type Vtable = IResponseInformation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4173044242, 63251, 18322, [139, 104, 217, 210, 151, 249, 29, 46]);
}
impl ::windows::runtime::RuntimeName for ResponseInformation {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.ResponseInformation";
}
impl ::std::convert::From<ResponseInformation> for ::windows::runtime::IUnknown {
    fn from(value: ResponseInformation) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ResponseInformation> for ::windows::runtime::IUnknown {
    fn from(value: &ResponseInformation) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ResponseInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ResponseInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ResponseInformation> for ::windows::runtime::IInspectable {
    fn from(value: ResponseInformation) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ResponseInformation> for ::windows::runtime::IInspectable {
    fn from(value: &ResponseInformation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ResponseInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ResponseInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ResponseInformation {}
unsafe impl ::std::marker::Sync for ResponseInformation {}
#[doc = "*Required features: `Networking_BackgroundTransfer`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct UnconstrainedTransferRequestResult(::windows::runtime::IInspectable);
impl UnconstrainedTransferRequestResult {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Networking_BackgroundTransfer`*"]
    pub fn IsUnconstrained(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for UnconstrainedTransferRequestResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.BackgroundTransfer.UnconstrainedTransferRequestResult;{4c24b81f-d944-4112-a98e-6a69522b7ebb})");
}
unsafe impl ::windows::runtime::Interface for UnconstrainedTransferRequestResult {
    type Vtable = IUnconstrainedTransferRequestResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1277474847, 55620, 16658, [169, 142, 106, 105, 82, 43, 126, 187]);
}
impl ::windows::runtime::RuntimeName for UnconstrainedTransferRequestResult {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.UnconstrainedTransferRequestResult";
}
impl ::std::convert::From<UnconstrainedTransferRequestResult> for ::windows::runtime::IUnknown {
    fn from(value: UnconstrainedTransferRequestResult) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&UnconstrainedTransferRequestResult> for ::windows::runtime::IUnknown {
    fn from(value: &UnconstrainedTransferRequestResult) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for UnconstrainedTransferRequestResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &UnconstrainedTransferRequestResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<UnconstrainedTransferRequestResult> for ::windows::runtime::IInspectable {
    fn from(value: UnconstrainedTransferRequestResult) -> Self {
        value.0
    }
}
impl ::std::convert::From<&UnconstrainedTransferRequestResult> for ::windows::runtime::IInspectable {
    fn from(value: &UnconstrainedTransferRequestResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for UnconstrainedTransferRequestResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a UnconstrainedTransferRequestResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for UnconstrainedTransferRequestResult {}
unsafe impl ::std::marker::Sync for UnconstrainedTransferRequestResult {}
#[doc = "*Required features: `Networking_BackgroundTransfer`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct UploadOperation(::windows::runtime::IInspectable);
impl UploadOperation {
    #[cfg(feature = "Storage")]
    #[doc = "*Required features: `Networking_BackgroundTransfer`, `Storage`*"]
    pub fn SourceFile(&self) -> ::windows::runtime::Result<super::super::Storage::IStorageFile> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::IStorageFile>(result__)
        }
    }
    #[doc = "*Required features: `Networking_BackgroundTransfer`*"]
    pub fn Progress(&self) -> ::windows::runtime::Result<BackgroundUploadProgress> {
        let this = self;
        unsafe {
            let mut result__: BackgroundUploadProgress = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<BackgroundUploadProgress>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_BackgroundTransfer`, `Foundation`*"]
    pub fn StartAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperationWithProgress<UploadOperation, UploadOperation>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<UploadOperation, UploadOperation>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_BackgroundTransfer`, `Foundation`*"]
    pub fn AttachAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperationWithProgress<UploadOperation, UploadOperation>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<UploadOperation, UploadOperation>>(result__)
        }
    }
    #[doc = "*Required features: `Networking_BackgroundTransfer`*"]
    pub fn Guid(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundTransferOperation>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_BackgroundTransfer`, `Foundation`*"]
    pub fn RequestedUri(&self) -> ::windows::runtime::Result<super::super::Foundation::Uri> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundTransferOperation>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `Networking_BackgroundTransfer`*"]
    pub fn Method(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundTransferOperation>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Networking_BackgroundTransfer`*"]
    pub fn Group(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundTransferOperation>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Networking_BackgroundTransfer`*"]
    pub fn CostPolicy(&self) -> ::windows::runtime::Result<BackgroundTransferCostPolicy> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundTransferOperation>(self)?;
        unsafe {
            let mut result__: BackgroundTransferCostPolicy = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<BackgroundTransferCostPolicy>(result__)
        }
    }
    #[doc = "*Required features: `Networking_BackgroundTransfer`*"]
    pub fn SetCostPolicy(&self, value: BackgroundTransferCostPolicy) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundTransferOperation>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Networking_BackgroundTransfer`, `Storage_Streams`*"]
    pub fn GetResultStreamAt(&self, position: u64) -> ::windows::runtime::Result<super::super::Storage::Streams::IInputStream> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundTransferOperation>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), position, &mut result__).from_abi::<super::super::Storage::Streams::IInputStream>(result__)
        }
    }
    #[doc = "*Required features: `Networking_BackgroundTransfer`*"]
    pub fn GetResponseInformation(&self) -> ::windows::runtime::Result<ResponseInformation> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundTransferOperation>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ResponseInformation>(result__)
        }
    }
    #[doc = "*Required features: `Networking_BackgroundTransfer`*"]
    pub fn Priority(&self) -> ::windows::runtime::Result<BackgroundTransferPriority> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundTransferOperationPriority>(self)?;
        unsafe {
            let mut result__: BackgroundTransferPriority = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<BackgroundTransferPriority>(result__)
        }
    }
    #[doc = "*Required features: `Networking_BackgroundTransfer`*"]
    pub fn SetPriority(&self, value: BackgroundTransferPriority) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IBackgroundTransferOperationPriority>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Networking_BackgroundTransfer`*"]
    pub fn TransferGroup(&self) -> ::windows::runtime::Result<BackgroundTransferGroup> {
        let this = &::windows::runtime::Interface::cast::<IUploadOperation2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<BackgroundTransferGroup>(result__)
        }
    }
    #[doc = "*Required features: `Networking_BackgroundTransfer`*"]
    pub fn MakeCurrentInTransferGroup(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IUploadOperation3>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Networking_BackgroundTransfer`*"]
    pub fn SetRequestHeader<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, headername: Param0, headervalue: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IUploadOperation4>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), headername.into_param().abi(), headervalue.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Networking_BackgroundTransfer`*"]
    pub fn RemoveRequestHeader<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, headername: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IUploadOperation4>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), headername.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for UploadOperation {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.BackgroundTransfer.UploadOperation;{3e5624e0-7389-434c-8b35-427fd36bbdae})");
}
unsafe impl ::windows::runtime::Interface for UploadOperation {
    type Vtable = IUploadOperation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1045832928, 29577, 17228, [139, 53, 66, 127, 211, 107, 189, 174]);
}
impl ::windows::runtime::RuntimeName for UploadOperation {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.UploadOperation";
}
impl ::std::convert::From<UploadOperation> for ::windows::runtime::IUnknown {
    fn from(value: UploadOperation) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&UploadOperation> for ::windows::runtime::IUnknown {
    fn from(value: &UploadOperation) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for UploadOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &UploadOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<UploadOperation> for ::windows::runtime::IInspectable {
    fn from(value: UploadOperation) -> Self {
        value.0
    }
}
impl ::std::convert::From<&UploadOperation> for ::windows::runtime::IInspectable {
    fn from(value: &UploadOperation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for UploadOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a UploadOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<UploadOperation> for IBackgroundTransferOperation {
    type Error = ::windows::runtime::Error;
    fn try_from(value: UploadOperation) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&UploadOperation> for IBackgroundTransferOperation {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &UploadOperation) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTransferOperation> for UploadOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTransferOperation> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTransferOperation> for &UploadOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTransferOperation> {
        ::std::convert::TryInto::<IBackgroundTransferOperation>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<UploadOperation> for IBackgroundTransferOperationPriority {
    type Error = ::windows::runtime::Error;
    fn try_from(value: UploadOperation) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&UploadOperation> for IBackgroundTransferOperationPriority {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &UploadOperation) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTransferOperationPriority> for UploadOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTransferOperationPriority> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundTransferOperationPriority> for &UploadOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundTransferOperationPriority> {
        ::std::convert::TryInto::<IBackgroundTransferOperationPriority>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for UploadOperation {}
unsafe impl ::std::marker::Sync for UploadOperation {}
