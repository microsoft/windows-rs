#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Media_Streaming_Adaptive`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct AdaptiveMediaSource(pub ::windows::runtime::IInspectable);
impl AdaptiveMediaSource {
    #[doc = "*Required features: `Media_Streaming_Adaptive`*"]
    pub fn IsLive(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Streaming_Adaptive`, `Foundation`*"]
    pub fn DesiredLiveOffset(&self) -> ::windows::runtime::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Streaming_Adaptive`, `Foundation`*"]
    pub fn SetDesiredLiveOffset<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media_Streaming_Adaptive`*"]
    pub fn InitialBitrate(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Media_Streaming_Adaptive`*"]
    pub fn SetInitialBitrate(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Media_Streaming_Adaptive`*"]
    pub fn CurrentDownloadBitrate(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Media_Streaming_Adaptive`*"]
    pub fn CurrentPlaybackBitrate(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Media_Streaming_Adaptive`, `Foundation_Collections`*"]
    pub fn AvailableBitrates(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Streaming_Adaptive`, `Foundation`*"]
    pub fn DesiredMinBitrate(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Streaming_Adaptive`, `Foundation`*"]
    pub fn SetDesiredMinBitrate<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IReference<u32>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Streaming_Adaptive`, `Foundation`*"]
    pub fn DesiredMaxBitrate(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Streaming_Adaptive`, `Foundation`*"]
    pub fn SetDesiredMaxBitrate<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IReference<u32>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media_Streaming_Adaptive`*"]
    pub fn AudioOnlyPlayback(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media_Streaming_Adaptive`*"]
    pub fn InboundBitsPerSecond(&self) -> ::windows::runtime::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Streaming_Adaptive`, `Foundation`*"]
    pub fn InboundBitsPerSecondWindow(&self) -> ::windows::runtime::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Streaming_Adaptive`, `Foundation`*"]
    pub fn SetInboundBitsPerSecondWindow<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Streaming_Adaptive`, `Foundation`*"]
    pub fn DownloadBitrateChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<AdaptiveMediaSource, AdaptiveMediaSourceDownloadBitrateChangedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Streaming_Adaptive`, `Foundation`*"]
    pub fn RemoveDownloadBitrateChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Streaming_Adaptive`, `Foundation`*"]
    pub fn PlaybackBitrateChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<AdaptiveMediaSource, AdaptiveMediaSourcePlaybackBitrateChangedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Streaming_Adaptive`, `Foundation`*"]
    pub fn RemovePlaybackBitrateChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).25)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Streaming_Adaptive`, `Foundation`*"]
    pub fn DownloadRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<AdaptiveMediaSource, AdaptiveMediaSourceDownloadRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).26)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Streaming_Adaptive`, `Foundation`*"]
    pub fn RemoveDownloadRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).27)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Streaming_Adaptive`, `Foundation`*"]
    pub fn DownloadCompleted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<AdaptiveMediaSource, AdaptiveMediaSourceDownloadCompletedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).28)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Streaming_Adaptive`, `Foundation`*"]
    pub fn RemoveDownloadCompleted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).29)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Streaming_Adaptive`, `Foundation`*"]
    pub fn DownloadFailed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<AdaptiveMediaSource, AdaptiveMediaSourceDownloadFailedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).30)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Streaming_Adaptive`, `Foundation`*"]
    pub fn RemoveDownloadFailed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).31)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Streaming_Adaptive`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Media_Streaming_Adaptive`*"]
    pub fn IsContentTypeSupported<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(contenttype: Param0) -> ::windows::runtime::Result<bool> {
        Self::IAdaptiveMediaSourceStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), contenttype.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Streaming_Adaptive`, `Foundation`*"]
    pub fn CreateFromUriAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Uri>>(uri: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<AdaptiveMediaSourceCreationResult>> {
        Self::IAdaptiveMediaSourceStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), uri.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<AdaptiveMediaSourceCreationResult>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Web_Http"))]
    #[doc = "*Required features: `Media_Streaming_Adaptive`, `Foundation`, `Web_Http`*"]
    pub fn CreateFromUriWithDownloaderAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Uri>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Web::Http::HttpClient>>(uri: Param0, httpclient: Param1) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<AdaptiveMediaSourceCreationResult>> {
        Self::IAdaptiveMediaSourceStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), uri.into_param().abi(), httpclient.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<AdaptiveMediaSourceCreationResult>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Media_Streaming_Adaptive`, `Foundation`, `Storage_Streams`*"]
    pub fn CreateFromStreamAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Storage::Streams::IInputStream>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Uri>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(stream: Param0, uri: Param1, contenttype: Param2) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<AdaptiveMediaSourceCreationResult>> {
        Self::IAdaptiveMediaSourceStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), stream.into_param().abi(), uri.into_param().abi(), contenttype.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<AdaptiveMediaSourceCreationResult>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "Web_Http"))]
    #[doc = "*Required features: `Media_Streaming_Adaptive`, `Foundation`, `Storage_Streams`, `Web_Http`*"]
    pub fn CreateFromStreamWithDownloaderAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Storage::Streams::IInputStream>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Uri>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param3: ::windows::runtime::IntoParam<'a, super::super::super::Web::Http::HttpClient>>(
        stream: Param0,
        uri: Param1,
        contenttype: Param2,
        httpclient: Param3,
    ) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<AdaptiveMediaSourceCreationResult>> {
        Self::IAdaptiveMediaSourceStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), stream.into_param().abi(), uri.into_param().abi(), contenttype.into_param().abi(), httpclient.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<AdaptiveMediaSourceCreationResult>>(result__)
        })
    }
    #[doc = "*Required features: `Media_Streaming_Adaptive`*"]
    pub fn AdvancedSettings(&self) -> ::windows::runtime::Result<AdaptiveMediaSourceAdvancedSettings> {
        let this = &::windows::runtime::Interface::cast::<IAdaptiveMediaSource2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AdaptiveMediaSourceAdvancedSettings>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Streaming_Adaptive`, `Foundation`*"]
    pub fn MinLiveOffset(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>> {
        let this = &::windows::runtime::Interface::cast::<IAdaptiveMediaSource3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Streaming_Adaptive`, `Foundation`*"]
    pub fn MaxSeekableWindowSize(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>> {
        let this = &::windows::runtime::Interface::cast::<IAdaptiveMediaSource3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Streaming_Adaptive`, `Foundation`*"]
    pub fn DesiredSeekableWindowSize(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>> {
        let this = &::windows::runtime::Interface::cast::<IAdaptiveMediaSource3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Streaming_Adaptive`, `Foundation`*"]
    pub fn SetDesiredSeekableWindowSize<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAdaptiveMediaSource3>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media_Streaming_Adaptive`*"]
    pub fn Diagnostics(&self) -> ::windows::runtime::Result<AdaptiveMediaSourceDiagnostics> {
        let this = &::windows::runtime::Interface::cast::<IAdaptiveMediaSource3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AdaptiveMediaSourceDiagnostics>(result__)
        }
    }
    #[doc = "*Required features: `Media_Streaming_Adaptive`*"]
    pub fn GetCorrelatedTimes(&self) -> ::windows::runtime::Result<AdaptiveMediaSourceCorrelatedTimes> {
        let this = &::windows::runtime::Interface::cast::<IAdaptiveMediaSource3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AdaptiveMediaSourceCorrelatedTimes>(result__)
        }
    }
    pub fn IAdaptiveMediaSourceStatics<R, F: FnOnce(&IAdaptiveMediaSourceStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<AdaptiveMediaSource, IAdaptiveMediaSourceStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AdaptiveMediaSource {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Streaming.Adaptive.AdaptiveMediaSource;{4c7332ef-d39f-4396-b4d9-043957a7c964})");
}
unsafe impl ::windows::runtime::Interface for AdaptiveMediaSource {
    type Vtable = IAdaptiveMediaSource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1282618095, 54175, 17302, [180, 217, 4, 57, 87, 167, 201, 100]);
}
impl ::windows::runtime::RuntimeName for AdaptiveMediaSource {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.AdaptiveMediaSource";
}
impl ::std::convert::From<AdaptiveMediaSource> for ::windows::runtime::IUnknown {
    fn from(value: AdaptiveMediaSource) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&AdaptiveMediaSource> for ::windows::runtime::IUnknown {
    fn from(value: &AdaptiveMediaSource) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AdaptiveMediaSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a AdaptiveMediaSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<AdaptiveMediaSource> for ::windows::runtime::IInspectable {
    fn from(value: AdaptiveMediaSource) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AdaptiveMediaSource> for ::windows::runtime::IInspectable {
    fn from(value: &AdaptiveMediaSource) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AdaptiveMediaSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AdaptiveMediaSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<AdaptiveMediaSource> for super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: AdaptiveMediaSource) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&AdaptiveMediaSource> for super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &AdaptiveMediaSource) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable> for AdaptiveMediaSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable> for &AdaptiveMediaSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Media_Core")]
impl ::std::convert::TryFrom<AdaptiveMediaSource> for super::super::Core::IMediaSource {
    type Error = ::windows::runtime::Error;
    fn try_from(value: AdaptiveMediaSource) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Media_Core")]
impl ::std::convert::TryFrom<&AdaptiveMediaSource> for super::super::Core::IMediaSource {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &AdaptiveMediaSource) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Media_Core")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Core::IMediaSource> for AdaptiveMediaSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Core::IMediaSource> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Media_Core")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Core::IMediaSource> for &AdaptiveMediaSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Core::IMediaSource> {
        ::std::convert::TryInto::<super::super::Core::IMediaSource>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for AdaptiveMediaSource {}
unsafe impl ::std::marker::Sync for AdaptiveMediaSource {}
#[doc = "*Required features: `Media_Streaming_Adaptive`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct AdaptiveMediaSourceAdvancedSettings(pub ::windows::runtime::IInspectable);
impl AdaptiveMediaSourceAdvancedSettings {
    #[doc = "*Required features: `Media_Streaming_Adaptive`*"]
    pub fn AllSegmentsIndependent(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media_Streaming_Adaptive`*"]
    pub fn SetAllSegmentsIndependent(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Streaming_Adaptive`, `Foundation`*"]
    pub fn DesiredBitrateHeadroomRatio(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<f64>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Streaming_Adaptive`, `Foundation`*"]
    pub fn SetDesiredBitrateHeadroomRatio<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IReference<f64>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Streaming_Adaptive`, `Foundation`*"]
    pub fn BitrateDowngradeTriggerRatio(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<f64>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Streaming_Adaptive`, `Foundation`*"]
    pub fn SetBitrateDowngradeTriggerRatio<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IReference<f64>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AdaptiveMediaSourceAdvancedSettings {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceAdvancedSettings;{55db1680-1aeb-47dc-aa08-9a11610ba45a})");
}
unsafe impl ::windows::runtime::Interface for AdaptiveMediaSourceAdvancedSettings {
    type Vtable = IAdaptiveMediaSourceAdvancedSettings_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1440421504, 6891, 18396, [170, 8, 154, 17, 97, 11, 164, 90]);
}
impl ::windows::runtime::RuntimeName for AdaptiveMediaSourceAdvancedSettings {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceAdvancedSettings";
}
impl ::std::convert::From<AdaptiveMediaSourceAdvancedSettings> for ::windows::runtime::IUnknown {
    fn from(value: AdaptiveMediaSourceAdvancedSettings) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&AdaptiveMediaSourceAdvancedSettings> for ::windows::runtime::IUnknown {
    fn from(value: &AdaptiveMediaSourceAdvancedSettings) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AdaptiveMediaSourceAdvancedSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a AdaptiveMediaSourceAdvancedSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<AdaptiveMediaSourceAdvancedSettings> for ::windows::runtime::IInspectable {
    fn from(value: AdaptiveMediaSourceAdvancedSettings) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AdaptiveMediaSourceAdvancedSettings> for ::windows::runtime::IInspectable {
    fn from(value: &AdaptiveMediaSourceAdvancedSettings) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AdaptiveMediaSourceAdvancedSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AdaptiveMediaSourceAdvancedSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for AdaptiveMediaSourceAdvancedSettings {}
unsafe impl ::std::marker::Sync for AdaptiveMediaSourceAdvancedSettings {}
#[doc = "*Required features: `Media_Streaming_Adaptive`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct AdaptiveMediaSourceCorrelatedTimes(pub ::windows::runtime::IInspectable);
impl AdaptiveMediaSourceCorrelatedTimes {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Streaming_Adaptive`, `Foundation`*"]
    pub fn Position(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Streaming_Adaptive`, `Foundation`*"]
    pub fn PresentationTimeStamp(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Streaming_Adaptive`, `Foundation`*"]
    pub fn ProgramDateTime(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AdaptiveMediaSourceCorrelatedTimes {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceCorrelatedTimes;{05108787-e032-48e1-ab8d-002b0b3051df})");
}
unsafe impl ::windows::runtime::Interface for AdaptiveMediaSourceCorrelatedTimes {
    type Vtable = IAdaptiveMediaSourceCorrelatedTimes_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(84969351, 57394, 18657, [171, 141, 0, 43, 11, 48, 81, 223]);
}
impl ::windows::runtime::RuntimeName for AdaptiveMediaSourceCorrelatedTimes {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceCorrelatedTimes";
}
impl ::std::convert::From<AdaptiveMediaSourceCorrelatedTimes> for ::windows::runtime::IUnknown {
    fn from(value: AdaptiveMediaSourceCorrelatedTimes) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&AdaptiveMediaSourceCorrelatedTimes> for ::windows::runtime::IUnknown {
    fn from(value: &AdaptiveMediaSourceCorrelatedTimes) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AdaptiveMediaSourceCorrelatedTimes {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a AdaptiveMediaSourceCorrelatedTimes {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<AdaptiveMediaSourceCorrelatedTimes> for ::windows::runtime::IInspectable {
    fn from(value: AdaptiveMediaSourceCorrelatedTimes) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AdaptiveMediaSourceCorrelatedTimes> for ::windows::runtime::IInspectable {
    fn from(value: &AdaptiveMediaSourceCorrelatedTimes) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AdaptiveMediaSourceCorrelatedTimes {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AdaptiveMediaSourceCorrelatedTimes {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for AdaptiveMediaSourceCorrelatedTimes {}
unsafe impl ::std::marker::Sync for AdaptiveMediaSourceCorrelatedTimes {}
#[doc = "*Required features: `Media_Streaming_Adaptive`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct AdaptiveMediaSourceCreationResult(pub ::windows::runtime::IInspectable);
impl AdaptiveMediaSourceCreationResult {
    #[doc = "*Required features: `Media_Streaming_Adaptive`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<AdaptiveMediaSourceCreationStatus> {
        let this = self;
        unsafe {
            let mut result__: AdaptiveMediaSourceCreationStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AdaptiveMediaSourceCreationStatus>(result__)
        }
    }
    #[doc = "*Required features: `Media_Streaming_Adaptive`*"]
    pub fn MediaSource(&self) -> ::windows::runtime::Result<AdaptiveMediaSource> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AdaptiveMediaSource>(result__)
        }
    }
    #[cfg(feature = "Web_Http")]
    #[doc = "*Required features: `Media_Streaming_Adaptive`, `Web_Http`*"]
    pub fn HttpResponseMessage(&self) -> ::windows::runtime::Result<super::super::super::Web::Http::HttpResponseMessage> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Web::Http::HttpResponseMessage>(result__)
        }
    }
    #[doc = "*Required features: `Media_Streaming_Adaptive`*"]
    pub fn ExtendedError(&self) -> ::windows::runtime::Result<::windows::runtime::HRESULT> {
        let this = &::windows::runtime::Interface::cast::<IAdaptiveMediaSourceCreationResult2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::HRESULT = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HRESULT>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AdaptiveMediaSourceCreationResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceCreationResult;{4686b6b2-800f-4e31-9093-76d4782013e7})");
}
unsafe impl ::windows::runtime::Interface for AdaptiveMediaSourceCreationResult {
    type Vtable = IAdaptiveMediaSourceCreationResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1183233714, 32783, 20017, [144, 147, 118, 212, 120, 32, 19, 231]);
}
impl ::windows::runtime::RuntimeName for AdaptiveMediaSourceCreationResult {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceCreationResult";
}
impl ::std::convert::From<AdaptiveMediaSourceCreationResult> for ::windows::runtime::IUnknown {
    fn from(value: AdaptiveMediaSourceCreationResult) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&AdaptiveMediaSourceCreationResult> for ::windows::runtime::IUnknown {
    fn from(value: &AdaptiveMediaSourceCreationResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AdaptiveMediaSourceCreationResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a AdaptiveMediaSourceCreationResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<AdaptiveMediaSourceCreationResult> for ::windows::runtime::IInspectable {
    fn from(value: AdaptiveMediaSourceCreationResult) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AdaptiveMediaSourceCreationResult> for ::windows::runtime::IInspectable {
    fn from(value: &AdaptiveMediaSourceCreationResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AdaptiveMediaSourceCreationResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AdaptiveMediaSourceCreationResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for AdaptiveMediaSourceCreationResult {}
unsafe impl ::std::marker::Sync for AdaptiveMediaSourceCreationResult {}
#[doc = "*Required features: `Media_Streaming_Adaptive`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct AdaptiveMediaSourceCreationStatus(pub i32);
impl AdaptiveMediaSourceCreationStatus {
    pub const Success: AdaptiveMediaSourceCreationStatus = AdaptiveMediaSourceCreationStatus(0i32);
    pub const ManifestDownloadFailure: AdaptiveMediaSourceCreationStatus = AdaptiveMediaSourceCreationStatus(1i32);
    pub const ManifestParseFailure: AdaptiveMediaSourceCreationStatus = AdaptiveMediaSourceCreationStatus(2i32);
    pub const UnsupportedManifestContentType: AdaptiveMediaSourceCreationStatus = AdaptiveMediaSourceCreationStatus(3i32);
    pub const UnsupportedManifestVersion: AdaptiveMediaSourceCreationStatus = AdaptiveMediaSourceCreationStatus(4i32);
    pub const UnsupportedManifestProfile: AdaptiveMediaSourceCreationStatus = AdaptiveMediaSourceCreationStatus(5i32);
    pub const UnknownFailure: AdaptiveMediaSourceCreationStatus = AdaptiveMediaSourceCreationStatus(6i32);
}
impl ::std::convert::From<i32> for AdaptiveMediaSourceCreationStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AdaptiveMediaSourceCreationStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for AdaptiveMediaSourceCreationStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceCreationStatus;i4)");
}
impl ::windows::runtime::DefaultType for AdaptiveMediaSourceCreationStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `Media_Streaming_Adaptive`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct AdaptiveMediaSourceDiagnosticAvailableEventArgs(pub ::windows::runtime::IInspectable);
impl AdaptiveMediaSourceDiagnosticAvailableEventArgs {
    #[doc = "*Required features: `Media_Streaming_Adaptive`*"]
    pub fn DiagnosticType(&self) -> ::windows::runtime::Result<AdaptiveMediaSourceDiagnosticType> {
        let this = self;
        unsafe {
            let mut result__: AdaptiveMediaSourceDiagnosticType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AdaptiveMediaSourceDiagnosticType>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Streaming_Adaptive`, `Foundation`*"]
    pub fn RequestId(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Streaming_Adaptive`, `Foundation`*"]
    pub fn Position(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Streaming_Adaptive`, `Foundation`*"]
    pub fn SegmentId(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<u64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<u64>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Streaming_Adaptive`, `Foundation`*"]
    pub fn ResourceType(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<AdaptiveMediaSourceResourceType>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<AdaptiveMediaSourceResourceType>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Streaming_Adaptive`, `Foundation`*"]
    pub fn ResourceUri(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Streaming_Adaptive`, `Foundation`*"]
    pub fn ResourceByteRangeOffset(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<u64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<u64>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Streaming_Adaptive`, `Foundation`*"]
    pub fn ResourceByteRangeLength(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<u64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<u64>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Streaming_Adaptive`, `Foundation`*"]
    pub fn Bitrate(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<u32>>(result__)
        }
    }
    #[doc = "*Required features: `Media_Streaming_Adaptive`*"]
    pub fn ExtendedError(&self) -> ::windows::runtime::Result<::windows::runtime::HRESULT> {
        let this = &::windows::runtime::Interface::cast::<IAdaptiveMediaSourceDiagnosticAvailableEventArgs2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::HRESULT = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HRESULT>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Streaming_Adaptive`, `Foundation`*"]
    pub fn ResourceDuration(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>> {
        let this = &::windows::runtime::Interface::cast::<IAdaptiveMediaSourceDiagnosticAvailableEventArgs3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[doc = "*Required features: `Media_Streaming_Adaptive`*"]
    pub fn ResourceContentType(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IAdaptiveMediaSourceDiagnosticAvailableEventArgs3>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AdaptiveMediaSourceDiagnosticAvailableEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDiagnosticAvailableEventArgs;{3af64f06-6d9c-494a-b7a9-b3a5dee6ad68})");
}
unsafe impl ::windows::runtime::Interface for AdaptiveMediaSourceDiagnosticAvailableEventArgs {
    type Vtable = IAdaptiveMediaSourceDiagnosticAvailableEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(989220614, 28060, 18762, [183, 169, 179, 165, 222, 230, 173, 104]);
}
impl ::windows::runtime::RuntimeName for AdaptiveMediaSourceDiagnosticAvailableEventArgs {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDiagnosticAvailableEventArgs";
}
impl ::std::convert::From<AdaptiveMediaSourceDiagnosticAvailableEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: AdaptiveMediaSourceDiagnosticAvailableEventArgs) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&AdaptiveMediaSourceDiagnosticAvailableEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &AdaptiveMediaSourceDiagnosticAvailableEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AdaptiveMediaSourceDiagnosticAvailableEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a AdaptiveMediaSourceDiagnosticAvailableEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<AdaptiveMediaSourceDiagnosticAvailableEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: AdaptiveMediaSourceDiagnosticAvailableEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AdaptiveMediaSourceDiagnosticAvailableEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &AdaptiveMediaSourceDiagnosticAvailableEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AdaptiveMediaSourceDiagnosticAvailableEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AdaptiveMediaSourceDiagnosticAvailableEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for AdaptiveMediaSourceDiagnosticAvailableEventArgs {}
unsafe impl ::std::marker::Sync for AdaptiveMediaSourceDiagnosticAvailableEventArgs {}
#[doc = "*Required features: `Media_Streaming_Adaptive`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct AdaptiveMediaSourceDiagnosticType(pub i32);
impl AdaptiveMediaSourceDiagnosticType {
    pub const ManifestUnchangedUponReload: AdaptiveMediaSourceDiagnosticType = AdaptiveMediaSourceDiagnosticType(0i32);
    pub const ManifestMismatchUponReload: AdaptiveMediaSourceDiagnosticType = AdaptiveMediaSourceDiagnosticType(1i32);
    pub const ManifestSignaledEndOfLiveEventUponReload: AdaptiveMediaSourceDiagnosticType = AdaptiveMediaSourceDiagnosticType(2i32);
    pub const MediaSegmentSkipped: AdaptiveMediaSourceDiagnosticType = AdaptiveMediaSourceDiagnosticType(3i32);
    pub const ResourceNotFound: AdaptiveMediaSourceDiagnosticType = AdaptiveMediaSourceDiagnosticType(4i32);
    pub const ResourceTimedOut: AdaptiveMediaSourceDiagnosticType = AdaptiveMediaSourceDiagnosticType(5i32);
    pub const ResourceParsingError: AdaptiveMediaSourceDiagnosticType = AdaptiveMediaSourceDiagnosticType(6i32);
    pub const BitrateDisabled: AdaptiveMediaSourceDiagnosticType = AdaptiveMediaSourceDiagnosticType(7i32);
    pub const FatalMediaSourceError: AdaptiveMediaSourceDiagnosticType = AdaptiveMediaSourceDiagnosticType(8i32);
}
impl ::std::convert::From<i32> for AdaptiveMediaSourceDiagnosticType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AdaptiveMediaSourceDiagnosticType {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for AdaptiveMediaSourceDiagnosticType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDiagnosticType;i4)");
}
impl ::windows::runtime::DefaultType for AdaptiveMediaSourceDiagnosticType {
    type DefaultType = Self;
}
#[doc = "*Required features: `Media_Streaming_Adaptive`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct AdaptiveMediaSourceDiagnostics(pub ::windows::runtime::IInspectable);
impl AdaptiveMediaSourceDiagnostics {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Streaming_Adaptive`, `Foundation`*"]
    pub fn DiagnosticAvailable<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<AdaptiveMediaSourceDiagnostics, AdaptiveMediaSourceDiagnosticAvailableEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Streaming_Adaptive`, `Foundation`*"]
    pub fn RemoveDiagnosticAvailable<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AdaptiveMediaSourceDiagnostics {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDiagnostics;{9b24ee68-962e-448c-aebf-b29b56098e23})");
}
unsafe impl ::windows::runtime::Interface for AdaptiveMediaSourceDiagnostics {
    type Vtable = IAdaptiveMediaSourceDiagnostics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2602888808, 38446, 17548, [174, 191, 178, 155, 86, 9, 142, 35]);
}
impl ::windows::runtime::RuntimeName for AdaptiveMediaSourceDiagnostics {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDiagnostics";
}
impl ::std::convert::From<AdaptiveMediaSourceDiagnostics> for ::windows::runtime::IUnknown {
    fn from(value: AdaptiveMediaSourceDiagnostics) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&AdaptiveMediaSourceDiagnostics> for ::windows::runtime::IUnknown {
    fn from(value: &AdaptiveMediaSourceDiagnostics) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AdaptiveMediaSourceDiagnostics {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a AdaptiveMediaSourceDiagnostics {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<AdaptiveMediaSourceDiagnostics> for ::windows::runtime::IInspectable {
    fn from(value: AdaptiveMediaSourceDiagnostics) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AdaptiveMediaSourceDiagnostics> for ::windows::runtime::IInspectable {
    fn from(value: &AdaptiveMediaSourceDiagnostics) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AdaptiveMediaSourceDiagnostics {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AdaptiveMediaSourceDiagnostics {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for AdaptiveMediaSourceDiagnostics {}
unsafe impl ::std::marker::Sync for AdaptiveMediaSourceDiagnostics {}
#[doc = "*Required features: `Media_Streaming_Adaptive`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct AdaptiveMediaSourceDownloadBitrateChangedEventArgs(pub ::windows::runtime::IInspectable);
impl AdaptiveMediaSourceDownloadBitrateChangedEventArgs {
    #[doc = "*Required features: `Media_Streaming_Adaptive`*"]
    pub fn OldValue(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Media_Streaming_Adaptive`*"]
    pub fn NewValue(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Media_Streaming_Adaptive`*"]
    pub fn Reason(&self) -> ::windows::runtime::Result<AdaptiveMediaSourceDownloadBitrateChangedReason> {
        let this = &::windows::runtime::Interface::cast::<IAdaptiveMediaSourceDownloadBitrateChangedEventArgs2>(self)?;
        unsafe {
            let mut result__: AdaptiveMediaSourceDownloadBitrateChangedReason = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AdaptiveMediaSourceDownloadBitrateChangedReason>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AdaptiveMediaSourceDownloadBitrateChangedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadBitrateChangedEventArgs;{670c0a44-e04e-4eff-816a-17399f78f4ba})");
}
unsafe impl ::windows::runtime::Interface for AdaptiveMediaSourceDownloadBitrateChangedEventArgs {
    type Vtable = IAdaptiveMediaSourceDownloadBitrateChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1728842308, 57422, 20223, [129, 106, 23, 57, 159, 120, 244, 186]);
}
impl ::windows::runtime::RuntimeName for AdaptiveMediaSourceDownloadBitrateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadBitrateChangedEventArgs";
}
impl ::std::convert::From<AdaptiveMediaSourceDownloadBitrateChangedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: AdaptiveMediaSourceDownloadBitrateChangedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&AdaptiveMediaSourceDownloadBitrateChangedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &AdaptiveMediaSourceDownloadBitrateChangedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AdaptiveMediaSourceDownloadBitrateChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a AdaptiveMediaSourceDownloadBitrateChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<AdaptiveMediaSourceDownloadBitrateChangedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: AdaptiveMediaSourceDownloadBitrateChangedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AdaptiveMediaSourceDownloadBitrateChangedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &AdaptiveMediaSourceDownloadBitrateChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AdaptiveMediaSourceDownloadBitrateChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AdaptiveMediaSourceDownloadBitrateChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for AdaptiveMediaSourceDownloadBitrateChangedEventArgs {}
unsafe impl ::std::marker::Sync for AdaptiveMediaSourceDownloadBitrateChangedEventArgs {}
#[doc = "*Required features: `Media_Streaming_Adaptive`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct AdaptiveMediaSourceDownloadBitrateChangedReason(pub i32);
impl AdaptiveMediaSourceDownloadBitrateChangedReason {
    pub const SufficientInboundBitsPerSecond: AdaptiveMediaSourceDownloadBitrateChangedReason = AdaptiveMediaSourceDownloadBitrateChangedReason(0i32);
    pub const InsufficientInboundBitsPerSecond: AdaptiveMediaSourceDownloadBitrateChangedReason = AdaptiveMediaSourceDownloadBitrateChangedReason(1i32);
    pub const LowBufferLevel: AdaptiveMediaSourceDownloadBitrateChangedReason = AdaptiveMediaSourceDownloadBitrateChangedReason(2i32);
    pub const PositionChanged: AdaptiveMediaSourceDownloadBitrateChangedReason = AdaptiveMediaSourceDownloadBitrateChangedReason(3i32);
    pub const TrackSelectionChanged: AdaptiveMediaSourceDownloadBitrateChangedReason = AdaptiveMediaSourceDownloadBitrateChangedReason(4i32);
    pub const DesiredBitratesChanged: AdaptiveMediaSourceDownloadBitrateChangedReason = AdaptiveMediaSourceDownloadBitrateChangedReason(5i32);
    pub const ErrorInPreviousBitrate: AdaptiveMediaSourceDownloadBitrateChangedReason = AdaptiveMediaSourceDownloadBitrateChangedReason(6i32);
}
impl ::std::convert::From<i32> for AdaptiveMediaSourceDownloadBitrateChangedReason {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AdaptiveMediaSourceDownloadBitrateChangedReason {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for AdaptiveMediaSourceDownloadBitrateChangedReason {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadBitrateChangedReason;i4)");
}
impl ::windows::runtime::DefaultType for AdaptiveMediaSourceDownloadBitrateChangedReason {
    type DefaultType = Self;
}
#[doc = "*Required features: `Media_Streaming_Adaptive`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct AdaptiveMediaSourceDownloadCompletedEventArgs(pub ::windows::runtime::IInspectable);
impl AdaptiveMediaSourceDownloadCompletedEventArgs {
    #[doc = "*Required features: `Media_Streaming_Adaptive`*"]
    pub fn ResourceType(&self) -> ::windows::runtime::Result<AdaptiveMediaSourceResourceType> {
        let this = self;
        unsafe {
            let mut result__: AdaptiveMediaSourceResourceType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AdaptiveMediaSourceResourceType>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Streaming_Adaptive`, `Foundation`*"]
    pub fn ResourceUri(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Streaming_Adaptive`, `Foundation`*"]
    pub fn ResourceByteRangeOffset(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<u64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<u64>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Streaming_Adaptive`, `Foundation`*"]
    pub fn ResourceByteRangeLength(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<u64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<u64>>(result__)
        }
    }
    #[cfg(feature = "Web_Http")]
    #[doc = "*Required features: `Media_Streaming_Adaptive`, `Web_Http`*"]
    pub fn HttpResponseMessage(&self) -> ::windows::runtime::Result<super::super::super::Web::Http::HttpResponseMessage> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Web::Http::HttpResponseMessage>(result__)
        }
    }
    #[doc = "*Required features: `Media_Streaming_Adaptive`*"]
    pub fn RequestId(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<IAdaptiveMediaSourceDownloadCompletedEventArgs2>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Media_Streaming_Adaptive`*"]
    pub fn Statistics(&self) -> ::windows::runtime::Result<AdaptiveMediaSourceDownloadStatistics> {
        let this = &::windows::runtime::Interface::cast::<IAdaptiveMediaSourceDownloadCompletedEventArgs2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AdaptiveMediaSourceDownloadStatistics>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Streaming_Adaptive`, `Foundation`*"]
    pub fn Position(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>> {
        let this = &::windows::runtime::Interface::cast::<IAdaptiveMediaSourceDownloadCompletedEventArgs2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Streaming_Adaptive`, `Foundation`*"]
    pub fn ResourceDuration(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>> {
        let this = &::windows::runtime::Interface::cast::<IAdaptiveMediaSourceDownloadCompletedEventArgs3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[doc = "*Required features: `Media_Streaming_Adaptive`*"]
    pub fn ResourceContentType(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IAdaptiveMediaSourceDownloadCompletedEventArgs3>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AdaptiveMediaSourceDownloadCompletedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadCompletedEventArgs;{19240dc3-5b37-4a1a-8970-d621cb6ca83b})");
}
unsafe impl ::windows::runtime::Interface for AdaptiveMediaSourceDownloadCompletedEventArgs {
    type Vtable = IAdaptiveMediaSourceDownloadCompletedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(421793219, 23351, 18970, [137, 112, 214, 33, 203, 108, 168, 59]);
}
impl ::windows::runtime::RuntimeName for AdaptiveMediaSourceDownloadCompletedEventArgs {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadCompletedEventArgs";
}
impl ::std::convert::From<AdaptiveMediaSourceDownloadCompletedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: AdaptiveMediaSourceDownloadCompletedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&AdaptiveMediaSourceDownloadCompletedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &AdaptiveMediaSourceDownloadCompletedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AdaptiveMediaSourceDownloadCompletedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a AdaptiveMediaSourceDownloadCompletedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<AdaptiveMediaSourceDownloadCompletedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: AdaptiveMediaSourceDownloadCompletedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AdaptiveMediaSourceDownloadCompletedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &AdaptiveMediaSourceDownloadCompletedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AdaptiveMediaSourceDownloadCompletedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AdaptiveMediaSourceDownloadCompletedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for AdaptiveMediaSourceDownloadCompletedEventArgs {}
unsafe impl ::std::marker::Sync for AdaptiveMediaSourceDownloadCompletedEventArgs {}
#[doc = "*Required features: `Media_Streaming_Adaptive`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct AdaptiveMediaSourceDownloadFailedEventArgs(pub ::windows::runtime::IInspectable);
impl AdaptiveMediaSourceDownloadFailedEventArgs {
    #[doc = "*Required features: `Media_Streaming_Adaptive`*"]
    pub fn ResourceType(&self) -> ::windows::runtime::Result<AdaptiveMediaSourceResourceType> {
        let this = self;
        unsafe {
            let mut result__: AdaptiveMediaSourceResourceType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AdaptiveMediaSourceResourceType>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Streaming_Adaptive`, `Foundation`*"]
    pub fn ResourceUri(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Streaming_Adaptive`, `Foundation`*"]
    pub fn ResourceByteRangeOffset(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<u64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<u64>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Streaming_Adaptive`, `Foundation`*"]
    pub fn ResourceByteRangeLength(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<u64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<u64>>(result__)
        }
    }
    #[cfg(feature = "Web_Http")]
    #[doc = "*Required features: `Media_Streaming_Adaptive`, `Web_Http`*"]
    pub fn HttpResponseMessage(&self) -> ::windows::runtime::Result<super::super::super::Web::Http::HttpResponseMessage> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Web::Http::HttpResponseMessage>(result__)
        }
    }
    #[doc = "*Required features: `Media_Streaming_Adaptive`*"]
    pub fn RequestId(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<IAdaptiveMediaSourceDownloadFailedEventArgs2>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Media_Streaming_Adaptive`*"]
    pub fn ExtendedError(&self) -> ::windows::runtime::Result<::windows::runtime::HRESULT> {
        let this = &::windows::runtime::Interface::cast::<IAdaptiveMediaSourceDownloadFailedEventArgs2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::HRESULT = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HRESULT>(result__)
        }
    }
    #[doc = "*Required features: `Media_Streaming_Adaptive`*"]
    pub fn Statistics(&self) -> ::windows::runtime::Result<AdaptiveMediaSourceDownloadStatistics> {
        let this = &::windows::runtime::Interface::cast::<IAdaptiveMediaSourceDownloadFailedEventArgs2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AdaptiveMediaSourceDownloadStatistics>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Streaming_Adaptive`, `Foundation`*"]
    pub fn Position(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>> {
        let this = &::windows::runtime::Interface::cast::<IAdaptiveMediaSourceDownloadFailedEventArgs2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Streaming_Adaptive`, `Foundation`*"]
    pub fn ResourceDuration(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>> {
        let this = &::windows::runtime::Interface::cast::<IAdaptiveMediaSourceDownloadFailedEventArgs3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[doc = "*Required features: `Media_Streaming_Adaptive`*"]
    pub fn ResourceContentType(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IAdaptiveMediaSourceDownloadFailedEventArgs3>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AdaptiveMediaSourceDownloadFailedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadFailedEventArgs;{37739048-f4ab-40a4-b135-c6dfd8bd7ff1})");
}
unsafe impl ::windows::runtime::Interface for AdaptiveMediaSourceDownloadFailedEventArgs {
    type Vtable = IAdaptiveMediaSourceDownloadFailedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(930320456, 62635, 16548, [177, 53, 198, 223, 216, 189, 127, 241]);
}
impl ::windows::runtime::RuntimeName for AdaptiveMediaSourceDownloadFailedEventArgs {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadFailedEventArgs";
}
impl ::std::convert::From<AdaptiveMediaSourceDownloadFailedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: AdaptiveMediaSourceDownloadFailedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&AdaptiveMediaSourceDownloadFailedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &AdaptiveMediaSourceDownloadFailedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AdaptiveMediaSourceDownloadFailedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a AdaptiveMediaSourceDownloadFailedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<AdaptiveMediaSourceDownloadFailedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: AdaptiveMediaSourceDownloadFailedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AdaptiveMediaSourceDownloadFailedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &AdaptiveMediaSourceDownloadFailedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AdaptiveMediaSourceDownloadFailedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AdaptiveMediaSourceDownloadFailedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for AdaptiveMediaSourceDownloadFailedEventArgs {}
unsafe impl ::std::marker::Sync for AdaptiveMediaSourceDownloadFailedEventArgs {}
#[doc = "*Required features: `Media_Streaming_Adaptive`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct AdaptiveMediaSourceDownloadRequestedDeferral(pub ::windows::runtime::IInspectable);
impl AdaptiveMediaSourceDownloadRequestedDeferral {
    #[doc = "*Required features: `Media_Streaming_Adaptive`*"]
    pub fn Complete(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AdaptiveMediaSourceDownloadRequestedDeferral {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadRequestedDeferral;{05c68f64-fa20-4dbd-9821-4bf4c9bf77ab})");
}
unsafe impl ::windows::runtime::Interface for AdaptiveMediaSourceDownloadRequestedDeferral {
    type Vtable = IAdaptiveMediaSourceDownloadRequestedDeferral_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(96898916, 64032, 19901, [152, 33, 75, 244, 201, 191, 119, 171]);
}
impl ::windows::runtime::RuntimeName for AdaptiveMediaSourceDownloadRequestedDeferral {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadRequestedDeferral";
}
impl ::std::convert::From<AdaptiveMediaSourceDownloadRequestedDeferral> for ::windows::runtime::IUnknown {
    fn from(value: AdaptiveMediaSourceDownloadRequestedDeferral) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&AdaptiveMediaSourceDownloadRequestedDeferral> for ::windows::runtime::IUnknown {
    fn from(value: &AdaptiveMediaSourceDownloadRequestedDeferral) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AdaptiveMediaSourceDownloadRequestedDeferral {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a AdaptiveMediaSourceDownloadRequestedDeferral {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<AdaptiveMediaSourceDownloadRequestedDeferral> for ::windows::runtime::IInspectable {
    fn from(value: AdaptiveMediaSourceDownloadRequestedDeferral) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AdaptiveMediaSourceDownloadRequestedDeferral> for ::windows::runtime::IInspectable {
    fn from(value: &AdaptiveMediaSourceDownloadRequestedDeferral) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AdaptiveMediaSourceDownloadRequestedDeferral {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AdaptiveMediaSourceDownloadRequestedDeferral {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for AdaptiveMediaSourceDownloadRequestedDeferral {}
unsafe impl ::std::marker::Sync for AdaptiveMediaSourceDownloadRequestedDeferral {}
#[doc = "*Required features: `Media_Streaming_Adaptive`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct AdaptiveMediaSourceDownloadRequestedEventArgs(pub ::windows::runtime::IInspectable);
impl AdaptiveMediaSourceDownloadRequestedEventArgs {
    #[doc = "*Required features: `Media_Streaming_Adaptive`*"]
    pub fn ResourceType(&self) -> ::windows::runtime::Result<AdaptiveMediaSourceResourceType> {
        let this = self;
        unsafe {
            let mut result__: AdaptiveMediaSourceResourceType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AdaptiveMediaSourceResourceType>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Streaming_Adaptive`, `Foundation`*"]
    pub fn ResourceUri(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Streaming_Adaptive`, `Foundation`*"]
    pub fn ResourceByteRangeOffset(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<u64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<u64>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Streaming_Adaptive`, `Foundation`*"]
    pub fn ResourceByteRangeLength(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<u64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<u64>>(result__)
        }
    }
    #[doc = "*Required features: `Media_Streaming_Adaptive`*"]
    pub fn Result(&self) -> ::windows::runtime::Result<AdaptiveMediaSourceDownloadResult> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AdaptiveMediaSourceDownloadResult>(result__)
        }
    }
    #[doc = "*Required features: `Media_Streaming_Adaptive`*"]
    pub fn GetDeferral(&self) -> ::windows::runtime::Result<AdaptiveMediaSourceDownloadRequestedDeferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AdaptiveMediaSourceDownloadRequestedDeferral>(result__)
        }
    }
    #[doc = "*Required features: `Media_Streaming_Adaptive`*"]
    pub fn RequestId(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<IAdaptiveMediaSourceDownloadRequestedEventArgs2>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Streaming_Adaptive`, `Foundation`*"]
    pub fn Position(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>> {
        let this = &::windows::runtime::Interface::cast::<IAdaptiveMediaSourceDownloadRequestedEventArgs2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Streaming_Adaptive`, `Foundation`*"]
    pub fn ResourceDuration(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>> {
        let this = &::windows::runtime::Interface::cast::<IAdaptiveMediaSourceDownloadRequestedEventArgs3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[doc = "*Required features: `Media_Streaming_Adaptive`*"]
    pub fn ResourceContentType(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IAdaptiveMediaSourceDownloadRequestedEventArgs3>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AdaptiveMediaSourceDownloadRequestedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadRequestedEventArgs;{c83fdffd-44a9-47a2-bf96-03398b4bfaaf})");
}
unsafe impl ::windows::runtime::Interface for AdaptiveMediaSourceDownloadRequestedEventArgs {
    type Vtable = IAdaptiveMediaSourceDownloadRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3359629309, 17577, 18338, [191, 150, 3, 57, 139, 75, 250, 175]);
}
impl ::windows::runtime::RuntimeName for AdaptiveMediaSourceDownloadRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadRequestedEventArgs";
}
impl ::std::convert::From<AdaptiveMediaSourceDownloadRequestedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: AdaptiveMediaSourceDownloadRequestedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&AdaptiveMediaSourceDownloadRequestedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &AdaptiveMediaSourceDownloadRequestedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AdaptiveMediaSourceDownloadRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a AdaptiveMediaSourceDownloadRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<AdaptiveMediaSourceDownloadRequestedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: AdaptiveMediaSourceDownloadRequestedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AdaptiveMediaSourceDownloadRequestedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &AdaptiveMediaSourceDownloadRequestedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AdaptiveMediaSourceDownloadRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AdaptiveMediaSourceDownloadRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for AdaptiveMediaSourceDownloadRequestedEventArgs {}
unsafe impl ::std::marker::Sync for AdaptiveMediaSourceDownloadRequestedEventArgs {}
#[doc = "*Required features: `Media_Streaming_Adaptive`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct AdaptiveMediaSourceDownloadResult(pub ::windows::runtime::IInspectable);
impl AdaptiveMediaSourceDownloadResult {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Streaming_Adaptive`, `Foundation`*"]
    pub fn ResourceUri(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Streaming_Adaptive`, `Foundation`*"]
    pub fn SetResourceUri<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Media_Streaming_Adaptive`, `Storage_Streams`*"]
    pub fn InputStream(&self) -> ::windows::runtime::Result<super::super::super::Storage::Streams::IInputStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Storage::Streams::IInputStream>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Media_Streaming_Adaptive`, `Storage_Streams`*"]
    pub fn SetInputStream<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Storage::Streams::IInputStream>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Media_Streaming_Adaptive`, `Storage_Streams`*"]
    pub fn Buffer(&self) -> ::windows::runtime::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Media_Streaming_Adaptive`, `Storage_Streams`*"]
    pub fn SetBuffer<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media_Streaming_Adaptive`*"]
    pub fn ContentType(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Media_Streaming_Adaptive`*"]
    pub fn SetContentType<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Media_Streaming_Adaptive`*"]
    pub fn ExtendedStatus(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Media_Streaming_Adaptive`*"]
    pub fn SetExtendedStatus(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Streaming_Adaptive`, `Foundation`*"]
    pub fn ResourceByteRangeOffset(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<u64>> {
        let this = &::windows::runtime::Interface::cast::<IAdaptiveMediaSourceDownloadResult2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<u64>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Streaming_Adaptive`, `Foundation`*"]
    pub fn SetResourceByteRangeOffset<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IReference<u64>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAdaptiveMediaSourceDownloadResult2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Streaming_Adaptive`, `Foundation`*"]
    pub fn ResourceByteRangeLength(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<u64>> {
        let this = &::windows::runtime::Interface::cast::<IAdaptiveMediaSourceDownloadResult2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<u64>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Streaming_Adaptive`, `Foundation`*"]
    pub fn SetResourceByteRangeLength<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IReference<u64>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAdaptiveMediaSourceDownloadResult2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AdaptiveMediaSourceDownloadResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadResult;{f4afdc73-bcee-4a6a-9f0a-fec41e2339b0})");
}
unsafe impl ::windows::runtime::Interface for AdaptiveMediaSourceDownloadResult {
    type Vtable = IAdaptiveMediaSourceDownloadResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4105165939, 48366, 19050, [159, 10, 254, 196, 30, 35, 57, 176]);
}
impl ::windows::runtime::RuntimeName for AdaptiveMediaSourceDownloadResult {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadResult";
}
impl ::std::convert::From<AdaptiveMediaSourceDownloadResult> for ::windows::runtime::IUnknown {
    fn from(value: AdaptiveMediaSourceDownloadResult) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&AdaptiveMediaSourceDownloadResult> for ::windows::runtime::IUnknown {
    fn from(value: &AdaptiveMediaSourceDownloadResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AdaptiveMediaSourceDownloadResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a AdaptiveMediaSourceDownloadResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<AdaptiveMediaSourceDownloadResult> for ::windows::runtime::IInspectable {
    fn from(value: AdaptiveMediaSourceDownloadResult) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AdaptiveMediaSourceDownloadResult> for ::windows::runtime::IInspectable {
    fn from(value: &AdaptiveMediaSourceDownloadResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AdaptiveMediaSourceDownloadResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AdaptiveMediaSourceDownloadResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for AdaptiveMediaSourceDownloadResult {}
unsafe impl ::std::marker::Sync for AdaptiveMediaSourceDownloadResult {}
#[doc = "*Required features: `Media_Streaming_Adaptive`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct AdaptiveMediaSourceDownloadStatistics(pub ::windows::runtime::IInspectable);
impl AdaptiveMediaSourceDownloadStatistics {
    #[doc = "*Required features: `Media_Streaming_Adaptive`*"]
    pub fn ContentBytesReceivedCount(&self) -> ::windows::runtime::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Streaming_Adaptive`, `Foundation`*"]
    pub fn TimeToHeadersReceived(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Streaming_Adaptive`, `Foundation`*"]
    pub fn TimeToFirstByteReceived(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_Streaming_Adaptive`, `Foundation`*"]
    pub fn TimeToLastByteReceived(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AdaptiveMediaSourceDownloadStatistics {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadStatistics;{a306cefb-e96a-4dff-a9b8-1ae08c01ae98})");
}
unsafe impl ::windows::runtime::Interface for AdaptiveMediaSourceDownloadStatistics {
    type Vtable = IAdaptiveMediaSourceDownloadStatistics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2735132411, 59754, 19967, [169, 184, 26, 224, 140, 1, 174, 152]);
}
impl ::windows::runtime::RuntimeName for AdaptiveMediaSourceDownloadStatistics {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadStatistics";
}
impl ::std::convert::From<AdaptiveMediaSourceDownloadStatistics> for ::windows::runtime::IUnknown {
    fn from(value: AdaptiveMediaSourceDownloadStatistics) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&AdaptiveMediaSourceDownloadStatistics> for ::windows::runtime::IUnknown {
    fn from(value: &AdaptiveMediaSourceDownloadStatistics) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AdaptiveMediaSourceDownloadStatistics {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a AdaptiveMediaSourceDownloadStatistics {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<AdaptiveMediaSourceDownloadStatistics> for ::windows::runtime::IInspectable {
    fn from(value: AdaptiveMediaSourceDownloadStatistics) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AdaptiveMediaSourceDownloadStatistics> for ::windows::runtime::IInspectable {
    fn from(value: &AdaptiveMediaSourceDownloadStatistics) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AdaptiveMediaSourceDownloadStatistics {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AdaptiveMediaSourceDownloadStatistics {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for AdaptiveMediaSourceDownloadStatistics {}
unsafe impl ::std::marker::Sync for AdaptiveMediaSourceDownloadStatistics {}
#[doc = "*Required features: `Media_Streaming_Adaptive`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct AdaptiveMediaSourcePlaybackBitrateChangedEventArgs(pub ::windows::runtime::IInspectable);
impl AdaptiveMediaSourcePlaybackBitrateChangedEventArgs {
    #[doc = "*Required features: `Media_Streaming_Adaptive`*"]
    pub fn OldValue(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Media_Streaming_Adaptive`*"]
    pub fn NewValue(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Media_Streaming_Adaptive`*"]
    pub fn AudioOnly(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AdaptiveMediaSourcePlaybackBitrateChangedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Streaming.Adaptive.AdaptiveMediaSourcePlaybackBitrateChangedEventArgs;{23a29f6d-7dda-4a51-87a9-6fa8c5b292be})");
}
unsafe impl ::windows::runtime::Interface for AdaptiveMediaSourcePlaybackBitrateChangedEventArgs {
    type Vtable = IAdaptiveMediaSourcePlaybackBitrateChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(597860205, 32218, 19025, [135, 169, 111, 168, 197, 178, 146, 190]);
}
impl ::windows::runtime::RuntimeName for AdaptiveMediaSourcePlaybackBitrateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.AdaptiveMediaSourcePlaybackBitrateChangedEventArgs";
}
impl ::std::convert::From<AdaptiveMediaSourcePlaybackBitrateChangedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: AdaptiveMediaSourcePlaybackBitrateChangedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&AdaptiveMediaSourcePlaybackBitrateChangedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &AdaptiveMediaSourcePlaybackBitrateChangedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AdaptiveMediaSourcePlaybackBitrateChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a AdaptiveMediaSourcePlaybackBitrateChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<AdaptiveMediaSourcePlaybackBitrateChangedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: AdaptiveMediaSourcePlaybackBitrateChangedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AdaptiveMediaSourcePlaybackBitrateChangedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &AdaptiveMediaSourcePlaybackBitrateChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AdaptiveMediaSourcePlaybackBitrateChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AdaptiveMediaSourcePlaybackBitrateChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for AdaptiveMediaSourcePlaybackBitrateChangedEventArgs {}
unsafe impl ::std::marker::Sync for AdaptiveMediaSourcePlaybackBitrateChangedEventArgs {}
#[doc = "*Required features: `Media_Streaming_Adaptive`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct AdaptiveMediaSourceResourceType(pub i32);
impl AdaptiveMediaSourceResourceType {
    pub const Manifest: AdaptiveMediaSourceResourceType = AdaptiveMediaSourceResourceType(0i32);
    pub const InitializationSegment: AdaptiveMediaSourceResourceType = AdaptiveMediaSourceResourceType(1i32);
    pub const MediaSegment: AdaptiveMediaSourceResourceType = AdaptiveMediaSourceResourceType(2i32);
    pub const Key: AdaptiveMediaSourceResourceType = AdaptiveMediaSourceResourceType(3i32);
    pub const InitializationVector: AdaptiveMediaSourceResourceType = AdaptiveMediaSourceResourceType(4i32);
    pub const MediaSegmentIndex: AdaptiveMediaSourceResourceType = AdaptiveMediaSourceResourceType(5i32);
}
impl ::std::convert::From<i32> for AdaptiveMediaSourceResourceType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AdaptiveMediaSourceResourceType {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for AdaptiveMediaSourceResourceType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceResourceType;i4)");
}
impl ::windows::runtime::DefaultType for AdaptiveMediaSourceResourceType {
    type DefaultType = Self;
}
#[repr(transparent)]
#[doc(hidden)]
pub struct IAdaptiveMediaSource(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAdaptiveMediaSource {
    type Vtable = IAdaptiveMediaSource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1282618095, 54175, 17302, [180, 217, 4, 57, 87, 167, 201, 100]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveMediaSource_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u64) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAdaptiveMediaSource2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAdaptiveMediaSource2 {
    type Vtable = IAdaptiveMediaSource2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(394855234, 26464, 19385, [165, 138, 247, 170, 152, 176, 140, 14]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveMediaSource2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAdaptiveMediaSource3(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAdaptiveMediaSource3 {
    type Vtable = IAdaptiveMediaSource3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3127911421, 49972, 17947, [163, 110, 201, 159, 84, 247, 23, 74]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveMediaSource3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceAdvancedSettings(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAdaptiveMediaSourceAdvancedSettings {
    type Vtable = IAdaptiveMediaSourceAdvancedSettings_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1440421504, 6891, 18396, [170, 8, 154, 17, 97, 11, 164, 90]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceAdvancedSettings_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceCorrelatedTimes(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAdaptiveMediaSourceCorrelatedTimes {
    type Vtable = IAdaptiveMediaSourceCorrelatedTimes_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(84969351, 57394, 18657, [171, 141, 0, 43, 11, 48, 81, 223]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceCorrelatedTimes_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceCreationResult(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAdaptiveMediaSourceCreationResult {
    type Vtable = IAdaptiveMediaSourceCreationResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1183233714, 32783, 20017, [144, 147, 118, 212, 120, 32, 19, 231]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceCreationResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut AdaptiveMediaSourceCreationStatus) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Web_Http")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Web_Http"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceCreationResult2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAdaptiveMediaSourceCreationResult2 {
    type Vtable = IAdaptiveMediaSourceCreationResult2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(473056191, 7236, 16459, [162, 1, 223, 69, 172, 120, 152, 232]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceCreationResult2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceDiagnosticAvailableEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAdaptiveMediaSourceDiagnosticAvailableEventArgs {
    type Vtable = IAdaptiveMediaSourceDiagnosticAvailableEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(989220614, 28060, 18762, [183, 169, 179, 165, 222, 230, 173, 104]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceDiagnosticAvailableEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut AdaptiveMediaSourceDiagnosticType) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceDiagnosticAvailableEventArgs2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAdaptiveMediaSourceDiagnosticAvailableEventArgs2 {
    type Vtable = IAdaptiveMediaSourceDiagnosticAvailableEventArgs2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2356009047, 5797, 19871, [129, 14, 0, 189, 144, 27, 62, 249]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceDiagnosticAvailableEventArgs2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceDiagnosticAvailableEventArgs3(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAdaptiveMediaSourceDiagnosticAvailableEventArgs3 {
    type Vtable = IAdaptiveMediaSourceDiagnosticAvailableEventArgs3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3278179541, 56043, 16643, [132, 218, 104, 118, 154, 213, 19, 255]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceDiagnosticAvailableEventArgs3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceDiagnostics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAdaptiveMediaSourceDiagnostics {
    type Vtable = IAdaptiveMediaSourceDiagnostics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2602888808, 38446, 17548, [174, 191, 178, 155, 86, 9, 142, 35]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceDiagnostics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceDownloadBitrateChangedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAdaptiveMediaSourceDownloadBitrateChangedEventArgs {
    type Vtable = IAdaptiveMediaSourceDownloadBitrateChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1728842308, 57422, 20223, [129, 106, 23, 57, 159, 120, 244, 186]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceDownloadBitrateChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceDownloadBitrateChangedEventArgs2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAdaptiveMediaSourceDownloadBitrateChangedEventArgs2 {
    type Vtable = IAdaptiveMediaSourceDownloadBitrateChangedEventArgs2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4092720196, 38574, 19936, [181, 64, 43, 50, 70, 230, 150, 140]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceDownloadBitrateChangedEventArgs2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut AdaptiveMediaSourceDownloadBitrateChangedReason) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceDownloadCompletedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAdaptiveMediaSourceDownloadCompletedEventArgs {
    type Vtable = IAdaptiveMediaSourceDownloadCompletedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(421793219, 23351, 18970, [137, 112, 214, 33, 203, 108, 168, 59]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceDownloadCompletedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut AdaptiveMediaSourceResourceType) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Web_Http")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Web_Http"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceDownloadCompletedEventArgs2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAdaptiveMediaSourceDownloadCompletedEventArgs2 {
    type Vtable = IAdaptiveMediaSourceDownloadCompletedEventArgs2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1883718852, 38474, 16612, [175, 149, 145, 119, 221, 109, 250, 0]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceDownloadCompletedEventArgs2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceDownloadCompletedEventArgs3(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAdaptiveMediaSourceDownloadCompletedEventArgs3 {
    type Vtable = IAdaptiveMediaSourceDownloadCompletedEventArgs3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(260738001, 37810, 18374, [186, 220, 139, 226, 200, 247, 246, 232]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceDownloadCompletedEventArgs3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceDownloadFailedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAdaptiveMediaSourceDownloadFailedEventArgs {
    type Vtable = IAdaptiveMediaSourceDownloadFailedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(930320456, 62635, 16548, [177, 53, 198, 223, 216, 189, 127, 241]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceDownloadFailedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut AdaptiveMediaSourceResourceType) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Web_Http")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Web_Http"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceDownloadFailedEventArgs2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAdaptiveMediaSourceDownloadFailedEventArgs2 {
    type Vtable = IAdaptiveMediaSourceDownloadFailedEventArgs2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1888589160, 38524, 18822, [144, 197, 198, 252, 75, 49, 226, 216]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceDownloadFailedEventArgs2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceDownloadFailedEventArgs3(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAdaptiveMediaSourceDownloadFailedEventArgs3 {
    type Vtable = IAdaptiveMediaSourceDownloadFailedEventArgs3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3493152073, 4402, 18960, [145, 90, 194, 33, 27, 91, 148, 9]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceDownloadFailedEventArgs3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceDownloadRequestedDeferral(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAdaptiveMediaSourceDownloadRequestedDeferral {
    type Vtable = IAdaptiveMediaSourceDownloadRequestedDeferral_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(96898916, 64032, 19901, [152, 33, 75, 244, 201, 191, 119, 171]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceDownloadRequestedDeferral_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceDownloadRequestedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAdaptiveMediaSourceDownloadRequestedEventArgs {
    type Vtable = IAdaptiveMediaSourceDownloadRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3359629309, 17577, 18338, [191, 150, 3, 57, 139, 75, 250, 175]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceDownloadRequestedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut AdaptiveMediaSourceResourceType) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceDownloadRequestedEventArgs2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAdaptiveMediaSourceDownloadRequestedEventArgs2 {
    type Vtable = IAdaptiveMediaSourceDownloadRequestedEventArgs2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3011349502, 43588, 19842, [130, 91, 97, 29, 227, 188, 254, 203]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceDownloadRequestedEventArgs2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceDownloadRequestedEventArgs3(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAdaptiveMediaSourceDownloadRequestedEventArgs3 {
    type Vtable = IAdaptiveMediaSourceDownloadRequestedEventArgs3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(859590909, 20322, 17537, [171, 68, 30, 71, 176, 87, 66, 37]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceDownloadRequestedEventArgs3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceDownloadResult(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAdaptiveMediaSourceDownloadResult {
    type Vtable = IAdaptiveMediaSourceDownloadResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4105165939, 48366, 19050, [159, 10, 254, 196, 30, 35, 57, 176]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceDownloadResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceDownloadResult2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAdaptiveMediaSourceDownloadResult2 {
    type Vtable = IAdaptiveMediaSourceDownloadResult2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(357903543, 31616, 19140, [134, 96, 164, 185, 127, 124, 112, 240]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceDownloadResult2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceDownloadStatistics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAdaptiveMediaSourceDownloadStatistics {
    type Vtable = IAdaptiveMediaSourceDownloadStatistics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2735132411, 59754, 19967, [169, 184, 26, 224, 140, 1, 174, 152]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceDownloadStatistics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u64) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourcePlaybackBitrateChangedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAdaptiveMediaSourcePlaybackBitrateChangedEventArgs {
    type Vtable = IAdaptiveMediaSourcePlaybackBitrateChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(597860205, 32218, 19025, [135, 169, 111, 168, 197, 178, 146, 190]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourcePlaybackBitrateChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAdaptiveMediaSourceStatics {
    type Vtable = IAdaptiveMediaSourceStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1353104733, 26351, 19667, [149, 121, 158, 102, 5, 7, 220, 63]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, contenttype: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uri: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Web_Http"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uri: ::windows::runtime::RawPtr, httpclient: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Web_Http")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, stream: ::windows::runtime::RawPtr, uri: ::windows::runtime::RawPtr, contenttype: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "Web_Http"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, stream: ::windows::runtime::RawPtr, uri: ::windows::runtime::RawPtr, contenttype: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, httpclient: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams", feature = "Web_Http")))] usize,
);
