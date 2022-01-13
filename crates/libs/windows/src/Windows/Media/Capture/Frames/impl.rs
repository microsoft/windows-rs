#[cfg(all(feature = "Media_MediaProperties", feature = "implement_exclusive"))]
pub trait IAudioMediaFrameImpl: Sized {
    fn FrameReference(&mut self) -> ::windows::core::Result<MediaFrameReference>;
    fn AudioEncodingProperties(&mut self) -> ::windows::core::Result<super::super::MediaProperties::AudioEncodingProperties>;
    fn GetAudioFrame(&mut self) -> ::windows::core::Result<super::super::AudioFrame>;
}
#[cfg(all(feature = "Media_MediaProperties", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAudioMediaFrame {
    const NAME: &'static str = "Windows.Media.Capture.Frames.IAudioMediaFrame";
}
#[cfg(all(feature = "Media_MediaProperties", feature = "implement_exclusive"))]
impl IAudioMediaFrameVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioMediaFrameImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioMediaFrameVtbl {
        unsafe extern "system" fn FrameReference<Impl: IAudioMediaFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FrameReference() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioEncodingProperties<Impl: IAudioMediaFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioEncodingProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAudioFrame<Impl: IAudioMediaFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAudioFrame() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAudioMediaFrame, BASE_OFFSET>(),
            FrameReference: FrameReference::<Impl, IMPL_OFFSET>,
            AudioEncodingProperties: AudioEncodingProperties::<Impl, IMPL_OFFSET>,
            GetAudioFrame: GetAudioFrame::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioMediaFrame as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IBufferMediaFrameImpl: Sized {
    fn FrameReference(&mut self) -> ::windows::core::Result<MediaFrameReference>;
    fn Buffer(&mut self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBufferMediaFrame {
    const NAME: &'static str = "Windows.Media.Capture.Frames.IBufferMediaFrame";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IBufferMediaFrameVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBufferMediaFrameImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBufferMediaFrameVtbl {
        unsafe extern "system" fn FrameReference<Impl: IBufferMediaFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FrameReference() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Buffer<Impl: IBufferMediaFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBufferMediaFrame, BASE_OFFSET>(),
            FrameReference: FrameReference::<Impl, IMPL_OFFSET>,
            Buffer: Buffer::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBufferMediaFrame as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Media_Devices_Core", feature = "Perception_Spatial", feature = "implement_exclusive"))]
pub trait IDepthMediaFrameImpl: Sized {
    fn FrameReference(&mut self) -> ::windows::core::Result<MediaFrameReference>;
    fn VideoMediaFrame(&mut self) -> ::windows::core::Result<VideoMediaFrame>;
    fn DepthFormat(&mut self) -> ::windows::core::Result<DepthMediaFrameFormat>;
    fn TryCreateCoordinateMapper(&mut self, cameraintrinsics: &::core::option::Option<super::super::Devices::Core::CameraIntrinsics>, coordinatesystem: &::core::option::Option<super::super::super::Perception::Spatial::SpatialCoordinateSystem>) -> ::windows::core::Result<super::super::Devices::Core::DepthCorrelatedCoordinateMapper>;
}
#[cfg(all(feature = "Media_Devices_Core", feature = "Perception_Spatial", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDepthMediaFrame {
    const NAME: &'static str = "Windows.Media.Capture.Frames.IDepthMediaFrame";
}
#[cfg(all(feature = "Media_Devices_Core", feature = "Perception_Spatial", feature = "implement_exclusive"))]
impl IDepthMediaFrameVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDepthMediaFrameImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDepthMediaFrameVtbl {
        unsafe extern "system" fn FrameReference<Impl: IDepthMediaFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FrameReference() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoMediaFrame<Impl: IDepthMediaFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoMediaFrame() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DepthFormat<Impl: IDepthMediaFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DepthFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryCreateCoordinateMapper<Impl: IDepthMediaFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cameraintrinsics: ::windows::core::RawPtr, coordinatesystem: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryCreateCoordinateMapper(&*(&cameraintrinsics as *const <super::super::Devices::Core::CameraIntrinsics as ::windows::core::Abi>::Abi as *const <super::super::Devices::Core::CameraIntrinsics as ::windows::core::DefaultType>::DefaultType), &*(&coordinatesystem as *const <super::super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::Abi>::Abi as *const <super::super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDepthMediaFrame, BASE_OFFSET>(),
            FrameReference: FrameReference::<Impl, IMPL_OFFSET>,
            VideoMediaFrame: VideoMediaFrame::<Impl, IMPL_OFFSET>,
            DepthFormat: DepthFormat::<Impl, IMPL_OFFSET>,
            TryCreateCoordinateMapper: TryCreateCoordinateMapper::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDepthMediaFrame as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDepthMediaFrame2Impl: Sized {
    fn MaxReliableDepth(&mut self) -> ::windows::core::Result<u32>;
    fn MinReliableDepth(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDepthMediaFrame2 {
    const NAME: &'static str = "Windows.Media.Capture.Frames.IDepthMediaFrame2";
}
#[cfg(feature = "implement_exclusive")]
impl IDepthMediaFrame2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDepthMediaFrame2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDepthMediaFrame2Vtbl {
        unsafe extern "system" fn MaxReliableDepth<Impl: IDepthMediaFrame2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxReliableDepth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinReliableDepth<Impl: IDepthMediaFrame2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinReliableDepth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDepthMediaFrame2, BASE_OFFSET>(),
            MaxReliableDepth: MaxReliableDepth::<Impl, IMPL_OFFSET>,
            MinReliableDepth: MinReliableDepth::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDepthMediaFrame2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDepthMediaFrameFormatImpl: Sized {
    fn VideoFormat(&mut self) -> ::windows::core::Result<VideoMediaFrameFormat>;
    fn DepthScaleInMeters(&mut self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDepthMediaFrameFormat {
    const NAME: &'static str = "Windows.Media.Capture.Frames.IDepthMediaFrameFormat";
}
#[cfg(feature = "implement_exclusive")]
impl IDepthMediaFrameFormatVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDepthMediaFrameFormatImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDepthMediaFrameFormatVtbl {
        unsafe extern "system" fn VideoFormat<Impl: IDepthMediaFrameFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DepthScaleInMeters<Impl: IDepthMediaFrameFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DepthScaleInMeters() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDepthMediaFrameFormat, BASE_OFFSET>(),
            VideoFormat: VideoFormat::<Impl, IMPL_OFFSET>,
            DepthScaleInMeters: DepthScaleInMeters::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDepthMediaFrameFormat as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInfraredMediaFrameImpl: Sized {
    fn FrameReference(&mut self) -> ::windows::core::Result<MediaFrameReference>;
    fn VideoMediaFrame(&mut self) -> ::windows::core::Result<VideoMediaFrame>;
    fn IsIlluminated(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInfraredMediaFrame {
    const NAME: &'static str = "Windows.Media.Capture.Frames.IInfraredMediaFrame";
}
#[cfg(feature = "implement_exclusive")]
impl IInfraredMediaFrameVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInfraredMediaFrameImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInfraredMediaFrameVtbl {
        unsafe extern "system" fn FrameReference<Impl: IInfraredMediaFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FrameReference() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoMediaFrame<Impl: IInfraredMediaFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoMediaFrame() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsIlluminated<Impl: IInfraredMediaFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsIlluminated() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInfraredMediaFrame, BASE_OFFSET>(),
            FrameReference: FrameReference::<Impl, IMPL_OFFSET>,
            VideoMediaFrame: VideoMediaFrame::<Impl, IMPL_OFFSET>,
            IsIlluminated: IsIlluminated::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInfraredMediaFrame as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaFrameArrivedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaFrameArrivedEventArgsVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaFrameArrivedEventArgs, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaFrameArrivedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_MediaProperties", feature = "implement_exclusive"))]
pub trait IMediaFrameFormatImpl: Sized {
    fn MajorType(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Subtype(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FrameRate(&mut self) -> ::windows::core::Result<super::super::MediaProperties::MediaRatio>;
    fn Properties(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::GUID, ::windows::core::IInspectable>>;
    fn VideoFormat(&mut self) -> ::windows::core::Result<VideoMediaFrameFormat>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_MediaProperties", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaFrameFormat {
    const NAME: &'static str = "Windows.Media.Capture.Frames.IMediaFrameFormat";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_MediaProperties", feature = "implement_exclusive"))]
impl IMediaFrameFormatVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaFrameFormatImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaFrameFormatVtbl {
        unsafe extern "system" fn MajorType<Impl: IMediaFrameFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MajorType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Subtype<Impl: IMediaFrameFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Subtype() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FrameRate<Impl: IMediaFrameFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FrameRate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IMediaFrameFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoFormat<Impl: IMediaFrameFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaFrameFormat, BASE_OFFSET>(),
            MajorType: MajorType::<Impl, IMPL_OFFSET>,
            Subtype: Subtype::<Impl, IMPL_OFFSET>,
            FrameRate: FrameRate::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
            VideoFormat: VideoFormat::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaFrameFormat as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Media_MediaProperties", feature = "implement_exclusive"))]
pub trait IMediaFrameFormat2Impl: Sized {
    fn AudioEncodingProperties(&mut self) -> ::windows::core::Result<super::super::MediaProperties::AudioEncodingProperties>;
}
#[cfg(all(feature = "Media_MediaProperties", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaFrameFormat2 {
    const NAME: &'static str = "Windows.Media.Capture.Frames.IMediaFrameFormat2";
}
#[cfg(all(feature = "Media_MediaProperties", feature = "implement_exclusive"))]
impl IMediaFrameFormat2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaFrameFormat2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaFrameFormat2Vtbl {
        unsafe extern "system" fn AudioEncodingProperties<Impl: IMediaFrameFormat2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioEncodingProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaFrameFormat2, BASE_OFFSET>(),
            AudioEncodingProperties: AudioEncodingProperties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaFrameFormat2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMediaFrameReaderImpl: Sized + IClosableImpl {
    fn FrameArrived(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<MediaFrameReader, MediaFrameArrivedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveFrameArrived(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn TryAcquireLatestFrame(&mut self) -> ::windows::core::Result<MediaFrameReference>;
    fn StartAsync(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<MediaFrameReaderStartStatus>>;
    fn StopAsync(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaFrameReader {
    const NAME: &'static str = "Windows.Media.Capture.Frames.IMediaFrameReader";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMediaFrameReaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaFrameReaderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaFrameReaderVtbl {
        unsafe extern "system" fn FrameArrived<Impl: IMediaFrameReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FrameArrived(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<MediaFrameReader, MediaFrameArrivedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<MediaFrameReader, MediaFrameArrivedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveFrameArrived<Impl: IMediaFrameReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveFrameArrived(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TryAcquireLatestFrame<Impl: IMediaFrameReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryAcquireLatestFrame() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartAsync<Impl: IMediaFrameReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StopAsync<Impl: IMediaFrameReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StopAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaFrameReader, BASE_OFFSET>(),
            FrameArrived: FrameArrived::<Impl, IMPL_OFFSET>,
            RemoveFrameArrived: RemoveFrameArrived::<Impl, IMPL_OFFSET>,
            TryAcquireLatestFrame: TryAcquireLatestFrame::<Impl, IMPL_OFFSET>,
            StartAsync: StartAsync::<Impl, IMPL_OFFSET>,
            StopAsync: StopAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaFrameReader as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaFrameReader2Impl: Sized {
    fn SetAcquisitionMode(&mut self, value: MediaFrameReaderAcquisitionMode) -> ::windows::core::Result<()>;
    fn AcquisitionMode(&mut self) -> ::windows::core::Result<MediaFrameReaderAcquisitionMode>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaFrameReader2 {
    const NAME: &'static str = "Windows.Media.Capture.Frames.IMediaFrameReader2";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaFrameReader2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaFrameReader2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaFrameReader2Vtbl {
        unsafe extern "system" fn SetAcquisitionMode<Impl: IMediaFrameReader2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: MediaFrameReaderAcquisitionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAcquisitionMode(value).into()
        }
        unsafe extern "system" fn AcquisitionMode<Impl: IMediaFrameReader2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MediaFrameReaderAcquisitionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AcquisitionMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaFrameReader2, BASE_OFFSET>(),
            SetAcquisitionMode: SetAcquisitionMode::<Impl, IMPL_OFFSET>,
            AcquisitionMode: AcquisitionMode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaFrameReader2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Perception_Spatial", feature = "implement_exclusive"))]
pub trait IMediaFrameReferenceImpl: Sized + IClosableImpl {
    fn SourceKind(&mut self) -> ::windows::core::Result<MediaFrameSourceKind>;
    fn Format(&mut self) -> ::windows::core::Result<MediaFrameFormat>;
    fn SystemRelativeTime(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>;
    fn Duration(&mut self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn Properties(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::GUID, ::windows::core::IInspectable>>;
    fn BufferMediaFrame(&mut self) -> ::windows::core::Result<BufferMediaFrame>;
    fn VideoMediaFrame(&mut self) -> ::windows::core::Result<VideoMediaFrame>;
    fn CoordinateSystem(&mut self) -> ::windows::core::Result<super::super::super::Perception::Spatial::SpatialCoordinateSystem>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Perception_Spatial", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaFrameReference {
    const NAME: &'static str = "Windows.Media.Capture.Frames.IMediaFrameReference";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Perception_Spatial", feature = "implement_exclusive"))]
impl IMediaFrameReferenceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaFrameReferenceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaFrameReferenceVtbl {
        unsafe extern "system" fn SourceKind<Impl: IMediaFrameReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MediaFrameSourceKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SourceKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Format<Impl: IMediaFrameReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Format() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SystemRelativeTime<Impl: IMediaFrameReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SystemRelativeTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Duration<Impl: IMediaFrameReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Duration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IMediaFrameReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BufferMediaFrame<Impl: IMediaFrameReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BufferMediaFrame() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoMediaFrame<Impl: IMediaFrameReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoMediaFrame() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CoordinateSystem<Impl: IMediaFrameReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CoordinateSystem() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaFrameReference, BASE_OFFSET>(),
            SourceKind: SourceKind::<Impl, IMPL_OFFSET>,
            Format: Format::<Impl, IMPL_OFFSET>,
            SystemRelativeTime: SystemRelativeTime::<Impl, IMPL_OFFSET>,
            Duration: Duration::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
            BufferMediaFrame: BufferMediaFrame::<Impl, IMPL_OFFSET>,
            VideoMediaFrame: VideoMediaFrame::<Impl, IMPL_OFFSET>,
            CoordinateSystem: CoordinateSystem::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaFrameReference as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaFrameReference2Impl: Sized {
    fn AudioMediaFrame(&mut self) -> ::windows::core::Result<AudioMediaFrame>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaFrameReference2 {
    const NAME: &'static str = "Windows.Media.Capture.Frames.IMediaFrameReference2";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaFrameReference2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaFrameReference2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaFrameReference2Vtbl {
        unsafe extern "system" fn AudioMediaFrame<Impl: IMediaFrameReference2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioMediaFrame() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaFrameReference2, BASE_OFFSET>(),
            AudioMediaFrame: AudioMediaFrame::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaFrameReference2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Devices_Core", feature = "implement_exclusive"))]
pub trait IMediaFrameSourceImpl: Sized {
    fn Info(&mut self) -> ::windows::core::Result<MediaFrameSourceInfo>;
    fn Controller(&mut self) -> ::windows::core::Result<MediaFrameSourceController>;
    fn SupportedFormats(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<MediaFrameFormat>>;
    fn CurrentFormat(&mut self) -> ::windows::core::Result<MediaFrameFormat>;
    fn SetFormatAsync(&mut self, format: &::core::option::Option<MediaFrameFormat>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn FormatChanged(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<MediaFrameSource, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveFormatChanged(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn TryGetCameraIntrinsics(&mut self, format: &::core::option::Option<MediaFrameFormat>) -> ::windows::core::Result<super::super::Devices::Core::CameraIntrinsics>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Devices_Core", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaFrameSource {
    const NAME: &'static str = "Windows.Media.Capture.Frames.IMediaFrameSource";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Devices_Core", feature = "implement_exclusive"))]
impl IMediaFrameSourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaFrameSourceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaFrameSourceVtbl {
        unsafe extern "system" fn Info<Impl: IMediaFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Info() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Controller<Impl: IMediaFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Controller() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedFormats<Impl: IMediaFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedFormats() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentFormat<Impl: IMediaFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormatAsync<Impl: IMediaFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetFormatAsync(&*(&format as *const <MediaFrameFormat as ::windows::core::Abi>::Abi as *const <MediaFrameFormat as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FormatChanged<Impl: IMediaFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FormatChanged(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<MediaFrameSource, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<MediaFrameSource, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveFormatChanged<Impl: IMediaFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveFormatChanged(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TryGetCameraIntrinsics<Impl: IMediaFrameSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryGetCameraIntrinsics(&*(&format as *const <MediaFrameFormat as ::windows::core::Abi>::Abi as *const <MediaFrameFormat as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaFrameSource, BASE_OFFSET>(),
            Info: Info::<Impl, IMPL_OFFSET>,
            Controller: Controller::<Impl, IMPL_OFFSET>,
            SupportedFormats: SupportedFormats::<Impl, IMPL_OFFSET>,
            CurrentFormat: CurrentFormat::<Impl, IMPL_OFFSET>,
            SetFormatAsync: SetFormatAsync::<Impl, IMPL_OFFSET>,
            FormatChanged: FormatChanged::<Impl, IMPL_OFFSET>,
            RemoveFormatChanged: RemoveFormatChanged::<Impl, IMPL_OFFSET>,
            TryGetCameraIntrinsics: TryGetCameraIntrinsics::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaFrameSource as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Media_Devices", feature = "implement_exclusive"))]
pub trait IMediaFrameSourceControllerImpl: Sized {
    fn GetPropertyAsync(&mut self, propertyid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<MediaFrameSourceGetPropertyResult>>;
    fn SetPropertyAsync(&mut self, propertyid: &::windows::core::HSTRING, propertyvalue: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<MediaFrameSourceSetPropertyStatus>>;
    fn VideoDeviceController(&mut self) -> ::windows::core::Result<super::super::Devices::VideoDeviceController>;
}
#[cfg(all(feature = "Foundation", feature = "Media_Devices", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaFrameSourceController {
    const NAME: &'static str = "Windows.Media.Capture.Frames.IMediaFrameSourceController";
}
#[cfg(all(feature = "Foundation", feature = "Media_Devices", feature = "implement_exclusive"))]
impl IMediaFrameSourceControllerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaFrameSourceControllerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaFrameSourceControllerVtbl {
        unsafe extern "system" fn GetPropertyAsync<Impl: IMediaFrameSourceControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPropertyAsync(&*(&propertyid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPropertyAsync<Impl: IMediaFrameSourceControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, propertyvalue: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPropertyAsync(&*(&propertyid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&propertyvalue as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoDeviceController<Impl: IMediaFrameSourceControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoDeviceController() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaFrameSourceController, BASE_OFFSET>(),
            GetPropertyAsync: GetPropertyAsync::<Impl, IMPL_OFFSET>,
            SetPropertyAsync: SetPropertyAsync::<Impl, IMPL_OFFSET>,
            VideoDeviceController: VideoDeviceController::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaFrameSourceController as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMediaFrameSourceController2Impl: Sized {
    fn GetPropertyByExtendedIdAsync(&mut self, extendedpropertyid: &[<u8 as ::windows::core::DefaultType>::DefaultType], maxpropertyvaluesize: &::core::option::Option<super::super::super::Foundation::IReference<u32>>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<MediaFrameSourceGetPropertyResult>>;
    fn SetPropertyByExtendedIdAsync(&mut self, extendedpropertyid: &[<u8 as ::windows::core::DefaultType>::DefaultType], propertyvalue: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<MediaFrameSourceSetPropertyStatus>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaFrameSourceController2 {
    const NAME: &'static str = "Windows.Media.Capture.Frames.IMediaFrameSourceController2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMediaFrameSourceController2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaFrameSourceController2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaFrameSourceController2Vtbl {
        unsafe extern "system" fn GetPropertyByExtendedIdAsync<Impl: IMediaFrameSourceController2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, extendedPropertyId_array_size: u32, extendedpropertyid: *const u8, maxpropertyvaluesize: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPropertyByExtendedIdAsync(::core::slice::from_raw_parts(::core::mem::transmute_copy(&extendedpropertyid), extendedPropertyId_array_size as _), &*(&maxpropertyvaluesize as *const <super::super::super::Foundation::IReference<u32> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<u32> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPropertyByExtendedIdAsync<Impl: IMediaFrameSourceController2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, extendedPropertyId_array_size: u32, extendedpropertyid: *const u8, propertyValue_array_size: u32, propertyvalue: *const u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPropertyByExtendedIdAsync(::core::slice::from_raw_parts(::core::mem::transmute_copy(&extendedpropertyid), extendedPropertyId_array_size as _), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&propertyvalue), propertyValue_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaFrameSourceController2, BASE_OFFSET>(),
            GetPropertyByExtendedIdAsync: GetPropertyByExtendedIdAsync::<Impl, IMPL_OFFSET>,
            SetPropertyByExtendedIdAsync: SetPropertyByExtendedIdAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaFrameSourceController2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Media_Devices", feature = "implement_exclusive"))]
pub trait IMediaFrameSourceController3Impl: Sized {
    fn AudioDeviceController(&mut self) -> ::windows::core::Result<super::super::Devices::AudioDeviceController>;
}
#[cfg(all(feature = "Media_Devices", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaFrameSourceController3 {
    const NAME: &'static str = "Windows.Media.Capture.Frames.IMediaFrameSourceController3";
}
#[cfg(all(feature = "Media_Devices", feature = "implement_exclusive"))]
impl IMediaFrameSourceController3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaFrameSourceController3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaFrameSourceController3Vtbl {
        unsafe extern "system" fn AudioDeviceController<Impl: IMediaFrameSourceController3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioDeviceController() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaFrameSourceController3, BASE_OFFSET>(),
            AudioDeviceController: AudioDeviceController::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaFrameSourceController3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaFrameSourceGetPropertyResultImpl: Sized {
    fn Status(&mut self) -> ::windows::core::Result<MediaFrameSourceGetPropertyStatus>;
    fn Value(&mut self) -> ::windows::core::Result<::windows::core::IInspectable>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaFrameSourceGetPropertyResult {
    const NAME: &'static str = "Windows.Media.Capture.Frames.IMediaFrameSourceGetPropertyResult";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaFrameSourceGetPropertyResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaFrameSourceGetPropertyResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaFrameSourceGetPropertyResultVtbl {
        unsafe extern "system" fn Status<Impl: IMediaFrameSourceGetPropertyResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MediaFrameSourceGetPropertyStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Value<Impl: IMediaFrameSourceGetPropertyResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaFrameSourceGetPropertyResult, BASE_OFFSET>(),
            Status: Status::<Impl, IMPL_OFFSET>,
            Value: Value::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaFrameSourceGetPropertyResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMediaFrameSourceGroupImpl: Sized {
    fn Id(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SourceInfos(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<MediaFrameSourceInfo>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaFrameSourceGroup {
    const NAME: &'static str = "Windows.Media.Capture.Frames.IMediaFrameSourceGroup";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMediaFrameSourceGroupVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaFrameSourceGroupImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaFrameSourceGroupVtbl {
        unsafe extern "system" fn Id<Impl: IMediaFrameSourceGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayName<Impl: IMediaFrameSourceGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SourceInfos<Impl: IMediaFrameSourceGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SourceInfos() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaFrameSourceGroup, BASE_OFFSET>(),
            Id: Id::<Impl, IMPL_OFFSET>,
            DisplayName: DisplayName::<Impl, IMPL_OFFSET>,
            SourceInfos: SourceInfos::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaFrameSourceGroup as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMediaFrameSourceGroupStaticsImpl: Sized {
    fn FindAllAsync(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<MediaFrameSourceGroup>>>;
    fn FromIdAsync(&mut self, id: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<MediaFrameSourceGroup>>;
    fn GetDeviceSelector(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaFrameSourceGroupStatics {
    const NAME: &'static str = "Windows.Media.Capture.Frames.IMediaFrameSourceGroupStatics";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMediaFrameSourceGroupStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaFrameSourceGroupStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaFrameSourceGroupStaticsVtbl {
        unsafe extern "system" fn FindAllAsync<Impl: IMediaFrameSourceGroupStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindAllAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromIdAsync<Impl: IMediaFrameSourceGroupStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromIdAsync(&*(&id as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceSelector<Impl: IMediaFrameSourceGroupStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelector() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaFrameSourceGroupStatics, BASE_OFFSET>(),
            FindAllAsync: FindAllAsync::<Impl, IMPL_OFFSET>,
            FromIdAsync: FromIdAsync::<Impl, IMPL_OFFSET>,
            GetDeviceSelector: GetDeviceSelector::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaFrameSourceGroupStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Enumeration", feature = "Foundation_Collections", feature = "Perception_Spatial", feature = "implement_exclusive"))]
pub trait IMediaFrameSourceInfoImpl: Sized {
    fn Id(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MediaStreamType(&mut self) -> ::windows::core::Result<super::MediaStreamType>;
    fn SourceKind(&mut self) -> ::windows::core::Result<MediaFrameSourceKind>;
    fn SourceGroup(&mut self) -> ::windows::core::Result<MediaFrameSourceGroup>;
    fn DeviceInformation(&mut self) -> ::windows::core::Result<super::super::super::Devices::Enumeration::DeviceInformation>;
    fn Properties(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::GUID, ::windows::core::IInspectable>>;
    fn CoordinateSystem(&mut self) -> ::windows::core::Result<super::super::super::Perception::Spatial::SpatialCoordinateSystem>;
}
#[cfg(all(feature = "Devices_Enumeration", feature = "Foundation_Collections", feature = "Perception_Spatial", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaFrameSourceInfo {
    const NAME: &'static str = "Windows.Media.Capture.Frames.IMediaFrameSourceInfo";
}
#[cfg(all(feature = "Devices_Enumeration", feature = "Foundation_Collections", feature = "Perception_Spatial", feature = "implement_exclusive"))]
impl IMediaFrameSourceInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaFrameSourceInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaFrameSourceInfoVtbl {
        unsafe extern "system" fn Id<Impl: IMediaFrameSourceInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediaStreamType<Impl: IMediaFrameSourceInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::MediaStreamType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MediaStreamType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SourceKind<Impl: IMediaFrameSourceInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MediaFrameSourceKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SourceKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SourceGroup<Impl: IMediaFrameSourceInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SourceGroup() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceInformation<Impl: IMediaFrameSourceInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceInformation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IMediaFrameSourceInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CoordinateSystem<Impl: IMediaFrameSourceInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CoordinateSystem() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaFrameSourceInfo, BASE_OFFSET>(),
            Id: Id::<Impl, IMPL_OFFSET>,
            MediaStreamType: MediaStreamType::<Impl, IMPL_OFFSET>,
            SourceKind: SourceKind::<Impl, IMPL_OFFSET>,
            SourceGroup: SourceGroup::<Impl, IMPL_OFFSET>,
            DeviceInformation: DeviceInformation::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
            CoordinateSystem: CoordinateSystem::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaFrameSourceInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMediaFrameSourceInfo2Impl: Sized {
    fn ProfileId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoProfileMediaDescription(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<super::MediaCaptureVideoProfileMediaDescription>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaFrameSourceInfo2 {
    const NAME: &'static str = "Windows.Media.Capture.Frames.IMediaFrameSourceInfo2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMediaFrameSourceInfo2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaFrameSourceInfo2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaFrameSourceInfo2Vtbl {
        unsafe extern "system" fn ProfileId<Impl: IMediaFrameSourceInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProfileId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoProfileMediaDescription<Impl: IMediaFrameSourceInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoProfileMediaDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaFrameSourceInfo2, BASE_OFFSET>(),
            ProfileId: ProfileId::<Impl, IMPL_OFFSET>,
            VideoProfileMediaDescription: VideoProfileMediaDescription::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaFrameSourceInfo2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Enumeration", feature = "UI_WindowManagement", feature = "implement_exclusive"))]
pub trait IMediaFrameSourceInfo3Impl: Sized {
    fn GetRelativePanel(&mut self, displayregion: &::core::option::Option<super::super::super::UI::WindowManagement::DisplayRegion>) -> ::windows::core::Result<super::super::super::Devices::Enumeration::Panel>;
}
#[cfg(all(feature = "Devices_Enumeration", feature = "UI_WindowManagement", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaFrameSourceInfo3 {
    const NAME: &'static str = "Windows.Media.Capture.Frames.IMediaFrameSourceInfo3";
}
#[cfg(all(feature = "Devices_Enumeration", feature = "UI_WindowManagement", feature = "implement_exclusive"))]
impl IMediaFrameSourceInfo3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaFrameSourceInfo3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaFrameSourceInfo3Vtbl {
        unsafe extern "system" fn GetRelativePanel<Impl: IMediaFrameSourceInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, displayregion: ::windows::core::RawPtr, result__: *mut super::super::super::Devices::Enumeration::Panel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRelativePanel(&*(&displayregion as *const <super::super::super::UI::WindowManagement::DisplayRegion as ::windows::core::Abi>::Abi as *const <super::super::super::UI::WindowManagement::DisplayRegion as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaFrameSourceInfo3, BASE_OFFSET>(),
            GetRelativePanel: GetRelativePanel::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaFrameSourceInfo3 as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMultiSourceMediaFrameArrivedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMultiSourceMediaFrameArrivedEventArgsVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IMultiSourceMediaFrameArrivedEventArgs, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMultiSourceMediaFrameArrivedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMultiSourceMediaFrameReaderImpl: Sized + IClosableImpl {
    fn FrameArrived(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<MultiSourceMediaFrameReader, MultiSourceMediaFrameArrivedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveFrameArrived(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn TryAcquireLatestFrame(&mut self) -> ::windows::core::Result<MultiSourceMediaFrameReference>;
    fn StartAsync(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<MultiSourceMediaFrameReaderStartStatus>>;
    fn StopAsync(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMultiSourceMediaFrameReader {
    const NAME: &'static str = "Windows.Media.Capture.Frames.IMultiSourceMediaFrameReader";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMultiSourceMediaFrameReaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMultiSourceMediaFrameReaderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMultiSourceMediaFrameReaderVtbl {
        unsafe extern "system" fn FrameArrived<Impl: IMultiSourceMediaFrameReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FrameArrived(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<MultiSourceMediaFrameReader, MultiSourceMediaFrameArrivedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<MultiSourceMediaFrameReader, MultiSourceMediaFrameArrivedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveFrameArrived<Impl: IMultiSourceMediaFrameReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveFrameArrived(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TryAcquireLatestFrame<Impl: IMultiSourceMediaFrameReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryAcquireLatestFrame() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartAsync<Impl: IMultiSourceMediaFrameReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StopAsync<Impl: IMultiSourceMediaFrameReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StopAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMultiSourceMediaFrameReader, BASE_OFFSET>(),
            FrameArrived: FrameArrived::<Impl, IMPL_OFFSET>,
            RemoveFrameArrived: RemoveFrameArrived::<Impl, IMPL_OFFSET>,
            TryAcquireLatestFrame: TryAcquireLatestFrame::<Impl, IMPL_OFFSET>,
            StartAsync: StartAsync::<Impl, IMPL_OFFSET>,
            StopAsync: StopAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMultiSourceMediaFrameReader as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMultiSourceMediaFrameReader2Impl: Sized {
    fn SetAcquisitionMode(&mut self, value: MediaFrameReaderAcquisitionMode) -> ::windows::core::Result<()>;
    fn AcquisitionMode(&mut self) -> ::windows::core::Result<MediaFrameReaderAcquisitionMode>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMultiSourceMediaFrameReader2 {
    const NAME: &'static str = "Windows.Media.Capture.Frames.IMultiSourceMediaFrameReader2";
}
#[cfg(feature = "implement_exclusive")]
impl IMultiSourceMediaFrameReader2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMultiSourceMediaFrameReader2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMultiSourceMediaFrameReader2Vtbl {
        unsafe extern "system" fn SetAcquisitionMode<Impl: IMultiSourceMediaFrameReader2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: MediaFrameReaderAcquisitionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAcquisitionMode(value).into()
        }
        unsafe extern "system" fn AcquisitionMode<Impl: IMultiSourceMediaFrameReader2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MediaFrameReaderAcquisitionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AcquisitionMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMultiSourceMediaFrameReader2, BASE_OFFSET>(),
            SetAcquisitionMode: SetAcquisitionMode::<Impl, IMPL_OFFSET>,
            AcquisitionMode: AcquisitionMode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMultiSourceMediaFrameReader2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMultiSourceMediaFrameReferenceImpl: Sized + IClosableImpl {
    fn TryGetFrameReferenceBySourceId(&mut self, sourceid: &::windows::core::HSTRING) -> ::windows::core::Result<MediaFrameReference>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMultiSourceMediaFrameReference {
    const NAME: &'static str = "Windows.Media.Capture.Frames.IMultiSourceMediaFrameReference";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMultiSourceMediaFrameReferenceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMultiSourceMediaFrameReferenceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMultiSourceMediaFrameReferenceVtbl {
        unsafe extern "system" fn TryGetFrameReferenceBySourceId<Impl: IMultiSourceMediaFrameReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryGetFrameReferenceBySourceId(&*(&sourceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMultiSourceMediaFrameReference, BASE_OFFSET>(),
            TryGetFrameReferenceBySourceId: TryGetFrameReferenceBySourceId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMultiSourceMediaFrameReference as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Graphics_DirectX_Direct3D11", feature = "Graphics_Imaging", feature = "Media_Devices_Core", feature = "implement_exclusive"))]
pub trait IVideoMediaFrameImpl: Sized {
    fn FrameReference(&mut self) -> ::windows::core::Result<MediaFrameReference>;
    fn VideoFormat(&mut self) -> ::windows::core::Result<VideoMediaFrameFormat>;
    fn SoftwareBitmap(&mut self) -> ::windows::core::Result<super::super::super::Graphics::Imaging::SoftwareBitmap>;
    fn Direct3DSurface(&mut self) -> ::windows::core::Result<super::super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface>;
    fn CameraIntrinsics(&mut self) -> ::windows::core::Result<super::super::Devices::Core::CameraIntrinsics>;
    fn InfraredMediaFrame(&mut self) -> ::windows::core::Result<InfraredMediaFrame>;
    fn DepthMediaFrame(&mut self) -> ::windows::core::Result<DepthMediaFrame>;
    fn GetVideoFrame(&mut self) -> ::windows::core::Result<super::super::VideoFrame>;
}
#[cfg(all(feature = "Graphics_DirectX_Direct3D11", feature = "Graphics_Imaging", feature = "Media_Devices_Core", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVideoMediaFrame {
    const NAME: &'static str = "Windows.Media.Capture.Frames.IVideoMediaFrame";
}
#[cfg(all(feature = "Graphics_DirectX_Direct3D11", feature = "Graphics_Imaging", feature = "Media_Devices_Core", feature = "implement_exclusive"))]
impl IVideoMediaFrameVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVideoMediaFrameImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVideoMediaFrameVtbl {
        unsafe extern "system" fn FrameReference<Impl: IVideoMediaFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FrameReference() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoFormat<Impl: IVideoMediaFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SoftwareBitmap<Impl: IVideoMediaFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SoftwareBitmap() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Direct3DSurface<Impl: IVideoMediaFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Direct3DSurface() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CameraIntrinsics<Impl: IVideoMediaFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CameraIntrinsics() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InfraredMediaFrame<Impl: IVideoMediaFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InfraredMediaFrame() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DepthMediaFrame<Impl: IVideoMediaFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DepthMediaFrame() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVideoFrame<Impl: IVideoMediaFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVideoFrame() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVideoMediaFrame, BASE_OFFSET>(),
            FrameReference: FrameReference::<Impl, IMPL_OFFSET>,
            VideoFormat: VideoFormat::<Impl, IMPL_OFFSET>,
            SoftwareBitmap: SoftwareBitmap::<Impl, IMPL_OFFSET>,
            Direct3DSurface: Direct3DSurface::<Impl, IMPL_OFFSET>,
            CameraIntrinsics: CameraIntrinsics::<Impl, IMPL_OFFSET>,
            InfraredMediaFrame: InfraredMediaFrame::<Impl, IMPL_OFFSET>,
            DepthMediaFrame: DepthMediaFrame::<Impl, IMPL_OFFSET>,
            GetVideoFrame: GetVideoFrame::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVideoMediaFrame as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVideoMediaFrameFormatImpl: Sized {
    fn MediaFrameFormat(&mut self) -> ::windows::core::Result<MediaFrameFormat>;
    fn DepthFormat(&mut self) -> ::windows::core::Result<DepthMediaFrameFormat>;
    fn Width(&mut self) -> ::windows::core::Result<u32>;
    fn Height(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVideoMediaFrameFormat {
    const NAME: &'static str = "Windows.Media.Capture.Frames.IVideoMediaFrameFormat";
}
#[cfg(feature = "implement_exclusive")]
impl IVideoMediaFrameFormatVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVideoMediaFrameFormatImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVideoMediaFrameFormatVtbl {
        unsafe extern "system" fn MediaFrameFormat<Impl: IVideoMediaFrameFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MediaFrameFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DepthFormat<Impl: IVideoMediaFrameFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DepthFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Width<Impl: IVideoMediaFrameFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Width() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Height<Impl: IVideoMediaFrameFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Height() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVideoMediaFrameFormat, BASE_OFFSET>(),
            MediaFrameFormat: MediaFrameFormat::<Impl, IMPL_OFFSET>,
            DepthFormat: DepthFormat::<Impl, IMPL_OFFSET>,
            Width: Width::<Impl, IMPL_OFFSET>,
            Height: Height::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVideoMediaFrameFormat as ::windows::core::Interface>::IID
    }
}
