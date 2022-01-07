pub trait ITraceEventImpl: Sized {
    fn Clone();
    fn GetUserContext();
    fn GetEventRecord();
    fn SetPayload();
    fn SetEventDescriptor();
    fn SetProcessId();
    fn SetProcessorIndex();
    fn SetThreadId();
    fn SetThreadTimes();
    fn SetActivityId();
    fn SetTimeStamp();
    fn SetProviderId();
}
impl ::windows::core::RuntimeName for ITraceEvent {
    const NAME: &'static str = "Windows.Win32.System.Diagnostics.Etw.ITraceEvent";
}
impl ITraceEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITraceEventImpl, const OFFSET: isize>() -> ITraceEventVtbl {
        unsafe extern "system" fn Clone<Impl: ITraceEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newevent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&newevent)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUserContext<Impl: ITraceEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usercontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUserContext(::core::mem::transmute_copy(&usercontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEventRecord<Impl: ITraceEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventrecord: *mut *mut EVENT_RECORD) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEventRecord(::core::mem::transmute_copy(&eventrecord)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPayload<Impl: ITraceEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, payload: *const u8, payloadsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPayload(payload, payloadsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventDescriptor<Impl: ITraceEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventdescriptor: *const EVENT_DESCRIPTOR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetEventDescriptor(&*(&eventdescriptor as *const <EVENT_DESCRIPTOR as ::windows::core::Abi>::Abi as *const <EVENT_DESCRIPTOR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProcessId<Impl: ITraceEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, processid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetProcessId(processid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProcessorIndex<Impl: ITraceEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, processorindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetProcessorIndex(processorindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetThreadId<Impl: ITraceEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, threadid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetThreadId(threadid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetThreadTimes<Impl: ITraceEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, kerneltime: u32, usertime: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetThreadTimes(kerneltime, usertime) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetActivityId<Impl: ITraceEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activityid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetActivityId(&*(&activityid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTimeStamp<Impl: ITraceEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timestamp: *const i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetTimeStamp(timestamp) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProviderId<Impl: ITraceEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, providerid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetProviderId(&*(&providerid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
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
            ::windows::core::GetRuntimeClassName::<ITraceEvent>,
            ::windows::core::GetTrustLevel,
            Clone::<Impl, OFFSET>,
            GetUserContext::<Impl, OFFSET>,
            GetEventRecord::<Impl, OFFSET>,
            SetPayload::<Impl, OFFSET>,
            SetEventDescriptor::<Impl, OFFSET>,
            SetProcessId::<Impl, OFFSET>,
            SetProcessorIndex::<Impl, OFFSET>,
            SetThreadId::<Impl, OFFSET>,
            SetThreadTimes::<Impl, OFFSET>,
            SetActivityId::<Impl, OFFSET>,
            SetTimeStamp::<Impl, OFFSET>,
            SetProviderId::<Impl, OFFSET>,
        )
    }
}
pub trait ITraceEventCallbackImpl: Sized {
    fn OnBeginProcessTrace();
    fn OnFinalizeProcessTrace();
    fn OnEvent();
}
impl ::windows::core::RuntimeName for ITraceEventCallback {
    const NAME: &'static str = "Windows.Win32.System.Diagnostics.Etw.ITraceEventCallback";
}
impl ITraceEventCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITraceEventCallbackImpl, const OFFSET: isize>() -> ITraceEventCallbackVtbl {
        unsafe extern "system" fn OnBeginProcessTrace<Impl: ITraceEventCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, headerevent: ::windows::core::RawPtr, relogger: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnBeginProcessTrace(&*(&headerevent as *const <ITraceEvent as ::windows::core::Abi>::Abi as *const <ITraceEvent as ::windows::core::DefaultType>::DefaultType), &*(&relogger as *const <ITraceRelogger as ::windows::core::Abi>::Abi as *const <ITraceRelogger as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnFinalizeProcessTrace<Impl: ITraceEventCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relogger: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnFinalizeProcessTrace(&*(&relogger as *const <ITraceRelogger as ::windows::core::Abi>::Abi as *const <ITraceRelogger as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnEvent<Impl: ITraceEventCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, event: ::windows::core::RawPtr, relogger: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnEvent(&*(&event as *const <ITraceEvent as ::windows::core::Abi>::Abi as *const <ITraceEvent as ::windows::core::DefaultType>::DefaultType), &*(&relogger as *const <ITraceRelogger as ::windows::core::Abi>::Abi as *const <ITraceRelogger as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITraceEventCallback>, ::windows::core::GetTrustLevel, OnBeginProcessTrace::<Impl, OFFSET>, OnFinalizeProcessTrace::<Impl, OFFSET>, OnEvent::<Impl, OFFSET>)
    }
}
pub trait ITraceReloggerImpl: Sized {
    fn AddLogfileTraceStream();
    fn AddRealtimeTraceStream();
    fn RegisterCallback();
    fn Inject();
    fn CreateEventInstance();
    fn ProcessTrace();
    fn SetOutputFilename();
    fn SetCompressionMode();
    fn Cancel();
}
impl ::windows::core::RuntimeName for ITraceRelogger {
    const NAME: &'static str = "Windows.Win32.System.Diagnostics.Etw.ITraceRelogger";
}
impl ITraceReloggerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITraceReloggerImpl, const OFFSET: isize>() -> ITraceReloggerVtbl {
        unsafe extern "system" fn AddLogfileTraceStream<Impl: ITraceReloggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, logfilename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, usercontext: *const ::core::ffi::c_void, tracehandle: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddLogfileTraceStream(&*(&logfilename as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&usercontext as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&tracehandle)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddRealtimeTraceStream<Impl: ITraceReloggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, loggername: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, usercontext: *const ::core::ffi::c_void, tracehandle: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddRealtimeTraceStream(&*(&loggername as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&usercontext as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&tracehandle)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterCallback<Impl: ITraceReloggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, callback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterCallback(&*(&callback as *const <ITraceEventCallback as ::windows::core::Abi>::Abi as *const <ITraceEventCallback as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Inject<Impl: ITraceReloggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, event: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Inject(&*(&event as *const <ITraceEvent as ::windows::core::Abi>::Abi as *const <ITraceEvent as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateEventInstance<Impl: ITraceReloggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tracehandle: u64, flags: u32, event: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateEventInstance(tracehandle, flags, ::core::mem::transmute_copy(&event)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProcessTrace<Impl: ITraceReloggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProcessTrace() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutputFilename<Impl: ITraceReloggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, logfilename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetOutputFilename(&*(&logfilename as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCompressionMode<Impl: ITraceReloggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, compressionmode: super::super::super::Foundation::BOOLEAN) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCompressionMode(&*(&compressionmode as *const <super::super::super::Foundation::BOOLEAN as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BOOLEAN as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cancel<Impl: ITraceReloggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Cancel() {
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
            ::windows::core::GetRuntimeClassName::<ITraceRelogger>,
            ::windows::core::GetTrustLevel,
            AddLogfileTraceStream::<Impl, OFFSET>,
            AddRealtimeTraceStream::<Impl, OFFSET>,
            RegisterCallback::<Impl, OFFSET>,
            Inject::<Impl, OFFSET>,
            CreateEventInstance::<Impl, OFFSET>,
            ProcessTrace::<Impl, OFFSET>,
            SetOutputFilename::<Impl, OFFSET>,
            SetCompressionMode::<Impl, OFFSET>,
            Cancel::<Impl, OFFSET>,
        )
    }
}
