#[cfg(feature = "implement_exclusive")]
pub trait IAudioStreamDescriptorImpl: Sized + IMediaStreamDescriptorImpl {
    fn EncodingProperties(&self) -> ::windows::core::Result<super::MediaProperties::AudioEncodingProperties>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAudioStreamDescriptor {
    const NAME: &'static str = "Windows.Media.Core.IAudioStreamDescriptor";
}
#[cfg(feature = "implement_exclusive")]
impl IAudioStreamDescriptorVtbl {
    pub const fn new<Impl: IAudioStreamDescriptorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAudioStreamDescriptorVtbl {
        unsafe extern "system" fn EncodingProperties<Impl: IAudioStreamDescriptorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EncodingProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAudioStreamDescriptor>, base.5, EncodingProperties::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioStreamDescriptor2Impl: Sized + IMediaStreamDescriptorImpl {
    fn SetLeadingEncoderPadding(&self, value: &::core::option::Option<super::super::Foundation::IReference<u32>>) -> ::windows::core::Result<()>;
    fn LeadingEncoderPadding(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>>;
    fn SetTrailingEncoderPadding(&self, value: &::core::option::Option<super::super::Foundation::IReference<u32>>) -> ::windows::core::Result<()>;
    fn TrailingEncoderPadding(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAudioStreamDescriptor2 {
    const NAME: &'static str = "Windows.Media.Core.IAudioStreamDescriptor2";
}
#[cfg(feature = "implement_exclusive")]
impl IAudioStreamDescriptor2Vtbl {
    pub const fn new<Impl: IAudioStreamDescriptor2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAudioStreamDescriptor2Vtbl {
        unsafe extern "system" fn SetLeadingEncoderPadding<Impl: IAudioStreamDescriptor2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetLeadingEncoderPadding(&*(&value as *const <super::super::Foundation::IReference<u32> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<u32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn LeadingEncoderPadding<Impl: IAudioStreamDescriptor2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LeadingEncoderPadding() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTrailingEncoderPadding<Impl: IAudioStreamDescriptor2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTrailingEncoderPadding(&*(&value as *const <super::super::Foundation::IReference<u32> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<u32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TrailingEncoderPadding<Impl: IAudioStreamDescriptor2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TrailingEncoderPadding() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAudioStreamDescriptor2>, base.5, SetLeadingEncoderPadding::<Impl, OFFSET>, LeadingEncoderPadding::<Impl, OFFSET>, SetTrailingEncoderPadding::<Impl, OFFSET>, TrailingEncoderPadding::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioStreamDescriptor3Impl: Sized {
    fn Copy(&self) -> ::windows::core::Result<AudioStreamDescriptor>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAudioStreamDescriptor3 {
    const NAME: &'static str = "Windows.Media.Core.IAudioStreamDescriptor3";
}
#[cfg(feature = "implement_exclusive")]
impl IAudioStreamDescriptor3Vtbl {
    pub const fn new<Impl: IAudioStreamDescriptor3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAudioStreamDescriptor3Vtbl {
        unsafe extern "system" fn Copy<Impl: IAudioStreamDescriptor3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Copy() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAudioStreamDescriptor3>, base.5, Copy::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioStreamDescriptorFactoryImpl: Sized {
    fn Create(&self, encodingproperties: &::core::option::Option<super::MediaProperties::AudioEncodingProperties>) -> ::windows::core::Result<AudioStreamDescriptor>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAudioStreamDescriptorFactory {
    const NAME: &'static str = "Windows.Media.Core.IAudioStreamDescriptorFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IAudioStreamDescriptorFactoryVtbl {
    pub const fn new<Impl: IAudioStreamDescriptorFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAudioStreamDescriptorFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IAudioStreamDescriptorFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, encodingproperties: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(&*(&encodingproperties as *const <super::MediaProperties::AudioEncodingProperties as ::windows::core::Abi>::Abi as *const <super::MediaProperties::AudioEncodingProperties as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAudioStreamDescriptorFactory>, base.5, Create::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioTrackImpl: Sized {
    fn OpenFailed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AudioTrack, AudioTrackOpenFailedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveOpenFailed(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GetEncodingProperties(&self) -> ::windows::core::Result<super::MediaProperties::AudioEncodingProperties>;
    fn PlaybackItem(&self) -> ::windows::core::Result<super::Playback::MediaPlaybackItem>;
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SupportInfo(&self) -> ::windows::core::Result<AudioTrackSupportInfo>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAudioTrack {
    const NAME: &'static str = "Windows.Media.Core.IAudioTrack";
}
#[cfg(feature = "implement_exclusive")]
impl IAudioTrackVtbl {
    pub const fn new<Impl: IAudioTrackImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAudioTrackVtbl {
        unsafe extern "system" fn OpenFailed<Impl: IAudioTrackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OpenFailed(&*(&handler as *const <super::super::Foundation::TypedEventHandler<AudioTrack, AudioTrackOpenFailedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AudioTrack, AudioTrackOpenFailedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveOpenFailed<Impl: IAudioTrackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveOpenFailed(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetEncodingProperties<Impl: IAudioTrackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetEncodingProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PlaybackItem<Impl: IAudioTrackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PlaybackItem() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: IAudioTrackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportInfo<Impl: IAudioTrackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SupportInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAudioTrack>, base.5, OpenFailed::<Impl, OFFSET>, RemoveOpenFailed::<Impl, OFFSET>, GetEncodingProperties::<Impl, OFFSET>, PlaybackItem::<Impl, OFFSET>, Name::<Impl, OFFSET>, SupportInfo::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioTrackOpenFailedEventArgsImpl: Sized {
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAudioTrackOpenFailedEventArgs {
    const NAME: &'static str = "Windows.Media.Core.IAudioTrackOpenFailedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IAudioTrackOpenFailedEventArgsVtbl {
    pub const fn new<Impl: IAudioTrackOpenFailedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAudioTrackOpenFailedEventArgsVtbl {
        unsafe extern "system" fn ExtendedError<Impl: IAudioTrackOpenFailedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExtendedError() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAudioTrackOpenFailedEventArgs>, base.5, ExtendedError::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioTrackSupportInfoImpl: Sized {
    fn DecoderStatus(&self) -> ::windows::core::Result<MediaDecoderStatus>;
    fn Degradation(&self) -> ::windows::core::Result<AudioDecoderDegradation>;
    fn DegradationReason(&self) -> ::windows::core::Result<AudioDecoderDegradationReason>;
    fn MediaSourceStatus(&self) -> ::windows::core::Result<MediaSourceStatus>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAudioTrackSupportInfo {
    const NAME: &'static str = "Windows.Media.Core.IAudioTrackSupportInfo";
}
#[cfg(feature = "implement_exclusive")]
impl IAudioTrackSupportInfoVtbl {
    pub const fn new<Impl: IAudioTrackSupportInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAudioTrackSupportInfoVtbl {
        unsafe extern "system" fn DecoderStatus<Impl: IAudioTrackSupportInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut MediaDecoderStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DecoderStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Degradation<Impl: IAudioTrackSupportInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut AudioDecoderDegradation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Degradation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DegradationReason<Impl: IAudioTrackSupportInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut AudioDecoderDegradationReason) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DegradationReason() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediaSourceStatus<Impl: IAudioTrackSupportInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut MediaSourceStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MediaSourceStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAudioTrackSupportInfo>, base.5, DecoderStatus::<Impl, OFFSET>, Degradation::<Impl, OFFSET>, DegradationReason::<Impl, OFFSET>, MediaSourceStatus::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IChapterCueImpl: Sized + IMediaCueImpl {
    fn SetTitle(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IChapterCue {
    const NAME: &'static str = "Windows.Media.Core.IChapterCue";
}
#[cfg(feature = "implement_exclusive")]
impl IChapterCueVtbl {
    pub const fn new<Impl: IChapterCueImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IChapterCueVtbl {
        unsafe extern "system" fn SetTitle<Impl: IChapterCueImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTitle(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Title<Impl: IChapterCueImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Title() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IChapterCue>, base.5, SetTitle::<Impl, OFFSET>, Title::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICodecInfoImpl: Sized {
    fn Kind(&self) -> ::windows::core::Result<CodecKind>;
    fn Category(&self) -> ::windows::core::Result<CodecCategory>;
    fn Subtypes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsTrusted(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICodecInfo {
    const NAME: &'static str = "Windows.Media.Core.ICodecInfo";
}
#[cfg(feature = "implement_exclusive")]
impl ICodecInfoVtbl {
    pub const fn new<Impl: ICodecInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICodecInfoVtbl {
        unsafe extern "system" fn Kind<Impl: ICodecInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut CodecKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Kind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Category<Impl: ICodecInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut CodecCategory) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Category() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Subtypes<Impl: ICodecInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Subtypes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayName<Impl: ICodecInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsTrusted<Impl: ICodecInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsTrusted() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICodecInfo>, base.5, Kind::<Impl, OFFSET>, Category::<Impl, OFFSET>, Subtypes::<Impl, OFFSET>, DisplayName::<Impl, OFFSET>, IsTrusted::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICodecQueryImpl: Sized {
    fn FindAllAsync(&self, kind: CodecKind, category: CodecCategory, subtype: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<CodecInfo>>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICodecQuery {
    const NAME: &'static str = "Windows.Media.Core.ICodecQuery";
}
#[cfg(feature = "implement_exclusive")]
impl ICodecQueryVtbl {
    pub const fn new<Impl: ICodecQueryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICodecQueryVtbl {
        unsafe extern "system" fn FindAllAsync<Impl: ICodecQueryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, kind: CodecKind, category: CodecCategory, subtype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FindAllAsync(kind, category, &*(&subtype as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICodecQuery>, base.5, FindAllAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICodecSubtypesStaticsImpl: Sized {
    fn VideoFormatDV25(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoFormatDV50(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoFormatDvc(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoFormatDvh1(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoFormatDvhD(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoFormatDvsd(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoFormatDvsl(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoFormatH263(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoFormatH264(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoFormatH265(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoFormatH264ES(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoFormatHevc(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoFormatHevcES(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoFormatM4S2(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoFormatMjpg(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoFormatMP43(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoFormatMP4S(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoFormatMP4V(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoFormatMpeg2(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoFormatVP80(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoFormatVP90(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoFormatMpg1(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoFormatMss1(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoFormatMss2(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoFormatWmv1(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoFormatWmv2(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoFormatWmv3(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoFormatWvc1(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoFormat420O(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AudioFormatAac(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AudioFormatAdts(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AudioFormatAlac(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AudioFormatAmrNB(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AudioFormatAmrWB(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AudioFormatAmrWP(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AudioFormatDolbyAC3(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AudioFormatDolbyAC3Spdif(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AudioFormatDolbyDDPlus(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AudioFormatDrm(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AudioFormatDts(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AudioFormatFlac(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AudioFormatFloat(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AudioFormatMP3(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AudioFormatMPeg(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AudioFormatMsp1(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AudioFormatOpus(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AudioFormatPcm(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AudioFormatWmaSpdif(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AudioFormatWMAudioLossless(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AudioFormatWMAudioV8(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AudioFormatWMAudioV9(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICodecSubtypesStatics {
    const NAME: &'static str = "Windows.Media.Core.ICodecSubtypesStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ICodecSubtypesStaticsVtbl {
    pub const fn new<Impl: ICodecSubtypesStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICodecSubtypesStaticsVtbl {
        unsafe extern "system" fn VideoFormatDV25<Impl: ICodecSubtypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VideoFormatDV25() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoFormatDV50<Impl: ICodecSubtypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VideoFormatDV50() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoFormatDvc<Impl: ICodecSubtypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VideoFormatDvc() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoFormatDvh1<Impl: ICodecSubtypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VideoFormatDvh1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoFormatDvhD<Impl: ICodecSubtypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VideoFormatDvhD() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoFormatDvsd<Impl: ICodecSubtypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VideoFormatDvsd() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoFormatDvsl<Impl: ICodecSubtypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VideoFormatDvsl() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoFormatH263<Impl: ICodecSubtypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VideoFormatH263() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoFormatH264<Impl: ICodecSubtypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VideoFormatH264() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoFormatH265<Impl: ICodecSubtypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VideoFormatH265() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoFormatH264ES<Impl: ICodecSubtypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VideoFormatH264ES() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoFormatHevc<Impl: ICodecSubtypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VideoFormatHevc() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoFormatHevcES<Impl: ICodecSubtypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VideoFormatHevcES() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoFormatM4S2<Impl: ICodecSubtypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VideoFormatM4S2() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoFormatMjpg<Impl: ICodecSubtypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VideoFormatMjpg() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoFormatMP43<Impl: ICodecSubtypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VideoFormatMP43() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoFormatMP4S<Impl: ICodecSubtypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VideoFormatMP4S() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoFormatMP4V<Impl: ICodecSubtypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VideoFormatMP4V() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoFormatMpeg2<Impl: ICodecSubtypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VideoFormatMpeg2() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoFormatVP80<Impl: ICodecSubtypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VideoFormatVP80() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoFormatVP90<Impl: ICodecSubtypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VideoFormatVP90() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoFormatMpg1<Impl: ICodecSubtypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VideoFormatMpg1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoFormatMss1<Impl: ICodecSubtypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VideoFormatMss1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoFormatMss2<Impl: ICodecSubtypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VideoFormatMss2() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoFormatWmv1<Impl: ICodecSubtypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VideoFormatWmv1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoFormatWmv2<Impl: ICodecSubtypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VideoFormatWmv2() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoFormatWmv3<Impl: ICodecSubtypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VideoFormatWmv3() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoFormatWvc1<Impl: ICodecSubtypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VideoFormatWvc1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoFormat420O<Impl: ICodecSubtypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VideoFormat420O() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioFormatAac<Impl: ICodecSubtypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AudioFormatAac() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioFormatAdts<Impl: ICodecSubtypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AudioFormatAdts() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioFormatAlac<Impl: ICodecSubtypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AudioFormatAlac() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioFormatAmrNB<Impl: ICodecSubtypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AudioFormatAmrNB() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioFormatAmrWB<Impl: ICodecSubtypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AudioFormatAmrWB() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioFormatAmrWP<Impl: ICodecSubtypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AudioFormatAmrWP() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioFormatDolbyAC3<Impl: ICodecSubtypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AudioFormatDolbyAC3() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioFormatDolbyAC3Spdif<Impl: ICodecSubtypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AudioFormatDolbyAC3Spdif() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioFormatDolbyDDPlus<Impl: ICodecSubtypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AudioFormatDolbyDDPlus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioFormatDrm<Impl: ICodecSubtypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AudioFormatDrm() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioFormatDts<Impl: ICodecSubtypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AudioFormatDts() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioFormatFlac<Impl: ICodecSubtypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AudioFormatFlac() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioFormatFloat<Impl: ICodecSubtypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AudioFormatFloat() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioFormatMP3<Impl: ICodecSubtypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AudioFormatMP3() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioFormatMPeg<Impl: ICodecSubtypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AudioFormatMPeg() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioFormatMsp1<Impl: ICodecSubtypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AudioFormatMsp1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioFormatOpus<Impl: ICodecSubtypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AudioFormatOpus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioFormatPcm<Impl: ICodecSubtypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AudioFormatPcm() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioFormatWmaSpdif<Impl: ICodecSubtypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AudioFormatWmaSpdif() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioFormatWMAudioLossless<Impl: ICodecSubtypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AudioFormatWMAudioLossless() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioFormatWMAudioV8<Impl: ICodecSubtypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AudioFormatWMAudioV8() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioFormatWMAudioV9<Impl: ICodecSubtypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AudioFormatWMAudioV9() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<ICodecSubtypesStatics>,
            base.5,
            VideoFormatDV25::<Impl, OFFSET>,
            VideoFormatDV50::<Impl, OFFSET>,
            VideoFormatDvc::<Impl, OFFSET>,
            VideoFormatDvh1::<Impl, OFFSET>,
            VideoFormatDvhD::<Impl, OFFSET>,
            VideoFormatDvsd::<Impl, OFFSET>,
            VideoFormatDvsl::<Impl, OFFSET>,
            VideoFormatH263::<Impl, OFFSET>,
            VideoFormatH264::<Impl, OFFSET>,
            VideoFormatH265::<Impl, OFFSET>,
            VideoFormatH264ES::<Impl, OFFSET>,
            VideoFormatHevc::<Impl, OFFSET>,
            VideoFormatHevcES::<Impl, OFFSET>,
            VideoFormatM4S2::<Impl, OFFSET>,
            VideoFormatMjpg::<Impl, OFFSET>,
            VideoFormatMP43::<Impl, OFFSET>,
            VideoFormatMP4S::<Impl, OFFSET>,
            VideoFormatMP4V::<Impl, OFFSET>,
            VideoFormatMpeg2::<Impl, OFFSET>,
            VideoFormatVP80::<Impl, OFFSET>,
            VideoFormatVP90::<Impl, OFFSET>,
            VideoFormatMpg1::<Impl, OFFSET>,
            VideoFormatMss1::<Impl, OFFSET>,
            VideoFormatMss2::<Impl, OFFSET>,
            VideoFormatWmv1::<Impl, OFFSET>,
            VideoFormatWmv2::<Impl, OFFSET>,
            VideoFormatWmv3::<Impl, OFFSET>,
            VideoFormatWvc1::<Impl, OFFSET>,
            VideoFormat420O::<Impl, OFFSET>,
            AudioFormatAac::<Impl, OFFSET>,
            AudioFormatAdts::<Impl, OFFSET>,
            AudioFormatAlac::<Impl, OFFSET>,
            AudioFormatAmrNB::<Impl, OFFSET>,
            AudioFormatAmrWB::<Impl, OFFSET>,
            AudioFormatAmrWP::<Impl, OFFSET>,
            AudioFormatDolbyAC3::<Impl, OFFSET>,
            AudioFormatDolbyAC3Spdif::<Impl, OFFSET>,
            AudioFormatDolbyDDPlus::<Impl, OFFSET>,
            AudioFormatDrm::<Impl, OFFSET>,
            AudioFormatDts::<Impl, OFFSET>,
            AudioFormatFlac::<Impl, OFFSET>,
            AudioFormatFloat::<Impl, OFFSET>,
            AudioFormatMP3::<Impl, OFFSET>,
            AudioFormatMPeg::<Impl, OFFSET>,
            AudioFormatMsp1::<Impl, OFFSET>,
            AudioFormatOpus::<Impl, OFFSET>,
            AudioFormatPcm::<Impl, OFFSET>,
            AudioFormatWmaSpdif::<Impl, OFFSET>,
            AudioFormatWMAudioLossless::<Impl, OFFSET>,
            AudioFormatWMAudioV8::<Impl, OFFSET>,
            AudioFormatWMAudioV9::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataCueImpl: Sized + IMediaCueImpl {
    fn SetData(&self, value: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
    fn Data(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDataCue {
    const NAME: &'static str = "Windows.Media.Core.IDataCue";
}
#[cfg(feature = "implement_exclusive")]
impl IDataCueVtbl {
    pub const fn new<Impl: IDataCueImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDataCueVtbl {
        unsafe extern "system" fn SetData<Impl: IDataCueImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetData(&*(&value as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Data<Impl: IDataCueImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Data() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDataCue>, base.5, SetData::<Impl, OFFSET>, Data::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataCue2Impl: Sized + IDataCueImpl + IMediaCueImpl {
    fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::PropertySet>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDataCue2 {
    const NAME: &'static str = "Windows.Media.Core.IDataCue2";
}
#[cfg(feature = "implement_exclusive")]
impl IDataCue2Vtbl {
    pub const fn new<Impl: IDataCue2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDataCue2Vtbl {
        unsafe extern "system" fn Properties<Impl: IDataCue2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDataCue2>, base.5, Properties::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFaceDetectedEventArgsImpl: Sized {
    fn ResultFrame(&self) -> ::windows::core::Result<FaceDetectionEffectFrame>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFaceDetectedEventArgs {
    const NAME: &'static str = "Windows.Media.Core.IFaceDetectedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IFaceDetectedEventArgsVtbl {
    pub const fn new<Impl: IFaceDetectedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFaceDetectedEventArgsVtbl {
        unsafe extern "system" fn ResultFrame<Impl: IFaceDetectedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ResultFrame() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFaceDetectedEventArgs>, base.5, ResultFrame::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFaceDetectionEffectImpl: Sized + IMediaExtensionImpl {
    fn SetEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn Enabled(&self) -> ::windows::core::Result<bool>;
    fn SetDesiredDetectionInterval(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn DesiredDetectionInterval(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn FaceDetected(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<FaceDetectionEffect, FaceDetectedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveFaceDetected(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFaceDetectionEffect {
    const NAME: &'static str = "Windows.Media.Core.IFaceDetectionEffect";
}
#[cfg(feature = "implement_exclusive")]
impl IFaceDetectionEffectVtbl {
    pub const fn new<Impl: IFaceDetectionEffectImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFaceDetectionEffectVtbl {
        unsafe extern "system" fn SetEnabled<Impl: IFaceDetectionEffectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetEnabled(value).into()
        }
        unsafe extern "system" fn Enabled<Impl: IFaceDetectionEffectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Enabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredDetectionInterval<Impl: IFaceDetectionEffectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDesiredDetectionInterval(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DesiredDetectionInterval<Impl: IFaceDetectionEffectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DesiredDetectionInterval() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FaceDetected<Impl: IFaceDetectionEffectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FaceDetected(&*(&handler as *const <super::super::Foundation::TypedEventHandler<FaceDetectionEffect, FaceDetectedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<FaceDetectionEffect, FaceDetectedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveFaceDetected<Impl: IFaceDetectionEffectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveFaceDetected(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFaceDetectionEffect>, base.5, SetEnabled::<Impl, OFFSET>, Enabled::<Impl, OFFSET>, SetDesiredDetectionInterval::<Impl, OFFSET>, DesiredDetectionInterval::<Impl, OFFSET>, FaceDetected::<Impl, OFFSET>, RemoveFaceDetected::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "Media_Effects", feature = "implement_exclusive"))]
pub trait IFaceDetectionEffectDefinitionImpl: Sized + IVideoEffectDefinitionImpl {
    fn SetDetectionMode(&self, value: FaceDetectionMode) -> ::windows::core::Result<()>;
    fn DetectionMode(&self) -> ::windows::core::Result<FaceDetectionMode>;
    fn SetSynchronousDetectionEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn SynchronousDetectionEnabled(&self) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Media_Effects", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IFaceDetectionEffectDefinition {
    const NAME: &'static str = "Windows.Media.Core.IFaceDetectionEffectDefinition";
}
#[cfg(all(feature = "Media_Effects", feature = "implement_exclusive"))]
impl IFaceDetectionEffectDefinitionVtbl {
    pub const fn new<Impl: IFaceDetectionEffectDefinitionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFaceDetectionEffectDefinitionVtbl {
        unsafe extern "system" fn SetDetectionMode<Impl: IFaceDetectionEffectDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: FaceDetectionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDetectionMode(value).into()
        }
        unsafe extern "system" fn DetectionMode<Impl: IFaceDetectionEffectDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut FaceDetectionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DetectionMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSynchronousDetectionEnabled<Impl: IFaceDetectionEffectDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSynchronousDetectionEnabled(value).into()
        }
        unsafe extern "system" fn SynchronousDetectionEnabled<Impl: IFaceDetectionEffectDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SynchronousDetectionEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFaceDetectionEffectDefinition>, base.5, SetDetectionMode::<Impl, OFFSET>, DetectionMode::<Impl, OFFSET>, SetSynchronousDetectionEnabled::<Impl, OFFSET>, SynchronousDetectionEnabled::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IFaceDetectionEffectFrameImpl: Sized + IClosableImpl + IMediaFrameImpl {
    fn DetectedFaces(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::FaceAnalysis::DetectedFace>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IFaceDetectionEffectFrame {
    const NAME: &'static str = "Windows.Media.Core.IFaceDetectionEffectFrame";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IFaceDetectionEffectFrameVtbl {
    pub const fn new<Impl: IFaceDetectionEffectFrameImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFaceDetectionEffectFrameVtbl {
        unsafe extern "system" fn DetectedFaces<Impl: IFaceDetectionEffectFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DetectedFaces() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFaceDetectionEffectFrame>, base.5, DetectedFaces::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHighDynamicRangeControlImpl: Sized {
    fn SetEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn Enabled(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHighDynamicRangeControl {
    const NAME: &'static str = "Windows.Media.Core.IHighDynamicRangeControl";
}
#[cfg(feature = "implement_exclusive")]
impl IHighDynamicRangeControlVtbl {
    pub const fn new<Impl: IHighDynamicRangeControlImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IHighDynamicRangeControlVtbl {
        unsafe extern "system" fn SetEnabled<Impl: IHighDynamicRangeControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetEnabled(value).into()
        }
        unsafe extern "system" fn Enabled<Impl: IHighDynamicRangeControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Enabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IHighDynamicRangeControl>, base.5, SetEnabled::<Impl, OFFSET>, Enabled::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHighDynamicRangeOutputImpl: Sized {
    fn Certainty(&self) -> ::windows::core::Result<f64>;
    fn FrameControllers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::Devices::Core::FrameController>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHighDynamicRangeOutput {
    const NAME: &'static str = "Windows.Media.Core.IHighDynamicRangeOutput";
}
#[cfg(feature = "implement_exclusive")]
impl IHighDynamicRangeOutputVtbl {
    pub const fn new<Impl: IHighDynamicRangeOutputImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IHighDynamicRangeOutputVtbl {
        unsafe extern "system" fn Certainty<Impl: IHighDynamicRangeOutputImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Certainty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FrameControllers<Impl: IHighDynamicRangeOutputImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FrameControllers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IHighDynamicRangeOutput>, base.5, Certainty::<Impl, OFFSET>, FrameControllers::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IImageCueImpl: Sized + IMediaCueImpl {
    fn Position(&self) -> ::windows::core::Result<TimedTextPoint>;
    fn SetPosition(&self, value: &TimedTextPoint) -> ::windows::core::Result<()>;
    fn Extent(&self) -> ::windows::core::Result<TimedTextSize>;
    fn SetExtent(&self, value: &TimedTextSize) -> ::windows::core::Result<()>;
    fn SetSoftwareBitmap(&self, value: &::core::option::Option<super::super::Graphics::Imaging::SoftwareBitmap>) -> ::windows::core::Result<()>;
    fn SoftwareBitmap(&self) -> ::windows::core::Result<super::super::Graphics::Imaging::SoftwareBitmap>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IImageCue {
    const NAME: &'static str = "Windows.Media.Core.IImageCue";
}
#[cfg(feature = "implement_exclusive")]
impl IImageCueVtbl {
    pub const fn new<Impl: IImageCueImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IImageCueVtbl {
        unsafe extern "system" fn Position<Impl: IImageCueImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut TimedTextPoint) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Position() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPosition<Impl: IImageCueImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: TimedTextPoint) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPosition(&*(&value as *const <TimedTextPoint as ::windows::core::Abi>::Abi as *const <TimedTextPoint as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Extent<Impl: IImageCueImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut TimedTextSize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Extent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExtent<Impl: IImageCueImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: TimedTextSize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetExtent(&*(&value as *const <TimedTextSize as ::windows::core::Abi>::Abi as *const <TimedTextSize as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetSoftwareBitmap<Impl: IImageCueImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSoftwareBitmap(&*(&value as *const <super::super::Graphics::Imaging::SoftwareBitmap as ::windows::core::Abi>::Abi as *const <super::super::Graphics::Imaging::SoftwareBitmap as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SoftwareBitmap<Impl: IImageCueImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IImageCue>, base.5, Position::<Impl, OFFSET>, SetPosition::<Impl, OFFSET>, Extent::<Impl, OFFSET>, SetExtent::<Impl, OFFSET>, SetSoftwareBitmap::<Impl, OFFSET>, SoftwareBitmap::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInitializeMediaStreamSourceRequestedEventArgsImpl: Sized {
    fn Source(&self) -> ::windows::core::Result<MediaStreamSource>;
    fn RandomAccessStream(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStream>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInitializeMediaStreamSourceRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.Core.IInitializeMediaStreamSourceRequestedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IInitializeMediaStreamSourceRequestedEventArgsVtbl {
    pub const fn new<Impl: IInitializeMediaStreamSourceRequestedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IInitializeMediaStreamSourceRequestedEventArgsVtbl {
        unsafe extern "system" fn Source<Impl: IInitializeMediaStreamSourceRequestedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Source() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RandomAccessStream<Impl: IInitializeMediaStreamSourceRequestedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RandomAccessStream() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeferral<Impl: IInitializeMediaStreamSourceRequestedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IInitializeMediaStreamSourceRequestedEventArgs>, base.5, Source::<Impl, OFFSET>, RandomAccessStream::<Impl, OFFSET>, GetDeferral::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILowLightFusionResultImpl: Sized {
    fn Frame(&self) -> ::windows::core::Result<super::super::Graphics::Imaging::SoftwareBitmap>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILowLightFusionResult {
    const NAME: &'static str = "Windows.Media.Core.ILowLightFusionResult";
}
#[cfg(feature = "implement_exclusive")]
impl ILowLightFusionResultVtbl {
    pub const fn new<Impl: ILowLightFusionResultImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ILowLightFusionResultVtbl {
        unsafe extern "system" fn Frame<Impl: ILowLightFusionResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Frame() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ILowLightFusionResult>, base.5, Frame::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILowLightFusionStaticsImpl: Sized {
    fn SupportedBitmapPixelFormats(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Graphics::Imaging::BitmapPixelFormat>>;
    fn MaxSupportedFrameCount(&self) -> ::windows::core::Result<i32>;
    fn FuseAsync(&self, frameset: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Graphics::Imaging::SoftwareBitmap>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<LowLightFusionResult, f64>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILowLightFusionStatics {
    const NAME: &'static str = "Windows.Media.Core.ILowLightFusionStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ILowLightFusionStaticsVtbl {
    pub const fn new<Impl: ILowLightFusionStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ILowLightFusionStaticsVtbl {
        unsafe extern "system" fn SupportedBitmapPixelFormats<Impl: ILowLightFusionStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SupportedBitmapPixelFormats() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxSupportedFrameCount<Impl: ILowLightFusionStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MaxSupportedFrameCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FuseAsync<Impl: ILowLightFusionStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, frameset: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FuseAsync(&*(&frameset as *const <super::super::Foundation::Collections::IIterable<super::super::Graphics::Imaging::SoftwareBitmap> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<super::super::Graphics::Imaging::SoftwareBitmap> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ILowLightFusionStatics>, base.5, SupportedBitmapPixelFormats::<Impl, OFFSET>, MaxSupportedFrameCount::<Impl, OFFSET>, FuseAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaBinderImpl: Sized {
    fn Binding(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaBinder, MediaBindingEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveBinding(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Token(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetToken(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Source(&self) -> ::windows::core::Result<MediaSource>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaBinder {
    const NAME: &'static str = "Windows.Media.Core.IMediaBinder";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaBinderVtbl {
    pub const fn new<Impl: IMediaBinderImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaBinderVtbl {
        unsafe extern "system" fn Binding<Impl: IMediaBinderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Binding(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MediaBinder, MediaBindingEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MediaBinder, MediaBindingEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveBinding<Impl: IMediaBinderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveBinding(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Token<Impl: IMediaBinderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Token() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetToken<Impl: IMediaBinderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetToken(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Source<Impl: IMediaBinderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Source() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaBinder>, base.5, Binding::<Impl, OFFSET>, RemoveBinding::<Impl, OFFSET>, Token::<Impl, OFFSET>, SetToken::<Impl, OFFSET>, Source::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaBindingEventArgsImpl: Sized {
    fn Canceled(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaBindingEventArgs, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCanceled(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn MediaBinder(&self) -> ::windows::core::Result<MediaBinder>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
    fn SetUri(&self, uri: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn SetStream(&self, stream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>, contenttype: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SetStreamReference(&self, stream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStreamReference>, contenttype: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaBindingEventArgs {
    const NAME: &'static str = "Windows.Media.Core.IMediaBindingEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaBindingEventArgsVtbl {
    pub const fn new<Impl: IMediaBindingEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaBindingEventArgsVtbl {
        unsafe extern "system" fn Canceled<Impl: IMediaBindingEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Canceled(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MediaBindingEventArgs, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MediaBindingEventArgs, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCanceled<Impl: IMediaBindingEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveCanceled(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MediaBinder<Impl: IMediaBindingEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MediaBinder() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeferral<Impl: IMediaBindingEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUri<Impl: IMediaBindingEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetUri(&*(&uri as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetStream<Impl: IMediaBindingEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, contenttype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetStream(&*(&stream as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::DefaultType>::DefaultType), &*(&contenttype as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetStreamReference<Impl: IMediaBindingEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, contenttype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetStreamReference(&*(&stream as *const <super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::DefaultType>::DefaultType), &*(&contenttype as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaBindingEventArgs>, base.5, Canceled::<Impl, OFFSET>, RemoveCanceled::<Impl, OFFSET>, MediaBinder::<Impl, OFFSET>, GetDeferral::<Impl, OFFSET>, SetUri::<Impl, OFFSET>, SetStream::<Impl, OFFSET>, SetStreamReference::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaBindingEventArgs2Impl: Sized {
    fn SetAdaptiveMediaSource(&self, mediasource: &::core::option::Option<super::Streaming::Adaptive::AdaptiveMediaSource>) -> ::windows::core::Result<()>;
    fn SetStorageFile(&self, file: &::core::option::Option<super::super::Storage::IStorageFile>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaBindingEventArgs2 {
    const NAME: &'static str = "Windows.Media.Core.IMediaBindingEventArgs2";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaBindingEventArgs2Vtbl {
    pub const fn new<Impl: IMediaBindingEventArgs2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaBindingEventArgs2Vtbl {
        unsafe extern "system" fn SetAdaptiveMediaSource<Impl: IMediaBindingEventArgs2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mediasource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAdaptiveMediaSource(&*(&mediasource as *const <super::Streaming::Adaptive::AdaptiveMediaSource as ::windows::core::Abi>::Abi as *const <super::Streaming::Adaptive::AdaptiveMediaSource as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetStorageFile<Impl: IMediaBindingEventArgs2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetStorageFile(&*(&file as *const <super::super::Storage::IStorageFile as ::windows::core::Abi>::Abi as *const <super::super::Storage::IStorageFile as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaBindingEventArgs2>, base.5, SetAdaptiveMediaSource::<Impl, OFFSET>, SetStorageFile::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaBindingEventArgs3Impl: Sized {
    fn SetDownloadOperation(&self, downloadoperation: &::core::option::Option<super::super::Networking::BackgroundTransfer::DownloadOperation>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaBindingEventArgs3 {
    const NAME: &'static str = "Windows.Media.Core.IMediaBindingEventArgs3";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaBindingEventArgs3Vtbl {
    pub const fn new<Impl: IMediaBindingEventArgs3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaBindingEventArgs3Vtbl {
        unsafe extern "system" fn SetDownloadOperation<Impl: IMediaBindingEventArgs3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, downloadoperation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDownloadOperation(&*(&downloadoperation as *const <super::super::Networking::BackgroundTransfer::DownloadOperation as ::windows::core::Abi>::Abi as *const <super::super::Networking::BackgroundTransfer::DownloadOperation as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaBindingEventArgs3>, base.5, SetDownloadOperation::<Impl, OFFSET>)
    }
}
pub trait IMediaCueImpl: Sized {
    fn SetStartTime(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn StartTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetDuration(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for IMediaCue {
    const NAME: &'static str = "Windows.Media.Core.IMediaCue";
}
impl IMediaCueVtbl {
    pub const fn new<Impl: IMediaCueImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaCueVtbl {
        unsafe extern "system" fn SetStartTime<Impl: IMediaCueImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetStartTime(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StartTime<Impl: IMediaCueImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StartTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDuration<Impl: IMediaCueImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDuration(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Duration<Impl: IMediaCueImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetId<Impl: IMediaCueImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetId(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Id<Impl: IMediaCueImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaCue>, base.5, SetStartTime::<Impl, OFFSET>, StartTime::<Impl, OFFSET>, SetDuration::<Impl, OFFSET>, Duration::<Impl, OFFSET>, SetId::<Impl, OFFSET>, Id::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaCueEventArgsImpl: Sized {
    fn Cue(&self) -> ::windows::core::Result<IMediaCue>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaCueEventArgs {
    const NAME: &'static str = "Windows.Media.Core.IMediaCueEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaCueEventArgsVtbl {
    pub const fn new<Impl: IMediaCueEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaCueEventArgsVtbl {
        unsafe extern "system" fn Cue<Impl: IMediaCueEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Cue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaCueEventArgs>, base.5, Cue::<Impl, OFFSET>)
    }
}
pub trait IMediaSourceImpl: Sized {}
impl ::windows::core::RuntimeName for IMediaSource {
    const NAME: &'static str = "Windows.Media.Core.IMediaSource";
}
impl IMediaSourceVtbl {
    pub const fn new<Impl: IMediaSourceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaSourceVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaSource>, base.5)
    }
}
#[cfg(all(feature = "Foundation", feature = "Media_Playback", feature = "implement_exclusive"))]
pub trait IMediaSource2Impl: Sized + IClosableImpl + IMediaPlaybackSourceImpl {
    fn OpenOperationCompleted(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaSource, MediaSourceOpenOperationCompletedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveOpenOperationCompleted(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CustomProperties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet>;
    fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>;
    fn IsOpen(&self) -> ::windows::core::Result<bool>;
    fn ExternalTimedTextSources(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IObservableVector<TimedTextSource>>;
    fn ExternalTimedMetadataTracks(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IObservableVector<TimedMetadataTrack>>;
}
#[cfg(all(feature = "Foundation", feature = "Media_Playback", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaSource2 {
    const NAME: &'static str = "Windows.Media.Core.IMediaSource2";
}
#[cfg(all(feature = "Foundation", feature = "Media_Playback", feature = "implement_exclusive"))]
impl IMediaSource2Vtbl {
    pub const fn new<Impl: IMediaSource2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaSource2Vtbl {
        unsafe extern "system" fn OpenOperationCompleted<Impl: IMediaSource2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OpenOperationCompleted(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MediaSource, MediaSourceOpenOperationCompletedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MediaSource, MediaSourceOpenOperationCompletedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveOpenOperationCompleted<Impl: IMediaSource2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveOpenOperationCompleted(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CustomProperties<Impl: IMediaSource2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CustomProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Duration<Impl: IMediaSource2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsOpen<Impl: IMediaSource2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsOpen() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExternalTimedTextSources<Impl: IMediaSource2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExternalTimedTextSources() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExternalTimedMetadataTracks<Impl: IMediaSource2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExternalTimedMetadataTracks() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaSource2>, base.5, OpenOperationCompleted::<Impl, OFFSET>, RemoveOpenOperationCompleted::<Impl, OFFSET>, CustomProperties::<Impl, OFFSET>, Duration::<Impl, OFFSET>, IsOpen::<Impl, OFFSET>, ExternalTimedTextSources::<Impl, OFFSET>, ExternalTimedMetadataTracks::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "Foundation", feature = "Media_Playback", feature = "implement_exclusive"))]
pub trait IMediaSource3Impl: Sized + IClosableImpl + IMediaPlaybackSourceImpl + IMediaSource2Impl {
    fn StateChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaSource, MediaSourceStateChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStateChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn State(&self) -> ::windows::core::Result<MediaSourceState>;
    fn Reset(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Media_Playback", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaSource3 {
    const NAME: &'static str = "Windows.Media.Core.IMediaSource3";
}
#[cfg(all(feature = "Foundation", feature = "Media_Playback", feature = "implement_exclusive"))]
impl IMediaSource3Vtbl {
    pub const fn new<Impl: IMediaSource3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaSource3Vtbl {
        unsafe extern "system" fn StateChanged<Impl: IMediaSource3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StateChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MediaSource, MediaSourceStateChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MediaSource, MediaSourceStateChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStateChanged<Impl: IMediaSource3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveStateChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn State<Impl: IMediaSource3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut MediaSourceState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IMediaSource3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaSource3>, base.5, StateChanged::<Impl, OFFSET>, RemoveStateChanged::<Impl, OFFSET>, State::<Impl, OFFSET>, Reset::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "Foundation", feature = "Media_Playback", feature = "implement_exclusive"))]
pub trait IMediaSource4Impl: Sized + IClosableImpl + IMediaPlaybackSourceImpl + IMediaSource2Impl + IMediaSource3Impl {
    fn AdaptiveMediaSource(&self) -> ::windows::core::Result<super::Streaming::Adaptive::AdaptiveMediaSource>;
    fn MediaStreamSource(&self) -> ::windows::core::Result<MediaStreamSource>;
    fn MseStreamSource(&self) -> ::windows::core::Result<MseStreamSource>;
    fn Uri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn OpenAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "Media_Playback", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaSource4 {
    const NAME: &'static str = "Windows.Media.Core.IMediaSource4";
}
#[cfg(all(feature = "Foundation", feature = "Media_Playback", feature = "implement_exclusive"))]
impl IMediaSource4Vtbl {
    pub const fn new<Impl: IMediaSource4Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaSource4Vtbl {
        unsafe extern "system" fn AdaptiveMediaSource<Impl: IMediaSource4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AdaptiveMediaSource() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediaStreamSource<Impl: IMediaSource4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MediaStreamSource() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MseStreamSource<Impl: IMediaSource4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MseStreamSource() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Uri<Impl: IMediaSource4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Uri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenAsync<Impl: IMediaSource4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OpenAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaSource4>, base.5, AdaptiveMediaSource::<Impl, OFFSET>, MediaStreamSource::<Impl, OFFSET>, MseStreamSource::<Impl, OFFSET>, Uri::<Impl, OFFSET>, OpenAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaSource5Impl: Sized {
    fn DownloadOperation(&self) -> ::windows::core::Result<super::super::Networking::BackgroundTransfer::DownloadOperation>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaSource5 {
    const NAME: &'static str = "Windows.Media.Core.IMediaSource5";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaSource5Vtbl {
    pub const fn new<Impl: IMediaSource5Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaSource5Vtbl {
        unsafe extern "system" fn DownloadOperation<Impl: IMediaSource5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DownloadOperation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaSource5>, base.5, DownloadOperation::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaSourceAppServiceConnectionImpl: Sized {
    fn InitializeMediaStreamSourceRequested(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaSourceAppServiceConnection, InitializeMediaStreamSourceRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveInitializeMediaStreamSourceRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Start(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaSourceAppServiceConnection {
    const NAME: &'static str = "Windows.Media.Core.IMediaSourceAppServiceConnection";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaSourceAppServiceConnectionVtbl {
    pub const fn new<Impl: IMediaSourceAppServiceConnectionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaSourceAppServiceConnectionVtbl {
        unsafe extern "system" fn InitializeMediaStreamSourceRequested<Impl: IMediaSourceAppServiceConnectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InitializeMediaStreamSourceRequested(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MediaSourceAppServiceConnection, InitializeMediaStreamSourceRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MediaSourceAppServiceConnection, InitializeMediaStreamSourceRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveInitializeMediaStreamSourceRequested<Impl: IMediaSourceAppServiceConnectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveInitializeMediaStreamSourceRequested(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Start<Impl: IMediaSourceAppServiceConnectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Start().into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaSourceAppServiceConnection>, base.5, InitializeMediaStreamSourceRequested::<Impl, OFFSET>, RemoveInitializeMediaStreamSourceRequested::<Impl, OFFSET>, Start::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaSourceAppServiceConnectionFactoryImpl: Sized {
    fn Create(&self, appserviceconnection: &::core::option::Option<super::super::ApplicationModel::AppService::AppServiceConnection>) -> ::windows::core::Result<MediaSourceAppServiceConnection>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaSourceAppServiceConnectionFactory {
    const NAME: &'static str = "Windows.Media.Core.IMediaSourceAppServiceConnectionFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaSourceAppServiceConnectionFactoryVtbl {
    pub const fn new<Impl: IMediaSourceAppServiceConnectionFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaSourceAppServiceConnectionFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IMediaSourceAppServiceConnectionFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, appserviceconnection: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(&*(&appserviceconnection as *const <super::super::ApplicationModel::AppService::AppServiceConnection as ::windows::core::Abi>::Abi as *const <super::super::ApplicationModel::AppService::AppServiceConnection as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaSourceAppServiceConnectionFactory>, base.5, Create::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaSourceErrorImpl: Sized {
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaSourceError {
    const NAME: &'static str = "Windows.Media.Core.IMediaSourceError";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaSourceErrorVtbl {
    pub const fn new<Impl: IMediaSourceErrorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaSourceErrorVtbl {
        unsafe extern "system" fn ExtendedError<Impl: IMediaSourceErrorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExtendedError() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaSourceError>, base.5, ExtendedError::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaSourceOpenOperationCompletedEventArgsImpl: Sized {
    fn Error(&self) -> ::windows::core::Result<MediaSourceError>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaSourceOpenOperationCompletedEventArgs {
    const NAME: &'static str = "Windows.Media.Core.IMediaSourceOpenOperationCompletedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaSourceOpenOperationCompletedEventArgsVtbl {
    pub const fn new<Impl: IMediaSourceOpenOperationCompletedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaSourceOpenOperationCompletedEventArgsVtbl {
        unsafe extern "system" fn Error<Impl: IMediaSourceOpenOperationCompletedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Error() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaSourceOpenOperationCompletedEventArgs>, base.5, Error::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaSourceStateChangedEventArgsImpl: Sized {
    fn OldState(&self) -> ::windows::core::Result<MediaSourceState>;
    fn NewState(&self) -> ::windows::core::Result<MediaSourceState>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaSourceStateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Core.IMediaSourceStateChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaSourceStateChangedEventArgsVtbl {
    pub const fn new<Impl: IMediaSourceStateChangedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaSourceStateChangedEventArgsVtbl {
        unsafe extern "system" fn OldState<Impl: IMediaSourceStateChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut MediaSourceState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OldState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NewState<Impl: IMediaSourceStateChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut MediaSourceState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NewState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaSourceStateChangedEventArgs>, base.5, OldState::<Impl, OFFSET>, NewState::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaSourceStaticsImpl: Sized {
    fn CreateFromAdaptiveMediaSource(&self, mediasource: &::core::option::Option<super::Streaming::Adaptive::AdaptiveMediaSource>) -> ::windows::core::Result<MediaSource>;
    fn CreateFromMediaStreamSource(&self, mediasource: &::core::option::Option<MediaStreamSource>) -> ::windows::core::Result<MediaSource>;
    fn CreateFromMseStreamSource(&self, mediasource: &::core::option::Option<MseStreamSource>) -> ::windows::core::Result<MediaSource>;
    fn CreateFromIMediaSource(&self, mediasource: &::core::option::Option<IMediaSource>) -> ::windows::core::Result<MediaSource>;
    fn CreateFromStorageFile(&self, file: &::core::option::Option<super::super::Storage::IStorageFile>) -> ::windows::core::Result<MediaSource>;
    fn CreateFromStream(&self, stream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>, contenttype: &::windows::core::HSTRING) -> ::windows::core::Result<MediaSource>;
    fn CreateFromStreamReference(&self, stream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStreamReference>, contenttype: &::windows::core::HSTRING) -> ::windows::core::Result<MediaSource>;
    fn CreateFromUri(&self, uri: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<MediaSource>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaSourceStatics {
    const NAME: &'static str = "Windows.Media.Core.IMediaSourceStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaSourceStaticsVtbl {
    pub const fn new<Impl: IMediaSourceStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaSourceStaticsVtbl {
        unsafe extern "system" fn CreateFromAdaptiveMediaSource<Impl: IMediaSourceStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mediasource: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateFromAdaptiveMediaSource(&*(&mediasource as *const <super::Streaming::Adaptive::AdaptiveMediaSource as ::windows::core::Abi>::Abi as *const <super::Streaming::Adaptive::AdaptiveMediaSource as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromMediaStreamSource<Impl: IMediaSourceStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mediasource: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateFromMediaStreamSource(&*(&mediasource as *const <MediaStreamSource as ::windows::core::Abi>::Abi as *const <MediaStreamSource as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromMseStreamSource<Impl: IMediaSourceStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mediasource: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateFromMseStreamSource(&*(&mediasource as *const <MseStreamSource as ::windows::core::Abi>::Abi as *const <MseStreamSource as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromIMediaSource<Impl: IMediaSourceStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mediasource: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateFromIMediaSource(&*(&mediasource as *const <IMediaSource as ::windows::core::Abi>::Abi as *const <IMediaSource as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromStorageFile<Impl: IMediaSourceStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateFromStorageFile(&*(&file as *const <super::super::Storage::IStorageFile as ::windows::core::Abi>::Abi as *const <super::super::Storage::IStorageFile as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromStream<Impl: IMediaSourceStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, contenttype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateFromStream(&*(&stream as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::DefaultType>::DefaultType), &*(&contenttype as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromStreamReference<Impl: IMediaSourceStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, contenttype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateFromStreamReference(&*(&stream as *const <super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::DefaultType>::DefaultType), &*(&contenttype as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromUri<Impl: IMediaSourceStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateFromUri(&*(&uri as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaSourceStatics>, base.5, CreateFromAdaptiveMediaSource::<Impl, OFFSET>, CreateFromMediaStreamSource::<Impl, OFFSET>, CreateFromMseStreamSource::<Impl, OFFSET>, CreateFromIMediaSource::<Impl, OFFSET>, CreateFromStorageFile::<Impl, OFFSET>, CreateFromStream::<Impl, OFFSET>, CreateFromStreamReference::<Impl, OFFSET>, CreateFromUri::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaSourceStatics2Impl: Sized {
    fn CreateFromMediaBinder(&self, binder: &::core::option::Option<MediaBinder>) -> ::windows::core::Result<MediaSource>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaSourceStatics2 {
    const NAME: &'static str = "Windows.Media.Core.IMediaSourceStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaSourceStatics2Vtbl {
    pub const fn new<Impl: IMediaSourceStatics2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaSourceStatics2Vtbl {
        unsafe extern "system" fn CreateFromMediaBinder<Impl: IMediaSourceStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, binder: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateFromMediaBinder(&*(&binder as *const <MediaBinder as ::windows::core::Abi>::Abi as *const <MediaBinder as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaSourceStatics2>, base.5, CreateFromMediaBinder::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaSourceStatics3Impl: Sized {
    fn CreateFromMediaFrameSource(&self, framesource: &::core::option::Option<super::Capture::Frames::MediaFrameSource>) -> ::windows::core::Result<MediaSource>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaSourceStatics3 {
    const NAME: &'static str = "Windows.Media.Core.IMediaSourceStatics3";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaSourceStatics3Vtbl {
    pub const fn new<Impl: IMediaSourceStatics3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaSourceStatics3Vtbl {
        unsafe extern "system" fn CreateFromMediaFrameSource<Impl: IMediaSourceStatics3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, framesource: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateFromMediaFrameSource(&*(&framesource as *const <super::Capture::Frames::MediaFrameSource as ::windows::core::Abi>::Abi as *const <super::Capture::Frames::MediaFrameSource as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaSourceStatics3>, base.5, CreateFromMediaFrameSource::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaSourceStatics4Impl: Sized {
    fn CreateFromDownloadOperation(&self, downloadoperation: &::core::option::Option<super::super::Networking::BackgroundTransfer::DownloadOperation>) -> ::windows::core::Result<MediaSource>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaSourceStatics4 {
    const NAME: &'static str = "Windows.Media.Core.IMediaSourceStatics4";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaSourceStatics4Vtbl {
    pub const fn new<Impl: IMediaSourceStatics4Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaSourceStatics4Vtbl {
        unsafe extern "system" fn CreateFromDownloadOperation<Impl: IMediaSourceStatics4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, downloadoperation: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateFromDownloadOperation(&*(&downloadoperation as *const <super::super::Networking::BackgroundTransfer::DownloadOperation as ::windows::core::Abi>::Abi as *const <super::super::Networking::BackgroundTransfer::DownloadOperation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaSourceStatics4>, base.5, CreateFromDownloadOperation::<Impl, OFFSET>)
    }
}
pub trait IMediaStreamDescriptorImpl: Sized {
    fn IsSelected(&self) -> ::windows::core::Result<bool>;
    fn SetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetLanguage(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for IMediaStreamDescriptor {
    const NAME: &'static str = "Windows.Media.Core.IMediaStreamDescriptor";
}
impl IMediaStreamDescriptorVtbl {
    pub const fn new<Impl: IMediaStreamDescriptorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaStreamDescriptorVtbl {
        unsafe extern "system" fn IsSelected<Impl: IMediaStreamDescriptorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsSelected() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: IMediaStreamDescriptorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Name<Impl: IMediaStreamDescriptorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLanguage<Impl: IMediaStreamDescriptorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetLanguage(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Language<Impl: IMediaStreamDescriptorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Language() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaStreamDescriptor>, base.5, IsSelected::<Impl, OFFSET>, SetName::<Impl, OFFSET>, Name::<Impl, OFFSET>, SetLanguage::<Impl, OFFSET>, Language::<Impl, OFFSET>)
    }
}
pub trait IMediaStreamDescriptor2Impl: Sized + IMediaStreamDescriptorImpl {
    fn SetLabel(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Label(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for IMediaStreamDescriptor2 {
    const NAME: &'static str = "Windows.Media.Core.IMediaStreamDescriptor2";
}
impl IMediaStreamDescriptor2Vtbl {
    pub const fn new<Impl: IMediaStreamDescriptor2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaStreamDescriptor2Vtbl {
        unsafe extern "system" fn SetLabel<Impl: IMediaStreamDescriptor2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetLabel(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Label<Impl: IMediaStreamDescriptor2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Label() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaStreamDescriptor2>, base.5, SetLabel::<Impl, OFFSET>, Label::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaStreamSampleImpl: Sized {
    fn Processed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaStreamSample, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveProcessed(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Buffer(&self) -> ::windows::core::Result<super::super::Storage::Streams::Buffer>;
    fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn ExtendedProperties(&self) -> ::windows::core::Result<MediaStreamSamplePropertySet>;
    fn Protection(&self) -> ::windows::core::Result<MediaStreamSampleProtectionProperties>;
    fn SetDecodeTimestamp(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn DecodeTimestamp(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetDuration(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetKeyFrame(&self, value: bool) -> ::windows::core::Result<()>;
    fn KeyFrame(&self) -> ::windows::core::Result<bool>;
    fn SetDiscontinuous(&self, value: bool) -> ::windows::core::Result<()>;
    fn Discontinuous(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaStreamSample {
    const NAME: &'static str = "Windows.Media.Core.IMediaStreamSample";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaStreamSampleVtbl {
    pub const fn new<Impl: IMediaStreamSampleImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaStreamSampleVtbl {
        unsafe extern "system" fn Processed<Impl: IMediaStreamSampleImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Processed(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MediaStreamSample, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MediaStreamSample, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveProcessed<Impl: IMediaStreamSampleImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveProcessed(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Buffer<Impl: IMediaStreamSampleImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Timestamp<Impl: IMediaStreamSampleImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Timestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedProperties<Impl: IMediaStreamSampleImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExtendedProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Protection<Impl: IMediaStreamSampleImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Protection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDecodeTimestamp<Impl: IMediaStreamSampleImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDecodeTimestamp(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DecodeTimestamp<Impl: IMediaStreamSampleImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DecodeTimestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDuration<Impl: IMediaStreamSampleImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDuration(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Duration<Impl: IMediaStreamSampleImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetKeyFrame<Impl: IMediaStreamSampleImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetKeyFrame(value).into()
        }
        unsafe extern "system" fn KeyFrame<Impl: IMediaStreamSampleImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).KeyFrame() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDiscontinuous<Impl: IMediaStreamSampleImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDiscontinuous(value).into()
        }
        unsafe extern "system" fn Discontinuous<Impl: IMediaStreamSampleImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Discontinuous() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IMediaStreamSample>,
            base.5,
            Processed::<Impl, OFFSET>,
            RemoveProcessed::<Impl, OFFSET>,
            Buffer::<Impl, OFFSET>,
            Timestamp::<Impl, OFFSET>,
            ExtendedProperties::<Impl, OFFSET>,
            Protection::<Impl, OFFSET>,
            SetDecodeTimestamp::<Impl, OFFSET>,
            DecodeTimestamp::<Impl, OFFSET>,
            SetDuration::<Impl, OFFSET>,
            Duration::<Impl, OFFSET>,
            SetKeyFrame::<Impl, OFFSET>,
            KeyFrame::<Impl, OFFSET>,
            SetDiscontinuous::<Impl, OFFSET>,
            Discontinuous::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaStreamSample2Impl: Sized {
    fn Direct3D11Surface(&self) -> ::windows::core::Result<super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaStreamSample2 {
    const NAME: &'static str = "Windows.Media.Core.IMediaStreamSample2";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaStreamSample2Vtbl {
    pub const fn new<Impl: IMediaStreamSample2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaStreamSample2Vtbl {
        unsafe extern "system" fn Direct3D11Surface<Impl: IMediaStreamSample2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Direct3D11Surface() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaStreamSample2>, base.5, Direct3D11Surface::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaStreamSampleProtectionPropertiesImpl: Sized {
    fn SetKeyIdentifier(&self, value: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn GetKeyIdentifier(&self, value: &mut ::windows::core::Array<u8>) -> ::windows::core::Result<()>;
    fn SetInitializationVector(&self, value: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn GetInitializationVector(&self, value: &mut ::windows::core::Array<u8>) -> ::windows::core::Result<()>;
    fn SetSubSampleMapping(&self, value: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn GetSubSampleMapping(&self, value: &mut ::windows::core::Array<u8>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaStreamSampleProtectionProperties {
    const NAME: &'static str = "Windows.Media.Core.IMediaStreamSampleProtectionProperties";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaStreamSampleProtectionPropertiesVtbl {
    pub const fn new<Impl: IMediaStreamSampleProtectionPropertiesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaStreamSampleProtectionPropertiesVtbl {
        unsafe extern "system" fn SetKeyIdentifier<Impl: IMediaStreamSampleProtectionPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetKeyIdentifier(::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _)).into()
        }
        unsafe extern "system" fn GetKeyIdentifier<Impl: IMediaStreamSampleProtectionPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).GetKeyIdentifier(::windows::core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size).as_array()).into()
        }
        unsafe extern "system" fn SetInitializationVector<Impl: IMediaStreamSampleProtectionPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetInitializationVector(::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _)).into()
        }
        unsafe extern "system" fn GetInitializationVector<Impl: IMediaStreamSampleProtectionPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).GetInitializationVector(::windows::core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size).as_array()).into()
        }
        unsafe extern "system" fn SetSubSampleMapping<Impl: IMediaStreamSampleProtectionPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSubSampleMapping(::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _)).into()
        }
        unsafe extern "system" fn GetSubSampleMapping<Impl: IMediaStreamSampleProtectionPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).GetSubSampleMapping(::windows::core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size).as_array()).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaStreamSampleProtectionProperties>, base.5, SetKeyIdentifier::<Impl, OFFSET>, GetKeyIdentifier::<Impl, OFFSET>, SetInitializationVector::<Impl, OFFSET>, GetInitializationVector::<Impl, OFFSET>, SetSubSampleMapping::<Impl, OFFSET>, GetSubSampleMapping::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaStreamSampleStaticsImpl: Sized {
    fn CreateFromBuffer(&self, buffer: &::core::option::Option<super::super::Storage::Streams::IBuffer>, timestamp: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<MediaStreamSample>;
    fn CreateFromStreamAsync(&self, stream: &::core::option::Option<super::super::Storage::Streams::IInputStream>, count: u32, timestamp: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MediaStreamSample>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaStreamSampleStatics {
    const NAME: &'static str = "Windows.Media.Core.IMediaStreamSampleStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaStreamSampleStaticsVtbl {
    pub const fn new<Impl: IMediaStreamSampleStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaStreamSampleStaticsVtbl {
        unsafe extern "system" fn CreateFromBuffer<Impl: IMediaStreamSampleStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, buffer: ::windows::core::RawPtr, timestamp: super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateFromBuffer(&*(&buffer as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType), &*(&timestamp as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromStreamAsync<Impl: IMediaStreamSampleStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, count: u32, timestamp: super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateFromStreamAsync(&*(&stream as *const <super::super::Storage::Streams::IInputStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IInputStream as ::windows::core::DefaultType>::DefaultType), count, &*(&timestamp as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaStreamSampleStatics>, base.5, CreateFromBuffer::<Impl, OFFSET>, CreateFromStreamAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaStreamSampleStatics2Impl: Sized {
    fn CreateFromDirect3D11Surface(&self, surface: &::core::option::Option<super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface>, timestamp: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<MediaStreamSample>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaStreamSampleStatics2 {
    const NAME: &'static str = "Windows.Media.Core.IMediaStreamSampleStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaStreamSampleStatics2Vtbl {
    pub const fn new<Impl: IMediaStreamSampleStatics2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaStreamSampleStatics2Vtbl {
        unsafe extern "system" fn CreateFromDirect3D11Surface<Impl: IMediaStreamSampleStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, surface: ::windows::core::RawPtr, timestamp: super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateFromDirect3D11Surface(&*(&surface as *const <super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface as ::windows::core::Abi>::Abi as *const <super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface as ::windows::core::DefaultType>::DefaultType), &*(&timestamp as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaStreamSampleStatics2>, base.5, CreateFromDirect3D11Surface::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaStreamSourceImpl: Sized + IMediaSourceImpl {
    fn Closed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaStreamSource, MediaStreamSourceClosedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveClosed(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Starting(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaStreamSource, MediaStreamSourceStartingEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStarting(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Paused(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaStreamSource, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePaused(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SampleRequested(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaStreamSource, MediaStreamSourceSampleRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSampleRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SwitchStreamsRequested(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaStreamSource, MediaStreamSourceSwitchStreamsRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSwitchStreamsRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn NotifyError(&self, errorstatus: MediaStreamSourceErrorStatus) -> ::windows::core::Result<()>;
    fn AddStreamDescriptor(&self, descriptor: &::core::option::Option<IMediaStreamDescriptor>) -> ::windows::core::Result<()>;
    fn SetMediaProtectionManager(&self, value: &::core::option::Option<super::Protection::MediaProtectionManager>) -> ::windows::core::Result<()>;
    fn MediaProtectionManager(&self) -> ::windows::core::Result<super::Protection::MediaProtectionManager>;
    fn SetDuration(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetCanSeek(&self, value: bool) -> ::windows::core::Result<()>;
    fn CanSeek(&self) -> ::windows::core::Result<bool>;
    fn SetBufferTime(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn BufferTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetBufferedRange(&self, startoffset: &super::super::Foundation::TimeSpan, endoffset: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn MusicProperties(&self) -> ::windows::core::Result<super::super::Storage::FileProperties::MusicProperties>;
    fn VideoProperties(&self) -> ::windows::core::Result<super::super::Storage::FileProperties::VideoProperties>;
    fn SetThumbnail(&self, value: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<()>;
    fn Thumbnail(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference>;
    fn AddProtectionKey(&self, streamdescriptor: &::core::option::Option<IMediaStreamDescriptor>, keyidentifier: &[<u8 as ::windows::core::DefaultType>::DefaultType], licensedata: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaStreamSource {
    const NAME: &'static str = "Windows.Media.Core.IMediaStreamSource";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaStreamSourceVtbl {
    pub const fn new<Impl: IMediaStreamSourceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaStreamSourceVtbl {
        unsafe extern "system" fn Closed<Impl: IMediaStreamSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Closed(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MediaStreamSource, MediaStreamSourceClosedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MediaStreamSource, MediaStreamSourceClosedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveClosed<Impl: IMediaStreamSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveClosed(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Starting<Impl: IMediaStreamSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Starting(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MediaStreamSource, MediaStreamSourceStartingEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MediaStreamSource, MediaStreamSourceStartingEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStarting<Impl: IMediaStreamSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveStarting(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Paused<Impl: IMediaStreamSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Paused(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MediaStreamSource, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MediaStreamSource, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePaused<Impl: IMediaStreamSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemovePaused(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SampleRequested<Impl: IMediaStreamSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SampleRequested(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MediaStreamSource, MediaStreamSourceSampleRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MediaStreamSource, MediaStreamSourceSampleRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSampleRequested<Impl: IMediaStreamSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveSampleRequested(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SwitchStreamsRequested<Impl: IMediaStreamSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SwitchStreamsRequested(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MediaStreamSource, MediaStreamSourceSwitchStreamsRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MediaStreamSource, MediaStreamSourceSwitchStreamsRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSwitchStreamsRequested<Impl: IMediaStreamSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveSwitchStreamsRequested(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn NotifyError<Impl: IMediaStreamSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, errorstatus: MediaStreamSourceErrorStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).NotifyError(errorstatus).into()
        }
        unsafe extern "system" fn AddStreamDescriptor<Impl: IMediaStreamSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, descriptor: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).AddStreamDescriptor(&*(&descriptor as *const <IMediaStreamDescriptor as ::windows::core::Abi>::Abi as *const <IMediaStreamDescriptor as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetMediaProtectionManager<Impl: IMediaStreamSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetMediaProtectionManager(&*(&value as *const <super::Protection::MediaProtectionManager as ::windows::core::Abi>::Abi as *const <super::Protection::MediaProtectionManager as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MediaProtectionManager<Impl: IMediaStreamSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MediaProtectionManager() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDuration<Impl: IMediaStreamSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDuration(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Duration<Impl: IMediaStreamSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCanSeek<Impl: IMediaStreamSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCanSeek(value).into()
        }
        unsafe extern "system" fn CanSeek<Impl: IMediaStreamSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CanSeek() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBufferTime<Impl: IMediaStreamSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetBufferTime(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BufferTime<Impl: IMediaStreamSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BufferTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBufferedRange<Impl: IMediaStreamSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startoffset: super::super::Foundation::TimeSpan, endoffset: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetBufferedRange(&*(&startoffset as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType), &*(&endoffset as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MusicProperties<Impl: IMediaStreamSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MusicProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VideoProperties<Impl: IMediaStreamSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VideoProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetThumbnail<Impl: IMediaStreamSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetThumbnail(&*(&value as *const <super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStreamReference as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Thumbnail<Impl: IMediaStreamSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Thumbnail() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddProtectionKey<Impl: IMediaStreamSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, streamdescriptor: ::windows::core::RawPtr, keyIdentifier_array_size: u32, keyidentifier: *const u8, licenseData_array_size: u32, licensedata: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).AddProtectionKey(&*(&streamdescriptor as *const <IMediaStreamDescriptor as ::windows::core::Abi>::Abi as *const <IMediaStreamDescriptor as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&keyidentifier), keyIdentifier_array_size as _), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&licensedata), licenseData_array_size as _)).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IMediaStreamSource>,
            base.5,
            Closed::<Impl, OFFSET>,
            RemoveClosed::<Impl, OFFSET>,
            Starting::<Impl, OFFSET>,
            RemoveStarting::<Impl, OFFSET>,
            Paused::<Impl, OFFSET>,
            RemovePaused::<Impl, OFFSET>,
            SampleRequested::<Impl, OFFSET>,
            RemoveSampleRequested::<Impl, OFFSET>,
            SwitchStreamsRequested::<Impl, OFFSET>,
            RemoveSwitchStreamsRequested::<Impl, OFFSET>,
            NotifyError::<Impl, OFFSET>,
            AddStreamDescriptor::<Impl, OFFSET>,
            SetMediaProtectionManager::<Impl, OFFSET>,
            MediaProtectionManager::<Impl, OFFSET>,
            SetDuration::<Impl, OFFSET>,
            Duration::<Impl, OFFSET>,
            SetCanSeek::<Impl, OFFSET>,
            CanSeek::<Impl, OFFSET>,
            SetBufferTime::<Impl, OFFSET>,
            BufferTime::<Impl, OFFSET>,
            SetBufferedRange::<Impl, OFFSET>,
            MusicProperties::<Impl, OFFSET>,
            VideoProperties::<Impl, OFFSET>,
            SetThumbnail::<Impl, OFFSET>,
            Thumbnail::<Impl, OFFSET>,
            AddProtectionKey::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaStreamSource2Impl: Sized + IMediaSourceImpl + IMediaStreamSourceImpl {
    fn SampleRendered(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MediaStreamSource, MediaStreamSourceSampleRenderedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSampleRendered(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaStreamSource2 {
    const NAME: &'static str = "Windows.Media.Core.IMediaStreamSource2";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaStreamSource2Vtbl {
    pub const fn new<Impl: IMediaStreamSource2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaStreamSource2Vtbl {
        unsafe extern "system" fn SampleRendered<Impl: IMediaStreamSource2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SampleRendered(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MediaStreamSource, MediaStreamSourceSampleRenderedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MediaStreamSource, MediaStreamSourceSampleRenderedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSampleRendered<Impl: IMediaStreamSource2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveSampleRendered(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaStreamSource2>, base.5, SampleRendered::<Impl, OFFSET>, RemoveSampleRendered::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaStreamSource3Impl: Sized + IMediaSourceImpl + IMediaStreamSourceImpl {
    fn SetMaxSupportedPlaybackRate(&self, value: &::core::option::Option<super::super::Foundation::IReference<f64>>) -> ::windows::core::Result<()>;
    fn MaxSupportedPlaybackRate(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaStreamSource3 {
    const NAME: &'static str = "Windows.Media.Core.IMediaStreamSource3";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaStreamSource3Vtbl {
    pub const fn new<Impl: IMediaStreamSource3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaStreamSource3Vtbl {
        unsafe extern "system" fn SetMaxSupportedPlaybackRate<Impl: IMediaStreamSource3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetMaxSupportedPlaybackRate(&*(&value as *const <super::super::Foundation::IReference<f64> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<f64> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MaxSupportedPlaybackRate<Impl: IMediaStreamSource3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MaxSupportedPlaybackRate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaStreamSource3>, base.5, SetMaxSupportedPlaybackRate::<Impl, OFFSET>, MaxSupportedPlaybackRate::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaStreamSource4Impl: Sized + IMediaSourceImpl + IMediaStreamSourceImpl {
    fn SetIsLive(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsLive(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaStreamSource4 {
    const NAME: &'static str = "Windows.Media.Core.IMediaStreamSource4";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaStreamSource4Vtbl {
    pub const fn new<Impl: IMediaStreamSource4Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaStreamSource4Vtbl {
        unsafe extern "system" fn SetIsLive<Impl: IMediaStreamSource4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsLive(value).into()
        }
        unsafe extern "system" fn IsLive<Impl: IMediaStreamSource4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsLive() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaStreamSource4>, base.5, SetIsLive::<Impl, OFFSET>, IsLive::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaStreamSourceClosedEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<MediaStreamSourceClosedRequest>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaStreamSourceClosedEventArgs {
    const NAME: &'static str = "Windows.Media.Core.IMediaStreamSourceClosedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaStreamSourceClosedEventArgsVtbl {
    pub const fn new<Impl: IMediaStreamSourceClosedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaStreamSourceClosedEventArgsVtbl {
        unsafe extern "system" fn Request<Impl: IMediaStreamSourceClosedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Request() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaStreamSourceClosedEventArgs>, base.5, Request::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaStreamSourceClosedRequestImpl: Sized {
    fn Reason(&self) -> ::windows::core::Result<MediaStreamSourceClosedReason>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaStreamSourceClosedRequest {
    const NAME: &'static str = "Windows.Media.Core.IMediaStreamSourceClosedRequest";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaStreamSourceClosedRequestVtbl {
    pub const fn new<Impl: IMediaStreamSourceClosedRequestImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaStreamSourceClosedRequestVtbl {
        unsafe extern "system" fn Reason<Impl: IMediaStreamSourceClosedRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut MediaStreamSourceClosedReason) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Reason() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaStreamSourceClosedRequest>, base.5, Reason::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaStreamSourceFactoryImpl: Sized {
    fn CreateFromDescriptor(&self, descriptor: &::core::option::Option<IMediaStreamDescriptor>) -> ::windows::core::Result<MediaStreamSource>;
    fn CreateFromDescriptors(&self, descriptor: &::core::option::Option<IMediaStreamDescriptor>, descriptor2: &::core::option::Option<IMediaStreamDescriptor>) -> ::windows::core::Result<MediaStreamSource>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaStreamSourceFactory {
    const NAME: &'static str = "Windows.Media.Core.IMediaStreamSourceFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaStreamSourceFactoryVtbl {
    pub const fn new<Impl: IMediaStreamSourceFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaStreamSourceFactoryVtbl {
        unsafe extern "system" fn CreateFromDescriptor<Impl: IMediaStreamSourceFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, descriptor: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateFromDescriptor(&*(&descriptor as *const <IMediaStreamDescriptor as ::windows::core::Abi>::Abi as *const <IMediaStreamDescriptor as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromDescriptors<Impl: IMediaStreamSourceFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, descriptor: ::windows::core::RawPtr, descriptor2: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateFromDescriptors(&*(&descriptor as *const <IMediaStreamDescriptor as ::windows::core::Abi>::Abi as *const <IMediaStreamDescriptor as ::windows::core::DefaultType>::DefaultType), &*(&descriptor2 as *const <IMediaStreamDescriptor as ::windows::core::Abi>::Abi as *const <IMediaStreamDescriptor as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaStreamSourceFactory>, base.5, CreateFromDescriptor::<Impl, OFFSET>, CreateFromDescriptors::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaStreamSourceSampleRenderedEventArgsImpl: Sized {
    fn SampleLag(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaStreamSourceSampleRenderedEventArgs {
    const NAME: &'static str = "Windows.Media.Core.IMediaStreamSourceSampleRenderedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaStreamSourceSampleRenderedEventArgsVtbl {
    pub const fn new<Impl: IMediaStreamSourceSampleRenderedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaStreamSourceSampleRenderedEventArgsVtbl {
        unsafe extern "system" fn SampleLag<Impl: IMediaStreamSourceSampleRenderedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SampleLag() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaStreamSourceSampleRenderedEventArgs>, base.5, SampleLag::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaStreamSourceSampleRequestImpl: Sized {
    fn StreamDescriptor(&self) -> ::windows::core::Result<IMediaStreamDescriptor>;
    fn GetDeferral(&self) -> ::windows::core::Result<MediaStreamSourceSampleRequestDeferral>;
    fn SetSample(&self, value: &::core::option::Option<MediaStreamSample>) -> ::windows::core::Result<()>;
    fn Sample(&self) -> ::windows::core::Result<MediaStreamSample>;
    fn ReportSampleProgress(&self, progress: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaStreamSourceSampleRequest {
    const NAME: &'static str = "Windows.Media.Core.IMediaStreamSourceSampleRequest";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaStreamSourceSampleRequestVtbl {
    pub const fn new<Impl: IMediaStreamSourceSampleRequestImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaStreamSourceSampleRequestVtbl {
        unsafe extern "system" fn StreamDescriptor<Impl: IMediaStreamSourceSampleRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StreamDescriptor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeferral<Impl: IMediaStreamSourceSampleRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSample<Impl: IMediaStreamSourceSampleRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSample(&*(&value as *const <MediaStreamSample as ::windows::core::Abi>::Abi as *const <MediaStreamSample as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Sample<Impl: IMediaStreamSourceSampleRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Sample() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportSampleProgress<Impl: IMediaStreamSourceSampleRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, progress: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).ReportSampleProgress(progress).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaStreamSourceSampleRequest>, base.5, StreamDescriptor::<Impl, OFFSET>, GetDeferral::<Impl, OFFSET>, SetSample::<Impl, OFFSET>, Sample::<Impl, OFFSET>, ReportSampleProgress::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaStreamSourceSampleRequestDeferralImpl: Sized {
    fn Complete(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaStreamSourceSampleRequestDeferral {
    const NAME: &'static str = "Windows.Media.Core.IMediaStreamSourceSampleRequestDeferral";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaStreamSourceSampleRequestDeferralVtbl {
    pub const fn new<Impl: IMediaStreamSourceSampleRequestDeferralImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaStreamSourceSampleRequestDeferralVtbl {
        unsafe extern "system" fn Complete<Impl: IMediaStreamSourceSampleRequestDeferralImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Complete().into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaStreamSourceSampleRequestDeferral>, base.5, Complete::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaStreamSourceSampleRequestedEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<MediaStreamSourceSampleRequest>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaStreamSourceSampleRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.Core.IMediaStreamSourceSampleRequestedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaStreamSourceSampleRequestedEventArgsVtbl {
    pub const fn new<Impl: IMediaStreamSourceSampleRequestedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaStreamSourceSampleRequestedEventArgsVtbl {
        unsafe extern "system" fn Request<Impl: IMediaStreamSourceSampleRequestedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Request() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaStreamSourceSampleRequestedEventArgs>, base.5, Request::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaStreamSourceStartingEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<MediaStreamSourceStartingRequest>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaStreamSourceStartingEventArgs {
    const NAME: &'static str = "Windows.Media.Core.IMediaStreamSourceStartingEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaStreamSourceStartingEventArgsVtbl {
    pub const fn new<Impl: IMediaStreamSourceStartingEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaStreamSourceStartingEventArgsVtbl {
        unsafe extern "system" fn Request<Impl: IMediaStreamSourceStartingEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Request() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaStreamSourceStartingEventArgs>, base.5, Request::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaStreamSourceStartingRequestImpl: Sized {
    fn StartPosition(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>;
    fn GetDeferral(&self) -> ::windows::core::Result<MediaStreamSourceStartingRequestDeferral>;
    fn SetActualStartPosition(&self, position: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaStreamSourceStartingRequest {
    const NAME: &'static str = "Windows.Media.Core.IMediaStreamSourceStartingRequest";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaStreamSourceStartingRequestVtbl {
    pub const fn new<Impl: IMediaStreamSourceStartingRequestImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaStreamSourceStartingRequestVtbl {
        unsafe extern "system" fn StartPosition<Impl: IMediaStreamSourceStartingRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StartPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeferral<Impl: IMediaStreamSourceStartingRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetActualStartPosition<Impl: IMediaStreamSourceStartingRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, position: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetActualStartPosition(&*(&position as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaStreamSourceStartingRequest>, base.5, StartPosition::<Impl, OFFSET>, GetDeferral::<Impl, OFFSET>, SetActualStartPosition::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaStreamSourceStartingRequestDeferralImpl: Sized {
    fn Complete(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaStreamSourceStartingRequestDeferral {
    const NAME: &'static str = "Windows.Media.Core.IMediaStreamSourceStartingRequestDeferral";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaStreamSourceStartingRequestDeferralVtbl {
    pub const fn new<Impl: IMediaStreamSourceStartingRequestDeferralImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaStreamSourceStartingRequestDeferralVtbl {
        unsafe extern "system" fn Complete<Impl: IMediaStreamSourceStartingRequestDeferralImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Complete().into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaStreamSourceStartingRequestDeferral>, base.5, Complete::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaStreamSourceSwitchStreamsRequestImpl: Sized {
    fn OldStreamDescriptor(&self) -> ::windows::core::Result<IMediaStreamDescriptor>;
    fn NewStreamDescriptor(&self) -> ::windows::core::Result<IMediaStreamDescriptor>;
    fn GetDeferral(&self) -> ::windows::core::Result<MediaStreamSourceSwitchStreamsRequestDeferral>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaStreamSourceSwitchStreamsRequest {
    const NAME: &'static str = "Windows.Media.Core.IMediaStreamSourceSwitchStreamsRequest";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaStreamSourceSwitchStreamsRequestVtbl {
    pub const fn new<Impl: IMediaStreamSourceSwitchStreamsRequestImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaStreamSourceSwitchStreamsRequestVtbl {
        unsafe extern "system" fn OldStreamDescriptor<Impl: IMediaStreamSourceSwitchStreamsRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OldStreamDescriptor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NewStreamDescriptor<Impl: IMediaStreamSourceSwitchStreamsRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NewStreamDescriptor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeferral<Impl: IMediaStreamSourceSwitchStreamsRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaStreamSourceSwitchStreamsRequest>, base.5, OldStreamDescriptor::<Impl, OFFSET>, NewStreamDescriptor::<Impl, OFFSET>, GetDeferral::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaStreamSourceSwitchStreamsRequestDeferralImpl: Sized {
    fn Complete(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaStreamSourceSwitchStreamsRequestDeferral {
    const NAME: &'static str = "Windows.Media.Core.IMediaStreamSourceSwitchStreamsRequestDeferral";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaStreamSourceSwitchStreamsRequestDeferralVtbl {
    pub const fn new<Impl: IMediaStreamSourceSwitchStreamsRequestDeferralImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaStreamSourceSwitchStreamsRequestDeferralVtbl {
        unsafe extern "system" fn Complete<Impl: IMediaStreamSourceSwitchStreamsRequestDeferralImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Complete().into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaStreamSourceSwitchStreamsRequestDeferral>, base.5, Complete::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaStreamSourceSwitchStreamsRequestedEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<MediaStreamSourceSwitchStreamsRequest>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaStreamSourceSwitchStreamsRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.Core.IMediaStreamSourceSwitchStreamsRequestedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaStreamSourceSwitchStreamsRequestedEventArgsVtbl {
    pub const fn new<Impl: IMediaStreamSourceSwitchStreamsRequestedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaStreamSourceSwitchStreamsRequestedEventArgsVtbl {
        unsafe extern "system" fn Request<Impl: IMediaStreamSourceSwitchStreamsRequestedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Request() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaStreamSourceSwitchStreamsRequestedEventArgs>, base.5, Request::<Impl, OFFSET>)
    }
}
pub trait IMediaTrackImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TrackKind(&self) -> ::windows::core::Result<MediaTrackKind>;
    fn SetLabel(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Label(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for IMediaTrack {
    const NAME: &'static str = "Windows.Media.Core.IMediaTrack";
}
impl IMediaTrackVtbl {
    pub const fn new<Impl: IMediaTrackImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaTrackVtbl {
        unsafe extern "system" fn Id<Impl: IMediaTrackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Language<Impl: IMediaTrackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Language() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrackKind<Impl: IMediaTrackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut MediaTrackKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TrackKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLabel<Impl: IMediaTrackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetLabel(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Label<Impl: IMediaTrackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Label() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaTrack>, base.5, Id::<Impl, OFFSET>, Language::<Impl, OFFSET>, TrackKind::<Impl, OFFSET>, SetLabel::<Impl, OFFSET>, Label::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMseSourceBufferImpl: Sized {
    fn UpdateStarting(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MseSourceBuffer, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveUpdateStarting(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Updated(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MseSourceBuffer, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveUpdated(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn UpdateEnded(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MseSourceBuffer, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveUpdateEnded(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ErrorOccurred(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MseSourceBuffer, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveErrorOccurred(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Aborted(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MseSourceBuffer, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAborted(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Mode(&self) -> ::windows::core::Result<MseAppendMode>;
    fn SetMode(&self, value: MseAppendMode) -> ::windows::core::Result<()>;
    fn IsUpdating(&self) -> ::windows::core::Result<bool>;
    fn Buffered(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MseTimeRange>>;
    fn TimestampOffset(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetTimestampOffset(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn AppendWindowStart(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetAppendWindowStart(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn AppendWindowEnd(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>;
    fn SetAppendWindowEnd(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>) -> ::windows::core::Result<()>;
    fn AppendBuffer(&self, buffer: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
    fn AppendStream(&self, stream: &::core::option::Option<super::super::Storage::Streams::IInputStream>) -> ::windows::core::Result<()>;
    fn AppendStreamMaxSize(&self, stream: &::core::option::Option<super::super::Storage::Streams::IInputStream>, maxsize: u64) -> ::windows::core::Result<()>;
    fn Abort(&self) -> ::windows::core::Result<()>;
    fn Remove(&self, start: &super::super::Foundation::TimeSpan, end: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMseSourceBuffer {
    const NAME: &'static str = "Windows.Media.Core.IMseSourceBuffer";
}
#[cfg(feature = "implement_exclusive")]
impl IMseSourceBufferVtbl {
    pub const fn new<Impl: IMseSourceBufferImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMseSourceBufferVtbl {
        unsafe extern "system" fn UpdateStarting<Impl: IMseSourceBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UpdateStarting(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MseSourceBuffer, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MseSourceBuffer, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveUpdateStarting<Impl: IMseSourceBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveUpdateStarting(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Updated<Impl: IMseSourceBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Updated(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MseSourceBuffer, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MseSourceBuffer, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveUpdated<Impl: IMseSourceBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveUpdated(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn UpdateEnded<Impl: IMseSourceBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UpdateEnded(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MseSourceBuffer, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MseSourceBuffer, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveUpdateEnded<Impl: IMseSourceBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveUpdateEnded(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ErrorOccurred<Impl: IMseSourceBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ErrorOccurred(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MseSourceBuffer, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MseSourceBuffer, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveErrorOccurred<Impl: IMseSourceBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveErrorOccurred(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Aborted<Impl: IMseSourceBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Aborted(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MseSourceBuffer, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MseSourceBuffer, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAborted<Impl: IMseSourceBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveAborted(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Mode<Impl: IMseSourceBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut MseAppendMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Mode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMode<Impl: IMseSourceBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: MseAppendMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetMode(value).into()
        }
        unsafe extern "system" fn IsUpdating<Impl: IMseSourceBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsUpdating() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Buffered<Impl: IMseSourceBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Buffered() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TimestampOffset<Impl: IMseSourceBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TimestampOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTimestampOffset<Impl: IMseSourceBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTimestampOffset(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AppendWindowStart<Impl: IMseSourceBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AppendWindowStart() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAppendWindowStart<Impl: IMseSourceBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAppendWindowStart(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AppendWindowEnd<Impl: IMseSourceBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AppendWindowEnd() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAppendWindowEnd<Impl: IMseSourceBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAppendWindowEnd(&*(&value as *const <super::super::Foundation::IReference<super::super::Foundation::TimeSpan> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::super::Foundation::TimeSpan> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AppendBuffer<Impl: IMseSourceBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, buffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).AppendBuffer(&*(&buffer as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AppendStream<Impl: IMseSourceBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).AppendStream(&*(&stream as *const <super::super::Storage::Streams::IInputStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IInputStream as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AppendStreamMaxSize<Impl: IMseSourceBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, maxsize: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).AppendStreamMaxSize(&*(&stream as *const <super::super::Storage::Streams::IInputStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IInputStream as ::windows::core::DefaultType>::DefaultType), maxsize).into()
        }
        unsafe extern "system" fn Abort<Impl: IMseSourceBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Abort().into()
        }
        unsafe extern "system" fn Remove<Impl: IMseSourceBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, start: super::super::Foundation::TimeSpan, end: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Remove(&*(&start as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType), &*(&end as *const <super::super::Foundation::IReference<super::super::Foundation::TimeSpan> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::super::Foundation::TimeSpan> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IMseSourceBuffer>,
            base.5,
            UpdateStarting::<Impl, OFFSET>,
            RemoveUpdateStarting::<Impl, OFFSET>,
            Updated::<Impl, OFFSET>,
            RemoveUpdated::<Impl, OFFSET>,
            UpdateEnded::<Impl, OFFSET>,
            RemoveUpdateEnded::<Impl, OFFSET>,
            ErrorOccurred::<Impl, OFFSET>,
            RemoveErrorOccurred::<Impl, OFFSET>,
            Aborted::<Impl, OFFSET>,
            RemoveAborted::<Impl, OFFSET>,
            Mode::<Impl, OFFSET>,
            SetMode::<Impl, OFFSET>,
            IsUpdating::<Impl, OFFSET>,
            Buffered::<Impl, OFFSET>,
            TimestampOffset::<Impl, OFFSET>,
            SetTimestampOffset::<Impl, OFFSET>,
            AppendWindowStart::<Impl, OFFSET>,
            SetAppendWindowStart::<Impl, OFFSET>,
            AppendWindowEnd::<Impl, OFFSET>,
            SetAppendWindowEnd::<Impl, OFFSET>,
            AppendBuffer::<Impl, OFFSET>,
            AppendStream::<Impl, OFFSET>,
            AppendStreamMaxSize::<Impl, OFFSET>,
            Abort::<Impl, OFFSET>,
            Remove::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMseSourceBufferListImpl: Sized {
    fn SourceBufferAdded(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MseSourceBufferList, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSourceBufferAdded(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SourceBufferRemoved(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MseSourceBufferList, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSourceBufferRemoved(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Buffers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MseSourceBuffer>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMseSourceBufferList {
    const NAME: &'static str = "Windows.Media.Core.IMseSourceBufferList";
}
#[cfg(feature = "implement_exclusive")]
impl IMseSourceBufferListVtbl {
    pub const fn new<Impl: IMseSourceBufferListImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMseSourceBufferListVtbl {
        unsafe extern "system" fn SourceBufferAdded<Impl: IMseSourceBufferListImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SourceBufferAdded(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MseSourceBufferList, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MseSourceBufferList, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSourceBufferAdded<Impl: IMseSourceBufferListImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveSourceBufferAdded(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SourceBufferRemoved<Impl: IMseSourceBufferListImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SourceBufferRemoved(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MseSourceBufferList, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MseSourceBufferList, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSourceBufferRemoved<Impl: IMseSourceBufferListImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveSourceBufferRemoved(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Buffers<Impl: IMseSourceBufferListImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Buffers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMseSourceBufferList>, base.5, SourceBufferAdded::<Impl, OFFSET>, RemoveSourceBufferAdded::<Impl, OFFSET>, SourceBufferRemoved::<Impl, OFFSET>, RemoveSourceBufferRemoved::<Impl, OFFSET>, Buffers::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMseStreamSourceImpl: Sized + IMediaSourceImpl {
    fn Opened(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MseStreamSource, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveOpened(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Ended(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MseStreamSource, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveEnded(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Closed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MseStreamSource, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveClosed(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SourceBuffers(&self) -> ::windows::core::Result<MseSourceBufferList>;
    fn ActiveSourceBuffers(&self) -> ::windows::core::Result<MseSourceBufferList>;
    fn ReadyState(&self) -> ::windows::core::Result<MseReadyState>;
    fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>;
    fn SetDuration(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>) -> ::windows::core::Result<()>;
    fn AddSourceBuffer(&self, mimetype: &::windows::core::HSTRING) -> ::windows::core::Result<MseSourceBuffer>;
    fn RemoveSourceBuffer(&self, buffer: &::core::option::Option<MseSourceBuffer>) -> ::windows::core::Result<()>;
    fn EndOfStream(&self, status: MseEndOfStreamStatus) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMseStreamSource {
    const NAME: &'static str = "Windows.Media.Core.IMseStreamSource";
}
#[cfg(feature = "implement_exclusive")]
impl IMseStreamSourceVtbl {
    pub const fn new<Impl: IMseStreamSourceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMseStreamSourceVtbl {
        unsafe extern "system" fn Opened<Impl: IMseStreamSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Opened(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MseStreamSource, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MseStreamSource, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveOpened<Impl: IMseStreamSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveOpened(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Ended<Impl: IMseStreamSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Ended(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MseStreamSource, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MseStreamSource, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveEnded<Impl: IMseStreamSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveEnded(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Closed<Impl: IMseStreamSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Closed(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MseStreamSource, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MseStreamSource, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveClosed<Impl: IMseStreamSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveClosed(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SourceBuffers<Impl: IMseStreamSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SourceBuffers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActiveSourceBuffers<Impl: IMseStreamSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ActiveSourceBuffers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadyState<Impl: IMseStreamSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut MseReadyState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReadyState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Duration<Impl: IMseStreamSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDuration<Impl: IMseStreamSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDuration(&*(&value as *const <super::super::Foundation::IReference<super::super::Foundation::TimeSpan> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::super::Foundation::TimeSpan> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AddSourceBuffer<Impl: IMseStreamSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mimetype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddSourceBuffer(&*(&mimetype as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSourceBuffer<Impl: IMseStreamSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, buffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveSourceBuffer(&*(&buffer as *const <MseSourceBuffer as ::windows::core::Abi>::Abi as *const <MseSourceBuffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EndOfStream<Impl: IMseStreamSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, status: MseEndOfStreamStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).EndOfStream(status).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IMseStreamSource>,
            base.5,
            Opened::<Impl, OFFSET>,
            RemoveOpened::<Impl, OFFSET>,
            Ended::<Impl, OFFSET>,
            RemoveEnded::<Impl, OFFSET>,
            Closed::<Impl, OFFSET>,
            RemoveClosed::<Impl, OFFSET>,
            SourceBuffers::<Impl, OFFSET>,
            ActiveSourceBuffers::<Impl, OFFSET>,
            ReadyState::<Impl, OFFSET>,
            Duration::<Impl, OFFSET>,
            SetDuration::<Impl, OFFSET>,
            AddSourceBuffer::<Impl, OFFSET>,
            RemoveSourceBuffer::<Impl, OFFSET>,
            EndOfStream::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMseStreamSource2Impl: Sized {
    fn LiveSeekableRange(&self) -> ::windows::core::Result<super::super::Foundation::IReference<MseTimeRange>>;
    fn SetLiveSeekableRange(&self, value: &::core::option::Option<super::super::Foundation::IReference<MseTimeRange>>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMseStreamSource2 {
    const NAME: &'static str = "Windows.Media.Core.IMseStreamSource2";
}
#[cfg(feature = "implement_exclusive")]
impl IMseStreamSource2Vtbl {
    pub const fn new<Impl: IMseStreamSource2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMseStreamSource2Vtbl {
        unsafe extern "system" fn LiveSeekableRange<Impl: IMseStreamSource2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LiveSeekableRange() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLiveSeekableRange<Impl: IMseStreamSource2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetLiveSeekableRange(&*(&value as *const <super::super::Foundation::IReference<MseTimeRange> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<MseTimeRange> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMseStreamSource2>, base.5, LiveSeekableRange::<Impl, OFFSET>, SetLiveSeekableRange::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMseStreamSourceStaticsImpl: Sized {
    fn IsContentTypeSupported(&self, contenttype: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMseStreamSourceStatics {
    const NAME: &'static str = "Windows.Media.Core.IMseStreamSourceStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IMseStreamSourceStaticsVtbl {
    pub const fn new<Impl: IMseStreamSourceStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMseStreamSourceStaticsVtbl {
        unsafe extern "system" fn IsContentTypeSupported<Impl: IMseStreamSourceStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, contenttype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsContentTypeSupported(&*(&contenttype as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMseStreamSourceStatics>, base.5, IsContentTypeSupported::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISceneAnalysisEffectImpl: Sized + IMediaExtensionImpl {
    fn HighDynamicRangeAnalyzer(&self) -> ::windows::core::Result<HighDynamicRangeControl>;
    fn SetDesiredAnalysisInterval(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn DesiredAnalysisInterval(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SceneAnalyzed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<SceneAnalysisEffect, SceneAnalyzedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSceneAnalyzed(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISceneAnalysisEffect {
    const NAME: &'static str = "Windows.Media.Core.ISceneAnalysisEffect";
}
#[cfg(feature = "implement_exclusive")]
impl ISceneAnalysisEffectVtbl {
    pub const fn new<Impl: ISceneAnalysisEffectImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISceneAnalysisEffectVtbl {
        unsafe extern "system" fn HighDynamicRangeAnalyzer<Impl: ISceneAnalysisEffectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HighDynamicRangeAnalyzer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredAnalysisInterval<Impl: ISceneAnalysisEffectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDesiredAnalysisInterval(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DesiredAnalysisInterval<Impl: ISceneAnalysisEffectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DesiredAnalysisInterval() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SceneAnalyzed<Impl: ISceneAnalysisEffectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SceneAnalyzed(&*(&handler as *const <super::super::Foundation::TypedEventHandler<SceneAnalysisEffect, SceneAnalyzedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<SceneAnalysisEffect, SceneAnalyzedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSceneAnalyzed<Impl: ISceneAnalysisEffectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveSceneAnalyzed(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISceneAnalysisEffect>, base.5, HighDynamicRangeAnalyzer::<Impl, OFFSET>, SetDesiredAnalysisInterval::<Impl, OFFSET>, DesiredAnalysisInterval::<Impl, OFFSET>, SceneAnalyzed::<Impl, OFFSET>, RemoveSceneAnalyzed::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ISceneAnalysisEffectFrameImpl: Sized + IClosableImpl + IMediaFrameImpl {
    fn FrameControlValues(&self) -> ::windows::core::Result<super::Capture::CapturedFrameControlValues>;
    fn HighDynamicRange(&self) -> ::windows::core::Result<HighDynamicRangeOutput>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISceneAnalysisEffectFrame {
    const NAME: &'static str = "Windows.Media.Core.ISceneAnalysisEffectFrame";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ISceneAnalysisEffectFrameVtbl {
    pub const fn new<Impl: ISceneAnalysisEffectFrameImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISceneAnalysisEffectFrameVtbl {
        unsafe extern "system" fn FrameControlValues<Impl: ISceneAnalysisEffectFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FrameControlValues() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HighDynamicRange<Impl: ISceneAnalysisEffectFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HighDynamicRange() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISceneAnalysisEffectFrame>, base.5, FrameControlValues::<Impl, OFFSET>, HighDynamicRange::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ISceneAnalysisEffectFrame2Impl: Sized + IClosableImpl + IMediaFrameImpl {
    fn AnalysisRecommendation(&self) -> ::windows::core::Result<SceneAnalysisRecommendation>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISceneAnalysisEffectFrame2 {
    const NAME: &'static str = "Windows.Media.Core.ISceneAnalysisEffectFrame2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ISceneAnalysisEffectFrame2Vtbl {
    pub const fn new<Impl: ISceneAnalysisEffectFrame2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISceneAnalysisEffectFrame2Vtbl {
        unsafe extern "system" fn AnalysisRecommendation<Impl: ISceneAnalysisEffectFrame2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut SceneAnalysisRecommendation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AnalysisRecommendation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISceneAnalysisEffectFrame2>, base.5, AnalysisRecommendation::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISceneAnalyzedEventArgsImpl: Sized {
    fn ResultFrame(&self) -> ::windows::core::Result<SceneAnalysisEffectFrame>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISceneAnalyzedEventArgs {
    const NAME: &'static str = "Windows.Media.Core.ISceneAnalyzedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ISceneAnalyzedEventArgsVtbl {
    pub const fn new<Impl: ISceneAnalyzedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISceneAnalyzedEventArgsVtbl {
        unsafe extern "system" fn ResultFrame<Impl: ISceneAnalyzedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ResultFrame() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISceneAnalyzedEventArgs>, base.5, ResultFrame::<Impl, OFFSET>)
    }
}
pub trait ISingleSelectMediaTrackListImpl: Sized {
    fn SelectedIndexChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ISingleSelectMediaTrackList, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSelectedIndexChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SetSelectedIndex(&self, value: i32) -> ::windows::core::Result<()>;
    fn SelectedIndex(&self) -> ::windows::core::Result<i32>;
}
impl ::windows::core::RuntimeName for ISingleSelectMediaTrackList {
    const NAME: &'static str = "Windows.Media.Core.ISingleSelectMediaTrackList";
}
impl ISingleSelectMediaTrackListVtbl {
    pub const fn new<Impl: ISingleSelectMediaTrackListImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISingleSelectMediaTrackListVtbl {
        unsafe extern "system" fn SelectedIndexChanged<Impl: ISingleSelectMediaTrackListImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SelectedIndexChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<ISingleSelectMediaTrackList, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<ISingleSelectMediaTrackList, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSelectedIndexChanged<Impl: ISingleSelectMediaTrackListImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveSelectedIndexChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetSelectedIndex<Impl: ISingleSelectMediaTrackListImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSelectedIndex(value).into()
        }
        unsafe extern "system" fn SelectedIndex<Impl: ISingleSelectMediaTrackListImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SelectedIndex() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISingleSelectMediaTrackList>, base.5, SelectedIndexChanged::<Impl, OFFSET>, RemoveSelectedIndexChanged::<Impl, OFFSET>, SetSelectedIndex::<Impl, OFFSET>, SelectedIndex::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpeechCueImpl: Sized + IMediaCueImpl {
    fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn StartPositionInInput(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn SetStartPositionInInput(&self, value: &::core::option::Option<super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
    fn EndPositionInInput(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn SetEndPositionInInput(&self, value: &::core::option::Option<super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpeechCue {
    const NAME: &'static str = "Windows.Media.Core.ISpeechCue";
}
#[cfg(feature = "implement_exclusive")]
impl ISpeechCueVtbl {
    pub const fn new<Impl: ISpeechCueImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpeechCueVtbl {
        unsafe extern "system" fn Text<Impl: ISpeechCueImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Text() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetText<Impl: ISpeechCueImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetText(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StartPositionInInput<Impl: ISpeechCueImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StartPositionInInput() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartPositionInInput<Impl: ISpeechCueImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetStartPositionInInput(&*(&value as *const <super::super::Foundation::IReference<i32> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<i32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EndPositionInInput<Impl: ISpeechCueImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EndPositionInInput() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEndPositionInInput<Impl: ISpeechCueImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetEndPositionInInput(&*(&value as *const <super::super::Foundation::IReference<i32> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<i32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpeechCue>, base.5, Text::<Impl, OFFSET>, SetText::<Impl, OFFSET>, StartPositionInInput::<Impl, OFFSET>, SetStartPositionInInput::<Impl, OFFSET>, EndPositionInInput::<Impl, OFFSET>, SetEndPositionInInput::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimedMetadataStreamDescriptorImpl: Sized {
    fn EncodingProperties(&self) -> ::windows::core::Result<super::MediaProperties::TimedMetadataEncodingProperties>;
    fn Copy(&self) -> ::windows::core::Result<TimedMetadataStreamDescriptor>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITimedMetadataStreamDescriptor {
    const NAME: &'static str = "Windows.Media.Core.ITimedMetadataStreamDescriptor";
}
#[cfg(feature = "implement_exclusive")]
impl ITimedMetadataStreamDescriptorVtbl {
    pub const fn new<Impl: ITimedMetadataStreamDescriptorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITimedMetadataStreamDescriptorVtbl {
        unsafe extern "system" fn EncodingProperties<Impl: ITimedMetadataStreamDescriptorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EncodingProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Copy<Impl: ITimedMetadataStreamDescriptorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Copy() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITimedMetadataStreamDescriptor>, base.5, EncodingProperties::<Impl, OFFSET>, Copy::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimedMetadataStreamDescriptorFactoryImpl: Sized {
    fn Create(&self, encodingproperties: &::core::option::Option<super::MediaProperties::TimedMetadataEncodingProperties>) -> ::windows::core::Result<TimedMetadataStreamDescriptor>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITimedMetadataStreamDescriptorFactory {
    const NAME: &'static str = "Windows.Media.Core.ITimedMetadataStreamDescriptorFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ITimedMetadataStreamDescriptorFactoryVtbl {
    pub const fn new<Impl: ITimedMetadataStreamDescriptorFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITimedMetadataStreamDescriptorFactoryVtbl {
        unsafe extern "system" fn Create<Impl: ITimedMetadataStreamDescriptorFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, encodingproperties: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(&*(&encodingproperties as *const <super::MediaProperties::TimedMetadataEncodingProperties as ::windows::core::Abi>::Abi as *const <super::MediaProperties::TimedMetadataEncodingProperties as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITimedMetadataStreamDescriptorFactory>, base.5, Create::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimedMetadataTrackImpl: Sized + IMediaTrackImpl {
    fn CueEntered(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<TimedMetadataTrack, MediaCueEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCueEntered(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CueExited(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<TimedMetadataTrack, MediaCueEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCueExited(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn TrackFailed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<TimedMetadataTrack, TimedMetadataTrackFailedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveTrackFailed(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Cues(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<IMediaCue>>;
    fn ActiveCues(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<IMediaCue>>;
    fn TimedMetadataKind(&self) -> ::windows::core::Result<TimedMetadataKind>;
    fn DispatchType(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AddCue(&self, cue: &::core::option::Option<IMediaCue>) -> ::windows::core::Result<()>;
    fn RemoveCue(&self, cue: &::core::option::Option<IMediaCue>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITimedMetadataTrack {
    const NAME: &'static str = "Windows.Media.Core.ITimedMetadataTrack";
}
#[cfg(feature = "implement_exclusive")]
impl ITimedMetadataTrackVtbl {
    pub const fn new<Impl: ITimedMetadataTrackImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITimedMetadataTrackVtbl {
        unsafe extern "system" fn CueEntered<Impl: ITimedMetadataTrackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CueEntered(&*(&handler as *const <super::super::Foundation::TypedEventHandler<TimedMetadataTrack, MediaCueEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<TimedMetadataTrack, MediaCueEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCueEntered<Impl: ITimedMetadataTrackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveCueEntered(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CueExited<Impl: ITimedMetadataTrackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CueExited(&*(&handler as *const <super::super::Foundation::TypedEventHandler<TimedMetadataTrack, MediaCueEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<TimedMetadataTrack, MediaCueEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCueExited<Impl: ITimedMetadataTrackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveCueExited(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TrackFailed<Impl: ITimedMetadataTrackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TrackFailed(&*(&handler as *const <super::super::Foundation::TypedEventHandler<TimedMetadataTrack, TimedMetadataTrackFailedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<TimedMetadataTrack, TimedMetadataTrackFailedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveTrackFailed<Impl: ITimedMetadataTrackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveTrackFailed(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Cues<Impl: ITimedMetadataTrackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Cues() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActiveCues<Impl: ITimedMetadataTrackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ActiveCues() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TimedMetadataKind<Impl: ITimedMetadataTrackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut TimedMetadataKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TimedMetadataKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DispatchType<Impl: ITimedMetadataTrackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DispatchType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddCue<Impl: ITimedMetadataTrackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cue: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).AddCue(&*(&cue as *const <IMediaCue as ::windows::core::Abi>::Abi as *const <IMediaCue as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RemoveCue<Impl: ITimedMetadataTrackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cue: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveCue(&*(&cue as *const <IMediaCue as ::windows::core::Abi>::Abi as *const <IMediaCue as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITimedMetadataTrack>, base.5, CueEntered::<Impl, OFFSET>, RemoveCueEntered::<Impl, OFFSET>, CueExited::<Impl, OFFSET>, RemoveCueExited::<Impl, OFFSET>, TrackFailed::<Impl, OFFSET>, RemoveTrackFailed::<Impl, OFFSET>, Cues::<Impl, OFFSET>, ActiveCues::<Impl, OFFSET>, TimedMetadataKind::<Impl, OFFSET>, DispatchType::<Impl, OFFSET>, AddCue::<Impl, OFFSET>, RemoveCue::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimedMetadataTrack2Impl: Sized + IMediaTrackImpl + ITimedMetadataTrackImpl {
    fn PlaybackItem(&self) -> ::windows::core::Result<super::Playback::MediaPlaybackItem>;
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITimedMetadataTrack2 {
    const NAME: &'static str = "Windows.Media.Core.ITimedMetadataTrack2";
}
#[cfg(feature = "implement_exclusive")]
impl ITimedMetadataTrack2Vtbl {
    pub const fn new<Impl: ITimedMetadataTrack2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITimedMetadataTrack2Vtbl {
        unsafe extern "system" fn PlaybackItem<Impl: ITimedMetadataTrack2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PlaybackItem() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: ITimedMetadataTrack2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITimedMetadataTrack2>, base.5, PlaybackItem::<Impl, OFFSET>, Name::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimedMetadataTrackErrorImpl: Sized {
    fn ErrorCode(&self) -> ::windows::core::Result<TimedMetadataTrackErrorCode>;
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITimedMetadataTrackError {
    const NAME: &'static str = "Windows.Media.Core.ITimedMetadataTrackError";
}
#[cfg(feature = "implement_exclusive")]
impl ITimedMetadataTrackErrorVtbl {
    pub const fn new<Impl: ITimedMetadataTrackErrorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITimedMetadataTrackErrorVtbl {
        unsafe extern "system" fn ErrorCode<Impl: ITimedMetadataTrackErrorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut TimedMetadataTrackErrorCode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ErrorCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedError<Impl: ITimedMetadataTrackErrorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExtendedError() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITimedMetadataTrackError>, base.5, ErrorCode::<Impl, OFFSET>, ExtendedError::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimedMetadataTrackFactoryImpl: Sized {
    fn Create(&self, id: &::windows::core::HSTRING, language: &::windows::core::HSTRING, kind: TimedMetadataKind) -> ::windows::core::Result<TimedMetadataTrack>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITimedMetadataTrackFactory {
    const NAME: &'static str = "Windows.Media.Core.ITimedMetadataTrackFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ITimedMetadataTrackFactoryVtbl {
    pub const fn new<Impl: ITimedMetadataTrackFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITimedMetadataTrackFactoryVtbl {
        unsafe extern "system" fn Create<Impl: ITimedMetadataTrackFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, language: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, kind: TimedMetadataKind, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(&*(&id as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&language as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), kind) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITimedMetadataTrackFactory>, base.5, Create::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimedMetadataTrackFailedEventArgsImpl: Sized {
    fn Error(&self) -> ::windows::core::Result<TimedMetadataTrackError>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITimedMetadataTrackFailedEventArgs {
    const NAME: &'static str = "Windows.Media.Core.ITimedMetadataTrackFailedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ITimedMetadataTrackFailedEventArgsVtbl {
    pub const fn new<Impl: ITimedMetadataTrackFailedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITimedMetadataTrackFailedEventArgsVtbl {
        unsafe extern "system" fn Error<Impl: ITimedMetadataTrackFailedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Error() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITimedMetadataTrackFailedEventArgs>, base.5, Error::<Impl, OFFSET>)
    }
}
pub trait ITimedMetadataTrackProviderImpl: Sized {
    fn TimedMetadataTracks(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<TimedMetadataTrack>>;
}
impl ::windows::core::RuntimeName for ITimedMetadataTrackProvider {
    const NAME: &'static str = "Windows.Media.Core.ITimedMetadataTrackProvider";
}
impl ITimedMetadataTrackProviderVtbl {
    pub const fn new<Impl: ITimedMetadataTrackProviderImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITimedMetadataTrackProviderVtbl {
        unsafe extern "system" fn TimedMetadataTracks<Impl: ITimedMetadataTrackProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TimedMetadataTracks() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITimedMetadataTrackProvider>, base.5, TimedMetadataTracks::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimedTextBoutenImpl: Sized {
    fn Type(&self) -> ::windows::core::Result<TimedTextBoutenType>;
    fn SetType(&self, value: TimedTextBoutenType) -> ::windows::core::Result<()>;
    fn Color(&self) -> ::windows::core::Result<super::super::UI::Color>;
    fn SetColor(&self, value: &super::super::UI::Color) -> ::windows::core::Result<()>;
    fn Position(&self) -> ::windows::core::Result<TimedTextBoutenPosition>;
    fn SetPosition(&self, value: TimedTextBoutenPosition) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITimedTextBouten {
    const NAME: &'static str = "Windows.Media.Core.ITimedTextBouten";
}
#[cfg(feature = "implement_exclusive")]
impl ITimedTextBoutenVtbl {
    pub const fn new<Impl: ITimedTextBoutenImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITimedTextBoutenVtbl {
        unsafe extern "system" fn Type<Impl: ITimedTextBoutenImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut TimedTextBoutenType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetType<Impl: ITimedTextBoutenImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: TimedTextBoutenType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetType(value).into()
        }
        unsafe extern "system" fn Color<Impl: ITimedTextBoutenImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Color() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColor<Impl: ITimedTextBoutenImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::UI::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetColor(&*(&value as *const <super::super::UI::Color as ::windows::core::Abi>::Abi as *const <super::super::UI::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Position<Impl: ITimedTextBoutenImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut TimedTextBoutenPosition) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Position() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPosition<Impl: ITimedTextBoutenImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: TimedTextBoutenPosition) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPosition(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITimedTextBouten>, base.5, Type::<Impl, OFFSET>, SetType::<Impl, OFFSET>, Color::<Impl, OFFSET>, SetColor::<Impl, OFFSET>, Position::<Impl, OFFSET>, SetPosition::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimedTextCueImpl: Sized + IMediaCueImpl {
    fn CueRegion(&self) -> ::windows::core::Result<TimedTextRegion>;
    fn SetCueRegion(&self, value: &::core::option::Option<TimedTextRegion>) -> ::windows::core::Result<()>;
    fn CueStyle(&self) -> ::windows::core::Result<TimedTextStyle>;
    fn SetCueStyle(&self, value: &::core::option::Option<TimedTextStyle>) -> ::windows::core::Result<()>;
    fn Lines(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<TimedTextLine>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITimedTextCue {
    const NAME: &'static str = "Windows.Media.Core.ITimedTextCue";
}
#[cfg(feature = "implement_exclusive")]
impl ITimedTextCueVtbl {
    pub const fn new<Impl: ITimedTextCueImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITimedTextCueVtbl {
        unsafe extern "system" fn CueRegion<Impl: ITimedTextCueImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CueRegion() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCueRegion<Impl: ITimedTextCueImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCueRegion(&*(&value as *const <TimedTextRegion as ::windows::core::Abi>::Abi as *const <TimedTextRegion as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CueStyle<Impl: ITimedTextCueImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CueStyle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCueStyle<Impl: ITimedTextCueImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCueStyle(&*(&value as *const <TimedTextStyle as ::windows::core::Abi>::Abi as *const <TimedTextStyle as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Lines<Impl: ITimedTextCueImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Lines() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITimedTextCue>, base.5, CueRegion::<Impl, OFFSET>, SetCueRegion::<Impl, OFFSET>, CueStyle::<Impl, OFFSET>, SetCueStyle::<Impl, OFFSET>, Lines::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimedTextLineImpl: Sized {
    fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Subformats(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<TimedTextSubformat>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITimedTextLine {
    const NAME: &'static str = "Windows.Media.Core.ITimedTextLine";
}
#[cfg(feature = "implement_exclusive")]
impl ITimedTextLineVtbl {
    pub const fn new<Impl: ITimedTextLineImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITimedTextLineVtbl {
        unsafe extern "system" fn Text<Impl: ITimedTextLineImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Text() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetText<Impl: ITimedTextLineImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetText(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Subformats<Impl: ITimedTextLineImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Subformats() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITimedTextLine>, base.5, Text::<Impl, OFFSET>, SetText::<Impl, OFFSET>, Subformats::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimedTextRegionImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Position(&self) -> ::windows::core::Result<TimedTextPoint>;
    fn SetPosition(&self, value: &TimedTextPoint) -> ::windows::core::Result<()>;
    fn Extent(&self) -> ::windows::core::Result<TimedTextSize>;
    fn SetExtent(&self, value: &TimedTextSize) -> ::windows::core::Result<()>;
    fn Background(&self) -> ::windows::core::Result<super::super::UI::Color>;
    fn SetBackground(&self, value: &super::super::UI::Color) -> ::windows::core::Result<()>;
    fn WritingMode(&self) -> ::windows::core::Result<TimedTextWritingMode>;
    fn SetWritingMode(&self, value: TimedTextWritingMode) -> ::windows::core::Result<()>;
    fn DisplayAlignment(&self) -> ::windows::core::Result<TimedTextDisplayAlignment>;
    fn SetDisplayAlignment(&self, value: TimedTextDisplayAlignment) -> ::windows::core::Result<()>;
    fn LineHeight(&self) -> ::windows::core::Result<TimedTextDouble>;
    fn SetLineHeight(&self, value: &TimedTextDouble) -> ::windows::core::Result<()>;
    fn IsOverflowClipped(&self) -> ::windows::core::Result<bool>;
    fn SetIsOverflowClipped(&self, value: bool) -> ::windows::core::Result<()>;
    fn Padding(&self) -> ::windows::core::Result<TimedTextPadding>;
    fn SetPadding(&self, value: &TimedTextPadding) -> ::windows::core::Result<()>;
    fn TextWrapping(&self) -> ::windows::core::Result<TimedTextWrapping>;
    fn SetTextWrapping(&self, value: TimedTextWrapping) -> ::windows::core::Result<()>;
    fn ZIndex(&self) -> ::windows::core::Result<i32>;
    fn SetZIndex(&self, value: i32) -> ::windows::core::Result<()>;
    fn ScrollMode(&self) -> ::windows::core::Result<TimedTextScrollMode>;
    fn SetScrollMode(&self, value: TimedTextScrollMode) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITimedTextRegion {
    const NAME: &'static str = "Windows.Media.Core.ITimedTextRegion";
}
#[cfg(feature = "implement_exclusive")]
impl ITimedTextRegionVtbl {
    pub const fn new<Impl: ITimedTextRegionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITimedTextRegionVtbl {
        unsafe extern "system" fn Name<Impl: ITimedTextRegionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: ITimedTextRegionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Position<Impl: ITimedTextRegionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut TimedTextPoint) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Position() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPosition<Impl: ITimedTextRegionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: TimedTextPoint) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPosition(&*(&value as *const <TimedTextPoint as ::windows::core::Abi>::Abi as *const <TimedTextPoint as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Extent<Impl: ITimedTextRegionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut TimedTextSize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Extent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExtent<Impl: ITimedTextRegionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: TimedTextSize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetExtent(&*(&value as *const <TimedTextSize as ::windows::core::Abi>::Abi as *const <TimedTextSize as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Background<Impl: ITimedTextRegionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Background() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBackground<Impl: ITimedTextRegionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::UI::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetBackground(&*(&value as *const <super::super::UI::Color as ::windows::core::Abi>::Abi as *const <super::super::UI::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn WritingMode<Impl: ITimedTextRegionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut TimedTextWritingMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WritingMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWritingMode<Impl: ITimedTextRegionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: TimedTextWritingMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetWritingMode(value).into()
        }
        unsafe extern "system" fn DisplayAlignment<Impl: ITimedTextRegionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut TimedTextDisplayAlignment) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DisplayAlignment() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayAlignment<Impl: ITimedTextRegionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: TimedTextDisplayAlignment) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDisplayAlignment(value).into()
        }
        unsafe extern "system" fn LineHeight<Impl: ITimedTextRegionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut TimedTextDouble) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LineHeight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLineHeight<Impl: ITimedTextRegionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: TimedTextDouble) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetLineHeight(&*(&value as *const <TimedTextDouble as ::windows::core::Abi>::Abi as *const <TimedTextDouble as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsOverflowClipped<Impl: ITimedTextRegionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsOverflowClipped() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsOverflowClipped<Impl: ITimedTextRegionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsOverflowClipped(value).into()
        }
        unsafe extern "system" fn Padding<Impl: ITimedTextRegionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut TimedTextPadding) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Padding() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPadding<Impl: ITimedTextRegionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: TimedTextPadding) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPadding(&*(&value as *const <TimedTextPadding as ::windows::core::Abi>::Abi as *const <TimedTextPadding as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TextWrapping<Impl: ITimedTextRegionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut TimedTextWrapping) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TextWrapping() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTextWrapping<Impl: ITimedTextRegionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: TimedTextWrapping) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTextWrapping(value).into()
        }
        unsafe extern "system" fn ZIndex<Impl: ITimedTextRegionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ZIndex() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetZIndex<Impl: ITimedTextRegionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetZIndex(value).into()
        }
        unsafe extern "system" fn ScrollMode<Impl: ITimedTextRegionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut TimedTextScrollMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ScrollMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScrollMode<Impl: ITimedTextRegionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: TimedTextScrollMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetScrollMode(value).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<ITimedTextRegion>,
            base.5,
            Name::<Impl, OFFSET>,
            SetName::<Impl, OFFSET>,
            Position::<Impl, OFFSET>,
            SetPosition::<Impl, OFFSET>,
            Extent::<Impl, OFFSET>,
            SetExtent::<Impl, OFFSET>,
            Background::<Impl, OFFSET>,
            SetBackground::<Impl, OFFSET>,
            WritingMode::<Impl, OFFSET>,
            SetWritingMode::<Impl, OFFSET>,
            DisplayAlignment::<Impl, OFFSET>,
            SetDisplayAlignment::<Impl, OFFSET>,
            LineHeight::<Impl, OFFSET>,
            SetLineHeight::<Impl, OFFSET>,
            IsOverflowClipped::<Impl, OFFSET>,
            SetIsOverflowClipped::<Impl, OFFSET>,
            Padding::<Impl, OFFSET>,
            SetPadding::<Impl, OFFSET>,
            TextWrapping::<Impl, OFFSET>,
            SetTextWrapping::<Impl, OFFSET>,
            ZIndex::<Impl, OFFSET>,
            SetZIndex::<Impl, OFFSET>,
            ScrollMode::<Impl, OFFSET>,
            SetScrollMode::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimedTextRubyImpl: Sized {
    fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Position(&self) -> ::windows::core::Result<TimedTextRubyPosition>;
    fn SetPosition(&self, value: TimedTextRubyPosition) -> ::windows::core::Result<()>;
    fn Align(&self) -> ::windows::core::Result<TimedTextRubyAlign>;
    fn SetAlign(&self, value: TimedTextRubyAlign) -> ::windows::core::Result<()>;
    fn Reserve(&self) -> ::windows::core::Result<TimedTextRubyReserve>;
    fn SetReserve(&self, value: TimedTextRubyReserve) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITimedTextRuby {
    const NAME: &'static str = "Windows.Media.Core.ITimedTextRuby";
}
#[cfg(feature = "implement_exclusive")]
impl ITimedTextRubyVtbl {
    pub const fn new<Impl: ITimedTextRubyImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITimedTextRubyVtbl {
        unsafe extern "system" fn Text<Impl: ITimedTextRubyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Text() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetText<Impl: ITimedTextRubyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetText(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Position<Impl: ITimedTextRubyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut TimedTextRubyPosition) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Position() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPosition<Impl: ITimedTextRubyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: TimedTextRubyPosition) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPosition(value).into()
        }
        unsafe extern "system" fn Align<Impl: ITimedTextRubyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut TimedTextRubyAlign) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Align() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlign<Impl: ITimedTextRubyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: TimedTextRubyAlign) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAlign(value).into()
        }
        unsafe extern "system" fn Reserve<Impl: ITimedTextRubyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut TimedTextRubyReserve) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Reserve() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReserve<Impl: ITimedTextRubyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: TimedTextRubyReserve) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetReserve(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITimedTextRuby>, base.5, Text::<Impl, OFFSET>, SetText::<Impl, OFFSET>, Position::<Impl, OFFSET>, SetPosition::<Impl, OFFSET>, Align::<Impl, OFFSET>, SetAlign::<Impl, OFFSET>, Reserve::<Impl, OFFSET>, SetReserve::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimedTextSourceImpl: Sized {
    fn Resolved(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<TimedTextSource, TimedTextSourceResolveResultEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveResolved(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITimedTextSource {
    const NAME: &'static str = "Windows.Media.Core.ITimedTextSource";
}
#[cfg(feature = "implement_exclusive")]
impl ITimedTextSourceVtbl {
    pub const fn new<Impl: ITimedTextSourceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITimedTextSourceVtbl {
        unsafe extern "system" fn Resolved<Impl: ITimedTextSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Resolved(&*(&handler as *const <super::super::Foundation::TypedEventHandler<TimedTextSource, TimedTextSourceResolveResultEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<TimedTextSource, TimedTextSourceResolveResultEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveResolved<Impl: ITimedTextSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveResolved(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITimedTextSource>, base.5, Resolved::<Impl, OFFSET>, RemoveResolved::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimedTextSourceResolveResultEventArgsImpl: Sized {
    fn Error(&self) -> ::windows::core::Result<TimedMetadataTrackError>;
    fn Tracks(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<TimedMetadataTrack>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITimedTextSourceResolveResultEventArgs {
    const NAME: &'static str = "Windows.Media.Core.ITimedTextSourceResolveResultEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ITimedTextSourceResolveResultEventArgsVtbl {
    pub const fn new<Impl: ITimedTextSourceResolveResultEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITimedTextSourceResolveResultEventArgsVtbl {
        unsafe extern "system" fn Error<Impl: ITimedTextSourceResolveResultEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Error() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Tracks<Impl: ITimedTextSourceResolveResultEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Tracks() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITimedTextSourceResolveResultEventArgs>, base.5, Error::<Impl, OFFSET>, Tracks::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimedTextSourceStaticsImpl: Sized {
    fn CreateFromStream(&self, stream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<TimedTextSource>;
    fn CreateFromUri(&self, uri: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<TimedTextSource>;
    fn CreateFromStreamWithLanguage(&self, stream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>, defaultlanguage: &::windows::core::HSTRING) -> ::windows::core::Result<TimedTextSource>;
    fn CreateFromUriWithLanguage(&self, uri: &::core::option::Option<super::super::Foundation::Uri>, defaultlanguage: &::windows::core::HSTRING) -> ::windows::core::Result<TimedTextSource>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITimedTextSourceStatics {
    const NAME: &'static str = "Windows.Media.Core.ITimedTextSourceStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ITimedTextSourceStaticsVtbl {
    pub const fn new<Impl: ITimedTextSourceStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITimedTextSourceStaticsVtbl {
        unsafe extern "system" fn CreateFromStream<Impl: ITimedTextSourceStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateFromStream(&*(&stream as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromUri<Impl: ITimedTextSourceStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateFromUri(&*(&uri as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromStreamWithLanguage<Impl: ITimedTextSourceStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, defaultlanguage: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateFromStreamWithLanguage(&*(&stream as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::DefaultType>::DefaultType), &*(&defaultlanguage as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromUriWithLanguage<Impl: ITimedTextSourceStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, defaultlanguage: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateFromUriWithLanguage(&*(&uri as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType), &*(&defaultlanguage as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITimedTextSourceStatics>, base.5, CreateFromStream::<Impl, OFFSET>, CreateFromUri::<Impl, OFFSET>, CreateFromStreamWithLanguage::<Impl, OFFSET>, CreateFromUriWithLanguage::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimedTextSourceStatics2Impl: Sized {
    fn CreateFromStreamWithIndex(&self, stream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>, indexstream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<TimedTextSource>;
    fn CreateFromUriWithIndex(&self, uri: &::core::option::Option<super::super::Foundation::Uri>, indexuri: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<TimedTextSource>;
    fn CreateFromStreamWithIndexAndLanguage(&self, stream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>, indexstream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>, defaultlanguage: &::windows::core::HSTRING) -> ::windows::core::Result<TimedTextSource>;
    fn CreateFromUriWithIndexAndLanguage(&self, uri: &::core::option::Option<super::super::Foundation::Uri>, indexuri: &::core::option::Option<super::super::Foundation::Uri>, defaultlanguage: &::windows::core::HSTRING) -> ::windows::core::Result<TimedTextSource>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITimedTextSourceStatics2 {
    const NAME: &'static str = "Windows.Media.Core.ITimedTextSourceStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl ITimedTextSourceStatics2Vtbl {
    pub const fn new<Impl: ITimedTextSourceStatics2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITimedTextSourceStatics2Vtbl {
        unsafe extern "system" fn CreateFromStreamWithIndex<Impl: ITimedTextSourceStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, indexstream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateFromStreamWithIndex(&*(&stream as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::DefaultType>::DefaultType), &*(&indexstream as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromUriWithIndex<Impl: ITimedTextSourceStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, indexuri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateFromUriWithIndex(&*(&uri as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType), &*(&indexuri as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromStreamWithIndexAndLanguage<Impl: ITimedTextSourceStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, indexstream: ::windows::core::RawPtr, defaultlanguage: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
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
        unsafe extern "system" fn CreateFromUriWithIndexAndLanguage<Impl: ITimedTextSourceStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, indexuri: ::windows::core::RawPtr, defaultlanguage: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITimedTextSourceStatics2>, base.5, CreateFromStreamWithIndex::<Impl, OFFSET>, CreateFromUriWithIndex::<Impl, OFFSET>, CreateFromStreamWithIndexAndLanguage::<Impl, OFFSET>, CreateFromUriWithIndexAndLanguage::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimedTextStyleImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn FontFamily(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetFontFamily(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn FontSize(&self) -> ::windows::core::Result<TimedTextDouble>;
    fn SetFontSize(&self, value: &TimedTextDouble) -> ::windows::core::Result<()>;
    fn FontWeight(&self) -> ::windows::core::Result<TimedTextWeight>;
    fn SetFontWeight(&self, value: TimedTextWeight) -> ::windows::core::Result<()>;
    fn Foreground(&self) -> ::windows::core::Result<super::super::UI::Color>;
    fn SetForeground(&self, value: &super::super::UI::Color) -> ::windows::core::Result<()>;
    fn Background(&self) -> ::windows::core::Result<super::super::UI::Color>;
    fn SetBackground(&self, value: &super::super::UI::Color) -> ::windows::core::Result<()>;
    fn IsBackgroundAlwaysShown(&self) -> ::windows::core::Result<bool>;
    fn SetIsBackgroundAlwaysShown(&self, value: bool) -> ::windows::core::Result<()>;
    fn FlowDirection(&self) -> ::windows::core::Result<TimedTextFlowDirection>;
    fn SetFlowDirection(&self, value: TimedTextFlowDirection) -> ::windows::core::Result<()>;
    fn LineAlignment(&self) -> ::windows::core::Result<TimedTextLineAlignment>;
    fn SetLineAlignment(&self, value: TimedTextLineAlignment) -> ::windows::core::Result<()>;
    fn OutlineColor(&self) -> ::windows::core::Result<super::super::UI::Color>;
    fn SetOutlineColor(&self, value: &super::super::UI::Color) -> ::windows::core::Result<()>;
    fn OutlineThickness(&self) -> ::windows::core::Result<TimedTextDouble>;
    fn SetOutlineThickness(&self, value: &TimedTextDouble) -> ::windows::core::Result<()>;
    fn OutlineRadius(&self) -> ::windows::core::Result<TimedTextDouble>;
    fn SetOutlineRadius(&self, value: &TimedTextDouble) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITimedTextStyle {
    const NAME: &'static str = "Windows.Media.Core.ITimedTextStyle";
}
#[cfg(feature = "implement_exclusive")]
impl ITimedTextStyleVtbl {
    pub const fn new<Impl: ITimedTextStyleImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITimedTextStyleVtbl {
        unsafe extern "system" fn Name<Impl: ITimedTextStyleImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: ITimedTextStyleImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FontFamily<Impl: ITimedTextStyleImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FontFamily() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFontFamily<Impl: ITimedTextStyleImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetFontFamily(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FontSize<Impl: ITimedTextStyleImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut TimedTextDouble) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FontSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFontSize<Impl: ITimedTextStyleImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: TimedTextDouble) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetFontSize(&*(&value as *const <TimedTextDouble as ::windows::core::Abi>::Abi as *const <TimedTextDouble as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FontWeight<Impl: ITimedTextStyleImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut TimedTextWeight) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FontWeight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFontWeight<Impl: ITimedTextStyleImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: TimedTextWeight) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetFontWeight(value).into()
        }
        unsafe extern "system" fn Foreground<Impl: ITimedTextStyleImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Foreground() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetForeground<Impl: ITimedTextStyleImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::UI::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetForeground(&*(&value as *const <super::super::UI::Color as ::windows::core::Abi>::Abi as *const <super::super::UI::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Background<Impl: ITimedTextStyleImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Background() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBackground<Impl: ITimedTextStyleImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::UI::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetBackground(&*(&value as *const <super::super::UI::Color as ::windows::core::Abi>::Abi as *const <super::super::UI::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsBackgroundAlwaysShown<Impl: ITimedTextStyleImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsBackgroundAlwaysShown() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsBackgroundAlwaysShown<Impl: ITimedTextStyleImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsBackgroundAlwaysShown(value).into()
        }
        unsafe extern "system" fn FlowDirection<Impl: ITimedTextStyleImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut TimedTextFlowDirection) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FlowDirection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFlowDirection<Impl: ITimedTextStyleImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: TimedTextFlowDirection) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetFlowDirection(value).into()
        }
        unsafe extern "system" fn LineAlignment<Impl: ITimedTextStyleImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut TimedTextLineAlignment) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LineAlignment() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLineAlignment<Impl: ITimedTextStyleImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: TimedTextLineAlignment) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetLineAlignment(value).into()
        }
        unsafe extern "system" fn OutlineColor<Impl: ITimedTextStyleImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OutlineColor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutlineColor<Impl: ITimedTextStyleImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::UI::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetOutlineColor(&*(&value as *const <super::super::UI::Color as ::windows::core::Abi>::Abi as *const <super::super::UI::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OutlineThickness<Impl: ITimedTextStyleImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut TimedTextDouble) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OutlineThickness() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutlineThickness<Impl: ITimedTextStyleImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: TimedTextDouble) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetOutlineThickness(&*(&value as *const <TimedTextDouble as ::windows::core::Abi>::Abi as *const <TimedTextDouble as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OutlineRadius<Impl: ITimedTextStyleImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut TimedTextDouble) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OutlineRadius() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutlineRadius<Impl: ITimedTextStyleImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: TimedTextDouble) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetOutlineRadius(&*(&value as *const <TimedTextDouble as ::windows::core::Abi>::Abi as *const <TimedTextDouble as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<ITimedTextStyle>,
            base.5,
            Name::<Impl, OFFSET>,
            SetName::<Impl, OFFSET>,
            FontFamily::<Impl, OFFSET>,
            SetFontFamily::<Impl, OFFSET>,
            FontSize::<Impl, OFFSET>,
            SetFontSize::<Impl, OFFSET>,
            FontWeight::<Impl, OFFSET>,
            SetFontWeight::<Impl, OFFSET>,
            Foreground::<Impl, OFFSET>,
            SetForeground::<Impl, OFFSET>,
            Background::<Impl, OFFSET>,
            SetBackground::<Impl, OFFSET>,
            IsBackgroundAlwaysShown::<Impl, OFFSET>,
            SetIsBackgroundAlwaysShown::<Impl, OFFSET>,
            FlowDirection::<Impl, OFFSET>,
            SetFlowDirection::<Impl, OFFSET>,
            LineAlignment::<Impl, OFFSET>,
            SetLineAlignment::<Impl, OFFSET>,
            OutlineColor::<Impl, OFFSET>,
            SetOutlineColor::<Impl, OFFSET>,
            OutlineThickness::<Impl, OFFSET>,
            SetOutlineThickness::<Impl, OFFSET>,
            OutlineRadius::<Impl, OFFSET>,
            SetOutlineRadius::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimedTextStyle2Impl: Sized {
    fn FontStyle(&self) -> ::windows::core::Result<TimedTextFontStyle>;
    fn SetFontStyle(&self, value: TimedTextFontStyle) -> ::windows::core::Result<()>;
    fn IsUnderlineEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsUnderlineEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsLineThroughEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsLineThroughEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsOverlineEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsOverlineEnabled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITimedTextStyle2 {
    const NAME: &'static str = "Windows.Media.Core.ITimedTextStyle2";
}
#[cfg(feature = "implement_exclusive")]
impl ITimedTextStyle2Vtbl {
    pub const fn new<Impl: ITimedTextStyle2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITimedTextStyle2Vtbl {
        unsafe extern "system" fn FontStyle<Impl: ITimedTextStyle2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut TimedTextFontStyle) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FontStyle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFontStyle<Impl: ITimedTextStyle2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: TimedTextFontStyle) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetFontStyle(value).into()
        }
        unsafe extern "system" fn IsUnderlineEnabled<Impl: ITimedTextStyle2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsUnderlineEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsUnderlineEnabled<Impl: ITimedTextStyle2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsUnderlineEnabled(value).into()
        }
        unsafe extern "system" fn IsLineThroughEnabled<Impl: ITimedTextStyle2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsLineThroughEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsLineThroughEnabled<Impl: ITimedTextStyle2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsLineThroughEnabled(value).into()
        }
        unsafe extern "system" fn IsOverlineEnabled<Impl: ITimedTextStyle2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsOverlineEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsOverlineEnabled<Impl: ITimedTextStyle2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsOverlineEnabled(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITimedTextStyle2>, base.5, FontStyle::<Impl, OFFSET>, SetFontStyle::<Impl, OFFSET>, IsUnderlineEnabled::<Impl, OFFSET>, SetIsUnderlineEnabled::<Impl, OFFSET>, IsLineThroughEnabled::<Impl, OFFSET>, SetIsLineThroughEnabled::<Impl, OFFSET>, IsOverlineEnabled::<Impl, OFFSET>, SetIsOverlineEnabled::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimedTextStyle3Impl: Sized {
    fn Ruby(&self) -> ::windows::core::Result<TimedTextRuby>;
    fn Bouten(&self) -> ::windows::core::Result<TimedTextBouten>;
    fn IsTextCombined(&self) -> ::windows::core::Result<bool>;
    fn SetIsTextCombined(&self, value: bool) -> ::windows::core::Result<()>;
    fn FontAngleInDegrees(&self) -> ::windows::core::Result<f64>;
    fn SetFontAngleInDegrees(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITimedTextStyle3 {
    const NAME: &'static str = "Windows.Media.Core.ITimedTextStyle3";
}
#[cfg(feature = "implement_exclusive")]
impl ITimedTextStyle3Vtbl {
    pub const fn new<Impl: ITimedTextStyle3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITimedTextStyle3Vtbl {
        unsafe extern "system" fn Ruby<Impl: ITimedTextStyle3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Ruby() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Bouten<Impl: ITimedTextStyle3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Bouten() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsTextCombined<Impl: ITimedTextStyle3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsTextCombined() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsTextCombined<Impl: ITimedTextStyle3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsTextCombined(value).into()
        }
        unsafe extern "system" fn FontAngleInDegrees<Impl: ITimedTextStyle3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FontAngleInDegrees() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFontAngleInDegrees<Impl: ITimedTextStyle3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetFontAngleInDegrees(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITimedTextStyle3>, base.5, Ruby::<Impl, OFFSET>, Bouten::<Impl, OFFSET>, IsTextCombined::<Impl, OFFSET>, SetIsTextCombined::<Impl, OFFSET>, FontAngleInDegrees::<Impl, OFFSET>, SetFontAngleInDegrees::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimedTextSubformatImpl: Sized {
    fn StartIndex(&self) -> ::windows::core::Result<i32>;
    fn SetStartIndex(&self, value: i32) -> ::windows::core::Result<()>;
    fn Length(&self) -> ::windows::core::Result<i32>;
    fn SetLength(&self, value: i32) -> ::windows::core::Result<()>;
    fn SubformatStyle(&self) -> ::windows::core::Result<TimedTextStyle>;
    fn SetSubformatStyle(&self, value: &::core::option::Option<TimedTextStyle>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITimedTextSubformat {
    const NAME: &'static str = "Windows.Media.Core.ITimedTextSubformat";
}
#[cfg(feature = "implement_exclusive")]
impl ITimedTextSubformatVtbl {
    pub const fn new<Impl: ITimedTextSubformatImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITimedTextSubformatVtbl {
        unsafe extern "system" fn StartIndex<Impl: ITimedTextSubformatImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StartIndex() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartIndex<Impl: ITimedTextSubformatImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetStartIndex(value).into()
        }
        unsafe extern "system" fn Length<Impl: ITimedTextSubformatImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Length() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLength<Impl: ITimedTextSubformatImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetLength(value).into()
        }
        unsafe extern "system" fn SubformatStyle<Impl: ITimedTextSubformatImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SubformatStyle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSubformatStyle<Impl: ITimedTextSubformatImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSubformatStyle(&*(&value as *const <TimedTextStyle as ::windows::core::Abi>::Abi as *const <TimedTextStyle as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITimedTextSubformat>, base.5, StartIndex::<Impl, OFFSET>, SetStartIndex::<Impl, OFFSET>, Length::<Impl, OFFSET>, SetLength::<Impl, OFFSET>, SubformatStyle::<Impl, OFFSET>, SetSubformatStyle::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVideoStabilizationEffectImpl: Sized + IMediaExtensionImpl {
    fn SetEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn Enabled(&self) -> ::windows::core::Result<bool>;
    fn EnabledChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<VideoStabilizationEffect, VideoStabilizationEffectEnabledChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveEnabledChanged(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GetRecommendedStreamConfiguration(&self, controller: &::core::option::Option<super::Devices::VideoDeviceController>, desiredproperties: &::core::option::Option<super::MediaProperties::VideoEncodingProperties>) -> ::windows::core::Result<super::Capture::VideoStreamConfiguration>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVideoStabilizationEffect {
    const NAME: &'static str = "Windows.Media.Core.IVideoStabilizationEffect";
}
#[cfg(feature = "implement_exclusive")]
impl IVideoStabilizationEffectVtbl {
    pub const fn new<Impl: IVideoStabilizationEffectImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVideoStabilizationEffectVtbl {
        unsafe extern "system" fn SetEnabled<Impl: IVideoStabilizationEffectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetEnabled(value).into()
        }
        unsafe extern "system" fn Enabled<Impl: IVideoStabilizationEffectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Enabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnabledChanged<Impl: IVideoStabilizationEffectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnabledChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<VideoStabilizationEffect, VideoStabilizationEffectEnabledChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<VideoStabilizationEffect, VideoStabilizationEffectEnabledChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveEnabledChanged<Impl: IVideoStabilizationEffectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveEnabledChanged(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetRecommendedStreamConfiguration<Impl: IVideoStabilizationEffectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, controller: ::windows::core::RawPtr, desiredproperties: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetRecommendedStreamConfiguration(&*(&controller as *const <super::Devices::VideoDeviceController as ::windows::core::Abi>::Abi as *const <super::Devices::VideoDeviceController as ::windows::core::DefaultType>::DefaultType), &*(&desiredproperties as *const <super::MediaProperties::VideoEncodingProperties as ::windows::core::Abi>::Abi as *const <super::MediaProperties::VideoEncodingProperties as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVideoStabilizationEffect>, base.5, SetEnabled::<Impl, OFFSET>, Enabled::<Impl, OFFSET>, EnabledChanged::<Impl, OFFSET>, RemoveEnabledChanged::<Impl, OFFSET>, GetRecommendedStreamConfiguration::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVideoStabilizationEffectEnabledChangedEventArgsImpl: Sized {
    fn Reason(&self) -> ::windows::core::Result<VideoStabilizationEffectEnabledChangedReason>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVideoStabilizationEffectEnabledChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Core.IVideoStabilizationEffectEnabledChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IVideoStabilizationEffectEnabledChangedEventArgsVtbl {
    pub const fn new<Impl: IVideoStabilizationEffectEnabledChangedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVideoStabilizationEffectEnabledChangedEventArgsVtbl {
        unsafe extern "system" fn Reason<Impl: IVideoStabilizationEffectEnabledChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut VideoStabilizationEffectEnabledChangedReason) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Reason() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVideoStabilizationEffectEnabledChangedEventArgs>, base.5, Reason::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVideoStreamDescriptorImpl: Sized + IMediaStreamDescriptorImpl {
    fn EncodingProperties(&self) -> ::windows::core::Result<super::MediaProperties::VideoEncodingProperties>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVideoStreamDescriptor {
    const NAME: &'static str = "Windows.Media.Core.IVideoStreamDescriptor";
}
#[cfg(feature = "implement_exclusive")]
impl IVideoStreamDescriptorVtbl {
    pub const fn new<Impl: IVideoStreamDescriptorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVideoStreamDescriptorVtbl {
        unsafe extern "system" fn EncodingProperties<Impl: IVideoStreamDescriptorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EncodingProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVideoStreamDescriptor>, base.5, EncodingProperties::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVideoStreamDescriptor2Impl: Sized {
    fn Copy(&self) -> ::windows::core::Result<VideoStreamDescriptor>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVideoStreamDescriptor2 {
    const NAME: &'static str = "Windows.Media.Core.IVideoStreamDescriptor2";
}
#[cfg(feature = "implement_exclusive")]
impl IVideoStreamDescriptor2Vtbl {
    pub const fn new<Impl: IVideoStreamDescriptor2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVideoStreamDescriptor2Vtbl {
        unsafe extern "system" fn Copy<Impl: IVideoStreamDescriptor2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Copy() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVideoStreamDescriptor2>, base.5, Copy::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVideoStreamDescriptorFactoryImpl: Sized {
    fn Create(&self, encodingproperties: &::core::option::Option<super::MediaProperties::VideoEncodingProperties>) -> ::windows::core::Result<VideoStreamDescriptor>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVideoStreamDescriptorFactory {
    const NAME: &'static str = "Windows.Media.Core.IVideoStreamDescriptorFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IVideoStreamDescriptorFactoryVtbl {
    pub const fn new<Impl: IVideoStreamDescriptorFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVideoStreamDescriptorFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IVideoStreamDescriptorFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, encodingproperties: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(&*(&encodingproperties as *const <super::MediaProperties::VideoEncodingProperties as ::windows::core::Abi>::Abi as *const <super::MediaProperties::VideoEncodingProperties as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVideoStreamDescriptorFactory>, base.5, Create::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVideoTrackImpl: Sized {
    fn OpenFailed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<VideoTrack, VideoTrackOpenFailedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveOpenFailed(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GetEncodingProperties(&self) -> ::windows::core::Result<super::MediaProperties::VideoEncodingProperties>;
    fn PlaybackItem(&self) -> ::windows::core::Result<super::Playback::MediaPlaybackItem>;
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SupportInfo(&self) -> ::windows::core::Result<VideoTrackSupportInfo>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVideoTrack {
    const NAME: &'static str = "Windows.Media.Core.IVideoTrack";
}
#[cfg(feature = "implement_exclusive")]
impl IVideoTrackVtbl {
    pub const fn new<Impl: IVideoTrackImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVideoTrackVtbl {
        unsafe extern "system" fn OpenFailed<Impl: IVideoTrackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OpenFailed(&*(&handler as *const <super::super::Foundation::TypedEventHandler<VideoTrack, VideoTrackOpenFailedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<VideoTrack, VideoTrackOpenFailedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveOpenFailed<Impl: IVideoTrackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveOpenFailed(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetEncodingProperties<Impl: IVideoTrackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetEncodingProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PlaybackItem<Impl: IVideoTrackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PlaybackItem() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: IVideoTrackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportInfo<Impl: IVideoTrackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SupportInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVideoTrack>, base.5, OpenFailed::<Impl, OFFSET>, RemoveOpenFailed::<Impl, OFFSET>, GetEncodingProperties::<Impl, OFFSET>, PlaybackItem::<Impl, OFFSET>, Name::<Impl, OFFSET>, SupportInfo::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVideoTrackOpenFailedEventArgsImpl: Sized {
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVideoTrackOpenFailedEventArgs {
    const NAME: &'static str = "Windows.Media.Core.IVideoTrackOpenFailedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IVideoTrackOpenFailedEventArgsVtbl {
    pub const fn new<Impl: IVideoTrackOpenFailedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVideoTrackOpenFailedEventArgsVtbl {
        unsafe extern "system" fn ExtendedError<Impl: IVideoTrackOpenFailedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExtendedError() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVideoTrackOpenFailedEventArgs>, base.5, ExtendedError::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVideoTrackSupportInfoImpl: Sized {
    fn DecoderStatus(&self) -> ::windows::core::Result<MediaDecoderStatus>;
    fn MediaSourceStatus(&self) -> ::windows::core::Result<MediaSourceStatus>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVideoTrackSupportInfo {
    const NAME: &'static str = "Windows.Media.Core.IVideoTrackSupportInfo";
}
#[cfg(feature = "implement_exclusive")]
impl IVideoTrackSupportInfoVtbl {
    pub const fn new<Impl: IVideoTrackSupportInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVideoTrackSupportInfoVtbl {
        unsafe extern "system" fn DecoderStatus<Impl: IVideoTrackSupportInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut MediaDecoderStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DecoderStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediaSourceStatus<Impl: IVideoTrackSupportInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut MediaSourceStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MediaSourceStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVideoTrackSupportInfo>, base.5, DecoderStatus::<Impl, OFFSET>, MediaSourceStatus::<Impl, OFFSET>)
    }
}
