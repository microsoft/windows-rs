#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait DICounterItem_Impl: Sized + super::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl DICounterItem_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: DICounterItem_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> DICounterItem_Vtbl {
        Self { base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<DICounterItem as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait DILogFileItem_Impl: Sized + super::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl DILogFileItem_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: DILogFileItem_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> DILogFileItem_Vtbl {
        Self { base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<DILogFileItem as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait DISystemMonitor_Impl: Sized + super::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl DISystemMonitor_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: DISystemMonitor_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> DISystemMonitor_Vtbl {
        Self { base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<DISystemMonitor as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait DISystemMonitorEvents_Impl: Sized + super::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl DISystemMonitorEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: DISystemMonitorEvents_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> DISystemMonitorEvents_Vtbl {
        Self { base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<DISystemMonitorEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait DISystemMonitorInternal_Impl: Sized + super::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl DISystemMonitorInternal_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: DISystemMonitorInternal_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> DISystemMonitorInternal_Vtbl {
        Self { base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<DISystemMonitorInternal as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAlertDataCollector_Impl: Sized + super::Com::IDispatch_Impl + IDataCollector_Impl {
    fn AlertThresholds(&mut self) -> ::windows::core::Result<*mut super::Com::SAFEARRAY>;
    fn SetAlertThresholds(&mut self, alerts: *const super::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn EventLog(&mut self) -> ::windows::core::Result<i16>;
    fn SetEventLog(&mut self, log: i16) -> ::windows::core::Result<()>;
    fn SampleInterval(&mut self) -> ::windows::core::Result<u32>;
    fn SetSampleInterval(&mut self, interval: u32) -> ::windows::core::Result<()>;
    fn Task(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetTask(&mut self, task: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn TaskRunAsSelf(&mut self) -> ::windows::core::Result<i16>;
    fn SetTaskRunAsSelf(&mut self, runasself: i16) -> ::windows::core::Result<()>;
    fn TaskArguments(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetTaskArguments(&mut self, task: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn TaskUserTextArguments(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetTaskUserTextArguments(&mut self, task: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn TriggerDataCollectorSet(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetTriggerDataCollectorSet(&mut self, name: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAlertDataCollector_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAlertDataCollector_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAlertDataCollector_Vtbl {
        unsafe extern "system" fn AlertThresholds<Impl: IAlertDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, alerts: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AlertThresholds() {
                ::core::result::Result::Ok(ok__) => {
                    *alerts = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlertThresholds<Impl: IAlertDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, alerts: *const super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAlertThresholds(::core::mem::transmute_copy(&alerts)).into()
        }
        unsafe extern "system" fn EventLog<Impl: IAlertDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, log: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EventLog() {
                ::core::result::Result::Ok(ok__) => {
                    *log = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventLog<Impl: IAlertDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, log: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEventLog(::core::mem::transmute_copy(&log)).into()
        }
        unsafe extern "system" fn SampleInterval<Impl: IAlertDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SampleInterval() {
                ::core::result::Result::Ok(ok__) => {
                    *interval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSampleInterval<Impl: IAlertDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interval: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSampleInterval(::core::mem::transmute_copy(&interval)).into()
        }
        unsafe extern "system" fn Task<Impl: IAlertDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, task: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Task() {
                ::core::result::Result::Ok(ok__) => {
                    *task = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTask<Impl: IAlertDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, task: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTask(::core::mem::transmute_copy(&task)).into()
        }
        unsafe extern "system" fn TaskRunAsSelf<Impl: IAlertDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, runasself: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TaskRunAsSelf() {
                ::core::result::Result::Ok(ok__) => {
                    *runasself = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTaskRunAsSelf<Impl: IAlertDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, runasself: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTaskRunAsSelf(::core::mem::transmute_copy(&runasself)).into()
        }
        unsafe extern "system" fn TaskArguments<Impl: IAlertDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, task: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TaskArguments() {
                ::core::result::Result::Ok(ok__) => {
                    *task = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTaskArguments<Impl: IAlertDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, task: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTaskArguments(::core::mem::transmute_copy(&task)).into()
        }
        unsafe extern "system" fn TaskUserTextArguments<Impl: IAlertDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, task: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TaskUserTextArguments() {
                ::core::result::Result::Ok(ok__) => {
                    *task = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTaskUserTextArguments<Impl: IAlertDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, task: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTaskUserTextArguments(::core::mem::transmute_copy(&task)).into()
        }
        unsafe extern "system" fn TriggerDataCollectorSet<Impl: IAlertDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TriggerDataCollectorSet() {
                ::core::result::Result::Ok(ok__) => {
                    *name = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTriggerDataCollectorSet<Impl: IAlertDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTriggerDataCollectorSet(::core::mem::transmute_copy(&name)).into()
        }
        Self {
            base: IDataCollector_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            AlertThresholds: AlertThresholds::<Impl, IMPL_OFFSET>,
            SetAlertThresholds: SetAlertThresholds::<Impl, IMPL_OFFSET>,
            EventLog: EventLog::<Impl, IMPL_OFFSET>,
            SetEventLog: SetEventLog::<Impl, IMPL_OFFSET>,
            SampleInterval: SampleInterval::<Impl, IMPL_OFFSET>,
            SetSampleInterval: SetSampleInterval::<Impl, IMPL_OFFSET>,
            Task: Task::<Impl, IMPL_OFFSET>,
            SetTask: SetTask::<Impl, IMPL_OFFSET>,
            TaskRunAsSelf: TaskRunAsSelf::<Impl, IMPL_OFFSET>,
            SetTaskRunAsSelf: SetTaskRunAsSelf::<Impl, IMPL_OFFSET>,
            TaskArguments: TaskArguments::<Impl, IMPL_OFFSET>,
            SetTaskArguments: SetTaskArguments::<Impl, IMPL_OFFSET>,
            TaskUserTextArguments: TaskUserTextArguments::<Impl, IMPL_OFFSET>,
            SetTaskUserTextArguments: SetTaskUserTextArguments::<Impl, IMPL_OFFSET>,
            TriggerDataCollectorSet: TriggerDataCollectorSet::<Impl, IMPL_OFFSET>,
            SetTriggerDataCollectorSet: SetTriggerDataCollectorSet::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAlertDataCollector as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IApiTracingDataCollector_Impl: Sized + super::Com::IDispatch_Impl + IDataCollector_Impl {
    fn LogApiNamesOnly(&mut self) -> ::windows::core::Result<i16>;
    fn SetLogApiNamesOnly(&mut self, logapinames: i16) -> ::windows::core::Result<()>;
    fn LogApisRecursively(&mut self) -> ::windows::core::Result<i16>;
    fn SetLogApisRecursively(&mut self, logrecursively: i16) -> ::windows::core::Result<()>;
    fn ExePath(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetExePath(&mut self, exepath: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn LogFilePath(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetLogFilePath(&mut self, logfilepath: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn IncludeModules(&mut self) -> ::windows::core::Result<*mut super::Com::SAFEARRAY>;
    fn SetIncludeModules(&mut self, includemodules: *const super::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn IncludeApis(&mut self) -> ::windows::core::Result<*mut super::Com::SAFEARRAY>;
    fn SetIncludeApis(&mut self, includeapis: *const super::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn ExcludeApis(&mut self) -> ::windows::core::Result<*mut super::Com::SAFEARRAY>;
    fn SetExcludeApis(&mut self, excludeapis: *const super::Com::SAFEARRAY) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IApiTracingDataCollector_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApiTracingDataCollector_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IApiTracingDataCollector_Vtbl {
        unsafe extern "system" fn LogApiNamesOnly<Impl: IApiTracingDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, logapinames: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LogApiNamesOnly() {
                ::core::result::Result::Ok(ok__) => {
                    *logapinames = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLogApiNamesOnly<Impl: IApiTracingDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, logapinames: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLogApiNamesOnly(::core::mem::transmute_copy(&logapinames)).into()
        }
        unsafe extern "system" fn LogApisRecursively<Impl: IApiTracingDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, logrecursively: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LogApisRecursively() {
                ::core::result::Result::Ok(ok__) => {
                    *logrecursively = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLogApisRecursively<Impl: IApiTracingDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, logrecursively: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLogApisRecursively(::core::mem::transmute_copy(&logrecursively)).into()
        }
        unsafe extern "system" fn ExePath<Impl: IApiTracingDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, exepath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExePath() {
                ::core::result::Result::Ok(ok__) => {
                    *exepath = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExePath<Impl: IApiTracingDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, exepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExePath(::core::mem::transmute_copy(&exepath)).into()
        }
        unsafe extern "system" fn LogFilePath<Impl: IApiTracingDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, logfilepath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LogFilePath() {
                ::core::result::Result::Ok(ok__) => {
                    *logfilepath = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLogFilePath<Impl: IApiTracingDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, logfilepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLogFilePath(::core::mem::transmute_copy(&logfilepath)).into()
        }
        unsafe extern "system" fn IncludeModules<Impl: IApiTracingDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, includemodules: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IncludeModules() {
                ::core::result::Result::Ok(ok__) => {
                    *includemodules = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIncludeModules<Impl: IApiTracingDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, includemodules: *const super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIncludeModules(::core::mem::transmute_copy(&includemodules)).into()
        }
        unsafe extern "system" fn IncludeApis<Impl: IApiTracingDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, includeapis: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IncludeApis() {
                ::core::result::Result::Ok(ok__) => {
                    *includeapis = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIncludeApis<Impl: IApiTracingDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, includeapis: *const super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIncludeApis(::core::mem::transmute_copy(&includeapis)).into()
        }
        unsafe extern "system" fn ExcludeApis<Impl: IApiTracingDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, excludeapis: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExcludeApis() {
                ::core::result::Result::Ok(ok__) => {
                    *excludeapis = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExcludeApis<Impl: IApiTracingDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, excludeapis: *const super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExcludeApis(::core::mem::transmute_copy(&excludeapis)).into()
        }
        Self {
            base: IDataCollector_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            LogApiNamesOnly: LogApiNamesOnly::<Impl, IMPL_OFFSET>,
            SetLogApiNamesOnly: SetLogApiNamesOnly::<Impl, IMPL_OFFSET>,
            LogApisRecursively: LogApisRecursively::<Impl, IMPL_OFFSET>,
            SetLogApisRecursively: SetLogApisRecursively::<Impl, IMPL_OFFSET>,
            ExePath: ExePath::<Impl, IMPL_OFFSET>,
            SetExePath: SetExePath::<Impl, IMPL_OFFSET>,
            LogFilePath: LogFilePath::<Impl, IMPL_OFFSET>,
            SetLogFilePath: SetLogFilePath::<Impl, IMPL_OFFSET>,
            IncludeModules: IncludeModules::<Impl, IMPL_OFFSET>,
            SetIncludeModules: SetIncludeModules::<Impl, IMPL_OFFSET>,
            IncludeApis: IncludeApis::<Impl, IMPL_OFFSET>,
            SetIncludeApis: SetIncludeApis::<Impl, IMPL_OFFSET>,
            ExcludeApis: ExcludeApis::<Impl, IMPL_OFFSET>,
            SetExcludeApis: SetExcludeApis::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IApiTracingDataCollector as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IConfigurationDataCollector_Impl: Sized + super::Com::IDispatch_Impl + IDataCollector_Impl {
    fn FileMaxCount(&mut self) -> ::windows::core::Result<u32>;
    fn SetFileMaxCount(&mut self, count: u32) -> ::windows::core::Result<()>;
    fn FileMaxRecursiveDepth(&mut self) -> ::windows::core::Result<u32>;
    fn SetFileMaxRecursiveDepth(&mut self, depth: u32) -> ::windows::core::Result<()>;
    fn FileMaxTotalSize(&mut self) -> ::windows::core::Result<u32>;
    fn SetFileMaxTotalSize(&mut self, size: u32) -> ::windows::core::Result<()>;
    fn Files(&mut self) -> ::windows::core::Result<*mut super::Com::SAFEARRAY>;
    fn SetFiles(&mut self, files: *const super::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn ManagementQueries(&mut self) -> ::windows::core::Result<*mut super::Com::SAFEARRAY>;
    fn SetManagementQueries(&mut self, queries: *const super::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn QueryNetworkAdapters(&mut self) -> ::windows::core::Result<i16>;
    fn SetQueryNetworkAdapters(&mut self, network: i16) -> ::windows::core::Result<()>;
    fn RegistryKeys(&mut self) -> ::windows::core::Result<*mut super::Com::SAFEARRAY>;
    fn SetRegistryKeys(&mut self, query: *const super::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn RegistryMaxRecursiveDepth(&mut self) -> ::windows::core::Result<u32>;
    fn SetRegistryMaxRecursiveDepth(&mut self, depth: u32) -> ::windows::core::Result<()>;
    fn SystemStateFile(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetSystemStateFile(&mut self, filename: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IConfigurationDataCollector_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConfigurationDataCollector_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IConfigurationDataCollector_Vtbl {
        unsafe extern "system" fn FileMaxCount<Impl: IConfigurationDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FileMaxCount() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFileMaxCount<Impl: IConfigurationDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFileMaxCount(::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn FileMaxRecursiveDepth<Impl: IConfigurationDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, depth: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FileMaxRecursiveDepth() {
                ::core::result::Result::Ok(ok__) => {
                    *depth = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFileMaxRecursiveDepth<Impl: IConfigurationDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, depth: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFileMaxRecursiveDepth(::core::mem::transmute_copy(&depth)).into()
        }
        unsafe extern "system" fn FileMaxTotalSize<Impl: IConfigurationDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, size: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FileMaxTotalSize() {
                ::core::result::Result::Ok(ok__) => {
                    *size = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFileMaxTotalSize<Impl: IConfigurationDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, size: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFileMaxTotalSize(::core::mem::transmute_copy(&size)).into()
        }
        unsafe extern "system" fn Files<Impl: IConfigurationDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, files: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Files() {
                ::core::result::Result::Ok(ok__) => {
                    *files = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFiles<Impl: IConfigurationDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, files: *const super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFiles(::core::mem::transmute_copy(&files)).into()
        }
        unsafe extern "system" fn ManagementQueries<Impl: IConfigurationDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, queries: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ManagementQueries() {
                ::core::result::Result::Ok(ok__) => {
                    *queries = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetManagementQueries<Impl: IConfigurationDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, queries: *const super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetManagementQueries(::core::mem::transmute_copy(&queries)).into()
        }
        unsafe extern "system" fn QueryNetworkAdapters<Impl: IConfigurationDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, network: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryNetworkAdapters() {
                ::core::result::Result::Ok(ok__) => {
                    *network = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQueryNetworkAdapters<Impl: IConfigurationDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, network: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetQueryNetworkAdapters(::core::mem::transmute_copy(&network)).into()
        }
        unsafe extern "system" fn RegistryKeys<Impl: IConfigurationDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, query: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegistryKeys() {
                ::core::result::Result::Ok(ok__) => {
                    *query = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRegistryKeys<Impl: IConfigurationDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, query: *const super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRegistryKeys(::core::mem::transmute_copy(&query)).into()
        }
        unsafe extern "system" fn RegistryMaxRecursiveDepth<Impl: IConfigurationDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, depth: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegistryMaxRecursiveDepth() {
                ::core::result::Result::Ok(ok__) => {
                    *depth = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRegistryMaxRecursiveDepth<Impl: IConfigurationDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, depth: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRegistryMaxRecursiveDepth(::core::mem::transmute_copy(&depth)).into()
        }
        unsafe extern "system" fn SystemStateFile<Impl: IConfigurationDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SystemStateFile() {
                ::core::result::Result::Ok(ok__) => {
                    *filename = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSystemStateFile<Impl: IConfigurationDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSystemStateFile(::core::mem::transmute_copy(&filename)).into()
        }
        Self {
            base: IDataCollector_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            FileMaxCount: FileMaxCount::<Impl, IMPL_OFFSET>,
            SetFileMaxCount: SetFileMaxCount::<Impl, IMPL_OFFSET>,
            FileMaxRecursiveDepth: FileMaxRecursiveDepth::<Impl, IMPL_OFFSET>,
            SetFileMaxRecursiveDepth: SetFileMaxRecursiveDepth::<Impl, IMPL_OFFSET>,
            FileMaxTotalSize: FileMaxTotalSize::<Impl, IMPL_OFFSET>,
            SetFileMaxTotalSize: SetFileMaxTotalSize::<Impl, IMPL_OFFSET>,
            Files: Files::<Impl, IMPL_OFFSET>,
            SetFiles: SetFiles::<Impl, IMPL_OFFSET>,
            ManagementQueries: ManagementQueries::<Impl, IMPL_OFFSET>,
            SetManagementQueries: SetManagementQueries::<Impl, IMPL_OFFSET>,
            QueryNetworkAdapters: QueryNetworkAdapters::<Impl, IMPL_OFFSET>,
            SetQueryNetworkAdapters: SetQueryNetworkAdapters::<Impl, IMPL_OFFSET>,
            RegistryKeys: RegistryKeys::<Impl, IMPL_OFFSET>,
            SetRegistryKeys: SetRegistryKeys::<Impl, IMPL_OFFSET>,
            RegistryMaxRecursiveDepth: RegistryMaxRecursiveDepth::<Impl, IMPL_OFFSET>,
            SetRegistryMaxRecursiveDepth: SetRegistryMaxRecursiveDepth::<Impl, IMPL_OFFSET>,
            SystemStateFile: SystemStateFile::<Impl, IMPL_OFFSET>,
            SetSystemStateFile: SetSystemStateFile::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConfigurationDataCollector as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ICounterItem_Impl: Sized {
    fn Value(&mut self) -> ::windows::core::Result<f64>;
    fn SetColor(&mut self, color: u32) -> ::windows::core::Result<()>;
    fn Color(&mut self) -> ::windows::core::Result<u32>;
    fn SetWidth(&mut self, iwidth: i32) -> ::windows::core::Result<()>;
    fn Width(&mut self) -> ::windows::core::Result<i32>;
    fn SetLineStyle(&mut self, ilinestyle: i32) -> ::windows::core::Result<()>;
    fn LineStyle(&mut self) -> ::windows::core::Result<i32>;
    fn SetScaleFactor(&mut self, iscale: i32) -> ::windows::core::Result<()>;
    fn ScaleFactor(&mut self) -> ::windows::core::Result<i32>;
    fn Path(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetValue(&mut self, value: *mut f64, status: *mut i32) -> ::windows::core::Result<()>;
    fn GetStatistics(&mut self, max: *mut f64, min: *mut f64, avg: *mut f64, status: *mut i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ICounterItem_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICounterItem_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICounterItem_Vtbl {
        unsafe extern "system" fn Value<Impl: ICounterItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdblvalue: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *pdblvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColor<Impl: ICounterItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, color: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetColor(::core::mem::transmute_copy(&color)).into()
        }
        unsafe extern "system" fn Color<Impl: ICounterItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcolor: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Color() {
                ::core::result::Result::Ok(ok__) => {
                    *pcolor = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWidth<Impl: ICounterItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iwidth: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWidth(::core::mem::transmute_copy(&iwidth)).into()
        }
        unsafe extern "system" fn Width<Impl: ICounterItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pivalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Width() {
                ::core::result::Result::Ok(ok__) => {
                    *pivalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLineStyle<Impl: ICounterItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ilinestyle: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLineStyle(::core::mem::transmute_copy(&ilinestyle)).into()
        }
        unsafe extern "system" fn LineStyle<Impl: ICounterItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pivalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LineStyle() {
                ::core::result::Result::Ok(ok__) => {
                    *pivalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScaleFactor<Impl: ICounterItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iscale: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScaleFactor(::core::mem::transmute_copy(&iscale)).into()
        }
        unsafe extern "system" fn ScaleFactor<Impl: ICounterItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pivalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScaleFactor() {
                ::core::result::Result::Ok(ok__) => {
                    *pivalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Path<Impl: ICounterItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstrvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Path() {
                ::core::result::Result::Ok(ok__) => {
                    *pstrvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValue<Impl: ICounterItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut f64, status: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetValue(::core::mem::transmute_copy(&value), ::core::mem::transmute_copy(&status)).into()
        }
        unsafe extern "system" fn GetStatistics<Impl: ICounterItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, max: *mut f64, min: *mut f64, avg: *mut f64, status: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetStatistics(::core::mem::transmute_copy(&max), ::core::mem::transmute_copy(&min), ::core::mem::transmute_copy(&avg), ::core::mem::transmute_copy(&status)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Value: Value::<Impl, IMPL_OFFSET>,
            SetColor: SetColor::<Impl, IMPL_OFFSET>,
            Color: Color::<Impl, IMPL_OFFSET>,
            SetWidth: SetWidth::<Impl, IMPL_OFFSET>,
            Width: Width::<Impl, IMPL_OFFSET>,
            SetLineStyle: SetLineStyle::<Impl, IMPL_OFFSET>,
            LineStyle: LineStyle::<Impl, IMPL_OFFSET>,
            SetScaleFactor: SetScaleFactor::<Impl, IMPL_OFFSET>,
            ScaleFactor: ScaleFactor::<Impl, IMPL_OFFSET>,
            Path: Path::<Impl, IMPL_OFFSET>,
            GetValue: GetValue::<Impl, IMPL_OFFSET>,
            GetStatistics: GetStatistics::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICounterItem as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICounterItem2_Impl: Sized + ICounterItem_Impl {
    fn SetSelected(&mut self, bstate: i16) -> ::windows::core::Result<()>;
    fn Selected(&mut self) -> ::windows::core::Result<i16>;
    fn SetVisible(&mut self, bstate: i16) -> ::windows::core::Result<()>;
    fn Visible(&mut self) -> ::windows::core::Result<i16>;
    fn GetDataAt(&mut self, iindex: i32, iwhich: SysmonDataType) -> ::windows::core::Result<super::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICounterItem2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICounterItem2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICounterItem2_Vtbl {
        unsafe extern "system" fn SetSelected<Impl: ICounterItem2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelected(::core::mem::transmute_copy(&bstate)).into()
        }
        unsafe extern "system" fn Selected<Impl: ICounterItem2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Selected() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVisible<Impl: ICounterItem2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVisible(::core::mem::transmute_copy(&bstate)).into()
        }
        unsafe extern "system" fn Visible<Impl: ICounterItem2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Visible() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDataAt<Impl: ICounterItem2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iindex: i32, iwhich: SysmonDataType, pvariant: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDataAt(::core::mem::transmute_copy(&iindex), ::core::mem::transmute_copy(&iwhich)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ICounterItem_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetSelected: SetSelected::<Impl, IMPL_OFFSET>,
            Selected: Selected::<Impl, IMPL_OFFSET>,
            SetVisible: SetVisible::<Impl, IMPL_OFFSET>,
            Visible: Visible::<Impl, IMPL_OFFSET>,
            GetDataAt: GetDataAt::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICounterItem2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICounters_Impl: Sized + super::Com::IDispatch_Impl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Item(&mut self, index: &super::Com::VARIANT) -> ::windows::core::Result<DICounterItem>;
    fn Add(&mut self, pathname: &super::super::Foundation::BSTR) -> ::windows::core::Result<DICounterItem>;
    fn Remove(&mut self, index: &super::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICounters_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICounters_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICounters_Vtbl {
        unsafe extern "system" fn Count<Impl: ICounters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plong: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *plong = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ICounters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppiunk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: ICounters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppi: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppi = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: ICounters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pathname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppi: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Add(::core::mem::transmute_copy(&pathname)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppi = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Impl: ICounters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Remove(::core::mem::transmute_copy(&index)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Count: Count::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            Add: Add::<Impl, IMPL_OFFSET>,
            Remove: Remove::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICounters as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IDataCollector_Impl: Sized + super::Com::IDispatch_Impl {
    fn DataCollectorSet(&mut self) -> ::windows::core::Result<IDataCollectorSet>;
    fn SetDataCollectorSet(&mut self, group: &::core::option::Option<IDataCollectorSet>) -> ::windows::core::Result<()>;
    fn DataCollectorType(&mut self) -> ::windows::core::Result<DataCollectorType>;
    fn FileName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetFileName(&mut self, name: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn FileNameFormat(&mut self) -> ::windows::core::Result<AutoPathFormat>;
    fn SetFileNameFormat(&mut self, format: AutoPathFormat) -> ::windows::core::Result<()>;
    fn FileNameFormatPattern(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetFileNameFormatPattern(&mut self, pattern: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn LatestOutputLocation(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetLatestOutputLocation(&mut self, path: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn LogAppend(&mut self) -> ::windows::core::Result<i16>;
    fn SetLogAppend(&mut self, append: i16) -> ::windows::core::Result<()>;
    fn LogCircular(&mut self) -> ::windows::core::Result<i16>;
    fn SetLogCircular(&mut self, circular: i16) -> ::windows::core::Result<()>;
    fn LogOverwrite(&mut self) -> ::windows::core::Result<i16>;
    fn SetLogOverwrite(&mut self, overwrite: i16) -> ::windows::core::Result<()>;
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetName(&mut self, name: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn OutputLocation(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Index(&mut self) -> ::windows::core::Result<i32>;
    fn SetIndex(&mut self, index: i32) -> ::windows::core::Result<()>;
    fn Xml(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetXml(&mut self, xml: &super::super::Foundation::BSTR) -> ::windows::core::Result<IValueMap>;
    fn CreateOutputLocation(&mut self, latest: i16) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IDataCollector_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollector_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDataCollector_Vtbl {
        unsafe extern "system" fn DataCollectorSet<Impl: IDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, group: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DataCollectorSet() {
                ::core::result::Result::Ok(ok__) => {
                    *group = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDataCollectorSet<Impl: IDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, group: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDataCollectorSet(::core::mem::transmute(&group)).into()
        }
        unsafe extern "system" fn DataCollectorType<Impl: IDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: *mut DataCollectorType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DataCollectorType() {
                ::core::result::Result::Ok(ok__) => {
                    *r#type = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FileName<Impl: IDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FileName() {
                ::core::result::Result::Ok(ok__) => {
                    *name = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFileName<Impl: IDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFileName(::core::mem::transmute_copy(&name)).into()
        }
        unsafe extern "system" fn FileNameFormat<Impl: IDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: *mut AutoPathFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FileNameFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *format = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFileNameFormat<Impl: IDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: AutoPathFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFileNameFormat(::core::mem::transmute_copy(&format)).into()
        }
        unsafe extern "system" fn FileNameFormatPattern<Impl: IDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pattern: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FileNameFormatPattern() {
                ::core::result::Result::Ok(ok__) => {
                    *pattern = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFileNameFormatPattern<Impl: IDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pattern: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFileNameFormatPattern(::core::mem::transmute_copy(&pattern)).into()
        }
        unsafe extern "system" fn LatestOutputLocation<Impl: IDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LatestOutputLocation() {
                ::core::result::Result::Ok(ok__) => {
                    *path = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLatestOutputLocation<Impl: IDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLatestOutputLocation(::core::mem::transmute_copy(&path)).into()
        }
        unsafe extern "system" fn LogAppend<Impl: IDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, append: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LogAppend() {
                ::core::result::Result::Ok(ok__) => {
                    *append = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLogAppend<Impl: IDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, append: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLogAppend(::core::mem::transmute_copy(&append)).into()
        }
        unsafe extern "system" fn LogCircular<Impl: IDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, circular: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LogCircular() {
                ::core::result::Result::Ok(ok__) => {
                    *circular = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLogCircular<Impl: IDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, circular: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLogCircular(::core::mem::transmute_copy(&circular)).into()
        }
        unsafe extern "system" fn LogOverwrite<Impl: IDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, overwrite: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LogOverwrite() {
                ::core::result::Result::Ok(ok__) => {
                    *overwrite = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLogOverwrite<Impl: IDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, overwrite: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLogOverwrite(::core::mem::transmute_copy(&overwrite)).into()
        }
        unsafe extern "system" fn Name<Impl: IDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *name = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: IDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetName(::core::mem::transmute_copy(&name)).into()
        }
        unsafe extern "system" fn OutputLocation<Impl: IDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OutputLocation() {
                ::core::result::Result::Ok(ok__) => {
                    *path = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Index<Impl: IDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Index() {
                ::core::result::Result::Ok(ok__) => {
                    *index = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIndex<Impl: IDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIndex(::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn Xml<Impl: IDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xml: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Xml() {
                ::core::result::Result::Ok(ok__) => {
                    *xml = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetXml<Impl: IDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xml: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, validation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetXml(::core::mem::transmute_copy(&xml)) {
                ::core::result::Result::Ok(ok__) => {
                    *validation = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateOutputLocation<Impl: IDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, latest: i16, location: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateOutputLocation(::core::mem::transmute_copy(&latest)) {
                ::core::result::Result::Ok(ok__) => {
                    *location = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            DataCollectorSet: DataCollectorSet::<Impl, IMPL_OFFSET>,
            SetDataCollectorSet: SetDataCollectorSet::<Impl, IMPL_OFFSET>,
            DataCollectorType: DataCollectorType::<Impl, IMPL_OFFSET>,
            FileName: FileName::<Impl, IMPL_OFFSET>,
            SetFileName: SetFileName::<Impl, IMPL_OFFSET>,
            FileNameFormat: FileNameFormat::<Impl, IMPL_OFFSET>,
            SetFileNameFormat: SetFileNameFormat::<Impl, IMPL_OFFSET>,
            FileNameFormatPattern: FileNameFormatPattern::<Impl, IMPL_OFFSET>,
            SetFileNameFormatPattern: SetFileNameFormatPattern::<Impl, IMPL_OFFSET>,
            LatestOutputLocation: LatestOutputLocation::<Impl, IMPL_OFFSET>,
            SetLatestOutputLocation: SetLatestOutputLocation::<Impl, IMPL_OFFSET>,
            LogAppend: LogAppend::<Impl, IMPL_OFFSET>,
            SetLogAppend: SetLogAppend::<Impl, IMPL_OFFSET>,
            LogCircular: LogCircular::<Impl, IMPL_OFFSET>,
            SetLogCircular: SetLogCircular::<Impl, IMPL_OFFSET>,
            LogOverwrite: LogOverwrite::<Impl, IMPL_OFFSET>,
            SetLogOverwrite: SetLogOverwrite::<Impl, IMPL_OFFSET>,
            Name: Name::<Impl, IMPL_OFFSET>,
            SetName: SetName::<Impl, IMPL_OFFSET>,
            OutputLocation: OutputLocation::<Impl, IMPL_OFFSET>,
            Index: Index::<Impl, IMPL_OFFSET>,
            SetIndex: SetIndex::<Impl, IMPL_OFFSET>,
            Xml: Xml::<Impl, IMPL_OFFSET>,
            SetXml: SetXml::<Impl, IMPL_OFFSET>,
            CreateOutputLocation: CreateOutputLocation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDataCollector as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IDataCollectorCollection_Impl: Sized + super::Com::IDispatch_Impl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn Item(&mut self, index: &super::Com::VARIANT) -> ::windows::core::Result<IDataCollector>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Add(&mut self, collector: &::core::option::Option<IDataCollector>) -> ::windows::core::Result<()>;
    fn Remove(&mut self, collector: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Clear(&mut self) -> ::windows::core::Result<()>;
    fn AddRange(&mut self, collectors: &::core::option::Option<IDataCollectorCollection>) -> ::windows::core::Result<()>;
    fn CreateDataCollectorFromXml(&mut self, bstrxml: &super::super::Foundation::BSTR, pvalidation: *mut ::core::option::Option<IValueMap>, pcollector: *mut ::core::option::Option<IDataCollector>) -> ::windows::core::Result<()>;
    fn CreateDataCollector(&mut self, r#type: DataCollectorType) -> ::windows::core::Result<IDataCollector>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IDataCollectorCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollectorCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDataCollectorCollection_Vtbl {
        unsafe extern "system" fn Count<Impl: IDataCollectorCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IDataCollectorCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, collector: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *collector = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IDataCollectorCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: IDataCollectorCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, collector: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Add(::core::mem::transmute(&collector)).into()
        }
        unsafe extern "system" fn Remove<Impl: IDataCollectorCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, collector: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Remove(::core::mem::transmute_copy(&collector)).into()
        }
        unsafe extern "system" fn Clear<Impl: IDataCollectorCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Clear().into()
        }
        unsafe extern "system" fn AddRange<Impl: IDataCollectorCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, collectors: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddRange(::core::mem::transmute(&collectors)).into()
        }
        unsafe extern "system" fn CreateDataCollectorFromXml<Impl: IDataCollectorCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrxml: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvalidation: *mut ::windows::core::RawPtr, pcollector: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateDataCollectorFromXml(::core::mem::transmute_copy(&bstrxml), ::core::mem::transmute_copy(&pvalidation), ::core::mem::transmute_copy(&pcollector)).into()
        }
        unsafe extern "system" fn CreateDataCollector<Impl: IDataCollectorCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: DataCollectorType, collector: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDataCollector(::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    *collector = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Count: Count::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Add: Add::<Impl, IMPL_OFFSET>,
            Remove: Remove::<Impl, IMPL_OFFSET>,
            Clear: Clear::<Impl, IMPL_OFFSET>,
            AddRange: AddRange::<Impl, IMPL_OFFSET>,
            CreateDataCollectorFromXml: CreateDataCollectorFromXml::<Impl, IMPL_OFFSET>,
            CreateDataCollector: CreateDataCollector::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDataCollectorCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IDataCollectorSet_Impl: Sized + super::Com::IDispatch_Impl {
    fn DataCollectors(&mut self) -> ::windows::core::Result<IDataCollectorCollection>;
    fn Duration(&mut self) -> ::windows::core::Result<u32>;
    fn SetDuration(&mut self, seconds: u32) -> ::windows::core::Result<()>;
    fn Description(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDescription(&mut self, description: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn DescriptionUnresolved(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn DisplayName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDisplayName(&mut self, displayname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn DisplayNameUnresolved(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Keywords(&mut self) -> ::windows::core::Result<*mut super::Com::SAFEARRAY>;
    fn SetKeywords(&mut self, keywords: *const super::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn LatestOutputLocation(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetLatestOutputLocation(&mut self, path: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn OutputLocation(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn RootPath(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetRootPath(&mut self, folder: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Segment(&mut self) -> ::windows::core::Result<i16>;
    fn SetSegment(&mut self, segment: i16) -> ::windows::core::Result<()>;
    fn SegmentMaxDuration(&mut self) -> ::windows::core::Result<u32>;
    fn SetSegmentMaxDuration(&mut self, seconds: u32) -> ::windows::core::Result<()>;
    fn SegmentMaxSize(&mut self) -> ::windows::core::Result<u32>;
    fn SetSegmentMaxSize(&mut self, size: u32) -> ::windows::core::Result<()>;
    fn SerialNumber(&mut self) -> ::windows::core::Result<u32>;
    fn SetSerialNumber(&mut self, index: u32) -> ::windows::core::Result<()>;
    fn Server(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Status(&mut self) -> ::windows::core::Result<DataCollectorSetStatus>;
    fn Subdirectory(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetSubdirectory(&mut self, folder: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SubdirectoryFormat(&mut self) -> ::windows::core::Result<AutoPathFormat>;
    fn SetSubdirectoryFormat(&mut self, format: AutoPathFormat) -> ::windows::core::Result<()>;
    fn SubdirectoryFormatPattern(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetSubdirectoryFormatPattern(&mut self, pattern: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Task(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetTask(&mut self, task: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn TaskRunAsSelf(&mut self) -> ::windows::core::Result<i16>;
    fn SetTaskRunAsSelf(&mut self, runasself: i16) -> ::windows::core::Result<()>;
    fn TaskArguments(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetTaskArguments(&mut self, task: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn TaskUserTextArguments(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetTaskUserTextArguments(&mut self, usertext: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Schedules(&mut self) -> ::windows::core::Result<IScheduleCollection>;
    fn SchedulesEnabled(&mut self) -> ::windows::core::Result<i16>;
    fn SetSchedulesEnabled(&mut self, enabled: i16) -> ::windows::core::Result<()>;
    fn UserAccount(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Xml(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Security(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetSecurity(&mut self, bstrsecurity: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn StopOnCompletion(&mut self) -> ::windows::core::Result<i16>;
    fn SetStopOnCompletion(&mut self, stop: i16) -> ::windows::core::Result<()>;
    fn DataManager(&mut self) -> ::windows::core::Result<IDataManager>;
    fn SetCredentials(&mut self, user: &super::super::Foundation::BSTR, password: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Query(&mut self, name: &super::super::Foundation::BSTR, server: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Commit(&mut self, name: &super::super::Foundation::BSTR, server: &super::super::Foundation::BSTR, mode: CommitMode) -> ::windows::core::Result<IValueMap>;
    fn Delete(&mut self) -> ::windows::core::Result<()>;
    fn Start(&mut self, synchronous: i16) -> ::windows::core::Result<()>;
    fn Stop(&mut self, synchronous: i16) -> ::windows::core::Result<()>;
    fn SetXml(&mut self, xml: &super::super::Foundation::BSTR) -> ::windows::core::Result<IValueMap>;
    fn SetValue(&mut self, key: &super::super::Foundation::BSTR, value: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetValue(&mut self, key: &super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IDataCollectorSet_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollectorSet_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDataCollectorSet_Vtbl {
        unsafe extern "system" fn DataCollectors<Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, collectors: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DataCollectors() {
                ::core::result::Result::Ok(ok__) => {
                    *collectors = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Duration<Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, seconds: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Duration() {
                ::core::result::Result::Ok(ok__) => {
                    *seconds = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDuration<Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, seconds: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDuration(::core::mem::transmute_copy(&seconds)).into()
        }
        unsafe extern "system" fn Description<Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, description: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *description = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, description: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDescription(::core::mem::transmute_copy(&description)).into()
        }
        unsafe extern "system" fn DescriptionUnresolved<Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, descr: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DescriptionUnresolved() {
                ::core::result::Result::Ok(ok__) => {
                    *descr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayName<Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, displayname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *displayname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayName<Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, displayname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisplayName(::core::mem::transmute_copy(&displayname)).into()
        }
        unsafe extern "system" fn DisplayNameUnresolved<Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayNameUnresolved() {
                ::core::result::Result::Ok(ok__) => {
                    *name = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Keywords<Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, keywords: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Keywords() {
                ::core::result::Result::Ok(ok__) => {
                    *keywords = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeywords<Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, keywords: *const super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetKeywords(::core::mem::transmute_copy(&keywords)).into()
        }
        unsafe extern "system" fn LatestOutputLocation<Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LatestOutputLocation() {
                ::core::result::Result::Ok(ok__) => {
                    *path = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLatestOutputLocation<Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLatestOutputLocation(::core::mem::transmute_copy(&path)).into()
        }
        unsafe extern "system" fn Name<Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *name = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OutputLocation<Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OutputLocation() {
                ::core::result::Result::Ok(ok__) => {
                    *path = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RootPath<Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, folder: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RootPath() {
                ::core::result::Result::Ok(ok__) => {
                    *folder = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRootPath<Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, folder: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRootPath(::core::mem::transmute_copy(&folder)).into()
        }
        unsafe extern "system" fn Segment<Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, segment: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Segment() {
                ::core::result::Result::Ok(ok__) => {
                    *segment = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSegment<Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, segment: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSegment(::core::mem::transmute_copy(&segment)).into()
        }
        unsafe extern "system" fn SegmentMaxDuration<Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, seconds: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SegmentMaxDuration() {
                ::core::result::Result::Ok(ok__) => {
                    *seconds = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSegmentMaxDuration<Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, seconds: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSegmentMaxDuration(::core::mem::transmute_copy(&seconds)).into()
        }
        unsafe extern "system" fn SegmentMaxSize<Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, size: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SegmentMaxSize() {
                ::core::result::Result::Ok(ok__) => {
                    *size = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSegmentMaxSize<Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, size: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSegmentMaxSize(::core::mem::transmute_copy(&size)).into()
        }
        unsafe extern "system" fn SerialNumber<Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SerialNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *index = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSerialNumber<Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSerialNumber(::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn Server<Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, server: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Server() {
                ::core::result::Result::Ok(ok__) => {
                    *server = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: *mut DataCollectorSetStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *status = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Subdirectory<Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, folder: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Subdirectory() {
                ::core::result::Result::Ok(ok__) => {
                    *folder = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSubdirectory<Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, folder: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSubdirectory(::core::mem::transmute_copy(&folder)).into()
        }
        unsafe extern "system" fn SubdirectoryFormat<Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: *mut AutoPathFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SubdirectoryFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *format = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSubdirectoryFormat<Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: AutoPathFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSubdirectoryFormat(::core::mem::transmute_copy(&format)).into()
        }
        unsafe extern "system" fn SubdirectoryFormatPattern<Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pattern: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SubdirectoryFormatPattern() {
                ::core::result::Result::Ok(ok__) => {
                    *pattern = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSubdirectoryFormatPattern<Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pattern: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSubdirectoryFormatPattern(::core::mem::transmute_copy(&pattern)).into()
        }
        unsafe extern "system" fn Task<Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, task: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Task() {
                ::core::result::Result::Ok(ok__) => {
                    *task = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTask<Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, task: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTask(::core::mem::transmute_copy(&task)).into()
        }
        unsafe extern "system" fn TaskRunAsSelf<Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, runasself: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TaskRunAsSelf() {
                ::core::result::Result::Ok(ok__) => {
                    *runasself = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTaskRunAsSelf<Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, runasself: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTaskRunAsSelf(::core::mem::transmute_copy(&runasself)).into()
        }
        unsafe extern "system" fn TaskArguments<Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, task: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TaskArguments() {
                ::core::result::Result::Ok(ok__) => {
                    *task = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTaskArguments<Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, task: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTaskArguments(::core::mem::transmute_copy(&task)).into()
        }
        unsafe extern "system" fn TaskUserTextArguments<Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usertext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TaskUserTextArguments() {
                ::core::result::Result::Ok(ok__) => {
                    *usertext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTaskUserTextArguments<Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usertext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTaskUserTextArguments(::core::mem::transmute_copy(&usertext)).into()
        }
        unsafe extern "system" fn Schedules<Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppschedules: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Schedules() {
                ::core::result::Result::Ok(ok__) => {
                    *ppschedules = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SchedulesEnabled<Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SchedulesEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *enabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSchedulesEnabled<Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSchedulesEnabled(::core::mem::transmute_copy(&enabled)).into()
        }
        unsafe extern "system" fn UserAccount<Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserAccount() {
                ::core::result::Result::Ok(ok__) => {
                    *user = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Xml<Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xml: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Xml() {
                ::core::result::Result::Ok(ok__) => {
                    *xml = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Security<Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsecurity: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Security() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrsecurity = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSecurity<Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsecurity: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSecurity(::core::mem::transmute_copy(&bstrsecurity)).into()
        }
        unsafe extern "system" fn StopOnCompletion<Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stop: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StopOnCompletion() {
                ::core::result::Result::Ok(ok__) => {
                    *stop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStopOnCompletion<Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stop: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStopOnCompletion(::core::mem::transmute_copy(&stop)).into()
        }
        unsafe extern "system" fn DataManager<Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, datamanager: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DataManager() {
                ::core::result::Result::Ok(ok__) => {
                    *datamanager = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCredentials<Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, password: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCredentials(::core::mem::transmute_copy(&user), ::core::mem::transmute_copy(&password)).into()
        }
        unsafe extern "system" fn Query<Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, server: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Query(::core::mem::transmute_copy(&name), ::core::mem::transmute_copy(&server)).into()
        }
        unsafe extern "system" fn Commit<Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, server: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, mode: CommitMode, validation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Commit(::core::mem::transmute_copy(&name), ::core::mem::transmute_copy(&server), ::core::mem::transmute_copy(&mode)) {
                ::core::result::Result::Ok(ok__) => {
                    *validation = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Delete().into()
        }
        unsafe extern "system" fn Start<Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, synchronous: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Start(::core::mem::transmute_copy(&synchronous)).into()
        }
        unsafe extern "system" fn Stop<Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, synchronous: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Stop(::core::mem::transmute_copy(&synchronous)).into()
        }
        unsafe extern "system" fn SetXml<Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xml: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, validation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetXml(::core::mem::transmute_copy(&xml)) {
                ::core::result::Result::Ok(ok__) => {
                    *validation = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValue(::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetValue<Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetValue(::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            DataCollectors: DataCollectors::<Impl, IMPL_OFFSET>,
            Duration: Duration::<Impl, IMPL_OFFSET>,
            SetDuration: SetDuration::<Impl, IMPL_OFFSET>,
            Description: Description::<Impl, IMPL_OFFSET>,
            SetDescription: SetDescription::<Impl, IMPL_OFFSET>,
            DescriptionUnresolved: DescriptionUnresolved::<Impl, IMPL_OFFSET>,
            DisplayName: DisplayName::<Impl, IMPL_OFFSET>,
            SetDisplayName: SetDisplayName::<Impl, IMPL_OFFSET>,
            DisplayNameUnresolved: DisplayNameUnresolved::<Impl, IMPL_OFFSET>,
            Keywords: Keywords::<Impl, IMPL_OFFSET>,
            SetKeywords: SetKeywords::<Impl, IMPL_OFFSET>,
            LatestOutputLocation: LatestOutputLocation::<Impl, IMPL_OFFSET>,
            SetLatestOutputLocation: SetLatestOutputLocation::<Impl, IMPL_OFFSET>,
            Name: Name::<Impl, IMPL_OFFSET>,
            OutputLocation: OutputLocation::<Impl, IMPL_OFFSET>,
            RootPath: RootPath::<Impl, IMPL_OFFSET>,
            SetRootPath: SetRootPath::<Impl, IMPL_OFFSET>,
            Segment: Segment::<Impl, IMPL_OFFSET>,
            SetSegment: SetSegment::<Impl, IMPL_OFFSET>,
            SegmentMaxDuration: SegmentMaxDuration::<Impl, IMPL_OFFSET>,
            SetSegmentMaxDuration: SetSegmentMaxDuration::<Impl, IMPL_OFFSET>,
            SegmentMaxSize: SegmentMaxSize::<Impl, IMPL_OFFSET>,
            SetSegmentMaxSize: SetSegmentMaxSize::<Impl, IMPL_OFFSET>,
            SerialNumber: SerialNumber::<Impl, IMPL_OFFSET>,
            SetSerialNumber: SetSerialNumber::<Impl, IMPL_OFFSET>,
            Server: Server::<Impl, IMPL_OFFSET>,
            Status: Status::<Impl, IMPL_OFFSET>,
            Subdirectory: Subdirectory::<Impl, IMPL_OFFSET>,
            SetSubdirectory: SetSubdirectory::<Impl, IMPL_OFFSET>,
            SubdirectoryFormat: SubdirectoryFormat::<Impl, IMPL_OFFSET>,
            SetSubdirectoryFormat: SetSubdirectoryFormat::<Impl, IMPL_OFFSET>,
            SubdirectoryFormatPattern: SubdirectoryFormatPattern::<Impl, IMPL_OFFSET>,
            SetSubdirectoryFormatPattern: SetSubdirectoryFormatPattern::<Impl, IMPL_OFFSET>,
            Task: Task::<Impl, IMPL_OFFSET>,
            SetTask: SetTask::<Impl, IMPL_OFFSET>,
            TaskRunAsSelf: TaskRunAsSelf::<Impl, IMPL_OFFSET>,
            SetTaskRunAsSelf: SetTaskRunAsSelf::<Impl, IMPL_OFFSET>,
            TaskArguments: TaskArguments::<Impl, IMPL_OFFSET>,
            SetTaskArguments: SetTaskArguments::<Impl, IMPL_OFFSET>,
            TaskUserTextArguments: TaskUserTextArguments::<Impl, IMPL_OFFSET>,
            SetTaskUserTextArguments: SetTaskUserTextArguments::<Impl, IMPL_OFFSET>,
            Schedules: Schedules::<Impl, IMPL_OFFSET>,
            SchedulesEnabled: SchedulesEnabled::<Impl, IMPL_OFFSET>,
            SetSchedulesEnabled: SetSchedulesEnabled::<Impl, IMPL_OFFSET>,
            UserAccount: UserAccount::<Impl, IMPL_OFFSET>,
            Xml: Xml::<Impl, IMPL_OFFSET>,
            Security: Security::<Impl, IMPL_OFFSET>,
            SetSecurity: SetSecurity::<Impl, IMPL_OFFSET>,
            StopOnCompletion: StopOnCompletion::<Impl, IMPL_OFFSET>,
            SetStopOnCompletion: SetStopOnCompletion::<Impl, IMPL_OFFSET>,
            DataManager: DataManager::<Impl, IMPL_OFFSET>,
            SetCredentials: SetCredentials::<Impl, IMPL_OFFSET>,
            Query: Query::<Impl, IMPL_OFFSET>,
            Commit: Commit::<Impl, IMPL_OFFSET>,
            Delete: Delete::<Impl, IMPL_OFFSET>,
            Start: Start::<Impl, IMPL_OFFSET>,
            Stop: Stop::<Impl, IMPL_OFFSET>,
            SetXml: SetXml::<Impl, IMPL_OFFSET>,
            SetValue: SetValue::<Impl, IMPL_OFFSET>,
            GetValue: GetValue::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDataCollectorSet as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IDataCollectorSetCollection_Impl: Sized + super::Com::IDispatch_Impl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn Item(&mut self, index: &super::Com::VARIANT) -> ::windows::core::Result<IDataCollectorSet>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Add(&mut self, set: &::core::option::Option<IDataCollectorSet>) -> ::windows::core::Result<()>;
    fn Remove(&mut self, set: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Clear(&mut self) -> ::windows::core::Result<()>;
    fn AddRange(&mut self, sets: &::core::option::Option<IDataCollectorSetCollection>) -> ::windows::core::Result<()>;
    fn GetDataCollectorSets(&mut self, server: &super::super::Foundation::BSTR, filter: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IDataCollectorSetCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollectorSetCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDataCollectorSetCollection_Vtbl {
        unsafe extern "system" fn Count<Impl: IDataCollectorSetCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IDataCollectorSetCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, set: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *set = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IDataCollectorSetCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: IDataCollectorSetCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, set: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Add(::core::mem::transmute(&set)).into()
        }
        unsafe extern "system" fn Remove<Impl: IDataCollectorSetCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, set: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Remove(::core::mem::transmute_copy(&set)).into()
        }
        unsafe extern "system" fn Clear<Impl: IDataCollectorSetCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Clear().into()
        }
        unsafe extern "system" fn AddRange<Impl: IDataCollectorSetCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sets: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddRange(::core::mem::transmute(&sets)).into()
        }
        unsafe extern "system" fn GetDataCollectorSets<Impl: IDataCollectorSetCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, server: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, filter: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDataCollectorSets(::core::mem::transmute_copy(&server), ::core::mem::transmute_copy(&filter)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Count: Count::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Add: Add::<Impl, IMPL_OFFSET>,
            Remove: Remove::<Impl, IMPL_OFFSET>,
            Clear: Clear::<Impl, IMPL_OFFSET>,
            AddRange: AddRange::<Impl, IMPL_OFFSET>,
            GetDataCollectorSets: GetDataCollectorSets::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDataCollectorSetCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IDataManager_Impl: Sized + super::Com::IDispatch_Impl {
    fn Enabled(&mut self) -> ::windows::core::Result<i16>;
    fn SetEnabled(&mut self, fenabled: i16) -> ::windows::core::Result<()>;
    fn CheckBeforeRunning(&mut self) -> ::windows::core::Result<i16>;
    fn SetCheckBeforeRunning(&mut self, fcheck: i16) -> ::windows::core::Result<()>;
    fn MinFreeDisk(&mut self) -> ::windows::core::Result<u32>;
    fn SetMinFreeDisk(&mut self, minfreedisk: u32) -> ::windows::core::Result<()>;
    fn MaxSize(&mut self) -> ::windows::core::Result<u32>;
    fn SetMaxSize(&mut self, ulmaxsize: u32) -> ::windows::core::Result<()>;
    fn MaxFolderCount(&mut self) -> ::windows::core::Result<u32>;
    fn SetMaxFolderCount(&mut self, ulmaxfoldercount: u32) -> ::windows::core::Result<()>;
    fn ResourcePolicy(&mut self) -> ::windows::core::Result<ResourcePolicy>;
    fn SetResourcePolicy(&mut self, policy: ResourcePolicy) -> ::windows::core::Result<()>;
    fn FolderActions(&mut self) -> ::windows::core::Result<IFolderActionCollection>;
    fn ReportSchema(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetReportSchema(&mut self, reportschema: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ReportFileName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetReportFileName(&mut self, pbstrfilename: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn RuleTargetFileName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetRuleTargetFileName(&mut self, filename: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn EventsFileName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetEventsFileName(&mut self, pbstrfilename: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Rules(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetRules(&mut self, bstrxml: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Run(&mut self, steps: DataManagerSteps, bstrfolder: &super::super::Foundation::BSTR) -> ::windows::core::Result<IValueMap>;
    fn Extract(&mut self, cabfilename: &super::super::Foundation::BSTR, destinationpath: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IDataManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDataManager_Vtbl {
        unsafe extern "system" fn Enabled<Impl: IDataManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Enabled() {
                ::core::result::Result::Ok(ok__) => {
                    *pfenabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Impl: IDataManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnabled(::core::mem::transmute_copy(&fenabled)).into()
        }
        unsafe extern "system" fn CheckBeforeRunning<Impl: IDataManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfcheck: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckBeforeRunning() {
                ::core::result::Result::Ok(ok__) => {
                    *pfcheck = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCheckBeforeRunning<Impl: IDataManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fcheck: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCheckBeforeRunning(::core::mem::transmute_copy(&fcheck)).into()
        }
        unsafe extern "system" fn MinFreeDisk<Impl: IDataManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minfreedisk: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinFreeDisk() {
                ::core::result::Result::Ok(ok__) => {
                    *minfreedisk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinFreeDisk<Impl: IDataManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minfreedisk: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMinFreeDisk(::core::mem::transmute_copy(&minfreedisk)).into()
        }
        unsafe extern "system" fn MaxSize<Impl: IDataManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulmaxsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxSize() {
                ::core::result::Result::Ok(ok__) => {
                    *pulmaxsize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxSize<Impl: IDataManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulmaxsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxSize(::core::mem::transmute_copy(&ulmaxsize)).into()
        }
        unsafe extern "system" fn MaxFolderCount<Impl: IDataManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulmaxfoldercount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxFolderCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pulmaxfoldercount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxFolderCount<Impl: IDataManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulmaxfoldercount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxFolderCount(::core::mem::transmute_copy(&ulmaxfoldercount)).into()
        }
        unsafe extern "system" fn ResourcePolicy<Impl: IDataManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppolicy: *mut ResourcePolicy) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResourcePolicy() {
                ::core::result::Result::Ok(ok__) => {
                    *ppolicy = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetResourcePolicy<Impl: IDataManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, policy: ResourcePolicy) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetResourcePolicy(::core::mem::transmute_copy(&policy)).into()
        }
        unsafe extern "system" fn FolderActions<Impl: IDataManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, actions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FolderActions() {
                ::core::result::Result::Ok(ok__) => {
                    *actions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportSchema<Impl: IDataManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reportschema: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportSchema() {
                ::core::result::Result::Ok(ok__) => {
                    *reportschema = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReportSchema<Impl: IDataManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reportschema: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReportSchema(::core::mem::transmute_copy(&reportschema)).into()
        }
        unsafe extern "system" fn ReportFileName<Impl: IDataManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrfilename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportFileName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrfilename = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReportFileName<Impl: IDataManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReportFileName(::core::mem::transmute_copy(&pbstrfilename)).into()
        }
        unsafe extern "system" fn RuleTargetFileName<Impl: IDataManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RuleTargetFileName() {
                ::core::result::Result::Ok(ok__) => {
                    *filename = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRuleTargetFileName<Impl: IDataManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRuleTargetFileName(::core::mem::transmute_copy(&filename)).into()
        }
        unsafe extern "system" fn EventsFileName<Impl: IDataManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrfilename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EventsFileName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrfilename = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventsFileName<Impl: IDataManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEventsFileName(::core::mem::transmute_copy(&pbstrfilename)).into()
        }
        unsafe extern "system" fn Rules<Impl: IDataManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrxml: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Rules() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrxml = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRules<Impl: IDataManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrxml: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRules(::core::mem::transmute_copy(&bstrxml)).into()
        }
        unsafe extern "system" fn Run<Impl: IDataManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, steps: DataManagerSteps, bstrfolder: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, errors: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Run(::core::mem::transmute_copy(&steps), ::core::mem::transmute_copy(&bstrfolder)) {
                ::core::result::Result::Ok(ok__) => {
                    *errors = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Extract<Impl: IDataManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cabfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, destinationpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Extract(::core::mem::transmute_copy(&cabfilename), ::core::mem::transmute_copy(&destinationpath)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Enabled: Enabled::<Impl, IMPL_OFFSET>,
            SetEnabled: SetEnabled::<Impl, IMPL_OFFSET>,
            CheckBeforeRunning: CheckBeforeRunning::<Impl, IMPL_OFFSET>,
            SetCheckBeforeRunning: SetCheckBeforeRunning::<Impl, IMPL_OFFSET>,
            MinFreeDisk: MinFreeDisk::<Impl, IMPL_OFFSET>,
            SetMinFreeDisk: SetMinFreeDisk::<Impl, IMPL_OFFSET>,
            MaxSize: MaxSize::<Impl, IMPL_OFFSET>,
            SetMaxSize: SetMaxSize::<Impl, IMPL_OFFSET>,
            MaxFolderCount: MaxFolderCount::<Impl, IMPL_OFFSET>,
            SetMaxFolderCount: SetMaxFolderCount::<Impl, IMPL_OFFSET>,
            ResourcePolicy: ResourcePolicy::<Impl, IMPL_OFFSET>,
            SetResourcePolicy: SetResourcePolicy::<Impl, IMPL_OFFSET>,
            FolderActions: FolderActions::<Impl, IMPL_OFFSET>,
            ReportSchema: ReportSchema::<Impl, IMPL_OFFSET>,
            SetReportSchema: SetReportSchema::<Impl, IMPL_OFFSET>,
            ReportFileName: ReportFileName::<Impl, IMPL_OFFSET>,
            SetReportFileName: SetReportFileName::<Impl, IMPL_OFFSET>,
            RuleTargetFileName: RuleTargetFileName::<Impl, IMPL_OFFSET>,
            SetRuleTargetFileName: SetRuleTargetFileName::<Impl, IMPL_OFFSET>,
            EventsFileName: EventsFileName::<Impl, IMPL_OFFSET>,
            SetEventsFileName: SetEventsFileName::<Impl, IMPL_OFFSET>,
            Rules: Rules::<Impl, IMPL_OFFSET>,
            SetRules: SetRules::<Impl, IMPL_OFFSET>,
            Run: Run::<Impl, IMPL_OFFSET>,
            Extract: Extract::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDataManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFolderAction_Impl: Sized + super::Com::IDispatch_Impl {
    fn Age(&mut self) -> ::windows::core::Result<u32>;
    fn SetAge(&mut self, ulage: u32) -> ::windows::core::Result<()>;
    fn Size(&mut self) -> ::windows::core::Result<u32>;
    fn SetSize(&mut self, ulage: u32) -> ::windows::core::Result<()>;
    fn Actions(&mut self) -> ::windows::core::Result<FolderActionSteps>;
    fn SetActions(&mut self, steps: FolderActionSteps) -> ::windows::core::Result<()>;
    fn SendCabTo(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetSendCabTo(&mut self, bstrdestination: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFolderAction_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFolderAction_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFolderAction_Vtbl {
        unsafe extern "system" fn Age<Impl: IFolderAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulage: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Age() {
                ::core::result::Result::Ok(ok__) => {
                    *pulage = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAge<Impl: IFolderAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulage: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAge(::core::mem::transmute_copy(&ulage)).into()
        }
        unsafe extern "system" fn Size<Impl: IFolderAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulage: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Size() {
                ::core::result::Result::Ok(ok__) => {
                    *pulage = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSize<Impl: IFolderAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulage: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSize(::core::mem::transmute_copy(&ulage)).into()
        }
        unsafe extern "system" fn Actions<Impl: IFolderAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, steps: *mut FolderActionSteps) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Actions() {
                ::core::result::Result::Ok(ok__) => {
                    *steps = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetActions<Impl: IFolderAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, steps: FolderActionSteps) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetActions(::core::mem::transmute_copy(&steps)).into()
        }
        unsafe extern "system" fn SendCabTo<Impl: IFolderAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdestination: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SendCabTo() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrdestination = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSendCabTo<Impl: IFolderAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdestination: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSendCabTo(::core::mem::transmute_copy(&bstrdestination)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Age: Age::<Impl, IMPL_OFFSET>,
            SetAge: SetAge::<Impl, IMPL_OFFSET>,
            Size: Size::<Impl, IMPL_OFFSET>,
            SetSize: SetSize::<Impl, IMPL_OFFSET>,
            Actions: Actions::<Impl, IMPL_OFFSET>,
            SetActions: SetActions::<Impl, IMPL_OFFSET>,
            SendCabTo: SendCabTo::<Impl, IMPL_OFFSET>,
            SetSendCabTo: SetSendCabTo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFolderAction as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFolderActionCollection_Impl: Sized + super::Com::IDispatch_Impl {
    fn Count(&mut self) -> ::windows::core::Result<u32>;
    fn Item(&mut self, index: &super::Com::VARIANT) -> ::windows::core::Result<IFolderAction>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Add(&mut self, action: &::core::option::Option<IFolderAction>) -> ::windows::core::Result<()>;
    fn Remove(&mut self, index: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Clear(&mut self) -> ::windows::core::Result<()>;
    fn AddRange(&mut self, actions: &::core::option::Option<IFolderActionCollection>) -> ::windows::core::Result<()>;
    fn CreateFolderAction(&mut self) -> ::windows::core::Result<IFolderAction>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFolderActionCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFolderActionCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFolderActionCollection_Vtbl {
        unsafe extern "system" fn Count<Impl: IFolderActionCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IFolderActionCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, action: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *action = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IFolderActionCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#enum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *r#enum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: IFolderActionCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, action: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Add(::core::mem::transmute(&action)).into()
        }
        unsafe extern "system" fn Remove<Impl: IFolderActionCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Remove(::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn Clear<Impl: IFolderActionCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Clear().into()
        }
        unsafe extern "system" fn AddRange<Impl: IFolderActionCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, actions: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddRange(::core::mem::transmute(&actions)).into()
        }
        unsafe extern "system" fn CreateFolderAction<Impl: IFolderActionCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, folderaction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFolderAction() {
                ::core::result::Result::Ok(ok__) => {
                    *folderaction = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Count: Count::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Add: Add::<Impl, IMPL_OFFSET>,
            Remove: Remove::<Impl, IMPL_OFFSET>,
            Clear: Clear::<Impl, IMPL_OFFSET>,
            AddRange: AddRange::<Impl, IMPL_OFFSET>,
            CreateFolderAction: CreateFolderAction::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFolderActionCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ILogFileItem_Impl: Sized {
    fn Path(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl ILogFileItem_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILogFileItem_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILogFileItem_Vtbl {
        unsafe extern "system" fn Path<Impl: ILogFileItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstrvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Path() {
                ::core::result::Result::Ok(ok__) => {
                    *pstrvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Path: Path::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILogFileItem as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ILogFiles_Impl: Sized + super::Com::IDispatch_Impl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Item(&mut self, index: &super::Com::VARIANT) -> ::windows::core::Result<DILogFileItem>;
    fn Add(&mut self, pathname: &super::super::Foundation::BSTR) -> ::windows::core::Result<DILogFileItem>;
    fn Remove(&mut self, index: &super::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ILogFiles_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILogFiles_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILogFiles_Vtbl {
        unsafe extern "system" fn Count<Impl: ILogFiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plong: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *plong = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ILogFiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppiunk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: ILogFiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppi: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppi = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: ILogFiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pathname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppi: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Add(::core::mem::transmute_copy(&pathname)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppi = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Impl: ILogFiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Remove(::core::mem::transmute_copy(&index)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Count: Count::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            Add: Add::<Impl, IMPL_OFFSET>,
            Remove: Remove::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILogFiles as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPerformanceCounterDataCollector_Impl: Sized + super::Com::IDispatch_Impl + IDataCollector_Impl {
    fn DataSourceName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDataSourceName(&mut self, dsn: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn PerformanceCounters(&mut self) -> ::windows::core::Result<*mut super::Com::SAFEARRAY>;
    fn SetPerformanceCounters(&mut self, counters: *const super::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn LogFileFormat(&mut self) -> ::windows::core::Result<FileFormat>;
    fn SetLogFileFormat(&mut self, format: FileFormat) -> ::windows::core::Result<()>;
    fn SampleInterval(&mut self) -> ::windows::core::Result<u32>;
    fn SetSampleInterval(&mut self, interval: u32) -> ::windows::core::Result<()>;
    fn SegmentMaxRecords(&mut self) -> ::windows::core::Result<u32>;
    fn SetSegmentMaxRecords(&mut self, records: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPerformanceCounterDataCollector_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPerformanceCounterDataCollector_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPerformanceCounterDataCollector_Vtbl {
        unsafe extern "system" fn DataSourceName<Impl: IPerformanceCounterDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dsn: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DataSourceName() {
                ::core::result::Result::Ok(ok__) => {
                    *dsn = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDataSourceName<Impl: IPerformanceCounterDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dsn: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDataSourceName(::core::mem::transmute_copy(&dsn)).into()
        }
        unsafe extern "system" fn PerformanceCounters<Impl: IPerformanceCounterDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, counters: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PerformanceCounters() {
                ::core::result::Result::Ok(ok__) => {
                    *counters = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPerformanceCounters<Impl: IPerformanceCounterDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, counters: *const super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPerformanceCounters(::core::mem::transmute_copy(&counters)).into()
        }
        unsafe extern "system" fn LogFileFormat<Impl: IPerformanceCounterDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: *mut FileFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LogFileFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *format = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLogFileFormat<Impl: IPerformanceCounterDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: FileFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLogFileFormat(::core::mem::transmute_copy(&format)).into()
        }
        unsafe extern "system" fn SampleInterval<Impl: IPerformanceCounterDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SampleInterval() {
                ::core::result::Result::Ok(ok__) => {
                    *interval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSampleInterval<Impl: IPerformanceCounterDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interval: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSampleInterval(::core::mem::transmute_copy(&interval)).into()
        }
        unsafe extern "system" fn SegmentMaxRecords<Impl: IPerformanceCounterDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, records: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SegmentMaxRecords() {
                ::core::result::Result::Ok(ok__) => {
                    *records = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSegmentMaxRecords<Impl: IPerformanceCounterDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, records: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSegmentMaxRecords(::core::mem::transmute_copy(&records)).into()
        }
        Self {
            base: IDataCollector_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            DataSourceName: DataSourceName::<Impl, IMPL_OFFSET>,
            SetDataSourceName: SetDataSourceName::<Impl, IMPL_OFFSET>,
            PerformanceCounters: PerformanceCounters::<Impl, IMPL_OFFSET>,
            SetPerformanceCounters: SetPerformanceCounters::<Impl, IMPL_OFFSET>,
            LogFileFormat: LogFileFormat::<Impl, IMPL_OFFSET>,
            SetLogFileFormat: SetLogFileFormat::<Impl, IMPL_OFFSET>,
            SampleInterval: SampleInterval::<Impl, IMPL_OFFSET>,
            SetSampleInterval: SetSampleInterval::<Impl, IMPL_OFFSET>,
            SegmentMaxRecords: SegmentMaxRecords::<Impl, IMPL_OFFSET>,
            SetSegmentMaxRecords: SetSegmentMaxRecords::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPerformanceCounterDataCollector as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISchedule_Impl: Sized + super::Com::IDispatch_Impl {
    fn StartDate(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetStartDate(&mut self, start: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn EndDate(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetEndDate(&mut self, end: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn StartTime(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetStartTime(&mut self, start: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Days(&mut self) -> ::windows::core::Result<WeekDays>;
    fn SetDays(&mut self, days: WeekDays) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISchedule_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISchedule_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISchedule_Vtbl {
        unsafe extern "system" fn StartDate<Impl: ISchedule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, start: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartDate() {
                ::core::result::Result::Ok(ok__) => {
                    *start = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartDate<Impl: ISchedule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, start: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStartDate(::core::mem::transmute_copy(&start)).into()
        }
        unsafe extern "system" fn EndDate<Impl: ISchedule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, end: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndDate() {
                ::core::result::Result::Ok(ok__) => {
                    *end = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEndDate<Impl: ISchedule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, end: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEndDate(::core::mem::transmute_copy(&end)).into()
        }
        unsafe extern "system" fn StartTime<Impl: ISchedule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, start: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartTime() {
                ::core::result::Result::Ok(ok__) => {
                    *start = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartTime<Impl: ISchedule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, start: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStartTime(::core::mem::transmute_copy(&start)).into()
        }
        unsafe extern "system" fn Days<Impl: ISchedule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, days: *mut WeekDays) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Days() {
                ::core::result::Result::Ok(ok__) => {
                    *days = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDays<Impl: ISchedule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, days: WeekDays) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDays(::core::mem::transmute_copy(&days)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            StartDate: StartDate::<Impl, IMPL_OFFSET>,
            SetStartDate: SetStartDate::<Impl, IMPL_OFFSET>,
            EndDate: EndDate::<Impl, IMPL_OFFSET>,
            SetEndDate: SetEndDate::<Impl, IMPL_OFFSET>,
            StartTime: StartTime::<Impl, IMPL_OFFSET>,
            SetStartTime: SetStartTime::<Impl, IMPL_OFFSET>,
            Days: Days::<Impl, IMPL_OFFSET>,
            SetDays: SetDays::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISchedule as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IScheduleCollection_Impl: Sized + super::Com::IDispatch_Impl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn Item(&mut self, index: &super::Com::VARIANT) -> ::windows::core::Result<ISchedule>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Add(&mut self, pschedule: &::core::option::Option<ISchedule>) -> ::windows::core::Result<()>;
    fn Remove(&mut self, vschedule: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Clear(&mut self) -> ::windows::core::Result<()>;
    fn AddRange(&mut self, pschedules: &::core::option::Option<IScheduleCollection>) -> ::windows::core::Result<()>;
    fn CreateSchedule(&mut self) -> ::windows::core::Result<ISchedule>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IScheduleCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IScheduleCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IScheduleCollection_Vtbl {
        unsafe extern "system" fn Count<Impl: IScheduleCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IScheduleCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppschedule: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppschedule = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IScheduleCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ienum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ienum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: IScheduleCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pschedule: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Add(::core::mem::transmute(&pschedule)).into()
        }
        unsafe extern "system" fn Remove<Impl: IScheduleCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vschedule: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Remove(::core::mem::transmute_copy(&vschedule)).into()
        }
        unsafe extern "system" fn Clear<Impl: IScheduleCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Clear().into()
        }
        unsafe extern "system" fn AddRange<Impl: IScheduleCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pschedules: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddRange(::core::mem::transmute(&pschedules)).into()
        }
        unsafe extern "system" fn CreateSchedule<Impl: IScheduleCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, schedule: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSchedule() {
                ::core::result::Result::Ok(ok__) => {
                    *schedule = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Count: Count::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Add: Add::<Impl, IMPL_OFFSET>,
            Remove: Remove::<Impl, IMPL_OFFSET>,
            Clear: Clear::<Impl, IMPL_OFFSET>,
            AddRange: AddRange::<Impl, IMPL_OFFSET>,
            CreateSchedule: CreateSchedule::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IScheduleCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISystemMonitor_Impl: Sized {
    fn Appearance(&mut self) -> ::windows::core::Result<i32>;
    fn SetAppearance(&mut self, iappearance: i32) -> ::windows::core::Result<()>;
    fn BackColor(&mut self) -> ::windows::core::Result<u32>;
    fn SetBackColor(&mut self, color: u32) -> ::windows::core::Result<()>;
    fn BorderStyle(&mut self) -> ::windows::core::Result<i32>;
    fn SetBorderStyle(&mut self, iborderstyle: i32) -> ::windows::core::Result<()>;
    fn ForeColor(&mut self) -> ::windows::core::Result<u32>;
    fn SetForeColor(&mut self, color: u32) -> ::windows::core::Result<()>;
    fn Font(&mut self) -> ::windows::core::Result<super::Ole::IFontDisp>;
    fn putref_Font(&mut self, pfont: &::core::option::Option<super::Ole::IFontDisp>) -> ::windows::core::Result<()>;
    fn Counters(&mut self) -> ::windows::core::Result<ICounters>;
    fn SetShowVerticalGrid(&mut self, bstate: i16) -> ::windows::core::Result<()>;
    fn ShowVerticalGrid(&mut self) -> ::windows::core::Result<i16>;
    fn SetShowHorizontalGrid(&mut self, bstate: i16) -> ::windows::core::Result<()>;
    fn ShowHorizontalGrid(&mut self) -> ::windows::core::Result<i16>;
    fn SetShowLegend(&mut self, bstate: i16) -> ::windows::core::Result<()>;
    fn ShowLegend(&mut self) -> ::windows::core::Result<i16>;
    fn SetShowScaleLabels(&mut self, bstate: i16) -> ::windows::core::Result<()>;
    fn ShowScaleLabels(&mut self) -> ::windows::core::Result<i16>;
    fn SetShowValueBar(&mut self, bstate: i16) -> ::windows::core::Result<()>;
    fn ShowValueBar(&mut self) -> ::windows::core::Result<i16>;
    fn SetMaximumScale(&mut self, ivalue: i32) -> ::windows::core::Result<()>;
    fn MaximumScale(&mut self) -> ::windows::core::Result<i32>;
    fn SetMinimumScale(&mut self, ivalue: i32) -> ::windows::core::Result<()>;
    fn MinimumScale(&mut self) -> ::windows::core::Result<i32>;
    fn SetUpdateInterval(&mut self, fvalue: f32) -> ::windows::core::Result<()>;
    fn UpdateInterval(&mut self) -> ::windows::core::Result<f32>;
    fn SetDisplayType(&mut self, edisplaytype: DisplayTypeConstants) -> ::windows::core::Result<()>;
    fn DisplayType(&mut self) -> ::windows::core::Result<DisplayTypeConstants>;
    fn SetManualUpdate(&mut self, bstate: i16) -> ::windows::core::Result<()>;
    fn ManualUpdate(&mut self) -> ::windows::core::Result<i16>;
    fn SetGraphTitle(&mut self, bstitle: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GraphTitle(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetYAxisLabel(&mut self, bstitle: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn YAxisLabel(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CollectSample(&mut self) -> ::windows::core::Result<()>;
    fn UpdateGraph(&mut self) -> ::windows::core::Result<()>;
    fn BrowseCounters(&mut self) -> ::windows::core::Result<()>;
    fn DisplayProperties(&mut self) -> ::windows::core::Result<()>;
    fn Counter(&mut self, iindex: i32) -> ::windows::core::Result<ICounterItem>;
    fn AddCounter(&mut self, bspath: &super::super::Foundation::BSTR) -> ::windows::core::Result<ICounterItem>;
    fn DeleteCounter(&mut self, pctr: &::core::option::Option<ICounterItem>) -> ::windows::core::Result<()>;
    fn BackColorCtl(&mut self) -> ::windows::core::Result<u32>;
    fn SetBackColorCtl(&mut self, color: u32) -> ::windows::core::Result<()>;
    fn SetLogFileName(&mut self, bsfilename: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn LogFileName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetLogViewStart(&mut self, starttime: f64) -> ::windows::core::Result<()>;
    fn LogViewStart(&mut self) -> ::windows::core::Result<f64>;
    fn SetLogViewStop(&mut self, stoptime: f64) -> ::windows::core::Result<()>;
    fn LogViewStop(&mut self) -> ::windows::core::Result<f64>;
    fn GridColor(&mut self) -> ::windows::core::Result<u32>;
    fn SetGridColor(&mut self, color: u32) -> ::windows::core::Result<()>;
    fn TimeBarColor(&mut self) -> ::windows::core::Result<u32>;
    fn SetTimeBarColor(&mut self, color: u32) -> ::windows::core::Result<()>;
    fn Highlight(&mut self) -> ::windows::core::Result<i16>;
    fn SetHighlight(&mut self, bstate: i16) -> ::windows::core::Result<()>;
    fn ShowToolbar(&mut self) -> ::windows::core::Result<i16>;
    fn SetShowToolbar(&mut self, bstate: i16) -> ::windows::core::Result<()>;
    fn Paste(&mut self) -> ::windows::core::Result<()>;
    fn Copy(&mut self) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn SetReadOnly(&mut self, bstate: i16) -> ::windows::core::Result<()>;
    fn ReadOnly(&mut self) -> ::windows::core::Result<i16>;
    fn SetReportValueType(&mut self, ereportvaluetype: ReportValueTypeConstants) -> ::windows::core::Result<()>;
    fn ReportValueType(&mut self) -> ::windows::core::Result<ReportValueTypeConstants>;
    fn SetMonitorDuplicateInstances(&mut self, bstate: i16) -> ::windows::core::Result<()>;
    fn MonitorDuplicateInstances(&mut self) -> ::windows::core::Result<i16>;
    fn SetDisplayFilter(&mut self, ivalue: i32) -> ::windows::core::Result<()>;
    fn DisplayFilter(&mut self) -> ::windows::core::Result<i32>;
    fn LogFiles(&mut self) -> ::windows::core::Result<ILogFiles>;
    fn SetDataSourceType(&mut self, edatasourcetype: DataSourceTypeConstants) -> ::windows::core::Result<()>;
    fn DataSourceType(&mut self) -> ::windows::core::Result<DataSourceTypeConstants>;
    fn SetSqlDsnName(&mut self, bssqldsnname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SqlDsnName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetSqlLogSetName(&mut self, bssqllogsetname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SqlLogSetName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISystemMonitor_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISystemMonitor_Vtbl {
        unsafe extern "system" fn Appearance<Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iappearance: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Appearance() {
                ::core::result::Result::Ok(ok__) => {
                    *iappearance = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAppearance<Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iappearance: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAppearance(::core::mem::transmute_copy(&iappearance)).into()
        }
        unsafe extern "system" fn BackColor<Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcolor: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BackColor() {
                ::core::result::Result::Ok(ok__) => {
                    *pcolor = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBackColor<Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, color: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBackColor(::core::mem::transmute_copy(&color)).into()
        }
        unsafe extern "system" fn BorderStyle<Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iborderstyle: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BorderStyle() {
                ::core::result::Result::Ok(ok__) => {
                    *iborderstyle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBorderStyle<Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iborderstyle: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBorderStyle(::core::mem::transmute_copy(&iborderstyle)).into()
        }
        unsafe extern "system" fn ForeColor<Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcolor: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ForeColor() {
                ::core::result::Result::Ok(ok__) => {
                    *pcolor = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetForeColor<Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, color: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetForeColor(::core::mem::transmute_copy(&color)).into()
        }
        unsafe extern "system" fn Font<Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfont: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Font() {
                ::core::result::Result::Ok(ok__) => {
                    *ppfont = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_Font<Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfont: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).putref_Font(::core::mem::transmute(&pfont)).into()
        }
        unsafe extern "system" fn Counters<Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppicounters: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Counters() {
                ::core::result::Result::Ok(ok__) => {
                    *ppicounters = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShowVerticalGrid<Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetShowVerticalGrid(::core::mem::transmute_copy(&bstate)).into()
        }
        unsafe extern "system" fn ShowVerticalGrid<Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowVerticalGrid() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShowHorizontalGrid<Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetShowHorizontalGrid(::core::mem::transmute_copy(&bstate)).into()
        }
        unsafe extern "system" fn ShowHorizontalGrid<Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowHorizontalGrid() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShowLegend<Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetShowLegend(::core::mem::transmute_copy(&bstate)).into()
        }
        unsafe extern "system" fn ShowLegend<Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowLegend() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShowScaleLabels<Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetShowScaleLabels(::core::mem::transmute_copy(&bstate)).into()
        }
        unsafe extern "system" fn ShowScaleLabels<Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowScaleLabels() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShowValueBar<Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetShowValueBar(::core::mem::transmute_copy(&bstate)).into()
        }
        unsafe extern "system" fn ShowValueBar<Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowValueBar() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaximumScale<Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ivalue: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaximumScale(::core::mem::transmute_copy(&ivalue)).into()
        }
        unsafe extern "system" fn MaximumScale<Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pivalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaximumScale() {
                ::core::result::Result::Ok(ok__) => {
                    *pivalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinimumScale<Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ivalue: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMinimumScale(::core::mem::transmute_copy(&ivalue)).into()
        }
        unsafe extern "system" fn MinimumScale<Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pivalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinimumScale() {
                ::core::result::Result::Ok(ok__) => {
                    *pivalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUpdateInterval<Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fvalue: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUpdateInterval(::core::mem::transmute_copy(&fvalue)).into()
        }
        unsafe extern "system" fn UpdateInterval<Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfvalue: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpdateInterval() {
                ::core::result::Result::Ok(ok__) => {
                    *pfvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayType<Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, edisplaytype: DisplayTypeConstants) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisplayType(::core::mem::transmute_copy(&edisplaytype)).into()
        }
        unsafe extern "system" fn DisplayType<Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pedisplaytype: *mut DisplayTypeConstants) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayType() {
                ::core::result::Result::Ok(ok__) => {
                    *pedisplaytype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetManualUpdate<Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetManualUpdate(::core::mem::transmute_copy(&bstate)).into()
        }
        unsafe extern "system" fn ManualUpdate<Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ManualUpdate() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGraphTitle<Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstitle: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGraphTitle(::core::mem::transmute_copy(&bstitle)).into()
        }
        unsafe extern "system" fn GraphTitle<Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstitle: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GraphTitle() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstitle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetYAxisLabel<Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstitle: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetYAxisLabel(::core::mem::transmute_copy(&bstitle)).into()
        }
        unsafe extern "system" fn YAxisLabel<Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstitle: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).YAxisLabel() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstitle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CollectSample<Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CollectSample().into()
        }
        unsafe extern "system" fn UpdateGraph<Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpdateGraph().into()
        }
        unsafe extern "system" fn BrowseCounters<Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BrowseCounters().into()
        }
        unsafe extern "system" fn DisplayProperties<Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DisplayProperties().into()
        }
        unsafe extern "system" fn Counter<Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iindex: i32, ppicounter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Counter(::core::mem::transmute_copy(&iindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppicounter = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddCounter<Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bspath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppicounter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddCounter(::core::mem::transmute_copy(&bspath)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppicounter = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteCounter<Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pctr: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteCounter(::core::mem::transmute(&pctr)).into()
        }
        unsafe extern "system" fn BackColorCtl<Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcolor: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BackColorCtl() {
                ::core::result::Result::Ok(ok__) => {
                    *pcolor = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBackColorCtl<Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, color: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBackColorCtl(::core::mem::transmute_copy(&color)).into()
        }
        unsafe extern "system" fn SetLogFileName<Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bsfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLogFileName(::core::mem::transmute_copy(&bsfilename)).into()
        }
        unsafe extern "system" fn LogFileName<Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bsfilename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LogFileName() {
                ::core::result::Result::Ok(ok__) => {
                    *bsfilename = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLogViewStart<Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, starttime: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLogViewStart(::core::mem::transmute_copy(&starttime)).into()
        }
        unsafe extern "system" fn LogViewStart<Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, starttime: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LogViewStart() {
                ::core::result::Result::Ok(ok__) => {
                    *starttime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLogViewStop<Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stoptime: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLogViewStop(::core::mem::transmute_copy(&stoptime)).into()
        }
        unsafe extern "system" fn LogViewStop<Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stoptime: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LogViewStop() {
                ::core::result::Result::Ok(ok__) => {
                    *stoptime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GridColor<Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcolor: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GridColor() {
                ::core::result::Result::Ok(ok__) => {
                    *pcolor = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGridColor<Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, color: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGridColor(::core::mem::transmute_copy(&color)).into()
        }
        unsafe extern "system" fn TimeBarColor<Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcolor: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TimeBarColor() {
                ::core::result::Result::Ok(ok__) => {
                    *pcolor = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTimeBarColor<Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, color: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTimeBarColor(::core::mem::transmute_copy(&color)).into()
        }
        unsafe extern "system" fn Highlight<Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Highlight() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHighlight<Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHighlight(::core::mem::transmute_copy(&bstate)).into()
        }
        unsafe extern "system" fn ShowToolbar<Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowToolbar() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShowToolbar<Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetShowToolbar(::core::mem::transmute_copy(&bstate)).into()
        }
        unsafe extern "system" fn Paste<Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Paste().into()
        }
        unsafe extern "system" fn Copy<Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Copy().into()
        }
        unsafe extern "system" fn Reset<Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn SetReadOnly<Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReadOnly(::core::mem::transmute_copy(&bstate)).into()
        }
        unsafe extern "system" fn ReadOnly<Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadOnly() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReportValueType<Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ereportvaluetype: ReportValueTypeConstants) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReportValueType(::core::mem::transmute_copy(&ereportvaluetype)).into()
        }
        unsafe extern "system" fn ReportValueType<Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pereportvaluetype: *mut ReportValueTypeConstants) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportValueType() {
                ::core::result::Result::Ok(ok__) => {
                    *pereportvaluetype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMonitorDuplicateInstances<Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMonitorDuplicateInstances(::core::mem::transmute_copy(&bstate)).into()
        }
        unsafe extern "system" fn MonitorDuplicateInstances<Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MonitorDuplicateInstances() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayFilter<Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ivalue: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisplayFilter(::core::mem::transmute_copy(&ivalue)).into()
        }
        unsafe extern "system" fn DisplayFilter<Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pivalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayFilter() {
                ::core::result::Result::Ok(ok__) => {
                    *pivalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LogFiles<Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppilogfiles: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LogFiles() {
                ::core::result::Result::Ok(ok__) => {
                    *ppilogfiles = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDataSourceType<Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, edatasourcetype: DataSourceTypeConstants) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDataSourceType(::core::mem::transmute_copy(&edatasourcetype)).into()
        }
        unsafe extern "system" fn DataSourceType<Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pedatasourcetype: *mut DataSourceTypeConstants) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DataSourceType() {
                ::core::result::Result::Ok(ok__) => {
                    *pedatasourcetype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSqlDsnName<Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bssqldsnname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSqlDsnName(::core::mem::transmute_copy(&bssqldsnname)).into()
        }
        unsafe extern "system" fn SqlDsnName<Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bssqldsnname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SqlDsnName() {
                ::core::result::Result::Ok(ok__) => {
                    *bssqldsnname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSqlLogSetName<Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bssqllogsetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSqlLogSetName(::core::mem::transmute_copy(&bssqllogsetname)).into()
        }
        unsafe extern "system" fn SqlLogSetName<Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bssqllogsetname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SqlLogSetName() {
                ::core::result::Result::Ok(ok__) => {
                    *bssqllogsetname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Appearance: Appearance::<Impl, IMPL_OFFSET>,
            SetAppearance: SetAppearance::<Impl, IMPL_OFFSET>,
            BackColor: BackColor::<Impl, IMPL_OFFSET>,
            SetBackColor: SetBackColor::<Impl, IMPL_OFFSET>,
            BorderStyle: BorderStyle::<Impl, IMPL_OFFSET>,
            SetBorderStyle: SetBorderStyle::<Impl, IMPL_OFFSET>,
            ForeColor: ForeColor::<Impl, IMPL_OFFSET>,
            SetForeColor: SetForeColor::<Impl, IMPL_OFFSET>,
            Font: Font::<Impl, IMPL_OFFSET>,
            putref_Font: putref_Font::<Impl, IMPL_OFFSET>,
            Counters: Counters::<Impl, IMPL_OFFSET>,
            SetShowVerticalGrid: SetShowVerticalGrid::<Impl, IMPL_OFFSET>,
            ShowVerticalGrid: ShowVerticalGrid::<Impl, IMPL_OFFSET>,
            SetShowHorizontalGrid: SetShowHorizontalGrid::<Impl, IMPL_OFFSET>,
            ShowHorizontalGrid: ShowHorizontalGrid::<Impl, IMPL_OFFSET>,
            SetShowLegend: SetShowLegend::<Impl, IMPL_OFFSET>,
            ShowLegend: ShowLegend::<Impl, IMPL_OFFSET>,
            SetShowScaleLabels: SetShowScaleLabels::<Impl, IMPL_OFFSET>,
            ShowScaleLabels: ShowScaleLabels::<Impl, IMPL_OFFSET>,
            SetShowValueBar: SetShowValueBar::<Impl, IMPL_OFFSET>,
            ShowValueBar: ShowValueBar::<Impl, IMPL_OFFSET>,
            SetMaximumScale: SetMaximumScale::<Impl, IMPL_OFFSET>,
            MaximumScale: MaximumScale::<Impl, IMPL_OFFSET>,
            SetMinimumScale: SetMinimumScale::<Impl, IMPL_OFFSET>,
            MinimumScale: MinimumScale::<Impl, IMPL_OFFSET>,
            SetUpdateInterval: SetUpdateInterval::<Impl, IMPL_OFFSET>,
            UpdateInterval: UpdateInterval::<Impl, IMPL_OFFSET>,
            SetDisplayType: SetDisplayType::<Impl, IMPL_OFFSET>,
            DisplayType: DisplayType::<Impl, IMPL_OFFSET>,
            SetManualUpdate: SetManualUpdate::<Impl, IMPL_OFFSET>,
            ManualUpdate: ManualUpdate::<Impl, IMPL_OFFSET>,
            SetGraphTitle: SetGraphTitle::<Impl, IMPL_OFFSET>,
            GraphTitle: GraphTitle::<Impl, IMPL_OFFSET>,
            SetYAxisLabel: SetYAxisLabel::<Impl, IMPL_OFFSET>,
            YAxisLabel: YAxisLabel::<Impl, IMPL_OFFSET>,
            CollectSample: CollectSample::<Impl, IMPL_OFFSET>,
            UpdateGraph: UpdateGraph::<Impl, IMPL_OFFSET>,
            BrowseCounters: BrowseCounters::<Impl, IMPL_OFFSET>,
            DisplayProperties: DisplayProperties::<Impl, IMPL_OFFSET>,
            Counter: Counter::<Impl, IMPL_OFFSET>,
            AddCounter: AddCounter::<Impl, IMPL_OFFSET>,
            DeleteCounter: DeleteCounter::<Impl, IMPL_OFFSET>,
            BackColorCtl: BackColorCtl::<Impl, IMPL_OFFSET>,
            SetBackColorCtl: SetBackColorCtl::<Impl, IMPL_OFFSET>,
            SetLogFileName: SetLogFileName::<Impl, IMPL_OFFSET>,
            LogFileName: LogFileName::<Impl, IMPL_OFFSET>,
            SetLogViewStart: SetLogViewStart::<Impl, IMPL_OFFSET>,
            LogViewStart: LogViewStart::<Impl, IMPL_OFFSET>,
            SetLogViewStop: SetLogViewStop::<Impl, IMPL_OFFSET>,
            LogViewStop: LogViewStop::<Impl, IMPL_OFFSET>,
            GridColor: GridColor::<Impl, IMPL_OFFSET>,
            SetGridColor: SetGridColor::<Impl, IMPL_OFFSET>,
            TimeBarColor: TimeBarColor::<Impl, IMPL_OFFSET>,
            SetTimeBarColor: SetTimeBarColor::<Impl, IMPL_OFFSET>,
            Highlight: Highlight::<Impl, IMPL_OFFSET>,
            SetHighlight: SetHighlight::<Impl, IMPL_OFFSET>,
            ShowToolbar: ShowToolbar::<Impl, IMPL_OFFSET>,
            SetShowToolbar: SetShowToolbar::<Impl, IMPL_OFFSET>,
            Paste: Paste::<Impl, IMPL_OFFSET>,
            Copy: Copy::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            SetReadOnly: SetReadOnly::<Impl, IMPL_OFFSET>,
            ReadOnly: ReadOnly::<Impl, IMPL_OFFSET>,
            SetReportValueType: SetReportValueType::<Impl, IMPL_OFFSET>,
            ReportValueType: ReportValueType::<Impl, IMPL_OFFSET>,
            SetMonitorDuplicateInstances: SetMonitorDuplicateInstances::<Impl, IMPL_OFFSET>,
            MonitorDuplicateInstances: MonitorDuplicateInstances::<Impl, IMPL_OFFSET>,
            SetDisplayFilter: SetDisplayFilter::<Impl, IMPL_OFFSET>,
            DisplayFilter: DisplayFilter::<Impl, IMPL_OFFSET>,
            LogFiles: LogFiles::<Impl, IMPL_OFFSET>,
            SetDataSourceType: SetDataSourceType::<Impl, IMPL_OFFSET>,
            DataSourceType: DataSourceType::<Impl, IMPL_OFFSET>,
            SetSqlDsnName: SetSqlDsnName::<Impl, IMPL_OFFSET>,
            SqlDsnName: SqlDsnName::<Impl, IMPL_OFFSET>,
            SetSqlLogSetName: SetSqlLogSetName::<Impl, IMPL_OFFSET>,
            SqlLogSetName: SqlLogSetName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISystemMonitor as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISystemMonitor2_Impl: Sized + ISystemMonitor_Impl {
    fn SetEnableDigitGrouping(&mut self, bstate: i16) -> ::windows::core::Result<()>;
    fn EnableDigitGrouping(&mut self) -> ::windows::core::Result<i16>;
    fn SetEnableToolTips(&mut self, bstate: i16) -> ::windows::core::Result<()>;
    fn EnableToolTips(&mut self) -> ::windows::core::Result<i16>;
    fn SetShowTimeAxisLabels(&mut self, bstate: i16) -> ::windows::core::Result<()>;
    fn ShowTimeAxisLabels(&mut self) -> ::windows::core::Result<i16>;
    fn SetChartScroll(&mut self, bscroll: i16) -> ::windows::core::Result<()>;
    fn ChartScroll(&mut self) -> ::windows::core::Result<i16>;
    fn SetDataPointCount(&mut self, inewcount: i32) -> ::windows::core::Result<()>;
    fn DataPointCount(&mut self) -> ::windows::core::Result<i32>;
    fn ScaleToFit(&mut self, bselectedcountersonly: i16) -> ::windows::core::Result<()>;
    fn SaveAs(&mut self, bstrfilename: &super::super::Foundation::BSTR, esysmonfiletype: SysmonFileType) -> ::windows::core::Result<()>;
    fn Relog(&mut self, bstrfilename: &super::super::Foundation::BSTR, esysmonfiletype: SysmonFileType, ifilter: i32) -> ::windows::core::Result<()>;
    fn ClearData(&mut self) -> ::windows::core::Result<()>;
    fn LogSourceStartTime(&mut self) -> ::windows::core::Result<f64>;
    fn LogSourceStopTime(&mut self) -> ::windows::core::Result<f64>;
    fn SetLogViewRange(&mut self, starttime: f64, stoptime: f64) -> ::windows::core::Result<()>;
    fn GetLogViewRange(&mut self, starttime: *mut f64, stoptime: *mut f64) -> ::windows::core::Result<()>;
    fn BatchingLock(&mut self, flock: i16, ebatchreason: SysmonBatchReason) -> ::windows::core::Result<()>;
    fn LoadSettings(&mut self, bstrsettingfilename: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISystemMonitor2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISystemMonitor2_Vtbl {
        unsafe extern "system" fn SetEnableDigitGrouping<Impl: ISystemMonitor2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnableDigitGrouping(::core::mem::transmute_copy(&bstate)).into()
        }
        unsafe extern "system" fn EnableDigitGrouping<Impl: ISystemMonitor2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnableDigitGrouping() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnableToolTips<Impl: ISystemMonitor2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnableToolTips(::core::mem::transmute_copy(&bstate)).into()
        }
        unsafe extern "system" fn EnableToolTips<Impl: ISystemMonitor2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnableToolTips() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShowTimeAxisLabels<Impl: ISystemMonitor2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetShowTimeAxisLabels(::core::mem::transmute_copy(&bstate)).into()
        }
        unsafe extern "system" fn ShowTimeAxisLabels<Impl: ISystemMonitor2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowTimeAxisLabels() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChartScroll<Impl: ISystemMonitor2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bscroll: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetChartScroll(::core::mem::transmute_copy(&bscroll)).into()
        }
        unsafe extern "system" fn ChartScroll<Impl: ISystemMonitor2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbscroll: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ChartScroll() {
                ::core::result::Result::Ok(ok__) => {
                    *pbscroll = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDataPointCount<Impl: ISystemMonitor2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inewcount: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDataPointCount(::core::mem::transmute_copy(&inewcount)).into()
        }
        unsafe extern "system" fn DataPointCount<Impl: ISystemMonitor2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidatapointcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DataPointCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pidatapointcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScaleToFit<Impl: ISystemMonitor2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bselectedcountersonly: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ScaleToFit(::core::mem::transmute_copy(&bselectedcountersonly)).into()
        }
        unsafe extern "system" fn SaveAs<Impl: ISystemMonitor2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, esysmonfiletype: SysmonFileType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SaveAs(::core::mem::transmute_copy(&bstrfilename), ::core::mem::transmute_copy(&esysmonfiletype)).into()
        }
        unsafe extern "system" fn Relog<Impl: ISystemMonitor2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, esysmonfiletype: SysmonFileType, ifilter: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Relog(::core::mem::transmute_copy(&bstrfilename), ::core::mem::transmute_copy(&esysmonfiletype), ::core::mem::transmute_copy(&ifilter)).into()
        }
        unsafe extern "system" fn ClearData<Impl: ISystemMonitor2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearData().into()
        }
        unsafe extern "system" fn LogSourceStartTime<Impl: ISystemMonitor2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdate: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LogSourceStartTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pdate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LogSourceStopTime<Impl: ISystemMonitor2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdate: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LogSourceStopTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pdate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLogViewRange<Impl: ISystemMonitor2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, starttime: f64, stoptime: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLogViewRange(::core::mem::transmute_copy(&starttime), ::core::mem::transmute_copy(&stoptime)).into()
        }
        unsafe extern "system" fn GetLogViewRange<Impl: ISystemMonitor2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, starttime: *mut f64, stoptime: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetLogViewRange(::core::mem::transmute_copy(&starttime), ::core::mem::transmute_copy(&stoptime)).into()
        }
        unsafe extern "system" fn BatchingLock<Impl: ISystemMonitor2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flock: i16, ebatchreason: SysmonBatchReason) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BatchingLock(::core::mem::transmute_copy(&flock), ::core::mem::transmute_copy(&ebatchreason)).into()
        }
        unsafe extern "system" fn LoadSettings<Impl: ISystemMonitor2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsettingfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LoadSettings(::core::mem::transmute_copy(&bstrsettingfilename)).into()
        }
        Self {
            base: ISystemMonitor_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetEnableDigitGrouping: SetEnableDigitGrouping::<Impl, IMPL_OFFSET>,
            EnableDigitGrouping: EnableDigitGrouping::<Impl, IMPL_OFFSET>,
            SetEnableToolTips: SetEnableToolTips::<Impl, IMPL_OFFSET>,
            EnableToolTips: EnableToolTips::<Impl, IMPL_OFFSET>,
            SetShowTimeAxisLabels: SetShowTimeAxisLabels::<Impl, IMPL_OFFSET>,
            ShowTimeAxisLabels: ShowTimeAxisLabels::<Impl, IMPL_OFFSET>,
            SetChartScroll: SetChartScroll::<Impl, IMPL_OFFSET>,
            ChartScroll: ChartScroll::<Impl, IMPL_OFFSET>,
            SetDataPointCount: SetDataPointCount::<Impl, IMPL_OFFSET>,
            DataPointCount: DataPointCount::<Impl, IMPL_OFFSET>,
            ScaleToFit: ScaleToFit::<Impl, IMPL_OFFSET>,
            SaveAs: SaveAs::<Impl, IMPL_OFFSET>,
            Relog: Relog::<Impl, IMPL_OFFSET>,
            ClearData: ClearData::<Impl, IMPL_OFFSET>,
            LogSourceStartTime: LogSourceStartTime::<Impl, IMPL_OFFSET>,
            LogSourceStopTime: LogSourceStopTime::<Impl, IMPL_OFFSET>,
            SetLogViewRange: SetLogViewRange::<Impl, IMPL_OFFSET>,
            GetLogViewRange: GetLogViewRange::<Impl, IMPL_OFFSET>,
            BatchingLock: BatchingLock::<Impl, IMPL_OFFSET>,
            LoadSettings: LoadSettings::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISystemMonitor2 as ::windows::core::Interface>::IID
    }
}
pub trait ISystemMonitorEvents_Impl: Sized {
    fn OnCounterSelected(&mut self, index: i32);
    fn OnCounterAdded(&mut self, index: i32);
    fn OnCounterDeleted(&mut self, index: i32);
    fn OnSampleCollected(&mut self);
    fn OnDblClick(&mut self, index: i32);
}
impl ISystemMonitorEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitorEvents_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISystemMonitorEvents_Vtbl {
        unsafe extern "system" fn OnCounterSelected<Impl: ISystemMonitorEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnCounterSelected(::core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn OnCounterAdded<Impl: ISystemMonitorEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnCounterAdded(::core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn OnCounterDeleted<Impl: ISystemMonitorEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnCounterDeleted(::core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn OnSampleCollected<Impl: ISystemMonitorEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnSampleCollected()
        }
        unsafe extern "system" fn OnDblClick<Impl: ISystemMonitorEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnDblClick(::core::mem::transmute_copy(&index))
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnCounterSelected: OnCounterSelected::<Impl, IMPL_OFFSET>,
            OnCounterAdded: OnCounterAdded::<Impl, IMPL_OFFSET>,
            OnCounterDeleted: OnCounterDeleted::<Impl, IMPL_OFFSET>,
            OnSampleCollected: OnSampleCollected::<Impl, IMPL_OFFSET>,
            OnDblClick: OnDblClick::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISystemMonitorEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITraceDataCollector_Impl: Sized + super::Com::IDispatch_Impl + IDataCollector_Impl {
    fn BufferSize(&mut self) -> ::windows::core::Result<u32>;
    fn SetBufferSize(&mut self, size: u32) -> ::windows::core::Result<()>;
    fn BuffersLost(&mut self) -> ::windows::core::Result<u32>;
    fn SetBuffersLost(&mut self, buffers: u32) -> ::windows::core::Result<()>;
    fn BuffersWritten(&mut self) -> ::windows::core::Result<u32>;
    fn SetBuffersWritten(&mut self, buffers: u32) -> ::windows::core::Result<()>;
    fn ClockType(&mut self) -> ::windows::core::Result<ClockType>;
    fn SetClockType(&mut self, clock: ClockType) -> ::windows::core::Result<()>;
    fn EventsLost(&mut self) -> ::windows::core::Result<u32>;
    fn SetEventsLost(&mut self, events: u32) -> ::windows::core::Result<()>;
    fn ExtendedModes(&mut self) -> ::windows::core::Result<u32>;
    fn SetExtendedModes(&mut self, mode: u32) -> ::windows::core::Result<()>;
    fn FlushTimer(&mut self) -> ::windows::core::Result<u32>;
    fn SetFlushTimer(&mut self, seconds: u32) -> ::windows::core::Result<()>;
    fn FreeBuffers(&mut self) -> ::windows::core::Result<u32>;
    fn SetFreeBuffers(&mut self, buffers: u32) -> ::windows::core::Result<()>;
    fn Guid(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn SetGuid(&mut self, guid: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn IsKernelTrace(&mut self) -> ::windows::core::Result<i16>;
    fn MaximumBuffers(&mut self) -> ::windows::core::Result<u32>;
    fn SetMaximumBuffers(&mut self, buffers: u32) -> ::windows::core::Result<()>;
    fn MinimumBuffers(&mut self) -> ::windows::core::Result<u32>;
    fn SetMinimumBuffers(&mut self, buffers: u32) -> ::windows::core::Result<()>;
    fn NumberOfBuffers(&mut self) -> ::windows::core::Result<u32>;
    fn SetNumberOfBuffers(&mut self, buffers: u32) -> ::windows::core::Result<()>;
    fn PreallocateFile(&mut self) -> ::windows::core::Result<i16>;
    fn SetPreallocateFile(&mut self, allocate: i16) -> ::windows::core::Result<()>;
    fn ProcessMode(&mut self) -> ::windows::core::Result<i16>;
    fn SetProcessMode(&mut self, process: i16) -> ::windows::core::Result<()>;
    fn RealTimeBuffersLost(&mut self) -> ::windows::core::Result<u32>;
    fn SetRealTimeBuffersLost(&mut self, buffers: u32) -> ::windows::core::Result<()>;
    fn SessionId(&mut self) -> ::windows::core::Result<u64>;
    fn SetSessionId(&mut self, id: u64) -> ::windows::core::Result<()>;
    fn SessionName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetSessionName(&mut self, name: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SessionThreadId(&mut self) -> ::windows::core::Result<u32>;
    fn SetSessionThreadId(&mut self, tid: u32) -> ::windows::core::Result<()>;
    fn StreamMode(&mut self) -> ::windows::core::Result<StreamMode>;
    fn SetStreamMode(&mut self, mode: StreamMode) -> ::windows::core::Result<()>;
    fn TraceDataProviders(&mut self) -> ::windows::core::Result<ITraceDataProviderCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITraceDataCollector_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITraceDataCollector_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITraceDataCollector_Vtbl {
        unsafe extern "system" fn BufferSize<Impl: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, size: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BufferSize() {
                ::core::result::Result::Ok(ok__) => {
                    *size = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBufferSize<Impl: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, size: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBufferSize(::core::mem::transmute_copy(&size)).into()
        }
        unsafe extern "system" fn BuffersLost<Impl: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffers: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BuffersLost() {
                ::core::result::Result::Ok(ok__) => {
                    *buffers = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBuffersLost<Impl: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffers: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBuffersLost(::core::mem::transmute_copy(&buffers)).into()
        }
        unsafe extern "system" fn BuffersWritten<Impl: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffers: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BuffersWritten() {
                ::core::result::Result::Ok(ok__) => {
                    *buffers = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBuffersWritten<Impl: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffers: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBuffersWritten(::core::mem::transmute_copy(&buffers)).into()
        }
        unsafe extern "system" fn ClockType<Impl: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clock: *mut ClockType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClockType() {
                ::core::result::Result::Ok(ok__) => {
                    *clock = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClockType<Impl: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clock: ClockType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClockType(::core::mem::transmute_copy(&clock)).into()
        }
        unsafe extern "system" fn EventsLost<Impl: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, events: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EventsLost() {
                ::core::result::Result::Ok(ok__) => {
                    *events = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventsLost<Impl: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, events: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEventsLost(::core::mem::transmute_copy(&events)).into()
        }
        unsafe extern "system" fn ExtendedModes<Impl: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtendedModes() {
                ::core::result::Result::Ok(ok__) => {
                    *mode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExtendedModes<Impl: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExtendedModes(::core::mem::transmute_copy(&mode)).into()
        }
        unsafe extern "system" fn FlushTimer<Impl: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, seconds: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FlushTimer() {
                ::core::result::Result::Ok(ok__) => {
                    *seconds = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFlushTimer<Impl: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, seconds: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFlushTimer(::core::mem::transmute_copy(&seconds)).into()
        }
        unsafe extern "system" fn FreeBuffers<Impl: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffers: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FreeBuffers() {
                ::core::result::Result::Ok(ok__) => {
                    *buffers = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFreeBuffers<Impl: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffers: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFreeBuffers(::core::mem::transmute_copy(&buffers)).into()
        }
        unsafe extern "system" fn Guid<Impl: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Guid() {
                ::core::result::Result::Ok(ok__) => {
                    *guid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGuid<Impl: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGuid(::core::mem::transmute_copy(&guid)).into()
        }
        unsafe extern "system" fn IsKernelTrace<Impl: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, kernel: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsKernelTrace() {
                ::core::result::Result::Ok(ok__) => {
                    *kernel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaximumBuffers<Impl: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffers: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaximumBuffers() {
                ::core::result::Result::Ok(ok__) => {
                    *buffers = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaximumBuffers<Impl: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffers: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaximumBuffers(::core::mem::transmute_copy(&buffers)).into()
        }
        unsafe extern "system" fn MinimumBuffers<Impl: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffers: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinimumBuffers() {
                ::core::result::Result::Ok(ok__) => {
                    *buffers = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinimumBuffers<Impl: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffers: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMinimumBuffers(::core::mem::transmute_copy(&buffers)).into()
        }
        unsafe extern "system" fn NumberOfBuffers<Impl: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffers: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NumberOfBuffers() {
                ::core::result::Result::Ok(ok__) => {
                    *buffers = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNumberOfBuffers<Impl: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffers: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNumberOfBuffers(::core::mem::transmute_copy(&buffers)).into()
        }
        unsafe extern "system" fn PreallocateFile<Impl: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allocate: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PreallocateFile() {
                ::core::result::Result::Ok(ok__) => {
                    *allocate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPreallocateFile<Impl: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allocate: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPreallocateFile(::core::mem::transmute_copy(&allocate)).into()
        }
        unsafe extern "system" fn ProcessMode<Impl: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, process: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProcessMode() {
                ::core::result::Result::Ok(ok__) => {
                    *process = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProcessMode<Impl: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, process: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProcessMode(::core::mem::transmute_copy(&process)).into()
        }
        unsafe extern "system" fn RealTimeBuffersLost<Impl: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffers: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RealTimeBuffersLost() {
                ::core::result::Result::Ok(ok__) => {
                    *buffers = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRealTimeBuffersLost<Impl: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffers: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRealTimeBuffersLost(::core::mem::transmute_copy(&buffers)).into()
        }
        unsafe extern "system" fn SessionId<Impl: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SessionId() {
                ::core::result::Result::Ok(ok__) => {
                    *id = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSessionId<Impl: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSessionId(::core::mem::transmute_copy(&id)).into()
        }
        unsafe extern "system" fn SessionName<Impl: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SessionName() {
                ::core::result::Result::Ok(ok__) => {
                    *name = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSessionName<Impl: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSessionName(::core::mem::transmute_copy(&name)).into()
        }
        unsafe extern "system" fn SessionThreadId<Impl: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SessionThreadId() {
                ::core::result::Result::Ok(ok__) => {
                    *tid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSessionThreadId<Impl: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSessionThreadId(::core::mem::transmute_copy(&tid)).into()
        }
        unsafe extern "system" fn StreamMode<Impl: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: *mut StreamMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StreamMode() {
                ::core::result::Result::Ok(ok__) => {
                    *mode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStreamMode<Impl: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: StreamMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStreamMode(::core::mem::transmute_copy(&mode)).into()
        }
        unsafe extern "system" fn TraceDataProviders<Impl: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, providers: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TraceDataProviders() {
                ::core::result::Result::Ok(ok__) => {
                    *providers = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDataCollector_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            BufferSize: BufferSize::<Impl, IMPL_OFFSET>,
            SetBufferSize: SetBufferSize::<Impl, IMPL_OFFSET>,
            BuffersLost: BuffersLost::<Impl, IMPL_OFFSET>,
            SetBuffersLost: SetBuffersLost::<Impl, IMPL_OFFSET>,
            BuffersWritten: BuffersWritten::<Impl, IMPL_OFFSET>,
            SetBuffersWritten: SetBuffersWritten::<Impl, IMPL_OFFSET>,
            ClockType: ClockType::<Impl, IMPL_OFFSET>,
            SetClockType: SetClockType::<Impl, IMPL_OFFSET>,
            EventsLost: EventsLost::<Impl, IMPL_OFFSET>,
            SetEventsLost: SetEventsLost::<Impl, IMPL_OFFSET>,
            ExtendedModes: ExtendedModes::<Impl, IMPL_OFFSET>,
            SetExtendedModes: SetExtendedModes::<Impl, IMPL_OFFSET>,
            FlushTimer: FlushTimer::<Impl, IMPL_OFFSET>,
            SetFlushTimer: SetFlushTimer::<Impl, IMPL_OFFSET>,
            FreeBuffers: FreeBuffers::<Impl, IMPL_OFFSET>,
            SetFreeBuffers: SetFreeBuffers::<Impl, IMPL_OFFSET>,
            Guid: Guid::<Impl, IMPL_OFFSET>,
            SetGuid: SetGuid::<Impl, IMPL_OFFSET>,
            IsKernelTrace: IsKernelTrace::<Impl, IMPL_OFFSET>,
            MaximumBuffers: MaximumBuffers::<Impl, IMPL_OFFSET>,
            SetMaximumBuffers: SetMaximumBuffers::<Impl, IMPL_OFFSET>,
            MinimumBuffers: MinimumBuffers::<Impl, IMPL_OFFSET>,
            SetMinimumBuffers: SetMinimumBuffers::<Impl, IMPL_OFFSET>,
            NumberOfBuffers: NumberOfBuffers::<Impl, IMPL_OFFSET>,
            SetNumberOfBuffers: SetNumberOfBuffers::<Impl, IMPL_OFFSET>,
            PreallocateFile: PreallocateFile::<Impl, IMPL_OFFSET>,
            SetPreallocateFile: SetPreallocateFile::<Impl, IMPL_OFFSET>,
            ProcessMode: ProcessMode::<Impl, IMPL_OFFSET>,
            SetProcessMode: SetProcessMode::<Impl, IMPL_OFFSET>,
            RealTimeBuffersLost: RealTimeBuffersLost::<Impl, IMPL_OFFSET>,
            SetRealTimeBuffersLost: SetRealTimeBuffersLost::<Impl, IMPL_OFFSET>,
            SessionId: SessionId::<Impl, IMPL_OFFSET>,
            SetSessionId: SetSessionId::<Impl, IMPL_OFFSET>,
            SessionName: SessionName::<Impl, IMPL_OFFSET>,
            SetSessionName: SetSessionName::<Impl, IMPL_OFFSET>,
            SessionThreadId: SessionThreadId::<Impl, IMPL_OFFSET>,
            SetSessionThreadId: SetSessionThreadId::<Impl, IMPL_OFFSET>,
            StreamMode: StreamMode::<Impl, IMPL_OFFSET>,
            SetStreamMode: SetStreamMode::<Impl, IMPL_OFFSET>,
            TraceDataProviders: TraceDataProviders::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITraceDataCollector as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITraceDataProvider_Impl: Sized + super::Com::IDispatch_Impl {
    fn DisplayName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDisplayName(&mut self, name: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Guid(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn SetGuid(&mut self, guid: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn Level(&mut self) -> ::windows::core::Result<IValueMap>;
    fn KeywordsAny(&mut self) -> ::windows::core::Result<IValueMap>;
    fn KeywordsAll(&mut self) -> ::windows::core::Result<IValueMap>;
    fn Properties(&mut self) -> ::windows::core::Result<IValueMap>;
    fn FilterEnabled(&mut self) -> ::windows::core::Result<i16>;
    fn SetFilterEnabled(&mut self, filterenabled: i16) -> ::windows::core::Result<()>;
    fn FilterType(&mut self) -> ::windows::core::Result<u32>;
    fn SetFilterType(&mut self, ultype: u32) -> ::windows::core::Result<()>;
    fn FilterData(&mut self) -> ::windows::core::Result<*mut super::Com::SAFEARRAY>;
    fn SetFilterData(&mut self, pdata: *const super::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn Query(&mut self, bstrname: &super::super::Foundation::BSTR, bstrserver: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Resolve(&mut self, pfrom: &::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<()>;
    fn SetSecurity(&mut self, sddl: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetSecurity(&mut self, securityinfo: u32) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetRegisteredProcesses(&mut self) -> ::windows::core::Result<IValueMap>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITraceDataProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITraceDataProvider_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITraceDataProvider_Vtbl {
        unsafe extern "system" fn DisplayName<Impl: ITraceDataProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *name = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayName<Impl: ITraceDataProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisplayName(::core::mem::transmute_copy(&name)).into()
        }
        unsafe extern "system" fn Guid<Impl: ITraceDataProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Guid() {
                ::core::result::Result::Ok(ok__) => {
                    *guid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGuid<Impl: ITraceDataProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGuid(::core::mem::transmute_copy(&guid)).into()
        }
        unsafe extern "system" fn Level<Impl: ITraceDataProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplevel: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Level() {
                ::core::result::Result::Ok(ok__) => {
                    *pplevel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeywordsAny<Impl: ITraceDataProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppkeywords: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeywordsAny() {
                ::core::result::Result::Ok(ok__) => {
                    *ppkeywords = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeywordsAll<Impl: ITraceDataProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppkeywords: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeywordsAll() {
                ::core::result::Result::Ok(ok__) => {
                    *ppkeywords = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: ITraceDataProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FilterEnabled<Impl: ITraceDataProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filterenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FilterEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *filterenabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFilterEnabled<Impl: ITraceDataProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filterenabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFilterEnabled(::core::mem::transmute_copy(&filterenabled)).into()
        }
        unsafe extern "system" fn FilterType<Impl: ITraceDataProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pultype: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FilterType() {
                ::core::result::Result::Ok(ok__) => {
                    *pultype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFilterType<Impl: ITraceDataProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ultype: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFilterType(::core::mem::transmute_copy(&ultype)).into()
        }
        unsafe extern "system" fn FilterData<Impl: ITraceDataProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdata: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FilterData() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFilterData<Impl: ITraceDataProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *const super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFilterData(::core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn Query<Impl: ITraceDataProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrserver: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Query(::core::mem::transmute_copy(&bstrname), ::core::mem::transmute_copy(&bstrserver)).into()
        }
        unsafe extern "system" fn Resolve<Impl: ITraceDataProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfrom: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Resolve(::core::mem::transmute(&pfrom)).into()
        }
        unsafe extern "system" fn SetSecurity<Impl: ITraceDataProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sddl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSecurity(::core::mem::transmute_copy(&sddl)).into()
        }
        unsafe extern "system" fn GetSecurity<Impl: ITraceDataProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, securityinfo: u32, sddl: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSecurity(::core::mem::transmute_copy(&securityinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *sddl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRegisteredProcesses<Impl: ITraceDataProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, processes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRegisteredProcesses() {
                ::core::result::Result::Ok(ok__) => {
                    *processes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            DisplayName: DisplayName::<Impl, IMPL_OFFSET>,
            SetDisplayName: SetDisplayName::<Impl, IMPL_OFFSET>,
            Guid: Guid::<Impl, IMPL_OFFSET>,
            SetGuid: SetGuid::<Impl, IMPL_OFFSET>,
            Level: Level::<Impl, IMPL_OFFSET>,
            KeywordsAny: KeywordsAny::<Impl, IMPL_OFFSET>,
            KeywordsAll: KeywordsAll::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
            FilterEnabled: FilterEnabled::<Impl, IMPL_OFFSET>,
            SetFilterEnabled: SetFilterEnabled::<Impl, IMPL_OFFSET>,
            FilterType: FilterType::<Impl, IMPL_OFFSET>,
            SetFilterType: SetFilterType::<Impl, IMPL_OFFSET>,
            FilterData: FilterData::<Impl, IMPL_OFFSET>,
            SetFilterData: SetFilterData::<Impl, IMPL_OFFSET>,
            Query: Query::<Impl, IMPL_OFFSET>,
            Resolve: Resolve::<Impl, IMPL_OFFSET>,
            SetSecurity: SetSecurity::<Impl, IMPL_OFFSET>,
            GetSecurity: GetSecurity::<Impl, IMPL_OFFSET>,
            GetRegisteredProcesses: GetRegisteredProcesses::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITraceDataProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITraceDataProviderCollection_Impl: Sized + super::Com::IDispatch_Impl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn Item(&mut self, index: &super::Com::VARIANT) -> ::windows::core::Result<ITraceDataProvider>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Add(&mut self, pprovider: &::core::option::Option<ITraceDataProvider>) -> ::windows::core::Result<()>;
    fn Remove(&mut self, vprovider: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Clear(&mut self) -> ::windows::core::Result<()>;
    fn AddRange(&mut self, providers: &::core::option::Option<ITraceDataProviderCollection>) -> ::windows::core::Result<()>;
    fn CreateTraceDataProvider(&mut self) -> ::windows::core::Result<ITraceDataProvider>;
    fn GetTraceDataProviders(&mut self, server: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetTraceDataProvidersByProcess(&mut self, server: &super::super::Foundation::BSTR, pid: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITraceDataProviderCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITraceDataProviderCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITraceDataProviderCollection_Vtbl {
        unsafe extern "system" fn Count<Impl: ITraceDataProviderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: ITraceDataProviderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppprovider: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppprovider = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ITraceDataProviderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: ITraceDataProviderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprovider: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Add(::core::mem::transmute(&pprovider)).into()
        }
        unsafe extern "system" fn Remove<Impl: ITraceDataProviderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vprovider: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Remove(::core::mem::transmute_copy(&vprovider)).into()
        }
        unsafe extern "system" fn Clear<Impl: ITraceDataProviderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Clear().into()
        }
        unsafe extern "system" fn AddRange<Impl: ITraceDataProviderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, providers: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddRange(::core::mem::transmute(&providers)).into()
        }
        unsafe extern "system" fn CreateTraceDataProvider<Impl: ITraceDataProviderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, provider: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTraceDataProvider() {
                ::core::result::Result::Ok(ok__) => {
                    *provider = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTraceDataProviders<Impl: ITraceDataProviderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, server: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetTraceDataProviders(::core::mem::transmute_copy(&server)).into()
        }
        unsafe extern "system" fn GetTraceDataProvidersByProcess<Impl: ITraceDataProviderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, server: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetTraceDataProvidersByProcess(::core::mem::transmute_copy(&server), ::core::mem::transmute_copy(&pid)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Count: Count::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Add: Add::<Impl, IMPL_OFFSET>,
            Remove: Remove::<Impl, IMPL_OFFSET>,
            Clear: Clear::<Impl, IMPL_OFFSET>,
            AddRange: AddRange::<Impl, IMPL_OFFSET>,
            CreateTraceDataProvider: CreateTraceDataProvider::<Impl, IMPL_OFFSET>,
            GetTraceDataProviders: GetTraceDataProviders::<Impl, IMPL_OFFSET>,
            GetTraceDataProvidersByProcess: GetTraceDataProvidersByProcess::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITraceDataProviderCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IValueMap_Impl: Sized + super::Com::IDispatch_Impl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn Item(&mut self, index: &super::Com::VARIANT) -> ::windows::core::Result<IValueMapItem>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Description(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDescription(&mut self, description: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Value(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetValue(&mut self, value: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn ValueMapType(&mut self) -> ::windows::core::Result<ValueMapType>;
    fn SetValueMapType(&mut self, r#type: ValueMapType) -> ::windows::core::Result<()>;
    fn Add(&mut self, value: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Remove(&mut self, value: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Clear(&mut self) -> ::windows::core::Result<()>;
    fn AddRange(&mut self, map: &::core::option::Option<IValueMap>) -> ::windows::core::Result<()>;
    fn CreateValueMapItem(&mut self) -> ::windows::core::Result<IValueMapItem>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IValueMap_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IValueMap_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IValueMap_Vtbl {
        unsafe extern "system" fn Count<Impl: IValueMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IValueMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IValueMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Impl: IValueMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, description: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *description = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Impl: IValueMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, description: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDescription(::core::mem::transmute_copy(&description)).into()
        }
        unsafe extern "system" fn Value<Impl: IValueMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Impl: IValueMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValue(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn ValueMapType<Impl: IValueMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: *mut ValueMapType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ValueMapType() {
                ::core::result::Result::Ok(ok__) => {
                    *r#type = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValueMapType<Impl: IValueMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: ValueMapType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValueMapType(::core::mem::transmute_copy(&r#type)).into()
        }
        unsafe extern "system" fn Add<Impl: IValueMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Add(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn Remove<Impl: IValueMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Remove(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn Clear<Impl: IValueMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Clear().into()
        }
        unsafe extern "system" fn AddRange<Impl: IValueMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, map: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddRange(::core::mem::transmute(&map)).into()
        }
        unsafe extern "system" fn CreateValueMapItem<Impl: IValueMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateValueMapItem() {
                ::core::result::Result::Ok(ok__) => {
                    *item = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Count: Count::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Description: Description::<Impl, IMPL_OFFSET>,
            SetDescription: SetDescription::<Impl, IMPL_OFFSET>,
            Value: Value::<Impl, IMPL_OFFSET>,
            SetValue: SetValue::<Impl, IMPL_OFFSET>,
            ValueMapType: ValueMapType::<Impl, IMPL_OFFSET>,
            SetValueMapType: SetValueMapType::<Impl, IMPL_OFFSET>,
            Add: Add::<Impl, IMPL_OFFSET>,
            Remove: Remove::<Impl, IMPL_OFFSET>,
            Clear: Clear::<Impl, IMPL_OFFSET>,
            AddRange: AddRange::<Impl, IMPL_OFFSET>,
            CreateValueMapItem: CreateValueMapItem::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IValueMap as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IValueMapItem_Impl: Sized + super::Com::IDispatch_Impl {
    fn Description(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDescription(&mut self, description: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Enabled(&mut self) -> ::windows::core::Result<i16>;
    fn SetEnabled(&mut self, enabled: i16) -> ::windows::core::Result<()>;
    fn Key(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetKey(&mut self, key: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Value(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetValue(&mut self, value: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn ValueMapType(&mut self) -> ::windows::core::Result<ValueMapType>;
    fn SetValueMapType(&mut self, r#type: ValueMapType) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IValueMapItem_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IValueMapItem_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IValueMapItem_Vtbl {
        unsafe extern "system" fn Description<Impl: IValueMapItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, description: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *description = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Impl: IValueMapItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, description: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDescription(::core::mem::transmute_copy(&description)).into()
        }
        unsafe extern "system" fn Enabled<Impl: IValueMapItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Enabled() {
                ::core::result::Result::Ok(ok__) => {
                    *enabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Impl: IValueMapItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnabled(::core::mem::transmute_copy(&enabled)).into()
        }
        unsafe extern "system" fn Key<Impl: IValueMapItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Key() {
                ::core::result::Result::Ok(ok__) => {
                    *key = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKey<Impl: IValueMapItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetKey(::core::mem::transmute_copy(&key)).into()
        }
        unsafe extern "system" fn Value<Impl: IValueMapItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Impl: IValueMapItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValue(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn ValueMapType<Impl: IValueMapItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: *mut ValueMapType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ValueMapType() {
                ::core::result::Result::Ok(ok__) => {
                    *r#type = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValueMapType<Impl: IValueMapItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: ValueMapType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValueMapType(::core::mem::transmute_copy(&r#type)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Description: Description::<Impl, IMPL_OFFSET>,
            SetDescription: SetDescription::<Impl, IMPL_OFFSET>,
            Enabled: Enabled::<Impl, IMPL_OFFSET>,
            SetEnabled: SetEnabled::<Impl, IMPL_OFFSET>,
            Key: Key::<Impl, IMPL_OFFSET>,
            SetKey: SetKey::<Impl, IMPL_OFFSET>,
            Value: Value::<Impl, IMPL_OFFSET>,
            SetValue: SetValue::<Impl, IMPL_OFFSET>,
            ValueMapType: ValueMapType::<Impl, IMPL_OFFSET>,
            SetValueMapType: SetValueMapType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IValueMapItem as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait _ICounterItemUnion_Impl: Sized {
    fn Value(&mut self) -> ::windows::core::Result<f64>;
    fn SetColor(&mut self, color: u32) -> ::windows::core::Result<()>;
    fn Color(&mut self) -> ::windows::core::Result<u32>;
    fn SetWidth(&mut self, iwidth: i32) -> ::windows::core::Result<()>;
    fn Width(&mut self) -> ::windows::core::Result<i32>;
    fn SetLineStyle(&mut self, ilinestyle: i32) -> ::windows::core::Result<()>;
    fn LineStyle(&mut self) -> ::windows::core::Result<i32>;
    fn SetScaleFactor(&mut self, iscale: i32) -> ::windows::core::Result<()>;
    fn ScaleFactor(&mut self) -> ::windows::core::Result<i32>;
    fn Path(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetValue(&mut self, value: *mut f64, status: *mut i32) -> ::windows::core::Result<()>;
    fn GetStatistics(&mut self, max: *mut f64, min: *mut f64, avg: *mut f64, status: *mut i32) -> ::windows::core::Result<()>;
    fn SetSelected(&mut self, bstate: i16) -> ::windows::core::Result<()>;
    fn Selected(&mut self) -> ::windows::core::Result<i16>;
    fn SetVisible(&mut self, bstate: i16) -> ::windows::core::Result<()>;
    fn Visible(&mut self) -> ::windows::core::Result<i16>;
    fn GetDataAt(&mut self, iindex: i32, iwhich: SysmonDataType) -> ::windows::core::Result<super::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl _ICounterItemUnion_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: _ICounterItemUnion_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> _ICounterItemUnion_Vtbl {
        unsafe extern "system" fn Value<Impl: _ICounterItemUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdblvalue: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *pdblvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColor<Impl: _ICounterItemUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, color: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetColor(::core::mem::transmute_copy(&color)).into()
        }
        unsafe extern "system" fn Color<Impl: _ICounterItemUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcolor: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Color() {
                ::core::result::Result::Ok(ok__) => {
                    *pcolor = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWidth<Impl: _ICounterItemUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iwidth: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWidth(::core::mem::transmute_copy(&iwidth)).into()
        }
        unsafe extern "system" fn Width<Impl: _ICounterItemUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pivalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Width() {
                ::core::result::Result::Ok(ok__) => {
                    *pivalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLineStyle<Impl: _ICounterItemUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ilinestyle: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLineStyle(::core::mem::transmute_copy(&ilinestyle)).into()
        }
        unsafe extern "system" fn LineStyle<Impl: _ICounterItemUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pivalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LineStyle() {
                ::core::result::Result::Ok(ok__) => {
                    *pivalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScaleFactor<Impl: _ICounterItemUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iscale: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScaleFactor(::core::mem::transmute_copy(&iscale)).into()
        }
        unsafe extern "system" fn ScaleFactor<Impl: _ICounterItemUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pivalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScaleFactor() {
                ::core::result::Result::Ok(ok__) => {
                    *pivalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Path<Impl: _ICounterItemUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstrvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Path() {
                ::core::result::Result::Ok(ok__) => {
                    *pstrvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValue<Impl: _ICounterItemUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut f64, status: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetValue(::core::mem::transmute_copy(&value), ::core::mem::transmute_copy(&status)).into()
        }
        unsafe extern "system" fn GetStatistics<Impl: _ICounterItemUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, max: *mut f64, min: *mut f64, avg: *mut f64, status: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetStatistics(::core::mem::transmute_copy(&max), ::core::mem::transmute_copy(&min), ::core::mem::transmute_copy(&avg), ::core::mem::transmute_copy(&status)).into()
        }
        unsafe extern "system" fn SetSelected<Impl: _ICounterItemUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelected(::core::mem::transmute_copy(&bstate)).into()
        }
        unsafe extern "system" fn Selected<Impl: _ICounterItemUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Selected() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVisible<Impl: _ICounterItemUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVisible(::core::mem::transmute_copy(&bstate)).into()
        }
        unsafe extern "system" fn Visible<Impl: _ICounterItemUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Visible() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDataAt<Impl: _ICounterItemUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iindex: i32, iwhich: SysmonDataType, pvariant: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDataAt(::core::mem::transmute_copy(&iindex), ::core::mem::transmute_copy(&iwhich)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Value: Value::<Impl, IMPL_OFFSET>,
            SetColor: SetColor::<Impl, IMPL_OFFSET>,
            Color: Color::<Impl, IMPL_OFFSET>,
            SetWidth: SetWidth::<Impl, IMPL_OFFSET>,
            Width: Width::<Impl, IMPL_OFFSET>,
            SetLineStyle: SetLineStyle::<Impl, IMPL_OFFSET>,
            LineStyle: LineStyle::<Impl, IMPL_OFFSET>,
            SetScaleFactor: SetScaleFactor::<Impl, IMPL_OFFSET>,
            ScaleFactor: ScaleFactor::<Impl, IMPL_OFFSET>,
            Path: Path::<Impl, IMPL_OFFSET>,
            GetValue: GetValue::<Impl, IMPL_OFFSET>,
            GetStatistics: GetStatistics::<Impl, IMPL_OFFSET>,
            SetSelected: SetSelected::<Impl, IMPL_OFFSET>,
            Selected: Selected::<Impl, IMPL_OFFSET>,
            SetVisible: SetVisible::<Impl, IMPL_OFFSET>,
            Visible: Visible::<Impl, IMPL_OFFSET>,
            GetDataAt: GetDataAt::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<_ICounterItemUnion as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait _ISystemMonitorUnion_Impl: Sized {
    fn Appearance(&mut self) -> ::windows::core::Result<i32>;
    fn SetAppearance(&mut self, iappearance: i32) -> ::windows::core::Result<()>;
    fn BackColor(&mut self) -> ::windows::core::Result<u32>;
    fn SetBackColor(&mut self, color: u32) -> ::windows::core::Result<()>;
    fn BorderStyle(&mut self) -> ::windows::core::Result<i32>;
    fn SetBorderStyle(&mut self, iborderstyle: i32) -> ::windows::core::Result<()>;
    fn ForeColor(&mut self) -> ::windows::core::Result<u32>;
    fn SetForeColor(&mut self, color: u32) -> ::windows::core::Result<()>;
    fn Font(&mut self) -> ::windows::core::Result<super::Ole::IFontDisp>;
    fn putref_Font(&mut self, pfont: &::core::option::Option<super::Ole::IFontDisp>) -> ::windows::core::Result<()>;
    fn Counters(&mut self) -> ::windows::core::Result<ICounters>;
    fn SetShowVerticalGrid(&mut self, bstate: i16) -> ::windows::core::Result<()>;
    fn ShowVerticalGrid(&mut self) -> ::windows::core::Result<i16>;
    fn SetShowHorizontalGrid(&mut self, bstate: i16) -> ::windows::core::Result<()>;
    fn ShowHorizontalGrid(&mut self) -> ::windows::core::Result<i16>;
    fn SetShowLegend(&mut self, bstate: i16) -> ::windows::core::Result<()>;
    fn ShowLegend(&mut self) -> ::windows::core::Result<i16>;
    fn SetShowScaleLabels(&mut self, bstate: i16) -> ::windows::core::Result<()>;
    fn ShowScaleLabels(&mut self) -> ::windows::core::Result<i16>;
    fn SetShowValueBar(&mut self, bstate: i16) -> ::windows::core::Result<()>;
    fn ShowValueBar(&mut self) -> ::windows::core::Result<i16>;
    fn SetMaximumScale(&mut self, ivalue: i32) -> ::windows::core::Result<()>;
    fn MaximumScale(&mut self) -> ::windows::core::Result<i32>;
    fn SetMinimumScale(&mut self, ivalue: i32) -> ::windows::core::Result<()>;
    fn MinimumScale(&mut self) -> ::windows::core::Result<i32>;
    fn SetUpdateInterval(&mut self, fvalue: f32) -> ::windows::core::Result<()>;
    fn UpdateInterval(&mut self) -> ::windows::core::Result<f32>;
    fn SetDisplayType(&mut self, edisplaytype: DisplayTypeConstants) -> ::windows::core::Result<()>;
    fn DisplayType(&mut self) -> ::windows::core::Result<DisplayTypeConstants>;
    fn SetManualUpdate(&mut self, bstate: i16) -> ::windows::core::Result<()>;
    fn ManualUpdate(&mut self) -> ::windows::core::Result<i16>;
    fn SetGraphTitle(&mut self, bstitle: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GraphTitle(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetYAxisLabel(&mut self, bstitle: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn YAxisLabel(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CollectSample(&mut self) -> ::windows::core::Result<()>;
    fn UpdateGraph(&mut self) -> ::windows::core::Result<()>;
    fn BrowseCounters(&mut self) -> ::windows::core::Result<()>;
    fn DisplayProperties(&mut self) -> ::windows::core::Result<()>;
    fn Counter(&mut self, iindex: i32) -> ::windows::core::Result<ICounterItem>;
    fn AddCounter(&mut self, bspath: &super::super::Foundation::BSTR) -> ::windows::core::Result<ICounterItem>;
    fn DeleteCounter(&mut self, pctr: &::core::option::Option<ICounterItem>) -> ::windows::core::Result<()>;
    fn BackColorCtl(&mut self) -> ::windows::core::Result<u32>;
    fn SetBackColorCtl(&mut self, color: u32) -> ::windows::core::Result<()>;
    fn SetLogFileName(&mut self, bsfilename: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn LogFileName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetLogViewStart(&mut self, starttime: f64) -> ::windows::core::Result<()>;
    fn LogViewStart(&mut self) -> ::windows::core::Result<f64>;
    fn SetLogViewStop(&mut self, stoptime: f64) -> ::windows::core::Result<()>;
    fn LogViewStop(&mut self) -> ::windows::core::Result<f64>;
    fn GridColor(&mut self) -> ::windows::core::Result<u32>;
    fn SetGridColor(&mut self, color: u32) -> ::windows::core::Result<()>;
    fn TimeBarColor(&mut self) -> ::windows::core::Result<u32>;
    fn SetTimeBarColor(&mut self, color: u32) -> ::windows::core::Result<()>;
    fn Highlight(&mut self) -> ::windows::core::Result<i16>;
    fn SetHighlight(&mut self, bstate: i16) -> ::windows::core::Result<()>;
    fn ShowToolbar(&mut self) -> ::windows::core::Result<i16>;
    fn SetShowToolbar(&mut self, bstate: i16) -> ::windows::core::Result<()>;
    fn Paste(&mut self) -> ::windows::core::Result<()>;
    fn Copy(&mut self) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn SetReadOnly(&mut self, bstate: i16) -> ::windows::core::Result<()>;
    fn ReadOnly(&mut self) -> ::windows::core::Result<i16>;
    fn SetReportValueType(&mut self, ereportvaluetype: ReportValueTypeConstants) -> ::windows::core::Result<()>;
    fn ReportValueType(&mut self) -> ::windows::core::Result<ReportValueTypeConstants>;
    fn SetMonitorDuplicateInstances(&mut self, bstate: i16) -> ::windows::core::Result<()>;
    fn MonitorDuplicateInstances(&mut self) -> ::windows::core::Result<i16>;
    fn SetDisplayFilter(&mut self, ivalue: i32) -> ::windows::core::Result<()>;
    fn DisplayFilter(&mut self) -> ::windows::core::Result<i32>;
    fn LogFiles(&mut self) -> ::windows::core::Result<ILogFiles>;
    fn SetDataSourceType(&mut self, edatasourcetype: DataSourceTypeConstants) -> ::windows::core::Result<()>;
    fn DataSourceType(&mut self) -> ::windows::core::Result<DataSourceTypeConstants>;
    fn SetSqlDsnName(&mut self, bssqldsnname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SqlDsnName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetSqlLogSetName(&mut self, bssqllogsetname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SqlLogSetName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetEnableDigitGrouping(&mut self, bstate: i16) -> ::windows::core::Result<()>;
    fn EnableDigitGrouping(&mut self) -> ::windows::core::Result<i16>;
    fn SetEnableToolTips(&mut self, bstate: i16) -> ::windows::core::Result<()>;
    fn EnableToolTips(&mut self) -> ::windows::core::Result<i16>;
    fn SetShowTimeAxisLabels(&mut self, bstate: i16) -> ::windows::core::Result<()>;
    fn ShowTimeAxisLabels(&mut self) -> ::windows::core::Result<i16>;
    fn SetChartScroll(&mut self, bscroll: i16) -> ::windows::core::Result<()>;
    fn ChartScroll(&mut self) -> ::windows::core::Result<i16>;
    fn SetDataPointCount(&mut self, inewcount: i32) -> ::windows::core::Result<()>;
    fn DataPointCount(&mut self) -> ::windows::core::Result<i32>;
    fn ScaleToFit(&mut self, bselectedcountersonly: i16) -> ::windows::core::Result<()>;
    fn SaveAs(&mut self, bstrfilename: &super::super::Foundation::BSTR, esysmonfiletype: SysmonFileType) -> ::windows::core::Result<()>;
    fn Relog(&mut self, bstrfilename: &super::super::Foundation::BSTR, esysmonfiletype: SysmonFileType, ifilter: i32) -> ::windows::core::Result<()>;
    fn ClearData(&mut self) -> ::windows::core::Result<()>;
    fn LogSourceStartTime(&mut self) -> ::windows::core::Result<f64>;
    fn LogSourceStopTime(&mut self) -> ::windows::core::Result<f64>;
    fn SetLogViewRange(&mut self, starttime: f64, stoptime: f64) -> ::windows::core::Result<()>;
    fn GetLogViewRange(&mut self, starttime: *mut f64, stoptime: *mut f64) -> ::windows::core::Result<()>;
    fn BatchingLock(&mut self, flock: i16, ebatchreason: SysmonBatchReason) -> ::windows::core::Result<()>;
    fn LoadSettings(&mut self, bstrsettingfilename: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl _ISystemMonitorUnion_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> _ISystemMonitorUnion_Vtbl {
        unsafe extern "system" fn Appearance<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iappearance: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Appearance() {
                ::core::result::Result::Ok(ok__) => {
                    *iappearance = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAppearance<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iappearance: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAppearance(::core::mem::transmute_copy(&iappearance)).into()
        }
        unsafe extern "system" fn BackColor<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcolor: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BackColor() {
                ::core::result::Result::Ok(ok__) => {
                    *pcolor = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBackColor<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, color: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBackColor(::core::mem::transmute_copy(&color)).into()
        }
        unsafe extern "system" fn BorderStyle<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iborderstyle: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BorderStyle() {
                ::core::result::Result::Ok(ok__) => {
                    *iborderstyle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBorderStyle<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iborderstyle: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBorderStyle(::core::mem::transmute_copy(&iborderstyle)).into()
        }
        unsafe extern "system" fn ForeColor<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcolor: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ForeColor() {
                ::core::result::Result::Ok(ok__) => {
                    *pcolor = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetForeColor<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, color: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetForeColor(::core::mem::transmute_copy(&color)).into()
        }
        unsafe extern "system" fn Font<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfont: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Font() {
                ::core::result::Result::Ok(ok__) => {
                    *ppfont = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_Font<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfont: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).putref_Font(::core::mem::transmute(&pfont)).into()
        }
        unsafe extern "system" fn Counters<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppicounters: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Counters() {
                ::core::result::Result::Ok(ok__) => {
                    *ppicounters = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShowVerticalGrid<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetShowVerticalGrid(::core::mem::transmute_copy(&bstate)).into()
        }
        unsafe extern "system" fn ShowVerticalGrid<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowVerticalGrid() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShowHorizontalGrid<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetShowHorizontalGrid(::core::mem::transmute_copy(&bstate)).into()
        }
        unsafe extern "system" fn ShowHorizontalGrid<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowHorizontalGrid() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShowLegend<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetShowLegend(::core::mem::transmute_copy(&bstate)).into()
        }
        unsafe extern "system" fn ShowLegend<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowLegend() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShowScaleLabels<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetShowScaleLabels(::core::mem::transmute_copy(&bstate)).into()
        }
        unsafe extern "system" fn ShowScaleLabels<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowScaleLabels() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShowValueBar<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetShowValueBar(::core::mem::transmute_copy(&bstate)).into()
        }
        unsafe extern "system" fn ShowValueBar<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowValueBar() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaximumScale<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ivalue: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaximumScale(::core::mem::transmute_copy(&ivalue)).into()
        }
        unsafe extern "system" fn MaximumScale<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pivalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaximumScale() {
                ::core::result::Result::Ok(ok__) => {
                    *pivalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinimumScale<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ivalue: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMinimumScale(::core::mem::transmute_copy(&ivalue)).into()
        }
        unsafe extern "system" fn MinimumScale<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pivalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinimumScale() {
                ::core::result::Result::Ok(ok__) => {
                    *pivalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUpdateInterval<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fvalue: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUpdateInterval(::core::mem::transmute_copy(&fvalue)).into()
        }
        unsafe extern "system" fn UpdateInterval<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfvalue: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpdateInterval() {
                ::core::result::Result::Ok(ok__) => {
                    *pfvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayType<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, edisplaytype: DisplayTypeConstants) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisplayType(::core::mem::transmute_copy(&edisplaytype)).into()
        }
        unsafe extern "system" fn DisplayType<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pedisplaytype: *mut DisplayTypeConstants) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayType() {
                ::core::result::Result::Ok(ok__) => {
                    *pedisplaytype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetManualUpdate<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetManualUpdate(::core::mem::transmute_copy(&bstate)).into()
        }
        unsafe extern "system" fn ManualUpdate<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ManualUpdate() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGraphTitle<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstitle: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGraphTitle(::core::mem::transmute_copy(&bstitle)).into()
        }
        unsafe extern "system" fn GraphTitle<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstitle: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GraphTitle() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstitle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetYAxisLabel<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstitle: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetYAxisLabel(::core::mem::transmute_copy(&bstitle)).into()
        }
        unsafe extern "system" fn YAxisLabel<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstitle: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).YAxisLabel() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstitle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CollectSample<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CollectSample().into()
        }
        unsafe extern "system" fn UpdateGraph<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpdateGraph().into()
        }
        unsafe extern "system" fn BrowseCounters<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BrowseCounters().into()
        }
        unsafe extern "system" fn DisplayProperties<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DisplayProperties().into()
        }
        unsafe extern "system" fn Counter<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iindex: i32, ppicounter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Counter(::core::mem::transmute_copy(&iindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppicounter = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddCounter<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bspath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppicounter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddCounter(::core::mem::transmute_copy(&bspath)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppicounter = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteCounter<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pctr: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteCounter(::core::mem::transmute(&pctr)).into()
        }
        unsafe extern "system" fn BackColorCtl<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcolor: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BackColorCtl() {
                ::core::result::Result::Ok(ok__) => {
                    *pcolor = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBackColorCtl<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, color: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBackColorCtl(::core::mem::transmute_copy(&color)).into()
        }
        unsafe extern "system" fn SetLogFileName<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bsfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLogFileName(::core::mem::transmute_copy(&bsfilename)).into()
        }
        unsafe extern "system" fn LogFileName<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bsfilename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LogFileName() {
                ::core::result::Result::Ok(ok__) => {
                    *bsfilename = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLogViewStart<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, starttime: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLogViewStart(::core::mem::transmute_copy(&starttime)).into()
        }
        unsafe extern "system" fn LogViewStart<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, starttime: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LogViewStart() {
                ::core::result::Result::Ok(ok__) => {
                    *starttime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLogViewStop<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stoptime: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLogViewStop(::core::mem::transmute_copy(&stoptime)).into()
        }
        unsafe extern "system" fn LogViewStop<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stoptime: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LogViewStop() {
                ::core::result::Result::Ok(ok__) => {
                    *stoptime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GridColor<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcolor: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GridColor() {
                ::core::result::Result::Ok(ok__) => {
                    *pcolor = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGridColor<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, color: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGridColor(::core::mem::transmute_copy(&color)).into()
        }
        unsafe extern "system" fn TimeBarColor<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcolor: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TimeBarColor() {
                ::core::result::Result::Ok(ok__) => {
                    *pcolor = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTimeBarColor<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, color: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTimeBarColor(::core::mem::transmute_copy(&color)).into()
        }
        unsafe extern "system" fn Highlight<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Highlight() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHighlight<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHighlight(::core::mem::transmute_copy(&bstate)).into()
        }
        unsafe extern "system" fn ShowToolbar<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowToolbar() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShowToolbar<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetShowToolbar(::core::mem::transmute_copy(&bstate)).into()
        }
        unsafe extern "system" fn Paste<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Paste().into()
        }
        unsafe extern "system" fn Copy<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Copy().into()
        }
        unsafe extern "system" fn Reset<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn SetReadOnly<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReadOnly(::core::mem::transmute_copy(&bstate)).into()
        }
        unsafe extern "system" fn ReadOnly<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadOnly() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReportValueType<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ereportvaluetype: ReportValueTypeConstants) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReportValueType(::core::mem::transmute_copy(&ereportvaluetype)).into()
        }
        unsafe extern "system" fn ReportValueType<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pereportvaluetype: *mut ReportValueTypeConstants) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportValueType() {
                ::core::result::Result::Ok(ok__) => {
                    *pereportvaluetype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMonitorDuplicateInstances<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMonitorDuplicateInstances(::core::mem::transmute_copy(&bstate)).into()
        }
        unsafe extern "system" fn MonitorDuplicateInstances<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MonitorDuplicateInstances() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayFilter<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ivalue: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisplayFilter(::core::mem::transmute_copy(&ivalue)).into()
        }
        unsafe extern "system" fn DisplayFilter<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pivalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayFilter() {
                ::core::result::Result::Ok(ok__) => {
                    *pivalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LogFiles<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppilogfiles: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LogFiles() {
                ::core::result::Result::Ok(ok__) => {
                    *ppilogfiles = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDataSourceType<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, edatasourcetype: DataSourceTypeConstants) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDataSourceType(::core::mem::transmute_copy(&edatasourcetype)).into()
        }
        unsafe extern "system" fn DataSourceType<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pedatasourcetype: *mut DataSourceTypeConstants) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DataSourceType() {
                ::core::result::Result::Ok(ok__) => {
                    *pedatasourcetype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSqlDsnName<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bssqldsnname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSqlDsnName(::core::mem::transmute_copy(&bssqldsnname)).into()
        }
        unsafe extern "system" fn SqlDsnName<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bssqldsnname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SqlDsnName() {
                ::core::result::Result::Ok(ok__) => {
                    *bssqldsnname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSqlLogSetName<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bssqllogsetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSqlLogSetName(::core::mem::transmute_copy(&bssqllogsetname)).into()
        }
        unsafe extern "system" fn SqlLogSetName<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bssqllogsetname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SqlLogSetName() {
                ::core::result::Result::Ok(ok__) => {
                    *bssqllogsetname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnableDigitGrouping<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnableDigitGrouping(::core::mem::transmute_copy(&bstate)).into()
        }
        unsafe extern "system" fn EnableDigitGrouping<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnableDigitGrouping() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnableToolTips<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnableToolTips(::core::mem::transmute_copy(&bstate)).into()
        }
        unsafe extern "system" fn EnableToolTips<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnableToolTips() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShowTimeAxisLabels<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetShowTimeAxisLabels(::core::mem::transmute_copy(&bstate)).into()
        }
        unsafe extern "system" fn ShowTimeAxisLabels<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowTimeAxisLabels() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChartScroll<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bscroll: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetChartScroll(::core::mem::transmute_copy(&bscroll)).into()
        }
        unsafe extern "system" fn ChartScroll<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbscroll: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ChartScroll() {
                ::core::result::Result::Ok(ok__) => {
                    *pbscroll = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDataPointCount<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inewcount: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDataPointCount(::core::mem::transmute_copy(&inewcount)).into()
        }
        unsafe extern "system" fn DataPointCount<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidatapointcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DataPointCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pidatapointcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScaleToFit<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bselectedcountersonly: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ScaleToFit(::core::mem::transmute_copy(&bselectedcountersonly)).into()
        }
        unsafe extern "system" fn SaveAs<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, esysmonfiletype: SysmonFileType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SaveAs(::core::mem::transmute_copy(&bstrfilename), ::core::mem::transmute_copy(&esysmonfiletype)).into()
        }
        unsafe extern "system" fn Relog<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, esysmonfiletype: SysmonFileType, ifilter: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Relog(::core::mem::transmute_copy(&bstrfilename), ::core::mem::transmute_copy(&esysmonfiletype), ::core::mem::transmute_copy(&ifilter)).into()
        }
        unsafe extern "system" fn ClearData<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearData().into()
        }
        unsafe extern "system" fn LogSourceStartTime<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdate: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LogSourceStartTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pdate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LogSourceStopTime<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdate: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LogSourceStopTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pdate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLogViewRange<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, starttime: f64, stoptime: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLogViewRange(::core::mem::transmute_copy(&starttime), ::core::mem::transmute_copy(&stoptime)).into()
        }
        unsafe extern "system" fn GetLogViewRange<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, starttime: *mut f64, stoptime: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetLogViewRange(::core::mem::transmute_copy(&starttime), ::core::mem::transmute_copy(&stoptime)).into()
        }
        unsafe extern "system" fn BatchingLock<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flock: i16, ebatchreason: SysmonBatchReason) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BatchingLock(::core::mem::transmute_copy(&flock), ::core::mem::transmute_copy(&ebatchreason)).into()
        }
        unsafe extern "system" fn LoadSettings<Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsettingfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LoadSettings(::core::mem::transmute_copy(&bstrsettingfilename)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Appearance: Appearance::<Impl, IMPL_OFFSET>,
            SetAppearance: SetAppearance::<Impl, IMPL_OFFSET>,
            BackColor: BackColor::<Impl, IMPL_OFFSET>,
            SetBackColor: SetBackColor::<Impl, IMPL_OFFSET>,
            BorderStyle: BorderStyle::<Impl, IMPL_OFFSET>,
            SetBorderStyle: SetBorderStyle::<Impl, IMPL_OFFSET>,
            ForeColor: ForeColor::<Impl, IMPL_OFFSET>,
            SetForeColor: SetForeColor::<Impl, IMPL_OFFSET>,
            Font: Font::<Impl, IMPL_OFFSET>,
            putref_Font: putref_Font::<Impl, IMPL_OFFSET>,
            Counters: Counters::<Impl, IMPL_OFFSET>,
            SetShowVerticalGrid: SetShowVerticalGrid::<Impl, IMPL_OFFSET>,
            ShowVerticalGrid: ShowVerticalGrid::<Impl, IMPL_OFFSET>,
            SetShowHorizontalGrid: SetShowHorizontalGrid::<Impl, IMPL_OFFSET>,
            ShowHorizontalGrid: ShowHorizontalGrid::<Impl, IMPL_OFFSET>,
            SetShowLegend: SetShowLegend::<Impl, IMPL_OFFSET>,
            ShowLegend: ShowLegend::<Impl, IMPL_OFFSET>,
            SetShowScaleLabels: SetShowScaleLabels::<Impl, IMPL_OFFSET>,
            ShowScaleLabels: ShowScaleLabels::<Impl, IMPL_OFFSET>,
            SetShowValueBar: SetShowValueBar::<Impl, IMPL_OFFSET>,
            ShowValueBar: ShowValueBar::<Impl, IMPL_OFFSET>,
            SetMaximumScale: SetMaximumScale::<Impl, IMPL_OFFSET>,
            MaximumScale: MaximumScale::<Impl, IMPL_OFFSET>,
            SetMinimumScale: SetMinimumScale::<Impl, IMPL_OFFSET>,
            MinimumScale: MinimumScale::<Impl, IMPL_OFFSET>,
            SetUpdateInterval: SetUpdateInterval::<Impl, IMPL_OFFSET>,
            UpdateInterval: UpdateInterval::<Impl, IMPL_OFFSET>,
            SetDisplayType: SetDisplayType::<Impl, IMPL_OFFSET>,
            DisplayType: DisplayType::<Impl, IMPL_OFFSET>,
            SetManualUpdate: SetManualUpdate::<Impl, IMPL_OFFSET>,
            ManualUpdate: ManualUpdate::<Impl, IMPL_OFFSET>,
            SetGraphTitle: SetGraphTitle::<Impl, IMPL_OFFSET>,
            GraphTitle: GraphTitle::<Impl, IMPL_OFFSET>,
            SetYAxisLabel: SetYAxisLabel::<Impl, IMPL_OFFSET>,
            YAxisLabel: YAxisLabel::<Impl, IMPL_OFFSET>,
            CollectSample: CollectSample::<Impl, IMPL_OFFSET>,
            UpdateGraph: UpdateGraph::<Impl, IMPL_OFFSET>,
            BrowseCounters: BrowseCounters::<Impl, IMPL_OFFSET>,
            DisplayProperties: DisplayProperties::<Impl, IMPL_OFFSET>,
            Counter: Counter::<Impl, IMPL_OFFSET>,
            AddCounter: AddCounter::<Impl, IMPL_OFFSET>,
            DeleteCounter: DeleteCounter::<Impl, IMPL_OFFSET>,
            BackColorCtl: BackColorCtl::<Impl, IMPL_OFFSET>,
            SetBackColorCtl: SetBackColorCtl::<Impl, IMPL_OFFSET>,
            SetLogFileName: SetLogFileName::<Impl, IMPL_OFFSET>,
            LogFileName: LogFileName::<Impl, IMPL_OFFSET>,
            SetLogViewStart: SetLogViewStart::<Impl, IMPL_OFFSET>,
            LogViewStart: LogViewStart::<Impl, IMPL_OFFSET>,
            SetLogViewStop: SetLogViewStop::<Impl, IMPL_OFFSET>,
            LogViewStop: LogViewStop::<Impl, IMPL_OFFSET>,
            GridColor: GridColor::<Impl, IMPL_OFFSET>,
            SetGridColor: SetGridColor::<Impl, IMPL_OFFSET>,
            TimeBarColor: TimeBarColor::<Impl, IMPL_OFFSET>,
            SetTimeBarColor: SetTimeBarColor::<Impl, IMPL_OFFSET>,
            Highlight: Highlight::<Impl, IMPL_OFFSET>,
            SetHighlight: SetHighlight::<Impl, IMPL_OFFSET>,
            ShowToolbar: ShowToolbar::<Impl, IMPL_OFFSET>,
            SetShowToolbar: SetShowToolbar::<Impl, IMPL_OFFSET>,
            Paste: Paste::<Impl, IMPL_OFFSET>,
            Copy: Copy::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            SetReadOnly: SetReadOnly::<Impl, IMPL_OFFSET>,
            ReadOnly: ReadOnly::<Impl, IMPL_OFFSET>,
            SetReportValueType: SetReportValueType::<Impl, IMPL_OFFSET>,
            ReportValueType: ReportValueType::<Impl, IMPL_OFFSET>,
            SetMonitorDuplicateInstances: SetMonitorDuplicateInstances::<Impl, IMPL_OFFSET>,
            MonitorDuplicateInstances: MonitorDuplicateInstances::<Impl, IMPL_OFFSET>,
            SetDisplayFilter: SetDisplayFilter::<Impl, IMPL_OFFSET>,
            DisplayFilter: DisplayFilter::<Impl, IMPL_OFFSET>,
            LogFiles: LogFiles::<Impl, IMPL_OFFSET>,
            SetDataSourceType: SetDataSourceType::<Impl, IMPL_OFFSET>,
            DataSourceType: DataSourceType::<Impl, IMPL_OFFSET>,
            SetSqlDsnName: SetSqlDsnName::<Impl, IMPL_OFFSET>,
            SqlDsnName: SqlDsnName::<Impl, IMPL_OFFSET>,
            SetSqlLogSetName: SetSqlLogSetName::<Impl, IMPL_OFFSET>,
            SqlLogSetName: SqlLogSetName::<Impl, IMPL_OFFSET>,
            SetEnableDigitGrouping: SetEnableDigitGrouping::<Impl, IMPL_OFFSET>,
            EnableDigitGrouping: EnableDigitGrouping::<Impl, IMPL_OFFSET>,
            SetEnableToolTips: SetEnableToolTips::<Impl, IMPL_OFFSET>,
            EnableToolTips: EnableToolTips::<Impl, IMPL_OFFSET>,
            SetShowTimeAxisLabels: SetShowTimeAxisLabels::<Impl, IMPL_OFFSET>,
            ShowTimeAxisLabels: ShowTimeAxisLabels::<Impl, IMPL_OFFSET>,
            SetChartScroll: SetChartScroll::<Impl, IMPL_OFFSET>,
            ChartScroll: ChartScroll::<Impl, IMPL_OFFSET>,
            SetDataPointCount: SetDataPointCount::<Impl, IMPL_OFFSET>,
            DataPointCount: DataPointCount::<Impl, IMPL_OFFSET>,
            ScaleToFit: ScaleToFit::<Impl, IMPL_OFFSET>,
            SaveAs: SaveAs::<Impl, IMPL_OFFSET>,
            Relog: Relog::<Impl, IMPL_OFFSET>,
            ClearData: ClearData::<Impl, IMPL_OFFSET>,
            LogSourceStartTime: LogSourceStartTime::<Impl, IMPL_OFFSET>,
            LogSourceStopTime: LogSourceStopTime::<Impl, IMPL_OFFSET>,
            SetLogViewRange: SetLogViewRange::<Impl, IMPL_OFFSET>,
            GetLogViewRange: GetLogViewRange::<Impl, IMPL_OFFSET>,
            BatchingLock: BatchingLock::<Impl, IMPL_OFFSET>,
            LoadSettings: LoadSettings::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<_ISystemMonitorUnion as ::windows::core::Interface>::IID
    }
}
