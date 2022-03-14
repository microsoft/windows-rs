#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Media_Streaming_Adaptive\"`*"]
#[repr(transparent)]
pub struct AdaptiveMediaSource(::windows::core::IUnknown);
impl AdaptiveMediaSource {
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`*"]
    pub fn IsLive(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsLive)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DesiredLiveOffset(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DesiredLiveOffset)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDesiredLiveOffset<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDesiredLiveOffset)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`*"]
    pub fn InitialBitrate(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).InitialBitrate)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`*"]
    pub fn SetInitialBitrate(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetInitialBitrate)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`*"]
    pub fn CurrentDownloadBitrate(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CurrentDownloadBitrate)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`*"]
    pub fn CurrentPlaybackBitrate(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CurrentPlaybackBitrate)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AvailableBitrates(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AvailableBitrates)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<u32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DesiredMinBitrate(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DesiredMinBitrate)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<u32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDesiredMinBitrate<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::IReference<u32>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDesiredMinBitrate)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DesiredMaxBitrate(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DesiredMaxBitrate)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<u32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDesiredMaxBitrate<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::IReference<u32>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDesiredMaxBitrate)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`*"]
    pub fn AudioOnlyPlayback(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AudioOnlyPlayback)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`*"]
    pub fn InboundBitsPerSecond(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).InboundBitsPerSecond)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn InboundBitsPerSecondWindow(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).InboundBitsPerSecondWindow)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetInboundBitsPerSecondWindow<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetInboundBitsPerSecondWindow)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DownloadBitrateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<AdaptiveMediaSource, AdaptiveMediaSourceDownloadBitrateChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DownloadBitrateChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveDownloadBitrateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveDownloadBitrateChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PlaybackBitrateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<AdaptiveMediaSource, AdaptiveMediaSourcePlaybackBitrateChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PlaybackBitrateChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePlaybackBitrateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemovePlaybackBitrateChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DownloadRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<AdaptiveMediaSource, AdaptiveMediaSourceDownloadRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DownloadRequested)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveDownloadRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveDownloadRequested)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DownloadCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<AdaptiveMediaSource, AdaptiveMediaSourceDownloadCompletedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DownloadCompleted)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveDownloadCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveDownloadCompleted)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DownloadFailed<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<AdaptiveMediaSource, AdaptiveMediaSourceDownloadFailedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DownloadFailed)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveDownloadFailed<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveDownloadFailed)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`*"]
    pub fn AdvancedSettings(&self) -> ::windows::core::Result<AdaptiveMediaSourceAdvancedSettings> {
        let this = &::windows::core::Interface::cast::<IAdaptiveMediaSource2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AdvancedSettings)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AdaptiveMediaSourceAdvancedSettings>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MinLiveOffset(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>> {
        let this = &::windows::core::Interface::cast::<IAdaptiveMediaSource3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MinLiveOffset)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxSeekableWindowSize(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>> {
        let this = &::windows::core::Interface::cast::<IAdaptiveMediaSource3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MaxSeekableWindowSize)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DesiredSeekableWindowSize(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>> {
        let this = &::windows::core::Interface::cast::<IAdaptiveMediaSource3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DesiredSeekableWindowSize)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDesiredSeekableWindowSize<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAdaptiveMediaSource3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetDesiredSeekableWindowSize)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`*"]
    pub fn Diagnostics(&self) -> ::windows::core::Result<AdaptiveMediaSourceDiagnostics> {
        let this = &::windows::core::Interface::cast::<IAdaptiveMediaSource3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Diagnostics)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AdaptiveMediaSourceDiagnostics>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`*"]
    pub fn GetCorrelatedTimes(&self) -> ::windows::core::Result<AdaptiveMediaSourceCorrelatedTimes> {
        let this = &::windows::core::Interface::cast::<IAdaptiveMediaSource3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetCorrelatedTimes)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AdaptiveMediaSourceCorrelatedTimes>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`*"]
    pub fn IsContentTypeSupported<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(contenttype: Param0) -> ::windows::core::Result<bool> {
        Self::IAdaptiveMediaSourceStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsContentTypeSupported)(::core::mem::transmute_copy(this), contenttype.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateFromUriAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Uri>>(uri: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<AdaptiveMediaSourceCreationResult>> {
        Self::IAdaptiveMediaSourceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromUriAsync)(::core::mem::transmute_copy(this), uri.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<AdaptiveMediaSourceCreationResult>>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`, `\"Foundation\"`, `\"Web_Http\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Web_Http"))]
    pub fn CreateFromUriWithDownloaderAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Uri>, Param1: ::windows::core::IntoParam<'a, super::super::super::Web::Http::HttpClient>>(uri: Param0, httpclient: Param1) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<AdaptiveMediaSourceCreationResult>> {
        Self::IAdaptiveMediaSourceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromUriWithDownloaderAsync)(::core::mem::transmute_copy(this), uri.into_param().abi(), httpclient.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<AdaptiveMediaSourceCreationResult>>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`, `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn CreateFromStreamAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IInputStream>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::Uri>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(stream: Param0, uri: Param1, contenttype: Param2) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<AdaptiveMediaSourceCreationResult>> {
        Self::IAdaptiveMediaSourceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromStreamAsync)(::core::mem::transmute_copy(this), stream.into_param().abi(), uri.into_param().abi(), contenttype.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<AdaptiveMediaSourceCreationResult>>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`, `\"Foundation\"`, `\"Storage_Streams\"`, `\"Web_Http\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "Web_Http"))]
    pub fn CreateFromStreamWithDownloaderAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IInputStream>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::Uri>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, super::super::super::Web::Http::HttpClient>>(stream: Param0, uri: Param1, contenttype: Param2, httpclient: Param3) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<AdaptiveMediaSourceCreationResult>> {
        Self::IAdaptiveMediaSourceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromStreamWithDownloaderAsync)(::core::mem::transmute_copy(this), stream.into_param().abi(), uri.into_param().abi(), contenttype.into_param().abi(), httpclient.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<AdaptiveMediaSourceCreationResult>>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc(hidden)]
    pub fn IAdaptiveMediaSourceStatics<R, F: FnOnce(&IAdaptiveMediaSourceStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AdaptiveMediaSource, IAdaptiveMediaSourceStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for AdaptiveMediaSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AdaptiveMediaSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AdaptiveMediaSource {}
impl ::core::fmt::Debug for AdaptiveMediaSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AdaptiveMediaSource").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AdaptiveMediaSource {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Streaming.Adaptive.AdaptiveMediaSource;{4c7332ef-d39f-4396-b4d9-043957a7c964})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AdaptiveMediaSource {
    type Vtable = IAdaptiveMediaSource_Vtbl;
    const IID: ::windows::core::GUID = <IAdaptiveMediaSource as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AdaptiveMediaSource {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.AdaptiveMediaSource";
}
impl ::core::convert::From<AdaptiveMediaSource> for ::windows::core::IUnknown {
    fn from(value: AdaptiveMediaSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AdaptiveMediaSource> for ::windows::core::IUnknown {
    fn from(value: &AdaptiveMediaSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AdaptiveMediaSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AdaptiveMediaSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AdaptiveMediaSource> for ::windows::core::IInspectable {
    fn from(value: AdaptiveMediaSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AdaptiveMediaSource> for ::windows::core::IInspectable {
    fn from(value: &AdaptiveMediaSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AdaptiveMediaSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AdaptiveMediaSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<AdaptiveMediaSource> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: AdaptiveMediaSource) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&AdaptiveMediaSource> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &AdaptiveMediaSource) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IClosable> for AdaptiveMediaSource {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IClosable> for &AdaptiveMediaSource {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Media_Core")]
impl ::core::convert::TryFrom<AdaptiveMediaSource> for super::super::Core::IMediaSource {
    type Error = ::windows::core::Error;
    fn try_from(value: AdaptiveMediaSource) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Media_Core")]
impl ::core::convert::TryFrom<&AdaptiveMediaSource> for super::super::Core::IMediaSource {
    type Error = ::windows::core::Error;
    fn try_from(value: &AdaptiveMediaSource) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Media_Core")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Core::IMediaSource> for AdaptiveMediaSource {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Core::IMediaSource> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Media_Core")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Core::IMediaSource> for &AdaptiveMediaSource {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Core::IMediaSource> {
        ::core::convert::TryInto::<super::super::Core::IMediaSource>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for AdaptiveMediaSource {}
unsafe impl ::core::marker::Sync for AdaptiveMediaSource {}
#[doc = "*Required features: `\"Media_Streaming_Adaptive\"`*"]
#[repr(transparent)]
pub struct AdaptiveMediaSourceAdvancedSettings(::windows::core::IUnknown);
impl AdaptiveMediaSourceAdvancedSettings {
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`*"]
    pub fn AllSegmentsIndependent(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AllSegmentsIndependent)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`*"]
    pub fn SetAllSegmentsIndependent(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAllSegmentsIndependent)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DesiredBitrateHeadroomRatio(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DesiredBitrateHeadroomRatio)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<f64>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDesiredBitrateHeadroomRatio<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::IReference<f64>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDesiredBitrateHeadroomRatio)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BitrateDowngradeTriggerRatio(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BitrateDowngradeTriggerRatio)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<f64>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetBitrateDowngradeTriggerRatio<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::IReference<f64>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetBitrateDowngradeTriggerRatio)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for AdaptiveMediaSourceAdvancedSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AdaptiveMediaSourceAdvancedSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AdaptiveMediaSourceAdvancedSettings {}
impl ::core::fmt::Debug for AdaptiveMediaSourceAdvancedSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AdaptiveMediaSourceAdvancedSettings").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AdaptiveMediaSourceAdvancedSettings {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceAdvancedSettings;{55db1680-1aeb-47dc-aa08-9a11610ba45a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AdaptiveMediaSourceAdvancedSettings {
    type Vtable = IAdaptiveMediaSourceAdvancedSettings_Vtbl;
    const IID: ::windows::core::GUID = <IAdaptiveMediaSourceAdvancedSettings as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AdaptiveMediaSourceAdvancedSettings {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceAdvancedSettings";
}
impl ::core::convert::From<AdaptiveMediaSourceAdvancedSettings> for ::windows::core::IUnknown {
    fn from(value: AdaptiveMediaSourceAdvancedSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AdaptiveMediaSourceAdvancedSettings> for ::windows::core::IUnknown {
    fn from(value: &AdaptiveMediaSourceAdvancedSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AdaptiveMediaSourceAdvancedSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AdaptiveMediaSourceAdvancedSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AdaptiveMediaSourceAdvancedSettings> for ::windows::core::IInspectable {
    fn from(value: AdaptiveMediaSourceAdvancedSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AdaptiveMediaSourceAdvancedSettings> for ::windows::core::IInspectable {
    fn from(value: &AdaptiveMediaSourceAdvancedSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AdaptiveMediaSourceAdvancedSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AdaptiveMediaSourceAdvancedSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AdaptiveMediaSourceAdvancedSettings {}
unsafe impl ::core::marker::Sync for AdaptiveMediaSourceAdvancedSettings {}
#[doc = "*Required features: `\"Media_Streaming_Adaptive\"`*"]
#[repr(transparent)]
pub struct AdaptiveMediaSourceCorrelatedTimes(::windows::core::IUnknown);
impl AdaptiveMediaSourceCorrelatedTimes {
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Position(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Position)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PresentationTimeStamp(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PresentationTimeStamp)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ProgramDateTime(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ProgramDateTime)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>(result__)
        }
    }
}
impl ::core::clone::Clone for AdaptiveMediaSourceCorrelatedTimes {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AdaptiveMediaSourceCorrelatedTimes {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AdaptiveMediaSourceCorrelatedTimes {}
impl ::core::fmt::Debug for AdaptiveMediaSourceCorrelatedTimes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AdaptiveMediaSourceCorrelatedTimes").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AdaptiveMediaSourceCorrelatedTimes {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceCorrelatedTimes;{05108787-e032-48e1-ab8d-002b0b3051df})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AdaptiveMediaSourceCorrelatedTimes {
    type Vtable = IAdaptiveMediaSourceCorrelatedTimes_Vtbl;
    const IID: ::windows::core::GUID = <IAdaptiveMediaSourceCorrelatedTimes as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AdaptiveMediaSourceCorrelatedTimes {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceCorrelatedTimes";
}
impl ::core::convert::From<AdaptiveMediaSourceCorrelatedTimes> for ::windows::core::IUnknown {
    fn from(value: AdaptiveMediaSourceCorrelatedTimes) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AdaptiveMediaSourceCorrelatedTimes> for ::windows::core::IUnknown {
    fn from(value: &AdaptiveMediaSourceCorrelatedTimes) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AdaptiveMediaSourceCorrelatedTimes {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AdaptiveMediaSourceCorrelatedTimes {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AdaptiveMediaSourceCorrelatedTimes> for ::windows::core::IInspectable {
    fn from(value: AdaptiveMediaSourceCorrelatedTimes) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AdaptiveMediaSourceCorrelatedTimes> for ::windows::core::IInspectable {
    fn from(value: &AdaptiveMediaSourceCorrelatedTimes) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AdaptiveMediaSourceCorrelatedTimes {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AdaptiveMediaSourceCorrelatedTimes {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AdaptiveMediaSourceCorrelatedTimes {}
unsafe impl ::core::marker::Sync for AdaptiveMediaSourceCorrelatedTimes {}
#[doc = "*Required features: `\"Media_Streaming_Adaptive\"`*"]
#[repr(transparent)]
pub struct AdaptiveMediaSourceCreationResult(::windows::core::IUnknown);
impl AdaptiveMediaSourceCreationResult {
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`*"]
    pub fn Status(&self) -> ::windows::core::Result<AdaptiveMediaSourceCreationStatus> {
        let this = self;
        unsafe {
            let mut result__: AdaptiveMediaSourceCreationStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AdaptiveMediaSourceCreationStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`*"]
    pub fn MediaSource(&self) -> ::windows::core::Result<AdaptiveMediaSource> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MediaSource)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AdaptiveMediaSource>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`, `\"Web_Http\"`*"]
    #[cfg(feature = "Web_Http")]
    pub fn HttpResponseMessage(&self) -> ::windows::core::Result<super::super::super::Web::Http::HttpResponseMessage> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HttpResponseMessage)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Web::Http::HttpResponseMessage>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`*"]
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = &::windows::core::Interface::cast::<IAdaptiveMediaSourceCreationResult2>(self)?;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExtendedError)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
}
impl ::core::clone::Clone for AdaptiveMediaSourceCreationResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AdaptiveMediaSourceCreationResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AdaptiveMediaSourceCreationResult {}
impl ::core::fmt::Debug for AdaptiveMediaSourceCreationResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AdaptiveMediaSourceCreationResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AdaptiveMediaSourceCreationResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceCreationResult;{4686b6b2-800f-4e31-9093-76d4782013e7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AdaptiveMediaSourceCreationResult {
    type Vtable = IAdaptiveMediaSourceCreationResult_Vtbl;
    const IID: ::windows::core::GUID = <IAdaptiveMediaSourceCreationResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AdaptiveMediaSourceCreationResult {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceCreationResult";
}
impl ::core::convert::From<AdaptiveMediaSourceCreationResult> for ::windows::core::IUnknown {
    fn from(value: AdaptiveMediaSourceCreationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AdaptiveMediaSourceCreationResult> for ::windows::core::IUnknown {
    fn from(value: &AdaptiveMediaSourceCreationResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AdaptiveMediaSourceCreationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AdaptiveMediaSourceCreationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AdaptiveMediaSourceCreationResult> for ::windows::core::IInspectable {
    fn from(value: AdaptiveMediaSourceCreationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AdaptiveMediaSourceCreationResult> for ::windows::core::IInspectable {
    fn from(value: &AdaptiveMediaSourceCreationResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AdaptiveMediaSourceCreationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AdaptiveMediaSourceCreationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AdaptiveMediaSourceCreationResult {}
unsafe impl ::core::marker::Sync for AdaptiveMediaSourceCreationResult {}
#[doc = "*Required features: `\"Media_Streaming_Adaptive\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AdaptiveMediaSourceCreationStatus(pub i32);
impl AdaptiveMediaSourceCreationStatus {
    pub const Success: Self = Self(0i32);
    pub const ManifestDownloadFailure: Self = Self(1i32);
    pub const ManifestParseFailure: Self = Self(2i32);
    pub const UnsupportedManifestContentType: Self = Self(3i32);
    pub const UnsupportedManifestVersion: Self = Self(4i32);
    pub const UnsupportedManifestProfile: Self = Self(5i32);
    pub const UnknownFailure: Self = Self(6i32);
}
impl ::core::marker::Copy for AdaptiveMediaSourceCreationStatus {}
impl ::core::clone::Clone for AdaptiveMediaSourceCreationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AdaptiveMediaSourceCreationStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AdaptiveMediaSourceCreationStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for AdaptiveMediaSourceCreationStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AdaptiveMediaSourceCreationStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AdaptiveMediaSourceCreationStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceCreationStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Streaming_Adaptive\"`*"]
#[repr(transparent)]
pub struct AdaptiveMediaSourceDiagnosticAvailableEventArgs(::windows::core::IUnknown);
impl AdaptiveMediaSourceDiagnosticAvailableEventArgs {
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`*"]
    pub fn DiagnosticType(&self) -> ::windows::core::Result<AdaptiveMediaSourceDiagnosticType> {
        let this = self;
        unsafe {
            let mut result__: AdaptiveMediaSourceDiagnosticType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DiagnosticType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AdaptiveMediaSourceDiagnosticType>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestId(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Position(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Position)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SegmentId(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SegmentId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<u64>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ResourceType(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<AdaptiveMediaSourceResourceType>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ResourceType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<AdaptiveMediaSourceResourceType>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ResourceUri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ResourceUri)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ResourceByteRangeOffset(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ResourceByteRangeOffset)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<u64>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ResourceByteRangeLength(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ResourceByteRangeLength)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<u64>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Bitrate(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Bitrate)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<u32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`*"]
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = &::windows::core::Interface::cast::<IAdaptiveMediaSourceDiagnosticAvailableEventArgs2>(self)?;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExtendedError)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ResourceDuration(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>> {
        let this = &::windows::core::Interface::cast::<IAdaptiveMediaSourceDiagnosticAvailableEventArgs3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ResourceDuration)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`*"]
    pub fn ResourceContentType(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAdaptiveMediaSourceDiagnosticAvailableEventArgs3>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ResourceContentType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for AdaptiveMediaSourceDiagnosticAvailableEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AdaptiveMediaSourceDiagnosticAvailableEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AdaptiveMediaSourceDiagnosticAvailableEventArgs {}
impl ::core::fmt::Debug for AdaptiveMediaSourceDiagnosticAvailableEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AdaptiveMediaSourceDiagnosticAvailableEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AdaptiveMediaSourceDiagnosticAvailableEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDiagnosticAvailableEventArgs;{3af64f06-6d9c-494a-b7a9-b3a5dee6ad68})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AdaptiveMediaSourceDiagnosticAvailableEventArgs {
    type Vtable = IAdaptiveMediaSourceDiagnosticAvailableEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IAdaptiveMediaSourceDiagnosticAvailableEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AdaptiveMediaSourceDiagnosticAvailableEventArgs {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDiagnosticAvailableEventArgs";
}
impl ::core::convert::From<AdaptiveMediaSourceDiagnosticAvailableEventArgs> for ::windows::core::IUnknown {
    fn from(value: AdaptiveMediaSourceDiagnosticAvailableEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AdaptiveMediaSourceDiagnosticAvailableEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AdaptiveMediaSourceDiagnosticAvailableEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AdaptiveMediaSourceDiagnosticAvailableEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AdaptiveMediaSourceDiagnosticAvailableEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AdaptiveMediaSourceDiagnosticAvailableEventArgs> for ::windows::core::IInspectable {
    fn from(value: AdaptiveMediaSourceDiagnosticAvailableEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AdaptiveMediaSourceDiagnosticAvailableEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AdaptiveMediaSourceDiagnosticAvailableEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AdaptiveMediaSourceDiagnosticAvailableEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AdaptiveMediaSourceDiagnosticAvailableEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AdaptiveMediaSourceDiagnosticAvailableEventArgs {}
unsafe impl ::core::marker::Sync for AdaptiveMediaSourceDiagnosticAvailableEventArgs {}
#[doc = "*Required features: `\"Media_Streaming_Adaptive\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AdaptiveMediaSourceDiagnosticType(pub i32);
impl AdaptiveMediaSourceDiagnosticType {
    pub const ManifestUnchangedUponReload: Self = Self(0i32);
    pub const ManifestMismatchUponReload: Self = Self(1i32);
    pub const ManifestSignaledEndOfLiveEventUponReload: Self = Self(2i32);
    pub const MediaSegmentSkipped: Self = Self(3i32);
    pub const ResourceNotFound: Self = Self(4i32);
    pub const ResourceTimedOut: Self = Self(5i32);
    pub const ResourceParsingError: Self = Self(6i32);
    pub const BitrateDisabled: Self = Self(7i32);
    pub const FatalMediaSourceError: Self = Self(8i32);
}
impl ::core::marker::Copy for AdaptiveMediaSourceDiagnosticType {}
impl ::core::clone::Clone for AdaptiveMediaSourceDiagnosticType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AdaptiveMediaSourceDiagnosticType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AdaptiveMediaSourceDiagnosticType {
    type Abi = Self;
}
impl ::core::fmt::Debug for AdaptiveMediaSourceDiagnosticType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AdaptiveMediaSourceDiagnosticType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AdaptiveMediaSourceDiagnosticType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDiagnosticType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Streaming_Adaptive\"`*"]
#[repr(transparent)]
pub struct AdaptiveMediaSourceDiagnostics(::windows::core::IUnknown);
impl AdaptiveMediaSourceDiagnostics {
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DiagnosticAvailable<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<AdaptiveMediaSourceDiagnostics, AdaptiveMediaSourceDiagnosticAvailableEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DiagnosticAvailable)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveDiagnosticAvailable<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveDiagnosticAvailable)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for AdaptiveMediaSourceDiagnostics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AdaptiveMediaSourceDiagnostics {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AdaptiveMediaSourceDiagnostics {}
impl ::core::fmt::Debug for AdaptiveMediaSourceDiagnostics {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AdaptiveMediaSourceDiagnostics").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AdaptiveMediaSourceDiagnostics {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDiagnostics;{9b24ee68-962e-448c-aebf-b29b56098e23})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AdaptiveMediaSourceDiagnostics {
    type Vtable = IAdaptiveMediaSourceDiagnostics_Vtbl;
    const IID: ::windows::core::GUID = <IAdaptiveMediaSourceDiagnostics as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AdaptiveMediaSourceDiagnostics {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDiagnostics";
}
impl ::core::convert::From<AdaptiveMediaSourceDiagnostics> for ::windows::core::IUnknown {
    fn from(value: AdaptiveMediaSourceDiagnostics) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AdaptiveMediaSourceDiagnostics> for ::windows::core::IUnknown {
    fn from(value: &AdaptiveMediaSourceDiagnostics) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AdaptiveMediaSourceDiagnostics {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AdaptiveMediaSourceDiagnostics {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AdaptiveMediaSourceDiagnostics> for ::windows::core::IInspectable {
    fn from(value: AdaptiveMediaSourceDiagnostics) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AdaptiveMediaSourceDiagnostics> for ::windows::core::IInspectable {
    fn from(value: &AdaptiveMediaSourceDiagnostics) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AdaptiveMediaSourceDiagnostics {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AdaptiveMediaSourceDiagnostics {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AdaptiveMediaSourceDiagnostics {}
unsafe impl ::core::marker::Sync for AdaptiveMediaSourceDiagnostics {}
#[doc = "*Required features: `\"Media_Streaming_Adaptive\"`*"]
#[repr(transparent)]
pub struct AdaptiveMediaSourceDownloadBitrateChangedEventArgs(::windows::core::IUnknown);
impl AdaptiveMediaSourceDownloadBitrateChangedEventArgs {
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`*"]
    pub fn OldValue(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OldValue)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`*"]
    pub fn NewValue(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NewValue)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`*"]
    pub fn Reason(&self) -> ::windows::core::Result<AdaptiveMediaSourceDownloadBitrateChangedReason> {
        let this = &::windows::core::Interface::cast::<IAdaptiveMediaSourceDownloadBitrateChangedEventArgs2>(self)?;
        unsafe {
            let mut result__: AdaptiveMediaSourceDownloadBitrateChangedReason = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Reason)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AdaptiveMediaSourceDownloadBitrateChangedReason>(result__)
        }
    }
}
impl ::core::clone::Clone for AdaptiveMediaSourceDownloadBitrateChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AdaptiveMediaSourceDownloadBitrateChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AdaptiveMediaSourceDownloadBitrateChangedEventArgs {}
impl ::core::fmt::Debug for AdaptiveMediaSourceDownloadBitrateChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AdaptiveMediaSourceDownloadBitrateChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AdaptiveMediaSourceDownloadBitrateChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadBitrateChangedEventArgs;{670c0a44-e04e-4eff-816a-17399f78f4ba})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AdaptiveMediaSourceDownloadBitrateChangedEventArgs {
    type Vtable = IAdaptiveMediaSourceDownloadBitrateChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IAdaptiveMediaSourceDownloadBitrateChangedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AdaptiveMediaSourceDownloadBitrateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadBitrateChangedEventArgs";
}
impl ::core::convert::From<AdaptiveMediaSourceDownloadBitrateChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: AdaptiveMediaSourceDownloadBitrateChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AdaptiveMediaSourceDownloadBitrateChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AdaptiveMediaSourceDownloadBitrateChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AdaptiveMediaSourceDownloadBitrateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AdaptiveMediaSourceDownloadBitrateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AdaptiveMediaSourceDownloadBitrateChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: AdaptiveMediaSourceDownloadBitrateChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AdaptiveMediaSourceDownloadBitrateChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AdaptiveMediaSourceDownloadBitrateChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AdaptiveMediaSourceDownloadBitrateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AdaptiveMediaSourceDownloadBitrateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AdaptiveMediaSourceDownloadBitrateChangedEventArgs {}
unsafe impl ::core::marker::Sync for AdaptiveMediaSourceDownloadBitrateChangedEventArgs {}
#[doc = "*Required features: `\"Media_Streaming_Adaptive\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AdaptiveMediaSourceDownloadBitrateChangedReason(pub i32);
impl AdaptiveMediaSourceDownloadBitrateChangedReason {
    pub const SufficientInboundBitsPerSecond: Self = Self(0i32);
    pub const InsufficientInboundBitsPerSecond: Self = Self(1i32);
    pub const LowBufferLevel: Self = Self(2i32);
    pub const PositionChanged: Self = Self(3i32);
    pub const TrackSelectionChanged: Self = Self(4i32);
    pub const DesiredBitratesChanged: Self = Self(5i32);
    pub const ErrorInPreviousBitrate: Self = Self(6i32);
}
impl ::core::marker::Copy for AdaptiveMediaSourceDownloadBitrateChangedReason {}
impl ::core::clone::Clone for AdaptiveMediaSourceDownloadBitrateChangedReason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AdaptiveMediaSourceDownloadBitrateChangedReason {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AdaptiveMediaSourceDownloadBitrateChangedReason {
    type Abi = Self;
}
impl ::core::fmt::Debug for AdaptiveMediaSourceDownloadBitrateChangedReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AdaptiveMediaSourceDownloadBitrateChangedReason").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AdaptiveMediaSourceDownloadBitrateChangedReason {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadBitrateChangedReason;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Streaming_Adaptive\"`*"]
#[repr(transparent)]
pub struct AdaptiveMediaSourceDownloadCompletedEventArgs(::windows::core::IUnknown);
impl AdaptiveMediaSourceDownloadCompletedEventArgs {
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`*"]
    pub fn ResourceType(&self) -> ::windows::core::Result<AdaptiveMediaSourceResourceType> {
        let this = self;
        unsafe {
            let mut result__: AdaptiveMediaSourceResourceType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ResourceType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AdaptiveMediaSourceResourceType>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ResourceUri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ResourceUri)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ResourceByteRangeOffset(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ResourceByteRangeOffset)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<u64>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ResourceByteRangeLength(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ResourceByteRangeLength)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<u64>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`, `\"Web_Http\"`*"]
    #[cfg(feature = "Web_Http")]
    pub fn HttpResponseMessage(&self) -> ::windows::core::Result<super::super::super::Web::Http::HttpResponseMessage> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HttpResponseMessage)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Web::Http::HttpResponseMessage>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`*"]
    pub fn RequestId(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<IAdaptiveMediaSourceDownloadCompletedEventArgs2>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`*"]
    pub fn Statistics(&self) -> ::windows::core::Result<AdaptiveMediaSourceDownloadStatistics> {
        let this = &::windows::core::Interface::cast::<IAdaptiveMediaSourceDownloadCompletedEventArgs2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Statistics)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AdaptiveMediaSourceDownloadStatistics>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Position(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>> {
        let this = &::windows::core::Interface::cast::<IAdaptiveMediaSourceDownloadCompletedEventArgs2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Position)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ResourceDuration(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>> {
        let this = &::windows::core::Interface::cast::<IAdaptiveMediaSourceDownloadCompletedEventArgs3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ResourceDuration)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`*"]
    pub fn ResourceContentType(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAdaptiveMediaSourceDownloadCompletedEventArgs3>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ResourceContentType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for AdaptiveMediaSourceDownloadCompletedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AdaptiveMediaSourceDownloadCompletedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AdaptiveMediaSourceDownloadCompletedEventArgs {}
impl ::core::fmt::Debug for AdaptiveMediaSourceDownloadCompletedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AdaptiveMediaSourceDownloadCompletedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AdaptiveMediaSourceDownloadCompletedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadCompletedEventArgs;{19240dc3-5b37-4a1a-8970-d621cb6ca83b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AdaptiveMediaSourceDownloadCompletedEventArgs {
    type Vtable = IAdaptiveMediaSourceDownloadCompletedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IAdaptiveMediaSourceDownloadCompletedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AdaptiveMediaSourceDownloadCompletedEventArgs {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadCompletedEventArgs";
}
impl ::core::convert::From<AdaptiveMediaSourceDownloadCompletedEventArgs> for ::windows::core::IUnknown {
    fn from(value: AdaptiveMediaSourceDownloadCompletedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AdaptiveMediaSourceDownloadCompletedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AdaptiveMediaSourceDownloadCompletedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AdaptiveMediaSourceDownloadCompletedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AdaptiveMediaSourceDownloadCompletedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AdaptiveMediaSourceDownloadCompletedEventArgs> for ::windows::core::IInspectable {
    fn from(value: AdaptiveMediaSourceDownloadCompletedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AdaptiveMediaSourceDownloadCompletedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AdaptiveMediaSourceDownloadCompletedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AdaptiveMediaSourceDownloadCompletedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AdaptiveMediaSourceDownloadCompletedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AdaptiveMediaSourceDownloadCompletedEventArgs {}
unsafe impl ::core::marker::Sync for AdaptiveMediaSourceDownloadCompletedEventArgs {}
#[doc = "*Required features: `\"Media_Streaming_Adaptive\"`*"]
#[repr(transparent)]
pub struct AdaptiveMediaSourceDownloadFailedEventArgs(::windows::core::IUnknown);
impl AdaptiveMediaSourceDownloadFailedEventArgs {
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`*"]
    pub fn ResourceType(&self) -> ::windows::core::Result<AdaptiveMediaSourceResourceType> {
        let this = self;
        unsafe {
            let mut result__: AdaptiveMediaSourceResourceType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ResourceType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AdaptiveMediaSourceResourceType>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ResourceUri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ResourceUri)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ResourceByteRangeOffset(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ResourceByteRangeOffset)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<u64>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ResourceByteRangeLength(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ResourceByteRangeLength)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<u64>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`, `\"Web_Http\"`*"]
    #[cfg(feature = "Web_Http")]
    pub fn HttpResponseMessage(&self) -> ::windows::core::Result<super::super::super::Web::Http::HttpResponseMessage> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HttpResponseMessage)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Web::Http::HttpResponseMessage>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`*"]
    pub fn RequestId(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<IAdaptiveMediaSourceDownloadFailedEventArgs2>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`*"]
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = &::windows::core::Interface::cast::<IAdaptiveMediaSourceDownloadFailedEventArgs2>(self)?;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExtendedError)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`*"]
    pub fn Statistics(&self) -> ::windows::core::Result<AdaptiveMediaSourceDownloadStatistics> {
        let this = &::windows::core::Interface::cast::<IAdaptiveMediaSourceDownloadFailedEventArgs2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Statistics)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AdaptiveMediaSourceDownloadStatistics>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Position(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>> {
        let this = &::windows::core::Interface::cast::<IAdaptiveMediaSourceDownloadFailedEventArgs2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Position)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ResourceDuration(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>> {
        let this = &::windows::core::Interface::cast::<IAdaptiveMediaSourceDownloadFailedEventArgs3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ResourceDuration)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`*"]
    pub fn ResourceContentType(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAdaptiveMediaSourceDownloadFailedEventArgs3>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ResourceContentType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for AdaptiveMediaSourceDownloadFailedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AdaptiveMediaSourceDownloadFailedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AdaptiveMediaSourceDownloadFailedEventArgs {}
impl ::core::fmt::Debug for AdaptiveMediaSourceDownloadFailedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AdaptiveMediaSourceDownloadFailedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AdaptiveMediaSourceDownloadFailedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadFailedEventArgs;{37739048-f4ab-40a4-b135-c6dfd8bd7ff1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AdaptiveMediaSourceDownloadFailedEventArgs {
    type Vtable = IAdaptiveMediaSourceDownloadFailedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IAdaptiveMediaSourceDownloadFailedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AdaptiveMediaSourceDownloadFailedEventArgs {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadFailedEventArgs";
}
impl ::core::convert::From<AdaptiveMediaSourceDownloadFailedEventArgs> for ::windows::core::IUnknown {
    fn from(value: AdaptiveMediaSourceDownloadFailedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AdaptiveMediaSourceDownloadFailedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AdaptiveMediaSourceDownloadFailedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AdaptiveMediaSourceDownloadFailedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AdaptiveMediaSourceDownloadFailedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AdaptiveMediaSourceDownloadFailedEventArgs> for ::windows::core::IInspectable {
    fn from(value: AdaptiveMediaSourceDownloadFailedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AdaptiveMediaSourceDownloadFailedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AdaptiveMediaSourceDownloadFailedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AdaptiveMediaSourceDownloadFailedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AdaptiveMediaSourceDownloadFailedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AdaptiveMediaSourceDownloadFailedEventArgs {}
unsafe impl ::core::marker::Sync for AdaptiveMediaSourceDownloadFailedEventArgs {}
#[doc = "*Required features: `\"Media_Streaming_Adaptive\"`*"]
#[repr(transparent)]
pub struct AdaptiveMediaSourceDownloadRequestedDeferral(::windows::core::IUnknown);
impl AdaptiveMediaSourceDownloadRequestedDeferral {
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`*"]
    pub fn Complete(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Complete)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::clone::Clone for AdaptiveMediaSourceDownloadRequestedDeferral {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AdaptiveMediaSourceDownloadRequestedDeferral {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AdaptiveMediaSourceDownloadRequestedDeferral {}
impl ::core::fmt::Debug for AdaptiveMediaSourceDownloadRequestedDeferral {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AdaptiveMediaSourceDownloadRequestedDeferral").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AdaptiveMediaSourceDownloadRequestedDeferral {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadRequestedDeferral;{05c68f64-fa20-4dbd-9821-4bf4c9bf77ab})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AdaptiveMediaSourceDownloadRequestedDeferral {
    type Vtable = IAdaptiveMediaSourceDownloadRequestedDeferral_Vtbl;
    const IID: ::windows::core::GUID = <IAdaptiveMediaSourceDownloadRequestedDeferral as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AdaptiveMediaSourceDownloadRequestedDeferral {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadRequestedDeferral";
}
impl ::core::convert::From<AdaptiveMediaSourceDownloadRequestedDeferral> for ::windows::core::IUnknown {
    fn from(value: AdaptiveMediaSourceDownloadRequestedDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AdaptiveMediaSourceDownloadRequestedDeferral> for ::windows::core::IUnknown {
    fn from(value: &AdaptiveMediaSourceDownloadRequestedDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AdaptiveMediaSourceDownloadRequestedDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AdaptiveMediaSourceDownloadRequestedDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AdaptiveMediaSourceDownloadRequestedDeferral> for ::windows::core::IInspectable {
    fn from(value: AdaptiveMediaSourceDownloadRequestedDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AdaptiveMediaSourceDownloadRequestedDeferral> for ::windows::core::IInspectable {
    fn from(value: &AdaptiveMediaSourceDownloadRequestedDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AdaptiveMediaSourceDownloadRequestedDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AdaptiveMediaSourceDownloadRequestedDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AdaptiveMediaSourceDownloadRequestedDeferral {}
unsafe impl ::core::marker::Sync for AdaptiveMediaSourceDownloadRequestedDeferral {}
#[doc = "*Required features: `\"Media_Streaming_Adaptive\"`*"]
#[repr(transparent)]
pub struct AdaptiveMediaSourceDownloadRequestedEventArgs(::windows::core::IUnknown);
impl AdaptiveMediaSourceDownloadRequestedEventArgs {
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`*"]
    pub fn ResourceType(&self) -> ::windows::core::Result<AdaptiveMediaSourceResourceType> {
        let this = self;
        unsafe {
            let mut result__: AdaptiveMediaSourceResourceType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ResourceType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AdaptiveMediaSourceResourceType>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ResourceUri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ResourceUri)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ResourceByteRangeOffset(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ResourceByteRangeOffset)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<u64>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ResourceByteRangeLength(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ResourceByteRangeLength)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<u64>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`*"]
    pub fn Result(&self) -> ::windows::core::Result<AdaptiveMediaSourceDownloadResult> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Result)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AdaptiveMediaSourceDownloadResult>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`*"]
    pub fn GetDeferral(&self) -> ::windows::core::Result<AdaptiveMediaSourceDownloadRequestedDeferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeferral)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AdaptiveMediaSourceDownloadRequestedDeferral>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`*"]
    pub fn RequestId(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<IAdaptiveMediaSourceDownloadRequestedEventArgs2>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Position(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>> {
        let this = &::windows::core::Interface::cast::<IAdaptiveMediaSourceDownloadRequestedEventArgs2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Position)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ResourceDuration(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>> {
        let this = &::windows::core::Interface::cast::<IAdaptiveMediaSourceDownloadRequestedEventArgs3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ResourceDuration)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`*"]
    pub fn ResourceContentType(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAdaptiveMediaSourceDownloadRequestedEventArgs3>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ResourceContentType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for AdaptiveMediaSourceDownloadRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AdaptiveMediaSourceDownloadRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AdaptiveMediaSourceDownloadRequestedEventArgs {}
impl ::core::fmt::Debug for AdaptiveMediaSourceDownloadRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AdaptiveMediaSourceDownloadRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AdaptiveMediaSourceDownloadRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadRequestedEventArgs;{c83fdffd-44a9-47a2-bf96-03398b4bfaaf})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AdaptiveMediaSourceDownloadRequestedEventArgs {
    type Vtable = IAdaptiveMediaSourceDownloadRequestedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IAdaptiveMediaSourceDownloadRequestedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AdaptiveMediaSourceDownloadRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadRequestedEventArgs";
}
impl ::core::convert::From<AdaptiveMediaSourceDownloadRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: AdaptiveMediaSourceDownloadRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AdaptiveMediaSourceDownloadRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AdaptiveMediaSourceDownloadRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AdaptiveMediaSourceDownloadRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AdaptiveMediaSourceDownloadRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AdaptiveMediaSourceDownloadRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: AdaptiveMediaSourceDownloadRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AdaptiveMediaSourceDownloadRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AdaptiveMediaSourceDownloadRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AdaptiveMediaSourceDownloadRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AdaptiveMediaSourceDownloadRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AdaptiveMediaSourceDownloadRequestedEventArgs {}
unsafe impl ::core::marker::Sync for AdaptiveMediaSourceDownloadRequestedEventArgs {}
#[doc = "*Required features: `\"Media_Streaming_Adaptive\"`*"]
#[repr(transparent)]
pub struct AdaptiveMediaSourceDownloadResult(::windows::core::IUnknown);
impl AdaptiveMediaSourceDownloadResult {
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ResourceUri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ResourceUri)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetResourceUri<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetResourceUri)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn InputStream(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IInputStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).InputStream)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Storage::Streams::IInputStream>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetInputStream<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IInputStream>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetInputStream)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Buffer(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Buffer)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetBuffer<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetBuffer)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`*"]
    pub fn ContentType(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ContentType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`*"]
    pub fn SetContentType<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetContentType)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`*"]
    pub fn ExtendedStatus(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExtendedStatus)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`*"]
    pub fn SetExtendedStatus(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetExtendedStatus)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ResourceByteRangeOffset(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u64>> {
        let this = &::windows::core::Interface::cast::<IAdaptiveMediaSourceDownloadResult2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ResourceByteRangeOffset)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<u64>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetResourceByteRangeOffset<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::IReference<u64>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAdaptiveMediaSourceDownloadResult2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetResourceByteRangeOffset)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ResourceByteRangeLength(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u64>> {
        let this = &::windows::core::Interface::cast::<IAdaptiveMediaSourceDownloadResult2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ResourceByteRangeLength)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<u64>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetResourceByteRangeLength<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::IReference<u64>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAdaptiveMediaSourceDownloadResult2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetResourceByteRangeLength)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for AdaptiveMediaSourceDownloadResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AdaptiveMediaSourceDownloadResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AdaptiveMediaSourceDownloadResult {}
impl ::core::fmt::Debug for AdaptiveMediaSourceDownloadResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AdaptiveMediaSourceDownloadResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AdaptiveMediaSourceDownloadResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadResult;{f4afdc73-bcee-4a6a-9f0a-fec41e2339b0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AdaptiveMediaSourceDownloadResult {
    type Vtable = IAdaptiveMediaSourceDownloadResult_Vtbl;
    const IID: ::windows::core::GUID = <IAdaptiveMediaSourceDownloadResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AdaptiveMediaSourceDownloadResult {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadResult";
}
impl ::core::convert::From<AdaptiveMediaSourceDownloadResult> for ::windows::core::IUnknown {
    fn from(value: AdaptiveMediaSourceDownloadResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AdaptiveMediaSourceDownloadResult> for ::windows::core::IUnknown {
    fn from(value: &AdaptiveMediaSourceDownloadResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AdaptiveMediaSourceDownloadResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AdaptiveMediaSourceDownloadResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AdaptiveMediaSourceDownloadResult> for ::windows::core::IInspectable {
    fn from(value: AdaptiveMediaSourceDownloadResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AdaptiveMediaSourceDownloadResult> for ::windows::core::IInspectable {
    fn from(value: &AdaptiveMediaSourceDownloadResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AdaptiveMediaSourceDownloadResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AdaptiveMediaSourceDownloadResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AdaptiveMediaSourceDownloadResult {}
unsafe impl ::core::marker::Sync for AdaptiveMediaSourceDownloadResult {}
#[doc = "*Required features: `\"Media_Streaming_Adaptive\"`*"]
#[repr(transparent)]
pub struct AdaptiveMediaSourceDownloadStatistics(::windows::core::IUnknown);
impl AdaptiveMediaSourceDownloadStatistics {
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`*"]
    pub fn ContentBytesReceivedCount(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ContentBytesReceivedCount)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TimeToHeadersReceived(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TimeToHeadersReceived)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TimeToFirstByteReceived(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TimeToFirstByteReceived)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TimeToLastByteReceived(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TimeToLastByteReceived)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>(result__)
        }
    }
}
impl ::core::clone::Clone for AdaptiveMediaSourceDownloadStatistics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AdaptiveMediaSourceDownloadStatistics {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AdaptiveMediaSourceDownloadStatistics {}
impl ::core::fmt::Debug for AdaptiveMediaSourceDownloadStatistics {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AdaptiveMediaSourceDownloadStatistics").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AdaptiveMediaSourceDownloadStatistics {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadStatistics;{a306cefb-e96a-4dff-a9b8-1ae08c01ae98})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AdaptiveMediaSourceDownloadStatistics {
    type Vtable = IAdaptiveMediaSourceDownloadStatistics_Vtbl;
    const IID: ::windows::core::GUID = <IAdaptiveMediaSourceDownloadStatistics as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AdaptiveMediaSourceDownloadStatistics {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadStatistics";
}
impl ::core::convert::From<AdaptiveMediaSourceDownloadStatistics> for ::windows::core::IUnknown {
    fn from(value: AdaptiveMediaSourceDownloadStatistics) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AdaptiveMediaSourceDownloadStatistics> for ::windows::core::IUnknown {
    fn from(value: &AdaptiveMediaSourceDownloadStatistics) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AdaptiveMediaSourceDownloadStatistics {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AdaptiveMediaSourceDownloadStatistics {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AdaptiveMediaSourceDownloadStatistics> for ::windows::core::IInspectable {
    fn from(value: AdaptiveMediaSourceDownloadStatistics) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AdaptiveMediaSourceDownloadStatistics> for ::windows::core::IInspectable {
    fn from(value: &AdaptiveMediaSourceDownloadStatistics) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AdaptiveMediaSourceDownloadStatistics {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AdaptiveMediaSourceDownloadStatistics {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AdaptiveMediaSourceDownloadStatistics {}
unsafe impl ::core::marker::Sync for AdaptiveMediaSourceDownloadStatistics {}
#[doc = "*Required features: `\"Media_Streaming_Adaptive\"`*"]
#[repr(transparent)]
pub struct AdaptiveMediaSourcePlaybackBitrateChangedEventArgs(::windows::core::IUnknown);
impl AdaptiveMediaSourcePlaybackBitrateChangedEventArgs {
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`*"]
    pub fn OldValue(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OldValue)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`*"]
    pub fn NewValue(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NewValue)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`*"]
    pub fn AudioOnly(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AudioOnly)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for AdaptiveMediaSourcePlaybackBitrateChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AdaptiveMediaSourcePlaybackBitrateChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AdaptiveMediaSourcePlaybackBitrateChangedEventArgs {}
impl ::core::fmt::Debug for AdaptiveMediaSourcePlaybackBitrateChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AdaptiveMediaSourcePlaybackBitrateChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AdaptiveMediaSourcePlaybackBitrateChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Streaming.Adaptive.AdaptiveMediaSourcePlaybackBitrateChangedEventArgs;{23a29f6d-7dda-4a51-87a9-6fa8c5b292be})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AdaptiveMediaSourcePlaybackBitrateChangedEventArgs {
    type Vtable = IAdaptiveMediaSourcePlaybackBitrateChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IAdaptiveMediaSourcePlaybackBitrateChangedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AdaptiveMediaSourcePlaybackBitrateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.AdaptiveMediaSourcePlaybackBitrateChangedEventArgs";
}
impl ::core::convert::From<AdaptiveMediaSourcePlaybackBitrateChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: AdaptiveMediaSourcePlaybackBitrateChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AdaptiveMediaSourcePlaybackBitrateChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AdaptiveMediaSourcePlaybackBitrateChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AdaptiveMediaSourcePlaybackBitrateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AdaptiveMediaSourcePlaybackBitrateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AdaptiveMediaSourcePlaybackBitrateChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: AdaptiveMediaSourcePlaybackBitrateChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AdaptiveMediaSourcePlaybackBitrateChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AdaptiveMediaSourcePlaybackBitrateChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AdaptiveMediaSourcePlaybackBitrateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AdaptiveMediaSourcePlaybackBitrateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AdaptiveMediaSourcePlaybackBitrateChangedEventArgs {}
unsafe impl ::core::marker::Sync for AdaptiveMediaSourcePlaybackBitrateChangedEventArgs {}
#[doc = "*Required features: `\"Media_Streaming_Adaptive\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AdaptiveMediaSourceResourceType(pub i32);
impl AdaptiveMediaSourceResourceType {
    pub const Manifest: Self = Self(0i32);
    pub const InitializationSegment: Self = Self(1i32);
    pub const MediaSegment: Self = Self(2i32);
    pub const Key: Self = Self(3i32);
    pub const InitializationVector: Self = Self(4i32);
    pub const MediaSegmentIndex: Self = Self(5i32);
}
impl ::core::marker::Copy for AdaptiveMediaSourceResourceType {}
impl ::core::clone::Clone for AdaptiveMediaSourceResourceType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AdaptiveMediaSourceResourceType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AdaptiveMediaSourceResourceType {
    type Abi = Self;
}
impl ::core::fmt::Debug for AdaptiveMediaSourceResourceType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AdaptiveMediaSourceResourceType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AdaptiveMediaSourceResourceType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceResourceType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdaptiveMediaSource(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAdaptiveMediaSource {
    type Vtable = IAdaptiveMediaSource_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4c7332ef_d39f_4396_b4d9_043957a7c964);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveMediaSource_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsLive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DesiredLiveOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DesiredLiveOffset: usize,
    #[cfg(feature = "Foundation")]
    pub SetDesiredLiveOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDesiredLiveOffset: usize,
    pub InitialBitrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetInitialBitrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub CurrentDownloadBitrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub CurrentPlaybackBitrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub AvailableBitrates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AvailableBitrates: usize,
    #[cfg(feature = "Foundation")]
    pub DesiredMinBitrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DesiredMinBitrate: usize,
    #[cfg(feature = "Foundation")]
    pub SetDesiredMinBitrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDesiredMinBitrate: usize,
    #[cfg(feature = "Foundation")]
    pub DesiredMaxBitrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DesiredMaxBitrate: usize,
    #[cfg(feature = "Foundation")]
    pub SetDesiredMaxBitrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDesiredMaxBitrate: usize,
    pub AudioOnlyPlayback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub InboundBitsPerSecond: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub InboundBitsPerSecondWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InboundBitsPerSecondWindow: usize,
    #[cfg(feature = "Foundation")]
    pub SetInboundBitsPerSecondWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetInboundBitsPerSecondWindow: usize,
    #[cfg(feature = "Foundation")]
    pub DownloadBitrateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DownloadBitrateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDownloadBitrateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDownloadBitrateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub PlaybackBitrateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PlaybackBitrateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePlaybackBitrateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePlaybackBitrateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub DownloadRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DownloadRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDownloadRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDownloadRequested: usize,
    #[cfg(feature = "Foundation")]
    pub DownloadCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DownloadCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDownloadCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDownloadCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub DownloadFailed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DownloadFailed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDownloadFailed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDownloadFailed: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdaptiveMediaSource2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAdaptiveMediaSource2 {
    type Vtable = IAdaptiveMediaSource2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x17890342_6760_4bb9_a58a_f7aa98b08c0e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveMediaSource2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub AdvancedSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdaptiveMediaSource3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAdaptiveMediaSource3 {
    type Vtable = IAdaptiveMediaSource3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xba7023fd_c334_461b_a36e_c99f54f7174a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveMediaSource3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub MinLiveOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MinLiveOffset: usize,
    #[cfg(feature = "Foundation")]
    pub MaxSeekableWindowSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxSeekableWindowSize: usize,
    #[cfg(feature = "Foundation")]
    pub DesiredSeekableWindowSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DesiredSeekableWindowSize: usize,
    #[cfg(feature = "Foundation")]
    pub SetDesiredSeekableWindowSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDesiredSeekableWindowSize: usize,
    pub Diagnostics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetCorrelatedTimes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdaptiveMediaSourceAdvancedSettings(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAdaptiveMediaSourceAdvancedSettings {
    type Vtable = IAdaptiveMediaSourceAdvancedSettings_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x55db1680_1aeb_47dc_aa08_9a11610ba45a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceAdvancedSettings_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub AllSegmentsIndependent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetAllSegmentsIndependent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DesiredBitrateHeadroomRatio: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DesiredBitrateHeadroomRatio: usize,
    #[cfg(feature = "Foundation")]
    pub SetDesiredBitrateHeadroomRatio: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDesiredBitrateHeadroomRatio: usize,
    #[cfg(feature = "Foundation")]
    pub BitrateDowngradeTriggerRatio: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BitrateDowngradeTriggerRatio: usize,
    #[cfg(feature = "Foundation")]
    pub SetBitrateDowngradeTriggerRatio: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetBitrateDowngradeTriggerRatio: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdaptiveMediaSourceCorrelatedTimes(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAdaptiveMediaSourceCorrelatedTimes {
    type Vtable = IAdaptiveMediaSourceCorrelatedTimes_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x05108787_e032_48e1_ab8d_002b0b3051df);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceCorrelatedTimes_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Position: usize,
    #[cfg(feature = "Foundation")]
    pub PresentationTimeStamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PresentationTimeStamp: usize,
    #[cfg(feature = "Foundation")]
    pub ProgramDateTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ProgramDateTime: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdaptiveMediaSourceCreationResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAdaptiveMediaSourceCreationResult {
    type Vtable = IAdaptiveMediaSourceCreationResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4686b6b2_800f_4e31_9093_76d4782013e7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceCreationResult_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AdaptiveMediaSourceCreationStatus) -> ::windows::core::HRESULT,
    pub MediaSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Web_Http")]
    pub HttpResponseMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Web_Http"))]
    HttpResponseMessage: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdaptiveMediaSourceCreationResult2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAdaptiveMediaSourceCreationResult2 {
    type Vtable = IAdaptiveMediaSourceCreationResult2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1c3243bf_1c44_404b_a201_df45ac7898e8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceCreationResult2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdaptiveMediaSourceDiagnosticAvailableEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAdaptiveMediaSourceDiagnosticAvailableEventArgs {
    type Vtable = IAdaptiveMediaSourceDiagnosticAvailableEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3af64f06_6d9c_494a_b7a9_b3a5dee6ad68);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceDiagnosticAvailableEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub DiagnosticType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AdaptiveMediaSourceDiagnosticType) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RequestId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestId: usize,
    #[cfg(feature = "Foundation")]
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Position: usize,
    #[cfg(feature = "Foundation")]
    pub SegmentId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SegmentId: usize,
    #[cfg(feature = "Foundation")]
    pub ResourceType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ResourceType: usize,
    #[cfg(feature = "Foundation")]
    pub ResourceUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ResourceUri: usize,
    #[cfg(feature = "Foundation")]
    pub ResourceByteRangeOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ResourceByteRangeOffset: usize,
    #[cfg(feature = "Foundation")]
    pub ResourceByteRangeLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ResourceByteRangeLength: usize,
    #[cfg(feature = "Foundation")]
    pub Bitrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Bitrate: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdaptiveMediaSourceDiagnosticAvailableEventArgs2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAdaptiveMediaSourceDiagnosticAvailableEventArgs2 {
    type Vtable = IAdaptiveMediaSourceDiagnosticAvailableEventArgs2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8c6dd857_16a5_4d9f_810e_00bd901b3ef9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceDiagnosticAvailableEventArgs2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdaptiveMediaSourceDiagnosticAvailableEventArgs3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAdaptiveMediaSourceDiagnosticAvailableEventArgs3 {
    type Vtable = IAdaptiveMediaSourceDiagnosticAvailableEventArgs3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc3650cd5_daeb_4103_84da_68769ad513ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceDiagnosticAvailableEventArgs3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub ResourceDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ResourceDuration: usize,
    pub ResourceContentType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdaptiveMediaSourceDiagnostics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAdaptiveMediaSourceDiagnostics {
    type Vtable = IAdaptiveMediaSourceDiagnostics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b24ee68_962e_448c_aebf_b29b56098e23);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceDiagnostics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub DiagnosticAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DiagnosticAvailable: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDiagnosticAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDiagnosticAvailable: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdaptiveMediaSourceDownloadBitrateChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAdaptiveMediaSourceDownloadBitrateChangedEventArgs {
    type Vtable = IAdaptiveMediaSourceDownloadBitrateChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x670c0a44_e04e_4eff_816a_17399f78f4ba);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceDownloadBitrateChangedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub OldValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub NewValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdaptiveMediaSourceDownloadBitrateChangedEventArgs2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAdaptiveMediaSourceDownloadBitrateChangedEventArgs2 {
    type Vtable = IAdaptiveMediaSourceDownloadBitrateChangedEventArgs2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf3f1f444_96ae_4de0_b540_2b3246e6968c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceDownloadBitrateChangedEventArgs2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Reason: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AdaptiveMediaSourceDownloadBitrateChangedReason) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdaptiveMediaSourceDownloadCompletedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAdaptiveMediaSourceDownloadCompletedEventArgs {
    type Vtable = IAdaptiveMediaSourceDownloadCompletedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x19240dc3_5b37_4a1a_8970_d621cb6ca83b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceDownloadCompletedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ResourceType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AdaptiveMediaSourceResourceType) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ResourceUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ResourceUri: usize,
    #[cfg(feature = "Foundation")]
    pub ResourceByteRangeOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ResourceByteRangeOffset: usize,
    #[cfg(feature = "Foundation")]
    pub ResourceByteRangeLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ResourceByteRangeLength: usize,
    #[cfg(feature = "Web_Http")]
    pub HttpResponseMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Web_Http"))]
    HttpResponseMessage: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdaptiveMediaSourceDownloadCompletedEventArgs2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAdaptiveMediaSourceDownloadCompletedEventArgs2 {
    type Vtable = IAdaptiveMediaSourceDownloadCompletedEventArgs2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x704744c4_964a_40e4_af95_9177dd6dfa00);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceDownloadCompletedEventArgs2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub RequestId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub Statistics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Position: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdaptiveMediaSourceDownloadCompletedEventArgs3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAdaptiveMediaSourceDownloadCompletedEventArgs3 {
    type Vtable = IAdaptiveMediaSourceDownloadCompletedEventArgs3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0f8a8bd1_93b2_47c6_badc_8be2c8f7f6e8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceDownloadCompletedEventArgs3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub ResourceDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ResourceDuration: usize,
    pub ResourceContentType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdaptiveMediaSourceDownloadFailedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAdaptiveMediaSourceDownloadFailedEventArgs {
    type Vtable = IAdaptiveMediaSourceDownloadFailedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x37739048_f4ab_40a4_b135_c6dfd8bd7ff1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceDownloadFailedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ResourceType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AdaptiveMediaSourceResourceType) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ResourceUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ResourceUri: usize,
    #[cfg(feature = "Foundation")]
    pub ResourceByteRangeOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ResourceByteRangeOffset: usize,
    #[cfg(feature = "Foundation")]
    pub ResourceByteRangeLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ResourceByteRangeLength: usize,
    #[cfg(feature = "Web_Http")]
    pub HttpResponseMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Web_Http"))]
    HttpResponseMessage: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdaptiveMediaSourceDownloadFailedEventArgs2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAdaptiveMediaSourceDownloadFailedEventArgs2 {
    type Vtable = IAdaptiveMediaSourceDownloadFailedEventArgs2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x70919568_967c_4986_90c5_c6fc4b31e2d8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceDownloadFailedEventArgs2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub RequestId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub Statistics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Position: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdaptiveMediaSourceDownloadFailedEventArgs3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAdaptiveMediaSourceDownloadFailedEventArgs3 {
    type Vtable = IAdaptiveMediaSourceDownloadFailedEventArgs3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd0354549_1132_4a10_915a_c2211b5b9409);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceDownloadFailedEventArgs3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub ResourceDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ResourceDuration: usize,
    pub ResourceContentType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdaptiveMediaSourceDownloadRequestedDeferral(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAdaptiveMediaSourceDownloadRequestedDeferral {
    type Vtable = IAdaptiveMediaSourceDownloadRequestedDeferral_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x05c68f64_fa20_4dbd_9821_4bf4c9bf77ab);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceDownloadRequestedDeferral_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Complete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdaptiveMediaSourceDownloadRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAdaptiveMediaSourceDownloadRequestedEventArgs {
    type Vtable = IAdaptiveMediaSourceDownloadRequestedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc83fdffd_44a9_47a2_bf96_03398b4bfaaf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceDownloadRequestedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ResourceType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AdaptiveMediaSourceResourceType) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ResourceUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ResourceUri: usize,
    #[cfg(feature = "Foundation")]
    pub ResourceByteRangeOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ResourceByteRangeOffset: usize,
    #[cfg(feature = "Foundation")]
    pub ResourceByteRangeLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ResourceByteRangeLength: usize,
    pub Result: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdaptiveMediaSourceDownloadRequestedEventArgs2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAdaptiveMediaSourceDownloadRequestedEventArgs2 {
    type Vtable = IAdaptiveMediaSourceDownloadRequestedEventArgs2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb37d8bfe_aa44_4d82_825b_611de3bcfecb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceDownloadRequestedEventArgs2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub RequestId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Position: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdaptiveMediaSourceDownloadRequestedEventArgs3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAdaptiveMediaSourceDownloadRequestedEventArgs3 {
    type Vtable = IAdaptiveMediaSourceDownloadRequestedEventArgs3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x333c50fd_4f62_4481_ab44_1e47b0574225);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceDownloadRequestedEventArgs3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub ResourceDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ResourceDuration: usize,
    pub ResourceContentType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdaptiveMediaSourceDownloadResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAdaptiveMediaSourceDownloadResult {
    type Vtable = IAdaptiveMediaSourceDownloadResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf4afdc73_bcee_4a6a_9f0a_fec41e2339b0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceDownloadResult_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub ResourceUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ResourceUri: usize,
    #[cfg(feature = "Foundation")]
    pub SetResourceUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetResourceUri: usize,
    #[cfg(feature = "Storage_Streams")]
    pub InputStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    InputStream: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetInputStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetInputStream: usize,
    #[cfg(feature = "Storage_Streams")]
    pub Buffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Buffer: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetBuffer: usize,
    pub ContentType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetContentType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ExtendedStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetExtendedStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdaptiveMediaSourceDownloadResult2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAdaptiveMediaSourceDownloadResult2 {
    type Vtable = IAdaptiveMediaSourceDownloadResult2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x15552cb7_7b80_4ac4_8660_a4b97f7c70f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceDownloadResult2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub ResourceByteRangeOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ResourceByteRangeOffset: usize,
    #[cfg(feature = "Foundation")]
    pub SetResourceByteRangeOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetResourceByteRangeOffset: usize,
    #[cfg(feature = "Foundation")]
    pub ResourceByteRangeLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ResourceByteRangeLength: usize,
    #[cfg(feature = "Foundation")]
    pub SetResourceByteRangeLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetResourceByteRangeLength: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdaptiveMediaSourceDownloadStatistics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAdaptiveMediaSourceDownloadStatistics {
    type Vtable = IAdaptiveMediaSourceDownloadStatistics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa306cefb_e96a_4dff_a9b8_1ae08c01ae98);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceDownloadStatistics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ContentBytesReceivedCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TimeToHeadersReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TimeToHeadersReceived: usize,
    #[cfg(feature = "Foundation")]
    pub TimeToFirstByteReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TimeToFirstByteReceived: usize,
    #[cfg(feature = "Foundation")]
    pub TimeToLastByteReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TimeToLastByteReceived: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdaptiveMediaSourcePlaybackBitrateChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAdaptiveMediaSourcePlaybackBitrateChangedEventArgs {
    type Vtable = IAdaptiveMediaSourcePlaybackBitrateChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x23a29f6d_7dda_4a51_87a9_6fa8c5b292be);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourcePlaybackBitrateChangedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub OldValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub NewValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub AudioOnly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdaptiveMediaSourceStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAdaptiveMediaSourceStatics {
    type Vtable = IAdaptiveMediaSourceStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x50a6bd5d_66ef_4cd3_9579_9e660507dc3f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdaptiveMediaSourceStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsContentTypeSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contenttype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CreateFromUriAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateFromUriAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Web_Http"))]
    pub CreateFromUriWithDownloaderAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, httpclient: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Web_Http")))]
    CreateFromUriWithDownloaderAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub CreateFromStreamAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, uri: ::windows::core::RawPtr, contenttype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    CreateFromStreamAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "Web_Http"))]
    pub CreateFromStreamWithDownloaderAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, uri: ::windows::core::RawPtr, contenttype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, httpclient: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams", feature = "Web_Http")))]
    CreateFromStreamWithDownloaderAsync: usize,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
