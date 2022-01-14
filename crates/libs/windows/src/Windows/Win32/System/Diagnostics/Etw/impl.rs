pub trait ITraceEvent_Impl: Sized {
    fn Clone(&mut self) -> ::windows::core::Result<ITraceEvent>;
    fn GetUserContext(&mut self, usercontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetEventRecord(&mut self) -> ::windows::core::Result<*mut EVENT_RECORD>;
    fn SetPayload(&mut self, payload: *const u8, payloadsize: u32) -> ::windows::core::Result<()>;
    fn SetEventDescriptor(&mut self, eventdescriptor: *const EVENT_DESCRIPTOR) -> ::windows::core::Result<()>;
    fn SetProcessId(&mut self, processid: u32) -> ::windows::core::Result<()>;
    fn SetProcessorIndex(&mut self, processorindex: u32) -> ::windows::core::Result<()>;
    fn SetThreadId(&mut self, threadid: u32) -> ::windows::core::Result<()>;
    fn SetThreadTimes(&mut self, kerneltime: u32, usertime: u32) -> ::windows::core::Result<()>;
    fn SetActivityId(&mut self, activityid: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn SetTimeStamp(&mut self, timestamp: *const i64) -> ::windows::core::Result<()>;
    fn SetProviderId(&mut self, providerid: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
}
impl ITraceEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITraceEvent_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITraceEvent_Vtbl {
        unsafe extern "system" fn Clone<Impl: ITraceEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newevent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *newevent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUserContext<Impl: ITraceEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usercontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetUserContext(::core::mem::transmute_copy(&usercontext)).into()
        }
        unsafe extern "system" fn GetEventRecord<Impl: ITraceEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventrecord: *mut *mut EVENT_RECORD) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEventRecord() {
                ::core::result::Result::Ok(ok__) => {
                    *eventrecord = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPayload<Impl: ITraceEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, payload: *const u8, payloadsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPayload(::core::mem::transmute_copy(&payload), ::core::mem::transmute_copy(&payloadsize)).into()
        }
        unsafe extern "system" fn SetEventDescriptor<Impl: ITraceEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventdescriptor: *const EVENT_DESCRIPTOR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEventDescriptor(::core::mem::transmute_copy(&eventdescriptor)).into()
        }
        unsafe extern "system" fn SetProcessId<Impl: ITraceEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, processid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProcessId(::core::mem::transmute_copy(&processid)).into()
        }
        unsafe extern "system" fn SetProcessorIndex<Impl: ITraceEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, processorindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProcessorIndex(::core::mem::transmute_copy(&processorindex)).into()
        }
        unsafe extern "system" fn SetThreadId<Impl: ITraceEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, threadid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetThreadId(::core::mem::transmute_copy(&threadid)).into()
        }
        unsafe extern "system" fn SetThreadTimes<Impl: ITraceEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, kerneltime: u32, usertime: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetThreadTimes(::core::mem::transmute_copy(&kerneltime), ::core::mem::transmute_copy(&usertime)).into()
        }
        unsafe extern "system" fn SetActivityId<Impl: ITraceEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activityid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetActivityId(::core::mem::transmute_copy(&activityid)).into()
        }
        unsafe extern "system" fn SetTimeStamp<Impl: ITraceEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timestamp: *const i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTimeStamp(::core::mem::transmute_copy(&timestamp)).into()
        }
        unsafe extern "system" fn SetProviderId<Impl: ITraceEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, providerid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProviderId(::core::mem::transmute_copy(&providerid)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Clone: Clone::<Impl, IMPL_OFFSET>,
            GetUserContext: GetUserContext::<Impl, IMPL_OFFSET>,
            GetEventRecord: GetEventRecord::<Impl, IMPL_OFFSET>,
            SetPayload: SetPayload::<Impl, IMPL_OFFSET>,
            SetEventDescriptor: SetEventDescriptor::<Impl, IMPL_OFFSET>,
            SetProcessId: SetProcessId::<Impl, IMPL_OFFSET>,
            SetProcessorIndex: SetProcessorIndex::<Impl, IMPL_OFFSET>,
            SetThreadId: SetThreadId::<Impl, IMPL_OFFSET>,
            SetThreadTimes: SetThreadTimes::<Impl, IMPL_OFFSET>,
            SetActivityId: SetActivityId::<Impl, IMPL_OFFSET>,
            SetTimeStamp: SetTimeStamp::<Impl, IMPL_OFFSET>,
            SetProviderId: SetProviderId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITraceEvent as ::windows::core::Interface>::IID
    }
}
pub trait ITraceEventCallback_Impl: Sized {
    fn OnBeginProcessTrace(&mut self, headerevent: &::core::option::Option<ITraceEvent>, relogger: &::core::option::Option<ITraceRelogger>) -> ::windows::core::Result<()>;
    fn OnFinalizeProcessTrace(&mut self, relogger: &::core::option::Option<ITraceRelogger>) -> ::windows::core::Result<()>;
    fn OnEvent(&mut self, event: &::core::option::Option<ITraceEvent>, relogger: &::core::option::Option<ITraceRelogger>) -> ::windows::core::Result<()>;
}
impl ITraceEventCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITraceEventCallback_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITraceEventCallback_Vtbl {
        unsafe extern "system" fn OnBeginProcessTrace<Impl: ITraceEventCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, headerevent: ::windows::core::RawPtr, relogger: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnBeginProcessTrace(::core::mem::transmute(&headerevent), ::core::mem::transmute(&relogger)).into()
        }
        unsafe extern "system" fn OnFinalizeProcessTrace<Impl: ITraceEventCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relogger: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnFinalizeProcessTrace(::core::mem::transmute(&relogger)).into()
        }
        unsafe extern "system" fn OnEvent<Impl: ITraceEventCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, event: ::windows::core::RawPtr, relogger: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnEvent(::core::mem::transmute(&event), ::core::mem::transmute(&relogger)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnBeginProcessTrace: OnBeginProcessTrace::<Impl, IMPL_OFFSET>,
            OnFinalizeProcessTrace: OnFinalizeProcessTrace::<Impl, IMPL_OFFSET>,
            OnEvent: OnEvent::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITraceEventCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITraceRelogger_Impl: Sized {
    fn AddLogfileTraceStream(&mut self, logfilename: &super::super::super::Foundation::BSTR, usercontext: *const ::core::ffi::c_void) -> ::windows::core::Result<u64>;
    fn AddRealtimeTraceStream(&mut self, loggername: &super::super::super::Foundation::BSTR, usercontext: *const ::core::ffi::c_void) -> ::windows::core::Result<u64>;
    fn RegisterCallback(&mut self, callback: &::core::option::Option<ITraceEventCallback>) -> ::windows::core::Result<()>;
    fn Inject(&mut self, event: &::core::option::Option<ITraceEvent>) -> ::windows::core::Result<()>;
    fn CreateEventInstance(&mut self, tracehandle: u64, flags: u32) -> ::windows::core::Result<ITraceEvent>;
    fn ProcessTrace(&mut self) -> ::windows::core::Result<()>;
    fn SetOutputFilename(&mut self, logfilename: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetCompressionMode(&mut self, compressionmode: super::super::super::Foundation::BOOLEAN) -> ::windows::core::Result<()>;
    fn Cancel(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITraceRelogger_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITraceRelogger_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITraceRelogger_Vtbl {
        unsafe extern "system" fn AddLogfileTraceStream<Impl: ITraceRelogger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, logfilename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, usercontext: *const ::core::ffi::c_void, tracehandle: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddLogfileTraceStream(::core::mem::transmute_copy(&logfilename), ::core::mem::transmute_copy(&usercontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *tracehandle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddRealtimeTraceStream<Impl: ITraceRelogger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, loggername: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, usercontext: *const ::core::ffi::c_void, tracehandle: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddRealtimeTraceStream(::core::mem::transmute_copy(&loggername), ::core::mem::transmute_copy(&usercontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *tracehandle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterCallback<Impl: ITraceRelogger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, callback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RegisterCallback(::core::mem::transmute(&callback)).into()
        }
        unsafe extern "system" fn Inject<Impl: ITraceRelogger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, event: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Inject(::core::mem::transmute(&event)).into()
        }
        unsafe extern "system" fn CreateEventInstance<Impl: ITraceRelogger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tracehandle: u64, flags: u32, event: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateEventInstance(::core::mem::transmute_copy(&tracehandle), ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *event = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProcessTrace<Impl: ITraceRelogger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ProcessTrace().into()
        }
        unsafe extern "system" fn SetOutputFilename<Impl: ITraceRelogger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, logfilename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOutputFilename(::core::mem::transmute_copy(&logfilename)).into()
        }
        unsafe extern "system" fn SetCompressionMode<Impl: ITraceRelogger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, compressionmode: super::super::super::Foundation::BOOLEAN) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCompressionMode(::core::mem::transmute_copy(&compressionmode)).into()
        }
        unsafe extern "system" fn Cancel<Impl: ITraceRelogger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Cancel().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AddLogfileTraceStream: AddLogfileTraceStream::<Impl, IMPL_OFFSET>,
            AddRealtimeTraceStream: AddRealtimeTraceStream::<Impl, IMPL_OFFSET>,
            RegisterCallback: RegisterCallback::<Impl, IMPL_OFFSET>,
            Inject: Inject::<Impl, IMPL_OFFSET>,
            CreateEventInstance: CreateEventInstance::<Impl, IMPL_OFFSET>,
            ProcessTrace: ProcessTrace::<Impl, IMPL_OFFSET>,
            SetOutputFilename: SetOutputFilename::<Impl, IMPL_OFFSET>,
            SetCompressionMode: SetCompressionMode::<Impl, IMPL_OFFSET>,
            Cancel: Cancel::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITraceRelogger as ::windows::core::Interface>::IID
    }
}
