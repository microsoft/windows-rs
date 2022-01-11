pub trait IDMOQualityControlImpl: Sized {
    fn SetNow();
    fn SetStatus();
    fn GetStatus();
}
impl IDMOQualityControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDMOQualityControlImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDMOQualityControlVtbl {
        unsafe extern "system" fn SetNow<Impl: IDMOQualityControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rtnow: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetStatus<Impl: IDMOQualityControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStatus<Impl: IDMOQualityControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SetNow::<Impl, IMPL_OFFSET>, SetStatus::<Impl, IMPL_OFFSET>, GetStatus::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDMOQualityControl as ::windows::core::Interface>::IID
    }
}
pub trait IDMOVideoOutputOptimizationsImpl: Sized {
    fn QueryOperationModePreferences();
    fn SetOperationMode();
    fn GetCurrentOperationMode();
    fn GetCurrentSampleRequirements();
}
impl IDMOVideoOutputOptimizationsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDMOVideoOutputOptimizationsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDMOVideoOutputOptimizationsVtbl {
        unsafe extern "system" fn QueryOperationModePreferences<Impl: IDMOVideoOutputOptimizationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uloutputstreamindex: u32, pdwrequestedcapabilities: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOperationMode<Impl: IDMOVideoOutputOptimizationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uloutputstreamindex: u32, dwenabledfeatures: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCurrentOperationMode<Impl: IDMOVideoOutputOptimizationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uloutputstreamindex: u32, pdwenabledfeatures: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCurrentSampleRequirements<Impl: IDMOVideoOutputOptimizationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uloutputstreamindex: u32, pdwrequestedfeatures: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, QueryOperationModePreferences::<Impl, IMPL_OFFSET>, SetOperationMode::<Impl, IMPL_OFFSET>, GetCurrentOperationMode::<Impl, IMPL_OFFSET>, GetCurrentSampleRequirements::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDMOVideoOutputOptimizations as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IEnumDMOImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
#[cfg(feature = "Win32_Foundation")]
impl IEnumDMOVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumDMOImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumDMOVtbl {
        unsafe extern "system" fn Next<Impl: IEnumDMOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, citemstofetch: u32, pclsid: *mut ::windows::core::GUID, names: *mut super::super::Foundation::PWSTR, pcitemsfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumDMOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, citemstoskip: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumDMOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IEnumDMOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Next::<Impl, IMPL_OFFSET>, Skip::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>, Clone::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumDMO as ::windows::core::Interface>::IID
    }
}
pub trait IMediaBufferImpl: Sized {
    fn SetLength();
    fn GetMaxLength();
    fn GetBufferAndLength();
}
impl IMediaBufferVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaBufferImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaBufferVtbl {
        unsafe extern "system" fn SetLength<Impl: IMediaBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cblength: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMaxLength<Impl: IMediaBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbmaxlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBufferAndLength<Impl: IMediaBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppbuffer: *mut *mut u8, pcblength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SetLength::<Impl, IMPL_OFFSET>, GetMaxLength::<Impl, IMPL_OFFSET>, GetBufferAndLength::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaBuffer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMediaObjectImpl: Sized {
    fn GetStreamCount();
    fn GetInputStreamInfo();
    fn GetOutputStreamInfo();
    fn GetInputType();
    fn GetOutputType();
    fn SetInputType();
    fn SetOutputType();
    fn GetInputCurrentType();
    fn GetOutputCurrentType();
    fn GetInputSizeInfo();
    fn GetOutputSizeInfo();
    fn GetInputMaxLatency();
    fn SetInputMaxLatency();
    fn Flush();
    fn Discontinuity();
    fn AllocateStreamingResources();
    fn FreeStreamingResources();
    fn GetInputStatus();
    fn ProcessInput();
    fn ProcessOutput();
    fn Lock();
}
#[cfg(feature = "Win32_Foundation")]
impl IMediaObjectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaObjectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaObjectVtbl {
        unsafe extern "system" fn GetStreamCount<Impl: IMediaObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcinputstreams: *mut u32, pcoutputstreams: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetInputStreamInfo<Impl: IMediaObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputstreamindex: u32, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOutputStreamInfo<Impl: IMediaObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputstreamindex: u32, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetInputType<Impl: IMediaObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputstreamindex: u32, dwtypeindex: u32, pmt: *mut DMO_MEDIA_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOutputType<Impl: IMediaObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputstreamindex: u32, dwtypeindex: u32, pmt: *mut DMO_MEDIA_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetInputType<Impl: IMediaObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputstreamindex: u32, pmt: *const DMO_MEDIA_TYPE, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOutputType<Impl: IMediaObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputstreamindex: u32, pmt: *const DMO_MEDIA_TYPE, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetInputCurrentType<Impl: IMediaObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputstreamindex: u32, pmt: *mut DMO_MEDIA_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOutputCurrentType<Impl: IMediaObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputstreamindex: u32, pmt: *mut DMO_MEDIA_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetInputSizeInfo<Impl: IMediaObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputstreamindex: u32, pcbsize: *mut u32, pcbmaxlookahead: *mut u32, pcbalignment: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOutputSizeInfo<Impl: IMediaObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputstreamindex: u32, pcbsize: *mut u32, pcbalignment: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetInputMaxLatency<Impl: IMediaObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputstreamindex: u32, prtmaxlatency: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetInputMaxLatency<Impl: IMediaObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputstreamindex: u32, rtmaxlatency: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Flush<Impl: IMediaObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Discontinuity<Impl: IMediaObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputstreamindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AllocateStreamingResources<Impl: IMediaObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FreeStreamingResources<Impl: IMediaObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetInputStatus<Impl: IMediaObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputstreamindex: u32, dwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ProcessInput<Impl: IMediaObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputstreamindex: u32, pbuffer: ::windows::core::RawPtr, dwflags: u32, rttimestamp: i64, rttimelength: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ProcessOutput<Impl: IMediaObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, coutputbuffercount: u32, poutputbuffers: *mut DMO_OUTPUT_DATA_BUFFER, pdwstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Lock<Impl: IMediaObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, block: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetStreamCount::<Impl, IMPL_OFFSET>,
            GetInputStreamInfo::<Impl, IMPL_OFFSET>,
            GetOutputStreamInfo::<Impl, IMPL_OFFSET>,
            GetInputType::<Impl, IMPL_OFFSET>,
            GetOutputType::<Impl, IMPL_OFFSET>,
            SetInputType::<Impl, IMPL_OFFSET>,
            SetOutputType::<Impl, IMPL_OFFSET>,
            GetInputCurrentType::<Impl, IMPL_OFFSET>,
            GetOutputCurrentType::<Impl, IMPL_OFFSET>,
            GetInputSizeInfo::<Impl, IMPL_OFFSET>,
            GetOutputSizeInfo::<Impl, IMPL_OFFSET>,
            GetInputMaxLatency::<Impl, IMPL_OFFSET>,
            SetInputMaxLatency::<Impl, IMPL_OFFSET>,
            Flush::<Impl, IMPL_OFFSET>,
            Discontinuity::<Impl, IMPL_OFFSET>,
            AllocateStreamingResources::<Impl, IMPL_OFFSET>,
            FreeStreamingResources::<Impl, IMPL_OFFSET>,
            GetInputStatus::<Impl, IMPL_OFFSET>,
            ProcessInput::<Impl, IMPL_OFFSET>,
            ProcessOutput::<Impl, IMPL_OFFSET>,
            Lock::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaObject as ::windows::core::Interface>::IID
    }
}
pub trait IMediaObjectInPlaceImpl: Sized {
    fn Process();
    fn Clone();
    fn GetLatency();
}
impl IMediaObjectInPlaceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaObjectInPlaceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaObjectInPlaceVtbl {
        unsafe extern "system" fn Process<Impl: IMediaObjectInPlaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulsize: u32, pdata: *mut u8, reftimestart: i64, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IMediaObjectInPlaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmediaobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLatency<Impl: IMediaObjectInPlaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, platencytime: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Process::<Impl, IMPL_OFFSET>, Clone::<Impl, IMPL_OFFSET>, GetLatency::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaObjectInPlace as ::windows::core::Interface>::IID
    }
}
