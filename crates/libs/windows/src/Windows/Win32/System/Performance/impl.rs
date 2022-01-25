#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait DICounterItem_Impl: Sized + super::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl DICounterItem_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: DICounterItem_Impl, const OFFSET: isize>() -> DICounterItem_Vtbl {
        Self { base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<DICounterItem as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait DILogFileItem_Impl: Sized + super::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl DILogFileItem_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: DILogFileItem_Impl, const OFFSET: isize>() -> DILogFileItem_Vtbl {
        Self { base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<DILogFileItem as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait DISystemMonitor_Impl: Sized + super::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl DISystemMonitor_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: DISystemMonitor_Impl, const OFFSET: isize>() -> DISystemMonitor_Vtbl {
        Self { base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<DISystemMonitor as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait DISystemMonitorEvents_Impl: Sized + super::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl DISystemMonitorEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: DISystemMonitorEvents_Impl, const OFFSET: isize>() -> DISystemMonitorEvents_Vtbl {
        Self { base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<DISystemMonitorEvents as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait DISystemMonitorInternal_Impl: Sized + super::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl DISystemMonitorInternal_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: DISystemMonitorInternal_Impl, const OFFSET: isize>() -> DISystemMonitorInternal_Vtbl {
        Self { base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<DISystemMonitorInternal as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAlertDataCollector_Impl, const OFFSET: isize>() -> IAlertDataCollector_Vtbl {
        unsafe extern "system" fn AlertThresholds<Identity: ::windows::core::IUnknownImpl, Impl: IAlertDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, alerts: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AlertThresholds() {
                ::core::result::Result::Ok(ok__) => {
                    *alerts = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlertThresholds<Identity: ::windows::core::IUnknownImpl, Impl: IAlertDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, alerts: *const super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAlertThresholds(::core::mem::transmute_copy(&alerts)).into()
        }
        unsafe extern "system" fn EventLog<Identity: ::windows::core::IUnknownImpl, Impl: IAlertDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, log: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EventLog() {
                ::core::result::Result::Ok(ok__) => {
                    *log = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventLog<Identity: ::windows::core::IUnknownImpl, Impl: IAlertDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, log: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEventLog(::core::mem::transmute_copy(&log)).into()
        }
        unsafe extern "system" fn SampleInterval<Identity: ::windows::core::IUnknownImpl, Impl: IAlertDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SampleInterval() {
                ::core::result::Result::Ok(ok__) => {
                    *interval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSampleInterval<Identity: ::windows::core::IUnknownImpl, Impl: IAlertDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interval: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSampleInterval(::core::mem::transmute_copy(&interval)).into()
        }
        unsafe extern "system" fn Task<Identity: ::windows::core::IUnknownImpl, Impl: IAlertDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, task: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Task() {
                ::core::result::Result::Ok(ok__) => {
                    *task = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTask<Identity: ::windows::core::IUnknownImpl, Impl: IAlertDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, task: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTask(::core::mem::transmute_copy(&task)).into()
        }
        unsafe extern "system" fn TaskRunAsSelf<Identity: ::windows::core::IUnknownImpl, Impl: IAlertDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, runasself: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TaskRunAsSelf() {
                ::core::result::Result::Ok(ok__) => {
                    *runasself = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTaskRunAsSelf<Identity: ::windows::core::IUnknownImpl, Impl: IAlertDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, runasself: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTaskRunAsSelf(::core::mem::transmute_copy(&runasself)).into()
        }
        unsafe extern "system" fn TaskArguments<Identity: ::windows::core::IUnknownImpl, Impl: IAlertDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, task: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TaskArguments() {
                ::core::result::Result::Ok(ok__) => {
                    *task = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTaskArguments<Identity: ::windows::core::IUnknownImpl, Impl: IAlertDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, task: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTaskArguments(::core::mem::transmute_copy(&task)).into()
        }
        unsafe extern "system" fn TaskUserTextArguments<Identity: ::windows::core::IUnknownImpl, Impl: IAlertDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, task: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TaskUserTextArguments() {
                ::core::result::Result::Ok(ok__) => {
                    *task = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTaskUserTextArguments<Identity: ::windows::core::IUnknownImpl, Impl: IAlertDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, task: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTaskUserTextArguments(::core::mem::transmute_copy(&task)).into()
        }
        unsafe extern "system" fn TriggerDataCollectorSet<Identity: ::windows::core::IUnknownImpl, Impl: IAlertDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TriggerDataCollectorSet() {
                ::core::result::Result::Ok(ok__) => {
                    *name = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTriggerDataCollectorSet<Identity: ::windows::core::IUnknownImpl, Impl: IAlertDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTriggerDataCollectorSet(::core::mem::transmute_copy(&name)).into()
        }
        Self {
            base: IDataCollector_Vtbl::new::<Identity, Impl, OFFSET>(),
            AlertThresholds: AlertThresholds::<Identity, Impl, OFFSET>,
            SetAlertThresholds: SetAlertThresholds::<Identity, Impl, OFFSET>,
            EventLog: EventLog::<Identity, Impl, OFFSET>,
            SetEventLog: SetEventLog::<Identity, Impl, OFFSET>,
            SampleInterval: SampleInterval::<Identity, Impl, OFFSET>,
            SetSampleInterval: SetSampleInterval::<Identity, Impl, OFFSET>,
            Task: Task::<Identity, Impl, OFFSET>,
            SetTask: SetTask::<Identity, Impl, OFFSET>,
            TaskRunAsSelf: TaskRunAsSelf::<Identity, Impl, OFFSET>,
            SetTaskRunAsSelf: SetTaskRunAsSelf::<Identity, Impl, OFFSET>,
            TaskArguments: TaskArguments::<Identity, Impl, OFFSET>,
            SetTaskArguments: SetTaskArguments::<Identity, Impl, OFFSET>,
            TaskUserTextArguments: TaskUserTextArguments::<Identity, Impl, OFFSET>,
            SetTaskUserTextArguments: SetTaskUserTextArguments::<Identity, Impl, OFFSET>,
            TriggerDataCollectorSet: TriggerDataCollectorSet::<Identity, Impl, OFFSET>,
            SetTriggerDataCollectorSet: SetTriggerDataCollectorSet::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAlertDataCollector as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IDataCollector as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApiTracingDataCollector_Impl, const OFFSET: isize>() -> IApiTracingDataCollector_Vtbl {
        unsafe extern "system" fn LogApiNamesOnly<Identity: ::windows::core::IUnknownImpl, Impl: IApiTracingDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, logapinames: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LogApiNamesOnly() {
                ::core::result::Result::Ok(ok__) => {
                    *logapinames = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLogApiNamesOnly<Identity: ::windows::core::IUnknownImpl, Impl: IApiTracingDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, logapinames: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLogApiNamesOnly(::core::mem::transmute_copy(&logapinames)).into()
        }
        unsafe extern "system" fn LogApisRecursively<Identity: ::windows::core::IUnknownImpl, Impl: IApiTracingDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, logrecursively: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LogApisRecursively() {
                ::core::result::Result::Ok(ok__) => {
                    *logrecursively = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLogApisRecursively<Identity: ::windows::core::IUnknownImpl, Impl: IApiTracingDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, logrecursively: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLogApisRecursively(::core::mem::transmute_copy(&logrecursively)).into()
        }
        unsafe extern "system" fn ExePath<Identity: ::windows::core::IUnknownImpl, Impl: IApiTracingDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, exepath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ExePath() {
                ::core::result::Result::Ok(ok__) => {
                    *exepath = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExePath<Identity: ::windows::core::IUnknownImpl, Impl: IApiTracingDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, exepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetExePath(::core::mem::transmute_copy(&exepath)).into()
        }
        unsafe extern "system" fn LogFilePath<Identity: ::windows::core::IUnknownImpl, Impl: IApiTracingDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, logfilepath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LogFilePath() {
                ::core::result::Result::Ok(ok__) => {
                    *logfilepath = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLogFilePath<Identity: ::windows::core::IUnknownImpl, Impl: IApiTracingDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, logfilepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLogFilePath(::core::mem::transmute_copy(&logfilepath)).into()
        }
        unsafe extern "system" fn IncludeModules<Identity: ::windows::core::IUnknownImpl, Impl: IApiTracingDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, includemodules: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IncludeModules() {
                ::core::result::Result::Ok(ok__) => {
                    *includemodules = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIncludeModules<Identity: ::windows::core::IUnknownImpl, Impl: IApiTracingDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, includemodules: *const super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetIncludeModules(::core::mem::transmute_copy(&includemodules)).into()
        }
        unsafe extern "system" fn IncludeApis<Identity: ::windows::core::IUnknownImpl, Impl: IApiTracingDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, includeapis: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IncludeApis() {
                ::core::result::Result::Ok(ok__) => {
                    *includeapis = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIncludeApis<Identity: ::windows::core::IUnknownImpl, Impl: IApiTracingDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, includeapis: *const super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetIncludeApis(::core::mem::transmute_copy(&includeapis)).into()
        }
        unsafe extern "system" fn ExcludeApis<Identity: ::windows::core::IUnknownImpl, Impl: IApiTracingDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, excludeapis: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ExcludeApis() {
                ::core::result::Result::Ok(ok__) => {
                    *excludeapis = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExcludeApis<Identity: ::windows::core::IUnknownImpl, Impl: IApiTracingDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, excludeapis: *const super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetExcludeApis(::core::mem::transmute_copy(&excludeapis)).into()
        }
        Self {
            base: IDataCollector_Vtbl::new::<Identity, Impl, OFFSET>(),
            LogApiNamesOnly: LogApiNamesOnly::<Identity, Impl, OFFSET>,
            SetLogApiNamesOnly: SetLogApiNamesOnly::<Identity, Impl, OFFSET>,
            LogApisRecursively: LogApisRecursively::<Identity, Impl, OFFSET>,
            SetLogApisRecursively: SetLogApisRecursively::<Identity, Impl, OFFSET>,
            ExePath: ExePath::<Identity, Impl, OFFSET>,
            SetExePath: SetExePath::<Identity, Impl, OFFSET>,
            LogFilePath: LogFilePath::<Identity, Impl, OFFSET>,
            SetLogFilePath: SetLogFilePath::<Identity, Impl, OFFSET>,
            IncludeModules: IncludeModules::<Identity, Impl, OFFSET>,
            SetIncludeModules: SetIncludeModules::<Identity, Impl, OFFSET>,
            IncludeApis: IncludeApis::<Identity, Impl, OFFSET>,
            SetIncludeApis: SetIncludeApis::<Identity, Impl, OFFSET>,
            ExcludeApis: ExcludeApis::<Identity, Impl, OFFSET>,
            SetExcludeApis: SetExcludeApis::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IApiTracingDataCollector as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IDataCollector as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConfigurationDataCollector_Impl, const OFFSET: isize>() -> IConfigurationDataCollector_Vtbl {
        unsafe extern "system" fn FileMaxCount<Identity: ::windows::core::IUnknownImpl, Impl: IConfigurationDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FileMaxCount() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFileMaxCount<Identity: ::windows::core::IUnknownImpl, Impl: IConfigurationDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFileMaxCount(::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn FileMaxRecursiveDepth<Identity: ::windows::core::IUnknownImpl, Impl: IConfigurationDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, depth: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FileMaxRecursiveDepth() {
                ::core::result::Result::Ok(ok__) => {
                    *depth = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFileMaxRecursiveDepth<Identity: ::windows::core::IUnknownImpl, Impl: IConfigurationDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, depth: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFileMaxRecursiveDepth(::core::mem::transmute_copy(&depth)).into()
        }
        unsafe extern "system" fn FileMaxTotalSize<Identity: ::windows::core::IUnknownImpl, Impl: IConfigurationDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, size: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FileMaxTotalSize() {
                ::core::result::Result::Ok(ok__) => {
                    *size = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFileMaxTotalSize<Identity: ::windows::core::IUnknownImpl, Impl: IConfigurationDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, size: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFileMaxTotalSize(::core::mem::transmute_copy(&size)).into()
        }
        unsafe extern "system" fn Files<Identity: ::windows::core::IUnknownImpl, Impl: IConfigurationDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, files: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Files() {
                ::core::result::Result::Ok(ok__) => {
                    *files = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFiles<Identity: ::windows::core::IUnknownImpl, Impl: IConfigurationDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, files: *const super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFiles(::core::mem::transmute_copy(&files)).into()
        }
        unsafe extern "system" fn ManagementQueries<Identity: ::windows::core::IUnknownImpl, Impl: IConfigurationDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, queries: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ManagementQueries() {
                ::core::result::Result::Ok(ok__) => {
                    *queries = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetManagementQueries<Identity: ::windows::core::IUnknownImpl, Impl: IConfigurationDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, queries: *const super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetManagementQueries(::core::mem::transmute_copy(&queries)).into()
        }
        unsafe extern "system" fn QueryNetworkAdapters<Identity: ::windows::core::IUnknownImpl, Impl: IConfigurationDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, network: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).QueryNetworkAdapters() {
                ::core::result::Result::Ok(ok__) => {
                    *network = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQueryNetworkAdapters<Identity: ::windows::core::IUnknownImpl, Impl: IConfigurationDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, network: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetQueryNetworkAdapters(::core::mem::transmute_copy(&network)).into()
        }
        unsafe extern "system" fn RegistryKeys<Identity: ::windows::core::IUnknownImpl, Impl: IConfigurationDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, query: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RegistryKeys() {
                ::core::result::Result::Ok(ok__) => {
                    *query = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRegistryKeys<Identity: ::windows::core::IUnknownImpl, Impl: IConfigurationDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, query: *const super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRegistryKeys(::core::mem::transmute_copy(&query)).into()
        }
        unsafe extern "system" fn RegistryMaxRecursiveDepth<Identity: ::windows::core::IUnknownImpl, Impl: IConfigurationDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, depth: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RegistryMaxRecursiveDepth() {
                ::core::result::Result::Ok(ok__) => {
                    *depth = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRegistryMaxRecursiveDepth<Identity: ::windows::core::IUnknownImpl, Impl: IConfigurationDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, depth: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRegistryMaxRecursiveDepth(::core::mem::transmute_copy(&depth)).into()
        }
        unsafe extern "system" fn SystemStateFile<Identity: ::windows::core::IUnknownImpl, Impl: IConfigurationDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SystemStateFile() {
                ::core::result::Result::Ok(ok__) => {
                    *filename = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSystemStateFile<Identity: ::windows::core::IUnknownImpl, Impl: IConfigurationDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSystemStateFile(::core::mem::transmute_copy(&filename)).into()
        }
        Self {
            base: IDataCollector_Vtbl::new::<Identity, Impl, OFFSET>(),
            FileMaxCount: FileMaxCount::<Identity, Impl, OFFSET>,
            SetFileMaxCount: SetFileMaxCount::<Identity, Impl, OFFSET>,
            FileMaxRecursiveDepth: FileMaxRecursiveDepth::<Identity, Impl, OFFSET>,
            SetFileMaxRecursiveDepth: SetFileMaxRecursiveDepth::<Identity, Impl, OFFSET>,
            FileMaxTotalSize: FileMaxTotalSize::<Identity, Impl, OFFSET>,
            SetFileMaxTotalSize: SetFileMaxTotalSize::<Identity, Impl, OFFSET>,
            Files: Files::<Identity, Impl, OFFSET>,
            SetFiles: SetFiles::<Identity, Impl, OFFSET>,
            ManagementQueries: ManagementQueries::<Identity, Impl, OFFSET>,
            SetManagementQueries: SetManagementQueries::<Identity, Impl, OFFSET>,
            QueryNetworkAdapters: QueryNetworkAdapters::<Identity, Impl, OFFSET>,
            SetQueryNetworkAdapters: SetQueryNetworkAdapters::<Identity, Impl, OFFSET>,
            RegistryKeys: RegistryKeys::<Identity, Impl, OFFSET>,
            SetRegistryKeys: SetRegistryKeys::<Identity, Impl, OFFSET>,
            RegistryMaxRecursiveDepth: RegistryMaxRecursiveDepth::<Identity, Impl, OFFSET>,
            SetRegistryMaxRecursiveDepth: SetRegistryMaxRecursiveDepth::<Identity, Impl, OFFSET>,
            SystemStateFile: SystemStateFile::<Identity, Impl, OFFSET>,
            SetSystemStateFile: SetSystemStateFile::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConfigurationDataCollector as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IDataCollector as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICounterItem_Impl, const OFFSET: isize>() -> ICounterItem_Vtbl {
        unsafe extern "system" fn Value<Identity: ::windows::core::IUnknownImpl, Impl: ICounterItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdblvalue: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *pdblvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColor<Identity: ::windows::core::IUnknownImpl, Impl: ICounterItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, color: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetColor(::core::mem::transmute_copy(&color)).into()
        }
        unsafe extern "system" fn Color<Identity: ::windows::core::IUnknownImpl, Impl: ICounterItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcolor: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Color() {
                ::core::result::Result::Ok(ok__) => {
                    *pcolor = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWidth<Identity: ::windows::core::IUnknownImpl, Impl: ICounterItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iwidth: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetWidth(::core::mem::transmute_copy(&iwidth)).into()
        }
        unsafe extern "system" fn Width<Identity: ::windows::core::IUnknownImpl, Impl: ICounterItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pivalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Width() {
                ::core::result::Result::Ok(ok__) => {
                    *pivalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLineStyle<Identity: ::windows::core::IUnknownImpl, Impl: ICounterItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ilinestyle: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLineStyle(::core::mem::transmute_copy(&ilinestyle)).into()
        }
        unsafe extern "system" fn LineStyle<Identity: ::windows::core::IUnknownImpl, Impl: ICounterItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pivalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LineStyle() {
                ::core::result::Result::Ok(ok__) => {
                    *pivalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScaleFactor<Identity: ::windows::core::IUnknownImpl, Impl: ICounterItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iscale: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetScaleFactor(::core::mem::transmute_copy(&iscale)).into()
        }
        unsafe extern "system" fn ScaleFactor<Identity: ::windows::core::IUnknownImpl, Impl: ICounterItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pivalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ScaleFactor() {
                ::core::result::Result::Ok(ok__) => {
                    *pivalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Path<Identity: ::windows::core::IUnknownImpl, Impl: ICounterItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstrvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Path() {
                ::core::result::Result::Ok(ok__) => {
                    *pstrvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValue<Identity: ::windows::core::IUnknownImpl, Impl: ICounterItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut f64, status: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetValue(::core::mem::transmute_copy(&value), ::core::mem::transmute_copy(&status)).into()
        }
        unsafe extern "system" fn GetStatistics<Identity: ::windows::core::IUnknownImpl, Impl: ICounterItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, max: *mut f64, min: *mut f64, avg: *mut f64, status: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetStatistics(::core::mem::transmute_copy(&max), ::core::mem::transmute_copy(&min), ::core::mem::transmute_copy(&avg), ::core::mem::transmute_copy(&status)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Value: Value::<Identity, Impl, OFFSET>,
            SetColor: SetColor::<Identity, Impl, OFFSET>,
            Color: Color::<Identity, Impl, OFFSET>,
            SetWidth: SetWidth::<Identity, Impl, OFFSET>,
            Width: Width::<Identity, Impl, OFFSET>,
            SetLineStyle: SetLineStyle::<Identity, Impl, OFFSET>,
            LineStyle: LineStyle::<Identity, Impl, OFFSET>,
            SetScaleFactor: SetScaleFactor::<Identity, Impl, OFFSET>,
            ScaleFactor: ScaleFactor::<Identity, Impl, OFFSET>,
            Path: Path::<Identity, Impl, OFFSET>,
            GetValue: GetValue::<Identity, Impl, OFFSET>,
            GetStatistics: GetStatistics::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICounterItem2_Impl, const OFFSET: isize>() -> ICounterItem2_Vtbl {
        unsafe extern "system" fn SetSelected<Identity: ::windows::core::IUnknownImpl, Impl: ICounterItem2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSelected(::core::mem::transmute_copy(&bstate)).into()
        }
        unsafe extern "system" fn Selected<Identity: ::windows::core::IUnknownImpl, Impl: ICounterItem2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Selected() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVisible<Identity: ::windows::core::IUnknownImpl, Impl: ICounterItem2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetVisible(::core::mem::transmute_copy(&bstate)).into()
        }
        unsafe extern "system" fn Visible<Identity: ::windows::core::IUnknownImpl, Impl: ICounterItem2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Visible() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDataAt<Identity: ::windows::core::IUnknownImpl, Impl: ICounterItem2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iindex: i32, iwhich: SysmonDataType, pvariant: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDataAt(::core::mem::transmute_copy(&iindex), ::core::mem::transmute_copy(&iwhich)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ICounterItem_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetSelected: SetSelected::<Identity, Impl, OFFSET>,
            Selected: Selected::<Identity, Impl, OFFSET>,
            SetVisible: SetVisible::<Identity, Impl, OFFSET>,
            Visible: Visible::<Identity, Impl, OFFSET>,
            GetDataAt: GetDataAt::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICounterItem2 as ::windows::core::Interface>::IID || iid == &<ICounterItem as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICounters_Impl, const OFFSET: isize>() -> ICounters_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: ICounters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plong: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *plong = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: ICounters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppiunk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: ICounters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppi: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppi = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: ::windows::core::IUnknownImpl, Impl: ICounters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pathname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppi: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Add(::core::mem::transmute_copy(&pathname)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppi = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Identity: ::windows::core::IUnknownImpl, Impl: ICounters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Remove(::core::mem::transmute_copy(&index)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICounters as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollector_Impl, const OFFSET: isize>() -> IDataCollector_Vtbl {
        unsafe extern "system" fn DataCollectorSet<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, group: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DataCollectorSet() {
                ::core::result::Result::Ok(ok__) => {
                    *group = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDataCollectorSet<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, group: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDataCollectorSet(::core::mem::transmute(&group)).into()
        }
        unsafe extern "system" fn DataCollectorType<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: *mut DataCollectorType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DataCollectorType() {
                ::core::result::Result::Ok(ok__) => {
                    *r#type = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FileName<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FileName() {
                ::core::result::Result::Ok(ok__) => {
                    *name = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFileName<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFileName(::core::mem::transmute_copy(&name)).into()
        }
        unsafe extern "system" fn FileNameFormat<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: *mut AutoPathFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FileNameFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *format = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFileNameFormat<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: AutoPathFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFileNameFormat(::core::mem::transmute_copy(&format)).into()
        }
        unsafe extern "system" fn FileNameFormatPattern<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pattern: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FileNameFormatPattern() {
                ::core::result::Result::Ok(ok__) => {
                    *pattern = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFileNameFormatPattern<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pattern: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFileNameFormatPattern(::core::mem::transmute_copy(&pattern)).into()
        }
        unsafe extern "system" fn LatestOutputLocation<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LatestOutputLocation() {
                ::core::result::Result::Ok(ok__) => {
                    *path = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLatestOutputLocation<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLatestOutputLocation(::core::mem::transmute_copy(&path)).into()
        }
        unsafe extern "system" fn LogAppend<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, append: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LogAppend() {
                ::core::result::Result::Ok(ok__) => {
                    *append = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLogAppend<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, append: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLogAppend(::core::mem::transmute_copy(&append)).into()
        }
        unsafe extern "system" fn LogCircular<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, circular: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LogCircular() {
                ::core::result::Result::Ok(ok__) => {
                    *circular = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLogCircular<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, circular: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLogCircular(::core::mem::transmute_copy(&circular)).into()
        }
        unsafe extern "system" fn LogOverwrite<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, overwrite: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LogOverwrite() {
                ::core::result::Result::Ok(ok__) => {
                    *overwrite = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLogOverwrite<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, overwrite: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLogOverwrite(::core::mem::transmute_copy(&overwrite)).into()
        }
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *name = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetName(::core::mem::transmute_copy(&name)).into()
        }
        unsafe extern "system" fn OutputLocation<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).OutputLocation() {
                ::core::result::Result::Ok(ok__) => {
                    *path = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Index<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Index() {
                ::core::result::Result::Ok(ok__) => {
                    *index = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIndex<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetIndex(::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn Xml<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xml: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Xml() {
                ::core::result::Result::Ok(ok__) => {
                    *xml = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetXml<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xml: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, validation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SetXml(::core::mem::transmute_copy(&xml)) {
                ::core::result::Result::Ok(ok__) => {
                    *validation = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateOutputLocation<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, latest: i16, location: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateOutputLocation(::core::mem::transmute_copy(&latest)) {
                ::core::result::Result::Ok(ok__) => {
                    *location = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            DataCollectorSet: DataCollectorSet::<Identity, Impl, OFFSET>,
            SetDataCollectorSet: SetDataCollectorSet::<Identity, Impl, OFFSET>,
            DataCollectorType: DataCollectorType::<Identity, Impl, OFFSET>,
            FileName: FileName::<Identity, Impl, OFFSET>,
            SetFileName: SetFileName::<Identity, Impl, OFFSET>,
            FileNameFormat: FileNameFormat::<Identity, Impl, OFFSET>,
            SetFileNameFormat: SetFileNameFormat::<Identity, Impl, OFFSET>,
            FileNameFormatPattern: FileNameFormatPattern::<Identity, Impl, OFFSET>,
            SetFileNameFormatPattern: SetFileNameFormatPattern::<Identity, Impl, OFFSET>,
            LatestOutputLocation: LatestOutputLocation::<Identity, Impl, OFFSET>,
            SetLatestOutputLocation: SetLatestOutputLocation::<Identity, Impl, OFFSET>,
            LogAppend: LogAppend::<Identity, Impl, OFFSET>,
            SetLogAppend: SetLogAppend::<Identity, Impl, OFFSET>,
            LogCircular: LogCircular::<Identity, Impl, OFFSET>,
            SetLogCircular: SetLogCircular::<Identity, Impl, OFFSET>,
            LogOverwrite: LogOverwrite::<Identity, Impl, OFFSET>,
            SetLogOverwrite: SetLogOverwrite::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            OutputLocation: OutputLocation::<Identity, Impl, OFFSET>,
            Index: Index::<Identity, Impl, OFFSET>,
            SetIndex: SetIndex::<Identity, Impl, OFFSET>,
            Xml: Xml::<Identity, Impl, OFFSET>,
            SetXml: SetXml::<Identity, Impl, OFFSET>,
            CreateOutputLocation: CreateOutputLocation::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDataCollector as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollectorCollection_Impl, const OFFSET: isize>() -> IDataCollectorCollection_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollectorCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollectorCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, collector: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *collector = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollectorCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollectorCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, collector: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Add(::core::mem::transmute(&collector)).into()
        }
        unsafe extern "system" fn Remove<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollectorCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, collector: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Remove(::core::mem::transmute_copy(&collector)).into()
        }
        unsafe extern "system" fn Clear<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollectorCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Clear().into()
        }
        unsafe extern "system" fn AddRange<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollectorCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, collectors: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddRange(::core::mem::transmute(&collectors)).into()
        }
        unsafe extern "system" fn CreateDataCollectorFromXml<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollectorCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrxml: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvalidation: *mut ::windows::core::RawPtr, pcollector: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateDataCollectorFromXml(::core::mem::transmute_copy(&bstrxml), ::core::mem::transmute_copy(&pvalidation), ::core::mem::transmute_copy(&pcollector)).into()
        }
        unsafe extern "system" fn CreateDataCollector<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollectorCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: DataCollectorType, collector: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateDataCollector(::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    *collector = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            Clear: Clear::<Identity, Impl, OFFSET>,
            AddRange: AddRange::<Identity, Impl, OFFSET>,
            CreateDataCollectorFromXml: CreateDataCollectorFromXml::<Identity, Impl, OFFSET>,
            CreateDataCollector: CreateDataCollector::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDataCollectorCollection as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollectorSet_Impl, const OFFSET: isize>() -> IDataCollectorSet_Vtbl {
        unsafe extern "system" fn DataCollectors<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, collectors: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DataCollectors() {
                ::core::result::Result::Ok(ok__) => {
                    *collectors = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Duration<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, seconds: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Duration() {
                ::core::result::Result::Ok(ok__) => {
                    *seconds = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDuration<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, seconds: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDuration(::core::mem::transmute_copy(&seconds)).into()
        }
        unsafe extern "system" fn Description<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, description: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *description = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, description: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDescription(::core::mem::transmute_copy(&description)).into()
        }
        unsafe extern "system" fn DescriptionUnresolved<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, descr: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DescriptionUnresolved() {
                ::core::result::Result::Ok(ok__) => {
                    *descr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayName<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, displayname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *displayname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayName<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, displayname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDisplayName(::core::mem::transmute_copy(&displayname)).into()
        }
        unsafe extern "system" fn DisplayNameUnresolved<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DisplayNameUnresolved() {
                ::core::result::Result::Ok(ok__) => {
                    *name = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Keywords<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, keywords: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Keywords() {
                ::core::result::Result::Ok(ok__) => {
                    *keywords = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeywords<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, keywords: *const super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetKeywords(::core::mem::transmute_copy(&keywords)).into()
        }
        unsafe extern "system" fn LatestOutputLocation<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LatestOutputLocation() {
                ::core::result::Result::Ok(ok__) => {
                    *path = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLatestOutputLocation<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLatestOutputLocation(::core::mem::transmute_copy(&path)).into()
        }
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *name = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OutputLocation<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).OutputLocation() {
                ::core::result::Result::Ok(ok__) => {
                    *path = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RootPath<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, folder: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RootPath() {
                ::core::result::Result::Ok(ok__) => {
                    *folder = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRootPath<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, folder: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRootPath(::core::mem::transmute_copy(&folder)).into()
        }
        unsafe extern "system" fn Segment<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, segment: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Segment() {
                ::core::result::Result::Ok(ok__) => {
                    *segment = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSegment<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, segment: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSegment(::core::mem::transmute_copy(&segment)).into()
        }
        unsafe extern "system" fn SegmentMaxDuration<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, seconds: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SegmentMaxDuration() {
                ::core::result::Result::Ok(ok__) => {
                    *seconds = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSegmentMaxDuration<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, seconds: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSegmentMaxDuration(::core::mem::transmute_copy(&seconds)).into()
        }
        unsafe extern "system" fn SegmentMaxSize<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, size: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SegmentMaxSize() {
                ::core::result::Result::Ok(ok__) => {
                    *size = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSegmentMaxSize<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, size: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSegmentMaxSize(::core::mem::transmute_copy(&size)).into()
        }
        unsafe extern "system" fn SerialNumber<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SerialNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *index = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSerialNumber<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSerialNumber(::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn Server<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, server: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Server() {
                ::core::result::Result::Ok(ok__) => {
                    *server = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: *mut DataCollectorSetStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *status = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Subdirectory<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, folder: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Subdirectory() {
                ::core::result::Result::Ok(ok__) => {
                    *folder = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSubdirectory<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, folder: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSubdirectory(::core::mem::transmute_copy(&folder)).into()
        }
        unsafe extern "system" fn SubdirectoryFormat<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: *mut AutoPathFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SubdirectoryFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *format = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSubdirectoryFormat<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: AutoPathFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSubdirectoryFormat(::core::mem::transmute_copy(&format)).into()
        }
        unsafe extern "system" fn SubdirectoryFormatPattern<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pattern: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SubdirectoryFormatPattern() {
                ::core::result::Result::Ok(ok__) => {
                    *pattern = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSubdirectoryFormatPattern<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pattern: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSubdirectoryFormatPattern(::core::mem::transmute_copy(&pattern)).into()
        }
        unsafe extern "system" fn Task<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, task: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Task() {
                ::core::result::Result::Ok(ok__) => {
                    *task = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTask<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, task: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTask(::core::mem::transmute_copy(&task)).into()
        }
        unsafe extern "system" fn TaskRunAsSelf<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, runasself: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TaskRunAsSelf() {
                ::core::result::Result::Ok(ok__) => {
                    *runasself = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTaskRunAsSelf<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, runasself: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTaskRunAsSelf(::core::mem::transmute_copy(&runasself)).into()
        }
        unsafe extern "system" fn TaskArguments<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, task: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TaskArguments() {
                ::core::result::Result::Ok(ok__) => {
                    *task = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTaskArguments<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, task: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTaskArguments(::core::mem::transmute_copy(&task)).into()
        }
        unsafe extern "system" fn TaskUserTextArguments<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usertext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TaskUserTextArguments() {
                ::core::result::Result::Ok(ok__) => {
                    *usertext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTaskUserTextArguments<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usertext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTaskUserTextArguments(::core::mem::transmute_copy(&usertext)).into()
        }
        unsafe extern "system" fn Schedules<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppschedules: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Schedules() {
                ::core::result::Result::Ok(ok__) => {
                    *ppschedules = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SchedulesEnabled<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SchedulesEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *enabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSchedulesEnabled<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSchedulesEnabled(::core::mem::transmute_copy(&enabled)).into()
        }
        unsafe extern "system" fn UserAccount<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).UserAccount() {
                ::core::result::Result::Ok(ok__) => {
                    *user = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Xml<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xml: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Xml() {
                ::core::result::Result::Ok(ok__) => {
                    *xml = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Security<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsecurity: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Security() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrsecurity = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSecurity<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsecurity: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSecurity(::core::mem::transmute_copy(&bstrsecurity)).into()
        }
        unsafe extern "system" fn StopOnCompletion<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stop: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).StopOnCompletion() {
                ::core::result::Result::Ok(ok__) => {
                    *stop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStopOnCompletion<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stop: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetStopOnCompletion(::core::mem::transmute_copy(&stop)).into()
        }
        unsafe extern "system" fn DataManager<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, datamanager: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DataManager() {
                ::core::result::Result::Ok(ok__) => {
                    *datamanager = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCredentials<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, password: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCredentials(::core::mem::transmute_copy(&user), ::core::mem::transmute_copy(&password)).into()
        }
        unsafe extern "system" fn Query<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, server: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Query(::core::mem::transmute_copy(&name), ::core::mem::transmute_copy(&server)).into()
        }
        unsafe extern "system" fn Commit<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, server: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, mode: CommitMode, validation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Commit(::core::mem::transmute_copy(&name), ::core::mem::transmute_copy(&server), ::core::mem::transmute_copy(&mode)) {
                ::core::result::Result::Ok(ok__) => {
                    *validation = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Delete().into()
        }
        unsafe extern "system" fn Start<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, synchronous: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Start(::core::mem::transmute_copy(&synchronous)).into()
        }
        unsafe extern "system" fn Stop<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, synchronous: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Stop(::core::mem::transmute_copy(&synchronous)).into()
        }
        unsafe extern "system" fn SetXml<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xml: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, validation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SetXml(::core::mem::transmute_copy(&xml)) {
                ::core::result::Result::Ok(ok__) => {
                    *validation = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetValue(::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetValue<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetValue(::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            DataCollectors: DataCollectors::<Identity, Impl, OFFSET>,
            Duration: Duration::<Identity, Impl, OFFSET>,
            SetDuration: SetDuration::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            DescriptionUnresolved: DescriptionUnresolved::<Identity, Impl, OFFSET>,
            DisplayName: DisplayName::<Identity, Impl, OFFSET>,
            SetDisplayName: SetDisplayName::<Identity, Impl, OFFSET>,
            DisplayNameUnresolved: DisplayNameUnresolved::<Identity, Impl, OFFSET>,
            Keywords: Keywords::<Identity, Impl, OFFSET>,
            SetKeywords: SetKeywords::<Identity, Impl, OFFSET>,
            LatestOutputLocation: LatestOutputLocation::<Identity, Impl, OFFSET>,
            SetLatestOutputLocation: SetLatestOutputLocation::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            OutputLocation: OutputLocation::<Identity, Impl, OFFSET>,
            RootPath: RootPath::<Identity, Impl, OFFSET>,
            SetRootPath: SetRootPath::<Identity, Impl, OFFSET>,
            Segment: Segment::<Identity, Impl, OFFSET>,
            SetSegment: SetSegment::<Identity, Impl, OFFSET>,
            SegmentMaxDuration: SegmentMaxDuration::<Identity, Impl, OFFSET>,
            SetSegmentMaxDuration: SetSegmentMaxDuration::<Identity, Impl, OFFSET>,
            SegmentMaxSize: SegmentMaxSize::<Identity, Impl, OFFSET>,
            SetSegmentMaxSize: SetSegmentMaxSize::<Identity, Impl, OFFSET>,
            SerialNumber: SerialNumber::<Identity, Impl, OFFSET>,
            SetSerialNumber: SetSerialNumber::<Identity, Impl, OFFSET>,
            Server: Server::<Identity, Impl, OFFSET>,
            Status: Status::<Identity, Impl, OFFSET>,
            Subdirectory: Subdirectory::<Identity, Impl, OFFSET>,
            SetSubdirectory: SetSubdirectory::<Identity, Impl, OFFSET>,
            SubdirectoryFormat: SubdirectoryFormat::<Identity, Impl, OFFSET>,
            SetSubdirectoryFormat: SetSubdirectoryFormat::<Identity, Impl, OFFSET>,
            SubdirectoryFormatPattern: SubdirectoryFormatPattern::<Identity, Impl, OFFSET>,
            SetSubdirectoryFormatPattern: SetSubdirectoryFormatPattern::<Identity, Impl, OFFSET>,
            Task: Task::<Identity, Impl, OFFSET>,
            SetTask: SetTask::<Identity, Impl, OFFSET>,
            TaskRunAsSelf: TaskRunAsSelf::<Identity, Impl, OFFSET>,
            SetTaskRunAsSelf: SetTaskRunAsSelf::<Identity, Impl, OFFSET>,
            TaskArguments: TaskArguments::<Identity, Impl, OFFSET>,
            SetTaskArguments: SetTaskArguments::<Identity, Impl, OFFSET>,
            TaskUserTextArguments: TaskUserTextArguments::<Identity, Impl, OFFSET>,
            SetTaskUserTextArguments: SetTaskUserTextArguments::<Identity, Impl, OFFSET>,
            Schedules: Schedules::<Identity, Impl, OFFSET>,
            SchedulesEnabled: SchedulesEnabled::<Identity, Impl, OFFSET>,
            SetSchedulesEnabled: SetSchedulesEnabled::<Identity, Impl, OFFSET>,
            UserAccount: UserAccount::<Identity, Impl, OFFSET>,
            Xml: Xml::<Identity, Impl, OFFSET>,
            Security: Security::<Identity, Impl, OFFSET>,
            SetSecurity: SetSecurity::<Identity, Impl, OFFSET>,
            StopOnCompletion: StopOnCompletion::<Identity, Impl, OFFSET>,
            SetStopOnCompletion: SetStopOnCompletion::<Identity, Impl, OFFSET>,
            DataManager: DataManager::<Identity, Impl, OFFSET>,
            SetCredentials: SetCredentials::<Identity, Impl, OFFSET>,
            Query: Query::<Identity, Impl, OFFSET>,
            Commit: Commit::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            Start: Start::<Identity, Impl, OFFSET>,
            Stop: Stop::<Identity, Impl, OFFSET>,
            SetXml: SetXml::<Identity, Impl, OFFSET>,
            SetValue: SetValue::<Identity, Impl, OFFSET>,
            GetValue: GetValue::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDataCollectorSet as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollectorSetCollection_Impl, const OFFSET: isize>() -> IDataCollectorSetCollection_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollectorSetCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollectorSetCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, set: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *set = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollectorSetCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollectorSetCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, set: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Add(::core::mem::transmute(&set)).into()
        }
        unsafe extern "system" fn Remove<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollectorSetCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, set: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Remove(::core::mem::transmute_copy(&set)).into()
        }
        unsafe extern "system" fn Clear<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollectorSetCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Clear().into()
        }
        unsafe extern "system" fn AddRange<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollectorSetCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sets: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddRange(::core::mem::transmute(&sets)).into()
        }
        unsafe extern "system" fn GetDataCollectorSets<Identity: ::windows::core::IUnknownImpl, Impl: IDataCollectorSetCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, server: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, filter: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDataCollectorSets(::core::mem::transmute_copy(&server), ::core::mem::transmute_copy(&filter)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            Clear: Clear::<Identity, Impl, OFFSET>,
            AddRange: AddRange::<Identity, Impl, OFFSET>,
            GetDataCollectorSets: GetDataCollectorSets::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDataCollectorSetCollection as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataManager_Impl, const OFFSET: isize>() -> IDataManager_Vtbl {
        unsafe extern "system" fn Enabled<Identity: ::windows::core::IUnknownImpl, Impl: IDataManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Enabled() {
                ::core::result::Result::Ok(ok__) => {
                    *pfenabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: ::windows::core::IUnknownImpl, Impl: IDataManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEnabled(::core::mem::transmute_copy(&fenabled)).into()
        }
        unsafe extern "system" fn CheckBeforeRunning<Identity: ::windows::core::IUnknownImpl, Impl: IDataManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfcheck: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CheckBeforeRunning() {
                ::core::result::Result::Ok(ok__) => {
                    *pfcheck = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCheckBeforeRunning<Identity: ::windows::core::IUnknownImpl, Impl: IDataManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fcheck: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCheckBeforeRunning(::core::mem::transmute_copy(&fcheck)).into()
        }
        unsafe extern "system" fn MinFreeDisk<Identity: ::windows::core::IUnknownImpl, Impl: IDataManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minfreedisk: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MinFreeDisk() {
                ::core::result::Result::Ok(ok__) => {
                    *minfreedisk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinFreeDisk<Identity: ::windows::core::IUnknownImpl, Impl: IDataManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minfreedisk: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMinFreeDisk(::core::mem::transmute_copy(&minfreedisk)).into()
        }
        unsafe extern "system" fn MaxSize<Identity: ::windows::core::IUnknownImpl, Impl: IDataManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulmaxsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MaxSize() {
                ::core::result::Result::Ok(ok__) => {
                    *pulmaxsize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxSize<Identity: ::windows::core::IUnknownImpl, Impl: IDataManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulmaxsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMaxSize(::core::mem::transmute_copy(&ulmaxsize)).into()
        }
        unsafe extern "system" fn MaxFolderCount<Identity: ::windows::core::IUnknownImpl, Impl: IDataManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulmaxfoldercount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MaxFolderCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pulmaxfoldercount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxFolderCount<Identity: ::windows::core::IUnknownImpl, Impl: IDataManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulmaxfoldercount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMaxFolderCount(::core::mem::transmute_copy(&ulmaxfoldercount)).into()
        }
        unsafe extern "system" fn ResourcePolicy<Identity: ::windows::core::IUnknownImpl, Impl: IDataManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppolicy: *mut ResourcePolicy) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ResourcePolicy() {
                ::core::result::Result::Ok(ok__) => {
                    *ppolicy = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetResourcePolicy<Identity: ::windows::core::IUnknownImpl, Impl: IDataManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, policy: ResourcePolicy) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetResourcePolicy(::core::mem::transmute_copy(&policy)).into()
        }
        unsafe extern "system" fn FolderActions<Identity: ::windows::core::IUnknownImpl, Impl: IDataManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, actions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FolderActions() {
                ::core::result::Result::Ok(ok__) => {
                    *actions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportSchema<Identity: ::windows::core::IUnknownImpl, Impl: IDataManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reportschema: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ReportSchema() {
                ::core::result::Result::Ok(ok__) => {
                    *reportschema = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReportSchema<Identity: ::windows::core::IUnknownImpl, Impl: IDataManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reportschema: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetReportSchema(::core::mem::transmute_copy(&reportschema)).into()
        }
        unsafe extern "system" fn ReportFileName<Identity: ::windows::core::IUnknownImpl, Impl: IDataManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrfilename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ReportFileName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrfilename = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReportFileName<Identity: ::windows::core::IUnknownImpl, Impl: IDataManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetReportFileName(::core::mem::transmute_copy(&pbstrfilename)).into()
        }
        unsafe extern "system" fn RuleTargetFileName<Identity: ::windows::core::IUnknownImpl, Impl: IDataManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RuleTargetFileName() {
                ::core::result::Result::Ok(ok__) => {
                    *filename = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRuleTargetFileName<Identity: ::windows::core::IUnknownImpl, Impl: IDataManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRuleTargetFileName(::core::mem::transmute_copy(&filename)).into()
        }
        unsafe extern "system" fn EventsFileName<Identity: ::windows::core::IUnknownImpl, Impl: IDataManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrfilename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EventsFileName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrfilename = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventsFileName<Identity: ::windows::core::IUnknownImpl, Impl: IDataManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEventsFileName(::core::mem::transmute_copy(&pbstrfilename)).into()
        }
        unsafe extern "system" fn Rules<Identity: ::windows::core::IUnknownImpl, Impl: IDataManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrxml: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Rules() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrxml = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRules<Identity: ::windows::core::IUnknownImpl, Impl: IDataManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrxml: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRules(::core::mem::transmute_copy(&bstrxml)).into()
        }
        unsafe extern "system" fn Run<Identity: ::windows::core::IUnknownImpl, Impl: IDataManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, steps: DataManagerSteps, bstrfolder: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, errors: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Run(::core::mem::transmute_copy(&steps), ::core::mem::transmute_copy(&bstrfolder)) {
                ::core::result::Result::Ok(ok__) => {
                    *errors = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Extract<Identity: ::windows::core::IUnknownImpl, Impl: IDataManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cabfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, destinationpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Extract(::core::mem::transmute_copy(&cabfilename), ::core::mem::transmute_copy(&destinationpath)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Enabled: Enabled::<Identity, Impl, OFFSET>,
            SetEnabled: SetEnabled::<Identity, Impl, OFFSET>,
            CheckBeforeRunning: CheckBeforeRunning::<Identity, Impl, OFFSET>,
            SetCheckBeforeRunning: SetCheckBeforeRunning::<Identity, Impl, OFFSET>,
            MinFreeDisk: MinFreeDisk::<Identity, Impl, OFFSET>,
            SetMinFreeDisk: SetMinFreeDisk::<Identity, Impl, OFFSET>,
            MaxSize: MaxSize::<Identity, Impl, OFFSET>,
            SetMaxSize: SetMaxSize::<Identity, Impl, OFFSET>,
            MaxFolderCount: MaxFolderCount::<Identity, Impl, OFFSET>,
            SetMaxFolderCount: SetMaxFolderCount::<Identity, Impl, OFFSET>,
            ResourcePolicy: ResourcePolicy::<Identity, Impl, OFFSET>,
            SetResourcePolicy: SetResourcePolicy::<Identity, Impl, OFFSET>,
            FolderActions: FolderActions::<Identity, Impl, OFFSET>,
            ReportSchema: ReportSchema::<Identity, Impl, OFFSET>,
            SetReportSchema: SetReportSchema::<Identity, Impl, OFFSET>,
            ReportFileName: ReportFileName::<Identity, Impl, OFFSET>,
            SetReportFileName: SetReportFileName::<Identity, Impl, OFFSET>,
            RuleTargetFileName: RuleTargetFileName::<Identity, Impl, OFFSET>,
            SetRuleTargetFileName: SetRuleTargetFileName::<Identity, Impl, OFFSET>,
            EventsFileName: EventsFileName::<Identity, Impl, OFFSET>,
            SetEventsFileName: SetEventsFileName::<Identity, Impl, OFFSET>,
            Rules: Rules::<Identity, Impl, OFFSET>,
            SetRules: SetRules::<Identity, Impl, OFFSET>,
            Run: Run::<Identity, Impl, OFFSET>,
            Extract: Extract::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDataManager as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFolderAction_Impl, const OFFSET: isize>() -> IFolderAction_Vtbl {
        unsafe extern "system" fn Age<Identity: ::windows::core::IUnknownImpl, Impl: IFolderAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulage: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Age() {
                ::core::result::Result::Ok(ok__) => {
                    *pulage = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAge<Identity: ::windows::core::IUnknownImpl, Impl: IFolderAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulage: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAge(::core::mem::transmute_copy(&ulage)).into()
        }
        unsafe extern "system" fn Size<Identity: ::windows::core::IUnknownImpl, Impl: IFolderAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulage: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Size() {
                ::core::result::Result::Ok(ok__) => {
                    *pulage = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSize<Identity: ::windows::core::IUnknownImpl, Impl: IFolderAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulage: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSize(::core::mem::transmute_copy(&ulage)).into()
        }
        unsafe extern "system" fn Actions<Identity: ::windows::core::IUnknownImpl, Impl: IFolderAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, steps: *mut FolderActionSteps) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Actions() {
                ::core::result::Result::Ok(ok__) => {
                    *steps = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetActions<Identity: ::windows::core::IUnknownImpl, Impl: IFolderAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, steps: FolderActionSteps) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetActions(::core::mem::transmute_copy(&steps)).into()
        }
        unsafe extern "system" fn SendCabTo<Identity: ::windows::core::IUnknownImpl, Impl: IFolderAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdestination: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SendCabTo() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrdestination = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSendCabTo<Identity: ::windows::core::IUnknownImpl, Impl: IFolderAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdestination: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSendCabTo(::core::mem::transmute_copy(&bstrdestination)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Age: Age::<Identity, Impl, OFFSET>,
            SetAge: SetAge::<Identity, Impl, OFFSET>,
            Size: Size::<Identity, Impl, OFFSET>,
            SetSize: SetSize::<Identity, Impl, OFFSET>,
            Actions: Actions::<Identity, Impl, OFFSET>,
            SetActions: SetActions::<Identity, Impl, OFFSET>,
            SendCabTo: SendCabTo::<Identity, Impl, OFFSET>,
            SetSendCabTo: SetSendCabTo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFolderAction as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFolderActionCollection_Impl, const OFFSET: isize>() -> IFolderActionCollection_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: IFolderActionCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: IFolderActionCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, action: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *action = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: IFolderActionCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#enum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *r#enum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: ::windows::core::IUnknownImpl, Impl: IFolderActionCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, action: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Add(::core::mem::transmute(&action)).into()
        }
        unsafe extern "system" fn Remove<Identity: ::windows::core::IUnknownImpl, Impl: IFolderActionCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Remove(::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn Clear<Identity: ::windows::core::IUnknownImpl, Impl: IFolderActionCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Clear().into()
        }
        unsafe extern "system" fn AddRange<Identity: ::windows::core::IUnknownImpl, Impl: IFolderActionCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, actions: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddRange(::core::mem::transmute(&actions)).into()
        }
        unsafe extern "system" fn CreateFolderAction<Identity: ::windows::core::IUnknownImpl, Impl: IFolderActionCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, folderaction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateFolderAction() {
                ::core::result::Result::Ok(ok__) => {
                    *folderaction = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            Clear: Clear::<Identity, Impl, OFFSET>,
            AddRange: AddRange::<Identity, Impl, OFFSET>,
            CreateFolderAction: CreateFolderAction::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFolderActionCollection as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ILogFileItem_Impl: Sized {
    fn Path(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl ILogFileItem_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILogFileItem_Impl, const OFFSET: isize>() -> ILogFileItem_Vtbl {
        unsafe extern "system" fn Path<Identity: ::windows::core::IUnknownImpl, Impl: ILogFileItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstrvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Path() {
                ::core::result::Result::Ok(ok__) => {
                    *pstrvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), Path: Path::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILogFiles_Impl, const OFFSET: isize>() -> ILogFiles_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: ILogFiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plong: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *plong = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: ILogFiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppiunk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: ILogFiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppi: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppi = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: ::windows::core::IUnknownImpl, Impl: ILogFiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pathname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppi: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Add(::core::mem::transmute_copy(&pathname)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppi = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Identity: ::windows::core::IUnknownImpl, Impl: ILogFiles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Remove(::core::mem::transmute_copy(&index)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILogFiles as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPerformanceCounterDataCollector_Impl, const OFFSET: isize>() -> IPerformanceCounterDataCollector_Vtbl {
        unsafe extern "system" fn DataSourceName<Identity: ::windows::core::IUnknownImpl, Impl: IPerformanceCounterDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dsn: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DataSourceName() {
                ::core::result::Result::Ok(ok__) => {
                    *dsn = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDataSourceName<Identity: ::windows::core::IUnknownImpl, Impl: IPerformanceCounterDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dsn: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDataSourceName(::core::mem::transmute_copy(&dsn)).into()
        }
        unsafe extern "system" fn PerformanceCounters<Identity: ::windows::core::IUnknownImpl, Impl: IPerformanceCounterDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, counters: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PerformanceCounters() {
                ::core::result::Result::Ok(ok__) => {
                    *counters = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPerformanceCounters<Identity: ::windows::core::IUnknownImpl, Impl: IPerformanceCounterDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, counters: *const super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPerformanceCounters(::core::mem::transmute_copy(&counters)).into()
        }
        unsafe extern "system" fn LogFileFormat<Identity: ::windows::core::IUnknownImpl, Impl: IPerformanceCounterDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: *mut FileFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LogFileFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *format = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLogFileFormat<Identity: ::windows::core::IUnknownImpl, Impl: IPerformanceCounterDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: FileFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLogFileFormat(::core::mem::transmute_copy(&format)).into()
        }
        unsafe extern "system" fn SampleInterval<Identity: ::windows::core::IUnknownImpl, Impl: IPerformanceCounterDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SampleInterval() {
                ::core::result::Result::Ok(ok__) => {
                    *interval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSampleInterval<Identity: ::windows::core::IUnknownImpl, Impl: IPerformanceCounterDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interval: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSampleInterval(::core::mem::transmute_copy(&interval)).into()
        }
        unsafe extern "system" fn SegmentMaxRecords<Identity: ::windows::core::IUnknownImpl, Impl: IPerformanceCounterDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, records: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SegmentMaxRecords() {
                ::core::result::Result::Ok(ok__) => {
                    *records = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSegmentMaxRecords<Identity: ::windows::core::IUnknownImpl, Impl: IPerformanceCounterDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, records: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSegmentMaxRecords(::core::mem::transmute_copy(&records)).into()
        }
        Self {
            base: IDataCollector_Vtbl::new::<Identity, Impl, OFFSET>(),
            DataSourceName: DataSourceName::<Identity, Impl, OFFSET>,
            SetDataSourceName: SetDataSourceName::<Identity, Impl, OFFSET>,
            PerformanceCounters: PerformanceCounters::<Identity, Impl, OFFSET>,
            SetPerformanceCounters: SetPerformanceCounters::<Identity, Impl, OFFSET>,
            LogFileFormat: LogFileFormat::<Identity, Impl, OFFSET>,
            SetLogFileFormat: SetLogFileFormat::<Identity, Impl, OFFSET>,
            SampleInterval: SampleInterval::<Identity, Impl, OFFSET>,
            SetSampleInterval: SetSampleInterval::<Identity, Impl, OFFSET>,
            SegmentMaxRecords: SegmentMaxRecords::<Identity, Impl, OFFSET>,
            SetSegmentMaxRecords: SetSegmentMaxRecords::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPerformanceCounterDataCollector as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IDataCollector as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISchedule_Impl, const OFFSET: isize>() -> ISchedule_Vtbl {
        unsafe extern "system" fn StartDate<Identity: ::windows::core::IUnknownImpl, Impl: ISchedule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, start: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).StartDate() {
                ::core::result::Result::Ok(ok__) => {
                    *start = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartDate<Identity: ::windows::core::IUnknownImpl, Impl: ISchedule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, start: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetStartDate(::core::mem::transmute_copy(&start)).into()
        }
        unsafe extern "system" fn EndDate<Identity: ::windows::core::IUnknownImpl, Impl: ISchedule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, end: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EndDate() {
                ::core::result::Result::Ok(ok__) => {
                    *end = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEndDate<Identity: ::windows::core::IUnknownImpl, Impl: ISchedule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, end: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEndDate(::core::mem::transmute_copy(&end)).into()
        }
        unsafe extern "system" fn StartTime<Identity: ::windows::core::IUnknownImpl, Impl: ISchedule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, start: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).StartTime() {
                ::core::result::Result::Ok(ok__) => {
                    *start = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartTime<Identity: ::windows::core::IUnknownImpl, Impl: ISchedule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, start: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetStartTime(::core::mem::transmute_copy(&start)).into()
        }
        unsafe extern "system" fn Days<Identity: ::windows::core::IUnknownImpl, Impl: ISchedule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, days: *mut WeekDays) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Days() {
                ::core::result::Result::Ok(ok__) => {
                    *days = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDays<Identity: ::windows::core::IUnknownImpl, Impl: ISchedule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, days: WeekDays) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDays(::core::mem::transmute_copy(&days)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            StartDate: StartDate::<Identity, Impl, OFFSET>,
            SetStartDate: SetStartDate::<Identity, Impl, OFFSET>,
            EndDate: EndDate::<Identity, Impl, OFFSET>,
            SetEndDate: SetEndDate::<Identity, Impl, OFFSET>,
            StartTime: StartTime::<Identity, Impl, OFFSET>,
            SetStartTime: SetStartTime::<Identity, Impl, OFFSET>,
            Days: Days::<Identity, Impl, OFFSET>,
            SetDays: SetDays::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISchedule as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IScheduleCollection_Impl, const OFFSET: isize>() -> IScheduleCollection_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: IScheduleCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: IScheduleCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppschedule: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppschedule = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: IScheduleCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ienum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ienum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: ::windows::core::IUnknownImpl, Impl: IScheduleCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pschedule: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Add(::core::mem::transmute(&pschedule)).into()
        }
        unsafe extern "system" fn Remove<Identity: ::windows::core::IUnknownImpl, Impl: IScheduleCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vschedule: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Remove(::core::mem::transmute_copy(&vschedule)).into()
        }
        unsafe extern "system" fn Clear<Identity: ::windows::core::IUnknownImpl, Impl: IScheduleCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Clear().into()
        }
        unsafe extern "system" fn AddRange<Identity: ::windows::core::IUnknownImpl, Impl: IScheduleCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pschedules: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddRange(::core::mem::transmute(&pschedules)).into()
        }
        unsafe extern "system" fn CreateSchedule<Identity: ::windows::core::IUnknownImpl, Impl: IScheduleCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, schedule: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateSchedule() {
                ::core::result::Result::Ok(ok__) => {
                    *schedule = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            Clear: Clear::<Identity, Impl, OFFSET>,
            AddRange: AddRange::<Identity, Impl, OFFSET>,
            CreateSchedule: CreateSchedule::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IScheduleCollection as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor_Impl, const OFFSET: isize>() -> ISystemMonitor_Vtbl {
        unsafe extern "system" fn Appearance<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iappearance: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Appearance() {
                ::core::result::Result::Ok(ok__) => {
                    *iappearance = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAppearance<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iappearance: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAppearance(::core::mem::transmute_copy(&iappearance)).into()
        }
        unsafe extern "system" fn BackColor<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcolor: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BackColor() {
                ::core::result::Result::Ok(ok__) => {
                    *pcolor = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBackColor<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, color: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBackColor(::core::mem::transmute_copy(&color)).into()
        }
        unsafe extern "system" fn BorderStyle<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iborderstyle: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BorderStyle() {
                ::core::result::Result::Ok(ok__) => {
                    *iborderstyle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBorderStyle<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iborderstyle: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBorderStyle(::core::mem::transmute_copy(&iborderstyle)).into()
        }
        unsafe extern "system" fn ForeColor<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcolor: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ForeColor() {
                ::core::result::Result::Ok(ok__) => {
                    *pcolor = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetForeColor<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, color: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetForeColor(::core::mem::transmute_copy(&color)).into()
        }
        unsafe extern "system" fn Font<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfont: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Font() {
                ::core::result::Result::Ok(ok__) => {
                    *ppfont = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_Font<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfont: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putref_Font(::core::mem::transmute(&pfont)).into()
        }
        unsafe extern "system" fn Counters<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppicounters: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Counters() {
                ::core::result::Result::Ok(ok__) => {
                    *ppicounters = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShowVerticalGrid<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetShowVerticalGrid(::core::mem::transmute_copy(&bstate)).into()
        }
        unsafe extern "system" fn ShowVerticalGrid<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ShowVerticalGrid() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShowHorizontalGrid<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetShowHorizontalGrid(::core::mem::transmute_copy(&bstate)).into()
        }
        unsafe extern "system" fn ShowHorizontalGrid<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ShowHorizontalGrid() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShowLegend<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetShowLegend(::core::mem::transmute_copy(&bstate)).into()
        }
        unsafe extern "system" fn ShowLegend<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ShowLegend() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShowScaleLabels<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetShowScaleLabels(::core::mem::transmute_copy(&bstate)).into()
        }
        unsafe extern "system" fn ShowScaleLabels<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ShowScaleLabels() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShowValueBar<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetShowValueBar(::core::mem::transmute_copy(&bstate)).into()
        }
        unsafe extern "system" fn ShowValueBar<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ShowValueBar() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaximumScale<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ivalue: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMaximumScale(::core::mem::transmute_copy(&ivalue)).into()
        }
        unsafe extern "system" fn MaximumScale<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pivalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MaximumScale() {
                ::core::result::Result::Ok(ok__) => {
                    *pivalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinimumScale<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ivalue: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMinimumScale(::core::mem::transmute_copy(&ivalue)).into()
        }
        unsafe extern "system" fn MinimumScale<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pivalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MinimumScale() {
                ::core::result::Result::Ok(ok__) => {
                    *pivalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUpdateInterval<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fvalue: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetUpdateInterval(::core::mem::transmute_copy(&fvalue)).into()
        }
        unsafe extern "system" fn UpdateInterval<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfvalue: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).UpdateInterval() {
                ::core::result::Result::Ok(ok__) => {
                    *pfvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayType<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, edisplaytype: DisplayTypeConstants) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDisplayType(::core::mem::transmute_copy(&edisplaytype)).into()
        }
        unsafe extern "system" fn DisplayType<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pedisplaytype: *mut DisplayTypeConstants) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DisplayType() {
                ::core::result::Result::Ok(ok__) => {
                    *pedisplaytype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetManualUpdate<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetManualUpdate(::core::mem::transmute_copy(&bstate)).into()
        }
        unsafe extern "system" fn ManualUpdate<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ManualUpdate() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGraphTitle<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstitle: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetGraphTitle(::core::mem::transmute_copy(&bstitle)).into()
        }
        unsafe extern "system" fn GraphTitle<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstitle: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GraphTitle() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstitle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetYAxisLabel<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstitle: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetYAxisLabel(::core::mem::transmute_copy(&bstitle)).into()
        }
        unsafe extern "system" fn YAxisLabel<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstitle: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).YAxisLabel() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstitle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CollectSample<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CollectSample().into()
        }
        unsafe extern "system" fn UpdateGraph<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UpdateGraph().into()
        }
        unsafe extern "system" fn BrowseCounters<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BrowseCounters().into()
        }
        unsafe extern "system" fn DisplayProperties<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DisplayProperties().into()
        }
        unsafe extern "system" fn Counter<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iindex: i32, ppicounter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Counter(::core::mem::transmute_copy(&iindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppicounter = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddCounter<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bspath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppicounter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AddCounter(::core::mem::transmute_copy(&bspath)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppicounter = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteCounter<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pctr: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteCounter(::core::mem::transmute(&pctr)).into()
        }
        unsafe extern "system" fn BackColorCtl<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcolor: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BackColorCtl() {
                ::core::result::Result::Ok(ok__) => {
                    *pcolor = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBackColorCtl<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, color: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBackColorCtl(::core::mem::transmute_copy(&color)).into()
        }
        unsafe extern "system" fn SetLogFileName<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bsfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLogFileName(::core::mem::transmute_copy(&bsfilename)).into()
        }
        unsafe extern "system" fn LogFileName<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bsfilename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LogFileName() {
                ::core::result::Result::Ok(ok__) => {
                    *bsfilename = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLogViewStart<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, starttime: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLogViewStart(::core::mem::transmute_copy(&starttime)).into()
        }
        unsafe extern "system" fn LogViewStart<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, starttime: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LogViewStart() {
                ::core::result::Result::Ok(ok__) => {
                    *starttime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLogViewStop<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stoptime: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLogViewStop(::core::mem::transmute_copy(&stoptime)).into()
        }
        unsafe extern "system" fn LogViewStop<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stoptime: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LogViewStop() {
                ::core::result::Result::Ok(ok__) => {
                    *stoptime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GridColor<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcolor: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GridColor() {
                ::core::result::Result::Ok(ok__) => {
                    *pcolor = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGridColor<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, color: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetGridColor(::core::mem::transmute_copy(&color)).into()
        }
        unsafe extern "system" fn TimeBarColor<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcolor: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TimeBarColor() {
                ::core::result::Result::Ok(ok__) => {
                    *pcolor = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTimeBarColor<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, color: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTimeBarColor(::core::mem::transmute_copy(&color)).into()
        }
        unsafe extern "system" fn Highlight<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Highlight() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHighlight<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetHighlight(::core::mem::transmute_copy(&bstate)).into()
        }
        unsafe extern "system" fn ShowToolbar<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ShowToolbar() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShowToolbar<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetShowToolbar(::core::mem::transmute_copy(&bstate)).into()
        }
        unsafe extern "system" fn Paste<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Paste().into()
        }
        unsafe extern "system" fn Copy<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Copy().into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn SetReadOnly<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetReadOnly(::core::mem::transmute_copy(&bstate)).into()
        }
        unsafe extern "system" fn ReadOnly<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ReadOnly() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReportValueType<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ereportvaluetype: ReportValueTypeConstants) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetReportValueType(::core::mem::transmute_copy(&ereportvaluetype)).into()
        }
        unsafe extern "system" fn ReportValueType<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pereportvaluetype: *mut ReportValueTypeConstants) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ReportValueType() {
                ::core::result::Result::Ok(ok__) => {
                    *pereportvaluetype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMonitorDuplicateInstances<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMonitorDuplicateInstances(::core::mem::transmute_copy(&bstate)).into()
        }
        unsafe extern "system" fn MonitorDuplicateInstances<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MonitorDuplicateInstances() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayFilter<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ivalue: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDisplayFilter(::core::mem::transmute_copy(&ivalue)).into()
        }
        unsafe extern "system" fn DisplayFilter<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pivalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DisplayFilter() {
                ::core::result::Result::Ok(ok__) => {
                    *pivalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LogFiles<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppilogfiles: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LogFiles() {
                ::core::result::Result::Ok(ok__) => {
                    *ppilogfiles = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDataSourceType<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, edatasourcetype: DataSourceTypeConstants) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDataSourceType(::core::mem::transmute_copy(&edatasourcetype)).into()
        }
        unsafe extern "system" fn DataSourceType<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pedatasourcetype: *mut DataSourceTypeConstants) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DataSourceType() {
                ::core::result::Result::Ok(ok__) => {
                    *pedatasourcetype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSqlDsnName<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bssqldsnname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSqlDsnName(::core::mem::transmute_copy(&bssqldsnname)).into()
        }
        unsafe extern "system" fn SqlDsnName<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bssqldsnname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SqlDsnName() {
                ::core::result::Result::Ok(ok__) => {
                    *bssqldsnname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSqlLogSetName<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bssqllogsetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSqlLogSetName(::core::mem::transmute_copy(&bssqllogsetname)).into()
        }
        unsafe extern "system" fn SqlLogSetName<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bssqllogsetname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SqlLogSetName() {
                ::core::result::Result::Ok(ok__) => {
                    *bssqllogsetname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Appearance: Appearance::<Identity, Impl, OFFSET>,
            SetAppearance: SetAppearance::<Identity, Impl, OFFSET>,
            BackColor: BackColor::<Identity, Impl, OFFSET>,
            SetBackColor: SetBackColor::<Identity, Impl, OFFSET>,
            BorderStyle: BorderStyle::<Identity, Impl, OFFSET>,
            SetBorderStyle: SetBorderStyle::<Identity, Impl, OFFSET>,
            ForeColor: ForeColor::<Identity, Impl, OFFSET>,
            SetForeColor: SetForeColor::<Identity, Impl, OFFSET>,
            Font: Font::<Identity, Impl, OFFSET>,
            putref_Font: putref_Font::<Identity, Impl, OFFSET>,
            Counters: Counters::<Identity, Impl, OFFSET>,
            SetShowVerticalGrid: SetShowVerticalGrid::<Identity, Impl, OFFSET>,
            ShowVerticalGrid: ShowVerticalGrid::<Identity, Impl, OFFSET>,
            SetShowHorizontalGrid: SetShowHorizontalGrid::<Identity, Impl, OFFSET>,
            ShowHorizontalGrid: ShowHorizontalGrid::<Identity, Impl, OFFSET>,
            SetShowLegend: SetShowLegend::<Identity, Impl, OFFSET>,
            ShowLegend: ShowLegend::<Identity, Impl, OFFSET>,
            SetShowScaleLabels: SetShowScaleLabels::<Identity, Impl, OFFSET>,
            ShowScaleLabels: ShowScaleLabels::<Identity, Impl, OFFSET>,
            SetShowValueBar: SetShowValueBar::<Identity, Impl, OFFSET>,
            ShowValueBar: ShowValueBar::<Identity, Impl, OFFSET>,
            SetMaximumScale: SetMaximumScale::<Identity, Impl, OFFSET>,
            MaximumScale: MaximumScale::<Identity, Impl, OFFSET>,
            SetMinimumScale: SetMinimumScale::<Identity, Impl, OFFSET>,
            MinimumScale: MinimumScale::<Identity, Impl, OFFSET>,
            SetUpdateInterval: SetUpdateInterval::<Identity, Impl, OFFSET>,
            UpdateInterval: UpdateInterval::<Identity, Impl, OFFSET>,
            SetDisplayType: SetDisplayType::<Identity, Impl, OFFSET>,
            DisplayType: DisplayType::<Identity, Impl, OFFSET>,
            SetManualUpdate: SetManualUpdate::<Identity, Impl, OFFSET>,
            ManualUpdate: ManualUpdate::<Identity, Impl, OFFSET>,
            SetGraphTitle: SetGraphTitle::<Identity, Impl, OFFSET>,
            GraphTitle: GraphTitle::<Identity, Impl, OFFSET>,
            SetYAxisLabel: SetYAxisLabel::<Identity, Impl, OFFSET>,
            YAxisLabel: YAxisLabel::<Identity, Impl, OFFSET>,
            CollectSample: CollectSample::<Identity, Impl, OFFSET>,
            UpdateGraph: UpdateGraph::<Identity, Impl, OFFSET>,
            BrowseCounters: BrowseCounters::<Identity, Impl, OFFSET>,
            DisplayProperties: DisplayProperties::<Identity, Impl, OFFSET>,
            Counter: Counter::<Identity, Impl, OFFSET>,
            AddCounter: AddCounter::<Identity, Impl, OFFSET>,
            DeleteCounter: DeleteCounter::<Identity, Impl, OFFSET>,
            BackColorCtl: BackColorCtl::<Identity, Impl, OFFSET>,
            SetBackColorCtl: SetBackColorCtl::<Identity, Impl, OFFSET>,
            SetLogFileName: SetLogFileName::<Identity, Impl, OFFSET>,
            LogFileName: LogFileName::<Identity, Impl, OFFSET>,
            SetLogViewStart: SetLogViewStart::<Identity, Impl, OFFSET>,
            LogViewStart: LogViewStart::<Identity, Impl, OFFSET>,
            SetLogViewStop: SetLogViewStop::<Identity, Impl, OFFSET>,
            LogViewStop: LogViewStop::<Identity, Impl, OFFSET>,
            GridColor: GridColor::<Identity, Impl, OFFSET>,
            SetGridColor: SetGridColor::<Identity, Impl, OFFSET>,
            TimeBarColor: TimeBarColor::<Identity, Impl, OFFSET>,
            SetTimeBarColor: SetTimeBarColor::<Identity, Impl, OFFSET>,
            Highlight: Highlight::<Identity, Impl, OFFSET>,
            SetHighlight: SetHighlight::<Identity, Impl, OFFSET>,
            ShowToolbar: ShowToolbar::<Identity, Impl, OFFSET>,
            SetShowToolbar: SetShowToolbar::<Identity, Impl, OFFSET>,
            Paste: Paste::<Identity, Impl, OFFSET>,
            Copy: Copy::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            SetReadOnly: SetReadOnly::<Identity, Impl, OFFSET>,
            ReadOnly: ReadOnly::<Identity, Impl, OFFSET>,
            SetReportValueType: SetReportValueType::<Identity, Impl, OFFSET>,
            ReportValueType: ReportValueType::<Identity, Impl, OFFSET>,
            SetMonitorDuplicateInstances: SetMonitorDuplicateInstances::<Identity, Impl, OFFSET>,
            MonitorDuplicateInstances: MonitorDuplicateInstances::<Identity, Impl, OFFSET>,
            SetDisplayFilter: SetDisplayFilter::<Identity, Impl, OFFSET>,
            DisplayFilter: DisplayFilter::<Identity, Impl, OFFSET>,
            LogFiles: LogFiles::<Identity, Impl, OFFSET>,
            SetDataSourceType: SetDataSourceType::<Identity, Impl, OFFSET>,
            DataSourceType: DataSourceType::<Identity, Impl, OFFSET>,
            SetSqlDsnName: SetSqlDsnName::<Identity, Impl, OFFSET>,
            SqlDsnName: SqlDsnName::<Identity, Impl, OFFSET>,
            SetSqlLogSetName: SetSqlLogSetName::<Identity, Impl, OFFSET>,
            SqlLogSetName: SqlLogSetName::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor2_Impl, const OFFSET: isize>() -> ISystemMonitor2_Vtbl {
        unsafe extern "system" fn SetEnableDigitGrouping<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEnableDigitGrouping(::core::mem::transmute_copy(&bstate)).into()
        }
        unsafe extern "system" fn EnableDigitGrouping<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnableDigitGrouping() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnableToolTips<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEnableToolTips(::core::mem::transmute_copy(&bstate)).into()
        }
        unsafe extern "system" fn EnableToolTips<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnableToolTips() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShowTimeAxisLabels<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetShowTimeAxisLabels(::core::mem::transmute_copy(&bstate)).into()
        }
        unsafe extern "system" fn ShowTimeAxisLabels<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ShowTimeAxisLabels() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChartScroll<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bscroll: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetChartScroll(::core::mem::transmute_copy(&bscroll)).into()
        }
        unsafe extern "system" fn ChartScroll<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbscroll: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ChartScroll() {
                ::core::result::Result::Ok(ok__) => {
                    *pbscroll = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDataPointCount<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inewcount: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDataPointCount(::core::mem::transmute_copy(&inewcount)).into()
        }
        unsafe extern "system" fn DataPointCount<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidatapointcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DataPointCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pidatapointcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScaleToFit<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bselectedcountersonly: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ScaleToFit(::core::mem::transmute_copy(&bselectedcountersonly)).into()
        }
        unsafe extern "system" fn SaveAs<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, esysmonfiletype: SysmonFileType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SaveAs(::core::mem::transmute_copy(&bstrfilename), ::core::mem::transmute_copy(&esysmonfiletype)).into()
        }
        unsafe extern "system" fn Relog<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, esysmonfiletype: SysmonFileType, ifilter: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Relog(::core::mem::transmute_copy(&bstrfilename), ::core::mem::transmute_copy(&esysmonfiletype), ::core::mem::transmute_copy(&ifilter)).into()
        }
        unsafe extern "system" fn ClearData<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ClearData().into()
        }
        unsafe extern "system" fn LogSourceStartTime<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdate: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LogSourceStartTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pdate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LogSourceStopTime<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdate: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LogSourceStopTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pdate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLogViewRange<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, starttime: f64, stoptime: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLogViewRange(::core::mem::transmute_copy(&starttime), ::core::mem::transmute_copy(&stoptime)).into()
        }
        unsafe extern "system" fn GetLogViewRange<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, starttime: *mut f64, stoptime: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetLogViewRange(::core::mem::transmute_copy(&starttime), ::core::mem::transmute_copy(&stoptime)).into()
        }
        unsafe extern "system" fn BatchingLock<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flock: i16, ebatchreason: SysmonBatchReason) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BatchingLock(::core::mem::transmute_copy(&flock), ::core::mem::transmute_copy(&ebatchreason)).into()
        }
        unsafe extern "system" fn LoadSettings<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitor2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsettingfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).LoadSettings(::core::mem::transmute_copy(&bstrsettingfilename)).into()
        }
        Self {
            base: ISystemMonitor_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetEnableDigitGrouping: SetEnableDigitGrouping::<Identity, Impl, OFFSET>,
            EnableDigitGrouping: EnableDigitGrouping::<Identity, Impl, OFFSET>,
            SetEnableToolTips: SetEnableToolTips::<Identity, Impl, OFFSET>,
            EnableToolTips: EnableToolTips::<Identity, Impl, OFFSET>,
            SetShowTimeAxisLabels: SetShowTimeAxisLabels::<Identity, Impl, OFFSET>,
            ShowTimeAxisLabels: ShowTimeAxisLabels::<Identity, Impl, OFFSET>,
            SetChartScroll: SetChartScroll::<Identity, Impl, OFFSET>,
            ChartScroll: ChartScroll::<Identity, Impl, OFFSET>,
            SetDataPointCount: SetDataPointCount::<Identity, Impl, OFFSET>,
            DataPointCount: DataPointCount::<Identity, Impl, OFFSET>,
            ScaleToFit: ScaleToFit::<Identity, Impl, OFFSET>,
            SaveAs: SaveAs::<Identity, Impl, OFFSET>,
            Relog: Relog::<Identity, Impl, OFFSET>,
            ClearData: ClearData::<Identity, Impl, OFFSET>,
            LogSourceStartTime: LogSourceStartTime::<Identity, Impl, OFFSET>,
            LogSourceStopTime: LogSourceStopTime::<Identity, Impl, OFFSET>,
            SetLogViewRange: SetLogViewRange::<Identity, Impl, OFFSET>,
            GetLogViewRange: GetLogViewRange::<Identity, Impl, OFFSET>,
            BatchingLock: BatchingLock::<Identity, Impl, OFFSET>,
            LoadSettings: LoadSettings::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISystemMonitor2 as ::windows::core::Interface>::IID || iid == &<ISystemMonitor as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitorEvents_Impl, const OFFSET: isize>() -> ISystemMonitorEvents_Vtbl {
        unsafe extern "system" fn OnCounterSelected<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitorEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnCounterSelected(::core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn OnCounterAdded<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitorEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnCounterAdded(::core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn OnCounterDeleted<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitorEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnCounterDeleted(::core::mem::transmute_copy(&index))
        }
        unsafe extern "system" fn OnSampleCollected<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitorEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnSampleCollected()
        }
        unsafe extern "system" fn OnDblClick<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMonitorEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnDblClick(::core::mem::transmute_copy(&index))
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            OnCounterSelected: OnCounterSelected::<Identity, Impl, OFFSET>,
            OnCounterAdded: OnCounterAdded::<Identity, Impl, OFFSET>,
            OnCounterDeleted: OnCounterDeleted::<Identity, Impl, OFFSET>,
            OnSampleCollected: OnSampleCollected::<Identity, Impl, OFFSET>,
            OnDblClick: OnDblClick::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITraceDataCollector_Impl, const OFFSET: isize>() -> ITraceDataCollector_Vtbl {
        unsafe extern "system" fn BufferSize<Identity: ::windows::core::IUnknownImpl, Impl: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, size: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BufferSize() {
                ::core::result::Result::Ok(ok__) => {
                    *size = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBufferSize<Identity: ::windows::core::IUnknownImpl, Impl: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, size: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBufferSize(::core::mem::transmute_copy(&size)).into()
        }
        unsafe extern "system" fn BuffersLost<Identity: ::windows::core::IUnknownImpl, Impl: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffers: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BuffersLost() {
                ::core::result::Result::Ok(ok__) => {
                    *buffers = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBuffersLost<Identity: ::windows::core::IUnknownImpl, Impl: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffers: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBuffersLost(::core::mem::transmute_copy(&buffers)).into()
        }
        unsafe extern "system" fn BuffersWritten<Identity: ::windows::core::IUnknownImpl, Impl: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffers: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BuffersWritten() {
                ::core::result::Result::Ok(ok__) => {
                    *buffers = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBuffersWritten<Identity: ::windows::core::IUnknownImpl, Impl: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffers: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBuffersWritten(::core::mem::transmute_copy(&buffers)).into()
        }
        unsafe extern "system" fn ClockType<Identity: ::windows::core::IUnknownImpl, Impl: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clock: *mut ClockType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ClockType() {
                ::core::result::Result::Ok(ok__) => {
                    *clock = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClockType<Identity: ::windows::core::IUnknownImpl, Impl: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clock: ClockType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetClockType(::core::mem::transmute_copy(&clock)).into()
        }
        unsafe extern "system" fn EventsLost<Identity: ::windows::core::IUnknownImpl, Impl: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, events: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EventsLost() {
                ::core::result::Result::Ok(ok__) => {
                    *events = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventsLost<Identity: ::windows::core::IUnknownImpl, Impl: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, events: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEventsLost(::core::mem::transmute_copy(&events)).into()
        }
        unsafe extern "system" fn ExtendedModes<Identity: ::windows::core::IUnknownImpl, Impl: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ExtendedModes() {
                ::core::result::Result::Ok(ok__) => {
                    *mode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExtendedModes<Identity: ::windows::core::IUnknownImpl, Impl: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetExtendedModes(::core::mem::transmute_copy(&mode)).into()
        }
        unsafe extern "system" fn FlushTimer<Identity: ::windows::core::IUnknownImpl, Impl: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, seconds: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FlushTimer() {
                ::core::result::Result::Ok(ok__) => {
                    *seconds = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFlushTimer<Identity: ::windows::core::IUnknownImpl, Impl: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, seconds: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFlushTimer(::core::mem::transmute_copy(&seconds)).into()
        }
        unsafe extern "system" fn FreeBuffers<Identity: ::windows::core::IUnknownImpl, Impl: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffers: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FreeBuffers() {
                ::core::result::Result::Ok(ok__) => {
                    *buffers = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFreeBuffers<Identity: ::windows::core::IUnknownImpl, Impl: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffers: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFreeBuffers(::core::mem::transmute_copy(&buffers)).into()
        }
        unsafe extern "system" fn Guid<Identity: ::windows::core::IUnknownImpl, Impl: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Guid() {
                ::core::result::Result::Ok(ok__) => {
                    *guid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGuid<Identity: ::windows::core::IUnknownImpl, Impl: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetGuid(::core::mem::transmute_copy(&guid)).into()
        }
        unsafe extern "system" fn IsKernelTrace<Identity: ::windows::core::IUnknownImpl, Impl: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, kernel: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsKernelTrace() {
                ::core::result::Result::Ok(ok__) => {
                    *kernel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaximumBuffers<Identity: ::windows::core::IUnknownImpl, Impl: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffers: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MaximumBuffers() {
                ::core::result::Result::Ok(ok__) => {
                    *buffers = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaximumBuffers<Identity: ::windows::core::IUnknownImpl, Impl: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffers: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMaximumBuffers(::core::mem::transmute_copy(&buffers)).into()
        }
        unsafe extern "system" fn MinimumBuffers<Identity: ::windows::core::IUnknownImpl, Impl: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffers: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MinimumBuffers() {
                ::core::result::Result::Ok(ok__) => {
                    *buffers = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinimumBuffers<Identity: ::windows::core::IUnknownImpl, Impl: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffers: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMinimumBuffers(::core::mem::transmute_copy(&buffers)).into()
        }
        unsafe extern "system" fn NumberOfBuffers<Identity: ::windows::core::IUnknownImpl, Impl: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffers: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).NumberOfBuffers() {
                ::core::result::Result::Ok(ok__) => {
                    *buffers = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNumberOfBuffers<Identity: ::windows::core::IUnknownImpl, Impl: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffers: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetNumberOfBuffers(::core::mem::transmute_copy(&buffers)).into()
        }
        unsafe extern "system" fn PreallocateFile<Identity: ::windows::core::IUnknownImpl, Impl: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allocate: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PreallocateFile() {
                ::core::result::Result::Ok(ok__) => {
                    *allocate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPreallocateFile<Identity: ::windows::core::IUnknownImpl, Impl: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allocate: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPreallocateFile(::core::mem::transmute_copy(&allocate)).into()
        }
        unsafe extern "system" fn ProcessMode<Identity: ::windows::core::IUnknownImpl, Impl: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, process: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ProcessMode() {
                ::core::result::Result::Ok(ok__) => {
                    *process = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProcessMode<Identity: ::windows::core::IUnknownImpl, Impl: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, process: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetProcessMode(::core::mem::transmute_copy(&process)).into()
        }
        unsafe extern "system" fn RealTimeBuffersLost<Identity: ::windows::core::IUnknownImpl, Impl: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffers: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RealTimeBuffersLost() {
                ::core::result::Result::Ok(ok__) => {
                    *buffers = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRealTimeBuffersLost<Identity: ::windows::core::IUnknownImpl, Impl: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffers: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRealTimeBuffersLost(::core::mem::transmute_copy(&buffers)).into()
        }
        unsafe extern "system" fn SessionId<Identity: ::windows::core::IUnknownImpl, Impl: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SessionId() {
                ::core::result::Result::Ok(ok__) => {
                    *id = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSessionId<Identity: ::windows::core::IUnknownImpl, Impl: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSessionId(::core::mem::transmute_copy(&id)).into()
        }
        unsafe extern "system" fn SessionName<Identity: ::windows::core::IUnknownImpl, Impl: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SessionName() {
                ::core::result::Result::Ok(ok__) => {
                    *name = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSessionName<Identity: ::windows::core::IUnknownImpl, Impl: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSessionName(::core::mem::transmute_copy(&name)).into()
        }
        unsafe extern "system" fn SessionThreadId<Identity: ::windows::core::IUnknownImpl, Impl: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SessionThreadId() {
                ::core::result::Result::Ok(ok__) => {
                    *tid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSessionThreadId<Identity: ::windows::core::IUnknownImpl, Impl: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSessionThreadId(::core::mem::transmute_copy(&tid)).into()
        }
        unsafe extern "system" fn StreamMode<Identity: ::windows::core::IUnknownImpl, Impl: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: *mut StreamMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).StreamMode() {
                ::core::result::Result::Ok(ok__) => {
                    *mode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStreamMode<Identity: ::windows::core::IUnknownImpl, Impl: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: StreamMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetStreamMode(::core::mem::transmute_copy(&mode)).into()
        }
        unsafe extern "system" fn TraceDataProviders<Identity: ::windows::core::IUnknownImpl, Impl: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, providers: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TraceDataProviders() {
                ::core::result::Result::Ok(ok__) => {
                    *providers = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDataCollector_Vtbl::new::<Identity, Impl, OFFSET>(),
            BufferSize: BufferSize::<Identity, Impl, OFFSET>,
            SetBufferSize: SetBufferSize::<Identity, Impl, OFFSET>,
            BuffersLost: BuffersLost::<Identity, Impl, OFFSET>,
            SetBuffersLost: SetBuffersLost::<Identity, Impl, OFFSET>,
            BuffersWritten: BuffersWritten::<Identity, Impl, OFFSET>,
            SetBuffersWritten: SetBuffersWritten::<Identity, Impl, OFFSET>,
            ClockType: ClockType::<Identity, Impl, OFFSET>,
            SetClockType: SetClockType::<Identity, Impl, OFFSET>,
            EventsLost: EventsLost::<Identity, Impl, OFFSET>,
            SetEventsLost: SetEventsLost::<Identity, Impl, OFFSET>,
            ExtendedModes: ExtendedModes::<Identity, Impl, OFFSET>,
            SetExtendedModes: SetExtendedModes::<Identity, Impl, OFFSET>,
            FlushTimer: FlushTimer::<Identity, Impl, OFFSET>,
            SetFlushTimer: SetFlushTimer::<Identity, Impl, OFFSET>,
            FreeBuffers: FreeBuffers::<Identity, Impl, OFFSET>,
            SetFreeBuffers: SetFreeBuffers::<Identity, Impl, OFFSET>,
            Guid: Guid::<Identity, Impl, OFFSET>,
            SetGuid: SetGuid::<Identity, Impl, OFFSET>,
            IsKernelTrace: IsKernelTrace::<Identity, Impl, OFFSET>,
            MaximumBuffers: MaximumBuffers::<Identity, Impl, OFFSET>,
            SetMaximumBuffers: SetMaximumBuffers::<Identity, Impl, OFFSET>,
            MinimumBuffers: MinimumBuffers::<Identity, Impl, OFFSET>,
            SetMinimumBuffers: SetMinimumBuffers::<Identity, Impl, OFFSET>,
            NumberOfBuffers: NumberOfBuffers::<Identity, Impl, OFFSET>,
            SetNumberOfBuffers: SetNumberOfBuffers::<Identity, Impl, OFFSET>,
            PreallocateFile: PreallocateFile::<Identity, Impl, OFFSET>,
            SetPreallocateFile: SetPreallocateFile::<Identity, Impl, OFFSET>,
            ProcessMode: ProcessMode::<Identity, Impl, OFFSET>,
            SetProcessMode: SetProcessMode::<Identity, Impl, OFFSET>,
            RealTimeBuffersLost: RealTimeBuffersLost::<Identity, Impl, OFFSET>,
            SetRealTimeBuffersLost: SetRealTimeBuffersLost::<Identity, Impl, OFFSET>,
            SessionId: SessionId::<Identity, Impl, OFFSET>,
            SetSessionId: SetSessionId::<Identity, Impl, OFFSET>,
            SessionName: SessionName::<Identity, Impl, OFFSET>,
            SetSessionName: SetSessionName::<Identity, Impl, OFFSET>,
            SessionThreadId: SessionThreadId::<Identity, Impl, OFFSET>,
            SetSessionThreadId: SetSessionThreadId::<Identity, Impl, OFFSET>,
            StreamMode: StreamMode::<Identity, Impl, OFFSET>,
            SetStreamMode: SetStreamMode::<Identity, Impl, OFFSET>,
            TraceDataProviders: TraceDataProviders::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITraceDataCollector as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IDataCollector as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITraceDataProvider_Impl, const OFFSET: isize>() -> ITraceDataProvider_Vtbl {
        unsafe extern "system" fn DisplayName<Identity: ::windows::core::IUnknownImpl, Impl: ITraceDataProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *name = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayName<Identity: ::windows::core::IUnknownImpl, Impl: ITraceDataProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDisplayName(::core::mem::transmute_copy(&name)).into()
        }
        unsafe extern "system" fn Guid<Identity: ::windows::core::IUnknownImpl, Impl: ITraceDataProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Guid() {
                ::core::result::Result::Ok(ok__) => {
                    *guid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGuid<Identity: ::windows::core::IUnknownImpl, Impl: ITraceDataProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetGuid(::core::mem::transmute_copy(&guid)).into()
        }
        unsafe extern "system" fn Level<Identity: ::windows::core::IUnknownImpl, Impl: ITraceDataProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplevel: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Level() {
                ::core::result::Result::Ok(ok__) => {
                    *pplevel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeywordsAny<Identity: ::windows::core::IUnknownImpl, Impl: ITraceDataProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppkeywords: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).KeywordsAny() {
                ::core::result::Result::Ok(ok__) => {
                    *ppkeywords = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeywordsAll<Identity: ::windows::core::IUnknownImpl, Impl: ITraceDataProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppkeywords: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).KeywordsAll() {
                ::core::result::Result::Ok(ok__) => {
                    *ppkeywords = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Identity: ::windows::core::IUnknownImpl, Impl: ITraceDataProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FilterEnabled<Identity: ::windows::core::IUnknownImpl, Impl: ITraceDataProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filterenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FilterEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *filterenabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFilterEnabled<Identity: ::windows::core::IUnknownImpl, Impl: ITraceDataProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filterenabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFilterEnabled(::core::mem::transmute_copy(&filterenabled)).into()
        }
        unsafe extern "system" fn FilterType<Identity: ::windows::core::IUnknownImpl, Impl: ITraceDataProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pultype: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FilterType() {
                ::core::result::Result::Ok(ok__) => {
                    *pultype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFilterType<Identity: ::windows::core::IUnknownImpl, Impl: ITraceDataProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ultype: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFilterType(::core::mem::transmute_copy(&ultype)).into()
        }
        unsafe extern "system" fn FilterData<Identity: ::windows::core::IUnknownImpl, Impl: ITraceDataProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdata: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FilterData() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFilterData<Identity: ::windows::core::IUnknownImpl, Impl: ITraceDataProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *const super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFilterData(::core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn Query<Identity: ::windows::core::IUnknownImpl, Impl: ITraceDataProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrserver: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Query(::core::mem::transmute_copy(&bstrname), ::core::mem::transmute_copy(&bstrserver)).into()
        }
        unsafe extern "system" fn Resolve<Identity: ::windows::core::IUnknownImpl, Impl: ITraceDataProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfrom: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Resolve(::core::mem::transmute(&pfrom)).into()
        }
        unsafe extern "system" fn SetSecurity<Identity: ::windows::core::IUnknownImpl, Impl: ITraceDataProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sddl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSecurity(::core::mem::transmute_copy(&sddl)).into()
        }
        unsafe extern "system" fn GetSecurity<Identity: ::windows::core::IUnknownImpl, Impl: ITraceDataProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, securityinfo: u32, sddl: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSecurity(::core::mem::transmute_copy(&securityinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *sddl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRegisteredProcesses<Identity: ::windows::core::IUnknownImpl, Impl: ITraceDataProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, processes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetRegisteredProcesses() {
                ::core::result::Result::Ok(ok__) => {
                    *processes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            DisplayName: DisplayName::<Identity, Impl, OFFSET>,
            SetDisplayName: SetDisplayName::<Identity, Impl, OFFSET>,
            Guid: Guid::<Identity, Impl, OFFSET>,
            SetGuid: SetGuid::<Identity, Impl, OFFSET>,
            Level: Level::<Identity, Impl, OFFSET>,
            KeywordsAny: KeywordsAny::<Identity, Impl, OFFSET>,
            KeywordsAll: KeywordsAll::<Identity, Impl, OFFSET>,
            Properties: Properties::<Identity, Impl, OFFSET>,
            FilterEnabled: FilterEnabled::<Identity, Impl, OFFSET>,
            SetFilterEnabled: SetFilterEnabled::<Identity, Impl, OFFSET>,
            FilterType: FilterType::<Identity, Impl, OFFSET>,
            SetFilterType: SetFilterType::<Identity, Impl, OFFSET>,
            FilterData: FilterData::<Identity, Impl, OFFSET>,
            SetFilterData: SetFilterData::<Identity, Impl, OFFSET>,
            Query: Query::<Identity, Impl, OFFSET>,
            Resolve: Resolve::<Identity, Impl, OFFSET>,
            SetSecurity: SetSecurity::<Identity, Impl, OFFSET>,
            GetSecurity: GetSecurity::<Identity, Impl, OFFSET>,
            GetRegisteredProcesses: GetRegisteredProcesses::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITraceDataProvider as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITraceDataProviderCollection_Impl, const OFFSET: isize>() -> ITraceDataProviderCollection_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: ITraceDataProviderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: ITraceDataProviderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppprovider: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppprovider = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: ITraceDataProviderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: ::windows::core::IUnknownImpl, Impl: ITraceDataProviderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprovider: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Add(::core::mem::transmute(&pprovider)).into()
        }
        unsafe extern "system" fn Remove<Identity: ::windows::core::IUnknownImpl, Impl: ITraceDataProviderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vprovider: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Remove(::core::mem::transmute_copy(&vprovider)).into()
        }
        unsafe extern "system" fn Clear<Identity: ::windows::core::IUnknownImpl, Impl: ITraceDataProviderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Clear().into()
        }
        unsafe extern "system" fn AddRange<Identity: ::windows::core::IUnknownImpl, Impl: ITraceDataProviderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, providers: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddRange(::core::mem::transmute(&providers)).into()
        }
        unsafe extern "system" fn CreateTraceDataProvider<Identity: ::windows::core::IUnknownImpl, Impl: ITraceDataProviderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, provider: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateTraceDataProvider() {
                ::core::result::Result::Ok(ok__) => {
                    *provider = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTraceDataProviders<Identity: ::windows::core::IUnknownImpl, Impl: ITraceDataProviderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, server: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetTraceDataProviders(::core::mem::transmute_copy(&server)).into()
        }
        unsafe extern "system" fn GetTraceDataProvidersByProcess<Identity: ::windows::core::IUnknownImpl, Impl: ITraceDataProviderCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, server: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetTraceDataProvidersByProcess(::core::mem::transmute_copy(&server), ::core::mem::transmute_copy(&pid)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            Clear: Clear::<Identity, Impl, OFFSET>,
            AddRange: AddRange::<Identity, Impl, OFFSET>,
            CreateTraceDataProvider: CreateTraceDataProvider::<Identity, Impl, OFFSET>,
            GetTraceDataProviders: GetTraceDataProviders::<Identity, Impl, OFFSET>,
            GetTraceDataProvidersByProcess: GetTraceDataProvidersByProcess::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITraceDataProviderCollection as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IValueMap_Impl, const OFFSET: isize>() -> IValueMap_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: IValueMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: IValueMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: IValueMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Identity: ::windows::core::IUnknownImpl, Impl: IValueMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, description: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *description = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows::core::IUnknownImpl, Impl: IValueMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, description: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDescription(::core::mem::transmute_copy(&description)).into()
        }
        unsafe extern "system" fn Value<Identity: ::windows::core::IUnknownImpl, Impl: IValueMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Identity: ::windows::core::IUnknownImpl, Impl: IValueMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetValue(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn ValueMapType<Identity: ::windows::core::IUnknownImpl, Impl: IValueMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: *mut ValueMapType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ValueMapType() {
                ::core::result::Result::Ok(ok__) => {
                    *r#type = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValueMapType<Identity: ::windows::core::IUnknownImpl, Impl: IValueMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: ValueMapType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetValueMapType(::core::mem::transmute_copy(&r#type)).into()
        }
        unsafe extern "system" fn Add<Identity: ::windows::core::IUnknownImpl, Impl: IValueMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Add(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn Remove<Identity: ::windows::core::IUnknownImpl, Impl: IValueMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Remove(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn Clear<Identity: ::windows::core::IUnknownImpl, Impl: IValueMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Clear().into()
        }
        unsafe extern "system" fn AddRange<Identity: ::windows::core::IUnknownImpl, Impl: IValueMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, map: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddRange(::core::mem::transmute(&map)).into()
        }
        unsafe extern "system" fn CreateValueMapItem<Identity: ::windows::core::IUnknownImpl, Impl: IValueMap_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateValueMapItem() {
                ::core::result::Result::Ok(ok__) => {
                    *item = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            Value: Value::<Identity, Impl, OFFSET>,
            SetValue: SetValue::<Identity, Impl, OFFSET>,
            ValueMapType: ValueMapType::<Identity, Impl, OFFSET>,
            SetValueMapType: SetValueMapType::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            Clear: Clear::<Identity, Impl, OFFSET>,
            AddRange: AddRange::<Identity, Impl, OFFSET>,
            CreateValueMapItem: CreateValueMapItem::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IValueMap as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IValueMapItem_Impl, const OFFSET: isize>() -> IValueMapItem_Vtbl {
        unsafe extern "system" fn Description<Identity: ::windows::core::IUnknownImpl, Impl: IValueMapItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, description: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *description = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows::core::IUnknownImpl, Impl: IValueMapItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, description: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDescription(::core::mem::transmute_copy(&description)).into()
        }
        unsafe extern "system" fn Enabled<Identity: ::windows::core::IUnknownImpl, Impl: IValueMapItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Enabled() {
                ::core::result::Result::Ok(ok__) => {
                    *enabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: ::windows::core::IUnknownImpl, Impl: IValueMapItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEnabled(::core::mem::transmute_copy(&enabled)).into()
        }
        unsafe extern "system" fn Key<Identity: ::windows::core::IUnknownImpl, Impl: IValueMapItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Key() {
                ::core::result::Result::Ok(ok__) => {
                    *key = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKey<Identity: ::windows::core::IUnknownImpl, Impl: IValueMapItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetKey(::core::mem::transmute_copy(&key)).into()
        }
        unsafe extern "system" fn Value<Identity: ::windows::core::IUnknownImpl, Impl: IValueMapItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Identity: ::windows::core::IUnknownImpl, Impl: IValueMapItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetValue(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn ValueMapType<Identity: ::windows::core::IUnknownImpl, Impl: IValueMapItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: *mut ValueMapType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ValueMapType() {
                ::core::result::Result::Ok(ok__) => {
                    *r#type = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValueMapType<Identity: ::windows::core::IUnknownImpl, Impl: IValueMapItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: ValueMapType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetValueMapType(::core::mem::transmute_copy(&r#type)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            Enabled: Enabled::<Identity, Impl, OFFSET>,
            SetEnabled: SetEnabled::<Identity, Impl, OFFSET>,
            Key: Key::<Identity, Impl, OFFSET>,
            SetKey: SetKey::<Identity, Impl, OFFSET>,
            Value: Value::<Identity, Impl, OFFSET>,
            SetValue: SetValue::<Identity, Impl, OFFSET>,
            ValueMapType: ValueMapType::<Identity, Impl, OFFSET>,
            SetValueMapType: SetValueMapType::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IValueMapItem as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: _ICounterItemUnion_Impl, const OFFSET: isize>() -> _ICounterItemUnion_Vtbl {
        unsafe extern "system" fn Value<Identity: ::windows::core::IUnknownImpl, Impl: _ICounterItemUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdblvalue: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *pdblvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColor<Identity: ::windows::core::IUnknownImpl, Impl: _ICounterItemUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, color: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetColor(::core::mem::transmute_copy(&color)).into()
        }
        unsafe extern "system" fn Color<Identity: ::windows::core::IUnknownImpl, Impl: _ICounterItemUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcolor: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Color() {
                ::core::result::Result::Ok(ok__) => {
                    *pcolor = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWidth<Identity: ::windows::core::IUnknownImpl, Impl: _ICounterItemUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iwidth: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetWidth(::core::mem::transmute_copy(&iwidth)).into()
        }
        unsafe extern "system" fn Width<Identity: ::windows::core::IUnknownImpl, Impl: _ICounterItemUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pivalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Width() {
                ::core::result::Result::Ok(ok__) => {
                    *pivalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLineStyle<Identity: ::windows::core::IUnknownImpl, Impl: _ICounterItemUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ilinestyle: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLineStyle(::core::mem::transmute_copy(&ilinestyle)).into()
        }
        unsafe extern "system" fn LineStyle<Identity: ::windows::core::IUnknownImpl, Impl: _ICounterItemUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pivalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LineStyle() {
                ::core::result::Result::Ok(ok__) => {
                    *pivalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScaleFactor<Identity: ::windows::core::IUnknownImpl, Impl: _ICounterItemUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iscale: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetScaleFactor(::core::mem::transmute_copy(&iscale)).into()
        }
        unsafe extern "system" fn ScaleFactor<Identity: ::windows::core::IUnknownImpl, Impl: _ICounterItemUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pivalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ScaleFactor() {
                ::core::result::Result::Ok(ok__) => {
                    *pivalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Path<Identity: ::windows::core::IUnknownImpl, Impl: _ICounterItemUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstrvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Path() {
                ::core::result::Result::Ok(ok__) => {
                    *pstrvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValue<Identity: ::windows::core::IUnknownImpl, Impl: _ICounterItemUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut f64, status: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetValue(::core::mem::transmute_copy(&value), ::core::mem::transmute_copy(&status)).into()
        }
        unsafe extern "system" fn GetStatistics<Identity: ::windows::core::IUnknownImpl, Impl: _ICounterItemUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, max: *mut f64, min: *mut f64, avg: *mut f64, status: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetStatistics(::core::mem::transmute_copy(&max), ::core::mem::transmute_copy(&min), ::core::mem::transmute_copy(&avg), ::core::mem::transmute_copy(&status)).into()
        }
        unsafe extern "system" fn SetSelected<Identity: ::windows::core::IUnknownImpl, Impl: _ICounterItemUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSelected(::core::mem::transmute_copy(&bstate)).into()
        }
        unsafe extern "system" fn Selected<Identity: ::windows::core::IUnknownImpl, Impl: _ICounterItemUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Selected() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVisible<Identity: ::windows::core::IUnknownImpl, Impl: _ICounterItemUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetVisible(::core::mem::transmute_copy(&bstate)).into()
        }
        unsafe extern "system" fn Visible<Identity: ::windows::core::IUnknownImpl, Impl: _ICounterItemUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Visible() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDataAt<Identity: ::windows::core::IUnknownImpl, Impl: _ICounterItemUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iindex: i32, iwhich: SysmonDataType, pvariant: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDataAt(::core::mem::transmute_copy(&iindex), ::core::mem::transmute_copy(&iwhich)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Value: Value::<Identity, Impl, OFFSET>,
            SetColor: SetColor::<Identity, Impl, OFFSET>,
            Color: Color::<Identity, Impl, OFFSET>,
            SetWidth: SetWidth::<Identity, Impl, OFFSET>,
            Width: Width::<Identity, Impl, OFFSET>,
            SetLineStyle: SetLineStyle::<Identity, Impl, OFFSET>,
            LineStyle: LineStyle::<Identity, Impl, OFFSET>,
            SetScaleFactor: SetScaleFactor::<Identity, Impl, OFFSET>,
            ScaleFactor: ScaleFactor::<Identity, Impl, OFFSET>,
            Path: Path::<Identity, Impl, OFFSET>,
            GetValue: GetValue::<Identity, Impl, OFFSET>,
            GetStatistics: GetStatistics::<Identity, Impl, OFFSET>,
            SetSelected: SetSelected::<Identity, Impl, OFFSET>,
            Selected: Selected::<Identity, Impl, OFFSET>,
            SetVisible: SetVisible::<Identity, Impl, OFFSET>,
            Visible: Visible::<Identity, Impl, OFFSET>,
            GetDataAt: GetDataAt::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>() -> _ISystemMonitorUnion_Vtbl {
        unsafe extern "system" fn Appearance<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iappearance: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Appearance() {
                ::core::result::Result::Ok(ok__) => {
                    *iappearance = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAppearance<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iappearance: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAppearance(::core::mem::transmute_copy(&iappearance)).into()
        }
        unsafe extern "system" fn BackColor<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcolor: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BackColor() {
                ::core::result::Result::Ok(ok__) => {
                    *pcolor = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBackColor<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, color: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBackColor(::core::mem::transmute_copy(&color)).into()
        }
        unsafe extern "system" fn BorderStyle<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iborderstyle: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BorderStyle() {
                ::core::result::Result::Ok(ok__) => {
                    *iborderstyle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBorderStyle<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iborderstyle: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBorderStyle(::core::mem::transmute_copy(&iborderstyle)).into()
        }
        unsafe extern "system" fn ForeColor<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcolor: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ForeColor() {
                ::core::result::Result::Ok(ok__) => {
                    *pcolor = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetForeColor<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, color: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetForeColor(::core::mem::transmute_copy(&color)).into()
        }
        unsafe extern "system" fn Font<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfont: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Font() {
                ::core::result::Result::Ok(ok__) => {
                    *ppfont = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_Font<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfont: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).putref_Font(::core::mem::transmute(&pfont)).into()
        }
        unsafe extern "system" fn Counters<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppicounters: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Counters() {
                ::core::result::Result::Ok(ok__) => {
                    *ppicounters = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShowVerticalGrid<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetShowVerticalGrid(::core::mem::transmute_copy(&bstate)).into()
        }
        unsafe extern "system" fn ShowVerticalGrid<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ShowVerticalGrid() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShowHorizontalGrid<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetShowHorizontalGrid(::core::mem::transmute_copy(&bstate)).into()
        }
        unsafe extern "system" fn ShowHorizontalGrid<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ShowHorizontalGrid() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShowLegend<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetShowLegend(::core::mem::transmute_copy(&bstate)).into()
        }
        unsafe extern "system" fn ShowLegend<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ShowLegend() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShowScaleLabels<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetShowScaleLabels(::core::mem::transmute_copy(&bstate)).into()
        }
        unsafe extern "system" fn ShowScaleLabels<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ShowScaleLabels() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShowValueBar<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetShowValueBar(::core::mem::transmute_copy(&bstate)).into()
        }
        unsafe extern "system" fn ShowValueBar<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ShowValueBar() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaximumScale<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ivalue: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMaximumScale(::core::mem::transmute_copy(&ivalue)).into()
        }
        unsafe extern "system" fn MaximumScale<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pivalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MaximumScale() {
                ::core::result::Result::Ok(ok__) => {
                    *pivalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinimumScale<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ivalue: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMinimumScale(::core::mem::transmute_copy(&ivalue)).into()
        }
        unsafe extern "system" fn MinimumScale<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pivalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MinimumScale() {
                ::core::result::Result::Ok(ok__) => {
                    *pivalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUpdateInterval<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fvalue: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetUpdateInterval(::core::mem::transmute_copy(&fvalue)).into()
        }
        unsafe extern "system" fn UpdateInterval<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfvalue: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).UpdateInterval() {
                ::core::result::Result::Ok(ok__) => {
                    *pfvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayType<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, edisplaytype: DisplayTypeConstants) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDisplayType(::core::mem::transmute_copy(&edisplaytype)).into()
        }
        unsafe extern "system" fn DisplayType<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pedisplaytype: *mut DisplayTypeConstants) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DisplayType() {
                ::core::result::Result::Ok(ok__) => {
                    *pedisplaytype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetManualUpdate<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetManualUpdate(::core::mem::transmute_copy(&bstate)).into()
        }
        unsafe extern "system" fn ManualUpdate<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ManualUpdate() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGraphTitle<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstitle: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetGraphTitle(::core::mem::transmute_copy(&bstitle)).into()
        }
        unsafe extern "system" fn GraphTitle<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstitle: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GraphTitle() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstitle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetYAxisLabel<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstitle: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetYAxisLabel(::core::mem::transmute_copy(&bstitle)).into()
        }
        unsafe extern "system" fn YAxisLabel<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstitle: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).YAxisLabel() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstitle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CollectSample<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CollectSample().into()
        }
        unsafe extern "system" fn UpdateGraph<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UpdateGraph().into()
        }
        unsafe extern "system" fn BrowseCounters<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BrowseCounters().into()
        }
        unsafe extern "system" fn DisplayProperties<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DisplayProperties().into()
        }
        unsafe extern "system" fn Counter<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iindex: i32, ppicounter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Counter(::core::mem::transmute_copy(&iindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppicounter = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddCounter<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bspath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppicounter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AddCounter(::core::mem::transmute_copy(&bspath)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppicounter = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteCounter<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pctr: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteCounter(::core::mem::transmute(&pctr)).into()
        }
        unsafe extern "system" fn BackColorCtl<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcolor: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BackColorCtl() {
                ::core::result::Result::Ok(ok__) => {
                    *pcolor = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBackColorCtl<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, color: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBackColorCtl(::core::mem::transmute_copy(&color)).into()
        }
        unsafe extern "system" fn SetLogFileName<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bsfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLogFileName(::core::mem::transmute_copy(&bsfilename)).into()
        }
        unsafe extern "system" fn LogFileName<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bsfilename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LogFileName() {
                ::core::result::Result::Ok(ok__) => {
                    *bsfilename = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLogViewStart<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, starttime: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLogViewStart(::core::mem::transmute_copy(&starttime)).into()
        }
        unsafe extern "system" fn LogViewStart<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, starttime: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LogViewStart() {
                ::core::result::Result::Ok(ok__) => {
                    *starttime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLogViewStop<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stoptime: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLogViewStop(::core::mem::transmute_copy(&stoptime)).into()
        }
        unsafe extern "system" fn LogViewStop<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stoptime: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LogViewStop() {
                ::core::result::Result::Ok(ok__) => {
                    *stoptime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GridColor<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcolor: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GridColor() {
                ::core::result::Result::Ok(ok__) => {
                    *pcolor = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGridColor<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, color: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetGridColor(::core::mem::transmute_copy(&color)).into()
        }
        unsafe extern "system" fn TimeBarColor<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcolor: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TimeBarColor() {
                ::core::result::Result::Ok(ok__) => {
                    *pcolor = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTimeBarColor<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, color: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTimeBarColor(::core::mem::transmute_copy(&color)).into()
        }
        unsafe extern "system" fn Highlight<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Highlight() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHighlight<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetHighlight(::core::mem::transmute_copy(&bstate)).into()
        }
        unsafe extern "system" fn ShowToolbar<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ShowToolbar() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShowToolbar<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetShowToolbar(::core::mem::transmute_copy(&bstate)).into()
        }
        unsafe extern "system" fn Paste<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Paste().into()
        }
        unsafe extern "system" fn Copy<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Copy().into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn SetReadOnly<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetReadOnly(::core::mem::transmute_copy(&bstate)).into()
        }
        unsafe extern "system" fn ReadOnly<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ReadOnly() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReportValueType<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ereportvaluetype: ReportValueTypeConstants) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetReportValueType(::core::mem::transmute_copy(&ereportvaluetype)).into()
        }
        unsafe extern "system" fn ReportValueType<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pereportvaluetype: *mut ReportValueTypeConstants) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ReportValueType() {
                ::core::result::Result::Ok(ok__) => {
                    *pereportvaluetype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMonitorDuplicateInstances<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMonitorDuplicateInstances(::core::mem::transmute_copy(&bstate)).into()
        }
        unsafe extern "system" fn MonitorDuplicateInstances<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MonitorDuplicateInstances() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayFilter<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ivalue: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDisplayFilter(::core::mem::transmute_copy(&ivalue)).into()
        }
        unsafe extern "system" fn DisplayFilter<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pivalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DisplayFilter() {
                ::core::result::Result::Ok(ok__) => {
                    *pivalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LogFiles<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppilogfiles: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LogFiles() {
                ::core::result::Result::Ok(ok__) => {
                    *ppilogfiles = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDataSourceType<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, edatasourcetype: DataSourceTypeConstants) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDataSourceType(::core::mem::transmute_copy(&edatasourcetype)).into()
        }
        unsafe extern "system" fn DataSourceType<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pedatasourcetype: *mut DataSourceTypeConstants) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DataSourceType() {
                ::core::result::Result::Ok(ok__) => {
                    *pedatasourcetype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSqlDsnName<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bssqldsnname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSqlDsnName(::core::mem::transmute_copy(&bssqldsnname)).into()
        }
        unsafe extern "system" fn SqlDsnName<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bssqldsnname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SqlDsnName() {
                ::core::result::Result::Ok(ok__) => {
                    *bssqldsnname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSqlLogSetName<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bssqllogsetname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSqlLogSetName(::core::mem::transmute_copy(&bssqllogsetname)).into()
        }
        unsafe extern "system" fn SqlLogSetName<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bssqllogsetname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SqlLogSetName() {
                ::core::result::Result::Ok(ok__) => {
                    *bssqllogsetname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnableDigitGrouping<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEnableDigitGrouping(::core::mem::transmute_copy(&bstate)).into()
        }
        unsafe extern "system" fn EnableDigitGrouping<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnableDigitGrouping() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnableToolTips<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEnableToolTips(::core::mem::transmute_copy(&bstate)).into()
        }
        unsafe extern "system" fn EnableToolTips<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnableToolTips() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShowTimeAxisLabels<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstate: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetShowTimeAxisLabels(::core::mem::transmute_copy(&bstate)).into()
        }
        unsafe extern "system" fn ShowTimeAxisLabels<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstate: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ShowTimeAxisLabels() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChartScroll<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bscroll: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetChartScroll(::core::mem::transmute_copy(&bscroll)).into()
        }
        unsafe extern "system" fn ChartScroll<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbscroll: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ChartScroll() {
                ::core::result::Result::Ok(ok__) => {
                    *pbscroll = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDataPointCount<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inewcount: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDataPointCount(::core::mem::transmute_copy(&inewcount)).into()
        }
        unsafe extern "system" fn DataPointCount<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidatapointcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DataPointCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pidatapointcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScaleToFit<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bselectedcountersonly: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ScaleToFit(::core::mem::transmute_copy(&bselectedcountersonly)).into()
        }
        unsafe extern "system" fn SaveAs<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, esysmonfiletype: SysmonFileType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SaveAs(::core::mem::transmute_copy(&bstrfilename), ::core::mem::transmute_copy(&esysmonfiletype)).into()
        }
        unsafe extern "system" fn Relog<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, esysmonfiletype: SysmonFileType, ifilter: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Relog(::core::mem::transmute_copy(&bstrfilename), ::core::mem::transmute_copy(&esysmonfiletype), ::core::mem::transmute_copy(&ifilter)).into()
        }
        unsafe extern "system" fn ClearData<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ClearData().into()
        }
        unsafe extern "system" fn LogSourceStartTime<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdate: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LogSourceStartTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pdate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LogSourceStopTime<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdate: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LogSourceStopTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pdate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLogViewRange<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, starttime: f64, stoptime: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLogViewRange(::core::mem::transmute_copy(&starttime), ::core::mem::transmute_copy(&stoptime)).into()
        }
        unsafe extern "system" fn GetLogViewRange<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, starttime: *mut f64, stoptime: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetLogViewRange(::core::mem::transmute_copy(&starttime), ::core::mem::transmute_copy(&stoptime)).into()
        }
        unsafe extern "system" fn BatchingLock<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flock: i16, ebatchreason: SysmonBatchReason) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BatchingLock(::core::mem::transmute_copy(&flock), ::core::mem::transmute_copy(&ebatchreason)).into()
        }
        unsafe extern "system" fn LoadSettings<Identity: ::windows::core::IUnknownImpl, Impl: _ISystemMonitorUnion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsettingfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).LoadSettings(::core::mem::transmute_copy(&bstrsettingfilename)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Appearance: Appearance::<Identity, Impl, OFFSET>,
            SetAppearance: SetAppearance::<Identity, Impl, OFFSET>,
            BackColor: BackColor::<Identity, Impl, OFFSET>,
            SetBackColor: SetBackColor::<Identity, Impl, OFFSET>,
            BorderStyle: BorderStyle::<Identity, Impl, OFFSET>,
            SetBorderStyle: SetBorderStyle::<Identity, Impl, OFFSET>,
            ForeColor: ForeColor::<Identity, Impl, OFFSET>,
            SetForeColor: SetForeColor::<Identity, Impl, OFFSET>,
            Font: Font::<Identity, Impl, OFFSET>,
            putref_Font: putref_Font::<Identity, Impl, OFFSET>,
            Counters: Counters::<Identity, Impl, OFFSET>,
            SetShowVerticalGrid: SetShowVerticalGrid::<Identity, Impl, OFFSET>,
            ShowVerticalGrid: ShowVerticalGrid::<Identity, Impl, OFFSET>,
            SetShowHorizontalGrid: SetShowHorizontalGrid::<Identity, Impl, OFFSET>,
            ShowHorizontalGrid: ShowHorizontalGrid::<Identity, Impl, OFFSET>,
            SetShowLegend: SetShowLegend::<Identity, Impl, OFFSET>,
            ShowLegend: ShowLegend::<Identity, Impl, OFFSET>,
            SetShowScaleLabels: SetShowScaleLabels::<Identity, Impl, OFFSET>,
            ShowScaleLabels: ShowScaleLabels::<Identity, Impl, OFFSET>,
            SetShowValueBar: SetShowValueBar::<Identity, Impl, OFFSET>,
            ShowValueBar: ShowValueBar::<Identity, Impl, OFFSET>,
            SetMaximumScale: SetMaximumScale::<Identity, Impl, OFFSET>,
            MaximumScale: MaximumScale::<Identity, Impl, OFFSET>,
            SetMinimumScale: SetMinimumScale::<Identity, Impl, OFFSET>,
            MinimumScale: MinimumScale::<Identity, Impl, OFFSET>,
            SetUpdateInterval: SetUpdateInterval::<Identity, Impl, OFFSET>,
            UpdateInterval: UpdateInterval::<Identity, Impl, OFFSET>,
            SetDisplayType: SetDisplayType::<Identity, Impl, OFFSET>,
            DisplayType: DisplayType::<Identity, Impl, OFFSET>,
            SetManualUpdate: SetManualUpdate::<Identity, Impl, OFFSET>,
            ManualUpdate: ManualUpdate::<Identity, Impl, OFFSET>,
            SetGraphTitle: SetGraphTitle::<Identity, Impl, OFFSET>,
            GraphTitle: GraphTitle::<Identity, Impl, OFFSET>,
            SetYAxisLabel: SetYAxisLabel::<Identity, Impl, OFFSET>,
            YAxisLabel: YAxisLabel::<Identity, Impl, OFFSET>,
            CollectSample: CollectSample::<Identity, Impl, OFFSET>,
            UpdateGraph: UpdateGraph::<Identity, Impl, OFFSET>,
            BrowseCounters: BrowseCounters::<Identity, Impl, OFFSET>,
            DisplayProperties: DisplayProperties::<Identity, Impl, OFFSET>,
            Counter: Counter::<Identity, Impl, OFFSET>,
            AddCounter: AddCounter::<Identity, Impl, OFFSET>,
            DeleteCounter: DeleteCounter::<Identity, Impl, OFFSET>,
            BackColorCtl: BackColorCtl::<Identity, Impl, OFFSET>,
            SetBackColorCtl: SetBackColorCtl::<Identity, Impl, OFFSET>,
            SetLogFileName: SetLogFileName::<Identity, Impl, OFFSET>,
            LogFileName: LogFileName::<Identity, Impl, OFFSET>,
            SetLogViewStart: SetLogViewStart::<Identity, Impl, OFFSET>,
            LogViewStart: LogViewStart::<Identity, Impl, OFFSET>,
            SetLogViewStop: SetLogViewStop::<Identity, Impl, OFFSET>,
            LogViewStop: LogViewStop::<Identity, Impl, OFFSET>,
            GridColor: GridColor::<Identity, Impl, OFFSET>,
            SetGridColor: SetGridColor::<Identity, Impl, OFFSET>,
            TimeBarColor: TimeBarColor::<Identity, Impl, OFFSET>,
            SetTimeBarColor: SetTimeBarColor::<Identity, Impl, OFFSET>,
            Highlight: Highlight::<Identity, Impl, OFFSET>,
            SetHighlight: SetHighlight::<Identity, Impl, OFFSET>,
            ShowToolbar: ShowToolbar::<Identity, Impl, OFFSET>,
            SetShowToolbar: SetShowToolbar::<Identity, Impl, OFFSET>,
            Paste: Paste::<Identity, Impl, OFFSET>,
            Copy: Copy::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            SetReadOnly: SetReadOnly::<Identity, Impl, OFFSET>,
            ReadOnly: ReadOnly::<Identity, Impl, OFFSET>,
            SetReportValueType: SetReportValueType::<Identity, Impl, OFFSET>,
            ReportValueType: ReportValueType::<Identity, Impl, OFFSET>,
            SetMonitorDuplicateInstances: SetMonitorDuplicateInstances::<Identity, Impl, OFFSET>,
            MonitorDuplicateInstances: MonitorDuplicateInstances::<Identity, Impl, OFFSET>,
            SetDisplayFilter: SetDisplayFilter::<Identity, Impl, OFFSET>,
            DisplayFilter: DisplayFilter::<Identity, Impl, OFFSET>,
            LogFiles: LogFiles::<Identity, Impl, OFFSET>,
            SetDataSourceType: SetDataSourceType::<Identity, Impl, OFFSET>,
            DataSourceType: DataSourceType::<Identity, Impl, OFFSET>,
            SetSqlDsnName: SetSqlDsnName::<Identity, Impl, OFFSET>,
            SqlDsnName: SqlDsnName::<Identity, Impl, OFFSET>,
            SetSqlLogSetName: SetSqlLogSetName::<Identity, Impl, OFFSET>,
            SqlLogSetName: SqlLogSetName::<Identity, Impl, OFFSET>,
            SetEnableDigitGrouping: SetEnableDigitGrouping::<Identity, Impl, OFFSET>,
            EnableDigitGrouping: EnableDigitGrouping::<Identity, Impl, OFFSET>,
            SetEnableToolTips: SetEnableToolTips::<Identity, Impl, OFFSET>,
            EnableToolTips: EnableToolTips::<Identity, Impl, OFFSET>,
            SetShowTimeAxisLabels: SetShowTimeAxisLabels::<Identity, Impl, OFFSET>,
            ShowTimeAxisLabels: ShowTimeAxisLabels::<Identity, Impl, OFFSET>,
            SetChartScroll: SetChartScroll::<Identity, Impl, OFFSET>,
            ChartScroll: ChartScroll::<Identity, Impl, OFFSET>,
            SetDataPointCount: SetDataPointCount::<Identity, Impl, OFFSET>,
            DataPointCount: DataPointCount::<Identity, Impl, OFFSET>,
            ScaleToFit: ScaleToFit::<Identity, Impl, OFFSET>,
            SaveAs: SaveAs::<Identity, Impl, OFFSET>,
            Relog: Relog::<Identity, Impl, OFFSET>,
            ClearData: ClearData::<Identity, Impl, OFFSET>,
            LogSourceStartTime: LogSourceStartTime::<Identity, Impl, OFFSET>,
            LogSourceStopTime: LogSourceStopTime::<Identity, Impl, OFFSET>,
            SetLogViewRange: SetLogViewRange::<Identity, Impl, OFFSET>,
            GetLogViewRange: GetLogViewRange::<Identity, Impl, OFFSET>,
            BatchingLock: BatchingLock::<Identity, Impl, OFFSET>,
            LoadSettings: LoadSettings::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<_ISystemMonitorUnion as ::windows::core::Interface>::IID
    }
}
