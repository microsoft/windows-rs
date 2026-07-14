pub type AutoPathFormat = i32;
pub const BootTraceSession: windows_core::GUID = windows_core::GUID::from_u128(0x03837538_098b_11d8_9414_505054503030);
pub const BootTraceSessionCollection: windows_core::GUID = windows_core::GUID::from_u128(0x03837539_098b_11d8_9414_505054503030);
pub type ClockType = i32;
pub type CommitMode = i32;
pub const DataCollectorSet: windows_core::GUID = windows_core::GUID::from_u128(0x03837521_098b_11d8_9414_505054503030);
pub const DataCollectorSetCollection: windows_core::GUID = windows_core::GUID::from_u128(0x03837525_098b_11d8_9414_505054503030);
pub type DataCollectorSetStatus = i32;
pub type DataCollectorType = i32;
pub type DataManagerSteps = i32;
pub type FileFormat = i32;
pub type FolderActionSteps = i32;
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IAlertDataCollector, IAlertDataCollector_Vtbl, 0x03837516_098b_11d8_9414_505054503030);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IAlertDataCollector {
    type Target = IDataCollector;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IAlertDataCollector, windows_core::IUnknown, super::oaidl::IDispatch, IDataCollector);
#[cfg(feature = "oaidl")]
impl IAlertDataCollector {
    pub unsafe fn AlertThresholds(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AlertThresholds)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAlertThresholds(&self, alerts: *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAlertThresholds)(windows_core::Interface::as_raw(self), alerts) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn EventLog(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EventLog)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetEventLog(&self, log: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetEventLog)(windows_core::Interface::as_raw(self), log) }
    }
    pub unsafe fn SampleInterval(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SampleInterval)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSampleInterval(&self, interval: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSampleInterval)(windows_core::Interface::as_raw(self), interval) }
    }
    pub unsafe fn Task(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Task)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetTask(&self, task: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetTask)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(task)) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn TaskRunAsSelf(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TaskRunAsSelf)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetTaskRunAsSelf(&self, runasself: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetTaskRunAsSelf)(windows_core::Interface::as_raw(self), runasself) }
    }
    pub unsafe fn TaskArguments(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TaskArguments)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetTaskArguments(&self, task: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetTaskArguments)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(task)) }
    }
    pub unsafe fn TaskUserTextArguments(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TaskUserTextArguments)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetTaskUserTextArguments(&self, task: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetTaskUserTextArguments)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(task)) }
    }
    pub unsafe fn TriggerDataCollectorSet(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TriggerDataCollectorSet)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetTriggerDataCollectorSet(&self, name: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetTriggerDataCollectorSet)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IAlertDataCollector_Vtbl {
    pub base__: IDataCollector_Vtbl,
    pub AlertThresholds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    pub SetAlertThresholds: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub EventLog: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    EventLog: usize,
    #[cfg(feature = "wtypes")]
    pub SetEventLog: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetEventLog: usize,
    pub SampleInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetSampleInterval: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Task: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTask: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub TaskRunAsSelf: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    TaskRunAsSelf: usize,
    #[cfg(feature = "wtypes")]
    pub SetTaskRunAsSelf: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetTaskRunAsSelf: usize,
    pub TaskArguments: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTaskArguments: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TaskUserTextArguments: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTaskUserTextArguments: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TriggerDataCollectorSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTriggerDataCollectorSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IAlertDataCollector_Impl: IDataCollector_Impl {
    fn AlertThresholds(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY>;
    fn SetAlertThresholds(&self, alerts: *const super::oaidl::SAFEARRAY) -> windows_core::Result<()>;
    fn EventLog(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn SetEventLog(&self, log: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SampleInterval(&self) -> windows_core::Result<u32>;
    fn SetSampleInterval(&self, interval: u32) -> windows_core::Result<()>;
    fn Task(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetTask(&self, task: &windows_core::BSTR) -> windows_core::Result<()>;
    fn TaskRunAsSelf(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn SetTaskRunAsSelf(&self, runasself: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn TaskArguments(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetTaskArguments(&self, task: &windows_core::BSTR) -> windows_core::Result<()>;
    fn TaskUserTextArguments(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetTaskUserTextArguments(&self, task: &windows_core::BSTR) -> windows_core::Result<()>;
    fn TriggerDataCollectorSet(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetTriggerDataCollectorSet(&self, name: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IAlertDataCollector_Vtbl {
    pub const fn new<Identity: IAlertDataCollector_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AlertThresholds<Identity: IAlertDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, alerts: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAlertDataCollector_Impl::AlertThresholds(this) {
                    Ok(ok__) => {
                        alerts.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAlertThresholds<Identity: IAlertDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, alerts: *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAlertDataCollector_Impl::SetAlertThresholds(this, core::mem::transmute_copy(&alerts)).into()
            }
        }
        unsafe extern "system" fn EventLog<Identity: IAlertDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, log: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAlertDataCollector_Impl::EventLog(this) {
                    Ok(ok__) => {
                        log.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEventLog<Identity: IAlertDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, log: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAlertDataCollector_Impl::SetEventLog(this, core::mem::transmute_copy(&log)).into()
            }
        }
        unsafe extern "system" fn SampleInterval<Identity: IAlertDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, interval: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAlertDataCollector_Impl::SampleInterval(this) {
                    Ok(ok__) => {
                        interval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSampleInterval<Identity: IAlertDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, interval: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAlertDataCollector_Impl::SetSampleInterval(this, core::mem::transmute_copy(&interval)).into()
            }
        }
        unsafe extern "system" fn Task<Identity: IAlertDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, task: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAlertDataCollector_Impl::Task(this) {
                    Ok(ok__) => {
                        task.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTask<Identity: IAlertDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, task: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAlertDataCollector_Impl::SetTask(this, core::mem::transmute(&task)).into()
            }
        }
        unsafe extern "system" fn TaskRunAsSelf<Identity: IAlertDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, runasself: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAlertDataCollector_Impl::TaskRunAsSelf(this) {
                    Ok(ok__) => {
                        runasself.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTaskRunAsSelf<Identity: IAlertDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, runasself: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAlertDataCollector_Impl::SetTaskRunAsSelf(this, core::mem::transmute_copy(&runasself)).into()
            }
        }
        unsafe extern "system" fn TaskArguments<Identity: IAlertDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, task: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAlertDataCollector_Impl::TaskArguments(this) {
                    Ok(ok__) => {
                        task.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTaskArguments<Identity: IAlertDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, task: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAlertDataCollector_Impl::SetTaskArguments(this, core::mem::transmute(&task)).into()
            }
        }
        unsafe extern "system" fn TaskUserTextArguments<Identity: IAlertDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, task: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAlertDataCollector_Impl::TaskUserTextArguments(this) {
                    Ok(ok__) => {
                        task.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTaskUserTextArguments<Identity: IAlertDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, task: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAlertDataCollector_Impl::SetTaskUserTextArguments(this, core::mem::transmute(&task)).into()
            }
        }
        unsafe extern "system" fn TriggerDataCollectorSet<Identity: IAlertDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAlertDataCollector_Impl::TriggerDataCollectorSet(this) {
                    Ok(ok__) => {
                        name.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTriggerDataCollectorSet<Identity: IAlertDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAlertDataCollector_Impl::SetTriggerDataCollectorSet(this, core::mem::transmute(&name)).into()
            }
        }
        Self {
            base__: IDataCollector_Vtbl::new::<Identity, OFFSET>(),
            AlertThresholds: AlertThresholds::<Identity, OFFSET>,
            SetAlertThresholds: SetAlertThresholds::<Identity, OFFSET>,
            EventLog: EventLog::<Identity, OFFSET>,
            SetEventLog: SetEventLog::<Identity, OFFSET>,
            SampleInterval: SampleInterval::<Identity, OFFSET>,
            SetSampleInterval: SetSampleInterval::<Identity, OFFSET>,
            Task: Task::<Identity, OFFSET>,
            SetTask: SetTask::<Identity, OFFSET>,
            TaskRunAsSelf: TaskRunAsSelf::<Identity, OFFSET>,
            SetTaskRunAsSelf: SetTaskRunAsSelf::<Identity, OFFSET>,
            TaskArguments: TaskArguments::<Identity, OFFSET>,
            SetTaskArguments: SetTaskArguments::<Identity, OFFSET>,
            TaskUserTextArguments: TaskUserTextArguments::<Identity, OFFSET>,
            SetTaskUserTextArguments: SetTaskUserTextArguments::<Identity, OFFSET>,
            TriggerDataCollectorSet: TriggerDataCollectorSet::<Identity, OFFSET>,
            SetTriggerDataCollectorSet: SetTriggerDataCollectorSet::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAlertDataCollector as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<IDataCollector as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IAlertDataCollector {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IApiTracingDataCollector, IApiTracingDataCollector_Vtbl, 0x0383751a_098b_11d8_9414_505054503030);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IApiTracingDataCollector {
    type Target = IDataCollector;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IApiTracingDataCollector, windows_core::IUnknown, super::oaidl::IDispatch, IDataCollector);
#[cfg(feature = "oaidl")]
impl IApiTracingDataCollector {
    #[cfg(feature = "wtypes")]
    pub unsafe fn LogApiNamesOnly(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LogApiNamesOnly)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetLogApiNamesOnly(&self, logapinames: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetLogApiNamesOnly)(windows_core::Interface::as_raw(self), logapinames) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn LogApisRecursively(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LogApisRecursively)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetLogApisRecursively(&self, logrecursively: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetLogApisRecursively)(windows_core::Interface::as_raw(self), logrecursively) }
    }
    pub unsafe fn ExePath(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ExePath)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetExePath(&self, exepath: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetExePath)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(exepath)) }
    }
    pub unsafe fn LogFilePath(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LogFilePath)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetLogFilePath(&self, logfilepath: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetLogFilePath)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(logfilepath)) }
    }
    pub unsafe fn IncludeModules(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IncludeModules)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetIncludeModules(&self, includemodules: *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetIncludeModules)(windows_core::Interface::as_raw(self), includemodules) }
    }
    pub unsafe fn IncludeApis(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IncludeApis)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetIncludeApis(&self, includeapis: *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetIncludeApis)(windows_core::Interface::as_raw(self), includeapis) }
    }
    pub unsafe fn ExcludeApis(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ExcludeApis)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetExcludeApis(&self, excludeapis: *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetExcludeApis)(windows_core::Interface::as_raw(self), excludeapis) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IApiTracingDataCollector_Vtbl {
    pub base__: IDataCollector_Vtbl,
    #[cfg(feature = "wtypes")]
    pub LogApiNamesOnly: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    LogApiNamesOnly: usize,
    #[cfg(feature = "wtypes")]
    pub SetLogApiNamesOnly: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetLogApiNamesOnly: usize,
    #[cfg(feature = "wtypes")]
    pub LogApisRecursively: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    LogApisRecursively: usize,
    #[cfg(feature = "wtypes")]
    pub SetLogApisRecursively: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetLogApisRecursively: usize,
    pub ExePath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetExePath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LogFilePath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetLogFilePath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IncludeModules: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    pub SetIncludeModules: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    pub IncludeApis: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    pub SetIncludeApis: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    pub ExcludeApis: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    pub SetExcludeApis: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IApiTracingDataCollector_Impl: IDataCollector_Impl {
    fn LogApiNamesOnly(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn SetLogApiNamesOnly(&self, logapinames: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn LogApisRecursively(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn SetLogApisRecursively(&self, logrecursively: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ExePath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetExePath(&self, exepath: &windows_core::BSTR) -> windows_core::Result<()>;
    fn LogFilePath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetLogFilePath(&self, logfilepath: &windows_core::BSTR) -> windows_core::Result<()>;
    fn IncludeModules(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY>;
    fn SetIncludeModules(&self, includemodules: *const super::oaidl::SAFEARRAY) -> windows_core::Result<()>;
    fn IncludeApis(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY>;
    fn SetIncludeApis(&self, includeapis: *const super::oaidl::SAFEARRAY) -> windows_core::Result<()>;
    fn ExcludeApis(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY>;
    fn SetExcludeApis(&self, excludeapis: *const super::oaidl::SAFEARRAY) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IApiTracingDataCollector_Vtbl {
    pub const fn new<Identity: IApiTracingDataCollector_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn LogApiNamesOnly<Identity: IApiTracingDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, logapinames: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IApiTracingDataCollector_Impl::LogApiNamesOnly(this) {
                    Ok(ok__) => {
                        logapinames.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLogApiNamesOnly<Identity: IApiTracingDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, logapinames: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IApiTracingDataCollector_Impl::SetLogApiNamesOnly(this, core::mem::transmute_copy(&logapinames)).into()
            }
        }
        unsafe extern "system" fn LogApisRecursively<Identity: IApiTracingDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, logrecursively: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IApiTracingDataCollector_Impl::LogApisRecursively(this) {
                    Ok(ok__) => {
                        logrecursively.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLogApisRecursively<Identity: IApiTracingDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, logrecursively: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IApiTracingDataCollector_Impl::SetLogApisRecursively(this, core::mem::transmute_copy(&logrecursively)).into()
            }
        }
        unsafe extern "system" fn ExePath<Identity: IApiTracingDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, exepath: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IApiTracingDataCollector_Impl::ExePath(this) {
                    Ok(ok__) => {
                        exepath.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetExePath<Identity: IApiTracingDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, exepath: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IApiTracingDataCollector_Impl::SetExePath(this, core::mem::transmute(&exepath)).into()
            }
        }
        unsafe extern "system" fn LogFilePath<Identity: IApiTracingDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, logfilepath: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IApiTracingDataCollector_Impl::LogFilePath(this) {
                    Ok(ok__) => {
                        logfilepath.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLogFilePath<Identity: IApiTracingDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, logfilepath: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IApiTracingDataCollector_Impl::SetLogFilePath(this, core::mem::transmute(&logfilepath)).into()
            }
        }
        unsafe extern "system" fn IncludeModules<Identity: IApiTracingDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, includemodules: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IApiTracingDataCollector_Impl::IncludeModules(this) {
                    Ok(ok__) => {
                        includemodules.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIncludeModules<Identity: IApiTracingDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, includemodules: *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IApiTracingDataCollector_Impl::SetIncludeModules(this, core::mem::transmute_copy(&includemodules)).into()
            }
        }
        unsafe extern "system" fn IncludeApis<Identity: IApiTracingDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, includeapis: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IApiTracingDataCollector_Impl::IncludeApis(this) {
                    Ok(ok__) => {
                        includeapis.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIncludeApis<Identity: IApiTracingDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, includeapis: *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IApiTracingDataCollector_Impl::SetIncludeApis(this, core::mem::transmute_copy(&includeapis)).into()
            }
        }
        unsafe extern "system" fn ExcludeApis<Identity: IApiTracingDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, excludeapis: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IApiTracingDataCollector_Impl::ExcludeApis(this) {
                    Ok(ok__) => {
                        excludeapis.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetExcludeApis<Identity: IApiTracingDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, excludeapis: *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IApiTracingDataCollector_Impl::SetExcludeApis(this, core::mem::transmute_copy(&excludeapis)).into()
            }
        }
        Self {
            base__: IDataCollector_Vtbl::new::<Identity, OFFSET>(),
            LogApiNamesOnly: LogApiNamesOnly::<Identity, OFFSET>,
            SetLogApiNamesOnly: SetLogApiNamesOnly::<Identity, OFFSET>,
            LogApisRecursively: LogApisRecursively::<Identity, OFFSET>,
            SetLogApisRecursively: SetLogApisRecursively::<Identity, OFFSET>,
            ExePath: ExePath::<Identity, OFFSET>,
            SetExePath: SetExePath::<Identity, OFFSET>,
            LogFilePath: LogFilePath::<Identity, OFFSET>,
            SetLogFilePath: SetLogFilePath::<Identity, OFFSET>,
            IncludeModules: IncludeModules::<Identity, OFFSET>,
            SetIncludeModules: SetIncludeModules::<Identity, OFFSET>,
            IncludeApis: IncludeApis::<Identity, OFFSET>,
            SetIncludeApis: SetIncludeApis::<Identity, OFFSET>,
            ExcludeApis: ExcludeApis::<Identity, OFFSET>,
            SetExcludeApis: SetExcludeApis::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IApiTracingDataCollector as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<IDataCollector as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IApiTracingDataCollector {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IConfigurationDataCollector, IConfigurationDataCollector_Vtbl, 0x03837514_098b_11d8_9414_505054503030);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IConfigurationDataCollector {
    type Target = IDataCollector;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IConfigurationDataCollector, windows_core::IUnknown, super::oaidl::IDispatch, IDataCollector);
#[cfg(feature = "oaidl")]
impl IConfigurationDataCollector {
    pub unsafe fn FileMaxCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FileMaxCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetFileMaxCount(&self, count: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFileMaxCount)(windows_core::Interface::as_raw(self), count) }
    }
    pub unsafe fn FileMaxRecursiveDepth(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FileMaxRecursiveDepth)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetFileMaxRecursiveDepth(&self, depth: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFileMaxRecursiveDepth)(windows_core::Interface::as_raw(self), depth) }
    }
    pub unsafe fn FileMaxTotalSize(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FileMaxTotalSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetFileMaxTotalSize(&self, size: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFileMaxTotalSize)(windows_core::Interface::as_raw(self), size) }
    }
    pub unsafe fn Files(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Files)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetFiles(&self, files: *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFiles)(windows_core::Interface::as_raw(self), files) }
    }
    pub unsafe fn ManagementQueries(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ManagementQueries)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetManagementQueries(&self, queries: *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetManagementQueries)(windows_core::Interface::as_raw(self), queries) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn QueryNetworkAdapters(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QueryNetworkAdapters)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetQueryNetworkAdapters(&self, network: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetQueryNetworkAdapters)(windows_core::Interface::as_raw(self), network) }
    }
    pub unsafe fn RegistryKeys(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RegistryKeys)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetRegistryKeys(&self, query: *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRegistryKeys)(windows_core::Interface::as_raw(self), query) }
    }
    pub unsafe fn RegistryMaxRecursiveDepth(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RegistryMaxRecursiveDepth)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetRegistryMaxRecursiveDepth(&self, depth: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRegistryMaxRecursiveDepth)(windows_core::Interface::as_raw(self), depth) }
    }
    pub unsafe fn SystemStateFile(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SystemStateFile)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetSystemStateFile(&self, filename: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSystemStateFile)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(filename)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IConfigurationDataCollector_Vtbl {
    pub base__: IDataCollector_Vtbl,
    pub FileMaxCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetFileMaxCount: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub FileMaxRecursiveDepth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetFileMaxRecursiveDepth: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub FileMaxTotalSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetFileMaxTotalSize: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Files: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    pub SetFiles: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    pub ManagementQueries: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    pub SetManagementQueries: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub QueryNetworkAdapters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    QueryNetworkAdapters: usize,
    #[cfg(feature = "wtypes")]
    pub SetQueryNetworkAdapters: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetQueryNetworkAdapters: usize,
    pub RegistryKeys: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    pub SetRegistryKeys: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    pub RegistryMaxRecursiveDepth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetRegistryMaxRecursiveDepth: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SystemStateFile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSystemStateFile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IConfigurationDataCollector_Impl: IDataCollector_Impl {
    fn FileMaxCount(&self) -> windows_core::Result<u32>;
    fn SetFileMaxCount(&self, count: u32) -> windows_core::Result<()>;
    fn FileMaxRecursiveDepth(&self) -> windows_core::Result<u32>;
    fn SetFileMaxRecursiveDepth(&self, depth: u32) -> windows_core::Result<()>;
    fn FileMaxTotalSize(&self) -> windows_core::Result<u32>;
    fn SetFileMaxTotalSize(&self, size: u32) -> windows_core::Result<()>;
    fn Files(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY>;
    fn SetFiles(&self, files: *const super::oaidl::SAFEARRAY) -> windows_core::Result<()>;
    fn ManagementQueries(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY>;
    fn SetManagementQueries(&self, queries: *const super::oaidl::SAFEARRAY) -> windows_core::Result<()>;
    fn QueryNetworkAdapters(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn SetQueryNetworkAdapters(&self, network: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn RegistryKeys(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY>;
    fn SetRegistryKeys(&self, query: *const super::oaidl::SAFEARRAY) -> windows_core::Result<()>;
    fn RegistryMaxRecursiveDepth(&self) -> windows_core::Result<u32>;
    fn SetRegistryMaxRecursiveDepth(&self, depth: u32) -> windows_core::Result<()>;
    fn SystemStateFile(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetSystemStateFile(&self, filename: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IConfigurationDataCollector_Vtbl {
    pub const fn new<Identity: IConfigurationDataCollector_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn FileMaxCount<Identity: IConfigurationDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IConfigurationDataCollector_Impl::FileMaxCount(this) {
                    Ok(ok__) => {
                        count.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFileMaxCount<Identity: IConfigurationDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IConfigurationDataCollector_Impl::SetFileMaxCount(this, core::mem::transmute_copy(&count)).into()
            }
        }
        unsafe extern "system" fn FileMaxRecursiveDepth<Identity: IConfigurationDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, depth: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IConfigurationDataCollector_Impl::FileMaxRecursiveDepth(this) {
                    Ok(ok__) => {
                        depth.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFileMaxRecursiveDepth<Identity: IConfigurationDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, depth: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IConfigurationDataCollector_Impl::SetFileMaxRecursiveDepth(this, core::mem::transmute_copy(&depth)).into()
            }
        }
        unsafe extern "system" fn FileMaxTotalSize<Identity: IConfigurationDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, size: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IConfigurationDataCollector_Impl::FileMaxTotalSize(this) {
                    Ok(ok__) => {
                        size.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFileMaxTotalSize<Identity: IConfigurationDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, size: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IConfigurationDataCollector_Impl::SetFileMaxTotalSize(this, core::mem::transmute_copy(&size)).into()
            }
        }
        unsafe extern "system" fn Files<Identity: IConfigurationDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, files: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IConfigurationDataCollector_Impl::Files(this) {
                    Ok(ok__) => {
                        files.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFiles<Identity: IConfigurationDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, files: *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IConfigurationDataCollector_Impl::SetFiles(this, core::mem::transmute_copy(&files)).into()
            }
        }
        unsafe extern "system" fn ManagementQueries<Identity: IConfigurationDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, queries: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IConfigurationDataCollector_Impl::ManagementQueries(this) {
                    Ok(ok__) => {
                        queries.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetManagementQueries<Identity: IConfigurationDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, queries: *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IConfigurationDataCollector_Impl::SetManagementQueries(this, core::mem::transmute_copy(&queries)).into()
            }
        }
        unsafe extern "system" fn QueryNetworkAdapters<Identity: IConfigurationDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, network: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IConfigurationDataCollector_Impl::QueryNetworkAdapters(this) {
                    Ok(ok__) => {
                        network.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetQueryNetworkAdapters<Identity: IConfigurationDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, network: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IConfigurationDataCollector_Impl::SetQueryNetworkAdapters(this, core::mem::transmute_copy(&network)).into()
            }
        }
        unsafe extern "system" fn RegistryKeys<Identity: IConfigurationDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, query: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IConfigurationDataCollector_Impl::RegistryKeys(this) {
                    Ok(ok__) => {
                        query.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRegistryKeys<Identity: IConfigurationDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, query: *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IConfigurationDataCollector_Impl::SetRegistryKeys(this, core::mem::transmute_copy(&query)).into()
            }
        }
        unsafe extern "system" fn RegistryMaxRecursiveDepth<Identity: IConfigurationDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, depth: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IConfigurationDataCollector_Impl::RegistryMaxRecursiveDepth(this) {
                    Ok(ok__) => {
                        depth.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRegistryMaxRecursiveDepth<Identity: IConfigurationDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, depth: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IConfigurationDataCollector_Impl::SetRegistryMaxRecursiveDepth(this, core::mem::transmute_copy(&depth)).into()
            }
        }
        unsafe extern "system" fn SystemStateFile<Identity: IConfigurationDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IConfigurationDataCollector_Impl::SystemStateFile(this) {
                    Ok(ok__) => {
                        filename.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSystemStateFile<Identity: IConfigurationDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IConfigurationDataCollector_Impl::SetSystemStateFile(this, core::mem::transmute(&filename)).into()
            }
        }
        Self {
            base__: IDataCollector_Vtbl::new::<Identity, OFFSET>(),
            FileMaxCount: FileMaxCount::<Identity, OFFSET>,
            SetFileMaxCount: SetFileMaxCount::<Identity, OFFSET>,
            FileMaxRecursiveDepth: FileMaxRecursiveDepth::<Identity, OFFSET>,
            SetFileMaxRecursiveDepth: SetFileMaxRecursiveDepth::<Identity, OFFSET>,
            FileMaxTotalSize: FileMaxTotalSize::<Identity, OFFSET>,
            SetFileMaxTotalSize: SetFileMaxTotalSize::<Identity, OFFSET>,
            Files: Files::<Identity, OFFSET>,
            SetFiles: SetFiles::<Identity, OFFSET>,
            ManagementQueries: ManagementQueries::<Identity, OFFSET>,
            SetManagementQueries: SetManagementQueries::<Identity, OFFSET>,
            QueryNetworkAdapters: QueryNetworkAdapters::<Identity, OFFSET>,
            SetQueryNetworkAdapters: SetQueryNetworkAdapters::<Identity, OFFSET>,
            RegistryKeys: RegistryKeys::<Identity, OFFSET>,
            SetRegistryKeys: SetRegistryKeys::<Identity, OFFSET>,
            RegistryMaxRecursiveDepth: RegistryMaxRecursiveDepth::<Identity, OFFSET>,
            SetRegistryMaxRecursiveDepth: SetRegistryMaxRecursiveDepth::<Identity, OFFSET>,
            SystemStateFile: SystemStateFile::<Identity, OFFSET>,
            SetSystemStateFile: SetSystemStateFile::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IConfigurationDataCollector as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<IDataCollector as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IConfigurationDataCollector {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IDataCollector, IDataCollector_Vtbl, 0x038374ff_098b_11d8_9414_505054503030);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IDataCollector {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IDataCollector, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "oaidl")]
impl IDataCollector {
    pub unsafe fn DataCollectorSet(&self) -> windows_core::Result<IDataCollectorSet> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DataCollectorSet)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetDataCollectorSet<P0>(&self, group: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDataCollectorSet>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetDataCollectorSet)(windows_core::Interface::as_raw(self), group.param().abi()) }
    }
    pub unsafe fn DataCollectorType(&self) -> windows_core::Result<DataCollectorType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DataCollectorType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn FileName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FileName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetFileName(&self, name: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFileName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name)) }
    }
    pub unsafe fn FileNameFormat(&self) -> windows_core::Result<AutoPathFormat> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FileNameFormat)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetFileNameFormat(&self, format: AutoPathFormat) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFileNameFormat)(windows_core::Interface::as_raw(self), format) }
    }
    pub unsafe fn FileNameFormatPattern(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FileNameFormatPattern)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetFileNameFormatPattern(&self, pattern: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFileNameFormatPattern)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(pattern)) }
    }
    pub unsafe fn LatestOutputLocation(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LatestOutputLocation)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetLatestOutputLocation(&self, path: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetLatestOutputLocation)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(path)) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn LogAppend(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LogAppend)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetLogAppend(&self, append: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetLogAppend)(windows_core::Interface::as_raw(self), append) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn LogCircular(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LogCircular)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetLogCircular(&self, circular: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetLogCircular)(windows_core::Interface::as_raw(self), circular) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn LogOverwrite(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LogOverwrite)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetLogOverwrite(&self, overwrite: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetLogOverwrite)(windows_core::Interface::as_raw(self), overwrite) }
    }
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetName(&self, name: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name)) }
    }
    pub unsafe fn OutputLocation(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OutputLocation)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Index(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Index)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetIndex(&self, index: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetIndex)(windows_core::Interface::as_raw(self), index) }
    }
    pub unsafe fn Xml(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Xml)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetXml(&self, xml: &windows_core::BSTR) -> windows_core::Result<IValueMap> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SetXml)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(xml), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn CreateOutputLocation(&self, latest: super::wtypes::VARIANT_BOOL) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateOutputLocation)(windows_core::Interface::as_raw(self), latest, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IDataCollector_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub DataCollectorSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDataCollectorSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DataCollectorType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DataCollectorType) -> windows_core::HRESULT,
    pub FileName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetFileName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FileNameFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AutoPathFormat) -> windows_core::HRESULT,
    pub SetFileNameFormat: unsafe extern "system" fn(*mut core::ffi::c_void, AutoPathFormat) -> windows_core::HRESULT,
    pub FileNameFormatPattern: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetFileNameFormatPattern: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LatestOutputLocation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetLatestOutputLocation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub LogAppend: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    LogAppend: usize,
    #[cfg(feature = "wtypes")]
    pub SetLogAppend: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetLogAppend: usize,
    #[cfg(feature = "wtypes")]
    pub LogCircular: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    LogCircular: usize,
    #[cfg(feature = "wtypes")]
    pub SetLogCircular: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetLogCircular: usize,
    #[cfg(feature = "wtypes")]
    pub LogOverwrite: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    LogOverwrite: usize,
    #[cfg(feature = "wtypes")]
    pub SetLogOverwrite: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetLogOverwrite: usize,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OutputLocation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Index: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetIndex: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Xml: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetXml: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub CreateOutputLocation: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    CreateOutputLocation: usize,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IDataCollector_Impl: super::oaidl::IDispatch_Impl {
    fn DataCollectorSet(&self) -> windows_core::Result<IDataCollectorSet>;
    fn SetDataCollectorSet(&self, group: windows_core::Ref<IDataCollectorSet>) -> windows_core::Result<()>;
    fn DataCollectorType(&self) -> windows_core::Result<DataCollectorType>;
    fn FileName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetFileName(&self, name: &windows_core::BSTR) -> windows_core::Result<()>;
    fn FileNameFormat(&self) -> windows_core::Result<AutoPathFormat>;
    fn SetFileNameFormat(&self, format: AutoPathFormat) -> windows_core::Result<()>;
    fn FileNameFormatPattern(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetFileNameFormatPattern(&self, pattern: &windows_core::BSTR) -> windows_core::Result<()>;
    fn LatestOutputLocation(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetLatestOutputLocation(&self, path: &windows_core::BSTR) -> windows_core::Result<()>;
    fn LogAppend(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn SetLogAppend(&self, append: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn LogCircular(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn SetLogCircular(&self, circular: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn LogOverwrite(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn SetLogOverwrite(&self, overwrite: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetName(&self, name: &windows_core::BSTR) -> windows_core::Result<()>;
    fn OutputLocation(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Index(&self) -> windows_core::Result<i32>;
    fn SetIndex(&self, index: i32) -> windows_core::Result<()>;
    fn Xml(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetXml(&self, xml: &windows_core::BSTR) -> windows_core::Result<IValueMap>;
    fn CreateOutputLocation(&self, latest: super::wtypes::VARIANT_BOOL) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IDataCollector_Vtbl {
    pub const fn new<Identity: IDataCollector_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DataCollectorSet<Identity: IDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, group: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollector_Impl::DataCollectorSet(this) {
                    Ok(ok__) => {
                        group.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDataCollectorSet<Identity: IDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, group: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollector_Impl::SetDataCollectorSet(this, core::mem::transmute_copy(&group)).into()
            }
        }
        unsafe extern "system" fn DataCollectorType<Identity: IDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: *mut DataCollectorType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollector_Impl::DataCollectorType(this) {
                    Ok(ok__) => {
                        r#type.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FileName<Identity: IDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollector_Impl::FileName(this) {
                    Ok(ok__) => {
                        name.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFileName<Identity: IDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollector_Impl::SetFileName(this, core::mem::transmute(&name)).into()
            }
        }
        unsafe extern "system" fn FileNameFormat<Identity: IDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: *mut AutoPathFormat) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollector_Impl::FileNameFormat(this) {
                    Ok(ok__) => {
                        format.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFileNameFormat<Identity: IDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: AutoPathFormat) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollector_Impl::SetFileNameFormat(this, core::mem::transmute_copy(&format)).into()
            }
        }
        unsafe extern "system" fn FileNameFormatPattern<Identity: IDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pattern: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollector_Impl::FileNameFormatPattern(this) {
                    Ok(ok__) => {
                        pattern.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFileNameFormatPattern<Identity: IDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pattern: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollector_Impl::SetFileNameFormatPattern(this, core::mem::transmute(&pattern)).into()
            }
        }
        unsafe extern "system" fn LatestOutputLocation<Identity: IDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollector_Impl::LatestOutputLocation(this) {
                    Ok(ok__) => {
                        path.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLatestOutputLocation<Identity: IDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollector_Impl::SetLatestOutputLocation(this, core::mem::transmute(&path)).into()
            }
        }
        unsafe extern "system" fn LogAppend<Identity: IDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, append: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollector_Impl::LogAppend(this) {
                    Ok(ok__) => {
                        append.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLogAppend<Identity: IDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, append: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollector_Impl::SetLogAppend(this, core::mem::transmute_copy(&append)).into()
            }
        }
        unsafe extern "system" fn LogCircular<Identity: IDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, circular: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollector_Impl::LogCircular(this) {
                    Ok(ok__) => {
                        circular.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLogCircular<Identity: IDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, circular: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollector_Impl::SetLogCircular(this, core::mem::transmute_copy(&circular)).into()
            }
        }
        unsafe extern "system" fn LogOverwrite<Identity: IDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, overwrite: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollector_Impl::LogOverwrite(this) {
                    Ok(ok__) => {
                        overwrite.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLogOverwrite<Identity: IDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, overwrite: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollector_Impl::SetLogOverwrite(this, core::mem::transmute_copy(&overwrite)).into()
            }
        }
        unsafe extern "system" fn Name<Identity: IDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollector_Impl::Name(this) {
                    Ok(ok__) => {
                        name.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetName<Identity: IDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollector_Impl::SetName(this, core::mem::transmute(&name)).into()
            }
        }
        unsafe extern "system" fn OutputLocation<Identity: IDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollector_Impl::OutputLocation(this) {
                    Ok(ok__) => {
                        path.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Index<Identity: IDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollector_Impl::Index(this) {
                    Ok(ok__) => {
                        index.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIndex<Identity: IDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollector_Impl::SetIndex(this, core::mem::transmute_copy(&index)).into()
            }
        }
        unsafe extern "system" fn Xml<Identity: IDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, xml: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollector_Impl::Xml(this) {
                    Ok(ok__) => {
                        xml.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetXml<Identity: IDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, xml: *mut core::ffi::c_void, validation: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollector_Impl::SetXml(this, core::mem::transmute(&xml)) {
                    Ok(ok__) => {
                        validation.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateOutputLocation<Identity: IDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, latest: super::wtypes::VARIANT_BOOL, location: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollector_Impl::CreateOutputLocation(this, core::mem::transmute_copy(&latest)) {
                    Ok(ok__) => {
                        location.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            DataCollectorSet: DataCollectorSet::<Identity, OFFSET>,
            SetDataCollectorSet: SetDataCollectorSet::<Identity, OFFSET>,
            DataCollectorType: DataCollectorType::<Identity, OFFSET>,
            FileName: FileName::<Identity, OFFSET>,
            SetFileName: SetFileName::<Identity, OFFSET>,
            FileNameFormat: FileNameFormat::<Identity, OFFSET>,
            SetFileNameFormat: SetFileNameFormat::<Identity, OFFSET>,
            FileNameFormatPattern: FileNameFormatPattern::<Identity, OFFSET>,
            SetFileNameFormatPattern: SetFileNameFormatPattern::<Identity, OFFSET>,
            LatestOutputLocation: LatestOutputLocation::<Identity, OFFSET>,
            SetLatestOutputLocation: SetLatestOutputLocation::<Identity, OFFSET>,
            LogAppend: LogAppend::<Identity, OFFSET>,
            SetLogAppend: SetLogAppend::<Identity, OFFSET>,
            LogCircular: LogCircular::<Identity, OFFSET>,
            SetLogCircular: SetLogCircular::<Identity, OFFSET>,
            LogOverwrite: LogOverwrite::<Identity, OFFSET>,
            SetLogOverwrite: SetLogOverwrite::<Identity, OFFSET>,
            Name: Name::<Identity, OFFSET>,
            SetName: SetName::<Identity, OFFSET>,
            OutputLocation: OutputLocation::<Identity, OFFSET>,
            Index: Index::<Identity, OFFSET>,
            SetIndex: SetIndex::<Identity, OFFSET>,
            Xml: Xml::<Identity, OFFSET>,
            SetXml: SetXml::<Identity, OFFSET>,
            CreateOutputLocation: CreateOutputLocation::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDataCollector as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IDataCollector {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IDataCollectorCollection, IDataCollectorCollection_Vtbl, 0x03837502_098b_11d8_9414_505054503030);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IDataCollectorCollection {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IDataCollectorCollection, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "oaidl")]
impl IDataCollectorCollection {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn Item(&self, index: &super::oaidl::VARIANT) -> windows_core::Result<IDataCollector> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(index), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Add<P0>(&self, collector: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDataCollector>,
    {
        unsafe { (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), collector.param().abi()) }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn Remove(&self, collector: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(collector)) }
    }
    pub unsafe fn Clear(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AddRange<P0>(&self, collectors: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddRange)(windows_core::Interface::as_raw(self), collectors.param().abi()) }
    }
    pub unsafe fn CreateDataCollectorFromXml(&self, bstrxml: &windows_core::BSTR, pvalidation: *mut Option<IValueMap>) -> windows_core::Result<IDataCollector> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateDataCollectorFromXml)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrxml), core::mem::transmute(pvalidation), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateDataCollector(&self, r#type: DataCollectorType) -> windows_core::Result<IDataCollector> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateDataCollector)(windows_core::Interface::as_raw(self), r#type, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IDataCollectorCollection_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    Item: usize,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    Remove: usize,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateDataCollectorFromXml: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateDataCollector: unsafe extern "system" fn(*mut core::ffi::c_void, DataCollectorType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IDataCollectorCollection_Impl: super::oaidl::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn Item(&self, index: &super::oaidl::VARIANT) -> windows_core::Result<IDataCollector>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Add(&self, collector: windows_core::Ref<IDataCollector>) -> windows_core::Result<()>;
    fn Remove(&self, collector: &super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
    fn AddRange(&self, collectors: windows_core::Ref<IDataCollectorCollection>) -> windows_core::Result<()>;
    fn CreateDataCollectorFromXml(&self, bstrxml: &windows_core::BSTR, pvalidation: windows_core::OutRef<IValueMap>) -> windows_core::Result<IDataCollector>;
    fn CreateDataCollector(&self, r#type: DataCollectorType) -> windows_core::Result<IDataCollector>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IDataCollectorCollection_Vtbl {
    pub const fn new<Identity: IDataCollectorCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Count<Identity: IDataCollectorCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorCollection_Impl::Count(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Item<Identity: IDataCollectorCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::oaidl::VARIANT, collector: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorCollection_Impl::Item(this, core::mem::transmute(&index)) {
                    Ok(ok__) => {
                        collector.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: IDataCollectorCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorCollection_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Add<Identity: IDataCollectorCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, collector: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorCollection_Impl::Add(this, core::mem::transmute_copy(&collector)).into()
            }
        }
        unsafe extern "system" fn Remove<Identity: IDataCollectorCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, collector: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorCollection_Impl::Remove(this, core::mem::transmute(&collector)).into()
            }
        }
        unsafe extern "system" fn Clear<Identity: IDataCollectorCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorCollection_Impl::Clear(this).into()
            }
        }
        unsafe extern "system" fn AddRange<Identity: IDataCollectorCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, collectors: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorCollection_Impl::AddRange(this, core::mem::transmute_copy(&collectors)).into()
            }
        }
        unsafe extern "system" fn CreateDataCollectorFromXml<Identity: IDataCollectorCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrxml: *mut core::ffi::c_void, pvalidation: *mut *mut core::ffi::c_void, pcollector: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorCollection_Impl::CreateDataCollectorFromXml(this, core::mem::transmute(&bstrxml), core::mem::transmute_copy(&pvalidation)) {
                    Ok(ok__) => {
                        pcollector.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateDataCollector<Identity: IDataCollectorCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: DataCollectorType, collector: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorCollection_Impl::CreateDataCollector(this, core::mem::transmute_copy(&r#type)) {
                    Ok(ok__) => {
                        collector.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
            AddRange: AddRange::<Identity, OFFSET>,
            CreateDataCollectorFromXml: CreateDataCollectorFromXml::<Identity, OFFSET>,
            CreateDataCollector: CreateDataCollector::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDataCollectorCollection as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IDataCollectorCollection {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IDataCollectorSet, IDataCollectorSet_Vtbl, 0x03837520_098b_11d8_9414_505054503030);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IDataCollectorSet {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IDataCollectorSet, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "oaidl")]
impl IDataCollectorSet {
    pub unsafe fn DataCollectors(&self) -> windows_core::Result<IDataCollectorCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DataCollectors)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Duration(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Duration)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetDuration(&self, seconds: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDuration)(windows_core::Interface::as_raw(self), seconds) }
    }
    pub unsafe fn Description(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetDescription(&self, description: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDescription)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(description)) }
    }
    pub unsafe fn DescriptionUnresolved(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DescriptionUnresolved)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn DisplayName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DisplayName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetDisplayName(&self, displayname: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDisplayName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(displayname)) }
    }
    pub unsafe fn DisplayNameUnresolved(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DisplayNameUnresolved)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Keywords(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Keywords)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetKeywords(&self, keywords: *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetKeywords)(windows_core::Interface::as_raw(self), keywords) }
    }
    pub unsafe fn LatestOutputLocation(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LatestOutputLocation)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetLatestOutputLocation(&self, path: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetLatestOutputLocation)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(path)) }
    }
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn OutputLocation(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OutputLocation)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn RootPath(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RootPath)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetRootPath(&self, folder: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRootPath)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(folder)) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn Segment(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Segment)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetSegment(&self, segment: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSegment)(windows_core::Interface::as_raw(self), segment) }
    }
    pub unsafe fn SegmentMaxDuration(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SegmentMaxDuration)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSegmentMaxDuration(&self, seconds: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSegmentMaxDuration)(windows_core::Interface::as_raw(self), seconds) }
    }
    pub unsafe fn SegmentMaxSize(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SegmentMaxSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSegmentMaxSize(&self, size: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSegmentMaxSize)(windows_core::Interface::as_raw(self), size) }
    }
    pub unsafe fn SerialNumber(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SerialNumber)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSerialNumber(&self, index: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSerialNumber)(windows_core::Interface::as_raw(self), index) }
    }
    pub unsafe fn Server(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Server)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Status(&self) -> windows_core::Result<DataCollectorSetStatus> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Status)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Subdirectory(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Subdirectory)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetSubdirectory(&self, folder: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSubdirectory)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(folder)) }
    }
    pub unsafe fn SubdirectoryFormat(&self) -> windows_core::Result<AutoPathFormat> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SubdirectoryFormat)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSubdirectoryFormat(&self, format: AutoPathFormat) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSubdirectoryFormat)(windows_core::Interface::as_raw(self), format) }
    }
    pub unsafe fn SubdirectoryFormatPattern(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SubdirectoryFormatPattern)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetSubdirectoryFormatPattern(&self, pattern: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSubdirectoryFormatPattern)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(pattern)) }
    }
    pub unsafe fn Task(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Task)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetTask(&self, task: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetTask)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(task)) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn TaskRunAsSelf(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TaskRunAsSelf)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetTaskRunAsSelf(&self, runasself: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetTaskRunAsSelf)(windows_core::Interface::as_raw(self), runasself) }
    }
    pub unsafe fn TaskArguments(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TaskArguments)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetTaskArguments(&self, task: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetTaskArguments)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(task)) }
    }
    pub unsafe fn TaskUserTextArguments(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TaskUserTextArguments)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetTaskUserTextArguments(&self, usertext: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetTaskUserTextArguments)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(usertext)) }
    }
    pub unsafe fn Schedules(&self) -> windows_core::Result<IScheduleCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Schedules)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SchedulesEnabled(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SchedulesEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetSchedulesEnabled(&self, enabled: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSchedulesEnabled)(windows_core::Interface::as_raw(self), enabled) }
    }
    pub unsafe fn UserAccount(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UserAccount)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Xml(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Xml)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Security(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Security)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetSecurity(&self, bstrsecurity: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSecurity)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrsecurity)) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn StopOnCompletion(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StopOnCompletion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetStopOnCompletion(&self, stop: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetStopOnCompletion)(windows_core::Interface::as_raw(self), stop) }
    }
    pub unsafe fn DataManager(&self) -> windows_core::Result<IDataManager> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DataManager)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetCredentials(&self, user: &windows_core::BSTR, password: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCredentials)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(user), core::mem::transmute_copy(password)) }
    }
    pub unsafe fn Query(&self, name: &windows_core::BSTR, server: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Query)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name), core::mem::transmute_copy(server)) }
    }
    pub unsafe fn Commit(&self, name: &windows_core::BSTR, server: &windows_core::BSTR, mode: CommitMode) -> windows_core::Result<IValueMap> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Commit)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name), core::mem::transmute_copy(server), mode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Delete(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn Start(&self, synchronous: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Start)(windows_core::Interface::as_raw(self), synchronous) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn Stop(&self, synchronous: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Stop)(windows_core::Interface::as_raw(self), synchronous) }
    }
    pub unsafe fn SetXml(&self, xml: &windows_core::BSTR) -> windows_core::Result<IValueMap> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SetXml)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(xml), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetValue(&self, key: &windows_core::BSTR, value: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetValue)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(key), core::mem::transmute_copy(value)) }
    }
    pub unsafe fn GetValue(&self, key: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetValue)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(key), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IDataCollectorSet_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub DataCollectors: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Duration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetDuration: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DescriptionUnresolved: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DisplayNameUnresolved: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Keywords: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    pub SetKeywords: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    pub LatestOutputLocation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetLatestOutputLocation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OutputLocation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RootPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetRootPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub Segment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    Segment: usize,
    #[cfg(feature = "wtypes")]
    pub SetSegment: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetSegment: usize,
    pub SegmentMaxDuration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetSegmentMaxDuration: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SegmentMaxSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetSegmentMaxSize: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SerialNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetSerialNumber: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Server: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DataCollectorSetStatus) -> windows_core::HRESULT,
    pub Subdirectory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSubdirectory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SubdirectoryFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AutoPathFormat) -> windows_core::HRESULT,
    pub SetSubdirectoryFormat: unsafe extern "system" fn(*mut core::ffi::c_void, AutoPathFormat) -> windows_core::HRESULT,
    pub SubdirectoryFormatPattern: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSubdirectoryFormatPattern: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Task: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTask: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub TaskRunAsSelf: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    TaskRunAsSelf: usize,
    #[cfg(feature = "wtypes")]
    pub SetTaskRunAsSelf: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetTaskRunAsSelf: usize,
    pub TaskArguments: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTaskArguments: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TaskUserTextArguments: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTaskUserTextArguments: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Schedules: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub SchedulesEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SchedulesEnabled: usize,
    #[cfg(feature = "wtypes")]
    pub SetSchedulesEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetSchedulesEnabled: usize,
    pub UserAccount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Xml: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Security: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSecurity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub StopOnCompletion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    StopOnCompletion: usize,
    #[cfg(feature = "wtypes")]
    pub SetStopOnCompletion: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetStopOnCompletion: usize,
    pub DataManager: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCredentials: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Query: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Commit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, CommitMode, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    Start: usize,
    #[cfg(feature = "wtypes")]
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    Stop: usize,
    pub SetXml: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IDataCollectorSet_Impl: super::oaidl::IDispatch_Impl {
    fn DataCollectors(&self) -> windows_core::Result<IDataCollectorCollection>;
    fn Duration(&self) -> windows_core::Result<u32>;
    fn SetDuration(&self, seconds: u32) -> windows_core::Result<()>;
    fn Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDescription(&self, description: &windows_core::BSTR) -> windows_core::Result<()>;
    fn DescriptionUnresolved(&self) -> windows_core::Result<windows_core::BSTR>;
    fn DisplayName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDisplayName(&self, displayname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn DisplayNameUnresolved(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Keywords(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY>;
    fn SetKeywords(&self, keywords: *const super::oaidl::SAFEARRAY) -> windows_core::Result<()>;
    fn LatestOutputLocation(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetLatestOutputLocation(&self, path: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn OutputLocation(&self) -> windows_core::Result<windows_core::BSTR>;
    fn RootPath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetRootPath(&self, folder: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Segment(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn SetSegment(&self, segment: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SegmentMaxDuration(&self) -> windows_core::Result<u32>;
    fn SetSegmentMaxDuration(&self, seconds: u32) -> windows_core::Result<()>;
    fn SegmentMaxSize(&self) -> windows_core::Result<u32>;
    fn SetSegmentMaxSize(&self, size: u32) -> windows_core::Result<()>;
    fn SerialNumber(&self) -> windows_core::Result<u32>;
    fn SetSerialNumber(&self, index: u32) -> windows_core::Result<()>;
    fn Server(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Status(&self) -> windows_core::Result<DataCollectorSetStatus>;
    fn Subdirectory(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetSubdirectory(&self, folder: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SubdirectoryFormat(&self) -> windows_core::Result<AutoPathFormat>;
    fn SetSubdirectoryFormat(&self, format: AutoPathFormat) -> windows_core::Result<()>;
    fn SubdirectoryFormatPattern(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetSubdirectoryFormatPattern(&self, pattern: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Task(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetTask(&self, task: &windows_core::BSTR) -> windows_core::Result<()>;
    fn TaskRunAsSelf(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn SetTaskRunAsSelf(&self, runasself: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn TaskArguments(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetTaskArguments(&self, task: &windows_core::BSTR) -> windows_core::Result<()>;
    fn TaskUserTextArguments(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetTaskUserTextArguments(&self, usertext: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Schedules(&self) -> windows_core::Result<IScheduleCollection>;
    fn SchedulesEnabled(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn SetSchedulesEnabled(&self, enabled: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn UserAccount(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Xml(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Security(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetSecurity(&self, bstrsecurity: &windows_core::BSTR) -> windows_core::Result<()>;
    fn StopOnCompletion(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn SetStopOnCompletion(&self, stop: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn DataManager(&self) -> windows_core::Result<IDataManager>;
    fn SetCredentials(&self, user: &windows_core::BSTR, password: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Query(&self, name: &windows_core::BSTR, server: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Commit(&self, name: &windows_core::BSTR, server: &windows_core::BSTR, mode: CommitMode) -> windows_core::Result<IValueMap>;
    fn Delete(&self) -> windows_core::Result<()>;
    fn Start(&self, synchronous: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Stop(&self, synchronous: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetXml(&self, xml: &windows_core::BSTR) -> windows_core::Result<IValueMap>;
    fn SetValue(&self, key: &windows_core::BSTR, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetValue(&self, key: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IDataCollectorSet_Vtbl {
    pub const fn new<Identity: IDataCollectorSet_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DataCollectors<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, collectors: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::DataCollectors(this) {
                    Ok(ok__) => {
                        collectors.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Duration<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, seconds: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::Duration(this) {
                    Ok(ok__) => {
                        seconds.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDuration<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, seconds: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorSet_Impl::SetDuration(this, core::mem::transmute_copy(&seconds)).into()
            }
        }
        unsafe extern "system" fn Description<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, description: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::Description(this) {
                    Ok(ok__) => {
                        description.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDescription<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, description: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorSet_Impl::SetDescription(this, core::mem::transmute(&description)).into()
            }
        }
        unsafe extern "system" fn DescriptionUnresolved<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, descr: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::DescriptionUnresolved(this) {
                    Ok(ok__) => {
                        descr.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DisplayName<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, displayname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::DisplayName(this) {
                    Ok(ok__) => {
                        displayname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDisplayName<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, displayname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorSet_Impl::SetDisplayName(this, core::mem::transmute(&displayname)).into()
            }
        }
        unsafe extern "system" fn DisplayNameUnresolved<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::DisplayNameUnresolved(this) {
                    Ok(ok__) => {
                        name.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Keywords<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, keywords: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::Keywords(this) {
                    Ok(ok__) => {
                        keywords.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetKeywords<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, keywords: *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorSet_Impl::SetKeywords(this, core::mem::transmute_copy(&keywords)).into()
            }
        }
        unsafe extern "system" fn LatestOutputLocation<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::LatestOutputLocation(this) {
                    Ok(ok__) => {
                        path.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLatestOutputLocation<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorSet_Impl::SetLatestOutputLocation(this, core::mem::transmute(&path)).into()
            }
        }
        unsafe extern "system" fn Name<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::Name(this) {
                    Ok(ok__) => {
                        name.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OutputLocation<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::OutputLocation(this) {
                    Ok(ok__) => {
                        path.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RootPath<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, folder: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::RootPath(this) {
                    Ok(ok__) => {
                        folder.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRootPath<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, folder: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorSet_Impl::SetRootPath(this, core::mem::transmute(&folder)).into()
            }
        }
        unsafe extern "system" fn Segment<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, segment: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::Segment(this) {
                    Ok(ok__) => {
                        segment.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSegment<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, segment: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorSet_Impl::SetSegment(this, core::mem::transmute_copy(&segment)).into()
            }
        }
        unsafe extern "system" fn SegmentMaxDuration<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, seconds: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::SegmentMaxDuration(this) {
                    Ok(ok__) => {
                        seconds.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSegmentMaxDuration<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, seconds: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorSet_Impl::SetSegmentMaxDuration(this, core::mem::transmute_copy(&seconds)).into()
            }
        }
        unsafe extern "system" fn SegmentMaxSize<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, size: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::SegmentMaxSize(this) {
                    Ok(ok__) => {
                        size.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSegmentMaxSize<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, size: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorSet_Impl::SetSegmentMaxSize(this, core::mem::transmute_copy(&size)).into()
            }
        }
        unsafe extern "system" fn SerialNumber<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::SerialNumber(this) {
                    Ok(ok__) => {
                        index.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSerialNumber<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorSet_Impl::SetSerialNumber(this, core::mem::transmute_copy(&index)).into()
            }
        }
        unsafe extern "system" fn Server<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, server: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::Server(this) {
                    Ok(ok__) => {
                        server.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Status<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, status: *mut DataCollectorSetStatus) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::Status(this) {
                    Ok(ok__) => {
                        status.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Subdirectory<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, folder: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::Subdirectory(this) {
                    Ok(ok__) => {
                        folder.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSubdirectory<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, folder: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorSet_Impl::SetSubdirectory(this, core::mem::transmute(&folder)).into()
            }
        }
        unsafe extern "system" fn SubdirectoryFormat<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: *mut AutoPathFormat) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::SubdirectoryFormat(this) {
                    Ok(ok__) => {
                        format.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSubdirectoryFormat<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: AutoPathFormat) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorSet_Impl::SetSubdirectoryFormat(this, core::mem::transmute_copy(&format)).into()
            }
        }
        unsafe extern "system" fn SubdirectoryFormatPattern<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pattern: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::SubdirectoryFormatPattern(this) {
                    Ok(ok__) => {
                        pattern.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSubdirectoryFormatPattern<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pattern: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorSet_Impl::SetSubdirectoryFormatPattern(this, core::mem::transmute(&pattern)).into()
            }
        }
        unsafe extern "system" fn Task<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, task: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::Task(this) {
                    Ok(ok__) => {
                        task.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTask<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, task: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorSet_Impl::SetTask(this, core::mem::transmute(&task)).into()
            }
        }
        unsafe extern "system" fn TaskRunAsSelf<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, runasself: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::TaskRunAsSelf(this) {
                    Ok(ok__) => {
                        runasself.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTaskRunAsSelf<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, runasself: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorSet_Impl::SetTaskRunAsSelf(this, core::mem::transmute_copy(&runasself)).into()
            }
        }
        unsafe extern "system" fn TaskArguments<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, task: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::TaskArguments(this) {
                    Ok(ok__) => {
                        task.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTaskArguments<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, task: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorSet_Impl::SetTaskArguments(this, core::mem::transmute(&task)).into()
            }
        }
        unsafe extern "system" fn TaskUserTextArguments<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, usertext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::TaskUserTextArguments(this) {
                    Ok(ok__) => {
                        usertext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTaskUserTextArguments<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, usertext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorSet_Impl::SetTaskUserTextArguments(this, core::mem::transmute(&usertext)).into()
            }
        }
        unsafe extern "system" fn Schedules<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppschedules: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::Schedules(this) {
                    Ok(ok__) => {
                        ppschedules.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SchedulesEnabled<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::SchedulesEnabled(this) {
                    Ok(ok__) => {
                        enabled.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSchedulesEnabled<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorSet_Impl::SetSchedulesEnabled(this, core::mem::transmute_copy(&enabled)).into()
            }
        }
        unsafe extern "system" fn UserAccount<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, user: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::UserAccount(this) {
                    Ok(ok__) => {
                        user.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Xml<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, xml: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::Xml(this) {
                    Ok(ok__) => {
                        xml.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Security<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrsecurity: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::Security(this) {
                    Ok(ok__) => {
                        pbstrsecurity.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSecurity<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrsecurity: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorSet_Impl::SetSecurity(this, core::mem::transmute(&bstrsecurity)).into()
            }
        }
        unsafe extern "system" fn StopOnCompletion<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stop: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::StopOnCompletion(this) {
                    Ok(ok__) => {
                        stop.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetStopOnCompletion<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stop: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorSet_Impl::SetStopOnCompletion(this, core::mem::transmute_copy(&stop)).into()
            }
        }
        unsafe extern "system" fn DataManager<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, datamanager: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::DataManager(this) {
                    Ok(ok__) => {
                        datamanager.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCredentials<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, user: *mut core::ffi::c_void, password: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorSet_Impl::SetCredentials(this, core::mem::transmute(&user), core::mem::transmute(&password)).into()
            }
        }
        unsafe extern "system" fn Query<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void, server: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorSet_Impl::Query(this, core::mem::transmute(&name), core::mem::transmute(&server)).into()
            }
        }
        unsafe extern "system" fn Commit<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void, server: *mut core::ffi::c_void, mode: CommitMode, validation: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::Commit(this, core::mem::transmute(&name), core::mem::transmute(&server), core::mem::transmute_copy(&mode)) {
                    Ok(ok__) => {
                        validation.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Delete<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorSet_Impl::Delete(this).into()
            }
        }
        unsafe extern "system" fn Start<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, synchronous: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorSet_Impl::Start(this, core::mem::transmute_copy(&synchronous)).into()
            }
        }
        unsafe extern "system" fn Stop<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, synchronous: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorSet_Impl::Stop(this, core::mem::transmute_copy(&synchronous)).into()
            }
        }
        unsafe extern "system" fn SetXml<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, xml: *mut core::ffi::c_void, validation: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::SetXml(this, core::mem::transmute(&xml)) {
                    Ok(ok__) => {
                        validation.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetValue<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorSet_Impl::SetValue(this, core::mem::transmute(&key), core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn GetValue<Identity: IDataCollectorSet_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSet_Impl::GetValue(this, core::mem::transmute(&key)) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            DataCollectors: DataCollectors::<Identity, OFFSET>,
            Duration: Duration::<Identity, OFFSET>,
            SetDuration: SetDuration::<Identity, OFFSET>,
            Description: Description::<Identity, OFFSET>,
            SetDescription: SetDescription::<Identity, OFFSET>,
            DescriptionUnresolved: DescriptionUnresolved::<Identity, OFFSET>,
            DisplayName: DisplayName::<Identity, OFFSET>,
            SetDisplayName: SetDisplayName::<Identity, OFFSET>,
            DisplayNameUnresolved: DisplayNameUnresolved::<Identity, OFFSET>,
            Keywords: Keywords::<Identity, OFFSET>,
            SetKeywords: SetKeywords::<Identity, OFFSET>,
            LatestOutputLocation: LatestOutputLocation::<Identity, OFFSET>,
            SetLatestOutputLocation: SetLatestOutputLocation::<Identity, OFFSET>,
            Name: Name::<Identity, OFFSET>,
            OutputLocation: OutputLocation::<Identity, OFFSET>,
            RootPath: RootPath::<Identity, OFFSET>,
            SetRootPath: SetRootPath::<Identity, OFFSET>,
            Segment: Segment::<Identity, OFFSET>,
            SetSegment: SetSegment::<Identity, OFFSET>,
            SegmentMaxDuration: SegmentMaxDuration::<Identity, OFFSET>,
            SetSegmentMaxDuration: SetSegmentMaxDuration::<Identity, OFFSET>,
            SegmentMaxSize: SegmentMaxSize::<Identity, OFFSET>,
            SetSegmentMaxSize: SetSegmentMaxSize::<Identity, OFFSET>,
            SerialNumber: SerialNumber::<Identity, OFFSET>,
            SetSerialNumber: SetSerialNumber::<Identity, OFFSET>,
            Server: Server::<Identity, OFFSET>,
            Status: Status::<Identity, OFFSET>,
            Subdirectory: Subdirectory::<Identity, OFFSET>,
            SetSubdirectory: SetSubdirectory::<Identity, OFFSET>,
            SubdirectoryFormat: SubdirectoryFormat::<Identity, OFFSET>,
            SetSubdirectoryFormat: SetSubdirectoryFormat::<Identity, OFFSET>,
            SubdirectoryFormatPattern: SubdirectoryFormatPattern::<Identity, OFFSET>,
            SetSubdirectoryFormatPattern: SetSubdirectoryFormatPattern::<Identity, OFFSET>,
            Task: Task::<Identity, OFFSET>,
            SetTask: SetTask::<Identity, OFFSET>,
            TaskRunAsSelf: TaskRunAsSelf::<Identity, OFFSET>,
            SetTaskRunAsSelf: SetTaskRunAsSelf::<Identity, OFFSET>,
            TaskArguments: TaskArguments::<Identity, OFFSET>,
            SetTaskArguments: SetTaskArguments::<Identity, OFFSET>,
            TaskUserTextArguments: TaskUserTextArguments::<Identity, OFFSET>,
            SetTaskUserTextArguments: SetTaskUserTextArguments::<Identity, OFFSET>,
            Schedules: Schedules::<Identity, OFFSET>,
            SchedulesEnabled: SchedulesEnabled::<Identity, OFFSET>,
            SetSchedulesEnabled: SetSchedulesEnabled::<Identity, OFFSET>,
            UserAccount: UserAccount::<Identity, OFFSET>,
            Xml: Xml::<Identity, OFFSET>,
            Security: Security::<Identity, OFFSET>,
            SetSecurity: SetSecurity::<Identity, OFFSET>,
            StopOnCompletion: StopOnCompletion::<Identity, OFFSET>,
            SetStopOnCompletion: SetStopOnCompletion::<Identity, OFFSET>,
            DataManager: DataManager::<Identity, OFFSET>,
            SetCredentials: SetCredentials::<Identity, OFFSET>,
            Query: Query::<Identity, OFFSET>,
            Commit: Commit::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            Start: Start::<Identity, OFFSET>,
            Stop: Stop::<Identity, OFFSET>,
            SetXml: SetXml::<Identity, OFFSET>,
            SetValue: SetValue::<Identity, OFFSET>,
            GetValue: GetValue::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDataCollectorSet as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IDataCollectorSet {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IDataCollectorSetCollection, IDataCollectorSetCollection_Vtbl, 0x03837524_098b_11d8_9414_505054503030);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IDataCollectorSetCollection {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IDataCollectorSetCollection, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "oaidl")]
impl IDataCollectorSetCollection {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn Item(&self, index: &super::oaidl::VARIANT) -> windows_core::Result<IDataCollectorSet> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(index), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Add<P0>(&self, set: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDataCollectorSet>,
    {
        unsafe { (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), set.param().abi()) }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn Remove(&self, set: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(set)) }
    }
    pub unsafe fn Clear(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AddRange<P0>(&self, sets: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddRange)(windows_core::Interface::as_raw(self), sets.param().abi()) }
    }
    pub unsafe fn GetDataCollectorSets(&self, server: &windows_core::BSTR, filter: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDataCollectorSets)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(server), core::mem::transmute_copy(filter)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IDataCollectorSetCollection_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    Item: usize,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    Remove: usize,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDataCollectorSets: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IDataCollectorSetCollection_Impl: super::oaidl::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn Item(&self, index: &super::oaidl::VARIANT) -> windows_core::Result<IDataCollectorSet>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Add(&self, set: windows_core::Ref<IDataCollectorSet>) -> windows_core::Result<()>;
    fn Remove(&self, set: &super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
    fn AddRange(&self, sets: windows_core::Ref<IDataCollectorSetCollection>) -> windows_core::Result<()>;
    fn GetDataCollectorSets(&self, server: &windows_core::BSTR, filter: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IDataCollectorSetCollection_Vtbl {
    pub const fn new<Identity: IDataCollectorSetCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Count<Identity: IDataCollectorSetCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSetCollection_Impl::Count(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Item<Identity: IDataCollectorSetCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::oaidl::VARIANT, set: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSetCollection_Impl::Item(this, core::mem::transmute(&index)) {
                    Ok(ok__) => {
                        set.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: IDataCollectorSetCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataCollectorSetCollection_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Add<Identity: IDataCollectorSetCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, set: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorSetCollection_Impl::Add(this, core::mem::transmute_copy(&set)).into()
            }
        }
        unsafe extern "system" fn Remove<Identity: IDataCollectorSetCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, set: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorSetCollection_Impl::Remove(this, core::mem::transmute(&set)).into()
            }
        }
        unsafe extern "system" fn Clear<Identity: IDataCollectorSetCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorSetCollection_Impl::Clear(this).into()
            }
        }
        unsafe extern "system" fn AddRange<Identity: IDataCollectorSetCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sets: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorSetCollection_Impl::AddRange(this, core::mem::transmute_copy(&sets)).into()
            }
        }
        unsafe extern "system" fn GetDataCollectorSets<Identity: IDataCollectorSetCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, server: *mut core::ffi::c_void, filter: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataCollectorSetCollection_Impl::GetDataCollectorSets(this, core::mem::transmute(&server), core::mem::transmute(&filter)).into()
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
            AddRange: AddRange::<Identity, OFFSET>,
            GetDataCollectorSets: GetDataCollectorSets::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDataCollectorSetCollection as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IDataCollectorSetCollection {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IDataManager, IDataManager_Vtbl, 0x03837541_098b_11d8_9414_505054503030);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IDataManager {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IDataManager, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "oaidl")]
impl IDataManager {
    #[cfg(feature = "wtypes")]
    pub unsafe fn Enabled(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Enabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetEnabled(&self, fenabled: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetEnabled)(windows_core::Interface::as_raw(self), fenabled) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn CheckBeforeRunning(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CheckBeforeRunning)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetCheckBeforeRunning(&self, fcheck: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCheckBeforeRunning)(windows_core::Interface::as_raw(self), fcheck) }
    }
    pub unsafe fn MinFreeDisk(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MinFreeDisk)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetMinFreeDisk(&self, minfreedisk: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMinFreeDisk)(windows_core::Interface::as_raw(self), minfreedisk) }
    }
    pub unsafe fn MaxSize(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MaxSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetMaxSize(&self, ulmaxsize: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMaxSize)(windows_core::Interface::as_raw(self), ulmaxsize) }
    }
    pub unsafe fn MaxFolderCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MaxFolderCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetMaxFolderCount(&self, ulmaxfoldercount: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMaxFolderCount)(windows_core::Interface::as_raw(self), ulmaxfoldercount) }
    }
    pub unsafe fn ResourcePolicy(&self) -> windows_core::Result<ResourcePolicy> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ResourcePolicy)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetResourcePolicy(&self, policy: ResourcePolicy) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetResourcePolicy)(windows_core::Interface::as_raw(self), policy) }
    }
    pub unsafe fn FolderActions(&self) -> windows_core::Result<IFolderActionCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FolderActions)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ReportSchema(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReportSchema)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetReportSchema(&self, reportschema: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetReportSchema)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(reportschema)) }
    }
    pub unsafe fn ReportFileName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReportFileName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetReportFileName(&self, pbstrfilename: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetReportFileName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(pbstrfilename)) }
    }
    pub unsafe fn RuleTargetFileName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RuleTargetFileName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetRuleTargetFileName(&self, filename: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRuleTargetFileName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(filename)) }
    }
    pub unsafe fn EventsFileName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EventsFileName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetEventsFileName(&self, pbstrfilename: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetEventsFileName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(pbstrfilename)) }
    }
    pub unsafe fn Rules(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Rules)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetRules(&self, bstrxml: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRules)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrxml)) }
    }
    pub unsafe fn Run(&self, steps: DataManagerSteps, bstrfolder: &windows_core::BSTR) -> windows_core::Result<IValueMap> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Run)(windows_core::Interface::as_raw(self), steps, core::mem::transmute_copy(bstrfolder), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Extract(&self, cabfilename: &windows_core::BSTR, destinationpath: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Extract)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(cabfilename), core::mem::transmute_copy(destinationpath)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IDataManager_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    #[cfg(feature = "wtypes")]
    pub Enabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    Enabled: usize,
    #[cfg(feature = "wtypes")]
    pub SetEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetEnabled: usize,
    #[cfg(feature = "wtypes")]
    pub CheckBeforeRunning: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    CheckBeforeRunning: usize,
    #[cfg(feature = "wtypes")]
    pub SetCheckBeforeRunning: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetCheckBeforeRunning: usize,
    pub MinFreeDisk: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetMinFreeDisk: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub MaxSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetMaxSize: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub MaxFolderCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetMaxFolderCount: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub ResourcePolicy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ResourcePolicy) -> windows_core::HRESULT,
    pub SetResourcePolicy: unsafe extern "system" fn(*mut core::ffi::c_void, ResourcePolicy) -> windows_core::HRESULT,
    pub FolderActions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReportSchema: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetReportSchema: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReportFileName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetReportFileName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RuleTargetFileName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetRuleTargetFileName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EventsFileName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetEventsFileName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Rules: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetRules: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Run: unsafe extern "system" fn(*mut core::ffi::c_void, DataManagerSteps, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Extract: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IDataManager_Impl: super::oaidl::IDispatch_Impl {
    fn Enabled(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn SetEnabled(&self, fenabled: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn CheckBeforeRunning(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn SetCheckBeforeRunning(&self, fcheck: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn MinFreeDisk(&self) -> windows_core::Result<u32>;
    fn SetMinFreeDisk(&self, minfreedisk: u32) -> windows_core::Result<()>;
    fn MaxSize(&self) -> windows_core::Result<u32>;
    fn SetMaxSize(&self, ulmaxsize: u32) -> windows_core::Result<()>;
    fn MaxFolderCount(&self) -> windows_core::Result<u32>;
    fn SetMaxFolderCount(&self, ulmaxfoldercount: u32) -> windows_core::Result<()>;
    fn ResourcePolicy(&self) -> windows_core::Result<ResourcePolicy>;
    fn SetResourcePolicy(&self, policy: ResourcePolicy) -> windows_core::Result<()>;
    fn FolderActions(&self) -> windows_core::Result<IFolderActionCollection>;
    fn ReportSchema(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetReportSchema(&self, reportschema: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ReportFileName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetReportFileName(&self, pbstrfilename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn RuleTargetFileName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetRuleTargetFileName(&self, filename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn EventsFileName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetEventsFileName(&self, pbstrfilename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Rules(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetRules(&self, bstrxml: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Run(&self, steps: DataManagerSteps, bstrfolder: &windows_core::BSTR) -> windows_core::Result<IValueMap>;
    fn Extract(&self, cabfilename: &windows_core::BSTR, destinationpath: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IDataManager_Vtbl {
    pub const fn new<Identity: IDataManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Enabled<Identity: IDataManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfenabled: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataManager_Impl::Enabled(this) {
                    Ok(ok__) => {
                        pfenabled.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: IDataManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fenabled: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataManager_Impl::SetEnabled(this, core::mem::transmute_copy(&fenabled)).into()
            }
        }
        unsafe extern "system" fn CheckBeforeRunning<Identity: IDataManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfcheck: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataManager_Impl::CheckBeforeRunning(this) {
                    Ok(ok__) => {
                        pfcheck.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCheckBeforeRunning<Identity: IDataManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fcheck: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataManager_Impl::SetCheckBeforeRunning(this, core::mem::transmute_copy(&fcheck)).into()
            }
        }
        unsafe extern "system" fn MinFreeDisk<Identity: IDataManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, minfreedisk: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataManager_Impl::MinFreeDisk(this) {
                    Ok(ok__) => {
                        minfreedisk.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMinFreeDisk<Identity: IDataManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, minfreedisk: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataManager_Impl::SetMinFreeDisk(this, core::mem::transmute_copy(&minfreedisk)).into()
            }
        }
        unsafe extern "system" fn MaxSize<Identity: IDataManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulmaxsize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataManager_Impl::MaxSize(this) {
                    Ok(ok__) => {
                        pulmaxsize.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMaxSize<Identity: IDataManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulmaxsize: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataManager_Impl::SetMaxSize(this, core::mem::transmute_copy(&ulmaxsize)).into()
            }
        }
        unsafe extern "system" fn MaxFolderCount<Identity: IDataManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulmaxfoldercount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataManager_Impl::MaxFolderCount(this) {
                    Ok(ok__) => {
                        pulmaxfoldercount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMaxFolderCount<Identity: IDataManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulmaxfoldercount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataManager_Impl::SetMaxFolderCount(this, core::mem::transmute_copy(&ulmaxfoldercount)).into()
            }
        }
        unsafe extern "system" fn ResourcePolicy<Identity: IDataManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppolicy: *mut ResourcePolicy) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataManager_Impl::ResourcePolicy(this) {
                    Ok(ok__) => {
                        ppolicy.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetResourcePolicy<Identity: IDataManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, policy: ResourcePolicy) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataManager_Impl::SetResourcePolicy(this, core::mem::transmute_copy(&policy)).into()
            }
        }
        unsafe extern "system" fn FolderActions<Identity: IDataManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, actions: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataManager_Impl::FolderActions(this) {
                    Ok(ok__) => {
                        actions.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReportSchema<Identity: IDataManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reportschema: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataManager_Impl::ReportSchema(this) {
                    Ok(ok__) => {
                        reportschema.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetReportSchema<Identity: IDataManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reportschema: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataManager_Impl::SetReportSchema(this, core::mem::transmute(&reportschema)).into()
            }
        }
        unsafe extern "system" fn ReportFileName<Identity: IDataManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrfilename: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataManager_Impl::ReportFileName(this) {
                    Ok(ok__) => {
                        pbstrfilename.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetReportFileName<Identity: IDataManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrfilename: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataManager_Impl::SetReportFileName(this, core::mem::transmute(&pbstrfilename)).into()
            }
        }
        unsafe extern "system" fn RuleTargetFileName<Identity: IDataManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataManager_Impl::RuleTargetFileName(this) {
                    Ok(ok__) => {
                        filename.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRuleTargetFileName<Identity: IDataManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataManager_Impl::SetRuleTargetFileName(this, core::mem::transmute(&filename)).into()
            }
        }
        unsafe extern "system" fn EventsFileName<Identity: IDataManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrfilename: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataManager_Impl::EventsFileName(this) {
                    Ok(ok__) => {
                        pbstrfilename.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEventsFileName<Identity: IDataManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrfilename: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataManager_Impl::SetEventsFileName(this, core::mem::transmute(&pbstrfilename)).into()
            }
        }
        unsafe extern "system" fn Rules<Identity: IDataManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrxml: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataManager_Impl::Rules(this) {
                    Ok(ok__) => {
                        pbstrxml.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRules<Identity: IDataManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrxml: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataManager_Impl::SetRules(this, core::mem::transmute(&bstrxml)).into()
            }
        }
        unsafe extern "system" fn Run<Identity: IDataManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, steps: DataManagerSteps, bstrfolder: *mut core::ffi::c_void, errors: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataManager_Impl::Run(this, core::mem::transmute_copy(&steps), core::mem::transmute(&bstrfolder)) {
                    Ok(ok__) => {
                        errors.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Extract<Identity: IDataManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cabfilename: *mut core::ffi::c_void, destinationpath: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataManager_Impl::Extract(this, core::mem::transmute(&cabfilename), core::mem::transmute(&destinationpath)).into()
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Enabled: Enabled::<Identity, OFFSET>,
            SetEnabled: SetEnabled::<Identity, OFFSET>,
            CheckBeforeRunning: CheckBeforeRunning::<Identity, OFFSET>,
            SetCheckBeforeRunning: SetCheckBeforeRunning::<Identity, OFFSET>,
            MinFreeDisk: MinFreeDisk::<Identity, OFFSET>,
            SetMinFreeDisk: SetMinFreeDisk::<Identity, OFFSET>,
            MaxSize: MaxSize::<Identity, OFFSET>,
            SetMaxSize: SetMaxSize::<Identity, OFFSET>,
            MaxFolderCount: MaxFolderCount::<Identity, OFFSET>,
            SetMaxFolderCount: SetMaxFolderCount::<Identity, OFFSET>,
            ResourcePolicy: ResourcePolicy::<Identity, OFFSET>,
            SetResourcePolicy: SetResourcePolicy::<Identity, OFFSET>,
            FolderActions: FolderActions::<Identity, OFFSET>,
            ReportSchema: ReportSchema::<Identity, OFFSET>,
            SetReportSchema: SetReportSchema::<Identity, OFFSET>,
            ReportFileName: ReportFileName::<Identity, OFFSET>,
            SetReportFileName: SetReportFileName::<Identity, OFFSET>,
            RuleTargetFileName: RuleTargetFileName::<Identity, OFFSET>,
            SetRuleTargetFileName: SetRuleTargetFileName::<Identity, OFFSET>,
            EventsFileName: EventsFileName::<Identity, OFFSET>,
            SetEventsFileName: SetEventsFileName::<Identity, OFFSET>,
            Rules: Rules::<Identity, OFFSET>,
            SetRules: SetRules::<Identity, OFFSET>,
            Run: Run::<Identity, OFFSET>,
            Extract: Extract::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDataManager as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IDataManager {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IFolderAction, IFolderAction_Vtbl, 0x03837543_098b_11d8_9414_505054503030);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IFolderAction {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IFolderAction, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "oaidl")]
impl IFolderAction {
    pub unsafe fn Age(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Age)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAge(&self, ulage: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAge)(windows_core::Interface::as_raw(self), ulage) }
    }
    pub unsafe fn Size(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Size)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSize(&self, ulage: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSize)(windows_core::Interface::as_raw(self), ulage) }
    }
    pub unsafe fn Actions(&self) -> windows_core::Result<FolderActionSteps> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Actions)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetActions(&self, steps: FolderActionSteps) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetActions)(windows_core::Interface::as_raw(self), steps) }
    }
    pub unsafe fn SendCabTo(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SendCabTo)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetSendCabTo(&self, bstrdestination: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSendCabTo)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrdestination)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IFolderAction_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub Age: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetAge: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Size: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetSize: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Actions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FolderActionSteps) -> windows_core::HRESULT,
    pub SetActions: unsafe extern "system" fn(*mut core::ffi::c_void, FolderActionSteps) -> windows_core::HRESULT,
    pub SendCabTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSendCabTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IFolderAction_Impl: super::oaidl::IDispatch_Impl {
    fn Age(&self) -> windows_core::Result<u32>;
    fn SetAge(&self, ulage: u32) -> windows_core::Result<()>;
    fn Size(&self) -> windows_core::Result<u32>;
    fn SetSize(&self, ulage: u32) -> windows_core::Result<()>;
    fn Actions(&self) -> windows_core::Result<FolderActionSteps>;
    fn SetActions(&self, steps: FolderActionSteps) -> windows_core::Result<()>;
    fn SendCabTo(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetSendCabTo(&self, bstrdestination: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IFolderAction_Vtbl {
    pub const fn new<Identity: IFolderAction_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Age<Identity: IFolderAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulage: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFolderAction_Impl::Age(this) {
                    Ok(ok__) => {
                        pulage.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAge<Identity: IFolderAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulage: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFolderAction_Impl::SetAge(this, core::mem::transmute_copy(&ulage)).into()
            }
        }
        unsafe extern "system" fn Size<Identity: IFolderAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulage: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFolderAction_Impl::Size(this) {
                    Ok(ok__) => {
                        pulage.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSize<Identity: IFolderAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulage: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFolderAction_Impl::SetSize(this, core::mem::transmute_copy(&ulage)).into()
            }
        }
        unsafe extern "system" fn Actions<Identity: IFolderAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, steps: *mut FolderActionSteps) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFolderAction_Impl::Actions(this) {
                    Ok(ok__) => {
                        steps.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetActions<Identity: IFolderAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, steps: FolderActionSteps) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFolderAction_Impl::SetActions(this, core::mem::transmute_copy(&steps)).into()
            }
        }
        unsafe extern "system" fn SendCabTo<Identity: IFolderAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdestination: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFolderAction_Impl::SendCabTo(this) {
                    Ok(ok__) => {
                        pbstrdestination.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSendCabTo<Identity: IFolderAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdestination: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFolderAction_Impl::SetSendCabTo(this, core::mem::transmute(&bstrdestination)).into()
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Age: Age::<Identity, OFFSET>,
            SetAge: SetAge::<Identity, OFFSET>,
            Size: Size::<Identity, OFFSET>,
            SetSize: SetSize::<Identity, OFFSET>,
            Actions: Actions::<Identity, OFFSET>,
            SetActions: SetActions::<Identity, OFFSET>,
            SendCabTo: SendCabTo::<Identity, OFFSET>,
            SetSendCabTo: SetSendCabTo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFolderAction as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IFolderAction {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IFolderActionCollection, IFolderActionCollection_Vtbl, 0x03837544_098b_11d8_9414_505054503030);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IFolderActionCollection {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IFolderActionCollection, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "oaidl")]
impl IFolderActionCollection {
    pub unsafe fn Count(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn Item(&self, index: &super::oaidl::VARIANT) -> windows_core::Result<IFolderAction> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(index), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Add<P0>(&self, action: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IFolderAction>,
    {
        unsafe { (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), action.param().abi()) }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn Remove(&self, index: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(index)) }
    }
    pub unsafe fn Clear(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AddRange<P0>(&self, actions: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddRange)(windows_core::Interface::as_raw(self), actions.param().abi()) }
    }
    pub unsafe fn CreateFolderAction(&self) -> windows_core::Result<IFolderAction> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateFolderAction)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IFolderActionCollection_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    Item: usize,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    Remove: usize,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFolderAction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IFolderActionCollection_Impl: super::oaidl::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<u32>;
    fn Item(&self, index: &super::oaidl::VARIANT) -> windows_core::Result<IFolderAction>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Add(&self, action: windows_core::Ref<IFolderAction>) -> windows_core::Result<()>;
    fn Remove(&self, index: &super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
    fn AddRange(&self, actions: windows_core::Ref<IFolderActionCollection>) -> windows_core::Result<()>;
    fn CreateFolderAction(&self) -> windows_core::Result<IFolderAction>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IFolderActionCollection_Vtbl {
    pub const fn new<Identity: IFolderActionCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Count<Identity: IFolderActionCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFolderActionCollection_Impl::Count(this) {
                    Ok(ok__) => {
                        count.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Item<Identity: IFolderActionCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::oaidl::VARIANT, action: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFolderActionCollection_Impl::Item(this, core::mem::transmute(&index)) {
                    Ok(ok__) => {
                        action.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: IFolderActionCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#enum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFolderActionCollection_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        r#enum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Add<Identity: IFolderActionCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, action: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFolderActionCollection_Impl::Add(this, core::mem::transmute_copy(&action)).into()
            }
        }
        unsafe extern "system" fn Remove<Identity: IFolderActionCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFolderActionCollection_Impl::Remove(this, core::mem::transmute(&index)).into()
            }
        }
        unsafe extern "system" fn Clear<Identity: IFolderActionCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFolderActionCollection_Impl::Clear(this).into()
            }
        }
        unsafe extern "system" fn AddRange<Identity: IFolderActionCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, actions: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFolderActionCollection_Impl::AddRange(this, core::mem::transmute_copy(&actions)).into()
            }
        }
        unsafe extern "system" fn CreateFolderAction<Identity: IFolderActionCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, folderaction: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFolderActionCollection_Impl::CreateFolderAction(this) {
                    Ok(ok__) => {
                        folderaction.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
            AddRange: AddRange::<Identity, OFFSET>,
            CreateFolderAction: CreateFolderAction::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFolderActionCollection as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IFolderActionCollection {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IPerformanceCounterDataCollector, IPerformanceCounterDataCollector_Vtbl, 0x03837506_098b_11d8_9414_505054503030);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IPerformanceCounterDataCollector {
    type Target = IDataCollector;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IPerformanceCounterDataCollector, windows_core::IUnknown, super::oaidl::IDispatch, IDataCollector);
#[cfg(feature = "oaidl")]
impl IPerformanceCounterDataCollector {
    pub unsafe fn DataSourceName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DataSourceName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetDataSourceName(&self, dsn: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDataSourceName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(dsn)) }
    }
    pub unsafe fn PerformanceCounters(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PerformanceCounters)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetPerformanceCounters(&self, counters: *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPerformanceCounters)(windows_core::Interface::as_raw(self), counters) }
    }
    pub unsafe fn LogFileFormat(&self) -> windows_core::Result<FileFormat> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LogFileFormat)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetLogFileFormat(&self, format: FileFormat) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetLogFileFormat)(windows_core::Interface::as_raw(self), format) }
    }
    pub unsafe fn SampleInterval(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SampleInterval)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSampleInterval(&self, interval: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSampleInterval)(windows_core::Interface::as_raw(self), interval) }
    }
    pub unsafe fn SegmentMaxRecords(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SegmentMaxRecords)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSegmentMaxRecords(&self, records: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSegmentMaxRecords)(windows_core::Interface::as_raw(self), records) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IPerformanceCounterDataCollector_Vtbl {
    pub base__: IDataCollector_Vtbl,
    pub DataSourceName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDataSourceName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PerformanceCounters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    pub SetPerformanceCounters: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    pub LogFileFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FileFormat) -> windows_core::HRESULT,
    pub SetLogFileFormat: unsafe extern "system" fn(*mut core::ffi::c_void, FileFormat) -> windows_core::HRESULT,
    pub SampleInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetSampleInterval: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SegmentMaxRecords: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetSegmentMaxRecords: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IPerformanceCounterDataCollector_Impl: IDataCollector_Impl {
    fn DataSourceName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDataSourceName(&self, dsn: &windows_core::BSTR) -> windows_core::Result<()>;
    fn PerformanceCounters(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY>;
    fn SetPerformanceCounters(&self, counters: *const super::oaidl::SAFEARRAY) -> windows_core::Result<()>;
    fn LogFileFormat(&self) -> windows_core::Result<FileFormat>;
    fn SetLogFileFormat(&self, format: FileFormat) -> windows_core::Result<()>;
    fn SampleInterval(&self) -> windows_core::Result<u32>;
    fn SetSampleInterval(&self, interval: u32) -> windows_core::Result<()>;
    fn SegmentMaxRecords(&self) -> windows_core::Result<u32>;
    fn SetSegmentMaxRecords(&self, records: u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IPerformanceCounterDataCollector_Vtbl {
    pub const fn new<Identity: IPerformanceCounterDataCollector_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DataSourceName<Identity: IPerformanceCounterDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dsn: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPerformanceCounterDataCollector_Impl::DataSourceName(this) {
                    Ok(ok__) => {
                        dsn.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDataSourceName<Identity: IPerformanceCounterDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dsn: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPerformanceCounterDataCollector_Impl::SetDataSourceName(this, core::mem::transmute(&dsn)).into()
            }
        }
        unsafe extern "system" fn PerformanceCounters<Identity: IPerformanceCounterDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, counters: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPerformanceCounterDataCollector_Impl::PerformanceCounters(this) {
                    Ok(ok__) => {
                        counters.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPerformanceCounters<Identity: IPerformanceCounterDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, counters: *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPerformanceCounterDataCollector_Impl::SetPerformanceCounters(this, core::mem::transmute_copy(&counters)).into()
            }
        }
        unsafe extern "system" fn LogFileFormat<Identity: IPerformanceCounterDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: *mut FileFormat) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPerformanceCounterDataCollector_Impl::LogFileFormat(this) {
                    Ok(ok__) => {
                        format.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLogFileFormat<Identity: IPerformanceCounterDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: FileFormat) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPerformanceCounterDataCollector_Impl::SetLogFileFormat(this, core::mem::transmute_copy(&format)).into()
            }
        }
        unsafe extern "system" fn SampleInterval<Identity: IPerformanceCounterDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, interval: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPerformanceCounterDataCollector_Impl::SampleInterval(this) {
                    Ok(ok__) => {
                        interval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSampleInterval<Identity: IPerformanceCounterDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, interval: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPerformanceCounterDataCollector_Impl::SetSampleInterval(this, core::mem::transmute_copy(&interval)).into()
            }
        }
        unsafe extern "system" fn SegmentMaxRecords<Identity: IPerformanceCounterDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, records: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPerformanceCounterDataCollector_Impl::SegmentMaxRecords(this) {
                    Ok(ok__) => {
                        records.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSegmentMaxRecords<Identity: IPerformanceCounterDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, records: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPerformanceCounterDataCollector_Impl::SetSegmentMaxRecords(this, core::mem::transmute_copy(&records)).into()
            }
        }
        Self {
            base__: IDataCollector_Vtbl::new::<Identity, OFFSET>(),
            DataSourceName: DataSourceName::<Identity, OFFSET>,
            SetDataSourceName: SetDataSourceName::<Identity, OFFSET>,
            PerformanceCounters: PerformanceCounters::<Identity, OFFSET>,
            SetPerformanceCounters: SetPerformanceCounters::<Identity, OFFSET>,
            LogFileFormat: LogFileFormat::<Identity, OFFSET>,
            SetLogFileFormat: SetLogFileFormat::<Identity, OFFSET>,
            SampleInterval: SampleInterval::<Identity, OFFSET>,
            SetSampleInterval: SetSampleInterval::<Identity, OFFSET>,
            SegmentMaxRecords: SegmentMaxRecords::<Identity, OFFSET>,
            SetSegmentMaxRecords: SetSegmentMaxRecords::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPerformanceCounterDataCollector as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<IDataCollector as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IPerformanceCounterDataCollector {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(ISchedule, ISchedule_Vtbl, 0x0383753a_098b_11d8_9414_505054503030);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for ISchedule {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(ISchedule, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "oaidl")]
impl ISchedule {
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn StartDate(&self) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StartDate)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn SetStartDate(&self, start: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetStartDate)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(start)) }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn EndDate(&self) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EndDate)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn SetEndDate(&self, end: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetEndDate)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(end)) }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn StartTime(&self) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StartTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn SetStartTime(&self, start: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetStartTime)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(start)) }
    }
    pub unsafe fn Days(&self) -> windows_core::Result<WeekDays> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Days)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetDays(&self, days: WeekDays) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDays)(windows_core::Interface::as_raw(self), days) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ISchedule_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub StartDate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    StartDate: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub SetStartDate: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    SetStartDate: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub EndDate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    EndDate: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub SetEndDate: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    SetEndDate: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub StartTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    StartTime: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub SetStartTime: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    SetStartTime: usize,
    pub Days: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WeekDays) -> windows_core::HRESULT,
    pub SetDays: unsafe extern "system" fn(*mut core::ffi::c_void, WeekDays) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ISchedule_Impl: super::oaidl::IDispatch_Impl {
    fn StartDate(&self) -> windows_core::Result<super::oaidl::VARIANT>;
    fn SetStartDate(&self, start: &super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn EndDate(&self) -> windows_core::Result<super::oaidl::VARIANT>;
    fn SetEndDate(&self, end: &super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn StartTime(&self) -> windows_core::Result<super::oaidl::VARIANT>;
    fn SetStartTime(&self, start: &super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn Days(&self) -> windows_core::Result<WeekDays>;
    fn SetDays(&self, days: WeekDays) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ISchedule_Vtbl {
    pub const fn new<Identity: ISchedule_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn StartDate<Identity: ISchedule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, start: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISchedule_Impl::StartDate(this) {
                    Ok(ok__) => {
                        start.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetStartDate<Identity: ISchedule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, start: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISchedule_Impl::SetStartDate(this, core::mem::transmute(&start)).into()
            }
        }
        unsafe extern "system" fn EndDate<Identity: ISchedule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, end: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISchedule_Impl::EndDate(this) {
                    Ok(ok__) => {
                        end.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEndDate<Identity: ISchedule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, end: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISchedule_Impl::SetEndDate(this, core::mem::transmute(&end)).into()
            }
        }
        unsafe extern "system" fn StartTime<Identity: ISchedule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, start: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISchedule_Impl::StartTime(this) {
                    Ok(ok__) => {
                        start.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetStartTime<Identity: ISchedule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, start: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISchedule_Impl::SetStartTime(this, core::mem::transmute(&start)).into()
            }
        }
        unsafe extern "system" fn Days<Identity: ISchedule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, days: *mut WeekDays) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISchedule_Impl::Days(this) {
                    Ok(ok__) => {
                        days.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDays<Identity: ISchedule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, days: WeekDays) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISchedule_Impl::SetDays(this, core::mem::transmute_copy(&days)).into()
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            StartDate: StartDate::<Identity, OFFSET>,
            SetStartDate: SetStartDate::<Identity, OFFSET>,
            EndDate: EndDate::<Identity, OFFSET>,
            SetEndDate: SetEndDate::<Identity, OFFSET>,
            StartTime: StartTime::<Identity, OFFSET>,
            SetStartTime: SetStartTime::<Identity, OFFSET>,
            Days: Days::<Identity, OFFSET>,
            SetDays: SetDays::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISchedule as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ISchedule {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IScheduleCollection, IScheduleCollection_Vtbl, 0x0383753d_098b_11d8_9414_505054503030);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IScheduleCollection {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IScheduleCollection, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "oaidl")]
impl IScheduleCollection {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn Item(&self, index: &super::oaidl::VARIANT) -> windows_core::Result<ISchedule> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(index), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Add<P0>(&self, pschedule: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ISchedule>,
    {
        unsafe { (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), pschedule.param().abi()) }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn Remove(&self, vschedule: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vschedule)) }
    }
    pub unsafe fn Clear(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AddRange<P0>(&self, pschedules: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddRange)(windows_core::Interface::as_raw(self), pschedules.param().abi()) }
    }
    pub unsafe fn CreateSchedule(&self) -> windows_core::Result<ISchedule> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSchedule)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IScheduleCollection_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    Item: usize,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    Remove: usize,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateSchedule: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IScheduleCollection_Impl: super::oaidl::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn Item(&self, index: &super::oaidl::VARIANT) -> windows_core::Result<ISchedule>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Add(&self, pschedule: windows_core::Ref<ISchedule>) -> windows_core::Result<()>;
    fn Remove(&self, vschedule: &super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
    fn AddRange(&self, pschedules: windows_core::Ref<IScheduleCollection>) -> windows_core::Result<()>;
    fn CreateSchedule(&self) -> windows_core::Result<ISchedule>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IScheduleCollection_Vtbl {
    pub const fn new<Identity: IScheduleCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Count<Identity: IScheduleCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IScheduleCollection_Impl::Count(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Item<Identity: IScheduleCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::oaidl::VARIANT, ppschedule: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IScheduleCollection_Impl::Item(this, core::mem::transmute(&index)) {
                    Ok(ok__) => {
                        ppschedule.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: IScheduleCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ienum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IScheduleCollection_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        ienum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Add<Identity: IScheduleCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pschedule: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IScheduleCollection_Impl::Add(this, core::mem::transmute_copy(&pschedule)).into()
            }
        }
        unsafe extern "system" fn Remove<Identity: IScheduleCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vschedule: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IScheduleCollection_Impl::Remove(this, core::mem::transmute(&vschedule)).into()
            }
        }
        unsafe extern "system" fn Clear<Identity: IScheduleCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IScheduleCollection_Impl::Clear(this).into()
            }
        }
        unsafe extern "system" fn AddRange<Identity: IScheduleCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pschedules: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IScheduleCollection_Impl::AddRange(this, core::mem::transmute_copy(&pschedules)).into()
            }
        }
        unsafe extern "system" fn CreateSchedule<Identity: IScheduleCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, schedule: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IScheduleCollection_Impl::CreateSchedule(this) {
                    Ok(ok__) => {
                        schedule.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
            AddRange: AddRange::<Identity, OFFSET>,
            CreateSchedule: CreateSchedule::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IScheduleCollection as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IScheduleCollection {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(ITraceDataCollector, ITraceDataCollector_Vtbl, 0x0383750b_098b_11d8_9414_505054503030);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for ITraceDataCollector {
    type Target = IDataCollector;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(ITraceDataCollector, windows_core::IUnknown, super::oaidl::IDispatch, IDataCollector);
#[cfg(feature = "oaidl")]
impl ITraceDataCollector {
    pub unsafe fn BufferSize(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BufferSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetBufferSize(&self, size: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetBufferSize)(windows_core::Interface::as_raw(self), size) }
    }
    pub unsafe fn BuffersLost(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BuffersLost)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetBuffersLost(&self, buffers: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetBuffersLost)(windows_core::Interface::as_raw(self), buffers) }
    }
    pub unsafe fn BuffersWritten(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BuffersWritten)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetBuffersWritten(&self, buffers: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetBuffersWritten)(windows_core::Interface::as_raw(self), buffers) }
    }
    pub unsafe fn ClockType(&self) -> windows_core::Result<ClockType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ClockType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetClockType(&self, clock: ClockType) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetClockType)(windows_core::Interface::as_raw(self), clock) }
    }
    pub unsafe fn EventsLost(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EventsLost)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetEventsLost(&self, events: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetEventsLost)(windows_core::Interface::as_raw(self), events) }
    }
    pub unsafe fn ExtendedModes(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ExtendedModes)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetExtendedModes(&self, mode: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetExtendedModes)(windows_core::Interface::as_raw(self), mode) }
    }
    pub unsafe fn FlushTimer(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FlushTimer)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetFlushTimer(&self, seconds: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFlushTimer)(windows_core::Interface::as_raw(self), seconds) }
    }
    pub unsafe fn FreeBuffers(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FreeBuffers)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetFreeBuffers(&self, buffers: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFreeBuffers)(windows_core::Interface::as_raw(self), buffers) }
    }
    pub unsafe fn Guid(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Guid)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetGuid(&self, guid: windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetGuid)(windows_core::Interface::as_raw(self), guid) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn IsKernelTrace(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsKernelTrace)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn MaximumBuffers(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MaximumBuffers)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetMaximumBuffers(&self, buffers: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMaximumBuffers)(windows_core::Interface::as_raw(self), buffers) }
    }
    pub unsafe fn MinimumBuffers(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MinimumBuffers)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetMinimumBuffers(&self, buffers: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMinimumBuffers)(windows_core::Interface::as_raw(self), buffers) }
    }
    pub unsafe fn NumberOfBuffers(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NumberOfBuffers)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetNumberOfBuffers(&self, buffers: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetNumberOfBuffers)(windows_core::Interface::as_raw(self), buffers) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn PreallocateFile(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PreallocateFile)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetPreallocateFile(&self, allocate: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPreallocateFile)(windows_core::Interface::as_raw(self), allocate) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn ProcessMode(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ProcessMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetProcessMode(&self, process: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetProcessMode)(windows_core::Interface::as_raw(self), process) }
    }
    pub unsafe fn RealTimeBuffersLost(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RealTimeBuffersLost)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetRealTimeBuffersLost(&self, buffers: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRealTimeBuffersLost)(windows_core::Interface::as_raw(self), buffers) }
    }
    pub unsafe fn SessionId(&self) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SessionId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSessionId(&self, id: u64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSessionId)(windows_core::Interface::as_raw(self), id) }
    }
    pub unsafe fn SessionName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SessionName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetSessionName(&self, name: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSessionName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name)) }
    }
    pub unsafe fn SessionThreadId(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SessionThreadId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSessionThreadId(&self, tid: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSessionThreadId)(windows_core::Interface::as_raw(self), tid) }
    }
    pub unsafe fn StreamMode(&self) -> windows_core::Result<StreamMode> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StreamMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetStreamMode(&self, mode: StreamMode) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetStreamMode)(windows_core::Interface::as_raw(self), mode) }
    }
    pub unsafe fn TraceDataProviders(&self) -> windows_core::Result<ITraceDataProviderCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TraceDataProviders)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ITraceDataCollector_Vtbl {
    pub base__: IDataCollector_Vtbl,
    pub BufferSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetBufferSize: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub BuffersLost: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetBuffersLost: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub BuffersWritten: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetBuffersWritten: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub ClockType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ClockType) -> windows_core::HRESULT,
    pub SetClockType: unsafe extern "system" fn(*mut core::ffi::c_void, ClockType) -> windows_core::HRESULT,
    pub EventsLost: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetEventsLost: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub ExtendedModes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetExtendedModes: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub FlushTimer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetFlushTimer: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub FreeBuffers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetFreeBuffers: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Guid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub SetGuid: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub IsKernelTrace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    IsKernelTrace: usize,
    pub MaximumBuffers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetMaximumBuffers: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub MinimumBuffers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetMinimumBuffers: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub NumberOfBuffers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetNumberOfBuffers: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub PreallocateFile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    PreallocateFile: usize,
    #[cfg(feature = "wtypes")]
    pub SetPreallocateFile: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetPreallocateFile: usize,
    #[cfg(feature = "wtypes")]
    pub ProcessMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    ProcessMode: usize,
    #[cfg(feature = "wtypes")]
    pub SetProcessMode: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetProcessMode: usize,
    pub RealTimeBuffersLost: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetRealTimeBuffersLost: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SessionId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub SetSessionId: unsafe extern "system" fn(*mut core::ffi::c_void, u64) -> windows_core::HRESULT,
    pub SessionName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSessionName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SessionThreadId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetSessionThreadId: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub StreamMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut StreamMode) -> windows_core::HRESULT,
    pub SetStreamMode: unsafe extern "system" fn(*mut core::ffi::c_void, StreamMode) -> windows_core::HRESULT,
    pub TraceDataProviders: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ITraceDataCollector_Impl: IDataCollector_Impl {
    fn BufferSize(&self) -> windows_core::Result<u32>;
    fn SetBufferSize(&self, size: u32) -> windows_core::Result<()>;
    fn BuffersLost(&self) -> windows_core::Result<u32>;
    fn SetBuffersLost(&self, buffers: u32) -> windows_core::Result<()>;
    fn BuffersWritten(&self) -> windows_core::Result<u32>;
    fn SetBuffersWritten(&self, buffers: u32) -> windows_core::Result<()>;
    fn ClockType(&self) -> windows_core::Result<ClockType>;
    fn SetClockType(&self, clock: ClockType) -> windows_core::Result<()>;
    fn EventsLost(&self) -> windows_core::Result<u32>;
    fn SetEventsLost(&self, events: u32) -> windows_core::Result<()>;
    fn ExtendedModes(&self) -> windows_core::Result<u32>;
    fn SetExtendedModes(&self, mode: u32) -> windows_core::Result<()>;
    fn FlushTimer(&self) -> windows_core::Result<u32>;
    fn SetFlushTimer(&self, seconds: u32) -> windows_core::Result<()>;
    fn FreeBuffers(&self) -> windows_core::Result<u32>;
    fn SetFreeBuffers(&self, buffers: u32) -> windows_core::Result<()>;
    fn Guid(&self) -> windows_core::Result<windows_core::GUID>;
    fn SetGuid(&self, guid: &windows_core::GUID) -> windows_core::Result<()>;
    fn IsKernelTrace(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn MaximumBuffers(&self) -> windows_core::Result<u32>;
    fn SetMaximumBuffers(&self, buffers: u32) -> windows_core::Result<()>;
    fn MinimumBuffers(&self) -> windows_core::Result<u32>;
    fn SetMinimumBuffers(&self, buffers: u32) -> windows_core::Result<()>;
    fn NumberOfBuffers(&self) -> windows_core::Result<u32>;
    fn SetNumberOfBuffers(&self, buffers: u32) -> windows_core::Result<()>;
    fn PreallocateFile(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn SetPreallocateFile(&self, allocate: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ProcessMode(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn SetProcessMode(&self, process: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn RealTimeBuffersLost(&self) -> windows_core::Result<u32>;
    fn SetRealTimeBuffersLost(&self, buffers: u32) -> windows_core::Result<()>;
    fn SessionId(&self) -> windows_core::Result<u64>;
    fn SetSessionId(&self, id: u64) -> windows_core::Result<()>;
    fn SessionName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetSessionName(&self, name: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SessionThreadId(&self) -> windows_core::Result<u32>;
    fn SetSessionThreadId(&self, tid: u32) -> windows_core::Result<()>;
    fn StreamMode(&self) -> windows_core::Result<StreamMode>;
    fn SetStreamMode(&self, mode: StreamMode) -> windows_core::Result<()>;
    fn TraceDataProviders(&self) -> windows_core::Result<ITraceDataProviderCollection>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ITraceDataCollector_Vtbl {
    pub const fn new<Identity: ITraceDataCollector_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn BufferSize<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, size: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataCollector_Impl::BufferSize(this) {
                    Ok(ok__) => {
                        size.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBufferSize<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, size: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceDataCollector_Impl::SetBufferSize(this, core::mem::transmute_copy(&size)).into()
            }
        }
        unsafe extern "system" fn BuffersLost<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffers: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataCollector_Impl::BuffersLost(this) {
                    Ok(ok__) => {
                        buffers.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBuffersLost<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffers: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceDataCollector_Impl::SetBuffersLost(this, core::mem::transmute_copy(&buffers)).into()
            }
        }
        unsafe extern "system" fn BuffersWritten<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffers: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataCollector_Impl::BuffersWritten(this) {
                    Ok(ok__) => {
                        buffers.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBuffersWritten<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffers: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceDataCollector_Impl::SetBuffersWritten(this, core::mem::transmute_copy(&buffers)).into()
            }
        }
        unsafe extern "system" fn ClockType<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clock: *mut ClockType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataCollector_Impl::ClockType(this) {
                    Ok(ok__) => {
                        clock.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetClockType<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clock: ClockType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceDataCollector_Impl::SetClockType(this, core::mem::transmute_copy(&clock)).into()
            }
        }
        unsafe extern "system" fn EventsLost<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, events: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataCollector_Impl::EventsLost(this) {
                    Ok(ok__) => {
                        events.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEventsLost<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, events: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceDataCollector_Impl::SetEventsLost(this, core::mem::transmute_copy(&events)).into()
            }
        }
        unsafe extern "system" fn ExtendedModes<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mode: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataCollector_Impl::ExtendedModes(this) {
                    Ok(ok__) => {
                        mode.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetExtendedModes<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mode: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceDataCollector_Impl::SetExtendedModes(this, core::mem::transmute_copy(&mode)).into()
            }
        }
        unsafe extern "system" fn FlushTimer<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, seconds: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataCollector_Impl::FlushTimer(this) {
                    Ok(ok__) => {
                        seconds.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFlushTimer<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, seconds: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceDataCollector_Impl::SetFlushTimer(this, core::mem::transmute_copy(&seconds)).into()
            }
        }
        unsafe extern "system" fn FreeBuffers<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffers: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataCollector_Impl::FreeBuffers(this) {
                    Ok(ok__) => {
                        buffers.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFreeBuffers<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffers: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceDataCollector_Impl::SetFreeBuffers(this, core::mem::transmute_copy(&buffers)).into()
            }
        }
        unsafe extern "system" fn Guid<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataCollector_Impl::Guid(this) {
                    Ok(ok__) => {
                        guid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetGuid<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceDataCollector_Impl::SetGuid(this, core::mem::transmute(&guid)).into()
            }
        }
        unsafe extern "system" fn IsKernelTrace<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, kernel: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataCollector_Impl::IsKernelTrace(this) {
                    Ok(ok__) => {
                        kernel.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MaximumBuffers<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffers: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataCollector_Impl::MaximumBuffers(this) {
                    Ok(ok__) => {
                        buffers.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMaximumBuffers<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffers: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceDataCollector_Impl::SetMaximumBuffers(this, core::mem::transmute_copy(&buffers)).into()
            }
        }
        unsafe extern "system" fn MinimumBuffers<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffers: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataCollector_Impl::MinimumBuffers(this) {
                    Ok(ok__) => {
                        buffers.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMinimumBuffers<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffers: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceDataCollector_Impl::SetMinimumBuffers(this, core::mem::transmute_copy(&buffers)).into()
            }
        }
        unsafe extern "system" fn NumberOfBuffers<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffers: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataCollector_Impl::NumberOfBuffers(this) {
                    Ok(ok__) => {
                        buffers.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetNumberOfBuffers<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffers: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceDataCollector_Impl::SetNumberOfBuffers(this, core::mem::transmute_copy(&buffers)).into()
            }
        }
        unsafe extern "system" fn PreallocateFile<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allocate: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataCollector_Impl::PreallocateFile(this) {
                    Ok(ok__) => {
                        allocate.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPreallocateFile<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allocate: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceDataCollector_Impl::SetPreallocateFile(this, core::mem::transmute_copy(&allocate)).into()
            }
        }
        unsafe extern "system" fn ProcessMode<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, process: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataCollector_Impl::ProcessMode(this) {
                    Ok(ok__) => {
                        process.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetProcessMode<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, process: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceDataCollector_Impl::SetProcessMode(this, core::mem::transmute_copy(&process)).into()
            }
        }
        unsafe extern "system" fn RealTimeBuffersLost<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffers: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataCollector_Impl::RealTimeBuffersLost(this) {
                    Ok(ok__) => {
                        buffers.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRealTimeBuffersLost<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffers: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceDataCollector_Impl::SetRealTimeBuffersLost(this, core::mem::transmute_copy(&buffers)).into()
            }
        }
        unsafe extern "system" fn SessionId<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataCollector_Impl::SessionId(this) {
                    Ok(ok__) => {
                        id.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSessionId<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceDataCollector_Impl::SetSessionId(this, core::mem::transmute_copy(&id)).into()
            }
        }
        unsafe extern "system" fn SessionName<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataCollector_Impl::SessionName(this) {
                    Ok(ok__) => {
                        name.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSessionName<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceDataCollector_Impl::SetSessionName(this, core::mem::transmute(&name)).into()
            }
        }
        unsafe extern "system" fn SessionThreadId<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tid: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataCollector_Impl::SessionThreadId(this) {
                    Ok(ok__) => {
                        tid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSessionThreadId<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tid: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceDataCollector_Impl::SetSessionThreadId(this, core::mem::transmute_copy(&tid)).into()
            }
        }
        unsafe extern "system" fn StreamMode<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mode: *mut StreamMode) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataCollector_Impl::StreamMode(this) {
                    Ok(ok__) => {
                        mode.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetStreamMode<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mode: StreamMode) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceDataCollector_Impl::SetStreamMode(this, core::mem::transmute_copy(&mode)).into()
            }
        }
        unsafe extern "system" fn TraceDataProviders<Identity: ITraceDataCollector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, providers: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataCollector_Impl::TraceDataProviders(this) {
                    Ok(ok__) => {
                        providers.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IDataCollector_Vtbl::new::<Identity, OFFSET>(),
            BufferSize: BufferSize::<Identity, OFFSET>,
            SetBufferSize: SetBufferSize::<Identity, OFFSET>,
            BuffersLost: BuffersLost::<Identity, OFFSET>,
            SetBuffersLost: SetBuffersLost::<Identity, OFFSET>,
            BuffersWritten: BuffersWritten::<Identity, OFFSET>,
            SetBuffersWritten: SetBuffersWritten::<Identity, OFFSET>,
            ClockType: ClockType::<Identity, OFFSET>,
            SetClockType: SetClockType::<Identity, OFFSET>,
            EventsLost: EventsLost::<Identity, OFFSET>,
            SetEventsLost: SetEventsLost::<Identity, OFFSET>,
            ExtendedModes: ExtendedModes::<Identity, OFFSET>,
            SetExtendedModes: SetExtendedModes::<Identity, OFFSET>,
            FlushTimer: FlushTimer::<Identity, OFFSET>,
            SetFlushTimer: SetFlushTimer::<Identity, OFFSET>,
            FreeBuffers: FreeBuffers::<Identity, OFFSET>,
            SetFreeBuffers: SetFreeBuffers::<Identity, OFFSET>,
            Guid: Guid::<Identity, OFFSET>,
            SetGuid: SetGuid::<Identity, OFFSET>,
            IsKernelTrace: IsKernelTrace::<Identity, OFFSET>,
            MaximumBuffers: MaximumBuffers::<Identity, OFFSET>,
            SetMaximumBuffers: SetMaximumBuffers::<Identity, OFFSET>,
            MinimumBuffers: MinimumBuffers::<Identity, OFFSET>,
            SetMinimumBuffers: SetMinimumBuffers::<Identity, OFFSET>,
            NumberOfBuffers: NumberOfBuffers::<Identity, OFFSET>,
            SetNumberOfBuffers: SetNumberOfBuffers::<Identity, OFFSET>,
            PreallocateFile: PreallocateFile::<Identity, OFFSET>,
            SetPreallocateFile: SetPreallocateFile::<Identity, OFFSET>,
            ProcessMode: ProcessMode::<Identity, OFFSET>,
            SetProcessMode: SetProcessMode::<Identity, OFFSET>,
            RealTimeBuffersLost: RealTimeBuffersLost::<Identity, OFFSET>,
            SetRealTimeBuffersLost: SetRealTimeBuffersLost::<Identity, OFFSET>,
            SessionId: SessionId::<Identity, OFFSET>,
            SetSessionId: SetSessionId::<Identity, OFFSET>,
            SessionName: SessionName::<Identity, OFFSET>,
            SetSessionName: SetSessionName::<Identity, OFFSET>,
            SessionThreadId: SessionThreadId::<Identity, OFFSET>,
            SetSessionThreadId: SetSessionThreadId::<Identity, OFFSET>,
            StreamMode: StreamMode::<Identity, OFFSET>,
            SetStreamMode: SetStreamMode::<Identity, OFFSET>,
            TraceDataProviders: TraceDataProviders::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITraceDataCollector as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<IDataCollector as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ITraceDataCollector {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(ITraceDataProvider, ITraceDataProvider_Vtbl, 0x03837512_098b_11d8_9414_505054503030);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for ITraceDataProvider {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(ITraceDataProvider, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "oaidl")]
impl ITraceDataProvider {
    pub unsafe fn DisplayName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DisplayName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetDisplayName(&self, name: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDisplayName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name)) }
    }
    pub unsafe fn Guid(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Guid)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetGuid(&self, guid: windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetGuid)(windows_core::Interface::as_raw(self), guid) }
    }
    pub unsafe fn Level(&self) -> windows_core::Result<IValueMap> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Level)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn KeywordsAny(&self) -> windows_core::Result<IValueMap> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).KeywordsAny)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn KeywordsAll(&self) -> windows_core::Result<IValueMap> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).KeywordsAll)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Properties(&self) -> windows_core::Result<IValueMap> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Properties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn FilterEnabled(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FilterEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetFilterEnabled(&self, filterenabled: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFilterEnabled)(windows_core::Interface::as_raw(self), filterenabled) }
    }
    pub unsafe fn FilterType(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FilterType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetFilterType(&self, ultype: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFilterType)(windows_core::Interface::as_raw(self), ultype) }
    }
    pub unsafe fn FilterData(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FilterData)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetFilterData(&self, pdata: *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFilterData)(windows_core::Interface::as_raw(self), pdata) }
    }
    pub unsafe fn Query(&self, bstrname: &windows_core::BSTR, bstrserver: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Query)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrname), core::mem::transmute_copy(bstrserver)) }
    }
    pub unsafe fn Resolve<P0>(&self, pfrom: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::oaidl::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).Resolve)(windows_core::Interface::as_raw(self), pfrom.param().abi()) }
    }
    pub unsafe fn SetSecurity(&self, sddl: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSecurity)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(sddl)) }
    }
    pub unsafe fn GetSecurity(&self, securityinfo: u32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSecurity)(windows_core::Interface::as_raw(self), securityinfo, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetRegisteredProcesses(&self) -> windows_core::Result<IValueMap> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRegisteredProcesses)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ITraceDataProvider_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Guid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub SetGuid: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
    pub Level: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub KeywordsAny: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub KeywordsAll: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub FilterEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    FilterEnabled: usize,
    #[cfg(feature = "wtypes")]
    pub SetFilterEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetFilterEnabled: usize,
    pub FilterType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetFilterType: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub FilterData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    pub SetFilterData: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    pub Query: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Resolve: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSecurity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSecurity: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRegisteredProcesses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ITraceDataProvider_Impl: super::oaidl::IDispatch_Impl {
    fn DisplayName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDisplayName(&self, name: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Guid(&self) -> windows_core::Result<windows_core::GUID>;
    fn SetGuid(&self, guid: &windows_core::GUID) -> windows_core::Result<()>;
    fn Level(&self) -> windows_core::Result<IValueMap>;
    fn KeywordsAny(&self) -> windows_core::Result<IValueMap>;
    fn KeywordsAll(&self) -> windows_core::Result<IValueMap>;
    fn Properties(&self) -> windows_core::Result<IValueMap>;
    fn FilterEnabled(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn SetFilterEnabled(&self, filterenabled: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn FilterType(&self) -> windows_core::Result<u32>;
    fn SetFilterType(&self, ultype: u32) -> windows_core::Result<()>;
    fn FilterData(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY>;
    fn SetFilterData(&self, pdata: *const super::oaidl::SAFEARRAY) -> windows_core::Result<()>;
    fn Query(&self, bstrname: &windows_core::BSTR, bstrserver: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Resolve(&self, pfrom: windows_core::Ref<super::oaidl::IDispatch>) -> windows_core::Result<()>;
    fn SetSecurity(&self, sddl: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetSecurity(&self, securityinfo: u32) -> windows_core::Result<windows_core::BSTR>;
    fn GetRegisteredProcesses(&self) -> windows_core::Result<IValueMap>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ITraceDataProvider_Vtbl {
    pub const fn new<Identity: ITraceDataProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DisplayName<Identity: ITraceDataProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataProvider_Impl::DisplayName(this) {
                    Ok(ok__) => {
                        name.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDisplayName<Identity: ITraceDataProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceDataProvider_Impl::SetDisplayName(this, core::mem::transmute(&name)).into()
            }
        }
        unsafe extern "system" fn Guid<Identity: ITraceDataProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataProvider_Impl::Guid(this) {
                    Ok(ok__) => {
                        guid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetGuid<Identity: ITraceDataProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceDataProvider_Impl::SetGuid(this, core::mem::transmute(&guid)).into()
            }
        }
        unsafe extern "system" fn Level<Identity: ITraceDataProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pplevel: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataProvider_Impl::Level(this) {
                    Ok(ok__) => {
                        pplevel.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn KeywordsAny<Identity: ITraceDataProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppkeywords: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataProvider_Impl::KeywordsAny(this) {
                    Ok(ok__) => {
                        ppkeywords.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn KeywordsAll<Identity: ITraceDataProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppkeywords: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataProvider_Impl::KeywordsAll(this) {
                    Ok(ok__) => {
                        ppkeywords.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Properties<Identity: ITraceDataProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataProvider_Impl::Properties(this) {
                    Ok(ok__) => {
                        ppproperties.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FilterEnabled<Identity: ITraceDataProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filterenabled: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataProvider_Impl::FilterEnabled(this) {
                    Ok(ok__) => {
                        filterenabled.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFilterEnabled<Identity: ITraceDataProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filterenabled: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceDataProvider_Impl::SetFilterEnabled(this, core::mem::transmute_copy(&filterenabled)).into()
            }
        }
        unsafe extern "system" fn FilterType<Identity: ITraceDataProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pultype: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataProvider_Impl::FilterType(this) {
                    Ok(ok__) => {
                        pultype.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFilterType<Identity: ITraceDataProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ultype: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceDataProvider_Impl::SetFilterType(this, core::mem::transmute_copy(&ultype)).into()
            }
        }
        unsafe extern "system" fn FilterData<Identity: ITraceDataProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdata: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataProvider_Impl::FilterData(this) {
                    Ok(ok__) => {
                        ppdata.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFilterData<Identity: ITraceDataProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdata: *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceDataProvider_Impl::SetFilterData(this, core::mem::transmute_copy(&pdata)).into()
            }
        }
        unsafe extern "system" fn Query<Identity: ITraceDataProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: *mut core::ffi::c_void, bstrserver: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceDataProvider_Impl::Query(this, core::mem::transmute(&bstrname), core::mem::transmute(&bstrserver)).into()
            }
        }
        unsafe extern "system" fn Resolve<Identity: ITraceDataProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfrom: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceDataProvider_Impl::Resolve(this, core::mem::transmute_copy(&pfrom)).into()
            }
        }
        unsafe extern "system" fn SetSecurity<Identity: ITraceDataProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sddl: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceDataProvider_Impl::SetSecurity(this, core::mem::transmute(&sddl)).into()
            }
        }
        unsafe extern "system" fn GetSecurity<Identity: ITraceDataProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, securityinfo: u32, sddl: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataProvider_Impl::GetSecurity(this, core::mem::transmute_copy(&securityinfo)) {
                    Ok(ok__) => {
                        sddl.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRegisteredProcesses<Identity: ITraceDataProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, processes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataProvider_Impl::GetRegisteredProcesses(this) {
                    Ok(ok__) => {
                        processes.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            DisplayName: DisplayName::<Identity, OFFSET>,
            SetDisplayName: SetDisplayName::<Identity, OFFSET>,
            Guid: Guid::<Identity, OFFSET>,
            SetGuid: SetGuid::<Identity, OFFSET>,
            Level: Level::<Identity, OFFSET>,
            KeywordsAny: KeywordsAny::<Identity, OFFSET>,
            KeywordsAll: KeywordsAll::<Identity, OFFSET>,
            Properties: Properties::<Identity, OFFSET>,
            FilterEnabled: FilterEnabled::<Identity, OFFSET>,
            SetFilterEnabled: SetFilterEnabled::<Identity, OFFSET>,
            FilterType: FilterType::<Identity, OFFSET>,
            SetFilterType: SetFilterType::<Identity, OFFSET>,
            FilterData: FilterData::<Identity, OFFSET>,
            SetFilterData: SetFilterData::<Identity, OFFSET>,
            Query: Query::<Identity, OFFSET>,
            Resolve: Resolve::<Identity, OFFSET>,
            SetSecurity: SetSecurity::<Identity, OFFSET>,
            GetSecurity: GetSecurity::<Identity, OFFSET>,
            GetRegisteredProcesses: GetRegisteredProcesses::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITraceDataProvider as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ITraceDataProvider {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(ITraceDataProviderCollection, ITraceDataProviderCollection_Vtbl, 0x03837510_098b_11d8_9414_505054503030);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for ITraceDataProviderCollection {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(ITraceDataProviderCollection, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "oaidl")]
impl ITraceDataProviderCollection {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn Item(&self, index: &super::oaidl::VARIANT) -> windows_core::Result<ITraceDataProvider> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(index), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Add<P0>(&self, pprovider: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ITraceDataProvider>,
    {
        unsafe { (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), pprovider.param().abi()) }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn Remove(&self, vprovider: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vprovider)) }
    }
    pub unsafe fn Clear(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AddRange<P0>(&self, providers: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddRange)(windows_core::Interface::as_raw(self), providers.param().abi()) }
    }
    pub unsafe fn CreateTraceDataProvider(&self) -> windows_core::Result<ITraceDataProvider> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateTraceDataProvider)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetTraceDataProviders(&self, server: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetTraceDataProviders)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(server)) }
    }
    pub unsafe fn GetTraceDataProvidersByProcess(&self, server: &windows_core::BSTR, pid: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetTraceDataProvidersByProcess)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(server), pid) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ITraceDataProviderCollection_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    Item: usize,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    Remove: usize,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateTraceDataProvider: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetTraceDataProviders: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetTraceDataProvidersByProcess: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ITraceDataProviderCollection_Impl: super::oaidl::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn Item(&self, index: &super::oaidl::VARIANT) -> windows_core::Result<ITraceDataProvider>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Add(&self, pprovider: windows_core::Ref<ITraceDataProvider>) -> windows_core::Result<()>;
    fn Remove(&self, vprovider: &super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
    fn AddRange(&self, providers: windows_core::Ref<ITraceDataProviderCollection>) -> windows_core::Result<()>;
    fn CreateTraceDataProvider(&self) -> windows_core::Result<ITraceDataProvider>;
    fn GetTraceDataProviders(&self, server: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetTraceDataProvidersByProcess(&self, server: &windows_core::BSTR, pid: u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ITraceDataProviderCollection_Vtbl {
    pub const fn new<Identity: ITraceDataProviderCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Count<Identity: ITraceDataProviderCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataProviderCollection_Impl::Count(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Item<Identity: ITraceDataProviderCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::oaidl::VARIANT, ppprovider: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataProviderCollection_Impl::Item(this, core::mem::transmute(&index)) {
                    Ok(ok__) => {
                        ppprovider.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ITraceDataProviderCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataProviderCollection_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Add<Identity: ITraceDataProviderCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprovider: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceDataProviderCollection_Impl::Add(this, core::mem::transmute_copy(&pprovider)).into()
            }
        }
        unsafe extern "system" fn Remove<Identity: ITraceDataProviderCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vprovider: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceDataProviderCollection_Impl::Remove(this, core::mem::transmute(&vprovider)).into()
            }
        }
        unsafe extern "system" fn Clear<Identity: ITraceDataProviderCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceDataProviderCollection_Impl::Clear(this).into()
            }
        }
        unsafe extern "system" fn AddRange<Identity: ITraceDataProviderCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, providers: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceDataProviderCollection_Impl::AddRange(this, core::mem::transmute_copy(&providers)).into()
            }
        }
        unsafe extern "system" fn CreateTraceDataProvider<Identity: ITraceDataProviderCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, provider: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITraceDataProviderCollection_Impl::CreateTraceDataProvider(this) {
                    Ok(ok__) => {
                        provider.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTraceDataProviders<Identity: ITraceDataProviderCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, server: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceDataProviderCollection_Impl::GetTraceDataProviders(this, core::mem::transmute(&server)).into()
            }
        }
        unsafe extern "system" fn GetTraceDataProvidersByProcess<Identity: ITraceDataProviderCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, server: *mut core::ffi::c_void, pid: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITraceDataProviderCollection_Impl::GetTraceDataProvidersByProcess(this, core::mem::transmute(&server), core::mem::transmute_copy(&pid)).into()
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
            AddRange: AddRange::<Identity, OFFSET>,
            CreateTraceDataProvider: CreateTraceDataProvider::<Identity, OFFSET>,
            GetTraceDataProviders: GetTraceDataProviders::<Identity, OFFSET>,
            GetTraceDataProvidersByProcess: GetTraceDataProvidersByProcess::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITraceDataProviderCollection as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ITraceDataProviderCollection {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IValueMap, IValueMap_Vtbl, 0x03837534_098b_11d8_9414_505054503030);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IValueMap {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IValueMap, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "oaidl")]
impl IValueMap {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn Item(&self, index: &super::oaidl::VARIANT) -> windows_core::Result<IValueMapItem> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(index), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Description(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetDescription(&self, description: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDescription)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(description)) }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn Value(&self) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Value)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn SetValue(&self, value: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetValue)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(value)) }
    }
    pub unsafe fn ValueMapType(&self) -> windows_core::Result<ValueMapType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ValueMapType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetValueMapType(&self, r#type: ValueMapType) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetValueMapType)(windows_core::Interface::as_raw(self), r#type) }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn Add(&self, value: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(value)) }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn Remove(&self, value: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(value)) }
    }
    pub unsafe fn Clear(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AddRange<P0>(&self, map: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddRange)(windows_core::Interface::as_raw(self), map.param().abi()) }
    }
    pub unsafe fn CreateValueMapItem(&self) -> windows_core::Result<IValueMapItem> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateValueMapItem)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IValueMap_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    Item: usize,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    Value: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    SetValue: usize,
    pub ValueMapType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ValueMapType) -> windows_core::HRESULT,
    pub SetValueMapType: unsafe extern "system" fn(*mut core::ffi::c_void, ValueMapType) -> windows_core::HRESULT,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    Add: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    Remove: usize,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateValueMapItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IValueMap_Impl: super::oaidl::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn Item(&self, index: &super::oaidl::VARIANT) -> windows_core::Result<IValueMapItem>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDescription(&self, description: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Value(&self) -> windows_core::Result<super::oaidl::VARIANT>;
    fn SetValue(&self, value: &super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn ValueMapType(&self) -> windows_core::Result<ValueMapType>;
    fn SetValueMapType(&self, r#type: ValueMapType) -> windows_core::Result<()>;
    fn Add(&self, value: &super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn Remove(&self, value: &super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
    fn AddRange(&self, map: windows_core::Ref<IValueMap>) -> windows_core::Result<()>;
    fn CreateValueMapItem(&self) -> windows_core::Result<IValueMapItem>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IValueMap_Vtbl {
    pub const fn new<Identity: IValueMap_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Count<Identity: IValueMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IValueMap_Impl::Count(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Item<Identity: IValueMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::oaidl::VARIANT, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IValueMap_Impl::Item(this, core::mem::transmute(&index)) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: IValueMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IValueMap_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Description<Identity: IValueMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, description: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IValueMap_Impl::Description(this) {
                    Ok(ok__) => {
                        description.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDescription<Identity: IValueMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, description: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IValueMap_Impl::SetDescription(this, core::mem::transmute(&description)).into()
            }
        }
        unsafe extern "system" fn Value<Identity: IValueMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IValueMap_Impl::Value(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetValue<Identity: IValueMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IValueMap_Impl::SetValue(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn ValueMapType<Identity: IValueMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: *mut ValueMapType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IValueMap_Impl::ValueMapType(this) {
                    Ok(ok__) => {
                        r#type.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetValueMapType<Identity: IValueMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: ValueMapType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IValueMap_Impl::SetValueMapType(this, core::mem::transmute_copy(&r#type)).into()
            }
        }
        unsafe extern "system" fn Add<Identity: IValueMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IValueMap_Impl::Add(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn Remove<Identity: IValueMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IValueMap_Impl::Remove(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn Clear<Identity: IValueMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IValueMap_Impl::Clear(this).into()
            }
        }
        unsafe extern "system" fn AddRange<Identity: IValueMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, map: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IValueMap_Impl::AddRange(this, core::mem::transmute_copy(&map)).into()
            }
        }
        unsafe extern "system" fn CreateValueMapItem<Identity: IValueMap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, item: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IValueMap_Impl::CreateValueMapItem(this) {
                    Ok(ok__) => {
                        item.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Description: Description::<Identity, OFFSET>,
            SetDescription: SetDescription::<Identity, OFFSET>,
            Value: Value::<Identity, OFFSET>,
            SetValue: SetValue::<Identity, OFFSET>,
            ValueMapType: ValueMapType::<Identity, OFFSET>,
            SetValueMapType: SetValueMapType::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
            AddRange: AddRange::<Identity, OFFSET>,
            CreateValueMapItem: CreateValueMapItem::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IValueMap as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IValueMap {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IValueMapItem, IValueMapItem_Vtbl, 0x03837533_098b_11d8_9414_505054503030);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IValueMapItem {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IValueMapItem, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "oaidl")]
impl IValueMapItem {
    pub unsafe fn Description(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetDescription(&self, description: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDescription)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(description)) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn Enabled(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Enabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetEnabled(&self, enabled: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetEnabled)(windows_core::Interface::as_raw(self), enabled) }
    }
    pub unsafe fn Key(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Key)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetKey(&self, key: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetKey)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(key)) }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn Value(&self) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Value)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn SetValue(&self, value: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetValue)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(value)) }
    }
    pub unsafe fn ValueMapType(&self) -> windows_core::Result<ValueMapType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ValueMapType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetValueMapType(&self, r#type: ValueMapType) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetValueMapType)(windows_core::Interface::as_raw(self), r#type) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IValueMapItem_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub Enabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    Enabled: usize,
    #[cfg(feature = "wtypes")]
    pub SetEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetEnabled: usize,
    pub Key: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    Value: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    SetValue: usize,
    pub ValueMapType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ValueMapType) -> windows_core::HRESULT,
    pub SetValueMapType: unsafe extern "system" fn(*mut core::ffi::c_void, ValueMapType) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IValueMapItem_Impl: super::oaidl::IDispatch_Impl {
    fn Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDescription(&self, description: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Enabled(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn SetEnabled(&self, enabled: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Key(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetKey(&self, key: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Value(&self) -> windows_core::Result<super::oaidl::VARIANT>;
    fn SetValue(&self, value: &super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn ValueMapType(&self) -> windows_core::Result<ValueMapType>;
    fn SetValueMapType(&self, r#type: ValueMapType) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IValueMapItem_Vtbl {
    pub const fn new<Identity: IValueMapItem_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Description<Identity: IValueMapItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, description: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IValueMapItem_Impl::Description(this) {
                    Ok(ok__) => {
                        description.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDescription<Identity: IValueMapItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, description: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IValueMapItem_Impl::SetDescription(this, core::mem::transmute(&description)).into()
            }
        }
        unsafe extern "system" fn Enabled<Identity: IValueMapItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IValueMapItem_Impl::Enabled(this) {
                    Ok(ok__) => {
                        enabled.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: IValueMapItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IValueMapItem_Impl::SetEnabled(this, core::mem::transmute_copy(&enabled)).into()
            }
        }
        unsafe extern "system" fn Key<Identity: IValueMapItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IValueMapItem_Impl::Key(this) {
                    Ok(ok__) => {
                        key.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetKey<Identity: IValueMapItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IValueMapItem_Impl::SetKey(this, core::mem::transmute(&key)).into()
            }
        }
        unsafe extern "system" fn Value<Identity: IValueMapItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IValueMapItem_Impl::Value(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetValue<Identity: IValueMapItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IValueMapItem_Impl::SetValue(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn ValueMapType<Identity: IValueMapItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: *mut ValueMapType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IValueMapItem_Impl::ValueMapType(this) {
                    Ok(ok__) => {
                        r#type.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetValueMapType<Identity: IValueMapItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: ValueMapType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IValueMapItem_Impl::SetValueMapType(this, core::mem::transmute_copy(&r#type)).into()
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Description: Description::<Identity, OFFSET>,
            SetDescription: SetDescription::<Identity, OFFSET>,
            Enabled: Enabled::<Identity, OFFSET>,
            SetEnabled: SetEnabled::<Identity, OFFSET>,
            Key: Key::<Identity, OFFSET>,
            SetKey: SetKey::<Identity, OFFSET>,
            Value: Value::<Identity, OFFSET>,
            SetValue: SetValue::<Identity, OFFSET>,
            ValueMapType: ValueMapType::<Identity, OFFSET>,
            SetValueMapType: SetValueMapType::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IValueMapItem as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IValueMapItem {}
pub const LegacyDataCollectorSet: windows_core::GUID = windows_core::GUID::from_u128(0x03837526_098b_11d8_9414_505054503030);
pub const LegacyDataCollectorSetCollection: windows_core::GUID = windows_core::GUID::from_u128(0x03837527_098b_11d8_9414_505054503030);
pub const LegacyTraceSession: windows_core::GUID = windows_core::GUID::from_u128(0x03837528_098b_11d8_9414_505054503030);
pub const LegacyTraceSessionCollection: windows_core::GUID = windows_core::GUID::from_u128(0x03837529_098b_11d8_9414_505054503030);
pub const PLAL_ALERT_CMD_LINE_A_NAME: u32 = 512;
pub const PLAL_ALERT_CMD_LINE_C_NAME: u32 = 1024;
pub const PLAL_ALERT_CMD_LINE_D_TIME: u32 = 2048;
pub const PLAL_ALERT_CMD_LINE_L_VAL: u32 = 4096;
pub const PLAL_ALERT_CMD_LINE_MASK: u32 = 32512;
pub const PLAL_ALERT_CMD_LINE_M_VAL: u32 = 8192;
pub const PLAL_ALERT_CMD_LINE_SINGLE: u32 = 256;
pub const PLAL_ALERT_CMD_LINE_U_TEXT: u32 = 16384;
pub type PLA_CABEXTRACT_CALLBACK = Option<unsafe extern "system" fn(filename: windows_core::PCWSTR, context: *mut core::ffi::c_void)>;
pub const PLA_CAPABILITY_AUTOLOGGER: u32 = 32;
pub const PLA_CAPABILITY_LEGACY_SESSION: u32 = 8;
pub const PLA_CAPABILITY_LEGACY_SVC: u32 = 16;
pub const PLA_CAPABILITY_LOCAL: u32 = 268435456;
pub const PLA_CAPABILITY_V1_SESSION: u32 = 2;
pub const PLA_CAPABILITY_V1_SVC: u32 = 1;
pub const PLA_CAPABILITY_V1_SYSTEM: u32 = 4;
pub type ResourcePolicy = i32;
pub const ServerDataCollectorSet: windows_core::GUID = windows_core::GUID::from_u128(0x03837531_098b_11d8_9414_505054503030);
pub const ServerDataCollectorSetCollection: windows_core::GUID = windows_core::GUID::from_u128(0x03837532_098b_11d8_9414_505054503030);
pub type StreamMode = i32;
pub const SystemDataCollectorSet: windows_core::GUID = windows_core::GUID::from_u128(0x03837546_098b_11d8_9414_505054503030);
pub const SystemDataCollectorSetCollection: windows_core::GUID = windows_core::GUID::from_u128(0x03837547_098b_11d8_9414_505054503030);
pub const TraceDataProvider: windows_core::GUID = windows_core::GUID::from_u128(0x03837513_098b_11d8_9414_505054503030);
pub const TraceDataProviderCollection: windows_core::GUID = windows_core::GUID::from_u128(0x03837511_098b_11d8_9414_505054503030);
pub const TraceSession: windows_core::GUID = windows_core::GUID::from_u128(0x0383751c_098b_11d8_9414_505054503030);
pub const TraceSessionCollection: windows_core::GUID = windows_core::GUID::from_u128(0x03837530_098b_11d8_9414_505054503030);
pub type ValueMapType = i32;
pub type WeekDays = i32;
pub const plaAlert: DataCollectorType = 3;
pub const plaApiTrace: DataCollectorType = 4;
pub const plaBinary: FileFormat = 3;
pub const plaBoth: StreamMode = 3;
pub const plaBuffering: StreamMode = 4;
pub const plaCommaSeparated: FileFormat = 0;
pub const plaCompiling: DataCollectorSetStatus = 2;
pub const plaComputer: AutoPathFormat = 2;
pub const plaConfiguration: DataCollectorType = 2;
pub const plaCreateCab: FolderActionSteps = 1;
pub const plaCreateHtml: DataManagerSteps = 4;
pub const plaCreateNew: CommitMode = 1;
pub const plaCreateOrModify: CommitMode = 3;
pub const plaCreateReport: DataManagerSteps = 1;
pub const plaCycle: ClockType = 3;
pub const plaDeleteCab: FolderActionSteps = 8;
pub const plaDeleteData: FolderActionSteps = 2;
pub const plaDeleteLargest: ResourcePolicy = 0;
pub const plaDeleteOldest: ResourcePolicy = 1;
pub const plaDeleteReport: FolderActionSteps = 16;
pub const plaEveryday: WeekDays = 127;
pub const plaFile: StreamMode = 1;
pub const plaFlag: ValueMapType = 2;
pub const plaFlagArray: ValueMapType = 3;
pub const plaFlushTrace: CommitMode = 32;
pub const plaFolderActions: DataManagerSteps = 8;
pub const plaFriday: WeekDays = 32;
pub const plaIndex: ValueMapType = 1;
pub const plaModify: CommitMode = 2;
pub const plaMonday: WeekDays = 2;
pub const plaMonthDayHour: AutoPathFormat = 256;
pub const plaMonthDayHourMinute: AutoPathFormat = 16384;
pub const plaNone: AutoPathFormat = 0;
pub const plaPattern: AutoPathFormat = 1;
pub const plaPending: DataCollectorSetStatus = 3;
pub const plaPerformance: ClockType = 1;
pub const plaPerformanceCounter: DataCollectorType = 0;
pub const plaRealTime: StreamMode = 2;
pub const plaResourceFreeing: DataManagerSteps = 16;
pub const plaRunOnce: WeekDays = 0;
pub const plaRunRules: DataManagerSteps = 2;
pub const plaRunning: DataCollectorSetStatus = 1;
pub const plaSaturday: WeekDays = 64;
pub const plaSendCab: FolderActionSteps = 4;
pub const plaSerialNumber: AutoPathFormat = 512;
pub const plaSql: FileFormat = 2;
pub const plaStopped: DataCollectorSetStatus = 0;
pub const plaSunday: WeekDays = 1;
pub const plaSystem: ClockType = 2;
pub const plaTabSeparated: FileFormat = 1;
pub const plaThursday: WeekDays = 16;
pub const plaTimeStamp: ClockType = 0;
pub const plaTrace: DataCollectorType = 1;
pub const plaTuesday: WeekDays = 4;
pub const plaUndefined: DataCollectorSetStatus = 4;
pub const plaUpdateRunningInstance: CommitMode = 16;
pub const plaValidateOnly: CommitMode = 4096;
pub const plaValidation: ValueMapType = 4;
pub const plaWednesday: WeekDays = 8;
pub const plaYearDayOfYear: AutoPathFormat = 1024;
pub const plaYearMonth: AutoPathFormat = 2048;
pub const plaYearMonthDay: AutoPathFormat = 4096;
pub const plaYearMonthDayHour: AutoPathFormat = 8192;
