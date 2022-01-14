#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IAudioEncodingProperties_Impl: Sized + IMediaEncodingProperties_Impl {
    fn SetBitrate(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn Bitrate(&mut self) -> ::windows::core::Result<u32>;
    fn SetChannelCount(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn ChannelCount(&mut self) -> ::windows::core::Result<u32>;
    fn SetSampleRate(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn SampleRate(&mut self) -> ::windows::core::Result<u32>;
    fn SetBitsPerSample(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn BitsPerSample(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAudioEncodingProperties {
    const NAME: &'static str = "Windows.Media.MediaProperties.IAudioEncodingProperties";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IAudioEncodingProperties_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioEncodingProperties_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioEncodingProperties_Vtbl {
        unsafe extern "system" fn SetBitrate<Impl: IAudioEncodingProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBitrate(value).into()
        }
        unsafe extern "system" fn Bitrate<Impl: IAudioEncodingProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetChannelCount<Impl: IAudioEncodingProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetChannelCount(value).into()
        }
        unsafe extern "system" fn ChannelCount<Impl: IAudioEncodingProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ChannelCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSampleRate<Impl: IAudioEncodingProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSampleRate(value).into()
        }
        unsafe extern "system" fn SampleRate<Impl: IAudioEncodingProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SampleRate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBitsPerSample<Impl: IAudioEncodingProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBitsPerSample(value).into()
        }
        unsafe extern "system" fn BitsPerSample<Impl: IAudioEncodingProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BitsPerSample() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAudioEncodingProperties, BASE_OFFSET>(),
            SetBitrate: SetBitrate::<Impl, IMPL_OFFSET>,
            Bitrate: Bitrate::<Impl, IMPL_OFFSET>,
            SetChannelCount: SetChannelCount::<Impl, IMPL_OFFSET>,
            ChannelCount: ChannelCount::<Impl, IMPL_OFFSET>,
            SetSampleRate: SetSampleRate::<Impl, IMPL_OFFSET>,
            SampleRate: SampleRate::<Impl, IMPL_OFFSET>,
            SetBitsPerSample: SetBitsPerSample::<Impl, IMPL_OFFSET>,
            BitsPerSample: BitsPerSample::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioEncodingProperties as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioEncodingProperties2_Impl: Sized {
    fn IsSpatial(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAudioEncodingProperties2 {
    const NAME: &'static str = "Windows.Media.MediaProperties.IAudioEncodingProperties2";
}
#[cfg(feature = "implement_exclusive")]
impl IAudioEncodingProperties2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioEncodingProperties2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioEncodingProperties2_Vtbl {
        unsafe extern "system" fn IsSpatial<Impl: IAudioEncodingProperties2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSpatial() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IAudioEncodingProperties2, BASE_OFFSET>(), IsSpatial: IsSpatial::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioEncodingProperties2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioEncodingProperties3_Impl: Sized {
    fn Copy(&mut self) -> ::windows::core::Result<AudioEncodingProperties>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAudioEncodingProperties3 {
    const NAME: &'static str = "Windows.Media.MediaProperties.IAudioEncodingProperties3";
}
#[cfg(feature = "implement_exclusive")]
impl IAudioEncodingProperties3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioEncodingProperties3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioEncodingProperties3_Vtbl {
        unsafe extern "system" fn Copy<Impl: IAudioEncodingProperties3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IAudioEncodingProperties3, BASE_OFFSET>(), Copy: Copy::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioEncodingProperties3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioEncodingPropertiesStatics_Impl: Sized {
    fn CreateAac(&mut self, samplerate: u32, channelcount: u32, bitrate: u32) -> ::windows::core::Result<AudioEncodingProperties>;
    fn CreateAacAdts(&mut self, samplerate: u32, channelcount: u32, bitrate: u32) -> ::windows::core::Result<AudioEncodingProperties>;
    fn CreateMp3(&mut self, samplerate: u32, channelcount: u32, bitrate: u32) -> ::windows::core::Result<AudioEncodingProperties>;
    fn CreatePcm(&mut self, samplerate: u32, channelcount: u32, bitspersample: u32) -> ::windows::core::Result<AudioEncodingProperties>;
    fn CreateWma(&mut self, samplerate: u32, channelcount: u32, bitrate: u32) -> ::windows::core::Result<AudioEncodingProperties>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAudioEncodingPropertiesStatics {
    const NAME: &'static str = "Windows.Media.MediaProperties.IAudioEncodingPropertiesStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IAudioEncodingPropertiesStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioEncodingPropertiesStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioEncodingPropertiesStatics_Vtbl {
        unsafe extern "system" fn CreateAac<Impl: IAudioEncodingPropertiesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, samplerate: u32, channelcount: u32, bitrate: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateAac(samplerate, channelcount, bitrate) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAacAdts<Impl: IAudioEncodingPropertiesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, samplerate: u32, channelcount: u32, bitrate: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateAacAdts(samplerate, channelcount, bitrate) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateMp3<Impl: IAudioEncodingPropertiesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, samplerate: u32, channelcount: u32, bitrate: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateMp3(samplerate, channelcount, bitrate) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePcm<Impl: IAudioEncodingPropertiesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, samplerate: u32, channelcount: u32, bitspersample: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePcm(samplerate, channelcount, bitspersample) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWma<Impl: IAudioEncodingPropertiesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, samplerate: u32, channelcount: u32, bitrate: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWma(samplerate, channelcount, bitrate) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAudioEncodingPropertiesStatics, BASE_OFFSET>(),
            CreateAac: CreateAac::<Impl, IMPL_OFFSET>,
            CreateAacAdts: CreateAacAdts::<Impl, IMPL_OFFSET>,
            CreateMp3: CreateMp3::<Impl, IMPL_OFFSET>,
            CreatePcm: CreatePcm::<Impl, IMPL_OFFSET>,
            CreateWma: CreateWma::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioEncodingPropertiesStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioEncodingPropertiesStatics2_Impl: Sized {
    fn CreateAlac(&mut self, samplerate: u32, channelcount: u32, bitspersample: u32) -> ::windows::core::Result<AudioEncodingProperties>;
    fn CreateFlac(&mut self, samplerate: u32, channelcount: u32, bitspersample: u32) -> ::windows::core::Result<AudioEncodingProperties>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAudioEncodingPropertiesStatics2 {
    const NAME: &'static str = "Windows.Media.MediaProperties.IAudioEncodingPropertiesStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IAudioEncodingPropertiesStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioEncodingPropertiesStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioEncodingPropertiesStatics2_Vtbl {
        unsafe extern "system" fn CreateAlac<Impl: IAudioEncodingPropertiesStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, samplerate: u32, channelcount: u32, bitspersample: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateAlac(samplerate, channelcount, bitspersample) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFlac<Impl: IAudioEncodingPropertiesStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, samplerate: u32, channelcount: u32, bitspersample: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFlac(samplerate, channelcount, bitspersample) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAudioEncodingPropertiesStatics2, BASE_OFFSET>(),
            CreateAlac: CreateAlac::<Impl, IMPL_OFFSET>,
            CreateFlac: CreateFlac::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioEncodingPropertiesStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAudioEncodingPropertiesWithFormatUserData_Impl: Sized {
    fn SetFormatUserData(&mut self, value: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn GetFormatUserData(&mut self, value: &mut ::windows::core::Array<u8>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAudioEncodingPropertiesWithFormatUserData {
    const NAME: &'static str = "Windows.Media.MediaProperties.IAudioEncodingPropertiesWithFormatUserData";
}
#[cfg(feature = "implement_exclusive")]
impl IAudioEncodingPropertiesWithFormatUserData_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioEncodingPropertiesWithFormatUserData_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioEncodingPropertiesWithFormatUserData_Vtbl {
        unsafe extern "system" fn SetFormatUserData<Impl: IAudioEncodingPropertiesWithFormatUserData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFormatUserData(::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _)).into()
        }
        unsafe extern "system" fn GetFormatUserData<Impl: IAudioEncodingPropertiesWithFormatUserData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFormatUserData(::windows::core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size).as_array()).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAudioEncodingPropertiesWithFormatUserData, BASE_OFFSET>(),
            SetFormatUserData: SetFormatUserData::<Impl, IMPL_OFFSET>,
            GetFormatUserData: GetFormatUserData::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioEncodingPropertiesWithFormatUserData as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IContainerEncodingProperties_Impl: Sized + IMediaEncodingProperties_Impl {}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IContainerEncodingProperties {
    const NAME: &'static str = "Windows.Media.MediaProperties.IContainerEncodingProperties";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IContainerEncodingProperties_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContainerEncodingProperties_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContainerEncodingProperties_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IContainerEncodingProperties, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContainerEncodingProperties as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContainerEncodingProperties2_Impl: Sized {
    fn Copy(&mut self) -> ::windows::core::Result<ContainerEncodingProperties>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContainerEncodingProperties2 {
    const NAME: &'static str = "Windows.Media.MediaProperties.IContainerEncodingProperties2";
}
#[cfg(feature = "implement_exclusive")]
impl IContainerEncodingProperties2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContainerEncodingProperties2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContainerEncodingProperties2_Vtbl {
        unsafe extern "system" fn Copy<Impl: IContainerEncodingProperties2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IContainerEncodingProperties2, BASE_OFFSET>(), Copy: Copy::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContainerEncodingProperties2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IH264ProfileIdsStatics_Impl: Sized {
    fn ConstrainedBaseline(&mut self) -> ::windows::core::Result<i32>;
    fn Baseline(&mut self) -> ::windows::core::Result<i32>;
    fn Extended(&mut self) -> ::windows::core::Result<i32>;
    fn Main(&mut self) -> ::windows::core::Result<i32>;
    fn High(&mut self) -> ::windows::core::Result<i32>;
    fn High10(&mut self) -> ::windows::core::Result<i32>;
    fn High422(&mut self) -> ::windows::core::Result<i32>;
    fn High444(&mut self) -> ::windows::core::Result<i32>;
    fn StereoHigh(&mut self) -> ::windows::core::Result<i32>;
    fn MultiviewHigh(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IH264ProfileIdsStatics {
    const NAME: &'static str = "Windows.Media.MediaProperties.IH264ProfileIdsStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IH264ProfileIdsStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IH264ProfileIdsStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IH264ProfileIdsStatics_Vtbl {
        unsafe extern "system" fn ConstrainedBaseline<Impl: IH264ProfileIdsStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConstrainedBaseline() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Baseline<Impl: IH264ProfileIdsStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Baseline() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Extended<Impl: IH264ProfileIdsStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Extended() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Main<Impl: IH264ProfileIdsStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Main() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn High<Impl: IH264ProfileIdsStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).High() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn High10<Impl: IH264ProfileIdsStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).High10() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn High422<Impl: IH264ProfileIdsStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).High422() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn High444<Impl: IH264ProfileIdsStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).High444() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StereoHigh<Impl: IH264ProfileIdsStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StereoHigh() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MultiviewHigh<Impl: IH264ProfileIdsStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MultiviewHigh() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IH264ProfileIdsStatics, BASE_OFFSET>(),
            ConstrainedBaseline: ConstrainedBaseline::<Impl, IMPL_OFFSET>,
            Baseline: Baseline::<Impl, IMPL_OFFSET>,
            Extended: Extended::<Impl, IMPL_OFFSET>,
            Main: Main::<Impl, IMPL_OFFSET>,
            High: High::<Impl, IMPL_OFFSET>,
            High10: High10::<Impl, IMPL_OFFSET>,
            High422: High422::<Impl, IMPL_OFFSET>,
            High444: High444::<Impl, IMPL_OFFSET>,
            StereoHigh: StereoHigh::<Impl, IMPL_OFFSET>,
            MultiviewHigh: MultiviewHigh::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IH264ProfileIdsStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IImageEncodingProperties_Impl: Sized + IMediaEncodingProperties_Impl {
    fn SetWidth(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn Width(&mut self) -> ::windows::core::Result<u32>;
    fn SetHeight(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn Height(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IImageEncodingProperties {
    const NAME: &'static str = "Windows.Media.MediaProperties.IImageEncodingProperties";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IImageEncodingProperties_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IImageEncodingProperties_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IImageEncodingProperties_Vtbl {
        unsafe extern "system" fn SetWidth<Impl: IImageEncodingProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWidth(value).into()
        }
        unsafe extern "system" fn Width<Impl: IImageEncodingProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetHeight<Impl: IImageEncodingProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHeight(value).into()
        }
        unsafe extern "system" fn Height<Impl: IImageEncodingProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IImageEncodingProperties, BASE_OFFSET>(),
            SetWidth: SetWidth::<Impl, IMPL_OFFSET>,
            Width: Width::<Impl, IMPL_OFFSET>,
            SetHeight: SetHeight::<Impl, IMPL_OFFSET>,
            Height: Height::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IImageEncodingProperties as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IImageEncodingProperties2_Impl: Sized {
    fn Copy(&mut self) -> ::windows::core::Result<ImageEncodingProperties>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IImageEncodingProperties2 {
    const NAME: &'static str = "Windows.Media.MediaProperties.IImageEncodingProperties2";
}
#[cfg(feature = "implement_exclusive")]
impl IImageEncodingProperties2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IImageEncodingProperties2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IImageEncodingProperties2_Vtbl {
        unsafe extern "system" fn Copy<Impl: IImageEncodingProperties2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IImageEncodingProperties2, BASE_OFFSET>(), Copy: Copy::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IImageEncodingProperties2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IImageEncodingPropertiesStatics_Impl: Sized {
    fn CreateJpeg(&mut self) -> ::windows::core::Result<ImageEncodingProperties>;
    fn CreatePng(&mut self) -> ::windows::core::Result<ImageEncodingProperties>;
    fn CreateJpegXR(&mut self) -> ::windows::core::Result<ImageEncodingProperties>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IImageEncodingPropertiesStatics {
    const NAME: &'static str = "Windows.Media.MediaProperties.IImageEncodingPropertiesStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IImageEncodingPropertiesStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IImageEncodingPropertiesStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IImageEncodingPropertiesStatics_Vtbl {
        unsafe extern "system" fn CreateJpeg<Impl: IImageEncodingPropertiesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateJpeg() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePng<Impl: IImageEncodingPropertiesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePng() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateJpegXR<Impl: IImageEncodingPropertiesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateJpegXR() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IImageEncodingPropertiesStatics, BASE_OFFSET>(),
            CreateJpeg: CreateJpeg::<Impl, IMPL_OFFSET>,
            CreatePng: CreatePng::<Impl, IMPL_OFFSET>,
            CreateJpegXR: CreateJpegXR::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IImageEncodingPropertiesStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IImageEncodingPropertiesStatics2_Impl: Sized {
    fn CreateUncompressed(&mut self, format: MediaPixelFormat) -> ::windows::core::Result<ImageEncodingProperties>;
    fn CreateBmp(&mut self) -> ::windows::core::Result<ImageEncodingProperties>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IImageEncodingPropertiesStatics2 {
    const NAME: &'static str = "Windows.Media.MediaProperties.IImageEncodingPropertiesStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IImageEncodingPropertiesStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IImageEncodingPropertiesStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IImageEncodingPropertiesStatics2_Vtbl {
        unsafe extern "system" fn CreateUncompressed<Impl: IImageEncodingPropertiesStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: MediaPixelFormat, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateUncompressed(format) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBmp<Impl: IImageEncodingPropertiesStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateBmp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IImageEncodingPropertiesStatics2, BASE_OFFSET>(),
            CreateUncompressed: CreateUncompressed::<Impl, IMPL_OFFSET>,
            CreateBmp: CreateBmp::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IImageEncodingPropertiesStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IImageEncodingPropertiesStatics3_Impl: Sized {
    fn CreateHeif(&mut self) -> ::windows::core::Result<ImageEncodingProperties>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IImageEncodingPropertiesStatics3 {
    const NAME: &'static str = "Windows.Media.MediaProperties.IImageEncodingPropertiesStatics3";
}
#[cfg(feature = "implement_exclusive")]
impl IImageEncodingPropertiesStatics3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IImageEncodingPropertiesStatics3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IImageEncodingPropertiesStatics3_Vtbl {
        unsafe extern "system" fn CreateHeif<Impl: IImageEncodingPropertiesStatics3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateHeif() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IImageEncodingPropertiesStatics3, BASE_OFFSET>(),
            CreateHeif: CreateHeif::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IImageEncodingPropertiesStatics3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaEncodingProfile_Impl: Sized {
    fn SetAudio(&mut self, value: &::core::option::Option<AudioEncodingProperties>) -> ::windows::core::Result<()>;
    fn Audio(&mut self) -> ::windows::core::Result<AudioEncodingProperties>;
    fn SetVideo(&mut self, value: &::core::option::Option<VideoEncodingProperties>) -> ::windows::core::Result<()>;
    fn Video(&mut self) -> ::windows::core::Result<VideoEncodingProperties>;
    fn SetContainer(&mut self, value: &::core::option::Option<ContainerEncodingProperties>) -> ::windows::core::Result<()>;
    fn Container(&mut self) -> ::windows::core::Result<ContainerEncodingProperties>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaEncodingProfile {
    const NAME: &'static str = "Windows.Media.MediaProperties.IMediaEncodingProfile";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaEncodingProfile_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaEncodingProfile_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaEncodingProfile_Vtbl {
        unsafe extern "system" fn SetAudio<Impl: IMediaEncodingProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAudio(&*(&value as *const <AudioEncodingProperties as ::windows::core::Abi>::Abi as *const <AudioEncodingProperties as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Audio<Impl: IMediaEncodingProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Audio() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVideo<Impl: IMediaEncodingProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVideo(&*(&value as *const <VideoEncodingProperties as ::windows::core::Abi>::Abi as *const <VideoEncodingProperties as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Video<Impl: IMediaEncodingProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Video() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContainer<Impl: IMediaEncodingProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContainer(&*(&value as *const <ContainerEncodingProperties as ::windows::core::Abi>::Abi as *const <ContainerEncodingProperties as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Container<Impl: IMediaEncodingProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Container() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaEncodingProfile, BASE_OFFSET>(),
            SetAudio: SetAudio::<Impl, IMPL_OFFSET>,
            Audio: Audio::<Impl, IMPL_OFFSET>,
            SetVideo: SetVideo::<Impl, IMPL_OFFSET>,
            Video: Video::<Impl, IMPL_OFFSET>,
            SetContainer: SetContainer::<Impl, IMPL_OFFSET>,
            Container: Container::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaEncodingProfile as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core", feature = "implement_exclusive"))]
pub trait IMediaEncodingProfile2_Impl: Sized {
    fn SetAudioTracks(&mut self, value: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::Core::AudioStreamDescriptor>>) -> ::windows::core::Result<()>;
    fn GetAudioTracks(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Core::AudioStreamDescriptor>>;
    fn SetVideoTracks(&mut self, value: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::Core::VideoStreamDescriptor>>) -> ::windows::core::Result<()>;
    fn GetVideoTracks(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Core::VideoStreamDescriptor>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaEncodingProfile2 {
    const NAME: &'static str = "Windows.Media.MediaProperties.IMediaEncodingProfile2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core", feature = "implement_exclusive"))]
impl IMediaEncodingProfile2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaEncodingProfile2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaEncodingProfile2_Vtbl {
        unsafe extern "system" fn SetAudioTracks<Impl: IMediaEncodingProfile2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAudioTracks(&*(&value as *const <super::super::Foundation::Collections::IIterable<super::Core::AudioStreamDescriptor> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<super::Core::AudioStreamDescriptor> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetAudioTracks<Impl: IMediaEncodingProfile2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAudioTracks() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVideoTracks<Impl: IMediaEncodingProfile2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVideoTracks(&*(&value as *const <super::super::Foundation::Collections::IIterable<super::Core::VideoStreamDescriptor> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<super::Core::VideoStreamDescriptor> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetVideoTracks<Impl: IMediaEncodingProfile2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVideoTracks() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaEncodingProfile2, BASE_OFFSET>(),
            SetAudioTracks: SetAudioTracks::<Impl, IMPL_OFFSET>,
            GetAudioTracks: GetAudioTracks::<Impl, IMPL_OFFSET>,
            SetVideoTracks: SetVideoTracks::<Impl, IMPL_OFFSET>,
            GetVideoTracks: GetVideoTracks::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaEncodingProfile2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core", feature = "implement_exclusive"))]
pub trait IMediaEncodingProfile3_Impl: Sized {
    fn SetTimedMetadataTracks(&mut self, value: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::Core::TimedMetadataStreamDescriptor>>) -> ::windows::core::Result<()>;
    fn GetTimedMetadataTracks(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Core::TimedMetadataStreamDescriptor>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaEncodingProfile3 {
    const NAME: &'static str = "Windows.Media.MediaProperties.IMediaEncodingProfile3";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core", feature = "implement_exclusive"))]
impl IMediaEncodingProfile3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaEncodingProfile3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaEncodingProfile3_Vtbl {
        unsafe extern "system" fn SetTimedMetadataTracks<Impl: IMediaEncodingProfile3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTimedMetadataTracks(&*(&value as *const <super::super::Foundation::Collections::IIterable<super::Core::TimedMetadataStreamDescriptor> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<super::Core::TimedMetadataStreamDescriptor> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetTimedMetadataTracks<Impl: IMediaEncodingProfile3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTimedMetadataTracks() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaEncodingProfile3, BASE_OFFSET>(),
            SetTimedMetadataTracks: SetTimedMetadataTracks::<Impl, IMPL_OFFSET>,
            GetTimedMetadataTracks: GetTimedMetadataTracks::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaEncodingProfile3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IMediaEncodingProfileStatics_Impl: Sized {
    fn CreateM4a(&mut self, quality: AudioEncodingQuality) -> ::windows::core::Result<MediaEncodingProfile>;
    fn CreateMp3(&mut self, quality: AudioEncodingQuality) -> ::windows::core::Result<MediaEncodingProfile>;
    fn CreateWma(&mut self, quality: AudioEncodingQuality) -> ::windows::core::Result<MediaEncodingProfile>;
    fn CreateMp4(&mut self, quality: VideoEncodingQuality) -> ::windows::core::Result<MediaEncodingProfile>;
    fn CreateWmv(&mut self, quality: VideoEncodingQuality) -> ::windows::core::Result<MediaEncodingProfile>;
    fn CreateFromFileAsync(&mut self, file: &::core::option::Option<super::super::Storage::IStorageFile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MediaEncodingProfile>>;
    fn CreateFromStreamAsync(&mut self, stream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MediaEncodingProfile>>;
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaEncodingProfileStatics {
    const NAME: &'static str = "Windows.Media.MediaProperties.IMediaEncodingProfileStatics";
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IMediaEncodingProfileStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaEncodingProfileStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaEncodingProfileStatics_Vtbl {
        unsafe extern "system" fn CreateM4a<Impl: IMediaEncodingProfileStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, quality: AudioEncodingQuality, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateM4a(quality) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateMp3<Impl: IMediaEncodingProfileStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, quality: AudioEncodingQuality, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateMp3(quality) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWma<Impl: IMediaEncodingProfileStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, quality: AudioEncodingQuality, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWma(quality) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateMp4<Impl: IMediaEncodingProfileStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, quality: VideoEncodingQuality, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateMp4(quality) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWmv<Impl: IMediaEncodingProfileStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, quality: VideoEncodingQuality, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWmv(quality) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromFileAsync<Impl: IMediaEncodingProfileStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromFileAsync(&*(&file as *const <super::super::Storage::IStorageFile as ::windows::core::Abi>::Abi as *const <super::super::Storage::IStorageFile as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromStreamAsync<Impl: IMediaEncodingProfileStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromStreamAsync(&*(&stream as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaEncodingProfileStatics, BASE_OFFSET>(),
            CreateM4a: CreateM4a::<Impl, IMPL_OFFSET>,
            CreateMp3: CreateMp3::<Impl, IMPL_OFFSET>,
            CreateWma: CreateWma::<Impl, IMPL_OFFSET>,
            CreateMp4: CreateMp4::<Impl, IMPL_OFFSET>,
            CreateWmv: CreateWmv::<Impl, IMPL_OFFSET>,
            CreateFromFileAsync: CreateFromFileAsync::<Impl, IMPL_OFFSET>,
            CreateFromStreamAsync: CreateFromStreamAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaEncodingProfileStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaEncodingProfileStatics2_Impl: Sized {
    fn CreateWav(&mut self, quality: AudioEncodingQuality) -> ::windows::core::Result<MediaEncodingProfile>;
    fn CreateAvi(&mut self, quality: VideoEncodingQuality) -> ::windows::core::Result<MediaEncodingProfile>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaEncodingProfileStatics2 {
    const NAME: &'static str = "Windows.Media.MediaProperties.IMediaEncodingProfileStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaEncodingProfileStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaEncodingProfileStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaEncodingProfileStatics2_Vtbl {
        unsafe extern "system" fn CreateWav<Impl: IMediaEncodingProfileStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, quality: AudioEncodingQuality, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWav(quality) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAvi<Impl: IMediaEncodingProfileStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, quality: VideoEncodingQuality, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateAvi(quality) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaEncodingProfileStatics2, BASE_OFFSET>(),
            CreateWav: CreateWav::<Impl, IMPL_OFFSET>,
            CreateAvi: CreateAvi::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaEncodingProfileStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaEncodingProfileStatics3_Impl: Sized {
    fn CreateAlac(&mut self, quality: AudioEncodingQuality) -> ::windows::core::Result<MediaEncodingProfile>;
    fn CreateFlac(&mut self, quality: AudioEncodingQuality) -> ::windows::core::Result<MediaEncodingProfile>;
    fn CreateHevc(&mut self, quality: VideoEncodingQuality) -> ::windows::core::Result<MediaEncodingProfile>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaEncodingProfileStatics3 {
    const NAME: &'static str = "Windows.Media.MediaProperties.IMediaEncodingProfileStatics3";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaEncodingProfileStatics3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaEncodingProfileStatics3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaEncodingProfileStatics3_Vtbl {
        unsafe extern "system" fn CreateAlac<Impl: IMediaEncodingProfileStatics3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, quality: AudioEncodingQuality, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateAlac(quality) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFlac<Impl: IMediaEncodingProfileStatics3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, quality: AudioEncodingQuality, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFlac(quality) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateHevc<Impl: IMediaEncodingProfileStatics3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, quality: VideoEncodingQuality, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateHevc(quality) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaEncodingProfileStatics3, BASE_OFFSET>(),
            CreateAlac: CreateAlac::<Impl, IMPL_OFFSET>,
            CreateFlac: CreateFlac::<Impl, IMPL_OFFSET>,
            CreateHevc: CreateHevc::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaEncodingProfileStatics3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait IMediaEncodingProperties_Impl: Sized {
    fn Properties(&mut self) -> ::windows::core::Result<MediaPropertySet>;
    fn Type(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSubtype(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Subtype(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for IMediaEncodingProperties {
    const NAME: &'static str = "Windows.Media.MediaProperties.IMediaEncodingProperties";
}
#[cfg(feature = "Foundation_Collections")]
impl IMediaEncodingProperties_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaEncodingProperties_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaEncodingProperties_Vtbl {
        unsafe extern "system" fn Properties<Impl: IMediaEncodingProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Type<Impl: IMediaEncodingProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSubtype<Impl: IMediaEncodingProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSubtype(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Subtype<Impl: IMediaEncodingProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaEncodingProperties, BASE_OFFSET>(),
            Properties: Properties::<Impl, IMPL_OFFSET>,
            Type: Type::<Impl, IMPL_OFFSET>,
            SetSubtype: SetSubtype::<Impl, IMPL_OFFSET>,
            Subtype: Subtype::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaEncodingProperties as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaEncodingSubtypesStatics_Impl: Sized {
    fn Aac(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AacAdts(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Ac3(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AmrNb(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AmrWb(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Argb32(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Asf(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Avi(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Bgra8(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Bmp(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Eac3(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Float(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Gif(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn H263(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn H264(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn H264Es(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Hevc(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn HevcEs(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Iyuv(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Jpeg(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn JpegXr(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Mjpg(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Mpeg(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Mpeg1(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Mpeg2(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Mp3(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Mpeg4(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Nv12(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Pcm(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Png(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Rgb24(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Rgb32(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Tiff(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Wave(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Wma8(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Wma9(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Wmv3(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Wvc1(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Yuy2(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Yv12(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaEncodingSubtypesStatics {
    const NAME: &'static str = "Windows.Media.MediaProperties.IMediaEncodingSubtypesStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaEncodingSubtypesStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaEncodingSubtypesStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaEncodingSubtypesStatics_Vtbl {
        unsafe extern "system" fn Aac<Impl: IMediaEncodingSubtypesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Aac() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AacAdts<Impl: IMediaEncodingSubtypesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AacAdts() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Ac3<Impl: IMediaEncodingSubtypesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Ac3() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AmrNb<Impl: IMediaEncodingSubtypesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AmrNb() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AmrWb<Impl: IMediaEncodingSubtypesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AmrWb() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Argb32<Impl: IMediaEncodingSubtypesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Argb32() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Asf<Impl: IMediaEncodingSubtypesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Asf() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Avi<Impl: IMediaEncodingSubtypesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Avi() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Bgra8<Impl: IMediaEncodingSubtypesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Bgra8() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Bmp<Impl: IMediaEncodingSubtypesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Bmp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Eac3<Impl: IMediaEncodingSubtypesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Eac3() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Float<Impl: IMediaEncodingSubtypesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Float() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Gif<Impl: IMediaEncodingSubtypesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Gif() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn H263<Impl: IMediaEncodingSubtypesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).H263() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn H264<Impl: IMediaEncodingSubtypesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).H264() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn H264Es<Impl: IMediaEncodingSubtypesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).H264Es() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Hevc<Impl: IMediaEncodingSubtypesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Hevc() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HevcEs<Impl: IMediaEncodingSubtypesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HevcEs() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Iyuv<Impl: IMediaEncodingSubtypesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Iyuv() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Jpeg<Impl: IMediaEncodingSubtypesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Jpeg() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn JpegXr<Impl: IMediaEncodingSubtypesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).JpegXr() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Mjpg<Impl: IMediaEncodingSubtypesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Mjpg() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Mpeg<Impl: IMediaEncodingSubtypesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Mpeg() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Mpeg1<Impl: IMediaEncodingSubtypesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Mpeg1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Mpeg2<Impl: IMediaEncodingSubtypesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Mpeg2() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Mp3<Impl: IMediaEncodingSubtypesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Mp3() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Mpeg4<Impl: IMediaEncodingSubtypesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Mpeg4() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Nv12<Impl: IMediaEncodingSubtypesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Nv12() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pcm<Impl: IMediaEncodingSubtypesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Pcm() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Png<Impl: IMediaEncodingSubtypesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Png() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Rgb24<Impl: IMediaEncodingSubtypesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Rgb24() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Rgb32<Impl: IMediaEncodingSubtypesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Rgb32() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Tiff<Impl: IMediaEncodingSubtypesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Tiff() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Wave<Impl: IMediaEncodingSubtypesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Wave() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Wma8<Impl: IMediaEncodingSubtypesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Wma8() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Wma9<Impl: IMediaEncodingSubtypesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Wma9() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Wmv3<Impl: IMediaEncodingSubtypesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Wmv3() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Wvc1<Impl: IMediaEncodingSubtypesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Wvc1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Yuy2<Impl: IMediaEncodingSubtypesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Yuy2() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Yv12<Impl: IMediaEncodingSubtypesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Yv12() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaEncodingSubtypesStatics, BASE_OFFSET>(),
            Aac: Aac::<Impl, IMPL_OFFSET>,
            AacAdts: AacAdts::<Impl, IMPL_OFFSET>,
            Ac3: Ac3::<Impl, IMPL_OFFSET>,
            AmrNb: AmrNb::<Impl, IMPL_OFFSET>,
            AmrWb: AmrWb::<Impl, IMPL_OFFSET>,
            Argb32: Argb32::<Impl, IMPL_OFFSET>,
            Asf: Asf::<Impl, IMPL_OFFSET>,
            Avi: Avi::<Impl, IMPL_OFFSET>,
            Bgra8: Bgra8::<Impl, IMPL_OFFSET>,
            Bmp: Bmp::<Impl, IMPL_OFFSET>,
            Eac3: Eac3::<Impl, IMPL_OFFSET>,
            Float: Float::<Impl, IMPL_OFFSET>,
            Gif: Gif::<Impl, IMPL_OFFSET>,
            H263: H263::<Impl, IMPL_OFFSET>,
            H264: H264::<Impl, IMPL_OFFSET>,
            H264Es: H264Es::<Impl, IMPL_OFFSET>,
            Hevc: Hevc::<Impl, IMPL_OFFSET>,
            HevcEs: HevcEs::<Impl, IMPL_OFFSET>,
            Iyuv: Iyuv::<Impl, IMPL_OFFSET>,
            Jpeg: Jpeg::<Impl, IMPL_OFFSET>,
            JpegXr: JpegXr::<Impl, IMPL_OFFSET>,
            Mjpg: Mjpg::<Impl, IMPL_OFFSET>,
            Mpeg: Mpeg::<Impl, IMPL_OFFSET>,
            Mpeg1: Mpeg1::<Impl, IMPL_OFFSET>,
            Mpeg2: Mpeg2::<Impl, IMPL_OFFSET>,
            Mp3: Mp3::<Impl, IMPL_OFFSET>,
            Mpeg4: Mpeg4::<Impl, IMPL_OFFSET>,
            Nv12: Nv12::<Impl, IMPL_OFFSET>,
            Pcm: Pcm::<Impl, IMPL_OFFSET>,
            Png: Png::<Impl, IMPL_OFFSET>,
            Rgb24: Rgb24::<Impl, IMPL_OFFSET>,
            Rgb32: Rgb32::<Impl, IMPL_OFFSET>,
            Tiff: Tiff::<Impl, IMPL_OFFSET>,
            Wave: Wave::<Impl, IMPL_OFFSET>,
            Wma8: Wma8::<Impl, IMPL_OFFSET>,
            Wma9: Wma9::<Impl, IMPL_OFFSET>,
            Wmv3: Wmv3::<Impl, IMPL_OFFSET>,
            Wvc1: Wvc1::<Impl, IMPL_OFFSET>,
            Yuy2: Yuy2::<Impl, IMPL_OFFSET>,
            Yv12: Yv12::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaEncodingSubtypesStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaEncodingSubtypesStatics2_Impl: Sized {
    fn Vp9(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn L8(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn L16(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn D16(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaEncodingSubtypesStatics2 {
    const NAME: &'static str = "Windows.Media.MediaProperties.IMediaEncodingSubtypesStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaEncodingSubtypesStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaEncodingSubtypesStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaEncodingSubtypesStatics2_Vtbl {
        unsafe extern "system" fn Vp9<Impl: IMediaEncodingSubtypesStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Vp9() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn L8<Impl: IMediaEncodingSubtypesStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).L8() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn L16<Impl: IMediaEncodingSubtypesStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).L16() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn D16<Impl: IMediaEncodingSubtypesStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).D16() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaEncodingSubtypesStatics2, BASE_OFFSET>(),
            Vp9: Vp9::<Impl, IMPL_OFFSET>,
            L8: L8::<Impl, IMPL_OFFSET>,
            L16: L16::<Impl, IMPL_OFFSET>,
            D16: D16::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaEncodingSubtypesStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaEncodingSubtypesStatics3_Impl: Sized {
    fn Alac(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Flac(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaEncodingSubtypesStatics3 {
    const NAME: &'static str = "Windows.Media.MediaProperties.IMediaEncodingSubtypesStatics3";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaEncodingSubtypesStatics3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaEncodingSubtypesStatics3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaEncodingSubtypesStatics3_Vtbl {
        unsafe extern "system" fn Alac<Impl: IMediaEncodingSubtypesStatics3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Alac() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Flac<Impl: IMediaEncodingSubtypesStatics3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Flac() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaEncodingSubtypesStatics3, BASE_OFFSET>(),
            Alac: Alac::<Impl, IMPL_OFFSET>,
            Flac: Flac::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaEncodingSubtypesStatics3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaEncodingSubtypesStatics4_Impl: Sized {
    fn P010(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaEncodingSubtypesStatics4 {
    const NAME: &'static str = "Windows.Media.MediaProperties.IMediaEncodingSubtypesStatics4";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaEncodingSubtypesStatics4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaEncodingSubtypesStatics4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaEncodingSubtypesStatics4_Vtbl {
        unsafe extern "system" fn P010<Impl: IMediaEncodingSubtypesStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).P010() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaEncodingSubtypesStatics4, BASE_OFFSET>(), P010: P010::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaEncodingSubtypesStatics4 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaEncodingSubtypesStatics5_Impl: Sized {
    fn Heif(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaEncodingSubtypesStatics5 {
    const NAME: &'static str = "Windows.Media.MediaProperties.IMediaEncodingSubtypesStatics5";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaEncodingSubtypesStatics5_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaEncodingSubtypesStatics5_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaEncodingSubtypesStatics5_Vtbl {
        unsafe extern "system" fn Heif<Impl: IMediaEncodingSubtypesStatics5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Heif() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaEncodingSubtypesStatics5, BASE_OFFSET>(), Heif: Heif::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaEncodingSubtypesStatics5 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaEncodingSubtypesStatics6_Impl: Sized {
    fn Pgs(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Srt(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Ssa(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VobSub(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaEncodingSubtypesStatics6 {
    const NAME: &'static str = "Windows.Media.MediaProperties.IMediaEncodingSubtypesStatics6";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaEncodingSubtypesStatics6_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaEncodingSubtypesStatics6_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaEncodingSubtypesStatics6_Vtbl {
        unsafe extern "system" fn Pgs<Impl: IMediaEncodingSubtypesStatics6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Pgs() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Srt<Impl: IMediaEncodingSubtypesStatics6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Srt() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Ssa<Impl: IMediaEncodingSubtypesStatics6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Ssa() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VobSub<Impl: IMediaEncodingSubtypesStatics6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VobSub() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaEncodingSubtypesStatics6, BASE_OFFSET>(),
            Pgs: Pgs::<Impl, IMPL_OFFSET>,
            Srt: Srt::<Impl, IMPL_OFFSET>,
            Ssa: Ssa::<Impl, IMPL_OFFSET>,
            VobSub: VobSub::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaEncodingSubtypesStatics6 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaRatio_Impl: Sized {
    fn SetNumerator(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn Numerator(&mut self) -> ::windows::core::Result<u32>;
    fn SetDenominator(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn Denominator(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaRatio {
    const NAME: &'static str = "Windows.Media.MediaProperties.IMediaRatio";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaRatio_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaRatio_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaRatio_Vtbl {
        unsafe extern "system" fn SetNumerator<Impl: IMediaRatio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNumerator(value).into()
        }
        unsafe extern "system" fn Numerator<Impl: IMediaRatio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Numerator() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDenominator<Impl: IMediaRatio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDenominator(value).into()
        }
        unsafe extern "system" fn Denominator<Impl: IMediaRatio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Denominator() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaRatio, BASE_OFFSET>(),
            SetNumerator: SetNumerator::<Impl, IMPL_OFFSET>,
            Numerator: Numerator::<Impl, IMPL_OFFSET>,
            SetDenominator: SetDenominator::<Impl, IMPL_OFFSET>,
            Denominator: Denominator::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaRatio as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMpeg2ProfileIdsStatics_Impl: Sized {
    fn Simple(&mut self) -> ::windows::core::Result<i32>;
    fn Main(&mut self) -> ::windows::core::Result<i32>;
    fn SignalNoiseRatioScalable(&mut self) -> ::windows::core::Result<i32>;
    fn SpatiallyScalable(&mut self) -> ::windows::core::Result<i32>;
    fn High(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMpeg2ProfileIdsStatics {
    const NAME: &'static str = "Windows.Media.MediaProperties.IMpeg2ProfileIdsStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IMpeg2ProfileIdsStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMpeg2ProfileIdsStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMpeg2ProfileIdsStatics_Vtbl {
        unsafe extern "system" fn Simple<Impl: IMpeg2ProfileIdsStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Simple() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Main<Impl: IMpeg2ProfileIdsStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Main() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SignalNoiseRatioScalable<Impl: IMpeg2ProfileIdsStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SignalNoiseRatioScalable() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SpatiallyScalable<Impl: IMpeg2ProfileIdsStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SpatiallyScalable() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn High<Impl: IMpeg2ProfileIdsStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).High() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMpeg2ProfileIdsStatics, BASE_OFFSET>(),
            Simple: Simple::<Impl, IMPL_OFFSET>,
            Main: Main::<Impl, IMPL_OFFSET>,
            SignalNoiseRatioScalable: SignalNoiseRatioScalable::<Impl, IMPL_OFFSET>,
            SpatiallyScalable: SpatiallyScalable::<Impl, IMPL_OFFSET>,
            High: High::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMpeg2ProfileIdsStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimedMetadataEncodingProperties_Impl: Sized {
    fn SetFormatUserData(&mut self, value: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn GetFormatUserData(&mut self, value: &mut ::windows::core::Array<u8>) -> ::windows::core::Result<()>;
    fn Copy(&mut self) -> ::windows::core::Result<TimedMetadataEncodingProperties>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITimedMetadataEncodingProperties {
    const NAME: &'static str = "Windows.Media.MediaProperties.ITimedMetadataEncodingProperties";
}
#[cfg(feature = "implement_exclusive")]
impl ITimedMetadataEncodingProperties_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITimedMetadataEncodingProperties_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITimedMetadataEncodingProperties_Vtbl {
        unsafe extern "system" fn SetFormatUserData<Impl: ITimedMetadataEncodingProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFormatUserData(::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _)).into()
        }
        unsafe extern "system" fn GetFormatUserData<Impl: ITimedMetadataEncodingProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFormatUserData(::windows::core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size).as_array()).into()
        }
        unsafe extern "system" fn Copy<Impl: ITimedMetadataEncodingProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITimedMetadataEncodingProperties, BASE_OFFSET>(),
            SetFormatUserData: SetFormatUserData::<Impl, IMPL_OFFSET>,
            GetFormatUserData: GetFormatUserData::<Impl, IMPL_OFFSET>,
            Copy: Copy::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITimedMetadataEncodingProperties as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimedMetadataEncodingPropertiesStatics_Impl: Sized {
    fn CreatePgs(&mut self) -> ::windows::core::Result<TimedMetadataEncodingProperties>;
    fn CreateSrt(&mut self) -> ::windows::core::Result<TimedMetadataEncodingProperties>;
    fn CreateSsa(&mut self, formatuserdata: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<TimedMetadataEncodingProperties>;
    fn CreateVobSub(&mut self, formatuserdata: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<TimedMetadataEncodingProperties>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITimedMetadataEncodingPropertiesStatics {
    const NAME: &'static str = "Windows.Media.MediaProperties.ITimedMetadataEncodingPropertiesStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ITimedMetadataEncodingPropertiesStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITimedMetadataEncodingPropertiesStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITimedMetadataEncodingPropertiesStatics_Vtbl {
        unsafe extern "system" fn CreatePgs<Impl: ITimedMetadataEncodingPropertiesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePgs() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSrt<Impl: ITimedMetadataEncodingPropertiesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSrt() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSsa<Impl: ITimedMetadataEncodingPropertiesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, formatUserData_array_size: u32, formatuserdata: *const u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSsa(::core::slice::from_raw_parts(::core::mem::transmute_copy(&formatuserdata), formatUserData_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateVobSub<Impl: ITimedMetadataEncodingPropertiesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, formatUserData_array_size: u32, formatuserdata: *const u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateVobSub(::core::slice::from_raw_parts(::core::mem::transmute_copy(&formatuserdata), formatUserData_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITimedMetadataEncodingPropertiesStatics, BASE_OFFSET>(),
            CreatePgs: CreatePgs::<Impl, IMPL_OFFSET>,
            CreateSrt: CreateSrt::<Impl, IMPL_OFFSET>,
            CreateSsa: CreateSsa::<Impl, IMPL_OFFSET>,
            CreateVobSub: CreateVobSub::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITimedMetadataEncodingPropertiesStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IVideoEncodingProperties_Impl: Sized + IMediaEncodingProperties_Impl {
    fn SetBitrate(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn Bitrate(&mut self) -> ::windows::core::Result<u32>;
    fn SetWidth(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn Width(&mut self) -> ::windows::core::Result<u32>;
    fn SetHeight(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn Height(&mut self) -> ::windows::core::Result<u32>;
    fn FrameRate(&mut self) -> ::windows::core::Result<MediaRatio>;
    fn PixelAspectRatio(&mut self) -> ::windows::core::Result<MediaRatio>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVideoEncodingProperties {
    const NAME: &'static str = "Windows.Media.MediaProperties.IVideoEncodingProperties";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IVideoEncodingProperties_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVideoEncodingProperties_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVideoEncodingProperties_Vtbl {
        unsafe extern "system" fn SetBitrate<Impl: IVideoEncodingProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBitrate(value).into()
        }
        unsafe extern "system" fn Bitrate<Impl: IVideoEncodingProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetWidth<Impl: IVideoEncodingProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWidth(value).into()
        }
        unsafe extern "system" fn Width<Impl: IVideoEncodingProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetHeight<Impl: IVideoEncodingProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHeight(value).into()
        }
        unsafe extern "system" fn Height<Impl: IVideoEncodingProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FrameRate<Impl: IVideoEncodingProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PixelAspectRatio<Impl: IVideoEncodingProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PixelAspectRatio() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVideoEncodingProperties, BASE_OFFSET>(),
            SetBitrate: SetBitrate::<Impl, IMPL_OFFSET>,
            Bitrate: Bitrate::<Impl, IMPL_OFFSET>,
            SetWidth: SetWidth::<Impl, IMPL_OFFSET>,
            Width: Width::<Impl, IMPL_OFFSET>,
            SetHeight: SetHeight::<Impl, IMPL_OFFSET>,
            Height: Height::<Impl, IMPL_OFFSET>,
            FrameRate: FrameRate::<Impl, IMPL_OFFSET>,
            PixelAspectRatio: PixelAspectRatio::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVideoEncodingProperties as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVideoEncodingProperties2_Impl: Sized {
    fn SetFormatUserData(&mut self, value: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn GetFormatUserData(&mut self, value: &mut ::windows::core::Array<u8>) -> ::windows::core::Result<()>;
    fn SetProfileId(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn ProfileId(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVideoEncodingProperties2 {
    const NAME: &'static str = "Windows.Media.MediaProperties.IVideoEncodingProperties2";
}
#[cfg(feature = "implement_exclusive")]
impl IVideoEncodingProperties2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVideoEncodingProperties2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVideoEncodingProperties2_Vtbl {
        unsafe extern "system" fn SetFormatUserData<Impl: IVideoEncodingProperties2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFormatUserData(::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _)).into()
        }
        unsafe extern "system" fn GetFormatUserData<Impl: IVideoEncodingProperties2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFormatUserData(::windows::core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size).as_array()).into()
        }
        unsafe extern "system" fn SetProfileId<Impl: IVideoEncodingProperties2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProfileId(value).into()
        }
        unsafe extern "system" fn ProfileId<Impl: IVideoEncodingProperties2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVideoEncodingProperties2, BASE_OFFSET>(),
            SetFormatUserData: SetFormatUserData::<Impl, IMPL_OFFSET>,
            GetFormatUserData: GetFormatUserData::<Impl, IMPL_OFFSET>,
            SetProfileId: SetProfileId::<Impl, IMPL_OFFSET>,
            ProfileId: ProfileId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVideoEncodingProperties2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVideoEncodingProperties3_Impl: Sized {
    fn StereoscopicVideoPackingMode(&mut self) -> ::windows::core::Result<StereoscopicVideoPackingMode>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVideoEncodingProperties3 {
    const NAME: &'static str = "Windows.Media.MediaProperties.IVideoEncodingProperties3";
}
#[cfg(feature = "implement_exclusive")]
impl IVideoEncodingProperties3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVideoEncodingProperties3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVideoEncodingProperties3_Vtbl {
        unsafe extern "system" fn StereoscopicVideoPackingMode<Impl: IVideoEncodingProperties3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut StereoscopicVideoPackingMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StereoscopicVideoPackingMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVideoEncodingProperties3, BASE_OFFSET>(),
            StereoscopicVideoPackingMode: StereoscopicVideoPackingMode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVideoEncodingProperties3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVideoEncodingProperties4_Impl: Sized {
    fn SphericalVideoFrameFormat(&mut self) -> ::windows::core::Result<SphericalVideoFrameFormat>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVideoEncodingProperties4 {
    const NAME: &'static str = "Windows.Media.MediaProperties.IVideoEncodingProperties4";
}
#[cfg(feature = "implement_exclusive")]
impl IVideoEncodingProperties4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVideoEncodingProperties4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVideoEncodingProperties4_Vtbl {
        unsafe extern "system" fn SphericalVideoFrameFormat<Impl: IVideoEncodingProperties4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SphericalVideoFrameFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SphericalVideoFrameFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVideoEncodingProperties4, BASE_OFFSET>(),
            SphericalVideoFrameFormat: SphericalVideoFrameFormat::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVideoEncodingProperties4 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVideoEncodingProperties5_Impl: Sized {
    fn Copy(&mut self) -> ::windows::core::Result<VideoEncodingProperties>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVideoEncodingProperties5 {
    const NAME: &'static str = "Windows.Media.MediaProperties.IVideoEncodingProperties5";
}
#[cfg(feature = "implement_exclusive")]
impl IVideoEncodingProperties5_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVideoEncodingProperties5_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVideoEncodingProperties5_Vtbl {
        unsafe extern "system" fn Copy<Impl: IVideoEncodingProperties5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IVideoEncodingProperties5, BASE_OFFSET>(), Copy: Copy::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVideoEncodingProperties5 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVideoEncodingPropertiesStatics_Impl: Sized {
    fn CreateH264(&mut self) -> ::windows::core::Result<VideoEncodingProperties>;
    fn CreateMpeg2(&mut self) -> ::windows::core::Result<VideoEncodingProperties>;
    fn CreateUncompressed(&mut self, subtype: &::windows::core::HSTRING, width: u32, height: u32) -> ::windows::core::Result<VideoEncodingProperties>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVideoEncodingPropertiesStatics {
    const NAME: &'static str = "Windows.Media.MediaProperties.IVideoEncodingPropertiesStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IVideoEncodingPropertiesStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVideoEncodingPropertiesStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVideoEncodingPropertiesStatics_Vtbl {
        unsafe extern "system" fn CreateH264<Impl: IVideoEncodingPropertiesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateH264() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateMpeg2<Impl: IVideoEncodingPropertiesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateMpeg2() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateUncompressed<Impl: IVideoEncodingPropertiesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subtype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, width: u32, height: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateUncompressed(&*(&subtype as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), width, height) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVideoEncodingPropertiesStatics, BASE_OFFSET>(),
            CreateH264: CreateH264::<Impl, IMPL_OFFSET>,
            CreateMpeg2: CreateMpeg2::<Impl, IMPL_OFFSET>,
            CreateUncompressed: CreateUncompressed::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVideoEncodingPropertiesStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVideoEncodingPropertiesStatics2_Impl: Sized {
    fn CreateHevc(&mut self) -> ::windows::core::Result<VideoEncodingProperties>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVideoEncodingPropertiesStatics2 {
    const NAME: &'static str = "Windows.Media.MediaProperties.IVideoEncodingPropertiesStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IVideoEncodingPropertiesStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVideoEncodingPropertiesStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVideoEncodingPropertiesStatics2_Vtbl {
        unsafe extern "system" fn CreateHevc<Impl: IVideoEncodingPropertiesStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateHevc() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVideoEncodingPropertiesStatics2, BASE_OFFSET>(),
            CreateHevc: CreateHevc::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVideoEncodingPropertiesStatics2 as ::windows::core::Interface>::IID
    }
}
