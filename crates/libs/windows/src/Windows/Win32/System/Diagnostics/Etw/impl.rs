#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"implement\"`*"]
pub trait ITraceEvent_Impl: Sized {
    fn Clone(&self) -> ::windows_core::Result<ITraceEvent>;
    fn GetUserContext(&self) -> ::windows_core::Result<*mut ::core::ffi::c_void>;
    fn GetEventRecord(&self) -> ::windows_core::Result<*mut EVENT_RECORD>;
    fn SetPayload(&self, payload: *const u8, payloadsize: u32) -> ::windows_core::Result<()>;
    fn SetEventDescriptor(&self, eventdescriptor: *const EVENT_DESCRIPTOR) -> ::windows_core::Result<()>;
    fn SetProcessId(&self, processid: u32) -> ::windows_core::Result<()>;
    fn SetProcessorIndex(&self, processorindex: u32) -> ::windows_core::Result<()>;
    fn SetThreadId(&self, threadid: u32) -> ::windows_core::Result<()>;
    fn SetThreadTimes(&self, kerneltime: u32, usertime: u32) -> ::windows_core::Result<()>;
    fn SetActivityId(&self, activityid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn SetTimeStamp(&self, timestamp: *const i64) -> ::windows_core::Result<()>;
    fn SetProviderId(&self, providerid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ITraceEvent {}
impl ITraceEvent_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITraceEvent_Impl, const OFFSET: isize>() -> ITraceEvent_Vtbl {
        unsafe extern "system" fn Clone<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITraceEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newevent: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(newevent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUserContext<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITraceEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usercontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetUserContext() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(usercontext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEventRecord<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITraceEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventrecord: *mut *mut EVENT_RECORD) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetEventRecord() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(eventrecord, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPayload<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITraceEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, payload: *const u8, payloadsize: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPayload(::core::mem::transmute_copy(&payload), ::core::mem::transmute_copy(&payloadsize)).into()
        }
        unsafe extern "system" fn SetEventDescriptor<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITraceEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventdescriptor: *const EVENT_DESCRIPTOR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEventDescriptor(::core::mem::transmute_copy(&eventdescriptor)).into()
        }
        unsafe extern "system" fn SetProcessId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITraceEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, processid: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetProcessId(::core::mem::transmute_copy(&processid)).into()
        }
        unsafe extern "system" fn SetProcessorIndex<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITraceEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, processorindex: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetProcessorIndex(::core::mem::transmute_copy(&processorindex)).into()
        }
        unsafe extern "system" fn SetThreadId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITraceEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, threadid: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetThreadId(::core::mem::transmute_copy(&threadid)).into()
        }
        unsafe extern "system" fn SetThreadTimes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITraceEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, kerneltime: u32, usertime: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetThreadTimes(::core::mem::transmute_copy(&kerneltime), ::core::mem::transmute_copy(&usertime)).into()
        }
        unsafe extern "system" fn SetActivityId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITraceEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activityid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetActivityId(::core::mem::transmute_copy(&activityid)).into()
        }
        unsafe extern "system" fn SetTimeStamp<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITraceEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timestamp: *const i64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTimeStamp(::core::mem::transmute_copy(&timestamp)).into()
        }
        unsafe extern "system" fn SetProviderId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITraceEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, providerid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetProviderId(::core::mem::transmute_copy(&providerid)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITraceEvent as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"implement\"`*"]
pub trait ITraceEventCallback_Impl: Sized {
    fn OnBeginProcessTrace(&self, headerevent: ::core::option::Option<&ITraceEvent>, relogger: ::core::option::Option<&ITraceRelogger>) -> ::windows_core::Result<()>;
    fn OnFinalizeProcessTrace(&self, relogger: ::core::option::Option<&ITraceRelogger>) -> ::windows_core::Result<()>;
    fn OnEvent(&self, event: ::core::option::Option<&ITraceEvent>, relogger: ::core::option::Option<&ITraceRelogger>) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ITraceEventCallback {}
impl ITraceEventCallback_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITraceEventCallback_Impl, const OFFSET: isize>() -> ITraceEventCallback_Vtbl {
        unsafe extern "system" fn OnBeginProcessTrace<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITraceEventCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, headerevent: *mut ::core::ffi::c_void, relogger: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnBeginProcessTrace(::windows_core::from_raw_borrowed(&headerevent), ::windows_core::from_raw_borrowed(&relogger)).into()
        }
        unsafe extern "system" fn OnFinalizeProcessTrace<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITraceEventCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relogger: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnFinalizeProcessTrace(::windows_core::from_raw_borrowed(&relogger)).into()
        }
        unsafe extern "system" fn OnEvent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITraceEventCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, event: *mut ::core::ffi::c_void, relogger: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnEvent(::windows_core::from_raw_borrowed(&event), ::windows_core::from_raw_borrowed(&relogger)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnBeginProcessTrace: OnBeginProcessTrace::<Identity, Impl, OFFSET>,
            OnFinalizeProcessTrace: OnFinalizeProcessTrace::<Identity, Impl, OFFSET>,
            OnEvent: OnEvent::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITraceEventCallback as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Diagnostics_Etw\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITraceRelogger_Impl: Sized {
    fn AddLogfileTraceStream(&self, logfilename: &::windows_core::BSTR, usercontext: *const ::core::ffi::c_void) -> ::windows_core::Result<RELOGSTREAM_HANDLE>;
    fn AddRealtimeTraceStream(&self, loggername: &::windows_core::BSTR, usercontext: *const ::core::ffi::c_void) -> ::windows_core::Result<RELOGSTREAM_HANDLE>;
    fn RegisterCallback(&self, callback: ::core::option::Option<&ITraceEventCallback>) -> ::windows_core::Result<()>;
    fn Inject(&self, event: ::core::option::Option<&ITraceEvent>) -> ::windows_core::Result<()>;
    fn CreateEventInstance(&self, tracehandle: &RELOGSTREAM_HANDLE, flags: u32) -> ::windows_core::Result<ITraceEvent>;
    fn ProcessTrace(&self) -> ::windows_core::Result<()>;
    fn SetOutputFilename(&self, logfilename: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetCompressionMode(&self, compressionmode: super::super::super::Foundation::BOOLEAN) -> ::windows_core::Result<()>;
    fn Cancel(&self) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for ITraceRelogger {}
#[cfg(feature = "Win32_Foundation")]
impl ITraceRelogger_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITraceRelogger_Impl, const OFFSET: isize>() -> ITraceRelogger_Vtbl {
        unsafe extern "system" fn AddLogfileTraceStream<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITraceRelogger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, logfilename: ::std::mem::MaybeUninit<::windows_core::BSTR>, usercontext: *const ::core::ffi::c_void, tracehandle: *mut RELOGSTREAM_HANDLE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AddLogfileTraceStream(::core::mem::transmute(&logfilename), ::core::mem::transmute_copy(&usercontext)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(tracehandle, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddRealtimeTraceStream<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITraceRelogger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, loggername: ::std::mem::MaybeUninit<::windows_core::BSTR>, usercontext: *const ::core::ffi::c_void, tracehandle: *mut RELOGSTREAM_HANDLE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AddRealtimeTraceStream(::core::mem::transmute(&loggername), ::core::mem::transmute_copy(&usercontext)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(tracehandle, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterCallback<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITraceRelogger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, callback: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RegisterCallback(::windows_core::from_raw_borrowed(&callback)).into()
        }
        unsafe extern "system" fn Inject<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITraceRelogger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, event: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Inject(::windows_core::from_raw_borrowed(&event)).into()
        }
        unsafe extern "system" fn CreateEventInstance<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITraceRelogger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tracehandle: RELOGSTREAM_HANDLE, flags: u32, event: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateEventInstance(::core::mem::transmute(&tracehandle), ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(event, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProcessTrace<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITraceRelogger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ProcessTrace().into()
        }
        unsafe extern "system" fn SetOutputFilename<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITraceRelogger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, logfilename: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetOutputFilename(::core::mem::transmute(&logfilename)).into()
        }
        unsafe extern "system" fn SetCompressionMode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITraceRelogger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, compressionmode: super::super::super::Foundation::BOOLEAN) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCompressionMode(::core::mem::transmute_copy(&compressionmode)).into()
        }
        unsafe extern "system" fn Cancel<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITraceRelogger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Cancel().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITraceRelogger as ::windows_core::ComInterface>::IID
    }
}
