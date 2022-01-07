pub trait IDMOQualityControlImpl: Sized {
    fn SetNow();
    fn SetStatus();
    fn GetStatus();
}
impl ::windows::core::RuntimeName for IDMOQualityControl {
    const NAME: &'static str = "Windows.Win32.Media.DxMediaObjects.IDMOQualityControl";
}
impl IDMOQualityControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDMOQualityControlImpl, const OFFSET: isize>() -> IDMOQualityControlVtbl {
        unsafe extern "system" fn SetNow<Impl: IDMOQualityControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rtnow: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetNow(rtnow) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStatus<Impl: IDMOQualityControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetStatus(dwflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Impl: IDMOQualityControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStatus(::core::mem::transmute_copy(&pdwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDMOQualityControl>, ::windows::core::GetTrustLevel, SetNow::<Impl, OFFSET>, SetStatus::<Impl, OFFSET>, GetStatus::<Impl, OFFSET>)
    }
}
pub trait IDMOVideoOutputOptimizationsImpl: Sized {
    fn QueryOperationModePreferences();
    fn SetOperationMode();
    fn GetCurrentOperationMode();
    fn GetCurrentSampleRequirements();
}
impl ::windows::core::RuntimeName for IDMOVideoOutputOptimizations {
    const NAME: &'static str = "Windows.Win32.Media.DxMediaObjects.IDMOVideoOutputOptimizations";
}
impl IDMOVideoOutputOptimizationsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDMOVideoOutputOptimizationsImpl, const OFFSET: isize>() -> IDMOVideoOutputOptimizationsVtbl {
        unsafe extern "system" fn QueryOperationModePreferences<Impl: IDMOVideoOutputOptimizationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uloutputstreamindex: u32, pdwrequestedcapabilities: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryOperationModePreferences(uloutputstreamindex, ::core::mem::transmute_copy(&pdwrequestedcapabilities)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOperationMode<Impl: IDMOVideoOutputOptimizationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uloutputstreamindex: u32, dwenabledfeatures: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetOperationMode(uloutputstreamindex, dwenabledfeatures) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentOperationMode<Impl: IDMOVideoOutputOptimizationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uloutputstreamindex: u32, pdwenabledfeatures: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentOperationMode(uloutputstreamindex, ::core::mem::transmute_copy(&pdwenabledfeatures)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentSampleRequirements<Impl: IDMOVideoOutputOptimizationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uloutputstreamindex: u32, pdwrequestedfeatures: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentSampleRequirements(uloutputstreamindex, ::core::mem::transmute_copy(&pdwrequestedfeatures)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDMOVideoOutputOptimizations>, ::windows::core::GetTrustLevel, QueryOperationModePreferences::<Impl, OFFSET>, SetOperationMode::<Impl, OFFSET>, GetCurrentOperationMode::<Impl, OFFSET>, GetCurrentSampleRequirements::<Impl, OFFSET>)
    }
}
pub trait IEnumDMOImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl ::windows::core::RuntimeName for IEnumDMO {
    const NAME: &'static str = "Windows.Win32.Media.DxMediaObjects.IEnumDMO";
}
impl IEnumDMOVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumDMOImpl, const OFFSET: isize>() -> IEnumDMOVtbl {
        unsafe extern "system" fn Next<Impl: IEnumDMOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, citemstofetch: u32, pclsid: *mut ::windows::core::GUID, names: *mut super::super::Foundation::PWSTR, pcitemsfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next(citemstofetch, ::core::mem::transmute_copy(&pclsid), ::core::mem::transmute_copy(&names), ::core::mem::transmute_copy(&pcitemsfetched)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumDMOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, citemstoskip: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Skip(citemstoskip) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumDMOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IEnumDMOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEnumDMO>, ::windows::core::GetTrustLevel, Next::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IMediaBufferImpl: Sized {
    fn SetLength();
    fn GetMaxLength();
    fn GetBufferAndLength();
}
impl ::windows::core::RuntimeName for IMediaBuffer {
    const NAME: &'static str = "Windows.Win32.Media.DxMediaObjects.IMediaBuffer";
}
impl IMediaBufferVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaBufferImpl, const OFFSET: isize>() -> IMediaBufferVtbl {
        unsafe extern "system" fn SetLength<Impl: IMediaBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cblength: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetLength(cblength) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxLength<Impl: IMediaBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbmaxlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMaxLength(::core::mem::transmute_copy(&pcbmaxlength)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBufferAndLength<Impl: IMediaBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppbuffer: *mut *mut u8, pcblength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBufferAndLength(::core::mem::transmute_copy(&ppbuffer), ::core::mem::transmute_copy(&pcblength)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMediaBuffer>, ::windows::core::GetTrustLevel, SetLength::<Impl, OFFSET>, GetMaxLength::<Impl, OFFSET>, GetBufferAndLength::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IMediaObject {
    const NAME: &'static str = "Windows.Win32.Media.DxMediaObjects.IMediaObject";
}
impl IMediaObjectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaObjectImpl, const OFFSET: isize>() -> IMediaObjectVtbl {
        unsafe extern "system" fn GetStreamCount<Impl: IMediaObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcinputstreams: *mut u32, pcoutputstreams: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetInputStreamInfo<Impl: IMediaObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputstreamindex: u32, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInputStreamInfo(dwinputstreamindex, ::core::mem::transmute_copy(&pdwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputStreamInfo<Impl: IMediaObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputstreamindex: u32, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOutputStreamInfo(dwoutputstreamindex, ::core::mem::transmute_copy(&pdwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputType<Impl: IMediaObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputstreamindex: u32, dwtypeindex: u32, pmt: *mut DMO_MEDIA_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInputType(dwinputstreamindex, dwtypeindex, ::core::mem::transmute_copy(&pmt)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputType<Impl: IMediaObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputstreamindex: u32, dwtypeindex: u32, pmt: *mut DMO_MEDIA_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOutputType(dwoutputstreamindex, dwtypeindex, ::core::mem::transmute_copy(&pmt)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInputType<Impl: IMediaObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputstreamindex: u32, pmt: *const DMO_MEDIA_TYPE, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetInputType(dwinputstreamindex, &*(&pmt as *const <DMO_MEDIA_TYPE as ::windows::core::Abi>::Abi as *const <DMO_MEDIA_TYPE as ::windows::core::DefaultType>::DefaultType), dwflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutputType<Impl: IMediaObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputstreamindex: u32, pmt: *const DMO_MEDIA_TYPE, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetOutputType(dwoutputstreamindex, &*(&pmt as *const <DMO_MEDIA_TYPE as ::windows::core::Abi>::Abi as *const <DMO_MEDIA_TYPE as ::windows::core::DefaultType>::DefaultType), dwflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputCurrentType<Impl: IMediaObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputstreamindex: u32, pmt: *mut DMO_MEDIA_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInputCurrentType(dwinputstreamindex, ::core::mem::transmute_copy(&pmt)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputCurrentType<Impl: IMediaObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputstreamindex: u32, pmt: *mut DMO_MEDIA_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOutputCurrentType(dwoutputstreamindex, ::core::mem::transmute_copy(&pmt)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputSizeInfo<Impl: IMediaObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputstreamindex: u32, pcbsize: *mut u32, pcbmaxlookahead: *mut u32, pcbalignment: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInputSizeInfo(dwinputstreamindex, ::core::mem::transmute_copy(&pcbsize), ::core::mem::transmute_copy(&pcbmaxlookahead), ::core::mem::transmute_copy(&pcbalignment)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputSizeInfo<Impl: IMediaObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputstreamindex: u32, pcbsize: *mut u32, pcbalignment: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOutputSizeInfo(dwoutputstreamindex, ::core::mem::transmute_copy(&pcbsize), ::core::mem::transmute_copy(&pcbalignment)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputMaxLatency<Impl: IMediaObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputstreamindex: u32, prtmaxlatency: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInputMaxLatency(dwinputstreamindex, ::core::mem::transmute_copy(&prtmaxlatency)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInputMaxLatency<Impl: IMediaObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputstreamindex: u32, rtmaxlatency: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetInputMaxLatency(dwinputstreamindex, rtmaxlatency) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Flush<Impl: IMediaObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Flush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Discontinuity<Impl: IMediaObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputstreamindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Discontinuity(dwinputstreamindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllocateStreamingResources<Impl: IMediaObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllocateStreamingResources() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FreeStreamingResources<Impl: IMediaObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FreeStreamingResources() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputStatus<Impl: IMediaObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputstreamindex: u32, dwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInputStatus(dwinputstreamindex, ::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProcessInput<Impl: IMediaObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputstreamindex: u32, pbuffer: ::windows::core::RawPtr, dwflags: u32, rttimestamp: i64, rttimelength: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProcessInput(dwinputstreamindex, &*(&pbuffer as *const <IMediaBuffer as ::windows::core::Abi>::Abi as *const <IMediaBuffer as ::windows::core::DefaultType>::DefaultType), dwflags, rttimestamp, rttimelength) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProcessOutput<Impl: IMediaObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, coutputbuffercount: u32, poutputbuffers: *mut DMO_OUTPUT_DATA_BUFFER, pdwstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProcessOutput(dwflags, coutputbuffercount, ::core::mem::transmute_copy(&poutputbuffers), ::core::mem::transmute_copy(&pdwstatus)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Lock<Impl: IMediaObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, block: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Lock(block) {
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
            ::windows::core::GetRuntimeClassName::<IMediaObject>,
            ::windows::core::GetTrustLevel,
            GetStreamCount::<Impl, OFFSET>,
            GetInputStreamInfo::<Impl, OFFSET>,
            GetOutputStreamInfo::<Impl, OFFSET>,
            GetInputType::<Impl, OFFSET>,
            GetOutputType::<Impl, OFFSET>,
            SetInputType::<Impl, OFFSET>,
            SetOutputType::<Impl, OFFSET>,
            GetInputCurrentType::<Impl, OFFSET>,
            GetOutputCurrentType::<Impl, OFFSET>,
            GetInputSizeInfo::<Impl, OFFSET>,
            GetOutputSizeInfo::<Impl, OFFSET>,
            GetInputMaxLatency::<Impl, OFFSET>,
            SetInputMaxLatency::<Impl, OFFSET>,
            Flush::<Impl, OFFSET>,
            Discontinuity::<Impl, OFFSET>,
            AllocateStreamingResources::<Impl, OFFSET>,
            FreeStreamingResources::<Impl, OFFSET>,
            GetInputStatus::<Impl, OFFSET>,
            ProcessInput::<Impl, OFFSET>,
            ProcessOutput::<Impl, OFFSET>,
            Lock::<Impl, OFFSET>,
        )
    }
}
pub trait IMediaObjectInPlaceImpl: Sized {
    fn Process();
    fn Clone();
    fn GetLatency();
}
impl ::windows::core::RuntimeName for IMediaObjectInPlace {
    const NAME: &'static str = "Windows.Win32.Media.DxMediaObjects.IMediaObjectInPlace";
}
impl IMediaObjectInPlaceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaObjectInPlaceImpl, const OFFSET: isize>() -> IMediaObjectInPlaceVtbl {
        unsafe extern "system" fn Process<Impl: IMediaObjectInPlaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulsize: u32, pdata: *mut u8, reftimestart: i64, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Process(ulsize, ::core::mem::transmute_copy(&pdata), reftimestart, dwflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IMediaObjectInPlaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmediaobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppmediaobject)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLatency<Impl: IMediaObjectInPlaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, platencytime: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLatency(::core::mem::transmute_copy(&platencytime)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMediaObjectInPlace>, ::windows::core::GetTrustLevel, Process::<Impl, OFFSET>, Clone::<Impl, OFFSET>, GetLatency::<Impl, OFFSET>)
    }
}
