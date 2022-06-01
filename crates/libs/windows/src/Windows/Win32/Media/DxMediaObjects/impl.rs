pub trait IDMOQualityControl_Impl: Sized {
    fn SetNow(&self, rtnow: i64) -> ::windows::core::Result<()>;
    fn SetStatus(&self, dwflags: u32) -> ::windows::core::Result<()>;
    fn GetStatus(&self) -> ::windows::core::Result<u32>;
}
impl ::windows::core::RuntimeName for IDMOQualityControl {}
impl IDMOQualityControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDMOQualityControl_Impl, const OFFSET: isize>() -> IDMOQualityControl_Vtbl {
        unsafe extern "system" fn SetNow<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDMOQualityControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rtnow: i64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetNow(::core::mem::transmute_copy(&rtnow)).into()
        }
        unsafe extern "system" fn SetStatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDMOQualityControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetStatus(::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetStatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDMOQualityControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStatus() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwflags, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetNow: SetNow::<Identity, Impl, OFFSET>,
            SetStatus: SetStatus::<Identity, Impl, OFFSET>,
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDMOQualityControl as ::windows::core::Interface>::IID
    }
}
pub trait IDMOVideoOutputOptimizations_Impl: Sized {
    fn QueryOperationModePreferences(&self, uloutputstreamindex: u32) -> ::windows::core::Result<u32>;
    fn SetOperationMode(&self, uloutputstreamindex: u32, dwenabledfeatures: u32) -> ::windows::core::Result<()>;
    fn GetCurrentOperationMode(&self, uloutputstreamindex: u32) -> ::windows::core::Result<u32>;
    fn GetCurrentSampleRequirements(&self, uloutputstreamindex: u32) -> ::windows::core::Result<u32>;
}
impl ::windows::core::RuntimeName for IDMOVideoOutputOptimizations {}
impl IDMOVideoOutputOptimizations_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDMOVideoOutputOptimizations_Impl, const OFFSET: isize>() -> IDMOVideoOutputOptimizations_Vtbl {
        unsafe extern "system" fn QueryOperationModePreferences<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDMOVideoOutputOptimizations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uloutputstreamindex: u32, pdwrequestedcapabilities: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QueryOperationModePreferences(::core::mem::transmute_copy(&uloutputstreamindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwrequestedcapabilities, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOperationMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDMOVideoOutputOptimizations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uloutputstreamindex: u32, dwenabledfeatures: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetOperationMode(::core::mem::transmute_copy(&uloutputstreamindex), ::core::mem::transmute_copy(&dwenabledfeatures)).into()
        }
        unsafe extern "system" fn GetCurrentOperationMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDMOVideoOutputOptimizations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uloutputstreamindex: u32, pdwenabledfeatures: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCurrentOperationMode(::core::mem::transmute_copy(&uloutputstreamindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwenabledfeatures, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentSampleRequirements<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDMOVideoOutputOptimizations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uloutputstreamindex: u32, pdwrequestedfeatures: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCurrentSampleRequirements(::core::mem::transmute_copy(&uloutputstreamindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwrequestedfeatures, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            QueryOperationModePreferences: QueryOperationModePreferences::<Identity, Impl, OFFSET>,
            SetOperationMode: SetOperationMode::<Identity, Impl, OFFSET>,
            GetCurrentOperationMode: GetCurrentOperationMode::<Identity, Impl, OFFSET>,
            GetCurrentSampleRequirements: GetCurrentSampleRequirements::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDMOVideoOutputOptimizations as ::windows::core::Interface>::IID
    }
}
pub trait IEnumDMO_Impl: Sized {
    fn Next(&self, citemstofetch: u32, pclsid: *mut ::windows::core::GUID, names: *mut ::windows::core::PWSTR, pcitemsfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&self, citemstoskip: u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IEnumDMO>;
}
impl ::windows::core::RuntimeName for IEnumDMO {}
impl IEnumDMO_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumDMO_Impl, const OFFSET: isize>() -> IEnumDMO_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumDMO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, citemstofetch: u32, pclsid: *mut ::windows::core::GUID, names: *mut ::windows::core::PWSTR, pcitemsfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&citemstofetch), ::core::mem::transmute_copy(&pclsid), ::core::mem::transmute_copy(&names), ::core::mem::transmute_copy(&pcitemsfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumDMO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, citemstoskip: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&citemstoskip)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumDMO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumDMO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumDMO as ::windows::core::Interface>::IID
    }
}
pub trait IMediaBuffer_Impl: Sized {
    fn SetLength(&self, cblength: u32) -> ::windows::core::Result<()>;
    fn GetMaxLength(&self) -> ::windows::core::Result<u32>;
    fn GetBufferAndLength(&self, ppbuffer: *mut *mut u8, pcblength: *mut u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IMediaBuffer {}
impl IMediaBuffer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMediaBuffer_Impl, const OFFSET: isize>() -> IMediaBuffer_Vtbl {
        unsafe extern "system" fn SetLength<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMediaBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cblength: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLength(::core::mem::transmute_copy(&cblength)).into()
        }
        unsafe extern "system" fn GetMaxLength<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMediaBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbmaxlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMaxLength() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcbmaxlength, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBufferAndLength<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMediaBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppbuffer: *mut *mut u8, pcblength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetBufferAndLength(::core::mem::transmute_copy(&ppbuffer), ::core::mem::transmute_copy(&pcblength)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetLength: SetLength::<Identity, Impl, OFFSET>,
            GetMaxLength: GetMaxLength::<Identity, Impl, OFFSET>,
            GetBufferAndLength: GetBufferAndLength::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaBuffer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMediaObject_Impl: Sized {
    fn GetStreamCount(&self, pcinputstreams: *mut u32, pcoutputstreams: *mut u32) -> ::windows::core::Result<()>;
    fn GetInputStreamInfo(&self, dwinputstreamindex: u32) -> ::windows::core::Result<u32>;
    fn GetOutputStreamInfo(&self, dwoutputstreamindex: u32) -> ::windows::core::Result<u32>;
    fn GetInputType(&self, dwinputstreamindex: u32, dwtypeindex: u32) -> ::windows::core::Result<DMO_MEDIA_TYPE>;
    fn GetOutputType(&self, dwoutputstreamindex: u32, dwtypeindex: u32) -> ::windows::core::Result<DMO_MEDIA_TYPE>;
    fn SetInputType(&self, dwinputstreamindex: u32, pmt: *const DMO_MEDIA_TYPE, dwflags: u32) -> ::windows::core::Result<()>;
    fn SetOutputType(&self, dwoutputstreamindex: u32, pmt: *const DMO_MEDIA_TYPE, dwflags: u32) -> ::windows::core::Result<()>;
    fn GetInputCurrentType(&self, dwinputstreamindex: u32) -> ::windows::core::Result<DMO_MEDIA_TYPE>;
    fn GetOutputCurrentType(&self, dwoutputstreamindex: u32) -> ::windows::core::Result<DMO_MEDIA_TYPE>;
    fn GetInputSizeInfo(&self, dwinputstreamindex: u32, pcbsize: *mut u32, pcbmaxlookahead: *mut u32, pcbalignment: *mut u32) -> ::windows::core::Result<()>;
    fn GetOutputSizeInfo(&self, dwoutputstreamindex: u32, pcbsize: *mut u32, pcbalignment: *mut u32) -> ::windows::core::Result<()>;
    fn GetInputMaxLatency(&self, dwinputstreamindex: u32) -> ::windows::core::Result<i64>;
    fn SetInputMaxLatency(&self, dwinputstreamindex: u32, rtmaxlatency: i64) -> ::windows::core::Result<()>;
    fn Flush(&self) -> ::windows::core::Result<()>;
    fn Discontinuity(&self, dwinputstreamindex: u32) -> ::windows::core::Result<()>;
    fn AllocateStreamingResources(&self) -> ::windows::core::Result<()>;
    fn FreeStreamingResources(&self) -> ::windows::core::Result<()>;
    fn GetInputStatus(&self, dwinputstreamindex: u32) -> ::windows::core::Result<u32>;
    fn ProcessInput(&self, dwinputstreamindex: u32, pbuffer: &::core::option::Option<IMediaBuffer>, dwflags: u32, rttimestamp: i64, rttimelength: i64) -> ::windows::core::Result<()>;
    fn ProcessOutput(&self, dwflags: u32, coutputbuffercount: u32, poutputbuffers: *mut DMO_OUTPUT_DATA_BUFFER, pdwstatus: *mut u32) -> ::windows::core::Result<()>;
    fn Lock(&self, block: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IMediaObject {}
#[cfg(feature = "Win32_Foundation")]
impl IMediaObject_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMediaObject_Impl, const OFFSET: isize>() -> IMediaObject_Vtbl {
        unsafe extern "system" fn GetStreamCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMediaObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcinputstreams: *mut u32, pcoutputstreams: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetStreamCount(::core::mem::transmute_copy(&pcinputstreams), ::core::mem::transmute_copy(&pcoutputstreams)).into()
        }
        unsafe extern "system" fn GetInputStreamInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMediaObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputstreamindex: u32, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetInputStreamInfo(::core::mem::transmute_copy(&dwinputstreamindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwflags, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputStreamInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMediaObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputstreamindex: u32, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetOutputStreamInfo(::core::mem::transmute_copy(&dwoutputstreamindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwflags, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMediaObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputstreamindex: u32, dwtypeindex: u32, pmt: *mut DMO_MEDIA_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetInputType(::core::mem::transmute_copy(&dwinputstreamindex), ::core::mem::transmute_copy(&dwtypeindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pmt, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMediaObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputstreamindex: u32, dwtypeindex: u32, pmt: *mut DMO_MEDIA_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetOutputType(::core::mem::transmute_copy(&dwoutputstreamindex), ::core::mem::transmute_copy(&dwtypeindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pmt, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInputType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMediaObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputstreamindex: u32, pmt: *const DMO_MEDIA_TYPE, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetInputType(::core::mem::transmute_copy(&dwinputstreamindex), ::core::mem::transmute_copy(&pmt), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn SetOutputType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMediaObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputstreamindex: u32, pmt: *const DMO_MEDIA_TYPE, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetOutputType(::core::mem::transmute_copy(&dwoutputstreamindex), ::core::mem::transmute_copy(&pmt), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetInputCurrentType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMediaObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputstreamindex: u32, pmt: *mut DMO_MEDIA_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetInputCurrentType(::core::mem::transmute_copy(&dwinputstreamindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pmt, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputCurrentType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMediaObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputstreamindex: u32, pmt: *mut DMO_MEDIA_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetOutputCurrentType(::core::mem::transmute_copy(&dwoutputstreamindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pmt, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputSizeInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMediaObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputstreamindex: u32, pcbsize: *mut u32, pcbmaxlookahead: *mut u32, pcbalignment: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetInputSizeInfo(::core::mem::transmute_copy(&dwinputstreamindex), ::core::mem::transmute_copy(&pcbsize), ::core::mem::transmute_copy(&pcbmaxlookahead), ::core::mem::transmute_copy(&pcbalignment)).into()
        }
        unsafe extern "system" fn GetOutputSizeInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMediaObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputstreamindex: u32, pcbsize: *mut u32, pcbalignment: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetOutputSizeInfo(::core::mem::transmute_copy(&dwoutputstreamindex), ::core::mem::transmute_copy(&pcbsize), ::core::mem::transmute_copy(&pcbalignment)).into()
        }
        unsafe extern "system" fn GetInputMaxLatency<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMediaObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputstreamindex: u32, prtmaxlatency: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetInputMaxLatency(::core::mem::transmute_copy(&dwinputstreamindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prtmaxlatency, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInputMaxLatency<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMediaObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputstreamindex: u32, rtmaxlatency: i64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetInputMaxLatency(::core::mem::transmute_copy(&dwinputstreamindex), ::core::mem::transmute_copy(&rtmaxlatency)).into()
        }
        unsafe extern "system" fn Flush<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMediaObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Flush().into()
        }
        unsafe extern "system" fn Discontinuity<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMediaObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputstreamindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Discontinuity(::core::mem::transmute_copy(&dwinputstreamindex)).into()
        }
        unsafe extern "system" fn AllocateStreamingResources<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMediaObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AllocateStreamingResources().into()
        }
        unsafe extern "system" fn FreeStreamingResources<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMediaObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FreeStreamingResources().into()
        }
        unsafe extern "system" fn GetInputStatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMediaObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputstreamindex: u32, dwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetInputStatus(::core::mem::transmute_copy(&dwinputstreamindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(dwflags, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProcessInput<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMediaObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputstreamindex: u32, pbuffer: *mut ::core::ffi::c_void, dwflags: u32, rttimestamp: i64, rttimelength: i64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ProcessInput(::core::mem::transmute_copy(&dwinputstreamindex), ::core::mem::transmute(&pbuffer), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&rttimestamp), ::core::mem::transmute_copy(&rttimelength)).into()
        }
        unsafe extern "system" fn ProcessOutput<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMediaObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, coutputbuffercount: u32, poutputbuffers: *mut DMO_OUTPUT_DATA_BUFFER, pdwstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ProcessOutput(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&coutputbuffercount), ::core::mem::transmute_copy(&poutputbuffers), ::core::mem::transmute_copy(&pdwstatus)).into()
        }
        unsafe extern "system" fn Lock<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMediaObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, block: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Lock(::core::mem::transmute_copy(&block)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetStreamCount: GetStreamCount::<Identity, Impl, OFFSET>,
            GetInputStreamInfo: GetInputStreamInfo::<Identity, Impl, OFFSET>,
            GetOutputStreamInfo: GetOutputStreamInfo::<Identity, Impl, OFFSET>,
            GetInputType: GetInputType::<Identity, Impl, OFFSET>,
            GetOutputType: GetOutputType::<Identity, Impl, OFFSET>,
            SetInputType: SetInputType::<Identity, Impl, OFFSET>,
            SetOutputType: SetOutputType::<Identity, Impl, OFFSET>,
            GetInputCurrentType: GetInputCurrentType::<Identity, Impl, OFFSET>,
            GetOutputCurrentType: GetOutputCurrentType::<Identity, Impl, OFFSET>,
            GetInputSizeInfo: GetInputSizeInfo::<Identity, Impl, OFFSET>,
            GetOutputSizeInfo: GetOutputSizeInfo::<Identity, Impl, OFFSET>,
            GetInputMaxLatency: GetInputMaxLatency::<Identity, Impl, OFFSET>,
            SetInputMaxLatency: SetInputMaxLatency::<Identity, Impl, OFFSET>,
            Flush: Flush::<Identity, Impl, OFFSET>,
            Discontinuity: Discontinuity::<Identity, Impl, OFFSET>,
            AllocateStreamingResources: AllocateStreamingResources::<Identity, Impl, OFFSET>,
            FreeStreamingResources: FreeStreamingResources::<Identity, Impl, OFFSET>,
            GetInputStatus: GetInputStatus::<Identity, Impl, OFFSET>,
            ProcessInput: ProcessInput::<Identity, Impl, OFFSET>,
            ProcessOutput: ProcessOutput::<Identity, Impl, OFFSET>,
            Lock: Lock::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaObject as ::windows::core::Interface>::IID
    }
}
pub trait IMediaObjectInPlace_Impl: Sized {
    fn Process(&self, ulsize: u32, pdata: *mut u8, reftimestart: i64, dwflags: u32) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IMediaObjectInPlace>;
    fn GetLatency(&self) -> ::windows::core::Result<i64>;
}
impl ::windows::core::RuntimeName for IMediaObjectInPlace {}
impl IMediaObjectInPlace_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMediaObjectInPlace_Impl, const OFFSET: isize>() -> IMediaObjectInPlace_Vtbl {
        unsafe extern "system" fn Process<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMediaObjectInPlace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulsize: u32, pdata: *mut u8, reftimestart: i64, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Process(::core::mem::transmute_copy(&ulsize), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&reftimestart), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMediaObjectInPlace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmediaobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmediaobject, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLatency<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMediaObjectInPlace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, platencytime: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetLatency() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(platencytime, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Process: Process::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
            GetLatency: GetLatency::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaObjectInPlace as ::windows::core::Interface>::IID
    }
}
