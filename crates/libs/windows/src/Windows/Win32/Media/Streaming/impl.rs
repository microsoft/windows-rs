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
impl ::windows::core::RuntimeName for IMFDeviceTransform {
    const NAME: &'static str = "Windows.Win32.Media.Streaming.IMFDeviceTransform";
}
impl IMFDeviceTransformVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFDeviceTransformImpl, const OFFSET: isize>() -> IMFDeviceTransformVtbl {
        unsafe extern "system" fn InitializeTransform<Impl: IMFDeviceTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pattributes: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitializeTransform(&*(&pattributes as *const <super::MediaFoundation::IMFAttributes as ::windows::core::Abi>::Abi as *const <super::MediaFoundation::IMFAttributes as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputAvailableType<Impl: IMFDeviceTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputstreamid: u32, dwtypeindex: u32, pmediatype: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInputAvailableType(dwinputstreamid, dwtypeindex, ::core::mem::transmute_copy(&pmediatype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputCurrentType<Impl: IMFDeviceTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputstreamid: u32, pmediatype: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInputCurrentType(dwinputstreamid, ::core::mem::transmute_copy(&pmediatype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputStreamAttributes<Impl: IMFDeviceTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputstreamid: u32, ppattributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInputStreamAttributes(dwinputstreamid, ::core::mem::transmute_copy(&ppattributes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputAvailableType<Impl: IMFDeviceTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputstreamid: u32, dwtypeindex: u32, pmediatype: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOutputAvailableType(dwoutputstreamid, dwtypeindex, ::core::mem::transmute_copy(&pmediatype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputCurrentType<Impl: IMFDeviceTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputstreamid: u32, pmediatype: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOutputCurrentType(dwoutputstreamid, ::core::mem::transmute_copy(&pmediatype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputStreamAttributes<Impl: IMFDeviceTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputstreamid: u32, ppattributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOutputStreamAttributes(dwoutputstreamid, ::core::mem::transmute_copy(&ppattributes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStreamCount<Impl: IMFDeviceTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcinputstreams: *mut u32, pcoutputstreams: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStreamCount(::core::mem::transmute_copy(&pcinputstreams), ::core::mem::transmute_copy(&pcoutputstreams)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStreamIDs<Impl: IMFDeviceTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputidarraysize: u32, pdwinputstreamids: *mut u32, dwoutputidarraysize: u32, pdwoutputstreamids: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStreamIDs(dwinputidarraysize, ::core::mem::transmute_copy(&pdwinputstreamids), dwoutputidarraysize, ::core::mem::transmute_copy(&pdwoutputstreamids)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProcessEvent<Impl: IMFDeviceTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputstreamid: u32, pevent: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProcessEvent(dwinputstreamid, &*(&pevent as *const <super::MediaFoundation::IMFMediaEvent as ::windows::core::Abi>::Abi as *const <super::MediaFoundation::IMFMediaEvent as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProcessInput<Impl: IMFDeviceTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputstreamid: u32, psample: ::windows::core::RawPtr, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProcessInput(dwinputstreamid, &*(&psample as *const <super::MediaFoundation::IMFSample as ::windows::core::Abi>::Abi as *const <super::MediaFoundation::IMFSample as ::windows::core::DefaultType>::DefaultType), dwflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProcessMessage<Impl: IMFDeviceTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, emessage: super::MediaFoundation::MFT_MESSAGE_TYPE, ulparam: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProcessMessage(emessage, ulparam) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProcessOutput<Impl: IMFDeviceTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, coutputbuffercount: u32, poutputsample: *mut super::MediaFoundation::MFT_OUTPUT_DATA_BUFFER, pdwstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProcessOutput(dwflags, coutputbuffercount, &*(&poutputsample as *const <super::MediaFoundation::MFT_OUTPUT_DATA_BUFFER as ::windows::core::Abi>::Abi as *const <super::MediaFoundation::MFT_OUTPUT_DATA_BUFFER as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pdwstatus)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInputStreamState<Impl: IMFDeviceTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamid: u32, pmediatype: ::windows::core::RawPtr, value: super::MediaFoundation::DeviceStreamState, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetInputStreamState(dwstreamid, &*(&pmediatype as *const <super::MediaFoundation::IMFMediaType as ::windows::core::Abi>::Abi as *const <super::MediaFoundation::IMFMediaType as ::windows::core::DefaultType>::DefaultType), value, dwflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputStreamState<Impl: IMFDeviceTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamid: u32, value: *mut super::MediaFoundation::DeviceStreamState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInputStreamState(dwstreamid, ::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutputStreamState<Impl: IMFDeviceTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamid: u32, pmediatype: ::windows::core::RawPtr, value: super::MediaFoundation::DeviceStreamState, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetOutputStreamState(dwstreamid, &*(&pmediatype as *const <super::MediaFoundation::IMFMediaType as ::windows::core::Abi>::Abi as *const <super::MediaFoundation::IMFMediaType as ::windows::core::DefaultType>::DefaultType), value, dwflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputStreamState<Impl: IMFDeviceTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamid: u32, value: *mut super::MediaFoundation::DeviceStreamState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOutputStreamState(dwstreamid, ::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputStreamPreferredState<Impl: IMFDeviceTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamid: u32, value: *mut super::MediaFoundation::DeviceStreamState, ppmediatype: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInputStreamPreferredState(dwstreamid, ::core::mem::transmute_copy(&value), ::core::mem::transmute_copy(&ppmediatype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FlushInputStream<Impl: IMFDeviceTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamindex: u32, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FlushInputStream(dwstreamindex, dwflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FlushOutputStream<Impl: IMFDeviceTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamindex: u32, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FlushOutputStream(dwstreamindex, dwflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IMFDeviceTransform>,
            ::windows::core::GetTrustLevel,
            InitializeTransform::<Impl, OFFSET>,
            GetInputAvailableType::<Impl, OFFSET>,
            GetInputCurrentType::<Impl, OFFSET>,
            GetInputStreamAttributes::<Impl, OFFSET>,
            GetOutputAvailableType::<Impl, OFFSET>,
            GetOutputCurrentType::<Impl, OFFSET>,
            GetOutputStreamAttributes::<Impl, OFFSET>,
            GetStreamCount::<Impl, OFFSET>,
            GetStreamIDs::<Impl, OFFSET>,
            ProcessEvent::<Impl, OFFSET>,
            ProcessInput::<Impl, OFFSET>,
            ProcessMessage::<Impl, OFFSET>,
            ProcessOutput::<Impl, OFFSET>,
            SetInputStreamState::<Impl, OFFSET>,
            GetInputStreamState::<Impl, OFFSET>,
            SetOutputStreamState::<Impl, OFFSET>,
            GetOutputStreamState::<Impl, OFFSET>,
            GetInputStreamPreferredState::<Impl, OFFSET>,
            FlushInputStream::<Impl, OFFSET>,
            FlushOutputStream::<Impl, OFFSET>,
        )
    }
}
pub trait IMFDeviceTransformCallbackImpl: Sized {
    fn OnBufferSent();
}
impl ::windows::core::RuntimeName for IMFDeviceTransformCallback {
    const NAME: &'static str = "Windows.Win32.Media.Streaming.IMFDeviceTransformCallback";
}
impl IMFDeviceTransformCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMFDeviceTransformCallbackImpl, const OFFSET: isize>() -> IMFDeviceTransformCallbackVtbl {
        unsafe extern "system" fn OnBufferSent<Impl: IMFDeviceTransformCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallbackattributes: ::windows::core::RawPtr, pinid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnBufferSent(&*(&pcallbackattributes as *const <super::MediaFoundation::IMFAttributes as ::windows::core::Abi>::Abi as *const <super::MediaFoundation::IMFAttributes as ::windows::core::DefaultType>::DefaultType), pinid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMFDeviceTransformCallback>, ::windows::core::GetTrustLevel, OnBufferSent::<Impl, OFFSET>)
    }
}
