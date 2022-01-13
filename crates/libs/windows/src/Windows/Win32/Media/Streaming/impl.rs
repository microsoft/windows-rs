#[cfg(feature = "Win32_Media_MediaFoundation")]
pub trait IMFDeviceTransformImpl: Sized {
    fn InitializeTransform(&mut self, pattributes: ::core::option::Option<super::MediaFoundation::IMFAttributes>) -> ::windows::core::Result<()>;
    fn GetInputAvailableType(&mut self, dwinputstreamid: u32, dwtypeindex: u32) -> ::windows::core::Result<super::MediaFoundation::IMFMediaType>;
    fn GetInputCurrentType(&mut self, dwinputstreamid: u32) -> ::windows::core::Result<super::MediaFoundation::IMFMediaType>;
    fn GetInputStreamAttributes(&mut self, dwinputstreamid: u32) -> ::windows::core::Result<super::MediaFoundation::IMFAttributes>;
    fn GetOutputAvailableType(&mut self, dwoutputstreamid: u32, dwtypeindex: u32) -> ::windows::core::Result<super::MediaFoundation::IMFMediaType>;
    fn GetOutputCurrentType(&mut self, dwoutputstreamid: u32) -> ::windows::core::Result<super::MediaFoundation::IMFMediaType>;
    fn GetOutputStreamAttributes(&mut self, dwoutputstreamid: u32) -> ::windows::core::Result<super::MediaFoundation::IMFAttributes>;
    fn GetStreamCount(&mut self, pcinputstreams: *mut u32, pcoutputstreams: *mut u32) -> ::windows::core::Result<()>;
    fn GetStreamIDs(&mut self, dwinputidarraysize: u32, pdwinputstreamids: *mut u32, dwoutputidarraysize: u32, pdwoutputstreamids: *mut u32) -> ::windows::core::Result<()>;
    fn ProcessEvent(&mut self, dwinputstreamid: u32, pevent: ::core::option::Option<super::MediaFoundation::IMFMediaEvent>) -> ::windows::core::Result<()>;
    fn ProcessInput(&mut self, dwinputstreamid: u32, psample: ::core::option::Option<super::MediaFoundation::IMFSample>, dwflags: u32) -> ::windows::core::Result<()>;
    fn ProcessMessage(&mut self, emessage: super::MediaFoundation::MFT_MESSAGE_TYPE, ulparam: usize) -> ::windows::core::Result<()>;
    fn ProcessOutput(&mut self, dwflags: u32, coutputbuffercount: u32, poutputsample: *mut super::MediaFoundation::MFT_OUTPUT_DATA_BUFFER, pdwstatus: *mut u32) -> ::windows::core::Result<()>;
    fn SetInputStreamState(&mut self, dwstreamid: u32, pmediatype: ::core::option::Option<super::MediaFoundation::IMFMediaType>, value: super::MediaFoundation::DeviceStreamState, dwflags: u32) -> ::windows::core::Result<()>;
    fn GetInputStreamState(&mut self, dwstreamid: u32) -> ::windows::core::Result<super::MediaFoundation::DeviceStreamState>;
    fn SetOutputStreamState(&mut self, dwstreamid: u32, pmediatype: ::core::option::Option<super::MediaFoundation::IMFMediaType>, value: super::MediaFoundation::DeviceStreamState, dwflags: u32) -> ::windows::core::Result<()>;
    fn GetOutputStreamState(&mut self, dwstreamid: u32) -> ::windows::core::Result<super::MediaFoundation::DeviceStreamState>;
    fn GetInputStreamPreferredState(&mut self, dwstreamid: u32, value: *mut super::MediaFoundation::DeviceStreamState, ppmediatype: *mut ::core::option::Option<super::MediaFoundation::IMFMediaType>) -> ::windows::core::Result<()>;
    fn FlushInputStream(&mut self, dwstreamindex: u32, dwflags: u32) -> ::windows::core::Result<()>;
    fn FlushOutputStream(&mut self, dwstreamindex: u32, dwflags: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Media_MediaFoundation")]
impl IMFDeviceTransformVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFDeviceTransformImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFDeviceTransformVtbl {
        unsafe extern "system" fn InitializeTransform<Impl: IMFDeviceTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pattributes: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeTransform(::core::mem::transmute(&pattributes)).into()
        }
        unsafe extern "system" fn GetInputAvailableType<Impl: IMFDeviceTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputstreamid: u32, dwtypeindex: u32, pmediatype: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInputAvailableType(::core::mem::transmute_copy(&dwinputstreamid), ::core::mem::transmute_copy(&dwtypeindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pmediatype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputCurrentType<Impl: IMFDeviceTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputstreamid: u32, pmediatype: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInputCurrentType(::core::mem::transmute_copy(&dwinputstreamid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pmediatype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputStreamAttributes<Impl: IMFDeviceTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputstreamid: u32, ppattributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInputStreamAttributes(::core::mem::transmute_copy(&dwinputstreamid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppattributes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputAvailableType<Impl: IMFDeviceTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputstreamid: u32, dwtypeindex: u32, pmediatype: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOutputAvailableType(::core::mem::transmute_copy(&dwoutputstreamid), ::core::mem::transmute_copy(&dwtypeindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pmediatype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputCurrentType<Impl: IMFDeviceTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputstreamid: u32, pmediatype: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOutputCurrentType(::core::mem::transmute_copy(&dwoutputstreamid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pmediatype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputStreamAttributes<Impl: IMFDeviceTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputstreamid: u32, ppattributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOutputStreamAttributes(::core::mem::transmute_copy(&dwoutputstreamid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppattributes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStreamCount<Impl: IMFDeviceTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcinputstreams: *mut u32, pcoutputstreams: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetStreamCount(::core::mem::transmute_copy(&pcinputstreams), ::core::mem::transmute_copy(&pcoutputstreams)).into()
        }
        unsafe extern "system" fn GetStreamIDs<Impl: IMFDeviceTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputidarraysize: u32, pdwinputstreamids: *mut u32, dwoutputidarraysize: u32, pdwoutputstreamids: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetStreamIDs(::core::mem::transmute_copy(&dwinputidarraysize), ::core::mem::transmute_copy(&pdwinputstreamids), ::core::mem::transmute_copy(&dwoutputidarraysize), ::core::mem::transmute_copy(&pdwoutputstreamids)).into()
        }
        unsafe extern "system" fn ProcessEvent<Impl: IMFDeviceTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputstreamid: u32, pevent: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ProcessEvent(::core::mem::transmute_copy(&dwinputstreamid), ::core::mem::transmute(&pevent)).into()
        }
        unsafe extern "system" fn ProcessInput<Impl: IMFDeviceTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputstreamid: u32, psample: ::windows::core::RawPtr, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ProcessInput(::core::mem::transmute_copy(&dwinputstreamid), ::core::mem::transmute(&psample), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn ProcessMessage<Impl: IMFDeviceTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, emessage: super::MediaFoundation::MFT_MESSAGE_TYPE, ulparam: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ProcessMessage(::core::mem::transmute_copy(&emessage), ::core::mem::transmute_copy(&ulparam)).into()
        }
        unsafe extern "system" fn ProcessOutput<Impl: IMFDeviceTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, coutputbuffercount: u32, poutputsample: *mut super::MediaFoundation::MFT_OUTPUT_DATA_BUFFER, pdwstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ProcessOutput(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&coutputbuffercount), ::core::mem::transmute_copy(&poutputsample), ::core::mem::transmute_copy(&pdwstatus)).into()
        }
        unsafe extern "system" fn SetInputStreamState<Impl: IMFDeviceTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamid: u32, pmediatype: ::windows::core::RawPtr, value: super::MediaFoundation::DeviceStreamState, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInputStreamState(::core::mem::transmute_copy(&dwstreamid), ::core::mem::transmute(&pmediatype), ::core::mem::transmute_copy(&value), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetInputStreamState<Impl: IMFDeviceTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamid: u32, value: *mut super::MediaFoundation::DeviceStreamState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInputStreamState(::core::mem::transmute_copy(&dwstreamid)) {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutputStreamState<Impl: IMFDeviceTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamid: u32, pmediatype: ::windows::core::RawPtr, value: super::MediaFoundation::DeviceStreamState, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOutputStreamState(::core::mem::transmute_copy(&dwstreamid), ::core::mem::transmute(&pmediatype), ::core::mem::transmute_copy(&value), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetOutputStreamState<Impl: IMFDeviceTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamid: u32, value: *mut super::MediaFoundation::DeviceStreamState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOutputStreamState(::core::mem::transmute_copy(&dwstreamid)) {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputStreamPreferredState<Impl: IMFDeviceTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamid: u32, value: *mut super::MediaFoundation::DeviceStreamState, ppmediatype: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetInputStreamPreferredState(::core::mem::transmute_copy(&dwstreamid), ::core::mem::transmute_copy(&value), ::core::mem::transmute_copy(&ppmediatype)).into()
        }
        unsafe extern "system" fn FlushInputStream<Impl: IMFDeviceTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamindex: u32, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FlushInputStream(::core::mem::transmute_copy(&dwstreamindex), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn FlushOutputStream<Impl: IMFDeviceTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamindex: u32, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FlushOutputStream(::core::mem::transmute_copy(&dwstreamindex), ::core::mem::transmute_copy(&dwflags)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            InitializeTransform: InitializeTransform::<Impl, IMPL_OFFSET>,
            GetInputAvailableType: GetInputAvailableType::<Impl, IMPL_OFFSET>,
            GetInputCurrentType: GetInputCurrentType::<Impl, IMPL_OFFSET>,
            GetInputStreamAttributes: GetInputStreamAttributes::<Impl, IMPL_OFFSET>,
            GetOutputAvailableType: GetOutputAvailableType::<Impl, IMPL_OFFSET>,
            GetOutputCurrentType: GetOutputCurrentType::<Impl, IMPL_OFFSET>,
            GetOutputStreamAttributes: GetOutputStreamAttributes::<Impl, IMPL_OFFSET>,
            GetStreamCount: GetStreamCount::<Impl, IMPL_OFFSET>,
            GetStreamIDs: GetStreamIDs::<Impl, IMPL_OFFSET>,
            ProcessEvent: ProcessEvent::<Impl, IMPL_OFFSET>,
            ProcessInput: ProcessInput::<Impl, IMPL_OFFSET>,
            ProcessMessage: ProcessMessage::<Impl, IMPL_OFFSET>,
            ProcessOutput: ProcessOutput::<Impl, IMPL_OFFSET>,
            SetInputStreamState: SetInputStreamState::<Impl, IMPL_OFFSET>,
            GetInputStreamState: GetInputStreamState::<Impl, IMPL_OFFSET>,
            SetOutputStreamState: SetOutputStreamState::<Impl, IMPL_OFFSET>,
            GetOutputStreamState: GetOutputStreamState::<Impl, IMPL_OFFSET>,
            GetInputStreamPreferredState: GetInputStreamPreferredState::<Impl, IMPL_OFFSET>,
            FlushInputStream: FlushInputStream::<Impl, IMPL_OFFSET>,
            FlushOutputStream: FlushOutputStream::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFDeviceTransform as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Media_MediaFoundation")]
pub trait IMFDeviceTransformCallbackImpl: Sized {
    fn OnBufferSent(&mut self, pcallbackattributes: ::core::option::Option<super::MediaFoundation::IMFAttributes>, pinid: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Media_MediaFoundation")]
impl IMFDeviceTransformCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFDeviceTransformCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFDeviceTransformCallbackVtbl {
        unsafe extern "system" fn OnBufferSent<Impl: IMFDeviceTransformCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallbackattributes: ::windows::core::RawPtr, pinid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnBufferSent(::core::mem::transmute(&pcallbackattributes), ::core::mem::transmute_copy(&pinid)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnBufferSent: OnBufferSent::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFDeviceTransformCallback as ::windows::core::Interface>::IID
    }
}
