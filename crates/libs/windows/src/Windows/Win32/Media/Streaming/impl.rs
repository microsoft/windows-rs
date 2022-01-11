#[cfg(feature = "Win32_Media_MediaFoundation")]
pub trait IMFDeviceTransformImpl: Sized {
    fn InitializeTransform();
    fn GetInputAvailableType();
    fn GetInputCurrentType();
    fn GetInputStreamAttributes();
    fn GetOutputAvailableType();
    fn GetOutputCurrentType();
    fn GetOutputStreamAttributes();
    fn GetStreamCount();
    fn GetStreamIDs();
    fn ProcessEvent();
    fn ProcessInput();
    fn ProcessMessage();
    fn ProcessOutput();
    fn SetInputStreamState();
    fn GetInputStreamState();
    fn SetOutputStreamState();
    fn GetOutputStreamState();
    fn GetInputStreamPreferredState();
    fn FlushInputStream();
    fn FlushOutputStream();
}
#[cfg(feature = "Win32_Media_MediaFoundation")]
impl IMFDeviceTransformVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFDeviceTransformImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFDeviceTransformVtbl {
        unsafe extern "system" fn InitializeTransform<Impl: IMFDeviceTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pattributes: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetInputAvailableType<Impl: IMFDeviceTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputstreamid: u32, dwtypeindex: u32, pmediatype: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetInputCurrentType<Impl: IMFDeviceTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputstreamid: u32, pmediatype: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetInputStreamAttributes<Impl: IMFDeviceTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputstreamid: u32, ppattributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOutputAvailableType<Impl: IMFDeviceTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputstreamid: u32, dwtypeindex: u32, pmediatype: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOutputCurrentType<Impl: IMFDeviceTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputstreamid: u32, pmediatype: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOutputStreamAttributes<Impl: IMFDeviceTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputstreamid: u32, ppattributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStreamCount<Impl: IMFDeviceTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcinputstreams: *mut u32, pcoutputstreams: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStreamIDs<Impl: IMFDeviceTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputidarraysize: u32, pdwinputstreamids: *mut u32, dwoutputidarraysize: u32, pdwoutputstreamids: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ProcessEvent<Impl: IMFDeviceTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputstreamid: u32, pevent: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ProcessInput<Impl: IMFDeviceTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputstreamid: u32, psample: ::windows::core::RawPtr, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ProcessMessage<Impl: IMFDeviceTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, emessage: super::MediaFoundation::MFT_MESSAGE_TYPE, ulparam: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ProcessOutput<Impl: IMFDeviceTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, coutputbuffercount: u32, poutputsample: *mut super::MediaFoundation::MFT_OUTPUT_DATA_BUFFER, pdwstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetInputStreamState<Impl: IMFDeviceTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamid: u32, pmediatype: ::windows::core::RawPtr, value: super::MediaFoundation::DeviceStreamState, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetInputStreamState<Impl: IMFDeviceTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamid: u32, value: *mut super::MediaFoundation::DeviceStreamState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOutputStreamState<Impl: IMFDeviceTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamid: u32, pmediatype: ::windows::core::RawPtr, value: super::MediaFoundation::DeviceStreamState, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOutputStreamState<Impl: IMFDeviceTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamid: u32, value: *mut super::MediaFoundation::DeviceStreamState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetInputStreamPreferredState<Impl: IMFDeviceTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamid: u32, value: *mut super::MediaFoundation::DeviceStreamState, ppmediatype: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FlushInputStream<Impl: IMFDeviceTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamindex: u32, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FlushOutputStream<Impl: IMFDeviceTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamindex: u32, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            InitializeTransform::<Impl, IMPL_OFFSET>,
            GetInputAvailableType::<Impl, IMPL_OFFSET>,
            GetInputCurrentType::<Impl, IMPL_OFFSET>,
            GetInputStreamAttributes::<Impl, IMPL_OFFSET>,
            GetOutputAvailableType::<Impl, IMPL_OFFSET>,
            GetOutputCurrentType::<Impl, IMPL_OFFSET>,
            GetOutputStreamAttributes::<Impl, IMPL_OFFSET>,
            GetStreamCount::<Impl, IMPL_OFFSET>,
            GetStreamIDs::<Impl, IMPL_OFFSET>,
            ProcessEvent::<Impl, IMPL_OFFSET>,
            ProcessInput::<Impl, IMPL_OFFSET>,
            ProcessMessage::<Impl, IMPL_OFFSET>,
            ProcessOutput::<Impl, IMPL_OFFSET>,
            SetInputStreamState::<Impl, IMPL_OFFSET>,
            GetInputStreamState::<Impl, IMPL_OFFSET>,
            SetOutputStreamState::<Impl, IMPL_OFFSET>,
            GetOutputStreamState::<Impl, IMPL_OFFSET>,
            GetInputStreamPreferredState::<Impl, IMPL_OFFSET>,
            FlushInputStream::<Impl, IMPL_OFFSET>,
            FlushOutputStream::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFDeviceTransform as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Media_MediaFoundation")]
pub trait IMFDeviceTransformCallbackImpl: Sized {
    fn OnBufferSent();
}
#[cfg(feature = "Win32_Media_MediaFoundation")]
impl IMFDeviceTransformCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFDeviceTransformCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMFDeviceTransformCallbackVtbl {
        unsafe extern "system" fn OnBufferSent<Impl: IMFDeviceTransformCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallbackattributes: ::windows::core::RawPtr, pinid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, OnBufferSent::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMFDeviceTransformCallback as ::windows::core::Interface>::IID
    }
}
