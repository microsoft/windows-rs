#[cfg(feature = "implement_exclusive")]
pub trait IAudioMediaFrameImpl: Sized {
    fn FrameReference(&self) -> ::windows::core::Result<MediaFrameReference>;
    fn AudioEncodingProperties(&self) -> ::windows::core::Result<super::super::MediaProperties::AudioEncodingProperties>;
    fn GetAudioFrame(&self) -> ::windows::core::Result<super::super::AudioFrame>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAudioMediaFrame {
    const NAME: &'static str = "Windows.Media.Capture.Frames.IAudioMediaFrame";
}
#[cfg(feature = "implement_exclusive")]
impl IAudioMediaFrameVtbl {
    pub const fn new<Impl: IAudioMediaFrameImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAudioMediaFrameVtbl {
        unsafe extern "system" fn FrameReference<Impl: IAudioMediaFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FrameReference() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioEncodingProperties<Impl: IAudioMediaFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AudioEncodingProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAudioFrame<Impl: IAudioMediaFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAudioFrame() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAudioMediaFrame>, base.5, FrameReference::<Impl, OFFSET>, AudioEncodingProperties::<Impl, OFFSET>, GetAudioFrame::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBufferMediaFrameImpl: Sized {
    fn FrameReference(&self) -> ::windows::core::Result<MediaFrameReference>;
    fn Buffer(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBufferMediaFrame {
    const NAME: &'static str = "Windows.Media.Capture.Frames.IBufferMediaFrame";
}
#[cfg(feature = "implement_exclusive")]
impl IBufferMediaFrameVtbl {
    pub const fn new<Impl: IBufferMediaFrameImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBufferMediaFrameVtbl {
        unsafe extern "system" fn FrameReference<Impl: IBufferMediaFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FrameReference() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Buffer<Impl: IBufferMediaFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Buffer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBufferMediaFrame>, base.5, FrameReference::<Impl, OFFSET>, Buffer::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDepthMediaFrameImpl: Sized {
    fn FrameReference(&self) -> ::windows::core::Result<MediaFrameReference>;
    fn VideoMediaFrame(&self) -> ::windows::core::Result<VideoMediaFrame>;
    fn DepthFormat(&self) -> ::windows::core::Result<DepthMediaFrameFormat>;
    fn TryCreateCoordinateMapper(&self, cameraintrinsics: &::core::option::Option<super::super::Devices::Core::CameraIntrinsics>, coordinatesystem: &::core::option::Option<super::super::super::Perception::Spatial::SpatialCoordinateSystem>) -> ::windows::core::Result<super::super::Devices::Core::DepthCorrelatedCoordinateMapper>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDepthMediaFrame {
    const NAME: &'static str = "Windows.Media.Capture.Frames.IDepthMediaFrame";
}
#[cfg(feature = "implement_exclusive")]
impl IDepthMediaFrameVtbl {
    pub const fn new<Impl: IDepthMediaFrameImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDepthMediaFrameVtbl {
        unsafe extern "system" fn FrameReference<Impl: IDepthMediaFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FrameReference() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoMediaFrame<Impl: IDepthMediaFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VideoMediaFrame() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DepthFormat<Impl: IDepthMediaFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DepthFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryCreateCoordinateMapper<Impl: IDepthMediaFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cameraintrinsics: ::windows::core::RawPtr, coordinatesystem: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryCreateCoordinateMapper(&*(&cameraintrinsics as *const <super::super::Devices::Core::CameraIntrinsics as ::windows::core::Abi>::Abi as *const <super::super::Devices::Core::CameraIntrinsics as ::windows::core::DefaultType>::DefaultType), &*(&coordinatesystem as *const <super::super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::Abi>::Abi as *const <super::super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDepthMediaFrame>, base.5, FrameReference::<Impl, OFFSET>, VideoMediaFrame::<Impl, OFFSET>, DepthFormat::<Impl, OFFSET>, TryCreateCoordinateMapper::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDepthMediaFrame2Impl: Sized {
    fn MaxReliableDepth(&self) -> ::windows::core::Result<u32>;
    fn MinReliableDepth(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDepthMediaFrame2 {
    const NAME: &'static str = "Windows.Media.Capture.Frames.IDepthMediaFrame2";
}
#[cfg(feature = "implement_exclusive")]
impl IDepthMediaFrame2Vtbl {
    pub const fn new<Impl: IDepthMediaFrame2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDepthMediaFrame2Vtbl {
        unsafe extern "system" fn MaxReliableDepth<Impl: IDepthMediaFrame2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MaxReliableDepth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinReliableDepth<Impl: IDepthMediaFrame2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MinReliableDepth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDepthMediaFrame2>, base.5, MaxReliableDepth::<Impl, OFFSET>, MinReliableDepth::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDepthMediaFrameFormatImpl: Sized {
    fn VideoFormat(&self) -> ::windows::core::Result<VideoMediaFrameFormat>;
    fn DepthScaleInMeters(&self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDepthMediaFrameFormat {
    const NAME: &'static str = "Windows.Media.Capture.Frames.IDepthMediaFrameFormat";
}
#[cfg(feature = "implement_exclusive")]
impl IDepthMediaFrameFormatVtbl {
    pub const fn new<Impl: IDepthMediaFrameFormatImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDepthMediaFrameFormatVtbl {
        unsafe extern "system" fn VideoFormat<Impl: IDepthMediaFrameFormatImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VideoFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DepthScaleInMeters<Impl: IDepthMediaFrameFormatImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DepthScaleInMeters() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDepthMediaFrameFormat>, base.5, VideoFormat::<Impl, OFFSET>, DepthScaleInMeters::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInfraredMediaFrameImpl: Sized {
    fn FrameReference(&self) -> ::windows::core::Result<MediaFrameReference>;
    fn VideoMediaFrame(&self) -> ::windows::core::Result<VideoMediaFrame>;
    fn IsIlluminated(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInfraredMediaFrame {
    const NAME: &'static str = "Windows.Media.Capture.Frames.IInfraredMediaFrame";
}
#[cfg(feature = "implement_exclusive")]
impl IInfraredMediaFrameVtbl {
    pub const fn new<Impl: IInfraredMediaFrameImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IInfraredMediaFrameVtbl {
        unsafe extern "system" fn FrameReference<Impl: IInfraredMediaFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FrameReference() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoMediaFrame<Impl: IInfraredMediaFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VideoMediaFrame() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsIlluminated<Impl: IInfraredMediaFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsIlluminated() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IInfraredMediaFrame>, base.5, FrameReference::<Impl, OFFSET>, VideoMediaFrame::<Impl, OFFSET>, IsIlluminated::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaFrameArrivedEventArgsImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaFrameArrivedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.Frames.IMediaFrameArrivedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaFrameArrivedEventArgsVtbl {
    pub const fn new<Impl: IMediaFrameArrivedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaFrameArrivedEventArgsVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaFrameArrivedEventArgs>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaFrameFormatImpl: Sized {
    fn MajorType(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Subtype(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FrameRate(&self) -> ::windows::core::Result<super::super::MediaProperties::MediaRatio>;
    fn Properties(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::GUID, ::windows::core::IInspectable>>;
    fn VideoFormat(&self) -> ::windows::core::Result<VideoMediaFrameFormat>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaFrameFormat {
    const NAME: &'static str = "Windows.Media.Capture.Frames.IMediaFrameFormat";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaFrameFormatVtbl {
    pub const fn new<Impl: IMediaFrameFormatImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaFrameFormatVtbl {
        unsafe extern "system" fn MajorType<Impl: IMediaFrameFormatImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MajorType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Subtype<Impl: IMediaFrameFormatImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Subtype() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FrameRate<Impl: IMediaFrameFormatImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FrameRate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IMediaFrameFormatImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoFormat<Impl: IMediaFrameFormatImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VideoFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaFrameFormat>, base.5, MajorType::<Impl, OFFSET>, Subtype::<Impl, OFFSET>, FrameRate::<Impl, OFFSET>, Properties::<Impl, OFFSET>, VideoFormat::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaFrameFormat2Impl: Sized {
    fn AudioEncodingProperties(&self) -> ::windows::core::Result<super::super::MediaProperties::AudioEncodingProperties>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaFrameFormat2 {
    const NAME: &'static str = "Windows.Media.Capture.Frames.IMediaFrameFormat2";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaFrameFormat2Vtbl {
    pub const fn new<Impl: IMediaFrameFormat2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaFrameFormat2Vtbl {
        unsafe extern "system" fn AudioEncodingProperties<Impl: IMediaFrameFormat2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AudioEncodingProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaFrameFormat2>, base.5, AudioEncodingProperties::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMediaFrameReaderImpl: Sized + IClosableImpl {
    fn FrameArrived(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<MediaFrameReader, MediaFrameArrivedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveFrameArrived(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn TryAcquireLatestFrame(&self) -> ::windows::core::Result<MediaFrameReference>;
    fn StartAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<MediaFrameReaderStartStatus>>;
    fn StopAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaFrameReader {
    const NAME: &'static str = "Windows.Media.Capture.Frames.IMediaFrameReader";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMediaFrameReaderVtbl {
    pub const fn new<Impl: IMediaFrameReaderImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaFrameReaderVtbl {
        unsafe extern "system" fn FrameArrived<Impl: IMediaFrameReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FrameArrived(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<MediaFrameReader, MediaFrameArrivedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<MediaFrameReader, MediaFrameArrivedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveFrameArrived<Impl: IMediaFrameReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveFrameArrived(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TryAcquireLatestFrame<Impl: IMediaFrameReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryAcquireLatestFrame() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartAsync<Impl: IMediaFrameReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StartAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StopAsync<Impl: IMediaFrameReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StopAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaFrameReader>, base.5, FrameArrived::<Impl, OFFSET>, RemoveFrameArrived::<Impl, OFFSET>, TryAcquireLatestFrame::<Impl, OFFSET>, StartAsync::<Impl, OFFSET>, StopAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaFrameReader2Impl: Sized {
    fn SetAcquisitionMode(&self, value: MediaFrameReaderAcquisitionMode) -> ::windows::core::Result<()>;
    fn AcquisitionMode(&self) -> ::windows::core::Result<MediaFrameReaderAcquisitionMode>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaFrameReader2 {
    const NAME: &'static str = "Windows.Media.Capture.Frames.IMediaFrameReader2";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaFrameReader2Vtbl {
    pub const fn new<Impl: IMediaFrameReader2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaFrameReader2Vtbl {
        unsafe extern "system" fn SetAcquisitionMode<Impl: IMediaFrameReader2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: MediaFrameReaderAcquisitionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAcquisitionMode(value).into()
        }
        unsafe extern "system" fn AcquisitionMode<Impl: IMediaFrameReader2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut MediaFrameReaderAcquisitionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AcquisitionMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaFrameReader2>, base.5, SetAcquisitionMode::<Impl, OFFSET>, AcquisitionMode::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMediaFrameReferenceImpl: Sized + IClosableImpl {
    fn SourceKind(&self) -> ::windows::core::Result<MediaFrameSourceKind>;
    fn Format(&self) -> ::windows::core::Result<MediaFrameFormat>;
    fn SystemRelativeTime(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>;
    fn Duration(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn Properties(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::GUID, ::windows::core::IInspectable>>;
    fn BufferMediaFrame(&self) -> ::windows::core::Result<BufferMediaFrame>;
    fn VideoMediaFrame(&self) -> ::windows::core::Result<VideoMediaFrame>;
    fn CoordinateSystem(&self) -> ::windows::core::Result<super::super::super::Perception::Spatial::SpatialCoordinateSystem>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaFrameReference {
    const NAME: &'static str = "Windows.Media.Capture.Frames.IMediaFrameReference";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMediaFrameReferenceVtbl {
    pub const fn new<Impl: IMediaFrameReferenceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaFrameReferenceVtbl {
        unsafe extern "system" fn SourceKind<Impl: IMediaFrameReferenceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut MediaFrameSourceKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SourceKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Format<Impl: IMediaFrameReferenceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Format() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SystemRelativeTime<Impl: IMediaFrameReferenceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SystemRelativeTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Duration<Impl: IMediaFrameReferenceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Duration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IMediaFrameReferenceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BufferMediaFrame<Impl: IMediaFrameReferenceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BufferMediaFrame() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoMediaFrame<Impl: IMediaFrameReferenceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VideoMediaFrame() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CoordinateSystem<Impl: IMediaFrameReferenceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CoordinateSystem() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaFrameReference>, base.5, SourceKind::<Impl, OFFSET>, Format::<Impl, OFFSET>, SystemRelativeTime::<Impl, OFFSET>, Duration::<Impl, OFFSET>, Properties::<Impl, OFFSET>, BufferMediaFrame::<Impl, OFFSET>, VideoMediaFrame::<Impl, OFFSET>, CoordinateSystem::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaFrameReference2Impl: Sized {
    fn AudioMediaFrame(&self) -> ::windows::core::Result<AudioMediaFrame>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaFrameReference2 {
    const NAME: &'static str = "Windows.Media.Capture.Frames.IMediaFrameReference2";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaFrameReference2Vtbl {
    pub const fn new<Impl: IMediaFrameReference2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaFrameReference2Vtbl {
        unsafe extern "system" fn AudioMediaFrame<Impl: IMediaFrameReference2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AudioMediaFrame() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaFrameReference2>, base.5, AudioMediaFrame::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaFrameSourceImpl: Sized {
    fn Info(&self) -> ::windows::core::Result<MediaFrameSourceInfo>;
    fn Controller(&self) -> ::windows::core::Result<MediaFrameSourceController>;
    fn SupportedFormats(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<MediaFrameFormat>>;
    fn CurrentFormat(&self) -> ::windows::core::Result<MediaFrameFormat>;
    fn SetFormatAsync(&self, format: &::core::option::Option<MediaFrameFormat>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn FormatChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<MediaFrameSource, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveFormatChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn TryGetCameraIntrinsics(&self, format: &::core::option::Option<MediaFrameFormat>) -> ::windows::core::Result<super::super::Devices::Core::CameraIntrinsics>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaFrameSource {
    const NAME: &'static str = "Windows.Media.Capture.Frames.IMediaFrameSource";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaFrameSourceVtbl {
    pub const fn new<Impl: IMediaFrameSourceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaFrameSourceVtbl {
        unsafe extern "system" fn Info<Impl: IMediaFrameSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Info() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Controller<Impl: IMediaFrameSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Controller() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedFormats<Impl: IMediaFrameSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SupportedFormats() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentFormat<Impl: IMediaFrameSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CurrentFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormatAsync<Impl: IMediaFrameSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, format: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetFormatAsync(&*(&format as *const <MediaFrameFormat as ::windows::core::Abi>::Abi as *const <MediaFrameFormat as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FormatChanged<Impl: IMediaFrameSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FormatChanged(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<MediaFrameSource, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<MediaFrameSource, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveFormatChanged<Impl: IMediaFrameSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveFormatChanged(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TryGetCameraIntrinsics<Impl: IMediaFrameSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, format: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryGetCameraIntrinsics(&*(&format as *const <MediaFrameFormat as ::windows::core::Abi>::Abi as *const <MediaFrameFormat as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaFrameSource>, base.5, Info::<Impl, OFFSET>, Controller::<Impl, OFFSET>, SupportedFormats::<Impl, OFFSET>, CurrentFormat::<Impl, OFFSET>, SetFormatAsync::<Impl, OFFSET>, FormatChanged::<Impl, OFFSET>, RemoveFormatChanged::<Impl, OFFSET>, TryGetCameraIntrinsics::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaFrameSourceControllerImpl: Sized {
    fn GetPropertyAsync(&self, propertyid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<MediaFrameSourceGetPropertyResult>>;
    fn SetPropertyAsync(&self, propertyid: &::windows::core::HSTRING, propertyvalue: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<MediaFrameSourceSetPropertyStatus>>;
    fn VideoDeviceController(&self) -> ::windows::core::Result<super::super::Devices::VideoDeviceController>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaFrameSourceController {
    const NAME: &'static str = "Windows.Media.Capture.Frames.IMediaFrameSourceController";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaFrameSourceControllerVtbl {
    pub const fn new<Impl: IMediaFrameSourceControllerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaFrameSourceControllerVtbl {
        unsafe extern "system" fn GetPropertyAsync<Impl: IMediaFrameSourceControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propertyid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPropertyAsync(&*(&propertyid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPropertyAsync<Impl: IMediaFrameSourceControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propertyid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, propertyvalue: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetPropertyAsync(&*(&propertyid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&propertyvalue as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoDeviceController<Impl: IMediaFrameSourceControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VideoDeviceController() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaFrameSourceController>, base.5, GetPropertyAsync::<Impl, OFFSET>, SetPropertyAsync::<Impl, OFFSET>, VideoDeviceController::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaFrameSourceController2Impl: Sized {
    fn GetPropertyByExtendedIdAsync(&self, extendedpropertyid: &[<u8 as ::windows::core::DefaultType>::DefaultType], maxpropertyvaluesize: &::core::option::Option<super::super::super::Foundation::IReference<u32>>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<MediaFrameSourceGetPropertyResult>>;
    fn SetPropertyByExtendedIdAsync(&self, extendedpropertyid: &[<u8 as ::windows::core::DefaultType>::DefaultType], propertyvalue: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<MediaFrameSourceSetPropertyStatus>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaFrameSourceController2 {
    const NAME: &'static str = "Windows.Media.Capture.Frames.IMediaFrameSourceController2";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaFrameSourceController2Vtbl {
    pub const fn new<Impl: IMediaFrameSourceController2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaFrameSourceController2Vtbl {
        unsafe extern "system" fn GetPropertyByExtendedIdAsync<Impl: IMediaFrameSourceController2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, extendedPropertyId_array_size: u32, extendedpropertyid: *const u8, maxpropertyvaluesize: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPropertyByExtendedIdAsync(::core::slice::from_raw_parts(::core::mem::transmute_copy(&extendedpropertyid), extendedPropertyId_array_size as _), &*(&maxpropertyvaluesize as *const <super::super::super::Foundation::IReference<u32> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<u32> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPropertyByExtendedIdAsync<Impl: IMediaFrameSourceController2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, extendedPropertyId_array_size: u32, extendedpropertyid: *const u8, propertyValue_array_size: u32, propertyvalue: *const u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetPropertyByExtendedIdAsync(::core::slice::from_raw_parts(::core::mem::transmute_copy(&extendedpropertyid), extendedPropertyId_array_size as _), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&propertyvalue), propertyValue_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaFrameSourceController2>, base.5, GetPropertyByExtendedIdAsync::<Impl, OFFSET>, SetPropertyByExtendedIdAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaFrameSourceController3Impl: Sized {
    fn AudioDeviceController(&self) -> ::windows::core::Result<super::super::Devices::AudioDeviceController>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaFrameSourceController3 {
    const NAME: &'static str = "Windows.Media.Capture.Frames.IMediaFrameSourceController3";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaFrameSourceController3Vtbl {
    pub const fn new<Impl: IMediaFrameSourceController3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaFrameSourceController3Vtbl {
        unsafe extern "system" fn AudioDeviceController<Impl: IMediaFrameSourceController3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AudioDeviceController() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaFrameSourceController3>, base.5, AudioDeviceController::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaFrameSourceGetPropertyResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<MediaFrameSourceGetPropertyStatus>;
    fn Value(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaFrameSourceGetPropertyResult {
    const NAME: &'static str = "Windows.Media.Capture.Frames.IMediaFrameSourceGetPropertyResult";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaFrameSourceGetPropertyResultVtbl {
    pub const fn new<Impl: IMediaFrameSourceGetPropertyResultImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaFrameSourceGetPropertyResultVtbl {
        unsafe extern "system" fn Status<Impl: IMediaFrameSourceGetPropertyResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut MediaFrameSourceGetPropertyStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Value<Impl: IMediaFrameSourceGetPropertyResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaFrameSourceGetPropertyResult>, base.5, Status::<Impl, OFFSET>, Value::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaFrameSourceGroupImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SourceInfos(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<MediaFrameSourceInfo>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaFrameSourceGroup {
    const NAME: &'static str = "Windows.Media.Capture.Frames.IMediaFrameSourceGroup";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaFrameSourceGroupVtbl {
    pub const fn new<Impl: IMediaFrameSourceGroupImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaFrameSourceGroupVtbl {
        unsafe extern "system" fn Id<Impl: IMediaFrameSourceGroupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayName<Impl: IMediaFrameSourceGroupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SourceInfos<Impl: IMediaFrameSourceGroupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SourceInfos() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaFrameSourceGroup>, base.5, Id::<Impl, OFFSET>, DisplayName::<Impl, OFFSET>, SourceInfos::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaFrameSourceGroupStaticsImpl: Sized {
    fn FindAllAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<MediaFrameSourceGroup>>>;
    fn FromIdAsync(&self, id: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<MediaFrameSourceGroup>>;
    fn GetDeviceSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaFrameSourceGroupStatics {
    const NAME: &'static str = "Windows.Media.Capture.Frames.IMediaFrameSourceGroupStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaFrameSourceGroupStaticsVtbl {
    pub const fn new<Impl: IMediaFrameSourceGroupStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaFrameSourceGroupStaticsVtbl {
        unsafe extern "system" fn FindAllAsync<Impl: IMediaFrameSourceGroupStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FindAllAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromIdAsync<Impl: IMediaFrameSourceGroupStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FromIdAsync(&*(&id as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceSelector<Impl: IMediaFrameSourceGroupStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelector() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaFrameSourceGroupStatics>, base.5, FindAllAsync::<Impl, OFFSET>, FromIdAsync::<Impl, OFFSET>, GetDeviceSelector::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaFrameSourceInfoImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MediaStreamType(&self) -> ::windows::core::Result<super::MediaStreamType>;
    fn SourceKind(&self) -> ::windows::core::Result<MediaFrameSourceKind>;
    fn SourceGroup(&self) -> ::windows::core::Result<MediaFrameSourceGroup>;
    fn DeviceInformation(&self) -> ::windows::core::Result<super::super::super::Devices::Enumeration::DeviceInformation>;
    fn Properties(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::GUID, ::windows::core::IInspectable>>;
    fn CoordinateSystem(&self) -> ::windows::core::Result<super::super::super::Perception::Spatial::SpatialCoordinateSystem>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaFrameSourceInfo {
    const NAME: &'static str = "Windows.Media.Capture.Frames.IMediaFrameSourceInfo";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaFrameSourceInfoVtbl {
    pub const fn new<Impl: IMediaFrameSourceInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaFrameSourceInfoVtbl {
        unsafe extern "system" fn Id<Impl: IMediaFrameSourceInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediaStreamType<Impl: IMediaFrameSourceInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::MediaStreamType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MediaStreamType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SourceKind<Impl: IMediaFrameSourceInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut MediaFrameSourceKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SourceKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SourceGroup<Impl: IMediaFrameSourceInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SourceGroup() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceInformation<Impl: IMediaFrameSourceInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeviceInformation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IMediaFrameSourceInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CoordinateSystem<Impl: IMediaFrameSourceInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CoordinateSystem() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaFrameSourceInfo>, base.5, Id::<Impl, OFFSET>, MediaStreamType::<Impl, OFFSET>, SourceKind::<Impl, OFFSET>, SourceGroup::<Impl, OFFSET>, DeviceInformation::<Impl, OFFSET>, Properties::<Impl, OFFSET>, CoordinateSystem::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaFrameSourceInfo2Impl: Sized {
    fn ProfileId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoProfileMediaDescription(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<super::MediaCaptureVideoProfileMediaDescription>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaFrameSourceInfo2 {
    const NAME: &'static str = "Windows.Media.Capture.Frames.IMediaFrameSourceInfo2";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaFrameSourceInfo2Vtbl {
    pub const fn new<Impl: IMediaFrameSourceInfo2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaFrameSourceInfo2Vtbl {
        unsafe extern "system" fn ProfileId<Impl: IMediaFrameSourceInfo2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ProfileId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoProfileMediaDescription<Impl: IMediaFrameSourceInfo2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VideoProfileMediaDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaFrameSourceInfo2>, base.5, ProfileId::<Impl, OFFSET>, VideoProfileMediaDescription::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaFrameSourceInfo3Impl: Sized {
    fn GetRelativePanel(&self, displayregion: &::core::option::Option<super::super::super::UI::WindowManagement::DisplayRegion>) -> ::windows::core::Result<super::super::super::Devices::Enumeration::Panel>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaFrameSourceInfo3 {
    const NAME: &'static str = "Windows.Media.Capture.Frames.IMediaFrameSourceInfo3";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaFrameSourceInfo3Vtbl {
    pub const fn new<Impl: IMediaFrameSourceInfo3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaFrameSourceInfo3Vtbl {
        unsafe extern "system" fn GetRelativePanel<Impl: IMediaFrameSourceInfo3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, displayregion: ::windows::core::RawPtr, result__: *mut super::super::super::Devices::Enumeration::Panel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetRelativePanel(&*(&displayregion as *const <super::super::super::UI::WindowManagement::DisplayRegion as ::windows::core::Abi>::Abi as *const <super::super::super::UI::WindowManagement::DisplayRegion as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaFrameSourceInfo3>, base.5, GetRelativePanel::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMultiSourceMediaFrameArrivedEventArgsImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMultiSourceMediaFrameArrivedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.Frames.IMultiSourceMediaFrameArrivedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMultiSourceMediaFrameArrivedEventArgsVtbl {
    pub const fn new<Impl: IMultiSourceMediaFrameArrivedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMultiSourceMediaFrameArrivedEventArgsVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMultiSourceMediaFrameArrivedEventArgs>, base.5)
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMultiSourceMediaFrameReaderImpl: Sized + IClosableImpl {
    fn FrameArrived(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<MultiSourceMediaFrameReader, MultiSourceMediaFrameArrivedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveFrameArrived(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn TryAcquireLatestFrame(&self) -> ::windows::core::Result<MultiSourceMediaFrameReference>;
    fn StartAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<MultiSourceMediaFrameReaderStartStatus>>;
    fn StopAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMultiSourceMediaFrameReader {
    const NAME: &'static str = "Windows.Media.Capture.Frames.IMultiSourceMediaFrameReader";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMultiSourceMediaFrameReaderVtbl {
    pub const fn new<Impl: IMultiSourceMediaFrameReaderImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMultiSourceMediaFrameReaderVtbl {
        unsafe extern "system" fn FrameArrived<Impl: IMultiSourceMediaFrameReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FrameArrived(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<MultiSourceMediaFrameReader, MultiSourceMediaFrameArrivedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<MultiSourceMediaFrameReader, MultiSourceMediaFrameArrivedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveFrameArrived<Impl: IMultiSourceMediaFrameReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveFrameArrived(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TryAcquireLatestFrame<Impl: IMultiSourceMediaFrameReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryAcquireLatestFrame() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartAsync<Impl: IMultiSourceMediaFrameReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StartAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StopAsync<Impl: IMultiSourceMediaFrameReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StopAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMultiSourceMediaFrameReader>, base.5, FrameArrived::<Impl, OFFSET>, RemoveFrameArrived::<Impl, OFFSET>, TryAcquireLatestFrame::<Impl, OFFSET>, StartAsync::<Impl, OFFSET>, StopAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMultiSourceMediaFrameReader2Impl: Sized {
    fn SetAcquisitionMode(&self, value: MediaFrameReaderAcquisitionMode) -> ::windows::core::Result<()>;
    fn AcquisitionMode(&self) -> ::windows::core::Result<MediaFrameReaderAcquisitionMode>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMultiSourceMediaFrameReader2 {
    const NAME: &'static str = "Windows.Media.Capture.Frames.IMultiSourceMediaFrameReader2";
}
#[cfg(feature = "implement_exclusive")]
impl IMultiSourceMediaFrameReader2Vtbl {
    pub const fn new<Impl: IMultiSourceMediaFrameReader2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMultiSourceMediaFrameReader2Vtbl {
        unsafe extern "system" fn SetAcquisitionMode<Impl: IMultiSourceMediaFrameReader2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: MediaFrameReaderAcquisitionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAcquisitionMode(value).into()
        }
        unsafe extern "system" fn AcquisitionMode<Impl: IMultiSourceMediaFrameReader2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut MediaFrameReaderAcquisitionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AcquisitionMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMultiSourceMediaFrameReader2>, base.5, SetAcquisitionMode::<Impl, OFFSET>, AcquisitionMode::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMultiSourceMediaFrameReferenceImpl: Sized + IClosableImpl {
    fn TryGetFrameReferenceBySourceId(&self, sourceid: &::windows::core::HSTRING) -> ::windows::core::Result<MediaFrameReference>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMultiSourceMediaFrameReference {
    const NAME: &'static str = "Windows.Media.Capture.Frames.IMultiSourceMediaFrameReference";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMultiSourceMediaFrameReferenceVtbl {
    pub const fn new<Impl: IMultiSourceMediaFrameReferenceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMultiSourceMediaFrameReferenceVtbl {
        unsafe extern "system" fn TryGetFrameReferenceBySourceId<Impl: IMultiSourceMediaFrameReferenceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sourceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryGetFrameReferenceBySourceId(&*(&sourceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMultiSourceMediaFrameReference>, base.5, TryGetFrameReferenceBySourceId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVideoMediaFrameImpl: Sized {
    fn FrameReference(&self) -> ::windows::core::Result<MediaFrameReference>;
    fn VideoFormat(&self) -> ::windows::core::Result<VideoMediaFrameFormat>;
    fn SoftwareBitmap(&self) -> ::windows::core::Result<super::super::super::Graphics::Imaging::SoftwareBitmap>;
    fn Direct3DSurface(&self) -> ::windows::core::Result<super::super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface>;
    fn CameraIntrinsics(&self) -> ::windows::core::Result<super::super::Devices::Core::CameraIntrinsics>;
    fn InfraredMediaFrame(&self) -> ::windows::core::Result<InfraredMediaFrame>;
    fn DepthMediaFrame(&self) -> ::windows::core::Result<DepthMediaFrame>;
    fn GetVideoFrame(&self) -> ::windows::core::Result<super::super::VideoFrame>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVideoMediaFrame {
    const NAME: &'static str = "Windows.Media.Capture.Frames.IVideoMediaFrame";
}
#[cfg(feature = "implement_exclusive")]
impl IVideoMediaFrameVtbl {
    pub const fn new<Impl: IVideoMediaFrameImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVideoMediaFrameVtbl {
        unsafe extern "system" fn FrameReference<Impl: IVideoMediaFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FrameReference() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoFormat<Impl: IVideoMediaFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VideoFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SoftwareBitmap<Impl: IVideoMediaFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SoftwareBitmap() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Direct3DSurface<Impl: IVideoMediaFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Direct3DSurface() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CameraIntrinsics<Impl: IVideoMediaFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CameraIntrinsics() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InfraredMediaFrame<Impl: IVideoMediaFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InfraredMediaFrame() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DepthMediaFrame<Impl: IVideoMediaFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DepthMediaFrame() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVideoFrame<Impl: IVideoMediaFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetVideoFrame() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVideoMediaFrame>, base.5, FrameReference::<Impl, OFFSET>, VideoFormat::<Impl, OFFSET>, SoftwareBitmap::<Impl, OFFSET>, Direct3DSurface::<Impl, OFFSET>, CameraIntrinsics::<Impl, OFFSET>, InfraredMediaFrame::<Impl, OFFSET>, DepthMediaFrame::<Impl, OFFSET>, GetVideoFrame::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVideoMediaFrameFormatImpl: Sized {
    fn MediaFrameFormat(&self) -> ::windows::core::Result<MediaFrameFormat>;
    fn DepthFormat(&self) -> ::windows::core::Result<DepthMediaFrameFormat>;
    fn Width(&self) -> ::windows::core::Result<u32>;
    fn Height(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVideoMediaFrameFormat {
    const NAME: &'static str = "Windows.Media.Capture.Frames.IVideoMediaFrameFormat";
}
#[cfg(feature = "implement_exclusive")]
impl IVideoMediaFrameFormatVtbl {
    pub const fn new<Impl: IVideoMediaFrameFormatImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVideoMediaFrameFormatVtbl {
        unsafe extern "system" fn MediaFrameFormat<Impl: IVideoMediaFrameFormatImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MediaFrameFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DepthFormat<Impl: IVideoMediaFrameFormatImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DepthFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Width<Impl: IVideoMediaFrameFormatImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Width() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Height<Impl: IVideoMediaFrameFormatImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Height() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVideoMediaFrameFormat>, base.5, MediaFrameFormat::<Impl, OFFSET>, DepthFormat::<Impl, OFFSET>, Width::<Impl, OFFSET>, Height::<Impl, OFFSET>)
    }
}
