#[cfg(feature = "Win32_Media_MediaFoundation")]
pub trait IMFDeviceTransform_Impl: Sized {
    fn InitializeTransform(&self, pattributes: &::core::option::Option<super::MediaFoundation::IMFAttributes>) -> ::windows::core::Result<()>;
    fn GetInputAvailableType(&self, dwinputstreamid: u32, dwtypeindex: u32) -> ::windows::core::Result<super::MediaFoundation::IMFMediaType>;
    fn GetInputCurrentType(&self, dwinputstreamid: u32) -> ::windows::core::Result<super::MediaFoundation::IMFMediaType>;
    fn GetInputStreamAttributes(&self, dwinputstreamid: u32) -> ::windows::core::Result<super::MediaFoundation::IMFAttributes>;
    fn GetOutputAvailableType(&self, dwoutputstreamid: u32, dwtypeindex: u32) -> ::windows::core::Result<super::MediaFoundation::IMFMediaType>;
    fn GetOutputCurrentType(&self, dwoutputstreamid: u32) -> ::windows::core::Result<super::MediaFoundation::IMFMediaType>;
    fn GetOutputStreamAttributes(&self, dwoutputstreamid: u32) -> ::windows::core::Result<super::MediaFoundation::IMFAttributes>;
    fn GetStreamCount(&self, pcinputstreams: *mut u32, pcoutputstreams: *mut u32) -> ::windows::core::Result<()>;
    fn GetStreamIDs(&self, dwinputidarraysize: u32, pdwinputstreamids: *mut u32, dwoutputidarraysize: u32, pdwoutputstreamids: *mut u32) -> ::windows::core::Result<()>;
    fn ProcessEvent(&self, dwinputstreamid: u32, pevent: &::core::option::Option<super::MediaFoundation::IMFMediaEvent>) -> ::windows::core::Result<()>;
    fn ProcessInput(&self, dwinputstreamid: u32, psample: &::core::option::Option<super::MediaFoundation::IMFSample>, dwflags: u32) -> ::windows::core::Result<()>;
    fn ProcessMessage(&self, emessage: super::MediaFoundation::MFT_MESSAGE_TYPE, ulparam: usize) -> ::windows::core::Result<()>;
    fn ProcessOutput(&self, dwflags: u32, coutputbuffercount: u32, poutputsample: *mut super::MediaFoundation::MFT_OUTPUT_DATA_BUFFER, pdwstatus: *mut u32) -> ::windows::core::Result<()>;
    fn SetInputStreamState(&self, dwstreamid: u32, pmediatype: &::core::option::Option<super::MediaFoundation::IMFMediaType>, value: super::MediaFoundation::DeviceStreamState, dwflags: u32) -> ::windows::core::Result<()>;
    fn GetInputStreamState(&self, dwstreamid: u32) -> ::windows::core::Result<super::MediaFoundation::DeviceStreamState>;
    fn SetOutputStreamState(&self, dwstreamid: u32, pmediatype: &::core::option::Option<super::MediaFoundation::IMFMediaType>, value: super::MediaFoundation::DeviceStreamState, dwflags: u32) -> ::windows::core::Result<()>;
    fn GetOutputStreamState(&self, dwstreamid: u32) -> ::windows::core::Result<super::MediaFoundation::DeviceStreamState>;
    fn GetInputStreamPreferredState(&self, dwstreamid: u32, value: *mut super::MediaFoundation::DeviceStreamState, ppmediatype: *mut ::core::option::Option<super::MediaFoundation::IMFMediaType>) -> ::windows::core::Result<()>;
    fn FlushInputStream(&self, dwstreamindex: u32, dwflags: u32) -> ::windows::core::Result<()>;
    fn FlushOutputStream(&self, dwstreamindex: u32, dwflags: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Media_MediaFoundation")]
impl IMFDeviceTransform_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFDeviceTransform_Impl, const OFFSET: isize>() -> IMFDeviceTransform_Vtbl {
        unsafe extern "system" fn InitializeTransform<Identity: ::windows::core::IUnknownImpl, Impl: IMFDeviceTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pattributes: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InitializeTransform(::core::mem::transmute(&pattributes)).into()
        }
        unsafe extern "system" fn GetInputAvailableType<Identity: ::windows::core::IUnknownImpl, Impl: IMFDeviceTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputstreamid: u32, dwtypeindex: u32, pmediatype: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetInputAvailableType(::core::mem::transmute_copy(&dwinputstreamid), ::core::mem::transmute_copy(&dwtypeindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pmediatype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputCurrentType<Identity: ::windows::core::IUnknownImpl, Impl: IMFDeviceTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputstreamid: u32, pmediatype: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetInputCurrentType(::core::mem::transmute_copy(&dwinputstreamid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pmediatype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputStreamAttributes<Identity: ::windows::core::IUnknownImpl, Impl: IMFDeviceTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputstreamid: u32, ppattributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetInputStreamAttributes(::core::mem::transmute_copy(&dwinputstreamid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppattributes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputAvailableType<Identity: ::windows::core::IUnknownImpl, Impl: IMFDeviceTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputstreamid: u32, dwtypeindex: u32, pmediatype: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetOutputAvailableType(::core::mem::transmute_copy(&dwoutputstreamid), ::core::mem::transmute_copy(&dwtypeindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pmediatype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputCurrentType<Identity: ::windows::core::IUnknownImpl, Impl: IMFDeviceTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputstreamid: u32, pmediatype: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetOutputCurrentType(::core::mem::transmute_copy(&dwoutputstreamid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pmediatype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputStreamAttributes<Identity: ::windows::core::IUnknownImpl, Impl: IMFDeviceTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputstreamid: u32, ppattributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetOutputStreamAttributes(::core::mem::transmute_copy(&dwoutputstreamid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppattributes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStreamCount<Identity: ::windows::core::IUnknownImpl, Impl: IMFDeviceTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcinputstreams: *mut u32, pcoutputstreams: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetStreamCount(::core::mem::transmute_copy(&pcinputstreams), ::core::mem::transmute_copy(&pcoutputstreams)).into()
        }
        unsafe extern "system" fn GetStreamIDs<Identity: ::windows::core::IUnknownImpl, Impl: IMFDeviceTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputidarraysize: u32, pdwinputstreamids: *mut u32, dwoutputidarraysize: u32, pdwoutputstreamids: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetStreamIDs(::core::mem::transmute_copy(&dwinputidarraysize), ::core::mem::transmute_copy(&pdwinputstreamids), ::core::mem::transmute_copy(&dwoutputidarraysize), ::core::mem::transmute_copy(&pdwoutputstreamids)).into()
        }
        unsafe extern "system" fn ProcessEvent<Identity: ::windows::core::IUnknownImpl, Impl: IMFDeviceTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputstreamid: u32, pevent: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ProcessEvent(::core::mem::transmute_copy(&dwinputstreamid), ::core::mem::transmute(&pevent)).into()
        }
        unsafe extern "system" fn ProcessInput<Identity: ::windows::core::IUnknownImpl, Impl: IMFDeviceTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputstreamid: u32, psample: ::windows::core::RawPtr, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ProcessInput(::core::mem::transmute_copy(&dwinputstreamid), ::core::mem::transmute(&psample), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn ProcessMessage<Identity: ::windows::core::IUnknownImpl, Impl: IMFDeviceTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, emessage: super::MediaFoundation::MFT_MESSAGE_TYPE, ulparam: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ProcessMessage(::core::mem::transmute_copy(&emessage), ::core::mem::transmute_copy(&ulparam)).into()
        }
        unsafe extern "system" fn ProcessOutput<Identity: ::windows::core::IUnknownImpl, Impl: IMFDeviceTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, coutputbuffercount: u32, poutputsample: *mut super::MediaFoundation::MFT_OUTPUT_DATA_BUFFER, pdwstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ProcessOutput(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&coutputbuffercount), ::core::mem::transmute_copy(&poutputsample), ::core::mem::transmute_copy(&pdwstatus)).into()
        }
        unsafe extern "system" fn SetInputStreamState<Identity: ::windows::core::IUnknownImpl, Impl: IMFDeviceTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamid: u32, pmediatype: ::windows::core::RawPtr, value: super::MediaFoundation::DeviceStreamState, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetInputStreamState(::core::mem::transmute_copy(&dwstreamid), ::core::mem::transmute(&pmediatype), ::core::mem::transmute_copy(&value), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetInputStreamState<Identity: ::windows::core::IUnknownImpl, Impl: IMFDeviceTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamid: u32, value: *mut super::MediaFoundation::DeviceStreamState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetInputStreamState(::core::mem::transmute_copy(&dwstreamid)) {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutputStreamState<Identity: ::windows::core::IUnknownImpl, Impl: IMFDeviceTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamid: u32, pmediatype: ::windows::core::RawPtr, value: super::MediaFoundation::DeviceStreamState, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetOutputStreamState(::core::mem::transmute_copy(&dwstreamid), ::core::mem::transmute(&pmediatype), ::core::mem::transmute_copy(&value), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetOutputStreamState<Identity: ::windows::core::IUnknownImpl, Impl: IMFDeviceTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamid: u32, value: *mut super::MediaFoundation::DeviceStreamState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetOutputStreamState(::core::mem::transmute_copy(&dwstreamid)) {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputStreamPreferredState<Identity: ::windows::core::IUnknownImpl, Impl: IMFDeviceTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamid: u32, value: *mut super::MediaFoundation::DeviceStreamState, ppmediatype: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetInputStreamPreferredState(::core::mem::transmute_copy(&dwstreamid), ::core::mem::transmute_copy(&value), ::core::mem::transmute_copy(&ppmediatype)).into()
        }
        unsafe extern "system" fn FlushInputStream<Identity: ::windows::core::IUnknownImpl, Impl: IMFDeviceTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamindex: u32, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FlushInputStream(::core::mem::transmute_copy(&dwstreamindex), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn FlushOutputStream<Identity: ::windows::core::IUnknownImpl, Impl: IMFDeviceTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamindex: u32, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FlushOutputStream(::core::mem::transmute_copy(&dwstreamindex), ::core::mem::transmute_copy(&dwflags)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            InitializeTransform: InitializeTransform::<Identity, Impl, OFFSET>,
            GetInputAvailableType: GetInputAvailableType::<Identity, Impl, OFFSET>,
            GetInputCurrentType: GetInputCurrentType::<Identity, Impl, OFFSET>,
            GetInputStreamAttributes: GetInputStreamAttributes::<Identity, Impl, OFFSET>,
            GetOutputAvailableType: GetOutputAvailableType::<Identity, Impl, OFFSET>,
            GetOutputCurrentType: GetOutputCurrentType::<Identity, Impl, OFFSET>,
            GetOutputStreamAttributes: GetOutputStreamAttributes::<Identity, Impl, OFFSET>,
            GetStreamCount: GetStreamCount::<Identity, Impl, OFFSET>,
            GetStreamIDs: GetStreamIDs::<Identity, Impl, OFFSET>,
            ProcessEvent: ProcessEvent::<Identity, Impl, OFFSET>,
            ProcessInput: ProcessInput::<Identity, Impl, OFFSET>,
            ProcessMessage: ProcessMessage::<Identity, Impl, OFFSET>,
            ProcessOutput: ProcessOutput::<Identity, Impl, OFFSET>,
            SetInputStreamState: SetInputStreamState::<Identity, Impl, OFFSET>,
            GetInputStreamState: GetInputStreamState::<Identity, Impl, OFFSET>,
            SetOutputStreamState: SetOutputStreamState::<Identity, Impl, OFFSET>,
            GetOutputStreamState: GetOutputStreamState::<Identity, Impl, OFFSET>,
            GetInputStreamPreferredState: GetInputStreamPreferredState::<Identity, Impl, OFFSET>,
            FlushInputStream: FlushInputStream::<Identity, Impl, OFFSET>,
            FlushOutputStream: FlushOutputStream::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFDeviceTransform as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Media_MediaFoundation")]
pub trait IMFDeviceTransformCallback_Impl: Sized {
    fn OnBufferSent(&self, pcallbackattributes: &::core::option::Option<super::MediaFoundation::IMFAttributes>, pinid: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Media_MediaFoundation")]
impl IMFDeviceTransformCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFDeviceTransformCallback_Impl, const OFFSET: isize>() -> IMFDeviceTransformCallback_Vtbl {
        unsafe extern "system" fn OnBufferSent<Identity: ::windows::core::IUnknownImpl, Impl: IMFDeviceTransformCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallbackattributes: ::windows::core::RawPtr, pinid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnBufferSent(::core::mem::transmute(&pcallbackattributes), ::core::mem::transmute_copy(&pinid)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), OnBufferSent: OnBufferSent::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFDeviceTransformCallback as ::windows::core::Interface>::IID
    }
}
