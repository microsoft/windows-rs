pub trait ITraceEvent_Impl: Sized {
    fn Clone(&self) -> ::windows::core::Result<ITraceEvent>;
    fn GetUserContext(&self) -> ::windows::core::Result<*mut ::core::ffi::c_void>;
    fn GetEventRecord(&self) -> ::windows::core::Result<*mut EVENT_RECORD>;
    fn SetPayload(&self, payload: *const u8, payloadsize: u32) -> ::windows::core::Result<()>;
    fn SetEventDescriptor(&self, eventdescriptor: *const EVENT_DESCRIPTOR) -> ::windows::core::Result<()>;
    fn SetProcessId(&self, processid: u32) -> ::windows::core::Result<()>;
    fn SetProcessorIndex(&self, processorindex: u32) -> ::windows::core::Result<()>;
    fn SetThreadId(&self, threadid: u32) -> ::windows::core::Result<()>;
    fn SetThreadTimes(&self, kerneltime: u32, usertime: u32) -> ::windows::core::Result<()>;
    fn SetActivityId(&self, activityid: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn SetTimeStamp(&self, timestamp: *const i64) -> ::windows::core::Result<()>;
    fn SetProviderId(&self, providerid: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ITraceEvent {}
impl ITraceEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITraceEvent_Impl, const OFFSET: isize>() -> ITraceEvent_Vtbl {
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITraceEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newevent: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(newevent, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUserContext<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITraceEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usercontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetUserContext() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(usercontext, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEventRecord<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITraceEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventrecord: *mut *mut EVENT_RECORD) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetEventRecord() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(eventrecord, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPayload<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITraceEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, payload: *const u8, payloadsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPayload(::core::mem::transmute_copy(&payload), ::core::mem::transmute_copy(&payloadsize)).into()
        }
        unsafe extern "system" fn SetEventDescriptor<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITraceEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventdescriptor: *const EVENT_DESCRIPTOR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEventDescriptor(::core::mem::transmute_copy(&eventdescriptor)).into()
        }
        unsafe extern "system" fn SetProcessId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITraceEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, processid: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetProcessId(::core::mem::transmute_copy(&processid)).into()
        }
        unsafe extern "system" fn SetProcessorIndex<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITraceEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, processorindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetProcessorIndex(::core::mem::transmute_copy(&processorindex)).into()
        }
        unsafe extern "system" fn SetThreadId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITraceEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, threadid: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetThreadId(::core::mem::transmute_copy(&threadid)).into()
        }
        unsafe extern "system" fn SetThreadTimes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITraceEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, kerneltime: u32, usertime: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetThreadTimes(::core::mem::transmute_copy(&kerneltime), ::core::mem::transmute_copy(&usertime)).into()
        }
        unsafe extern "system" fn SetActivityId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITraceEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activityid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetActivityId(::core::mem::transmute_copy(&activityid)).into()
        }
        unsafe extern "system" fn SetTimeStamp<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITraceEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timestamp: *const i64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTimeStamp(::core::mem::transmute_copy(&timestamp)).into()
        }
        unsafe extern "system" fn SetProviderId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITraceEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, providerid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetProviderId(::core::mem::transmute_copy(&providerid)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Clone: Clone::<Identity, Impl, OFFSET>,
            GetUserContext: GetUserContext::<Identity, Impl, OFFSET>,
            GetEventRecord: GetEventRecord::<Identity, Impl, OFFSET>,
            SetPayload: SetPayload::<Identity, Impl, OFFSET>,
            SetEventDescriptor: SetEventDescriptor::<Identity, Impl, OFFSET>,
            SetProcessId: SetProcessId::<Identity, Impl, OFFSET>,
            SetProcessorIndex: SetProcessorIndex::<Identity, Impl, OFFSET>,
            SetThreadId: SetThreadId::<Identity, Impl, OFFSET>,
            SetThreadTimes: SetThreadTimes::<Identity, Impl, OFFSET>,
            SetActivityId: SetActivityId::<Identity, Impl, OFFSET>,
            SetTimeStamp: SetTimeStamp::<Identity, Impl, OFFSET>,
            SetProviderId: SetProviderId::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITraceEvent as ::windows::core::Interface>::IID
    }
}
pub trait ITraceEventCallback_Impl: Sized {
    fn OnBeginProcessTrace(&self, headerevent: &::core::option::Option<ITraceEvent>, relogger: &::core::option::Option<ITraceRelogger>) -> ::windows::core::Result<()>;
    fn OnFinalizeProcessTrace(&self, relogger: &::core::option::Option<ITraceRelogger>) -> ::windows::core::Result<()>;
    fn OnEvent(&self, event: &::core::option::Option<ITraceEvent>, relogger: &::core::option::Option<ITraceRelogger>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ITraceEventCallback {}
impl ITraceEventCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITraceEventCallback_Impl, const OFFSET: isize>() -> ITraceEventCallback_Vtbl {
        unsafe extern "system" fn OnBeginProcessTrace<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITraceEventCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, headerevent: *mut ::core::ffi::c_void, relogger: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnBeginProcessTrace(::core::mem::transmute(&headerevent), ::core::mem::transmute(&relogger)).into()
        }
        unsafe extern "system" fn OnFinalizeProcessTrace<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITraceEventCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relogger: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnFinalizeProcessTrace(::core::mem::transmute(&relogger)).into()
        }
        unsafe extern "system" fn OnEvent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITraceEventCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, event: *mut ::core::ffi::c_void, relogger: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnEvent(::core::mem::transmute(&event), ::core::mem::transmute(&relogger)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            OnBeginProcessTrace: OnBeginProcessTrace::<Identity, Impl, OFFSET>,
            OnFinalizeProcessTrace: OnFinalizeProcessTrace::<Identity, Impl, OFFSET>,
            OnEvent: OnEvent::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITraceEventCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITraceRelogger_Impl: Sized {
    fn AddLogfileTraceStream(&self, logfilename: &super::super::super::Foundation::BSTR, usercontext: *const ::core::ffi::c_void) -> ::windows::core::Result<u64>;
    fn AddRealtimeTraceStream(&self, loggername: &super::super::super::Foundation::BSTR, usercontext: *const ::core::ffi::c_void) -> ::windows::core::Result<u64>;
    fn RegisterCallback(&self, callback: &::core::option::Option<ITraceEventCallback>) -> ::windows::core::Result<()>;
    fn Inject(&self, event: &::core::option::Option<ITraceEvent>) -> ::windows::core::Result<()>;
    fn CreateEventInstance(&self, tracehandle: u64, flags: u32) -> ::windows::core::Result<ITraceEvent>;
    fn ProcessTrace(&self) -> ::windows::core::Result<()>;
    fn SetOutputFilename(&self, logfilename: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetCompressionMode(&self, compressionmode: super::super::super::Foundation::BOOLEAN) -> ::windows::core::Result<()>;
    fn Cancel(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ITraceRelogger {}
#[cfg(feature = "Win32_Foundation")]
impl ITraceRelogger_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITraceRelogger_Impl, const OFFSET: isize>() -> ITraceRelogger_Vtbl {
        unsafe extern "system" fn AddLogfileTraceStream<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITraceRelogger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, logfilename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, usercontext: *const ::core::ffi::c_void, tracehandle: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AddLogfileTraceStream(::core::mem::transmute(&logfilename), ::core::mem::transmute_copy(&usercontext)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(tracehandle, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddRealtimeTraceStream<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITraceRelogger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, loggername: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, usercontext: *const ::core::ffi::c_void, tracehandle: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AddRealtimeTraceStream(::core::mem::transmute(&loggername), ::core::mem::transmute_copy(&usercontext)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(tracehandle, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterCallback<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITraceRelogger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, callback: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RegisterCallback(::core::mem::transmute(&callback)).into()
        }
        unsafe extern "system" fn Inject<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITraceRelogger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, event: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Inject(::core::mem::transmute(&event)).into()
        }
        unsafe extern "system" fn CreateEventInstance<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITraceRelogger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tracehandle: u64, flags: u32, event: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateEventInstance(::core::mem::transmute_copy(&tracehandle), ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(event, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProcessTrace<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITraceRelogger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ProcessTrace().into()
        }
        unsafe extern "system" fn SetOutputFilename<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITraceRelogger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, logfilename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetOutputFilename(::core::mem::transmute(&logfilename)).into()
        }
        unsafe extern "system" fn SetCompressionMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITraceRelogger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, compressionmode: super::super::super::Foundation::BOOLEAN) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCompressionMode(::core::mem::transmute_copy(&compressionmode)).into()
        }
        unsafe extern "system" fn Cancel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITraceRelogger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Cancel().into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            AddLogfileTraceStream: AddLogfileTraceStream::<Identity, Impl, OFFSET>,
            AddRealtimeTraceStream: AddRealtimeTraceStream::<Identity, Impl, OFFSET>,
            RegisterCallback: RegisterCallback::<Identity, Impl, OFFSET>,
            Inject: Inject::<Identity, Impl, OFFSET>,
            CreateEventInstance: CreateEventInstance::<Identity, Impl, OFFSET>,
            ProcessTrace: ProcessTrace::<Identity, Impl, OFFSET>,
            SetOutputFilename: SetOutputFilename::<Identity, Impl, OFFSET>,
            SetCompressionMode: SetCompressionMode::<Identity, Impl, OFFSET>,
            Cancel: Cancel::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITraceRelogger as ::windows::core::Interface>::IID
    }
}
