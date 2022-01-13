#[cfg(all(feature = "Media_MediaProperties", feature = "implement_exclusive"))]
pub trait IAudioStreamDescriptorImpl: Sized + IMediaStreamDescriptorImpl {
    fn EncodingProperties(&mut self) -> ::windows::core::Result<super::MediaProperties::AudioEncodingProperties>;
}
#[cfg(all(feature = "Media_MediaProperties", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAudioStreamDescriptor {
    const NAME: &'static str = "Windows.Media.Core.IAudioStreamDescriptor";
}
#[cfg(all(feature = "Media_MediaProperties", feature = "implement_exclusive"))]
impl IAudioStreamDescriptorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioStreamDescriptorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioStreamDescriptorVtbl {
        unsafe extern "system" fn EncodingProperties<Impl: IAudioStreamDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EncodingProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAudioStreamDescriptor, BASE_OFFSET>(),
            EncodingProperties: EncodingProperties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioStreamDescriptor as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAudioStreamDescriptor2Impl: Sized + IMediaStreamDescriptorImpl {
    fn SetLeadingEncoderPadding(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<u32>>) -> ::windows::core::Result<()>;
    fn LeadingEncoderPadding(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>>;
    fn SetTrailingEncoderPadding(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<u32>>) -> ::windows::core::Result<()>;
    fn TrailingEncoderPadding(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAudioStreamDescriptor2 {
    const NAME: &'static str = "Windows.Media.Core.IAudioStreamDescriptor2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAudioStreamDescriptor2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioStreamDescriptor2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioStreamDescriptor2Vtbl {
        unsafe extern "system" fn SetLeadingEncoderPadding<Impl: IAudioStreamDescriptor2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLeadingEncoderPadding(&*(&value as *const <super::super::Foundation::IReference<u32> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<u32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn LeadingEncoderPadding<Impl: IAudioStreamDescriptor2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LeadingEncoderPadding() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTrailingEncoderPadding<Impl: IAudioStreamDescriptor2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTrailingEncoderPadding(&*(&value as *const <super::super::Foundation::IReference<u32> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<u32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TrailingEncoderPadding<Impl: IAudioStreamDescriptor2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrailingEncoderPadding() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAudioStreamDescriptor2, BASE_OFFSET>(),
            SetLeadingEncoderPadding: SetLeadingEncoderPadding::<Impl, IMPL_OFFSET>,
            LeadingEncoderPadding: LeadingEncoderPadding::<Impl, IMPL_OFFSET>,
            SetTrailingEncoderPadding: SetTrailingEncoderPadding::<Impl, IMPL_OFFSET>,
            TrailingEncoderPadding: TrailingEncoderPadding::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioStreamDescriptor2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioStreamDescriptor3Impl: Sized {
    fn Copy(&mut self) -> ::windows::core::Result<AudioStreamDescriptor>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAudioStreamDescriptor3 {
    const NAME: &'static str = "Windows.Media.Core.IAudioStreamDescriptor3";
}
#[cfg(feature = "implement_exclusive")]
impl IAudioStreamDescriptor3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioStreamDescriptor3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioStreamDescriptor3Vtbl {
        unsafe extern "system" fn Copy<Impl: IAudioStreamDescriptor3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Copy() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IAudioStreamDescriptor3, BASE_OFFSET>(), Copy: Copy::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioStreamDescriptor3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Media_MediaProperties", feature = "implement_exclusive"))]
pub trait IAudioStreamDescriptorFactoryImpl: Sized {
    fn Create(&mut self, encodingproperties: &::core::option::Option<super::MediaProperties::AudioEncodingProperties>) -> ::windows::core::Result<AudioStreamDescriptor>;
}
#[cfg(all(feature = "Media_MediaProperties", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAudioStreamDescriptorFactory {
    const NAME: &'static str = "Windows.Media.Core.IAudioStreamDescriptorFactory";
}
#[cfg(all(feature = "Media_MediaProperties", feature = "implement_exclusive"))]
impl IAudioStreamDescriptorFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioStreamDescriptorFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioStreamDescriptorFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IAudioStreamDescriptorFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encodingproperties: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&encodingproperties as *const <super::MediaProperties::AudioEncodingProperties as ::windows::core::Abi>::Abi as *const <super::MediaProperties::AudioEncodingProperties as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IAudioStreamDescriptorFactory, BASE_OFFSET>(), Create: Create::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioStreamDescriptorFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Media_Playback", feature = "implement_exclusive"))]
pub trait IAudioTrackImpl: Sized {
    fn OpenFailed(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AudioTrack, AudioTrackOpenFailedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveOpenFailed(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GetEncodingProperties(&mut self) -> ::windows::core::Result<super::MediaProperties::AudioEncodingProperties>;
    fn PlaybackItem(&mut self) -> ::windows::core::Result<super::Playback::MediaPlaybackItem>;
    fn Name(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SupportInfo(&mut self) -> ::windows::core::Result<AudioTrackSupportInfo>;
}
#[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Media_Playback", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAudioTrack {
    const NAME: &'static str = "Windows.Media.Core.IAudioTrack";
}
#[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Media_Playback", feature = "implement_exclusive"))]
impl IAudioTrackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioTrackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioTrackVtbl {
        unsafe extern "system" fn OpenFailed<Impl: IAudioTrackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenFailed(&*(&handler as *const <super::super::Foundation::TypedEventHandler<AudioTrack, AudioTrackOpenFailedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AudioTrack, AudioTrackOpenFailedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveOpenFailed<Impl: IAudioTrackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveOpenFailed(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetEncodingProperties<Impl: IAudioTrackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEncodingProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PlaybackItem<Impl: IAudioTrackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PlaybackItem() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: IAudioTrackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportInfo<Impl: IAudioTrackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAudioTrack, BASE_OFFSET>(),
            OpenFailed: OpenFailed::<Impl, IMPL_OFFSET>,
            RemoveOpenFailed: RemoveOpenFailed::<Impl, IMPL_OFFSET>,
            GetEncodingProperties: GetEncodingProperties::<Impl, IMPL_OFFSET>,
            PlaybackItem: PlaybackItem::<Impl, IMPL_OFFSET>,
            Name: Name::<Impl, IMPL_OFFSET>,
            SupportInfo: SupportInfo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioTrack as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioTrackOpenFailedEventArgsImpl: Sized {
    fn ExtendedError(&mut self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAudioTrackOpenFailedEventArgs {
    const NAME: &'static str = "Windows.Media.Core.IAudioTrackOpenFailedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IAudioTrackOpenFailedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioTrackOpenFailedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioTrackOpenFailedEventArgsVtbl {
        unsafe extern "system" fn ExtendedError<Impl: IAudioTrackOpenFailedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAudioTrackOpenFailedEventArgs, BASE_OFFSET>(),
            ExtendedError: ExtendedError::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioTrackOpenFailedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioTrackSupportInfoImpl: Sized {
    fn DecoderStatus(&mut self) -> ::windows::core::Result<MediaDecoderStatus>;
    fn Degradation(&mut self) -> ::windows::core::Result<AudioDecoderDegradation>;
    fn DegradationReason(&mut self) -> ::windows::core::Result<AudioDecoderDegradationReason>;
    fn MediaSourceStatus(&mut self) -> ::windows::core::Result<MediaSourceStatus>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAudioTrackSupportInfo {
    const NAME: &'static str = "Windows.Media.Core.IAudioTrackSupportInfo";
}
#[cfg(feature = "implement_exclusive")]
impl IAudioTrackSupportInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioTrackSupportInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioTrackSupportInfoVtbl {
        unsafe extern "system" fn DecoderStatus<Impl: IAudioTrackSupportInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MediaDecoderStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DecoderStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Degradation<Impl: IAudioTrackSupportInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AudioDecoderDegradation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Degradation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DegradationReason<Impl: IAudioTrackSupportInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AudioDecoderDegradationReason) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DegradationReason() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediaSourceStatus<Impl: IAudioTrackSupportInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MediaSourceStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MediaSourceStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAudioTrackSupportInfo, BASE_OFFSET>(),
            DecoderStatus: DecoderStatus::<Impl, IMPL_OFFSET>,
            Degradation: Degradation::<Impl, IMPL_OFFSET>,
            DegradationReason: DegradationReason::<Impl, IMPL_OFFSET>,
            MediaSourceStatus: MediaSourceStatus::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioTrackSupportInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IChapterCueImpl: Sized + IMediaCueImpl {
    fn SetTitle(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Title(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IChapterCue {
    const NAME: &'static str = "Windows.Media.Core.IChapterCue";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IChapterCueVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IChapterCueImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IChapterCueVtbl {
        unsafe extern "system" fn SetTitle<Impl: IChapterCueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTitle(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Title<Impl: IChapterCueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Title() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IChapterCue, BASE_OFFSET>(),
            SetTitle: SetTitle::<Impl, IMPL_OFFSET>,
            Title: Title::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IChapterCue as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ICodecInfoImpl: Sized {
    fn Kind(&mut self) -> ::windows::core::Result<CodecKind>;
    fn Category(&mut self) -> ::windows::core::Result<CodecCategory>;
    fn Subtypes(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn DisplayName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsTrusted(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICodecInfo {
    const NAME: &'static str = "Windows.Media.Core.ICodecInfo";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ICodecInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICodecInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICodecInfoVtbl {
        unsafe extern "system" fn Kind<Impl: ICodecInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CodecKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Kind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Category<Impl: ICodecInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CodecCategory) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Category() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Subtypes<Impl: ICodecInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Subtypes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayName<Impl: ICodecInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsTrusted<Impl: ICodecInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsTrusted() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICodecInfo, BASE_OFFSET>(),
            Kind: Kind::<Impl, IMPL_OFFSET>,
            Category: Category::<Impl, IMPL_OFFSET>,
            Subtypes: Subtypes::<Impl, IMPL_OFFSET>,
            DisplayName: DisplayName::<Impl, IMPL_OFFSET>,
            IsTrusted: IsTrusted::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICodecInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ICodecQueryImpl: Sized {
    fn FindAllAsync(&mut self, kind: CodecKind, category: CodecCategory, subtype: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<CodecInfo>>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICodecQuery {
    const NAME: &'static str = "Windows.Media.Core.ICodecQuery";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ICodecQueryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICodecQueryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICodecQueryVtbl {
        unsafe extern "system" fn FindAllAsync<Impl: ICodecQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, kind: CodecKind, category: CodecCategory, subtype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindAllAsync(kind, category, &*(&subtype as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICodecQuery, BASE_OFFSET>(), FindAllAsync: FindAllAsync::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICodecQuery as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICodecSubtypesStaticsImpl: Sized {
    fn VideoFormatDV25(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoFormatDV50(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoFormatDvc(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoFormatDvh1(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoFormatDvhD(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoFormatDvsd(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoFormatDvsl(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoFormatH263(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoFormatH264(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoFormatH265(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoFormatH264ES(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoFormatHevc(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoFormatHevcES(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoFormatM4S2(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoFormatMjpg(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoFormatMP43(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoFormatMP4S(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoFormatMP4V(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoFormatMpeg2(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoFormatVP80(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoFormatVP90(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoFormatMpg1(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoFormatMss1(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoFormatMss2(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoFormatWmv1(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoFormatWmv2(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoFormatWmv3(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoFormatWvc1(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoFormat420O(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AudioFormatAac(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AudioFormatAdts(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AudioFormatAlac(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AudioFormatAmrNB(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AudioFormatAmrWB(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AudioFormatAmrWP(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AudioFormatDolbyAC3(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AudioFormatDolbyAC3Spdif(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AudioFormatDolbyDDPlus(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AudioFormatDrm(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AudioFormatDts(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AudioFormatFlac(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AudioFormatFloat(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AudioFormatMP3(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AudioFormatMPeg(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AudioFormatMsp1(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AudioFormatOpus(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AudioFormatPcm(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AudioFormatWmaSpdif(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AudioFormatWMAudioLossless(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AudioFormatWMAudioV8(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AudioFormatWMAudioV9(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICodecSubtypesStatics {
    const NAME: &'static str = "Windows.Media.Core.ICodecSubtypesStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ICodecSubtypesStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICodecSubtypesStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICodecSubtypesStaticsVtbl {
        unsafe extern "system" fn VideoFormatDV25<Impl: ICodecSubtypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoFormatDV25() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoFormatDV50<Impl: ICodecSubtypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoFormatDV50() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoFormatDvc<Impl: ICodecSubtypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoFormatDvc() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoFormatDvh1<Impl: ICodecSubtypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoFormatDvh1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoFormatDvhD<Impl: ICodecSubtypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoFormatDvhD() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoFormatDvsd<Impl: ICodecSubtypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoFormatDvsd() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoFormatDvsl<Impl: ICodecSubtypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoFormatDvsl() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoFormatH263<Impl: ICodecSubtypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoFormatH263() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoFormatH264<Impl: ICodecSubtypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoFormatH264() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoFormatH265<Impl: ICodecSubtypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoFormatH265() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoFormatH264ES<Impl: ICodecSubtypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoFormatH264ES() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoFormatHevc<Impl: ICodecSubtypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoFormatHevc() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoFormatHevcES<Impl: ICodecSubtypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoFormatHevcES() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoFormatM4S2<Impl: ICodecSubtypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoFormatM4S2() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoFormatMjpg<Impl: ICodecSubtypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoFormatMjpg() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoFormatMP43<Impl: ICodecSubtypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoFormatMP43() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoFormatMP4S<Impl: ICodecSubtypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoFormatMP4S() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoFormatMP4V<Impl: ICodecSubtypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoFormatMP4V() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoFormatMpeg2<Impl: ICodecSubtypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoFormatMpeg2() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoFormatVP80<Impl: ICodecSubtypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoFormatVP80() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoFormatVP90<Impl: ICodecSubtypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoFormatVP90() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoFormatMpg1<Impl: ICodecSubtypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoFormatMpg1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoFormatMss1<Impl: ICodecSubtypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoFormatMss1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoFormatMss2<Impl: ICodecSubtypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoFormatMss2() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoFormatWmv1<Impl: ICodecSubtypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoFormatWmv1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoFormatWmv2<Impl: ICodecSubtypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoFormatWmv2() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoFormatWmv3<Impl: ICodecSubtypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoFormatWmv3() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoFormatWvc1<Impl: ICodecSubtypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoFormatWvc1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoFormat420O<Impl: ICodecSubtypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoFormat420O() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioFormatAac<Impl: ICodecSubtypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioFormatAac() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioFormatAdts<Impl: ICodecSubtypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioFormatAdts() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioFormatAlac<Impl: ICodecSubtypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioFormatAlac() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioFormatAmrNB<Impl: ICodecSubtypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioFormatAmrNB() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioFormatAmrWB<Impl: ICodecSubtypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioFormatAmrWB() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioFormatAmrWP<Impl: ICodecSubtypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioFormatAmrWP() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioFormatDolbyAC3<Impl: ICodecSubtypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioFormatDolbyAC3() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioFormatDolbyAC3Spdif<Impl: ICodecSubtypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioFormatDolbyAC3Spdif() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioFormatDolbyDDPlus<Impl: ICodecSubtypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioFormatDolbyDDPlus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioFormatDrm<Impl: ICodecSubtypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioFormatDrm() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioFormatDts<Impl: ICodecSubtypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioFormatDts() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioFormatFlac<Impl: ICodecSubtypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioFormatFlac() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioFormatFloat<Impl: ICodecSubtypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioFormatFloat() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioFormatMP3<Impl: ICodecSubtypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioFormatMP3() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioFormatMPeg<Impl: ICodecSubtypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioFormatMPeg() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioFormatMsp1<Impl: ICodecSubtypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioFormatMsp1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioFormatOpus<Impl: ICodecSubtypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioFormatOpus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioFormatPcm<Impl: ICodecSubtypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioFormatPcm() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioFormatWmaSpdif<Impl: ICodecSubtypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioFormatWmaSpdif() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioFormatWMAudioLossless<Impl: ICodecSubtypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioFormatWMAudioLossless() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioFormatWMAudioV8<Impl: ICodecSubtypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioFormatWMAudioV8() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioFormatWMAudioV9<Impl: ICodecSubtypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioFormatWMAudioV9() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICodecSubtypesStatics, BASE_OFFSET>(),
            VideoFormatDV25: VideoFormatDV25::<Impl, IMPL_OFFSET>,
            VideoFormatDV50: VideoFormatDV50::<Impl, IMPL_OFFSET>,
            VideoFormatDvc: VideoFormatDvc::<Impl, IMPL_OFFSET>,
            VideoFormatDvh1: VideoFormatDvh1::<Impl, IMPL_OFFSET>,
            VideoFormatDvhD: VideoFormatDvhD::<Impl, IMPL_OFFSET>,
            VideoFormatDvsd: VideoFormatDvsd::<Impl, IMPL_OFFSET>,
            VideoFormatDvsl: VideoFormatDvsl::<Impl, IMPL_OFFSET>,
            VideoFormatH263: VideoFormatH263::<Impl, IMPL_OFFSET>,
            VideoFormatH264: VideoFormatH264::<Impl, IMPL_OFFSET>,
            VideoFormatH265: VideoFormatH265::<Impl, IMPL_OFFSET>,
            VideoFormatH264ES: VideoFormatH264ES::<Impl, IMPL_OFFSET>,
            VideoFormatHevc: VideoFormatHevc::<Impl, IMPL_OFFSET>,
            VideoFormatHevcES: VideoFormatHevcES::<Impl, IMPL_OFFSET>,
            VideoFormatM4S2: VideoFormatM4S2::<Impl, IMPL_OFFSET>,
            VideoFormatMjpg: VideoFormatMjpg::<Impl, IMPL_OFFSET>,
            VideoFormatMP43: VideoFormatMP43::<Impl, IMPL_OFFSET>,
            VideoFormatMP4S: VideoFormatMP4S::<Impl, IMPL_OFFSET>,
            VideoFormatMP4V: VideoFormatMP4V::<Impl, IMPL_OFFSET>,
            VideoFormatMpeg2: VideoFormatMpeg2::<Impl, IMPL_OFFSET>,
            VideoFormatVP80: VideoFormatVP80::<Impl, IMPL_OFFSET>,
            VideoFormatVP90: VideoFormatVP90::<Impl, IMPL_OFFSET>,
            VideoFormatMpg1: VideoFormatMpg1::<Impl, IMPL_OFFSET>,
            VideoFormatMss1: VideoFormatMss1::<Impl, IMPL_OFFSET>,
            VideoFormatMss2: VideoFormatMss2::<Impl, IMPL_OFFSET>,
            VideoFormatWmv1: VideoFormatWmv1::<Impl, IMPL_OFFSET>,
            VideoFormatWmv2: VideoFormatWmv2::<Impl, IMPL_OFFSET>,
            VideoFormatWmv3: VideoFormatWmv3::<Impl, IMPL_OFFSET>,
            VideoFormatWvc1: VideoFormatWvc1::<Impl, IMPL_OFFSET>,
            VideoFormat420O: VideoFormat420O::<Impl, IMPL_OFFSET>,
            AudioFormatAac: AudioFormatAac::<Impl, IMPL_OFFSET>,
            AudioFormatAdts: AudioFormatAdts::<Impl, IMPL_OFFSET>,
            AudioFormatAlac: AudioFormatAlac::<Impl, IMPL_OFFSET>,
            AudioFormatAmrNB: AudioFormatAmrNB::<Impl, IMPL_OFFSET>,
            AudioFormatAmrWB: AudioFormatAmrWB::<Impl, IMPL_OFFSET>,
            AudioFormatAmrWP: AudioFormatAmrWP::<Impl, IMPL_OFFSET>,
            AudioFormatDolbyAC3: AudioFormatDolbyAC3::<Impl, IMPL_OFFSET>,
            AudioFormatDolbyAC3Spdif: AudioFormatDolbyAC3Spdif::<Impl, IMPL_OFFSET>,
            AudioFormatDolbyDDPlus: AudioFormatDolbyDDPlus::<Impl, IMPL_OFFSET>,
            AudioFormatDrm: AudioFormatDrm::<Impl, IMPL_OFFSET>,
            AudioFormatDts: AudioFormatDts::<Impl, IMPL_OFFSET>,
            AudioFormatFlac: AudioFormatFlac::<Impl, IMPL_OFFSET>,
            AudioFormatFloat: AudioFormatFloat::<Impl, IMPL_OFFSET>,
            AudioFormatMP3: AudioFormatMP3::<Impl, IMPL_OFFSET>,
            AudioFormatMPeg: AudioFormatMPeg::<Impl, IMPL_OFFSET>,
            AudioFormatMsp1: AudioFormatMsp1::<Impl, IMPL_OFFSET>,
            AudioFormatOpus: AudioFormatOpus::<Impl, IMPL_OFFSET>,
            AudioFormatPcm: AudioFormatPcm::<Impl, IMPL_OFFSET>,
            AudioFormatWmaSpdif: AudioFormatWmaSpdif::<Impl, IMPL_OFFSET>,
            AudioFormatWMAudioLossless: AudioFormatWMAudioLossless::<Impl, IMPL_OFFSET>,
            AudioFormatWMAudioV8: AudioFormatWMAudioV8::<Impl, IMPL_OFFSET>,
            AudioFormatWMAudioV9: AudioFormatWMAudioV9::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICodecSubtypesStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IDataCueImpl: Sized + IMediaCueImpl {
    fn SetData(&mut self, value: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
    fn Data(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDataCue {
    const NAME: &'static str = "Windows.Media.Core.IDataCue";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IDataCueVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataCueImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDataCueVtbl {
        unsafe extern "system" fn SetData<Impl: IDataCueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetData(&*(&value as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Data<Impl: IDataCueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Data() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDataCue, BASE_OFFSET>(),
            SetData: SetData::<Impl, IMPL_OFFSET>,
            Data: Data::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDataCue as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IDataCue2Impl: Sized + IDataCueImpl + IMediaCueImpl {
    fn Properties(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::PropertySet>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDataCue2 {
    const NAME: &'static str = "Windows.Media.Core.IDataCue2";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IDataCue2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataCue2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDataCue2Vtbl {
        unsafe extern "system" fn Properties<Impl: IDataCue2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IDataCue2, BASE_OFFSET>(), Properties: Properties::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDataCue2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFaceDetectedEventArgsImpl: Sized {
    fn ResultFrame(&mut self) -> ::windows::core::Result<FaceDetectionEffectFrame>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFaceDetectedEventArgs {
    const NAME: &'static str = "Windows.Media.Core.IFaceDetectedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IFaceDetectedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaceDetectedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFaceDetectedEventArgsVtbl {
        unsafe extern "system" fn ResultFrame<Impl: IFaceDetectedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResultFrame() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IFaceDetectedEventArgs, BASE_OFFSET>(), ResultFrame: ResultFrame::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaceDetectedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IFaceDetectionEffectImpl: Sized + IMediaExtensionImpl {
    fn SetEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn Enabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetDesiredDetectionInterval(&mut self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn DesiredDetectionInterval(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn FaceDetected(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<FaceDetectionEffect, FaceDetectedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveFaceDetected(&mut self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IFaceDetectionEffect {
    const NAME: &'static str = "Windows.Media.Core.IFaceDetectionEffect";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IFaceDetectionEffectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaceDetectionEffectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFaceDetectionEffectVtbl {
        unsafe extern "system" fn SetEnabled<Impl: IFaceDetectionEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnabled(value).into()
        }
        unsafe extern "system" fn Enabled<Impl: IFaceDetectionEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Enabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredDetectionInterval<Impl: IFaceDetectionEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDesiredDetectionInterval(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DesiredDetectionInterval<Impl: IFaceDetectionEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DesiredDetectionInterval() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FaceDetected<Impl: IFaceDetectionEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FaceDetected(&*(&handler as *const <super::super::Foundation::TypedEventHandler<FaceDetectionEffect, FaceDetectedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<FaceDetectionEffect, FaceDetectedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveFaceDetected<Impl: IFaceDetectionEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveFaceDetected(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFaceDetectionEffect, BASE_OFFSET>(),
            SetEnabled: SetEnabled::<Impl, IMPL_OFFSET>,
            Enabled: Enabled::<Impl, IMPL_OFFSET>,
            SetDesiredDetectionInterval: SetDesiredDetectionInterval::<Impl, IMPL_OFFSET>,
            DesiredDetectionInterval: DesiredDetectionInterval::<Impl, IMPL_OFFSET>,
            FaceDetected: FaceDetected::<Impl, IMPL_OFFSET>,
            RemoveFaceDetected: RemoveFaceDetected::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaceDetectionEffect as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects", feature = "implement_exclusive"))]
pub trait IFaceDetectionEffectDefinitionImpl: Sized + IVideoEffectDefinitionImpl {
    fn SetDetectionMode(&mut self, value: FaceDetectionMode) -> ::windows::core::Result<()>;
    fn DetectionMode(&mut self) -> ::windows::core::Result<FaceDetectionMode>;
    fn SetSynchronousDetectionEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn SynchronousDetectionEnabled(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IFaceDetectionEffectDefinition {
    const NAME: &'static str = "Windows.Media.Core.IFaceDetectionEffectDefinition";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects", feature = "implement_exclusive"))]
impl IFaceDetectionEffectDefinitionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaceDetectionEffectDefinitionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFaceDetectionEffectDefinitionVtbl {
        unsafe extern "system" fn SetDetectionMode<Impl: IFaceDetectionEffectDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: FaceDetectionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDetectionMode(value).into()
        }
        unsafe extern "system" fn DetectionMode<Impl: IFaceDetectionEffectDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FaceDetectionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DetectionMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSynchronousDetectionEnabled<Impl: IFaceDetectionEffectDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSynchronousDetectionEnabled(value).into()
        }
        unsafe extern "system" fn SynchronousDetectionEnabled<Impl: IFaceDetectionEffectDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SynchronousDetectionEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFaceDetectionEffectDefinition, BASE_OFFSET>(),
            SetDetectionMode: SetDetectionMode::<Impl, IMPL_OFFSET>,
            DetectionMode: DetectionMode::<Impl, IMPL_OFFSET>,
            SetSynchronousDetectionEnabled: SetSynchronousDetectionEnabled::<Impl, IMPL_OFFSET>,
            SynchronousDetectionEnabled: SynchronousDetectionEnabled::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaceDetectionEffectDefinition as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_FaceAnalysis", feature = "implement_exclusive"))]
pub trait IFaceDetectionEffectFrameImpl: Sized + IClosableImpl + IMediaFrameImpl {
    fn DetectedFaces(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::FaceAnalysis::DetectedFace>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_FaceAnalysis", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IFaceDetectionEffectFrame {
    const NAME: &'static str = "Windows.Media.Core.IFaceDetectionEffectFrame";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_FaceAnalysis", feature = "implement_exclusive"))]
impl IFaceDetectionEffectFrameVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFaceDetectionEffectFrameImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFaceDetectionEffectFrameVtbl {
        unsafe extern "system" fn DetectedFaces<Impl: IFaceDetectionEffectFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DetectedFaces() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFaceDetectionEffectFrame, BASE_OFFSET>(),
            DetectedFaces: DetectedFaces::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFaceDetectionEffectFrame as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHighDynamicRangeControlImpl: Sized {
    fn SetEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn Enabled(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHighDynamicRangeControl {
    const NAME: &'static str = "Windows.Media.Core.IHighDynamicRangeControl";
}
#[cfg(feature = "implement_exclusive")]
impl IHighDynamicRangeControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHighDynamicRangeControlImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHighDynamicRangeControlVtbl {
        unsafe extern "system" fn SetEnabled<Impl: IHighDynamicRangeControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnabled(value).into()
        }
        unsafe extern "system" fn Enabled<Impl: IHighDynamicRangeControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Enabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHighDynamicRangeControl, BASE_OFFSET>(),
            SetEnabled: SetEnabled::<Impl, IMPL_OFFSET>,
            Enabled: Enabled::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHighDynamicRangeControl as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Devices_Core", feature = "implement_exclusive"))]
pub trait IHighDynamicRangeOutputImpl: Sized {
    fn Certainty(&mut self) -> ::windows::core::Result<f64>;
    fn FrameControllers(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::Devices::Core::FrameController>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Devices_Core", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHighDynamicRangeOutput {
    const NAME: &'static str = "Windows.Media.Core.IHighDynamicRangeOutput";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Devices_Core", feature = "implement_exclusive"))]
impl IHighDynamicRangeOutputVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHighDynamicRangeOutputImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHighDynamicRangeOutputVtbl {
        unsafe extern "system" fn Certainty<Impl: IHighDynamicRangeOutputImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Certainty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FrameControllers<Impl: IHighDynamicRangeOutputImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FrameControllers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHighDynamicRangeOutput, BASE_OFFSET>(),
            Certainty: Certainty::<Impl, IMPL_OFFSET>,
            FrameControllers: FrameControllers::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHighDynamicRangeOutput as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Graphics_Imaging", feature = "implement_exclusive"))]
pub trait IImageCueImpl: Sized + IMediaCueImpl {
    fn Position(&mut self) -> ::windows::core::Result<TimedTextPoint>;
    fn SetPosition(&mut self, value: &TimedTextPoint) -> ::windows::core::Result<()>;
    fn Extent(&mut self) -> ::windows::core::Result<TimedTextSize>;
    fn SetExtent(&mut self, value: &TimedTextSize) -> ::windows::core::Result<()>;
    fn SetSoftwareBitmap(&mut self, value: &::core::option::Option<super::super::Graphics::Imaging::SoftwareBitmap>) -> ::windows::core::Result<()>;
    fn SoftwareBitmap(&mut self) -> ::windows::core::Result<super::super::Graphics::Imaging::SoftwareBitmap>;
}
#[cfg(all(feature = "Foundation", feature = "Graphics_Imaging", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IImageCue {
    const NAME: &'static str = "Windows.Media.Core.IImageCue";
}
#[cfg(all(feature = "Foundation", feature = "Graphics_Imaging", feature = "implement_exclusive"))]
impl IImageCueVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IImageCueImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IImageCueVtbl {
        unsafe extern "system" fn Position<Impl: IImageCueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut TimedTextPoint) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPosition<Impl: IImageCueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: TimedTextPoint) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPosition(&*(&value as *const <TimedTextPoint as ::windows::core::Abi>::Abi as *const <TimedTextPoint as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Extent<Impl: IImageCueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut TimedTextSize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Extent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExtent<Impl: IImageCueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: TimedTextSize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExtent(&*(&value as *const <TimedTextSize as ::windows::core::Abi>::Abi as *const <TimedTextSize as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetSoftwareBitmap<Impl: IImageCueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSoftwareBitmap(&*(&value as *const <super::super::Graphics::Imaging::SoftwareBitmap as ::windows::core::Abi>::Abi as *const <super::super::Graphics::Imaging::SoftwareBitmap as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SoftwareBitmap<Impl: IImageCueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IImageCue, BASE_OFFSET>(),
            Position: Position::<Impl, IMPL_OFFSET>,
            SetPosition: SetPosition::<Impl, IMPL_OFFSET>,
            Extent: Extent::<Impl, IMPL_OFFSET>,
            SetExtent: SetExtent::<Impl, IMPL_OFFSET>,
            SetSoftwareBitmap: SetSoftwareBitmap::<Impl, IMPL_OFFSET>,
            SoftwareBitmap: SoftwareBitmap::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IImageCue as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IInitializeMediaStreamSourceRequestedEventArgsImpl: Sized {
    fn Source(&mut self) -> ::windows::core::Result<MediaStreamSource>;
    fn RandomAccessStream(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStream>;
    fn GetDeferral(&mut self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInitializeMediaStreamSourceRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.Core.IInitializeMediaStreamSourceRequestedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IInitializeMediaStreamSourceRequestedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInitializeMediaStreamSourceRequestedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInitializeMediaStreamSourceRequestedEventArgsVtbl {
        unsafe extern "system" fn Source<Impl: IInitializeMediaStreamSourceRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Source() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RandomAccessStream<Impl: IInitializeMediaStreamSourceRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RandomAccessStream() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeferral<Impl: IInitializeMediaStreamSourceRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInitializeMediaStreamSourceRequestedEventArgs, BASE_OFFSET>(),
            Source: Source::<Impl, IMPL_OFFSET>,
            RandomAccessStream: RandomAccessStream::<Impl, IMPL_OFFSET>,
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInitializeMediaStreamSourceRequestedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Graphics_Imaging", feature = "implement_exclusive"))]
pub trait ILowLightFusionResultImpl: Sized {
    fn Frame(&mut self) -> ::windows::core::Result<super::super::Graphics::Imaging::SoftwareBitmap>;
}
#[cfg(all(feature = "Graphics_Imaging", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILowLightFusionResult {
    const NAME: &'static str = "Windows.Media.Core.ILowLightFusionResult";
}
#[cfg(all(feature = "Graphics_Imaging", feature = "implement_exclusive"))]
impl ILowLightFusionResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILowLightFusionResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILowLightFusionResultVtbl {
        unsafe extern "system" fn Frame<Impl: ILowLightFusionResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Frame() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ILowLightFusionResult, BASE_OFFSET>(), Frame: Frame::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILowLightFusionResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Graphics_Imaging", feature = "implement_exclusive"))]
pub trait ILowLightFusionStaticsImpl: Sized {
    fn SupportedBitmapPixelFormats(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Graphics::Imaging::BitmapPixelFormat>>;
    fn MaxSupportedFrameCount(&mut self) -> ::windows::core::Result<i32>;
    fn FuseAsync(&mut self, frameset: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Graphics::Imaging::SoftwareBitmap>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<LowLightFusionResult, f64>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Graphics_Imaging", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILowLightFusionStatics {
    const NAME: &'static str = "Windows.Media.Core.ILowLightFusionStatics";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Graphics_Imaging", feature = "implement_exclusive"))]
impl ILowLightFusionStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILowLightFusionStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILowLightFusionStaticsVtbl {
        unsafe extern "system" fn SupportedBitmapPixelFormats<Impl: ILowLightFusionStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedBitmapPixelFormats() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxSupportedFrameCount<Impl: ILowLightFusionStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxSupportedFrameCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FuseAsync<Impl: ILowLightFusionStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, frameset: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FuseAsync(&*(&frameset as *const <super::super::Foundation::Collections::IIterable<super::super::Graphics::Imaging::SoftwareBitmap> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<super::super::Graphics::Imaging::SoftwareBitmap> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILowLightFusionStatics, BASE_OFFSET>(),
            SupportedBitmapPixelFormats: SupportedBitmapPixelFormats::<Impl, IMPL_OFFSET>,
            MaxSupportedFrameCount: MaxSupportedFrameCount::<Impl, IMPL_OFFSET>,
            FuseAsync: FuseAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILowLightFusionStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMediaBinderImpl: Sized {
    fn Binding(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaBinder, MediaBindingEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveBinding(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Token(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetToken(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Source(&mut self) -> ::windows::core::Result<MediaSource>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaBinder {
    const NAME: &'static str = "Windows.Media.Core.IMediaBinder";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMediaBinderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaBinderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaBinderVtbl {
        unsafe extern "system" fn Binding<Impl: IMediaBinderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Binding(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MediaBinder, MediaBindingEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MediaBinder, MediaBindingEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveBinding<Impl: IMediaBinderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveBinding(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Token<Impl: IMediaBinderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Token() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetToken<Impl: IMediaBinderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetToken(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Source<Impl: IMediaBinderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Source() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaBinder, BASE_OFFSET>(),
            Binding: Binding::<Impl, IMPL_OFFSET>,
            RemoveBinding: RemoveBinding::<Impl, IMPL_OFFSET>,
            Token: Token::<Impl, IMPL_OFFSET>,
            SetToken: SetToken::<Impl, IMPL_OFFSET>,
            Source: Source::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaBinder as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IMediaBindingEventArgsImpl: Sized {
    fn Canceled(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaBindingEventArgs, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCanceled(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn MediaBinder(&mut self) -> ::windows::core::Result<MediaBinder>;
    fn GetDeferral(&mut self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
    fn SetUri(&mut self, uri: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn SetStream(&mut self, stream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>, contenttype: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SetStreamReference(&mut self, stream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStreamReference>, contenttype: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaBindingEventArgs {
    const NAME: &'static str = "Windows.Media.Core.IMediaBindingEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IMediaBindingEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaBindingEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaBindingEventArgsVtbl {
        unsafe extern "system" fn Canceled<Impl: IMediaBindingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Canceled(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MediaBindingEventArgs, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MediaBindingEventArgs, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCanceled<Impl: IMediaBindingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveCanceled(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MediaBinder<Impl: IMediaBindingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MediaBinder() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeferral<Impl: IMediaBindingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetUri<Impl: IMediaBindingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUri(&*(&uri as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetStream<Impl: IMediaBindingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, contenttype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStream(&*(&stream as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::DefaultType>::DefaultType), &*(&contenttype as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetStreamReference<Impl: IMediaBindingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, contenttype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStreamReference(&*(&stream as *const <super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::DefaultType>::DefaultType), &*(&contenttype as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaBindingEventArgs, BASE_OFFSET>(),
            Canceled: Canceled::<Impl, IMPL_OFFSET>,
            RemoveCanceled: RemoveCanceled::<Impl, IMPL_OFFSET>,
            MediaBinder: MediaBinder::<Impl, IMPL_OFFSET>,
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
            SetUri: SetUri::<Impl, IMPL_OFFSET>,
            SetStream: SetStream::<Impl, IMPL_OFFSET>,
            SetStreamReference: SetStreamReference::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaBindingEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Media_Streaming_Adaptive", feature = "Storage", feature = "implement_exclusive"))]
pub trait IMediaBindingEventArgs2Impl: Sized {
    fn SetAdaptiveMediaSource(&mut self, mediasource: &::core::option::Option<super::Streaming::Adaptive::AdaptiveMediaSource>) -> ::windows::core::Result<()>;
    fn SetStorageFile(&mut self, file: &::core::option::Option<super::super::Storage::IStorageFile>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Media_Streaming_Adaptive", feature = "Storage", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaBindingEventArgs2 {
    const NAME: &'static str = "Windows.Media.Core.IMediaBindingEventArgs2";
}
#[cfg(all(feature = "Media_Streaming_Adaptive", feature = "Storage", feature = "implement_exclusive"))]
impl IMediaBindingEventArgs2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaBindingEventArgs2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaBindingEventArgs2Vtbl {
        unsafe extern "system" fn SetAdaptiveMediaSource<Impl: IMediaBindingEventArgs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mediasource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAdaptiveMediaSource(&*(&mediasource as *const <super::Streaming::Adaptive::AdaptiveMediaSource as ::windows::core::Abi>::Abi as *const <super::Streaming::Adaptive::AdaptiveMediaSource as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetStorageFile<Impl: IMediaBindingEventArgs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStorageFile(&*(&file as *const <super::super::Storage::IStorageFile as ::windows::core::Abi>::Abi as *const <super::super::Storage::IStorageFile as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaBindingEventArgs2, BASE_OFFSET>(),
            SetAdaptiveMediaSource: SetAdaptiveMediaSource::<Impl, IMPL_OFFSET>,
            SetStorageFile: SetStorageFile::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaBindingEventArgs2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Networking_BackgroundTransfer", feature = "implement_exclusive"))]
pub trait IMediaBindingEventArgs3Impl: Sized {
    fn SetDownloadOperation(&mut self, downloadoperation: &::core::option::Option<super::super::Networking::BackgroundTransfer::DownloadOperation>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Networking_BackgroundTransfer", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaBindingEventArgs3 {
    const NAME: &'static str = "Windows.Media.Core.IMediaBindingEventArgs3";
}
#[cfg(all(feature = "Networking_BackgroundTransfer", feature = "implement_exclusive"))]
impl IMediaBindingEventArgs3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaBindingEventArgs3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaBindingEventArgs3Vtbl {
        unsafe extern "system" fn SetDownloadOperation<Impl: IMediaBindingEventArgs3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, downloadoperation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDownloadOperation(&*(&downloadoperation as *const <super::super::Networking::BackgroundTransfer::DownloadOperation as ::windows::core::Abi>::Abi as *const <super::super::Networking::BackgroundTransfer::DownloadOperation as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaBindingEventArgs3, BASE_OFFSET>(),
            SetDownloadOperation: SetDownloadOperation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaBindingEventArgs3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait IMediaCueImpl: Sized {
    fn SetStartTime(&mut self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn StartTime(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetDuration(&mut self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn Duration(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetId(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Id(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IMediaCue {
    const NAME: &'static str = "Windows.Media.Core.IMediaCue";
}
#[cfg(feature = "Foundation")]
impl IMediaCueVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaCueImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaCueVtbl {
        unsafe extern "system" fn SetStartTime<Impl: IMediaCueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStartTime(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StartTime<Impl: IMediaCueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDuration<Impl: IMediaCueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDuration(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Duration<Impl: IMediaCueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetId<Impl: IMediaCueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetId(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Id<Impl: IMediaCueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaCue, BASE_OFFSET>(),
            SetStartTime: SetStartTime::<Impl, IMPL_OFFSET>,
            StartTime: StartTime::<Impl, IMPL_OFFSET>,
            SetDuration: SetDuration::<Impl, IMPL_OFFSET>,
            Duration: Duration::<Impl, IMPL_OFFSET>,
            SetId: SetId::<Impl, IMPL_OFFSET>,
            Id: Id::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaCue as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaCueEventArgsImpl: Sized {
    fn Cue(&mut self) -> ::windows::core::Result<IMediaCue>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaCueEventArgs {
    const NAME: &'static str = "Windows.Media.Core.IMediaCueEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaCueEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaCueEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaCueEventArgsVtbl {
        unsafe extern "system" fn Cue<Impl: IMediaCueEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Cue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaCueEventArgs, BASE_OFFSET>(), Cue: Cue::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaCueEventArgs as ::windows::core::Interface>::IID
    }
}
pub trait IMediaSourceImpl: Sized {}
impl ::windows::core::RuntimeName for IMediaSource {
    const NAME: &'static str = "Windows.Media.Core.IMediaSource";
}
impl IMediaSourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaSourceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaSourceVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaSource, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaSource as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Playback", feature = "implement_exclusive"))]
pub trait IMediaSource2Impl: Sized + IClosableImpl + IMediaPlaybackSourceImpl {
    fn OpenOperationCompleted(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaSource, MediaSourceOpenOperationCompletedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveOpenOperationCompleted(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CustomProperties(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet>;
    fn Duration(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>;
    fn IsOpen(&mut self) -> ::windows::core::Result<bool>;
    fn ExternalTimedTextSources(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IObservableVector<TimedTextSource>>;
    fn ExternalTimedMetadataTracks(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IObservableVector<TimedMetadataTrack>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Playback", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaSource2 {
    const NAME: &'static str = "Windows.Media.Core.IMediaSource2";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Playback", feature = "implement_exclusive"))]
impl IMediaSource2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaSource2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaSource2Vtbl {
        unsafe extern "system" fn OpenOperationCompleted<Impl: IMediaSource2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenOperationCompleted(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MediaSource, MediaSourceOpenOperationCompletedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MediaSource, MediaSourceOpenOperationCompletedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveOpenOperationCompleted<Impl: IMediaSource2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveOpenOperationCompleted(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CustomProperties<Impl: IMediaSource2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CustomProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Duration<Impl: IMediaSource2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsOpen<Impl: IMediaSource2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsOpen() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExternalTimedTextSources<Impl: IMediaSource2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExternalTimedTextSources() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExternalTimedMetadataTracks<Impl: IMediaSource2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExternalTimedMetadataTracks() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaSource2, BASE_OFFSET>(),
            OpenOperationCompleted: OpenOperationCompleted::<Impl, IMPL_OFFSET>,
            RemoveOpenOperationCompleted: RemoveOpenOperationCompleted::<Impl, IMPL_OFFSET>,
            CustomProperties: CustomProperties::<Impl, IMPL_OFFSET>,
            Duration: Duration::<Impl, IMPL_OFFSET>,
            IsOpen: IsOpen::<Impl, IMPL_OFFSET>,
            ExternalTimedTextSources: ExternalTimedTextSources::<Impl, IMPL_OFFSET>,
            ExternalTimedMetadataTracks: ExternalTimedMetadataTracks::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaSource2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Playback", feature = "implement_exclusive"))]
pub trait IMediaSource3Impl: Sized + IClosableImpl + IMediaPlaybackSourceImpl + IMediaSource2Impl {
    fn StateChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaSource, MediaSourceStateChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStateChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn State(&mut self) -> ::windows::core::Result<MediaSourceState>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Playback", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaSource3 {
    const NAME: &'static str = "Windows.Media.Core.IMediaSource3";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Playback", feature = "implement_exclusive"))]
impl IMediaSource3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaSource3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaSource3Vtbl {
        unsafe extern "system" fn StateChanged<Impl: IMediaSource3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StateChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MediaSource, MediaSourceStateChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MediaSource, MediaSourceStateChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStateChanged<Impl: IMediaSource3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveStateChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn State<Impl: IMediaSource3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MediaSourceState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IMediaSource3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaSource3, BASE_OFFSET>(),
            StateChanged: StateChanged::<Impl, IMPL_OFFSET>,
            RemoveStateChanged: RemoveStateChanged::<Impl, IMPL_OFFSET>,
            State: State::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaSource3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Playback", feature = "Media_Streaming_Adaptive", feature = "implement_exclusive"))]
pub trait IMediaSource4Impl: Sized + IClosableImpl + IMediaPlaybackSourceImpl + IMediaSource2Impl + IMediaSource3Impl {
    fn AdaptiveMediaSource(&mut self) -> ::windows::core::Result<super::Streaming::Adaptive::AdaptiveMediaSource>;
    fn MediaStreamSource(&mut self) -> ::windows::core::Result<MediaStreamSource>;
    fn MseStreamSource(&mut self) -> ::windows::core::Result<MseStreamSource>;
    fn Uri(&mut self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn OpenAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Playback", feature = "Media_Streaming_Adaptive", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaSource4 {
    const NAME: &'static str = "Windows.Media.Core.IMediaSource4";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Playback", feature = "Media_Streaming_Adaptive", feature = "implement_exclusive"))]
impl IMediaSource4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaSource4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaSource4Vtbl {
        unsafe extern "system" fn AdaptiveMediaSource<Impl: IMediaSource4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdaptiveMediaSource() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediaStreamSource<Impl: IMediaSource4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MediaStreamSource() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MseStreamSource<Impl: IMediaSource4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MseStreamSource() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Uri<Impl: IMediaSource4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Uri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenAsync<Impl: IMediaSource4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaSource4, BASE_OFFSET>(),
            AdaptiveMediaSource: AdaptiveMediaSource::<Impl, IMPL_OFFSET>,
            MediaStreamSource: MediaStreamSource::<Impl, IMPL_OFFSET>,
            MseStreamSource: MseStreamSource::<Impl, IMPL_OFFSET>,
            Uri: Uri::<Impl, IMPL_OFFSET>,
            OpenAsync: OpenAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaSource4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Networking_BackgroundTransfer", feature = "implement_exclusive"))]
pub trait IMediaSource5Impl: Sized {
    fn DownloadOperation(&mut self) -> ::windows::core::Result<super::super::Networking::BackgroundTransfer::DownloadOperation>;
}
#[cfg(all(feature = "Networking_BackgroundTransfer", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaSource5 {
    const NAME: &'static str = "Windows.Media.Core.IMediaSource5";
}
#[cfg(all(feature = "Networking_BackgroundTransfer", feature = "implement_exclusive"))]
impl IMediaSource5Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaSource5Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaSource5Vtbl {
        unsafe extern "system" fn DownloadOperation<Impl: IMediaSource5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DownloadOperation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaSource5, BASE_OFFSET>(),
            DownloadOperation: DownloadOperation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaSource5 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMediaSourceAppServiceConnectionImpl: Sized {
    fn InitializeMediaStreamSourceRequested(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaSourceAppServiceConnection, InitializeMediaStreamSourceRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveInitializeMediaStreamSourceRequested(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Start(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaSourceAppServiceConnection {
    const NAME: &'static str = "Windows.Media.Core.IMediaSourceAppServiceConnection";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMediaSourceAppServiceConnectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaSourceAppServiceConnectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaSourceAppServiceConnectionVtbl {
        unsafe extern "system" fn InitializeMediaStreamSourceRequested<Impl: IMediaSourceAppServiceConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitializeMediaStreamSourceRequested(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MediaSourceAppServiceConnection, InitializeMediaStreamSourceRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MediaSourceAppServiceConnection, InitializeMediaStreamSourceRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveInitializeMediaStreamSourceRequested<Impl: IMediaSourceAppServiceConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveInitializeMediaStreamSourceRequested(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Start<Impl: IMediaSourceAppServiceConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Start().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaSourceAppServiceConnection, BASE_OFFSET>(),
            InitializeMediaStreamSourceRequested: InitializeMediaStreamSourceRequested::<Impl, IMPL_OFFSET>,
            RemoveInitializeMediaStreamSourceRequested: RemoveInitializeMediaStreamSourceRequested::<Impl, IMPL_OFFSET>,
            Start: Start::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaSourceAppServiceConnection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "ApplicationModel_AppService", feature = "implement_exclusive"))]
pub trait IMediaSourceAppServiceConnectionFactoryImpl: Sized {
    fn Create(&mut self, appserviceconnection: &::core::option::Option<super::super::ApplicationModel::AppService::AppServiceConnection>) -> ::windows::core::Result<MediaSourceAppServiceConnection>;
}
#[cfg(all(feature = "ApplicationModel_AppService", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaSourceAppServiceConnectionFactory {
    const NAME: &'static str = "Windows.Media.Core.IMediaSourceAppServiceConnectionFactory";
}
#[cfg(all(feature = "ApplicationModel_AppService", feature = "implement_exclusive"))]
impl IMediaSourceAppServiceConnectionFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaSourceAppServiceConnectionFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaSourceAppServiceConnectionFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IMediaSourceAppServiceConnectionFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appserviceconnection: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&appserviceconnection as *const <super::super::ApplicationModel::AppService::AppServiceConnection as ::windows::core::Abi>::Abi as *const <super::super::ApplicationModel::AppService::AppServiceConnection as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaSourceAppServiceConnectionFactory, BASE_OFFSET>(),
            Create: Create::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaSourceAppServiceConnectionFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaSourceErrorImpl: Sized {
    fn ExtendedError(&mut self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaSourceError {
    const NAME: &'static str = "Windows.Media.Core.IMediaSourceError";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaSourceErrorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaSourceErrorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaSourceErrorVtbl {
        unsafe extern "system" fn ExtendedError<Impl: IMediaSourceErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaSourceError, BASE_OFFSET>(), ExtendedError: ExtendedError::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaSourceError as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaSourceOpenOperationCompletedEventArgsImpl: Sized {
    fn Error(&mut self) -> ::windows::core::Result<MediaSourceError>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaSourceOpenOperationCompletedEventArgs {
    const NAME: &'static str = "Windows.Media.Core.IMediaSourceOpenOperationCompletedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaSourceOpenOperationCompletedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaSourceOpenOperationCompletedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaSourceOpenOperationCompletedEventArgsVtbl {
        unsafe extern "system" fn Error<Impl: IMediaSourceOpenOperationCompletedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Error() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaSourceOpenOperationCompletedEventArgs, BASE_OFFSET>(),
            Error: Error::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaSourceOpenOperationCompletedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaSourceStateChangedEventArgsImpl: Sized {
    fn OldState(&mut self) -> ::windows::core::Result<MediaSourceState>;
    fn NewState(&mut self) -> ::windows::core::Result<MediaSourceState>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaSourceStateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Core.IMediaSourceStateChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaSourceStateChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaSourceStateChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaSourceStateChangedEventArgsVtbl {
        unsafe extern "system" fn OldState<Impl: IMediaSourceStateChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MediaSourceState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OldState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NewState<Impl: IMediaSourceStateChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MediaSourceState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NewState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaSourceStateChangedEventArgs, BASE_OFFSET>(),
            OldState: OldState::<Impl, IMPL_OFFSET>,
            NewState: NewState::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaSourceStateChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Media_Streaming_Adaptive", feature = "Storage", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IMediaSourceStaticsImpl: Sized {
    fn CreateFromAdaptiveMediaSource(&mut self, mediasource: &::core::option::Option<super::Streaming::Adaptive::AdaptiveMediaSource>) -> ::windows::core::Result<MediaSource>;
    fn CreateFromMediaStreamSource(&mut self, mediasource: &::core::option::Option<MediaStreamSource>) -> ::windows::core::Result<MediaSource>;
    fn CreateFromMseStreamSource(&mut self, mediasource: &::core::option::Option<MseStreamSource>) -> ::windows::core::Result<MediaSource>;
    fn CreateFromIMediaSource(&mut self, mediasource: &::core::option::Option<IMediaSource>) -> ::windows::core::Result<MediaSource>;
    fn CreateFromStorageFile(&mut self, file: &::core::option::Option<super::super::Storage::IStorageFile>) -> ::windows::core::Result<MediaSource>;
    fn CreateFromStream(&mut self, stream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>, contenttype: &::windows::core::HSTRING) -> ::windows::core::Result<MediaSource>;
    fn CreateFromStreamReference(&mut self, stream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStreamReference>, contenttype: &::windows::core::HSTRING) -> ::windows::core::Result<MediaSource>;
    fn CreateFromUri(&mut self, uri: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<MediaSource>;
}
#[cfg(all(feature = "Foundation", feature = "Media_Streaming_Adaptive", feature = "Storage", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaSourceStatics {
    const NAME: &'static str = "Windows.Media.Core.IMediaSourceStatics";
}
#[cfg(all(feature = "Foundation", feature = "Media_Streaming_Adaptive", feature = "Storage", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IMediaSourceStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaSourceStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaSourceStaticsVtbl {
        unsafe extern "system" fn CreateFromAdaptiveMediaSource<Impl: IMediaSourceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mediasource: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromAdaptiveMediaSource(&*(&mediasource as *const <super::Streaming::Adaptive::AdaptiveMediaSource as ::windows::core::Abi>::Abi as *const <super::Streaming::Adaptive::AdaptiveMediaSource as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromMediaStreamSource<Impl: IMediaSourceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mediasource: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromMediaStreamSource(&*(&mediasource as *const <MediaStreamSource as ::windows::core::Abi>::Abi as *const <MediaStreamSource as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromMseStreamSource<Impl: IMediaSourceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mediasource: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromMseStreamSource(&*(&mediasource as *const <MseStreamSource as ::windows::core::Abi>::Abi as *const <MseStreamSource as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromIMediaSource<Impl: IMediaSourceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mediasource: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromIMediaSource(&*(&mediasource as *const <IMediaSource as ::windows::core::Abi>::Abi as *const <IMediaSource as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromStorageFile<Impl: IMediaSourceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromStorageFile(&*(&file as *const <super::super::Storage::IStorageFile as ::windows::core::Abi>::Abi as *const <super::super::Storage::IStorageFile as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromStream<Impl: IMediaSourceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, contenttype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromStream(&*(&stream as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::DefaultType>::DefaultType), &*(&contenttype as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromStreamReference<Impl: IMediaSourceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, contenttype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromStreamReference(&*(&stream as *const <super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::DefaultType>::DefaultType), &*(&contenttype as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromUri<Impl: IMediaSourceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromUri(&*(&uri as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaSourceStatics, BASE_OFFSET>(),
            CreateFromAdaptiveMediaSource: CreateFromAdaptiveMediaSource::<Impl, IMPL_OFFSET>,
            CreateFromMediaStreamSource: CreateFromMediaStreamSource::<Impl, IMPL_OFFSET>,
            CreateFromMseStreamSource: CreateFromMseStreamSource::<Impl, IMPL_OFFSET>,
            CreateFromIMediaSource: CreateFromIMediaSource::<Impl, IMPL_OFFSET>,
            CreateFromStorageFile: CreateFromStorageFile::<Impl, IMPL_OFFSET>,
            CreateFromStream: CreateFromStream::<Impl, IMPL_OFFSET>,
            CreateFromStreamReference: CreateFromStreamReference::<Impl, IMPL_OFFSET>,
            CreateFromUri: CreateFromUri::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaSourceStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaSourceStatics2Impl: Sized {
    fn CreateFromMediaBinder(&mut self, binder: &::core::option::Option<MediaBinder>) -> ::windows::core::Result<MediaSource>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaSourceStatics2 {
    const NAME: &'static str = "Windows.Media.Core.IMediaSourceStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaSourceStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaSourceStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaSourceStatics2Vtbl {
        unsafe extern "system" fn CreateFromMediaBinder<Impl: IMediaSourceStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, binder: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromMediaBinder(&*(&binder as *const <MediaBinder as ::windows::core::Abi>::Abi as *const <MediaBinder as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaSourceStatics2, BASE_OFFSET>(),
            CreateFromMediaBinder: CreateFromMediaBinder::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaSourceStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Media_Capture_Frames", feature = "implement_exclusive"))]
pub trait IMediaSourceStatics3Impl: Sized {
    fn CreateFromMediaFrameSource(&mut self, framesource: &::core::option::Option<super::Capture::Frames::MediaFrameSource>) -> ::windows::core::Result<MediaSource>;
}
#[cfg(all(feature = "Media_Capture_Frames", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaSourceStatics3 {
    const NAME: &'static str = "Windows.Media.Core.IMediaSourceStatics3";
}
#[cfg(all(feature = "Media_Capture_Frames", feature = "implement_exclusive"))]
impl IMediaSourceStatics3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaSourceStatics3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaSourceStatics3Vtbl {
        unsafe extern "system" fn CreateFromMediaFrameSource<Impl: IMediaSourceStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, framesource: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromMediaFrameSource(&*(&framesource as *const <super::Capture::Frames::MediaFrameSource as ::windows::core::Abi>::Abi as *const <super::Capture::Frames::MediaFrameSource as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaSourceStatics3, BASE_OFFSET>(),
            CreateFromMediaFrameSource: CreateFromMediaFrameSource::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaSourceStatics3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Networking_BackgroundTransfer", feature = "implement_exclusive"))]
pub trait IMediaSourceStatics4Impl: Sized {
    fn CreateFromDownloadOperation(&mut self, downloadoperation: &::core::option::Option<super::super::Networking::BackgroundTransfer::DownloadOperation>) -> ::windows::core::Result<MediaSource>;
}
#[cfg(all(feature = "Networking_BackgroundTransfer", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaSourceStatics4 {
    const NAME: &'static str = "Windows.Media.Core.IMediaSourceStatics4";
}
#[cfg(all(feature = "Networking_BackgroundTransfer", feature = "implement_exclusive"))]
impl IMediaSourceStatics4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaSourceStatics4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaSourceStatics4Vtbl {
        unsafe extern "system" fn CreateFromDownloadOperation<Impl: IMediaSourceStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, downloadoperation: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromDownloadOperation(&*(&downloadoperation as *const <super::super::Networking::BackgroundTransfer::DownloadOperation as ::windows::core::Abi>::Abi as *const <super::super::Networking::BackgroundTransfer::DownloadOperation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaSourceStatics4, BASE_OFFSET>(),
            CreateFromDownloadOperation: CreateFromDownloadOperation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaSourceStatics4 as ::windows::core::Interface>::IID
    }
}
pub trait IMediaStreamDescriptorImpl: Sized {
    fn IsSelected(&mut self) -> ::windows::core::Result<bool>;
    fn SetName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Name(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetLanguage(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Language(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for IMediaStreamDescriptor {
    const NAME: &'static str = "Windows.Media.Core.IMediaStreamDescriptor";
}
impl IMediaStreamDescriptorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaStreamDescriptorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaStreamDescriptorVtbl {
        unsafe extern "system" fn IsSelected<Impl: IMediaStreamDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSelected() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: IMediaStreamDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Name<Impl: IMediaStreamDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLanguage<Impl: IMediaStreamDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLanguage(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Language<Impl: IMediaStreamDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Language() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaStreamDescriptor, BASE_OFFSET>(),
            IsSelected: IsSelected::<Impl, IMPL_OFFSET>,
            SetName: SetName::<Impl, IMPL_OFFSET>,
            Name: Name::<Impl, IMPL_OFFSET>,
            SetLanguage: SetLanguage::<Impl, IMPL_OFFSET>,
            Language: Language::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaStreamDescriptor as ::windows::core::Interface>::IID
    }
}
pub trait IMediaStreamDescriptor2Impl: Sized + IMediaStreamDescriptorImpl {
    fn SetLabel(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Label(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for IMediaStreamDescriptor2 {
    const NAME: &'static str = "Windows.Media.Core.IMediaStreamDescriptor2";
}
impl IMediaStreamDescriptor2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaStreamDescriptor2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaStreamDescriptor2Vtbl {
        unsafe extern "system" fn SetLabel<Impl: IMediaStreamDescriptor2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLabel(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Label<Impl: IMediaStreamDescriptor2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Label() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaStreamDescriptor2, BASE_OFFSET>(),
            SetLabel: SetLabel::<Impl, IMPL_OFFSET>,
            Label: Label::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaStreamDescriptor2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IMediaStreamSampleImpl: Sized {
    fn Processed(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaStreamSample, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveProcessed(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Buffer(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::Buffer>;
    fn Timestamp(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn ExtendedProperties(&mut self) -> ::windows::core::Result<MediaStreamSamplePropertySet>;
    fn Protection(&mut self) -> ::windows::core::Result<MediaStreamSampleProtectionProperties>;
    fn SetDecodeTimestamp(&mut self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn DecodeTimestamp(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetDuration(&mut self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn Duration(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetKeyFrame(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn KeyFrame(&mut self) -> ::windows::core::Result<bool>;
    fn SetDiscontinuous(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn Discontinuous(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaStreamSample {
    const NAME: &'static str = "Windows.Media.Core.IMediaStreamSample";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IMediaStreamSampleVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaStreamSampleImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaStreamSampleVtbl {
        unsafe extern "system" fn Processed<Impl: IMediaStreamSampleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Processed(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MediaStreamSample, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MediaStreamSample, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveProcessed<Impl: IMediaStreamSampleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveProcessed(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Buffer<Impl: IMediaStreamSampleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Timestamp<Impl: IMediaStreamSampleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Timestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedProperties<Impl: IMediaStreamSampleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtendedProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Protection<Impl: IMediaStreamSampleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Protection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDecodeTimestamp<Impl: IMediaStreamSampleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDecodeTimestamp(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DecodeTimestamp<Impl: IMediaStreamSampleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DecodeTimestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDuration<Impl: IMediaStreamSampleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDuration(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Duration<Impl: IMediaStreamSampleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetKeyFrame<Impl: IMediaStreamSampleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetKeyFrame(value).into()
        }
        unsafe extern "system" fn KeyFrame<Impl: IMediaStreamSampleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyFrame() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDiscontinuous<Impl: IMediaStreamSampleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDiscontinuous(value).into()
        }
        unsafe extern "system" fn Discontinuous<Impl: IMediaStreamSampleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Discontinuous() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaStreamSample, BASE_OFFSET>(),
            Processed: Processed::<Impl, IMPL_OFFSET>,
            RemoveProcessed: RemoveProcessed::<Impl, IMPL_OFFSET>,
            Buffer: Buffer::<Impl, IMPL_OFFSET>,
            Timestamp: Timestamp::<Impl, IMPL_OFFSET>,
            ExtendedProperties: ExtendedProperties::<Impl, IMPL_OFFSET>,
            Protection: Protection::<Impl, IMPL_OFFSET>,
            SetDecodeTimestamp: SetDecodeTimestamp::<Impl, IMPL_OFFSET>,
            DecodeTimestamp: DecodeTimestamp::<Impl, IMPL_OFFSET>,
            SetDuration: SetDuration::<Impl, IMPL_OFFSET>,
            Duration: Duration::<Impl, IMPL_OFFSET>,
            SetKeyFrame: SetKeyFrame::<Impl, IMPL_OFFSET>,
            KeyFrame: KeyFrame::<Impl, IMPL_OFFSET>,
            SetDiscontinuous: SetDiscontinuous::<Impl, IMPL_OFFSET>,
            Discontinuous: Discontinuous::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaStreamSample as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Graphics_DirectX_Direct3D11", feature = "implement_exclusive"))]
pub trait IMediaStreamSample2Impl: Sized {
    fn Direct3D11Surface(&mut self) -> ::windows::core::Result<super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface>;
}
#[cfg(all(feature = "Graphics_DirectX_Direct3D11", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaStreamSample2 {
    const NAME: &'static str = "Windows.Media.Core.IMediaStreamSample2";
}
#[cfg(all(feature = "Graphics_DirectX_Direct3D11", feature = "implement_exclusive"))]
impl IMediaStreamSample2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaStreamSample2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaStreamSample2Vtbl {
        unsafe extern "system" fn Direct3D11Surface<Impl: IMediaStreamSample2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Direct3D11Surface() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaStreamSample2, BASE_OFFSET>(),
            Direct3D11Surface: Direct3D11Surface::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaStreamSample2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaStreamSampleProtectionPropertiesImpl: Sized {
    fn SetKeyIdentifier(&mut self, value: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn GetKeyIdentifier(&mut self, value: &mut ::windows::core::Array<u8>) -> ::windows::core::Result<()>;
    fn SetInitializationVector(&mut self, value: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn GetInitializationVector(&mut self, value: &mut ::windows::core::Array<u8>) -> ::windows::core::Result<()>;
    fn SetSubSampleMapping(&mut self, value: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn GetSubSampleMapping(&mut self, value: &mut ::windows::core::Array<u8>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaStreamSampleProtectionProperties {
    const NAME: &'static str = "Windows.Media.Core.IMediaStreamSampleProtectionProperties";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaStreamSampleProtectionPropertiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaStreamSampleProtectionPropertiesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaStreamSampleProtectionPropertiesVtbl {
        unsafe extern "system" fn SetKeyIdentifier<Impl: IMediaStreamSampleProtectionPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetKeyIdentifier(::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _)).into()
        }
        unsafe extern "system" fn GetKeyIdentifier<Impl: IMediaStreamSampleProtectionPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetKeyIdentifier(::windows::core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size).as_array()).into()
        }
        unsafe extern "system" fn SetInitializationVector<Impl: IMediaStreamSampleProtectionPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInitializationVector(::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _)).into()
        }
        unsafe extern "system" fn GetInitializationVector<Impl: IMediaStreamSampleProtectionPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetInitializationVector(::windows::core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size).as_array()).into()
        }
        unsafe extern "system" fn SetSubSampleMapping<Impl: IMediaStreamSampleProtectionPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSubSampleMapping(::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _)).into()
        }
        unsafe extern "system" fn GetSubSampleMapping<Impl: IMediaStreamSampleProtectionPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSubSampleMapping(::windows::core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size).as_array()).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaStreamSampleProtectionProperties, BASE_OFFSET>(),
            SetKeyIdentifier: SetKeyIdentifier::<Impl, IMPL_OFFSET>,
            GetKeyIdentifier: GetKeyIdentifier::<Impl, IMPL_OFFSET>,
            SetInitializationVector: SetInitializationVector::<Impl, IMPL_OFFSET>,
            GetInitializationVector: GetInitializationVector::<Impl, IMPL_OFFSET>,
            SetSubSampleMapping: SetSubSampleMapping::<Impl, IMPL_OFFSET>,
            GetSubSampleMapping: GetSubSampleMapping::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaStreamSampleProtectionProperties as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IMediaStreamSampleStaticsImpl: Sized {
    fn CreateFromBuffer(&mut self, buffer: &::core::option::Option<super::super::Storage::Streams::IBuffer>, timestamp: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<MediaStreamSample>;
    fn CreateFromStreamAsync(&mut self, stream: &::core::option::Option<super::super::Storage::Streams::IInputStream>, count: u32, timestamp: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MediaStreamSample>>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaStreamSampleStatics {
    const NAME: &'static str = "Windows.Media.Core.IMediaStreamSampleStatics";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IMediaStreamSampleStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaStreamSampleStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaStreamSampleStaticsVtbl {
        unsafe extern "system" fn CreateFromBuffer<Impl: IMediaStreamSampleStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffer: ::windows::core::RawPtr, timestamp: super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromBuffer(&*(&buffer as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType), &*(&timestamp as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromStreamAsync<Impl: IMediaStreamSampleStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, count: u32, timestamp: super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromStreamAsync(&*(&stream as *const <super::super::Storage::Streams::IInputStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IInputStream as ::windows::core::DefaultType>::DefaultType), count, &*(&timestamp as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaStreamSampleStatics, BASE_OFFSET>(),
            CreateFromBuffer: CreateFromBuffer::<Impl, IMPL_OFFSET>,
            CreateFromStreamAsync: CreateFromStreamAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaStreamSampleStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Graphics_DirectX_Direct3D11", feature = "implement_exclusive"))]
pub trait IMediaStreamSampleStatics2Impl: Sized {
    fn CreateFromDirect3D11Surface(&mut self, surface: &::core::option::Option<super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface>, timestamp: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<MediaStreamSample>;
}
#[cfg(all(feature = "Foundation", feature = "Graphics_DirectX_Direct3D11", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaStreamSampleStatics2 {
    const NAME: &'static str = "Windows.Media.Core.IMediaStreamSampleStatics2";
}
#[cfg(all(feature = "Foundation", feature = "Graphics_DirectX_Direct3D11", feature = "implement_exclusive"))]
impl IMediaStreamSampleStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaStreamSampleStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaStreamSampleStatics2Vtbl {
        unsafe extern "system" fn CreateFromDirect3D11Surface<Impl: IMediaStreamSampleStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, surface: ::windows::core::RawPtr, timestamp: super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromDirect3D11Surface(&*(&surface as *const <super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface as ::windows::core::Abi>::Abi as *const <super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface as ::windows::core::DefaultType>::DefaultType), &*(&timestamp as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaStreamSampleStatics2, BASE_OFFSET>(),
            CreateFromDirect3D11Surface: CreateFromDirect3D11Surface::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaStreamSampleStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Media_Protection", feature = "Storage_FileProperties", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IMediaStreamSourceImpl: Sized + IMediaSourceImpl {
    fn Closed(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaStreamSource, MediaStreamSourceClosedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveClosed(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Starting(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaStreamSource, MediaStreamSourceStartingEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStarting(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Paused(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaStreamSource, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePaused(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SampleRequested(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaStreamSource, MediaStreamSourceSampleRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSampleRequested(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SwitchStreamsRequested(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaStreamSource, MediaStreamSourceSwitchStreamsRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSwitchStreamsRequested(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn NotifyError(&mut self, errorstatus: MediaStreamSourceErrorStatus) -> ::windows::core::Result<()>;
    fn AddStreamDescriptor(&mut self, descriptor: &::core::option::Option<IMediaStreamDescriptor>) -> ::windows::core::Result<()>;
    fn SetMediaProtectionManager(&mut self, value: &::core::option::Option<super::Protection::MediaProtectionManager>) -> ::windows::core::Result<()>;
    fn MediaProtectionManager(&mut self) -> ::windows::core::Result<super::Protection::MediaProtectionManager>;
    fn SetDuration(&mut self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn Duration(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetCanSeek(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn CanSeek(&mut self) -> ::windows::core::Result<bool>;
    fn SetBufferTime(&mut self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn BufferTime(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetBufferedRange(&mut self, startoffset: &super::super::Foundation::TimeSpan, endoffset: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn MusicProperties(&mut self) -> ::windows::core::Result<super::super::Storage::FileProperties::MusicProperties>;
    fn VideoProperties(&mut self) -> ::windows::core::Result<super::super::Storage::FileProperties::VideoProperties>;
    fn SetThumbnail(&mut self, value: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<()>;
    fn Thumbnail(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference>;
    fn AddProtectionKey(&mut self, streamdescriptor: &::core::option::Option<IMediaStreamDescriptor>, keyidentifier: &[<u8 as ::windows::core::DefaultType>::DefaultType], licensedata: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Media_Protection", feature = "Storage_FileProperties", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaStreamSource {
    const NAME: &'static str = "Windows.Media.Core.IMediaStreamSource";
}
#[cfg(all(feature = "Foundation", feature = "Media_Protection", feature = "Storage_FileProperties", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IMediaStreamSourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaStreamSourceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaStreamSourceVtbl {
        unsafe extern "system" fn Closed<Impl: IMediaStreamSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Closed(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MediaStreamSource, MediaStreamSourceClosedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MediaStreamSource, MediaStreamSourceClosedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveClosed<Impl: IMediaStreamSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveClosed(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Starting<Impl: IMediaStreamSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Starting(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MediaStreamSource, MediaStreamSourceStartingEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MediaStreamSource, MediaStreamSourceStartingEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStarting<Impl: IMediaStreamSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveStarting(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Paused<Impl: IMediaStreamSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Paused(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MediaStreamSource, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MediaStreamSource, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePaused<Impl: IMediaStreamSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePaused(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SampleRequested<Impl: IMediaStreamSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SampleRequested(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MediaStreamSource, MediaStreamSourceSampleRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MediaStreamSource, MediaStreamSourceSampleRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSampleRequested<Impl: IMediaStreamSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSampleRequested(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SwitchStreamsRequested<Impl: IMediaStreamSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SwitchStreamsRequested(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MediaStreamSource, MediaStreamSourceSwitchStreamsRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MediaStreamSource, MediaStreamSourceSwitchStreamsRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSwitchStreamsRequested<Impl: IMediaStreamSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSwitchStreamsRequested(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn NotifyError<Impl: IMediaStreamSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, errorstatus: MediaStreamSourceErrorStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NotifyError(errorstatus).into()
        }
        unsafe extern "system" fn AddStreamDescriptor<Impl: IMediaStreamSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, descriptor: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddStreamDescriptor(&*(&descriptor as *const <IMediaStreamDescriptor as ::windows::core::Abi>::Abi as *const <IMediaStreamDescriptor as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetMediaProtectionManager<Impl: IMediaStreamSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMediaProtectionManager(&*(&value as *const <super::Protection::MediaProtectionManager as ::windows::core::Abi>::Abi as *const <super::Protection::MediaProtectionManager as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MediaProtectionManager<Impl: IMediaStreamSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MediaProtectionManager() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDuration<Impl: IMediaStreamSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDuration(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Duration<Impl: IMediaStreamSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCanSeek<Impl: IMediaStreamSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCanSeek(value).into()
        }
        unsafe extern "system" fn CanSeek<Impl: IMediaStreamSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanSeek() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBufferTime<Impl: IMediaStreamSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBufferTime(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BufferTime<Impl: IMediaStreamSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BufferTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBufferedRange<Impl: IMediaStreamSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startoffset: super::super::Foundation::TimeSpan, endoffset: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBufferedRange(&*(&startoffset as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType), &*(&endoffset as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MusicProperties<Impl: IMediaStreamSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MusicProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoProperties<Impl: IMediaStreamSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VideoProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetThumbnail<Impl: IMediaStreamSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetThumbnail(&*(&value as *const <super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Thumbnail<Impl: IMediaStreamSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Thumbnail() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddProtectionKey<Impl: IMediaStreamSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, streamdescriptor: ::windows::core::RawPtr, keyIdentifier_array_size: u32, keyidentifier: *const u8, licenseData_array_size: u32, licensedata: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddProtectionKey(&*(&streamdescriptor as *const <IMediaStreamDescriptor as ::windows::core::Abi>::Abi as *const <IMediaStreamDescriptor as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&keyidentifier), keyIdentifier_array_size as _), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&licensedata), licenseData_array_size as _)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaStreamSource, BASE_OFFSET>(),
            Closed: Closed::<Impl, IMPL_OFFSET>,
            RemoveClosed: RemoveClosed::<Impl, IMPL_OFFSET>,
            Starting: Starting::<Impl, IMPL_OFFSET>,
            RemoveStarting: RemoveStarting::<Impl, IMPL_OFFSET>,
            Paused: Paused::<Impl, IMPL_OFFSET>,
            RemovePaused: RemovePaused::<Impl, IMPL_OFFSET>,
            SampleRequested: SampleRequested::<Impl, IMPL_OFFSET>,
            RemoveSampleRequested: RemoveSampleRequested::<Impl, IMPL_OFFSET>,
            SwitchStreamsRequested: SwitchStreamsRequested::<Impl, IMPL_OFFSET>,
            RemoveSwitchStreamsRequested: RemoveSwitchStreamsRequested::<Impl, IMPL_OFFSET>,
            NotifyError: NotifyError::<Impl, IMPL_OFFSET>,
            AddStreamDescriptor: AddStreamDescriptor::<Impl, IMPL_OFFSET>,
            SetMediaProtectionManager: SetMediaProtectionManager::<Impl, IMPL_OFFSET>,
            MediaProtectionManager: MediaProtectionManager::<Impl, IMPL_OFFSET>,
            SetDuration: SetDuration::<Impl, IMPL_OFFSET>,
            Duration: Duration::<Impl, IMPL_OFFSET>,
            SetCanSeek: SetCanSeek::<Impl, IMPL_OFFSET>,
            CanSeek: CanSeek::<Impl, IMPL_OFFSET>,
            SetBufferTime: SetBufferTime::<Impl, IMPL_OFFSET>,
            BufferTime: BufferTime::<Impl, IMPL_OFFSET>,
            SetBufferedRange: SetBufferedRange::<Impl, IMPL_OFFSET>,
            MusicProperties: MusicProperties::<Impl, IMPL_OFFSET>,
            VideoProperties: VideoProperties::<Impl, IMPL_OFFSET>,
            SetThumbnail: SetThumbnail::<Impl, IMPL_OFFSET>,
            Thumbnail: Thumbnail::<Impl, IMPL_OFFSET>,
            AddProtectionKey: AddProtectionKey::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaStreamSource as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Media_Protection", feature = "Storage_FileProperties", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IMediaStreamSource2Impl: Sized + IMediaSourceImpl + IMediaStreamSourceImpl {
    fn SampleRendered(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaStreamSource, MediaStreamSourceSampleRenderedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSampleRendered(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Media_Protection", feature = "Storage_FileProperties", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaStreamSource2 {
    const NAME: &'static str = "Windows.Media.Core.IMediaStreamSource2";
}
#[cfg(all(feature = "Foundation", feature = "Media_Protection", feature = "Storage_FileProperties", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IMediaStreamSource2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaStreamSource2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaStreamSource2Vtbl {
        unsafe extern "system" fn SampleRendered<Impl: IMediaStreamSource2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SampleRendered(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MediaStreamSource, MediaStreamSourceSampleRenderedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MediaStreamSource, MediaStreamSourceSampleRenderedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSampleRendered<Impl: IMediaStreamSource2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSampleRendered(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaStreamSource2, BASE_OFFSET>(),
            SampleRendered: SampleRendered::<Impl, IMPL_OFFSET>,
            RemoveSampleRendered: RemoveSampleRendered::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaStreamSource2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Media_Protection", feature = "Storage_FileProperties", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IMediaStreamSource3Impl: Sized + IMediaSourceImpl + IMediaStreamSourceImpl {
    fn SetMaxSupportedPlaybackRate(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<f64>>) -> ::windows::core::Result<()>;
    fn MaxSupportedPlaybackRate(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
}
#[cfg(all(feature = "Foundation", feature = "Media_Protection", feature = "Storage_FileProperties", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaStreamSource3 {
    const NAME: &'static str = "Windows.Media.Core.IMediaStreamSource3";
}
#[cfg(all(feature = "Foundation", feature = "Media_Protection", feature = "Storage_FileProperties", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IMediaStreamSource3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaStreamSource3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaStreamSource3Vtbl {
        unsafe extern "system" fn SetMaxSupportedPlaybackRate<Impl: IMediaStreamSource3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxSupportedPlaybackRate(&*(&value as *const <super::super::Foundation::IReference<f64> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<f64> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MaxSupportedPlaybackRate<Impl: IMediaStreamSource3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxSupportedPlaybackRate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaStreamSource3, BASE_OFFSET>(),
            SetMaxSupportedPlaybackRate: SetMaxSupportedPlaybackRate::<Impl, IMPL_OFFSET>,
            MaxSupportedPlaybackRate: MaxSupportedPlaybackRate::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaStreamSource3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Media_Protection", feature = "Storage_FileProperties", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IMediaStreamSource4Impl: Sized + IMediaSourceImpl + IMediaStreamSourceImpl {
    fn SetIsLive(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsLive(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Foundation", feature = "Media_Protection", feature = "Storage_FileProperties", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaStreamSource4 {
    const NAME: &'static str = "Windows.Media.Core.IMediaStreamSource4";
}
#[cfg(all(feature = "Foundation", feature = "Media_Protection", feature = "Storage_FileProperties", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IMediaStreamSource4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaStreamSource4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaStreamSource4Vtbl {
        unsafe extern "system" fn SetIsLive<Impl: IMediaStreamSource4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsLive(value).into()
        }
        unsafe extern "system" fn IsLive<Impl: IMediaStreamSource4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaStreamSource4, BASE_OFFSET>(),
            SetIsLive: SetIsLive::<Impl, IMPL_OFFSET>,
            IsLive: IsLive::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaStreamSource4 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaStreamSourceClosedEventArgsImpl: Sized {
    fn Request(&mut self) -> ::windows::core::Result<MediaStreamSourceClosedRequest>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaStreamSourceClosedEventArgs {
    const NAME: &'static str = "Windows.Media.Core.IMediaStreamSourceClosedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaStreamSourceClosedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaStreamSourceClosedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaStreamSourceClosedEventArgsVtbl {
        unsafe extern "system" fn Request<Impl: IMediaStreamSourceClosedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Request() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaStreamSourceClosedEventArgs, BASE_OFFSET>(),
            Request: Request::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaStreamSourceClosedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaStreamSourceClosedRequestImpl: Sized {
    fn Reason(&mut self) -> ::windows::core::Result<MediaStreamSourceClosedReason>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaStreamSourceClosedRequest {
    const NAME: &'static str = "Windows.Media.Core.IMediaStreamSourceClosedRequest";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaStreamSourceClosedRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaStreamSourceClosedRequestImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaStreamSourceClosedRequestVtbl {
        unsafe extern "system" fn Reason<Impl: IMediaStreamSourceClosedRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MediaStreamSourceClosedReason) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaStreamSourceClosedRequest, BASE_OFFSET>(), Reason: Reason::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaStreamSourceClosedRequest as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaStreamSourceFactoryImpl: Sized {
    fn CreateFromDescriptor(&mut self, descriptor: &::core::option::Option<IMediaStreamDescriptor>) -> ::windows::core::Result<MediaStreamSource>;
    fn CreateFromDescriptors(&mut self, descriptor: &::core::option::Option<IMediaStreamDescriptor>, descriptor2: &::core::option::Option<IMediaStreamDescriptor>) -> ::windows::core::Result<MediaStreamSource>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaStreamSourceFactory {
    const NAME: &'static str = "Windows.Media.Core.IMediaStreamSourceFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaStreamSourceFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaStreamSourceFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaStreamSourceFactoryVtbl {
        unsafe extern "system" fn CreateFromDescriptor<Impl: IMediaStreamSourceFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, descriptor: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromDescriptor(&*(&descriptor as *const <IMediaStreamDescriptor as ::windows::core::Abi>::Abi as *const <IMediaStreamDescriptor as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromDescriptors<Impl: IMediaStreamSourceFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, descriptor: ::windows::core::RawPtr, descriptor2: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromDescriptors(&*(&descriptor as *const <IMediaStreamDescriptor as ::windows::core::Abi>::Abi as *const <IMediaStreamDescriptor as ::windows::core::DefaultType>::DefaultType), &*(&descriptor2 as *const <IMediaStreamDescriptor as ::windows::core::Abi>::Abi as *const <IMediaStreamDescriptor as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaStreamSourceFactory, BASE_OFFSET>(),
            CreateFromDescriptor: CreateFromDescriptor::<Impl, IMPL_OFFSET>,
            CreateFromDescriptors: CreateFromDescriptors::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaStreamSourceFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMediaStreamSourceSampleRenderedEventArgsImpl: Sized {
    fn SampleLag(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaStreamSourceSampleRenderedEventArgs {
    const NAME: &'static str = "Windows.Media.Core.IMediaStreamSourceSampleRenderedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMediaStreamSourceSampleRenderedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaStreamSourceSampleRenderedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaStreamSourceSampleRenderedEventArgsVtbl {
        unsafe extern "system" fn SampleLag<Impl: IMediaStreamSourceSampleRenderedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SampleLag() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaStreamSourceSampleRenderedEventArgs, BASE_OFFSET>(),
            SampleLag: SampleLag::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaStreamSourceSampleRenderedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaStreamSourceSampleRequestImpl: Sized {
    fn StreamDescriptor(&mut self) -> ::windows::core::Result<IMediaStreamDescriptor>;
    fn GetDeferral(&mut self) -> ::windows::core::Result<MediaStreamSourceSampleRequestDeferral>;
    fn SetSample(&mut self, value: &::core::option::Option<MediaStreamSample>) -> ::windows::core::Result<()>;
    fn Sample(&mut self) -> ::windows::core::Result<MediaStreamSample>;
    fn ReportSampleProgress(&mut self, progress: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaStreamSourceSampleRequest {
    const NAME: &'static str = "Windows.Media.Core.IMediaStreamSourceSampleRequest";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaStreamSourceSampleRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaStreamSourceSampleRequestImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaStreamSourceSampleRequestVtbl {
        unsafe extern "system" fn StreamDescriptor<Impl: IMediaStreamSourceSampleRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StreamDescriptor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeferral<Impl: IMediaStreamSourceSampleRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSample<Impl: IMediaStreamSourceSampleRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSample(&*(&value as *const <MediaStreamSample as ::windows::core::Abi>::Abi as *const <MediaStreamSample as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Sample<Impl: IMediaStreamSourceSampleRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Sample() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportSampleProgress<Impl: IMediaStreamSourceSampleRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, progress: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReportSampleProgress(progress).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaStreamSourceSampleRequest, BASE_OFFSET>(),
            StreamDescriptor: StreamDescriptor::<Impl, IMPL_OFFSET>,
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
            SetSample: SetSample::<Impl, IMPL_OFFSET>,
            Sample: Sample::<Impl, IMPL_OFFSET>,
            ReportSampleProgress: ReportSampleProgress::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaStreamSourceSampleRequest as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaStreamSourceSampleRequestDeferralImpl: Sized {
    fn Complete(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaStreamSourceSampleRequestDeferral {
    const NAME: &'static str = "Windows.Media.Core.IMediaStreamSourceSampleRequestDeferral";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaStreamSourceSampleRequestDeferralVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaStreamSourceSampleRequestDeferralImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaStreamSourceSampleRequestDeferralVtbl {
        unsafe extern "system" fn Complete<Impl: IMediaStreamSourceSampleRequestDeferralImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Complete().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaStreamSourceSampleRequestDeferral, BASE_OFFSET>(),
            Complete: Complete::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaStreamSourceSampleRequestDeferral as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaStreamSourceSampleRequestedEventArgsImpl: Sized {
    fn Request(&mut self) -> ::windows::core::Result<MediaStreamSourceSampleRequest>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaStreamSourceSampleRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.Core.IMediaStreamSourceSampleRequestedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaStreamSourceSampleRequestedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaStreamSourceSampleRequestedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaStreamSourceSampleRequestedEventArgsVtbl {
        unsafe extern "system" fn Request<Impl: IMediaStreamSourceSampleRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Request() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaStreamSourceSampleRequestedEventArgs, BASE_OFFSET>(),
            Request: Request::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaStreamSourceSampleRequestedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaStreamSourceStartingEventArgsImpl: Sized {
    fn Request(&mut self) -> ::windows::core::Result<MediaStreamSourceStartingRequest>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaStreamSourceStartingEventArgs {
    const NAME: &'static str = "Windows.Media.Core.IMediaStreamSourceStartingEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaStreamSourceStartingEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaStreamSourceStartingEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaStreamSourceStartingEventArgsVtbl {
        unsafe extern "system" fn Request<Impl: IMediaStreamSourceStartingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Request() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaStreamSourceStartingEventArgs, BASE_OFFSET>(),
            Request: Request::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaStreamSourceStartingEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMediaStreamSourceStartingRequestImpl: Sized {
    fn StartPosition(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>;
    fn GetDeferral(&mut self) -> ::windows::core::Result<MediaStreamSourceStartingRequestDeferral>;
    fn SetActualStartPosition(&mut self, position: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaStreamSourceStartingRequest {
    const NAME: &'static str = "Windows.Media.Core.IMediaStreamSourceStartingRequest";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMediaStreamSourceStartingRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaStreamSourceStartingRequestImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaStreamSourceStartingRequestVtbl {
        unsafe extern "system" fn StartPosition<Impl: IMediaStreamSourceStartingRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeferral<Impl: IMediaStreamSourceStartingRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetActualStartPosition<Impl: IMediaStreamSourceStartingRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, position: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetActualStartPosition(&*(&position as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaStreamSourceStartingRequest, BASE_OFFSET>(),
            StartPosition: StartPosition::<Impl, IMPL_OFFSET>,
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
            SetActualStartPosition: SetActualStartPosition::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaStreamSourceStartingRequest as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaStreamSourceStartingRequestDeferralImpl: Sized {
    fn Complete(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaStreamSourceStartingRequestDeferral {
    const NAME: &'static str = "Windows.Media.Core.IMediaStreamSourceStartingRequestDeferral";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaStreamSourceStartingRequestDeferralVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaStreamSourceStartingRequestDeferralImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaStreamSourceStartingRequestDeferralVtbl {
        unsafe extern "system" fn Complete<Impl: IMediaStreamSourceStartingRequestDeferralImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Complete().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaStreamSourceStartingRequestDeferral, BASE_OFFSET>(),
            Complete: Complete::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaStreamSourceStartingRequestDeferral as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaStreamSourceSwitchStreamsRequestImpl: Sized {
    fn OldStreamDescriptor(&mut self) -> ::windows::core::Result<IMediaStreamDescriptor>;
    fn NewStreamDescriptor(&mut self) -> ::windows::core::Result<IMediaStreamDescriptor>;
    fn GetDeferral(&mut self) -> ::windows::core::Result<MediaStreamSourceSwitchStreamsRequestDeferral>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaStreamSourceSwitchStreamsRequest {
    const NAME: &'static str = "Windows.Media.Core.IMediaStreamSourceSwitchStreamsRequest";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaStreamSourceSwitchStreamsRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaStreamSourceSwitchStreamsRequestImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaStreamSourceSwitchStreamsRequestVtbl {
        unsafe extern "system" fn OldStreamDescriptor<Impl: IMediaStreamSourceSwitchStreamsRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OldStreamDescriptor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NewStreamDescriptor<Impl: IMediaStreamSourceSwitchStreamsRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NewStreamDescriptor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeferral<Impl: IMediaStreamSourceSwitchStreamsRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaStreamSourceSwitchStreamsRequest, BASE_OFFSET>(),
            OldStreamDescriptor: OldStreamDescriptor::<Impl, IMPL_OFFSET>,
            NewStreamDescriptor: NewStreamDescriptor::<Impl, IMPL_OFFSET>,
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaStreamSourceSwitchStreamsRequest as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaStreamSourceSwitchStreamsRequestDeferralImpl: Sized {
    fn Complete(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaStreamSourceSwitchStreamsRequestDeferral {
    const NAME: &'static str = "Windows.Media.Core.IMediaStreamSourceSwitchStreamsRequestDeferral";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaStreamSourceSwitchStreamsRequestDeferralVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaStreamSourceSwitchStreamsRequestDeferralImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaStreamSourceSwitchStreamsRequestDeferralVtbl {
        unsafe extern "system" fn Complete<Impl: IMediaStreamSourceSwitchStreamsRequestDeferralImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Complete().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaStreamSourceSwitchStreamsRequestDeferral, BASE_OFFSET>(),
            Complete: Complete::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaStreamSourceSwitchStreamsRequestDeferral as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaStreamSourceSwitchStreamsRequestedEventArgsImpl: Sized {
    fn Request(&mut self) -> ::windows::core::Result<MediaStreamSourceSwitchStreamsRequest>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaStreamSourceSwitchStreamsRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.Core.IMediaStreamSourceSwitchStreamsRequestedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaStreamSourceSwitchStreamsRequestedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaStreamSourceSwitchStreamsRequestedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaStreamSourceSwitchStreamsRequestedEventArgsVtbl {
        unsafe extern "system" fn Request<Impl: IMediaStreamSourceSwitchStreamsRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Request() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaStreamSourceSwitchStreamsRequestedEventArgs, BASE_OFFSET>(),
            Request: Request::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaStreamSourceSwitchStreamsRequestedEventArgs as ::windows::core::Interface>::IID
    }
}
pub trait IMediaTrackImpl: Sized {
    fn Id(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Language(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TrackKind(&mut self) -> ::windows::core::Result<MediaTrackKind>;
    fn SetLabel(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Label(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for IMediaTrack {
    const NAME: &'static str = "Windows.Media.Core.IMediaTrack";
}
impl IMediaTrackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaTrackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaTrackVtbl {
        unsafe extern "system" fn Id<Impl: IMediaTrackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Language<Impl: IMediaTrackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Language() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrackKind<Impl: IMediaTrackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MediaTrackKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrackKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLabel<Impl: IMediaTrackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLabel(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Label<Impl: IMediaTrackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Label() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaTrack, BASE_OFFSET>(),
            Id: Id::<Impl, IMPL_OFFSET>,
            Language: Language::<Impl, IMPL_OFFSET>,
            TrackKind: TrackKind::<Impl, IMPL_OFFSET>,
            SetLabel: SetLabel::<Impl, IMPL_OFFSET>,
            Label: Label::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaTrack as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IMseSourceBufferImpl: Sized {
    fn UpdateStarting(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MseSourceBuffer, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveUpdateStarting(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Updated(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MseSourceBuffer, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveUpdated(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn UpdateEnded(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MseSourceBuffer, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveUpdateEnded(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ErrorOccurred(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MseSourceBuffer, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveErrorOccurred(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Aborted(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MseSourceBuffer, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAborted(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Mode(&mut self) -> ::windows::core::Result<MseAppendMode>;
    fn SetMode(&mut self, value: MseAppendMode) -> ::windows::core::Result<()>;
    fn IsUpdating(&mut self) -> ::windows::core::Result<bool>;
    fn Buffered(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MseTimeRange>>;
    fn TimestampOffset(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetTimestampOffset(&mut self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn AppendWindowStart(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetAppendWindowStart(&mut self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn AppendWindowEnd(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>;
    fn SetAppendWindowEnd(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>) -> ::windows::core::Result<()>;
    fn AppendBuffer(&mut self, buffer: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
    fn AppendStream(&mut self, stream: &::core::option::Option<super::super::Storage::Streams::IInputStream>) -> ::windows::core::Result<()>;
    fn AppendStreamMaxSize(&mut self, stream: &::core::option::Option<super::super::Storage::Streams::IInputStream>, maxsize: u64) -> ::windows::core::Result<()>;
    fn Abort(&mut self) -> ::windows::core::Result<()>;
    fn Remove(&mut self, start: &super::super::Foundation::TimeSpan, end: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMseSourceBuffer {
    const NAME: &'static str = "Windows.Media.Core.IMseSourceBuffer";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IMseSourceBufferVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMseSourceBufferImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMseSourceBufferVtbl {
        unsafe extern "system" fn UpdateStarting<Impl: IMseSourceBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpdateStarting(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MseSourceBuffer, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MseSourceBuffer, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveUpdateStarting<Impl: IMseSourceBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveUpdateStarting(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Updated<Impl: IMseSourceBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Updated(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MseSourceBuffer, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MseSourceBuffer, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveUpdated<Impl: IMseSourceBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveUpdated(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn UpdateEnded<Impl: IMseSourceBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpdateEnded(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MseSourceBuffer, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MseSourceBuffer, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveUpdateEnded<Impl: IMseSourceBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveUpdateEnded(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ErrorOccurred<Impl: IMseSourceBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ErrorOccurred(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MseSourceBuffer, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MseSourceBuffer, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveErrorOccurred<Impl: IMseSourceBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveErrorOccurred(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Aborted<Impl: IMseSourceBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Aborted(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MseSourceBuffer, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MseSourceBuffer, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAborted<Impl: IMseSourceBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAborted(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Mode<Impl: IMseSourceBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MseAppendMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Mode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMode<Impl: IMseSourceBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: MseAppendMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMode(value).into()
        }
        unsafe extern "system" fn IsUpdating<Impl: IMseSourceBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsUpdating() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Buffered<Impl: IMseSourceBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Buffered() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TimestampOffset<Impl: IMseSourceBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TimestampOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTimestampOffset<Impl: IMseSourceBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTimestampOffset(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AppendWindowStart<Impl: IMseSourceBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppendWindowStart() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAppendWindowStart<Impl: IMseSourceBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAppendWindowStart(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AppendWindowEnd<Impl: IMseSourceBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppendWindowEnd() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAppendWindowEnd<Impl: IMseSourceBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAppendWindowEnd(&*(&value as *const <super::super::Foundation::IReference<super::super::Foundation::TimeSpan> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::super::Foundation::TimeSpan> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AppendBuffer<Impl: IMseSourceBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AppendBuffer(&*(&buffer as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AppendStream<Impl: IMseSourceBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AppendStream(&*(&stream as *const <super::super::Storage::Streams::IInputStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IInputStream as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AppendStreamMaxSize<Impl: IMseSourceBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, maxsize: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AppendStreamMaxSize(&*(&stream as *const <super::super::Storage::Streams::IInputStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IInputStream as ::windows::core::DefaultType>::DefaultType), maxsize).into()
        }
        unsafe extern "system" fn Abort<Impl: IMseSourceBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Abort().into()
        }
        unsafe extern "system" fn Remove<Impl: IMseSourceBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, start: super::super::Foundation::TimeSpan, end: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Remove(&*(&start as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType), &*(&end as *const <super::super::Foundation::IReference<super::super::Foundation::TimeSpan> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::super::Foundation::TimeSpan> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMseSourceBuffer, BASE_OFFSET>(),
            UpdateStarting: UpdateStarting::<Impl, IMPL_OFFSET>,
            RemoveUpdateStarting: RemoveUpdateStarting::<Impl, IMPL_OFFSET>,
            Updated: Updated::<Impl, IMPL_OFFSET>,
            RemoveUpdated: RemoveUpdated::<Impl, IMPL_OFFSET>,
            UpdateEnded: UpdateEnded::<Impl, IMPL_OFFSET>,
            RemoveUpdateEnded: RemoveUpdateEnded::<Impl, IMPL_OFFSET>,
            ErrorOccurred: ErrorOccurred::<Impl, IMPL_OFFSET>,
            RemoveErrorOccurred: RemoveErrorOccurred::<Impl, IMPL_OFFSET>,
            Aborted: Aborted::<Impl, IMPL_OFFSET>,
            RemoveAborted: RemoveAborted::<Impl, IMPL_OFFSET>,
            Mode: Mode::<Impl, IMPL_OFFSET>,
            SetMode: SetMode::<Impl, IMPL_OFFSET>,
            IsUpdating: IsUpdating::<Impl, IMPL_OFFSET>,
            Buffered: Buffered::<Impl, IMPL_OFFSET>,
            TimestampOffset: TimestampOffset::<Impl, IMPL_OFFSET>,
            SetTimestampOffset: SetTimestampOffset::<Impl, IMPL_OFFSET>,
            AppendWindowStart: AppendWindowStart::<Impl, IMPL_OFFSET>,
            SetAppendWindowStart: SetAppendWindowStart::<Impl, IMPL_OFFSET>,
            AppendWindowEnd: AppendWindowEnd::<Impl, IMPL_OFFSET>,
            SetAppendWindowEnd: SetAppendWindowEnd::<Impl, IMPL_OFFSET>,
            AppendBuffer: AppendBuffer::<Impl, IMPL_OFFSET>,
            AppendStream: AppendStream::<Impl, IMPL_OFFSET>,
            AppendStreamMaxSize: AppendStreamMaxSize::<Impl, IMPL_OFFSET>,
            Abort: Abort::<Impl, IMPL_OFFSET>,
            Remove: Remove::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMseSourceBuffer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMseSourceBufferListImpl: Sized {
    fn SourceBufferAdded(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MseSourceBufferList, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSourceBufferAdded(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SourceBufferRemoved(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MseSourceBufferList, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSourceBufferRemoved(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Buffers(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MseSourceBuffer>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMseSourceBufferList {
    const NAME: &'static str = "Windows.Media.Core.IMseSourceBufferList";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMseSourceBufferListVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMseSourceBufferListImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMseSourceBufferListVtbl {
        unsafe extern "system" fn SourceBufferAdded<Impl: IMseSourceBufferListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SourceBufferAdded(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MseSourceBufferList, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MseSourceBufferList, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSourceBufferAdded<Impl: IMseSourceBufferListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSourceBufferAdded(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SourceBufferRemoved<Impl: IMseSourceBufferListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SourceBufferRemoved(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MseSourceBufferList, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MseSourceBufferList, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSourceBufferRemoved<Impl: IMseSourceBufferListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSourceBufferRemoved(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Buffers<Impl: IMseSourceBufferListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Buffers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMseSourceBufferList, BASE_OFFSET>(),
            SourceBufferAdded: SourceBufferAdded::<Impl, IMPL_OFFSET>,
            RemoveSourceBufferAdded: RemoveSourceBufferAdded::<Impl, IMPL_OFFSET>,
            SourceBufferRemoved: SourceBufferRemoved::<Impl, IMPL_OFFSET>,
            RemoveSourceBufferRemoved: RemoveSourceBufferRemoved::<Impl, IMPL_OFFSET>,
            Buffers: Buffers::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMseSourceBufferList as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMseStreamSourceImpl: Sized + IMediaSourceImpl {
    fn Opened(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MseStreamSource, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveOpened(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Ended(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MseStreamSource, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveEnded(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Closed(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MseStreamSource, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveClosed(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SourceBuffers(&mut self) -> ::windows::core::Result<MseSourceBufferList>;
    fn ActiveSourceBuffers(&mut self) -> ::windows::core::Result<MseSourceBufferList>;
    fn ReadyState(&mut self) -> ::windows::core::Result<MseReadyState>;
    fn Duration(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>;
    fn SetDuration(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>) -> ::windows::core::Result<()>;
    fn AddSourceBuffer(&mut self, mimetype: &::windows::core::HSTRING) -> ::windows::core::Result<MseSourceBuffer>;
    fn RemoveSourceBuffer(&mut self, buffer: &::core::option::Option<MseSourceBuffer>) -> ::windows::core::Result<()>;
    fn EndOfStream(&mut self, status: MseEndOfStreamStatus) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMseStreamSource {
    const NAME: &'static str = "Windows.Media.Core.IMseStreamSource";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMseStreamSourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMseStreamSourceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMseStreamSourceVtbl {
        unsafe extern "system" fn Opened<Impl: IMseStreamSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Opened(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MseStreamSource, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MseStreamSource, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveOpened<Impl: IMseStreamSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveOpened(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Ended<Impl: IMseStreamSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Ended(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MseStreamSource, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MseStreamSource, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveEnded<Impl: IMseStreamSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveEnded(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Closed<Impl: IMseStreamSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Closed(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MseStreamSource, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MseStreamSource, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveClosed<Impl: IMseStreamSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveClosed(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SourceBuffers<Impl: IMseStreamSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SourceBuffers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActiveSourceBuffers<Impl: IMseStreamSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActiveSourceBuffers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadyState<Impl: IMseStreamSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MseReadyState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadyState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Duration<Impl: IMseStreamSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDuration<Impl: IMseStreamSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDuration(&*(&value as *const <super::super::Foundation::IReference<super::super::Foundation::TimeSpan> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::super::Foundation::TimeSpan> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AddSourceBuffer<Impl: IMseStreamSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mimetype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddSourceBuffer(&*(&mimetype as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSourceBuffer<Impl: IMseStreamSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSourceBuffer(&*(&buffer as *const <MseSourceBuffer as ::windows::core::Abi>::Abi as *const <MseSourceBuffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EndOfStream<Impl: IMseStreamSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: MseEndOfStreamStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EndOfStream(status).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMseStreamSource, BASE_OFFSET>(),
            Opened: Opened::<Impl, IMPL_OFFSET>,
            RemoveOpened: RemoveOpened::<Impl, IMPL_OFFSET>,
            Ended: Ended::<Impl, IMPL_OFFSET>,
            RemoveEnded: RemoveEnded::<Impl, IMPL_OFFSET>,
            Closed: Closed::<Impl, IMPL_OFFSET>,
            RemoveClosed: RemoveClosed::<Impl, IMPL_OFFSET>,
            SourceBuffers: SourceBuffers::<Impl, IMPL_OFFSET>,
            ActiveSourceBuffers: ActiveSourceBuffers::<Impl, IMPL_OFFSET>,
            ReadyState: ReadyState::<Impl, IMPL_OFFSET>,
            Duration: Duration::<Impl, IMPL_OFFSET>,
            SetDuration: SetDuration::<Impl, IMPL_OFFSET>,
            AddSourceBuffer: AddSourceBuffer::<Impl, IMPL_OFFSET>,
            RemoveSourceBuffer: RemoveSourceBuffer::<Impl, IMPL_OFFSET>,
            EndOfStream: EndOfStream::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMseStreamSource as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMseStreamSource2Impl: Sized {
    fn LiveSeekableRange(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<MseTimeRange>>;
    fn SetLiveSeekableRange(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<MseTimeRange>>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMseStreamSource2 {
    const NAME: &'static str = "Windows.Media.Core.IMseStreamSource2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMseStreamSource2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMseStreamSource2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMseStreamSource2Vtbl {
        unsafe extern "system" fn LiveSeekableRange<Impl: IMseStreamSource2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LiveSeekableRange() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLiveSeekableRange<Impl: IMseStreamSource2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLiveSeekableRange(&*(&value as *const <super::super::Foundation::IReference<MseTimeRange> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<MseTimeRange> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMseStreamSource2, BASE_OFFSET>(),
            LiveSeekableRange: LiveSeekableRange::<Impl, IMPL_OFFSET>,
            SetLiveSeekableRange: SetLiveSeekableRange::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMseStreamSource2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMseStreamSourceStaticsImpl: Sized {
    fn IsContentTypeSupported(&mut self, contenttype: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMseStreamSourceStatics {
    const NAME: &'static str = "Windows.Media.Core.IMseStreamSourceStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IMseStreamSourceStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMseStreamSourceStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMseStreamSourceStaticsVtbl {
        unsafe extern "system" fn IsContentTypeSupported<Impl: IMseStreamSourceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contenttype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMseStreamSourceStatics, BASE_OFFSET>(),
            IsContentTypeSupported: IsContentTypeSupported::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMseStreamSourceStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ISceneAnalysisEffectImpl: Sized + IMediaExtensionImpl {
    fn HighDynamicRangeAnalyzer(&mut self) -> ::windows::core::Result<HighDynamicRangeControl>;
    fn SetDesiredAnalysisInterval(&mut self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn DesiredAnalysisInterval(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SceneAnalyzed(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<SceneAnalysisEffect, SceneAnalyzedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSceneAnalyzed(&mut self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISceneAnalysisEffect {
    const NAME: &'static str = "Windows.Media.Core.ISceneAnalysisEffect";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ISceneAnalysisEffectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISceneAnalysisEffectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISceneAnalysisEffectVtbl {
        unsafe extern "system" fn HighDynamicRangeAnalyzer<Impl: ISceneAnalysisEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HighDynamicRangeAnalyzer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredAnalysisInterval<Impl: ISceneAnalysisEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDesiredAnalysisInterval(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DesiredAnalysisInterval<Impl: ISceneAnalysisEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DesiredAnalysisInterval() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SceneAnalyzed<Impl: ISceneAnalysisEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SceneAnalyzed(&*(&handler as *const <super::super::Foundation::TypedEventHandler<SceneAnalysisEffect, SceneAnalyzedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<SceneAnalysisEffect, SceneAnalyzedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSceneAnalyzed<Impl: ISceneAnalysisEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSceneAnalyzed(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISceneAnalysisEffect, BASE_OFFSET>(),
            HighDynamicRangeAnalyzer: HighDynamicRangeAnalyzer::<Impl, IMPL_OFFSET>,
            SetDesiredAnalysisInterval: SetDesiredAnalysisInterval::<Impl, IMPL_OFFSET>,
            DesiredAnalysisInterval: DesiredAnalysisInterval::<Impl, IMPL_OFFSET>,
            SceneAnalyzed: SceneAnalyzed::<Impl, IMPL_OFFSET>,
            RemoveSceneAnalyzed: RemoveSceneAnalyzed::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISceneAnalysisEffect as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Capture", feature = "implement_exclusive"))]
pub trait ISceneAnalysisEffectFrameImpl: Sized + IClosableImpl + IMediaFrameImpl {
    fn FrameControlValues(&mut self) -> ::windows::core::Result<super::Capture::CapturedFrameControlValues>;
    fn HighDynamicRange(&mut self) -> ::windows::core::Result<HighDynamicRangeOutput>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Capture", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISceneAnalysisEffectFrame {
    const NAME: &'static str = "Windows.Media.Core.ISceneAnalysisEffectFrame";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Capture", feature = "implement_exclusive"))]
impl ISceneAnalysisEffectFrameVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISceneAnalysisEffectFrameImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISceneAnalysisEffectFrameVtbl {
        unsafe extern "system" fn FrameControlValues<Impl: ISceneAnalysisEffectFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FrameControlValues() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HighDynamicRange<Impl: ISceneAnalysisEffectFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HighDynamicRange() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISceneAnalysisEffectFrame, BASE_OFFSET>(),
            FrameControlValues: FrameControlValues::<Impl, IMPL_OFFSET>,
            HighDynamicRange: HighDynamicRange::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISceneAnalysisEffectFrame as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ISceneAnalysisEffectFrame2Impl: Sized + IClosableImpl + IMediaFrameImpl {
    fn AnalysisRecommendation(&mut self) -> ::windows::core::Result<SceneAnalysisRecommendation>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISceneAnalysisEffectFrame2 {
    const NAME: &'static str = "Windows.Media.Core.ISceneAnalysisEffectFrame2";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ISceneAnalysisEffectFrame2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISceneAnalysisEffectFrame2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISceneAnalysisEffectFrame2Vtbl {
        unsafe extern "system" fn AnalysisRecommendation<Impl: ISceneAnalysisEffectFrame2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SceneAnalysisRecommendation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AnalysisRecommendation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISceneAnalysisEffectFrame2, BASE_OFFSET>(),
            AnalysisRecommendation: AnalysisRecommendation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISceneAnalysisEffectFrame2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISceneAnalyzedEventArgsImpl: Sized {
    fn ResultFrame(&mut self) -> ::windows::core::Result<SceneAnalysisEffectFrame>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISceneAnalyzedEventArgs {
    const NAME: &'static str = "Windows.Media.Core.ISceneAnalyzedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ISceneAnalyzedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISceneAnalyzedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISceneAnalyzedEventArgsVtbl {
        unsafe extern "system" fn ResultFrame<Impl: ISceneAnalyzedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResultFrame() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ISceneAnalyzedEventArgs, BASE_OFFSET>(), ResultFrame: ResultFrame::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISceneAnalyzedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait ISingleSelectMediaTrackListImpl: Sized {
    fn SelectedIndexChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ISingleSelectMediaTrackList, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSelectedIndexChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SetSelectedIndex(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn SelectedIndex(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for ISingleSelectMediaTrackList {
    const NAME: &'static str = "Windows.Media.Core.ISingleSelectMediaTrackList";
}
#[cfg(feature = "Foundation")]
impl ISingleSelectMediaTrackListVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISingleSelectMediaTrackListImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISingleSelectMediaTrackListVtbl {
        unsafe extern "system" fn SelectedIndexChanged<Impl: ISingleSelectMediaTrackListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectedIndexChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<ISingleSelectMediaTrackList, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<ISingleSelectMediaTrackList, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSelectedIndexChanged<Impl: ISingleSelectMediaTrackListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSelectedIndexChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetSelectedIndex<Impl: ISingleSelectMediaTrackListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelectedIndex(value).into()
        }
        unsafe extern "system" fn SelectedIndex<Impl: ISingleSelectMediaTrackListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectedIndex() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISingleSelectMediaTrackList, BASE_OFFSET>(),
            SelectedIndexChanged: SelectedIndexChanged::<Impl, IMPL_OFFSET>,
            RemoveSelectedIndexChanged: RemoveSelectedIndexChanged::<Impl, IMPL_OFFSET>,
            SetSelectedIndex: SetSelectedIndex::<Impl, IMPL_OFFSET>,
            SelectedIndex: SelectedIndex::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISingleSelectMediaTrackList as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ISpeechCueImpl: Sized + IMediaCueImpl {
    fn Text(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetText(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn StartPositionInInput(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn SetStartPositionInInput(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
    fn EndPositionInInput(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn SetEndPositionInInput(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISpeechCue {
    const NAME: &'static str = "Windows.Media.Core.ISpeechCue";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ISpeechCueVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpeechCueImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpeechCueVtbl {
        unsafe extern "system" fn Text<Impl: ISpeechCueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Text() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetText<Impl: ISpeechCueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetText(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StartPositionInInput<Impl: ISpeechCueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartPositionInInput() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartPositionInInput<Impl: ISpeechCueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStartPositionInInput(&*(&value as *const <super::super::Foundation::IReference<i32> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<i32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EndPositionInInput<Impl: ISpeechCueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndPositionInInput() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEndPositionInInput<Impl: ISpeechCueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEndPositionInInput(&*(&value as *const <super::super::Foundation::IReference<i32> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<i32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpeechCue, BASE_OFFSET>(),
            Text: Text::<Impl, IMPL_OFFSET>,
            SetText: SetText::<Impl, IMPL_OFFSET>,
            StartPositionInInput: StartPositionInInput::<Impl, IMPL_OFFSET>,
            SetStartPositionInInput: SetStartPositionInInput::<Impl, IMPL_OFFSET>,
            EndPositionInInput: EndPositionInInput::<Impl, IMPL_OFFSET>,
            SetEndPositionInInput: SetEndPositionInInput::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpeechCue as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Media_MediaProperties", feature = "implement_exclusive"))]
pub trait ITimedMetadataStreamDescriptorImpl: Sized {
    fn EncodingProperties(&mut self) -> ::windows::core::Result<super::MediaProperties::TimedMetadataEncodingProperties>;
    fn Copy(&mut self) -> ::windows::core::Result<TimedMetadataStreamDescriptor>;
}
#[cfg(all(feature = "Media_MediaProperties", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITimedMetadataStreamDescriptor {
    const NAME: &'static str = "Windows.Media.Core.ITimedMetadataStreamDescriptor";
}
#[cfg(all(feature = "Media_MediaProperties", feature = "implement_exclusive"))]
impl ITimedMetadataStreamDescriptorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITimedMetadataStreamDescriptorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITimedMetadataStreamDescriptorVtbl {
        unsafe extern "system" fn EncodingProperties<Impl: ITimedMetadataStreamDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EncodingProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Copy<Impl: ITimedMetadataStreamDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Copy() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITimedMetadataStreamDescriptor, BASE_OFFSET>(),
            EncodingProperties: EncodingProperties::<Impl, IMPL_OFFSET>,
            Copy: Copy::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITimedMetadataStreamDescriptor as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Media_MediaProperties", feature = "implement_exclusive"))]
pub trait ITimedMetadataStreamDescriptorFactoryImpl: Sized {
    fn Create(&mut self, encodingproperties: &::core::option::Option<super::MediaProperties::TimedMetadataEncodingProperties>) -> ::windows::core::Result<TimedMetadataStreamDescriptor>;
}
#[cfg(all(feature = "Media_MediaProperties", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITimedMetadataStreamDescriptorFactory {
    const NAME: &'static str = "Windows.Media.Core.ITimedMetadataStreamDescriptorFactory";
}
#[cfg(all(feature = "Media_MediaProperties", feature = "implement_exclusive"))]
impl ITimedMetadataStreamDescriptorFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITimedMetadataStreamDescriptorFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITimedMetadataStreamDescriptorFactoryVtbl {
        unsafe extern "system" fn Create<Impl: ITimedMetadataStreamDescriptorFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encodingproperties: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&encodingproperties as *const <super::MediaProperties::TimedMetadataEncodingProperties as ::windows::core::Abi>::Abi as *const <super::MediaProperties::TimedMetadataEncodingProperties as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITimedMetadataStreamDescriptorFactory, BASE_OFFSET>(),
            Create: Create::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITimedMetadataStreamDescriptorFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ITimedMetadataTrackImpl: Sized + IMediaTrackImpl {
    fn CueEntered(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<TimedMetadataTrack, MediaCueEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCueEntered(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CueExited(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<TimedMetadataTrack, MediaCueEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCueExited(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn TrackFailed(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<TimedMetadataTrack, TimedMetadataTrackFailedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveTrackFailed(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Cues(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<IMediaCue>>;
    fn ActiveCues(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<IMediaCue>>;
    fn TimedMetadataKind(&mut self) -> ::windows::core::Result<TimedMetadataKind>;
    fn DispatchType(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AddCue(&mut self, cue: &::core::option::Option<IMediaCue>) -> ::windows::core::Result<()>;
    fn RemoveCue(&mut self, cue: &::core::option::Option<IMediaCue>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITimedMetadataTrack {
    const NAME: &'static str = "Windows.Media.Core.ITimedMetadataTrack";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ITimedMetadataTrackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITimedMetadataTrackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITimedMetadataTrackVtbl {
        unsafe extern "system" fn CueEntered<Impl: ITimedMetadataTrackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CueEntered(&*(&handler as *const <super::super::Foundation::TypedEventHandler<TimedMetadataTrack, MediaCueEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<TimedMetadataTrack, MediaCueEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCueEntered<Impl: ITimedMetadataTrackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveCueEntered(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CueExited<Impl: ITimedMetadataTrackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CueExited(&*(&handler as *const <super::super::Foundation::TypedEventHandler<TimedMetadataTrack, MediaCueEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<TimedMetadataTrack, MediaCueEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCueExited<Impl: ITimedMetadataTrackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveCueExited(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TrackFailed<Impl: ITimedMetadataTrackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrackFailed(&*(&handler as *const <super::super::Foundation::TypedEventHandler<TimedMetadataTrack, TimedMetadataTrackFailedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<TimedMetadataTrack, TimedMetadataTrackFailedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveTrackFailed<Impl: ITimedMetadataTrackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveTrackFailed(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Cues<Impl: ITimedMetadataTrackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Cues() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActiveCues<Impl: ITimedMetadataTrackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActiveCues() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TimedMetadataKind<Impl: ITimedMetadataTrackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut TimedMetadataKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TimedMetadataKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DispatchType<Impl: ITimedMetadataTrackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DispatchType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddCue<Impl: ITimedMetadataTrackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cue: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddCue(&*(&cue as *const <IMediaCue as ::windows::core::Abi>::Abi as *const <IMediaCue as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RemoveCue<Impl: ITimedMetadataTrackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cue: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveCue(&*(&cue as *const <IMediaCue as ::windows::core::Abi>::Abi as *const <IMediaCue as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITimedMetadataTrack, BASE_OFFSET>(),
            CueEntered: CueEntered::<Impl, IMPL_OFFSET>,
            RemoveCueEntered: RemoveCueEntered::<Impl, IMPL_OFFSET>,
            CueExited: CueExited::<Impl, IMPL_OFFSET>,
            RemoveCueExited: RemoveCueExited::<Impl, IMPL_OFFSET>,
            TrackFailed: TrackFailed::<Impl, IMPL_OFFSET>,
            RemoveTrackFailed: RemoveTrackFailed::<Impl, IMPL_OFFSET>,
            Cues: Cues::<Impl, IMPL_OFFSET>,
            ActiveCues: ActiveCues::<Impl, IMPL_OFFSET>,
            TimedMetadataKind: TimedMetadataKind::<Impl, IMPL_OFFSET>,
            DispatchType: DispatchType::<Impl, IMPL_OFFSET>,
            AddCue: AddCue::<Impl, IMPL_OFFSET>,
            RemoveCue: RemoveCue::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITimedMetadataTrack as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Playback", feature = "implement_exclusive"))]
pub trait ITimedMetadataTrack2Impl: Sized + IMediaTrackImpl + ITimedMetadataTrackImpl {
    fn PlaybackItem(&mut self) -> ::windows::core::Result<super::Playback::MediaPlaybackItem>;
    fn Name(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Playback", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITimedMetadataTrack2 {
    const NAME: &'static str = "Windows.Media.Core.ITimedMetadataTrack2";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Playback", feature = "implement_exclusive"))]
impl ITimedMetadataTrack2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITimedMetadataTrack2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITimedMetadataTrack2Vtbl {
        unsafe extern "system" fn PlaybackItem<Impl: ITimedMetadataTrack2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PlaybackItem() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: ITimedMetadataTrack2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITimedMetadataTrack2, BASE_OFFSET>(),
            PlaybackItem: PlaybackItem::<Impl, IMPL_OFFSET>,
            Name: Name::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITimedMetadataTrack2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimedMetadataTrackErrorImpl: Sized {
    fn ErrorCode(&mut self) -> ::windows::core::Result<TimedMetadataTrackErrorCode>;
    fn ExtendedError(&mut self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITimedMetadataTrackError {
    const NAME: &'static str = "Windows.Media.Core.ITimedMetadataTrackError";
}
#[cfg(feature = "implement_exclusive")]
impl ITimedMetadataTrackErrorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITimedMetadataTrackErrorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITimedMetadataTrackErrorVtbl {
        unsafe extern "system" fn ErrorCode<Impl: ITimedMetadataTrackErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut TimedMetadataTrackErrorCode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ErrorCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedError<Impl: ITimedMetadataTrackErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITimedMetadataTrackError, BASE_OFFSET>(),
            ErrorCode: ErrorCode::<Impl, IMPL_OFFSET>,
            ExtendedError: ExtendedError::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITimedMetadataTrackError as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimedMetadataTrackFactoryImpl: Sized {
    fn Create(&mut self, id: &::windows::core::HSTRING, language: &::windows::core::HSTRING, kind: TimedMetadataKind) -> ::windows::core::Result<TimedMetadataTrack>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITimedMetadataTrackFactory {
    const NAME: &'static str = "Windows.Media.Core.ITimedMetadataTrackFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ITimedMetadataTrackFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITimedMetadataTrackFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITimedMetadataTrackFactoryVtbl {
        unsafe extern "system" fn Create<Impl: ITimedMetadataTrackFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, language: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, kind: TimedMetadataKind, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&id as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&language as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), kind) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ITimedMetadataTrackFactory, BASE_OFFSET>(), Create: Create::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITimedMetadataTrackFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimedMetadataTrackFailedEventArgsImpl: Sized {
    fn Error(&mut self) -> ::windows::core::Result<TimedMetadataTrackError>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITimedMetadataTrackFailedEventArgs {
    const NAME: &'static str = "Windows.Media.Core.ITimedMetadataTrackFailedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ITimedMetadataTrackFailedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITimedMetadataTrackFailedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITimedMetadataTrackFailedEventArgsVtbl {
        unsafe extern "system" fn Error<Impl: ITimedMetadataTrackFailedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Error() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ITimedMetadataTrackFailedEventArgs, BASE_OFFSET>(), Error: Error::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITimedMetadataTrackFailedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait ITimedMetadataTrackProviderImpl: Sized {
    fn TimedMetadataTracks(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<TimedMetadataTrack>>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for ITimedMetadataTrackProvider {
    const NAME: &'static str = "Windows.Media.Core.ITimedMetadataTrackProvider";
}
#[cfg(feature = "Foundation_Collections")]
impl ITimedMetadataTrackProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITimedMetadataTrackProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITimedMetadataTrackProviderVtbl {
        unsafe extern "system" fn TimedMetadataTracks<Impl: ITimedMetadataTrackProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TimedMetadataTracks() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITimedMetadataTrackProvider, BASE_OFFSET>(),
            TimedMetadataTracks: TimedMetadataTracks::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITimedMetadataTrackProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI", feature = "implement_exclusive"))]
pub trait ITimedTextBoutenImpl: Sized {
    fn Type(&mut self) -> ::windows::core::Result<TimedTextBoutenType>;
    fn SetType(&mut self, value: TimedTextBoutenType) -> ::windows::core::Result<()>;
    fn Color(&mut self) -> ::windows::core::Result<super::super::UI::Color>;
    fn SetColor(&mut self, value: &super::super::UI::Color) -> ::windows::core::Result<()>;
    fn Position(&mut self) -> ::windows::core::Result<TimedTextBoutenPosition>;
    fn SetPosition(&mut self, value: TimedTextBoutenPosition) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "UI", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITimedTextBouten {
    const NAME: &'static str = "Windows.Media.Core.ITimedTextBouten";
}
#[cfg(all(feature = "UI", feature = "implement_exclusive"))]
impl ITimedTextBoutenVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITimedTextBoutenImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITimedTextBoutenVtbl {
        unsafe extern "system" fn Type<Impl: ITimedTextBoutenImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut TimedTextBoutenType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetType<Impl: ITimedTextBoutenImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: TimedTextBoutenType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetType(value).into()
        }
        unsafe extern "system" fn Color<Impl: ITimedTextBoutenImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Color() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColor<Impl: ITimedTextBoutenImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::UI::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetColor(&*(&value as *const <super::super::UI::Color as ::windows::core::Abi>::Abi as *const <super::super::UI::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Position<Impl: ITimedTextBoutenImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut TimedTextBoutenPosition) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPosition<Impl: ITimedTextBoutenImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: TimedTextBoutenPosition) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPosition(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITimedTextBouten, BASE_OFFSET>(),
            Type: Type::<Impl, IMPL_OFFSET>,
            SetType: SetType::<Impl, IMPL_OFFSET>,
            Color: Color::<Impl, IMPL_OFFSET>,
            SetColor: SetColor::<Impl, IMPL_OFFSET>,
            Position: Position::<Impl, IMPL_OFFSET>,
            SetPosition: SetPosition::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITimedTextBouten as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ITimedTextCueImpl: Sized + IMediaCueImpl {
    fn CueRegion(&mut self) -> ::windows::core::Result<TimedTextRegion>;
    fn SetCueRegion(&mut self, value: &::core::option::Option<TimedTextRegion>) -> ::windows::core::Result<()>;
    fn CueStyle(&mut self) -> ::windows::core::Result<TimedTextStyle>;
    fn SetCueStyle(&mut self, value: &::core::option::Option<TimedTextStyle>) -> ::windows::core::Result<()>;
    fn Lines(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<TimedTextLine>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITimedTextCue {
    const NAME: &'static str = "Windows.Media.Core.ITimedTextCue";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ITimedTextCueVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITimedTextCueImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITimedTextCueVtbl {
        unsafe extern "system" fn CueRegion<Impl: ITimedTextCueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CueRegion() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCueRegion<Impl: ITimedTextCueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCueRegion(&*(&value as *const <TimedTextRegion as ::windows::core::Abi>::Abi as *const <TimedTextRegion as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CueStyle<Impl: ITimedTextCueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CueStyle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCueStyle<Impl: ITimedTextCueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCueStyle(&*(&value as *const <TimedTextStyle as ::windows::core::Abi>::Abi as *const <TimedTextStyle as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Lines<Impl: ITimedTextCueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Lines() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITimedTextCue, BASE_OFFSET>(),
            CueRegion: CueRegion::<Impl, IMPL_OFFSET>,
            SetCueRegion: SetCueRegion::<Impl, IMPL_OFFSET>,
            CueStyle: CueStyle::<Impl, IMPL_OFFSET>,
            SetCueStyle: SetCueStyle::<Impl, IMPL_OFFSET>,
            Lines: Lines::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITimedTextCue as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ITimedTextLineImpl: Sized {
    fn Text(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetText(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Subformats(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<TimedTextSubformat>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITimedTextLine {
    const NAME: &'static str = "Windows.Media.Core.ITimedTextLine";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ITimedTextLineVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITimedTextLineImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITimedTextLineVtbl {
        unsafe extern "system" fn Text<Impl: ITimedTextLineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Text() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetText<Impl: ITimedTextLineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetText(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Subformats<Impl: ITimedTextLineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Subformats() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITimedTextLine, BASE_OFFSET>(),
            Text: Text::<Impl, IMPL_OFFSET>,
            SetText: SetText::<Impl, IMPL_OFFSET>,
            Subformats: Subformats::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITimedTextLine as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI", feature = "implement_exclusive"))]
pub trait ITimedTextRegionImpl: Sized {
    fn Name(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Position(&mut self) -> ::windows::core::Result<TimedTextPoint>;
    fn SetPosition(&mut self, value: &TimedTextPoint) -> ::windows::core::Result<()>;
    fn Extent(&mut self) -> ::windows::core::Result<TimedTextSize>;
    fn SetExtent(&mut self, value: &TimedTextSize) -> ::windows::core::Result<()>;
    fn Background(&mut self) -> ::windows::core::Result<super::super::UI::Color>;
    fn SetBackground(&mut self, value: &super::super::UI::Color) -> ::windows::core::Result<()>;
    fn WritingMode(&mut self) -> ::windows::core::Result<TimedTextWritingMode>;
    fn SetWritingMode(&mut self, value: TimedTextWritingMode) -> ::windows::core::Result<()>;
    fn DisplayAlignment(&mut self) -> ::windows::core::Result<TimedTextDisplayAlignment>;
    fn SetDisplayAlignment(&mut self, value: TimedTextDisplayAlignment) -> ::windows::core::Result<()>;
    fn LineHeight(&mut self) -> ::windows::core::Result<TimedTextDouble>;
    fn SetLineHeight(&mut self, value: &TimedTextDouble) -> ::windows::core::Result<()>;
    fn IsOverflowClipped(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsOverflowClipped(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn Padding(&mut self) -> ::windows::core::Result<TimedTextPadding>;
    fn SetPadding(&mut self, value: &TimedTextPadding) -> ::windows::core::Result<()>;
    fn TextWrapping(&mut self) -> ::windows::core::Result<TimedTextWrapping>;
    fn SetTextWrapping(&mut self, value: TimedTextWrapping) -> ::windows::core::Result<()>;
    fn ZIndex(&mut self) -> ::windows::core::Result<i32>;
    fn SetZIndex(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn ScrollMode(&mut self) -> ::windows::core::Result<TimedTextScrollMode>;
    fn SetScrollMode(&mut self, value: TimedTextScrollMode) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "UI", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITimedTextRegion {
    const NAME: &'static str = "Windows.Media.Core.ITimedTextRegion";
}
#[cfg(all(feature = "UI", feature = "implement_exclusive"))]
impl ITimedTextRegionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITimedTextRegionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITimedTextRegionVtbl {
        unsafe extern "system" fn Name<Impl: ITimedTextRegionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: ITimedTextRegionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Position<Impl: ITimedTextRegionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut TimedTextPoint) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPosition<Impl: ITimedTextRegionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: TimedTextPoint) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPosition(&*(&value as *const <TimedTextPoint as ::windows::core::Abi>::Abi as *const <TimedTextPoint as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Extent<Impl: ITimedTextRegionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut TimedTextSize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Extent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExtent<Impl: ITimedTextRegionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: TimedTextSize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExtent(&*(&value as *const <TimedTextSize as ::windows::core::Abi>::Abi as *const <TimedTextSize as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Background<Impl: ITimedTextRegionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Background() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBackground<Impl: ITimedTextRegionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::UI::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBackground(&*(&value as *const <super::super::UI::Color as ::windows::core::Abi>::Abi as *const <super::super::UI::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn WritingMode<Impl: ITimedTextRegionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut TimedTextWritingMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WritingMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWritingMode<Impl: ITimedTextRegionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: TimedTextWritingMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWritingMode(value).into()
        }
        unsafe extern "system" fn DisplayAlignment<Impl: ITimedTextRegionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut TimedTextDisplayAlignment) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayAlignment() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayAlignment<Impl: ITimedTextRegionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: TimedTextDisplayAlignment) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisplayAlignment(value).into()
        }
        unsafe extern "system" fn LineHeight<Impl: ITimedTextRegionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut TimedTextDouble) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LineHeight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLineHeight<Impl: ITimedTextRegionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: TimedTextDouble) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLineHeight(&*(&value as *const <TimedTextDouble as ::windows::core::Abi>::Abi as *const <TimedTextDouble as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsOverflowClipped<Impl: ITimedTextRegionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsOverflowClipped() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsOverflowClipped<Impl: ITimedTextRegionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsOverflowClipped(value).into()
        }
        unsafe extern "system" fn Padding<Impl: ITimedTextRegionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut TimedTextPadding) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Padding() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPadding<Impl: ITimedTextRegionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: TimedTextPadding) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPadding(&*(&value as *const <TimedTextPadding as ::windows::core::Abi>::Abi as *const <TimedTextPadding as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TextWrapping<Impl: ITimedTextRegionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut TimedTextWrapping) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TextWrapping() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTextWrapping<Impl: ITimedTextRegionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: TimedTextWrapping) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTextWrapping(value).into()
        }
        unsafe extern "system" fn ZIndex<Impl: ITimedTextRegionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ZIndex() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetZIndex<Impl: ITimedTextRegionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetZIndex(value).into()
        }
        unsafe extern "system" fn ScrollMode<Impl: ITimedTextRegionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut TimedTextScrollMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScrollMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScrollMode<Impl: ITimedTextRegionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: TimedTextScrollMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScrollMode(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITimedTextRegion, BASE_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            SetName: SetName::<Impl, IMPL_OFFSET>,
            Position: Position::<Impl, IMPL_OFFSET>,
            SetPosition: SetPosition::<Impl, IMPL_OFFSET>,
            Extent: Extent::<Impl, IMPL_OFFSET>,
            SetExtent: SetExtent::<Impl, IMPL_OFFSET>,
            Background: Background::<Impl, IMPL_OFFSET>,
            SetBackground: SetBackground::<Impl, IMPL_OFFSET>,
            WritingMode: WritingMode::<Impl, IMPL_OFFSET>,
            SetWritingMode: SetWritingMode::<Impl, IMPL_OFFSET>,
            DisplayAlignment: DisplayAlignment::<Impl, IMPL_OFFSET>,
            SetDisplayAlignment: SetDisplayAlignment::<Impl, IMPL_OFFSET>,
            LineHeight: LineHeight::<Impl, IMPL_OFFSET>,
            SetLineHeight: SetLineHeight::<Impl, IMPL_OFFSET>,
            IsOverflowClipped: IsOverflowClipped::<Impl, IMPL_OFFSET>,
            SetIsOverflowClipped: SetIsOverflowClipped::<Impl, IMPL_OFFSET>,
            Padding: Padding::<Impl, IMPL_OFFSET>,
            SetPadding: SetPadding::<Impl, IMPL_OFFSET>,
            TextWrapping: TextWrapping::<Impl, IMPL_OFFSET>,
            SetTextWrapping: SetTextWrapping::<Impl, IMPL_OFFSET>,
            ZIndex: ZIndex::<Impl, IMPL_OFFSET>,
            SetZIndex: SetZIndex::<Impl, IMPL_OFFSET>,
            ScrollMode: ScrollMode::<Impl, IMPL_OFFSET>,
            SetScrollMode: SetScrollMode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITimedTextRegion as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimedTextRubyImpl: Sized {
    fn Text(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetText(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Position(&mut self) -> ::windows::core::Result<TimedTextRubyPosition>;
    fn SetPosition(&mut self, value: TimedTextRubyPosition) -> ::windows::core::Result<()>;
    fn Align(&mut self) -> ::windows::core::Result<TimedTextRubyAlign>;
    fn SetAlign(&mut self, value: TimedTextRubyAlign) -> ::windows::core::Result<()>;
    fn Reserve(&mut self) -> ::windows::core::Result<TimedTextRubyReserve>;
    fn SetReserve(&mut self, value: TimedTextRubyReserve) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITimedTextRuby {
    const NAME: &'static str = "Windows.Media.Core.ITimedTextRuby";
}
#[cfg(feature = "implement_exclusive")]
impl ITimedTextRubyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITimedTextRubyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITimedTextRubyVtbl {
        unsafe extern "system" fn Text<Impl: ITimedTextRubyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Text() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetText<Impl: ITimedTextRubyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetText(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Position<Impl: ITimedTextRubyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut TimedTextRubyPosition) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPosition<Impl: ITimedTextRubyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: TimedTextRubyPosition) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPosition(value).into()
        }
        unsafe extern "system" fn Align<Impl: ITimedTextRubyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut TimedTextRubyAlign) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Align() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlign<Impl: ITimedTextRubyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: TimedTextRubyAlign) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAlign(value).into()
        }
        unsafe extern "system" fn Reserve<Impl: ITimedTextRubyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut TimedTextRubyReserve) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reserve() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReserve<Impl: ITimedTextRubyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: TimedTextRubyReserve) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReserve(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITimedTextRuby, BASE_OFFSET>(),
            Text: Text::<Impl, IMPL_OFFSET>,
            SetText: SetText::<Impl, IMPL_OFFSET>,
            Position: Position::<Impl, IMPL_OFFSET>,
            SetPosition: SetPosition::<Impl, IMPL_OFFSET>,
            Align: Align::<Impl, IMPL_OFFSET>,
            SetAlign: SetAlign::<Impl, IMPL_OFFSET>,
            Reserve: Reserve::<Impl, IMPL_OFFSET>,
            SetReserve: SetReserve::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITimedTextRuby as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ITimedTextSourceImpl: Sized {
    fn Resolved(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<TimedTextSource, TimedTextSourceResolveResultEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveResolved(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITimedTextSource {
    const NAME: &'static str = "Windows.Media.Core.ITimedTextSource";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ITimedTextSourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITimedTextSourceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITimedTextSourceVtbl {
        unsafe extern "system" fn Resolved<Impl: ITimedTextSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Resolved(&*(&handler as *const <super::super::Foundation::TypedEventHandler<TimedTextSource, TimedTextSourceResolveResultEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<TimedTextSource, TimedTextSourceResolveResultEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveResolved<Impl: ITimedTextSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveResolved(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITimedTextSource, BASE_OFFSET>(),
            Resolved: Resolved::<Impl, IMPL_OFFSET>,
            RemoveResolved: RemoveResolved::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITimedTextSource as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ITimedTextSourceResolveResultEventArgsImpl: Sized {
    fn Error(&mut self) -> ::windows::core::Result<TimedMetadataTrackError>;
    fn Tracks(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<TimedMetadataTrack>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITimedTextSourceResolveResultEventArgs {
    const NAME: &'static str = "Windows.Media.Core.ITimedTextSourceResolveResultEventArgs";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ITimedTextSourceResolveResultEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITimedTextSourceResolveResultEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITimedTextSourceResolveResultEventArgsVtbl {
        unsafe extern "system" fn Error<Impl: ITimedTextSourceResolveResultEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Error() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Tracks<Impl: ITimedTextSourceResolveResultEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Tracks() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITimedTextSourceResolveResultEventArgs, BASE_OFFSET>(),
            Error: Error::<Impl, IMPL_OFFSET>,
            Tracks: Tracks::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITimedTextSourceResolveResultEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait ITimedTextSourceStaticsImpl: Sized {
    fn CreateFromStream(&mut self, stream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<TimedTextSource>;
    fn CreateFromUri(&mut self, uri: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<TimedTextSource>;
    fn CreateFromStreamWithLanguage(&mut self, stream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>, defaultlanguage: &::windows::core::HSTRING) -> ::windows::core::Result<TimedTextSource>;
    fn CreateFromUriWithLanguage(&mut self, uri: &::core::option::Option<super::super::Foundation::Uri>, defaultlanguage: &::windows::core::HSTRING) -> ::windows::core::Result<TimedTextSource>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITimedTextSourceStatics {
    const NAME: &'static str = "Windows.Media.Core.ITimedTextSourceStatics";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ITimedTextSourceStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITimedTextSourceStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITimedTextSourceStaticsVtbl {
        unsafe extern "system" fn CreateFromStream<Impl: ITimedTextSourceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromStream(&*(&stream as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromUri<Impl: ITimedTextSourceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromUri(&*(&uri as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromStreamWithLanguage<Impl: ITimedTextSourceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, defaultlanguage: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromStreamWithLanguage(&*(&stream as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::DefaultType>::DefaultType), &*(&defaultlanguage as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromUriWithLanguage<Impl: ITimedTextSourceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, defaultlanguage: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromUriWithLanguage(&*(&uri as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType), &*(&defaultlanguage as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITimedTextSourceStatics, BASE_OFFSET>(),
            CreateFromStream: CreateFromStream::<Impl, IMPL_OFFSET>,
            CreateFromUri: CreateFromUri::<Impl, IMPL_OFFSET>,
            CreateFromStreamWithLanguage: CreateFromStreamWithLanguage::<Impl, IMPL_OFFSET>,
            CreateFromUriWithLanguage: CreateFromUriWithLanguage::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITimedTextSourceStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait ITimedTextSourceStatics2Impl: Sized {
    fn CreateFromStreamWithIndex(&mut self, stream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>, indexstream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<TimedTextSource>;
    fn CreateFromUriWithIndex(&mut self, uri: &::core::option::Option<super::super::Foundation::Uri>, indexuri: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<TimedTextSource>;
    fn CreateFromStreamWithIndexAndLanguage(&mut self, stream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>, indexstream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>, defaultlanguage: &::windows::core::HSTRING) -> ::windows::core::Result<TimedTextSource>;
    fn CreateFromUriWithIndexAndLanguage(&mut self, uri: &::core::option::Option<super::super::Foundation::Uri>, indexuri: &::core::option::Option<super::super::Foundation::Uri>, defaultlanguage: &::windows::core::HSTRING) -> ::windows::core::Result<TimedTextSource>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITimedTextSourceStatics2 {
    const NAME: &'static str = "Windows.Media.Core.ITimedTextSourceStatics2";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ITimedTextSourceStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITimedTextSourceStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITimedTextSourceStatics2Vtbl {
        unsafe extern "system" fn CreateFromStreamWithIndex<Impl: ITimedTextSourceStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, indexstream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromStreamWithIndex(&*(&stream as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::DefaultType>::DefaultType), &*(&indexstream as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromUriWithIndex<Impl: ITimedTextSourceStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, indexuri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromUriWithIndex(&*(&uri as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType), &*(&indexuri as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromStreamWithIndexAndLanguage<Impl: ITimedTextSourceStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, indexstream: ::windows::core::RawPtr, defaultlanguage: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromStreamWithIndexAndLanguage(
                &*(&stream as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::DefaultType>::DefaultType),
                &*(&indexstream as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::DefaultType>::DefaultType),
                &*(&defaultlanguage as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromUriWithIndexAndLanguage<Impl: ITimedTextSourceStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, indexuri: ::windows::core::RawPtr, defaultlanguage: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromUriWithIndexAndLanguage(
                &*(&uri as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType),
                &*(&indexuri as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType),
                &*(&defaultlanguage as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITimedTextSourceStatics2, BASE_OFFSET>(),
            CreateFromStreamWithIndex: CreateFromStreamWithIndex::<Impl, IMPL_OFFSET>,
            CreateFromUriWithIndex: CreateFromUriWithIndex::<Impl, IMPL_OFFSET>,
            CreateFromStreamWithIndexAndLanguage: CreateFromStreamWithIndexAndLanguage::<Impl, IMPL_OFFSET>,
            CreateFromUriWithIndexAndLanguage: CreateFromUriWithIndexAndLanguage::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITimedTextSourceStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI", feature = "implement_exclusive"))]
pub trait ITimedTextStyleImpl: Sized {
    fn Name(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn FontFamily(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetFontFamily(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn FontSize(&mut self) -> ::windows::core::Result<TimedTextDouble>;
    fn SetFontSize(&mut self, value: &TimedTextDouble) -> ::windows::core::Result<()>;
    fn FontWeight(&mut self) -> ::windows::core::Result<TimedTextWeight>;
    fn SetFontWeight(&mut self, value: TimedTextWeight) -> ::windows::core::Result<()>;
    fn Foreground(&mut self) -> ::windows::core::Result<super::super::UI::Color>;
    fn SetForeground(&mut self, value: &super::super::UI::Color) -> ::windows::core::Result<()>;
    fn Background(&mut self) -> ::windows::core::Result<super::super::UI::Color>;
    fn SetBackground(&mut self, value: &super::super::UI::Color) -> ::windows::core::Result<()>;
    fn IsBackgroundAlwaysShown(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsBackgroundAlwaysShown(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn FlowDirection(&mut self) -> ::windows::core::Result<TimedTextFlowDirection>;
    fn SetFlowDirection(&mut self, value: TimedTextFlowDirection) -> ::windows::core::Result<()>;
    fn LineAlignment(&mut self) -> ::windows::core::Result<TimedTextLineAlignment>;
    fn SetLineAlignment(&mut self, value: TimedTextLineAlignment) -> ::windows::core::Result<()>;
    fn OutlineColor(&mut self) -> ::windows::core::Result<super::super::UI::Color>;
    fn SetOutlineColor(&mut self, value: &super::super::UI::Color) -> ::windows::core::Result<()>;
    fn OutlineThickness(&mut self) -> ::windows::core::Result<TimedTextDouble>;
    fn SetOutlineThickness(&mut self, value: &TimedTextDouble) -> ::windows::core::Result<()>;
    fn OutlineRadius(&mut self) -> ::windows::core::Result<TimedTextDouble>;
    fn SetOutlineRadius(&mut self, value: &TimedTextDouble) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "UI", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITimedTextStyle {
    const NAME: &'static str = "Windows.Media.Core.ITimedTextStyle";
}
#[cfg(all(feature = "UI", feature = "implement_exclusive"))]
impl ITimedTextStyleVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITimedTextStyleImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITimedTextStyleVtbl {
        unsafe extern "system" fn Name<Impl: ITimedTextStyleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: ITimedTextStyleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FontFamily<Impl: ITimedTextStyleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FontFamily() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFontFamily<Impl: ITimedTextStyleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFontFamily(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FontSize<Impl: ITimedTextStyleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut TimedTextDouble) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FontSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFontSize<Impl: ITimedTextStyleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: TimedTextDouble) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFontSize(&*(&value as *const <TimedTextDouble as ::windows::core::Abi>::Abi as *const <TimedTextDouble as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FontWeight<Impl: ITimedTextStyleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut TimedTextWeight) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FontWeight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFontWeight<Impl: ITimedTextStyleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: TimedTextWeight) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFontWeight(value).into()
        }
        unsafe extern "system" fn Foreground<Impl: ITimedTextStyleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Foreground() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetForeground<Impl: ITimedTextStyleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::UI::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetForeground(&*(&value as *const <super::super::UI::Color as ::windows::core::Abi>::Abi as *const <super::super::UI::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Background<Impl: ITimedTextStyleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Background() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBackground<Impl: ITimedTextStyleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::UI::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBackground(&*(&value as *const <super::super::UI::Color as ::windows::core::Abi>::Abi as *const <super::super::UI::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsBackgroundAlwaysShown<Impl: ITimedTextStyleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsBackgroundAlwaysShown() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsBackgroundAlwaysShown<Impl: ITimedTextStyleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsBackgroundAlwaysShown(value).into()
        }
        unsafe extern "system" fn FlowDirection<Impl: ITimedTextStyleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut TimedTextFlowDirection) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FlowDirection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFlowDirection<Impl: ITimedTextStyleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: TimedTextFlowDirection) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFlowDirection(value).into()
        }
        unsafe extern "system" fn LineAlignment<Impl: ITimedTextStyleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut TimedTextLineAlignment) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LineAlignment() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLineAlignment<Impl: ITimedTextStyleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: TimedTextLineAlignment) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLineAlignment(value).into()
        }
        unsafe extern "system" fn OutlineColor<Impl: ITimedTextStyleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OutlineColor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutlineColor<Impl: ITimedTextStyleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::UI::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOutlineColor(&*(&value as *const <super::super::UI::Color as ::windows::core::Abi>::Abi as *const <super::super::UI::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OutlineThickness<Impl: ITimedTextStyleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut TimedTextDouble) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OutlineThickness() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutlineThickness<Impl: ITimedTextStyleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: TimedTextDouble) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOutlineThickness(&*(&value as *const <TimedTextDouble as ::windows::core::Abi>::Abi as *const <TimedTextDouble as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OutlineRadius<Impl: ITimedTextStyleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut TimedTextDouble) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OutlineRadius() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutlineRadius<Impl: ITimedTextStyleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: TimedTextDouble) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOutlineRadius(&*(&value as *const <TimedTextDouble as ::windows::core::Abi>::Abi as *const <TimedTextDouble as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITimedTextStyle, BASE_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            SetName: SetName::<Impl, IMPL_OFFSET>,
            FontFamily: FontFamily::<Impl, IMPL_OFFSET>,
            SetFontFamily: SetFontFamily::<Impl, IMPL_OFFSET>,
            FontSize: FontSize::<Impl, IMPL_OFFSET>,
            SetFontSize: SetFontSize::<Impl, IMPL_OFFSET>,
            FontWeight: FontWeight::<Impl, IMPL_OFFSET>,
            SetFontWeight: SetFontWeight::<Impl, IMPL_OFFSET>,
            Foreground: Foreground::<Impl, IMPL_OFFSET>,
            SetForeground: SetForeground::<Impl, IMPL_OFFSET>,
            Background: Background::<Impl, IMPL_OFFSET>,
            SetBackground: SetBackground::<Impl, IMPL_OFFSET>,
            IsBackgroundAlwaysShown: IsBackgroundAlwaysShown::<Impl, IMPL_OFFSET>,
            SetIsBackgroundAlwaysShown: SetIsBackgroundAlwaysShown::<Impl, IMPL_OFFSET>,
            FlowDirection: FlowDirection::<Impl, IMPL_OFFSET>,
            SetFlowDirection: SetFlowDirection::<Impl, IMPL_OFFSET>,
            LineAlignment: LineAlignment::<Impl, IMPL_OFFSET>,
            SetLineAlignment: SetLineAlignment::<Impl, IMPL_OFFSET>,
            OutlineColor: OutlineColor::<Impl, IMPL_OFFSET>,
            SetOutlineColor: SetOutlineColor::<Impl, IMPL_OFFSET>,
            OutlineThickness: OutlineThickness::<Impl, IMPL_OFFSET>,
            SetOutlineThickness: SetOutlineThickness::<Impl, IMPL_OFFSET>,
            OutlineRadius: OutlineRadius::<Impl, IMPL_OFFSET>,
            SetOutlineRadius: SetOutlineRadius::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITimedTextStyle as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimedTextStyle2Impl: Sized {
    fn FontStyle(&mut self) -> ::windows::core::Result<TimedTextFontStyle>;
    fn SetFontStyle(&mut self, value: TimedTextFontStyle) -> ::windows::core::Result<()>;
    fn IsUnderlineEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsUnderlineEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsLineThroughEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsLineThroughEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsOverlineEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsOverlineEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITimedTextStyle2 {
    const NAME: &'static str = "Windows.Media.Core.ITimedTextStyle2";
}
#[cfg(feature = "implement_exclusive")]
impl ITimedTextStyle2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITimedTextStyle2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITimedTextStyle2Vtbl {
        unsafe extern "system" fn FontStyle<Impl: ITimedTextStyle2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut TimedTextFontStyle) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FontStyle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFontStyle<Impl: ITimedTextStyle2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: TimedTextFontStyle) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFontStyle(value).into()
        }
        unsafe extern "system" fn IsUnderlineEnabled<Impl: ITimedTextStyle2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsUnderlineEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsUnderlineEnabled<Impl: ITimedTextStyle2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsUnderlineEnabled(value).into()
        }
        unsafe extern "system" fn IsLineThroughEnabled<Impl: ITimedTextStyle2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsLineThroughEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsLineThroughEnabled<Impl: ITimedTextStyle2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsLineThroughEnabled(value).into()
        }
        unsafe extern "system" fn IsOverlineEnabled<Impl: ITimedTextStyle2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsOverlineEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsOverlineEnabled<Impl: ITimedTextStyle2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsOverlineEnabled(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITimedTextStyle2, BASE_OFFSET>(),
            FontStyle: FontStyle::<Impl, IMPL_OFFSET>,
            SetFontStyle: SetFontStyle::<Impl, IMPL_OFFSET>,
            IsUnderlineEnabled: IsUnderlineEnabled::<Impl, IMPL_OFFSET>,
            SetIsUnderlineEnabled: SetIsUnderlineEnabled::<Impl, IMPL_OFFSET>,
            IsLineThroughEnabled: IsLineThroughEnabled::<Impl, IMPL_OFFSET>,
            SetIsLineThroughEnabled: SetIsLineThroughEnabled::<Impl, IMPL_OFFSET>,
            IsOverlineEnabled: IsOverlineEnabled::<Impl, IMPL_OFFSET>,
            SetIsOverlineEnabled: SetIsOverlineEnabled::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITimedTextStyle2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimedTextStyle3Impl: Sized {
    fn Ruby(&mut self) -> ::windows::core::Result<TimedTextRuby>;
    fn Bouten(&mut self) -> ::windows::core::Result<TimedTextBouten>;
    fn IsTextCombined(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsTextCombined(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn FontAngleInDegrees(&mut self) -> ::windows::core::Result<f64>;
    fn SetFontAngleInDegrees(&mut self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITimedTextStyle3 {
    const NAME: &'static str = "Windows.Media.Core.ITimedTextStyle3";
}
#[cfg(feature = "implement_exclusive")]
impl ITimedTextStyle3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITimedTextStyle3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITimedTextStyle3Vtbl {
        unsafe extern "system" fn Ruby<Impl: ITimedTextStyle3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Ruby() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Bouten<Impl: ITimedTextStyle3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Bouten() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsTextCombined<Impl: ITimedTextStyle3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsTextCombined() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsTextCombined<Impl: ITimedTextStyle3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsTextCombined(value).into()
        }
        unsafe extern "system" fn FontAngleInDegrees<Impl: ITimedTextStyle3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FontAngleInDegrees() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFontAngleInDegrees<Impl: ITimedTextStyle3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFontAngleInDegrees(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITimedTextStyle3, BASE_OFFSET>(),
            Ruby: Ruby::<Impl, IMPL_OFFSET>,
            Bouten: Bouten::<Impl, IMPL_OFFSET>,
            IsTextCombined: IsTextCombined::<Impl, IMPL_OFFSET>,
            SetIsTextCombined: SetIsTextCombined::<Impl, IMPL_OFFSET>,
            FontAngleInDegrees: FontAngleInDegrees::<Impl, IMPL_OFFSET>,
            SetFontAngleInDegrees: SetFontAngleInDegrees::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITimedTextStyle3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimedTextSubformatImpl: Sized {
    fn StartIndex(&mut self) -> ::windows::core::Result<i32>;
    fn SetStartIndex(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn Length(&mut self) -> ::windows::core::Result<i32>;
    fn SetLength(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn SubformatStyle(&mut self) -> ::windows::core::Result<TimedTextStyle>;
    fn SetSubformatStyle(&mut self, value: &::core::option::Option<TimedTextStyle>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITimedTextSubformat {
    const NAME: &'static str = "Windows.Media.Core.ITimedTextSubformat";
}
#[cfg(feature = "implement_exclusive")]
impl ITimedTextSubformatVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITimedTextSubformatImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITimedTextSubformatVtbl {
        unsafe extern "system" fn StartIndex<Impl: ITimedTextSubformatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartIndex() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartIndex<Impl: ITimedTextSubformatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStartIndex(value).into()
        }
        unsafe extern "system" fn Length<Impl: ITimedTextSubformatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Length() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLength<Impl: ITimedTextSubformatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLength(value).into()
        }
        unsafe extern "system" fn SubformatStyle<Impl: ITimedTextSubformatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SubformatStyle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSubformatStyle<Impl: ITimedTextSubformatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSubformatStyle(&*(&value as *const <TimedTextStyle as ::windows::core::Abi>::Abi as *const <TimedTextStyle as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITimedTextSubformat, BASE_OFFSET>(),
            StartIndex: StartIndex::<Impl, IMPL_OFFSET>,
            SetStartIndex: SetStartIndex::<Impl, IMPL_OFFSET>,
            Length: Length::<Impl, IMPL_OFFSET>,
            SetLength: SetLength::<Impl, IMPL_OFFSET>,
            SubformatStyle: SubformatStyle::<Impl, IMPL_OFFSET>,
            SetSubformatStyle: SetSubformatStyle::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITimedTextSubformat as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Capture", feature = "Media_Devices", feature = "Media_MediaProperties", feature = "implement_exclusive"))]
pub trait IVideoStabilizationEffectImpl: Sized + IMediaExtensionImpl {
    fn SetEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn Enabled(&mut self) -> ::windows::core::Result<bool>;
    fn EnabledChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<VideoStabilizationEffect, VideoStabilizationEffectEnabledChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveEnabledChanged(&mut self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GetRecommendedStreamConfiguration(&mut self, controller: &::core::option::Option<super::Devices::VideoDeviceController>, desiredproperties: &::core::option::Option<super::MediaProperties::VideoEncodingProperties>) -> ::windows::core::Result<super::Capture::VideoStreamConfiguration>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Capture", feature = "Media_Devices", feature = "Media_MediaProperties", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVideoStabilizationEffect {
    const NAME: &'static str = "Windows.Media.Core.IVideoStabilizationEffect";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Capture", feature = "Media_Devices", feature = "Media_MediaProperties", feature = "implement_exclusive"))]
impl IVideoStabilizationEffectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVideoStabilizationEffectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVideoStabilizationEffectVtbl {
        unsafe extern "system" fn SetEnabled<Impl: IVideoStabilizationEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnabled(value).into()
        }
        unsafe extern "system" fn Enabled<Impl: IVideoStabilizationEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Enabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnabledChanged<Impl: IVideoStabilizationEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnabledChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<VideoStabilizationEffect, VideoStabilizationEffectEnabledChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<VideoStabilizationEffect, VideoStabilizationEffectEnabledChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveEnabledChanged<Impl: IVideoStabilizationEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveEnabledChanged(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetRecommendedStreamConfiguration<Impl: IVideoStabilizationEffectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, controller: ::windows::core::RawPtr, desiredproperties: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRecommendedStreamConfiguration(&*(&controller as *const <super::Devices::VideoDeviceController as ::windows::core::Abi>::Abi as *const <super::Devices::VideoDeviceController as ::windows::core::DefaultType>::DefaultType), &*(&desiredproperties as *const <super::MediaProperties::VideoEncodingProperties as ::windows::core::Abi>::Abi as *const <super::MediaProperties::VideoEncodingProperties as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVideoStabilizationEffect, BASE_OFFSET>(),
            SetEnabled: SetEnabled::<Impl, IMPL_OFFSET>,
            Enabled: Enabled::<Impl, IMPL_OFFSET>,
            EnabledChanged: EnabledChanged::<Impl, IMPL_OFFSET>,
            RemoveEnabledChanged: RemoveEnabledChanged::<Impl, IMPL_OFFSET>,
            GetRecommendedStreamConfiguration: GetRecommendedStreamConfiguration::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVideoStabilizationEffect as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVideoStabilizationEffectEnabledChangedEventArgsImpl: Sized {
    fn Reason(&mut self) -> ::windows::core::Result<VideoStabilizationEffectEnabledChangedReason>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVideoStabilizationEffectEnabledChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Core.IVideoStabilizationEffectEnabledChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IVideoStabilizationEffectEnabledChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVideoStabilizationEffectEnabledChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVideoStabilizationEffectEnabledChangedEventArgsVtbl {
        unsafe extern "system" fn Reason<Impl: IVideoStabilizationEffectEnabledChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut VideoStabilizationEffectEnabledChangedReason) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVideoStabilizationEffectEnabledChangedEventArgs, BASE_OFFSET>(),
            Reason: Reason::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVideoStabilizationEffectEnabledChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Media_MediaProperties", feature = "implement_exclusive"))]
pub trait IVideoStreamDescriptorImpl: Sized + IMediaStreamDescriptorImpl {
    fn EncodingProperties(&mut self) -> ::windows::core::Result<super::MediaProperties::VideoEncodingProperties>;
}
#[cfg(all(feature = "Media_MediaProperties", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVideoStreamDescriptor {
    const NAME: &'static str = "Windows.Media.Core.IVideoStreamDescriptor";
}
#[cfg(all(feature = "Media_MediaProperties", feature = "implement_exclusive"))]
impl IVideoStreamDescriptorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVideoStreamDescriptorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVideoStreamDescriptorVtbl {
        unsafe extern "system" fn EncodingProperties<Impl: IVideoStreamDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EncodingProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVideoStreamDescriptor, BASE_OFFSET>(),
            EncodingProperties: EncodingProperties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVideoStreamDescriptor as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVideoStreamDescriptor2Impl: Sized {
    fn Copy(&mut self) -> ::windows::core::Result<VideoStreamDescriptor>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVideoStreamDescriptor2 {
    const NAME: &'static str = "Windows.Media.Core.IVideoStreamDescriptor2";
}
#[cfg(feature = "implement_exclusive")]
impl IVideoStreamDescriptor2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVideoStreamDescriptor2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVideoStreamDescriptor2Vtbl {
        unsafe extern "system" fn Copy<Impl: IVideoStreamDescriptor2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Copy() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IVideoStreamDescriptor2, BASE_OFFSET>(), Copy: Copy::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVideoStreamDescriptor2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Media_MediaProperties", feature = "implement_exclusive"))]
pub trait IVideoStreamDescriptorFactoryImpl: Sized {
    fn Create(&mut self, encodingproperties: &::core::option::Option<super::MediaProperties::VideoEncodingProperties>) -> ::windows::core::Result<VideoStreamDescriptor>;
}
#[cfg(all(feature = "Media_MediaProperties", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVideoStreamDescriptorFactory {
    const NAME: &'static str = "Windows.Media.Core.IVideoStreamDescriptorFactory";
}
#[cfg(all(feature = "Media_MediaProperties", feature = "implement_exclusive"))]
impl IVideoStreamDescriptorFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVideoStreamDescriptorFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVideoStreamDescriptorFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IVideoStreamDescriptorFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encodingproperties: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&encodingproperties as *const <super::MediaProperties::VideoEncodingProperties as ::windows::core::Abi>::Abi as *const <super::MediaProperties::VideoEncodingProperties as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IVideoStreamDescriptorFactory, BASE_OFFSET>(), Create: Create::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVideoStreamDescriptorFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Media_Playback", feature = "implement_exclusive"))]
pub trait IVideoTrackImpl: Sized {
    fn OpenFailed(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<VideoTrack, VideoTrackOpenFailedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveOpenFailed(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GetEncodingProperties(&mut self) -> ::windows::core::Result<super::MediaProperties::VideoEncodingProperties>;
    fn PlaybackItem(&mut self) -> ::windows::core::Result<super::Playback::MediaPlaybackItem>;
    fn Name(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SupportInfo(&mut self) -> ::windows::core::Result<VideoTrackSupportInfo>;
}
#[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Media_Playback", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVideoTrack {
    const NAME: &'static str = "Windows.Media.Core.IVideoTrack";
}
#[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Media_Playback", feature = "implement_exclusive"))]
impl IVideoTrackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVideoTrackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVideoTrackVtbl {
        unsafe extern "system" fn OpenFailed<Impl: IVideoTrackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenFailed(&*(&handler as *const <super::super::Foundation::TypedEventHandler<VideoTrack, VideoTrackOpenFailedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<VideoTrack, VideoTrackOpenFailedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveOpenFailed<Impl: IVideoTrackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveOpenFailed(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetEncodingProperties<Impl: IVideoTrackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEncodingProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PlaybackItem<Impl: IVideoTrackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PlaybackItem() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: IVideoTrackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportInfo<Impl: IVideoTrackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVideoTrack, BASE_OFFSET>(),
            OpenFailed: OpenFailed::<Impl, IMPL_OFFSET>,
            RemoveOpenFailed: RemoveOpenFailed::<Impl, IMPL_OFFSET>,
            GetEncodingProperties: GetEncodingProperties::<Impl, IMPL_OFFSET>,
            PlaybackItem: PlaybackItem::<Impl, IMPL_OFFSET>,
            Name: Name::<Impl, IMPL_OFFSET>,
            SupportInfo: SupportInfo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVideoTrack as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVideoTrackOpenFailedEventArgsImpl: Sized {
    fn ExtendedError(&mut self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVideoTrackOpenFailedEventArgs {
    const NAME: &'static str = "Windows.Media.Core.IVideoTrackOpenFailedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IVideoTrackOpenFailedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVideoTrackOpenFailedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVideoTrackOpenFailedEventArgsVtbl {
        unsafe extern "system" fn ExtendedError<Impl: IVideoTrackOpenFailedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVideoTrackOpenFailedEventArgs, BASE_OFFSET>(),
            ExtendedError: ExtendedError::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVideoTrackOpenFailedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVideoTrackSupportInfoImpl: Sized {
    fn DecoderStatus(&mut self) -> ::windows::core::Result<MediaDecoderStatus>;
    fn MediaSourceStatus(&mut self) -> ::windows::core::Result<MediaSourceStatus>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVideoTrackSupportInfo {
    const NAME: &'static str = "Windows.Media.Core.IVideoTrackSupportInfo";
}
#[cfg(feature = "implement_exclusive")]
impl IVideoTrackSupportInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVideoTrackSupportInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVideoTrackSupportInfoVtbl {
        unsafe extern "system" fn DecoderStatus<Impl: IVideoTrackSupportInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MediaDecoderStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DecoderStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediaSourceStatus<Impl: IVideoTrackSupportInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MediaSourceStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MediaSourceStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVideoTrackSupportInfo, BASE_OFFSET>(),
            DecoderStatus: DecoderStatus::<Impl, IMPL_OFFSET>,
            MediaSourceStatus: MediaSourceStatus::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVideoTrackSupportInfo as ::windows::core::Interface>::IID
    }
}
