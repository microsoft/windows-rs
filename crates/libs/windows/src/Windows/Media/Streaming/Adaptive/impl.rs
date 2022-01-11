#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Core", feature = "implement_exclusive"))]
pub trait IAdaptiveMediaSourceImpl: Sized + IMediaSourceImpl {
    fn IsLive(&self) -> ::windows::core::Result<bool>;
    fn DesiredLiveOffset(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn SetDesiredLiveOffset(&self, value: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn InitialBitrate(&self) -> ::windows::core::Result<u32>;
    fn SetInitialBitrate(&self, value: u32) -> ::windows::core::Result<()>;
    fn CurrentDownloadBitrate(&self) -> ::windows::core::Result<u32>;
    fn CurrentPlaybackBitrate(&self) -> ::windows::core::Result<u32>;
    fn AvailableBitrates(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<u32>>;
    fn DesiredMinBitrate(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u32>>;
    fn SetDesiredMinBitrate(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<u32>>) -> ::windows::core::Result<()>;
    fn DesiredMaxBitrate(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u32>>;
    fn SetDesiredMaxBitrate(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<u32>>) -> ::windows::core::Result<()>;
    fn AudioOnlyPlayback(&self) -> ::windows::core::Result<bool>;
    fn InboundBitsPerSecond(&self) -> ::windows::core::Result<u64>;
    fn InboundBitsPerSecondWindow(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn SetInboundBitsPerSecondWindow(&self, value: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn DownloadBitrateChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<AdaptiveMediaSource, AdaptiveMediaSourceDownloadBitrateChangedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveDownloadBitrateChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PlaybackBitrateChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<AdaptiveMediaSource, AdaptiveMediaSourcePlaybackBitrateChangedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePlaybackBitrateChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DownloadRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<AdaptiveMediaSource, AdaptiveMediaSourceDownloadRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveDownloadRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DownloadCompleted(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<AdaptiveMediaSource, AdaptiveMediaSourceDownloadCompletedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveDownloadCompleted(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DownloadFailed(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<AdaptiveMediaSource, AdaptiveMediaSourceDownloadFailedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveDownloadFailed(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Core", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAdaptiveMediaSource {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.IAdaptiveMediaSource";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Core", feature = "implement_exclusive"))]
impl IAdaptiveMediaSourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdaptiveMediaSourceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdaptiveMediaSourceVtbl {
        unsafe extern "system" fn IsLive<Impl: IAdaptiveMediaSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DesiredLiveOffset<Impl: IAdaptiveMediaSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDesiredLiveOffset<Impl: IAdaptiveMediaSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDesiredLiveOffset(&*(&value as *const <super::super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InitialBitrate<Impl: IAdaptiveMediaSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetInitialBitrate<Impl: IAdaptiveMediaSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInitialBitrate(value).into()
        }
        unsafe extern "system" fn CurrentDownloadBitrate<Impl: IAdaptiveMediaSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CurrentPlaybackBitrate<Impl: IAdaptiveMediaSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AvailableBitrates<Impl: IAdaptiveMediaSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DesiredMinBitrate<Impl: IAdaptiveMediaSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDesiredMinBitrate<Impl: IAdaptiveMediaSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDesiredMinBitrate(&*(&value as *const <super::super::super::Foundation::IReference<u32> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<u32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DesiredMaxBitrate<Impl: IAdaptiveMediaSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDesiredMaxBitrate<Impl: IAdaptiveMediaSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDesiredMaxBitrate(&*(&value as *const <super::super::super::Foundation::IReference<u32> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<u32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AudioOnlyPlayback<Impl: IAdaptiveMediaSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn InboundBitsPerSecond<Impl: IAdaptiveMediaSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn InboundBitsPerSecondWindow<Impl: IAdaptiveMediaSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetInboundBitsPerSecondWindow<Impl: IAdaptiveMediaSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInboundBitsPerSecondWindow(&*(&value as *const <super::super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DownloadBitrateChanged<Impl: IAdaptiveMediaSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveDownloadBitrateChanged<Impl: IAdaptiveMediaSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveDownloadBitrateChanged(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PlaybackBitrateChanged<Impl: IAdaptiveMediaSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemovePlaybackBitrateChanged<Impl: IAdaptiveMediaSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePlaybackBitrateChanged(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DownloadRequested<Impl: IAdaptiveMediaSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveDownloadRequested<Impl: IAdaptiveMediaSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveDownloadRequested(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DownloadCompleted<Impl: IAdaptiveMediaSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveDownloadCompleted<Impl: IAdaptiveMediaSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveDownloadCompleted(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DownloadFailed<Impl: IAdaptiveMediaSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveDownloadFailed<Impl: IAdaptiveMediaSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveDownloadFailed(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IAdaptiveMediaSource>,
            ::windows::core::GetTrustLevel,
            IsLive::<Impl, IMPL_OFFSET>,
            DesiredLiveOffset::<Impl, IMPL_OFFSET>,
            SetDesiredLiveOffset::<Impl, IMPL_OFFSET>,
            InitialBitrate::<Impl, IMPL_OFFSET>,
            SetInitialBitrate::<Impl, IMPL_OFFSET>,
            CurrentDownloadBitrate::<Impl, IMPL_OFFSET>,
            CurrentPlaybackBitrate::<Impl, IMPL_OFFSET>,
            AvailableBitrates::<Impl, IMPL_OFFSET>,
            DesiredMinBitrate::<Impl, IMPL_OFFSET>,
            SetDesiredMinBitrate::<Impl, IMPL_OFFSET>,
            DesiredMaxBitrate::<Impl, IMPL_OFFSET>,
            SetDesiredMaxBitrate::<Impl, IMPL_OFFSET>,
            AudioOnlyPlayback::<Impl, IMPL_OFFSET>,
            InboundBitsPerSecond::<Impl, IMPL_OFFSET>,
            InboundBitsPerSecondWindow::<Impl, IMPL_OFFSET>,
            SetInboundBitsPerSecondWindow::<Impl, IMPL_OFFSET>,
            DownloadBitrateChanged::<Impl, IMPL_OFFSET>,
            RemoveDownloadBitrateChanged::<Impl, IMPL_OFFSET>,
            PlaybackBitrateChanged::<Impl, IMPL_OFFSET>,
            RemovePlaybackBitrateChanged::<Impl, IMPL_OFFSET>,
            DownloadRequested::<Impl, IMPL_OFFSET>,
            RemoveDownloadRequested::<Impl, IMPL_OFFSET>,
            DownloadCompleted::<Impl, IMPL_OFFSET>,
            RemoveDownloadCompleted::<Impl, IMPL_OFFSET>,
            DownloadFailed::<Impl, IMPL_OFFSET>,
            RemoveDownloadFailed::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdaptiveMediaSource as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdaptiveMediaSource2Impl: Sized {
    fn AdvancedSettings(&self) -> ::windows::core::Result<AdaptiveMediaSourceAdvancedSettings>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAdaptiveMediaSource2 {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.IAdaptiveMediaSource2";
}
#[cfg(feature = "implement_exclusive")]
impl IAdaptiveMediaSource2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdaptiveMediaSource2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdaptiveMediaSource2Vtbl {
        unsafe extern "system" fn AdvancedSettings<Impl: IAdaptiveMediaSource2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAdaptiveMediaSource2>, ::windows::core::GetTrustLevel, AdvancedSettings::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdaptiveMediaSource2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAdaptiveMediaSource3Impl: Sized {
    fn MinLiveOffset(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>;
    fn MaxSeekableWindowSize(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>;
    fn DesiredSeekableWindowSize(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>;
    fn SetDesiredSeekableWindowSize(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>) -> ::windows::core::Result<()>;
    fn Diagnostics(&self) -> ::windows::core::Result<AdaptiveMediaSourceDiagnostics>;
    fn GetCorrelatedTimes(&self) -> ::windows::core::Result<AdaptiveMediaSourceCorrelatedTimes>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAdaptiveMediaSource3 {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.IAdaptiveMediaSource3";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAdaptiveMediaSource3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdaptiveMediaSource3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdaptiveMediaSource3Vtbl {
        unsafe extern "system" fn MinLiveOffset<Impl: IAdaptiveMediaSource3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MaxSeekableWindowSize<Impl: IAdaptiveMediaSource3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DesiredSeekableWindowSize<Impl: IAdaptiveMediaSource3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDesiredSeekableWindowSize<Impl: IAdaptiveMediaSource3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDesiredSeekableWindowSize(&*(&value as *const <super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Diagnostics<Impl: IAdaptiveMediaSource3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetCorrelatedTimes<Impl: IAdaptiveMediaSource3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IAdaptiveMediaSource3>,
            ::windows::core::GetTrustLevel,
            MinLiveOffset::<Impl, IMPL_OFFSET>,
            MaxSeekableWindowSize::<Impl, IMPL_OFFSET>,
            DesiredSeekableWindowSize::<Impl, IMPL_OFFSET>,
            SetDesiredSeekableWindowSize::<Impl, IMPL_OFFSET>,
            Diagnostics::<Impl, IMPL_OFFSET>,
            GetCorrelatedTimes::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdaptiveMediaSource3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAdaptiveMediaSourceAdvancedSettingsImpl: Sized {
    fn AllSegmentsIndependent(&self) -> ::windows::core::Result<bool>;
    fn SetAllSegmentsIndependent(&self, value: bool) -> ::windows::core::Result<()>;
    fn DesiredBitrateHeadroomRatio(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<f64>>;
    fn SetDesiredBitrateHeadroomRatio(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<f64>>) -> ::windows::core::Result<()>;
    fn BitrateDowngradeTriggerRatio(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<f64>>;
    fn SetBitrateDowngradeTriggerRatio(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<f64>>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAdaptiveMediaSourceAdvancedSettings {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceAdvancedSettings";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAdaptiveMediaSourceAdvancedSettingsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdaptiveMediaSourceAdvancedSettingsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdaptiveMediaSourceAdvancedSettingsVtbl {
        unsafe extern "system" fn AllSegmentsIndependent<Impl: IAdaptiveMediaSourceAdvancedSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAllSegmentsIndependent<Impl: IAdaptiveMediaSourceAdvancedSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllSegmentsIndependent(value).into()
        }
        unsafe extern "system" fn DesiredBitrateHeadroomRatio<Impl: IAdaptiveMediaSourceAdvancedSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDesiredBitrateHeadroomRatio<Impl: IAdaptiveMediaSourceAdvancedSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDesiredBitrateHeadroomRatio(&*(&value as *const <super::super::super::Foundation::IReference<f64> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<f64> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BitrateDowngradeTriggerRatio<Impl: IAdaptiveMediaSourceAdvancedSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetBitrateDowngradeTriggerRatio<Impl: IAdaptiveMediaSourceAdvancedSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBitrateDowngradeTriggerRatio(&*(&value as *const <super::super::super::Foundation::IReference<f64> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<f64> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IAdaptiveMediaSourceAdvancedSettings>,
            ::windows::core::GetTrustLevel,
            AllSegmentsIndependent::<Impl, IMPL_OFFSET>,
            SetAllSegmentsIndependent::<Impl, IMPL_OFFSET>,
            DesiredBitrateHeadroomRatio::<Impl, IMPL_OFFSET>,
            SetDesiredBitrateHeadroomRatio::<Impl, IMPL_OFFSET>,
            BitrateDowngradeTriggerRatio::<Impl, IMPL_OFFSET>,
            SetBitrateDowngradeTriggerRatio::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdaptiveMediaSourceAdvancedSettings as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAdaptiveMediaSourceCorrelatedTimesImpl: Sized {
    fn Position(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>;
    fn PresentationTimeStamp(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>;
    fn ProgramDateTime(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAdaptiveMediaSourceCorrelatedTimes {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceCorrelatedTimes";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAdaptiveMediaSourceCorrelatedTimesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdaptiveMediaSourceCorrelatedTimesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdaptiveMediaSourceCorrelatedTimesVtbl {
        unsafe extern "system" fn Position<Impl: IAdaptiveMediaSourceCorrelatedTimesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PresentationTimeStamp<Impl: IAdaptiveMediaSourceCorrelatedTimesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ProgramDateTime<Impl: IAdaptiveMediaSourceCorrelatedTimesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAdaptiveMediaSourceCorrelatedTimes>, ::windows::core::GetTrustLevel, Position::<Impl, IMPL_OFFSET>, PresentationTimeStamp::<Impl, IMPL_OFFSET>, ProgramDateTime::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdaptiveMediaSourceCorrelatedTimes as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Web_Http", feature = "implement_exclusive"))]
pub trait IAdaptiveMediaSourceCreationResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<AdaptiveMediaSourceCreationStatus>;
    fn MediaSource(&self) -> ::windows::core::Result<AdaptiveMediaSource>;
    fn HttpResponseMessage(&self) -> ::windows::core::Result<super::super::super::Web::Http::HttpResponseMessage>;
}
#[cfg(all(feature = "Web_Http", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAdaptiveMediaSourceCreationResult {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceCreationResult";
}
#[cfg(all(feature = "Web_Http", feature = "implement_exclusive"))]
impl IAdaptiveMediaSourceCreationResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdaptiveMediaSourceCreationResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdaptiveMediaSourceCreationResultVtbl {
        unsafe extern "system" fn Status<Impl: IAdaptiveMediaSourceCreationResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AdaptiveMediaSourceCreationStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MediaSource<Impl: IAdaptiveMediaSourceCreationResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn HttpResponseMessage<Impl: IAdaptiveMediaSourceCreationResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAdaptiveMediaSourceCreationResult>, ::windows::core::GetTrustLevel, Status::<Impl, IMPL_OFFSET>, MediaSource::<Impl, IMPL_OFFSET>, HttpResponseMessage::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdaptiveMediaSourceCreationResult as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdaptiveMediaSourceCreationResult2Impl: Sized {
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAdaptiveMediaSourceCreationResult2 {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceCreationResult2";
}
#[cfg(feature = "implement_exclusive")]
impl IAdaptiveMediaSourceCreationResult2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdaptiveMediaSourceCreationResult2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdaptiveMediaSourceCreationResult2Vtbl {
        unsafe extern "system" fn ExtendedError<Impl: IAdaptiveMediaSourceCreationResult2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAdaptiveMediaSourceCreationResult2>, ::windows::core::GetTrustLevel, ExtendedError::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdaptiveMediaSourceCreationResult2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAdaptiveMediaSourceDiagnosticAvailableEventArgsImpl: Sized {
    fn DiagnosticType(&self) -> ::windows::core::Result<AdaptiveMediaSourceDiagnosticType>;
    fn RequestId(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<i32>>;
    fn Position(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>;
    fn SegmentId(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u64>>;
    fn ResourceType(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<AdaptiveMediaSourceResourceType>>;
    fn ResourceUri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
    fn ResourceByteRangeOffset(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u64>>;
    fn ResourceByteRangeLength(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u64>>;
    fn Bitrate(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u32>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAdaptiveMediaSourceDiagnosticAvailableEventArgs {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDiagnosticAvailableEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAdaptiveMediaSourceDiagnosticAvailableEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdaptiveMediaSourceDiagnosticAvailableEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdaptiveMediaSourceDiagnosticAvailableEventArgsVtbl {
        unsafe extern "system" fn DiagnosticType<Impl: IAdaptiveMediaSourceDiagnosticAvailableEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AdaptiveMediaSourceDiagnosticType) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RequestId<Impl: IAdaptiveMediaSourceDiagnosticAvailableEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Position<Impl: IAdaptiveMediaSourceDiagnosticAvailableEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SegmentId<Impl: IAdaptiveMediaSourceDiagnosticAvailableEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ResourceType<Impl: IAdaptiveMediaSourceDiagnosticAvailableEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ResourceUri<Impl: IAdaptiveMediaSourceDiagnosticAvailableEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ResourceByteRangeOffset<Impl: IAdaptiveMediaSourceDiagnosticAvailableEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ResourceByteRangeLength<Impl: IAdaptiveMediaSourceDiagnosticAvailableEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Bitrate<Impl: IAdaptiveMediaSourceDiagnosticAvailableEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IAdaptiveMediaSourceDiagnosticAvailableEventArgs>,
            ::windows::core::GetTrustLevel,
            DiagnosticType::<Impl, IMPL_OFFSET>,
            RequestId::<Impl, IMPL_OFFSET>,
            Position::<Impl, IMPL_OFFSET>,
            SegmentId::<Impl, IMPL_OFFSET>,
            ResourceType::<Impl, IMPL_OFFSET>,
            ResourceUri::<Impl, IMPL_OFFSET>,
            ResourceByteRangeOffset::<Impl, IMPL_OFFSET>,
            ResourceByteRangeLength::<Impl, IMPL_OFFSET>,
            Bitrate::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdaptiveMediaSourceDiagnosticAvailableEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdaptiveMediaSourceDiagnosticAvailableEventArgs2Impl: Sized {
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAdaptiveMediaSourceDiagnosticAvailableEventArgs2 {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDiagnosticAvailableEventArgs2";
}
#[cfg(feature = "implement_exclusive")]
impl IAdaptiveMediaSourceDiagnosticAvailableEventArgs2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdaptiveMediaSourceDiagnosticAvailableEventArgs2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdaptiveMediaSourceDiagnosticAvailableEventArgs2Vtbl {
        unsafe extern "system" fn ExtendedError<Impl: IAdaptiveMediaSourceDiagnosticAvailableEventArgs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAdaptiveMediaSourceDiagnosticAvailableEventArgs2>, ::windows::core::GetTrustLevel, ExtendedError::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdaptiveMediaSourceDiagnosticAvailableEventArgs2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAdaptiveMediaSourceDiagnosticAvailableEventArgs3Impl: Sized {
    fn ResourceDuration(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>;
    fn ResourceContentType(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAdaptiveMediaSourceDiagnosticAvailableEventArgs3 {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDiagnosticAvailableEventArgs3";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAdaptiveMediaSourceDiagnosticAvailableEventArgs3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdaptiveMediaSourceDiagnosticAvailableEventArgs3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdaptiveMediaSourceDiagnosticAvailableEventArgs3Vtbl {
        unsafe extern "system" fn ResourceDuration<Impl: IAdaptiveMediaSourceDiagnosticAvailableEventArgs3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ResourceContentType<Impl: IAdaptiveMediaSourceDiagnosticAvailableEventArgs3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAdaptiveMediaSourceDiagnosticAvailableEventArgs3>, ::windows::core::GetTrustLevel, ResourceDuration::<Impl, IMPL_OFFSET>, ResourceContentType::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdaptiveMediaSourceDiagnosticAvailableEventArgs3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAdaptiveMediaSourceDiagnosticsImpl: Sized {
    fn DiagnosticAvailable(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<AdaptiveMediaSourceDiagnostics, AdaptiveMediaSourceDiagnosticAvailableEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveDiagnosticAvailable(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAdaptiveMediaSourceDiagnostics {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDiagnostics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAdaptiveMediaSourceDiagnosticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdaptiveMediaSourceDiagnosticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdaptiveMediaSourceDiagnosticsVtbl {
        unsafe extern "system" fn DiagnosticAvailable<Impl: IAdaptiveMediaSourceDiagnosticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveDiagnosticAvailable<Impl: IAdaptiveMediaSourceDiagnosticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveDiagnosticAvailable(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAdaptiveMediaSourceDiagnostics>, ::windows::core::GetTrustLevel, DiagnosticAvailable::<Impl, IMPL_OFFSET>, RemoveDiagnosticAvailable::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdaptiveMediaSourceDiagnostics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdaptiveMediaSourceDownloadBitrateChangedEventArgsImpl: Sized {
    fn OldValue(&self) -> ::windows::core::Result<u32>;
    fn NewValue(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAdaptiveMediaSourceDownloadBitrateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadBitrateChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IAdaptiveMediaSourceDownloadBitrateChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdaptiveMediaSourceDownloadBitrateChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdaptiveMediaSourceDownloadBitrateChangedEventArgsVtbl {
        unsafe extern "system" fn OldValue<Impl: IAdaptiveMediaSourceDownloadBitrateChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn NewValue<Impl: IAdaptiveMediaSourceDownloadBitrateChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAdaptiveMediaSourceDownloadBitrateChangedEventArgs>, ::windows::core::GetTrustLevel, OldValue::<Impl, IMPL_OFFSET>, NewValue::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdaptiveMediaSourceDownloadBitrateChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdaptiveMediaSourceDownloadBitrateChangedEventArgs2Impl: Sized {
    fn Reason(&self) -> ::windows::core::Result<AdaptiveMediaSourceDownloadBitrateChangedReason>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAdaptiveMediaSourceDownloadBitrateChangedEventArgs2 {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadBitrateChangedEventArgs2";
}
#[cfg(feature = "implement_exclusive")]
impl IAdaptiveMediaSourceDownloadBitrateChangedEventArgs2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdaptiveMediaSourceDownloadBitrateChangedEventArgs2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdaptiveMediaSourceDownloadBitrateChangedEventArgs2Vtbl {
        unsafe extern "system" fn Reason<Impl: IAdaptiveMediaSourceDownloadBitrateChangedEventArgs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AdaptiveMediaSourceDownloadBitrateChangedReason) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAdaptiveMediaSourceDownloadBitrateChangedEventArgs2>, ::windows::core::GetTrustLevel, Reason::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdaptiveMediaSourceDownloadBitrateChangedEventArgs2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Web_Http", feature = "implement_exclusive"))]
pub trait IAdaptiveMediaSourceDownloadCompletedEventArgsImpl: Sized {
    fn ResourceType(&self) -> ::windows::core::Result<AdaptiveMediaSourceResourceType>;
    fn ResourceUri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
    fn ResourceByteRangeOffset(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u64>>;
    fn ResourceByteRangeLength(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u64>>;
    fn HttpResponseMessage(&self) -> ::windows::core::Result<super::super::super::Web::Http::HttpResponseMessage>;
}
#[cfg(all(feature = "Foundation", feature = "Web_Http", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAdaptiveMediaSourceDownloadCompletedEventArgs {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadCompletedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "Web_Http", feature = "implement_exclusive"))]
impl IAdaptiveMediaSourceDownloadCompletedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdaptiveMediaSourceDownloadCompletedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdaptiveMediaSourceDownloadCompletedEventArgsVtbl {
        unsafe extern "system" fn ResourceType<Impl: IAdaptiveMediaSourceDownloadCompletedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AdaptiveMediaSourceResourceType) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ResourceUri<Impl: IAdaptiveMediaSourceDownloadCompletedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ResourceByteRangeOffset<Impl: IAdaptiveMediaSourceDownloadCompletedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ResourceByteRangeLength<Impl: IAdaptiveMediaSourceDownloadCompletedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn HttpResponseMessage<Impl: IAdaptiveMediaSourceDownloadCompletedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IAdaptiveMediaSourceDownloadCompletedEventArgs>,
            ::windows::core::GetTrustLevel,
            ResourceType::<Impl, IMPL_OFFSET>,
            ResourceUri::<Impl, IMPL_OFFSET>,
            ResourceByteRangeOffset::<Impl, IMPL_OFFSET>,
            ResourceByteRangeLength::<Impl, IMPL_OFFSET>,
            HttpResponseMessage::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdaptiveMediaSourceDownloadCompletedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAdaptiveMediaSourceDownloadCompletedEventArgs2Impl: Sized {
    fn RequestId(&self) -> ::windows::core::Result<i32>;
    fn Statistics(&self) -> ::windows::core::Result<AdaptiveMediaSourceDownloadStatistics>;
    fn Position(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAdaptiveMediaSourceDownloadCompletedEventArgs2 {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadCompletedEventArgs2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAdaptiveMediaSourceDownloadCompletedEventArgs2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdaptiveMediaSourceDownloadCompletedEventArgs2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdaptiveMediaSourceDownloadCompletedEventArgs2Vtbl {
        unsafe extern "system" fn RequestId<Impl: IAdaptiveMediaSourceDownloadCompletedEventArgs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Statistics<Impl: IAdaptiveMediaSourceDownloadCompletedEventArgs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Position<Impl: IAdaptiveMediaSourceDownloadCompletedEventArgs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAdaptiveMediaSourceDownloadCompletedEventArgs2>, ::windows::core::GetTrustLevel, RequestId::<Impl, IMPL_OFFSET>, Statistics::<Impl, IMPL_OFFSET>, Position::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdaptiveMediaSourceDownloadCompletedEventArgs2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAdaptiveMediaSourceDownloadCompletedEventArgs3Impl: Sized {
    fn ResourceDuration(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>;
    fn ResourceContentType(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAdaptiveMediaSourceDownloadCompletedEventArgs3 {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadCompletedEventArgs3";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAdaptiveMediaSourceDownloadCompletedEventArgs3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdaptiveMediaSourceDownloadCompletedEventArgs3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdaptiveMediaSourceDownloadCompletedEventArgs3Vtbl {
        unsafe extern "system" fn ResourceDuration<Impl: IAdaptiveMediaSourceDownloadCompletedEventArgs3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ResourceContentType<Impl: IAdaptiveMediaSourceDownloadCompletedEventArgs3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAdaptiveMediaSourceDownloadCompletedEventArgs3>, ::windows::core::GetTrustLevel, ResourceDuration::<Impl, IMPL_OFFSET>, ResourceContentType::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdaptiveMediaSourceDownloadCompletedEventArgs3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Web_Http", feature = "implement_exclusive"))]
pub trait IAdaptiveMediaSourceDownloadFailedEventArgsImpl: Sized {
    fn ResourceType(&self) -> ::windows::core::Result<AdaptiveMediaSourceResourceType>;
    fn ResourceUri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
    fn ResourceByteRangeOffset(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u64>>;
    fn ResourceByteRangeLength(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u64>>;
    fn HttpResponseMessage(&self) -> ::windows::core::Result<super::super::super::Web::Http::HttpResponseMessage>;
}
#[cfg(all(feature = "Foundation", feature = "Web_Http", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAdaptiveMediaSourceDownloadFailedEventArgs {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadFailedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "Web_Http", feature = "implement_exclusive"))]
impl IAdaptiveMediaSourceDownloadFailedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdaptiveMediaSourceDownloadFailedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdaptiveMediaSourceDownloadFailedEventArgsVtbl {
        unsafe extern "system" fn ResourceType<Impl: IAdaptiveMediaSourceDownloadFailedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AdaptiveMediaSourceResourceType) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ResourceUri<Impl: IAdaptiveMediaSourceDownloadFailedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ResourceByteRangeOffset<Impl: IAdaptiveMediaSourceDownloadFailedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ResourceByteRangeLength<Impl: IAdaptiveMediaSourceDownloadFailedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn HttpResponseMessage<Impl: IAdaptiveMediaSourceDownloadFailedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IAdaptiveMediaSourceDownloadFailedEventArgs>,
            ::windows::core::GetTrustLevel,
            ResourceType::<Impl, IMPL_OFFSET>,
            ResourceUri::<Impl, IMPL_OFFSET>,
            ResourceByteRangeOffset::<Impl, IMPL_OFFSET>,
            ResourceByteRangeLength::<Impl, IMPL_OFFSET>,
            HttpResponseMessage::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdaptiveMediaSourceDownloadFailedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAdaptiveMediaSourceDownloadFailedEventArgs2Impl: Sized {
    fn RequestId(&self) -> ::windows::core::Result<i32>;
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn Statistics(&self) -> ::windows::core::Result<AdaptiveMediaSourceDownloadStatistics>;
    fn Position(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAdaptiveMediaSourceDownloadFailedEventArgs2 {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadFailedEventArgs2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAdaptiveMediaSourceDownloadFailedEventArgs2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdaptiveMediaSourceDownloadFailedEventArgs2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdaptiveMediaSourceDownloadFailedEventArgs2Vtbl {
        unsafe extern "system" fn RequestId<Impl: IAdaptiveMediaSourceDownloadFailedEventArgs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ExtendedError<Impl: IAdaptiveMediaSourceDownloadFailedEventArgs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Statistics<Impl: IAdaptiveMediaSourceDownloadFailedEventArgs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Position<Impl: IAdaptiveMediaSourceDownloadFailedEventArgs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAdaptiveMediaSourceDownloadFailedEventArgs2>, ::windows::core::GetTrustLevel, RequestId::<Impl, IMPL_OFFSET>, ExtendedError::<Impl, IMPL_OFFSET>, Statistics::<Impl, IMPL_OFFSET>, Position::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdaptiveMediaSourceDownloadFailedEventArgs2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAdaptiveMediaSourceDownloadFailedEventArgs3Impl: Sized {
    fn ResourceDuration(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>;
    fn ResourceContentType(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAdaptiveMediaSourceDownloadFailedEventArgs3 {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadFailedEventArgs3";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAdaptiveMediaSourceDownloadFailedEventArgs3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdaptiveMediaSourceDownloadFailedEventArgs3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdaptiveMediaSourceDownloadFailedEventArgs3Vtbl {
        unsafe extern "system" fn ResourceDuration<Impl: IAdaptiveMediaSourceDownloadFailedEventArgs3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ResourceContentType<Impl: IAdaptiveMediaSourceDownloadFailedEventArgs3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAdaptiveMediaSourceDownloadFailedEventArgs3>, ::windows::core::GetTrustLevel, ResourceDuration::<Impl, IMPL_OFFSET>, ResourceContentType::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdaptiveMediaSourceDownloadFailedEventArgs3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdaptiveMediaSourceDownloadRequestedDeferralImpl: Sized {
    fn Complete(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAdaptiveMediaSourceDownloadRequestedDeferral {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadRequestedDeferral";
}
#[cfg(feature = "implement_exclusive")]
impl IAdaptiveMediaSourceDownloadRequestedDeferralVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdaptiveMediaSourceDownloadRequestedDeferralImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdaptiveMediaSourceDownloadRequestedDeferralVtbl {
        unsafe extern "system" fn Complete<Impl: IAdaptiveMediaSourceDownloadRequestedDeferralImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Complete().into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAdaptiveMediaSourceDownloadRequestedDeferral>, ::windows::core::GetTrustLevel, Complete::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdaptiveMediaSourceDownloadRequestedDeferral as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAdaptiveMediaSourceDownloadRequestedEventArgsImpl: Sized {
    fn ResourceType(&self) -> ::windows::core::Result<AdaptiveMediaSourceResourceType>;
    fn ResourceUri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
    fn ResourceByteRangeOffset(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u64>>;
    fn ResourceByteRangeLength(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u64>>;
    fn Result(&self) -> ::windows::core::Result<AdaptiveMediaSourceDownloadResult>;
    fn GetDeferral(&self) -> ::windows::core::Result<AdaptiveMediaSourceDownloadRequestedDeferral>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAdaptiveMediaSourceDownloadRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadRequestedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAdaptiveMediaSourceDownloadRequestedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdaptiveMediaSourceDownloadRequestedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdaptiveMediaSourceDownloadRequestedEventArgsVtbl {
        unsafe extern "system" fn ResourceType<Impl: IAdaptiveMediaSourceDownloadRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AdaptiveMediaSourceResourceType) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ResourceUri<Impl: IAdaptiveMediaSourceDownloadRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ResourceByteRangeOffset<Impl: IAdaptiveMediaSourceDownloadRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ResourceByteRangeLength<Impl: IAdaptiveMediaSourceDownloadRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Result<Impl: IAdaptiveMediaSourceDownloadRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDeferral<Impl: IAdaptiveMediaSourceDownloadRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IAdaptiveMediaSourceDownloadRequestedEventArgs>,
            ::windows::core::GetTrustLevel,
            ResourceType::<Impl, IMPL_OFFSET>,
            ResourceUri::<Impl, IMPL_OFFSET>,
            ResourceByteRangeOffset::<Impl, IMPL_OFFSET>,
            ResourceByteRangeLength::<Impl, IMPL_OFFSET>,
            Result::<Impl, IMPL_OFFSET>,
            GetDeferral::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdaptiveMediaSourceDownloadRequestedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAdaptiveMediaSourceDownloadRequestedEventArgs2Impl: Sized {
    fn RequestId(&self) -> ::windows::core::Result<i32>;
    fn Position(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAdaptiveMediaSourceDownloadRequestedEventArgs2 {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadRequestedEventArgs2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAdaptiveMediaSourceDownloadRequestedEventArgs2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdaptiveMediaSourceDownloadRequestedEventArgs2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdaptiveMediaSourceDownloadRequestedEventArgs2Vtbl {
        unsafe extern "system" fn RequestId<Impl: IAdaptiveMediaSourceDownloadRequestedEventArgs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Position<Impl: IAdaptiveMediaSourceDownloadRequestedEventArgs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAdaptiveMediaSourceDownloadRequestedEventArgs2>, ::windows::core::GetTrustLevel, RequestId::<Impl, IMPL_OFFSET>, Position::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdaptiveMediaSourceDownloadRequestedEventArgs2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAdaptiveMediaSourceDownloadRequestedEventArgs3Impl: Sized {
    fn ResourceDuration(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>;
    fn ResourceContentType(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAdaptiveMediaSourceDownloadRequestedEventArgs3 {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadRequestedEventArgs3";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAdaptiveMediaSourceDownloadRequestedEventArgs3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdaptiveMediaSourceDownloadRequestedEventArgs3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdaptiveMediaSourceDownloadRequestedEventArgs3Vtbl {
        unsafe extern "system" fn ResourceDuration<Impl: IAdaptiveMediaSourceDownloadRequestedEventArgs3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ResourceContentType<Impl: IAdaptiveMediaSourceDownloadRequestedEventArgs3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAdaptiveMediaSourceDownloadRequestedEventArgs3>, ::windows::core::GetTrustLevel, ResourceDuration::<Impl, IMPL_OFFSET>, ResourceContentType::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdaptiveMediaSourceDownloadRequestedEventArgs3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IAdaptiveMediaSourceDownloadResultImpl: Sized {
    fn ResourceUri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
    fn SetResourceUri(&self, value: &::core::option::Option<super::super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn InputStream(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IInputStream>;
    fn SetInputStream(&self, value: &::core::option::Option<super::super::super::Storage::Streams::IInputStream>) -> ::windows::core::Result<()>;
    fn Buffer(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
    fn SetBuffer(&self, value: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
    fn ContentType(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetContentType(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ExtendedStatus(&self) -> ::windows::core::Result<u32>;
    fn SetExtendedStatus(&self, value: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAdaptiveMediaSourceDownloadResult {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadResult";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IAdaptiveMediaSourceDownloadResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdaptiveMediaSourceDownloadResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdaptiveMediaSourceDownloadResultVtbl {
        unsafe extern "system" fn ResourceUri<Impl: IAdaptiveMediaSourceDownloadResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetResourceUri<Impl: IAdaptiveMediaSourceDownloadResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetResourceUri(&*(&value as *const <super::super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InputStream<Impl: IAdaptiveMediaSourceDownloadResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetInputStream<Impl: IAdaptiveMediaSourceDownloadResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInputStream(&*(&value as *const <super::super::super::Storage::Streams::IInputStream as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IInputStream as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Buffer<Impl: IAdaptiveMediaSourceDownloadResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetBuffer<Impl: IAdaptiveMediaSourceDownloadResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBuffer(&*(&value as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ContentType<Impl: IAdaptiveMediaSourceDownloadResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetContentType<Impl: IAdaptiveMediaSourceDownloadResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContentType(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ExtendedStatus<Impl: IAdaptiveMediaSourceDownloadResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetExtendedStatus<Impl: IAdaptiveMediaSourceDownloadResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExtendedStatus(value).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IAdaptiveMediaSourceDownloadResult>,
            ::windows::core::GetTrustLevel,
            ResourceUri::<Impl, IMPL_OFFSET>,
            SetResourceUri::<Impl, IMPL_OFFSET>,
            InputStream::<Impl, IMPL_OFFSET>,
            SetInputStream::<Impl, IMPL_OFFSET>,
            Buffer::<Impl, IMPL_OFFSET>,
            SetBuffer::<Impl, IMPL_OFFSET>,
            ContentType::<Impl, IMPL_OFFSET>,
            SetContentType::<Impl, IMPL_OFFSET>,
            ExtendedStatus::<Impl, IMPL_OFFSET>,
            SetExtendedStatus::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdaptiveMediaSourceDownloadResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAdaptiveMediaSourceDownloadResult2Impl: Sized {
    fn ResourceByteRangeOffset(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u64>>;
    fn SetResourceByteRangeOffset(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<u64>>) -> ::windows::core::Result<()>;
    fn ResourceByteRangeLength(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u64>>;
    fn SetResourceByteRangeLength(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<u64>>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAdaptiveMediaSourceDownloadResult2 {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadResult2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAdaptiveMediaSourceDownloadResult2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdaptiveMediaSourceDownloadResult2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdaptiveMediaSourceDownloadResult2Vtbl {
        unsafe extern "system" fn ResourceByteRangeOffset<Impl: IAdaptiveMediaSourceDownloadResult2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetResourceByteRangeOffset<Impl: IAdaptiveMediaSourceDownloadResult2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetResourceByteRangeOffset(&*(&value as *const <super::super::super::Foundation::IReference<u64> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<u64> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ResourceByteRangeLength<Impl: IAdaptiveMediaSourceDownloadResult2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetResourceByteRangeLength<Impl: IAdaptiveMediaSourceDownloadResult2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetResourceByteRangeLength(&*(&value as *const <super::super::super::Foundation::IReference<u64> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<u64> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IAdaptiveMediaSourceDownloadResult2>,
            ::windows::core::GetTrustLevel,
            ResourceByteRangeOffset::<Impl, IMPL_OFFSET>,
            SetResourceByteRangeOffset::<Impl, IMPL_OFFSET>,
            ResourceByteRangeLength::<Impl, IMPL_OFFSET>,
            SetResourceByteRangeLength::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdaptiveMediaSourceDownloadResult2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAdaptiveMediaSourceDownloadStatisticsImpl: Sized {
    fn ContentBytesReceivedCount(&self) -> ::windows::core::Result<u64>;
    fn TimeToHeadersReceived(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>;
    fn TimeToFirstByteReceived(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>;
    fn TimeToLastByteReceived(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAdaptiveMediaSourceDownloadStatistics {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadStatistics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAdaptiveMediaSourceDownloadStatisticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdaptiveMediaSourceDownloadStatisticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdaptiveMediaSourceDownloadStatisticsVtbl {
        unsafe extern "system" fn ContentBytesReceivedCount<Impl: IAdaptiveMediaSourceDownloadStatisticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TimeToHeadersReceived<Impl: IAdaptiveMediaSourceDownloadStatisticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TimeToFirstByteReceived<Impl: IAdaptiveMediaSourceDownloadStatisticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TimeToLastByteReceived<Impl: IAdaptiveMediaSourceDownloadStatisticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAdaptiveMediaSourceDownloadStatistics>, ::windows::core::GetTrustLevel, ContentBytesReceivedCount::<Impl, IMPL_OFFSET>, TimeToHeadersReceived::<Impl, IMPL_OFFSET>, TimeToFirstByteReceived::<Impl, IMPL_OFFSET>, TimeToLastByteReceived::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdaptiveMediaSourceDownloadStatistics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdaptiveMediaSourcePlaybackBitrateChangedEventArgsImpl: Sized {
    fn OldValue(&self) -> ::windows::core::Result<u32>;
    fn NewValue(&self) -> ::windows::core::Result<u32>;
    fn AudioOnly(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAdaptiveMediaSourcePlaybackBitrateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourcePlaybackBitrateChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IAdaptiveMediaSourcePlaybackBitrateChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdaptiveMediaSourcePlaybackBitrateChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdaptiveMediaSourcePlaybackBitrateChangedEventArgsVtbl {
        unsafe extern "system" fn OldValue<Impl: IAdaptiveMediaSourcePlaybackBitrateChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn NewValue<Impl: IAdaptiveMediaSourcePlaybackBitrateChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AudioOnly<Impl: IAdaptiveMediaSourcePlaybackBitrateChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAdaptiveMediaSourcePlaybackBitrateChangedEventArgs>, ::windows::core::GetTrustLevel, OldValue::<Impl, IMPL_OFFSET>, NewValue::<Impl, IMPL_OFFSET>, AudioOnly::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdaptiveMediaSourcePlaybackBitrateChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "Web_Http", feature = "implement_exclusive"))]
pub trait IAdaptiveMediaSourceStaticsImpl: Sized {
    fn IsContentTypeSupported(&self, contenttype: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
    fn CreateFromUriAsync(&self, uri: &::core::option::Option<super::super::super::Foundation::Uri>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<AdaptiveMediaSourceCreationResult>>;
    fn CreateFromUriWithDownloaderAsync(&self, uri: &::core::option::Option<super::super::super::Foundation::Uri>, httpclient: &::core::option::Option<super::super::super::Web::Http::HttpClient>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<AdaptiveMediaSourceCreationResult>>;
    fn CreateFromStreamAsync(&self, stream: &::core::option::Option<super::super::super::Storage::Streams::IInputStream>, uri: &::core::option::Option<super::super::super::Foundation::Uri>, contenttype: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<AdaptiveMediaSourceCreationResult>>;
    fn CreateFromStreamWithDownloaderAsync(&self, stream: &::core::option::Option<super::super::super::Storage::Streams::IInputStream>, uri: &::core::option::Option<super::super::super::Foundation::Uri>, contenttype: &::windows::core::HSTRING, httpclient: &::core::option::Option<super::super::super::Web::Http::HttpClient>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<AdaptiveMediaSourceCreationResult>>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "Web_Http", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAdaptiveMediaSourceStatics {
    const NAME: &'static str = "Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceStatics";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "Web_Http", feature = "implement_exclusive"))]
impl IAdaptiveMediaSourceStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdaptiveMediaSourceStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdaptiveMediaSourceStaticsVtbl {
        unsafe extern "system" fn IsContentTypeSupported<Impl: IAdaptiveMediaSourceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contenttype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateFromUriAsync<Impl: IAdaptiveMediaSourceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateFromUriWithDownloaderAsync<Impl: IAdaptiveMediaSourceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, httpclient: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateFromStreamAsync<Impl: IAdaptiveMediaSourceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, uri: ::windows::core::RawPtr, contenttype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateFromStreamWithDownloaderAsync<Impl: IAdaptiveMediaSourceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, uri: ::windows::core::RawPtr, contenttype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, httpclient: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IAdaptiveMediaSourceStatics>,
            ::windows::core::GetTrustLevel,
            IsContentTypeSupported::<Impl, IMPL_OFFSET>,
            CreateFromUriAsync::<Impl, IMPL_OFFSET>,
            CreateFromUriWithDownloaderAsync::<Impl, IMPL_OFFSET>,
            CreateFromStreamAsync::<Impl, IMPL_OFFSET>,
            CreateFromStreamWithDownloaderAsync::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdaptiveMediaSourceStatics as ::windows::core::Interface>::IID
    }
}
