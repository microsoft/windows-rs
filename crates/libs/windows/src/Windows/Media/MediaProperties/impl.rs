#[cfg(feature = "implement_exclusive")]
pub trait IAudioEncodingPropertiesImpl: Sized + IMediaEncodingPropertiesImpl {
    fn SetBitrate(&self, value: u32) -> ::windows::core::Result<()>;
    fn Bitrate(&self) -> ::windows::core::Result<u32>;
    fn SetChannelCount(&self, value: u32) -> ::windows::core::Result<()>;
    fn ChannelCount(&self) -> ::windows::core::Result<u32>;
    fn SetSampleRate(&self, value: u32) -> ::windows::core::Result<()>;
    fn SampleRate(&self) -> ::windows::core::Result<u32>;
    fn SetBitsPerSample(&self, value: u32) -> ::windows::core::Result<()>;
    fn BitsPerSample(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAudioEncodingProperties {
    const NAME: &'static str = "Windows.Media.MediaProperties.IAudioEncodingProperties";
}
#[cfg(feature = "implement_exclusive")]
impl IAudioEncodingPropertiesVtbl {
    pub const fn new<Impl: IAudioEncodingPropertiesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAudioEncodingPropertiesVtbl {
        unsafe extern "system" fn SetBitrate<Impl: IAudioEncodingPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetBitrate(value).into()
        }
        unsafe extern "system" fn Bitrate<Impl: IAudioEncodingPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Bitrate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChannelCount<Impl: IAudioEncodingPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetChannelCount(value).into()
        }
        unsafe extern "system" fn ChannelCount<Impl: IAudioEncodingPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ChannelCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSampleRate<Impl: IAudioEncodingPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSampleRate(value).into()
        }
        unsafe extern "system" fn SampleRate<Impl: IAudioEncodingPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SampleRate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBitsPerSample<Impl: IAudioEncodingPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetBitsPerSample(value).into()
        }
        unsafe extern "system" fn BitsPerSample<Impl: IAudioEncodingPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BitsPerSample() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAudioEncodingProperties>, base.5, SetBitrate::<Impl, OFFSET>, Bitrate::<Impl, OFFSET>, SetChannelCount::<Impl, OFFSET>, ChannelCount::<Impl, OFFSET>, SetSampleRate::<Impl, OFFSET>, SampleRate::<Impl, OFFSET>, SetBitsPerSample::<Impl, OFFSET>, BitsPerSample::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioEncodingProperties2Impl: Sized {
    fn IsSpatial(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAudioEncodingProperties2 {
    const NAME: &'static str = "Windows.Media.MediaProperties.IAudioEncodingProperties2";
}
#[cfg(feature = "implement_exclusive")]
impl IAudioEncodingProperties2Vtbl {
    pub const fn new<Impl: IAudioEncodingProperties2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAudioEncodingProperties2Vtbl {
        unsafe extern "system" fn IsSpatial<Impl: IAudioEncodingProperties2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsSpatial() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAudioEncodingProperties2>, base.5, IsSpatial::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioEncodingProperties3Impl: Sized {
    fn Copy(&self) -> ::windows::core::Result<AudioEncodingProperties>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAudioEncodingProperties3 {
    const NAME: &'static str = "Windows.Media.MediaProperties.IAudioEncodingProperties3";
}
#[cfg(feature = "implement_exclusive")]
impl IAudioEncodingProperties3Vtbl {
    pub const fn new<Impl: IAudioEncodingProperties3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAudioEncodingProperties3Vtbl {
        unsafe extern "system" fn Copy<Impl: IAudioEncodingProperties3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAudioEncodingProperties3>, base.5, Copy::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioEncodingPropertiesStaticsImpl: Sized {
    fn CreateAac(&self, samplerate: u32, channelcount: u32, bitrate: u32) -> ::windows::core::Result<AudioEncodingProperties>;
    fn CreateAacAdts(&self, samplerate: u32, channelcount: u32, bitrate: u32) -> ::windows::core::Result<AudioEncodingProperties>;
    fn CreateMp3(&self, samplerate: u32, channelcount: u32, bitrate: u32) -> ::windows::core::Result<AudioEncodingProperties>;
    fn CreatePcm(&self, samplerate: u32, channelcount: u32, bitspersample: u32) -> ::windows::core::Result<AudioEncodingProperties>;
    fn CreateWma(&self, samplerate: u32, channelcount: u32, bitrate: u32) -> ::windows::core::Result<AudioEncodingProperties>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAudioEncodingPropertiesStatics {
    const NAME: &'static str = "Windows.Media.MediaProperties.IAudioEncodingPropertiesStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IAudioEncodingPropertiesStaticsVtbl {
    pub const fn new<Impl: IAudioEncodingPropertiesStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAudioEncodingPropertiesStaticsVtbl {
        unsafe extern "system" fn CreateAac<Impl: IAudioEncodingPropertiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, samplerate: u32, channelcount: u32, bitrate: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateAac(samplerate, channelcount, bitrate) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAacAdts<Impl: IAudioEncodingPropertiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, samplerate: u32, channelcount: u32, bitrate: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateAacAdts(samplerate, channelcount, bitrate) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateMp3<Impl: IAudioEncodingPropertiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, samplerate: u32, channelcount: u32, bitrate: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateMp3(samplerate, channelcount, bitrate) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePcm<Impl: IAudioEncodingPropertiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, samplerate: u32, channelcount: u32, bitspersample: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreatePcm(samplerate, channelcount, bitspersample) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWma<Impl: IAudioEncodingPropertiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, samplerate: u32, channelcount: u32, bitrate: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateWma(samplerate, channelcount, bitrate) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAudioEncodingPropertiesStatics>, base.5, CreateAac::<Impl, OFFSET>, CreateAacAdts::<Impl, OFFSET>, CreateMp3::<Impl, OFFSET>, CreatePcm::<Impl, OFFSET>, CreateWma::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioEncodingPropertiesStatics2Impl: Sized {
    fn CreateAlac(&self, samplerate: u32, channelcount: u32, bitspersample: u32) -> ::windows::core::Result<AudioEncodingProperties>;
    fn CreateFlac(&self, samplerate: u32, channelcount: u32, bitspersample: u32) -> ::windows::core::Result<AudioEncodingProperties>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAudioEncodingPropertiesStatics2 {
    const NAME: &'static str = "Windows.Media.MediaProperties.IAudioEncodingPropertiesStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IAudioEncodingPropertiesStatics2Vtbl {
    pub const fn new<Impl: IAudioEncodingPropertiesStatics2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAudioEncodingPropertiesStatics2Vtbl {
        unsafe extern "system" fn CreateAlac<Impl: IAudioEncodingPropertiesStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, samplerate: u32, channelcount: u32, bitspersample: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateAlac(samplerate, channelcount, bitspersample) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFlac<Impl: IAudioEncodingPropertiesStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, samplerate: u32, channelcount: u32, bitspersample: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateFlac(samplerate, channelcount, bitspersample) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAudioEncodingPropertiesStatics2>, base.5, CreateAlac::<Impl, OFFSET>, CreateFlac::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioEncodingPropertiesWithFormatUserDataImpl: Sized {
    fn SetFormatUserData(&self, value: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn GetFormatUserData(&self, value: &mut ::windows::core::Array<u8>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAudioEncodingPropertiesWithFormatUserData {
    const NAME: &'static str = "Windows.Media.MediaProperties.IAudioEncodingPropertiesWithFormatUserData";
}
#[cfg(feature = "implement_exclusive")]
impl IAudioEncodingPropertiesWithFormatUserDataVtbl {
    pub const fn new<Impl: IAudioEncodingPropertiesWithFormatUserDataImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAudioEncodingPropertiesWithFormatUserDataVtbl {
        unsafe extern "system" fn SetFormatUserData<Impl: IAudioEncodingPropertiesWithFormatUserDataImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetFormatUserData(::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _)).into()
        }
        unsafe extern "system" fn GetFormatUserData<Impl: IAudioEncodingPropertiesWithFormatUserDataImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).GetFormatUserData(::windows::core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size).as_array()).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAudioEncodingPropertiesWithFormatUserData>, base.5, SetFormatUserData::<Impl, OFFSET>, GetFormatUserData::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContainerEncodingPropertiesImpl: Sized + IMediaEncodingPropertiesImpl {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContainerEncodingProperties {
    const NAME: &'static str = "Windows.Media.MediaProperties.IContainerEncodingProperties";
}
#[cfg(feature = "implement_exclusive")]
impl IContainerEncodingPropertiesVtbl {
    pub const fn new<Impl: IContainerEncodingPropertiesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IContainerEncodingPropertiesVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IContainerEncodingProperties>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContainerEncodingProperties2Impl: Sized {
    fn Copy(&self) -> ::windows::core::Result<ContainerEncodingProperties>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContainerEncodingProperties2 {
    const NAME: &'static str = "Windows.Media.MediaProperties.IContainerEncodingProperties2";
}
#[cfg(feature = "implement_exclusive")]
impl IContainerEncodingProperties2Vtbl {
    pub const fn new<Impl: IContainerEncodingProperties2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IContainerEncodingProperties2Vtbl {
        unsafe extern "system" fn Copy<Impl: IContainerEncodingProperties2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IContainerEncodingProperties2>, base.5, Copy::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IH264ProfileIdsStaticsImpl: Sized {
    fn ConstrainedBaseline(&self) -> ::windows::core::Result<i32>;
    fn Baseline(&self) -> ::windows::core::Result<i32>;
    fn Extended(&self) -> ::windows::core::Result<i32>;
    fn Main(&self) -> ::windows::core::Result<i32>;
    fn High(&self) -> ::windows::core::Result<i32>;
    fn High10(&self) -> ::windows::core::Result<i32>;
    fn High422(&self) -> ::windows::core::Result<i32>;
    fn High444(&self) -> ::windows::core::Result<i32>;
    fn StereoHigh(&self) -> ::windows::core::Result<i32>;
    fn MultiviewHigh(&self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IH264ProfileIdsStatics {
    const NAME: &'static str = "Windows.Media.MediaProperties.IH264ProfileIdsStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IH264ProfileIdsStaticsVtbl {
    pub const fn new<Impl: IH264ProfileIdsStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IH264ProfileIdsStaticsVtbl {
        unsafe extern "system" fn ConstrainedBaseline<Impl: IH264ProfileIdsStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ConstrainedBaseline() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Baseline<Impl: IH264ProfileIdsStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Baseline() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Extended<Impl: IH264ProfileIdsStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Extended() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Main<Impl: IH264ProfileIdsStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Main() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn High<Impl: IH264ProfileIdsStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).High() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn High10<Impl: IH264ProfileIdsStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).High10() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn High422<Impl: IH264ProfileIdsStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).High422() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn High444<Impl: IH264ProfileIdsStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).High444() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StereoHigh<Impl: IH264ProfileIdsStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StereoHigh() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MultiviewHigh<Impl: IH264ProfileIdsStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MultiviewHigh() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IH264ProfileIdsStatics>, base.5, ConstrainedBaseline::<Impl, OFFSET>, Baseline::<Impl, OFFSET>, Extended::<Impl, OFFSET>, Main::<Impl, OFFSET>, High::<Impl, OFFSET>, High10::<Impl, OFFSET>, High422::<Impl, OFFSET>, High444::<Impl, OFFSET>, StereoHigh::<Impl, OFFSET>, MultiviewHigh::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IImageEncodingPropertiesImpl: Sized + IMediaEncodingPropertiesImpl {
    fn SetWidth(&self, value: u32) -> ::windows::core::Result<()>;
    fn Width(&self) -> ::windows::core::Result<u32>;
    fn SetHeight(&self, value: u32) -> ::windows::core::Result<()>;
    fn Height(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IImageEncodingProperties {
    const NAME: &'static str = "Windows.Media.MediaProperties.IImageEncodingProperties";
}
#[cfg(feature = "implement_exclusive")]
impl IImageEncodingPropertiesVtbl {
    pub const fn new<Impl: IImageEncodingPropertiesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IImageEncodingPropertiesVtbl {
        unsafe extern "system" fn SetWidth<Impl: IImageEncodingPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetWidth(value).into()
        }
        unsafe extern "system" fn Width<Impl: IImageEncodingPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetHeight<Impl: IImageEncodingPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetHeight(value).into()
        }
        unsafe extern "system" fn Height<Impl: IImageEncodingPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IImageEncodingProperties>, base.5, SetWidth::<Impl, OFFSET>, Width::<Impl, OFFSET>, SetHeight::<Impl, OFFSET>, Height::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IImageEncodingProperties2Impl: Sized {
    fn Copy(&self) -> ::windows::core::Result<ImageEncodingProperties>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IImageEncodingProperties2 {
    const NAME: &'static str = "Windows.Media.MediaProperties.IImageEncodingProperties2";
}
#[cfg(feature = "implement_exclusive")]
impl IImageEncodingProperties2Vtbl {
    pub const fn new<Impl: IImageEncodingProperties2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IImageEncodingProperties2Vtbl {
        unsafe extern "system" fn Copy<Impl: IImageEncodingProperties2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IImageEncodingProperties2>, base.5, Copy::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IImageEncodingPropertiesStaticsImpl: Sized {
    fn CreateJpeg(&self) -> ::windows::core::Result<ImageEncodingProperties>;
    fn CreatePng(&self) -> ::windows::core::Result<ImageEncodingProperties>;
    fn CreateJpegXR(&self) -> ::windows::core::Result<ImageEncodingProperties>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IImageEncodingPropertiesStatics {
    const NAME: &'static str = "Windows.Media.MediaProperties.IImageEncodingPropertiesStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IImageEncodingPropertiesStaticsVtbl {
    pub const fn new<Impl: IImageEncodingPropertiesStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IImageEncodingPropertiesStaticsVtbl {
        unsafe extern "system" fn CreateJpeg<Impl: IImageEncodingPropertiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateJpeg() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePng<Impl: IImageEncodingPropertiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreatePng() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateJpegXR<Impl: IImageEncodingPropertiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateJpegXR() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IImageEncodingPropertiesStatics>, base.5, CreateJpeg::<Impl, OFFSET>, CreatePng::<Impl, OFFSET>, CreateJpegXR::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IImageEncodingPropertiesStatics2Impl: Sized {
    fn CreateUncompressed(&self, format: MediaPixelFormat) -> ::windows::core::Result<ImageEncodingProperties>;
    fn CreateBmp(&self) -> ::windows::core::Result<ImageEncodingProperties>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IImageEncodingPropertiesStatics2 {
    const NAME: &'static str = "Windows.Media.MediaProperties.IImageEncodingPropertiesStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IImageEncodingPropertiesStatics2Vtbl {
    pub const fn new<Impl: IImageEncodingPropertiesStatics2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IImageEncodingPropertiesStatics2Vtbl {
        unsafe extern "system" fn CreateUncompressed<Impl: IImageEncodingPropertiesStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, format: MediaPixelFormat, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateUncompressed(format) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBmp<Impl: IImageEncodingPropertiesStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateBmp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IImageEncodingPropertiesStatics2>, base.5, CreateUncompressed::<Impl, OFFSET>, CreateBmp::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IImageEncodingPropertiesStatics3Impl: Sized {
    fn CreateHeif(&self) -> ::windows::core::Result<ImageEncodingProperties>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IImageEncodingPropertiesStatics3 {
    const NAME: &'static str = "Windows.Media.MediaProperties.IImageEncodingPropertiesStatics3";
}
#[cfg(feature = "implement_exclusive")]
impl IImageEncodingPropertiesStatics3Vtbl {
    pub const fn new<Impl: IImageEncodingPropertiesStatics3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IImageEncodingPropertiesStatics3Vtbl {
        unsafe extern "system" fn CreateHeif<Impl: IImageEncodingPropertiesStatics3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateHeif() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IImageEncodingPropertiesStatics3>, base.5, CreateHeif::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaEncodingProfileImpl: Sized {
    fn SetAudio(&self, value: &::core::option::Option<AudioEncodingProperties>) -> ::windows::core::Result<()>;
    fn Audio(&self) -> ::windows::core::Result<AudioEncodingProperties>;
    fn SetVideo(&self, value: &::core::option::Option<VideoEncodingProperties>) -> ::windows::core::Result<()>;
    fn Video(&self) -> ::windows::core::Result<VideoEncodingProperties>;
    fn SetContainer(&self, value: &::core::option::Option<ContainerEncodingProperties>) -> ::windows::core::Result<()>;
    fn Container(&self) -> ::windows::core::Result<ContainerEncodingProperties>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaEncodingProfile {
    const NAME: &'static str = "Windows.Media.MediaProperties.IMediaEncodingProfile";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaEncodingProfileVtbl {
    pub const fn new<Impl: IMediaEncodingProfileImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaEncodingProfileVtbl {
        unsafe extern "system" fn SetAudio<Impl: IMediaEncodingProfileImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAudio(&*(&value as *const <AudioEncodingProperties as ::windows::core::Abi>::Abi as *const <AudioEncodingProperties as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Audio<Impl: IMediaEncodingProfileImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Audio() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVideo<Impl: IMediaEncodingProfileImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetVideo(&*(&value as *const <VideoEncodingProperties as ::windows::core::Abi>::Abi as *const <VideoEncodingProperties as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Video<Impl: IMediaEncodingProfileImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Video() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContainer<Impl: IMediaEncodingProfileImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetContainer(&*(&value as *const <ContainerEncodingProperties as ::windows::core::Abi>::Abi as *const <ContainerEncodingProperties as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Container<Impl: IMediaEncodingProfileImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Container() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaEncodingProfile>, base.5, SetAudio::<Impl, OFFSET>, Audio::<Impl, OFFSET>, SetVideo::<Impl, OFFSET>, Video::<Impl, OFFSET>, SetContainer::<Impl, OFFSET>, Container::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaEncodingProfile2Impl: Sized {
    fn SetAudioTracks(&self, value: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::Core::AudioStreamDescriptor>>) -> ::windows::core::Result<()>;
    fn GetAudioTracks(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Core::AudioStreamDescriptor>>;
    fn SetVideoTracks(&self, value: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::Core::VideoStreamDescriptor>>) -> ::windows::core::Result<()>;
    fn GetVideoTracks(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Core::VideoStreamDescriptor>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaEncodingProfile2 {
    const NAME: &'static str = "Windows.Media.MediaProperties.IMediaEncodingProfile2";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaEncodingProfile2Vtbl {
    pub const fn new<Impl: IMediaEncodingProfile2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaEncodingProfile2Vtbl {
        unsafe extern "system" fn SetAudioTracks<Impl: IMediaEncodingProfile2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAudioTracks(&*(&value as *const <super::super::Foundation::Collections::IIterable<super::Core::AudioStreamDescriptor> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<super::Core::AudioStreamDescriptor> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetAudioTracks<Impl: IMediaEncodingProfile2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAudioTracks() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVideoTracks<Impl: IMediaEncodingProfile2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetVideoTracks(&*(&value as *const <super::super::Foundation::Collections::IIterable<super::Core::VideoStreamDescriptor> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<super::Core::VideoStreamDescriptor> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetVideoTracks<Impl: IMediaEncodingProfile2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetVideoTracks() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaEncodingProfile2>, base.5, SetAudioTracks::<Impl, OFFSET>, GetAudioTracks::<Impl, OFFSET>, SetVideoTracks::<Impl, OFFSET>, GetVideoTracks::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaEncodingProfile3Impl: Sized {
    fn SetTimedMetadataTracks(&self, value: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::Core::TimedMetadataStreamDescriptor>>) -> ::windows::core::Result<()>;
    fn GetTimedMetadataTracks(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Core::TimedMetadataStreamDescriptor>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaEncodingProfile3 {
    const NAME: &'static str = "Windows.Media.MediaProperties.IMediaEncodingProfile3";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaEncodingProfile3Vtbl {
    pub const fn new<Impl: IMediaEncodingProfile3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaEncodingProfile3Vtbl {
        unsafe extern "system" fn SetTimedMetadataTracks<Impl: IMediaEncodingProfile3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTimedMetadataTracks(&*(&value as *const <super::super::Foundation::Collections::IIterable<super::Core::TimedMetadataStreamDescriptor> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<super::Core::TimedMetadataStreamDescriptor> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetTimedMetadataTracks<Impl: IMediaEncodingProfile3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetTimedMetadataTracks() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaEncodingProfile3>, base.5, SetTimedMetadataTracks::<Impl, OFFSET>, GetTimedMetadataTracks::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaEncodingProfileStaticsImpl: Sized {
    fn CreateM4a(&self, quality: AudioEncodingQuality) -> ::windows::core::Result<MediaEncodingProfile>;
    fn CreateMp3(&self, quality: AudioEncodingQuality) -> ::windows::core::Result<MediaEncodingProfile>;
    fn CreateWma(&self, quality: AudioEncodingQuality) -> ::windows::core::Result<MediaEncodingProfile>;
    fn CreateMp4(&self, quality: VideoEncodingQuality) -> ::windows::core::Result<MediaEncodingProfile>;
    fn CreateWmv(&self, quality: VideoEncodingQuality) -> ::windows::core::Result<MediaEncodingProfile>;
    fn CreateFromFileAsync(&self, file: &::core::option::Option<super::super::Storage::IStorageFile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MediaEncodingProfile>>;
    fn CreateFromStreamAsync(&self, stream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MediaEncodingProfile>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaEncodingProfileStatics {
    const NAME: &'static str = "Windows.Media.MediaProperties.IMediaEncodingProfileStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaEncodingProfileStaticsVtbl {
    pub const fn new<Impl: IMediaEncodingProfileStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaEncodingProfileStaticsVtbl {
        unsafe extern "system" fn CreateM4a<Impl: IMediaEncodingProfileStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, quality: AudioEncodingQuality, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateM4a(quality) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateMp3<Impl: IMediaEncodingProfileStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, quality: AudioEncodingQuality, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateMp3(quality) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWma<Impl: IMediaEncodingProfileStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, quality: AudioEncodingQuality, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateWma(quality) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateMp4<Impl: IMediaEncodingProfileStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, quality: VideoEncodingQuality, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateMp4(quality) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWmv<Impl: IMediaEncodingProfileStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, quality: VideoEncodingQuality, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateWmv(quality) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromFileAsync<Impl: IMediaEncodingProfileStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateFromFileAsync(&*(&file as *const <super::super::Storage::IStorageFile as ::windows::core::Abi>::Abi as *const <super::super::Storage::IStorageFile as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromStreamAsync<Impl: IMediaEncodingProfileStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateFromStreamAsync(&*(&stream as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaEncodingProfileStatics>, base.5, CreateM4a::<Impl, OFFSET>, CreateMp3::<Impl, OFFSET>, CreateWma::<Impl, OFFSET>, CreateMp4::<Impl, OFFSET>, CreateWmv::<Impl, OFFSET>, CreateFromFileAsync::<Impl, OFFSET>, CreateFromStreamAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaEncodingProfileStatics2Impl: Sized {
    fn CreateWav(&self, quality: AudioEncodingQuality) -> ::windows::core::Result<MediaEncodingProfile>;
    fn CreateAvi(&self, quality: VideoEncodingQuality) -> ::windows::core::Result<MediaEncodingProfile>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaEncodingProfileStatics2 {
    const NAME: &'static str = "Windows.Media.MediaProperties.IMediaEncodingProfileStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaEncodingProfileStatics2Vtbl {
    pub const fn new<Impl: IMediaEncodingProfileStatics2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaEncodingProfileStatics2Vtbl {
        unsafe extern "system" fn CreateWav<Impl: IMediaEncodingProfileStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, quality: AudioEncodingQuality, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateWav(quality) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAvi<Impl: IMediaEncodingProfileStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, quality: VideoEncodingQuality, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateAvi(quality) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaEncodingProfileStatics2>, base.5, CreateWav::<Impl, OFFSET>, CreateAvi::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaEncodingProfileStatics3Impl: Sized {
    fn CreateAlac(&self, quality: AudioEncodingQuality) -> ::windows::core::Result<MediaEncodingProfile>;
    fn CreateFlac(&self, quality: AudioEncodingQuality) -> ::windows::core::Result<MediaEncodingProfile>;
    fn CreateHevc(&self, quality: VideoEncodingQuality) -> ::windows::core::Result<MediaEncodingProfile>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaEncodingProfileStatics3 {
    const NAME: &'static str = "Windows.Media.MediaProperties.IMediaEncodingProfileStatics3";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaEncodingProfileStatics3Vtbl {
    pub const fn new<Impl: IMediaEncodingProfileStatics3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaEncodingProfileStatics3Vtbl {
        unsafe extern "system" fn CreateAlac<Impl: IMediaEncodingProfileStatics3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, quality: AudioEncodingQuality, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateAlac(quality) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFlac<Impl: IMediaEncodingProfileStatics3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, quality: AudioEncodingQuality, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateFlac(quality) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateHevc<Impl: IMediaEncodingProfileStatics3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, quality: VideoEncodingQuality, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateHevc(quality) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaEncodingProfileStatics3>, base.5, CreateAlac::<Impl, OFFSET>, CreateFlac::<Impl, OFFSET>, CreateHevc::<Impl, OFFSET>)
    }
}
pub trait IMediaEncodingPropertiesImpl: Sized {
    fn Properties(&self) -> ::windows::core::Result<MediaPropertySet>;
    fn Type(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSubtype(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Subtype(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for IMediaEncodingProperties {
    const NAME: &'static str = "Windows.Media.MediaProperties.IMediaEncodingProperties";
}
impl IMediaEncodingPropertiesVtbl {
    pub const fn new<Impl: IMediaEncodingPropertiesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaEncodingPropertiesVtbl {
        unsafe extern "system" fn Properties<Impl: IMediaEncodingPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Type<Impl: IMediaEncodingPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSubtype<Impl: IMediaEncodingPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSubtype(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Subtype<Impl: IMediaEncodingPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaEncodingProperties>, base.5, Properties::<Impl, OFFSET>, Type::<Impl, OFFSET>, SetSubtype::<Impl, OFFSET>, Subtype::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaEncodingSubtypesStaticsImpl: Sized {
    fn Aac(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AacAdts(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Ac3(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AmrNb(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AmrWb(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Argb32(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Asf(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Avi(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Bgra8(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Bmp(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Eac3(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Float(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Gif(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn H263(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn H264(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn H264Es(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Hevc(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn HevcEs(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Iyuv(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Jpeg(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn JpegXr(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Mjpg(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Mpeg(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Mpeg1(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Mpeg2(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Mp3(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Mpeg4(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Nv12(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Pcm(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Png(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Rgb24(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Rgb32(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Tiff(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Wave(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Wma8(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Wma9(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Wmv3(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Wvc1(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Yuy2(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Yv12(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaEncodingSubtypesStatics {
    const NAME: &'static str = "Windows.Media.MediaProperties.IMediaEncodingSubtypesStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaEncodingSubtypesStaticsVtbl {
    pub const fn new<Impl: IMediaEncodingSubtypesStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaEncodingSubtypesStaticsVtbl {
        unsafe extern "system" fn Aac<Impl: IMediaEncodingSubtypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Aac() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AacAdts<Impl: IMediaEncodingSubtypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AacAdts() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Ac3<Impl: IMediaEncodingSubtypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Ac3() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AmrNb<Impl: IMediaEncodingSubtypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AmrNb() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AmrWb<Impl: IMediaEncodingSubtypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AmrWb() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Argb32<Impl: IMediaEncodingSubtypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Argb32() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Asf<Impl: IMediaEncodingSubtypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Asf() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Avi<Impl: IMediaEncodingSubtypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Avi() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Bgra8<Impl: IMediaEncodingSubtypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Bgra8() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Bmp<Impl: IMediaEncodingSubtypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Bmp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Eac3<Impl: IMediaEncodingSubtypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Eac3() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Float<Impl: IMediaEncodingSubtypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Float() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Gif<Impl: IMediaEncodingSubtypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Gif() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn H263<Impl: IMediaEncodingSubtypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).H263() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn H264<Impl: IMediaEncodingSubtypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).H264() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn H264Es<Impl: IMediaEncodingSubtypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).H264Es() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Hevc<Impl: IMediaEncodingSubtypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Hevc() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HevcEs<Impl: IMediaEncodingSubtypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HevcEs() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Iyuv<Impl: IMediaEncodingSubtypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Iyuv() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Jpeg<Impl: IMediaEncodingSubtypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Jpeg() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn JpegXr<Impl: IMediaEncodingSubtypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).JpegXr() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Mjpg<Impl: IMediaEncodingSubtypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Mjpg() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Mpeg<Impl: IMediaEncodingSubtypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Mpeg() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Mpeg1<Impl: IMediaEncodingSubtypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Mpeg1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Mpeg2<Impl: IMediaEncodingSubtypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Mpeg2() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Mp3<Impl: IMediaEncodingSubtypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Mp3() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Mpeg4<Impl: IMediaEncodingSubtypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Mpeg4() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Nv12<Impl: IMediaEncodingSubtypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Nv12() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pcm<Impl: IMediaEncodingSubtypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Pcm() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Png<Impl: IMediaEncodingSubtypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Png() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Rgb24<Impl: IMediaEncodingSubtypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Rgb24() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Rgb32<Impl: IMediaEncodingSubtypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Rgb32() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Tiff<Impl: IMediaEncodingSubtypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Tiff() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Wave<Impl: IMediaEncodingSubtypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Wave() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Wma8<Impl: IMediaEncodingSubtypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Wma8() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Wma9<Impl: IMediaEncodingSubtypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Wma9() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Wmv3<Impl: IMediaEncodingSubtypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Wmv3() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Wvc1<Impl: IMediaEncodingSubtypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Wvc1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Yuy2<Impl: IMediaEncodingSubtypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Yuy2() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Yv12<Impl: IMediaEncodingSubtypesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Yv12() {
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
            ::windows::core::GetRuntimeClassName::<IMediaEncodingSubtypesStatics>,
            base.5,
            Aac::<Impl, OFFSET>,
            AacAdts::<Impl, OFFSET>,
            Ac3::<Impl, OFFSET>,
            AmrNb::<Impl, OFFSET>,
            AmrWb::<Impl, OFFSET>,
            Argb32::<Impl, OFFSET>,
            Asf::<Impl, OFFSET>,
            Avi::<Impl, OFFSET>,
            Bgra8::<Impl, OFFSET>,
            Bmp::<Impl, OFFSET>,
            Eac3::<Impl, OFFSET>,
            Float::<Impl, OFFSET>,
            Gif::<Impl, OFFSET>,
            H263::<Impl, OFFSET>,
            H264::<Impl, OFFSET>,
            H264Es::<Impl, OFFSET>,
            Hevc::<Impl, OFFSET>,
            HevcEs::<Impl, OFFSET>,
            Iyuv::<Impl, OFFSET>,
            Jpeg::<Impl, OFFSET>,
            JpegXr::<Impl, OFFSET>,
            Mjpg::<Impl, OFFSET>,
            Mpeg::<Impl, OFFSET>,
            Mpeg1::<Impl, OFFSET>,
            Mpeg2::<Impl, OFFSET>,
            Mp3::<Impl, OFFSET>,
            Mpeg4::<Impl, OFFSET>,
            Nv12::<Impl, OFFSET>,
            Pcm::<Impl, OFFSET>,
            Png::<Impl, OFFSET>,
            Rgb24::<Impl, OFFSET>,
            Rgb32::<Impl, OFFSET>,
            Tiff::<Impl, OFFSET>,
            Wave::<Impl, OFFSET>,
            Wma8::<Impl, OFFSET>,
            Wma9::<Impl, OFFSET>,
            Wmv3::<Impl, OFFSET>,
            Wvc1::<Impl, OFFSET>,
            Yuy2::<Impl, OFFSET>,
            Yv12::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaEncodingSubtypesStatics2Impl: Sized {
    fn Vp9(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn L8(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn L16(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn D16(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaEncodingSubtypesStatics2 {
    const NAME: &'static str = "Windows.Media.MediaProperties.IMediaEncodingSubtypesStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaEncodingSubtypesStatics2Vtbl {
    pub const fn new<Impl: IMediaEncodingSubtypesStatics2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaEncodingSubtypesStatics2Vtbl {
        unsafe extern "system" fn Vp9<Impl: IMediaEncodingSubtypesStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Vp9() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn L8<Impl: IMediaEncodingSubtypesStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).L8() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn L16<Impl: IMediaEncodingSubtypesStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).L16() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn D16<Impl: IMediaEncodingSubtypesStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).D16() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaEncodingSubtypesStatics2>, base.5, Vp9::<Impl, OFFSET>, L8::<Impl, OFFSET>, L16::<Impl, OFFSET>, D16::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaEncodingSubtypesStatics3Impl: Sized {
    fn Alac(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Flac(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaEncodingSubtypesStatics3 {
    const NAME: &'static str = "Windows.Media.MediaProperties.IMediaEncodingSubtypesStatics3";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaEncodingSubtypesStatics3Vtbl {
    pub const fn new<Impl: IMediaEncodingSubtypesStatics3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaEncodingSubtypesStatics3Vtbl {
        unsafe extern "system" fn Alac<Impl: IMediaEncodingSubtypesStatics3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Alac() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Flac<Impl: IMediaEncodingSubtypesStatics3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Flac() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaEncodingSubtypesStatics3>, base.5, Alac::<Impl, OFFSET>, Flac::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaEncodingSubtypesStatics4Impl: Sized {
    fn P010(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaEncodingSubtypesStatics4 {
    const NAME: &'static str = "Windows.Media.MediaProperties.IMediaEncodingSubtypesStatics4";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaEncodingSubtypesStatics4Vtbl {
    pub const fn new<Impl: IMediaEncodingSubtypesStatics4Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaEncodingSubtypesStatics4Vtbl {
        unsafe extern "system" fn P010<Impl: IMediaEncodingSubtypesStatics4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).P010() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaEncodingSubtypesStatics4>, base.5, P010::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaEncodingSubtypesStatics5Impl: Sized {
    fn Heif(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaEncodingSubtypesStatics5 {
    const NAME: &'static str = "Windows.Media.MediaProperties.IMediaEncodingSubtypesStatics5";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaEncodingSubtypesStatics5Vtbl {
    pub const fn new<Impl: IMediaEncodingSubtypesStatics5Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaEncodingSubtypesStatics5Vtbl {
        unsafe extern "system" fn Heif<Impl: IMediaEncodingSubtypesStatics5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Heif() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaEncodingSubtypesStatics5>, base.5, Heif::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaEncodingSubtypesStatics6Impl: Sized {
    fn Pgs(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Srt(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Ssa(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VobSub(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaEncodingSubtypesStatics6 {
    const NAME: &'static str = "Windows.Media.MediaProperties.IMediaEncodingSubtypesStatics6";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaEncodingSubtypesStatics6Vtbl {
    pub const fn new<Impl: IMediaEncodingSubtypesStatics6Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaEncodingSubtypesStatics6Vtbl {
        unsafe extern "system" fn Pgs<Impl: IMediaEncodingSubtypesStatics6Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Pgs() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Srt<Impl: IMediaEncodingSubtypesStatics6Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Srt() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Ssa<Impl: IMediaEncodingSubtypesStatics6Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Ssa() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VobSub<Impl: IMediaEncodingSubtypesStatics6Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VobSub() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaEncodingSubtypesStatics6>, base.5, Pgs::<Impl, OFFSET>, Srt::<Impl, OFFSET>, Ssa::<Impl, OFFSET>, VobSub::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaRatioImpl: Sized {
    fn SetNumerator(&self, value: u32) -> ::windows::core::Result<()>;
    fn Numerator(&self) -> ::windows::core::Result<u32>;
    fn SetDenominator(&self, value: u32) -> ::windows::core::Result<()>;
    fn Denominator(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaRatio {
    const NAME: &'static str = "Windows.Media.MediaProperties.IMediaRatio";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaRatioVtbl {
    pub const fn new<Impl: IMediaRatioImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaRatioVtbl {
        unsafe extern "system" fn SetNumerator<Impl: IMediaRatioImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetNumerator(value).into()
        }
        unsafe extern "system" fn Numerator<Impl: IMediaRatioImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Numerator() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDenominator<Impl: IMediaRatioImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDenominator(value).into()
        }
        unsafe extern "system" fn Denominator<Impl: IMediaRatioImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Denominator() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaRatio>, base.5, SetNumerator::<Impl, OFFSET>, Numerator::<Impl, OFFSET>, SetDenominator::<Impl, OFFSET>, Denominator::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMpeg2ProfileIdsStaticsImpl: Sized {
    fn Simple(&self) -> ::windows::core::Result<i32>;
    fn Main(&self) -> ::windows::core::Result<i32>;
    fn SignalNoiseRatioScalable(&self) -> ::windows::core::Result<i32>;
    fn SpatiallyScalable(&self) -> ::windows::core::Result<i32>;
    fn High(&self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMpeg2ProfileIdsStatics {
    const NAME: &'static str = "Windows.Media.MediaProperties.IMpeg2ProfileIdsStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IMpeg2ProfileIdsStaticsVtbl {
    pub const fn new<Impl: IMpeg2ProfileIdsStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMpeg2ProfileIdsStaticsVtbl {
        unsafe extern "system" fn Simple<Impl: IMpeg2ProfileIdsStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Simple() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Main<Impl: IMpeg2ProfileIdsStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Main() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SignalNoiseRatioScalable<Impl: IMpeg2ProfileIdsStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SignalNoiseRatioScalable() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SpatiallyScalable<Impl: IMpeg2ProfileIdsStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SpatiallyScalable() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn High<Impl: IMpeg2ProfileIdsStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).High() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMpeg2ProfileIdsStatics>, base.5, Simple::<Impl, OFFSET>, Main::<Impl, OFFSET>, SignalNoiseRatioScalable::<Impl, OFFSET>, SpatiallyScalable::<Impl, OFFSET>, High::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimedMetadataEncodingPropertiesImpl: Sized {
    fn SetFormatUserData(&self, value: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn GetFormatUserData(&self, value: &mut ::windows::core::Array<u8>) -> ::windows::core::Result<()>;
    fn Copy(&self) -> ::windows::core::Result<TimedMetadataEncodingProperties>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITimedMetadataEncodingProperties {
    const NAME: &'static str = "Windows.Media.MediaProperties.ITimedMetadataEncodingProperties";
}
#[cfg(feature = "implement_exclusive")]
impl ITimedMetadataEncodingPropertiesVtbl {
    pub const fn new<Impl: ITimedMetadataEncodingPropertiesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITimedMetadataEncodingPropertiesVtbl {
        unsafe extern "system" fn SetFormatUserData<Impl: ITimedMetadataEncodingPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetFormatUserData(::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _)).into()
        }
        unsafe extern "system" fn GetFormatUserData<Impl: ITimedMetadataEncodingPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).GetFormatUserData(::windows::core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size).as_array()).into()
        }
        unsafe extern "system" fn Copy<Impl: ITimedMetadataEncodingPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITimedMetadataEncodingProperties>, base.5, SetFormatUserData::<Impl, OFFSET>, GetFormatUserData::<Impl, OFFSET>, Copy::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimedMetadataEncodingPropertiesStaticsImpl: Sized {
    fn CreatePgs(&self) -> ::windows::core::Result<TimedMetadataEncodingProperties>;
    fn CreateSrt(&self) -> ::windows::core::Result<TimedMetadataEncodingProperties>;
    fn CreateSsa(&self, formatuserdata: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<TimedMetadataEncodingProperties>;
    fn CreateVobSub(&self, formatuserdata: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<TimedMetadataEncodingProperties>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITimedMetadataEncodingPropertiesStatics {
    const NAME: &'static str = "Windows.Media.MediaProperties.ITimedMetadataEncodingPropertiesStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ITimedMetadataEncodingPropertiesStaticsVtbl {
    pub const fn new<Impl: ITimedMetadataEncodingPropertiesStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITimedMetadataEncodingPropertiesStaticsVtbl {
        unsafe extern "system" fn CreatePgs<Impl: ITimedMetadataEncodingPropertiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreatePgs() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSrt<Impl: ITimedMetadataEncodingPropertiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateSrt() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSsa<Impl: ITimedMetadataEncodingPropertiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, formatUserData_array_size: u32, formatuserdata: *const u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateSsa(::core::slice::from_raw_parts(::core::mem::transmute_copy(&formatuserdata), formatUserData_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateVobSub<Impl: ITimedMetadataEncodingPropertiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, formatUserData_array_size: u32, formatuserdata: *const u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateVobSub(::core::slice::from_raw_parts(::core::mem::transmute_copy(&formatuserdata), formatUserData_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITimedMetadataEncodingPropertiesStatics>, base.5, CreatePgs::<Impl, OFFSET>, CreateSrt::<Impl, OFFSET>, CreateSsa::<Impl, OFFSET>, CreateVobSub::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVideoEncodingPropertiesImpl: Sized + IMediaEncodingPropertiesImpl {
    fn SetBitrate(&self, value: u32) -> ::windows::core::Result<()>;
    fn Bitrate(&self) -> ::windows::core::Result<u32>;
    fn SetWidth(&self, value: u32) -> ::windows::core::Result<()>;
    fn Width(&self) -> ::windows::core::Result<u32>;
    fn SetHeight(&self, value: u32) -> ::windows::core::Result<()>;
    fn Height(&self) -> ::windows::core::Result<u32>;
    fn FrameRate(&self) -> ::windows::core::Result<MediaRatio>;
    fn PixelAspectRatio(&self) -> ::windows::core::Result<MediaRatio>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVideoEncodingProperties {
    const NAME: &'static str = "Windows.Media.MediaProperties.IVideoEncodingProperties";
}
#[cfg(feature = "implement_exclusive")]
impl IVideoEncodingPropertiesVtbl {
    pub const fn new<Impl: IVideoEncodingPropertiesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVideoEncodingPropertiesVtbl {
        unsafe extern "system" fn SetBitrate<Impl: IVideoEncodingPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetBitrate(value).into()
        }
        unsafe extern "system" fn Bitrate<Impl: IVideoEncodingPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Bitrate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWidth<Impl: IVideoEncodingPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetWidth(value).into()
        }
        unsafe extern "system" fn Width<Impl: IVideoEncodingPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetHeight<Impl: IVideoEncodingPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetHeight(value).into()
        }
        unsafe extern "system" fn Height<Impl: IVideoEncodingPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FrameRate<Impl: IVideoEncodingPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PixelAspectRatio<Impl: IVideoEncodingPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PixelAspectRatio() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVideoEncodingProperties>, base.5, SetBitrate::<Impl, OFFSET>, Bitrate::<Impl, OFFSET>, SetWidth::<Impl, OFFSET>, Width::<Impl, OFFSET>, SetHeight::<Impl, OFFSET>, Height::<Impl, OFFSET>, FrameRate::<Impl, OFFSET>, PixelAspectRatio::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVideoEncodingProperties2Impl: Sized {
    fn SetFormatUserData(&self, value: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn GetFormatUserData(&self, value: &mut ::windows::core::Array<u8>) -> ::windows::core::Result<()>;
    fn SetProfileId(&self, value: i32) -> ::windows::core::Result<()>;
    fn ProfileId(&self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVideoEncodingProperties2 {
    const NAME: &'static str = "Windows.Media.MediaProperties.IVideoEncodingProperties2";
}
#[cfg(feature = "implement_exclusive")]
impl IVideoEncodingProperties2Vtbl {
    pub const fn new<Impl: IVideoEncodingProperties2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVideoEncodingProperties2Vtbl {
        unsafe extern "system" fn SetFormatUserData<Impl: IVideoEncodingProperties2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetFormatUserData(::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _)).into()
        }
        unsafe extern "system" fn GetFormatUserData<Impl: IVideoEncodingProperties2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).GetFormatUserData(::windows::core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size).as_array()).into()
        }
        unsafe extern "system" fn SetProfileId<Impl: IVideoEncodingProperties2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetProfileId(value).into()
        }
        unsafe extern "system" fn ProfileId<Impl: IVideoEncodingProperties2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVideoEncodingProperties2>, base.5, SetFormatUserData::<Impl, OFFSET>, GetFormatUserData::<Impl, OFFSET>, SetProfileId::<Impl, OFFSET>, ProfileId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVideoEncodingProperties3Impl: Sized {
    fn StereoscopicVideoPackingMode(&self) -> ::windows::core::Result<StereoscopicVideoPackingMode>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVideoEncodingProperties3 {
    const NAME: &'static str = "Windows.Media.MediaProperties.IVideoEncodingProperties3";
}
#[cfg(feature = "implement_exclusive")]
impl IVideoEncodingProperties3Vtbl {
    pub const fn new<Impl: IVideoEncodingProperties3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVideoEncodingProperties3Vtbl {
        unsafe extern "system" fn StereoscopicVideoPackingMode<Impl: IVideoEncodingProperties3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut StereoscopicVideoPackingMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StereoscopicVideoPackingMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVideoEncodingProperties3>, base.5, StereoscopicVideoPackingMode::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVideoEncodingProperties4Impl: Sized {
    fn SphericalVideoFrameFormat(&self) -> ::windows::core::Result<SphericalVideoFrameFormat>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVideoEncodingProperties4 {
    const NAME: &'static str = "Windows.Media.MediaProperties.IVideoEncodingProperties4";
}
#[cfg(feature = "implement_exclusive")]
impl IVideoEncodingProperties4Vtbl {
    pub const fn new<Impl: IVideoEncodingProperties4Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVideoEncodingProperties4Vtbl {
        unsafe extern "system" fn SphericalVideoFrameFormat<Impl: IVideoEncodingProperties4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut SphericalVideoFrameFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SphericalVideoFrameFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVideoEncodingProperties4>, base.5, SphericalVideoFrameFormat::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVideoEncodingProperties5Impl: Sized {
    fn Copy(&self) -> ::windows::core::Result<VideoEncodingProperties>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVideoEncodingProperties5 {
    const NAME: &'static str = "Windows.Media.MediaProperties.IVideoEncodingProperties5";
}
#[cfg(feature = "implement_exclusive")]
impl IVideoEncodingProperties5Vtbl {
    pub const fn new<Impl: IVideoEncodingProperties5Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVideoEncodingProperties5Vtbl {
        unsafe extern "system" fn Copy<Impl: IVideoEncodingProperties5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVideoEncodingProperties5>, base.5, Copy::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVideoEncodingPropertiesStaticsImpl: Sized {
    fn CreateH264(&self) -> ::windows::core::Result<VideoEncodingProperties>;
    fn CreateMpeg2(&self) -> ::windows::core::Result<VideoEncodingProperties>;
    fn CreateUncompressed(&self, subtype: &::windows::core::HSTRING, width: u32, height: u32) -> ::windows::core::Result<VideoEncodingProperties>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVideoEncodingPropertiesStatics {
    const NAME: &'static str = "Windows.Media.MediaProperties.IVideoEncodingPropertiesStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IVideoEncodingPropertiesStaticsVtbl {
    pub const fn new<Impl: IVideoEncodingPropertiesStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVideoEncodingPropertiesStaticsVtbl {
        unsafe extern "system" fn CreateH264<Impl: IVideoEncodingPropertiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateH264() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateMpeg2<Impl: IVideoEncodingPropertiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateMpeg2() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateUncompressed<Impl: IVideoEncodingPropertiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, subtype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, width: u32, height: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateUncompressed(&*(&subtype as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), width, height) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVideoEncodingPropertiesStatics>, base.5, CreateH264::<Impl, OFFSET>, CreateMpeg2::<Impl, OFFSET>, CreateUncompressed::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVideoEncodingPropertiesStatics2Impl: Sized {
    fn CreateHevc(&self) -> ::windows::core::Result<VideoEncodingProperties>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVideoEncodingPropertiesStatics2 {
    const NAME: &'static str = "Windows.Media.MediaProperties.IVideoEncodingPropertiesStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IVideoEncodingPropertiesStatics2Vtbl {
    pub const fn new<Impl: IVideoEncodingPropertiesStatics2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVideoEncodingPropertiesStatics2Vtbl {
        unsafe extern "system" fn CreateHevc<Impl: IVideoEncodingPropertiesStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateHevc() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVideoEncodingPropertiesStatics2>, base.5, CreateHevc::<Impl, OFFSET>)
    }
}
