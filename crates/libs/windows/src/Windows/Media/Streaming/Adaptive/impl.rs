#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Core", feature = "implement_exclusive"))]
pub trait IAdaptiveMediaSource_Impl: Sized + super::super::Core::IMediaSource_Impl {
    fn IsLive(&mut self) -> ::windows::core::Result<bool>;
    fn DesiredLiveOffset(&mut self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn SetDesiredLiveOffset(&mut self, value: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn InitialBitrate(&mut self) -> ::windows::core::Result<u32>;
    fn SetInitialBitrate(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn CurrentDownloadBitrate(&mut self) -> ::windows::core::Result<u32>;
    fn CurrentPlaybackBitrate(&mut self) -> ::windows::core::Result<u32>;
    fn AvailableBitrates(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<u32>>;
    fn DesiredMinBitrate(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u32>>;
    fn SetDesiredMinBitrate(&mut self, value: &::core::option::Option<super::super::super::Foundation::IReference<u32>>) -> ::windows::core::Result<()>;
    fn DesiredMaxBitrate(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u32>>;
    fn SetDesiredMaxBitrate(&mut self, value: &::core::option::Option<super::super::super::Foundation::IReference<u32>>) -> ::windows::core::Result<()>;
    fn AudioOnlyPlayback(&mut self) -> ::windows::core::Result<bool>;
    fn InboundBitsPerSecond(&mut self) -> ::windows::core::Result<u64>;
    fn InboundBitsPerSecondWindow(&mut self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn SetInboundBitsPerSecondWindow(&mut self, value: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn DownloadBitrateChanged(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<AdaptiveMediaSource, AdaptiveMediaSourceDownloadBitrateChangedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveDownloadBitrateChanged(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PlaybackBitrateChanged(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<AdaptiveMediaSource, AdaptiveMediaSourcePlaybackBitrateChangedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePlaybackBitrateChanged(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DownloadRequested(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<AdaptiveMediaSource, AdaptiveMediaSourceDownloadRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveDownloadRequested(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DownloadCompleted(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<AdaptiveMediaSource, AdaptiveMediaSourceDownloadCompletedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveDownloadCompleted(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DownloadFailed(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<AdaptiveMediaSource, AdaptiveMediaSourceDownloadFailedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveDownloadFailed(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Core", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAdaptiveMediaSource {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.IAdaptiveMediaSource";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Core", feature = "implement_exclusive"))]
impl IAdaptiveMediaSource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdaptiveMediaSource_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdaptiveMediaSource_Vtbl {
        unsafe extern "system" fn IsLive<Impl: IAdaptiveMediaSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsLive() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DesiredLiveOffset<Impl: IAdaptiveMediaSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DesiredLiveOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredLiveOffset<Impl: IAdaptiveMediaSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDesiredLiveOffset(&*(&value as *const <super::super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InitialBitrate<Impl: IAdaptiveMediaSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitialBitrate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInitialBitrate<Impl: IAdaptiveMediaSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInitialBitrate(value).into()
        }
        unsafe extern "system" fn CurrentDownloadBitrate<Impl: IAdaptiveMediaSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentDownloadBitrate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentPlaybackBitrate<Impl: IAdaptiveMediaSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentPlaybackBitrate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AvailableBitrates<Impl: IAdaptiveMediaSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AvailableBitrates() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DesiredMinBitrate<Impl: IAdaptiveMediaSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DesiredMinBitrate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredMinBitrate<Impl: IAdaptiveMediaSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDesiredMinBitrate(&*(&value as *const <super::super::super::Foundation::IReference<u32> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<u32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DesiredMaxBitrate<Impl: IAdaptiveMediaSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DesiredMaxBitrate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredMaxBitrate<Impl: IAdaptiveMediaSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDesiredMaxBitrate(&*(&value as *const <super::super::super::Foundation::IReference<u32> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<u32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AudioOnlyPlayback<Impl: IAdaptiveMediaSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioOnlyPlayback() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InboundBitsPerSecond<Impl: IAdaptiveMediaSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InboundBitsPerSecond() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InboundBitsPerSecondWindow<Impl: IAdaptiveMediaSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InboundBitsPerSecondWindow() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInboundBitsPerSecondWindow<Impl: IAdaptiveMediaSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInboundBitsPerSecondWindow(&*(&value as *const <super::super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DownloadBitrateChanged<Impl: IAdaptiveMediaSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DownloadBitrateChanged(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<AdaptiveMediaSource, AdaptiveMediaSourceDownloadBitrateChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<AdaptiveMediaSource, AdaptiveMediaSourceDownloadBitrateChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDownloadBitrateChanged<Impl: IAdaptiveMediaSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveDownloadBitrateChanged(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PlaybackBitrateChanged<Impl: IAdaptiveMediaSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PlaybackBitrateChanged(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<AdaptiveMediaSource, AdaptiveMediaSourcePlaybackBitrateChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<AdaptiveMediaSource, AdaptiveMediaSourcePlaybackBitrateChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePlaybackBitrateChanged<Impl: IAdaptiveMediaSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePlaybackBitrateChanged(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DownloadRequested<Impl: IAdaptiveMediaSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DownloadRequested(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<AdaptiveMediaSource, AdaptiveMediaSourceDownloadRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<AdaptiveMediaSource, AdaptiveMediaSourceDownloadRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDownloadRequested<Impl: IAdaptiveMediaSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveDownloadRequested(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DownloadCompleted<Impl: IAdaptiveMediaSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DownloadCompleted(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<AdaptiveMediaSource, AdaptiveMediaSourceDownloadCompletedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<AdaptiveMediaSource, AdaptiveMediaSourceDownloadCompletedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDownloadCompleted<Impl: IAdaptiveMediaSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveDownloadCompleted(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DownloadFailed<Impl: IAdaptiveMediaSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DownloadFailed(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<AdaptiveMediaSource, AdaptiveMediaSourceDownloadFailedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<AdaptiveMediaSource, AdaptiveMediaSourceDownloadFailedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDownloadFailed<Impl: IAdaptiveMediaSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveDownloadFailed(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAdaptiveMediaSource, BASE_OFFSET>(),
            IsLive: IsLive::<Impl, IMPL_OFFSET>,
            DesiredLiveOffset: DesiredLiveOffset::<Impl, IMPL_OFFSET>,
            SetDesiredLiveOffset: SetDesiredLiveOffset::<Impl, IMPL_OFFSET>,
            InitialBitrate: InitialBitrate::<Impl, IMPL_OFFSET>,
            SetInitialBitrate: SetInitialBitrate::<Impl, IMPL_OFFSET>,
            CurrentDownloadBitrate: CurrentDownloadBitrate::<Impl, IMPL_OFFSET>,
            CurrentPlaybackBitrate: CurrentPlaybackBitrate::<Impl, IMPL_OFFSET>,
            AvailableBitrates: AvailableBitrates::<Impl, IMPL_OFFSET>,
            DesiredMinBitrate: DesiredMinBitrate::<Impl, IMPL_OFFSET>,
            SetDesiredMinBitrate: SetDesiredMinBitrate::<Impl, IMPL_OFFSET>,
            DesiredMaxBitrate: DesiredMaxBitrate::<Impl, IMPL_OFFSET>,
            SetDesiredMaxBitrate: SetDesiredMaxBitrate::<Impl, IMPL_OFFSET>,
            AudioOnlyPlayback: AudioOnlyPlayback::<Impl, IMPL_OFFSET>,
            InboundBitsPerSecond: InboundBitsPerSecond::<Impl, IMPL_OFFSET>,
            InboundBitsPerSecondWindow: InboundBitsPerSecondWindow::<Impl, IMPL_OFFSET>,
            SetInboundBitsPerSecondWindow: SetInboundBitsPerSecondWindow::<Impl, IMPL_OFFSET>,
            DownloadBitrateChanged: DownloadBitrateChanged::<Impl, IMPL_OFFSET>,
            RemoveDownloadBitrateChanged: RemoveDownloadBitrateChanged::<Impl, IMPL_OFFSET>,
            PlaybackBitrateChanged: PlaybackBitrateChanged::<Impl, IMPL_OFFSET>,
            RemovePlaybackBitrateChanged: RemovePlaybackBitrateChanged::<Impl, IMPL_OFFSET>,
            DownloadRequested: DownloadRequested::<Impl, IMPL_OFFSET>,
            RemoveDownloadRequested: RemoveDownloadRequested::<Impl, IMPL_OFFSET>,
            DownloadCompleted: DownloadCompleted::<Impl, IMPL_OFFSET>,
            RemoveDownloadCompleted: RemoveDownloadCompleted::<Impl, IMPL_OFFSET>,
            DownloadFailed: DownloadFailed::<Impl, IMPL_OFFSET>,
            RemoveDownloadFailed: RemoveDownloadFailed::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdaptiveMediaSource as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdaptiveMediaSource2_Impl: Sized {
    fn AdvancedSettings(&mut self) -> ::windows::core::Result<AdaptiveMediaSourceAdvancedSettings>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAdaptiveMediaSource2 {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.IAdaptiveMediaSource2";
}
#[cfg(feature = "implement_exclusive")]
impl IAdaptiveMediaSource2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdaptiveMediaSource2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdaptiveMediaSource2_Vtbl {
        unsafe extern "system" fn AdvancedSettings<Impl: IAdaptiveMediaSource2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdvancedSettings() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAdaptiveMediaSource2, BASE_OFFSET>(),
            AdvancedSettings: AdvancedSettings::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdaptiveMediaSource2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAdaptiveMediaSource3_Impl: Sized {
    fn MinLiveOffset(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>;
    fn MaxSeekableWindowSize(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>;
    fn DesiredSeekableWindowSize(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>;
    fn SetDesiredSeekableWindowSize(&mut self, value: &::core::option::Option<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>) -> ::windows::core::Result<()>;
    fn Diagnostics(&mut self) -> ::windows::core::Result<AdaptiveMediaSourceDiagnostics>;
    fn GetCorrelatedTimes(&mut self) -> ::windows::core::Result<AdaptiveMediaSourceCorrelatedTimes>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAdaptiveMediaSource3 {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.IAdaptiveMediaSource3";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAdaptiveMediaSource3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdaptiveMediaSource3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdaptiveMediaSource3_Vtbl {
        unsafe extern "system" fn MinLiveOffset<Impl: IAdaptiveMediaSource3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinLiveOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxSeekableWindowSize<Impl: IAdaptiveMediaSource3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxSeekableWindowSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DesiredSeekableWindowSize<Impl: IAdaptiveMediaSource3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DesiredSeekableWindowSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredSeekableWindowSize<Impl: IAdaptiveMediaSource3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDesiredSeekableWindowSize(&*(&value as *const <super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Diagnostics<Impl: IAdaptiveMediaSource3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Diagnostics() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCorrelatedTimes<Impl: IAdaptiveMediaSource3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCorrelatedTimes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAdaptiveMediaSource3, BASE_OFFSET>(),
            MinLiveOffset: MinLiveOffset::<Impl, IMPL_OFFSET>,
            MaxSeekableWindowSize: MaxSeekableWindowSize::<Impl, IMPL_OFFSET>,
            DesiredSeekableWindowSize: DesiredSeekableWindowSize::<Impl, IMPL_OFFSET>,
            SetDesiredSeekableWindowSize: SetDesiredSeekableWindowSize::<Impl, IMPL_OFFSET>,
            Diagnostics: Diagnostics::<Impl, IMPL_OFFSET>,
            GetCorrelatedTimes: GetCorrelatedTimes::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdaptiveMediaSource3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAdaptiveMediaSourceAdvancedSettings_Impl: Sized {
    fn AllSegmentsIndependent(&mut self) -> ::windows::core::Result<bool>;
    fn SetAllSegmentsIndependent(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn DesiredBitrateHeadroomRatio(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<f64>>;
    fn SetDesiredBitrateHeadroomRatio(&mut self, value: &::core::option::Option<super::super::super::Foundation::IReference<f64>>) -> ::windows::core::Result<()>;
    fn BitrateDowngradeTriggerRatio(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<f64>>;
    fn SetBitrateDowngradeTriggerRatio(&mut self, value: &::core::option::Option<super::super::super::Foundation::IReference<f64>>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAdaptiveMediaSourceAdvancedSettings {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceAdvancedSettings";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAdaptiveMediaSourceAdvancedSettings_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdaptiveMediaSourceAdvancedSettings_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdaptiveMediaSourceAdvancedSettings_Vtbl {
        unsafe extern "system" fn AllSegmentsIndependent<Impl: IAdaptiveMediaSourceAdvancedSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllSegmentsIndependent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllSegmentsIndependent<Impl: IAdaptiveMediaSourceAdvancedSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllSegmentsIndependent(value).into()
        }
        unsafe extern "system" fn DesiredBitrateHeadroomRatio<Impl: IAdaptiveMediaSourceAdvancedSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DesiredBitrateHeadroomRatio() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredBitrateHeadroomRatio<Impl: IAdaptiveMediaSourceAdvancedSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDesiredBitrateHeadroomRatio(&*(&value as *const <super::super::super::Foundation::IReference<f64> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<f64> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BitrateDowngradeTriggerRatio<Impl: IAdaptiveMediaSourceAdvancedSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BitrateDowngradeTriggerRatio() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBitrateDowngradeTriggerRatio<Impl: IAdaptiveMediaSourceAdvancedSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBitrateDowngradeTriggerRatio(&*(&value as *const <super::super::super::Foundation::IReference<f64> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<f64> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAdaptiveMediaSourceAdvancedSettings, BASE_OFFSET>(),
            AllSegmentsIndependent: AllSegmentsIndependent::<Impl, IMPL_OFFSET>,
            SetAllSegmentsIndependent: SetAllSegmentsIndependent::<Impl, IMPL_OFFSET>,
            DesiredBitrateHeadroomRatio: DesiredBitrateHeadroomRatio::<Impl, IMPL_OFFSET>,
            SetDesiredBitrateHeadroomRatio: SetDesiredBitrateHeadroomRatio::<Impl, IMPL_OFFSET>,
            BitrateDowngradeTriggerRatio: BitrateDowngradeTriggerRatio::<Impl, IMPL_OFFSET>,
            SetBitrateDowngradeTriggerRatio: SetBitrateDowngradeTriggerRatio::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdaptiveMediaSourceAdvancedSettings as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAdaptiveMediaSourceCorrelatedTimes_Impl: Sized {
    fn Position(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>;
    fn PresentationTimeStamp(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>;
    fn ProgramDateTime(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAdaptiveMediaSourceCorrelatedTimes {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceCorrelatedTimes";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAdaptiveMediaSourceCorrelatedTimes_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdaptiveMediaSourceCorrelatedTimes_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdaptiveMediaSourceCorrelatedTimes_Vtbl {
        unsafe extern "system" fn Position<Impl: IAdaptiveMediaSourceCorrelatedTimes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Position() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PresentationTimeStamp<Impl: IAdaptiveMediaSourceCorrelatedTimes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PresentationTimeStamp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProgramDateTime<Impl: IAdaptiveMediaSourceCorrelatedTimes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProgramDateTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAdaptiveMediaSourceCorrelatedTimes, BASE_OFFSET>(),
            Position: Position::<Impl, IMPL_OFFSET>,
            PresentationTimeStamp: PresentationTimeStamp::<Impl, IMPL_OFFSET>,
            ProgramDateTime: ProgramDateTime::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdaptiveMediaSourceCorrelatedTimes as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Web_Http", feature = "implement_exclusive"))]
pub trait IAdaptiveMediaSourceCreationResult_Impl: Sized {
    fn Status(&mut self) -> ::windows::core::Result<AdaptiveMediaSourceCreationStatus>;
    fn MediaSource(&mut self) -> ::windows::core::Result<AdaptiveMediaSource>;
    fn HttpResponseMessage(&mut self) -> ::windows::core::Result<super::super::super::Web::Http::HttpResponseMessage>;
}
#[cfg(all(feature = "Web_Http", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAdaptiveMediaSourceCreationResult {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceCreationResult";
}
#[cfg(all(feature = "Web_Http", feature = "implement_exclusive"))]
impl IAdaptiveMediaSourceCreationResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdaptiveMediaSourceCreationResult_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdaptiveMediaSourceCreationResult_Vtbl {
        unsafe extern "system" fn Status<Impl: IAdaptiveMediaSourceCreationResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AdaptiveMediaSourceCreationStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediaSource<Impl: IAdaptiveMediaSourceCreationResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MediaSource() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HttpResponseMessage<Impl: IAdaptiveMediaSourceCreationResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HttpResponseMessage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAdaptiveMediaSourceCreationResult, BASE_OFFSET>(),
            Status: Status::<Impl, IMPL_OFFSET>,
            MediaSource: MediaSource::<Impl, IMPL_OFFSET>,
            HttpResponseMessage: HttpResponseMessage::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdaptiveMediaSourceCreationResult as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdaptiveMediaSourceCreationResult2_Impl: Sized {
    fn ExtendedError(&mut self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAdaptiveMediaSourceCreationResult2 {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceCreationResult2";
}
#[cfg(feature = "implement_exclusive")]
impl IAdaptiveMediaSourceCreationResult2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdaptiveMediaSourceCreationResult2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdaptiveMediaSourceCreationResult2_Vtbl {
        unsafe extern "system" fn ExtendedError<Impl: IAdaptiveMediaSourceCreationResult2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtendedError() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAdaptiveMediaSourceCreationResult2, BASE_OFFSET>(),
            ExtendedError: ExtendedError::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdaptiveMediaSourceCreationResult2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAdaptiveMediaSourceDiagnosticAvailableEventArgs_Impl: Sized {
    fn DiagnosticType(&mut self) -> ::windows::core::Result<AdaptiveMediaSourceDiagnosticType>;
    fn RequestId(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<i32>>;
    fn Position(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>;
    fn SegmentId(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u64>>;
    fn ResourceType(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<AdaptiveMediaSourceResourceType>>;
    fn ResourceUri(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
    fn ResourceByteRangeOffset(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u64>>;
    fn ResourceByteRangeLength(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u64>>;
    fn Bitrate(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u32>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAdaptiveMediaSourceDiagnosticAvailableEventArgs {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDiagnosticAvailableEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAdaptiveMediaSourceDiagnosticAvailableEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdaptiveMediaSourceDiagnosticAvailableEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdaptiveMediaSourceDiagnosticAvailableEventArgs_Vtbl {
        unsafe extern "system" fn DiagnosticType<Impl: IAdaptiveMediaSourceDiagnosticAvailableEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AdaptiveMediaSourceDiagnosticType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DiagnosticType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestId<Impl: IAdaptiveMediaSourceDiagnosticAvailableEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Position<Impl: IAdaptiveMediaSourceDiagnosticAvailableEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Position() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SegmentId<Impl: IAdaptiveMediaSourceDiagnosticAvailableEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SegmentId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResourceType<Impl: IAdaptiveMediaSourceDiagnosticAvailableEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResourceType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResourceUri<Impl: IAdaptiveMediaSourceDiagnosticAvailableEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResourceUri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResourceByteRangeOffset<Impl: IAdaptiveMediaSourceDiagnosticAvailableEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResourceByteRangeOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResourceByteRangeLength<Impl: IAdaptiveMediaSourceDiagnosticAvailableEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResourceByteRangeLength() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Bitrate<Impl: IAdaptiveMediaSourceDiagnosticAvailableEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Bitrate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAdaptiveMediaSourceDiagnosticAvailableEventArgs, BASE_OFFSET>(),
            DiagnosticType: DiagnosticType::<Impl, IMPL_OFFSET>,
            RequestId: RequestId::<Impl, IMPL_OFFSET>,
            Position: Position::<Impl, IMPL_OFFSET>,
            SegmentId: SegmentId::<Impl, IMPL_OFFSET>,
            ResourceType: ResourceType::<Impl, IMPL_OFFSET>,
            ResourceUri: ResourceUri::<Impl, IMPL_OFFSET>,
            ResourceByteRangeOffset: ResourceByteRangeOffset::<Impl, IMPL_OFFSET>,
            ResourceByteRangeLength: ResourceByteRangeLength::<Impl, IMPL_OFFSET>,
            Bitrate: Bitrate::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdaptiveMediaSourceDiagnosticAvailableEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdaptiveMediaSourceDiagnosticAvailableEventArgs2_Impl: Sized {
    fn ExtendedError(&mut self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAdaptiveMediaSourceDiagnosticAvailableEventArgs2 {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDiagnosticAvailableEventArgs2";
}
#[cfg(feature = "implement_exclusive")]
impl IAdaptiveMediaSourceDiagnosticAvailableEventArgs2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdaptiveMediaSourceDiagnosticAvailableEventArgs2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdaptiveMediaSourceDiagnosticAvailableEventArgs2_Vtbl {
        unsafe extern "system" fn ExtendedError<Impl: IAdaptiveMediaSourceDiagnosticAvailableEventArgs2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtendedError() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAdaptiveMediaSourceDiagnosticAvailableEventArgs2, BASE_OFFSET>(),
            ExtendedError: ExtendedError::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdaptiveMediaSourceDiagnosticAvailableEventArgs2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAdaptiveMediaSourceDiagnosticAvailableEventArgs3_Impl: Sized {
    fn ResourceDuration(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>;
    fn ResourceContentType(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAdaptiveMediaSourceDiagnosticAvailableEventArgs3 {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDiagnosticAvailableEventArgs3";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAdaptiveMediaSourceDiagnosticAvailableEventArgs3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdaptiveMediaSourceDiagnosticAvailableEventArgs3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdaptiveMediaSourceDiagnosticAvailableEventArgs3_Vtbl {
        unsafe extern "system" fn ResourceDuration<Impl: IAdaptiveMediaSourceDiagnosticAvailableEventArgs3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResourceDuration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResourceContentType<Impl: IAdaptiveMediaSourceDiagnosticAvailableEventArgs3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResourceContentType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAdaptiveMediaSourceDiagnosticAvailableEventArgs3, BASE_OFFSET>(),
            ResourceDuration: ResourceDuration::<Impl, IMPL_OFFSET>,
            ResourceContentType: ResourceContentType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdaptiveMediaSourceDiagnosticAvailableEventArgs3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAdaptiveMediaSourceDiagnostics_Impl: Sized {
    fn DiagnosticAvailable(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<AdaptiveMediaSourceDiagnostics, AdaptiveMediaSourceDiagnosticAvailableEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveDiagnosticAvailable(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAdaptiveMediaSourceDiagnostics {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDiagnostics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAdaptiveMediaSourceDiagnostics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdaptiveMediaSourceDiagnostics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdaptiveMediaSourceDiagnostics_Vtbl {
        unsafe extern "system" fn DiagnosticAvailable<Impl: IAdaptiveMediaSourceDiagnostics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DiagnosticAvailable(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<AdaptiveMediaSourceDiagnostics, AdaptiveMediaSourceDiagnosticAvailableEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<AdaptiveMediaSourceDiagnostics, AdaptiveMediaSourceDiagnosticAvailableEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDiagnosticAvailable<Impl: IAdaptiveMediaSourceDiagnostics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveDiagnosticAvailable(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAdaptiveMediaSourceDiagnostics, BASE_OFFSET>(),
            DiagnosticAvailable: DiagnosticAvailable::<Impl, IMPL_OFFSET>,
            RemoveDiagnosticAvailable: RemoveDiagnosticAvailable::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdaptiveMediaSourceDiagnostics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdaptiveMediaSourceDownloadBitrateChangedEventArgs_Impl: Sized {
    fn OldValue(&mut self) -> ::windows::core::Result<u32>;
    fn NewValue(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAdaptiveMediaSourceDownloadBitrateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadBitrateChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IAdaptiveMediaSourceDownloadBitrateChangedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdaptiveMediaSourceDownloadBitrateChangedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdaptiveMediaSourceDownloadBitrateChangedEventArgs_Vtbl {
        unsafe extern "system" fn OldValue<Impl: IAdaptiveMediaSourceDownloadBitrateChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OldValue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NewValue<Impl: IAdaptiveMediaSourceDownloadBitrateChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NewValue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAdaptiveMediaSourceDownloadBitrateChangedEventArgs, BASE_OFFSET>(),
            OldValue: OldValue::<Impl, IMPL_OFFSET>,
            NewValue: NewValue::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdaptiveMediaSourceDownloadBitrateChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdaptiveMediaSourceDownloadBitrateChangedEventArgs2_Impl: Sized {
    fn Reason(&mut self) -> ::windows::core::Result<AdaptiveMediaSourceDownloadBitrateChangedReason>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAdaptiveMediaSourceDownloadBitrateChangedEventArgs2 {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadBitrateChangedEventArgs2";
}
#[cfg(feature = "implement_exclusive")]
impl IAdaptiveMediaSourceDownloadBitrateChangedEventArgs2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdaptiveMediaSourceDownloadBitrateChangedEventArgs2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdaptiveMediaSourceDownloadBitrateChangedEventArgs2_Vtbl {
        unsafe extern "system" fn Reason<Impl: IAdaptiveMediaSourceDownloadBitrateChangedEventArgs2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AdaptiveMediaSourceDownloadBitrateChangedReason) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reason() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAdaptiveMediaSourceDownloadBitrateChangedEventArgs2, BASE_OFFSET>(),
            Reason: Reason::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdaptiveMediaSourceDownloadBitrateChangedEventArgs2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Web_Http", feature = "implement_exclusive"))]
pub trait IAdaptiveMediaSourceDownloadCompletedEventArgs_Impl: Sized {
    fn ResourceType(&mut self) -> ::windows::core::Result<AdaptiveMediaSourceResourceType>;
    fn ResourceUri(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
    fn ResourceByteRangeOffset(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u64>>;
    fn ResourceByteRangeLength(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u64>>;
    fn HttpResponseMessage(&mut self) -> ::windows::core::Result<super::super::super::Web::Http::HttpResponseMessage>;
}
#[cfg(all(feature = "Foundation", feature = "Web_Http", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAdaptiveMediaSourceDownloadCompletedEventArgs {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadCompletedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "Web_Http", feature = "implement_exclusive"))]
impl IAdaptiveMediaSourceDownloadCompletedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdaptiveMediaSourceDownloadCompletedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdaptiveMediaSourceDownloadCompletedEventArgs_Vtbl {
        unsafe extern "system" fn ResourceType<Impl: IAdaptiveMediaSourceDownloadCompletedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AdaptiveMediaSourceResourceType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResourceType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResourceUri<Impl: IAdaptiveMediaSourceDownloadCompletedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResourceUri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResourceByteRangeOffset<Impl: IAdaptiveMediaSourceDownloadCompletedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResourceByteRangeOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResourceByteRangeLength<Impl: IAdaptiveMediaSourceDownloadCompletedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResourceByteRangeLength() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HttpResponseMessage<Impl: IAdaptiveMediaSourceDownloadCompletedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HttpResponseMessage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAdaptiveMediaSourceDownloadCompletedEventArgs, BASE_OFFSET>(),
            ResourceType: ResourceType::<Impl, IMPL_OFFSET>,
            ResourceUri: ResourceUri::<Impl, IMPL_OFFSET>,
            ResourceByteRangeOffset: ResourceByteRangeOffset::<Impl, IMPL_OFFSET>,
            ResourceByteRangeLength: ResourceByteRangeLength::<Impl, IMPL_OFFSET>,
            HttpResponseMessage: HttpResponseMessage::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdaptiveMediaSourceDownloadCompletedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAdaptiveMediaSourceDownloadCompletedEventArgs2_Impl: Sized {
    fn RequestId(&mut self) -> ::windows::core::Result<i32>;
    fn Statistics(&mut self) -> ::windows::core::Result<AdaptiveMediaSourceDownloadStatistics>;
    fn Position(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAdaptiveMediaSourceDownloadCompletedEventArgs2 {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadCompletedEventArgs2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAdaptiveMediaSourceDownloadCompletedEventArgs2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdaptiveMediaSourceDownloadCompletedEventArgs2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdaptiveMediaSourceDownloadCompletedEventArgs2_Vtbl {
        unsafe extern "system" fn RequestId<Impl: IAdaptiveMediaSourceDownloadCompletedEventArgs2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Statistics<Impl: IAdaptiveMediaSourceDownloadCompletedEventArgs2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Statistics() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Position<Impl: IAdaptiveMediaSourceDownloadCompletedEventArgs2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Position() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAdaptiveMediaSourceDownloadCompletedEventArgs2, BASE_OFFSET>(),
            RequestId: RequestId::<Impl, IMPL_OFFSET>,
            Statistics: Statistics::<Impl, IMPL_OFFSET>,
            Position: Position::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdaptiveMediaSourceDownloadCompletedEventArgs2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAdaptiveMediaSourceDownloadCompletedEventArgs3_Impl: Sized {
    fn ResourceDuration(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>;
    fn ResourceContentType(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAdaptiveMediaSourceDownloadCompletedEventArgs3 {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadCompletedEventArgs3";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAdaptiveMediaSourceDownloadCompletedEventArgs3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdaptiveMediaSourceDownloadCompletedEventArgs3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdaptiveMediaSourceDownloadCompletedEventArgs3_Vtbl {
        unsafe extern "system" fn ResourceDuration<Impl: IAdaptiveMediaSourceDownloadCompletedEventArgs3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResourceDuration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResourceContentType<Impl: IAdaptiveMediaSourceDownloadCompletedEventArgs3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResourceContentType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAdaptiveMediaSourceDownloadCompletedEventArgs3, BASE_OFFSET>(),
            ResourceDuration: ResourceDuration::<Impl, IMPL_OFFSET>,
            ResourceContentType: ResourceContentType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdaptiveMediaSourceDownloadCompletedEventArgs3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Web_Http", feature = "implement_exclusive"))]
pub trait IAdaptiveMediaSourceDownloadFailedEventArgs_Impl: Sized {
    fn ResourceType(&mut self) -> ::windows::core::Result<AdaptiveMediaSourceResourceType>;
    fn ResourceUri(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
    fn ResourceByteRangeOffset(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u64>>;
    fn ResourceByteRangeLength(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u64>>;
    fn HttpResponseMessage(&mut self) -> ::windows::core::Result<super::super::super::Web::Http::HttpResponseMessage>;
}
#[cfg(all(feature = "Foundation", feature = "Web_Http", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAdaptiveMediaSourceDownloadFailedEventArgs {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadFailedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "Web_Http", feature = "implement_exclusive"))]
impl IAdaptiveMediaSourceDownloadFailedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdaptiveMediaSourceDownloadFailedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdaptiveMediaSourceDownloadFailedEventArgs_Vtbl {
        unsafe extern "system" fn ResourceType<Impl: IAdaptiveMediaSourceDownloadFailedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AdaptiveMediaSourceResourceType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResourceType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResourceUri<Impl: IAdaptiveMediaSourceDownloadFailedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResourceUri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResourceByteRangeOffset<Impl: IAdaptiveMediaSourceDownloadFailedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResourceByteRangeOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResourceByteRangeLength<Impl: IAdaptiveMediaSourceDownloadFailedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResourceByteRangeLength() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HttpResponseMessage<Impl: IAdaptiveMediaSourceDownloadFailedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HttpResponseMessage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAdaptiveMediaSourceDownloadFailedEventArgs, BASE_OFFSET>(),
            ResourceType: ResourceType::<Impl, IMPL_OFFSET>,
            ResourceUri: ResourceUri::<Impl, IMPL_OFFSET>,
            ResourceByteRangeOffset: ResourceByteRangeOffset::<Impl, IMPL_OFFSET>,
            ResourceByteRangeLength: ResourceByteRangeLength::<Impl, IMPL_OFFSET>,
            HttpResponseMessage: HttpResponseMessage::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdaptiveMediaSourceDownloadFailedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAdaptiveMediaSourceDownloadFailedEventArgs2_Impl: Sized {
    fn RequestId(&mut self) -> ::windows::core::Result<i32>;
    fn ExtendedError(&mut self) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn Statistics(&mut self) -> ::windows::core::Result<AdaptiveMediaSourceDownloadStatistics>;
    fn Position(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAdaptiveMediaSourceDownloadFailedEventArgs2 {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadFailedEventArgs2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAdaptiveMediaSourceDownloadFailedEventArgs2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdaptiveMediaSourceDownloadFailedEventArgs2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdaptiveMediaSourceDownloadFailedEventArgs2_Vtbl {
        unsafe extern "system" fn RequestId<Impl: IAdaptiveMediaSourceDownloadFailedEventArgs2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedError<Impl: IAdaptiveMediaSourceDownloadFailedEventArgs2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtendedError() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Statistics<Impl: IAdaptiveMediaSourceDownloadFailedEventArgs2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Statistics() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Position<Impl: IAdaptiveMediaSourceDownloadFailedEventArgs2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Position() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAdaptiveMediaSourceDownloadFailedEventArgs2, BASE_OFFSET>(),
            RequestId: RequestId::<Impl, IMPL_OFFSET>,
            ExtendedError: ExtendedError::<Impl, IMPL_OFFSET>,
            Statistics: Statistics::<Impl, IMPL_OFFSET>,
            Position: Position::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdaptiveMediaSourceDownloadFailedEventArgs2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAdaptiveMediaSourceDownloadFailedEventArgs3_Impl: Sized {
    fn ResourceDuration(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>;
    fn ResourceContentType(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAdaptiveMediaSourceDownloadFailedEventArgs3 {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadFailedEventArgs3";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAdaptiveMediaSourceDownloadFailedEventArgs3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdaptiveMediaSourceDownloadFailedEventArgs3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdaptiveMediaSourceDownloadFailedEventArgs3_Vtbl {
        unsafe extern "system" fn ResourceDuration<Impl: IAdaptiveMediaSourceDownloadFailedEventArgs3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResourceDuration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResourceContentType<Impl: IAdaptiveMediaSourceDownloadFailedEventArgs3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResourceContentType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAdaptiveMediaSourceDownloadFailedEventArgs3, BASE_OFFSET>(),
            ResourceDuration: ResourceDuration::<Impl, IMPL_OFFSET>,
            ResourceContentType: ResourceContentType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdaptiveMediaSourceDownloadFailedEventArgs3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdaptiveMediaSourceDownloadRequestedDeferral_Impl: Sized {
    fn Complete(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAdaptiveMediaSourceDownloadRequestedDeferral {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadRequestedDeferral";
}
#[cfg(feature = "implement_exclusive")]
impl IAdaptiveMediaSourceDownloadRequestedDeferral_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdaptiveMediaSourceDownloadRequestedDeferral_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdaptiveMediaSourceDownloadRequestedDeferral_Vtbl {
        unsafe extern "system" fn Complete<Impl: IAdaptiveMediaSourceDownloadRequestedDeferral_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Complete().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAdaptiveMediaSourceDownloadRequestedDeferral, BASE_OFFSET>(),
            Complete: Complete::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdaptiveMediaSourceDownloadRequestedDeferral as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAdaptiveMediaSourceDownloadRequestedEventArgs_Impl: Sized {
    fn ResourceType(&mut self) -> ::windows::core::Result<AdaptiveMediaSourceResourceType>;
    fn ResourceUri(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
    fn ResourceByteRangeOffset(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u64>>;
    fn ResourceByteRangeLength(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u64>>;
    fn Result(&mut self) -> ::windows::core::Result<AdaptiveMediaSourceDownloadResult>;
    fn GetDeferral(&mut self) -> ::windows::core::Result<AdaptiveMediaSourceDownloadRequestedDeferral>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAdaptiveMediaSourceDownloadRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadRequestedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAdaptiveMediaSourceDownloadRequestedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdaptiveMediaSourceDownloadRequestedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdaptiveMediaSourceDownloadRequestedEventArgs_Vtbl {
        unsafe extern "system" fn ResourceType<Impl: IAdaptiveMediaSourceDownloadRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AdaptiveMediaSourceResourceType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResourceType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResourceUri<Impl: IAdaptiveMediaSourceDownloadRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResourceUri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResourceByteRangeOffset<Impl: IAdaptiveMediaSourceDownloadRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResourceByteRangeOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResourceByteRangeLength<Impl: IAdaptiveMediaSourceDownloadRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResourceByteRangeLength() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Result<Impl: IAdaptiveMediaSourceDownloadRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Result() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeferral<Impl: IAdaptiveMediaSourceDownloadRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAdaptiveMediaSourceDownloadRequestedEventArgs, BASE_OFFSET>(),
            ResourceType: ResourceType::<Impl, IMPL_OFFSET>,
            ResourceUri: ResourceUri::<Impl, IMPL_OFFSET>,
            ResourceByteRangeOffset: ResourceByteRangeOffset::<Impl, IMPL_OFFSET>,
            ResourceByteRangeLength: ResourceByteRangeLength::<Impl, IMPL_OFFSET>,
            Result: Result::<Impl, IMPL_OFFSET>,
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdaptiveMediaSourceDownloadRequestedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAdaptiveMediaSourceDownloadRequestedEventArgs2_Impl: Sized {
    fn RequestId(&mut self) -> ::windows::core::Result<i32>;
    fn Position(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAdaptiveMediaSourceDownloadRequestedEventArgs2 {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadRequestedEventArgs2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAdaptiveMediaSourceDownloadRequestedEventArgs2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdaptiveMediaSourceDownloadRequestedEventArgs2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdaptiveMediaSourceDownloadRequestedEventArgs2_Vtbl {
        unsafe extern "system" fn RequestId<Impl: IAdaptiveMediaSourceDownloadRequestedEventArgs2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Position<Impl: IAdaptiveMediaSourceDownloadRequestedEventArgs2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Position() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAdaptiveMediaSourceDownloadRequestedEventArgs2, BASE_OFFSET>(),
            RequestId: RequestId::<Impl, IMPL_OFFSET>,
            Position: Position::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdaptiveMediaSourceDownloadRequestedEventArgs2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAdaptiveMediaSourceDownloadRequestedEventArgs3_Impl: Sized {
    fn ResourceDuration(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>;
    fn ResourceContentType(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAdaptiveMediaSourceDownloadRequestedEventArgs3 {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadRequestedEventArgs3";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAdaptiveMediaSourceDownloadRequestedEventArgs3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdaptiveMediaSourceDownloadRequestedEventArgs3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdaptiveMediaSourceDownloadRequestedEventArgs3_Vtbl {
        unsafe extern "system" fn ResourceDuration<Impl: IAdaptiveMediaSourceDownloadRequestedEventArgs3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResourceDuration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResourceContentType<Impl: IAdaptiveMediaSourceDownloadRequestedEventArgs3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResourceContentType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAdaptiveMediaSourceDownloadRequestedEventArgs3, BASE_OFFSET>(),
            ResourceDuration: ResourceDuration::<Impl, IMPL_OFFSET>,
            ResourceContentType: ResourceContentType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdaptiveMediaSourceDownloadRequestedEventArgs3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IAdaptiveMediaSourceDownloadResult_Impl: Sized {
    fn ResourceUri(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
    fn SetResourceUri(&mut self, value: &::core::option::Option<super::super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn InputStream(&mut self) -> ::windows::core::Result<super::super::super::Storage::Streams::IInputStream>;
    fn SetInputStream(&mut self, value: &::core::option::Option<super::super::super::Storage::Streams::IInputStream>) -> ::windows::core::Result<()>;
    fn Buffer(&mut self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
    fn SetBuffer(&mut self, value: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
    fn ContentType(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetContentType(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ExtendedStatus(&mut self) -> ::windows::core::Result<u32>;
    fn SetExtendedStatus(&mut self, value: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAdaptiveMediaSourceDownloadResult {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadResult";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IAdaptiveMediaSourceDownloadResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdaptiveMediaSourceDownloadResult_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdaptiveMediaSourceDownloadResult_Vtbl {
        unsafe extern "system" fn ResourceUri<Impl: IAdaptiveMediaSourceDownloadResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResourceUri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetResourceUri<Impl: IAdaptiveMediaSourceDownloadResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetResourceUri(&*(&value as *const <super::super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InputStream<Impl: IAdaptiveMediaSourceDownloadResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InputStream() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInputStream<Impl: IAdaptiveMediaSourceDownloadResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInputStream(&*(&value as *const <super::super::super::Storage::Streams::IInputStream as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IInputStream as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Buffer<Impl: IAdaptiveMediaSourceDownloadResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Buffer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBuffer<Impl: IAdaptiveMediaSourceDownloadResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBuffer(&*(&value as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ContentType<Impl: IAdaptiveMediaSourceDownloadResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContentType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContentType<Impl: IAdaptiveMediaSourceDownloadResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContentType(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ExtendedStatus<Impl: IAdaptiveMediaSourceDownloadResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtendedStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExtendedStatus<Impl: IAdaptiveMediaSourceDownloadResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExtendedStatus(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAdaptiveMediaSourceDownloadResult, BASE_OFFSET>(),
            ResourceUri: ResourceUri::<Impl, IMPL_OFFSET>,
            SetResourceUri: SetResourceUri::<Impl, IMPL_OFFSET>,
            InputStream: InputStream::<Impl, IMPL_OFFSET>,
            SetInputStream: SetInputStream::<Impl, IMPL_OFFSET>,
            Buffer: Buffer::<Impl, IMPL_OFFSET>,
            SetBuffer: SetBuffer::<Impl, IMPL_OFFSET>,
            ContentType: ContentType::<Impl, IMPL_OFFSET>,
            SetContentType: SetContentType::<Impl, IMPL_OFFSET>,
            ExtendedStatus: ExtendedStatus::<Impl, IMPL_OFFSET>,
            SetExtendedStatus: SetExtendedStatus::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdaptiveMediaSourceDownloadResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAdaptiveMediaSourceDownloadResult2_Impl: Sized {
    fn ResourceByteRangeOffset(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u64>>;
    fn SetResourceByteRangeOffset(&mut self, value: &::core::option::Option<super::super::super::Foundation::IReference<u64>>) -> ::windows::core::Result<()>;
    fn ResourceByteRangeLength(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u64>>;
    fn SetResourceByteRangeLength(&mut self, value: &::core::option::Option<super::super::super::Foundation::IReference<u64>>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAdaptiveMediaSourceDownloadResult2 {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadResult2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAdaptiveMediaSourceDownloadResult2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdaptiveMediaSourceDownloadResult2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdaptiveMediaSourceDownloadResult2_Vtbl {
        unsafe extern "system" fn ResourceByteRangeOffset<Impl: IAdaptiveMediaSourceDownloadResult2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResourceByteRangeOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetResourceByteRangeOffset<Impl: IAdaptiveMediaSourceDownloadResult2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetResourceByteRangeOffset(&*(&value as *const <super::super::super::Foundation::IReference<u64> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<u64> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ResourceByteRangeLength<Impl: IAdaptiveMediaSourceDownloadResult2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResourceByteRangeLength() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetResourceByteRangeLength<Impl: IAdaptiveMediaSourceDownloadResult2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetResourceByteRangeLength(&*(&value as *const <super::super::super::Foundation::IReference<u64> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<u64> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAdaptiveMediaSourceDownloadResult2, BASE_OFFSET>(),
            ResourceByteRangeOffset: ResourceByteRangeOffset::<Impl, IMPL_OFFSET>,
            SetResourceByteRangeOffset: SetResourceByteRangeOffset::<Impl, IMPL_OFFSET>,
            ResourceByteRangeLength: ResourceByteRangeLength::<Impl, IMPL_OFFSET>,
            SetResourceByteRangeLength: SetResourceByteRangeLength::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdaptiveMediaSourceDownloadResult2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAdaptiveMediaSourceDownloadStatistics_Impl: Sized {
    fn ContentBytesReceivedCount(&mut self) -> ::windows::core::Result<u64>;
    fn TimeToHeadersReceived(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>;
    fn TimeToFirstByteReceived(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>;
    fn TimeToLastByteReceived(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAdaptiveMediaSourceDownloadStatistics {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadStatistics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAdaptiveMediaSourceDownloadStatistics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdaptiveMediaSourceDownloadStatistics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdaptiveMediaSourceDownloadStatistics_Vtbl {
        unsafe extern "system" fn ContentBytesReceivedCount<Impl: IAdaptiveMediaSourceDownloadStatistics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContentBytesReceivedCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TimeToHeadersReceived<Impl: IAdaptiveMediaSourceDownloadStatistics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TimeToHeadersReceived() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TimeToFirstByteReceived<Impl: IAdaptiveMediaSourceDownloadStatistics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TimeToFirstByteReceived() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TimeToLastByteReceived<Impl: IAdaptiveMediaSourceDownloadStatistics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TimeToLastByteReceived() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAdaptiveMediaSourceDownloadStatistics, BASE_OFFSET>(),
            ContentBytesReceivedCount: ContentBytesReceivedCount::<Impl, IMPL_OFFSET>,
            TimeToHeadersReceived: TimeToHeadersReceived::<Impl, IMPL_OFFSET>,
            TimeToFirstByteReceived: TimeToFirstByteReceived::<Impl, IMPL_OFFSET>,
            TimeToLastByteReceived: TimeToLastByteReceived::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdaptiveMediaSourceDownloadStatistics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdaptiveMediaSourcePlaybackBitrateChangedEventArgs_Impl: Sized {
    fn OldValue(&mut self) -> ::windows::core::Result<u32>;
    fn NewValue(&mut self) -> ::windows::core::Result<u32>;
    fn AudioOnly(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAdaptiveMediaSourcePlaybackBitrateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourcePlaybackBitrateChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IAdaptiveMediaSourcePlaybackBitrateChangedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdaptiveMediaSourcePlaybackBitrateChangedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdaptiveMediaSourcePlaybackBitrateChangedEventArgs_Vtbl {
        unsafe extern "system" fn OldValue<Impl: IAdaptiveMediaSourcePlaybackBitrateChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OldValue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NewValue<Impl: IAdaptiveMediaSourcePlaybackBitrateChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NewValue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioOnly<Impl: IAdaptiveMediaSourcePlaybackBitrateChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioOnly() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAdaptiveMediaSourcePlaybackBitrateChangedEventArgs, BASE_OFFSET>(),
            OldValue: OldValue::<Impl, IMPL_OFFSET>,
            NewValue: NewValue::<Impl, IMPL_OFFSET>,
            AudioOnly: AudioOnly::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdaptiveMediaSourcePlaybackBitrateChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "Web_Http", feature = "implement_exclusive"))]
pub trait IAdaptiveMediaSourceStatics_Impl: Sized {
    fn IsContentTypeSupported(&mut self, contenttype: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
    fn CreateFromUriAsync(&mut self, uri: &::core::option::Option<super::super::super::Foundation::Uri>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<AdaptiveMediaSourceCreationResult>>;
    fn CreateFromUriWithDownloaderAsync(&mut self, uri: &::core::option::Option<super::super::super::Foundation::Uri>, httpclient: &::core::option::Option<super::super::super::Web::Http::HttpClient>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<AdaptiveMediaSourceCreationResult>>;
    fn CreateFromStreamAsync(&mut self, stream: &::core::option::Option<super::super::super::Storage::Streams::IInputStream>, uri: &::core::option::Option<super::super::super::Foundation::Uri>, contenttype: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<AdaptiveMediaSourceCreationResult>>;
    fn CreateFromStreamWithDownloaderAsync(&mut self, stream: &::core::option::Option<super::super::super::Storage::Streams::IInputStream>, uri: &::core::option::Option<super::super::super::Foundation::Uri>, contenttype: &::windows::core::HSTRING, httpclient: &::core::option::Option<super::super::super::Web::Http::HttpClient>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<AdaptiveMediaSourceCreationResult>>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "Web_Http", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAdaptiveMediaSourceStatics {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceStatics";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "Web_Http", feature = "implement_exclusive"))]
impl IAdaptiveMediaSourceStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdaptiveMediaSourceStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdaptiveMediaSourceStatics_Vtbl {
        unsafe extern "system" fn IsContentTypeSupported<Impl: IAdaptiveMediaSourceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contenttype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsContentTypeSupported(&*(&contenttype as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromUriAsync<Impl: IAdaptiveMediaSourceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromUriAsync(&*(&uri as *const <super::super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromUriWithDownloaderAsync<Impl: IAdaptiveMediaSourceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, httpclient: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromUriWithDownloaderAsync(&*(&uri as *const <super::super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType), &*(&httpclient as *const <super::super::super::Web::Http::HttpClient as ::windows::core::Abi>::Abi as *const <super::super::super::Web::Http::HttpClient as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromStreamAsync<Impl: IAdaptiveMediaSourceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, uri: ::windows::core::RawPtr, contenttype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromStreamAsync(
                &*(&stream as *const <super::super::super::Storage::Streams::IInputStream as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IInputStream as ::windows::core::DefaultType>::DefaultType),
                &*(&uri as *const <super::super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType),
                &*(&contenttype as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromStreamWithDownloaderAsync<Impl: IAdaptiveMediaSourceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, uri: ::windows::core::RawPtr, contenttype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, httpclient: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromStreamWithDownloaderAsync(
                &*(&stream as *const <super::super::super::Storage::Streams::IInputStream as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IInputStream as ::windows::core::DefaultType>::DefaultType),
                &*(&uri as *const <super::super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType),
                &*(&contenttype as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&httpclient as *const <super::super::super::Web::Http::HttpClient as ::windows::core::Abi>::Abi as *const <super::super::super::Web::Http::HttpClient as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAdaptiveMediaSourceStatics, BASE_OFFSET>(),
            IsContentTypeSupported: IsContentTypeSupported::<Impl, IMPL_OFFSET>,
            CreateFromUriAsync: CreateFromUriAsync::<Impl, IMPL_OFFSET>,
            CreateFromUriWithDownloaderAsync: CreateFromUriWithDownloaderAsync::<Impl, IMPL_OFFSET>,
            CreateFromStreamAsync: CreateFromStreamAsync::<Impl, IMPL_OFFSET>,
            CreateFromStreamWithDownloaderAsync: CreateFromStreamWithDownloaderAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdaptiveMediaSourceStatics as ::windows::core::Interface>::IID
    }
}
