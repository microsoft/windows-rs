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
impl ITraceEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITraceEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITraceEventVtbl {
        unsafe extern "system" fn Clone<Impl: ITraceEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newevent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetUserContext<Impl: ITraceEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usercontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEventRecord<Impl: ITraceEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventrecord: *mut *mut EVENT_RECORD) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPayload<Impl: ITraceEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, payload: *const u8, payloadsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEventDescriptor<Impl: ITraceEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventdescriptor: *const EVENT_DESCRIPTOR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetProcessId<Impl: ITraceEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, processid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetProcessorIndex<Impl: ITraceEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, processorindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetThreadId<Impl: ITraceEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, threadid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetThreadTimes<Impl: ITraceEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, kerneltime: u32, usertime: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetActivityId<Impl: ITraceEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activityid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTimeStamp<Impl: ITraceEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timestamp: *const i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetProviderId<Impl: ITraceEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, providerid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            Clone::<Impl, IMPL_OFFSET>,
            GetUserContext::<Impl, IMPL_OFFSET>,
            GetEventRecord::<Impl, IMPL_OFFSET>,
            SetPayload::<Impl, IMPL_OFFSET>,
            SetEventDescriptor::<Impl, IMPL_OFFSET>,
            SetProcessId::<Impl, IMPL_OFFSET>,
            SetProcessorIndex::<Impl, IMPL_OFFSET>,
            SetThreadId::<Impl, IMPL_OFFSET>,
            SetThreadTimes::<Impl, IMPL_OFFSET>,
            SetActivityId::<Impl, IMPL_OFFSET>,
            SetTimeStamp::<Impl, IMPL_OFFSET>,
            SetProviderId::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITraceEvent as ::windows::core::Interface>::IID
    }
}
pub trait ITraceEventCallbackImpl: Sized {
    fn OnBeginProcessTrace();
    fn OnFinalizeProcessTrace();
    fn OnEvent();
}
impl ITraceEventCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITraceEventCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITraceEventCallbackVtbl {
        unsafe extern "system" fn OnBeginProcessTrace<Impl: ITraceEventCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, headerevent: ::windows::core::RawPtr, relogger: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnFinalizeProcessTrace<Impl: ITraceEventCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relogger: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnEvent<Impl: ITraceEventCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, event: ::windows::core::RawPtr, relogger: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, OnBeginProcessTrace::<Impl, IMPL_OFFSET>, OnFinalizeProcessTrace::<Impl, IMPL_OFFSET>, OnEvent::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITraceEventCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
impl ITraceReloggerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITraceReloggerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITraceReloggerVtbl {
        unsafe extern "system" fn AddLogfileTraceStream<Impl: ITraceReloggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, logfilename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, usercontext: *const ::core::ffi::c_void, tracehandle: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddRealtimeTraceStream<Impl: ITraceReloggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, loggername: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, usercontext: *const ::core::ffi::c_void, tracehandle: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RegisterCallback<Impl: ITraceReloggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, callback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Inject<Impl: ITraceReloggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, event: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateEventInstance<Impl: ITraceReloggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tracehandle: u64, flags: u32, event: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ProcessTrace<Impl: ITraceReloggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOutputFilename<Impl: ITraceReloggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, logfilename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCompressionMode<Impl: ITraceReloggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, compressionmode: super::super::super::Foundation::BOOLEAN) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Cancel<Impl: ITraceReloggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            AddLogfileTraceStream::<Impl, IMPL_OFFSET>,
            AddRealtimeTraceStream::<Impl, IMPL_OFFSET>,
            RegisterCallback::<Impl, IMPL_OFFSET>,
            Inject::<Impl, IMPL_OFFSET>,
            CreateEventInstance::<Impl, IMPL_OFFSET>,
            ProcessTrace::<Impl, IMPL_OFFSET>,
            SetOutputFilename::<Impl, IMPL_OFFSET>,
            SetCompressionMode::<Impl, IMPL_OFFSET>,
            Cancel::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITraceRelogger as ::windows::core::Interface>::IID
    }
}
