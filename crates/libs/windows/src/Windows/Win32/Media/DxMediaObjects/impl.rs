pub trait IDMOQualityControl_Impl: Sized {
    fn SetNow(&mut self, rtnow: i64) -> ::windows::core::Result<()>;
    fn SetStatus(&mut self, dwflags: u32) -> ::windows::core::Result<()>;
    fn GetStatus(&mut self) -> ::windows::core::Result<u32>;
}
impl IDMOQualityControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDMOQualityControl_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDMOQualityControl_Vtbl {
        unsafe extern "system" fn SetNow<Impl: IDMOQualityControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rtnow: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNow(::core::mem::transmute_copy(&rtnow)).into()
        }
        unsafe extern "system" fn SetStatus<Impl: IDMOQualityControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStatus(::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetStatus<Impl: IDMOQualityControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetNow: SetNow::<Impl, IMPL_OFFSET>,
            SetStatus: SetStatus::<Impl, IMPL_OFFSET>,
            GetStatus: GetStatus::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDMOQualityControl as ::windows::core::Interface>::IID
    }
}
pub trait IDMOVideoOutputOptimizations_Impl: Sized {
    fn QueryOperationModePreferences(&mut self, uloutputstreamindex: u32) -> ::windows::core::Result<u32>;
    fn SetOperationMode(&mut self, uloutputstreamindex: u32, dwenabledfeatures: u32) -> ::windows::core::Result<()>;
    fn GetCurrentOperationMode(&mut self, uloutputstreamindex: u32) -> ::windows::core::Result<u32>;
    fn GetCurrentSampleRequirements(&mut self, uloutputstreamindex: u32) -> ::windows::core::Result<u32>;
}
impl IDMOVideoOutputOptimizations_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDMOVideoOutputOptimizations_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDMOVideoOutputOptimizations_Vtbl {
        unsafe extern "system" fn QueryOperationModePreferences<Impl: IDMOVideoOutputOptimizations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uloutputstreamindex: u32, pdwrequestedcapabilities: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryOperationModePreferences(::core::mem::transmute_copy(&uloutputstreamindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwrequestedcapabilities = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOperationMode<Impl: IDMOVideoOutputOptimizations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uloutputstreamindex: u32, dwenabledfeatures: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOperationMode(::core::mem::transmute_copy(&uloutputstreamindex), ::core::mem::transmute_copy(&dwenabledfeatures)).into()
        }
        unsafe extern "system" fn GetCurrentOperationMode<Impl: IDMOVideoOutputOptimizations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uloutputstreamindex: u32, pdwenabledfeatures: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentOperationMode(::core::mem::transmute_copy(&uloutputstreamindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwenabledfeatures = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentSampleRequirements<Impl: IDMOVideoOutputOptimizations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uloutputstreamindex: u32, pdwrequestedfeatures: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentSampleRequirements(::core::mem::transmute_copy(&uloutputstreamindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwrequestedfeatures = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            QueryOperationModePreferences: QueryOperationModePreferences::<Impl, IMPL_OFFSET>,
            SetOperationMode: SetOperationMode::<Impl, IMPL_OFFSET>,
            GetCurrentOperationMode: GetCurrentOperationMode::<Impl, IMPL_OFFSET>,
            GetCurrentSampleRequirements: GetCurrentSampleRequirements::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDMOVideoOutputOptimizations as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IEnumDMO_Impl: Sized {
    fn Next(&mut self, citemstofetch: u32, pclsid: *mut ::windows::core::GUID, names: *mut super::super::Foundation::PWSTR, pcitemsfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&mut self, citemstoskip: u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumDMO>;
}
#[cfg(feature = "Win32_Foundation")]
impl IEnumDMO_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumDMO_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumDMO_Vtbl {
        unsafe extern "system" fn Next<Impl: IEnumDMO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, citemstofetch: u32, pclsid: *mut ::windows::core::GUID, names: *mut super::super::Foundation::PWSTR, pcitemsfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&citemstofetch), ::core::mem::transmute_copy(&pclsid), ::core::mem::transmute_copy(&names), ::core::mem::transmute_copy(&pcitemsfetched)).into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumDMO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, citemstoskip: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&citemstoskip)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumDMO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumDMO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Next: Next::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumDMO as ::windows::core::Interface>::IID
    }
}
pub trait IMediaBuffer_Impl: Sized {
    fn SetLength(&mut self, cblength: u32) -> ::windows::core::Result<()>;
    fn GetMaxLength(&mut self) -> ::windows::core::Result<u32>;
    fn GetBufferAndLength(&mut self, ppbuffer: *mut *mut u8, pcblength: *mut u32) -> ::windows::core::Result<()>;
}
impl IMediaBuffer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaBuffer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaBuffer_Vtbl {
        unsafe extern "system" fn SetLength<Impl: IMediaBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cblength: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLength(::core::mem::transmute_copy(&cblength)).into()
        }
        unsafe extern "system" fn GetMaxLength<Impl: IMediaBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbmaxlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMaxLength() {
                ::core::result::Result::Ok(ok__) => {
                    *pcbmaxlength = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBufferAndLength<Impl: IMediaBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppbuffer: *mut *mut u8, pcblength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetBufferAndLength(::core::mem::transmute_copy(&ppbuffer), ::core::mem::transmute_copy(&pcblength)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetLength: SetLength::<Impl, IMPL_OFFSET>,
            GetMaxLength: GetMaxLength::<Impl, IMPL_OFFSET>,
            GetBufferAndLength: GetBufferAndLength::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaBuffer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMediaObject_Impl: Sized {
    fn GetStreamCount(&mut self, pcinputstreams: *mut u32, pcoutputstreams: *mut u32) -> ::windows::core::Result<()>;
    fn GetInputStreamInfo(&mut self, dwinputstreamindex: u32) -> ::windows::core::Result<u32>;
    fn GetOutputStreamInfo(&mut self, dwoutputstreamindex: u32) -> ::windows::core::Result<u32>;
    fn GetInputType(&mut self, dwinputstreamindex: u32, dwtypeindex: u32) -> ::windows::core::Result<DMO_MEDIA_TYPE>;
    fn GetOutputType(&mut self, dwoutputstreamindex: u32, dwtypeindex: u32) -> ::windows::core::Result<DMO_MEDIA_TYPE>;
    fn SetInputType(&mut self, dwinputstreamindex: u32, pmt: *const DMO_MEDIA_TYPE, dwflags: u32) -> ::windows::core::Result<()>;
    fn SetOutputType(&mut self, dwoutputstreamindex: u32, pmt: *const DMO_MEDIA_TYPE, dwflags: u32) -> ::windows::core::Result<()>;
    fn GetInputCurrentType(&mut self, dwinputstreamindex: u32) -> ::windows::core::Result<DMO_MEDIA_TYPE>;
    fn GetOutputCurrentType(&mut self, dwoutputstreamindex: u32) -> ::windows::core::Result<DMO_MEDIA_TYPE>;
    fn GetInputSizeInfo(&mut self, dwinputstreamindex: u32, pcbsize: *mut u32, pcbmaxlookahead: *mut u32, pcbalignment: *mut u32) -> ::windows::core::Result<()>;
    fn GetOutputSizeInfo(&mut self, dwoutputstreamindex: u32, pcbsize: *mut u32, pcbalignment: *mut u32) -> ::windows::core::Result<()>;
    fn GetInputMaxLatency(&mut self, dwinputstreamindex: u32) -> ::windows::core::Result<i64>;
    fn SetInputMaxLatency(&mut self, dwinputstreamindex: u32, rtmaxlatency: i64) -> ::windows::core::Result<()>;
    fn Flush(&mut self) -> ::windows::core::Result<()>;
    fn Discontinuity(&mut self, dwinputstreamindex: u32) -> ::windows::core::Result<()>;
    fn AllocateStreamingResources(&mut self) -> ::windows::core::Result<()>;
    fn FreeStreamingResources(&mut self) -> ::windows::core::Result<()>;
    fn GetInputStatus(&mut self, dwinputstreamindex: u32) -> ::windows::core::Result<u32>;
    fn ProcessInput(&mut self, dwinputstreamindex: u32, pbuffer: &::core::option::Option<IMediaBuffer>, dwflags: u32, rttimestamp: i64, rttimelength: i64) -> ::windows::core::Result<()>;
    fn ProcessOutput(&mut self, dwflags: u32, coutputbuffercount: u32, poutputbuffers: *mut DMO_OUTPUT_DATA_BUFFER, pdwstatus: *mut u32) -> ::windows::core::Result<()>;
    fn Lock(&mut self, block: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IMediaObject_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaObject_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaObject_Vtbl {
        unsafe extern "system" fn GetStreamCount<Impl: IMediaObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcinputstreams: *mut u32, pcoutputstreams: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetStreamCount(::core::mem::transmute_copy(&pcinputstreams), ::core::mem::transmute_copy(&pcoutputstreams)).into()
        }
        unsafe extern "system" fn GetInputStreamInfo<Impl: IMediaObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputstreamindex: u32, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInputStreamInfo(::core::mem::transmute_copy(&dwinputstreamindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputStreamInfo<Impl: IMediaObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputstreamindex: u32, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOutputStreamInfo(::core::mem::transmute_copy(&dwoutputstreamindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputType<Impl: IMediaObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputstreamindex: u32, dwtypeindex: u32, pmt: *mut DMO_MEDIA_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInputType(::core::mem::transmute_copy(&dwinputstreamindex), ::core::mem::transmute_copy(&dwtypeindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pmt = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputType<Impl: IMediaObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputstreamindex: u32, dwtypeindex: u32, pmt: *mut DMO_MEDIA_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOutputType(::core::mem::transmute_copy(&dwoutputstreamindex), ::core::mem::transmute_copy(&dwtypeindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pmt = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInputType<Impl: IMediaObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputstreamindex: u32, pmt: *const DMO_MEDIA_TYPE, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInputType(::core::mem::transmute_copy(&dwinputstreamindex), ::core::mem::transmute_copy(&pmt), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn SetOutputType<Impl: IMediaObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputstreamindex: u32, pmt: *const DMO_MEDIA_TYPE, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOutputType(::core::mem::transmute_copy(&dwoutputstreamindex), ::core::mem::transmute_copy(&pmt), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetInputCurrentType<Impl: IMediaObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputstreamindex: u32, pmt: *mut DMO_MEDIA_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInputCurrentType(::core::mem::transmute_copy(&dwinputstreamindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pmt = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputCurrentType<Impl: IMediaObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputstreamindex: u32, pmt: *mut DMO_MEDIA_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOutputCurrentType(::core::mem::transmute_copy(&dwoutputstreamindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pmt = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputSizeInfo<Impl: IMediaObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputstreamindex: u32, pcbsize: *mut u32, pcbmaxlookahead: *mut u32, pcbalignment: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetInputSizeInfo(::core::mem::transmute_copy(&dwinputstreamindex), ::core::mem::transmute_copy(&pcbsize), ::core::mem::transmute_copy(&pcbmaxlookahead), ::core::mem::transmute_copy(&pcbalignment)).into()
        }
        unsafe extern "system" fn GetOutputSizeInfo<Impl: IMediaObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputstreamindex: u32, pcbsize: *mut u32, pcbalignment: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetOutputSizeInfo(::core::mem::transmute_copy(&dwoutputstreamindex), ::core::mem::transmute_copy(&pcbsize), ::core::mem::transmute_copy(&pcbalignment)).into()
        }
        unsafe extern "system" fn GetInputMaxLatency<Impl: IMediaObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputstreamindex: u32, prtmaxlatency: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInputMaxLatency(::core::mem::transmute_copy(&dwinputstreamindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *prtmaxlatency = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInputMaxLatency<Impl: IMediaObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputstreamindex: u32, rtmaxlatency: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInputMaxLatency(::core::mem::transmute_copy(&dwinputstreamindex), ::core::mem::transmute_copy(&rtmaxlatency)).into()
        }
        unsafe extern "system" fn Flush<Impl: IMediaObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Flush().into()
        }
        unsafe extern "system" fn Discontinuity<Impl: IMediaObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputstreamindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Discontinuity(::core::mem::transmute_copy(&dwinputstreamindex)).into()
        }
        unsafe extern "system" fn AllocateStreamingResources<Impl: IMediaObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AllocateStreamingResources().into()
        }
        unsafe extern "system" fn FreeStreamingResources<Impl: IMediaObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FreeStreamingResources().into()
        }
        unsafe extern "system" fn GetInputStatus<Impl: IMediaObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputstreamindex: u32, dwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInputStatus(::core::mem::transmute_copy(&dwinputstreamindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *dwflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProcessInput<Impl: IMediaObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputstreamindex: u32, pbuffer: ::windows::core::RawPtr, dwflags: u32, rttimestamp: i64, rttimelength: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ProcessInput(::core::mem::transmute_copy(&dwinputstreamindex), ::core::mem::transmute(&pbuffer), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&rttimestamp), ::core::mem::transmute_copy(&rttimelength)).into()
        }
        unsafe extern "system" fn ProcessOutput<Impl: IMediaObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, coutputbuffercount: u32, poutputbuffers: *mut DMO_OUTPUT_DATA_BUFFER, pdwstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ProcessOutput(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&coutputbuffercount), ::core::mem::transmute_copy(&poutputbuffers), ::core::mem::transmute_copy(&pdwstatus)).into()
        }
        unsafe extern "system" fn Lock<Impl: IMediaObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, block: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Lock(::core::mem::transmute_copy(&block)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetStreamCount: GetStreamCount::<Impl, IMPL_OFFSET>,
            GetInputStreamInfo: GetInputStreamInfo::<Impl, IMPL_OFFSET>,
            GetOutputStreamInfo: GetOutputStreamInfo::<Impl, IMPL_OFFSET>,
            GetInputType: GetInputType::<Impl, IMPL_OFFSET>,
            GetOutputType: GetOutputType::<Impl, IMPL_OFFSET>,
            SetInputType: SetInputType::<Impl, IMPL_OFFSET>,
            SetOutputType: SetOutputType::<Impl, IMPL_OFFSET>,
            GetInputCurrentType: GetInputCurrentType::<Impl, IMPL_OFFSET>,
            GetOutputCurrentType: GetOutputCurrentType::<Impl, IMPL_OFFSET>,
            GetInputSizeInfo: GetInputSizeInfo::<Impl, IMPL_OFFSET>,
            GetOutputSizeInfo: GetOutputSizeInfo::<Impl, IMPL_OFFSET>,
            GetInputMaxLatency: GetInputMaxLatency::<Impl, IMPL_OFFSET>,
            SetInputMaxLatency: SetInputMaxLatency::<Impl, IMPL_OFFSET>,
            Flush: Flush::<Impl, IMPL_OFFSET>,
            Discontinuity: Discontinuity::<Impl, IMPL_OFFSET>,
            AllocateStreamingResources: AllocateStreamingResources::<Impl, IMPL_OFFSET>,
            FreeStreamingResources: FreeStreamingResources::<Impl, IMPL_OFFSET>,
            GetInputStatus: GetInputStatus::<Impl, IMPL_OFFSET>,
            ProcessInput: ProcessInput::<Impl, IMPL_OFFSET>,
            ProcessOutput: ProcessOutput::<Impl, IMPL_OFFSET>,
            Lock: Lock::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaObject as ::windows::core::Interface>::IID
    }
}
pub trait IMediaObjectInPlace_Impl: Sized {
    fn Process(&mut self, ulsize: u32, pdata: *mut u8, reftimestart: i64, dwflags: u32) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IMediaObjectInPlace>;
    fn GetLatency(&mut self) -> ::windows::core::Result<i64>;
}
impl IMediaObjectInPlace_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaObjectInPlace_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaObjectInPlace_Vtbl {
        unsafe extern "system" fn Process<Impl: IMediaObjectInPlace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulsize: u32, pdata: *mut u8, reftimestart: i64, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Process(::core::mem::transmute_copy(&ulsize), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&reftimestart), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn Clone<Impl: IMediaObjectInPlace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmediaobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppmediaobject = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLatency<Impl: IMediaObjectInPlace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, platencytime: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLatency() {
                ::core::result::Result::Ok(ok__) => {
                    *platencytime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Process: Process::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
            GetLatency: GetLatency::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaObjectInPlace as ::windows::core::Interface>::IID
    }
}
